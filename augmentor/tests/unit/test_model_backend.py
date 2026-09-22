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
