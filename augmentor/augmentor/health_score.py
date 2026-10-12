# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集健康度评分模块

功能：计算数据集的完整性、多样性、质量分布均衡性和覆盖率，
生成综合健康评分并提供等级判断（healthy / moderate / poor）。

使用示例：
    from augmentor import DatasetHealthScore
    scorer = DatasetHealthScore()
    result = scorer.score(dataset_items)
    print(result["health_score"], result["level"])
"""

import logging
from typing import List, Dict, Optional
import numpy as np
from .validation import require_ratio_list

logger = logging.getLogger(__name__)

#: 四维默认权重（完整性 / 多样性 / 质量分布均衡性 / 覆盖率）。
#: 只有这一份，页面与请求模型都不抄它（L235 的判据）。
DEFAULT_HEALTH_WEIGHTS = [0.25, 0.25, 0.25, 0.25]


class DatasetHealthScore:
    """数据集健康度评分器"""
    
    def __init__(self, weights: Optional[List[float]] = None):
        """初始化健康评分器
        
        Args:
            weights: 权重 [完整性, 多样性, 质量分布均衡性, 覆盖率]

        Raises:
            DataValidationError: 形状不对——不是 4 个元素、逐项不是 [0, 1] 内的数值
                （布尔不算）、某项是 NaN，或四项之和不在 1.0 ± 0.01（含端点）内

        判据不在本层手抄（A319 / A320，L238）：改前这里是两行 `len != 4` 与
        `abs(sum - 1.0) > 0.01`，于是**同一份权重在两面不同判**——端点门口那两行照抄
        了它，所以 `[2.0, -0.5, -0.25, -0.25]`（和恰为 1.0）能过门口也能过本层，把健康分
        顶到 1.17 并读成 `healthy`；而 `'abc'` 这类非数值元素在本层死在 `sum()` 的
        `TypeError` 上。现在与 `quality.weights` 那两面共引同一份
        `validation.require_ratio_list`（A77：一条权威只住一处），逐项 [0, 1] 的界一生效，
        「刻意用负权重压低某一维」就不再是一个能用的写法，而是 400。
        """
        self.weights = (
            require_ratio_list("weights", weights, length=4) or list(DEFAULT_HEALTH_WEIGHTS)
        )
    
    def calculate_completeness(self, items: List[Dict]) -> float:
        """计算数据完整性（必填字段完整率）"""
        if not items:
            return 0.0
        required_fields = {"instruction", "output"}
        complete = sum(
            1 for item in items
            if all(field in item and item[field] for field in required_fields)
        )
        return complete / len(items)
    
    def calculate_diversity(self, items: List[Dict]) -> float:
        """计算数据多样性（基于指令文本的唯一性比例）"""
        if not items:
            return 0.0
        instructions = [item.get("instruction", "") for item in items]
        unique_instructions = len(set(instructions))
        return unique_instructions / len(instructions) if instructions else 0.0
    
    def calculate_quality_balance(self, items: List[Dict]) -> float:
        """计算质量分布均衡性（假设已存在质量评分数据时使用）"""
        if not items:
            return 0.0
        # 简化计算：根据输出长度分布的标准差反推均衡性
        lengths = [len(str(item.get("output", ""))) for item in items]
        if not lengths or max(lengths) == min(lengths):
            return 1.0
        std_ratio = np.std(lengths) / (np.mean(lengths) + 1e-6)
        return float(max(0.0, 1.0 - std_ratio))
    
    def calculate_coverage(self, items: List[Dict]) -> float:
        """计算覆盖率（指令关键词覆盖广度简化估计）"""
        if not items:
            return 0.0
        all_text = " ".join(str(item.get("instruction", "")) for item in items)
        # 使用简单词频估计：词数与总字符数比例
        words = all_text.split()
        if not words:
            return 0.0
        unique_words = len(set(words))
        return min(1.0, unique_words / (len(words) + 1e-6) * 10)
    
    def score(self, items: List[Dict]) -> Dict:
        """计算数据集健康度评分

        `health_score` 的值域是 **[0, 1]**（A319，L238）。四项分量各自都在 [0, 1] 内
        （completeness / diversity 是比例、quality_balance 是 `max(0.0, 1.0 - std_ratio)`、
        coverage 带 `min(1.0, …)` 封顶），权重逐项也在 [0, 1] 且和为 1 ± 0.01，所以唯一
        还能越界的是那个容差：满分分量乘 1.01 份权重就是 **1.01**。改前这里不钳，
        实测 `[2.0, -0.5, -0.25, -0.25]` 在 4 条夹具上得到 **1.1697944143478562**，
        而 `level` 的三档阈值判的就是这个未钳的数 ⇒ 「健康分 1.17 / healthy」。
        钳在出口而不是靠调用方解释：字段名与页面文案承诺的都是一支比例。
        """
        completeness = self.calculate_completeness(items)
        diversity = self.calculate_diversity(items)
        quality_balance = self.calculate_quality_balance(items)
        coverage = self.calculate_coverage(items)
        
        raw = (
            self.weights[0] * completeness +
            self.weights[1] * diversity +
            self.weights[2] * quality_balance +
            self.weights[3] * coverage
        )
        total = min(1.0, max(0.0, float(raw)))
        
        return {
            "health_score": total,
            "level": "healthy" if total >= 0.7 else ("moderate" if total >= 0.4 else "poor"),
            "metrics": {
                "completeness": float(completeness),
                "diversity": float(diversity),
                "quality_balance": float(quality_balance),
                "coverage": float(coverage),
            },
            "weights": self.weights,
            "total_samples": len(items)
        }
