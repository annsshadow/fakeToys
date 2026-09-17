"""数据集质量报告模块

提供数据集质量评估和报告生成功能。
"""

import json
import logging
from typing import List, Dict, Optional, Any
from dataclasses import dataclass, field
from pathlib import Path
from datetime import datetime

logger = logging.getLogger(__name__)


@dataclass
class QualityMetric:
    """质量指标"""
    name: str
    value: float
    threshold: float
    passed: bool
    description: str
    recommendation: str = ""
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "name": self.name,
            "value": self.value,
            "threshold": self.threshold,
            "passed": self.passed,
            "description": self.description,
            "recommendation": self.recommendation
        }


@dataclass
class QualityReport:
    """质量报告"""
    dataset_name: str
    timestamp: str
    total_items: int
    metrics: List[QualityMetric] = field(default_factory=list)
    overall_score: float = 0.0
    overall_passed: bool = False
    summary: str = ""
    recommendations: List[str] = field(default_factory=list)
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "dataset_name": self.dataset_name,
            "timestamp": self.timestamp,
            "total_items": self.total_items,
            "metrics": [m.to_dict() for m in self.metrics],
            "overall_score": self.overall_score,
            "overall_passed": self.overall_passed,
            "summary": self.summary,
            "recommendations": self.recommendations
        }
    
    def to_markdown(self) -> str:
        """转换为Markdown格式"""
        lines = [
            f"# 数据集质量报告",
            f"",
            f"**数据集名称**: {self.dataset_name}",
            f"**生成时间**: {self.timestamp}",
            f"**数据总量**: {self.total_items}",
            f"**总体评分**: {self.overall_score:.2f}",
            f"**总体状态**: {'通过' if self.overall_passed else '未通过'}",
            f"",
            f"## 质量指标",
            f"",
            "| 指标 | 分数 | 阈值 | 状态 | 说明 |",
            "|------|------|------|------|------|"
        ]
        
        for metric in self.metrics:
            status = "✅" if metric.passed else "❌"
            lines.append(f"| {metric.name} | {metric.value:.2f} | {metric.threshold:.2f} | {status} | {metric.description} |")
        
        if self.recommendations:
            lines.extend([
                f"",
                f"## 改进建议",
                f""
            ])
            for rec in self.recommendations:
                lines.append(f"- {rec}")
        
        return "\n".join(lines)


