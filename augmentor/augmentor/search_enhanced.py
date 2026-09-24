# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集搜索增强模块

提供更强大的数据集搜索功能。
"""

import re
import logging
import threading
from typing import List, Dict, Optional, Any, Callable, Set, Union
from dataclasses import dataclass, field
from collections import defaultdict
from collections.abc import Mapping
import time

from augmentor.exceptions import DataValidationError

logger = logging.getLogger(__name__)


@dataclass
class SearchResult:
    """搜索结果
    
    `fuzzy_threshold` / `ngram_n` 是**本次方法真正消费的**松紧旋钮：方法不是
    `fuzzy` / `ngram` 时为 `None`，而不是重复默认值。回显的意义在于「找到 0 条」
    有两种完全不同的成因（语料里确实没有 / 旋钮拧得太紧），只报 `method` 分不出这两者。

    `applied_filters` / `matches_before_filters` 是同一族回显，管的是第三种成因：
    **过滤器把候选筛掉了**。前者回显**规范化之后**的过滤器（`{field, operator, value}`
    字典，不泄漏调用方传入的对象本身），后者回显过滤**之前**检索命中了多少条。
    没有传过滤器时两者都是 `None`（而不是 `[]` / `0`）——「没过滤」与「过滤后剩 0 条」
    必须是两个读得出来的状态。注意口径：`matches_before_filters` 与 `total_matches`
    都是**分页之前**的全集条数，只有 `items` 受 `limit` / `offset` 影响。
    """
    items: List[Dict]
    total_matches: int
    query_time_ms: float
    query: str
    method: str
    highlights: List[Dict] = field(default_factory=list)
    fuzzy_threshold: Optional[float] = None
    ngram_n: Optional[int] = None
    applied_filters: Optional[List[Dict[str, Any]]] = None
    matches_before_filters: Optional[int] = None
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "items": self.items,
            "total_matches": self.total_matches,
            "query_time_ms": self.query_time_ms,
            "query": self.query,
            "method": self.method,
            "highlights": self.highlights,
            "fuzzy_threshold": self.fuzzy_threshold,
            "ngram_n": self.ngram_n,
            "applied_filters": self.applied_filters,
            "matches_before_filters": self.matches_before_filters
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


FILTER_OPERATORS = ("eq", "ne", "contains", "gt", "lt", "gte", "lte", "in", "not_in")

# 每个算子对**过滤值形状**的要求：`None` 表示不挑形状（`eq` / `ne` 比的是任意值）。
# 判据只看用户传进来的那个值，**不看文档字段里的值**——字段类型不合是数据事实，
# 该算「不匹配」；过滤值形状不对是参数写错，必须报错。
_FILTER_VALUE_KINDS = {
    "contains": ((str,), "字符串"),
    "gt": ((int, float), "数字"),
    "lt": ((int, float), "数字"),
    "gte": ((int, float), "数字"),
    "lte": ((int, float), "数字"),
    "in": ((list, tuple, set), "列表或集合"),
    "not_in": ((list, tuple, set), "列表或集合"),
}


def normalize_filters(filters: Any) -> Optional[List[SearchFilter]]:
    """把公开入口收到的过滤器清单整理成 `SearchFilter` 列表，顺手判一遍形状

    元素可以是 `SearchFilter` 对象，也可以是 `{"field": ..., "operator": ..., "value": ...}`
    字典（CLI 与 API 只能拿到后者），两者混着传也行。

    形状不对**就地报错**，不返回空清单也不返回原样：未知算子在 `_evaluate_filter` 里
    返回 `False`，于是整条过滤器把候选清零；`in` 拿到字符串同样全清，而 `not_in` 拿到
    字符串却全留。用户读到的是「语料里没有」，真相是参数写错了——与 A27/A28④ 同一形状。

    Args:
        filters: `None` 或过滤器清单

    Returns:
        `SearchFilter` 列表；`filters` 为 `None` 时返回 `None`。成员档（`in` /
        `not_in`）收到的集合会归一成列表，所以这个返回值是规范化产物的唯一形态。

    Raises:
        DataValidationError: 清单本身不是列表、元素形状不对、算子不存在、
            字段名不是字符串，或过滤值的形状与该算子的要求不符
    """
    if filters is None:
        return None
    if isinstance(filters, (SearchFilter, str, bytes)) or not isinstance(filters, (list, tuple)):
        raise DataValidationError(
            "过滤器清单必须是列表（元素为 SearchFilter 或含 field / operator / value 的字典），"
            f"当前是 {type(filters).__name__}"
        )

    normalized: List[SearchFilter] = []
    for position, raw in enumerate(filters, start=1):
        if isinstance(raw, SearchFilter):
            field, operator, value = raw.field, raw.operator, raw.value
        elif isinstance(raw, Mapping):
            missing = [key for key in ("field", "operator", "value") if key not in raw]
            if missing:
                raise DataValidationError(
                    f"第 {position} 个过滤器缺少键 {missing}（需要 field / operator / value）"
                )
            field, operator, value = raw["field"], raw["operator"], raw["value"]
        else:
            raise DataValidationError(
                f"第 {position} 个过滤器既不是 SearchFilter 也不是字典，"
                f"而是 {type(raw).__name__}"
            )

        if not isinstance(field, str):
            raise DataValidationError(
                f"第 {position} 个过滤器的字段名必须是字符串，当前是 {type(field).__name__}"
            )
        if operator not in FILTER_OPERATORS:
            raise DataValidationError(
                f"第 {position} 个过滤器的算子 {operator!r} 不存在"
                f"（可选 {' / '.join(FILTER_OPERATORS)}）"
            )
        required = _FILTER_VALUE_KINDS.get(operator)
        if required is not None:
            kinds, label = required
            if isinstance(value, bool) or not isinstance(value, kinds):
                raise DataValidationError(
                    f"第 {position} 个过滤器（{field} {operator}）的值必须是{label}，"
                    f"当前是 {type(value).__name__}"
                )
            if isinstance(value, set):
                # 成员档收到集合时落成列表：`in` / `not_in` 的判定与集合等价，
                # 但 `SearchResult.applied_filters` 要能进 JSON（集合不能），
                # 而规范化产物是那份回显的唯一来源。不排序——混合类型排不了，
                # 集合本来也无序。
                value = list(value)

        normalized.append(SearchFilter(field=field, operator=operator, value=value))
    return normalized


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
        # 倒排索引**按需构建**：五种方法里只有 exact 读它，而建一份要扫一遍全文并
        # 常驻 6 MB（真实 6902 条实测 64.5 ms）。`search_dataset()` 每次调用都新建
        # 一个搜索器，于是默认的 contains 查询也要先付这 64.5 ms —— 实测端到端
        # 80.85 ms 里只有 7.5 ms 是查询本身。
        self._index_lock = threading.Lock()
        self._index_ready = False

    def load(self, items: List[Dict]):
        """加载数据

        Args:
            items: 数据列表
        """
        self._items = items
        # 换数据 = 作废索引（缓存键必须覆盖「值会变的东西」，否则新数据会被旧索引
        # 回答成「查无此项」）
        with self._index_lock:
            self._index_ready = False

    def _ensure_indexes(self) -> None:
        """需要索引时才构建，且**只构建一次**

        锁内二次检查：多个线程同时发来第一条 exact 查询时，只有一个构建，其余等待
        后直接复用。不加这层的话，先发布空字典再填内容的写法会让别的线程读到半成品
        索引并把结果误报成「无匹配」。
        """
        if self._index_ready:
            return
        with self._index_lock:
            if not self._index_ready:
                self._build_indexes()

    def _build_indexes(self):
        """构建索引

        建在局部字典上、最后整体发布，配合 `_index_ready` 才是上面那句「不会有人
        读到半成品」成立的前提。
        """
        indexes: Dict[str, Dict[str, List[int]]] = {}

        for idx, item in enumerate(self._items):
            for field, value in item.items():
                if isinstance(value, str):
                    if field not in indexes:
                        indexes[field] = defaultdict(list)

                    # 精确值索引
                    indexes[field][value.lower()].append(idx)

                    # 分词索引
                    words = self._tokenize(value)
                    for word in words:
                        indexes[field][word.lower()].append(idx)

        self._indexes = indexes
        self._index_ready = True

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

        **打分规模跟着查询走，不跟着文档走**：一个查询 gram 出现在文档里，等价于
        「它是文档的子串」，所以只需要对查询侧那 k 个 gram 各做一次子串判定。以前
        这里为每条文档都物化整串的 gram 集合（`_ngrams(value, n)`），代价是每条
        O(文档长度) 个字符串对象——真实 6902 条 × 3 字段实测一次 ngram 查询
        104~113 ms，而同样的数据 contains 只要 8.2 ms。改成子串判定后单字段
        22.2 → 3.2 ms（中位），三字段端到端 12~16 ms，结果与旧实现逐条相同
        （见 `test_scores_are_identical_to_a_naive_gram_set`）。

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

        total = len(query_ngrams)
        matches: Dict[int, float] = {}
        for idx, item in enumerate(self._items):
            value = item.get(field, "")
            if not isinstance(value, str):
                continue
            lowered = value.lower()
            hit = sum(1 for gram in query_ngrams if gram in lowered)
            if hit:
                matches[idx] = hit / total
        return matches
    
    def search(self, query: str, fields: List[str] = None, 
               method: str = "contains",
               filters: List[Union[SearchFilter, Dict[str, Any]]] = None,
               limit: int = 100, offset: int = 0,
               fuzzy_threshold: float = 0.6, ngram_n: int = 2) -> SearchResult:
        """搜索数据
        
        Args:
            query: 搜索查询
            fields: 搜索字段列表
            method: 搜索方法 (exact/contains/ngram/fuzzy/regex)
            filters: 过滤器列表，元素可以是 `SearchFilter` 或
                `{"field", "operator", "value"}` 字典（见 `normalize_filters`）
            limit: 返回数量限制
            offset: 偏移量
            fuzzy_threshold: fuzzy 的相似度门槛，开区间下界、闭区间上界
                `(0, 1]`；越大越严格，1.0 即要求整串出现
            ngram_n: ngram 的 gram 长度，`>= 1`；越大越严格

        Returns:
            搜索结果。`fuzzy_threshold` / `ngram_n` 两键回显**本次方法真正生效**的档位，
                方法没消费那个旋钮时是 `None`（见 `SearchResult`）。`applied_filters` /
                `matches_before_filters` 两键回显**本次真的用上了哪些过滤器**、以及过滤
                **之前**检索命中多少条，没传过滤器时同样是 `None`——「没过滤」和
                「过滤后剩 0 条」必须是两个读得出来的状态。

        Raises:
            DataValidationError: `fuzzy_threshold` 或 `ngram_n` 越界（它同时是
                `ValueError`，所以 API 那边照旧映射成 400）。两个旋钮只对各自的方法
                有意义，但**不判方法**——判据是值本身是否在文档域内，所以
                `--method contains --fuzzy-threshold 0` 也会失败：那是在请求一个
                任何方法都给不出结果的门槛，静默忽略等于骗人。
                同样在这里判的还有 `filters` 的形状（见 `normalize_filters`）。
        """
        if not 0 < fuzzy_threshold <= 1:
            raise DataValidationError(
                f"模糊阈值必须在 (0, 1] 区间内，当前是 {fuzzy_threshold}"
            )
        if not isinstance(ngram_n, int) or isinstance(ngram_n, bool) or ngram_n < 1:
            raise DataValidationError(
                f"n-gram 长度必须是大于 0 的整数，当前是 {ngram_n}"
            )
        # 只在这一处判；`_evaluate_filter` 里那些 `isinstance` 是文档侧的「不匹配」，
        # 不是参数校验，两者不能混（见 `normalize_filters` 的说明）。
        normalized_filters = normalize_filters(filters)

        start_time = time.time()
        
        fields = fields or ["instruction", "output", "input"]
        all_matches: Dict[int, float] = defaultdict(float)
        
        # 执行搜索
        for field in fields:
            if method == "exact":
                matches = self._search_exact(field, query)
            elif method == "ngram":
                matches = self._search_ngram(field, query, ngram_n)
            elif method == "fuzzy":
                matches = self._search_fuzzy(field, query, fuzzy_threshold)
            elif method == "regex":
                matches = self._search_regex(field, query)
            else:
                matches = self._search_contains(field, query)
            
            for idx, score in matches.items():
                all_matches[idx] += score
        
        # 应用过滤器
        applied_filters: Optional[List[Dict[str, Any]]] = None
        matches_before_filters: Optional[int] = None
        if normalized_filters:
            # 先记下过滤**之前**的候选数：「找到 0 条」到这里才有三种成因的完整读法——
            # 检索就没命中（before=0）、旋钮拧太紧（上面两键）、过滤器把候选筛光（before>0）
            matches_before_filters = len(all_matches)
            applied_filters = [filter_item.to_dict() for filter_item in normalized_filters]
            filtered_indices = set(all_matches.keys())
            for filter_item in normalized_filters:
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
            highlights=highlights,
            fuzzy_threshold=fuzzy_threshold if method == "fuzzy" else None,
            ngram_n=ngram_n if method == "ngram" else None,
            applied_filters=applied_filters,
            matches_before_filters=matches_before_filters,
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
        self._ensure_indexes()

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
        """模糊搜索：把查询当成**等长窗口**在文档里滑动，容许换字

        分数 = 最好的那个窗口的 `1 - 错配数 / 查询长度`，门槛是 `threshold`。所以
        完整包含查询的文档必然得 1.0 —— `contains` 命中的条目在这里一定是子集，
        fuzzy 多出来的那部分是「允许把几个字换掉」的近似命中（「公租屋」找得到
        「公租房」）。只处理**替换**（Hamming），不做插入/删除（Levenshtein）：
        窗口式全扫的代价是 O(文档长度 × 查询长度)，换成编辑距离 DP 会再乘一个查询长度。

        以前这里算的是查询 token 与**整条文档** token 集合的 Jaccard，在真实 6902 条
        上任何查询都恒 0 命中（A27），两条病因叠加致命：①`_tokenize` 把一整段连续
        中文当成**一个** token，查询 `{'租房'}` 与文档 `{'如何申请租房'}` 永不相交；
        ②即便相交，分母是并集，短查询摊不过长文档的 token 数，0.6 根本够不着
        （实测把阈值从 0.6 一路降到 0.02 仍为 0 命中）。对称的整档相似度对
        「短查询 vs 长文档」本身就是错的度量，所以换成窗口式。

        为了不把它退化成裸扫（朴素窗口实测 6902 条 1.0~1.9 s），加一层**鸽笼过滤**：
        错配 ≤ d 的窗口必然与查询切成 `d+1` 段中的某一段完全相同，于是先用 `in`
        把整条文档排除掉，只对候选做逐窗口校验。过滤是**纯优化**，结果与朴素全扫
        逐条相同（用例里拿朴素实现做等价对照）。

        Args:
            field: 字段名
            query: 搜索查询
            threshold: 相似度阈值，`(0, 1]`；越大越严格（1.0 即要求整串出现）

        Returns:
            匹配结果 {索引: 分数}
        """
        matches: Dict[int, float] = {}
        lowered_query = query.lower()
        length = len(lowered_query)
        if not length or not 0 < threshold <= 1:
            return matches

        allowed = int(length * (1.0 - threshold))   # 允许的错配数
        pieces = min(allowed + 1, length)           # 鸽笼：片数 = 错配预算 + 1
        filters = [lowered_query[i * length // pieces:(i + 1) * length // pieces]
                   for i in range(pieces)]

        for idx, item in enumerate(self._items):
            value = item.get(field, "")
            if not isinstance(value, str) or len(value) < length:
                continue
            lowered = value.lower()
            if not any(piece in lowered for piece in filters):
                continue

            best = 0.0
            for start in range(len(lowered) - length + 1):
                window = lowered[start:start + length]
                mismatches = sum(1 for a, b in zip(lowered_query, window) if a != b)
                if not mismatches:
                    best = 1.0
                    break
                score = 1.0 - mismatches / length
                if score > best:
                    best = score

            if best >= threshold:
                matches[idx] = best

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
        self._ensure_indexes()
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
                   offset: int = 0, fuzzy_threshold: float = 0.6,
                   ngram_n: int = 2,
                   filters: List[Union[SearchFilter, Dict[str, Any]]] = None) -> SearchResult:
    """搜索数据集
    
    Args:
        items: 数据列表
        query: 搜索查询
        fields: 搜索字段列表
        method: 搜索方法
        limit: 返回数量限制
        offset: 偏移量
        fuzzy_threshold: fuzzy 的相似度门槛 `(0, 1]`，见 `EnhancedSearcher.search`
        ngram_n: ngram 的 gram 长度 `>= 1`，见 `EnhancedSearcher.search`
        filters: 检索后的元数据收窄条件，见 `normalize_filters`

    Returns:
        搜索结果
    """
    searcher = EnhancedSearcher(items)
    return searcher.search(query, fields, method, filters=filters, limit=limit,
                           offset=offset, fuzzy_threshold=fuzzy_threshold,
                           ngram_n=ngram_n)


def create_searcher(items: List[Dict] = None) -> EnhancedSearcher:
    """创建搜索器
    
    Args:
        items: 数据列表
    
    Returns:
        搜索器实例
    """
    return EnhancedSearcher(items)
