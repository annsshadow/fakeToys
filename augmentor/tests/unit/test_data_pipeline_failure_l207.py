# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L207（B266⑤）：管道阶段失败不再让下游静默拿到上一阶段的输出

**缺陷**：`DataPipeline.run()` 的 except 分支只记 `result.error` 并打日志。
`stop_on_failure` 是 `add_stage` 的**默认 False**，所以阶段 B 抛异常时
`current` 从未被重新赋值 ⇒ **阶段 C 拿到的是阶段 A 的输出**，整条管道少跑一环。
而 `run()` 的返回值形状与条数和正常跑完完全一致，唯一信号是 `logger.error`
与 `self.results` 里那条 error。

**修法**（不动既有默认值、不改 `run()` 的返回形状）：
- 新增 `PipelineOutcome`，`run()` 把失败清档写进 `self.last_outcome.failed_stages`
  ——「有阶段失败了」这件事从此有机器可读出口；
- 新增 `run_strict()`：任何阶段失败即抛 `PipelineError`，不再跳过。
"""

import pytest

from augmentor.data_pipeline import DataPipeline, PipelineOutcome
from augmentor.exceptions import PipelineError


def _tag(tag):
    """给每条数据打上「经过哪些阶段」的标记，用来验证下游到底吃了哪一口"""

    def stage(items, context):
        return [dict(item, seen=item.get("seen", []) + [tag]) for item in items]

    return stage


def _boom(name):
    def stage(items, context):
        raise RuntimeError(f"{name} exploded")

    return stage


ITEMS = [{"id": 1}, {"id": 2}]


class TestFailureIsVisibleByDefault:
    """调用方不翻 results 也能知道有阶段失败了"""

    def test_last_outcome_records_the_failed_stage(self):
        pipe = DataPipeline("p").add_stage("a", _tag("a")).add_stage(
            "b", _boom("b")).add_stage("c", _tag("c"))
        pipe.run(ITEMS)
        assert pipe.last_outcome.failed_stages == ["b"], (
            "failed_stages 没记下失败的阶段 ⇒ 调用方仍只能翻 results"
        )

    def test_outcome_ok_is_false_on_failure(self):
        pipe = DataPipeline("p").add_stage("a", _boom("a"))
        pipe.run(ITEMS)
        assert pipe.last_outcome.ok is False

    def test_outcome_ok_is_true_on_success(self):
        pipe = DataPipeline("p").add_stage("a", _tag("a"))
        pipe.run(ITEMS)
        assert pipe.last_outcome.ok is True
        assert pipe.last_outcome.failed_stages == []

    def test_outcome_data_is_the_same_object_as_return(self):
        pipe = DataPipeline("p").add_stage("a", _tag("a"))
        result = pipe.run(ITEMS)
        assert pipe.last_outcome.data is result

    def test_stages_are_the_same_as_results(self):
        pipe = DataPipeline("p").add_stage("a", _tag("a"))
        pipe.run(ITEMS)
        assert pipe.last_outcome.stages == pipe.results


class TestDownstreamSeesThePreviousStageOutput:
    """这条钉住缺陷本身：跳过的阶段让下游拿到上一阶段的输出"""

    def test_stage_c_sees_stage_a_output_not_stage_b(self):
        pipe = DataPipeline("p").add_stage("a", _tag("a")).add_stage(
            "b", _boom("b")).add_stage("c", _tag("c"))
        result = pipe.run(ITEMS)
        # b 被跳过 ⇒ c 的输入是 a 的输出 ⇒ seen 里没有 "b"
        assert all(item["seen"] == ["a", "c"] for item in result), result

    def test_return_shape_is_unchanged_on_failure(self):
        """修法不改 run() 的返回形状——只补可见性"""
        pipe = DataPipeline("p").add_stage("a", _tag("a")).add_stage(
            "b", _boom("b")).add_stage("c", _tag("c"))
        result = pipe.run(ITEMS)
        assert isinstance(result, list)
        assert len(result) == len(ITEMS)


class TestRunStrict:
    """显式选择「不跳过」的那一档"""

    def test_strict_raises_on_stage_failure(self):
        pipe = DataPipeline("p").add_stage("a", _tag("a")).add_stage(
            "b", _boom("b")).add_stage("c", _tag("c"))
        with pytest.raises(PipelineError) as exc:
            pipe.run_strict(ITEMS)
        assert "b" in str(exc.value)
        assert "exploded" in str(exc.value)

    def test_strict_chains_the_original_exception(self):
        pipe = DataPipeline("p").add_stage("a", _boom("a"))
        with pytest.raises(PipelineError) as exc:
            pipe.run_strict(ITEMS)
        assert isinstance(exc.value.__cause__, RuntimeError), (
            "没有 from e 链接原始异常 ⇒ 排查时看不到真正的根因"
        )

    def test_strict_returns_full_result_on_success(self):
        pipe = (DataPipeline("p")
                .add_stage("a", _tag("a"))
                .add_stage("b", _tag("b")))
        result = pipe.run_strict(ITEMS)
        assert all(item["seen"] == ["a", "b"] for item in result)

    def test_strict_rejects_non_list_return(self):
        pipe = DataPipeline("p").add_stage("a", lambda items, ctx: "not a list")
        with pytest.raises(PipelineError):
            pipe.run_strict(ITEMS)

    def test_strict_does_not_mutate_input(self):
        pipe = DataPipeline("p").add_stage("a", _tag("a"))
        original = [{"id": 1}]
        pipe.run_strict(original)
        assert original == [{"id": 1}], "run_strict 改了调用方传进来的列表"


class TestRunSemanticsUnchanged:
    """防修过头：run() 的既有行为一个字不许变"""

    def test_stop_on_failure_still_stops(self):
        pipe = DataPipeline("p").add_stage("a", _tag("a")).add_stage(
            "b", _boom("b"), stop_on_failure=True).add_stage("c", _tag("c"))
        result = pipe.run(ITEMS)
        assert all(item["seen"] == ["a"] for item in result), result

    def test_all_success_runs_every_stage(self):
        pipe = (DataPipeline("p")
                .add_stage("a", _tag("a"))
                .add_stage("b", _tag("b"))
                .add_stage("c", _tag("c")))
        result = pipe.run(ITEMS)
        assert all(item["seen"] == ["a", "b", "c"] for item in result)

    def test_empty_pipeline_returns_input(self):
        pipe = DataPipeline("p")
        result = pipe.run(ITEMS)
        assert result == ITEMS

    def test_results_records_every_stage(self):
        pipe = DataPipeline("p").add_stage("a", _tag("a")).add_stage(
            "b", _boom("b")).add_stage("c", _tag("c"))
        pipe.run(ITEMS)
        assert [r.name for r in pipe.results] == ["a", "b", "c"]
        assert pipe.results[0].success is True
        assert pipe.results[1].success is False
        assert "exploded" in pipe.results[1].error
