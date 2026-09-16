"""数据质量评分模块 - 优化版"""

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
    """数据质量评分器 - 优化版"""
    
    def __init__(self, 
                 threshold: float = 0.6,
                 weights: Optional[List[float]] = None,
                 diversity_sample_size: int = 30):
        """初始化质量评分器
        
        Args:
            threshold: 质量阈值，低于此分数的样本被过滤
            weights: 评分权重 [语义相似度, 回答相关性, 多样性]
            diversity_sample_size: 多样性计算时的采样数量
        """
        self.threshold = threshold
        self.weights = weights or [0.3, 0.4, 0.3]
        self.diversity_sample_size = diversity_sample_size
        
        if len(self.weights) != 3:
            raise ValueError("权重必须包含 3 个元素")
        
        if abs(sum(self.weights) - 1.0) > 0.01:
            raise ValueError("权重之和必须为 1.0")
        
        self._model = None
        self._cross_encoder = None
        self._existing_embeddings = []  # 缓存已有文本的 embeddings
        self._existing_texts = []  # 缓存已有文本
    
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
    
    def _ngram_similarity(self, text1: str, text2: str, n: int = 2) -> float:
        """基于 n-gram 的相似度计算（fallback 方法）
        
        Args:
            text1: 文本 1
            text2: 文本 2
            n: n-gram 大小
        
        Returns:
            相似度分数 (0-1)
        """
        def get_ngrams(text: str) -> set:
            return set(text[i:i+n] for i in range(len(text) - n + 1))
        
        ngrams1 = get_ngrams(text1)
        ngrams2 = get_ngrams(text2)
        
        if not ngrams1 or not ngrams2:
            return 0.0
        
        intersection = len(ngrams1 & ngrams2)
        union = len(ngrams1 | ngrams2)
        
        return intersection / union if union > 0 else 0.0
    
    def _calculate_semantic_similarity(self, text1: str, text2: str) -> float:
        """计算语义相似度
        
        Args:
            text1: 文本 1
            text2: 文本 2
        
        Returns:
            相似度分数 (0-1)
        """
        if self._model == "fallback":
            return self._ngram_similarity(text1, text2)
        
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
            # 使用 n-gram 相似度
            return self._ngram_similarity(question, answer)
        
        score = self._cross_encoder.predict([(question, answer)])
        # 归一化到 0-1
        return float(max(0.0, min(1.0, (score + 1) / 2)))
    
    def _calculate_diversity(self, text: str, existing_texts: List[str]) -> float:
        """计算多样性 - 优化版
        
        Args:
            text: 当前文本
            existing_texts: 已有文本列表
        
        Returns:
            多样性分数 (0-1)
        """
        if not existing_texts:
            return 1.0
        
        if self._model == "fallback":
            # fallback: 使用 n-gram 相似度
            max_sim = 0.0
            for existing in existing_texts[:self.diversity_sample_size]:
                sim = self._ngram_similarity(text, existing)
                max_sim = max(max_sim, sim)
            return float(max(0.0, 1.0 - max_sim))
        
        # 编码当前文本
        text_embedding = self._model.encode([text])[0]
        
        # 使用缓存的 embeddings
        if len(self._existing_embeddings) != len(existing_texts):
            # 需要重新计算
            if existing_texts:
                self._existing_embeddings = self._model.encode(
                    existing_texts[:self.diversity_sample_size],
                    show_progress_bar=False,
                    batch_size=32
                )
                self._existing_texts = existing_texts[:self.diversity_sample_size]
        
        if len(self._existing_embeddings) == 0:
            return 1.0
        
        # 计算与所有已有文本的最大相似度
        similarities = np.dot(self._existing_embeddings, text_embedding) / (
            np.linalg.norm(self._existing_embeddings, axis=1) * np.linalg.norm(text_embedding)
        )
        max_similarity = float(np.max(similarities))
        
        # 多样性 = 1 - 最大相似度
        return float(max(0.0, 1.0 - max_similarity))
    
    def reset_cache(self):
        """重置缓存"""
        self._existing_embeddings = []
        self._existing_texts = []
    
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
        """批量计算质量评分 - 优化版
        
        Args:
            items: 数据列表，每项包含 original, generated, output 字段
            existing_generated: 已生成的问题列表
        
        Returns:
            评分结果列表
        """
        self._load_models()
        
        if not items:
            return []
        
        # 提取文本
        originals = [item.get("original", "") for item in items]
        generateds = [item.get("generated", "") for item in items]
        outputs = [item.get("output", "") for item in items]
        
        # 批量编码原始文本
        if self._model != "fallback":
            original_embeddings = self._model.encode(originals, show_progress_bar=False, batch_size=32)
            generated_embeddings = self._model.encode(generateds, show_progress_bar=False, batch_size=32)
            
            # 计算语义相似度
            semantic_sims = np.array([
                np.dot(original_embeddings[i], generated_embeddings[i]) / (
                    np.linalg.norm(original_embeddings[i]) * np.linalg.norm(generated_embeddings[i])
                )
                for i in range(len(items))
            ])
            semantic_sims = np.clip(semantic_sims, 0, 1)
        else:
            semantic_sims = np.array([
                self._ngram_similarity(orig, gen)
                for orig, gen in zip(originals, generateds)
            ])
        
        # 计算回答相关性
        if self._cross_encoder != "fallback":
            pairs = [(gen, out) for gen, out in zip(generateds, outputs)]
            relevance_scores = self._cross_encoder.predict(pairs)
            relevance_scores = np.clip((relevance_scores + 1) / 2, 0, 1)
        else:
            relevance_scores = np.array([
                self._ngram_similarity(gen, out)
                for gen, out in zip(generateds, outputs)
            ])
        
        # 计算多样性（逐步更新）
        existing = list(existing_generated) if existing_generated else []
        diversity_scores = []
        
        for i, gen in enumerate(generateds):
            diversity = self._calculate_diversity(gen, existing)
            diversity_scores.append(diversity)
            if semantic_sims[i] >= self.threshold:  # 只有通过的才加入
                existing.append(gen)
        
        diversity_scores = np.array(diversity_scores)
        
        # 计算总分
        total_scores = (
            self.weights[0] * semantic_sims +
            self.weights[1] * relevance_scores +
            self.weights[2] * diversity_scores
        )
        
        # 构建结果
        scores = []
        for i in range(len(items)):
            scores.append(QualityScore(
                semantic_similarity=float(semantic_sims[i]),
                relevance=float(relevance_scores[i]),
                diversity=float(diversity_scores[i]),
                total_score=float(total_scores[i]),
                passed=total_scores[i] >= self.threshold
            ))
        
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
