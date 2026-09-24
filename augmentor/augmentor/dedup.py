# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""智能去重模块 - 优化版"""

import logging
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass
import numpy as np
from .model_manager import model_manager
from .exceptions import DedupError

logger = logging.getLogger(__name__)

# 单个相似度块的单元数上限（float32）。用于把峰值内存钉在常量级：
# _MAX_BLOCK_CELLS × 4B = 16MB。无论数据集多大，任一时刻只持有这么多。
# 这是去重不再随 n 平方增长内存的关键，改动前请先读 _row_block_size 的说明。
_MAX_BLOCK_CELLS = 4_000_000


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
            raise DedupError("阈值必须在 0-1 之间")
        
        self.threshold = threshold
        self.use_faiss = use_faiss
        self._model = None
        self._faiss_index = None
    
    def _load_model(self):
        """延迟加载模型（使用共享实例）"""
        if self._model is None:
            self._model = model_manager.get_sentence_model()
    
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
    
    @staticmethod
    def _normalize(embeddings: np.ndarray) -> np.ndarray:
        """按行做 L2 归一化（零向量保持零向量，避免除零）"""
        norms = np.linalg.norm(embeddings, axis=1, keepdims=True)
        norms[norms == 0] = 1
        return embeddings / norms

    def _row_block_size(self, n: int, requested: int) -> int:
        """把行块大小压到「单块单元数 ≤ _MAX_BLOCK_CELLS」

        这是峰值内存不随 n 平方增长的关键：无论 n 多大，任一时刻持有的
        相似度块最多 `_MAX_BLOCK_CELLS` 个 float32。

        Args:
            n: 数据总量
            requested: 期望的行块大小

        Returns:
            实际可用的行块大小（至少为 1）
        """
        if n <= 0:
            return 1
        return max(1, min(requested, _MAX_BLOCK_CELLS // n))

    def _iter_similarity_row_blocks(self, normalized: np.ndarray, block_rows: int):
        """按行分块产出 `(行起点, 行终点, 相似度块)`

        与 :meth:`_compute_similarity_matrix_chunked` 的本质区别：这里
        **从不构造完整的 n×n 矩阵**，每次只产出一个 `(block_rows × (n-start))`
        的块，调用方消费完该块后即可被回收。

        列从 `start` 起算，而不是从 0：贪心归组只在 `j > i` 的方向上消费
        （见 :meth:`_greedy_group_from_row` 的 `row[i + 1 - start:]`），而块内
        最小的行号就是 `start`，所以列 < start 的那些结果**按定义没有读者**。
        以前每次都算满 `block_rows × n`，等于把整个下三角白算一遍——实测
        全量 6902 条真实数据时，相似度这一步占 31.2 s 里的 30.0 s，
        其中约一半 flops 落在这段被丢弃的列上。

        Args:
            normalized: 已 L2 归一化的向量矩阵
            block_rows: 每个块包含的行数

        Yields:
            (start, end, block)，block 形状为 (end-start, n-start)，
            其列下标 c 对应全局下标 c + start
        """
        n = len(normalized)
        for start in range(0, n, block_rows):
            end = min(start + block_rows, n)
            block = np.dot(normalized[start:end], normalized[start:].T)
            yield start, end, block
            # 交付后立刻松开生成器自身的引用：否则恢复执行时会先算好下一块、
            # 再覆盖这个变量，导致新旧两块同时存活（峰值凭空翻倍）。
            block = None

    def _compute_similarity_matrix_chunked(self, embeddings: np.ndarray, chunk_size: int = 1000) -> np.ndarray:
        """分块计算**完整**相似度矩阵

        .. warning::
           本方法按定义必须返回 n×n 矩阵，因此内存占用是 **O(n²)**——
           分块只降低了临时峰值，并不能避免最终那个大矩阵的分配。
           n=50000 时该矩阵约 10GB，会直接 OOM。

           生产路径（去重分组）已改用 :meth:`_iter_similarity_row_blocks`。
           本方法仅保留给小数据集的一次性检视与测试使用。

        Args:
            embeddings: 向量矩阵
            chunk_size: 分块大小

        Returns:
            相似度矩阵（n×n）
        """
        n = len(embeddings)
        normalized = self._normalize(embeddings)

        # 分块计算相似度
        similarity_matrix = np.zeros((n, n), dtype=np.float32)

        for i in range(0, n, chunk_size):
            end_i = min(i + chunk_size, n)
            chunk_i = normalized[i:end_i]

            for j in range(i, n, chunk_size):
                end_j = min(j + chunk_size, n)
                chunk_j = normalized[j:end_j]

                # 计算块间相似度
                block_sim = np.dot(chunk_i, chunk_j.T)

                # 填充对称矩阵
                similarity_matrix[i:end_i, j:end_j] = block_sim
                if i != j:
                    similarity_matrix[j:end_j, i:end_i] = block_sim.T

        return similarity_matrix

    def _find_duplicate_groups_chunked(self, texts: List[str], chunk_size: int = 1000) -> List[List[int]]:
        """按行分块查找重复组（内存占用与 n 成线性，而非平方）

        分组语义与逐行贪心完全一致：按索引升序扫描，每个尚未归组的 i 成为
        新组的代表，把其后所有「与 i 相似度达标、且尚未归组」的 j 并入该组。

        Args:
            texts: 文本列表
            chunk_size: 期望的行块大小（会被 `_MAX_BLOCK_CELLS` 进一步压缩）

        Returns:
            重复组列表
        """
        n = len(texts)

        if n == 0:
            return []

        # 批量编码
        embeddings = np.asarray(self._batch_encode(texts), dtype=np.float32)
        normalized = self._normalize(embeddings)

        block_rows = self._row_block_size(n, chunk_size)

        # 查找重复组
        visited = [False] * n
        duplicate_groups: List[List[int]] = []

        for start, end, block in self._iter_similarity_row_blocks(normalized, block_rows):
            for offset, i in enumerate(range(start, end)):
                if visited[i]:
                    continue

                group = self._greedy_group_from_row(block, offset, i, start, visited)
                if len(group) > 1:
                    duplicate_groups.append(group)

            # 显式释放本块。注意归组逻辑必须封装在方法里：若在此处直接写
            # `row = block[offset]`，row 作为**视图**会在 del block 之后继续
            # 持有整个块，导致生成器算下一块时新旧两块同时存活（峰值翻倍）。
            del block

        return duplicate_groups

    def _greedy_group_from_row(self,
                               block: np.ndarray,
                               offset: int,
                               i: int,
                               start: int,
                               visited: List[bool]) -> List[int]:
        """以 i 为组代表，把其后「相似度达标且尚未归组」的 j 并入同一组

        贪心语义与逐行扫描一致：只在 i 之后找，且已归组的 j 不再被认领。

        Args:
            block: 当前相似度块，形状 (block_rows, n - start)，列下标 c 对应全局 c + start
            offset: i 在 block 中的行下标
            i: 全局行索引，作为组代表
            start: 本块的起始全局行号（用于把块内列下标换回全局下标）
            visited: 归组标记（原地修改）

        Returns:
            组内索引列表（含代表 i 本身）
        """
        group = [i]
        visited[i] = True

        # 先在 C 层筛出相似度达标的候选，再在 Python 层做归组。
        # 若写成 `for j in range(i+1, n)` 逐元素比较，会把 O(n²) 次比较
        # 全部压到解释器里，大数据集下慢到不可用。
        # 块只覆盖列 [start, n)，故全局列 j 在块内是 j - start；i >= start，
        # 切片起点 `i + 1 - start` 恒 >= 1（跳过自己及自己左侧的列）。
        row = block[offset]
        candidates = np.flatnonzero(row[i + 1 - start:] >= self.threshold) + i + 1
        for j in candidates:
            j = int(j)
            if not visited[j]:
                group.append(j)
                visited[j] = True

        return group
    
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
            normalized = self._normalize(embeddings)
            
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
        
        # 查找重复组：按行分块流式计算，峰值内存与 n 成线性
        duplicate_groups = self._find_duplicate_groups_chunked(texts)
        
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
        
        # 批量编码（需对全部文本编码：fallback 的词表由全体文本决定，
        # 只编码前 200 条会改变向量本身，进而改变相似对结果）
        embeddings = np.asarray(self._batch_encode(texts), dtype=np.float32)

        # 只需前 limit 行的两两相似度，因此只算这一个 (limit × limit) 小块，
        # 不再构造完整的 n×n 矩阵。
        limit = min(n, 200)
        head = self._normalize(embeddings[:limit])
        similarity_block = np.dot(head, head.T)

        # 提取相似对
        pairs = []
        for i in range(limit):
            for j in range(i + 1, limit):
                sim = similarity_block[i, j]
                if sim >= self.threshold * 0.8:
                    pairs.append((i, j, float(sim)))
        
        # 按相似度排序
        pairs.sort(key=lambda x: x[2], reverse=True)
        
        return pairs[:top_k]
