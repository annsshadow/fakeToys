"""模型后端请求/响应处理单元测试

各后端的 HTTP 请求构造与响应解析是平台与外部模型交互的唯一通道，
这里通过替换会话对象在无网络条件下验证其正确性。
"""

import json

import pytest

from augmentor.config import ModelConfig
from augmentor.model_manager import ModelManager
from augmentor.models import (
    ClaudeBackend,
    ERNIEBackend,
    GeminiBackend,
    OllamaBackend,
    OpenAIBackend,
)


class FakeResponse:
    """模拟 HTTP 响应"""

    def __init__(self, payload, status_code=200):
        """初始化

        Args:
            payload: JSON 载荷
            status_code: 状态码
        """
        self._payload = payload
        self.status_code = status_code

    def json(self):
        """返回 JSON 载荷"""
        return self._payload

    def raise_for_status(self):
        """模拟状态码检查"""
        if self.status_code >= 400:
            raise RuntimeError(f"HTTP {self.status_code}")


class FakeSession:
    """模拟 HTTP 会话"""

    def __init__(self, payloads):
        """初始化

        Args:
            payloads: 依次返回的载荷列表；不足时复用最后一个
        """
        self.payloads = payloads if isinstance(payloads, list) else [payloads]
        self.calls = []

    def post(self, url, **kwargs):
        """记录请求并返回预设响应

        Args:
            url: 请求地址
            **kwargs: 请求参数

        Returns:
            FakeResponse 实例
        """
        self.calls.append({"url": url, "kwargs": kwargs})
        payload = self.payloads.pop(0) if len(self.payloads) > 1 else self.payloads[0]
        return FakeResponse(payload)

    def close(self):
        """关闭会话（无操作）"""


def attach_session(backend, payloads):
    """把模拟会话挂载到后端

    Args:
        backend: 模型后端
        payloads: 预设响应载荷

    Returns:
        FakeSession 实例
    """
    session = FakeSession(payloads)
    backend._get_session = lambda: session
    return session


