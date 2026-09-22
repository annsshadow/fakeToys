# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""LLM 框架集成单元测试

框架未安装时接口必须退化为等价字典结构，保证平台在离线环境可用。
"""

import pytest

from augmentor.frameworks import LangChainIntegration, LlamaIndexIntegration


class TestLangChainIntegration:
    """LangChain 集成"""

    def test_to_dataset_uses_page_content(self):
        """Document 字段名必须是 page_content / metadata"""
        integration = LangChainIntegration()
        dataset = integration.to_dicts(integration.to_langchain_dataset([
            {"instruction": "问题", "output": "回答"}
        ]))

        assert dataset[0]["page_content"] == "问题"
        assert dataset[0]["metadata"]["answer"] == "回答"

    def test_metadata_carries_index(self):
        """元数据需携带序号，便于回溯原始数据"""
        integration = LangChainIntegration()
        dataset = integration.to_dicts(integration.to_langchain_dataset([
            {"instruction": "a"}, {"instruction": "b"}
        ]))

        assert [d["metadata"]["index"] for d in dataset] == [0, 1]

    def test_round_trip(self):
        """转换后应能还原原始问答"""
        integration = LangChainIntegration()
        original = [{"instruction": "问题", "output": "回答"}]

        restored = integration.from_langchain_dataset(
            integration.to_langchain_dataset(original)
        )

        assert restored[0]["instruction"] == "问题"
        assert restored[0]["output"] == "回答"

    def test_from_dict_form(self):
        """应支持直接从字典形式还原（无 langchain 依赖）"""
        integration = LangChainIntegration()
        restored = integration.from_langchain_dataset([
            {"page_content": "内容", "metadata": {"answer": "答案"}}
        ])

        assert restored[0]["instruction"] == "内容"
        assert restored[0]["output"] == "答案"

    def test_availability_probe_does_not_raise(self):
        """可用性探测必须是纯查询，不应抛异常"""
        assert isinstance(LangChainIntegration.is_available(), bool)

    def test_empty_dataset(self):
        """空数据集返回空列表"""
        integration = LangChainIntegration()
        assert integration.to_langchain_dataset([]) == []
        assert integration.from_langchain_dataset([]) == []


class TestLlamaIndexIntegration:
    """LlamaIndex 集成"""

    def test_to_dataset_uses_text_field(self):
        """TextNode 字段名必须是 text / metadata"""
        integration = LlamaIndexIntegration()
        dataset = integration.to_dicts(integration.to_llamaindex_dataset([
            {"instruction": "问题", "output": "回答"}
        ]))

        assert dataset[0]["text"] == "问题"
        assert dataset[0]["metadata"]["answer"] == "回答"

    def test_node_ids_unique(self):
        """节点 ID 必须唯一"""
        integration = LlamaIndexIntegration()
        dataset = integration.to_dicts(integration.to_llamaindex_dataset([
            {"instruction": "a"}, {"instruction": "b"}
        ]))

        ids = [d["id_"] for d in dataset]
        assert len(ids) == len(set(ids))

    def test_round_trip(self):
        """转换后应能还原原始问答"""
        integration = LlamaIndexIntegration()
        original = [{"instruction": "问题", "output": "回答"}]

        restored = integration.from_llamaindex_dataset(
            integration.to_llamaindex_dataset(original)
        )

        assert restored[0]["instruction"] == "问题"
        assert restored[0]["output"] == "回答"

    def test_from_dict_form(self):
        """应支持直接从字典形式还原"""
        integration = LlamaIndexIntegration()
        restored = integration.from_llamaindex_dataset([
            {"text": "内容", "metadata": {"answer": "答案"}}
        ])

        assert restored[0]["instruction"] == "内容"

    def test_availability_probe_does_not_raise(self):
        """可用性探测必须是纯查询"""
        assert isinstance(LlamaIndexIntegration.is_available(), bool)

    def test_empty_dataset(self):
        """空数据集返回空列表"""
        integration = LlamaIndexIntegration()
        assert integration.to_llamaindex_dataset([]) == []
        assert integration.from_llamaindex_dataset([]) == []


class TestCustomKeys:
    """自定义字段名"""

    @pytest.mark.parametrize("cls", [LangChainIntegration, LlamaIndexIntegration])
    def test_custom_text_key(self, cls):
        """支持自定义正文字段，适配不同数据集结构"""
        integration = cls(text_key="question", answer_key="answer")
        dataset = integration.to_dicts(
            integration.to_langchain_dataset([{"question": "q", "answer": "a"}])
            if cls is LangChainIntegration
            else integration.to_llamaindex_dataset([{"question": "q", "answer": "a"}])
        )

        content_key = "page_content" if cls is LangChainIntegration else "text"
        assert dataset[0][content_key] == "q"


class TestFrameworksExtended:
    """框架扩展测试"""

    def test_langchain_to_dicts_empty(self):
        """LangChain 空数据转换"""
        integration = LangChainIntegration()
        assert integration.to_dicts([]) == []

    def test_langchain_to_dicts_dict_input(self):
        """LangChain 字典输入转换"""
        integration = LangChainIntegration()
        data = [{"page_content": "q", "metadata": {"answer": "a"}}]
        result = integration.to_dicts(data)
        assert result == data

    def test_langchain_from_dict_with_none_metadata(self):
        """LangChain 从字典还原，metadata 为 None"""
        integration = LangChainIntegration()
        data = [{"page_content": "q", "metadata": None}]
        result = integration.from_langchain_dataset(data)
        assert result[0]["instruction"] == "q"

    def test_llamaindex_to_dicts_empty(self):
        """LlamaIndex 空数据转换"""
        integration = LlamaIndexIntegration()
        assert integration.to_dicts([]) == []

    def test_llamaindex_to_dicts_dict_input(self):
        """LlamaIndex 字典输入转换"""
        integration = LlamaIndexIntegration()
        data = [{"text": "q", "metadata": {"answer": "a"}}]
        result = integration.to_dicts(data)
        assert result == data

    def test_llamaindex_from_dict_with_none_metadata(self):
        """LlamaIndex 从字典还原，metadata 为 None"""
        integration = LlamaIndexIntegration()
        data = [{"text": "q", "metadata": None}]
        result = integration.from_llamaindex_dataset(data)
        assert result[0]["instruction"] == "q"

    def test_langchain_metadata_source_language(self):
        """LangChain metadata 包含 source, language"""
        integration = LangChainIntegration()
        data = [{"instruction": "q", "output": "a", "source": "src", "language": "zh"}]
        docs = integration.to_langchain_dataset(data)
        result = integration.from_langchain_dataset(docs)
        assert result[0]["source"] == "src"
        assert result[0]["language"] == "zh"

    def test_from_object_form_page_content(self):
        """非 dict 形式的 Document 对象还原（langchain 已装时）"""
        class FakeDoc:
            def __init__(self, page_content, metadata):
                self.page_content = page_content
                self.metadata = metadata
        integration = LangChainIntegration()
        restored = integration.from_langchain_dataset(
            [FakeDoc("内容", {"answer": "答案", "source": "s", "language": "en"})]
        )
        assert restored[0]["instruction"] == "内容"
        assert restored[0]["output"] == "答案"
        assert restored[0]["source"] == "s"
        assert restored[0]["language"] == "en"

    def test_to_dicts_object_form(self):
        """非 dict 形式对象应转换为字典（覆盖 to_dicts 对象分支）"""
        class FakeDoc:
            page_content = "c"
            metadata = {"answer": "a"}
        integration = LangChainIntegration()
        result = integration.to_dicts([FakeDoc()])
        assert result[0]["page_content"] == "c"
        assert result[0]["metadata"]["answer"] == "a"

    def test_to_dicts_object_metadata_none(self):
        """对象 metadata 为 None 时应转为空 dict"""
        class FakeDoc:
            page_content = "c"
            metadata = None
        integration = LangChainIntegration()
        result = integration.to_dicts([FakeDoc()])
        assert result[0]["metadata"] == {}

    def test_from_object_metadata_missing_attrs(self):
        """对象缺少属性时应安全回退默认值"""
        class BareDoc:
            pass
        integration = LangChainIntegration()
        restored = integration.from_langchain_dataset([BareDoc()])
        assert restored[0]["instruction"] == ""
        assert restored[0]["output"] == ""

    def test_to_langchain_dataset_default_fields(self):
        """缺少 source/language 字段时 metadata 应为空字符串"""
        integration = LangChainIntegration()
        docs = integration.to_langchain_dataset([{"instruction": "q"}])
        assert docs[0]["metadata"]["source"] == ""
        assert docs[0]["metadata"]["language"] == ""

    def test_llamaindex_from_object_form_text_attr(self):
        """非 dict 形式 TextNode 对象还原（覆盖 text 属性分支）"""
        class FakeNode:
            def __init__(self, text, metadata):
                self.text = text
                self.metadata = metadata
        integration = LlamaIndexIntegration()
        restored = integration.from_llamaindex_dataset(
            [FakeNode("内容", {"answer": "答案", "source": "s", "language": "en"})]
        )
        assert restored[0]["instruction"] == "内容"
        assert restored[0]["output"] == "答案"
        assert restored[0]["source"] == "s"

    def test_llamaindex_from_object_form_get_content(self):
        """仅有 get_content 方法（无 text 属性）的对象也应可还原"""
        class OldNode:
            def get_content(self):
                return "旧版内容"
        integration = LlamaIndexIntegration()
        restored = integration.from_llamaindex_dataset([OldNode()])
        assert restored[0]["instruction"] == "旧版内容"

    def test_llamaindex_to_dicts_object_form(self):
        """非 dict 对象转字典（覆盖对象分支）"""
        class FakeNode:
            text = "t"
            metadata = {"answer": "a"}
        integration = LlamaIndexIntegration()
        result = integration.to_dicts([FakeNode()])
        assert result[0]["text"] == "t"

    def test_llamaindex_to_dicts_object_metadata_none(self):
        """对象 metadata 为 None 时应转为空 dict"""
        class FakeNode:
            text = "t"
            metadata = None
        integration = LlamaIndexIntegration()
        result = integration.to_dicts([FakeNode()])
        assert result[0]["metadata"] == {}

    def test_llamaindex_default_fields_empty_strings(self):
        """缺少 source/language 时 metadata 应为空字符串"""
        integration = LlamaIndexIntegration()
        nodes = integration.to_llamaindex_dataset([{"instruction": "q"}])
        assert nodes[0]["metadata"]["source"] == ""
        assert nodes[0]["metadata"]["language"] == ""

    def test_llamaindex_node_id_format(self):
        """节点 ID 应为 node-000000 格式"""
        integration = LlamaIndexIntegration()
        nodes = integration.to_llamaindex_dataset([{"instruction": "q"}])
        assert nodes[0]["id_"] == "node-000000"
