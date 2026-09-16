"""DomainExpander 单元测试

覆盖主题抽取、三种扩展策略、批量并行扩展与种子生成。
"""

import pytest

from augmentor.expander import DomainExpander, ExpansionResult


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


class TestExtractTopics:
    def test_extracts_each_known_topic(self):
        expander = DomainExpander(ScriptedBackend([]))
        items = [
            {"instruction": "晨阳计划怎么参加？"},
            {"instruction": "租金怎么计算"},
            {"instruction": "换房需要什么条件"},
            {"instruction": "押金什么时候退"},
            {"instruction": "合同到期怎么办"},
            {"instruction": "维修报修流程"},
            {"instruction": "我要投诉"},
        ]
        topics = expander._extract_topics(items)

        assert set(topics) == {
            "晨阳计划",
            "租金相关",
            "换房服务",
            "押金相关",
            "合同相关",
            "维修服务",
            "投诉建议",
        }

    def test_rent_keyword_matches_rent_or_fangzu(self):
        expander = DomainExpander(ScriptedBackend([]))
        assert expander._extract_topics([{"instruction": "房租太贵了"}]) == ["租金相关"]

    def test_falls_back_to_generic_topic(self):
        expander = DomainExpander(ScriptedBackend([]))
        assert expander._extract_topics([{"instruction": "你好"}]) == ["通用问题"]
        assert expander._extract_topics([]) == ["通用问题"]


class TestTopicGenerationStrategies:
    def test_similar_topics_parses_json_array(self):
        backend = ScriptedBackend(['["主题A", "主题B", "主题C", "主题D", "主题E", "主题F"]'])
        expander = DomainExpander(backend)
        topics = expander._generate_similar_topics(["租金相关"], num_topics=5)

        assert topics == ["主题A", "主题B", "主题C", "主题D", "主题E"]
        assert "租金相关" in backend.prompts[0]

    def test_similar_topics_parses_plain_lines(self):
        expander = DomainExpander(ScriptedBackend(["1. 主题A\n2. 主题B\n3. 主题C"]))
        assert expander._generate_similar_topics(["租金"], num_topics=2) == ["主题A", "主题B"]

    def test_similar_topics_returns_empty_on_failure(self):
        expander = DomainExpander(ScriptedBackend([RuntimeError("模型不可用")]))
        assert expander._generate_similar_topics(["租金"]) == []

    def test_related_topics_parses_json_array(self):
        expander = DomainExpander(ScriptedBackend(['["关联A", "关联B"]']))
        assert expander._generate_related_topics(["租金"], num_topics=5) == ["关联A", "关联B"]

    def test_related_topics_returns_empty_on_failure(self):
        expander = DomainExpander(ScriptedBackend([RuntimeError("boom")]))
        assert expander._generate_related_topics(["租金"]) == []

    def test_scenario_topics_parses_json_array(self):
        expander = DomainExpander(ScriptedBackend(['["场景A"]']))
        assert expander._generate_scenario_topics(["租金"], num_topics=5) == ["场景A"]

    def test_scenario_topics_returns_empty_on_failure(self):
        expander = DomainExpander(ScriptedBackend([RuntimeError("boom")]))
        assert expander._generate_scenario_topics(["租金"]) == []


class TestExpand:
    @pytest.mark.parametrize(
        "strategy,response",
        [
            ("similar", '["相似主题"]'),
            ("related", '["相关主题"]'),
            ("scenario", '["使用场景"]'),
        ],
    )
    def test_each_strategy_is_routed(self, strategy, response):
        expander = DomainExpander(ScriptedBackend([response]))
        result = expander.expand([{"instruction": "租金怎么算"}], strategy=strategy)

        assert isinstance(result, ExpansionResult)
        assert result.strategy == strategy
        assert result.original_topics == ["租金相关"]
        assert result.expanded_topics == [
            {"topic": response[2:-2], "strategy": strategy}
        ]

    def test_unknown_strategy_raises_value_error(self):
        expander = DomainExpander(ScriptedBackend([]))
        with pytest.raises(ValueError, match="不支持的扩展策略"):
            expander.expand([{"instruction": "租金"}], strategy="unknown")

    def test_suggests_more_seed_data_when_topics_are_few(self):
        expander = DomainExpander(ScriptedBackend(['["A", "B"]']))
        result = expander.expand([{"instruction": "租金"}], num_topics=2)

        assert len(result.original_topics) == 1
        assert "原始主题较少，建议增加更多种子数据" in result.suggestions

    def test_suggests_checking_model_when_expansion_is_empty(self):
        expander = DomainExpander(ScriptedBackend([RuntimeError("boom")]))
        result = expander.expand([{"instruction": "租金"}])

        assert result.expanded_topics == []
        assert "扩展失败，建议检查模型配置或调整策略" in result.suggestions

    def test_no_suggestions_when_expansion_is_healthy(self):
        items = [
            {"instruction": "租金怎么算"},
            {"instruction": "押金怎么退"},
            {"instruction": "合同到期"},
        ]
        expander = DomainExpander(ScriptedBackend(['["A", "B"]']))
        result = expander.expand(items)

        assert len(result.original_topics) == 3
        assert result.suggestions == []


