"""privacy API 路由测试

/api/privacy/sanitize 脱敏往返与 404/500 分支、/api/privacy/patterns 列表。
"""

import json
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent


@pytest.fixture
def privacy_env(tmp_path, monkeypatch):
    """隔离 CWD 并写入含 PII 的数据文件

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Yields:
        (TestClient, 数据文件名, 临时目录)
    """
    import os

    from fastapi.testclient import TestClient

    from api.main import app

    monkeypatch.chdir(tmp_path)
    items = [
        {"instruction": "联系 zhang@example.com 或 13800138000", "input": "", "output": "可"},
        {"instruction": "身份证 11010119900307123X", "input": "", "output": "好"},
    ]
    name = "pii_api.json"
    Path(os.getcwd(), name).write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    yield TestClient(app), name, tmp_path


class TestSanitizeEndpoint:
    def test_sanitize_returns_items_and_report(self, privacy_env):
        client, name, _ = privacy_env
        response = client.post("/api/privacy/sanitize", json={"input_file": name})
        assert response.status_code == 200
        payload = response.json()
        joined = json.dumps(payload["items"], ensure_ascii=False)
        assert "zhang@example.com" not in joined
        assert "13800138000" not in joined
        assert payload["report"]["touched_items"] == 2
        assert "email" in payload["report"]["matches"]

    def test_sanitize_missing_file_404(self, privacy_env):
        client, _, _ = privacy_env
        response = client.post(
            "/api/privacy/sanitize", json={"input_file": "missing.json"}
        )
        assert response.status_code == 404

    def test_sanitize_custom_fields(self, privacy_env):
        client, _, tmp = privacy_env
        custom = Path(tmp, "custom_pii.json")
        custom.write_text(
            json.dumps([{"question": "邮箱 a@b.com", "answer": "13800138000"}],
                      ensure_ascii=False),
            encoding="utf-8",
        )
        response = client.post(
            "/api/privacy/sanitize",
            json={"input_file": "custom_pii.json", "fields": ["question", "answer"]},
        )
        assert response.status_code == 200
        items = response.json()["items"]
        assert "[EMAIL]" in items[0]["question"]
        assert "[PHONE]" in items[0]["answer"]

    def test_sanitize_include_extra(self, privacy_env):
        client, _, tmp = privacy_env
        extra = Path(tmp, "card.json")
        extra.write_text(
            json.dumps([{"instruction": "卡号 4111 1111 1111 1111", "input": "", "output": ""}],
                      ensure_ascii=False),
            encoding="utf-8",
        )
        response = client.post(
            "/api/privacy/sanitize",
            json={"input_file": "card.json", "include_extra": True},
        )
        assert response.status_code == 200
        assert "credit_card" in response.json()["report"]["matches"]


class TestPatternsEndpoint:
    def test_patterns_listed(self, privacy_env):
        client, _, _ = privacy_env
        response = client.get("/api/privacy/patterns")
        assert response.status_code == 200
        payload = response.json()
        assert "email" in payload["default"]
        assert "credit_card" in payload["extra"]
