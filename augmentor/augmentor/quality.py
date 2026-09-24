# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据质量评分模块 - 优化版"""

import logging
from typing import List, Dict, Optional
from dataclasses import dataclass
import numpy as np
from .model_manager import model_manager
from .exceptions import QualityError

logger = logging.getLogger(__name__)

# 字符 n-gram 的阶数（多样性/相关性 fallback 共用的口径，改动会改变所有分数）
_NGRAM_SIZE = 2


def _char_ngrams(text: str, n: int = _NGRAM_SIZE) -> frozenset:
    """字符 n-gram 集合（文本短于 n 时为空集）

    口径与原实现一致：**不做大小写归一**，`How` 与 `how` 的 bigram 不视为相同。
    """
    if len(text) < n:
        return frozenset()
    return frozenset(text[i:i + n] for i in range(len(text) - n + 1))


def _jaccard(a: frozenset, b: frozenset) -> float:
    """Jaccard 相似度

    并集用容斥（`|A| + |B| - |A∩B|`）而不是物化第三个集合：结果相同，
    少一次全量插入。
    """
    if not a or not b:
        return 0.0
    intersection = len(a & b)
    union = len(a) + len(b) - intersection
    return intersection / union if union else 0.0


@dataclass
class QualityScore:
    """质量评分结果"""
    semantic_similarity: float  # 语义相似度
    relevance: float  # 回答相关性
    diversity: float  # 多样性
    total_score: float  # 总分
    passed: bool  # 是否通过质量检查（构造时必须 bool() 收敛，见下）
    # 语义维度是否真实参与评分。当调用方无法提供原始种子问题（如对已落盘的数据集
    # 做离线评估）时，语义相似度无从计算，此时该维度被跳过、权重在剩余维度上重归一化，
    # 本字段为 False 且 semantic_similarity 不代表任何测量结果。
    semantic_evaluated: bool = True

    # 注意：passed 必须由 numpy 比较结果经 bool() 收敛为 Python 布尔。
    # numpy 的 np.bool_ 不是 bool 的子类，且 json.dumps 无法序列化它
    # （TypeError: Object of type bool is not JSON serializable），
    # 一旦该字段进入任何 JSON 响应或落盘路径就会 500。
    # total_score 无需转换：np.float64 是 float 的子类，可直接序列化。


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
            raise QualityError("权重必须包含 3 个元素")
        
        if abs(sum(self.weights) - 1.0) > 0.01:
            raise QualityError("权重之和必须为 1.0")
        
        self._model = None
        self._cross_encoder = None
        # 两份参照集缓存都用「上一轮真正缓存的那一段列表」本身作失效判据，
        # 不做内容指纹——理由见 :meth:`_calculate_diversity`。
        self._existing_embeddings = []  # 缓存已有文本的 embeddings
        self._existing_texts: List[str] = []  # 向量侧缓存对应的参照切片
        # n-gram 侧的参照集缓存**独立于向量侧**：两份缓存装的是同一段文本的不同
        # 表示，共用一份判据的话，`_model` 一旦变化（测试与降级路径都会）就会让
        # 一边认定「缓存有效」而另一边根本没填。
        self._existing_ngrams: List[frozenset] = []
        self._existing_ngram_texts: List[str] = []
    
    def _load_models(self):
        """延迟加载模型（使用共享实例）"""
        if self._model is None:
            self._model = model_manager.get_sentence_model()
        
        if self._cross_encoder is None:
            try:
                from sentence_transformers import CrossEncoder
                self._cross_encoder = CrossEncoder('cross-encoder/ms-marco-MiniLM-L-6-v2')
                logger.info("加载 cross-encoder 模型成功")
            except ImportError:
                logger.warning("cross-encoder 未安装，使用简化评分")
                self._cross_encoder = "fallback"
    
    def _ngram_similarity(self, text1: str, text2: str, n: int = _NGRAM_SIZE) -> float:
        """基于 n-gram 的相似度计算（fallback 方法）

        Args:
            text1: 文本 1
            text2: 文本 2
            n: n-gram 大小

        Returns:
            相似度分数 (0-1)
        """
        return _jaccard(_char_ngrams(text1, n), _char_ngrams(text2, n))

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
    
    def _calculate_diversity(self, text: str, existing_texts: List[str],
                             text_embedding: Optional[np.ndarray] = None) -> float:
        """计算多样性 - 优化版
        
        Args:
            text: 当前文本
            existing_texts: 已有文本列表
            text_embedding: 当前文本的向量。调用方若已批量编码过则传入，
                避免对同一条文本重复编码（为 None 时本方法自行编码）
        
        Returns:
            多样性分数 (0-1)
        """
        if not existing_texts:
            return 1.0
        
        if self._model == "fallback":
            # fallback: 使用 n-gram 相似度
            #
            # 参照集的 n-gram 集合整批只算一次。原实现每条候选都把它比的那
            # 30 条参照文本重新切成 bigram：真实 1500 条 × 参照窗 100 实测
            # `_ngram_similarity` 调 46500 次（每条 31 次）、`get_ngrams` 建
            # 93000 个集合（每条 62 个，其中参照侧 30 个是纯重复），
            # 234 万次解释器迭代占了整条 `batch_score` 的 78%。
            # 失效判据是「上一轮缓存的那一段列表」与本轮切片逐元素相等。
            # 早期版本比的是 md5("|".join(切片[:100]))，两处不忠实：
            # ①分隔符非单射——`["a", "b"]` 与 `["a|b"]` 拼出同一个串，于是
            #   参照集换了内容还会命中旧缓存（实测多样性虚高 0.565）；
            # ②只覆盖前 100 条——`diversity_sample_size > 100` 时第 101 条起
            #   无论怎么变都不失效（实测虚高 0.837）。
            # 顺带还便宜：真实语料 30 条参照切片，md5 那套 32.7 µs/次（每条候选
            # 付一次），列表相等 0.042 µs/次。
            # 两份缓存**用独立的属性**，理由见 `__init__`。
            sample_texts = existing_texts[:self.diversity_sample_size]
            if sample_texts != self._existing_ngram_texts:
                self._existing_ngrams = [_char_ngrams(text) for text in sample_texts]
                self._existing_ngram_texts = sample_texts
            candidate = _char_ngrams(text)
            max_sim = 0.0
            for ref_ngrams in self._existing_ngrams:
                sim = _jaccard(candidate, ref_ngrams)
                max_sim = max(max_sim, sim)
            return float(max(0.0, 1.0 - max_sim))

        # 编码当前文本（调用方已编码时直接复用）
        if text_embedding is None:
            text_embedding = self._model.encode([text])[0]
        
        # 使用缓存的 embeddings
        # 失效判据必须精确等于「被缓存的东西」：这里缓存的是
        # `existing_texts[:diversity_sample_size]` 那一段的向量，所以就直接拿
        # 那一段列表本身作判据（同一口径的来龙去脉见上面的 fallback 分支）。
        sample_texts = existing_texts[:self.diversity_sample_size]
        if sample_texts != self._existing_texts:
            # 需要重新计算
            if sample_texts:
                self._existing_embeddings = self._model.encode(
                    sample_texts,
                    show_progress_bar=False,
                    batch_size=32
                )
                self._existing_texts = sample_texts
        
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
        self._existing_ngrams = []
        self._existing_ngram_texts = []
    
    def effective_weights(self, include_semantic: bool) -> List[float]:
        """计算实际生效的权重

        语义维度不可用时，其权重按比例分摊到其余维度，保证权重和仍为 1.0，
        使总分继续落在 [0, 1] 区间，可与 threshold 直接比较。

        Args:
            include_semantic: 语义维度是否参与评分

        Returns:
            长度为 3 的权重列表；跳过语义时首元素为 0.0
        """
        if include_semantic:
            return list(self.weights)

        remaining = self.weights[1] + self.weights[2]
        if remaining <= 0:
            # 退化情形：相关性与多样性权重均为 0，无从重归一化
            return [0.0, 0.5, 0.5]
        scale = 1.0 / remaining
        return [0.0, self.weights[1] * scale, self.weights[2] * scale]

    def score(self, 
              original: str, 
              generated: str, 
              output: str,
              existing_generated: Optional[List[str]] = None,
              include_semantic: bool = True) -> QualityScore:
        """计算质量评分
        
        Args:
            original: 原始种子问题
            generated: 生成的问题
            output: 对应的回答
            existing_generated: 已生成的问题列表（用于多样性计算）
            include_semantic: 是否计算语义相似度维度。无法提供原始种子问题时传 False，
                该维度将被跳过，权重在剩余维度上重归一化。
        
        Returns:
            QualityScore 实例
        """
        self._load_models()
        
        weights = self.effective_weights(include_semantic)
        
        semantic_sim = (
            self._calculate_semantic_similarity(original, generated)
            if include_semantic else 0.0
        )
        relevance = self._calculate_relevance(generated, output)
        diversity = self._calculate_diversity(generated, existing_generated or [])
        
        total_score = (
            weights[0] * semantic_sim +
            weights[1] * relevance +
            weights[2] * diversity
        )
        
        return QualityScore(
            semantic_similarity=semantic_sim,
            relevance=relevance,
            diversity=diversity,
            total_score=total_score,
            passed=bool(total_score >= self.threshold),
            semantic_evaluated=include_semantic,
        )
    
    def batch_score(self, 
                    items: List[Dict],
                    existing_generated: Optional[List[str]] = None,
                    include_semantic: bool = True) -> List[QualityScore]:
        """批量计算质量评分 - 优化版
        
        Args:
            items: 数据列表，每项包含 original, generated, output 字段
            existing_generated: 已生成的问题列表
            include_semantic: 是否计算语义相似度维度。为 False 时跳过语义相似度计算
                （同时省去一次批量编码），权重在相关性与多样性上重归一化。
                对已落盘的数据集做离线评估时应传 False——此时不存在原始种子问题，
                若把 instruction 同时当作 original 与 generated，语义相似度会恒为 1.0，
                使总分虚高。
        
        Returns:
            评分结果列表
        """
        self._load_models()
        
        if not items:
            return []
        
        weights = self.effective_weights(include_semantic)
        
        # 提取文本
        originals = [item.get("original", "") for item in items]
        generateds = [item.get("generated", "") for item in items]
        outputs = [item.get("output", "") for item in items]
        
        # 批量编码优化：缓存已编码文本（避免重复计算相同文本嵌入）
        if not hasattr(self, '_embedding_cache'):
            self._embedding_cache = {}
        
        def cached_encode(texts, batch_key):
            # 缓存键就是「被缓存的那份输入」本身（batch_key + 元组化的文本列表），
            # 不做内容指纹。两个理由：
            # ①指纹不忠实：原实现用 md5("\x00".join(texts))，`["a", "b\x00c"]` 与
            #   `["a\x00b", "c"]` 拼出同一个串——等长时新批次会静默拿到旧批次的
            #   向量（实测语义分 0.7189/0.8174，真值 0.8104/0.8486），变长时直接
            #   IndexError。
            # ②更便宜：1500 条真实文本，join+md5 是 646 µs/查，元组键 4.3 µs。
            # 元组键只多持有一份指针数组（1500 条约 12 KB），相对于它所保护的
            # 整批向量（float32×1500×模型维度，数 MB 级）可忽略。
            cache_key = (batch_key, tuple(texts))
            if cache_key in self._embedding_cache:
                return self._embedding_cache[cache_key]
            embeddings = self._model.encode(texts, show_progress_bar=False, batch_size=32)
            self._embedding_cache[cache_key] = embeddings
            return embeddings

        # 生成文本的批量编码是「语义相似度」与「多样性」共用的输入。
        # 这里惰性求值：只有真正需要向量时才编码，且整批只编一次。
        # 原实现在 _calculate_diversity 内对每条文本单独编码，同一条文本
        # 在语义维度已批量编码过之后又被单条编码一次。
        generated_embeddings = None

        def get_generated_embeddings():
            nonlocal generated_embeddings
            if generated_embeddings is None:
                generated_embeddings = cached_encode(generateds, "gen")
            return generated_embeddings
        
        # 计算语义相似度（仅在需要时编码，跳过时省去两次批量编码）
        if not include_semantic:
            semantic_sims = np.zeros(len(items))
        elif self._model != "fallback":
            original_embeddings = cached_encode(originals, "orig")
            generated_embeddings = get_generated_embeddings()
            
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
            # 仅在多样性真正需要向量时才取：existing 为空时它会提前返回 1.0，
            # 此时不该为此触发一次批量编码。
            text_embedding = None
            if existing and self._model != "fallback":
                text_embedding = get_generated_embeddings()[i]
            diversity = self._calculate_diversity(gen, existing, text_embedding)
            diversity_scores.append(diversity)
            # 只有质量达标的样本才进入多样性参照集，避免劣质样本污染基线。
            # 语义维度被跳过时它恒为 0，若仍用它作门槛则参照集永不增长、
            # 多样性会退化成恒 1.0，因此改用当前可用的质量维度（相关性）把关。
            gate = semantic_sims[i] if include_semantic else relevance_scores[i]
            if gate >= self.threshold:
                existing.append(gen)
        
        diversity_scores = np.array(diversity_scores)
        
        # 计算总分
        total_scores = (
            weights[0] * semantic_sims +
            weights[1] * relevance_scores +
            weights[2] * diversity_scores
        )
        
        # 构建结果
        scores = []
        for i in range(len(items)):
            scores.append(QualityScore(
                semantic_similarity=float(semantic_sims[i]),
                relevance=float(relevance_scores[i]),
                diversity=float(diversity_scores[i]),
                total_score=float(total_scores[i]),
                passed=bool(total_scores[i] >= self.threshold),
                semantic_evaluated=include_semantic,
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