class TestBatchExpand:
    def test_serial_expands_each_strategy(self):
        backend = ScriptedBackend(['["S1"]', '["R1"]'])
        expander = DomainExpander(backend)
        expanded = expander.batch_expand(
            [{"instruction": "租金"}],
            strategies=["similar", "related"],
            use_parallel=False,
        )

        assert expanded == [
            {"topic": "S1", "strategy": "similar"},
            {"topic": "R1", "strategy": "related"},
        ]

    def test_single_strategy_uses_serial_path(self):
        expander = DomainExpander(ScriptedBackend(['["S1"]']))
        expanded = expander.batch_expand(
            [{"instruction": "租金"}], strategies=["similar"]
        )
        assert expanded == [{"topic": "S1", "strategy": "similar"}]

    def test_parallel_expands_all_default_strategies(self):
        backend = ScriptedBackend(['["S1"]', '["R1"]', '["C1"]'])
        expander = DomainExpander(backend, max_workers=3)
        expanded = expander.batch_expand([{"instruction": "租金"}])

        assert len(expanded) == 3
        assert {e["strategy"] for e in expanded} == {"similar", "related", "scenario"}

    def test_parallel_failure_skips_only_failed_strategy(self):
        backend = ScriptedBackend([RuntimeError("boom"), '["R1"]', '["C1"]'])
        expander = DomainExpander(backend, max_workers=1)
        expanded = expander.batch_expand([{"instruction": "租金"}])

        assert len(expanded) == 2
        assert {e["strategy"] for e in expanded} == {"related", "scenario"}


class TestGenerateSeeds:
    def test_generate_seeds_for_topic_parses_json(self):
        backend = ScriptedBackend(
            ['[{"instruction": "q1", "output": "a1"}, {"instruction": "q2", "output": "a2"}]']
        )
        expander = DomainExpander(backend)
        seeds = expander._generate_seeds_for_topic({"topic": "租金相关"}, 2)

        assert seeds == [
            {"instruction": "q1", "output": "a1"},
            {"instruction": "q2", "output": "a2"},
        ]
        assert "租金相关" in backend.prompts[0]

    def test_generate_seeds_for_topic_truncates_to_requested_count(self):
        backend = ScriptedBackend(
            ['[{"instruction": "q1"}, {"instruction": "q2"}, {"instruction": "q3"}]']
        )
        expander = DomainExpander(backend)
        assert len(expander._generate_seeds_for_topic({"topic": "t"}, 2)) == 2

    def test_generate_seeds_for_topic_returns_empty_for_non_json(self):
        expander = DomainExpander(ScriptedBackend(["这不是 JSON"]))
        assert expander._generate_seeds_for_topic({"topic": "t"}, 3) == []

    def test_generate_seeds_for_topic_returns_empty_on_failure(self):
        expander = DomainExpander(ScriptedBackend([RuntimeError("boom")]))
        assert expander._generate_seeds_for_topic({"topic": "t"}, 3) == []

    def test_serial_generate_seeds_from_topics(self):
        backend = ScriptedBackend(['[{"instruction": "q1"}]', '[{"instruction": "q2"}]'])
        expander = DomainExpander(backend)
        seeds = expander.generate_seeds_from_topics(
            [{"topic": "t1"}, {"topic": "t2"}], use_parallel=False
        )
        assert seeds == [{"instruction": "q1"}, {"instruction": "q2"}]

    def test_single_topic_uses_serial_path(self):
        expander = DomainExpander(ScriptedBackend(['[{"instruction": "q1"}]']))
        seeds = expander.generate_seeds_from_topics([{"topic": "t1"}])
        assert seeds == [{"instruction": "q1"}]

    def test_parallel_generate_seeds_from_topics(self):
        backend = ScriptedBackend(
            ['[{"instruction": "q1"}]', '[{"instruction": "q2"}]', '[{"instruction": "q3"}]']
        )
        expander = DomainExpander(backend, max_workers=3)
        seeds = expander.generate_seeds_from_topics(
            [{"topic": "t1"}, {"topic": "t2"}, {"topic": "t3"}], use_parallel=True
        )
        assert {s["instruction"] for s in seeds} == {"q1", "q2", "q3"}

    def test_parallel_generate_seeds_skips_failed_topic(self):
        backend = ScriptedBackend(
            [RuntimeError("boom"), '[{"instruction": "q2"}]', '[{"instruction": "q3"}]']
        )
        expander = DomainExpander(backend, max_workers=1)
        seeds = expander.generate_seeds_from_topics(
            [{"topic": "t1"}, {"topic": "t2"}, {"topic": "t3"}], use_parallel=True
        )
        assert {s["instruction"] for s in seeds} == {"q2", "q3"}


class TestGenerateReport:
    def test_report_contains_supported_strategies(self):
        expander = DomainExpander(ScriptedBackend([]))
        report = expander.generate_report([{"instruction": "租金怎么算"}])

        assert report["original_topic_count"] == 1
        assert report["original_topics"] == ["租金相关"]
        assert report["supported_strategies"] == ["similar", "related", "scenario"]
        assert len(report["recommendations"]) == 3
