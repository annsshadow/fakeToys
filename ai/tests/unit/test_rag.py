"""RAG 训练数据格式单元测试

格式字段名必须与 LlamaIndex / LangChain 约定一致，否则下游无法加载。
"""

import pytest

from augmentor.rag import RAGFormatter, SUPPORTED_FORMATS


class TestChunking:
    """文本分块"""

    def test_short_text_single_chunk(self):
        """短文本不切分"""
        formatter = RAGFormatter(chunk_size=100, chunk_overlap=10)
        assert formatter.chunk_text("短文本") == ["短文本"]

    def test_long_text_multiple_chunks(self):
        """超长文本需被切分为多块"""
        formatter = RAGFormatter(chunk_size=10, chunk_overlap=2)
        chunks = formatter.chunk_text("0123456789" * 3)

        assert len(chunks) > 1
        assert all(len(c) <= 10 for c in chunks)

    def test_chunks_cover_full_text(self):
        """分块需完整覆盖原文，避免内容丢失"""
        formatter = RAGFormatter(chunk_size=10, chunk_overlap=3)
        text = "0123456789abcdefghij"
        chunks = formatter.chunk_text(text)

        assert chunks[0] == text[:10]
        assert text.endswith(chunks[-1])

    def test_overlap_must_be_smaller_than_size(self):
        """重叠必须小于块大小，否则会死循环"""
        with pytest.raises(ValueError):
            RAGFormatter(chunk_size=10, chunk_overlap=10)

    def test_empty_text(self):
        """空文本返回空列表"""
        assert RAGFormatter().chunk_text("") == []


class TestLlamaIndex:
    """LlamaIndex 格式"""

    def test_node_fields(self):
        """TextNode 需包含 id_ / text / metadata"""
        formatter = RAGFormatter()
        nodes = formatter.to_llamaindex([
            {"instruction": "问题", "output": "回答"}
        ])

        assert set(nodes[0].keys()) >= {"id_", "text", "metadata"}
        assert nodes[0]["text"] == "问题"
        assert nodes[0]["metadata"]["answer"] == "回答"

    def test_ids_are_unique(self):
        """节点 ID 必须唯一，否则索引会互相覆盖"""
        formatter = RAGFormatter()
        nodes = formatter.to_llamaindex([
            {"instruction": "问题A", "output": "回答A"},
            {"instruction": "问题B", "output": "回答B"},
        ])

        ids = [n["id_"] for n in nodes]
        assert len(ids) == len(set(ids))

    def test_long_text_produces_multiple_nodes(self):
        """长文本应产出多个节点，并记录分块序号"""
        formatter = RAGFormatter(chunk_size=5, chunk_overlap=1)
        nodes = formatter.to_llamaindex([{"instruction": "0123456789abcd", "output": "a"}])

        assert len(nodes) > 1
        assert nodes[0]["metadata"]["chunk_total"] == len(nodes)
        assert nodes[0]["metadata"]["chunk_index"] == 0

    def test_skips_empty_text(self):
        """空文本不应生成节点，避免污染索引"""
        nodes = RAGFormatter().to_llamaindex([{"instruction": "", "output": "x"}])
        assert nodes == []


class TestLangChain:
    """LangChain 格式"""

    def test_document_fields(self):
        """Document 需使用 page_content / metadata 字段名"""
        formatter = RAGFormatter()
        documents = formatter.to_langchain([{"instruction": "问题", "output": "回答"}])

        assert set(documents[0].keys()) == {"page_content", "metadata"}
        assert documents[0]["page_content"] == "问题"

    def test_empty_text_skipped(self):
        """空文本不生成文档"""
        assert RAGFormatter().to_langchain([{"instruction": ""}]) == []


class TestCustomFormat:
    """自定义格式"""

    def test_fields(self):
        """自定义结构需包含 id / query / answer / context"""
        records = RAGFormatter().to_custom([
            {"instruction": "问题", "output": "回答", "input": "背景"}
        ])

        assert set(records[0].keys()) == {"id", "query", "answer", "context", "metadata"}
        assert records[0]["context"] == "背景"

    def test_id_is_zero_padded(self):
        """ID 需固定宽度，便于按字典序排序"""
        records = RAGFormatter().to_custom([{"instruction": "a"}])
        assert records[0]["id"] == "rag-000000"


class TestFormatDispatch:
    """格式分发"""

    @pytest.mark.parametrize("fmt", SUPPORTED_FORMATS)
    def test_supported_formats_dispatch(self, fmt):
        """三种格式都应可分发且返回非空结果"""
        records = RAGFormatter().format([{"instruction": "问题", "output": "回答"}], fmt)
        assert len(records) == 1

    def test_unknown_format_raises(self):
        """未知格式必须报错并列出支持项"""
        with pytest.raises(ValueError) as excinfo:
            RAGFormatter().format([], "unknown")
        assert "不支持的 RAG 格式" in str(excinfo.value)


class TestRoundTrip:
    """双向转换"""

    def test_llamaindex_round_trip(self):
        """LlamaIndex 转换后应能还原出原始问答"""
        formatter = RAGFormatter()
        original = [{"instruction": "问题", "output": "回答"}]

        restored = formatter.from_llamaindex(formatter.to_llamaindex(original))

        assert restored[0]["instruction"] == "问题"
        assert restored[0]["output"] == "回答"

    def test_langchain_round_trip(self):
        """LangChain 转换后应能还原出原始问答"""
        formatter = RAGFormatter()
        original = [{"instruction": "问题", "output": "回答"}]

        restored = formatter.from_langchain(formatter.to_langchain(original))

        assert restored[0]["instruction"] == "问题"
        assert restored[0]["output"] == "回答"


class TestRAGExtended:
    """RAGFormatter 扩展测试"""

    def test_chunk_text_empty(self):
        """空文本分块"""
        formatter = RAGFormatter()
        assert formatter.chunk_text("") == []

    def test_chunk_text_single_char(self):
        """单字符文本分块"""
        formatter = RAGFormatter(chunk_size=5, chunk_overlap=1)
        assert formatter.chunk_text("a") == ["a"]

    def test_format_empty_dataset(self):
        """空数据集格式化"""
        formatter = RAGFormatter()
        records = formatter.format([], "llamaindex")
        assert records == []

    def test_to_llamaindex_empty(self):
        """空数据转 LlamaIndex"""
        formatter = RAGFormatter()
        records = formatter.to_llamaindex([])
        assert records == []

    def test_to_langchain_empty(self):
        """空数据转 LangChain"""
        formatter = RAGFormatter()
        records = formatter.to_langchain([])
        assert records == []

    def test_to_custom_empty(self):
        """空数据转自定义格式"""
        formatter = RAGFormatter()
        records = formatter.to_custom([])
        assert records == []

    def test_from_llamaindex_empty(self):
        """空 LlamaIndex 数据转回"""
        formatter = RAGFormatter()
        records = formatter.from_llamaindex([])
        assert records == []

    def test_from_langchain_empty(self):
        """空 LangChain 数据转回"""
        formatter = RAGFormatter()
        records = formatter.from_langchain([])
        assert records == []

    def test_supported_formats_list(self):
        """支持的格式列表"""
        assert "llamaindex" in SUPPORTED_FORMATS
        assert "langchain" in SUPPORTED_FORMATS
        assert "custom" in SUPPORTED_FORMATS
