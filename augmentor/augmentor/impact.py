# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""Data augmentation impact evaluator"""

import logging
from collections import Counter
from dataclasses import dataclass, field
from typing import Any, Dict, List

logger = logging.getLogger(__name__)


@dataclass
class AugmentationMetrics:
    """Single-side dataset metrics"""
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
    """Before/after comparison result"""
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
    """Augmentation effect evaluator"""

    def __init__(self, text_field: str = "instruction"):
        self.text_field = text_field

    def measure(self, items: List[Dict]) -> AugmentationMetrics:
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
        # 一次计数，而不是每条一次 `texts.count()`：后者把整体变成 O(n²)
        # （实测 n=1000/2000/4000 = 6.0/24.3/99.7 ms，每翻倍一次 ×4）。
        counts = Counter(texts)
        duplicates = sum(c for c in counts.values() if c > 1)
        avg = sum(lengths) / len(lengths)
        variance = sum((l - avg) ** 2 for l in lengths) / len(lengths)
        return AugmentationMetrics(
            total_items=len(items),
            unique_instructions=len(unique),
            avg_length=avg,
            length_std=variance ** 0.5,
            duplicate_rate=duplicates / len(items),
        )

    def evaluate(self, before: List[Dict], after: List[Dict]) -> AugmentationImpact:
        m_before = self.measure(before)
        m_after = self.measure(after)
        scale_gain = (m_after.total_items - m_before.total_items) / m_before.total_items if m_before.total_items > 0 else 0.0
        diversity_gain = ((m_after.unique_instructions - m_before.unique_instructions) / m_before.unique_instructions) if m_before.unique_instructions > 0 else 0.0
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

    def is_beneficial(self, impact: AugmentationImpact, min_scale_gain: float = 0.0) -> bool:
        if impact.gains.get("scale_gain", 0.0) < min_scale_gain:
            return False
        if impact.gains.get("dedup_gain", 0.0) < -0.05:
            return False
        return True


def evaluate_augmentation(before: List[Dict], after: List[Dict], text_field: str = "instruction") -> Dict[str, Any]:
    return ImpactEvaluator(text_field=text_field).evaluate(before, after).to_dict()


__all__ = [
    "ImpactEvaluator",
    "AugmentationMetrics",
    "AugmentationImpact",
    "evaluate_augmentation",
]
