# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""models 工厂与会话池测试

覆盖 create_model_backend 类型映射、不支持类型报错、_get_session
连接池复用、close 重置会话。
"""

import pytest

from augmentor.config import ModelConfig
from augmentor.models.factory import create_model_backend
from augmentor.models import (
    ERNIEBackend,
    OpenAIBackend,
    OllamaBackend,
    ClaudeBackend,
    GeminiBackend,
)


def _cfg(model_type, api_key="k"):
    return ModelConfig(
        type=model_type,
        api_key=api_key,
        secret_key="s",
        base_url="http://localhost:11434",
        model="m",
    )


class TestCreateModelBackend:
    @pytest.mark.parametrize(
        "type_name,cls",
        [
            ("baidu", ERNIEBackend),
            ("openai", OpenAIBackend),
            ("ollama", OllamaBackend),
            ("claude", ClaudeBackend),
            ("gemini", GeminiBackend),
        ],
    )
    def test_maps_type_to_backend(self, type_name, cls):
        backend = create_model_backend(_cfg(type_name))
        assert isinstance(backend, cls)

    def test_explicit_type_override(self):
        backend = create_model_backend(_cfg("baidu"), model_type="openai")
        assert isinstance(backend, OpenAIBackend)

    def test_unsupported_type_raises(self):
        with pytest.raises(ValueError, match="不支持的模型类型"):
            create_model_backend(_cfg("nope"))

    def test_default_from_config_type(self):
        backend = create_model_backend(_cfg("gemini"))
        assert isinstance(backend, GeminiBackend)


class TestRetryDefaultsPlumbing:
    """配置文件的重试旋钮经工厂到后端的透传

    `augmentation.max_retries` / `retry_delay` 在 3.x 时代是**死旋钮**：字段声明了、
    进了默认配置字典，但没有任何消费者，用户改它们行为一个字都不变。这里钉住
    「工厂 → 5 个后端 → 后端默认档位」这条链路，防止它再断一次。
    """

    @pytest.mark.parametrize(
        "type_name", ["baidu", "openai", "ollama", "claude", "gemini"]
    )
    def test_factory_forwards_retry_defaults_to_every_backend(self, type_name):
        backend = create_model_backend(
            _cfg(type_name), default_attempts=5, default_retry_delay=2.5
        )
        assert backend._default_attempts == 5
        assert backend._default_retry_delay == 2.5

    def test_absent_defaults_fall_back_to_documented_values(self):
        backend = create_model_backend(_cfg("openai"))
        assert backend._default_attempts == 3
        assert backend._default_retry_delay == 1.0

    def test_zero_is_not_read_as_absent(self):
        """0 是「只调用一次、失败立刻重试不等」，`or` 回落会把它悄悄换成 3"""
        backend = create_model_backend(
            _cfg("openai"), default_attempts=0, default_retry_delay=0.0
        )
        assert backend._default_attempts == 0
        assert backend._default_retry_delay == 0.0

    def test_out_of_range_defaults_rejected_by_factory(self):
        with pytest.raises(ValueError, match="default_attempts"):
            create_model_backend(_cfg("openai"), default_attempts=-1)
        with pytest.raises(ValueError, match="default_retry_delay"):
            create_model_backend(_cfg("openai"), default_retry_delay=-1.0)


class TestSessionPool:
    def test_get_session_returns_pool_session(self):
        backend = create_model_backend(_cfg("openai"))
        s1 = backend._get_session()
        assert s1 is not None
        # 二次调用复用同一会话
        assert backend._get_session() is s1

    def test_close_resets_session(self):
        backend = create_model_backend(_cfg("openai"))
        backend._get_session()
        backend.close()
        assert backend._session is None
        # close 幂等
        backend.close()
        assert backend._session is None

    def test_close_without_session_noop(self):
        backend = create_model_backend(_cfg("openai"))
        backend.close()  # 从未创建会话
        assert backend._session is None


class TestCounters:
    def test_request_and_error_counters(self):
        backend = create_model_backend(_cfg("openai"))
        assert backend.request_count == 0
        assert backend.error_count == 0


class TestPackageExports:
    def test_backends_exported(self):
        import augmentor

        assert augmentor.ModelBackend is not None
        from augmentor.models import ModelBackend

        assert callable(augmentor.create_model_backend)
