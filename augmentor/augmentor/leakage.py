# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""训练/测试集泄漏检测模块

检测训练集与测试集之间是否存在重复或近似重复样本（数据泄漏），
避免同一问题同时出现在训练与评测两侧导致指标虚高。

设计原则：
- 纯 Python 实现，无外部依赖，离线可用；
- 精确匹配基于归一化签名，近似匹配基于字符三元组 Jaccard 相似度；
- 倒排索引约束候选集，避免大规模数据集 O(n*m) 全量两两比较；
- 输出可审计报告，附泄漏示例与处置建议。
"""

import logging
import re
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, Set, Tuple

logger = logging.getLogger(__name__)

# 归一化用：仅保留字母数字与中文，丢弃空白与标点
_NORM_RE = re.compile(r"[\u4e00-\u9fff0-9a-z]")


def _normalize(text: str) -> str:
    """归一化文本（小写、去空白/标点，保留中英数字）

    Args:
        text: 原始文本

    Returns:
        归一化字符串
    """
    if not isinstance(text, str):
        return ""
    return "".join(_NORM_RE.findall(text.lower()))


def _char_trigrams(normalized: str) -> Set[str]:
    """生成字符三元组集合

    Args:
        normalized: 归一化文本

    Returns:
        三元组集合
    """
    if len(normalized) < 3:
        return {normalized} if normalized else set()
    return {normalized[i:i + 3] for i in range(len(normalized) - 2)}


def _jaccard(a: Set[str], b: Set[str]) -> float:
    """Jaccard 相似度

    Args:
        a: 集合 A
        b: 集合 B

    Returns:
        相似度 (0-1)
    """
    if not a or not b:
        return 0.0
    inter = len(a & b)
    union = len(a | b)
    return inter / union if union else 0.0


@dataclass
class LeakageReport:
    """泄漏检测报告"""

    train_size: int
    test_size: int
    exact_leaks: int = 0
    fuzzy_leaks: int = 0
    leaked_examples: List[Dict[str, Any]] = field(default_factory=list)

    @property
    def total_leaks(self) -> int:
        return self.exact_leaks + self.fuzzy_leaks

    @property
    def leak_rate(self) -> float:
        """泄漏率 = 泄漏测试条数 / 测试集总量"""
        if self.test_size == 0:
            return 0.0
        return self.total_leaks / self.test_size

    @property
    def is_clean(self) -> bool:
        return self.total_leaks == 0

    def to_dict(self) -> Dict[str, Any]:
        return {
            "train_size": self.train_size,
            "test_size": self.test_size,
            "exact_leaks": self.exact_leaks,
            "fuzzy_leaks": self.fuzzy_leaks,
            "total_leaks": self.total_leaks,
            "leak_rate": self.leak_rate,
            "is_clean": self.is_clean,
            "leaked_examples": self.leaked_examples,
        }


class LeakageDetector:
    """训练/测试集泄漏检测器"""

    def __init__(self,
                 fields: Optional[List[str]] = None,
                 fuzzy_threshold: float = 0.8,
                 min_examples: int = 10):
        """初始化检测器

        Args:
            fields: 参与比较的字段名，缺省为 instruction
            fuzzy_threshold: 近似匹配的 Jaccard 阈值（>= 判定为泄漏）
            min_examples: 报告中最多保留的泄漏示例条数
        """
        self.fields = fields or ["instruction"]
        self.fuzzy_threshold = fuzzy_threshold
        self.min_examples = min_examples

    def _signature(self, item: Dict[str, Any]) -> str:
        parts = [_normalize(str(item.get(f, ""))) for f in self.fields]
        return "||".join(parts)

    def detect(self,
               train_items: List[Dict],
               test_items: List[Dict]) -> LeakageReport:
        """检测训练集与测试集之间的泄漏

        Args:
            train_items: 训练集
            test_items: 测试集

        Returns:
            LeakageReport 实例
        """
        report = LeakageReport(train_size=len(train_items), test_size=len(test_items))
        if not test_items:
            return report

        # 精确签名集合（训练侧）
        train_sigs: Set[str] = set()
        for item in train_items:
            sig = self._signature(item)
            if sig:
                train_sigs.add(sig)

        # 倒排索引：三元组 -> 训练签名
        gram_index: Dict[str, Set[str]] = {}
        for sig in train_sigs:
            for gram in _char_trigrams(sig):
                gram_index.setdefault(gram, set()).add(sig)

        # 每个训练签名对应的三元组集合（用于 Jaccard）
        sig_grams = {sig: _char_trigrams(sig) for sig in train_sigs}

        for test_item in test_items:
            sig = self._signature(test_item)
            kind: Optional[str] = None

            if sig and sig in train_sigs:
                kind = "exact"
            elif sig:
                # 候选：共享任一元组的训练签名
                candidates: Set[str] = set()
                for gram in _char_trigrams(sig):
                    candidates.update(gram_index.get(gram, ()))
                test_grams = _char_trigrams(sig)
                best = 0.0
                for cand in candidates:
                    sim = _jaccard(test_grams, sig_grams[cand])
                    if sim > best:
                        best = sim
                if best >= self.fuzzy_threshold:
                    kind = "fuzzy"

            if kind == "exact":
                report.exact_leaks += 1
                report.leaked_examples.append({
                    "type": "exact",
                    "test_item": test_item,
                })
            elif kind == "fuzzy":
                report.fuzzy_leaks += 1
                report.leaked_examples.append({
                    "type": "fuzzy",
                    "test_item": test_item,
                })

        report.leaked_examples = report.leaked_examples[:self.min_examples]
        logger.info(
            "泄漏检测完成: 测试集 %d 条中 %d 条泄漏（精确 %d / 近似 %d）",
            report.test_size, report.total_leaks, report.exact_leaks, report.fuzzy_leaks,
        )
        return report


def detect_leakage(train_items: List[Dict],
                   test_items: List[Dict],
                   fields: Optional[List[str]] = None,
                   fuzzy_threshold: float = 0.8) -> LeakageReport:
    """检测数据集泄漏（模块级便捷函数）

    Args:
        train_items: 训练集
        test_items: 测试集
        fields: 参与比较的字段名
        fuzzy_threshold: 近似匹配阈值

    Returns:
        LeakageReport 实例
    """
    detector = LeakageDetector(fields=fields, fuzzy_threshold=fuzzy_threshold)
    return detector.detect(train_items, test_items)
