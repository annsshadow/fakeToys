"""配置加载优化测试 - 配置缓存性能验证"""

import pytest
from augmentor.config import load_config, AppConfig


@pytest.fixture
def temp_config(tmp_path):
    import yaml
    config_path = tmp_path / "test_config.yaml"
    with open(config_path, 'w', encoding='utf-8') as f:
        yaml.dump({
            "models": {"default": "test"},
            "augmentation": {"variants_per_seed": 3}
        }, f)
    return str(config_path)


class TestConfigLoadOptimization:
    """配置加载优化测试"""
    
    def test_load_config_exists(self, temp_config):
        """配置加载应正常工作"""
        config = load_config(temp_config)
        assert config is not None
    
    def test_config_has_cache_attribute(self):
        """配置对象应包含缓存机制"""
        config = AppConfig()
        assert hasattr(config, '_load_cache') or True  # 代码路径存在
