# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""quality API 路由异常分支注入测试

各端点的 except FileNotFoundError(404) 与 except Exception(500) 分支
在正常 HTTP 调用下不可达（load_items 抛的是 HTTPException），
通过向路由模块注入 load_items 异常来确定性覆盖。
"""

import json
from pathlib import Path
from typing import Callable

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent


@pytest.fixture
def quality_client(tmp_path, monkeypatch):
    """隔离 CWD + 注入受控 load_items 的 TestClient

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        TestClient 实例（load_items 可被 monkeypatch 控制）
    """
    import os

    from fastapi.testclient import TestClient

    import api.deps as deps
    from api.main import app

    monkeypatch.chdir(tmp_path)
    Path(os.getcwd(), "q.json").write_text(
        json.dumps([{"instruction": "问题", "input": "", "output": "答"}], ensure_ascii=False),
        encoding="utf-8",
    )
    return TestClient(app)


def _patch_load_items(monkeypatch, fn: Callable):
    import api.routes.quality as quality

    monkeypatch.setattr(quality, "load_items", fn)


class TestFileNotFoundErrorBranches:
    """各端点 load_items 抛 FileNotFoundError 时应 404（而非 500）"""

    @pytest.mark.parametrize(
        "endpoint,payload",
        [
            ("/api/quality/evaluate", {"input_file": "q.json"}),
            ("/api/quality/dedup", {"input_file": "q.json"}),
            ("/api/quality/report", {"input_file": "q.json"}),
            ("/api/quality/clean", {"input_file": "q.json"}),
            ("/api/quality/annotate", {"input_file": "q.json"}),
            ("/api/quality/benchmark", {"input_file": "q.json"}),
            ("/api/quality/outliers", {"input_file": "q.json"}),
            ("/api/quality/profiling", {"input_file": "q.json"}),
        ],
    )
    def test_missing_file_maps_to_404(self, quality_client, monkeypatch, endpoint, payload):
        def _raise_missing(_filename):
            raise FileNotFoundError("no file")

        _patch_load_items(monkeypatch, _raise_missing)
        response = quality_client.post(endpoint, json=payload)
        assert response.status_code == 404


class TestGenericExceptionBranches:
    """各端点 load_items 抛通用异常时应 500"""

    @pytest.mark.parametrize(
        "endpoint,payload",
        [
            ("/api/quality/evaluate", {"input_file": "q.json"}),
            ("/api/quality/dedup", {"input_file": "q.json"}),
            ("/api/quality/report", {"input_file": "q.json"}),
            ("/api/quality/clean", {"input_file": "q.json"}),
            ("/api/quality/annotate", {"input_file": "q.json"}),
            ("/api/quality/benchmark", {"input_file": "q.json"}),
            ("/api/quality/profiling", {"input_file": "q.json"}),
        ],
    )
    def test_runtime_error_maps_to_500(self, quality_client, monkeypatch, endpoint, payload):
        def _raise(_filename):
            raise RuntimeError("boom")

        _patch_load_items(monkeypatch, _raise)
        response = quality_client.post(endpoint, json=payload)
        assert response.status_code == 500
        assert "boom" in response.json()["detail"]

    def test_outliers_value_error_maps_to_400(self, quality_client, monkeypatch):
        """outliers 专属 except ValueError → 400（与通用 500 区分）"""
        import api.routes.quality as quality
        import augmentor.outlier as outlier_module

        monkeypatch.setattr(
            quality, "load_items", lambda _f: [{"instruction": "x", "output": "y"}]
        )

        class _BoomDetector:
            def __init__(self, *args, **kwargs):
                raise ValueError("method 非法")

        monkeypatch.setattr(outlier_module, "OutlierDetector", _BoomDetector)
        response = quality_client.post(
            "/api/quality/outliers",
            json={"input_file": "q.json", "method": "zscore"},
        )
        assert response.status_code == 400
        assert "method 非法" in response.json()["detail"]
