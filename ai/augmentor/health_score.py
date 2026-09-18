"""数据集健康度评分模块 - 新增功能"""

import logging
from typing import List, Dict, Optional
import numpy as np

logger = logging.getLogger(__name__)


class DatasetHealthScore:
    """数据集健康度评分器"""
    
    def __init__(self, weights: Optional[List[float]] = None):
        """初始化健康评分器
        
        Args:
            weights: 权重 [完整性, 多样性, 质量分布均衡性, 覆盖率]
        """
        self.weights = weights or [0.25, 0.25, 0.25, 0.25]
        if len(self.weights) != 4:
            raise ValueError("权重必须包含 4 个元素")
        if abs(sum(self.weights) - 1.0) > 0.01:
            raise ValueError("权重之和必须为 1.0")
    
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
        """计算数据集健康度评分"""
        completeness = self.calculate_completeness(items)
        diversity = self.calculate_diversity(items)
        quality_balance = self.calculate_quality_balance(items)
        coverage = self.calculate_coverage(items)
        
        total = (
            self.weights[0] * completeness +
            self.weights[1] * diversity +
            self.weights[2] * quality_balance +
            self.weights[3] * coverage
        )
        
        return {
            "health_score": float(total),
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
