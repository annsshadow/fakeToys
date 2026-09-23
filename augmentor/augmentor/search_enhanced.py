# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集搜索增强模块

提供更强大的数据集搜索功能。
"""

import re
import logging
from typing import List, Dict, Optional, Any, Callable, Set
from dataclasses import dataclass, field
from collections import defaultdict
import time

logger = logging.getLogger(__name__)


@dataclass
class SearchResult:
    """搜索结果"""
    items: List[Dict]
    total_matches: int
    query_time_ms: float
    query: str
    method: str
    highlights: List[Dict] = field(default_factory=list)
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "items": self.items,
            "total_matches": self.total_matches,
            "query_time_ms": self.query_time_ms,
            "query": self.query,
            "method": self.method,
            "highlights": self.highlights
        }


@dataclass
class SearchFilter:
    """搜索过滤器"""
    field: str
    operator: str  # eq, ne, contains, gt, lt, gte, lte, in, not_in
    value: Any
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "field": self.field,
            "operator": self.operator,
            "value": self.value
        }


class EnhancedSearcher:
    """增强搜索器
    
    提供更强大的数据集搜索功能。
    """
    
    def __init__(self, items: List[Dict] = None):
        """初始化搜索器
        
        Args:
            items: 数据列表
        """
        self._items = items or []
        self._indexes: Dict[str, Dict[str, List[int]]] = {}
        self._build_indexes()
    
    def load(self, items: List[Dict]):
        """加载数据
        
        Args:
            items: 数据列表
        """
        self._items = items
        self._build_indexes()
    
    def _build_indexes(self):
        """构建索引"""
        self._indexes.clear()
        
        for idx, item in enumerate(self._items):
            for field, value in item.items():
                if isinstance(value, str):
                    if field not in self._indexes:
                        self._indexes[field] = defaultdict(list)
                    
                    # 精确值索引
                    self._indexes[field][value.lower()].append(idx)
                    
                    # 分词索引
                    words = self._tokenize(value)
                    for word in words:
                        self._indexes[field][word.lower()].append(idx)
    
    def _tokenize(self, text: str) -> List[str]:
        """分词
        
        Args:
            text: 输入文本
        
        Returns:
            分词结果
        """
        # 简单分词：按标点和空格分割
        tokens = re.findall(r'[\u4e00-\u9fff]+|[a-zA-Z]+|\d+', text)
        return tokens

    @staticmethod
    def _ngrams(text: str, n: int) -> set:
        """字符 n-gram 集合（小写化；文本短于 n 时返回空集）

        Args:
            text: 文本
            n: n-gram 大小

        Returns:
            n-gram 集合
        """
        lowered = text.lower()
        if len(lowered) < n:
            return set()
        return {lowered[i:i + n] for i in range(len(lowered) - n + 1)}

    def _search_ngram(self, field: str, query: str, n: int = 2) -> Dict[int, float]:
        """n-gram 搜索

        按查询 n-gram 在文档中的覆盖率打分（命中数 / 查询 n-gram 总数）。

        移植自 `indexer.DatasetIndexer.search_ngram`：那个方法只存在于被合并掉的
        另一条 CLI 路径上，不一并移植的话 `--method ngram` 会在合并后凭空消失。
        差别是这里返回 0~1 的覆盖率而不是裸命中计数，这样它能和其它方法一样参与
        「多字段得分累加 + 按分排序」，否则混用多种方法时排序没有意义。

        Args:
            field: 字段名
            query: 查询字符串
            n: n-gram 大小

        Returns:
            {索引: 覆盖率}
        """
        query_ngrams = self._ngrams(query, n)
        if not query_ngrams:
            return {}

        matches: Dict[int, float] = {}
        for idx, item in enumerate(self._items):
            value = item.get(field, "")
            if not isinstance(value, str):
                continue
            hit = len(query_ngrams & self._ngrams(value, n))
            if hit:
                matches[idx] = hit / len(query_ngrams)
        return matches
    
    def search(self, query: str, fields: List[str] = None, 
               method: str = "contains", filters: List[SearchFilter] = None,
               limit: int = 100, offset: int = 0) -> SearchResult:
        """搜索数据
        
        Args:
            query: 搜索查询
            fields: 搜索字段列表
            method: 搜索方法 (exact/contains/ngram/fuzzy/regex)
            filters: 过滤器列表
            limit: 返回数量限制
            offset: 偏移量
        
        Returns:
            搜索结果
        """
        start_time = time.time()
        
        fields = fields or ["instruction", "output", "input"]
        all_matches: Dict[int, float] = defaultdict(float)
        
        # 执行搜索
        for field in fields:
            if method == "exact":
                matches = self._search_exact(field, query)
            elif method == "ngram":
                matches = self._search_ngram(field, query)
            elif method == "fuzzy":
                matches = self._search_fuzzy(field, query)
            elif method == "regex":
                matches = self._search_regex(field, query)
            else:
                matches = self._search_contains(field, query)
            
            for idx, score in matches.items():
                all_matches[idx] += score
        
        # 应用过滤器
        if filters:
            filtered_indices = set(all_matches.keys())
            for filter_item in filters:
                filtered_indices = self._apply_filter(filtered_indices, filter_item)
            all_matches = {idx: all_matches[idx] for idx in filtered_indices}
        
        # 按分数排序
        sorted_indices = sorted(all_matches.keys(), key=lambda x: all_matches[x], reverse=True)
        
        # 应用分页
        total_matches = len(sorted_indices)
        paginated_indices = sorted_indices[offset:offset + limit]
        
        # 生成高亮
        highlights = self._generate_highlights(paginated_indices, query, fields)
        
        # 获取结果
        result_items = [self._items[idx] for idx in paginated_indices]
        
        query_time = (time.time() - start_time) * 1000
        
        return SearchResult(
            items=result_items,
            total_matches=total_matches,
            query_time_ms=query_time,
            query=query,
            method=method,
            highlights=highlights
        )
    
    def _search_exact(self, field: str, query: str) -> Dict[int, float]:
        """精确搜索
        
        Args:
            field: 字段名
            query: 搜索查询
        
        Returns:
            匹配结果 {索引: 分数}
        """
        matches = {}
        
        if field in self._indexes:
            # 精确匹配
            exact_matches = self._indexes[field].get(query.lower(), [])
            for idx in exact_matches:
                matches[idx] = 1.0
        
        return matches
    
    def _search_contains(self, field: str, query: str) -> Dict[int, float]:
        """包含搜索
        
        Args:
            field: 字段名
            query: 搜索查询
        
        Returns:
            匹配结果 {索引: 分数}
        """
        matches = {}
        query_lower = query.lower()
        
        for idx, item in enumerate(self._items):
            value = item.get(field, "")
            if isinstance(value, str) and query_lower in value.lower():
                matches[idx] = 1.0
        
        return matches
    
    def _search_fuzzy(self, field: str, query: str, threshold: float = 0.6) -> Dict[int, float]:
        """模糊搜索
        
        Args:
            field: 字段名
            query: 搜索查询
            threshold: 相似度阈值
        
        Returns:
            匹配结果 {索引: 分数}
        """
        matches = {}
        query_tokens = set(self._tokenize(query.lower()))
        
        for idx, item in enumerate(self._items):
            value = item.get(field, "")
            if isinstance(value, str):
                value_tokens = set(self._tokenize(value.lower()))
                
                # 计算Jaccard相似度
                if query_tokens and value_tokens:
                    intersection = query_tokens & value_tokens
                    union = query_tokens | value_tokens
                    similarity = len(intersection) / len(union) if union else 0
                    
                    if similarity >= threshold:
                        matches[idx] = similarity
        
        return matches
    
    def _search_regex(self, field: str, query: str) -> Dict[int, float]:
        """正则表达式搜索
        
        Args:
            field: 字段名
            query: 正则表达式
        
        Returns:
            匹配结果 {索引: 分数}
        """
        matches = {}
        
        try:
            pattern = re.compile(query, re.IGNORECASE)
            
            for idx, item in enumerate(self._items):
                value = item.get(field, "")
                if isinstance(value, str) and pattern.search(value):
                    matches[idx] = 1.0
        except re.error as e:
            logger.warning(f"正则表达式错误: {e}")
        
        return matches
    
    def _apply_filter(self, indices: Set[int], filter_item: SearchFilter) -> Set[int]:
        """应用过滤器
        
        Args:
            indices: 索引集合
            filter_item: 过滤器
        
        Returns:
            过滤后的索引集合
        """
        filtered = set()
        
        for idx in indices:
            item = self._items[idx]
            value = item.get(filter_item.field)
            
            if self._evaluate_filter(value, filter_item.operator, filter_item.value):
                filtered.add(idx)
        
        return filtered
    
    def _evaluate_filter(self, value: Any, operator: str, filter_value: Any) -> bool:
        """评估过滤条件
        
        Args:
            value: 字段值
            operator: 操作符
            filter_value: 过滤值
        
        Returns:
            是否匹配
        """
        if operator == "eq":
            return value == filter_value
        elif operator == "ne":
            return value != filter_value
        elif operator == "contains":
            return isinstance(value, str) and filter_value in value
        elif operator == "gt":
            return isinstance(value, (int, float)) and value > filter_value
        elif operator == "lt":
            return isinstance(value, (int, float)) and value < filter_value
        elif operator == "gte":
            return isinstance(value, (int, float)) and value >= filter_value
        elif operator == "lte":
            return isinstance(value, (int, float)) and value <= filter_value
        elif operator == "in":
            return value in filter_value if isinstance(filter_value, (list, set)) else False
        elif operator == "not_in":
            return value not in filter_value if isinstance(filter_value, (list, set)) else True
        
        return False
    
    def _generate_highlights(self, indices: List[int], query: str, fields: List[str]) -> List[Dict]:
        """生成高亮
        
        Args:
            indices: 索引列表
            query: 搜索查询
            fields: 字段列表
        
        Returns:
            高亮列表
        """
        highlights = []
        query_lower = query.lower()
        
        for idx in indices:
            item = self._items[idx]
            item_highlights = {}
            
            for field in fields:
                value = item.get(field, "")
                if isinstance(value, str) and query_lower in value.lower():
                    # 生成高亮文本
                    start = value.lower().find(query_lower)
                    end = start + len(query)
                    
                    highlighted = (
                        value[:start],
                        value[start:end],
                        value[end:]
                    )
                    item_highlights[field] = highlighted
            
            if item_highlights:
                highlights.append({
                    "index": idx,
                    "highlights": item_highlights
                })
        
        return highlights
    
    def get_statistics(self) -> Dict:
        """获取统计信息
        
        Returns:
            统计信息
        """
        return {
            "total_items": len(self._items),
            "indexed_fields": list(self._indexes.keys()),
            "field_counts": {
                field: len(values)
                for field, values in self._indexes.items()
            }
        }


def search_dataset(items: List[Dict], query: str, fields: List[str] = None,
                   method: str = "contains", limit: int = 100,
                   offset: int = 0) -> SearchResult:
    """搜索数据集
    
    Args:
        items: 数据列表
        query: 搜索查询
        fields: 搜索字段列表
        method: 搜索方法
        limit: 返回数量限制
        offset: 偏移量
    
    Returns:
        搜索结果
    """
    searcher = EnhancedSearcher(items)
    return searcher.search(query, fields, method, limit=limit, offset=offset)


def create_searcher(items: List[Dict] = None) -> EnhancedSearcher:
    """创建搜索器
    
    Args:
        items: 数据列表
    
    Returns:
        搜索器实例
    """
    return EnhancedSearcher(items)
