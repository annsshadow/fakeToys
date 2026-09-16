"""智能去重模块 - 优化版"""

import logging
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass
import numpy as np

logger = logging.getLogger(__name__)


@dataclass
class DedupResult:
    """去重结果"""
    original_count: int  # 原始数量
    deduplicated_count: int  # 去重后数量
    removed_count: int  # 移除数量
    duplicate_groups: List[List[int]]  # 重复组（索引列表）
    kept_indices: List[int]  # 保留的索引


class Deduplicator:
    """智能去重器 - 优化版"""
    
    def __init__(self, threshold: float = 0.9, use_faiss: bool = False):
        """初始化去重器
        
        Args:
            threshold: 相似度阈值，高于此值被视为重复
            use_faiss: 是否使用 FAISS 加速（需要安装 faiss-cpu）
        """
        if threshold < 0 or threshold > 1:
            raise ValueError("阈值必须在 0-1 之间")
        
        self.threshold = threshold
        self.use_faiss = use_faiss
        self._model = None
        self._faiss_index = None
    
    def _load_model(self):
        """延迟加载模型"""
        if self._model is None:
            try:
                from sentence_transformers import SentenceTransformer
                self._model = SentenceTransformer('paraphrase-multilingual-MiniLM-L12-v2')
                logger.info("加载 sentence-transformers 模型成功")
            except ImportError:
                logger.warning("sentence-transformers 未安装，使用简化去重")
                self._model = "fallback"
    
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
    
    def _batch_encode(self, texts: List[str]) -> np.ndarray:
        """批量编码文本
        
        Args:
            texts: 文本列表
        
        Returns:
            embeddings 矩阵
        """
        if self._model == "fallback":
            # fallback: 使用 n-gram 向量化
            return self._fallback_encode(texts)
        
        return self._model.encode(texts, show_progress_bar=False, batch_size=64)
    
    def _fallback_encode(self, texts: List[str]) -> np.ndarray:
        """fallback 编码方法
        
        Args:
            texts: 文本列表
        
        Returns:
            向量矩阵
        """
        # 使用字符 n-gram 的 TF 向量
        vocab = {}
        for text in texts:
            for i in range(len(text) - 1):
                ngram = text[i:i+2]
                if ngram not in vocab:
                    vocab[ngram] = len(vocab)
        
        vectors = np.zeros((len(texts), len(vocab)))
        for i, text in enumerate(texts):
            for j in range(len(text) - 1):
                ngram = text[j:j+2]
                if ngram in vocab:
                    vectors[i, vocab[ngram]] += 1
        
        # L2 归一化
        norms = np.linalg.norm(vectors, axis=1, keepdims=True)
        norms[norms == 0] = 1
        vectors = vectors / norms
        
        return vectors
    
    def _compute_similarity_matrix(self, embeddings: np.ndarray) -> np.ndarray:
        """计算相似度矩阵
        
        Args:
            embeddings: 向量矩阵
        
        Returns:
            相似度矩阵
        """
        # 归一化
        norms = np.linalg.norm(embeddings, axis=1, keepdims=True)
        norms[norms == 0] = 1
        normalized = embeddings / norms
        
        # 余弦相似度矩阵
        similarity_matrix = np.dot(normalized, normalized.T)
        
        return similarity_matrix
    
    def _find_duplicate_groups(self, texts: List[str]) -> List[List[int]]:
        """查找重复组 - 优化版
        
        Args:
            texts: 文本列表
        
        Returns:
            重复组列表，每组包含重复文本的索引
        """
        n = len(texts)
        
        if n == 0:
            return []
        
        # 批量编码
        embeddings = self._batch_encode(texts)
        
        # 尝试使用 FAISS 加速
        if self.use_faiss and self._model != "fallback":
            try:
                return self._find_duplicates_faiss(texts, embeddings)
            except ImportError:
                logger.warning("FAISS 未安装，使用标准方法")
        
        # 计算相似度矩阵
        similarity_matrix = self._compute_similarity_matrix(embeddings)
        
        # 查找重复组
        visited = [False] * n
        duplicate_groups = []
        
        for i in range(n):
            if visited[i]:
                continue
            
            group = [i]
            visited[i] = True
            
            for j in range(i + 1, n):
                if visited[j]:
                    continue
                
                if similarity_matrix[i, j] >= self.threshold:
                    group.append(j)
                    visited[j] = True
            
            if len(group) > 1:
                duplicate_groups.append(group)
        
        return duplicate_groups
    
    def _find_duplicates_faiss(self, texts: List[str], embeddings: np.ndarray) -> List[List[int]]:
        """使用 FAISS 加速查找重复
        
        Args:
            texts: 文本列表
            embeddings: 向量矩阵
        
        Returns:
            重复组列表
        """
        try:
            import faiss
            
            # 创建 FAISS 索引
            dimension = embeddings.shape[1]
            index = faiss.IndexFlatIP(dimension)  # 内积（余弦相似度，假设已归一化）
            
            # 归一化
            norms = np.linalg.norm(embeddings, axis=1, keepdims=True)
            norms[norms == 0] = 1
            normalized = embeddings / norms
            
            index.add(normalized.astype('float32'))
            
            # 搜索相似项
            n = len(texts)
            visited = [False] * n
            duplicate_groups = []
            
            for i in range(n):
                if visited[i]:
                    continue
                
                # 搜索最近邻
                query = normalized[i:i+1].astype('float32')
                k = min(n, 100)  # 限制搜索数量
                distances, indices = index.search(query, k)
                
                group = [i]
                visited[i] = True
                
                for j, dist in zip(indices[0], distances[0]):
                    if j == i or visited[j]:
                        continue
                    
                    if dist >= self.threshold:
                        group.append(j)
                        visited[j] = True
                
                if len(group) > 1:
                    duplicate_groups.append(group)
            
            return duplicate_groups
            
        except ImportError:
            raise ImportError("FAISS 未安装")
    
    def deduplicate(self, 
                   items: List[Dict],
                   text_key: str = "instruction") -> DedupResult:
        """执行去重
        
        Args:
            items: 数据列表
            text_key: 用于去重的文本字段名
        
        Returns:
            DedupResult 实例
        """
        self._load_model()
        
        original_count = len(items)
        
        if original_count == 0:
            return DedupResult(
                original_count=0,
                deduplicated_count=0,
                removed_count=0,
                duplicate_groups=[],
                kept_indices=[]
            )
        
        # 提取文本
        texts = [item.get(text_key, "") for item in items]
        
        # 查找重复组
        duplicate_groups = self._find_duplicate_groups(texts)
        
        # 确定保留的索引
        removed_indices = set()
        for group in duplicate_groups:
            # 保留第一个，移除其余
            for idx in group[1:]:
                removed_indices.add(idx)
        
        kept_indices = [i for i in range(original_count) if i not in removed_indices]
        
        return DedupResult(
            original_count=original_count,
            deduplicated_count=len(kept_indices),
            removed_count=len(removed_indices),
            duplicate_groups=duplicate_groups,
            kept_indices=kept_indices
        )
    
    def deduplicate_and_filter(self,
                              items: List[Dict],
                              text_key: str = "instruction") -> List[Dict]:
        """执行去重并返回过滤后的数据
        
        Args:
            items: 数据列表
            text_key: 用于去重的文本字段名
        
        Returns:
            去重后的数据列表
        """
        result = self.deduplicate(items, text_key)
        return [items[i] for i in result.kept_indices]
    
    def generate_report(self,
                       items: List[Dict],
                       text_key: str = "instruction") -> Dict:
        """生成去重报告
        
        Args:
            items: 数据列表
            text_key: 用于去重的文本字段名
        
        Returns:
            去重报告字典
        """
        result = self.deduplicate(items, text_key)
        
        return {
            "original_count": result.original_count,
            "deduplicated_count": result.deduplicated_count,
            "removed_count": result.removed_count,
            "removal_rate": result.removed_count / result.original_count if result.original_count > 0 else 0,
            "duplicate_groups_count": len(result.duplicate_groups),
            "avg_group_size": np.mean([len(g) for g in result.duplicate_groups]) if result.duplicate_groups else 0,
            "threshold": self.threshold
        }
    
    def find_similar_pairs(self,
                          items: List[Dict],
                          text_key: str = "instruction",
                          top_k: int = 10) -> List[Tuple[int, int, float]]:
        """查找最相似的文本对
        
        Args:
            items: 数据列表
            text_key: 用于比较的文本字段名
            top_k: 返回前 k 个最相似对
        
        Returns:
            相似对列表 [(idx1, idx2, similarity), ...]
        """
        self._load_model()
        
        texts = [item.get(text_key, "") for item in items]
        n = len(texts)
        
        if n < 2:
            return []
        
        # 批量编码
        embeddings = self._batch_encode(texts)
        
        # 计算相似度矩阵
        similarity_matrix = self._compute_similarity_matrix(embeddings)
        
        # 提取相似对
        pairs = []
        for i in range(min(n, 200)):  # 增加到 200
            for j in range(i + 1, min(n, 200)):
                sim = similarity_matrix[i, j]
                if sim >= self.threshold * 0.8:
                    pairs.append((i, j, float(sim)))
        
        # 按相似度排序
        pairs.sort(key=lambda x: x[2], reverse=True)
        
        return pairs[:top_k]
