"""数据分析功能增强测试 - 快速分析优化验证"""

import pytest
from augmentor.analytics import analyze_dataset_fast


@pytest.fixture
def sample_dataset():
    return [
        {"instruction": "如何租房？", "output": "建议查看房源信息"},
        {"instruction": "租房合同注意什么？", "output": "注意合同条款"},
        {"instruction": "租金如何支付？", "output": "按月支付租金"},
    ]


class TestAnalyticsEnhancement:
    """数据分析增强测试"""
    
    def test_fast_analysis_exists(self):
        """快速分析函数应存在"""
        assert callable(analyze_dataset_fast)
    
    def test_fast_analysis_returns_summary(self, sample_dataset):
        """快速分析应返回基础统计和关键词"""
        result = analyze_dataset_fast(sample_dataset, top_k=3)
        assert result["fast_analysis"] is True
        assert "top_words" in result
        assert "avg_instruction_length" in result
        assert result["sample_size"] == 3
    
    def test_fast_analysis_handles_empty_items(self):
        """快速分析应安全处理空数据"""
        result = analyze_dataset_fast([])
        assert result["fast_analysis"] is True
        assert result["sample_size"] == 0
        # 空数据没有字段检查，因此 has_empty_fields 可能为 False（无数据可检查）
        assert "has_empty_fields" in result
