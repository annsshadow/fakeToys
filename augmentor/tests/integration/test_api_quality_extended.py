# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""quality API 路由扩展测试

补齐 clean / annotate / benchmark / outliers 方法 / profiling save
端点与 404/500 降级分支。
"""

import json
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent


@pytest.fixture
def quality_env(tmp_path, monkeypatch):
    """隔离 CWD 到临时目录并写入样本数据

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Yields:
        (TestClient, 样本文件名, 临时目录)
    """
    import os

    from fastapi.testclient import TestClient

    from api.main import app

    monkeypatch.chdir(tmp_path)
    items = [
        {"instruction": f"问题{i}？", "input": "", "output": f"回答{i}" * 3}
        for i in range(1, 6)
    ] + [{"instruction": "超长" * 80, "input": "", "output": "很长" * 100}]
    name = "quality_ext.json"
    Path(os.getcwd(), name).write_text(
        json.dumps(items, ensure_ascii=False), encoding="utf-8"
    )
    yield TestClient(app), name, tmp_path


class TestCleanEndpoint:
    def test_clean_returns_counts(self, quality_env):
        client, name, _ = quality_env
        response = client.post("/api/quality/clean", json={"input_file": name})
        assert response.status_code == 200
        payload = response.json()
        assert payload["original_count"] == 6
        assert "language_distribution" in payload

    def test_clean_missing_file_404(self, quality_env):
        client, _, _ = quality_env
        response = client.post(
            "/api/quality/clean", json={"input_file": "missing.json"}
        )
        assert response.status_code == 404


class TestAnnotateEndpoint:
    def test_annotate_report_structure(self, quality_env):
        client, name, _ = quality_env
        response = client.post("/api/quality/annotate", json={"input_file": name})
        assert response.status_code == 200
        payload = response.json()
        assert "total_items" in payload or "intent_distribution" in payload


class TestBenchmarkEndpoint:
    def test_benchmark_metrics(self, quality_env):
        client, name, _ = quality_env
        response = client.post("/api/quality/benchmark", json={"input_file": name})
        assert response.status_code == 200
        assert "metrics" in response.json()


class TestOutliersEndpoint:
    def test_outliers_iqr(self, quality_env):
        client, name, _ = quality_env
        response = client.post(
            "/api/quality/outliers",
            json={"input_file": name, "method": "iqr", "field": "length"},
        )
        assert response.status_code == 200
        assert response.json()["method"] == "iqr"

    def test_outliers_bad_method_400(self, quality_env):
        client, name, _ = quality_env
        response = client.post(
            "/api/quality/outliers",
            json={"input_file": name, "method": "no_such_method"},
        )
        assert response.status_code == 400

    def test_outliers_missing_file_404(self, quality_env):
        client, _, _ = quality_env
        response = client.post(
            "/api/quality/outliers", json={"input_file": "nope.json"}
        )
        assert response.status_code == 404


class TestProfilingEndpoint:
    def test_profiling_report(self, quality_env):
        client, name, _ = quality_env
        response = client.post(
            "/api/quality/profiling", json={"input_file": name}
        )
        assert response.status_code == 200
        assert "total_items" in response.json()

    def test_profiling_save_to_file(self, quality_env):
        client, name, tmp = quality_env
        out = str(tmp / "profile_out.json")
        response = client.post(
            "/api/quality/profiling",
            json={"input_file": name, "save": True, "output_path": out},
        )
        assert response.status_code == 200
        assert Path(out).exists()


class TestEvaluateEndpoints:
    def test_evaluate_404(self, quality_env):
        client, _, _ = quality_env
        response = client.post(
            "/api/quality/evaluate", json={"input_file": "missing.json"}
        )
        assert response.status_code == 404

    def test_dedup_removed_count(self, quality_env):
        client, name, _ = quality_env
        response = client.post(
            "/api/quality/dedup", json={"input_file": name, "threshold": 0.5}
        )
        assert response.status_code == 200
        payload = response.json()
        assert payload["original_count"] == 6
        assert payload["deduplicated_count"] <= 6

    def test_report_endpoint(self, quality_env):
        client, name, _ = quality_env
        response = client.post(
            "/api/quality/report", json={"input_file": name}
        )
        assert response.status_code == 200
        assert "total_samples" in response.json()
