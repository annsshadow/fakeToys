"""数据增强效果评估器

对比增强前后数据集的指标变化，量化增强收益（规模增长、多样性提升、
重复率下降等），输出可用于实验报告的增益字典。
"""

import logging
from dataclasses import dataclass, field
from typing import Any, Dict, List

logger = logging.getLogger(__name__)


@dataclass
class AugmentationMetrics:
    """单侧数据集度量"""
    total_items: int
    unique_instructions: int
    avg_length: float
    length_std: float
    duplicate_rate: float
    extra: Dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "total_items": self.total_items,
            "unique_instructions": self.unique_instructions,
            "avg_length": self.avg_length,
            "length_std": self.length_std,
            "duplicate_rate": self.duplicate_rate,
            "extra": self.extra,
        }


@dataclass
class AugmentationImpact:
    """增强前后对比结果"""
    before: Dict[str, Any]
    after: Dict[str, Any]
    gains: Dict[str, float]

    def to_dict(self) -> Dict[str, Any]:
        return {
            "before": self.before,
            "after": self.after,
            "gains": self.gains,
        }


class ImpactEvaluator:
    """增强效果评估器"""

    def __init__(self, text_field: str = "instruction"):
        """初始化评估器

        Args:
            text_field: 用于计算长度/去重的字段
        """
        self.text_field = text_field

    def measure(self, items: List[Dict]) -> AugmentationMetrics:
        """计算数据集度量

        Args:
            items: 数据列表

        Returns:
            度量结果
        """
        if not items:
            return AugmentationMetrics(
                total_items=0,
                unique_instructions=0,
                avg_length=0.0,
                length_std=0.0,
                duplicate_rate=0.0,
            )

        texts = [str(item.get(self.text_field, "")) for item in items]
        lengths = [len(text) for text in texts]
        unique = set(texts)
        duplicates = sum(1 for t in texts if texts.count(t) > 1)

        avg = sum(lengths) / len(lengths)
        variance = sum((l - avg) ** 2 for l in lengths) / len(lengths)

        return AugmentationMetrics(
            total_items=len(items),
            unique_instructions=len(unique),
            avg_length=avg,
            length_std=variance ** 0.5,
            duplicate_rate=duplicates / len(items),
        )

    def evaluate(self,
                 before: List[Dict],
                 after: List[Dict]) -> AugmentationImpact:
        """对比增强前后并计算增益

        增益定义：
        - scale_gain: 规模增长率
        - diversity_gain: 唯一文本数增长率
        - dedup_gain: 重复率下降幅度（before - after）
        - length_spread_gain: 长度标准差提升（反映多样性覆盖）

        Args:
            before: 增强前数据
            after: 增强后数据

        Returns:
            对比结果
        """
        m_before = self.measure(before)
        m_after = self.measure(after)

        scale_gain = 0.0
        if m_before.total_items > 0:
            scale_gain = (m_after.total_items - m_before.total_items) / m_before.total_items

        diversity_gain = 0.0
        if m_before.unique_instructions > 0:
            diversity_gain = (
                (m_after.unique_instructions - m_before.unique_instructions)
                / m_before.unique_instructions
            )

        dedup_gain = m_before.duplicate_rate - m_after.duplicate_rate
        spread_gain = m_after.length_std - m_before.length_std

        return AugmentationImpact(
            before=m_before.to_dict(),
            after=m_after.to_dict(),
            gains={
                "scale_gain": scale_gain,
                "diversity_gain": diversity_gain,
                "dedup_gain": dedup_gain,
                "length_spread_gain": spread_gain,
            },
        )

    def is_beneficial(self, impact: AugmentationImpact,
                      min_scale_gain: float = 0.0) -> bool:
        """判断增强是否带来收益：规模不缩水且质量不恶化

        Args:
            impact: 对比结果
            min_scale_gain: 最低规模增长率

        Returns:
            是否有益
        """
        if impact.gains["scale_gain"] < min_scale_gain:
            return False
        # 重复率不应显著上升（容忍 -0.05）
        if impact.gains["dedup_gain"] < -0.05:
            return False
        return True


def evaluate_augmentation(before: List[Dict],
                          after: List[Dict],
                          text_field: str = "instruction") -> Dict[str, Any]:
    """便捷函数：评估增强效果

    Args:
        before: 增强前数据
        after: 增强后数据
        text_field: 文本字段名

    Returns:
        对比结果字典
    """
    return ImpactEvaluator(text_field=text_field).evaluate(before, after).to_dict()


__all__ = [
    "ImpactEvaluator",
    "AugmentationMetrics",
    "AugmentationImpact",
    "evaluate_augmentation",
]
