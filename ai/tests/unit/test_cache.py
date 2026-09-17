"""数据集缓存模块测试"""

import time
import pytest
from augmentor.cache import (
    MemoryCache, DiskCache, CachedProcessor,
    cached, create_memory_cache, create_disk_cache
)


class TestMemoryCache:
    """MemoryCache 测试"""
    
    def test_init(self):
        """测试初始化"""
        cache = MemoryCache(max_size=100, default_ttl=60)
        assert cache._max_size == 100
        assert cache._default_ttl == 60
    
    def test_set_get(self):
        """测试设置和获取"""
        cache = MemoryCache()
        cache.set("key1", "value1")
        
        result = cache.get("key1")
        assert result == "value1"
    
    def test_get_nonexistent(self):
        """测试获取不存在的键"""
        cache = MemoryCache()
        result = cache.get("nonexistent")
        assert result is None
    
    def test_ttl_expiration(self):
        """测试TTL过期"""
        cache = MemoryCache()
        cache.set("key1", "value1", ttl=0.1)  # 0.1秒过期
        
        assert cache.get("key1") == "value1"
        time.sleep(0.2)
        assert cache.get("key1") is None
    
    def test_delete(self):
        """测试删除"""
        cache = MemoryCache()
        cache.set("key1", "value1")
        
        assert cache.delete("key1") is True
        assert cache.get("key1") is None
        assert cache.delete("nonexistent") is False
    
    def test_clear(self):
        """测试清空"""
        cache = MemoryCache()
        cache.set("key1", "value1")
        cache.set("key2", "value2")
        
        cache.clear()
        assert cache.get("key1") is None
        assert cache.get("key2") is None
    
    def test_eviction(self):
        """测试淘汰策略"""
        cache = MemoryCache(max_size=3)
        cache.set("key1", "value1")
        cache.set("key2", "value2")
        cache.set("key3", "value3")
        
        # 访问 key1 多次，增加 hit_count
        cache.get("key1")
        cache.get("key1")
        
        # 添加新条目，触发淘汰
        cache.set("key4", "value4")
        
        # key1 应该还在（hit_count高）
        assert cache.get("key1") == "value1"
    
    def test_stats(self):
        """测试统计信息"""
        cache = MemoryCache()
        cache.set("key1", "value1")
        cache.get("key1")
        cache.get("nonexistent")
        
        stats = cache.stats
        assert stats["size"] == 1
        assert stats["hits"] == 1
        assert stats["misses"] == 1


class TestDiskCache:
    """DiskCache 测试"""
    
    def test_init(self, tmp_path):
        """测试初始化"""
        cache_dir = tmp_path / "cache"
        cache = DiskCache(str(cache_dir))
        assert cache_dir.exists()
    
    def test_set_get(self, tmp_path):
        """测试设置和获取"""
        cache_dir = tmp_path / "cache"
        cache = DiskCache(str(cache_dir))
        
        cache.set("key1", {"data": "value1"})
        result = cache.get("key1")
        
        assert result == {"data": "value1"}
    
    def test_get_nonexistent(self, tmp_path):
        """测试获取不存在的键"""
        cache_dir = tmp_path / "cache"
        cache = DiskCache(str(cache_dir))
        
        result = cache.get("nonexistent")
        assert result is None
    
    def test_delete(self, tmp_path):
        """测试删除"""
        cache_dir = tmp_path / "cache"
        cache = DiskCache(str(cache_dir))
        
        cache.set("key1", "value1")
        assert cache.delete("key1") is True
        assert cache.get("key1") is None
    
    def test_clear(self, tmp_path):
        """测试清空"""
        cache_dir = tmp_path / "cache"
        cache = DiskCache(str(cache_dir))
        
        cache.set("key1", "value1")
        cache.set("key2", "value2")
        
        cache.clear()
        assert cache.get("key1") is None
        assert cache.get("key2") is None
    
    def test_stats(self, tmp_path):
        """测试统计信息"""
        cache_dir = tmp_path / "cache"
        cache = DiskCache(str(cache_dir))
        
        cache.set("key1", "value1")
        
        stats = cache.stats
        assert stats["entries"] == 1


class TestCachedProcessor:
    """CachedProcessor 测试"""
    
    def test_process(self):
        """测试处理"""
        def processor(x):
            return x * 2
        
        cached_proc = CachedProcessor(processor)
        
        result = cached_proc.process(5)
        assert result == 10
        
        # 第二次应该从缓存获取
        result2 = cached_proc.process(5)
        assert result2 == 10
    
    def test_process_without_cache(self):
        """测试不使用缓存"""
        def processor(x):
            return x * 2
        
        cached_proc = CachedProcessor(processor)
        
        result = cached_proc.process(5, use_cache=False)
        assert result == 10


class TestCachedDecorator:
    """缓存装饰器测试"""
    
    def test_cached_decorator(self):
        """测试缓存装饰器"""
        call_count = 0
        
        @cached(ttl=60)
        def expensive_function(x):
            nonlocal call_count
            call_count += 1
            return x * 2
        
        result1 = expensive_function(5)
        assert result1 == 10
        assert call_count == 1
        
        result2 = expensive_function(5)
        assert result2 == 10
        assert call_count == 1  # 应该从缓存获取
    
    def test_clear_cache(self):
        """测试清空缓存"""
        call_count = 0
        
        @cached(ttl=60)
        def expensive_function(x):
            nonlocal call_count
            call_count += 1
            return x * 2
        
        expensive_function(5)
        assert call_count == 1
        
        expensive_function.clear_cache()
        expensive_function(5)
        assert call_count == 2


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_create_memory_cache(self):
        """测试创建内存缓存"""
        cache = create_memory_cache(max_size=500)
        assert isinstance(cache, MemoryCache)
        assert cache._max_size == 500
    
    def test_create_disk_cache(self, tmp_path):
        """测试创建磁盘缓存"""
        cache_dir = tmp_path / "disk_cache"
        cache = create_disk_cache(str(cache_dir))
        assert isinstance(cache, DiskCache)
        assert cache_dir.exists()
