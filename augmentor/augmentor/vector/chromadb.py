# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""ChromaDB 向量数据库实现

依赖 chromadb 包；未安装时在构造阶段抛出明确的 ImportError。
"""

import json
import logging
from pathlib import Path
from typing import List, Dict, Optional, Any

import numpy as np

from .base import VectorDB, normalize_vectors
from ..exceptions import VectorError

logger = logging.getLogger(__name__)

# ChromaDB 元数据仅支持标量值
ALLOWED_SCALAR_TYPES = (str, int, float, bool)


def sanitize_metadata(metadata: Dict) -> Dict:
    """将元数据转换为 ChromaDB 可接受的标量字典

    嵌套结构会被序列化为 JSON 字符串。

    Args:
        metadata: 原始元数据

    Returns:
        清洗后的元数据
    """
    cleaned: Dict[str, Any] = {}
    for key, value in (metadata or {}).items():
        if value is None:
            cleaned[key] = ""
        elif isinstance(value, ALLOWED_SCALAR_TYPES):
            cleaned[key] = value
        else:
            cleaned[key] = json.dumps(value, ensure_ascii=False)
    return cleaned


class ChromaDB(VectorDB):
    """ChromaDB 向量数据库"""

    def __init__(self,
                 dimension: int = 384,
                 collection: str = "default",
                 storage_dir: Optional[str] = None,
                 persist: bool = True):
        """初始化 ChromaDB

        Args:
            dimension: 向量维度
            collection: 集合名称
            storage_dir: 持久化目录
            persist: 是否使用持久化客户端

        Raises:
            ImportError: chromadb 未安装
        """
        super().__init__(dimension=dimension, collection=collection)

        try:
            import chromadb
        except ImportError as e:
            raise ImportError(
                "ChromaDB 后端需要安装 chromadb: pip install chromadb"
            ) from e

        self.storage_dir = Path(storage_dir) if storage_dir else None

        if persist and self.storage_dir:
            self.storage_dir.mkdir(parents=True, exist_ok=True)
            self._client = chromadb.PersistentClient(path=str(self.storage_dir))
        else:
            self._client = chromadb.Client()

        self._collection = self._client.get_or_create_collection(
            name=collection,
            metadata={"hnsw:space": "cosine"}
        )

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

        existing = set(self._collection.get(ids=new_ids)["ids"])
        if existing:
            raise VectorError(f"ID 已存在: {sorted(existing)[:5]}")

        self._collection.add(
            ids=new_ids,
            embeddings=normalized.tolist(),
            metadatas=[sanitize_metadata(m) for m in metadata]
        )

        self._ids.extend(new_ids)
        self._metadata.extend(metadata)

        logger.info(f"添加 {len(new_ids)} 个向量到 ChromaDB，当前总数 {self._collection.count()}")
        return new_ids

    def search(self, query: np.ndarray, top_k: int = 5) -> List[Dict[str, Any]]:
        """检索相似向量

        Args:
            query: 查询向量
            top_k: 返回条数

        Returns:
            结果列表，按相似度降序
        """
        if self._collection.count() == 0:
            return []

        query_array = np.asarray(query, dtype=np.float32)
        if query_array.ndim == 1:
            query_array = query_array.reshape(1, -1)

        if query_array.shape[1] != self.dimension:
            raise VectorError(
                f"查询向量维度不匹配: 期望 {self.dimension}，实际 {query_array.shape[1]}"
            )

        normalized = normalize_vectors(query_array)
        result = self._collection.query(
            query_embeddings=normalized.tolist(),
            n_results=min(top_k, self._collection.count()),
            include=["metadatas", "distances"]
        )

        ids = result.get("ids", [[]])[0]
        metadatas = result.get("metadatas", [[]])[0]
        distances = result.get("distances", [[]])[0]

        # cosine 距离转相似度
        return [
            {
                "id": vid,
                "score": float(1.0 - distance),
                "metadata": metadata
            }
            for vid, metadata, distance in zip(ids, metadatas, distances)
        ]

    def delete(self, ids: List[str]) -> int:
        """删除向量

        Args:
            ids: 待删除的 ID 列表

        Returns:
            实际删除数量
        """
        if not ids:
            return 0

        existing = set(self._collection.get(ids=ids)["ids"])
        if not existing:
            return 0

        self._collection.delete(ids=sorted(existing))

        kept = [
            (vid, meta) for vid, meta in zip(self._ids, self._metadata)
            if vid not in existing
        ]
        self._ids = [vid for vid, _ in kept]
        self._metadata = [meta for _, meta in kept]

        logger.info(f"从 ChromaDB 删除 {len(existing)} 个向量")
        return len(existing)

    def count(self) -> int:
        """获取向量数量

        Returns:
            向量数量
        """
        return self._collection.count()

    def clear(self):
        """清空数据库"""
        self._client.delete_collection(name=self.collection)
        self._collection = self._client.get_or_create_collection(
            name=self.collection,
            metadata={"hnsw:space": "cosine"}
        )
        super().clear()
