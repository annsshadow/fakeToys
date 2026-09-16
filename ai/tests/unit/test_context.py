"""ContextAugmentor 单元测试

覆盖多轮对话生成：后续问题抽取、重复过滤、串行/并行批量生成。
"""

import pytest

from augmentor.context import ContextAugmentor


class ScriptedBackend:
    """按脚本顺序返回响应的假模型后端"""

    def __init__(self, responses):
        self.responses = list(responses)
        self.prompts = []

    def generate(self, prompt: str) -> str:
        self.prompts.append(prompt)
        if not self.responses:
            raise RuntimeError("脚本响应已耗尽")
        response = self.responses.pop(0)
        if isinstance(response, Exception):
            raise response
        return response


class TestFollowUpQuestion:
    def test_strips_follow_up_prefix(self):
        augmentor = ContextAugmentor(ScriptedBackend(["后续问题：如何续租？"]))
        result = augmentor._generate_follow_up_question([], "如何入住？", "通过 App 申请。")
        assert result == "如何续租？"

    def test_strips_question_prefix(self):
        augmentor = ContextAugmentor(ScriptedBackend(["问题：押金怎么退？"]))
        result = augmentor._generate_follow_up_question([], "如何入住？", "通过 App 申请。")
        assert result == "押金怎么退？"

    def test_prompt_contains_history_roles(self):
        backend = ScriptedBackend(["还想问什么？"])
        augmentor = ContextAugmentor(backend)
        history = [
            {"role": "user", "content": "历史问题"},
            {"role": "assistant", "content": "历史回答"},
        ]
        augmentor._generate_follow_up_question(history, "当前问题", "当前回答")

        prompt = backend.prompts[0]
        assert "用户: 历史问题" in prompt
        assert "客服: 历史回答" in prompt
        assert "用户: 当前问题" in prompt
        assert "客服: 当前回答" in prompt

    def test_returns_empty_string_on_model_failure(self):
        augmentor = ContextAugmentor(ScriptedBackend([RuntimeError("模型超时")]))
        assert augmentor._generate_follow_up_question([], "q", "a") == ""


class TestGenerateMultiTurn:
    def test_builds_history_across_turns(self):
        augmentor = ContextAugmentor(ScriptedBackend(["Q2", "Q3"]), num_turns=3)
        result = augmentor.generate_multi_turn({"instruction": "Q1", "output": "A1"})

        assert result["instruction"] == "Q3"
        assert result["output"] == "A1"
        assert [t["content"] for t in result["history"]] == ["Q1", "A1", "Q2", "A1"]
        assert result["system"] == "你是安居乐寓的智能客服"

    def test_preserves_input_and_system_fields(self):
        augmentor = ContextAugmentor(ScriptedBackend([]), num_turns=1)
        result = augmentor.generate_multi_turn(
            {"instruction": "Q1", "output": "A1", "input": "上下文", "system": "自定义"}
        )
        assert result["input"] == "上下文"
        assert result["system"] == "自定义"
        assert result["history"] == []

    def test_stops_when_follow_up_is_empty(self):
        augmentor = ContextAugmentor(ScriptedBackend([""]), num_turns=3)
        result = augmentor.generate_multi_turn({"instruction": "Q1", "output": "A1"})

        assert result["instruction"] == "Q1"
        assert result["history"] == []

    def test_skips_follow_up_present_in_existing_histories(self):
        augmentor = ContextAugmentor(ScriptedBackend(["Q2", "Q3"]), num_turns=3)
        existing = [[{"role": "user", "content": "Q2"}]]
        result = augmentor.generate_multi_turn(
            {"instruction": "Q1", "output": "A1"}, existing_histories=existing
        )

        # 第一轮的 Q2 被判定为重复并跳过，第二轮 Q3 被采纳
        assert result["instruction"] == "Q3"
        assert [t["content"] for t in result["history"]] == ["Q1", "A1"]

    def test_existing_histories_without_duplicate_are_accepted(self):
        augmentor = ContextAugmentor(ScriptedBackend(["Q2"]), num_turns=2)
        existing = [[{"role": "user", "content": "别的历史"}]]
        result = augmentor.generate_multi_turn(
            {"instruction": "Q1", "output": "A1"}, existing_histories=existing
        )
        assert result["instruction"] == "Q2"
        assert len(result["history"]) == 2


