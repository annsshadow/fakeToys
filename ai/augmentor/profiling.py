"""数据集画像模块

对训练数据做全维度画像统计：字段完整性、长度分布、重复率、语言检测、
关键词热度等，输出可直接用于质量报告的画像字典。
"""

import json
import logging
from collections import Counter
from dataclasses import dataclass
from typing import Any, Dict, List, Optional

logger = logging.getLogger(__name__)

# 中文字符区间
_CJK_RANGES = (
    (0x4E00, 0x9FFF),
    (0x3400, 0x4DBF),
)


def _char_count(text: str) -> int:
    """计算字符数（去除首尾空白）"""
    return len(text.strip())


def _detect_language(text: str) -> str:
    """粗粒度语言检测：中文 / 英文 / 混合 / 未知

    Args:
        text: 文本

    Returns:
        语言标记
    """
    if not text:
        return "unknown"

    cjk = 0
    latin = 0
    for char in text:
        code = ord(char)
        if any(lo <= code <= hi for lo, hi in _CJK_RANGES):
            cjk += 1
        elif char.isascii() and char.isalpha():
            latin += 1

    total = cjk + latin
    if total == 0:
        return "unknown"

    cjk_ratio = cjk / total
    if cjk_ratio >= 0.8:
        return "zh"
    if cjk_ratio <= 0.2:
        return "en"
    if cjk_ratio > 0:
        return "mixed"
    return "unknown"


@dataclass
class ProfilingConfig:
    """画像配置"""
    length_field: str = "instruction"
    top_keywords: int = 20
    min_keyword_length: int = 2
    language_sample_limit: int = 200


class DataProfiler:
    """数据集画像器"""

    def __init__(self, config: Optional[ProfilingConfig] = None):
        """初始化画像器

        Args:
            config: 画像配置
        """
        self.config = config or ProfilingConfig()

    def profile(self, items: List[Dict]) -> Dict[str, Any]:
        """生成数据集画像

        Args:
            items: 数据列表

        Returns:
            画像字典
        """
        if not items:
            return {
                "total_items": 0,
                "field_completeness": {},
                "length_stats": {},
                "duplicate_rate": 0.0,
                "language_distribution": {},
                "top_keywords": [],
            }

        total = len(items)
        field_completeness = self._field_completeness(items)
        length_stats = self._length_stats(items)
        duplicate_rate = self._duplicate_rate(items)
        language_distribution = self._language_distribution(items)
        top_keywords = self._top_keywords(items)

        return {
            "total_items": total,
            "field_completeness": field_completeness,
            "length_stats": length_stats,
            "duplicate_rate": duplicate_rate,
            "language_distribution": language_distribution,
            "top_keywords": top_keywords,
        }

    def _field_completeness(self, items: List[Dict]) -> Dict[str, float]:
        """计算各字段非空率

        Args:
            items: 数据列表

        Returns:
            字段名 -> 非空率
        """
        all_fields: set = set()
        for item in items:
            all_fields.update(item.keys())

        total = len(items)
        completeness: Dict[str, float] = {}
        for field in sorted(all_fields):
            filled = sum(1 for item in items if str(item.get(field, "")).strip())
            completeness[field] = filled / total if total else 0.0
        return completeness

    def _length_stats(self, items: List[Dict]) -> Dict[str, float]:
        """计算指定字段的长度统计

        Args:
            items: 数据列表

        Returns:
            min/avg/max/median 长度
        """
        lengths = [
            _char_count(str(item.get(self.config.length_field, "")))
            for item in items
        ]
        if not lengths:
            return {"min": 0, "max": 0, "avg": 0.0, "median": 0.0}

        ordered = sorted(lengths)
        median = ordered[len(ordered) // 2]
        return {
            "min": min(lengths),
            "max": max(lengths),
            "avg": sum(lengths) / len(lengths),
            "median": median,
        }

    def _duplicate_rate(self, items: List[Dict]) -> float:
        """计算 instruction 字段的重复率

        Args:
            items: 数据列表

        Returns:
            重复条数占比
        """
        if not items:
            return 0.0

        texts = [str(item.get(self.config.length_field, "")).strip() for item in items]
        counter = Counter(texts)
        duplicates = sum(count - 1 for count in counter.values() if count > 1)
        return duplicates / len(items)

    def _language_distribution(self, items: List[Dict]) -> Dict[str, int]:
        """估算语言分布（抽样）

        Args:
            items: 数据列表

        Returns:
            语言标记 -> 样本数
        """
        sample = items[:self.config.language_sample_limit]
        distribution: Counter = Counter()
        for item in sample:
            text = str(item.get(self.config.length_field, ""))
            distribution[_detect_language(text)] += 1
        return dict(distribution)

    def _top_keywords(self, items: List[Dict]) -> List[Dict[str, Any]]:
        """提取高频关键词（按单字 + 双字统计，过滤低频）

        Args:
            items: 数据列表

        Returns:
            [{keyword, count}, ...] 按频次降序
        """
        keyword_counter: Counter = Counter()
        min_len = self.config.min_keyword_length

        for item in items:
            text = str(item.get(self.config.length_field, ""))
            seen: set = set()
            for size in (min_len, min_len + 1):
                for i in range(max(0, len(text) - size + 1)):
                    gram = text[i:i + size]
                    if gram and gram not in seen:
                        keyword_counter[gram] += 1
                        seen.add(gram)

        ranked = keyword_counter.most_common(self.config.top_keywords)
        return [{"keyword": kw, "count": cnt} for kw, cnt in ranked]

    def profile_to_json(self, items: List[Dict]) -> str:
        """生成 JSON 字符串画像

        Args:
            items: 数据列表

        Returns:
            JSON 字符串
        """
        return json.dumps(self.profile(items), ensure_ascii=False, indent=2)

    def save_profile(self, items: List[Dict], output_path: str) -> str:
        """保存画像到文件

        Args:
            items: 数据列表
            output_path: 输出路径

        Returns:
            实际写入的文件路径
        """
        content = self.profile_to_json(items)
        with open(output_path, "w", encoding="utf-8") as f:
            f.write(content)
        logger.info(f"画像已保存到 {output_path}")
        return output_path


def profile_dataset(items: List[Dict],
                   output_path: Optional[str] = None) -> Dict[str, Any]:
    """便捷函数：执行数据集画像

    Args:
        items: 数据列表
        output_path: 可选的输出文件路径

    Returns:
        画像字典
    """
    profiler = DataProfiler()
    if output_path:
        profiler.save_profile(items, output_path)
    return profiler.profile(items)


__all__ = [
    "DataProfiler",
    "ProfilingConfig",
    "profile_dataset",
    "_detect_language",
]
