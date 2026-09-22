"""audit/leakage/privacy 路由异常分支注入测试

正常路径已在各功能测试覆盖；本文件向路由模块注入 load_items 异常，
确定性覆盖 except FileNotFoundError(404) 与 except Exception(500) 分支。
"""

import json
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent


@pytest.fixture
def env(tmp_path, monkeypatch):
    import os

    from fastapi.testclient import TestClient

    from api.main import app

    monkeypatch.chdir(tmp_path)
    items = [
        {"instruction": "如何申请？", "input": "", "output": "登录"},
        {"instruction": "多少钱？", "input": "", "output": "按月"},
    ]
    Path(os.getcwd(), "a.json").write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    Path(os.getcwd(), "b.json").write_text(json.dumps(items[:1], ensure_ascii=False), encoding="utf-8")
    yield TestClient(app), tmp_path


class TestAuditErrorBranches:
    def test_missing_input_404(self, env, monkeypatch):
        client, _ = env
        response = client.post("/api/audit", json={"input_file": "missing.json"})
        assert response.status_code == 404

    def test_missing_reference_404(self, env, monkeypatch):
        client, _ = env
        response = client.post(
            "/api/audit", json={"input_file": "a.json", "reference_file": "missing.json"}
        )
        assert response.status_code == 404

    def test_generic_error_500(self, env, monkeypatch):
        import api.routes.audit as audit_module

        def _boom(filename):
            raise RuntimeError("内部错误")

        monkeypatch.setattr(audit_module, "load_items", _boom)
        client, _ = env
        response = client.post("/api/audit", json={"input_file": "a.json"})
        assert response.status_code == 500
        assert "内部错误" in response.json()["detail"]


class TestLeakageErrorBranches:
    def test_missing_train_404(self, env, monkeypatch):
        client, _ = env
        response = client.post(
            "/api/leakage/check",
            json={"train_file": "missing.json", "test_file": "b.json"},
        )
        assert response.status_code == 404

    def test_missing_test_404(self, env, monkeypatch):
        client, _ = env
        response = client.post(
            "/api/leakage/check",
            json={"train_file": "a.json", "test_file": "missing.json"},
        )
        assert response.status_code == 404

    def test_generic_error_500(self, env, monkeypatch):
        import api.routes.leakage as leakage_module

        def _boom(filename):
            raise RuntimeError("崩了")

        monkeypatch.setattr(leakage_module, "load_items", _boom)
        client, _ = env
        response = client.post(
            "/api/leakage/check",
            json={"train_file": "a.json", "test_file": "b.json"},
        )
        assert response.status_code == 500


class TestPrivacyErrorBranches:
    def test_missing_input_404(self, env, monkeypatch):
        client, _ = env
        response = client.post("/api/privacy/sanitize", json={"input_file": "missing.json"})
        assert response.status_code == 404

    def test_generic_error_500(self, env, monkeypatch):
        import api.routes.privacy as privacy_module

        def _boom(filename):
            raise RuntimeError("脱敏器故障")

        monkeypatch.setattr(privacy_module, "load_items", _boom)
        client, _ = env
        response = client.post("/api/privacy/sanitize", json={"input_file": "a.json"})
        assert response.status_code == 500
        assert "脱敏器故障" in response.json()["detail"]
