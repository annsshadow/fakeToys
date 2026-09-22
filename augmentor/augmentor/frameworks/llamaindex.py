"""LlamaIndex 集成模块

在 LlamaIndex 未安装时退化为同构的字典表示，保证接口可用。
"""

import logging
from typing import List, Dict, Any

logger = logging.getLogger(__name__)


class LlamaIndexIntegration:
    """LlamaIndex 数据集集成"""

    def __init__(self,
                 text_key: str = "instruction",
                 answer_key: str = "output"):
        """初始化集成器

        Args:
            text_key: 正文字段名
            answer_key: 答案字段名
        """
        self.text_key = text_key
        self.answer_key = answer_key

    @staticmethod
    def is_available() -> bool:
        """检查 LlamaIndex 是否可用

        Returns:
            是否已安装 llama-index-core
        """
        try:
            import llama_index.core  # noqa: F401
            return True
        except ImportError:
            return False

    def to_llamaindex_dataset(self, data: List[Dict]) -> List[Any]:
        """转换为 LlamaIndex TextNode 列表

        若 llama-index-core 已安装则返回 TextNode 对象，否则返回等价字典。

        Args:
            data: 数据列表

        Returns:
            TextNode 列表或字典列表
        """
        nodes: List[Dict[str, Any]] = []
        for index, item in enumerate(data):
            nodes.append({
                "id_": f"node-{index:06d}",
                "text": item.get(self.text_key, ""),
                "metadata": {
                    "index": index,
                    "answer": item.get(self.answer_key, ""),
                    "source": item.get("source", ""),
                    "language": item.get("language", "")
                }
            })

        if not self.is_available():
            logger.warning("llama-index-core 未安装，返回字典形式的节点")
            return nodes

        try:
            from llama_index.core.schema import TextNode
            return [
                TextNode(
                    id_=n["id_"],
                    text=n["text"],
                    metadata=n["metadata"]
                )
                for n in nodes
            ]
        except ImportError:
            return nodes

    def from_llamaindex_dataset(self, dataset: List[Any]) -> List[Dict]:
        """从 LlamaIndex TextNode 列表还原

        Args:
            dataset: TextNode 列表或字典列表

        Returns:
            通用格式数据列表
        """
        records = []
        for node in dataset:
            if isinstance(node, dict):
                text = node.get("text", "")
                metadata = node.get("metadata", {}) or {}
            else:
                text = getattr(node, "text", "") or getattr(node, "get_content", lambda: "")()
                metadata = getattr(node, "metadata", {}) or {}

            records.append({
                self.text_key: text,
                self.answer_key: metadata.get("answer", ""),
                "source": metadata.get("source", ""),
                "language": metadata.get("language", "")
            })

        return records

    def to_dicts(self, dataset: List[Any]) -> List[Dict[str, Any]]:
        """将任意形式的 LlamaIndex 数据集转为字典形式

        Args:
            dataset: TextNode 列表或字典列表

        Returns:
            字典列表
        """
        result = []
        for node in dataset:
            if isinstance(node, dict):
                result.append(node)
            else:
                result.append({
                    "id_": getattr(node, "id_", ""),
                    "text": getattr(node, "text", ""),
                    "metadata": getattr(node, "metadata", {}) or {}
                })
        return result
