# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""models 工厂与会话池测试

覆盖 create_model_backend 类型映射、不支持类型报错、_get_session
连接池复用、close 重置会话、传输层重试口径唯一（A69/L46）、
HTTPAdapter 不可用时的降级路径（A71/L46）。
"""

import sys
from types import ModuleType

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


class TestTransportRetrySingleSource:
    """传输层不重试——重试口径唯一（A69 / L46）

    业务层 `with_retries` + `classify_error` 是本仓库唯一的重试层。历史上
    `_get_session` 还挂了第二层 urllib3 `Retry(total=3, status_forcelist=[429,...])`，
    但它对**本仓库的 6 个调用点一个都不生效**（全部是 POST，而 urllib3 默认只重
    idempotent 方法），只会让读代码的人误判「最坏请求数」。删除它的理由是契约诚实，
    不是性能（实测差异在噪声内，见 docs/ARCHITECTURE.md §3.20）。

    本组测试钉住三件事：
    1. 传输层重试为 0 —— 若有人重新挂上 Retry，这里必须连同口径文档一起改；
    2. 连接池参数没被顺手删掉 —— 那才是 `_get_session` 存在的意义；
    3. urllib3 的默认幂等方法集不含 POST —— 这条是「删除等价于无操作」的前提，
       若哪天上游改了默认值，本测试会红，提示重新审计而不是静默放大请求数。
    """

    @pytest.mark.parametrize(
        "type_name", ["openai", "ollama", "claude", "gemini", "baidu"]
    )
    @pytest.mark.parametrize("url", ["http://api.example.com/v1", "https://api.example.com/v1"])
    def test_transport_layer_retries_nothing(self, type_name, url):
        """每个后端的 http/https 适配器都必须 0 次传输层重试

        分两个 scheme 是必要的：真实 API 端点全是 https，只钉 http 会留下
        「https 上偷偷挂着重试」的盲区。
        """
        adapter = create_model_backend(_cfg(type_name))._get_session().get_adapter(url)
        assert adapter.max_retries.total == 0

    @pytest.mark.parametrize("url", ["http://api.example.com/v1", "https://api.example.com/v1"])
    def test_status_codes_are_not_adjudicated_by_transport(self, url):
        """状态码是否可重试只能由 `classify_error` 判定，传输层不得持有 status_forcelist"""
        adapter = create_model_backend(_cfg("openai"))._get_session().get_adapter(url)
        assert not getattr(adapter.max_retries, "status_forcelist", None)

    def test_connection_pool_survives_the_removal(self):
        """删 Retry 不能把连接池一起删了——池才是复用会话的收益来源"""
        adapter = (
            create_model_backend(_cfg("openai"))._get_session().get_adapter("https://x/v1")
        )
        assert (adapter._pool_connections, adapter._pool_maxsize) == (10, 20)

    def test_urllib3_does_not_retry_post_by_default(self):
        """上游事实的变化会改变「删掉 Retry」的语义，因此把它钉成断言

        这解释了为什么删除前后逐档请求数一致（恒 503 服务上：
        max_retries=0/1/3/None → 服务端 1/1/3/3 次）。
        """
        from urllib3.util.retry import Retry

        allowed = Retry(3).DEFAULT_ALLOWED_METHODS
        assert "POST" not in allowed
        assert {"GET", "HEAD", "OPTIONS", "PUT", "DELETE", "TRACE"} <= set(allowed)

    def test_no_transport_retry_is_mounted_anywhere_in_the_session(self):
        """逐个已挂载前缀检查，不看默认的两个键会不会漏掉 git+http:// 之类"""
        session = create_model_backend(_cfg("openai"))._get_session()
        prefixes = {"http://", "https://"}
        assert prefixes <= set(session.adapters), "会话必须挂载 http/https 适配器"
        assert all(a.max_retries.total == 0 for a in session.adapters.values())


    def test_degraded_session_names_the_real_cause(self, monkeypatch, caplog):
        """HTTPAdapter 导入失败时，日志必须指出少的是连接池而不是 requests 本体

        A71：原文案「requests 未安装」在这种情况下必然为假——能走到这个
        except 说明 `import requests` 刚刚成功；把排查方向带偏的日志等于没有日志。
        """
        import requests

        # 一个没有 HTTPAdapter 属性的假 requests.adapters ⇒ 精确命中降级分支
        monkeypatch.setitem(sys.modules, "requests.adapters", ModuleType("requests.adapters"))
        with caplog.at_level("WARNING"):
            session = create_model_backend(_cfg("openai"))._get_session()

        assert isinstance(session, requests.Session)
        assert "HTTPAdapter" in caplog.text
        assert "requests 未安装" not in caplog.text
        # 降级路径也不得留下第二层重试：requests 自带适配器同样是 0 次
        assert session.get_adapter("https://api.example.com/v1").max_retries.total == 0

    def test_missing_requests_raises_instead_of_faking_a_degradation(self, monkeypatch):
        """requests 本体缺失必须原样抛 ImportError，而不是走降级分支

        A71 的另一半：原写法在 `except ImportError` 里再 `import requests`，
        等价于「把同一个 ImportError 原样再抛一次，但先打一条假告警」；
        去掉那次冗余 import 后，若还留着裸 `requests.Session()`，症状会换成
        一个与根因无关的 NameError。硬依赖缺失 = 直接崩，才是 fail loud。
        """
        monkeypatch.setitem(sys.modules, "requests", None)
        backend = create_model_backend(_cfg("openai"))
        with pytest.raises(ImportError):
            backend._get_session()
        assert backend._session is None, "失败后不得留下半初始化的会话"


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
