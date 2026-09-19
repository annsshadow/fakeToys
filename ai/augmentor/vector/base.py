"""向量数据库抽象接口"""

import logging
from abc import ABC, abstractmethod
from typing import List, Dict, Optional, Any

import numpy as np

logger = logging.getLogger(__name__)


def normalize_vectors(vectors: np.ndarray) -> np.ndarray:
    """对向量做 L2 归一化

    Args:
        vectors: 向量矩阵 (n, d)

    Returns:
        归一化后的向量矩阵
    """
    vectors = np.asarray(vectors, dtype=np.float32)
    if vectors.ndim == 1:
        vectors = vectors.reshape(1, -1)

    norms = np.linalg.norm(vectors, axis=1, keepdims=True)
    norms[norms == 0] = 1.0
    return vectors / norms


class VectorDB(ABC):
    """向量数据库抽象基类"""

    def __init__(self, dimension: int = 384, collection: str = "default"):
        """初始化向量数据库

        Args:
            dimension: 向量维度
            collection: 集合名称
        """
        if dimension <= 0:
            raise ValueError("向量维度必须为正整数")

        self.dimension = dimension
        self.collection = collection
        self._ids: List[str] = []
        self._metadata: List[Dict] = []

    def _validate_vectors(self, vectors: List[np.ndarray]):
        """校验向量列表

        Args:
            vectors: 向量列表

        Raises:
            ValueError: 向量维度不匹配
        """
        for vector in vectors:
            array = np.asarray(vector)
            if array.ndim != 1 or array.shape[0] != self.dimension:
                raise ValueError(
                    f"向量维度不匹配: 期望 {self.dimension}，实际 {array.shape}"
                )

    def _validate_lengths(self, vectors: List[np.ndarray], metadata: List[Dict]):
        """校验向量与元数据长度一致

        Args:
            vectors: 向量列表
            metadata: 元数据列表

        Raises:
            ValueError: 长度不一致
        """
        if len(vectors) != len(metadata):
            raise ValueError("vectors 与 metadata 长度不一致")

    def _generate_ids(self, count: int, start: Optional[int] = None) -> List[str]:
        """生成自增 ID

        Args:
            count: 数量
            start: 起始序号，为 None 时接续已有数量

        Returns:
            ID 列表
        """
        base = len(self._ids) if start is None else start
        return [f"{self.collection}-{base + i}" for i in range(count)]

    @abstractmethod
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
        raise NotImplementedError

    @abstractmethod
    def search(self, query: np.ndarray, top_k: int = 5) -> List[Dict[str, Any]]:
        """检索相似向量

        Args:
            query: 查询向量
            top_k: 返回条数

        Returns:
            结果列表 [{"id":..., "score":..., "metadata":...}]
        """
        raise NotImplementedError

    @abstractmethod
    def delete(self, ids: List[str]) -> int:
        """删除向量

        Args:
            ids: 待删除的 ID 列表

        Returns:
            实际删除数量
        """
        raise NotImplementedError

    def count(self) -> int:
        """获取向量数量

        Returns:
            向量数量
        """
        return len(self._ids)

    def clear(self):
        """清空数据库"""
        self._ids = []
        self._metadata = []

    def get_metadata(self, vector_id: str) -> Optional[Dict]:
        """按 ID 获取元数据

        Args:
            vector_id: 向量 ID

        Returns:
            元数据字典，不存在时返回 None
        """
        if vector_id in self._ids:
            return self._metadata[self._ids.index(vector_id)]
        return None
