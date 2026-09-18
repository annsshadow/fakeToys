"""数据集索引模块

提供数据集的快速检索、查询和管理功能。
"""

import json
import logging
from typing import List, Dict, Optional, Any, Callable, Set
from dataclasses import dataclass, field
from pathlib import Path
from enum import Enum
from collections import defaultdict

logger = logging.getLogger(__name__)


class IndexType(Enum):
    """索引类型"""
    HASH = "hash"
    INVERTED = "inverted"
    ENHANCED = "enhanced"  # 增强索引类型（优化：支持混合索引策略）
    NGRAM = "ngram"


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
        
        if self._items:
            self._build_default_indexes()
    
    def load(self, items: List[Dict]):
        """加载数据
        
        Args:
            items: 数据列表
        """
        self._items = items
        self._indexes.clear()
        self._field_indexes.clear()
        self._ngram_indexes.clear()
        self._build_default_indexes()
    
    def _build_default_indexes(self):
        """构建默认索引"""
        # 为常用字段构建倒排索引
        for field_name in ["instruction", "output", "input"]:
            self._build_field_index(field_name)
        
        # 为instruction构建n-gram索引
        self._build_ngram_index("instruction", n=2)
    
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
                # 也添加小写版本
                index[value.lower()].append(idx)
        
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
            field: 字段名
            value: 搜索值
        
        Returns:
            匹配的索引列表
        """
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
            field: 字段名
            query: 查询字符串
            n: n-gram大小
            min_match: 最小匹配数
        
        Returns:
            匹配的索引列表
        """
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
        return DatasetView(self._items[:n], f"{self.name}_head")
    
    def tail(self, n: int = 10) -> 'DatasetView':
        """获取后N条数据
        
        Args:
            n: 数量
        
        Returns:
            新视图
        """
        return DatasetView(self._items[-n:] if len(self._items) >= n else self._items, 
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
        if seed is not None:
            random.seed(seed)
        sampled = random.sample(self._items, min(n, len(self._items)))
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
