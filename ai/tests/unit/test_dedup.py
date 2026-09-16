"""智能去重单元测试

去重直接影响训练数据规模，阈值校验、索引保留与相似对查询必须正确。
"""

import pytest

from augmentor.dedup import Deduplicator, DedupResult


class TestInit:
    """初始化校验"""

    def test_rejects_out_of_range_threshold(self):
        """阈值必须在 0-1 之间，否则相似度比较失去意义"""
        with pytest.raises(ValueError):
            Deduplicator(threshold=1.5)
        with pytest.raises(ValueError):
            Deduplicator(threshold=-0.1)


class TestDeduplicate:
    """去重主流程"""

    def test_removes_duplicates_and_keeps_first(self):
        """重复组应保留首个样本，保证结果稳定可复现"""
        dedup = Deduplicator(threshold=0.9)
        items = [
            {"instruction": "如何申请入住安居乐寓？"},
            {"instruction": "如何申请入住安居乐寓？"},
            {"instruction": "退租流程是什么？"},
        ]
        result = dedup.deduplicate(items)

        assert isinstance(result, DedupResult)
        assert result.original_count == 3
        assert result.removed_count >= 1
        assert 0 in result.kept_indices
        assert len(result.kept_indices) == result.deduplicated_count

    def test_empty_dataset(self):
        """空数据集应返回零值结果而非异常"""
        dedup = Deduplicator()
        result = dedup.deduplicate([])

        assert result.original_count == 0
        assert result.deduplicated_count == 0
        assert result.kept_indices == []

    def test_deduplicate_and_filter_returns_items(self):
        """过滤后的列表长度必须与 kept_indices 一致"""
        dedup = Deduplicator(threshold=0.9)
        items = [{"instruction": f"问题 {i}"} for i in range(5)]
        filtered = dedup.deduplicate_and_filter(items)

        assert len(filtered) <= len(items)
        assert all(isinstance(item, dict) for item in filtered)

    def test_custom_text_key(self):
        """支持自定义去重字段，适配不同数据结构"""
        dedup = Deduplicator(threshold=0.9)
        items = [{"question": "相同问题"}, {"question": "相同问题"}]
        result = dedup.deduplicate(items, text_key="question")

        assert result.original_count == 2
        assert result.deduplicated_count <= 2


class TestReport:
    """去重报告"""

    def test_report_fields(self):
        """报告需给出移除比例，供质量报告聚合"""
        dedup = Deduplicator(threshold=0.9)
        items = [{"instruction": "一样的问题"}, {"instruction": "一样的问题"}]
        report = dedup.generate_report(items)

        assert report["original_count"] == 2
        assert "removal_rate" in report
        assert 0.0 <= report["removal_rate"] <= 1.0
        assert report["threshold"] == 0.9

    def test_report_on_empty(self):
        """空数据集移除比例应为 0"""
        report = Deduplicator().generate_report([])
        assert report["removal_rate"] == 0


class TestFindSimilarPairs:
    """相似对查询（回归测试）

    该方法此前调用了不存在的 _compute_similarity_matrix，会在运行期抛
    AttributeError，导致 Web UI 的相似样本视图直接 500。
    """

    def test_returns_pairs_without_attribute_error(self):
        """必须能正常返回相似对，不能再抛 AttributeError"""
        dedup = Deduplicator(threshold=0.5)
        items = [
            {"instruction": "如何申请入住安居乐寓？"},
            {"instruction": "如何申请入住安居乐寓呢？"},
            {"instruction": "完全不相关的问题内容"},
        ]
        pairs = dedup.find_similar_pairs(items, top_k=5)

        assert isinstance(pairs, list)
        for first, second, similarity in pairs:
            assert isinstance(first, int) and isinstance(second, int)
            assert 0.0 <= similarity <= 1.0

    def test_returns_empty_for_single_item(self):
        """少于两条数据时无相似对可言"""
        dedup = Deduplicator()
        assert dedup.find_similar_pairs([{"instruction": "唯一"}]) == []
