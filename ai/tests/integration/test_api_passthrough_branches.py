"""L69：api 路由异常直通分支 + 限流器 reset

自然路径下 require_file 已将缺文件转为 HTTPException(404)，
`except FileNotFoundError` 与内层 `except HTTPException: raise` 分支
需要确定性注入才能命中；限流器 reset 的 None/指定键两条路径同样补齐。
"""

import json
import os
from pathlib import Path
from types import SimpleNamespace

import pytest


@pytest.fixture
def env(tmp_path, monkeypatch):
    from fastapi.testclient import TestClient

    from api.main import app

    monkeypatch.chdir(tmp_path)
    items = [
        {"instruction": "如何申请？", "input": "", "output": "登录"},
        {"instruction": "多少钱？", "input": "", "output": "按月"},
    ]
    Path(os.getcwd(), "a.json").write_text(
        json.dumps(items, ensure_ascii=False), encoding="utf-8"
    )
    yield TestClient(app), tmp_path


def _boom_fnf(filename):
    raise FileNotFoundError(filename)


class TestFileNotFoundBranches:
    def test_audit_file_not_found_404(self, env, monkeypatch):
        import api.routes.audit as audit_module

        monkeypatch.setattr(audit_module, "load_items", _boom_fnf)
        client, _ = env
        response = client.post("/api/audit", json={"input_file": "a.json"})
        assert response.status_code == 404

    def test_leakage_file_not_found_404(self, env, monkeypatch):
        import api.routes.leakage as leakage_module

        monkeypatch.setattr(leakage_module, "load_items", _boom_fnf)
        client, _ = env
        response = client.post(
            "/api/leakage/check",
            json={"train_file": "a.json", "test_file": "a.json"},
        )
        assert response.status_code == 404

    def test_privacy_file_not_found_404(self, env, monkeypatch):
        import api.routes.privacy as privacy_module

        monkeypatch.setattr(privacy_module, "load_items", _boom_fnf)
        client, _ = env
        response = client.post("/api/privacy/sanitize", json={"input_file": "a.json"})
        assert response.status_code == 404


def _boom_http(filename):
    from fastapi import HTTPException

    raise HTTPException(status_code=404, detail="注入的直通异常")


class TestQualityHttpPassthrough:
    @pytest.mark.parametrize(
        "path,payload",
        [
            ("/api/quality/report", {"input_file": "a.json"}),
            ("/api/quality/annotate", {"input_file": "a.json"}),
            ("/api/quality/benchmark", {"input_file": "a.json"}),
            ("/api/quality/profiling", {"input_file": "a.json"}),
        ],
    )
    def test_http_exception_passes_through(self, env, monkeypatch, path, payload):
        import api.routes.quality as quality_module

        monkeypatch.setattr(quality_module, "load_items", _boom_http)
        client, _ = env
        response = client.post(path, json=payload)
        assert response.status_code == 404
        assert "直通" in response.json()["detail"]

    def test_outliers_generic_500(self, env, monkeypatch):
        import augmentor.outlier as outlier_module

        class _BoomDetector:
            def __init__(self, *args, **kwargs):
                raise RuntimeError("outlier boom")

        monkeypatch.setattr(outlier_module, "OutlierDetector", _BoomDetector)
        client, _ = env
        response = client.post(
            "/api/quality/outliers",
            json={"input_file": "a.json", "method": "zscore", "threshold": 3.0},
        )
        assert response.status_code == 500
        assert "outlier boom" in response.json()["detail"]


class TestDataRoutePassthrough:
    def test_upload_http_exception_500(self, env, monkeypatch):
        from fastapi import HTTPException

        import api.routes.data as data_module

        async def _boom_write(file_path, data):
            raise HTTPException(status_code=500, detail="disk full")

        monkeypatch.setattr(data_module, "write_json_file", _boom_write)
        client, _ = env
        response = client.post(
            "/api/data/upload",
            files={"file": ("a.json", b"[]", "application/json")},
        )
        assert response.status_code == 500
        assert response.json()["detail"] == "disk full"

    def test_analyze_http_exception_passthrough(self, env, monkeypatch):
        from fastapi import HTTPException

        import api.routes.data as data_module

        def _analyze(file_path):
            raise HTTPException(status_code=503, detail="pipeline busy")

        fake_pipeline = SimpleNamespace(analyze_dataset=_analyze, visualize_dataset=None)
        monkeypatch.setattr(data_module, "get_pipeline", lambda: fake_pipeline)
        client, _ = env
        response = client.get("/api/analyze/a.json")
        assert response.status_code == 503
        assert response.json()["detail"] == "pipeline busy"

    def test_visualize_http_exception_passthrough(self, env, monkeypatch):
        from fastapi import HTTPException

        import api.routes.data as data_module

        def _visualize(file_path):
            raise HTTPException(status_code=503, detail="viz busy")

        fake_pipeline = SimpleNamespace(analyze_dataset=None, visualize_dataset=_visualize)
        monkeypatch.setattr(data_module, "get_pipeline", lambda: fake_pipeline)
        client, _ = env
        response = client.get("/api/visualize/a.json")
        assert response.status_code == 503
        assert response.json()["detail"] == "viz busy"


class TestAugmentStart500:
    def test_pipeline_error_500(self, env, monkeypatch):
        import api.routes.augment as augment_module

        def _boom():
            raise RuntimeError("pipeline down")

        monkeypatch.setattr(augment_module, "get_pipeline", _boom)
        client, _ = env
        response = client.post(
            "/api/augment/start",
            json={"input_file": "a.json", "output_file": "out.json"},
        )
        assert response.status_code == 500
        assert "pipeline down" in response.json()["detail"]


class TestRateLimiterReset:
    def test_reset_none_key_clears_all(self):
        from api.middleware.rate_limit import RateLimiter

        limiter = RateLimiter(max_requests=10, window_seconds=60)
        limiter.allow("a")
        limiter.allow("b")
        assert limiter._hits
        limiter.reset()  # key=None → 清空全部
        assert limiter._hits == {}

    def test_reset_specific_key(self):
        from api.middleware.rate_limit import RateLimiter

        limiter = RateLimiter(max_requests=10, window_seconds=60)
        limiter.allow("a")
        limiter.allow("b")
        limiter.reset("a")
        assert "a" not in limiter._hits
        assert "b" in limiter._hits
