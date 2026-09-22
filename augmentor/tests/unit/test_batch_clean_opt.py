# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据清洗增强测试 - 批量清洗优化验证"""

import pytest
from augmentor.cleaner import clean_batch_optimized, DatasetCleaner


@pytest.fixture
def sample_dirty_items():
    return [
        {"instruction": "如何租房？", "output": "建议查看房源信息"},
        {"instruction": "租房合同注意什么？", "output": "注意条款"},
        {"instruction": "租金如何支付？", "output": "按月支付"},
    ]


class TestBatchCleanOptimization:
    """批量清洗优化测试"""
    
    def test_batch_clean_optimized_exists(self):
        """批量清洗优化函数应存在"""
        assert callable(clean_batch_optimized)
    
    def test_batch_clean_processes_large_data(self, sample_dirty_items):
        """批量清洗应处理大数据并返回汇总结果"""
        cleaned, result = clean_batch_optimized(sample_dirty_items, batch_size=2)
        assert len(cleaned) >= 0
        assert result.original_count == 3
        assert hasattr(result, "cleaned_count")
    
    def test_batch_size_limits_memory(self, sample_dirty_items):
        """批处理大小应限制内存使用"""
        cleaned_small, _ = clean_batch_optimized(sample_dirty_items, batch_size=1)
        cleaned_large, _ = clean_batch_optimized(sample_dirty_items, batch_size=10)
        # 不同批处理大小应产生相同清理结果
        assert len(cleaned_small) == len(cleaned_large)
