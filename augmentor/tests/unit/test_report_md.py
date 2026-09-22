# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""report_md 报告 Markdown 渲染模块测试

覆盖 PII/泄漏/审计/通用报告渲染，含表格、空态、自定义标题与包级导出。
"""

import pytest

from augmentor import (
    audit_report_to_markdown,
    generic_report_to_markdown,
    leakage_report_to_markdown,
    sanitize_report_to_markdown,
)
from augmentor.audit import audit_dataset
from augmentor.leakage import detect_leakage
from augmentor.privacy import PiiSanitizer


class TestSanitizeMarkdown:
    def test_render_with_matches(self):
        items = [{"instruction": "a@b.com", "input": "", "output": "x"}]
        _, report = PiiSanitizer().sanitize_dataset(items)
        md = sanitize_report_to_markdown(report)
        assert "# PII 脱敏报告" in md
        assert "email" in md
        assert "| 类型 | 命中数 |" in md

    def test_render_empty(self):
        _, report = PiiSanitizer().sanitize_dataset([{"instruction": "干净", "output": ""}])
        md = sanitize_report_to_markdown(report, title="脱敏")
        assert "# 脱敏" in md
        assert "无" in md

    def test_custom_title(self):
        _, report = PiiSanitizer().sanitize_dataset([])
        assert "自定义" in sanitize_report_to_markdown(report, title="自定义")


class TestLeakageMarkdown:
    def test_render_with_leaks(self):
        train = [{"instruction": "如何申请？"}]
        test = [{"instruction": "如何申请？"}, {"instruction": "其他"}]
        report = detect_leakage(train, test)
        md = leakage_report_to_markdown(report)
        assert "泄漏检测" in md
        assert "精确泄漏: 1" in md
        assert "如何申请" in md

    def test_clean_report(self):
        report = detect_leakage([{"instruction": "a"}], [{"instruction": "b"}])
        md = leakage_report_to_markdown(report)
        assert "无泄漏" in md
        assert "## 示例" in md
        assert "无" in md


class TestAuditMarkdown:
    def test_render_dirty(self):
        items = [{"instruction": "a@b.com", "output": "x"},
                 {"instruction": "重复", "output": "y"},
                 {"instruction": "重复", "output": "z"}]
        md = audit_report_to_markdown(audit_dataset(items))
        assert "审计" in md
        assert "未就绪" in md
        assert "PII" in md

    def test_render_clean(self):
        items = [{"instruction": f"问题{i}", "output": f"答案{i}"} for i in range(3)]
        md = audit_report_to_markdown(audit_dataset(items))
        assert "就绪" in md


class TestGenericMarkdown:
    def test_scalars_and_collections(self):
        payload = {"a": 1, "b": "x", "list": [1, 2, 3], "dict": {"k": "v"}}
        md = generic_report_to_markdown(payload, title="G")
        assert "# G" in md
        assert "**a**: 1" in md
        assert "**list**: 3" in md      # 集合显示长度
        assert "**dict**: 1" in md

    def test_empty_payload(self):
        md = generic_report_to_markdown({})
        assert md.strip() != ""
        assert "#" in md


class TestExports:
    def test_package_exports(self):
        import augmentor

        assert callable(augmentor.sanitize_report_to_markdown)
        assert callable(augmentor.leakage_report_to_markdown)
        assert callable(augmentor.audit_report_to_markdown)
        assert callable(augmentor.generic_report_to_markdown)
