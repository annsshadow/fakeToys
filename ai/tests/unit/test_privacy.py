"""privacy PII 脱敏模块测试

覆盖邮箱/手机号/身份证/IP 命中、长串优先匹配、占位符可定制、
字段级/数据集级脱敏、报告统计、额外模式开关与包级导出。
"""

import re

import pytest

from augmentor import PiiSanitizer, SanitizeReport, sanitize_pii
from augmentor.privacy import DEFAULT_PATTERNS, EXTRA_PATTERNS


@pytest.fixture
def items():
    return [
        {"instruction": "联系 zhang@example.com 或 13800138000", "input": "", "output": "可拨打"},
        {"instruction": "身份证 11010119900307123X 办理", "input": "", "output": "好的"},
        {"instruction": "服务器 192.168.1.10 宕机", "input": "", "output": "重启"},
        {"instruction": "干净的问题", "input": "", "output": "干净的回答"},
    ]


class TestSanitizeText:
    def test_email_masked(self):
        text, hits = PiiSanitizer().sanitize_text("联系 a@b.com 谢谢")
        assert "a@b.com" not in text
        assert "[EMAIL]" in text
        assert "email" in hits

    def test_phone_masked(self):
        text, hits = PiiSanitizer().sanitize_text("电话 13800138000")
        assert "13800138000" not in text
        assert "phone" in hits

    def test_id_card_masked_not_phone(self):
        """18 位身份证应先按身份证掩码，手机号不抢子串"""
        text, hits = PiiSanitizer().sanitize_text("证号 11010119900307123X")
        assert "11010119900307123X" not in text
        assert "id_card" in hits
        assert "phone" not in hits

    def test_ip_masked(self):
        text, hits = PiiSanitizer().sanitize_text("IP 192.168.1.10 挂了")
        assert "192.168.1.10" not in text
        assert "ip" in hits

    def test_clean_text_untouched(self):
        text, hits = PiiSanitizer().sanitize_text("这是一段干净的话")
        assert text == "这是一段干净的话"
        assert hits == []

    def test_empty_and_non_str(self):
        assert PiiSanitizer().sanitize_text("") == ("", [])
        result, hits = PiiSanitizer().sanitize_text(None)
        assert result is None and hits == []


class TestSanitizeItemAndDataset:
    def test_item_sanitized_returns_new_dict(self, items):
        sanitizer = PiiSanitizer()
        original = dict(items[0])
        new_item, hits = sanitizer.sanitize_item(items[0])
        assert "zhang@example.com" not in new_item["instruction"]
        # 原对象不被修改
        assert items[0] == original
        assert "email" in hits and "phone" in hits

    def test_dataset_report_counts(self, items):
        dataset, report = PiiSanitizer().sanitize_dataset(items)
        assert isinstance(report, SanitizeReport)
        assert report.total_items == 4
        # 前 3 条命中，第 4 条干净
        assert report.touched_items == 3
        assert report.matches.get("email") == 1
        assert report.matches.get("id_card") == 1
        assert report.matches.get("ip") == 1
        assert report.total_matches == 4
        # 脱敏后数据不可回搜到原始敏感值
        joined = repr(dataset)
        assert "zhang@example.com" not in joined
        assert "11010119900307123X" not in joined

    def test_report_to_dict(self, items):
        _, report = PiiSanitizer().sanitize_dataset(items)
        d = report.to_dict()
        assert d["total_items"] == 4
        assert "matches" in d and "total_matches" in d

    def test_custom_fields(self):
        sanitizer = PiiSanitizer(fields=["question", "answer"])
        item, _ = sanitizer.sanitize_item({"question": "邮箱 a@b.com", "answer": "电话 13800138000"})
        assert "[EMAIL]" in item["question"]
        assert "[PHONE]" in item["answer"]

    def test_custom_placeholders(self):
        sanitizer = PiiSanitizer(placeholders={"email": "<E>"})
        text, _ = sanitizer.sanitize_text("a@b.com")
        assert "<E>" in text


class TestExtraPatterns:
    def test_credit_card_only_when_extra_enabled(self, items):
        card_text = {"instruction": "卡号 4111 1111 1111 1111", "input": "", "output": ""}
        _, report_default = PiiSanitizer().sanitize_dataset([card_text])
        assert "credit_card" not in report_default.matches

        results, report = sanitize_pii([card_text], include_extra=True)
        assert "credit_card" in report.matches
        assert "4111" not in results[0]["instruction"].replace("[CREDIT", "")

    def test_extra_patterns_defined(self):
        assert "credit_card" in EXTRA_PATTERNS
        assert "url" in EXTRA_PATTERNS


class TestModuleConvenience:
    def test_sanitize_pii_function(self, items):
        results, report = sanitize_pii(items)
        assert len(results) == len(items)
        assert report.touched_items == 3

    def test_package_exports(self):
        import augmentor

        assert augmentor.PiiSanitizer is PiiSanitizer
        assert augmentor.SanitizeReport is SanitizeReport
        assert callable(augmentor.sanitize_pii)

    def test_default_patterns_order_id_before_phone(self):
        """模式按长串优先：身份证先于手机号匹配"""
        keys = list(DEFAULT_PATTERNS.keys())
        assert keys.index("id_card") < keys.index("phone")
