# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""data 路由剩余 500 异常分支注入测试

向 read_json_file / get_pipeline 注入异常，确定性覆盖 load/update/
delete/upload/analyze/visualize 的通用 500 降级分支。
"""

import json
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent


@pytest.fixture
def data_env(tmp_path, monkeypatch):
    import os

    from fastapi.testclient import TestClient

    from augmentor import AugmentorPipeline, load_config
    from augmentor.checkpoint import CheckpointManager
    from augmentor.versioning import VersionManager

    import api.deps as deps
    from api.main import app

    monkeypatch.chdir(tmp_path)
    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    instance = AugmentorPipeline(config)
    instance.version_manager = VersionManager(storage_dir=str(tmp_path / "versions"))
    instance.checkpoint_manager = CheckpointManager(checkpoint_dir=str(tmp_path / "checkpoints"))
    monkeypatch.setattr(deps, "_pipeline", instance)

    items = [{"instruction": f"问题{i}", "input": "", "output": f"答{i}"} for i in range(3)]
    Path(os.getcwd(), "d.json").write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    yield TestClient(app), tmp_path


class TestLoad500:
    def test_read_failure_500(self, data_env, monkeypatch):
        import api.routes.data as data_module

        async def _boom(path):
            raise OSError("磁盘读失败")

        monkeypatch.setattr(data_module, "read_json_file", _boom)
        client, _ = data_env
        response = client.get("/api/data/load/d.json")
        assert response.status_code == 500
        assert "磁盘读失败" in response.json()["detail"]


class TestUpdateDelete500:
    def test_update_read_failure_500(self, data_env, monkeypatch):
        import api.routes.data as data_module

        async def _boom(path):
            raise OSError("读失败")

        monkeypatch.setattr(data_module, "read_json_file", _boom)
        client, _ = data_env
        response = client.put("/api/data/update/d.json", params={"index": 0}, json={"x": 1})
        assert response.status_code == 500

    def test_delete_read_failure_500(self, data_env, monkeypatch):
        import api.routes.data as data_module

        async def _boom(path):
            raise OSError("读失败")

        monkeypatch.setattr(data_module, "read_json_file", _boom)
        client, _ = data_env
        response = client.delete("/api/data/delete/d.json", params={"index": 0})
        assert response.status_code == 500


class TestUpload500:
    def test_write_failure_500(self, data_env, monkeypatch):
        import api.routes.data as data_module

        async def _boom(path, data):
            raise OSError("写失败")

        monkeypatch.setattr(data_module, "write_json_file", _boom)
        client, _ = data_env
        content = json.dumps([{"instruction": "x", "output": "y"}]).encode("utf-8")
        response = client.post(
            "/api/data/upload",
            files={"file": ("u.json", content, "application/json")},
        )
        assert response.status_code == 500


class TestAnalyzeVisualize500:
    def _break_pipeline(self, data_env, monkeypatch, target):
        import importlib

        module = importlib.import_module("api.routes.data")

        class _Broken:
            def analyze_dataset(self, *a, **k):
                raise RuntimeError(target)

            def visualize_dataset(self, *a, **k):
                raise RuntimeError(target)

        import api.deps as deps

        monkeypatch.setattr(deps, "_pipeline", _Broken())

    def test_analyze_500(self, data_env, monkeypatch):
        self._break_pipeline(data_env, monkeypatch, "分析故障")
        client, _ = data_env
        assert client.get("/api/analyze/d.json").status_code == 500

    def test_visualize_500(self, data_env, monkeypatch):
        self._break_pipeline(data_env, monkeypatch, "可视化故障")
        client, _ = data_env
        assert client.get("/api/visualize/d.json").status_code == 500
