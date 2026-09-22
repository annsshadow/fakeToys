# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""配置加载测试

原实现含一条虚假断言：

    assert hasattr(config, '_load_cache') or True  # 代码路径存在

`AppConfig` 根本没有 `_load_cache` 属性（grep 全仓无此名），
`or True` 把一个永远为假的判断掩盖成了恒真。该用例已删除；
若将来真的引入配置缓存，应改为断言"同一路径二次加载返回缓存实例"这类可观测行为。
"""

import pytest
from augmentor.config import load_config


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


class TestConfigLoad:
    def test_load_config_from_file(self, temp_config):
        """配置加载应读出 YAML 中的值，而不只是断言返回值非 None"""
        config = load_config(temp_config)

        assert config.augmentation.variants_per_seed == 3

    def test_missing_file_falls_back_to_defaults(self, tmp_path):
        """配置文件不存在时使用 dataclass 默认值，而不是崩溃"""
        config = load_config(str(tmp_path / "not_there.yaml"))

        assert config.augmentation.variants_per_seed > 0
