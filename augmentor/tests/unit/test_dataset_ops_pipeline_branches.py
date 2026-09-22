# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""dataset_ops 不支持采样方法 + data_pipeline 报告阶段 补测"""

import pytest

from augmentor.data_pipeline import DataPipeline, report_stage
from augmentor.dataset_ops import DatasetOperations, SampleConfig


class TestSampleUnsupportedMethod:
    def test_unknown_method_raises(self):
        ops = DatasetOperations()
        items = [{"instruction": f"q{i}"} for i in range(10)]
        with pytest.raises(ValueError, match="不支持的采样方法"):
            ops.sample(items, SampleConfig(method="magic", size=3))

    def test_random_sample_size_capped(self):
        ops = DatasetOperations()
        items = [{"instruction": f"q{i}"} for i in range(5)]
        sampled = ops.sample(items, SampleConfig(method="random", size=99, seed=1))
        assert len(sampled) == 5  # min(size, len)


class TestPipelineReportStage:
    def test_report_stage_passthrough_and_context(self):
        pipeline = DataPipeline("p")
        pipeline.add_stage("report", report_stage("total", 42))
        ctx = {}
        items = [{"instruction": "q"}]
        result = pipeline.run(items, ctx)
        assert result == items  # 透传原数据
        assert ctx.get("total") == 42

    def test_pipeline_stop_on_failure(self):
        pipeline = DataPipeline("p")

        def boom(items, ctx):
            raise RuntimeError("阶段故障")

        pipeline.add_stage("bad", boom, stop_on_failure=True)
        items = [{"instruction": "q"}]
        pipeline.run(items)
        report = pipeline.get_report()
        assert "bad" in report["failed_stages"]
        bad = next(s for s in report["stages"] if s["name"] == "bad")
        assert bad["success"] is False
        assert "阶段故障" in bad["error"]

    def test_stage_must_return_list(self):
        pipeline = DataPipeline("p")
        pipeline.add_stage("wrong", lambda items, ctx: "not a list")
        pipeline.run([{"instruction": "q"}])
        report = pipeline.get_report()
        wrong = next(s for s in report["stages"] if s["name"] == "wrong")
        assert wrong["success"] is False
        assert "列表" in wrong["error"]
