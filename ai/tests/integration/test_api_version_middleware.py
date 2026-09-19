"""version API 路由与请求日志中间件扩展测试

版本生命周期（create/list/diff/rollback/delete/history/data）+
get_pipeline 异常 500 降级 + 慢请求告警分支。
"""

import json
import os
from pathlib import Path
from typing import Any, Dict

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent

SAMPLE = [
    {"instruction": f"问题{i}？", "input": "", "output": f"回答{i}"}
    for i in range(1, 6)
]


@pytest.fixture
def version_env(tmp_path, monkeypatch):
    """隔离 CWD + 真实管道（版本存储 tmp），写两份样本数据

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Yields:
        (TestClient, 文件A名, 文件B名, 临时目录)
    """
    from fastapi.testclient import TestClient

    from augmentor import AugmentorPipeline, load_config
    from augmentor.checkpoint import CheckpointManager
    from augmentor.versioning import VersionManager

    from api.main import app

    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    instance = AugmentorPipeline(config)
    instance.version_manager = VersionManager(storage_dir=str(tmp_path / "versions"))
    instance.checkpoint_manager = CheckpointManager(checkpoint_dir=str(tmp_path / "checkpoints"))

    import api.deps as deps
    monkeypatch.setattr(deps, "_pipeline", instance)
    monkeypatch.chdir(tmp_path)

    a = "version_a.json"
    b = "version_b.json"
    Path(os.getcwd(), a).write_text(json.dumps(SAMPLE, ensure_ascii=False), encoding="utf-8")
    Path(os.getcwd(), b).write_text(
        json.dumps(SAMPLE[:2] + [{"instruction": "新数据", "input": "", "output": "新答案"}], ensure_ascii=False),
        encoding="utf-8",
    )
    yield TestClient(app), a, b, tmp_path


def _create(client, filename, label=None):
    payload: Dict[str, Any] = {}
    if label:
        payload["label"] = label
    response = client.post(
        "/api/versions/create",
        params={"filename": filename},
        json=payload,
    )
    assert response.status_code == 200, response.text
    return response.json()["version_id"]


class TestVersionLifecycle:
    def test_create_and_list(self, version_env):
        client, a, _, _ = version_env
        vid = _create(client, a, label="v1")
        response = client.get("/api/versions")
        assert response.status_code == 200
        versions = response.json()["versions"]
        assert any(v["version_id"] == vid for v in versions)
        assert any(v["label"] == "v1" for v in versions)

    def test_get_detail_and_data(self, version_env):
        client, a, _, _ = version_env
        vid = _create(client, a)

        detail = client.get(f"/api/versions/{vid}")
        assert detail.status_code == 200
        assert detail.json()["item_count"] == len(SAMPLE)

        data = client.get(f"/api/versions/{vid}/data")
        assert data.status_code == 200
        assert data.json()["items"] == SAMPLE

    def test_diff_versions(self, version_env):
        client, a, b, _ = version_env
        vid1 = _create(client, a)
        vid2 = _create(client, b)
        response = client.post(
            "/api/versions/diff", json={"version1": vid1, "version2": vid2}
        )
        assert response.status_code == 200
        payload = response.json()
        assert payload["version1"] == vid1
        assert payload["added_count"] >= 1
        assert payload["removed_count"] >= 1

    def test_rollback_and_delete(self, version_env):
        client, a, b, _ = version_env
        vid1 = _create(client, a)
        vid2 = _create(client, b)  # 使 vid1 不再是当前版本
        response = client.post(f"/api/versions/{vid2}/rollback")
        assert response.status_code == 200
        assert response.json()["success"] is True

        deleted = client.delete(f"/api/versions/{vid1}")
        assert deleted.status_code == 200
        assert deleted.json()["success"] is True

    def test_history_with_limit(self, version_env):
        client, a, _, _ = version_env
        _create(client, a)
        _create(client, a)
        response = client.get("/api/versions/history", params={"limit": 1})
        assert response.status_code == 200
        assert len(response.json()["history"]) <= 1

    def test_create_missing_file_404(self, version_env):
        client, _, _, _ = version_env
        response = client.post(
            "/api/versions/create", params={"filename": "no_such.json"}, json={}
        )
        assert response.status_code == 404


class TestVersionErrorPaths:
    def _stub_boom_pipeline(self, monkeypatch, route_module_name):
        import importlib

        module = importlib.import_module(route_module_name)

        class _BrokenManager:
            def __getattr__(self, _):
                raise RuntimeError("manager broken")

        class _BrokenPipeline:
            version_manager = _BrokenManager()
            checkpoint_manager = _BrokenManager()

        monkeypatch.setattr(module, "get_pipeline", lambda: _BrokenPipeline())

    def test_list_500_when_manager_broken(self, version_env, monkeypatch):
        client, *_ = version_env
        self._stub_boom_pipeline(monkeypatch, "api.routes.version")
        assert client.get("/api/versions").status_code == 500

    def test_history_500_when_manager_broken(self, version_env, monkeypatch):
        client, *_ = version_env
        self._stub_boom_pipeline(monkeypatch, "api.routes.version")
        assert client.get("/api/versions/history").status_code == 500


class TestSlowRequestMiddleware:
    def test_slow_request_logs_warning(self, caplog):
        """低于阈值的请求不告警，超阈值需记 WARNING 慢请求日志"""
        import time

        from fastapi import FastAPI

        from api.middleware import RequestLoggingMiddleware

        app = FastAPI()

        @app.get("/slow")
        def slow():
            time.sleep(0.02)
            return {"ok": True}

        @app.get("/fast")
        def fast():
            return {"ok": True}

        app.add_middleware(RequestLoggingMiddleware, slow_threshold=0.005)
        from fastapi.testclient import TestClient

        client = TestClient(app)
        with caplog.at_level("WARNING"):
            client.get("/fast")
            client.get("/slow")
        messages = [r.message for r in caplog.records if r.levelname == "WARNING"]
        assert any("慢请求" in m for m in messages)

    def test_process_time_header(self):
        from fastapi import FastAPI

        from api.middleware import RequestLoggingMiddleware

        app = FastAPI()

        @app.get("/t")
        def t():
            return {"ok": True}

        app.add_middleware(RequestLoggingMiddleware)
        from fastapi.testclient import TestClient

        client = TestClient(app)
        response = client.get("/t")
        assert "X-Process-Time" in response.headers