class QualityReporter:
    """质量报告生成器
    
    提供数据集质量评估和报告生成功能。
    """
    
    def __init__(self, threshold: float = 0.7):
        """初始化报告生成器
        
        Args:
            threshold: 质量阈值
        """
        self._threshold = threshold
    
    def generate_report(self, items: List[Dict], dataset_name: str = "unknown") -> QualityReport:
        """生成质量报告
        
        Args:
            items: 数据列表
            dataset_name: 数据集名称
        
        Returns:
            质量报告
        """
        report = QualityReport(
            dataset_name=dataset_name,
            timestamp=datetime.now().isoformat(),
            total_items=len(items)
        )
        
        # 计算各项指标
        metrics = []
        
        # 1. 完整性指标
        completeness = self._calculate_completeness(items)
        metrics.append(completeness)
        
        # 2. 一致性指标
        consistency = self._calculate_consistency(items)
        metrics.append(consistency)
        
        # 3. 多样性指标
        diversity = self._calculate_diversity(items)
        metrics.append(diversity)
        
        # 4. 长度分布指标
        length_distribution = self._calculate_length_distribution(items)
        metrics.append(length_distribution)
        
        # 5. 重复率指标
        duplication_rate = self._calculate_duplication_rate(items)
        metrics.append(duplication_rate)
        
        report.metrics = metrics
        
        # 计算总体分数
        total_score = sum(m.value for m in metrics) / len(metrics) if metrics else 0
        report.overall_score = total_score
        report.overall_passed = total_score >= self._threshold
        
        # 生成摘要和建议
        report.summary = self._generate_summary(report)
        report.recommendations = self._generate_recommendations(report)
        
        return report
    
    def _calculate_completeness(self, items: List[Dict]) -> QualityMetric:
        """计算完整性指标
        
        Args:
            items: 数据列表
        
        Returns:
            质量指标
        """
        required_fields = ["instruction", "output"]
        total_fields = len(required_fields) * len(items)
        filled_fields = sum(
            1 for item in items
            for field in required_fields
            if item.get(field)
        )
        
        score = filled_fields / total_fields if total_fields > 0 else 0
        
        return QualityMetric(
            name="完整性",
            value=score,
            threshold=self._threshold,
            passed=score >= self._threshold,
            description=f"字段填充率 {score:.1%}",
            recommendation="确保所有必填字段都有值"
        )
    
    def _calculate_consistency(self, items: List[Dict]) -> QualityMetric:
        """计算一致性指标
        
        Args:
            items: 数据列表
        
        Returns:
            质量指标
        """
        if not items:
            return QualityMetric(
                name="一致性",
                value=0,
                threshold=self._threshold,
                passed=False,
                description="数据为空"
            )
        
        # 检查问题和回答的一致性
        consistent_count = 0
        for item in items:
            instruction = item.get("instruction", "")
            output = item.get("output", "")
            
            if instruction and output:
                # 简单检查：问题和回答不应该完全相同
                if instruction != output:
                    consistent_count += 1
        
        score = consistent_count / len(items) if items else 0
        
        return QualityMetric(
            name="一致性",
            value=score,
            threshold=self._threshold,
            passed=score >= self._threshold,
            description=f"问答一致性 {score:.1%}",
            recommendation="确保问题和回答内容相关但不重复"
        )
    
    def _calculate_diversity(self, items: List[Dict]) -> QualityMetric:
        """计算多样性指标
        
        Args:
            items: 数据列表
        
        Returns:
            质量指标
        """
        if not items:
            return QualityMetric(
                name="多样性",
                value=0,
                threshold=self._threshold,
                passed=False,
                description="数据为空"
            )
        
        # 收集所有问题
        instructions = [item.get("instruction", "") for item in items if item.get("instruction")]
        
        if not instructions:
            return QualityMetric(
                name="多样性",
                value=0,
                threshold=self._threshold,
                passed=False,
                description="没有有效问题"
            )
        
        # 计算唯一问题比例
        unique_instructions = set(instructions)
        score = len(unique_instructions) / len(instructions)
        
        return QualityMetric(
            name="多样性",
            value=score,
            threshold=self._threshold,
            passed=score >= self._threshold,
            description=f"问题多样性 {score:.1%}",
            recommendation="增加问题的多样性，使用不同的表达方式"
        )
    
    def _calculate_length_distribution(self, items: List[Dict]) -> QualityMetric:
        """计算长度分布指标
        
        Args:
            items: 数据列表
        
        Returns:
            质量指标
        """
        if not items:
            return QualityMetric(
                name="长度分布",
                value=0,
                threshold=self._threshold,
                passed=False,
                description="数据为空"
            )
        
        # 计算问题长度
        lengths = [len(item.get("instruction", "")) for item in items if item.get("instruction")]
        
        if not lengths:
            return QualityMetric(
                name="长度分布",
                value=0,
                threshold=self._threshold,
                passed=False,
                description="没有有效问题"
            )
        
        # 计算长度标准差
        avg_length = sum(lengths) / len(lengths)
        variance = sum((l - avg_length) ** 2 for l in lengths) / len(lengths)
        std_dev = variance ** 0.5
        
        # 标准化分数（标准差越小越好）
        score = max(0, 1 - std_dev / avg_length) if avg_length > 0 else 0
        
        return QualityMetric(
            name="长度分布",
            value=score,
            threshold=self._threshold,
            passed=score >= self._threshold,
            description=f"长度分布均匀度 {score:.1%}",
            recommendation="保持问题长度在合理范围内"
        )
    
    def _calculate_duplication_rate(self, items: List[Dict]) -> QualityMetric:
        """计算重复率指标
        
        Args:
            items: 数据列表
        
        Returns:
            质量指标
        """
        if not items:
            return QualityMetric(
                name="重复率",
                value=1.0,
                threshold=self._threshold,
                passed=True,
                description="数据为空"
            )
        
        # 计算重复问题
        instructions = [item.get("instruction", "") for item in items if item.get("instruction")]
        
        if not instructions:
            return QualityMetric(
                name="重复率",
                value=1.0,
                threshold=self._threshold,
                passed=True,
                description="没有有效问题"
            )
        
        unique_instructions = set(instructions)
        score = len(unique_instructions) / len(instructions)
        
        return QualityMetric(
            name="重复率",
            value=score,
            threshold=self._threshold,
            passed=score >= self._threshold,
            description=f"唯一问题比例 {score:.1%}",
            recommendation="减少重复问题，增加数据多样性"
        )
    
    def _generate_summary(self, report: QualityReport) -> str:
        """生成摘要
        
        Args:
            report: 质量报告
        
        Returns:
            摘要文本
        """
        passed_count = sum(1 for m in report.metrics if m.passed)
        total_count = len(report.metrics)
        
        return f"数据集 {report.dataset_name} 共 {report.total_items} 条数据，" \
               f"总体评分 {report.overall_score:.2f}，" \
               f"{passed_count}/{total_count} 项指标达标。"
    
    def _generate_recommendations(self, report: QualityReport) -> List[str]:
        """生成改进建议
        
        Args:
            report: 质量报告
        
        Returns:
            建议列表
        """
        recommendations = []
        
        for metric in report.metrics:
            if not metric.passed and metric.recommendation:
                recommendations.append(f"[{metric.name}] {metric.recommendation}")
        
        if not recommendations:
            recommendations.append("数据集质量良好，无需特别改进。")
        
        return recommendations


def generate_quality_report(items: List[Dict], dataset_name: str = "unknown", 
                           threshold: float = 0.7) -> QualityReport:
    """生成质量报告
    
    Args:
        items: 数据列表
        dataset_name: 数据集名称
        threshold: 质量阈值
    
    Returns:
        质量报告
    """
    reporter = QualityReporter(threshold=threshold)
    return reporter.generate_report(items, dataset_name)


def save_quality_report(report: QualityReport, output_path: str, format: str = "json"):
    """保存质量报告
    
    Args:
        report: 质量报告
        output_path: 输出路径
        format: 输出格式 (json/markdown)
    """
    output = Path(output_path)
    output.parent.mkdir(parents=True, exist_ok=True)
    
    if format == "markdown":
        with open(output, 'w', encoding='utf-8') as f:
            f.write(report.to_markdown())
    else:
        with open(output, 'w', encoding='utf-8') as f:
            json.dump(report.to_dict(), f, ensure_ascii=False, indent=2)
