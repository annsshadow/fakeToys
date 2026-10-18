# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""PII 脱敏模块

对训练数据中的个人身份信息（邮箱、手机号、身份证号、IP 等）做掩码处理，
降低数据集直接分发或微调时的隐私泄露风险，并输出可审计的脱敏报告。

设计原则：
- 纯文本规则（正则）实现，无外部依赖，离线可用；
- 默认仅启用高置信度、低误伤的模式（邮箱/手机号/身份证/IP）；
- 每个命中以类型化占位符替换（如 [EMAIL]），保留语义边界便于模型学习；
- 长串数字优先匹配（身份证先于手机号），避免子串误替换。
"""

import logging
import re
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, Tuple

logger = logging.getLogger(__name__)


# 内置 PII 模式（顺序即优先级：先长后短，避免子串抢占）
DEFAULT_PATTERNS: "Dict[str, re.Pattern]" = {
    "email": re.compile(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}"),
    "id_card": re.compile(r"(?<![0-9Xx])[0-9]{17}[0-9Xx](?![0-9Xx])"),
    "phone": re.compile(r"(?<!\d)1[3-9]\d{9}(?!\d)"),
    "ip": re.compile(r"(?<!\d)(?:\d{1,3}\.){3}\d{1,3}(?!\d)"),
}

# 可选扩展模式（默认关闭，误伤面更大）
EXTRA_PATTERNS: "Dict[str, re.Pattern]" = {
    "credit_card": re.compile(r"(?<!\d)(?:\d[ -]?){13,16}\d(?!\d)"),
    "url": re.compile(r"https?://[^\s\"'<>]+"),
}


@dataclass
class SanitizeReport:
    """脱敏结果报告"""

    total_items: int
    touched_items: int
    matches: Dict[str, int] = field(default_factory=dict)

    @property
    def total_matches(self) -> int:
        return sum(self.matches.values())

    def to_dict(self) -> Dict[str, Any]:
        return {
            "total_items": self.total_items,
            "touched_items": self.touched_items,
            "matches": dict(self.matches),
            "total_matches": self.total_matches,
        }


class PiiSanitizer:
    """PII 脱敏器"""

    def __init__(self,
                 fields: Optional[List[str]] = None,
                 patterns: Optional[Dict[str, re.Pattern]] = None,
                 placeholders: Optional[Dict[str, str]] = None):
        """初始化脱敏器

        Args:
            fields: 参与脱敏的字段名列表，缺省为 instruction/input/output
            patterns: 自定义模式映射（类型名 -> 正则），缺省为 DEFAULT_PATTERNS
            placeholders: 各类型占位符，缺省为 [TYPE] 形式（大写类型名）
        """
        # L188: falsy-zero guard — 显式空容器（[] / {}）是「零元素」的合法意图，不是「没传」；
        # 旧写法 `x or default` 把空容器读成没传而悄悄套上默认，脱敏范围/字段面被无声放大。
        self.fields = fields if fields is not None else list(DEFAULT_TEXT_FIELDS)
        self.patterns = patterns if patterns is not None else DEFAULT_PATTERNS
        self.placeholders = placeholders if placeholders is not None else {
            name: f"[{name.upper()}]" for name in self.patterns
        }

    def sanitize_text(self, text: str) -> Tuple[str, List[str]]:
        """脱敏单段文本

        Args:
            text: 原始文本

        Returns:
            (脱敏后文本, 命中的类型名列表，按应用顺序)
        """
        if not isinstance(text, str) or not text:
            return text, []

        hits: List[str] = []
        result = text
        for name, pattern in self.patterns.items():
            matches = pattern.findall(result)
            if not matches:
                continue
            placeholder = self.placeholders.get(name, f"[{name.upper()}]")
            result = pattern.sub(placeholder, result)
            hits.append(name)
        return result, hits

    def sanitize_item(self, item: Dict[str, Any]) -> Tuple[Dict[str, Any], List[str]]:
        """脱敏单条数据（原地不修改，返回新 dict）

        Args:
            item: 数据项

        Returns:
            (脱敏后数据项, 命中类型并集)
        """
        sanitized = dict(item)
        hit_types: List[str] = []
        for field_name in self.fields:
            if field_name in sanitized and isinstance(sanitized[field_name], str):
                cleaned, hits = self.sanitize_text(sanitized[field_name])
                sanitized[field_name] = cleaned
                for h in hits:
                    if h not in hit_types:
                        hit_types.append(h)
        return sanitized, hit_types

    def sanitize_dataset(self, items: List[Dict[str, Any]]) -> Tuple[List[Dict[str, Any]], SanitizeReport]:
        """脱敏整个数据集

        Args:
            items: 数据列表

        Returns:
            (脱敏后数据列表, 脱敏报告)
        """
        match_counter: Dict[str, int] = {}
        touched = 0
        results: List[Dict[str, Any]] = []

        for item in items:
            sanitized, hits = self.sanitize_item(item)
            results.append(sanitized)
            if hits:
                touched += 1
                for h in hits:
                    match_counter[h] = match_counter.get(h, 0) + 1

        report = SanitizeReport(
            total_items=len(items),
            touched_items=touched,
            matches=match_counter,
        )
        logger.info(
            "PII 脱敏完成: %d 条中 %d 条含敏感信息，命中 %s",
            len(items), touched, match_counter or "无",
        )
        return results, report


def sanitize_pii(items: List[Dict[str, Any]],
                 fields: Optional[List[str]] = None,
                 include_extra: bool = False) -> Tuple[List[Dict[str, Any]], SanitizeReport]:
    """脱敏数据集（模块级便捷函数）

    Args:
        items: 数据列表
        fields: 参与脱敏的字段名，缺省为 instruction/input/output
        include_extra: 是否启用 EXTRA_PATTERNS（信用卡号/URL，误伤面较大）

    Returns:
        (脱敏后数据列表, 脱敏报告)
    """
    patterns = dict(DEFAULT_PATTERNS)
    if include_extra:
        patterns.update(EXTRA_PATTERNS)
    return PiiSanitizer(fields=fields, patterns=patterns).sanitize_dataset(items)


# 缺省文本面的唯一权威（A285 / L257）：「不给字段时看哪三格」此前在仓内手写五遍——
# 本模块的 PiiSanitizer 与 audit.DatasetAuditor 各一遍，CLI 的 clean / sanitize /
# audit 三条 --fields 各一遍。五份字面量语义是同一句话，抄五份的代价是任何一处改动
# 就让五面各说各话，而 l227 那把「页面发得出的每一档都被端点接受」的尺子在没有权威
# 常数时只能退到运行时判据（升级成逐项对账的前提就是这一格存在）。
# 同场另有两条 --fields 不属这一族：analyze 的默认面是 instruction + output 两格，
# check-leakage 只比 instruction 一格——它们各有权威，不由这一格收编。
#
# 为什么定义在文件末尾而不是紧挨 DEFAULT_PATTERNS：账本与 l227 / l228 两把守卫的模块
# docstring 把本模块第 76 行和 audit.py 第 69 行钉成活引用，在它们之上插任何一行都会
# 把落点平移成上一行——那正是 A337 / A338 那一族「行号还在、指的已不是那句」的形状。
# 取用点在第 76 行（`__init__` 运行时才查这个名字），所以定义住在尾部既能保持五格引用
# 逐字不漂，又仍然只有一份字面量。
#
# 为什么是元组而不是列表：模块级可变列表被多处共引，任何一处就地 append 会无声改掉
# 其余各处的默认面。需要列表的调用点自己 list() 一份。
DEFAULT_TEXT_FIELDS = ("instruction", "input", "output")
