# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""质量报告模块"""

import logging
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any

logger = logging.getLogger(__name__)

DEFAULT_BUCKETS = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]


@dataclass
class QualityReport:
    total_samples: int = 0
    passed_samples: int = 0
    filtered_samples: int = 0
    pass_rate: float = 0.0
    score_distribution: Dict[str, float] = field(default_factory=dict)
    filter_statistics: Dict[str, int] = field(default_factory=dict)
    metric_summary: Dict[str, Dict[str, float]] = field(default_factory=dict)
    dedup_summary: Dict[str, Any] = field(default_factory=dict)
    improvement_suggestions: List[str] = field(default_factory=list)
    charts: Dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "total_samples": self.total_samples,
            "passed_samples": self.passed_samples,
            "filtered_samples": self.filtered_samples,
            "pass_rate": self.pass_rate,
            "score_distribution": self.score_distribution,
            "filter_statistics": self.filter_statistics,
            "metric_summary": self.metric_summary,
            "dedup_summary": self.dedup_summary,
            "improvement_suggestions": self.improvement_suggestions,
            "charts": self.charts,
        }

    def to_markdown(self) -> str:
        lines = [
            "# 数据质量报告",
            "",
            "## 总体概况",
            "",
            f"- 样本总数: {self.total_samples}",
            f"- 通过数: {self.passed_samples}",
            f"- 过滤数: {self.filtered_samples}",
            f"- 通过率: {self.pass_rate:.2%}",
            "",
            "## 改进建议",
            "",
        ]
        for s in self.improvement_suggestions:
            lines.append(f"- {s}")
        return "\n".join(lines)


class ReportGenerator:
    def __init__(self, buckets: Optional[List[float]] = None, threshold: float = 0.6):
        self.buckets = buckets or DEFAULT_BUCKETS
        self.threshold = threshold

    def generate(self, items: List[Dict], scores: List[Any],
                 dedup_summary: Optional[Dict[str, Any]] = None) -> QualityReport:
        if len(items) != len(scores):
            raise ValueError("items和scores长度不一致")

        total = len(items)
        if total == 0:
            return QualityReport()

        passed = sum(1 for s in scores if getattr(s, "passed", False))
        filtered = total - passed

        score_values = [getattr(s, "total_score", 0.0) for s in scores]

        distribution = self._compute_distribution(score_values)
        metric_summary = self._compute_metric_summary(scores)
        suggestions = self._generate_suggestions(passed, total, scores, dedup_summary or {})
        charts = self._build_charts(score_values, metric_summary)

        return QualityReport(
            total_samples=total,
            passed_samples=passed,
            filtered_samples=filtered,
            pass_rate=passed / total,
            score_distribution=distribution,
            metric_summary=metric_summary,
            dedup_summary=dedup_summary or {},
            improvement_suggestions=suggestions,
            charts=charts,
        )

    def _compute_distribution(self, values: List[float]) -> Dict[str, float]:
        n = len(values)
        if n == 0:
            return {}
        buckets = {}
        for i in range(len(self.buckets) - 1):
            low, high = self.buckets[i], self.buckets[i + 1]
            label = f"{low:.1f}-{high:.1f}"
            count = sum(1 for v in values if low <= v < high or (high == self.buckets[-1] and v == high))
            buckets[label] = count / n
        return buckets

    def _compute_metric_summary(self, scores: List[Any]) -> Dict[str, Dict[str, float]]:
        metrics = ["semantic_similarity", "relevance", "diversity", "total_score"]
        summary = {}
        for metric in metrics:
            values = [getattr(s, metric, 0.0) for s in scores]
            if values:
                summary[metric] = {
                    "mean": sum(values) / len(values),
                    "min": min(values),
                    "max": max(values),
                }
        return summary

    def _generate_suggestions(self, passed: int, total: int,
                              scores: List[Any], dedup_summary: Dict) -> List[str]:
        suggestions = []
        pass_rate = passed / total if total else 0.0

        if pass_rate < 0.5:
            suggestions.append("通过率偏低，建议放宽过滤阈值以保留更多有效样本")
        elif pass_rate > 0.95:
            suggestions.append("通过率过高，建议适当提高阈值以提升数据质量")

        if dedup_summary.get("removal_rate", 0) > 0.5:
            suggestions.append("去重移除比例较高，种子数据同质化严重，建议增加多样性")

        metrics = ["semantic_similarity", "relevance", "diversity"]
        labels = ["语义相似度", "回答相关性", "多样性"]
        for metric, label in zip(metrics, labels):
            values = [getattr(s, metric, 0.0) for s in scores]
            if values and sum(values) / len(values) < 0.5:
                if metric == "semantic_similarity":
                    suggestions.append(f"{label}偏低，建议收敛生成约束或优化prompt")
                elif metric == "relevance":
                    suggestions.append(f"{label}偏低，建议检查问答匹配质量")
                elif metric == "diversity":
                    suggestions.append(f"{label}偏低，建议提高temperature或增加采样多样性")

        if not suggestions:
            suggestions.append("各项指标均在合理区间，数据质量良好")

        return suggestions

    def _build_charts(self, values: List[float], metric_summary: Dict) -> Dict[str, Any]:
        histogram = {}
        for i in range(len(self.buckets) - 1):
            low, high = self.buckets[i], self.buckets[i + 1]
            label = f"{low:.1f}-{high:.1f}"
            histogram[label] = sum(1 for v in values if low <= v < high or (high == self.buckets[-1] and v == high))

        radar_labels = list(metric_summary.keys())
        radar_values = [metric_summary[m]["mean"] for m in radar_labels]

        return {
            "score_histogram": histogram,
            "metric_radar": {
                "labels": radar_labels,
                "values": radar_values,
            },
        }


def build_report(items: List[Dict], scores: List[Any]) -> Dict[str, Any]:
    generator = ReportGenerator()
    report = generator.generate(items, scores)
    return report.to_dict()
