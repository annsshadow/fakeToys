"""frameworks 集成模块测试（langchain / llamaindex）

覆盖未安装降级、伪框架注入后的 Document/TextNode 路径、
框架子模块缺失时的 ImportError 回退、from_* 还原与 to_dicts 双向。
"""

import sys
import types

import pytest

from augmentor.frameworks.langchain import LangChainIntegration
from augmentor.frameworks.llamaindex import LlamaIndexIntegration

DATA = [
    {"instruction": "如何申请？", "output": "登录官网", "source": "s1", "language": "zh"},
    {"instruction": "多少钱？", "output": "按月计", "source": "s2", "language": "zh"},
]


class _FakeDocument:
    def __init__(self, page_content, metadata=None):
        self.page_content = page_content
        self.metadata = metadata or {}


class _FakeTextNode:
    def __init__(self, text, id_="", metadata=None):
        self.text = text
        self.id_ = id_
        self.metadata = metadata or {}


def _install_fake_langchain(monkeypatch, with_documents=True):
    core = types.ModuleType("langchain_core")
    monkeypatch.setitem(sys.modules, "langchain_core", core)
    if with_documents:
        docs = types.ModuleType("langchain_core.documents")
        docs.Document = _FakeDocument
        core.documents = docs
        monkeypatch.setitem(sys.modules, "langchain_core.documents", docs)


def _install_fake_llamaindex(monkeypatch, with_schema=True):
    root = types.ModuleType("llama_index")
    core = types.ModuleType("llama_index.core")
    root.core = core
    monkeypatch.setitem(sys.modules, "llama_index", root)
    monkeypatch.setitem(sys.modules, "llama_index.core", core)
    if with_schema:
        schema = types.ModuleType("llama_index.core.schema")
        schema.TextNode = _FakeTextNode
        core.schema = schema
        monkeypatch.setitem(sys.modules, "llama_index.core.schema", schema)


class TestLangChainIntegration:
    def test_unavailable_returns_dicts(self):
        """未安装 langchain-core 时返回字典文档（降级路径）"""
        assert not LangChainIntegration.is_available()
        documents = LangChainIntegration().to_langchain_dataset(DATA)
        assert isinstance(documents, list)
        assert documents[0]["page_content"] == "如何申请？"
        assert documents[0]["metadata"]["answer"] == "登录官网"
        assert documents[0]["metadata"]["index"] == 0

    def test_available_returns_document_objects(self, monkeypatch):
        _install_fake_langchain(monkeypatch)
        integration = LangChainIntegration()
        assert integration.is_available() is True
        documents = integration.to_langchain_dataset(DATA)
        assert all(isinstance(d, _FakeDocument) for d in documents)
        assert documents[0].page_content == "如何申请？"

    def test_document_import_failure_falls_back_to_dicts(self, monkeypatch):
        """langchain_core 可导入但缺少 documents 子模块 → 回退字典"""
        _install_fake_langchain(monkeypatch, with_documents=False)
        integration = LangChainIntegration()
        documents = integration.to_langchain_dataset(DATA)
        assert all(isinstance(d, dict) for d in documents)

    def test_from_dataset_dicts(self):
        integration = LangChainIntegration()
        restored = integration.from_langchain_dataset(
            [{"page_content": "p", "metadata": {"answer": "a", "source": "s", "language": "l"}}]
        )
        assert restored[0]["instruction"] == "p"
        assert restored[0]["output"] == "a"

    def test_from_dataset_objects(self, monkeypatch):
        _install_fake_langchain(monkeypatch)
        documents = LangChainIntegration().to_langchain_dataset(DATA)
        restored = LangChainIntegration().from_langchain_dataset(documents)
        assert restored[0]["instruction"] == "如何申请？"
        assert restored[0]["output"] == "登录官网"

    def test_to_dicts_mixed(self, monkeypatch):
        _install_fake_langchain(monkeypatch)
        integration = LangChainIntegration()
        docs = integration.to_langchain_dataset(DATA)
        mixed = docs + [{"page_content": "x", "metadata": {}}]
        result = integration.to_dicts(mixed)
        assert len(result) == 3
        assert all(isinstance(d, dict) for d in result)

    def test_custom_keys(self):
        integration = LangChainIntegration(text_key="q", answer_key="a")
        documents = integration.to_langchain_dataset([{"q": "Q", "a": "A"}])
        assert documents[0]["page_content"] == "Q"
        assert documents[0]["metadata"]["answer"] == "A"


class TestLlamaIndexIntegration:
    def test_unavailable_returns_dicts(self):
        assert not LlamaIndexIntegration.is_available()
        nodes = LlamaIndexIntegration().to_llamaindex_dataset(DATA)
        assert nodes[0]["text"] == "如何申请？"
        assert nodes[0]["id_"] == "node-000000"
        assert nodes[1]["id_"] == "node-000001"

    def test_available_returns_textnodes(self, monkeypatch):
        _install_fake_llamaindex(monkeypatch)
        integration = LlamaIndexIntegration()
        assert integration.is_available() is True
        nodes = integration.to_llamaindex_dataset(DATA)
        assert all(isinstance(n, _FakeTextNode) for n in nodes)
        assert nodes[0].text == "如何申请？"
        assert nodes[0].metadata["index"] == 0

    def test_schema_import_failure_falls_back(self, monkeypatch):
        """llama_index.core 可导入但缺 schema 子模块 → 回退字典"""
        _install_fake_llamaindex(monkeypatch, with_schema=False)
        nodes = LlamaIndexIntegration().to_llamaindex_dataset(DATA)
        assert all(isinstance(n, dict) for n in nodes)

    def test_from_dataset_dicts(self):
        integration = LlamaIndexIntegration()
        restored = integration.from_llamaindex_dataset(
            [{"text": "t", "metadata": {"answer": "a"}}]
        )
        assert restored[0]["instruction"] == "t"
        assert restored[0]["output"] == "a"

    def test_from_dataset_objects_with_get_content(self, monkeypatch):
        """对象缺 text 属性时应回退 get_content() 提取文本"""
        integration = LlamaIndexIntegration()

        class _LegacyNode:
            metadata = {"answer": "ans"}

            def get_content(self):
                return "legacy text"

        restored = integration.from_llamaindex_dataset([_LegacyNode()])
        assert restored[0]["instruction"] == "legacy text"
        assert restored[0]["output"] == "ans"

    def test_to_dicts_mixed(self, monkeypatch):
        _install_fake_llamaindex(monkeypatch)
        integration = LlamaIndexIntegration()
        nodes = integration.to_llamaindex_dataset(DATA)
        mixed = nodes + [{"id_": "n", "text": "t", "metadata": {}}]
        result = integration.to_dicts(mixed)
        assert len(result) == 3
        assert result[2] == {"id_": "n", "text": "t", "metadata": {}}

    def test_roundtrip_dicts(self):
        integration = LlamaIndexIntegration()
        nodes = integration.to_llamaindex_dataset(DATA)
        restored = integration.from_llamaindex_dataset(nodes)
        assert restored[0] == {
            "instruction": "如何申请？",
            "output": "登录官网",
            "source": "s1",
            "language": "zh",
        }
