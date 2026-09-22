"""缓存统计功能验证

原实现是两处占位断言：

    assert hasattr(cache, '_stats') or True   # `or True` 使整式恒真
    assert True

前者断言的 `_stats` 是 `set()` 里惰性创建的私有字典，**写入后再无任何读取**
（`stats()` 用的是 `self._hits` / `self._misses`），所以这条断言既恒真又指向了错误的对象。
改为对公开的 `stats` property 做行为断言。
"""

from augmentor.cache import MemoryCache


class TestCacheStats:
    def test_stats_available_before_any_write(self):
        """未写入时 stats 也应可用且计数为零"""
        cache = MemoryCache(max_size=10)

        stats = cache.stats

        assert stats["hits"] == 0
        assert stats["misses"] == 0
        assert stats["size"] == 0
        assert stats["max_size"] == 10

    def test_stats_track_hit_and_miss(self):
        """命中与未命中必须分别计数——这是缓存效率监控的唯一依据"""
        cache = MemoryCache(max_size=10)
        cache.set("a", 1, ttl=60)

        assert cache.get("a") == 1            # 命中
        assert cache.get("missing") is None   # 未命中

        stats = cache.stats
        assert stats["hits"] == 1
        assert stats["misses"] == 1
        assert stats["size"] == 1
        assert stats["hit_rate"] == 0.5

    def test_expired_entry_counts_as_miss(self):
        """过期条目应先删除再计为未命中，不能同时算作命中"""
        cache = MemoryCache(max_size=10)
        cache.set("a", 1, ttl=-1)

        assert cache.get("a") is None

        stats = cache.stats
        assert stats["hits"] == 0
        assert stats["misses"] == 1
        assert stats["size"] == 0
