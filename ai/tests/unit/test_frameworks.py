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
