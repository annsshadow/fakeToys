# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第53轮: profiling内存分析测试"""
import pytest
from augmentor.profiling import estimate_memory, field_completeness


class TestEstimateMemory:
    def test_empty_list(self):
        result = estimate_memory([])
        assert result["total_items"] == 0
        assert result["estimated_bytes"] >= 0

    def test_single_item(self):
        items = [{"instruction": "hello", "output": "world"}]
        result = estimate_memory(items)
        assert result["total_items"] == 1
        assert result["avg_bytes_per_item"] > 0

    def test_multiple_items(self):
        items = [{"instruction": f"test_{i}", "output": f"out_{i}"} for i in range(10)]
        result = estimate_memory(items)
        assert result["total_items"] == 10
        assert result["estimated_mb"] >= 0


class TestFieldCompleteness:
    def test_all_filled(self):
        items = [{"a": "x", "b": "y"} for _ in range(5)]
        result = field_completeness(items)
        assert result["a"] == 1.0
        assert result["b"] == 1.0

    def test_partial_fill(self):
        items = [{"a": "x", "b": ""}, {"a": "", "b": "y"}]
        result = field_completeness(items)
        assert result["a"] == 0.5
        assert result["b"] == 0.5

    def test_empty_items(self):
        result = field_completeness([])
        assert result == {}

    def test_custom_fields(self):
        items = [{"a": "x", "b": "y", "c": ""}]
        result = field_completeness(items, fields=["a", "c"])
        assert "a" in result
        assert "c" in result
        assert "b" not in result
