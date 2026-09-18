"""status API 路由测试

覆盖 /api/status 正常返回结构、管道缺失降级、依赖诊断嵌入。
"""

import pytest


@pytest.fixture
def client_with_pipeline(tmp_path, monkeypatch):
    """注入重定向存储的真实管道"""
    from fastapi.testclient import TestClient

    from augmentor import AugmentorPipeline, load_config
    from augmentor.checkpoint import CheckpointManager
    from augmentor.versioning import VersionManager

    import api.deps as deps
    from api.main import app

    import pathlib

    ai_dir = pathlib.Path(__file__).resolve().parent.parent.parent
    config = load_config(str(ai_dir / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    instance = AugmentorPipeline(config)
    instance.version_manager = VersionManager(storage_dir=str(tmp_path / "versions"))
    instance.checkpoint_manager = CheckpointManager(checkpoint_dir=str(tmp_path / "checkpoints"))
    monkeypatch.setattr(deps, "_pipeline", instance)
    return TestClient(app)


class TestStatusEndpoint:
    def test_status_ok_structure(self, client_with_pipeline):
        client = client_with_pipeline
        response = client.get("/api/status")
        assert response.status_code == 200
        payload = response.json()
        assert payload["status"] == "ok"
        assert payload["version"] == "2.0.0"
        assert "dependencies" in payload
        assert "installed" in payload["dependencies"]

    def test_status_reports_model_availability(self, client_with_pipeline):
        payload = client_with_pipeline.get("/api/status").json()
        # 无 API 密钥时模型后端降级为 None
        assert "model_available" in payload
        assert isinstance(payload["model_available"], bool)

    def test_status_degrades_when_pipeline_missing(self, monkeypatch):
        import api.deps as deps
        import api.routes.status as status_module

        from fastapi.testclient import TestClient
        from api.main import app

        monkeypatch.setattr(deps, "_pipeline", None)
        # 让 get_pipeline 抛出，验证降级路径
        def _boom():
            raise RuntimeError("no pipeline")

        monkeypatch.setattr(status_module, "get_pipeline", _boom)
        client = TestClient(app)
        response = client.get("/api/status")
        assert response.status_code == 200
        payload = response.json()
        assert payload["model_default"] == "unknown"
        assert payload["model_available"] is False

    def test_yaml_dep_present(self, client_with_pipeline):
        payload = client_with_pipeline.get("/api/status").json()
        assert payload["dependencies"]["installed"]["yaml"] is True
