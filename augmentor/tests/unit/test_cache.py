# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

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


class TestCacheEntry:
    """CacheEntry 测试"""

    def test_is_expired_none_ttl(self):
        """ttl 为 None 时不应过期"""
        from augmentor.cache import CacheEntry
        entry = CacheEntry(key="k", value="v", created_at=time.time(), ttl=None)
        assert entry.is_expired is False

    def test_is_expired_not_yet(self):
        """未过期条目应返回 False"""
        from augmentor.cache import CacheEntry
        entry = CacheEntry(key="k", value="v", created_at=time.time(), ttl=3600)
        assert entry.is_expired is False

    def test_is_expired_already(self):
        """已过期条目应返回 True"""
        from augmentor.cache import CacheEntry
        entry = CacheEntry(key="k", value="v", created_at=time.time() - 100, ttl=1)
        assert entry.is_expired is True

    def test_to_dict(self):
        """to_dict 应返回正确结构"""
        from augmentor.cache import CacheEntry
        entry = CacheEntry(key="k", value="v", created_at=1000.0, ttl=60, hit_count=5)
        d = entry.to_dict()
        assert d["key"] == "k"
        assert d["created_at"] == 1000.0
        assert d["ttl"] == 60
        assert d["hit_count"] == 5
        assert "is_expired" in d


class TestDiskCacheExtended:
    """DiskCache 扩展测试"""

    def test_ttl_expiration(self, tmp_path):
        """磁盘缓存 TTL 过期应删除条目"""
        cache_dir = tmp_path / "cache"
        cache = DiskCache(str(cache_dir), default_ttl=0.1)
        cache.set("key1", "value1")
        assert cache.get("key1") == "value1"
        time.sleep(0.2)
        assert cache.get("key1") is None

    def test_get_cache_path(self, tmp_path):
        """缓存路径应使用 MD5 哈希"""
        cache_dir = tmp_path / "cache"
        cache = DiskCache(str(cache_dir))
        path = cache._get_cache_path("test_key")
        assert path.suffix == ".json"
        assert len(path.stem) == 32  # MD5 hex length

    def test_load_metadata(self, tmp_path):
        """加载元数据应从文件读取"""
        cache_dir = tmp_path / "cache"
        cache = DiskCache(str(cache_dir))
        cache.set("key1", "value1", ttl=60)
        
        # 重新加载
        meta = cache._load_metadata()
        assert "key1" in meta
        assert "created_at" in meta["key1"]

    def test_save_metadata(self, tmp_path):
        """保存元数据应写入文件"""
        cache_dir = tmp_path / "cache"
        cache = DiskCache(str(cache_dir))
        cache._metadata["test"] = {"value": 123}
        cache._save_metadata()
        
        # 验证文件存在
        assert cache._metadata_file.exists()

    def test_delete_nonexistent(self, tmp_path):
        """删除不存在的键应正常返回"""
        cache_dir = tmp_path / "cache"
        cache = DiskCache(str(cache_dir))
        assert cache.delete("nonexistent") is True

    def test_stats_with_entries(self, tmp_path):
        """统计信息应包含条目数和总大小"""
        cache_dir = tmp_path / "cache"
        cache = DiskCache(str(cache_dir))
        cache.set("key1", "value1")
        cache.set("key2", {"nested": "data"})
        
        stats = cache.stats
        assert stats["entries"] == 2
        assert stats["total_size_bytes"] > 0


class TestMemoryCacheExtended:
    """MemoryCache 扩展测试"""

    def test_stats_no_requests(self):
        """无请求时命中率应为 0"""
        cache = MemoryCache()
        stats = cache.stats
        assert stats["hit_rate"] == 0

    def test_evict_expired_entries(self):
        """淘汰时应先删除过期条目"""
        cache = MemoryCache(max_size=2)
        cache.set("expired", "val", ttl=0.01)
        time.sleep(0.05)
        cache.set("key2", "value2")
        
        # expired 应被删除
        assert cache.get("expired") is None

    def test_default_ttl(self):
        """默认 TTL 应被使用"""
        cache = MemoryCache(default_ttl=0.1)
        cache.set("key1", "value1")
        assert cache.get("key1") == "value1"
        time.sleep(0.2)
        assert cache.get("key1") is None

    def test_hit_count_increments(self):
        """访问应增加 hit_count"""
        cache = MemoryCache()
        cache.set("key1", "value1")
        cache.get("key1")
        cache.get("key1")
        
        assert cache._cache["key1"].hit_count == 2

    def test_clear_memory_cache(self):
        """清空内存缓存"""
        cache = MemoryCache()
        cache.set("key1", "value1")
        cache.clear()
        assert cache.get("key1") is None
