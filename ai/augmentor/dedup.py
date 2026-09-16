"""智能去重模块"""

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
    """智能去重器"""
    
    def __init__(self, threshold: float = 0.9):
        """初始化去重器
        
        Args:
            threshold: 相似度阈值，高于此值被视为重复
        """
        if threshold < 0 or threshold > 1:
            raise ValueError("阈值必须在 0-1 之间")
        
        self.threshold = threshold
        self._model = None
    
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
    
    def _calculate_similarity(self, text1: str, text2: str) -> float:
        """计算文本相似度
        
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
    
    def _find_duplicate_groups(self, texts: List[str]) -> List[List[int]]:
        """查找重复组
        
        Args:
            texts: 文本列表
        
        Returns:
            重复组列表，每组包含重复文本的索引
        """
        n = len(texts)
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
                
                similarity = self._calculate_similarity(texts[i], texts[j])
                if similarity >= self.threshold:
                    group.append(j)
                    visited[j] = True
            
            if len(group) > 1:
                duplicate_groups.append(group)
        
        return duplicate_groups
    
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
        
        # 计算所有对的相似度
        pairs = []
        for i in range(min(n, 100)):  # 限制数量避免性能问题
            for j in range(i + 1, min(n, 100)):
                similarity = self._calculate_similarity(texts[i], texts[j])
                if similarity >= self.threshold * 0.8:  # 放宽阈值以找到更多潜在重复
                    pairs.append((i, j, similarity))
        
        # 按相似度排序
        pairs.sort(key=lambda x: x[2], reverse=True)
        
        return pairs[:top_k]
