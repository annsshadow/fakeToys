# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集索引模块

提供数据集的快速检索、查询和管理功能。
"""

import json
import logging
import threading
from typing import List, Dict, Optional, Any, Callable, Set, Tuple
from dataclasses import dataclass, field
from pathlib import Path
from enum import Enum
from collections import defaultdict
from .validation import require_count

logger = logging.getLogger(__name__)


class IndexType(Enum):
    """索引类型"""
    HASH = "hash"
    INVERTED = "inverted"
    ENHANCED = "enhanced"  # 增强索引类型（优化：支持混合索引策略）
    NGRAM = "ngram"


#: 默认清单：只决定 `list_indexes()` / `get_statistics()` 该报出哪几份索引，
#: **不限制能查哪些字段**——清单外的字段在第一次被查询时同样按需建索引。
DEFAULT_INDEX_FIELDS: Tuple[str, ...] = ("instruction", "output", "input")
#: 同上，默认 n-gram 索引的 `(字段, n)` 清单
DEFAULT_NGRAM_INDEXES: Tuple[Tuple[str, int], ...] = (("instruction", 2),)



@dataclass
class QueryResult:
    """查询结果"""
    items: List[Dict]
    total_matches: int
    query_time_ms: float
    index_used: str


@dataclass
class DatasetIndex:
    """数据集索引"""
    name: str
    index_type: IndexType
    field: str
    size: int
    created_at: str


class DatasetIndexer:
    """数据集索引器
    
    支持多种索引类型，提供快速检索能力。
    """
    
    def __init__(self, items: List[Dict] = None):
        """初始化索引器

        Args:
            items: 数据列表
        """
        self._items = items or []
        self._indexes: Dict[str, Any] = {}
        self._field_indexes: Dict[str, Dict[str, List[int]]] = {}
        self._ngram_indexes: Dict[str, Dict[str, Set[int]]] = {}
        # 索引**按需构建**。`DatasetView` 的每次 `filter/head/tail/sample/切片`
        # 都新建一个索引器，而旧实现在构造时无条件建齐 4 份索引（真实 6902 条实测
        # 70 ms、峰值 19 MB），可是这些操作连同 `search()` 的默认方法 contains
        # **一个都不读它们**，白建。待建清单按「哪一份还没建」精确记录，见
        # `_ensure_field_index` / `_ensure_ngram_index`。
        self._build_lock = threading.Lock()
        self._pending_fields: Set[str] = (
            set(DEFAULT_INDEX_FIELDS) if self._items else set())
        self._pending_ngrams: Set[Tuple[str, int]] = (
            set(DEFAULT_NGRAM_INDEXES) if self._items else set())

    def load(self, items: List[Dict]):
        """加载数据
        
        Args:
            items: 数据列表
        """
        with self._build_lock:
            self._items = items
            self._indexes.clear()
            self._field_indexes.clear()
            self._ngram_indexes.clear()
            # 换数据 = 作废索引：待建清单必须重新欠满，否则新数据会被旧索引
            # 回答成「查无此项」。这里连空数据集也欠着，是为了保住旧实现
            # 「`load([])` 之后 `list_indexes()` 报出 4 个空索引」的可见行为。
            self._pending_fields = set(DEFAULT_INDEX_FIELDS)
            self._pending_ngrams = set(DEFAULT_NGRAM_INDEXES)

    def _ensure_field_index(self, field: str) -> None:
        """要用到 `field` 的倒排索引时才建它

        默认清单（`DEFAULT_INDEX_FIELDS`）只决定「报告里该有哪几份」，**不限制
        能查哪些字段**：清单外的字段第一次被查询时同样按需建索引（L19 修 A23）。
        判据是「要么欠着默认清单的账，要么手上真有数据可建」——后者让空数据集
        不会凭空造出一份空索引，从而保住 `list_indexes()` 旧有的两种报告口径。
        """
        with self._build_lock:
            if field in self._field_indexes:
                return
            # 建完才销账：先销后建会让并发进来的第二个线程以为已就绪，
            # 却读不到 `_field_indexes[field]` 而把结果误报成「无匹配」。
            if field in self._pending_fields or self._items:
                self._build_field_index(field)
                self._pending_fields.discard(field)

    def _ensure_ngram_index(self, field: str, n: int) -> None:
        """要用到 `(field, n)` 这份 n-gram 索引时才建它（同 `_ensure_field_index`，
        字段与 `n` 都不限默认清单：L19 修 A23）"""
        spec = (field, n)
        key = f"{field}_{n}"
        with self._build_lock:
            if key in self._ngram_indexes:
                return
            if spec in self._pending_ngrams or self._items:
                self._build_ngram_index(field, n)
                self._pending_ngrams.discard(spec)

    def _ensure_default_indexes(self) -> None:
        """把默认清单里的索引全部建齐（`list_indexes` / `get_statistics` 用）"""
        for field_name in DEFAULT_INDEX_FIELDS:
            self._ensure_field_index(field_name)
        for field_name, n in DEFAULT_NGRAM_INDEXES:
            self._ensure_ngram_index(field_name, n)

    def _build_field_index(self, field: str):
        """构建字段倒排索引
        
        Args:
            field: 字段名
        """
        index: Dict[str, List[int]] = defaultdict(list)
        
        for idx, item in enumerate(self._items):
            value = item.get(field, "")
            if isinstance(value, str):
                # 使用精确值作为key
                index[value].append(idx)
                # 也添加小写版本（去重：值本身已全小写时跳过，避免同一索引被记录两次）
                lowered = value.lower()
                if lowered != value:
                    index[lowered].append(idx)
        
        self._field_indexes[field] = dict(index)
    
    def _build_ngram_index(self, field: str, n: int = 2):
        """构建n-gram索引
        
        Args:
            field: 字段名
            n: n-gram大小
        """
        index: Dict[str, Set[int]] = defaultdict(set)
        
        for idx, item in enumerate(self._items):
            value = item.get(field, "")
            if isinstance(value, str):
                # 生成n-grams
                for i in range(len(value) - n + 1):
                    ngram = value[i:i+n].lower()
                    index[ngram].add(idx)
        
        self._ngram_indexes[f"{field}_{n}"] = dict(index)
    
    def search_exact(self, field: str, value: str) -> List[int]:
        """精确搜索

        Args:
            field: 字段名，任意字段都可查（不局限于默认清单）。
                该字段第一次被查询时才会为其建一份倒排索引，
                也就是那一次调用要付一遍扫全表的代价，之后同字段查询直接吃索引。
            value: 搜索值

        Returns:
            匹配的索引列表
        """
        self._ensure_field_index(field)

        if field not in self._field_indexes:
            return []
        
        return self._field_indexes[field].get(value, []) or \
               self._field_indexes[field].get(value.lower(), [])
    
    def search_contains(self, field: str, query: str) -> List[int]:
        """包含搜索
        
        Args:
            field: 字段名
            query: 查询字符串
        
        Returns:
            匹配的索引列表
        """
        results = set()
        query_lower = query.lower()
        
        for idx, item in enumerate(self._items):
            value = item.get(field, "")
            if isinstance(value, str) and query_lower in value.lower():
                results.add(idx)
        
        return list(results)
    
    def search_ngram(self, field: str, query: str, n: int = 2, min_match: int = 1) -> List[int]:
        """n-gram搜索

        Args:
            field: 字段名，任意字段都可查（不局限于默认清单）
            query: 查询字符串
            n: n-gram大小，任意值都可（每个 (字段, n) 组合各是一份独立索引，
                第一次用到时才建，那一次调用付一遍扫全表的代价）
            min_match: 最小匹配数

        Returns:
            匹配的索引列表
        """
        self._ensure_ngram_index(field, n)

        index_key = f"{field}_{n}"
        if index_key not in self._ngram_indexes:
            return []
        
        index = self._ngram_indexes[index_key]
        match_counts: Dict[int, int] = defaultdict(int)
        
        # 生成查询的n-grams
        for i in range(len(query) - n + 1):
            ngram = query[i:i+n].lower()
            if ngram in index:
                for idx in index[ngram]:
                    match_counts[idx] += 1
        
        # 过滤满足最小匹配数的结果
        return [idx for idx, count in match_counts.items() if count >= min_match]
    
    def search(self, 
              query: str, 
              fields: List[str] = None,
              method: str = "contains") -> QueryResult:
        """通用搜索
        
        Args:
            query: 查询字符串
            fields: 搜索字段列表
            method: 搜索方法 (exact/contains/ngram)
        
        Returns:
            查询结果
        """
        import time
        start_time = time.time()
        
        fields = fields or ["instruction", "output"]
        all_matches: Dict[int, float] = defaultdict(float)
        
        for field in fields:
            if method == "exact":
                matches = self.search_exact(field, query)
            elif method == "ngram":
                matches = self.search_ngram(field, query)
            else:
                matches = self.search_contains(field, query)
            
            for idx in matches:
                all_matches[idx] += 1.0
        
        # 按匹配度排序
        sorted_indices = sorted(all_matches.keys(), key=lambda x: all_matches[x], reverse=True)
        
        query_time = (time.time() - start_time) * 1000
        
        return QueryResult(
            items=[self._items[idx] for idx in sorted_indices],
            total_matches=len(sorted_indices),
            query_time_ms=query_time,
            index_used=f"{method}:{','.join(fields)}"
        )
    
    def get_item(self, index: int) -> Optional[Dict]:
        """获取指定索引的数据
        
        Args:
            index: 索引
        
        Returns:
            数据项
        """
        if 0 <= index < len(self._items):
            return self._items[index]
        return None
    
    def get_batch(self, indices: List[int]) -> List[Dict]:
        """获取批量数据
        
        Args:
            indices: 索引列表
        
        Returns:
            数据列表
        """
        return [self._items[idx] for idx in indices if 0 <= idx < len(self._items)]
    
    def filter(self, 
              predicate: Callable[[Dict], bool]) -> List[Dict]:
        """过滤数据
        
        Args:
            predicate: 过滤条件
        
        Returns:
            过滤后的数据列表
        """
        return [item for item in self._items if predicate(item)]
    
    def get_statistics(self) -> Dict:
        """获取统计信息
        
        Returns:
            统计信息
        """
        # 这份返回值里带着「建了哪些索引」的键，所以要先把默认清单建齐——
        # 不然按需构建会让统计口径凭空少报几份索引。
        self._ensure_default_indexes()

        field_stats = {}
        for field in ["instruction", "output", "input"]:
            values = [item.get(field, "") for item in self._items if field in item]
            lengths = [len(v) for v in values if isinstance(v, str)]
            
            if lengths:
                field_stats[field] = {
                    "count": len(values),
                    "avg_length": sum(lengths) / len(lengths),
                    "min_length": min(lengths),
                    "max_length": max(lengths),
                    "unique_values": len(set(values))
                }
        
        return {
            "total_items": len(self._items),
            "indexes": list(self._field_indexes.keys()),
            "ngram_indexes": list(self._ngram_indexes.keys()),
            "field_statistics": field_stats
        }
    
    def list_indexes(self) -> List[DatasetIndex]:
        """列出所有索引
        
        Returns:
            索引列表
        """
        from datetime import datetime

        # 同 `get_statistics()`：列出来的必须是「默认清单欠的账都还清」后的状态
        self._ensure_default_indexes()

        indexes = []
        
        for field, index in self._field_indexes.items():
            indexes.append(DatasetIndex(
                name=f"field_{field}",
                index_type=IndexType.INVERTED,
                field=field,
                size=len(index),
                created_at=datetime.now().isoformat()
            ))
        
        for key, index in self._ngram_indexes.items():
            field, n = key.rsplit("_", 1)
            indexes.append(DatasetIndex(
                name=f"ngram_{field}_{n}",
                index_type=IndexType.NGRAM,
                field=field,
                size=len(index),
                created_at=datetime.now().isoformat()
            ))
        
        return indexes


class DatasetView:
    """数据集视图
    
    提供数据集的切片和筛选视图。
    """
    
    def __init__(self, items: List[Dict], name: str = "default"):
        """初始化视图
        
        Args:
            items: 数据列表
            name: 视图名称
        """
        self._items = items
        self.name = name
        self._indexer = DatasetIndexer(items)
    
    def __len__(self) -> int:
        return len(self._items)
    
    def __getitem__(self, index) -> Any:
        if isinstance(index, slice):
            return DatasetView(self._items[index], f"{self.name}_slice")
        return self._items[index]
    
    def __iter__(self):
        return iter(self._items)
    
    @property
    def items(self) -> List[Dict]:
        """获取数据列表"""
        return self._items
    
    def head(self, n: int = 10) -> 'DatasetView':
        """获取前N条数据
        
        Args:
            n: 数量
        
        Returns:
            新视图
        """
        require_count("n", n)
        return DatasetView(self._items[:n], f"{self.name}_head")
    
    def tail(self, n: int = 10) -> 'DatasetView':
        """获取后N条数据
        
        Args:
            n: 数量
        
        Returns:
            新视图
        """
        require_count("n", n)
        # `n=0` 要的是空视图：`[-0:]` 就是 `[0:]`，旧口径下这里返回过全部条目（实测 100 条）
        return DatasetView(self._items[-n:] if n else [],
                          f"{self.name}_tail")
    
    def sample(self, n: int = 10, seed: int = None) -> 'DatasetView':
        """随机采样
        
        Args:
            n: 采样数量
            seed: 随机种子
        
        Returns:
            新视图
        """
        import random
        # 局部 Random：不改动进程级 RNG 状态（seed=None 时仍取系统熵）
        require_count("n", n)
        sampled = random.Random(seed).sample(self._items, min(n, len(self._items)))
        return DatasetView(sampled, f"{self.name}_sample")
    
    def filter(self, predicate: Callable[[Dict], bool]) -> 'DatasetView':
        """过滤数据
        
        Args:
            predicate: 过滤条件
        
        Returns:
            新视图
        """
        return DatasetView([item for item in self._items if predicate(item)], 
                          f"{self.name}_filtered")
    
    def search(self, query: str, **kwargs) -> QueryResult:
        """搜索数据
        
        Args:
            query: 查询字符串
        
        Returns:
            查询结果
        """
        return self._indexer.search(query, **kwargs)
    
    def to_list(self) -> List[Dict]:
        """转换为列表
        
        Returns:
            数据列表
        """
        return self._items.copy()
    
    def to_file(self, path: str, format: str = "json"):
        """保存到文件
        
        Args:
            path: 文件路径
            format: 文件格式
        """
        output = Path(path)
        output.parent.mkdir(parents=True, exist_ok=True)
        
        with open(output, 'w', encoding='utf-8') as f:
            if format == "jsonl":
                for item in self._items:
                    f.write(json.dumps(item, ensure_ascii=False) + '\n')
            else:
                json.dump(self._items, f, ensure_ascii=False, indent=2)


def create_indexer(items: List[Dict]) -> DatasetIndexer:
    """创建索引器
    
    Args:
        items: 数据列表
    
    Returns:
        索引器实例
    """
    return DatasetIndexer(items)


def create_view(items: List[Dict], name: str = "default") -> DatasetView:
    """创建视图
    
    Args:
        items: 数据列表
        name: 视图名称
    
    Returns:
        视图实例
    """
    return DatasetView(items, name)