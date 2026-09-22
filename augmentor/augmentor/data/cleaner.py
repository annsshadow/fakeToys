# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据清洗模块

提供去噪、格式标准化、语言检测等纯规则能力，便于离线运行与单元测试。

分工边界（与 `augmentor.cleaner`）
    本模块的 `DataCleaner` / `CleanResult` 由 `augmentor.data` 包导出，
    调用方是 CLI 的 `clean` 命令与 REST API 的清洗端点。
    `augmentor.cleaner` 是另一套实现（`DatasetCleaner` / `CleaningResult`），
    由 CLI 的 `clean-enhanced` 使用。两者是**同一能力的两种实现**，这一对才是
    真正值得合并的（合并需先统一结果类型，登记为后续任务）。
"""

import html
import logging
import re
from dataclasses import dataclass, field
from typing import List, Dict, Any

logger = logging.getLogger(__name__)

# 全角标点到半角标点的映射
FULLWIDTH_PUNCTUATION = {
    "，": ",", "。": ".", "！": "!", "？": "?", "；": ";", "：": ":",
    "（": "(", "）": ")", "【": "[", "】": "]", "《": "<", "》": ">",
    "“": '"', "”": '"', "‘": "'", "’": "'", "、": ",", "～": "~"
}

# 需要清理的零宽字符
ZERO_WIDTH_CHARS = ["\u200b", "\u200c", "\u200d", "\ufeff"]

URL_PATTERN = re.compile(r"https?://\S+|www\.\S+")
HTML_TAG_PATTERN = re.compile(r"<[^>]+>")
CONTROL_CHAR_PATTERN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f\x7f]")
WHITESPACE_PATTERN = re.compile(r"[ \t\u3000]+")
CJK_PATTERN = re.compile(r"[\u4e00-\u9fff]")
LATIN_PATTERN = re.compile(r"[A-Za-z]")


@dataclass
class CleanResult:
    """清洗结果"""
    original_count: int = 0
    cleaned_count: int = 0
    dropped_count: int = 0
    changed_count: int = 0
    language_distribution: Dict[str, int] = field(default_factory=dict)
    issues: Dict[str, int] = field(default_factory=dict)
    items: List[Dict] = field(default_factory=list)


class DataCleaner:
    """数据清洗器"""

    def __init__(self,
                 text_key: str = "instruction",
                 remove_urls: bool = True,
                 drop_empty: bool = True,
                 min_length: int = 1):
        """初始化清洗器

        Args:
            text_key: 参与清洗的文本字段
            remove_urls: 是否移除 URL
            drop_empty: 清洗后为空是否丢弃该条数据
            min_length: 清洗后文本的最小长度
        """
        self.text_key = text_key
        self.remove_urls = remove_urls
        self.drop_empty = drop_empty
        self.min_length = min_length

    def remove_noise(self, text: str) -> str:
        """去除文本噪声

        包括 HTML 标签、URL、零宽字符、控制字符与多余空白。

        Args:
            text: 原始文本

        Returns:
            去噪后的文本
        """
        if not isinstance(text, str):
            return ""

        cleaned = html.unescape(text)
        cleaned = HTML_TAG_PATTERN.sub(" ", cleaned)
        if self.remove_urls:
            cleaned = URL_PATTERN.sub(" ", cleaned)
        for char in ZERO_WIDTH_CHARS:
            cleaned = cleaned.replace(char, "")
        cleaned = CONTROL_CHAR_PATTERN.sub("", cleaned)
        cleaned = WHITESPACE_PATTERN.sub(" ", cleaned)
        return cleaned.strip()

    def normalize_format(self, text: str) -> str:
        """标准化文本格式

        统一全角标点为半角，规范化空白与换行。

        Args:
            text: 原始文本

        Returns:
            标准化后的文本
        """
        if not isinstance(text, str):
            return ""

        normalized = text
        for full, half in FULLWIDTH_PUNCTUATION.items():
            normalized = normalized.replace(full, half)

        # 折叠连续换行与空白
        normalized = re.sub(r"\r\n?", "\n", normalized)
        normalized = re.sub(r"\n{3,}", "\n\n", normalized)
        normalized = WHITESPACE_PATTERN.sub(" ", normalized)
        return normalized.strip()

    def detect_language(self, text: str) -> str:
        """检测文本语言

        基于中日韩字符与拉丁字母的占比进行判断。

        Args:
            text: 待检测文本

        Returns:
            zh / en / mixed / unknown
        """
        if not isinstance(text, str) or not text:
            return "unknown"

        cjk_count = len(CJK_PATTERN.findall(text))
        latin_count = len(LATIN_PATTERN.findall(text))
        total = cjk_count + latin_count

        if total == 0:
            return "unknown"

        cjk_ratio = cjk_count / total
        latin_ratio = latin_count / total

        if cjk_ratio >= 0.8:
            return "zh"
        if latin_ratio >= 0.8:
            return "en"
        # 此处 cjk<0.8 且 latin<0.8；因 cjk+latin=1，必有一方 >0.1，即混合
        return "mixed"

    def clean_text(self, text: str) -> str:
        """组合执行去噪与格式标准化

        Args:
            text: 原始文本

        Returns:
            清洗后的文本
        """
        return self.normalize_format(self.remove_noise(text))

    def clean(self, items: List[Dict]) -> CleanResult:
        """批量清洗数据

        Args:
            items: 数据列表

        Returns:
            CleanResult 实例
        """
        result = CleanResult(original_count=len(items))
        language_distribution: Dict[str, int] = {}
        issues: Dict[str, int] = {"noise_removed": 0, "format_normalized": 0, "dropped": 0}

        for item in items:
            original_text = item.get(self.text_key, "")
            if not isinstance(original_text, str):
                original_text = str(original_text) if original_text is not None else ""

            denoised = self.remove_noise(original_text)
            normalized = self.normalize_format(denoised)

            if denoised != original_text:
                issues["noise_removed"] += 1
            if normalized != denoised:
                issues["format_normalized"] += 1

            language = self.detect_language(normalized)
            language_distribution[language] = language_distribution.get(language, 0) + 1

            if len(normalized) < self.min_length and self.drop_empty:
                issues["dropped"] += 1
                continue

            if normalized != original_text:
                result.changed_count += 1

            cleaned_item = dict(item)
            cleaned_item[self.text_key] = normalized
            result.items.append(cleaned_item)

        result.cleaned_count = len(result.items)
        result.dropped_count = result.original_count - result.cleaned_count
        result.language_distribution = language_distribution
        result.issues = issues

        logger.info(
            f"清洗完成: {result.original_count} -> {result.cleaned_count} 条，"
            f"丢弃 {result.dropped_count} 条"
        )
        return result

    def generate_report(self, items: List[Dict]) -> Dict[str, Any]:
        """生成清洗报告

        Args:
            items: 数据列表

        Returns:
            报告字典
        """
        result = self.clean(items)

        return {
            "original_count": result.original_count,
            "cleaned_count": result.cleaned_count,
            "dropped_count": result.dropped_count,
            "changed_count": result.changed_count,
            "drop_rate": (
                result.dropped_count / result.original_count
                if result.original_count else 0.0
            ),
            "language_distribution": result.language_distribution,
            "issues": result.issues,
            "text_key": self.text_key
        }
