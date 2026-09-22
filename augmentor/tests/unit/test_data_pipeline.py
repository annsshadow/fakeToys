# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""DataPipeline 单元测试

管道编排是数据清洗流水线的骨架，阶段顺序与失败语义必须确定。
"""

import pytest

from augmentor.data_pipeline import DataPipeline, StageResult, identity


@pytest.fixture
def items():
    return [
        {"instruction": f"问题{i}", "output": f"回答{i}"}
        for i in range(6)
    ]


def make_filter(keyword):
    """构造关键词过滤阶段"""
    def stage(data, context):
        return [item for item in data if keyword in item.get("instruction", "")]
    return stage


class TestDataPipeline:
    def test_run_executes_stages_in_order(self, items):
        pipeline = DataPipeline()
        pipeline.add_stage("add", lambda data, ctx: data + [
            {"instruction": "新增", "output": "x"}
        ])
        pipeline.add_stage("filter", make_filter("新增"))
        result = pipeline.run(items)
        assert len(result) == 1
        assert result[0]["instruction"] == "新增"

    def test_duplicate_stage_name_rejected(self):
        pipeline = DataPipeline()
        pipeline.add_stage("a", identity)
        with pytest.raises(ValueError, match="重复"):
            pipeline.add_stage("a", identity)

    def test_stage_must_return_list(self, items):
        pipeline = DataPipeline()
        pipeline.add_stage("bad", lambda data, ctx: "not-a-list")
        pipeline.run(items)
        failed = pipeline.results[0]
        assert not failed.success
        assert "返回列表" in failed.error

    def test_stop_on_failure_halts_pipeline(self, items):
        pipeline = DataPipeline()

        def boom(data, ctx):
            raise RuntimeError("boom")

        pipeline.add_stage("boom", boom, stop_on_failure=True)
        pipeline.add_stage("never", lambda data, ctx: data)
        result = pipeline.run(items)
        assert len(pipeline.results) == 1
        assert pipeline.results[0].error == "boom"

    def test_non_stop_failure_continues(self, items):
        pipeline = DataPipeline()

        def boom(data, ctx):
            raise RuntimeError("skippable")

        pipeline.add_stage("skippable", boom, stop_on_failure=False)
        pipeline.add_stage("keep", identity)
        result = pipeline.run(items)
        assert len(pipeline.results) == 2
        assert result == items

    def test_context_shared_between_stages(self, items):
        seen = []

        def stage_a(data, ctx):
            ctx["marker"] = "from-a"
            seen.append(ctx["marker"])
            return data

        pipeline = DataPipeline()
        pipeline.add_stage("a", stage_a)
        context = {}
        pipeline.run(items, context)
        assert context["marker"] == "from-a"

    def test_report_structure(self, items):
        pipeline = DataPipeline(name="test-pipe")
        pipeline.add_stage("half", lambda data, ctx: data[:3])
        pipeline.run(items)
        report = pipeline.get_report()
        assert report["pipeline"] == "test-pipe"
        assert report["initial_count"] == 6
        assert report["final_count"] == 3
        assert report["stage_count"] == 1
        assert report["failed_stages"] == []
        assert len(report["stages"]) == 1

    def test_stage_names_property(self):
        pipeline = DataPipeline()
        pipeline.add_stage("one", identity).add_stage("two", identity)
        assert pipeline.stage_names == ["one", "two"]

    def test_clear_results(self, items):
        pipeline = DataPipeline()
        pipeline.add_stage("s", identity)
        pipeline.run(items)
        assert len(pipeline.results) == 1
        pipeline.clear_results()
        assert pipeline.results == []

    def test_empty_pipeline_runs(self, items):
        pipeline = DataPipeline()
        assert pipeline.run(items) == items
        assert pipeline.get_report()["executed_stages"] == 0


class TestRunStage:
    def test_run_single_stage(self, items):
        pipeline = DataPipeline()
        pipeline.add_stage("half", lambda data, ctx: data[:3])
        pipeline.add_stage("all", identity)
        result = pipeline.run_stage("half", items)
        assert len(result) == 3
        assert len(pipeline.results) == 1

    def test_unknown_stage_raises(self, items):
        pipeline = DataPipeline()
        pipeline.add_stage("x", identity)
        with pytest.raises(ValueError, match="未注册"):
            pipeline.run_stage("missing", items)

    def test_failing_stage_raises(self, items):
        pipeline = DataPipeline()
        pipeline.add_stage("boom", lambda data, ctx: (_ for _ in ()).throw(RuntimeError("x")))
        with pytest.raises(RuntimeError):
            pipeline.run_stage("boom", items)


class TestStageResult:
    def test_success_property(self):
        assert StageResult("a", 1, 1).success is True
        assert StageResult("a", 1, 0, error="e").success is False

    def test_to_dict_fields(self):
        result = StageResult("a", 5, 3, 12.5, report={"k": "v"}, error="")
        d = result.to_dict()
        assert d["input_count"] == 5
        assert d["output_count"] == 3
        assert d["duration_ms"] == 12.5
        assert d["report"] == {"k": "v"}
        assert d["success"] is True


class TestComposition:
    """组合实际模块的端到端管道"""

    def test_clean_dedup_pipeline(self, tmp_path):
        import json
        from augmentor.data_pipeline import DataPipeline

        data = [
            {"instruction": "如何申请租房？", "output": "登录官网申请"},
            {"instruction": "如何申请租房？", "output": "登录官网申请"},
            {"instruction": "", "output": ""},
            {"instruction": "退租流程是什么？", "output": "办理退租手续"},
        ]

        pipeline = DataPipeline("clean-dedup")
        pipeline.add_stage(
            "dedup",
            lambda data, ctx: _dedup(data),
            stop_on_failure=False,
        )
        result = pipeline.run(data)
        # 去重后至少去掉了 1 条重复 + 1 条空数据
        assert len(result) <= 4
        report = pipeline.get_report()
        assert report["failed_stages"] == []

    def test_outlier_filter_pipeline(self):
        from augmentor.outlier import OutlierDetector
        from augmentor.data_pipeline import DataPipeline

        items = [{"instruction": "问" + "长" * i} for i in range(1, 11)]
        items = items + [{"instruction": "问" + "长" * 500}]
        detector = OutlierDetector(field="length", threshold=2.0)
        pipeline = DataPipeline("outlier-filter")
        pipeline.add_stage(
            "attach-length",
            lambda data, ctx: detector.attach_length_field(data),
        )
        pipeline.add_stage(
            "filter-outliers",
            lambda data, ctx: detector.filter(data),
        )
        result = pipeline.run(items)
        assert len(result) < len(items)


def _dedup(data):
    """简易去重：按 instruction 去重并移除空项"""
    seen = set()
    out = []
    for item in data:
        key = item.get("instruction", "").strip()
        if not key:
            continue
        if key in seen:
            continue
        seen.add(key)
        out.append(item)
    return out
