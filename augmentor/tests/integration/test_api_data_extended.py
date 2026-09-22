# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""data API 路由扩展测试

覆盖路径遍历防护、索引越界、upload 往返、demo 数据、
分析/可视化 500 降级与文件缺失 404。
"""

import json
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent


@pytest.fixture
def pipeline(tmp_path, monkeypatch):
    """使用临时目录的版本/检查点存储的真实管道

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Yields:
        AugmentorPipeline 实例
    """
    from augmentor import AugmentorPipeline, load_config
    from augmentor.checkpoint import CheckpointManager
    from augmentor.versioning import VersionManager

    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    instance = AugmentorPipeline(config)
    instance.version_manager = VersionManager(storage_dir=str(tmp_path / "versions"))
    instance.checkpoint_manager = CheckpointManager(checkpoint_dir=str(tmp_path / "checkpoints"))
    import api.deps as deps
    monkeypatch.chdir(tmp_path)  # data 路由按 CWD 读写文件，隔离到临时目录
    monkeypatch.setattr(deps, "_pipeline", instance)
    return instance


@pytest.fixture
def client(pipeline):
    from fastapi.testclient import TestClient

    from api.main import app
    return TestClient(app)


@pytest.fixture
def sample_file(client, tmp_path):
    """写入 CWD（tmp_path）的样本数据文件，返回文件名

    Args:
        client: TestClient
        tmp_path: pytest 临时目录

    Yields:
        文件名
    """
    import os

    items = [
        {"instruction": f"问题{i}？", "input": "", "output": f"回答{i}"}
        for i in range(1, 6)
    ]
    name = "api_data_ext.json"
    path = Path(os.getcwd()) / name
    path.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    try:
        yield name
    finally:
        path.unlink(missing_ok=True)


class TestPathTraversal:
    def test_safe_path_rejects_dotdot(self):
        """_safe_data_path 需对含 .. 的路径抛 400（防路径遍历）"""
        from fastapi import HTTPException

        from api.routes.data import _safe_data_path

        with pytest.raises(HTTPException) as exc_info:
            _safe_data_path("../etc/passwd.json")
        assert exc_info.value.status_code == 400

    def test_safe_path_normalizes_plain_name(self):
        from api.routes.data import _safe_data_path

        path = _safe_data_path("data.json")
        assert ".." not in path.parts


class TestMissingFile:
    def test_load_404(self, client):
        assert client.get("/api/data/load/no_such.json").status_code == 404

    def test_update_404(self, client):
        response = client.put(
            "/api/data/update/no_such.json", params={"index": 0}, json={"x": 1}
        )
        assert response.status_code == 404

    def test_delete_404(self, client):
        assert client.delete("/api/data/delete/no_such.json", params={"index": 0}).status_code == 404

    def test_analyze_404(self, client):
        assert client.get("/api/analyze/no_such.json").status_code == 404

    def test_visualize_404(self, client):
        assert client.get("/api/visualize/no_such.json").status_code == 404


class TestUpdateDelete:
    def test_update_item(self, client, sample_file):
        response = client.put(
            f"/api/data/update/{sample_file}",
            params={"index": 0},
            json={"instruction": "被改过", "input": "", "output": "新值"},
        )
        assert response.status_code == 200
        items = json.loads(
            (Path.cwd() / sample_file).read_text(encoding="utf-8")
        )
        assert items[0]["instruction"] == "被改过"

    def test_update_index_out_of_bounds(self, client, sample_file):
        response = client.put(
            f"/api/data/update/{sample_file}",
            params={"index": 99},
            json={"x": 1},
        )
        assert response.status_code == 400

    def test_delete_item(self, client, sample_file):
        response = client.delete(f"/api/data/delete/{sample_file}", params={"index": 2})
        assert response.status_code == 200
        items = json.loads((Path.cwd() / sample_file).read_text(encoding="utf-8"))
        assert len(items) == 4
        assert all(item["instruction"] != "问题3？" for item in items)

    def test_delete_index_out_of_bounds(self, client, sample_file):
        response = client.delete(f"/api/data/delete/{sample_file}", params={"index": -5})
        assert response.status_code == 400


class TestUpload:
    def test_upload_roundtrip(self, client):
        content = json.dumps(
            [{"instruction": "新数据", "input": "", "output": "新回答"}],
            ensure_ascii=False,
        ).encode("utf-8")
        response = client.post(
            "/api/data/upload",
            files={"file": ("uploaded.json", content, "application/json")},
        )
        assert response.status_code == 200
        payload = response.json()
        assert payload["count"] == 1
        saved = Path.cwd() / "uploaded.json"
        assert saved.exists()

    def test_upload_invalid_json_500(self, client):
        response = client.post(
            "/api/data/upload",
            files={"file": ("bad.json", b"not json", "application/json")},
        )
        assert response.status_code == 500


class TestAnalyzeVisualize:
    def test_analyze_returns_report(self, client, sample_file):
        response = client.get(f"/api/analyze/{sample_file}")
        assert response.status_code == 200
        assert "coverage_analysis" in response.json()

    def test_visualize_returns_charts(self, client, sample_file):
        response = client.get(f"/api/visualize/{sample_file}")
        assert response.status_code == 200
        assert "charts" in response.json()


class TestDemoData:
    def test_demo_data_builtin(self, client):
        response = client.get("/api/demo/data")
        assert response.status_code == 200
        items = response.json()
        assert len(items) == 5
        assert all("instruction" in i and "output" in i for i in items)


class TestLoadSearch:
    def test_search_filters_items(self, client, sample_file):
        response = client.get(
            f"/api/data/load/{sample_file}", params={"search": "回答1"}
        )
        assert response.status_code == 200
        payload = response.json()
        assert payload["total"] == 1
        assert payload["items"][0]["output"] == "回答1"

    def test_pagination_bounds(self, client, sample_file):
        response = client.get(
            f"/api/data/load/{sample_file}",
            params={"page": 2, "page_size": 2},
        )
        assert response.json()["items"][0]["instruction"] == "问题3？"
