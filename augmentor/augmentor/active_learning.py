# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""主动学习循环模块

按不确定性 / 多样性 / 混合策略挑选最有价值的样本，并记录迭代历史。
"""

import logging
import random
from dataclasses import dataclass, field
from typing import List, Dict, Any, Optional, Callable

from .evaluation import compute_similarity

logger = logging.getLogger(__name__)

SUPPORTED_STRATEGIES = ["uncertainty", "diversity", "hybrid", "random"]


@dataclass
class IterationRecord:
    """单轮迭代记录"""
    iteration: int
    strategy: str
    selected_count: int
    selected_indices: List[int] = field(default_factory=list)
    performance: Dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        """转换为字典

        Returns:
            记录字典
        """
        return {
            "iteration": self.iteration,
            "strategy": self.strategy,
            "selected_count": self.selected_count,
            "selected_indices": self.selected_indices,
            "performance": self.performance
        }


class ActiveLearningLoop:
    """主动学习循环"""

    def __init__(self,
                 strategy: str = "uncertainty",
                 batch_size: int = 50,
                 threshold: float = 0.6,
                 text_key: str = "instruction",
                 scorer: Optional[Any] = None,
                 score_fn: Optional[Callable[[Dict], float]] = None,
                 random_state: Optional[int] = None):
        """初始化主动学习循环

        Args:
            strategy: 采样策略（uncertainty / diversity / hybrid / random）
            batch_size: 每轮选择样本数
            threshold: 质量阈值，用于不确定性计算
            text_key: 文本字段名
            scorer: 质量评分器，为 None 时惰性创建 QualityScorer
            score_fn: 自定义打分函数，优先级高于 scorer
            random_state: 随机种子，便于复现
        """
        if strategy not in SUPPORTED_STRATEGIES:
            raise ValueError(
                f"不支持的采样策略: {strategy}。支持: {SUPPORTED_STRATEGIES}"
            )
        if batch_size <= 0:
            raise ValueError("batch_size 必须为正整数")

        self.strategy = strategy
        self.batch_size = batch_size
        self.threshold = threshold
        self.text_key = text_key
        self._scorer = scorer
        self._score_fn = score_fn
        self._random = random.Random(random_state)

        self._labeled: List[Dict] = []
        self._iteration = 0
        self.history: List[IterationRecord] = []

    def _get_scorer(self):
        """惰性获取质量评分器

        Returns:
            QualityScorer 实例
        """
        if self._scorer is None:
            from .quality import QualityScorer
            self._scorer = QualityScorer(threshold=self.threshold)
        return self._scorer

    def _quality_score(self, item: Dict) -> float:
        """计算单条数据的质量分

        Args:
            item: 数据项

        Returns:
            质量分 (0-1)
        """
        if self._score_fn is not None:
            return float(self._score_fn(item))

        text = item.get(self.text_key, "")
        score = self._get_scorer().score(text, text, item.get("output", ""))
        return float(score.total_score)

    def _uncertainty_scores(self, items: List[Dict]) -> List[float]:
        """计算不确定性分数（越接近阈值越不确定，分数越高）

        Args:
            items: 数据列表

        Returns:
            不确定性分数列表
        """
        scores = []
        for item in items:
            quality = self._quality_score(item)
            # 距离阈值越近越不确定；用 1.0 作为最大可能距离
            uncertainty = 1.0 - abs(quality - self.threshold)
            scores.append(max(0.0, min(1.0, uncertainty)))
        return scores

    def _diversity_scores(self, items: List[Dict]) -> List[float]:
        """计算多样性分数（与已有样本差异越大，分数越高）

        Args:
            items: 数据列表

        Returns:
            多样性分数列表
        """
        texts = [item.get(self.text_key, "") for item in items]
        if not texts:
            return []

        scores = []
        for index, text in enumerate(texts):
            max_similarity = 0.0
            for other_index, other_text in enumerate(texts):
                if index == other_index:
                    continue
                similarity = compute_similarity(text, other_text)
                if similarity > max_similarity:
                    max_similarity = similarity
            scores.append(1.0 - max_similarity)
        return scores

    def _random_scores(self, items: List[Dict]) -> List[float]:
        """生成随机分数

        Args:
            items: 数据列表

        Returns:
            随机分数列表
        """
        return [self._random.random() for _ in items]

    def _compute_scores(self, items: List[Dict], strategy: str) -> List[float]:
        """按策略计算打分

        Args:
            items: 数据列表
            strategy: 采样策略

        Returns:
            分数列表
        """
        if strategy == "uncertainty":
            return self._uncertainty_scores(items)
        if strategy == "diversity":
            return self._diversity_scores(items)
        if strategy == "random":
            return self._random_scores(items)

        # hybrid: 不确定性与多样性各占一半
        uncertainty = self._uncertainty_scores(items)
        diversity = self._diversity_scores(items)
        return [
            (u + d) / 2.0
            for u, d in zip(uncertainty, diversity)
        ]

    def select_samples(self,
                       data: List[Dict],
                       strategy: Optional[str] = None,
                       batch_size: Optional[int] = None) -> List[Dict]:
        """选择最有价值的样本

        Args:
            data: 候选数据列表
            strategy: 采样策略，为 None 时使用默认策略
            batch_size: 选择数量，为 None 时使用默认值

        Returns:
            选中的样本列表（按分值降序）
        """
        strategy = strategy or self.strategy
        if strategy not in SUPPORTED_STRATEGIES:
            raise ValueError(
                f"不支持的采样策略: {strategy}。支持: {SUPPORTED_STRATEGIES}"
            )

        if not data:
            return []

        size = min(batch_size or self.batch_size, len(data))
        scores = self._compute_scores(data, strategy)

        ranked = sorted(
            range(len(data)),
            key=lambda i: scores[i],
            reverse=True
        )[:size]

        selected = []
        for index in ranked:
            item = dict(data[index])
            item["_al_score"] = round(scores[index], 6)
            item["_al_index"] = index
            selected.append(item)

        logger.info(
            f"主动学习选样: 策略 {strategy}，从 {len(data)} 条中选出 {len(selected)} 条"
        )
        return selected

    def update_model(self, labeled_data: List[Dict]):
        """更新已标注数据集合

        本项目不涉及模型训练本身，这里记录标注数据并推进迭代计数。

        Args:
            labeled_data: 新标注的数据
        """
        if labeled_data:
            self._labeled.extend(labeled_data)
        self._iteration += 1
        logger.info(
            f"第 {self._iteration} 轮更新，累计标注 {len(self._labeled)} 条"
        )

    def evaluate_performance(self, data: Optional[List[Dict]] = None) -> Dict[str, Any]:
        """评估当前循环表现

        Args:
            data: 候选数据列表，用于统计覆盖情况

        Returns:
            性能报告
        """
        report: Dict[str, Any] = {
            "iteration": self._iteration,
            "labeled_count": len(self._labeled)
        }

        if self._labeled:
            scores = [float(item.get("_al_score", 0.0)) for item in self._labeled]
            report["avg_selection_score"] = sum(scores) / len(scores)
            report["max_selection_score"] = max(scores)
            report["min_selection_score"] = min(scores)

        if data:
            report["candidate_count"] = len(data)
            report["labeled_ratio"] = (
                len(self._labeled) / len(data) if data else 0.0
            )

        if self.history:
            report["iterations"] = len(self.history)
            report["avg_batch_size"] = (
                sum(h.selected_count for h in self.history) / len(self.history)
            )

        return report

    def run(self,
            data: List[Dict],
            iterations: Optional[int] = None,
            strategy: Optional[str] = None) -> Dict[str, Any]:
        """运行主动学习循环

        Args:
            data: 候选数据列表
            iterations: 迭代轮数，为 None 时单轮
            strategy: 采样策略

        Returns:
            循环报告，包含每轮选中的样本
        """
        strategy = strategy or self.strategy
        rounds = iterations if iterations is not None else 1

        if rounds <= 0:
            raise ValueError("iterations 必须为正整数")

        # 保留原始下标，保证跨轮次的选中记录都能追溯到同一份输入数据
        remaining = [(index, item) for index, item in enumerate(data)]
        selected_batches: List[List[Dict]] = []

        for _ in range(rounds):
            if not remaining:
                logger.warning("候选数据已耗尽，提前结束主动学习循环")
                break

            candidates = [item for _, item in remaining]
            selected = self.select_samples(candidates, strategy=strategy)
            if not selected:
                break

            # select_samples 返回的 _al_index 是候选列表内的位置，需映射回原始下标
            local_positions = {item["_al_index"] for item in selected}
            original_indices = [
                remaining[position][0] for position in sorted(local_positions)
            ]

            for item, original_index in zip(
                sorted(selected, key=lambda x: x["_al_index"]), original_indices
            ):
                item["_al_index"] = original_index

            remaining = [
                (index, item)
                for position, (index, item) in enumerate(remaining)
                if position not in local_positions
            ]

            self.update_model(selected)
            selected_batches.append(selected)

            record = IterationRecord(
                iteration=self._iteration,
                strategy=strategy,
                selected_count=len(selected),
                selected_indices=original_indices,
                performance=self.evaluate_performance()
            )
            self.history.append(record)

        return {
            "strategy": strategy,
            "iterations": len(selected_batches),
            "total_selected": sum(len(b) for b in selected_batches),
            "remaining_count": len(remaining),
            "batches": [
                [item for item in batch]
                for batch in selected_batches
            ],
            "history": [record.to_dict() for record in self.history],
            "performance": self.evaluate_performance(data)
        }
