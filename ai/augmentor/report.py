"""Report generation module"""

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
        lines = ["# Quality Report", ""]
        lines += [
            f"- Samples: {self.total_samples}",
            f"- Passed: {self.passed_samples}",
            f"- Pass rate: {self.pass_rate:.2%}",
        ]
        return "\n".join(lines)


class ReportGenerator:
    def __init__(self, buckets: Optional[List[float]] = None, threshold: float = 0.6):
        self.buckets = buckets or DEFAULT_BUCKETS
        self.threshold = threshold

    def generate(self, items: List[Dict], scores: List[Any], dedup_summary: Optional[Dict[str, Any]] = None) -> QualityReport:
        total = len(items)
        passed = sum(1 for s in scores if getattr(s, "passed", False))
        return QualityReport(
            total_samples=total,
            passed_samples=passed,
            pass_rate=passed / total if total else 0.0,
        )


def build_report(items: List[Dict], scores: List[Any]) -> Dict[str, Any]:
    generator = ReportGenerator()
    report = generator.generate(items, scores)
    return report.to_dict()
