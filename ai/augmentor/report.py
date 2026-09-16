"""质量报告生成模块

聚合质量评分、去重统计，产出结构化报告与图表数据。
"""

import logging
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any

logger = logging.getLogger(__name__)

# 默认评分直方图分桶边界
DEFAULT_BUCKETS = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]


@dataclass
class QualityReport:
    """质量报告"""
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
        """转换为字典

        Returns:
            报告字典
        """
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
            "charts": self.charts
        }

    def to_markdown(self) -> str:
        """渲染为 Markdown 文本

        Returns:
            Markdown 报告
        """
        lines = [
            "# 数据质量报告",
            "",
            "## 总体概况",
            "",
            f"- 样本总数: {self.total_samples}",
            f"- 通过样本: {self.passed_samples}",
            f"- 过滤样本: {self.filtered_samples}",
            f"- 通过率: {self.pass_rate:.2%}",
            ""
        ]

        if self.metric_summary:
            lines += ["## 指标统计", "", "| 指标 | 均值 | 标准差 | 最小值 | 最大值 |",
                      "|------|------|--------|--------|--------|"]
            for name, stats in self.metric_summary.items():
                lines.append(
                    f"| {name} | {stats.get('mean', 0):.3f} | {stats.get('std', 0):.3f} | "
                    f"{stats.get('min', 0):.3f} | {stats.get('max', 0):.3f} |"
                )
            lines.append("")

        if self.score_distribution:
            lines += ["## 评分分布", ""]
            for bucket, ratio in self.score_distribution.items():
                lines.append(f"- {bucket}: {ratio:.2%}")
            lines.append("")

        if self.dedup_summary:
            lines += [
                "## 去重统计", "",
                f"- 原始数量: {self.dedup_summary.get('original_count', 0)}",
                f"- 去重后数量: {self.dedup_summary.get('deduplicated_count', 0)}",
                f"- 移除数量: {self.dedup_summary.get('removed_count', 0)}",
                f"- 移除比例: {self.dedup_summary.get('removal_rate', 0):.2%}",
                ""
            ]

        if self.improvement_suggestions:
            lines += ["## 改进建议", ""]
            lines += [f"- {s}" for s in self.improvement_suggestions]
            lines.append("")

        return "\n".join(lines)


class ReportGenerator:
    """质量报告生成器"""

    def __init__(self,
                 buckets: Optional[List[float]] = None,
                 threshold: float = 0.6):
        """初始化报告生成器

        Args:
            buckets: 评分分桶边界
            threshold: 质量阈值，用于生成建议
        """
        self.buckets = buckets or DEFAULT_BUCKETS
        self.threshold = threshold

    def _build_distribution(self, scores: List[float]) -> Dict[str, float]:
        """构建评分分布

        Args:
            scores: 评分列表

        Returns:
            分桶到占比的映射
        """
        total = len(scores)
        distribution: Dict[str, float] = {}

        for i in range(len(self.buckets) - 1):
            low, high = self.buckets[i], self.buckets[i + 1]
            label = f"{low:.1f}-{high:.1f}"
            if i == len(self.buckets) - 2:
                count = sum(1 for s in scores if low <= s <= high)
            else:
                count = sum(1 for s in scores if low <= s < high)
            distribution[label] = count / total if total else 0.0

        return distribution

    def _build_suggestions(self,
                           pass_rate: float,
                           metric_summary: Dict[str, Dict[str, float]],
                           dedup_summary: Dict[str, Any]) -> List[str]:
        """基于统计结果生成改进建议

        Args:
            pass_rate: 通过率
            metric_summary: 指标统计
            dedup_summary: 去重统计

        Returns:
            建议列表
        """
        suggestions: List[str] = []

        if pass_rate < 0.3:
            suggestions.append(
                f"通过率偏低（{pass_rate:.1%}），建议放宽质量阈值或优化增强提示词"
            )
        elif pass_rate > 0.95:
            suggestions.append(
                f"通过率过高（{pass_rate:.1%}），阈值可能过松，建议提高阈值以提升数据质量"
            )

        semantic = metric_summary.get("semantic_similarity", {})
        if semantic and semantic.get("mean", 1.0) < 0.7:
            suggestions.append(
                "语义相似度均值偏低，增强结果与原意偏离较大，建议收敛生成约束"
            )

        relevance = metric_summary.get("relevance", {})
        if relevance and relevance.get("mean", 1.0) < 0.7:
            suggestions.append(
                "回答相关性均值偏低，建议检查问答对是否匹配或补充答案改写"
            )

        diversity = metric_summary.get("diversity", {})
        if diversity and diversity.get("mean", 1.0) < 0.3:
            suggestions.append(
                "多样性均值偏低，生成结果同质化严重，建议提高 temperature 或增加种子覆盖"
            )

        removal_rate = dedup_summary.get("removal_rate", 0.0)
        if removal_rate > 0.3:
            suggestions.append(
                f"去重移除比例较高（{removal_rate:.1%}），建议降低相似度阈值或增加种子多样性"
            )

        if not suggestions:
            suggestions.append("各项指标均在合理区间，数据质量良好")

        return suggestions

    def generate(self,
                 items: List[Dict],
                 scores: List[Any],
                 dedup_summary: Optional[Dict[str, Any]] = None) -> QualityReport:
        """生成质量报告

        Args:
            items: 数据列表
            scores: QualityScore 列表，长度需与 items 一致
            dedup_summary: 去重统计字典（可选）

        Returns:
            QualityReport 实例
        """
        if len(items) != len(scores):
            raise ValueError("items 与 scores 长度不一致")

        total = len(items)
        passed = sum(1 for s in scores if getattr(s, "passed", False))
        total_scores = [float(getattr(s, "total_score", 0.0)) for s in scores]

        metric_names = ["total_score", "semantic_similarity", "relevance", "diversity"]
        metric_summary: Dict[str, Dict[str, float]] = {}

        for name in metric_names:
            values = [float(getattr(s, name, 0.0)) for s in scores]
            if not values:
                continue
            mean = sum(values) / len(values)
            variance = sum((v - mean) ** 2 for v in values) / len(values)
            metric_summary[name] = {
                "mean": mean,
                "std": variance ** 0.5,
                "min": min(values),
                "max": max(values)
            }

        distribution = self._build_distribution(total_scores)
        dedup_summary = dedup_summary or {}

        filter_statistics = {
            "passed": passed,
            "filtered": total - passed,
            "below_threshold": total - passed
        }

        report = QualityReport(
            total_samples=total,
            passed_samples=passed,
            filtered_samples=total - passed,
            pass_rate=passed / total if total else 0.0,
            score_distribution=distribution,
            filter_statistics=filter_statistics,
            metric_summary=metric_summary,
            dedup_summary=dedup_summary,
            improvement_suggestions=self._build_suggestions(
                passed / total if total else 0.0, metric_summary, dedup_summary
            ),
            charts={
                "score_histogram": {
                    "labels": list(distribution.keys()),
                    "values": [round(v, 4) for v in distribution.values()]
                },
                "metric_radar": {
                    "labels": [n for n in metric_names if n in metric_summary],
                    "values": [
                        round(metric_summary[n]["mean"], 4)
                        for n in metric_names if n in metric_summary
                    ]
                }
            }
        )

        logger.info(f"生成质量报告: {total} 条样本，通过率 {report.pass_rate:.2%}")
        return report
