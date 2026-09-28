# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L118（A205）：类型/规则/字段分支里「非常规值」触发的可达假支批量清零

逐条对应 L100 终态偏支：
- feature_detect.py 130->122   字段值既非 bool/数值/字符串（如列表）⇒ 三档 elif 全不中
- cleaner.py 153->152          规则名不在内置表 ⇒ 跳过
- compare_enhanced.py 251->250 item_a 缺该字段 ⇒ 跳过
- audit.py 131->137            参考集无泄漏（leak_count==0）⇒ 不追加 finding
- indexer.py 147->145          字段值非字符串 ⇒ 不入倒排索引
- indexer.py 152->145          值本身已全小写 ⇒ 不重复记小写键
"""


class TestFeatureDetectNonScalarValue:
    def test_list_value_matches_no_type_branch(self):
        from augmentor.feature_detect import FeatureDetector

        # 列表值：既不是 bool，也不是 int/float，也不是 str ⇒ 三档 elif 全落空（假支）
        t = FeatureDetector._infer_type([{"f": [1, 2, 3]}], "f")
        # 没有任何已知类型被记录 ⇒ 归为 unknown
        assert t == "unknown"


class TestCleanerUnknownRule:
    def test_unknown_rule_is_skipped(self):
        from augmentor.cleaner import DatasetCleaner

        items = [{"instruction": "  空白  ", "input": "", "output": "答"}]
        cleaner = DatasetCleaner()
        cleaned, result = cleaner.clean(items, rules=["l118_unknown_rule"])
        # 未知规则被跳过 ⇒ 不报错、数据条数不变
        assert len(cleaned) == 1


class TestCompareMissingField:
    def test_item_missing_field_is_skipped(self):
        from augmentor.compare_enhanced import EnhancedComparator

        # A 的第二条缺 instruction 字段 ⇒ 值比较循环走假支跳过它
        items_a = [{"instruction": "问1", "output": "答1"}, {"output": "答2"}]
        items_b = [{"instruction": "问1", "output": "答1"}]
        result = EnhancedComparator().compare(items_a, items_b)
        assert result is not None


class TestAuditNoLeak:
    def test_clean_reference_adds_no_leak_finding(self):
        from augmentor.audit import DatasetAuditor

        items = [{"instruction": "问A", "input": "", "output": "答A"}]
        reference = [{"instruction": "完全不同的问", "input": "", "output": "答"}]
        report = DatasetAuditor().audit(items, reference=reference)
        # 无泄漏 ⇒ leak_count 为 0，不追加「与参考集存在…泄漏」这条 finding（假支）
        assert report.leak_count == 0
        assert not any("与参考集存在" in f for f in report.findings)


class TestIndexerNonStringAndLowercase:
    def test_non_string_value_not_indexed(self):
        from augmentor.indexer import DatasetIndexer

        indexer = DatasetIndexer([{"score": 42}])
        indexer._ensure_field_index("score")
        # 非字符串值不入索引 ⇒ 该字段索引为空（假支 147->145）
        assert indexer._field_indexes["score"] == {}

    def test_already_lowercase_value_not_double_indexed(self):
        from augmentor.indexer import DatasetIndexer

        indexer = DatasetIndexer([{"tag": "abc"}])
        indexer._ensure_field_index("tag")
        idx = indexer._field_indexes["tag"]
        # 值本身已全小写 ⇒ 只有一个键 "abc"，不额外记一份小写键（假支 152->145）
        assert list(idx.keys()) == ["abc"]
        assert idx["abc"] == [0]
