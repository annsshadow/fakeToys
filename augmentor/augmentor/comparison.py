# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集对比模块

提供数据集之间的A/B测试对比功能。

分工边界（与 `augmentor.compare_enhanced`）
    本模块按**文件路径**对比：`compare_datasets(file_a, file_b)` 自行读盘，
    产出 `ComparisonResult`。CLI 的 `compare` 命令走这里。
    需要字段级指标（相似度 / 仅在 A / 仅在 B）或数据已在内存里，用 `compare_enhanced`。
"""

import json
import logging
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass, field
from pathlib import Path

from .quality import QualityScorer
from .dedup import Deduplicator

logger = logging.getLogger(__name__)


@dataclass
class ComparisonResult:
    """对比结果"""
    dataset_a_name: str
    dataset_b_name: str
    dataset_a_count: int
    dataset_b_count: int
    
    # 质量对比
    quality_a: Dict[str, float] = field(default_factory=dict)
    quality_b: Dict[str, float] = field(default_factory=dict)
    quality_diff: Dict[str, float] = field(default_factory=dict)
    
    # 去重对比
    dedup_a: Dict[str, float] = field(default_factory=dict)
    dedup_b: Dict[str, float] = field(default_factory=dict)
    
    # 长度对比
    length_a: Dict[str, float] = field(default_factory=dict)
    length_b: Dict[str, float] = field(default_factory=dict)
    
    # 词汇对比
    vocabulary_a: Dict[str, int] = field(default_factory=dict)
    vocabulary_b: Dict[str, int] = field(default_factory=dict)
    
    # 相似度对比
    similarity_to_a: float = 0.0
    similarity_to_b: float = 0.0
    
    # 结论
    winner: str = "tie"  # "a", "b", "tie"
    summary: str = ""


class DatasetComparator:
    """数据集对比器
    
    提供两个数据集之间的全面对比功能。
    """
    
    def __init__(self, 
                 quality_threshold: float = 0.6,
                 dedup_threshold: float = 0.9):
        """初始化对比器
        
        Args:
            quality_threshold: 质量评分阈值
            dedup_threshold: 去重相似度阈值
        """
        self.quality_scorer = QualityScorer(threshold=quality_threshold)
        self.deduplicator = Deduplicator(threshold=dedup_threshold)
    
    def _compute_quality_metrics(self, items: List[Dict]) -> Dict[str, float]:
        """计算质量指标
        
        Args:
            items: 数据列表
        
        Returns:
            质量指标字典
        """
        if not items:
            return {
                "avg_score": 0.0,
                "pass_rate": 0.0,
                "min_score": 0.0,
                "max_score": 0.0
            }
        
        scoring_items = [
            {
                "original": item.get("instruction", ""),
                "generated": item.get("instruction", ""),
                "output": item.get("output", "")
            }
            for item in items
        ]
        
        scores = self.quality_scorer.batch_score(scoring_items)
        total_scores = [s.total_score for s in scores]
        passed = sum(1 for s in scores if s.passed)
        
        return {
            "avg_score": sum(total_scores) / len(total_scores) if total_scores else 0,
            "pass_rate": passed / len(items) if items else 0,
            "min_score": min(total_scores) if total_scores else 0,
            "max_score": max(total_scores) if total_scores else 0
        }
    
    def _compute_length_metrics(self, items: List[Dict], key: str = "instruction") -> Dict[str, float]:
        """计算长度指标
        
        Args:
            items: 数据列表
            key: 文本字段名
        
        Returns:
            长度指标字典
        """
        lengths = [len(item.get(key, "")) for item in items]
        
        if not lengths:
            return {"avg": 0, "min": 0, "max": 0, "std": 0}
        
        avg_len = sum(lengths) / len(lengths)
        variance = sum((l - avg_len) ** 2 for l in lengths) / len(lengths)
        
        return {
            "avg": avg_len,
            "min": min(lengths),
            "max": max(lengths),
            "std": variance ** 0.5
        }
    
    def _compute_vocabulary_size(self, items: List[Dict], key: str = "instruction") -> Dict[str, int]:
        """计算词汇量
        
        Args:
            items: 数据列表
            key: 文本字段名
        
        Returns:
            词汇量信息
        """
        all_chars = set()
        for item in items:
            text = item.get(key, "")
            all_chars.update(text)
        
        return {
            "unique_chars": len(all_chars),
            "total_chars": sum(len(item.get(key, "")) for item in items)
        }
    
    def _compute_dedup_metrics(self, items: List[Dict]) -> Dict[str, float]:
        """计算去重指标
        
        Args:
            items: 数据列表
        
        Returns:
            去重指标字典
        """
        if len(items) < 2:
            return {"duplication_rate": 0.0, "unique_count": len(items)}
        
        report = self.deduplicator.generate_report(items)
        
        return {
            "duplication_rate": report.get("removal_rate", 0.0),
            "unique_count": report.get("deduplicated_count", len(items))
        }
    
    def compare(self,
                dataset_a: List[Dict],
                dataset_b: List[Dict],
                name_a: str = "Dataset A",
                name_b: str = "Dataset B") -> ComparisonResult:
        """对比两个数据集
        
        Args:
            dataset_a: 数据集A
            dataset_b: 数据集B
            name_a: 数据集A的名称
            name_b: 数据集B的名称
        
        Returns:
            对比结果
        """
        result = ComparisonResult(
            dataset_a_name=name_a,
            dataset_b_name=name_b,
            dataset_a_count=len(dataset_a),
            dataset_b_count=len(dataset_b)
        )
        
        # 计算质量指标
        result.quality_a = self._compute_quality_metrics(dataset_a)
        result.quality_b = self._compute_quality_metrics(dataset_b)
        result.quality_diff = {
            k: result.quality_b.get(k, 0) - result.quality_a.get(k, 0)
            for k in result.quality_a.keys()
        }
        
        # 计算去重指标
        result.dedup_a = self._compute_dedup_metrics(dataset_a)
        result.dedup_b = self._compute_dedup_metrics(dataset_b)
        
        # 计算长度指标
        result.length_a = self._compute_length_metrics(dataset_a)
        result.length_b = self._compute_length_metrics(dataset_b)
        
        # 计算词汇量
        result.vocabulary_a = self._compute_vocabulary_size(dataset_a)
        result.vocabulary_b = self._compute_vocabulary_size(dataset_b)
        
        # 判断获胜者
        score_a = result.quality_a.get("avg_score", 0)
        score_b = result.quality_b.get("avg_score", 0)
        
        if abs(score_a - score_b) < 0.01:
            result.winner = "tie"
        elif score_a > score_b:
            result.winner = "a"
        else:
            result.winner = "b"
        
        # 生成摘要
        result.summary = self._generate_summary(result)
        
        return result
    
    def _generate_summary(self, result: ComparisonResult) -> str:
        """生成对比摘要
        
        Args:
            result: 对比结果
        
        Returns:
            摘要文本
        """
        lines = [
            f"## 数据集对比: {result.dataset_a_name} vs {result.dataset_b_name}",
            "",
            f"### 基本信息",
            f"- {result.dataset_a_name}: {result.dataset_a_count} 条",
            f"- {result.dataset_b_name}: {result.dataset_b_count} 条",
            "",
            "### 质量对比",
            f"- 平均质量分: {result.quality_a.get('avg_score', 0):.3f} → {result.quality_b.get('avg_score', 0):.3f}",
            f"- 通过率: {result.quality_a.get('pass_rate', 0):.1%} → {result.quality_b.get('pass_rate', 0):.1%}",
            "",
            "### 长度对比",
            f"- 平均长度: {result.length_a.get('avg', 0):.1f} → {result.length_b.get('avg', 0):.1f}",
            "",
            "### 结论",
            f"- 获胜者: {result.winner.upper() if result.winner != 'tie' else '平局'}"
        ]
        
        return "\n".join(lines)
    
    def save_comparison(self, result: ComparisonResult, output_path: str):
        """保存对比结果
        
        Args:
            result: 对比结果
            output_path: 输出路径
        """
        output = Path(output_path)
        output.parent.mkdir(parents=True, exist_ok=True)
        
        with open(output, 'w', encoding='utf-8') as f:
            json.dump({
                "dataset_a_name": result.dataset_a_name,
                "dataset_b_name": result.dataset_b_name,
                "dataset_a_count": result.dataset_a_count,
                "dataset_b_count": result.dataset_b_count,
                "quality_a": result.quality_a,
                "quality_b": result.quality_b,
                "quality_diff": result.quality_diff,
                "dedup_a": result.dedup_a,
                "dedup_b": result.dedup_b,
                "length_a": result.length_a,
                "length_b": result.length_b,
                "vocabulary_a": result.vocabulary_a,
                "vocabulary_b": result.vocabulary_b,
                "winner": result.winner,
                "summary": result.summary
            }, f, ensure_ascii=False, indent=2)
        
        logger.info(f"对比结果已保存到 {output_path}")


def compare_datasets(file_a: str, 
                    file_b: str,
                    name_a: str = "Dataset A",
                    name_b: str = "Dataset B",
                    output_path: Optional[str] = None) -> ComparisonResult:
    """对比两个数据集文件
    
    Args:
        file_a: 数据集A文件路径
        file_b: 数据集B文件路径
        name_a: 数据集A的名称
        name_b: 数据集B的名称
        output_path: 输出路径（可选）
    
    Returns:
        对比结果
    """
    # 加载数据
    with open(file_a, 'r', encoding='utf-8') as f:
        data_a = json.load(f)
    
    with open(file_b, 'r', encoding='utf-8') as f:
        data_b = json.load(f)
    
    # 创建对比器并执行对比
    comparator = DatasetComparator()
    result = comparator.compare(data_a, data_b, name_a, name_b)
    
    # 保存结果
    if output_path:
        comparator.save_comparison(result, output_path)
    
    return result
