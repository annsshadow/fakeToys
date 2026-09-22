# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集验证增强测试 - 批量验证报告和自定义规则"""

import pytest
from augmentor.validation import DatasetValidator, generate_validation_report, ValidationSeverity


@pytest.fixture
def valid_items():
    return [
        {"instruction": "如何租房？", "input": "", "output": "建议查看房源信息"},
        {"instruction": "租房合同注意什么？", "input": "", "output": "注意条款"},
    ]


@pytest.fixture
def invalid_items():
    return [
        {"instruction": ""},  # 缺失 output
        {"output": "回答"},  # 缺失 instruction
        {"instruction": "正常问题", "output": "正常回答"},
    ]


class TestValidationEnhancement:
    """验证增强功能测试"""
    
    def test_generate_validation_report_exists(self, valid_items):
        """批量验证报告应存在并返回完整结构"""
        report = generate_validation_report(valid_items)
        assert "summary" in report
        assert "issues_by_severity" in report
        assert "total_items" in report
        assert "suggestions" in report
    
    def test_report_pass_rate_calculated(self, valid_items, invalid_items):
        """报告应正确计算通过率"""
        valid_report = generate_validation_report(valid_items)
        invalid_report = generate_validation_report(invalid_items)
        
        assert valid_report["pass_rate"] >= invalid_report["pass_rate"]
    
    def test_custom_validation_preserves_severity(self, valid_items):
        """自定义验证应保留严重程度分类"""
        report = generate_validation_report(valid_items, preset="basic")
        # 验证结构完整性
        assert isinstance(report["issues_by_severity"], dict)
        assert isinstance(report["suggestions"], list)
        assert len(report["suggestions"]) > 0
