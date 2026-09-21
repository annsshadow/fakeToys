"""RAG 训练数据格式模块

支持 LlamaIndex、LangChain 标准格式以及自定义 JSON 结构，并提供文本分块能力。
"""

import hashlib
import logging
from typing import List, Dict, Any, Optional

logger = logging.getLogger(__name__)

SUPPORTED_FORMATS = ["llamaindex", "langchain", "custom"]


class RAGFormatter:
    """RAG 训练数据格式转换器"""

    def __init__(self,
                 text_key: str = "instruction",
                 answer_key: str = "output",
                 chunk_size: int = 512,
                 chunk_overlap: int = 64):
        """初始化格式转换器

        Args:
            text_key: 正文字段名
            answer_key: 答案字段名
            chunk_size: 分块大小（字符数）
            chunk_overlap: 分块重叠字符数
        """
        if chunk_overlap >= chunk_size:
            raise ValueError("chunk_overlap 必须小于 chunk_size")

        self.text_key = text_key
        self.answer_key = answer_key
        self.chunk_size = chunk_size
        self.chunk_overlap = chunk_overlap

    def chunk_text(self, text: str) -> List[str]:
        """将长文本切分为重叠块

        Args:
            text: 原始文本

        Returns:
            文本块列表
        """
        if not isinstance(text, str) or not text:
            return []

        if len(text) <= self.chunk_size:
            return [text]

        chunks = []
        step = self.chunk_size - self.chunk_overlap
        for start in range(0, len(text), step):
            chunk = text[start:start + self.chunk_size]
            if chunk:
                chunks.append(chunk)
            if start + self.chunk_size >= len(text):
                break

        return chunks

    def _build_metadata(self, item: Dict, index: int, extra: Optional[Dict] = None) -> Dict:
        """构建元数据

        Args:
            item: 原始数据项
            index: 序号
            extra: 附加元数据

        Returns:
            元数据字典
        """
        metadata = {
            "index": index,
            "source": item.get("source", ""),
            "language": item.get("language", "")
        }
        if extra:
            metadata.update(extra)
        return metadata

    def to_llamaindex(self, data: List[Dict]) -> List[Dict]:
        """转换为 LlamaIndex TextNode 格式

        Args:
            data: 数据列表

        Returns:
            LlamaIndex 格式数据列表
        """
        nodes: List[Dict] = []
        node_id = 0

        for index, item in enumerate(data):
            text = item.get(self.text_key, "")
            if not text:
                continue

            chunks = self.chunk_text(text) or [""]
            for chunk_index, chunk in enumerate(chunks):
                digest = hashlib.md5(
                    f"{index}-{chunk_index}-{chunk}".encode()
                ).hexdigest()[:16]

                nodes.append({
                    "id_": f"node-{digest}",
                    "text": chunk,
                    "metadata": self._build_metadata(item, index, {
                        "chunk_index": chunk_index,
                        "chunk_total": len(chunks),
                        "answer": item.get(self.answer_key, "")
                    }),
                    "excluded_embed_metadata_keys": [],
                    "excluded_llm_metadata_keys": [],
                    "relationships": {}
                })
                node_id += 1

        logger.info(f"转换为 LlamaIndex 格式: {len(data)} -> {len(nodes)} 个节点")
        return nodes

    def to_langchain(self, data: List[Dict]) -> List[Dict]:
        """转换为 LangChain Document 格式

        Args:
            data: 数据列表

        Returns:
            LangChain 格式数据列表
        """
        documents: List[Dict] = []

        for index, item in enumerate(data):
            text = item.get(self.text_key, "")
            if not text:
                continue

            chunks = self.chunk_text(text) or [""]
            for chunk_index, chunk in enumerate(chunks):
                documents.append({
                    "page_content": chunk,
                    "metadata": self._build_metadata(item, index, {
                        "chunk_index": chunk_index,
                        "chunk_total": len(chunks),
                        "answer": item.get(self.answer_key, "")
                    })
                })

        logger.info(f"转换为 LangChain 格式: {len(data)} -> {len(documents)} 个文档")
        return documents

    def to_custom(self, data: List[Dict]) -> List[Dict]:
        """转换为自定义 JSON 结构

        Args:
            data: 数据列表

        Returns:
            自定义格式数据列表
        """
        records: List[Dict] = []

        for index, item in enumerate(data):
            text = item.get(self.text_key, "")
            if not text:
                continue

            records.append({
                "id": f"rag-{index:06d}",
                "query": text,
                "answer": item.get(self.answer_key, ""),
                "context": item.get("input", ""),
                "metadata": self._build_metadata(item, index)
            })

        logger.info(f"转换为自定义格式: {len(data)} 条记录")
        return records

    def format(self, data: List[Dict], fmt: str = "llamaindex") -> List[Dict]:
        """按指定格式转换数据

        Args:
            data: 数据列表
            fmt: 目标格式（llamaindex / langchain / custom）

        Returns:
            转换后的数据列表

        Raises:
            ValueError: 不支持的格式
        """
        if fmt not in SUPPORTED_FORMATS:
            raise ValueError(
                f"不支持的 RAG 格式: {fmt}。支持: {SUPPORTED_FORMATS}"
            )

        if fmt == "llamaindex":
            return self.to_llamaindex(data)
        if fmt == "langchain":
            return self.to_langchain(data)
        return self.to_custom(data)

    def from_llamaindex(self, nodes: List[Dict]) -> List[Dict]:
        """从 LlamaIndex 格式还原为通用格式

        Args:
            nodes: LlamaIndex 节点列表

        Returns:
            通用格式数据列表
        """
        records = []
        for node in nodes:
            metadata = node.get("metadata", {}) or {}
            records.append({
                self.text_key: node.get("text", ""),
                self.answer_key: metadata.get("answer", ""),
                "source": metadata.get("source", ""),
                "language": metadata.get("language", "")
            })
        return records

    def from_langchain(self, documents: List[Dict]) -> List[Dict]:
        """从 LangChain 格式还原为通用格式

        Args:
            documents: LangChain 文档列表

        Returns:
            通用格式数据列表
        """
        records = []
        for doc in documents:
            metadata = doc.get("metadata", {}) or {}
            records.append({
                self.text_key: doc.get("page_content", ""),
                self.answer_key: metadata.get("answer", ""),
                "source": metadata.get("source", ""),
                "language": metadata.get("language", "")
            })
        return records
