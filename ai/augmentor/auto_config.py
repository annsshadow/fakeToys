"""智能配置推荐模块

根据数据集画像与特征检测结果，推荐去重阈值、质量门禁、采样策略等
关键参数，让管线无需手工调参即可适配新数据。
"""

import logging
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional

logger = logging.getLogger(__name__)

# 推荐参数范围
DEDUP_THRESHOLD_RANGE = (0.85, 0.98)
QUALITY_THRESHOLD_RANGE = (0.5, 0.8)
SAMPLE_SIZE_MIN_RATIO = 0.1
SAMPLE_SIZE_MAX_RATIO = 0.5


@dataclass
class AutoConfigRecommendation:
    """自动配置推荐结果"""
    dedup_threshold: float
    quality_threshold: float
    recommended_sample_size: int
    reasoning: List[str] = field(default_factory=list)
    source_stats: Dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "dedup_threshold": self.dedup_threshold,
            "quality_threshold": self.quality_threshold,
            "recommended_sample_size": self.recommended_sample_size,
            "reasoning": self.reasoning,
            "source_stats": self.source_stats,
        }


class AutoConfig:
    """基于数据集统计的智能配置推荐器"""

    def __init__(self,
                 dedup_range: tuple = DEDUP_THRESHOLD_RANGE,
                 quality_range: tuple = QUALITY_THRESHOLD_RANGE,
                 sample_ratio_range: tuple = (SAMPLE_SIZE_MIN_RATIO,
                                              SAMPLE_SIZE_MAX_RATIO)):
        """初始化推荐器

        Args:
            dedup_range: 去重阈值允许范围
            quality_range: 质量门禁允许范围
            sample_ratio_range: 采样比例范围
        """
        if dedup_range[0] >= dedup_range[1]:
            raise ValueError("dedup_range 下界必须小于上界")
        if quality_range[0] >= quality_range[1]:
            raise ValueError("quality_range 下界必须小于上界")

        self.dedup_range = dedup_range
        self.quality_range = quality_range
        self.sample_ratio_range = sample_ratio_range

    def recommend(self,
                  dataset_stats: Dict[str, Any],
                  total_items: int) -> AutoConfigRecommendation:
        """根据数据集统计推荐配置

        Args:
            dataset_stats: 包含 duplicate_rate, quality_pass_rate,
                avg_length, language_distribution 等键的字典
            total_items: 数据集总条数

        Returns:
            推荐配置
        """
        if total_items <= 0:
            raise ValueError("total_items 必须为正数")

        reasoning: List[str] = []

        duplicate_rate = float(dataset_stats.get("duplicate_rate", 0.0))
        pass_rate = float(dataset_stats.get("quality_pass_rate", 1.0))

        # 去重阈值：重复率高则提高阈值，避免误删相似样本
        dedup_threshold = self._pick_dedup_threshold(duplicate_rate)
        if duplicate_rate > 0.3:
            reasoning.append(f"重复率 {duplicate_rate:.2f} 偏高，去重阈值上调至 {dedup_threshold:.2f}")
        elif duplicate_rate < 0.05:
            reasoning.append(f"重复率 {duplicate_rate:.2f} 较低，去重阈值下调至 {dedup_threshold:.2f}")

        # 质量门禁：通过率低则放宽阈值
        quality_threshold = self._pick_quality_threshold(pass_rate)
        if pass_rate < 0.5:
            reasoning.append(f"质量通过率 {pass_rate:.2f} 偏低，质量门禁放宽至 {quality_threshold:.2f}")

        # 采样比例：长度方差越大，需要的样本越多
        sample_size = self._recommend_sample_size(dataset_stats, total_items)

        return AutoConfigRecommendation(
            dedup_threshold=dedup_threshold,
            quality_threshold=quality_threshold,
            recommended_sample_size=sample_size,
            reasoning=reasoning,
            source_stats={k: dataset_stats.get(k) for k in (
                "duplicate_rate", "quality_pass_rate", "total_items"
            )},
        )

    def _pick_dedup_threshold(self, duplicate_rate: float) -> float:
        """按重复率线性插值去重阈值

        Args:
            duplicate_rate: 重复率 (0-1)

        Returns:
            去重阈值
        """
        lo, hi = self.dedup_range
        if duplicate_rate >= 0.3:
            return round(hi, 2)
        if duplicate_rate <= 0.05:
            return round(lo, 2)
        # 0.05 -> 0.3 线性映射到 [lo, hi]
        ratio = (duplicate_rate - 0.05) / 0.25
        return round(lo + ratio * (hi - lo), 2)

    def _pick_quality_threshold(self, pass_rate: float) -> float:
        """按通过率推荐质量门禁

        Args:
            pass_rate: 当前质量通过率

        Returns:
            质量门禁阈值
        """
        lo, hi = self.quality_range
        if pass_rate < 0.4:
            return round(lo, 2)
        if pass_rate > 0.9:
            return round(hi, 2)
        # 0.4 -> 0.9 线性映射到 [lo, hi]
        ratio = (pass_rate - 0.4) / 0.5
        ratio = min(max(ratio, 0.0), 1.0)
        return round(lo + ratio * (hi - lo), 2)

    def _recommend_sample_size(self,
                               dataset_stats: Dict[str, Any],
                               total_items: int) -> int:
        """推荐采样数量

        Args:
            dataset_stats: 统计字典
            total_items: 总条数

        Returns:
            推荐采样数（至少 1）
        """
        min_ratio, max_ratio = self.sample_ratio_range

        length_stats = dataset_stats.get("length_stats", {})
        avg_len = float(length_stats.get("avg", 0))
        max_len = float(length_stats.get("max", 0))

        # 长度越不均衡，采样比例越高
        spread = (max_len / avg_len) if avg_len > 0 else 1.0
        if spread > 10:
            ratio = max_ratio
        elif spread > 3:
            ratio = (min_ratio + max_ratio) / 2
        else:
            ratio = min_ratio

        # 语言多样性加分
        language_distribution = dataset_stats.get("language_distribution", {})
        languages = sum(1 for v in language_distribution.values() if v > 0)
        if languages > 2:
            ratio = min(ratio + 0.05, max_ratio)

        sample_size = int(total_items * ratio)
        return max(1, sample_size)

    def validate_recommendation(self,
                                recommendation: AutoConfigRecommendation) -> List[str]:
        """校验推荐参数是否在允许范围内

        Args:
            recommendation: 推荐配置

        Returns:
            校验问题列表（空列表表示全部合规）
        """
        problems: List[str] = []
        lo, hi = self.dedup_range
        if not (lo <= recommendation.dedup_threshold <= hi):
            problems.append(
                f"去重阈值 {recommendation.dedup_threshold} 超出范围 [{lo}, {hi}]"
            )

        lo_q, hi_q = self.quality_range
        if not (lo_q <= recommendation.quality_threshold <= hi_q):
            problems.append(
                f"质量阈值 {recommendation.quality_threshold} 超出范围 [{lo_q}, {hi_q}]"
            )

        if recommendation.recommended_sample_size < 1:
            problems.append("采样数量不能小于 1")

        return problems


def auto_recommend(dataset_stats: Dict[str, Any],
                   total_items: int) -> Dict[str, Any]:
    """便捷函数：推荐配置并返回字典形式

    Args:
        dataset_stats: 统计字典
        total_items: 总条数

    Returns:
        推荐配置字典
    """
    return AutoConfig().recommend(dataset_stats, total_items).to_dict()


__all__ = [
    "AutoConfig",
    "AutoConfigRecommendation",
    "auto_recommend",
    "DEDUP_THRESHOLD_RANGE",
    "QUALITY_THRESHOLD_RANGE",
]
