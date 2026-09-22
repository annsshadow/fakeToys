# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""FAISS 向量数据库实现

优先使用 faiss-cpu 加速检索，未安装时退化为 numpy 精确检索，接口保持一致。
"""

import hashlib
import json
import logging
from pathlib import Path
from typing import List, Dict, Optional, Any

import numpy as np

from .base import VectorDB, normalize_vectors

logger = logging.getLogger(__name__)


class FAISSDB(VectorDB):
    """FAISS 向量数据库"""

    def __init__(self,
                 dimension: int = 384,
                 collection: str = "default",
                 storage_dir: Optional[str] = None):
        """初始化 FAISS 数据库

        Args:
            dimension: 向量维度
            collection: 集合名称
            storage_dir: 持久化目录，为 None 时不落盘
        """
        super().__init__(dimension=dimension, collection=collection)

        self.storage_dir = Path(storage_dir) if storage_dir else None
        self._vectors = np.zeros((0, dimension), dtype=np.float32)
        self._index = None
        self._faiss = None

        self._init_index()

    def _init_index(self):
        """初始化 FAISS 索引（不可用时退化为 numpy）"""
        try:
            import faiss
            self._faiss = faiss
            self._index = faiss.IndexFlatIP(self.dimension)
            logger.info("FAISS 索引初始化成功")
        except ImportError:
            self._faiss = None
            self._index = None
            logger.warning("faiss 未安装，使用 numpy 精确检索")

    @property
    def backend(self) -> str:
        """当前实际使用的后端

        Returns:
            faiss 或 numpy
        """
        return "faiss" if self._index is not None else "numpy"

    def add_vectors(self,
                    vectors: List[np.ndarray],
                    metadata: List[Dict],
                    ids: Optional[List[str]] = None) -> List[str]:
        """添加向量

        Args:
            vectors: 向量列表
            metadata: 元数据列表
            ids: 可选的 ID 列表

        Returns:
            写入的 ID 列表
        """
        self._validate_lengths(vectors, metadata)
        if not vectors:
            return []

        self._validate_vectors(vectors)

        normalized = normalize_vectors(np.array(vectors, dtype=np.float32))

        new_ids = ids or self._generate_ids(len(vectors))
        if len(new_ids) != len(vectors):
            raise ValueError("ids 长度与 vectors 不一致")
        if len(set(new_ids)) != len(new_ids):
            raise ValueError("ids 中存在重复值")
        duplicates = set(new_ids) & set(self._ids)
        if duplicates:
            raise ValueError(f"ID 已存在: {sorted(duplicates)[:5]}")

        if self._index is not None:
            self._index.add(normalized)
        self._vectors = np.vstack([self._vectors, normalized])

        self._ids.extend(new_ids)
        self._metadata.extend(metadata)

        logger.info(f"添加 {len(new_ids)} 个向量，当前总数 {len(self._ids)}")
        return new_ids

    def search(self, query: np.ndarray, top_k: int = 5) -> List[Dict[str, Any]]:
        """检索相似向量

        Args:
            query: 查询向量
            top_k: 返回条数

        Returns:
            结果列表，按相似度降序
        """
        if not self._ids:
            return []

        query_array = np.asarray(query, dtype=np.float32)
        if query_array.ndim == 1:
            query_array = query_array.reshape(1, -1)

        if query_array.shape[1] != self.dimension:
            raise ValueError(
                f"查询向量维度不匹配: 期望 {self.dimension}，实际 {query_array.shape[1]}"
            )

        normalized_query = normalize_vectors(query_array)
        k = min(top_k, len(self._ids))
        
        # 向量查询优化：缓存查询结果（基于查询向量内容的简单缓存）
        query_key = hashlib.md5(query_array.tobytes()).hexdigest()[:12]
        if hasattr(self, '_query_cache') and query_key in self._query_cache:
            cached_results = self._query_cache[query_key]
            # 根据当前索引状态过滤有效结果
            valid_results = [
                r for r in cached_results
                if r["id"] in self._ids
            ]
            if valid_results:
                logger.debug(f"查询缓存命中（优化）：返回缓存结果")
                return [{"id": r["id"], "score": r["score"], "metadata": self.get_metadata(r["id"])} for r in valid_results[:k]]
        
        if self._index is not None:
            distances, indices = self._index.search(normalized_query, k)
            pairs = list(zip(indices[0], distances[0]))
        else:
            # 分块计算：避免大矩阵直接计算导致内存峰值
            chunk_size = max(1000, len(self._vectors) // 4 + 1)
            best_pairs = []
            for chunk_start in range(0, len(self._vectors), chunk_size):
                chunk_end = min(chunk_start + chunk_size, len(self._vectors))
                chunk_vectors = self._vectors[chunk_start:chunk_end]
                similarities = (chunk_vectors @ normalized_query[0])
                top_in_chunk = np.argsort(-similarities)[:k]
                for idx in top_in_chunk:
                    best_pairs.append((chunk_start + int(idx), float(similarities[idx])))
            best_pairs.sort(key=lambda x: x[1], reverse=True)
            pairs = best_pairs[:k]

        results = []
        for index, score in pairs:
            if index < 0:
                continue
            results.append({
                "id": self._ids[index],
                "score": float(score),
                "metadata": self._metadata[index]
            })

        # 保存搜索结果到缓存（查询优化：避免重复计算相同查询）
        if not hasattr(self, '_query_cache'):
            self._query_cache = {}
        self._query_cache[query_key] = results
        
        return results
    
    def delete(self, ids: List[str]) -> int:
        """删除向量

        由于 FAISS IndexFlatIP 不支持原地删除，这里重建索引。

        Args:
            ids: 待删除的 ID 列表

        Returns:
            实际删除数量
        """
        if not ids:
            return 0

        targets = set(ids)
        keep_indices = [i for i, vid in enumerate(self._ids) if vid not in targets]
        removed = len(self._ids) - len(keep_indices)

        if removed == 0:
            return 0

        self._ids = [self._ids[i] for i in keep_indices]
        self._metadata = [self._metadata[i] for i in keep_indices]
        self._vectors = self._vectors[keep_indices] if keep_indices else np.zeros(
            (0, self.dimension), dtype=np.float32
        )

        if self._faiss is not None:
            self._index = self._faiss.IndexFlatIP(self.dimension)
            if len(self._vectors):
                self._index.add(self._vectors)

        logger.info(f"删除 {removed} 个向量，当前总数 {len(self._ids)}")
        return removed

    def clear(self):
        """清空数据库"""
        super().clear()
        self._vectors = np.zeros((0, self.dimension), dtype=np.float32)
        if self._faiss is not None:
            self._index = self._faiss.IndexFlatIP(self.dimension)

    def persist(self):
        """持久化到磁盘

        Raises:
            ValueError: 未配置 storage_dir
        """
        if self.storage_dir is None:
            raise ValueError("未配置 storage_dir，无法持久化")

        self.storage_dir.mkdir(parents=True, exist_ok=True)

        np.save(self.storage_dir / f"{self.collection}_vectors.npy", self._vectors)
        with open(self.storage_dir / f"{self.collection}_meta.json", 'w', encoding='utf-8') as f:
            json.dump(
                {"ids": self._ids, "metadata": self._metadata, "dimension": self.dimension},
                f, ensure_ascii=False
            )

        logger.info(f"持久化 {len(self._ids)} 个向量到 {self.storage_dir}")

    def load(self) -> bool:
        """从磁盘加载

        Returns:
            是否加载成功
        """
        if self.storage_dir is None:
            return False

        vectors_path = self.storage_dir / f"{self.collection}_vectors.npy"
        meta_path = self.storage_dir / f"{self.collection}_meta.json"

        if not vectors_path.exists() or not meta_path.exists():
            return False

        with open(meta_path, 'r', encoding='utf-8') as f:
            payload = json.load(f)

        vectors = np.load(vectors_path).astype(np.float32)
        if vectors.ndim == 1:
            vectors = vectors.reshape(-1, self.dimension)

        self._ids = payload.get("ids", [])
        self._metadata = payload.get("metadata", [])
        self._vectors = vectors

        if self._faiss is not None:
            self._index = self._faiss.IndexFlatIP(self.dimension)
            if len(vectors):
                self._index.add(vectors)

        logger.info(f"从 {self.storage_dir} 加载 {len(self._ids)} 个向量")
        return True
