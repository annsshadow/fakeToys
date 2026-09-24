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
from ..cache import MemoryCache
from ..exceptions import VectorError

logger = logging.getLogger(__name__)


class FAISSDB(VectorDB):
    """FAISS 向量数据库"""

    # 查询结果缓存容量上限（条）。查询向量内容千变万化，原实现是无上限 dict，
    # 长跑进程里每个新查询都会永久驻留一份 top-k 结果。
    _QUERY_CACHE_MAX = 256

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
        # 写缓冲按几何倍率增长，`_vectors` 只是它的前缀视图（见下面的属性）。
        # 原实现每次 `add_vectors` 都 `np.vstack` 整份矩阵：逐条写入时复制总量
        # 是 n²/2 行（真实规模 6902 条 × 384 维 ≈ 36 GB memcpy）。
        self._buffer = np.zeros((0, dimension), dtype=np.float32)
        self._size = 0
        # `_ids` 的镜像集合：重复 ID 检测原本每次 `set(self._ids)` 全量重建，
        # 同样是逐条写入时的 O(n²)。
        self._id_set: set = set()
        self._index = None
        self._faiss = None
        self._query_cache = MemoryCache(max_size=self._QUERY_CACHE_MAX)

        self._init_index()

    @property
    def _vectors(self) -> np.ndarray:
        """行与 `_ids` / `_metadata` 对齐的向量矩阵（写缓冲的有效前缀）"""
        return self._buffer[:self._size]

    @_vectors.setter
    def _vectors(self, matrix: np.ndarray) -> None:
        matrix = np.ascontiguousarray(matrix, dtype=np.float32)
        self._buffer = matrix
        self._size = matrix.shape[0]

    def _grow(self, rows_needed: int) -> None:
        """扩容写缓冲（只在装不下时才复制，所以逐条 add 的复制总量是 O(n)）"""
        capacity = self._buffer.shape[0]
        buffer = np.zeros((max(rows_needed, capacity * 2, 8), self.dimension),
                          dtype=np.float32)
        if self._size:
            buffer[:self._size] = self._buffer[:self._size]
        self._buffer = buffer

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
            raise VectorError("ids 长度与 vectors 不一致")
        if len(set(new_ids)) != len(new_ids):
            raise VectorError("ids 中存在重复值")
        if len(self._id_set) != len(self._ids):
            # 镜像集合与 `_ids` 长度不符 → 有人绕过写接口改了 `_ids`，整份重建。
            # 长度相同而内容不同是重建不到的，但那种改法本身就会让 `_vectors`
            # 与 `_ids` 错位，不在受支持的用法里。
            self._id_set = set(self._ids)
        duplicates = self._id_set.intersection(new_ids)
        if duplicates:
            raise VectorError(f"ID 已存在: {sorted(duplicates)[:5]}")

        if self._index is not None:
            self._index.add(normalized)
        rows = self._size + normalized.shape[0]
        if rows > self._buffer.shape[0]:
            self._grow(rows)
        self._buffer[self._size:rows] = normalized
        self._size = rows

        self._ids.extend(new_ids)
        self._metadata.extend(metadata)
        self._id_set.update(new_ids)
        self._invalidate_query_cache()

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
            raise VectorError(
                f"查询向量维度不匹配: 期望 {self.dimension}，实际 {query_array.shape[1]}"
            )

        normalized_query = normalize_vectors(query_array)
        k = min(top_k, len(self._ids))

        # 查询缓存：键必须覆盖所有影响结果的输入。
        # 原实现只用查询向量做键，于是 search(q, top_k=5) 之后
        # search(q, top_k=10) 会命中那条已经被截断成 5 条的缓存。
        digest = hashlib.md5(query_array.tobytes()).hexdigest()[:12]
        query_key = (digest, k)

        cached_results = self._query_cache.get(query_key)
        if cached_results is not None:
            # 兜底：若有人绕过写接口直接改了 _ids，过滤掉已失效的条目；
            # 一旦发生过滤就整条作废重算，避免返回「少几条」的残缺结果。
            valid_results = [r for r in cached_results if r["id"] in self._ids]
            if len(valid_results) == len(cached_results):
                logger.debug(f"查询缓存命中: {digest}/k={k}")
                return [
                    {"id": r["id"], "score": r["score"], "metadata": self.get_metadata(r["id"])}
                    for r in valid_results
                ]

        if self._index is not None:
            distances, indices = self._index.search(normalized_query, k)
            pairs = list(zip(indices[0], distances[0]))
        else:
            # 分块计算：避免大矩阵直接计算导致内存峰值
            matrix = self._vectors
            chunk_size = max(1000, len(matrix) // 4 + 1)
            best_pairs = []
            for chunk_start in range(0, len(matrix), chunk_size):
                chunk_end = min(chunk_start + chunk_size, len(matrix))
                chunk_vectors = matrix[chunk_start:chunk_end]
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

        self._query_cache.set(query_key, results)

        return results

    def _invalidate_query_cache(self):
        """写操作后失效查询缓存

        缓存的是「某个索引状态下某个查询的 top-k」；索引一变结果就可能变，
        继续命中会让新写入的向量永远查不出来。
        """
        self._query_cache.clear()
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
        self._id_set = set(self._ids)

        if self._faiss is not None:
            self._index = self._faiss.IndexFlatIP(self.dimension)
            if len(self._vectors):
                self._index.add(self._vectors)

        self._invalidate_query_cache()

        logger.info(f"删除 {removed} 个向量，当前总数 {len(self._ids)}")
        return removed

    def clear(self):
        """清空数据库"""
        super().clear()
        self._vectors = np.zeros((0, self.dimension), dtype=np.float32)
        self._id_set = set()
        if self._faiss is not None:
            self._index = self._faiss.IndexFlatIP(self.dimension)
        self._invalidate_query_cache()

    def persist(self):
        """持久化到磁盘

        Raises:
            VectorError: 未配置 storage_dir
        """
        if self.storage_dir is None:
            raise VectorError("未配置 storage_dir，无法持久化")

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
        self._id_set = set(self._ids)

        if self._faiss is not None:
            self._index = self._faiss.IndexFlatIP(self.dimension)
            if len(vectors):
                self._index.add(vectors)

        self._invalidate_query_cache()

        logger.info(f"从 {self.storage_dir} 加载 {len(self._ids)} 个向量")
        return True
