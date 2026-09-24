# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""分层数据划分模块

在数据集划分（train/val/test）时支持按字段分层采样，
保证各子集的特征分布尽量与整体一致，避免训练/验证集偏差。
"""

import logging
import random
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, Tuple
from .exceptions import DataValidationError
from .allocation import largest_remainder

logger = logging.getLogger(__name__)


@dataclass
class SplitResult:
    """划分结果"""
    train: List[Dict]
    val: List[Dict]
    test: List[Dict]
    stratify_field: Optional[str] = None
    stratify_distribution: Dict[str, Dict[str, int]] = field(default_factory=dict)

    @property
    def total_items(self) -> int:
        return len(self.train) + len(self.val) + len(self.test)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "train_count": len(self.train),
            "val_count": len(self.val),
            "test_count": len(self.test),
            "total_items": self.total_items,
            "stratify_field": self.stratify_field,
            "stratify_distribution": self.stratify_distribution,
        }


class DataSplitter:
    """数据划分器（支持分层）"""

    def __init__(self,
                 train_ratio: float = 0.8,
                 val_ratio: float = 0.1,
                 test_ratio: float = 0.1,
                 seed: Optional[int] = None,
                 stratify_field: Optional[str] = None):
        """初始化划分器

        Args:
            train_ratio: 训练集比例
            val_ratio: 验证集比例
            test_ratio: 测试集比例
            seed: 随机种子
            stratify_field: 分层字段（可选）
        """
        total = train_ratio + val_ratio + test_ratio
        if abs(total - 1.0) > 1e-6:
            raise DataValidationError(
                f"比例之和必须为 1.0，实际为 {total:.4f}"
            )
        if any(r < 0 for r in (train_ratio, val_ratio, test_ratio)):
            raise DataValidationError("比例不能为负数")

        self.train_ratio = train_ratio
        self.val_ratio = val_ratio
        self.test_ratio = test_ratio
        self.seed = seed
        self.stratify_field = stratify_field

    def split(self, items: List[Dict]) -> SplitResult:
        """执行划分

        Args:
            items: 数据列表

        Returns:
            划分结果
        """
        if not items:
            return SplitResult(train=[], val=[], test=[])

        # 局部 Random：`random.seed()` 会改写进程级 RNG 状态，污染同进程内其它
        # 调用方的随机性（`seed=None` 时等价于取系统熵，行为不变）。
        rng = random.Random(self.seed)

        if self.stratify_field:
            return self._stratified_split(items, rng)
        return self._random_split(items, rng)

    def _random_split(self, items: List[Dict], rng: "random.Random") -> SplitResult:
        """随机划分（不分层）"""
        shuffled = list(items)
        rng.shuffle(shuffled)

        total = len(shuffled)
        # 余数按最大余数法分给三段，不再整份留给 test
        train_end, val_n, _ = largest_remainder(
            total, (self.train_ratio, self.val_ratio, self.test_ratio))
        val_end = train_end + val_n

        train = shuffled[:train_end]
        val = shuffled[train_end:val_end]
        test = shuffled[val_end:]

        return SplitResult(
            train=train,
            val=val,
            test=test,
            stratify_field=self.stratify_field,
        )

    def _stratified_split(self, items: List[Dict], rng: "random.Random") -> SplitResult:
        """按 stratify_field 分层划分

        每个特征值组内按 train/val/test 比例切分，再合并，
        保证各子集分布与总体分布接近。
        """
        groups: Dict[str, List[Dict]] = {}
        for item in items:
            key = str(item.get(self.stratify_field, "")) or "empty"
            groups.setdefault(key, []).append(item)

        train: List[Dict] = []
        val: List[Dict] = []
        test: List[Dict] = []
        distribution: Dict[str, Dict[str, int]] = {}

        for key, group in groups.items():
            rng.shuffle(group)
            n = len(group)
            train_n = int(n * self.train_ratio)
            val_n = int(n * self.val_ratio)
            test_n = n - train_n - val_n

            g_train = group[:train_n]
            g_val = group[train_n:train_n + val_n]
            g_test = group[train_n + val_n:train_n + val_n + test_n]

            train.extend(g_train)
            val.extend(g_val)
            test.extend(g_test)

            distribution[key] = {
                "train": len(g_train),
                "val": len(g_val),
                "test": len(g_test),
            }

        rng.shuffle(train)
        rng.shuffle(val)
        rng.shuffle(test)

        logger.info(f"分层划分完成: {distribution}")

        return SplitResult(
            train=train,
            val=val,
            test=test,
            stratify_field=self.stratify_field,
            stratify_distribution=distribution,
        )


def split_dataset(items: List[Dict],
                  train_ratio: float = 0.8,
                  val_ratio: float = 0.1,
                  test_ratio: float = 0.1,
                  seed: Optional[int] = None,
                  stratify_field: Optional[str] = None) -> SplitResult:
    """便捷函数：执行数据划分

    Args:
        items: 数据列表
        train_ratio: 训练集比例
        val_ratio: 验证集比例
        test_ratio: 测试集比例
        seed: 随机种子
        stratify_field: 分层字段

    Returns:
        划分结果
    """
    splitter = DataSplitter(
        train_ratio=train_ratio,
        val_ratio=val_ratio,
        test_ratio=test_ratio,
        seed=seed,
        stratify_field=stratify_field,
    )
    return splitter.split(items)


__all__ = [
    "DataSplitter",
    "SplitResult",
    "split_dataset",
]
