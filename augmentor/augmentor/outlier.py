# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""异常值检测模块

基于统计分布（均值/标准差、分位数）检测数据集中数值型字段的异常样本，
用于在增强前隔离长度异常的可疑脏数据。
"""

import logging
import math
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, Tuple

logger = logging.getLogger(__name__)


@dataclass
class OutlierReport:
    """异常检测结果"""
    total_items: int
    outliers: List[Dict[str, Any]] = field(default_factory=list)
    method: str = "zscore"
    threshold: float = 3.0
    field: str = "length"

    @property
    def outlier_count(self) -> int:
        return len(self.outliers)

    @property
    def outlier_rate(self) -> float:
        if self.total_items == 0:
            return 0.0
        return self.outlier_count / self.total_items

    def to_dict(self) -> Dict[str, Any]:
        return {
            "total_items": self.total_items,
            "outlier_count": self.outlier_count,
            "outlier_rate": self.outlier_rate,
            "method": self.method,
            "threshold": self.threshold,
            "field": self.field,
            "outliers": self.outliers,
        }


class OutlierDetector:
    """数值字段异常值检测器

    支持三种方法：
    - zscore: 基于均值 ± N*标准差（默认 3）
    - iqr: 基于四分位距 [Q1 - k*IQR, Q3 + k*IQR]（默认 k=1.5）
    - zscore_one_sided: 仅检测上侧异常（文本过长场景）
    """

    def __init__(self,
                 method: str = "zscore",
                 threshold: float = 3.0,
                 field: str = "length"):
        """初始化检测器

        Args:
            method: 检测方法 (zscore / iqr / zscore_one_sided)
            threshold: 判定阈值（zscore 为倍数，iqr 为 IQR 倍数）
            field: 检测的数值字段名
        """
        if method not in ("zscore", "iqr", "zscore_one_sided"):
            raise ValueError(f"不支持的异常检测方法: {method}")
        if threshold <= 0:
            raise ValueError("threshold 必须为正数")

        self.method = method
        self.threshold = threshold
        self.field = field

    def _extract_values(self, items: List[Dict]) -> List[Optional[float]]:
        """提取数值字段，缺失或非数值时返回 None

        Args:
            items: 数据列表

        Returns:
            与 items 等长的数值列表
        """
        values: List[Optional[float]] = []
        for item in items:
            raw = item.get(self.field)
            if raw is None:
                values.append(None)
                continue
            try:
                values.append(float(raw))
            except (TypeError, ValueError):
                values.append(None)
        return values

    def _mean_std(self, values: List[float]) -> Tuple[float, float]:
        """计算均值与总体标准差

        Args:
            values: 数值列表

        Returns:
            (均值, 标准差)
        """
        n = len(values)
        if n == 0:
            return 0.0, 0.0
        mean = sum(values) / n
        variance = sum((v - mean) ** 2 for v in values) / n
        return mean, math.sqrt(variance)

    def _quantile(self, values: List[float], q: float) -> float:
        """线性插值分位数

        Args:
            values: 数值列表
            q: 分位点 (0-1)

        Returns:
            分位数值
        """
        if not values:
            return 0.0
        ordered = sorted(values)
        if len(ordered) == 1:
            return ordered[0]
        position = q * (len(ordered) - 1)
        lower = int(math.floor(position))
        upper = min(lower + 1, len(ordered) - 1)
        fraction = position - lower
        return ordered[lower] + (ordered[upper] - ordered[lower]) * fraction

    def detect(self, items: List[Dict]) -> OutlierReport:
        """检测异常样本

        Args:
            items: 数据列表

        Returns:
            异常检测报表
        """
        if not items:
            return OutlierReport(
                total_items=0, method=self.method,
                threshold=self.threshold, field=self.field
            )

        raw_values = self._extract_values(items)
        valid = [v for v in raw_values if v is not None]

        if len(valid) < 2:
            return OutlierReport(
                total_items=len(items), method=self.method,
                threshold=self.threshold, field=self.field
            )

        if self.method == "iqr":
            q1 = self._quantile(valid, 0.25)
            q3 = self._quantile(valid, 0.75)
            iqr = q3 - q1
            lower = q1 - self.threshold * iqr
            upper = q3 + self.threshold * iqr
        else:
            mean, std = self._mean_std(valid)
            if std == 0:
                return OutlierReport(
                    total_items=len(items), method=self.method,
                    threshold=self.threshold, field=self.field
                )
            if self.method == "zscore_one_sided":
                lower = -math.inf
                upper = mean + self.threshold * std
            else:
                lower = mean - self.threshold * std
                upper = mean + self.threshold * std

        outliers: List[Dict[str, Any]] = []
        for index, (item, value) in enumerate(zip(items, raw_values)):
            if value is None:
                continue
            if value < lower or value > upper:
                outliers.append({
                    "index": index,
                    "value": value,
                    "item": item,
                })

        logger.info(
            f"异常检测完成: {len(outliers)}/{len(items)} ({self.method})"
        )

        return OutlierReport(
            total_items=len(items),
            outliers=outliers,
            method=self.method,
            threshold=self.threshold,
            field=self.field,
        )

    def filter(self, items: List[Dict]) -> List[Dict]:
        """移除异常样本

        Args:
            items: 数据列表

        Returns:
            保留的数据列表
        """
        report = self.detect(items)
        if not report.outliers:
            return list(items)

        outlier_indices = {o["index"] for o in report.outliers}
        return [
            item for index, item in enumerate(items)
            if index not in outlier_indices
        ]

    def attach_length_field(self, items: List[Dict],
                           source_field: str = "instruction") -> List[Dict]:
        """为数据附加长度字段，便于后续检测

        Args:
            items: 数据列表
            source_field: 用于计算长度的源字段

        Returns:
            附加了 length 字段的副本列表
        """
        enriched: List[Dict] = []
        for item in items:
            new_item = dict(item)
            new_item["length"] = len(str(new_item.get(source_field, "")))
            enriched.append(new_item)
        return enriched


def detect_outliers(items: List[Dict],
                    method: str = "zscore",
                    threshold: float = 3.0,
                    field: str = "length") -> OutlierReport:
    """便捷函数：执行异常检测

    Args:
        items: 数据列表
        method: 检测方法
        threshold: 阈值
        field: 数值字段

    Returns:
        异常检测报表
    """
    return OutlierDetector(method=method, threshold=threshold, field=field).detect(items)


__all__ = [
    "OutlierDetector",
    "OutlierReport",
    "detect_outliers",
]
