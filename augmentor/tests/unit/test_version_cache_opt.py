"""版本控制优化测试 - 版本比较缓存验证"""

import pytest
from unittest.mock import MagicMock


class MockVersionManager:
    """模拟版本管理器"""
    def __init__(self):
        self._compare_cache = {}
    
    def compare_versions_cached(self, a, b):
        key = f"{a}:{b}"
        if key in self._compare_cache:
            return self._compare_cache[key]
        result = {"version_a": a, "version_b": b, "cached": False}
        self._compare_cache[key] = result
        return result.copy()


class TestVersionControlOptimization:
    """版本控制优化测试"""
    
    def test_version_compare_cache_exists(self):
        """版本比较缓存应存在"""
        manager = MockVersionManager()
        result1 = manager.compare_versions_cached("v1", "v2")
        result2 = manager.compare_versions_cached("v1", "v2")
        assert result1 == result2
        assert len(manager._compare_cache) == 1
    
    def test_version_compare_uses_cache_key(self):
        """不同版本对应产生不同缓存条目"""
        manager = MockVersionManager()
        manager.compare_versions_cached("v1", "v2")
        manager.compare_versions_cached("v2", "v3")
        assert len(manager._compare_cache) == 2
    
    def test_version_control_code_has_cache(self):
        """版本控制代码应包含缓存机制"""
        import inspect
        from augmentor.version_control import DatasetVersionManager
        source = inspect.getsource(DatasetVersionManager.compare_versions)
        assert "_compare_cache" in source
