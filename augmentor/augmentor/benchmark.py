# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据质量基准模块

提供标准化的质量评估指标、基准存档与对比能力。
"""

import json
import logging
from datetime import datetime
from pathlib import Path
from typing import List, Dict, Any, Optional
from .exceptions import QualityError

logger = logging.getLogger(__name__)

# 基准支持的指标
SUPPORTED_METRICS = ["pass_rate", "avg_total_score", "diversity", "duplication_rate"]

# 指标方向：True 表示越大越好
METRIC_DIRECTIONS = {
    "pass_rate": True,
    "avg_total_score": True,
    "diversity": True,
    "duplication_rate": False
}


class QualityBenchmark:
    """数据质量基准"""

    def __init__(self,
                 metrics: Optional[List[str]] = None,
                 baseline_file: Optional[str] = None,
                 threshold: float = 0.6,
                 scorer: Optional[Any] = None,
                 deduplicator: Optional[Any] = None):
        """初始化质量基准

        Args:
            metrics: 参与基准的指标列表
            baseline_file: 基准文件路径
            threshold: 质量阈值
            scorer: 质量评分器，为 None 时惰性创建
            deduplicator: 去重器，为 None 时惰性创建
        """
        self.metrics = metrics or list(SUPPORTED_METRICS)

        unsupported = [m for m in self.metrics if m not in SUPPORTED_METRICS]
        if unsupported:
            raise QualityError(
                f"不支持的基准指标: {unsupported}。支持: {SUPPORTED_METRICS}"
            )

        self.baseline_file = Path(baseline_file) if baseline_file else None
        self.threshold = threshold
        self._scorer = scorer
        self._deduplicator = deduplicator

    def _get_scorer(self):
        """惰性获取质量评分器

        Returns:
            QualityScorer 实例
        """
        if self._scorer is None:
            from .quality import QualityScorer
            self._scorer = QualityScorer(threshold=self.threshold)
        return self._scorer

    def _get_deduplicator(self):
        """惰性获取去重器

        Returns:
            Deduplicator 实例
        """
        if self._deduplicator is None:
            from .dedup import Deduplicator
            self._deduplicator = Deduplicator()
        return self._deduplicator

    def _compute_duplication_rate(self, data: List[Dict]) -> float:
        """计算重复率

        Args:
            data: 数据列表

        Returns:
            重复率 (0-1)
        """
        if len(data) < 2:
            return 0.0

        report = self._get_deduplicator().generate_report(data)
        return float(report.get("removal_rate", 0.0))

    def run_benchmark(self, data: List[Dict]) -> Dict[str, Any]:
        """运行基准测试

        Args:
            data: 数据列表

        Returns:
            基准结果字典
        """
        if not data:
            return {
                "sample_count": 0,
                "metrics": {m: 0.0 for m in self.metrics},
                "threshold": self.threshold,
                "timestamp": datetime.now().isoformat()
            }

        items = [
            {
                "original": item.get("instruction", ""),
                "generated": item.get("instruction", ""),
                "output": item.get("output", "")
            }
            for item in data
        ]

        scores = self._get_scorer().batch_score(items)

        total_scores = [float(s.total_score) for s in scores]
        diversity_scores = [float(s.diversity) for s in scores]
        passed = sum(1 for s in scores if s.passed)

        computed: Dict[str, float] = {
            "pass_rate": passed / len(scores),
            "avg_total_score": sum(total_scores) / len(total_scores),
            "diversity": sum(diversity_scores) / len(diversity_scores),
            "duplication_rate": self._compute_duplication_rate(data)
        }

        result = {
            "sample_count": len(data),
            "metrics": {m: computed[m] for m in self.metrics},
            "all_metrics": computed,
            "threshold": self.threshold,
            "timestamp": datetime.now().isoformat()
        }

        logger.info(f"基准测试完成: {len(data)} 条样本，指标 {result['metrics']}")
        return result

    def save_baseline(self, results: Dict[str, Any]) -> str:
        """保存基准结果

        Args:
            results: 基准结果

        Returns:
            基准文件路径

        Raises:
            QualityError: 未配置 baseline_file
        """
        if self.baseline_file is None:
            raise QualityError("未配置 baseline_file，无法保存基准")

        self.baseline_file.parent.mkdir(parents=True, exist_ok=True)
        with open(self.baseline_file, 'w', encoding='utf-8') as f:
            json.dump(results, f, ensure_ascii=False, indent=2)

        logger.info(f"基准已保存到 {self.baseline_file}")
        return str(self.baseline_file)

    def load_baseline(self) -> Optional[Dict[str, Any]]:
        """加载基准结果

        Returns:
            基准结果字典，不存在时返回 None
        """
        if self.baseline_file is None or not self.baseline_file.exists():
            return None

        with open(self.baseline_file, 'r', encoding='utf-8') as f:
            return json.load(f)

    def compare_with_baseline(self,
                              results: Dict[str, Any],
                              baseline: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """与基准结果对比

        Args:
            results: 当前结果
            baseline: 基准结果，为 None 时从文件加载

        Returns:
            对比结果

        Raises:
            QualityError: 基准不存在
        """
        baseline = baseline if baseline is not None else self.load_baseline()

        if not baseline:
            raise QualityError("基准不存在，请先运行并保存基准")

        current_metrics = results.get("metrics", {})
        baseline_metrics = baseline.get("metrics", {})

        comparisons: Dict[str, Dict[str, Any]] = {}
        improved, regressed, unchanged = [], [], []

        for metric in self.metrics:
            current = float(current_metrics.get(metric, 0.0))

            # 基准中未记录的指标不做对比，否则会被当成 0 造成虚假提升
            if metric not in baseline_metrics:
                comparisons[metric] = {
                    "current": current,
                    "baseline": None,
                    "delta": None,
                    "delta_ratio": None,
                    "status": "no_baseline"
                }
                continue

            reference = float(baseline_metrics[metric])
            delta = current - reference

            higher_is_better = METRIC_DIRECTIONS.get(metric, True)
            if abs(delta) < 1e-9:
                status = "unchanged"
                unchanged.append(metric)
            elif (delta > 0) == higher_is_better:
                status = "improved"
                improved.append(metric)
            else:
                status = "regressed"
                regressed.append(metric)

            comparisons[metric] = {
                "current": current,
                "baseline": reference,
                "delta": delta,
                "delta_ratio": (delta / reference) if reference else 0.0,
                "status": status
            }

        return {
            "baseline_timestamp": baseline.get("timestamp"),
            "current_timestamp": results.get("timestamp"),
            "comparisons": comparisons,
            "improved": improved,
            "regressed": regressed,
            "unchanged": unchanged,
            "overall": (
                "improved" if len(improved) > len(regressed)
                else "regressed" if len(regressed) > len(improved)
                else "unchanged"
            )
        }

    def generate_report(self, results: Dict[str, Any]) -> str:
        """生成 Markdown 格式的基准报告

        Args:
            results: 基准结果

        Returns:
            Markdown 报告
        """
        lines = [
            "# 数据质量基准报告",
            "",
            f"- 样本数量: {results.get('sample_count', 0)}",
            f"- 质量阈值: {results.get('threshold', self.threshold)}",
            f"- 生成时间: {results.get('timestamp', '')}",
            "",
            "## 指标结果",
            "",
            "| 指标 | 数值 | 方向 |",
            "|------|------|------|"
        ]

        metrics = results.get("metrics", {})
        for metric in self.metrics:
            value = metrics.get(metric, 0.0)
            direction = "越大越好" if METRIC_DIRECTIONS.get(metric, True) else "越小越好"
            lines.append(f"| {metric} | {value:.4f} | {direction} |")

        lines.append("")

        comparisons = results.get("comparisons")
        if comparisons:
            lines += [
                "## 与基准对比",
                "",
                "| 指标 | 当前 | 基准 | 变化 | 结论 |",
                "|------|------|------|------|------|"
            ]
            for metric, data in comparisons.items():
                if data.get("baseline") is None:
                    lines.append(
                        f"| {metric} | {data['current']:.4f} | - | - | no_baseline |"
                    )
                    continue
                lines.append(
                    f"| {metric} | {data['current']:.4f} | {data['baseline']:.4f} | "
                    f"{data['delta']:+.4f} | {data['status']} |"
                )
            lines.append("")

        return "\n".join(lines)
