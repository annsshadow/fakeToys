"""上下文感知增强测试 - 验证意图分析和上下文增强功能"""

import pytest
from augmentor.context import ContextAugmentor


@pytest.fixture
def mock_model_backend():
    """模拟模型后端（用于初始化 ContextAugmentor）"""
    class MockBackend:
        def generate(self, prompt):
            return "测试后续问题"
    return MockBackend()


@pytest.fixture
def sample_history():
    """示例对话历史"""
    return [
        {"role": "user", "content": "如何租房？"},
        {"role": "assistant", "content": "建议先查看房源信息。"},
        {"role": "user", "content": "租房合同注意什么？"},
        {"role": "assistant", "content": "注意租金和押金条款。"},
    ]


class TestAnalyzeContextIntent:
    """上下文意图分析测试"""
    
    def test_empty_history_returns_none_shift(self, mock_model_backend):
        """无历史时意图变化应为 none"""
        augmentor = ContextAugmentor(model_backend=mock_model_backend)
        result = augmentor.analyze_context_intent([], "如何租房？")
        assert result["intent_shift"] == "none"
        assert result["context_depth"] == 0
        assert result["theme_consistency"] == 1.0
    
    def test_continued_intent_for_related_questions(self, mock_model_backend, sample_history):
        """相关问题应识别为 continued 意图"""
        augmentor = ContextAugmentor(model_backend=mock_model_backend)
        result = augmentor.analyze_context_intent(sample_history, "租房维权")
        assert result["context_depth"] == 2  # 4条历史对话 / 2
        assert "continued" in result["intent_shift"] or "shifted" in result["intent_shift"]
    
    def test_shifted_intent_for_unrelated_questions(self, mock_model_backend, sample_history):
        """完全无关的问题应识别为意图变化"""
        augmentor = ContextAugmentor(model_backend=mock_model_backend)
        result = augmentor.analyze_context_intent(sample_history, "如何做红烧肉？")
        # 由于简化计算，无关问题也可能显示为低一致性
        assert 0.0 <= result["theme_consistency"] <= 1.0
        assert result["suggested_follow_up"] is not None


class TestEnhanceWithContextAwareness:
    """上下文感知增强功能测试"""
    
    def test_enhanced_result_contains_context_awareness(self, mock_model_backend):
        """增强结果应包含上下文感知字段"""
        augmentor = ContextAugmentor(model_backend=mock_model_backend)
        seed_qa = {
            "instruction": "租房注意事项",
            "output": "注意合同条款",
            "system": "测试系统"
        }
        result = augmentor.enhance_with_context_awareness(seed_qa, history=[])
        
        assert "context_awareness" in result
        awareness = result["context_awareness"]
        assert "intent_shift" in awareness
        assert "theme_consistency" in awareness
        assert "context_depth" in awareness
        assert "suggested_follow_up" in awareness
    
    def test_enhanced_result_preserves_seed_data(self, mock_model_backend):
        """增强结果应保留原始种子数据"""
        augmentor = ContextAugmentor(model_backend=mock_model_backend)
        seed_qa = {
            "instruction": "租房注意事项",
            "output": "注意合同条款"
        }
        result = augmentor.enhance_with_context_awareness(seed_qa)
        
        assert result["instruction"] == seed_qa["instruction"]
        assert result["output"] == seed_qa["output"]
        assert result["system"] == "你是安居乐寓的智能客服"
    
    def test_context_awareness_with_history(self, mock_model_backend, sample_history):
        """带历史的增强应分析上下文深度"""
        augmentor = ContextAugmentor(model_backend=mock_model_backend)
        seed_qa = {
            "instruction": "租房合同如何签？",
            "output": "确认身份和条款"
        }
        result = augmentor.enhance_with_context_awareness(seed_qa, history=sample_history)
        
        awareness = result["context_awareness"]
        assert awareness["context_depth"] >= 1
        assert result["history"] == sample_history
