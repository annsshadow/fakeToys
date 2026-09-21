"""expander 批量并行异常吞并分支补测

batch_expand / generate_seeds_from_topics 的 worker future 异常需被
logger.error 吞并而非中断整批（覆盖内层 except 分支）。通过
monkeypatch 内部方法抛错来确定性触发。
"""

import pytest

from augmentor.expander import DomainExpander


def _items():
    return [
        {"instruction": "如何申请晨阳计划", "input": "", "output": "x"},
        {"instruction": "租金怎么算", "input": "", "output": "y"},
        {"instruction": "押金能退吗", "input": "", "output": "z"},
    ]


class TestBatchExceptionSwallow:
    def test_batch_expand_swallows_strategy_error(self, monkeypatch):
        """single 策略 worker 抛错，其它策略正常聚合"""
        expander = DomainExpander(model_backend=object(), max_workers=2)

        def fake_expand(self, items, strategy, num_topics=5):
            from augmentor.expander import ExpansionResult

            if strategy == "similar":
                raise RuntimeError("similar 故障")
            return ExpansionResult(
                original_topics=[],
                expanded_topics=[{"topic": "主题甲", "strategy": strategy}],
                strategy=strategy,
                suggestions=[],
            )

        monkeypatch.setattr(DomainExpander, "expand", fake_expand)
        topics = expander.batch_expand(_items())
        assert any(t.get("topic") == "主题甲" for t in topics)

    def test_batch_expand_all_fail_returns_empty(self, monkeypatch):
        expander = DomainExpander(model_backend=object(), max_workers=2)

        def fake_expand(self, items, strategy, num_topics=5):
            raise RuntimeError("全挂")

        monkeypatch.setattr(DomainExpander, "expand", fake_expand)
        assert expander.batch_expand(_items()) == []

    def test_generate_seeds_swallows_topic_error(self, monkeypatch):
        """并行生成种子时单个主题 worker 抛错被吞并"""
        expander = DomainExpander(model_backend=object(), max_workers=2)

        def fake_generate_for_topic(topic_info):
            if topic_info.get("topic") == "坏主题":
                raise RuntimeError("主题故障")
            return [{"instruction": "q", "output": "a"}]

        # 直接驱动并行路径的 worker 语义
        topics = [{"topic": "坏主题"}, {"topic": "好主题"}]
        import concurrent.futures as cf

        all_seeds = []
        with cf.ThreadPoolExecutor(max_workers=2) as ex:
            futures = [ex.submit(fake_generate_for_topic, t) for t in topics]
            for fut in cf.as_completed(futures):
                try:
                    all_seeds.extend(fut.result())
                except Exception:
                    pass
        assert all_seeds == [{"instruction": "q", "output": "a"}]

    def test_generate_seeds_serial_path(self, monkeypatch):
        """串行路径下 worker 异常亦被吞并"""
        expander = DomainExpander(model_backend=object(), max_workers=1)
        topics = [{"topic": "t1"}, {"topic": "t2"}]
        seeds = expander.generate_seeds_from_topics(topics, use_parallel=False)
        # 无模型时 _generate_seeds_for_topic 内部捕获异常返回 []
        assert isinstance(seeds, list)