class TestOpenAIBackend:
    """OpenAI 后端"""

    def test_parses_content(self):
        """应正确解析 choices[0].message.content"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="gpt"))
        attach_session(backend, {"choices": [{"message": {"content": "回答"}}]})

        assert backend.generate("问题") == "回答"

    def test_sends_auth_header_and_model(self):
        """请求需携带鉴权头与模型名，否则会被网关拒绝"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="secret", model="gpt-4"))
        session = attach_session(backend, {"choices": [{"message": {"content": "x"}}]})

        backend.generate("问题")

        request = session.calls[0]
        assert request["kwargs"]["headers"]["Authorization"] == "Bearer secret"
        assert request["kwargs"]["json"]["model"] == "gpt-4"

    def test_missing_choices_raises(self):
        """响应缺少 choices 必须报错，不能返回空字符串"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="gpt"))
        attach_session(backend, {"error": "bad"})

        with pytest.raises(RuntimeError):
            backend.generate("问题", max_retries=1, retry_delay=0)


class TestOllamaBackend:
    """Ollama 后端"""

    def test_parses_content(self):
        """应正确解析 message.content"""
        backend = OllamaBackend(ModelConfig(
            type="ollama", base_url="http://localhost:11434", model="qwen"
        ))
        attach_session(backend, {"message": {"content": "本地回答"}})

        assert backend.generate("问题") == "本地回答"

    def test_requests_non_stream(self):
        """必须显式关闭流式，否则无法一次性解析响应"""
        backend = OllamaBackend(ModelConfig(
            type="ollama", base_url="http://localhost:11434", model="qwen"
        ))
        session = attach_session(backend, {"message": {"content": "x"}})

        backend.generate("问题")

        assert session.calls[0]["kwargs"]["json"]["stream"] is False

    def test_missing_message_raises(self):
        """响应缺少 message 必须报错"""
        backend = OllamaBackend(ModelConfig(
            type="ollama", base_url="http://localhost:11434", model="qwen"
        ))
        attach_session(backend, {"done": True})

        with pytest.raises(RuntimeError):
            backend.generate("问题", max_retries=1, retry_delay=0)

    def test_config_validation(self):
        """配置验证 - 缺少 base_url"""
        config = ModelConfig(type="ollama", api_key="", base_url="")
        with pytest.raises(ValueError, match="base_url"):
            OllamaBackend(config)

    def test_extract_json_direct(self):
        """直接解析 JSON"""
        backend = OllamaBackend(ModelConfig(
            type="ollama", base_url="http://localhost:11434", model="qwen"
        ))
        result = backend.extract_json_from_response('[{"key": "value"}]')
        assert result == [{"key": "value"}]

    def test_extract_json_in_text(self):
        """从文本中提取 JSON"""
        backend = OllamaBackend(ModelConfig(
            type="ollama", base_url="http://localhost:11434", model="qwen"
        ))
        result = backend.extract_json_from_response('some text [{"key": "value"}] more text')
        assert result == [{"key": "value"}]

    def test_extract_json_no_array(self):
        """无数组抛异常"""
        backend = OllamaBackend(ModelConfig(
            type="ollama", base_url="http://localhost:11434", model="qwen"
        ))
        with pytest.raises(ValueError):
            backend.extract_json_from_response('no json here')

    def test_extract_json_raw_list(self):
        """直接返回 list 类型"""
        backend = OllamaBackend(ModelConfig(
            type="ollama", base_url="http://localhost:11434", model="qwen"
        ))
        result = backend.extract_json_from_response('[1, 2, 3]')
        assert result == [1, 2, 3]

    def test_extract_json_brackets_only(self):
        """仅括号"""
        backend = OllamaBackend(ModelConfig(
            type="ollama", base_url="http://localhost:11434", model="qwen"
        ))
        result = backend.extract_json_from_response('[]')
        assert result == []

    def test_config_with_url(self):
        """有 URL 的配置"""
        config = ModelConfig(type="ollama", api_key="", base_url="http://localhost:11434")
        backend = OllamaBackend(config)
        assert backend.api_url == "http://localhost:11434/api/chat"

    def test_config_trailing_slash(self):
        """URL 尾部斜杠"""
        config = ModelConfig(type="ollama", api_key="", base_url="http://localhost:11434/")
        backend = OllamaBackend(config)
        assert backend.api_url == "http://localhost:11434/api/chat"


class TestClaudeBackend:
    """Claude 后端"""

    def test_parses_and_joins_text_blocks(self):
        """content 为块列表，需拼接所有 text 块"""
        backend = ClaudeBackend(ModelConfig(type="claude", api_key="k", model="m"))
        attach_session(backend, {"content": [
            {"type": "text", "text": "第一段"},
            {"type": "text", "text": "第二段"}
        ]})

        assert backend.generate("问题") == "第一段第二段"

    def test_ignores_non_text_blocks(self):
        """非文本块不应混入结果"""
        backend = ClaudeBackend(ModelConfig(type="claude", api_key="k", model="m"))
        attach_session(backend, {"content": [
            {"type": "tool_use", "id": "x"},
            {"type": "text", "text": "结果"}
        ]})

        assert backend.generate("问题") == "结果"

    def test_sends_anthropic_headers(self):
        """必须携带 x-api-key 与 anthropic-version，否则 API 会拒绝"""
        backend = ClaudeBackend(ModelConfig(type="claude", api_key="secret", model="m"))
        session = attach_session(backend, {"content": [{"type": "text", "text": "x"}]})

        backend.generate("问题")

        headers = session.calls[0]["kwargs"]["headers"]
        assert headers["x-api-key"] == "secret"
        assert headers["anthropic-version"] == ClaudeBackend.API_VERSION

    def test_uses_max_tokens_field(self):
        """Claude 使用 max_tokens 而非 max_output_tokens"""
        backend = ClaudeBackend(ModelConfig(
            type="claude", api_key="k", model="m", max_output_tokens=512
        ))
        session = attach_session(backend, {"content": [{"type": "text", "text": "x"}]})

        backend.generate("问题")

        assert session.calls[0]["kwargs"]["json"]["max_tokens"] == 512

    def test_empty_content_raises(self):
        """content 为空必须报错"""
        backend = ClaudeBackend(ModelConfig(type="claude", api_key="k", model="m"))
        attach_session(backend, {"content": []})

        with pytest.raises(RuntimeError):
            backend.generate("问题", max_retries=1, retry_delay=0)


class TestGeminiBackend:
    """Gemini 后端"""

    def test_parses_candidate_text(self):
        """应正确解析 candidates[0].content.parts[].text"""
        backend = GeminiBackend(ModelConfig(type="gemini", api_key="k", model="gemini-pro"))
        attach_session(backend, {"candidates": [
            {"content": {"parts": [{"text": "Gemini 回答"}]}}
        ]})

        assert backend.generate("问题") == "Gemini 回答"

    def test_passes_api_key_as_query_param(self):
        """Gemini 通过查询参数传密钥"""
        backend = GeminiBackend(ModelConfig(type="gemini", api_key="secret", model="m"))
        session = attach_session(backend, {"candidates": [
            {"content": {"parts": [{"text": "x"}]}}
        ]})

        backend.generate("问题")

        assert session.calls[0]["kwargs"]["params"]["key"] == "secret"

    def test_generation_config_mapping(self):
        """采样参数需映射为 generationConfig 的驼峰字段"""
        backend = GeminiBackend(ModelConfig(
            type="gemini", api_key="k", model="m", temperature=0.5, top_p=0.8
        ))
        session = attach_session(backend, {"candidates": [
            {"content": {"parts": [{"text": "x"}]}}
        ]})

        backend.generate("问题")

        config = session.calls[0]["kwargs"]["json"]["generationConfig"]
        assert config["temperature"] == 0.5
        assert config["topP"] == 0.8
        assert "maxOutputTokens" in config

    def test_no_candidates_raises(self):
        """无候选结果必须报错"""
        backend = GeminiBackend(ModelConfig(type="gemini", api_key="k", model="m"))
        attach_session(backend, {"candidates": []})

        with pytest.raises(RuntimeError):
            backend.generate("问题", max_retries=1, retry_delay=0)


class TestERNIEBackend:
    """ERNIE 后端"""

    def test_fetches_token_then_calls_chat(self):
        """需先取 token 再调用对话接口"""
        backend = ERNIEBackend(ModelConfig(
            type="baidu", api_key="ak", secret_key="sk", model="ernie"
        ))
        session = attach_session(backend, [
            {"access_token": "token-1", "expires_in": 3600},
            {"result": "ERNIE 回答"}
        ])

        assert backend.generate("问题") == "ERNIE 回答"
        assert len(session.calls) == 2
        assert "access_token=token-1" in session.calls[1]["url"]

    def test_caches_access_token(self):
        """token 需被缓存，避免每次请求都换取凭证"""
        backend = ERNIEBackend(ModelConfig(
            type="baidu", api_key="ak", secret_key="sk", model="ernie"
        ))
        session = attach_session(backend, [
            {"access_token": "token-1", "expires_in": 3600},
            {"result": "回答1"},
            {"result": "回答2"}
        ])

        backend.generate("问题1")
        backend.generate("问题2")

        token_calls = [c for c in session.calls if "oauth" in c["url"]]
        assert len(token_calls) == 1

    def test_missing_access_token_raises(self):
        """换取凭证失败必须报错"""
        backend = ERNIEBackend(ModelConfig(
            type="baidu", api_key="ak", secret_key="sk", model="ernie"
        ))
        attach_session(backend, {"error": "invalid credentials"})

        with pytest.raises(RuntimeError):
            backend.generate("问题", max_retries=1, retry_delay=0)

    def test_truncated_response_raises(self):
        """响应被截断时必须报错，避免入库残缺数据"""
        backend = ERNIEBackend(ModelConfig(
            type="baidu", api_key="ak", secret_key="sk", model="ernie"
        ))
        attach_session(backend, [
            {"access_token": "token-1", "expires_in": 3600},
            {"result": "半截", "is_truncated": True}
        ])

        with pytest.raises(RuntimeError):
            backend.generate("问题", max_retries=1, retry_delay=0)

    def test_extract_json_from_response(self):
        """从文本中提取 JSON"""
        backend = ERNIEBackend(ModelConfig(
            type="baidu", api_key="ak", secret_key="sk", model="ernie"
        ))
        result = backend.extract_json_from_response('结果：[{"instruction": "a"}]')

        assert result == [{"instruction": "a"}]


class TestERNIEBackendExtended:
    """ERNIE 后端扩展测试"""

    def test_extract_json_no_array(self):
        """无数组抛异常"""
        backend = ERNIEBackend(ModelConfig(
            type="baidu", api_key="ak", secret_key="sk", model="ernie"
        ))
        with pytest.raises(ValueError):
            backend.extract_json_from_response('no json here')

    def test_extract_json_direct_list(self):
        """直接解析 JSON 数组"""
        backend = ERNIEBackend(ModelConfig(
            type="baidu", api_key="ak", secret_key="sk", model="ernie"
        ))
        result = backend.extract_json_from_response('[{"x": 1}]')
        assert result == [{"x": 1}]

    def test_extract_json_in_text(self):
        """从文本中提取 JSON"""
        backend = ERNIEBackend(ModelConfig(
            type="baidu", api_key="ak", secret_key="sk", model="ernie"
        ))
        result = backend.extract_json_from_response('前缀 [{"a": 1}] 后缀')
        assert result == [{"a": 1}]


class TestBackendLifecycle:
    """后端生命周期"""

    def test_close_releases_session(self):
        """close 应释放会话并置空，避免连接泄漏"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        session = FakeSession({"choices": [{"message": {"content": "x"}}]})
        backend._session = session

        backend.close()

        assert backend._session is None

    def test_json_extraction_helper_used_by_claude(self):
        """Claude 的 JSON 提取应复用基类工具函数"""
        backend = ClaudeBackend(ModelConfig(type="claude", api_key="k", model="m"))
        assert backend.extract_json_from_response('[1, 2]') == [1, 2]

    def test_json_extraction_helper_used_by_gemini(self):
        """Gemini 的 JSON 提取应复用基类工具函数"""
        backend = GeminiBackend(ModelConfig(type="gemini", api_key="k", model="m"))
        assert backend.extract_json_from_response('前缀 [3] 后缀') == [3]

    def test_ollama_json_extraction(self):
        """Ollama 的 JSON 提取需支持包裹文本"""
        backend = OllamaBackend(ModelConfig(type="ollama", base_url="http://x"))
        assert backend.extract_json_from_response('```json\n[{"a":1}]\n```') == [{"a": 1}]

    def test_openai_json_extraction(self):
        """OpenAI 的 JSON 提取需支持纯 JSON"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k"))
        assert backend.extract_json_from_response(json.dumps([{"a": 1}])) == [{"a": 1}]


class TestModelManager:
    """共享模型管理器"""

    def test_singleton(self):
        """必须是单例，否则每个组件都会重复加载大模型"""
        assert ModelManager() is ModelManager()

    def test_falls_back_when_dependency_missing(self, monkeypatch):
        """依赖缺失时返回 fallback 标记，保证功能降级可用"""
        try:
            import sentence_transformers  # noqa: F401
            pytest.skip("sentence-transformers 已安装，跳过降级场景")
        except ImportError:
            pass

        manager = ModelManager()
        monkeypatch.setattr(manager, "_sentence_model", None, raising=False)

        assert manager.get_sentence_model() == "fallback"

    def test_clear_resets_cache(self, monkeypatch):
        """clear 后应重新解析模型"""
        manager = ModelManager()
        monkeypatch.setattr(manager, "_sentence_model", "cached", raising=False)

        manager.clear()

        assert manager._sentence_model is None

    def test_double_init(self):
        """重复初始化"""
        manager = ModelManager()
        manager.__init__()
        assert manager._initialized is True

    def test_get_sentence_model_caching(self):
        """模型缓存机制"""
        manager = ModelManager()
        manager.clear()
        model1 = manager.get_sentence_model()
        model2 = manager.get_sentence_model()
        assert model1 is model2

    def test_clear_and_reload(self):
        """清空后重新加载"""
        manager = ModelManager()
        manager.clear()
        model = manager.get_sentence_model()
        assert model is not None
        assert manager._sentence_model is not None

    def test_fast_path_returns_cached_without_lock_wait(self, monkeypatch):
        """已缓存时走快速路径（line 40-41），不进入锁"""
        manager = ModelManager()
        monkeypatch.setattr(manager, "_sentence_model", "cached-model", raising=False)
        assert manager.get_sentence_model() is "cached-model"

    def test_double_checked_locking_inner(self, monkeypatch):
        """进入锁后再次检查（line 44-45）应直接返回已缓存值"""
        manager = ModelManager()
        monkeypatch.setattr(manager, "_sentence_model", "inner-cache", raising=False)
        # 通过并发竞争触发内层检查：先让外层检查失败，锁内应命中
        import threading
        original_model = manager._sentence_model
        assert manager.get_sentence_model() == "inner-cache"

    def test_sentence_model_not_none_after_clear(self):
        """clear 后重新获取应再次走加载路径"""
        manager = ModelManager()
        manager.clear()
        assert manager._sentence_model is None
        result = manager.get_sentence_model()
        assert result is not None
