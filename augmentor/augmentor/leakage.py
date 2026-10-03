# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""训练/测试集泄漏检测模块

检测训练集与测试集之间是否存在重复或近似重复样本（数据泄漏），
避免同一问题同时出现在训练与评测两侧导致指标虚高。

设计原则：
- 纯 Python 实现，无外部依赖，离线可用；
- 精确匹配基于归一化签名，近似匹配基于字符三元组 Jaccard 相似度；
- 倒排索引把候选集缩到「共享至少一个三元组」的训练签名，并按可采纳剪枝
  `I >= t*|a|` 进一步收窄；相似度由「共享三元组个数 + 两侧基数」直接算出，
  热路径不做任何集合运算 ⇒ 实测规模翻倍比 ×2.3~×2.8（改写前 ×4.0，即两两全比），
  仍随候选规模增长但不再等于全量 O(n*m) 比较；
- 输出可审计报告，附泄漏示例与处置建议。
"""

import logging
import re
from collections import Counter
from dataclasses import dataclass, field
from itertools import chain
from typing import Any, Dict, List, Optional, Set

from augmentor.validation import DataValidationError, require_ratio

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


def _jaccard_from_counts(intersection: int, size_a: int, size_b: int) -> float:
    """由「交集大小 + 两侧基数」算 Jaccard（并集不必真的建出来）

    倒排索引里逐对数出来的共享三元组个数**就是**交集大小，所以并集可以按
    `|a| + |b| - |a∩b|` 算出来，热路径因此一次集合运算都不做。这一支是算式的
    唯一权威，`_jaccard` 只是它的集合版外壳。

    Args:
        intersection: 交集元素个数
        size_a: 集合 A 的基数
        size_b: 集合 B 的基数

    Returns:
        相似度 (0-1)
    """
    union = size_a + size_b - intersection
    return intersection / union if union else 0.0


def _jaccard(a: Set[str], b: Set[str]) -> float:
    """Jaccard 相似度（集合版外壳，两侧已是集合时用）

    Args:
        a: 集合 A
        b: 集合 B

    Returns:
        相似度 (0-1)
    """
    if not a or not b:
        return 0.0
    return _jaccard_from_counts(len(a & b), len(a), len(b))


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
            fuzzy_threshold: 近似匹配的 Jaccard 阈值（>= 判定为泄漏）。可用区间 (0, 1]：
                0 档是「假零」（Jaccard 下界 0 ⇒ 任何 token 相交即判泄漏，干净数据也报
                100% 泄漏），> 1 档「静默关闭」（Jaccard 永不超过 1，永远报不出近似泄漏）。
                两方向都能造出假读数，故构造期即拒（A172，L181；fail-loud 方向）。
            min_examples: 报告中最多保留的泄漏示例条数

        Raises:
            DataValidationError: fuzzy_threshold 为 None / 非数值 / bool / NaN / 越界（(0,1] 之外）
        """
        # fuzzy_threshold 权威判据住构造器（A77 一条判据一处）：API/CLI 面经 detect_leakage
        # 委托到本构造器，自动吃到同一档；下界取开（0 档是假零家族 L144 同式），上界取闭。
        if fuzzy_threshold is None:
            raise DataValidationError(
                "fuzzy_threshold 不能为 None（必填比例旋钮，显式 None 属坏值非缺省）"
            )
        fuzzy_threshold = require_ratio(
            "fuzzy_threshold", fuzzy_threshold, minimum=0.0, maximum=1.0
        )
        if fuzzy_threshold <= 0:
            raise DataValidationError(
                "fuzzy_threshold 必须大于 0（0 档会把干净数据全判为泄漏，假零家族 L144 同式）"
            )
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

        # 训练侧：签名去重成序号 + 三元组→序号倒排 + 序号→该签名三元组个数
        # （旧版在这里另存一份 `sig -> 三元组集合` 供逐对交并；改成「共享元组数即交集」
        #  之后那整份第二拷贝不再需要，峰值内存随训练侧三元组总量减半）
        sig_ids: Dict[str, int] = {}
        gram_index: Dict[str, Set[int]] = {}
        sig_gram_counts: List[int] = []
        for item in train_items:
            sig = self._signature(item)
            if not sig or sig in sig_ids:
                continue
            sid = len(sig_gram_counts)
            sig_ids[sig] = sid
            grams = _char_trigrams(sig)
            sig_gram_counts.append(len(grams))
            for gram in grams:
                gram_index.setdefault(gram, set()).add(sid)

        threshold = self.fuzzy_threshold
        for test_item in test_items:
            sig = self._signature(test_item)
            kind: Optional[str] = None

            if sig:
                if sig in sig_ids:
                    kind = "exact"
                else:
                    test_grams = _char_trigrams(sig)
                    size_a = len(test_grams)
                    # 剪枝：J = I/U 且 U >= |a| ⇒ J >= t 蕴含 I >= t*|a|。取整用 int()（向零
                    # 截断）而不是 ceil：浮点误差只会**少剪**，不会把该比的对误剪掉（L30 平局教训）。
                    need = int(threshold * size_a)
                    shared = Counter(chain.from_iterable(
                        gram_index.get(gram, ()) for gram in test_grams))
                    best = max(
                        (_jaccard_from_counts(overlap, size_a, sig_gram_counts[sid])
                         for sid, overlap in shared.items() if overlap >= need),
                        # 旧版在这里从 best=0.0 起算，所以「阈值 <= 0 且无可比候选」也判 fuzzy；
                        # 这条 default 就是那一份口径，不许顺手改掉
                        default=0.0,
                    )
                    if best >= threshold:
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
