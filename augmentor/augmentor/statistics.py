# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集统计模块

提供数据集的详细统计信息。

分工边界（与 `augmentor.analytics`）
    本模块做**字段级统计**：填充率 / 平均长度 / 唯一值 + `quality_metrics`，
    入口 `calculate_statistics`。CLI 的 `stats` 命令走这里。
    要整体评估与改进建议（质量 / 多样性 / 完整性分数 + insights），用 `analytics`。
"""

import re
import logging
from typing import List, Dict, Optional, Any, Set
from dataclasses import dataclass, field
from collections import Counter, defaultdict
from pathlib import Path

logger = logging.getLogger(__name__)


@dataclass
class FieldStatistics:
    """字段统计信息"""
    field_name: str
    total_count: int
    filled_count: int
    empty_count: int
    avg_length: float
    min_length: int
    max_length: int
    median_length: float
    std_deviation: float
    unique_count: int
    top_values: List[tuple] = field(default_factory=list)
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "field_name": self.field_name,
            "total_count": self.total_count,
            "filled_count": self.filled_count,
            "empty_count": self.empty_count,
            "fill_rate": self.filled_count / self.total_count if self.total_count > 0 else 0,
            "avg_length": self.avg_length,
            "min_length": self.min_length,
            "max_length": self.max_length,
            "median_length": self.median_length,
            "std_deviation": self.std_deviation,
            "unique_count": self.unique_count,
            "top_values": self.top_values
        }


@dataclass
class DatasetStatistics:
    """数据集统计信息"""
    dataset_name: str
    total_items: int
    field_statistics: Dict[str, FieldStatistics] = field(default_factory=dict)
    content_statistics: Dict[str, Any] = field(default_factory=dict)
    quality_metrics: Dict[str, float] = field(default_factory=dict)
    summary: str = ""
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "dataset_name": self.dataset_name,
            "total_items": self.total_items,
            "field_statistics": {
                name: stats.to_dict()
                for name, stats in self.field_statistics.items()
            },
            "content_statistics": self.content_statistics,
            "quality_metrics": self.quality_metrics,
            "summary": self.summary
        }


class DatasetStatisticsCalculator:
    """数据集统计计算器
    
    提供数据集的详细统计信息。
    """
    
    def __init__(self, items: List[Dict] = None, dataset_name: str = "unknown"):
        """初始化统计计算器
        
        Args:
            items: 数据列表
            dataset_name: 数据集名称
        """
        self._items = items or []
        self._dataset_name = dataset_name
    
    def load(self, items: List[Dict], dataset_name: str = "unknown"):
        """加载数据
        
        Args:
            items: 数据列表
            dataset_name: 数据集名称
        """
        self._items = items
        self._dataset_name = dataset_name
    
    def calculate(self, fields: List[str] = None) -> DatasetStatistics:
        """计算统计信息
        
        Args:
            fields: 要统计的字段列表
        
        Returns:
            数据集统计信息
        """
        fields = fields or self._get_all_fields()
        
        stats = DatasetStatistics(
            dataset_name=self._dataset_name,
            total_items=len(self._items)
        )
        
        # 计算每个字段的统计信息
        for field_name in fields:
            stats.field_statistics[field_name] = self._calculate_field_statistics(field_name)
        
        # 计算内容统计
        stats.content_statistics = self._calculate_content_statistics()
        
        # 计算质量指标
        stats.quality_metrics = self._calculate_quality_metrics()
        
        # 生成摘要
        stats.summary = self._generate_summary(stats)
        
        return stats
    
    def _get_all_fields(self) -> List[str]:
        """获取所有字段
        
        Returns:
            字段列表
        """
        all_fields = set()
        for item in self._items:
            all_fields.update(item.keys())
        return sorted(list(all_fields))
    
    def _calculate_field_statistics(self, field_name: str) -> FieldStatistics:
        """计算字段统计信息
        
        Args:
            field_name: 字段名
        
        Returns:
            字段统计信息
        """
        values = []
        lengths = []
        filled_count = 0
        
        for item in self._items:
            value = item.get(field_name, "")
            values.append(value)
            
            if value:
                filled_count += 1
                if isinstance(value, str):
                    lengths.append(len(value))
                else:
                    lengths.append(len(str(value)))
        
        total_count = len(self._items)
        empty_count = total_count - filled_count
        
        # 计算长度统计
        if lengths:
            avg_length = sum(lengths) / len(lengths)
            min_length = min(lengths)
            max_length = max(lengths)
            
            # 中位数
            sorted_lengths = sorted(lengths)
            n = len(sorted_lengths)
            if n % 2 == 0:
                median_length = (sorted_lengths[n//2 - 1] + sorted_lengths[n//2]) / 2
            else:
                median_length = sorted_lengths[n//2]
            
            # 标准差
            variance = sum((l - avg_length) ** 2 for l in lengths) / len(lengths)
            std_deviation = variance ** 0.5
        else:
            avg_length = 0
            min_length = 0
            max_length = 0
            median_length = 0
            std_deviation = 0
        
        # 唯一值数量
        unique_values = set(str(v) for v in values if v)
        unique_count = len(unique_values)
        
        # 热门值
        value_counter = Counter(str(v) for v in values if v)
        top_values = value_counter.most_common(10)
        
        return FieldStatistics(
            field_name=field_name,
            total_count=total_count,
            filled_count=filled_count,
            empty_count=empty_count,
            avg_length=avg_length,
            min_length=min_length,
            max_length=max_length,
            median_length=median_length,
            std_deviation=std_deviation,
            unique_count=unique_count,
            top_values=top_values
        )
    
    def _calculate_content_statistics(self) -> Dict[str, Any]:
        """计算内容统计
        
        Returns:
            内容统计信息
        """
        stats = {}
        
        # 收集所有文本
        all_texts = []
        for item in self._items:
            for field, value in item.items():
                if isinstance(value, str) and value:
                    all_texts.append(value)
        
        if not all_texts:
            return stats
        
        # 文本长度分布
        lengths = [len(text) for text in all_texts]
        stats["text_length"] = {
            "avg": sum(lengths) / len(lengths),
            "min": min(lengths),
            "max": max(lengths),
            "total": sum(lengths)
        }
        
        # 词汇统计
        all_words = []
        for text in all_texts:
            words = re.findall(r'[\u4e00-\u9fff]+|[a-zA-Z]+|\d+', text)
            all_words.extend(words)
        
        word_freq = Counter(all_words)
        stats["vocabulary"] = {
            "total_words": len(all_words),
            "unique_words": len(word_freq),
            "top_words": word_freq.most_common(20)
        }
        
        return stats
    
    def _calculate_quality_metrics(self) -> Dict[str, float]:
        """计算质量指标
        
        Returns:
            质量指标
        """
        metrics = {}
        
        if not self._items:
            return metrics
        
        # 完整性指标
        required_fields = ["instruction", "output"]
        total_required = len(required_fields) * len(self._items)
        filled_required = sum(
            1 for item in self._items
            for field in required_fields
            if item.get(field)
        )
        metrics["completeness"] = filled_required / total_required if total_required > 0 else 0
        
        # 多样性指标
        instructions = [item.get("instruction", "") for item in self._items if item.get("instruction")]
        if instructions:
            unique_instructions = set(instructions)
            metrics["diversity"] = len(unique_instructions) / len(instructions)
        else:
            metrics["diversity"] = 0
        
        # 一致性指标（简单检查）
        consistent_count = 0
        for item in self._items:
            instruction = item.get("instruction", "")
            output = item.get("output", "")
            if instruction and output and instruction != output:
                consistent_count += 1
        metrics["consistency"] = consistent_count / len(self._items) if self._items else 0
        
        return metrics
    
    def _generate_summary(self, stats: DatasetStatistics) -> str:
        """生成摘要
        
        Args:
            stats: 数据集统计信息
        
        Returns:
            摘要文本
        """
        lines = [
            f"数据集 '{stats.dataset_name}' 共 {stats.total_items} 条数据",
            f"包含 {len(stats.field_statistics)} 个字段"
        ]
        
        # 添加字段摘要
        for field_name, field_stats in stats.field_statistics.items():
            fill_rate = field_stats.filled_count / field_stats.total_count if field_stats.total_count > 0 else 0
            lines.append(f"  - {field_name}: 填充率 {fill_rate:.1%}, 平均长度 {field_stats.avg_length:.1f}")
        
        # 添加质量指标
        if stats.quality_metrics:
            lines.append("质量指标:")
            for metric_name, metric_value in stats.quality_metrics.items():
                lines.append(f"  - {metric_name}: {metric_value:.2f}")
        
        return "\n".join(lines)


def calculate_statistics(items: List[Dict], dataset_name: str = "unknown",
                        fields: List[str] = None) -> DatasetStatistics:
    """计算统计信息
    
    Args:
        items: 数据列表
        dataset_name: 数据集名称
        fields: 要统计的字段列表
    
    Returns:
        数据集统计信息
    """
    calculator = DatasetStatisticsCalculator(items, dataset_name)
    return calculator.calculate(fields)


def get_field_summary(items: List[Dict], field_name: str) -> Dict:
    """获取字段摘要
    
    Args:
        items: 数据列表
        field_name: 字段名
    
    Returns:
        字段摘要
    """
    calculator = DatasetStatisticsCalculator(items)
    stats = calculator._calculate_field_statistics(field_name)
    return stats.to_dict()
