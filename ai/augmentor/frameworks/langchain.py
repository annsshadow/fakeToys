"""LangChain 集成模块

在 LangChain 未安装时退化为同构的字典表示，保证接口可用。
"""

import logging
from typing import List, Dict, Any, Optional

logger = logging.getLogger(__name__)


class LangChainIntegration:
    """LangChain 数据集集成"""

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
        """检查 LangChain 是否可用

        Returns:
            是否已安装 langchain-core
        """
        try:
            import langchain_core  # noqa: F401
            return True
        except ImportError:
            return False

    def to_langchain_dataset(self, data: List[Dict]) -> List[Any]:
        """转换为 LangChain Document 列表

        若 langchain-core 已安装则返回 Document 对象，否则返回等价字典。

        Args:
            data: 数据列表

        Returns:
            Document 列表或字典列表
        """
        documents: List[Dict[str, Any]] = []
        for index, item in enumerate(data):
            documents.append({
                "page_content": item.get(self.text_key, ""),
                "metadata": {
                    "index": index,
                    "answer": item.get(self.answer_key, ""),
                    "source": item.get("source", ""),
                    "language": item.get("language", "")
                }
            })

        if not self.is_available():
            logger.warning("langchain-core 未安装，返回字典形式的文档")
            return documents

        try:
            from langchain_core.documents import Document
            return [
                Document(page_content=d["page_content"], metadata=d["metadata"])
                for d in documents
            ]
        except ImportError:
            return documents

    def from_langchain_dataset(self, dataset: List[Any]) -> List[Dict]:
        """从 LangChain Document 列表还原

        Args:
            dataset: Document 列表或字典列表

        Returns:
            通用格式数据列表
        """
        records = []
        for document in dataset:
            if isinstance(document, dict):
                content = document.get("page_content", "")
                metadata = document.get("metadata", {}) or {}
            else:
                content = getattr(document, "page_content", "")
                metadata = getattr(document, "metadata", {}) or {}

            records.append({
                self.text_key: content,
                self.answer_key: metadata.get("answer", ""),
                "source": metadata.get("source", ""),
                "language": metadata.get("language", "")
            })

        return records

    def to_dicts(self, dataset: List[Any]) -> List[Dict[str, Any]]:
        """将任意形式的 LangChain 数据集转为字典形式

        Args:
            dataset: Document 列表或字典列表

        Returns:
            字典列表
        """
        result = []
        for document in dataset:
            if isinstance(document, dict):
                result.append(document)
            else:
                result.append({
                    "page_content": getattr(document, "page_content", ""),
                    "metadata": getattr(document, "metadata", {}) or {}
                })
        return result
