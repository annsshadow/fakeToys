"""智能数据聚合模块

将多个数据集按策略（并集/交集/加权采样/一致性过滤）聚合为单一
训练集，支持字段映射与去重。
"""

import logging
import random
from dataclasses import dataclass, field
from typing import Any, Callable, Dict, List, Optional, Tuple

logger = logging.getLogger(__name__)


@dataclass
class AggregationResult:
    """聚合结果"""
    aggregated: List[Dict[str, Any]]
    source_counts: Dict[str, int] = field(default_factory=dict)
    removed_duplicates: int = 0
    conflicts: List[Dict[str, Any]] = field(default_factory=list)

    @property
    def total_count(self) -> int:
        return len(self.aggregated)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "total_count": self.total_count,
            "source_counts": self.source_counts,
            "removed_duplicates": self.removed_duplicates,
            "conflict_count": len(self.conflicts),
            "conflicts": self.conflicts,
        }


class DataAggregator:
    """数据集聚合器

    策略：
    - union: 取所有数据并集
    - intersection: 仅保留各源都出现的 key
    - weighted: 按 source_weights 加权随机采样
    - consistent: 仅保留对相同 key 答案一致的数据
    """

    def __init__(self,
                 key_fields: Optional[List[str]] = None,
                 value_fields: Optional[List[str]] = None,
                 random_state: Optional[int] = None):
        """初始化聚合器

        Args:
            key_fields: 用于对齐/去重的字段列表（如 instruction）
            value_fields: 用于一致性判断的字段列表（如 output）
            random_state: 随机种子
        """
        self.key_fields = key_fields or ["instruction"]
        self.value_fields = value_fields or ["output"]
        self._random = random.Random(random_state)

    def _key_of(self, item: Dict) -> str:
        """计算数据项的对齐 key"""
        return "||".join(str(item.get(f, "")) for f in self.key_fields)

    def aggregate_union(self,
                       datasets: Dict[str, List[Dict]]) -> AggregationResult:
        """并集聚合：合并所有源，按 key 去重（保留首次出现）

        Args:
            datasets: 源名 -> 数据列表

        Returns:
            聚合结果
        """
        seen: Dict[str, int] = {}
        result: List[Dict] = []
        source_counts: Dict[str, int] = {}
        removed = 0

        for source, items in datasets.items():
            source_counts[source] = len(items)
            for item in items:
                key = self._key_of(item)
                if key in seen:
                    removed += 1
                    continue
                seen[key] = source
                result.append(dict(item))

        return AggregationResult(
            aggregated=result,
            source_counts=source_counts,
            removed_duplicates=removed,
        )

    def aggregate_intersection(self,
                               datasets: Dict[str, List[Dict]]) -> AggregationResult:
        """交集聚合：仅保留所有源中都出现的 key

        保留第一条源的数据。

        Args:
            datasets: 源名 -> 数据列表

        Returns:
            聚合结果
        """
        if not datasets:
            return AggregationResult(aggregated=[], source_counts={})

        source_lists = list(datasets.values())

        key_sets = [
            {self._key_of(item) for item in items}
            for items in source_lists
        ]
        common_keys = set.intersection(*key_sets)

        result: List[Dict] = []
        for item in source_lists[0]:
            if self._key_of(item) in common_keys:
                result.append(dict(item))

        source_counts = {name: len(items) for name, items in datasets.items()}
        return AggregationResult(
            aggregated=result,
            source_counts=source_counts,
        )

    def aggregate_weighted(self,
                           datasets: Dict[str, List[Dict]],
                           weights: Optional[Dict[str, float]] = None,
                           target_size: int = 100) -> AggregationResult:
        """加权采样聚合：按权重比例从各源抽取数据

        Args:
            datasets: 源名 -> 数据列表
            weights: 源名 -> 权重（缺省各源等权）
            target_size: 目标采样总量

        Returns:
            聚合结果（按源顺序拼接，内部已按 key 去重）
        """
        weights = weights or {}
        total_weight = sum(max(0.0, weights.get(name, 1.0))
                           for name in datasets)
        if total_weight <= 0:
            total_weight = 1.0

        source_counts: Dict[str, int] = {}
        seen: Dict[str, int] = {}
        result: List[Dict] = []

        for source, items in datasets.items():
            source_counts[source] = len(items)
            weight = max(0.0, weights.get(source, 1.0))
            quota = int(target_size * (weight / total_weight))

            pool = list(items)
            self._random.shuffle(pool)

            taken = 0
            for item in pool:
                if taken >= quota:
                    break
                key = self._key_of(item)
                if key in seen:
                    continue
                seen[key] = source
                result.append(dict(item))
                taken += 1

        return AggregationResult(
            aggregated=result,
            source_counts=source_counts,
        )

    def aggregate_consistent(self,
                             datasets: Dict[str, List[Dict]]) -> AggregationResult:
        """一致性聚合：仅保留各源对相同 key 答案一致的数据

        不一致的 key 会记录到 conflicts。

        Args:
            datasets: 源名 -> 数据列表

        Returns:
            聚合结果（含 conflicts）
        """
        answers: Dict[str, List[Tuple[str, Any]]] = {}
        order: Dict[str, Dict] = {}

        for source, items in datasets.items():
            for item in items:
                key = self._key_of(item)
                value = "||".join(
                    str(item.get(f, "")) for f in self.value_fields
                )
                answers.setdefault(key, []).append((source, value))
                if key not in order:
                    order[key] = item

        result: List[Dict] = []
        conflicts: List[Dict] = []
        source_counts = {name: len(items) for name, items in datasets.items()}

        for key, pairs in answers.items():
            values = {value for _, value in pairs}
            if len(values) == 1:
                item = dict(order[key])
                item["_sources"] = [source for source, _ in pairs]
                result.append(item)
            else:
                conflicts.append({
                    "key": key,
                    "values": [value for _, value in pairs],
                    "sources": [source for source, _ in pairs],
                })

        return AggregationResult(
            aggregated=result,
            source_counts=source_counts,
            conflicts=conflicts,
        )

    def aggregate(self,
                  datasets: Dict[str, List[Dict]],
                  strategy: str = "union",
                  **kwargs: Any) -> AggregationResult:
        """按策略执行聚合

        Args:
            datasets: 源名 -> 数据列表
            strategy: 聚合策略 (union / intersection / weighted / consistent)
            **kwargs: 传递给具体策略的参数（weighted 支持 weights/target_size）

        Returns:
            聚合结果
        """
        if strategy == "union":
            return self.aggregate_union(datasets)
        if strategy == "intersection":
            return self.aggregate_intersection(datasets)
        if strategy == "weighted":
            return self.aggregate_weighted(
                datasets,
                weights=kwargs.get("weights"),
                target_size=kwargs.get("target_size", 100),
            )
        if strategy == "consistent":
            return self.aggregate_consistent(datasets)
        raise ValueError(f"不支持的聚合策略: {strategy}")


def aggregate_datasets(datasets: Dict[str, List[Dict]],
                       strategy: str = "union",
                       **kwargs: Any) -> AggregationResult:
    """便捷函数：按策略聚合多个数据集

    Args:
        datasets: 源名 -> 数据列表
        strategy: 聚合策略
        **kwargs: 策略参数

    Returns:
        聚合结果
    """
    return DataAggregator().aggregate(datasets, strategy, **kwargs)


__all__ = [
    "DataAggregator",
    "AggregationResult",
    "aggregate_datasets",
]
