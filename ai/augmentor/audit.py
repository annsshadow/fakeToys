"""数据集就绪审计模块

在数据交付/微调前做一次性体检，聚合若干高价值信号（不与质量评分重叠）：
- PII 暴露量（隐私风险）
- 集内重复率（数据冗余）
- 空字段率（数据完整性）
- 与参考集的泄漏率（train/test 交叉污染，可选）

输出单一可读的审计报告与总体就绪判定，供 CLI/API 直接消费。
"""

import logging
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional

from .dedup import Deduplicator
from .leakage import detect_leakage
from .privacy import PiiSanitizer

logger = logging.getLogger(__name__)


@dataclass
class AuditReport:
    """数据集审计报告"""

    total_items: int
    pii_matches: int = 0
    duplicate_count: int = 0
    duplicate_rate: float = 0.0
    empty_field_rate: float = 0.0
    leak_count: Optional[int] = None
    leak_rate: Optional[float] = None
    findings: List[str] = field(default_factory=list)
    ready: bool = True

    def to_dict(self) -> Dict[str, Any]:
        return {
            "total_items": self.total_items,
            "pii_matches": self.pii_matches,
            "duplicate_count": self.duplicate_count,
            "duplicate_rate": self.duplicate_rate,
            "empty_field_rate": self.empty_field_rate,
            "leak_count": self.leak_count,
            "leak_rate": self.leak_rate,
            "findings": self.findings,
            "ready": self.ready,
        }


class DatasetAuditor:
    """数据集就绪审计器"""

    def __init__(self,
                 fields: Optional[List[str]] = None,
                 duplicate_threshold: float = 0.9,
                 pii_free_required: bool = True):
        """初始化审计器

        Args:
            fields: 参与空值统计的字段，缺省 instruction/input/output
            duplicate_threshold: 去重相似度阈值
            pii_free_required: 是否要求零 PII 才判定就绪
        """
        self.fields = fields or ["instruction", "input", "output"]
        self.duplicate_threshold = duplicate_threshold
        self.pii_free_required = pii_free_required

    def audit(self,
              items: List[Dict[str, Any]],
              reference: Optional[List[Dict[str, Any]]] = None) -> AuditReport:
        """审计数据集

        Args:
            items: 待审计数据
            reference: 参考集（如测试集），提供时检测泄漏

        Returns:
            AuditReport 实例
        """
        report = AuditReport(total_items=len(items))

        if not items:
            report.findings.append("数据集为空")
            report.ready = False
            return report

        # 1. PII 暴露
        _, pii_report = PiiSanitizer(fields=self.fields).sanitize_dataset(items)
        report.pii_matches = pii_report.total_matches
        if report.pii_matches and self.pii_free_required:
            report.findings.append(f"检测到 {report.pii_matches} 处 PII，建议先脱敏")

        # 2. 集内重复
        dedup = Deduplicator(threshold=self.duplicate_threshold)
        dedup_result = dedup.deduplicate(items)
        report.duplicate_count = dedup_result.removed_count
        report.duplicate_rate = (
            report.duplicate_count / len(items) if items else 0.0
        )
        if report.duplicate_count:
            report.findings.append(
                f"存在 {report.duplicate_count} 条重复（{report.duplicate_rate:.1%}）"
            )

        # 3. 空字段率
        empty_cells = 0
        total_cells = 0
        for item in items:
            for f in self.fields:
                total_cells += 1
                value = item.get(f)
                if value is None or (isinstance(value, str) and not value.strip()):
                    empty_cells += 1
        report.empty_field_rate = (
            empty_cells / total_cells if total_cells else 0.0
        )
        if report.empty_field_rate > 0.5:
            report.findings.append(
                f"空字段率 {report.empty_field_rate:.1%} 偏高，数据不完整"
            )

        # 4. 泄漏（可选）——基于问题文本比较，避免答案差异导致漏判
        if reference is not None:
            leak_report = detect_leakage(items, reference, fields=["instruction"])
            report.leak_count = leak_report.total_leaks
            report.leak_rate = leak_report.leak_rate
            if report.leak_count:
                report.findings.append(
                    f"与参考集存在 {report.leak_count} 条泄漏，需隔离"
                )

        # 总体判定
        report.ready = not any(
            f.startswith(("检测到", "与参考集存在")) for f in report.findings
        )
        if not items:
            report.ready = False

        logger.info(
            "审计完成: %d 条，ready=%s，findings=%s",
            len(items), report.ready, report.findings or "无",
        )
        return report


def audit_dataset(items: List[Dict[str, Any]],
                  reference: Optional[List[Dict[str, Any]]] = None,
                  fields: Optional[List[str]] = None) -> AuditReport:
    """审计数据集（模块级便捷函数）

    Args:
        items: 待审计数据
        reference: 参考集（可选）
        fields: 参与统计的字段

    Returns:
        AuditReport 实例
    """
    return DatasetAuditor(fields=fields).audit(items, reference)
