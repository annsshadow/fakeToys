"""向量数据库模块

提供统一的向量库接口与工厂方法。
"""

import logging
from typing import Optional

from .base import VectorDB, normalize_vectors
from .faiss import FAISSDB
from .chromadb import ChromaDB

logger = logging.getLogger(__name__)

SUPPORTED_BACKENDS = ["faiss", "chromadb"]


def create_vector_db(backend: str = "faiss",
                     dimension: int = 384,
                     collection: str = "default",
                     storage_dir: Optional[str] = None) -> VectorDB:
    """创建向量数据库实例

    Args:
        backend: 后端类型（faiss / chromadb）
        dimension: 向量维度
        collection: 集合名称
        storage_dir: 持久化目录

    Returns:
        VectorDB 实例

    Raises:
        ValueError: 不支持的后端类型
        ImportError: 所选后端依赖未安装
    """
    if backend not in SUPPORTED_BACKENDS:
        raise ValueError(
            f"不支持的向量数据库后端: {backend}。支持: {SUPPORTED_BACKENDS}"
        )

    if backend == "faiss":
        return FAISSDB(
            dimension=dimension,
            collection=collection,
            storage_dir=storage_dir
        )

    return ChromaDB(
        dimension=dimension,
        collection=collection,
        storage_dir=storage_dir
    )


__all__ = [
    "VectorDB",
    "FAISSDB",
    "ChromaDB",
    "create_vector_db",
    "normalize_vectors",
    "SUPPORTED_BACKENDS"
]
