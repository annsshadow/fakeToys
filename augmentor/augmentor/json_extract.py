# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""LLM 响应 JSON 鲁棒提取模块

模型生成的 JSON 常被 markdown 代码围栏、前后缀说明文字包裹，
或带尾随逗号。本模块按多级策略稳健提取可解析的 JSON 值：
直接解析 → 代码围栏 → 平衡括号扫描 → 尾部逗号修复。

纯标准库实现，离线可用。
"""

import json
import logging
import re
from dataclasses import dataclass, field
from typing import Any, Dict, Optional

logger = logging.getLogger(__name__)

_FENCE_RE = re.compile(r"```(?:json|javascript)?\s*(.*?)```", re.DOTALL)
_TRAILING_COMMA_RE = re.compile(r",\s*([}\]])")


@dataclass
class ExtractResult:
    """JSON 提取结果"""

    ok: bool
    value: Any = None
    method: str = "none"

    def to_dict(self) -> Dict[str, Any]:
        return {"ok": self.ok, "value": self.value, "method": self.method}


def _balanced_span(text: str) -> Optional[str]:
    """截取首个平衡的 {...} 或 [...] 片段

    扫描时感知字符串字面量与转义，避免括号在字符串内误配平。

    Args:
        text: 原始文本

    Returns:
        平衡片段，找不到返回 None
    """
    start = -1
    opener = None
    for i, ch in enumerate(text):
        if ch in "{[":
            start = i
            opener = ch
            break
    if start == -1:
        return None

    closer = "}" if opener == "{" else "]"
    depth = 0
    in_string = False
    escaped = False
    for i in range(start, len(text)):
        ch = text[i]
        if in_string:
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == '"':
                in_string = False
            continue
        if ch == '"':
            in_string = True
        elif ch == opener:
            depth += 1
        elif ch == closer:
            depth -= 1
            if depth == 0:
                return text[start:i + 1]
    return None


def _strip_trailing_commas(text: str) -> str:
    """去除对象/数组结束括号前的尾随逗号（单次扫描感知字符串）"""
    result = []
    in_string = False
    escaped = False
    for i, ch in enumerate(text):
        if in_string:
            result.append(ch)
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == '"':
                in_string = False
            continue
        if ch == '"':
            in_string = True
            result.append(ch)
            continue
        if ch == ",":
            j = i + 1
            while j < len(text) and text[j] in " \t\r\n":
                j += 1
            if j < len(text) and text[j] in "}]":
                continue  # 跳过尾随逗号
        result.append(ch)
    return "".join(result)


def extract_json(text: str) -> ExtractResult:
    """从文本中提取 JSON 值

    Args:
        text: 可能包含说明文字/围栏的模型响应

    Returns:
        ExtractResult（ok 表示是否提取成功，method 记录命中策略）
    """
    if not isinstance(text, str) or not text.strip():
        return ExtractResult(ok=False)

    candidate = text.strip()

    # 1. 直接解析
    try:
        return ExtractResult(ok=True, value=json.loads(candidate), method="direct")
    except (json.JSONDecodeError, ValueError):
        pass

    # 2. 代码围栏
    fence = _FENCE_RE.search(candidate)
    if fence:
        body = fence.group(1).strip()
        for payload in (body, _strip_trailing_commas(body)):
            try:
                return ExtractResult(ok=True, value=json.loads(payload), method="fence")
            except (json.JSONDecodeError, ValueError):
                continue

    # 3. 平衡括号扫描（优先对象，其次数组）
    for opener in ("{", "["):
        idx = candidate.find(opener)
        if idx == -1:
            continue
        span = _balanced_span(candidate[idx:])
        if not span:
            continue
        for payload in (span, _strip_trailing_commas(span)):
            try:
                return ExtractResult(ok=True, value=json.loads(payload), method="balanced")
            except (json.JSONDecodeError, ValueError):
                continue

    logger.debug("JSON 提取失败，未找到可解析结构")
    return ExtractResult(ok=False, method="none")


def extract_json_list(text: str) -> ExtractResult:
    """提取 JSON 数组（限定返回 list）

    Args:
        text: 模型响应

    Returns:
        ExtractResult，value 为 list（失败时 ok=False）
    """
    result = extract_json(text)
    if result.ok and not isinstance(result.value, list):
        result.ok = False
        result.method = "not_list"
    return result
