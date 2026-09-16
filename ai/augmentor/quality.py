"""数据质量评分模块"""

import logging
from typing import List, Dict, Optional
from dataclasses import dataclass
import numpy as np

logger = logging.getLogger(__name__)


@dataclass
class QualityScore:
    """质量评分结果"""
    semantic_similarity: float  # 语义相似度
    relevance: float  # 回答相关性
    diversity: float  # 多样性
    total_score: float  # 总分
    passed: bool  # 是否通过质量检查


class QualityScorer:
    """数据质量评分器"""
    
    def __init__(self, 
                 threshold: float = 0.6,
                 weights: Optional[List[float]] = None):
        """初始化质量评分器
        
        Args:
            threshold: 质量阈值，低于此分数的样本被过滤
            weights: 评分权重 [语义相似度, 回答相关性, 多样性]
        """
        self.threshold = threshold
        self.weights = weights or [0.3, 0.4, 0.3]
        
        if len(self.weights) != 3:
            raise ValueError("权重必须包含 3 个元素")
        
        if abs(sum(self.weights) - 1.0) > 0.01:
            raise ValueError("权重之和必须为 1.0")
        
        self._model = None
        self._cross_encoder = None
    
    def _load_models(self):
        """延迟加载模型"""
        if self._model is None:
            try:
                from sentence_transformers import SentenceTransformer
                self._model = SentenceTransformer('paraphrase-multilingual-MiniLM-L12-v2')
                logger.info("加载 sentence-transformers 模型成功")
            except ImportError:
                logger.warning("sentence-transformers 未安装，使用简化评分")
                self._model = "fallback"
        
        if self._cross_encoder is None:
            try:
                from sentence_transformers import CrossEncoder
                self._cross_encoder = CrossEncoder('cross-encoder/ms-marco-MiniLM-L-6-v2')
                logger.info("加载 cross-encoder 模型成功")
            except ImportError:
                logger.warning("cross-encoder 未安装，使用简化评分")
                self._cross_encoder = "fallback"
    
    def _calculate_semantic_similarity(self, text1: str, text2: str) -> float:
        """计算语义相似度
        
        Args:
            text1: 文本 1
            text2: 文本 2
        
        Returns:
            相似度分数 (0-1)
        """
        if self._model == "fallback":
            # 简化评分：基于字符重叠率
            set1 = set(text1)
            set2 = set(text2)
            intersection = len(set1 & set2)
            union = len(set1 | set2)
            return intersection / union if union > 0 else 0.0
        
        embeddings = self._model.encode([text1, text2])
        similarity = np.dot(embeddings[0], embeddings[1]) / (
            np.linalg.norm(embeddings[0]) * np.linalg.norm(embeddings[1])
        )
        return float(max(0.0, min(1.0, similarity)))
    
    def _calculate_relevance(self, question: str, answer: str) -> float:
        """计算回答相关性
        
        Args:
            question: 问题
            answer: 回答
        
        Returns:
            相关性分数 (0-1)
        """
        if self._cross_encoder == "fallback":
            # 简化评分：基于关键词匹配
            question_words = set(question)
            answer_words = set(answer)
            overlap = len(question_words & answer_words)
            return min(1.0, overlap / 10) if overlap > 0 else 0.3
        
        score = self._cross_encoder.predict([(question, answer)])
        # 归一化到 0-1
        return float(max(0.0, min(1.0, (score + 1) / 2)))
    
    def _calculate_diversity(self, text: str, existing_texts: List[str]) -> float:
        """计算多样性
        
        Args:
            text: 当前文本
            existing_texts: 已有文本列表
        
        Returns:
            多样性分数 (0-1)
        """
        if not existing_texts:
            return 1.0
        
        if self._model == "fallback":
            # 简化评分：基于文本长度和字符多样性
            unique_chars = len(set(text))
            return min(1.0, unique_chars / 50)
        
        # 计算与所有已有文本的最大相似度
        max_similarity = 0.0
        text_embedding = self._model.encode([text])[0]
        
        for existing in existing_texts[:100]:  # 限制数量避免性能问题
            existing_embedding = self._model.encode([existing])[0]
            similarity = np.dot(text_embedding, existing_embedding) / (
                np.linalg.norm(text_embedding) * np.linalg.norm(existing_embedding)
            )
            max_similarity = max(max_similarity, similarity)
        
        # 多样性 = 1 - 最大相似度
        return float(max(0.0, 1.0 - max_similarity))
    
    def score(self, 
              original: str, 
              generated: str, 
              output: str,
              existing_generated: Optional[List[str]] = None) -> QualityScore:
        """计算质量评分
        
        Args:
            original: 原始种子问题
            generated: 生成的问题
            output: 对应的回答
            existing_generated: 已生成的问题列表（用于多样性计算）
        
        Returns:
            QualityScore 实例
        """
        self._load_models()
        
        semantic_sim = self._calculate_semantic_similarity(original, generated)
        relevance = self._calculate_relevance(generated, output)
        diversity = self._calculate_diversity(generated, existing_generated or [])
        
        total_score = (
            self.weights[0] * semantic_sim +
            self.weights[1] * relevance +
            self.weights[2] * diversity
        )
        
        return QualityScore(
            semantic_similarity=semantic_sim,
            relevance=relevance,
            diversity=diversity,
            total_score=total_score,
            passed=total_score >= self.threshold
        )
    
    def batch_score(self, 
                    items: List[Dict],
                    existing_generated: Optional[List[str]] = None) -> List[QualityScore]:
        """批量计算质量评分
        
        Args:
            items: 数据列表，每项包含 original, generated, output 字段
            existing_generated: 已生成的问题列表
        
        Returns:
            评分结果列表
        """
        self._load_models()
        
        scores = []
        existing = existing_generated or []
        
        for item in items:
            score = self.score(
                original=item.get("original", ""),
                generated=item.get("generated", ""),
                output=item.get("output", ""),
                existing_generated=existing
            )
            scores.append(score)
            if score.passed:
                existing.append(item.get("generated", ""))
        
        return scores
    
    def filter_by_quality(self, 
                          items: List[Dict],
                          existing_generated: Optional[List[str]] = None) -> List[Dict]:
        """根据质量过滤数据
        
        Args:
            items: 数据列表
            existing_generated: 已生成的问题列表
        
        Returns:
            过滤后的数据列表
        """
        scores = self.batch_score(items, existing_generated)
        
        filtered = []
        for item, score in zip(items, scores):
            if score.passed:
                filtered.append(item)
            else:
                logger.debug(f"过滤低质量样本: {score.total_score:.3f} < {self.threshold}")
        
        return filtered
    
    def generate_report(self, 
                       items: List[Dict],
                       existing_generated: Optional[List[str]] = None) -> Dict:
        """生成质量报告
        
        Args:
            items: 数据列表
            existing_generated: 已生成的问题列表
        
        Returns:
            质量报告字典
        """
        scores = self.batch_score(items, existing_generated)
        
        total_scores = [s.total_score for s in scores]
        semantic_scores = [s.semantic_similarity for s in scores]
        relevance_scores = [s.relevance for s in scores]
        diversity_scores = [s.diversity for s in scores]
        passed_count = sum(1 for s in scores if s.passed)
        
        return {
            "total_samples": len(items),
            "passed_samples": passed_count,
            "filtered_samples": len(items) - passed_count,
            "pass_rate": passed_count / len(items) if items else 0,
            "total_score": {
                "mean": float(np.mean(total_scores)) if total_scores else 0,
                "std": float(np.std(total_scores)) if total_scores else 0,
                "min": float(np.min(total_scores)) if total_scores else 0,
                "max": float(np.max(total_scores)) if total_scores else 0,
            },
            "semantic_similarity": {
                "mean": float(np.mean(semantic_scores)) if semantic_scores else 0,
                "std": float(np.std(semantic_scores)) if semantic_scores else 0,
            },
            "relevance": {
                "mean": float(np.mean(relevance_scores)) if relevance_scores else 0,
                "std": float(np.std(relevance_scores)) if relevance_scores else 0,
            },
            "diversity": {
                "mean": float(np.mean(diversity_scores)) if diversity_scores else 0,
                "std": float(np.std(diversity_scores)) if diversity_scores else 0,
            },
            "threshold": self.threshold,
            "weights": self.weights
        }
