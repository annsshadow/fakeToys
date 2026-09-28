# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L117（A205）：数据/质量辅助模块里「退化输入」触发的可达假支批量清零

这一批全是 `for ... : if 条件:` 或 `if 开关:` 的**假支**——正常样本走真支，只有当输入
退化（空文本、缺字段、重复命中、开关关闭、非字典项）时才走假支。它们是真实数据形态，
不是不可达的防御码，所以补会真失败的行为用例，而不是记档。逐条对应 L100 终态偏支：

- privacy.py 119->118        重复 PII 命中类型不重复入表
- quality_monitor.py 194->193 快照缺该指标 ⇒ 跳过
- quality_trend.py 93->90      历史条目缺该指标 ⇒ 跳过
- data/cleaner.py 97->99       remove_urls=False ⇒ 跳过 URL 清洗
- data/multimodal.py 96->99    空文本 ⇒ 不记 text 模态
- migration.py 124->123        对话项非字典 ⇒ 跳过
"""

import pytest


class TestPrivacyDuplicateHitType:
    def test_same_pattern_across_fields_recorded_once(self):
        from augmentor.privacy import PiiSanitizer

        sanitizer = PiiSanitizer(fields=["instruction", "output"])
        # 两个字段都含邮箱 ⇒ 第二个字段的 email 命中已在表里 ⇒ 假支（不重复追加）
        item = {"instruction": "联系 a@x.com", "output": "或 b@y.com"}
        _, hit_types = sanitizer.sanitize_item(item)
        assert hit_types.count("email") == 1


class TestQualityMonitorMissingMetric:
    def test_get_trend_skips_snapshot_without_metric(self):
        from augmentor.quality_monitor import QualityMonitor

        monitor = QualityMonitor()
        monitor.check_quality([{"instruction": "问", "input": "", "output": "答"}])
        # 快照里不会有这个瞎编的指标名 ⇒ 循环走假支跳过，返回空
        assert monitor.get_trend("l117_bogus_metric") == []


class TestQualityTrendMissingMetric:
    def test_get_trend_for_metric_skips_entry_without_metric(self):
        from augmentor.quality_trend import QualityTrendTracker

        tracker = QualityTrendTracker()
        tracker.record_quality_metrics("ds", {"pass_rate": 0.9})
        # 条目里没有这个指标 ⇒ 假支跳过
        assert tracker.get_trend_for_metric("l117_bogus_metric") == []


class TestDataCleanerUrlToggle:
    def test_remove_urls_false_keeps_url(self):
        from augmentor.data.cleaner import DataCleaner

        text = "看 http://example.com/x 这里"
        assert "http://example.com/x" in DataCleaner(remove_urls=False).clean_text(text)
        assert "http://example.com/x" not in DataCleaner(remove_urls=True).clean_text(text)


class TestMultimodalEmptyText:
    def test_empty_text_not_recorded_as_modality(self):
        from augmentor.data.multimodal import MultimodalProcessor

        record = MultimodalProcessor().fuse_modalities({"text": ""})
        assert "text" not in record.modalities


class TestMigrationNonDictConversationItem:
    def test_non_dict_items_are_skipped(self):
        from augmentor.migration import DatasetMigrator

        migrator = DatasetMigrator()
        # 对话列表里混入非字典项 ⇒ 假支跳过它，只展平字典项
        flattened = migrator._flatten_conversations(
            [{"from": "human", "value": "问"}, "不是字典", {"role": "gpt", "content": "答"}]
        )
        assert "human: 问" in flattened
        assert "gpt: 答" in flattened
        assert "不是字典" not in flattened