class TestBatchGenerate:
    def test_serial_generation_keeps_order(self):
        backend = ScriptedBackend(["Q1-2", "Q2-2", "Q3-2"])
        augmentor = ContextAugmentor(backend, num_turns=2)
        items = [
            {"instruction": "Q1", "output": "A1"},
            {"instruction": "Q2", "output": "A2"},
            {"instruction": "Q3", "output": "A3"},
        ]
        results = augmentor.batch_generate(items, use_parallel=False)

        assert len(results) == 3
        assert [r["instruction"] for r in results] == ["Q1-2", "Q2-2", "Q3-2"]

    def test_single_item_uses_serial_path(self):
        backend = ScriptedBackend(["Q1-2"])
        augmentor = ContextAugmentor(backend, num_turns=2)
        results = augmentor.batch_generate([{"instruction": "Q1", "output": "A1"}])

        assert len(results) == 1
        assert results[0]["instruction"] == "Q1-2"

    def test_parallel_generation_returns_all_items(self):
        backend = ScriptedBackend(["R1", "R2", "R3", "R4"])
        augmentor = ContextAugmentor(backend, num_turns=2, max_workers=2)
        items = [{"instruction": f"Q{i}", "output": f"A{i}"} for i in range(4)]
        results = augmentor.batch_generate(items, use_parallel=True)

        assert len(results) == 4
        assert all(len(r["history"]) == 2 for r in results)

    def test_parallel_failure_does_not_drop_other_items(self):
        backend = ScriptedBackend([RuntimeError("boom"), "R2", "R3"])
        augmentor = ContextAugmentor(backend, num_turns=2, max_workers=1)
        items = [{"instruction": f"Q{i}", "output": f"A{i}"} for i in range(3)]
        results = augmentor.batch_generate(items, use_parallel=True)

        # 失败项退化为无历史的单轮数据，但不会导致整体丢失
        assert len(results) == 3
        assert sum(1 for r in results if r["history"] == []) == 1


class TestConvertSingleToMultiTurn:
    def test_num_turns_override_is_restored(self):
        backend = ScriptedBackend(["Q1-2"])
        augmentor = ContextAugmentor(backend, num_turns=5)
        results = augmentor.convert_single_to_multi_turn(
            [{"instruction": "Q1", "output": "A1"}], num_turns=2
        )

        assert results[0]["instruction"] == "Q1-2"
        assert augmentor.num_turns == 5

    def test_without_override_uses_instance_setting(self):
        backend = ScriptedBackend(["Q1-2"])
        augmentor = ContextAugmentor(backend, num_turns=2)
        results = augmentor.convert_single_to_multi_turn(
            [{"instruction": "Q1", "output": "A1"}]
        )
        assert results[0]["instruction"] == "Q1-2"
        assert augmentor.num_turns == 2


class TestAddHistoryToSingleTurn:
    def test_history_accumulates_from_previous_items(self):
        augmentor = ContextAugmentor(ScriptedBackend([]))
        items = [
            {"instruction": "Q1", "output": "A1"},
            {"instruction": "Q2", "output": "A2"},
            {"instruction": "Q3", "output": "A3"},
        ]
        results = augmentor.add_history_to_single_turn(items, history_length=2)

        assert results[0]["history"] == []
        assert [t["content"] for t in results[1]["history"]] == ["Q1", "A1"]
        assert [t["content"] for t in results[2]["history"]] == ["Q1", "A1", "Q2", "A2"]

    def test_original_qa_is_unchanged(self):
        augmentor = ContextAugmentor(ScriptedBackend([]))
        items = [{"instruction": "Q1", "output": "A1", "input": "ctx", "system": "sys"}]
        result = augmentor.add_history_to_single_turn(items)[0]

        assert result["instruction"] == "Q1"
        assert result["output"] == "A1"
        assert result["input"] == "ctx"
        assert result["system"] == "sys"

    def test_empty_input_returns_empty_list(self):
        augmentor = ContextAugmentor(ScriptedBackend([]))
        assert augmentor.add_history_to_single_turn([]) == []
