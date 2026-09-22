# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""报告 Markdown 渲染模块

把 PII/泄漏/审计等报告字典渲染为可读 Markdown，便于写进文档或
CI 评论。纯字符串处理，离线可用。
"""

from typing import Any, Dict

from .audit import AuditReport
from .leakage import LeakageReport
from .privacy import SanitizeReport


def _md_table(headers, rows) -> str:
    """渲染简单 Markdown 表格

    Args:
        headers: 表头列
        rows: 行（与表头等长的列表/元组）

    Returns:
        Markdown 表格文本
    """
    lines = [
        "| " + " | ".join(str(h) for h in headers) + " |",
        "| " + " | ".join("---" for _ in headers) + " |",
    ]
    for row in rows:
        lines.append("| " + " | ".join(str(c) for c in row) + " |")
    return "\n".join(lines)


def sanitize_report_to_markdown(report: SanitizeReport, title: str = "PII 脱敏报告") -> str:
    """渲染 PII 脱敏报告

    Args:
        report: 脱敏报告
        title: 标题

    Returns:
        Markdown 文本
    """
    d = report.to_dict()
    lines = [f"# {title}", "",
             f"- 总条数: {d['total_items']}",
             f"- 涉及脱敏条数: {d['touched_items']}",
             f"- 总命中: {d['total_matches']}",
             "",
             "## 命中类型", ""]
    matches = d.get("matches", {})
    if matches:
        lines.append(_md_table(["类型", "命中数"], [[k, v] for k, v in matches.items()]))
    else:
        lines.append("无")
    return "\n".join(lines)


def leakage_report_to_markdown(report: LeakageReport, title: str = "泄漏检测报告") -> str:
    """渲染泄漏检测报告

    Args:
        report: 泄漏报告
        title: 标题

    Returns:
        Markdown 文本
    """
    d = report.to_dict()
    status = "✅ 无泄漏" if d["is_clean"] else "⚠️ 存在泄漏"
    lines = [f"# {title}", "", status, "",
             f"- 训练集: {d['train_size']} 条",
             f"- 测试集: {d['test_size']} 条",
             f"- 精确泄漏: {d['exact_leaks']}",
             f"- 近似泄漏: {d['fuzzy_leaks']}",
             f"- 泄漏率: {d['leak_rate']:.2%}",
             "",
             "## 示例", ""]
    if d.get("leaked_examples"):
        lines.append(_md_table(
            ["类型", "测试条目"],
            [[e.get("type"), str(e.get("test_item", {}).get("instruction", ""))[:40]]
             for e in d["leaked_examples"]],
        ))
    else:
        lines.append("无")
    return "\n".join(lines)


def audit_report_to_markdown(report: AuditReport, title: str = "数据集审计") -> str:
    """渲染审计报告

    Args:
        report: 审计报告
        title: 标题

    Returns:
        Markdown 文本
    """
    d = report.to_dict()
    verdict = "✅ 就绪" if d["ready"] else "❌ 未就绪"
    lines = [f"# {title}", "", verdict, "",
             f"- 数据量: {d['total_items']}",
             f"- PII 命中: {d['pii_matches']}",
             f"- 重复: {d['duplicate_count']}（{d['duplicate_rate']:.1%}）",
             f"- 空字段率: {d['empty_field_rate']:.1%}",
             f"- 泄漏: {d['leak_count'] if d['leak_count'] is not None else '未检测'}",
             "",
             "## 发现", ""]
    if d.get("findings"):
        for f in d["findings"]:
            lines.append(f"- {f}")
    else:
        lines.append("无")
    return "\n".join(lines)


def generic_report_to_markdown(payload: Dict[str, Any], title: str = "报告") -> str:
    """把任意报告字典渲染为 Markdown 键值列表

    Args:
        payload: 报告字典
        title: 标题

    Returns:
        Markdown 文本
    """
    lines = [f"# {title}", ""]
    for key, value in payload.items():
        if isinstance(value, (dict, list)):
            lines.append(f"- **{key}**: {len(value)}")
        else:
            lines.append(f"- **{key}**: {value}")
    return "\n".join(lines)


__all__ = [
    "sanitize_report_to_markdown",
    "leakage_report_to_markdown",
    "audit_report_to_markdown",
    "generic_report_to_markdown",
]
