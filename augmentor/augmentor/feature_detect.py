"""数据集特征检测模块

自动识别数据集中存在的特征维度（字段、长度、语言、主题、意图等），
并生成特征矩阵，用于主动学习与采样策略的覆盖分析。
"""

import logging
import re
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional

logger = logging.getLogger(__name__)

# 意图启发式关键词表（可扩展）
INTENT_KEYWORDS: Dict[str, List[str]] = {
    "how": ["怎么", "如何", "怎样", "什么流程"],
    "what": ["是什么", "什么是", "哪些"],
    "when": ["什么时候", "多久", "几时"],
    "where": ["哪里", "在哪", "哪个城市"],
    "why": ["为什么", "为何"],
    "cost": ["多少钱", "费用", "价格", "押金"],
    "process": ["流程", "手续", "步骤"],
}

# 特征类型
FEATURE_TYPES = {
    "numeric": lambda v: isinstance(v, (int, float)) and not isinstance(v, bool),
    "text": lambda v: isinstance(v, str),
    "boolean": lambda v: isinstance(v, bool),
}


@dataclass
class FeatureInfo:
    """单个特征的描述"""
    name: str
    feature_type: str
    coverage: float
    unique_ratio: float
    description: str = ""

    def to_dict(self) -> Dict[str, Any]:
        return {
            "name": self.name,
            "feature_type": self.feature_type,
            "coverage": self.coverage,
            "unique_ratio": self.unique_ratio,
            "description": self.description,
        }


class FeatureDetector:
    """数据集特征检测器

    检测维度：
    - 显式字段：数据集中每个键的覆盖率/唯一性
    - 派生特征：文本长度、语言标记、意图分类
    """

    def __init__(self,
                 text_field: str = "instruction",
                 min_coverage: float = 0.1):
        """初始化特征检测器

        Args:
            text_field: 用于文本派生特征的字段名
            min_coverage: 低于该覆盖率的字段将被标记为稀疏特征
        """
        if not 0 <= min_coverage <= 1:
            raise ValueError("min_coverage 必须在 [0, 1] 内")
        self.text_field = text_field
        self.min_coverage = min_coverage

    @staticmethod
    def _field_coverage(items: List[Dict], field: str) -> float:
        """计算字段非空覆盖率

        Args:
            items: 数据列表
            field: 字段名

        Returns:
            非空率 (0-1)
        """
        if not items:
            return 0.0
        filled = sum(1 for item in items if str(item.get(field, "")).strip())
        return filled / len(items)

    @staticmethod
    def _unique_ratio(items: List[Dict], field: str) -> float:
        """计算字段唯一值占比

        Args:
            items: 数据列表
            field: 字段名

        Returns:
            唯一值数量 / 非空样本数
        """
        values = [item.get(field) for item in items if item.get(field) not in (None, "")]
        if not values:
            return 0.0
        return len(set(values)) / len(values)

    @staticmethod
    def _infer_type(items: List[Dict], field: str) -> str:
        """推断字段类型：numeric / text / boolean / mixed / unknown

        Args:
            items: 数据列表
            field: 字段名

        Returns:
            类型标记
        """
        seen_types = set()
        for item in items:
            value = item.get(field)
            if value is None or value == "":
                continue
            if isinstance(value, bool):
                seen_types.add("boolean")
            elif isinstance(value, (int, float)):
                seen_types.add("numeric")
            elif isinstance(value, str):
                seen_types.add("text")

        if not seen_types:
            return "unknown"
        if len(seen_types) == 1:
            return seen_types.pop()
        return "mixed"

    def detect_field_features(self, items: List[Dict]) -> List[FeatureInfo]:
        """检测显式字段特征

        Args:
            items: 数据列表

        Returns:
            特征信息列表（按字段名排序）
        """
        if not items:
            return []

        all_fields: set = set()
        for item in items:
            all_fields.update(item.keys())

        features: List[FeatureInfo] = []
        for field in sorted(all_fields):
            coverage = self._field_coverage(items, field)
            unique = self._unique_ratio(items, field)
            ftype = self._infer_type(items, field)
            description = (
                "稀疏特征" if coverage < self.min_coverage else "常规特征"
            )
            features.append(FeatureInfo(
                name=field,
                feature_type=ftype,
                coverage=coverage,
                unique_ratio=unique,
                description=description,
            ))
        return features

    def detect_intent_distribution(self, items: List[Dict]) -> Dict[str, int]:
        """基于启发式关键词统计意图分布

        一条数据可命中多个意图，因此各意图计数之和可能超过样本数。

        Args:
            items: 数据列表

        Returns:
            意图 -> 计数
        """
        distribution: Dict[str, int] = {}
        for item in items:
            text = str(item.get(self.text_field, ""))
            for intent, keywords in INTENT_KEYWORDS.items():
                if any(kw in text for kw in keywords):
                    distribution[intent] = distribution.get(intent, 0) + 1
        return distribution

    def build_feature_matrix(self,
                             items: List[Dict],
                             fields: Optional[List[str]] = None) -> List[Dict]:
        """构建特征矩阵（每条样本一行，缺失值填 None）

        Args:
            items: 数据列表
            fields: 指定字段列表；缺省时使用全部并集字段

        Returns:
            特征行列表
        """
        if not items:
            return []

        if fields is None:
            field_set: set = set()
            for item in items:
                field_set.update(item.keys())
            fields = sorted(field_set)

        return [
            {name: item.get(name) for name in fields}
            for item in items
        ]

    def detect(self, items: List[Dict]) -> Dict[str, Any]:
        """全量特征检测

        Args:
            items: 数据列表

        Returns:
            特征检测报表
        """
        features = self.detect_field_features(items)
        intent_distribution = self.detect_intent_distribution(items)

        return {
            "total_items": len(items),
            "field_features": [f.to_dict() for f in features],
            "intent_distribution": intent_distribution,
            "sparse_fields": [f.name for f in features if f.coverage < self.min_coverage],
        }


def detect_features(items: List[Dict],
                   text_field: str = "instruction",
                   min_coverage: float = 0.1) -> Dict[str, Any]:
    """便捷函数：执行特征检测

    Args:
        items: 数据列表
        text_field: 文本字段名
        min_coverage: 稀疏特征判定阈值

    Returns:
        特征检测报表
    """
    return FeatureDetector(text_field=text_field, min_coverage=min_coverage).detect(items)


__all__ = [
    "FeatureDetector",
    "FeatureInfo",
    "INTENT_KEYWORDS",
    "detect_features",
]
