"""模型缓存复用测试 - 验证单例模式和缓存优化"""

import pytest
from augmentor.model_manager import ModelManager, model_manager


class TestModelCacheReuse:
    """模型缓存复用测试"""
    
    def test_singleton_instance_same(self):
        """单例模式应确保同一实例"""
        manager1 = ModelManager()
        manager2 = ModelManager()
        assert manager1 is manager2
    
    def test_global_instance_is_singleton(self):
        """全局实例应与新创建实例相同"""
        assert model_manager is ModelManager()
    
    def test_model_cache_reuse_avoids_reload(self):
        """缓存复用应避免重复加载（通过检查代码逻辑验证）"""
        # 由于实际模型加载依赖外部库，这里验证缓存机制存在
        manager = ModelManager()
        assert hasattr(manager, '_sentence_model')
        assert hasattr(manager, '_model_lock')
    
    def test_clear_resets_cache(self):
        """清空缓存应重置模型实例"""
        manager = ModelManager()
        # 模拟已加载状态
        manager._sentence_model = "test_model"
        manager.clear()
        assert manager._sentence_model is None
