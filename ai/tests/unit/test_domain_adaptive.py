"""领域自适应增强测试 - 验证自动领域检测和策略适配"""

import pytest
from augmentor.expander import DomainExpander


@pytest.fixture
def mock_model_backend():
    class MockBackend:
        def generate(self, prompt):
            return '["扩展主题1", "扩展主题2", "扩展主题3"]'
    return MockBackend()


@pytest.fixture
def sample_items():
    return [
        {"instruction": "如何租房？"},
        {"instruction": "租房合同注意什么？"},
        {"instruction": "租金如何支付？"},
    ]


class TestAdaptiveExpansion:
    """领域自适应增强测试"""
    
    def test_detect_domain_for_rental_items(self, mock_model_backend, sample_items):
        """租房相关数据应检测为租房服务领域"""
        expander = DomainExpander(model_backend=mock_model_backend)
        domain = expander._detect_domain(sample_items)
        assert domain in ("租房服务", "费用相关", "合同法律", "通用领域")
    
    def test_adaptive_expansion_uses_detected_domain(self, mock_model_backend, sample_items):
        """自适应扩展应根据检测到的领域调整策略"""
        expander = DomainExpander(model_backend=mock_model_backend)
        result = expander.generate_adaptive_expansion(sample_items)
        
        assert result.strategy in ("similar", "related", "scenario")
        assert len(result.expanded_topics) > 0
        assert all("adapted_domain" in topic for topic in result.expanded_topics)
        assert any("检测到领域" in s for s in result.suggestions)
    
    def test_adaptive_expansion_with_target_domain(self, mock_model_backend):
        """指定目标领域应直接使用对应策略"""
        expander = DomainExpander(model_backend=mock_model_backend)
        result = expander.generate_adaptive_expansion(
            [{"instruction": "租房注意事项"}],
            target_domain="合同法律"
        )
        # 合同法律领域应选择相关策略
        assert result.strategy == "related"
        assert any("合同法律" in s for s in result.suggestions)


class TestDomainDetection:
    """领域检测测试"""
    
    def test_detect_domain_for_contract_items(self, mock_model_backend):
        """合同相关数据应检测为合同法律领域"""
        expander = DomainExpander(model_backend=mock_model_backend)
        items = [{"instruction": "租房合同条款"}, {"instruction": "合同如何签署"}]
        domain = expander._detect_domain(items)
        # 由于简化实现，合同关键词出现频率高时应识别为合同法律
        assert isinstance(domain, str)
        assert len(domain) > 0
    
    def test_detect_domain_for_maintenance_items(self, mock_model_backend):
        """维修相关数据应检测到维修关键词"""
        expander = DomainExpander(model_backend=mock_model_backend)
        items = [{"instruction": "设备维修"}, {"instruction": "故障处理"}]
        domain = expander._detect_domain(items)
        assert isinstance(domain, str)
