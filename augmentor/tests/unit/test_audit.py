# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""audit 数据集就绪审计模块测试

覆盖 PII/重复/空字段/泄漏四类信号、就绪判定、空数据集、
报告序列化与包级导出。
"""

import pytest

from augmentor import AuditReport, DatasetAuditor, audit_dataset


class TestCleanDataset:
    def test_ready_when_clean(self):
        items = [
            {"instruction": f"问题{i}？", "input": "", "output": f"回答{i}"}
            for i in range(5)
        ]
        report = audit_dataset(items)
        assert report.pii_matches == 0
        assert report.duplicate_count == 0
        assert report.ready is True

    def test_report_to_dict(self):
        report = audit_dataset([{"instruction": "q", "output": "a"}])
        d = report.to_dict()
        assert d["total_items"] == 1
        assert "findings" in d and "ready" in d


class TestPiiSignal:
    def test_pii_triggers_finding_and_not_ready(self):
        items = [
            {"instruction": "联系 zhang@example.com", "input": "", "output": "好"},
        ]
        report = DatasetAuditor().audit(items)
        assert report.pii_matches == 1
        assert report.ready is False
        assert any("PII" in f for f in report.findings)

    def test_pii_free_not_required_flag(self):
        items = [{"instruction": "邮箱 a@b.com", "input": "", "output": ""}]
        auditor = DatasetAuditor(pii_free_required=False)
        report = auditor.audit(items)
        assert report.pii_matches == 1
        # 不要求零 PII 时该项不影响 ready
        assert report.ready is True


class TestDuplicateSignal:
    def test_duplicates_reported(self):
        items = [
            {"instruction": "如何申请？", "input": "", "output": "A"},
            {"instruction": "如何申请？", "input": "", "output": "B"},
            {"instruction": "完全不同的另一个问题", "input": "", "output": "C"},
        ]
        report = audit_dataset(items)
        assert report.duplicate_count >= 1
        assert report.duplicate_rate > 0
        assert any("重复" in f for f in report.findings)


class TestEmptyFieldSignal:
    def test_high_empty_rate_finding(self):
        items = [
            {"instruction": "问题", "input": "", "output": ""},
            {"instruction": "", "input": "", "output": ""},
        ]
        report = DatasetAuditor(fields=["instruction", "output"]).audit(items)
        assert report.empty_field_rate > 0.5
        assert any("空字段" in f for f in report.findings)


class TestLeakageSignal:
    def test_leakage_vs_reference(self):
        train = [
            {"instruction": "如何申请？", "input": "", "output": "A"},
            {"instruction": "多少钱？", "input": "", "output": "B"},
        ]
        test_with_leak = [
            {"instruction": "如何申请？", "input": "", "output": "C"},
            {"instruction": "天气如何？", "input": "", "output": "D"},
        ]
        report = DatasetAuditor().audit(test_with_leak, reference=train)
        assert report.leak_count == 1
        assert report.leak_rate == pytest.approx(0.5)
        assert report.ready is False
        assert any("泄漏" in f for f in report.findings)

    def test_no_reference_leak_null(self):
        items = [{"instruction": "q", "input": "", "output": "a"}]
        report = audit_dataset(items)
        assert report.leak_count is None
        assert report.leak_rate is None


class TestEmptyDataset:
    def test_empty_not_ready(self):
        report = audit_dataset([])
        assert report.total_items == 0
        assert report.ready is False
        assert any("为空" in f for f in report.findings)


class TestPackageExports:
    def test_exports(self):
        import augmentor

        assert augmentor.DatasetAuditor is DatasetAuditor
        assert augmentor.AuditReport is AuditReport
        assert callable(augmentor.audit_dataset)
