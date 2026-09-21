"""aggregator / cache / compare_enhanced 剩余分支补测

aggregator：空数据集 union、全零权重 weighted 归一化；
cache：MemoryCache 过期清理、DiskCache 损坏 JSON 返回 None、
`cached` 无 cache_dir 走内存缓存；
compare_enhanced：size_diff 正/负建议、字段类型不匹配建议。
"""

import json
import time

from augmentor.aggregator import DataAggregator
from augmentor.cache import DiskCache, MemoryCache, cached
from augmentor.compare_enhanced import compare_datasets_enhanced


class TestAggregatorEdges:
    def test_union_empty_datasets(self):
        result = DataAggregator().aggregate({}, "union")
        assert result.aggregated == []
        assert result.source_counts == {}

    def test_weighted_all_zero_weights_normalized(self):
        """所有源权重为 0 时 total_weight 归一化为 1.0 而非除零崩溃"""
        datasets = {
            "a": [{"instruction": "1"}, {"instruction": "2"}],
            "b": [{"instruction": "3"}],
        }
        # 权重全 0 → 每源 quota 为 0，结果为空但不得抛 ZeroDivisionError
        result = DataAggregator().aggregate(
            datasets, "weighted", weights={"a": 0.0, "b": 0.0}, target_size=10
        )
        assert len(result.aggregated) <= 10
        assert result.source_counts == {"a": 2, "b": 1}


class TestCacheEdges:
    def test_memory_cache_evicts_expired(self):
        cache = MemoryCache(max_size=100)
        cache.set("k1", "v1", ttl=0.01)
        cache.set("k2", "v2", ttl=60)
        time.sleep(0.03)
        # 触发淘汰：清理过期 + 满则删最少用
        cache._evict()
        assert cache.get("k1") is None
        assert cache.get("k2") == "v2"

    def test_disk_cache_corrupt_json_returns_none(self, tmp_path):
        cache = DiskCache(str(tmp_path))
        # 手动写入一个损坏的缓存文件
        cache_path = cache._get_cache_path("broken")
        cache_path.write_text("{ not json", encoding="utf-8")
        assert cache.get("broken") is None

    def test_cached_decorator_without_dir_uses_memory(self):
        calls = {"n": 0}

        @cached(ttl=60)
        def double(x):
            calls["n"] += 1
            return x * 2

        assert double(3) == 6
        assert double(3) == 6
        assert calls["n"] == 1  # 第二次命中内存缓存
        # 装饰器挂在 MemoryCache 上
        assert isinstance(double.cache, MemoryCache)


class TestCompareEnhancedRecommendations:
    def test_size_diff_large_positive(self):
        items_a = [{"instruction": f"q{i}"} for i in range(105)]
        items_b = [{"instruction": f"q{i}"} for i in range(0)]
        result = compare_datasets_enhanced(items_a, items_b)
        joined = " ".join(result.recommendations)
        assert "数据集A比B多" in joined

    def test_size_diff_large_negative(self):
        items_a = [{"instruction": f"q{i}"} for i in range(0)]
        items_b = [{"instruction": f"q{i}"} for i in range(105)]
        result = compare_datasets_enhanced(items_a, items_b)
        joined = " ".join(result.recommendations)
        assert "数据集B比A多" in joined

    def test_type_mismatch_recommendation(self):
        items_a = [{"instruction": "q", "count": 5}]
        items_b = [{"instruction": "q", "count": "5"}]  # int vs str
        result = compare_datasets_enhanced(items_a, items_b)
        joined = " ".join(result.recommendations)
        assert "类型不匹配" in joined or "值差异" in joined
