# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""status API 路由测试

覆盖 /api/status 正常返回结构、管道缺失降级、依赖诊断嵌入。
"""

import pytest

import augmentor


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
        # 版本号单一来源：断言与包版本一致，而不是硬编码某个字面量
        assert payload["version"] == augmentor.__version__
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


class TestStatusRetryStats:
    """A66 模型重试统计的 API 出口（A112）：`/api/status` 回显后端重试计数

    放在 status 而非 health：`HealthResponse` 被容器探针依赖、明令不加可能失败/阻塞的
    字段，而 `get_status` 本就在 try 里读 `p.model_backend`。这三例锁的正是 `Optional`
    存在的理由——「没有后端」与「后端在、但一次没重试」是两件事，不能被折叠成同一个值。
    """

    def _client_with_backend(self, monkeypatch, backend):
        """把 `get_pipeline` 换成一个只暴露 status 需要的最小属性的桩管道"""
        import api.deps as deps
        import api.routes.status as status_module

        from fastapi.testclient import TestClient
        from api.main import app

        class _Cfg:
            default_model = "gpt-test"

        class _Pipe:
            config = _Cfg()
            model_backend = backend

        monkeypatch.setattr(deps, "_pipeline", None)
        monkeypatch.setattr(status_module, "get_pipeline", lambda: _Pipe())
        return TestClient(app)

    def test_no_backend_reports_none_not_zero(self, monkeypatch):
        """无后端 → 两键都是 `None`：把「没数据」与「重试过 0 次」区分开"""
        client = self._client_with_backend(monkeypatch, None)
        payload = client.get("/api/status").json()
        assert payload["model_available"] is False
        assert payload["model_retry_count"] is None
        assert payload["model_retry_wait_seconds"] is None

    def test_backend_with_zero_retries_reports_zero(self, monkeypatch):
        """后端在、但一次没重试 → `0` / `0.0`，**不能**被折叠成 `None`

        若实现写成 `backend.retry_count or None`，0 会被误判成「无后端」——这正是
        `Optional` 口径要防的混淆，故单独立一条。
        """
        class _Zero:
            retry_count = 0
            retry_wait_seconds = 0.0

        payload = self._client_with_backend(monkeypatch, _Zero()).get("/api/status").json()
        assert payload["model_available"] is True
        assert payload["model_retry_count"] == 0
        assert payload["model_retry_wait_seconds"] == 0.0

    def test_backend_retries_echo_actual_values(self, monkeypatch):
        """后端重试过 → 原样回显真实计数，不是硬编码 0"""
        class _Retried:
            retry_count = 2
            retry_wait_seconds = 4.5

        payload = self._client_with_backend(monkeypatch, _Retried()).get("/api/status").json()
        assert payload["model_retry_count"] == 2
        assert payload["model_retry_wait_seconds"] == pytest.approx(4.5)

    def test_degraded_pipeline_leaves_retry_keys_present_as_none(self, monkeypatch):
        """管道整体抛异常时，两键仍存在且为 `None`（响应契约不能因降级而少键）

        承 `test_status_degrades_when_pipeline_missing` 的降级路径，额外确认新键没被
        悄悄省略——少一键就会撞 `test_api_openapi_contract` 的「键集完全一致」判据。
        """
        import api.deps as deps
        import api.routes.status as status_module

        from fastapi.testclient import TestClient
        from api.main import app

        def _boom():
            raise RuntimeError("no pipeline")

        monkeypatch.setattr(deps, "_pipeline", None)
        monkeypatch.setattr(status_module, "get_pipeline", _boom)
        payload = TestClient(app).get("/api/status").json()
        assert payload["model_retry_count"] is None
        assert payload["model_retry_wait_seconds"] is None
