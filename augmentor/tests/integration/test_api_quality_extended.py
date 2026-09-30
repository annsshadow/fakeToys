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


class TestHealthGateEndpoint:
    """`/api/quality/health-gate`：健康分 + 质量门禁

    断言重心是「指标缺失时的判定形态」。`GateRule.evaluate` 对缺失的指标键返回
    False，所以调用方没传 `pass_rate` 时端点必须**跳过**那条规则并把它记进
    `skipped_rules`；照原样跑三条规则会把「没人算过通过率」表达成「通过率不及格」。
    """

    def test_defaults_skip_the_unmeasurable_rule(self, quality_env):
        client, name, _ = quality_env
        response = client.post("/api/quality/health-gate", json={"input_file": name})
        assert response.status_code == 200
        payload = response.json()
        assert payload["health"]["total_samples"] == 6
        assert payload["gate"]["verdict"] == "passed"
        assert payload["skipped_rules"] == ["pass_rate"]
        assert "pass_rate" not in payload["gate"]["metrics"]
        # 6 条 instruction 全唯一
        assert payload["gate"]["metrics"]["duplicate_rate"] == 0.0

    def test_supplied_pass_rate_participates(self, quality_env):
        client, name, _ = quality_env
        response = client.post(
            "/api/quality/health-gate",
            json={"input_file": name, "pass_rate": 0.2},
        )
        assert response.status_code == 200
        payload = response.json()
        assert payload["skipped_rules"] == []
        assert payload["gate"]["metrics"]["pass_rate"] == 0.2
        assert payload["gate"]["failed_rules"] == ["pass_rate"]
        assert payload["gate"]["verdict"] == "failed"
        assert payload["gate"]["passed"] is False

    def test_valid_weights_are_echoed(self, quality_env):
        client, name, _ = quality_env
        response = client.post(
            "/api/quality/health-gate",
            json={"input_file": name, "weights": [0.1, 0.2, 0.3, 0.4]},
        )
        assert response.status_code == 200
        assert response.json()["health"]["weights"] == [0.1, 0.2, 0.3, 0.4]

    def test_bad_weights_rejected_before_reading_file(self, quality_env):
        """非法权重必须报 400，而且**早于**读文件

        故意用一个不存在的文件：若校验排在读取之后，这里会先撞上 404，
        客户端就把「参数写错」读成「文件没了」。
        """
        client, _, _ = quality_env
        response = client.post(
            "/api/quality/health-gate",
            json={"input_file": "missing.json", "weights": [0.5, 0.5]},
        )
        assert response.status_code == 400
        assert "4 个元素" in response.json()["detail"]

    def test_weights_must_sum_to_one(self, quality_env):
        client, name, _ = quality_env
        response = client.post(
            "/api/quality/health-gate",
            json={"input_file": name, "weights": [0.1, 0.2, 0.3, 0.9]},
        )
        assert response.status_code == 400
        assert "1.0" in response.json()["detail"]

    def test_pass_rate_out_of_range_rejected(self, quality_env):
        client, name, _ = quality_env
        response = client.post(
            "/api/quality/health-gate",
            json={"input_file": name, "pass_rate": 1.5},
        )
        assert response.status_code == 400

    def test_missing_file_404(self, quality_env):
        client, _, _ = quality_env
        response = client.post(
            "/api/quality/health-gate", json={"input_file": "missing.json"}
        )
        assert response.status_code == 404

    def test_empty_dataset_rejected_not_passed(self, quality_env):
        """空数据集是 400，不是「指标全 0 于是放行」"""
        client, _, tmp_path = quality_env
        (tmp_path / "empty.json").write_text("[]", encoding="utf-8")
        response = client.post(
            "/api/quality/health-gate", json={"input_file": "empty.json"}
        )
        assert response.status_code == 400
        assert "为空" in response.json()["detail"]

    def test_block_on_warning_switch(self, quality_env):
        """completeness 是 warning 级：默认只告警，`block_on_warning` 才判负"""
        client, _, tmp_path = quality_env
        (tmp_path / "partial.json").write_text(
            json.dumps(
                [{"instruction": "问题一", "output": "回答一"}, {"instruction": "问题二"}],
                ensure_ascii=False,
            ),
            encoding="utf-8",
        )
        body = {"input_file": "partial.json"}

        warned = client.post("/api/quality/health-gate", json=body)
        assert warned.status_code == 200
        assert warned.json()["gate"]["verdict"] == "warned"
        assert warned.json()["gate"]["passed"] is True

        blocked = client.post(
            "/api/quality/health-gate", json={**body, "block_on_warning": True}
        )
        assert blocked.status_code == 200
        assert blocked.json()["gate"]["verdict"] == "failed"
        assert blocked.json()["gate"]["passed"] is False
