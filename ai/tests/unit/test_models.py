"""模型后端单元测试

多后端工厂与重试机制是平台可靠性的基础。
"""

import pytest

from augmentor.config import ModelConfig
from augmentor.models import (
    create_model_backend,
    extract_json_array,
    ClaudeBackend,
    GeminiBackend,
    OpenAIBackend,
    OllamaBackend,
    ERNIEBackend,
)
from augmentor.models.base import ModelBackend


class TestFactory:
    """模型工厂"""

    @pytest.mark.parametrize("model_type,expected", [
        ("baidu", ERNIEBackend),
        ("openai", OpenAIBackend),
        ("ollama", OllamaBackend),
        ("claude", ClaudeBackend),
        ("gemini", GeminiBackend),
    ])
    def test_creates_expected_backend(self, model_type, expected):
        """每种类型都应映射到对应后端实现"""
        config = ModelConfig(
            type=model_type,
            api_key="key",
            secret_key="secret",
            base_url="http://localhost:11434",
            model="test-model"
        )
        assert isinstance(create_model_backend(config), expected)

    def test_explicit_type_overrides_config(self):
        """显式传入类型时应覆盖配置中的类型"""
        config = ModelConfig(type="baidu", api_key="k", secret_key="s")
        backend = create_model_backend(config, model_type="claude")
        assert isinstance(backend, ClaudeBackend)

    def test_unknown_type_raises_with_hint(self):
        """未知类型报错需列出支持的类型，便于排查配置错误"""
        with pytest.raises(ValueError) as excinfo:
            create_model_backend(ModelConfig(type="unknown"))
        assert "支持的类型" in str(excinfo.value)


class TestCredentialValidation:
    """凭据校验"""

    def test_claude_requires_api_key(self):
        """缺少密钥必须构造期报错，避免请求期才失败"""
        with pytest.raises(ValueError):
            ClaudeBackend(ModelConfig(type="claude", model="m"))

    def test_gemini_requires_api_key(self):
        """缺少密钥必须构造期报错"""
        with pytest.raises(ValueError):
            GeminiBackend(ModelConfig(type="gemini", model="m"))

    def test_openai_requires_api_key(self):
        """缺少密钥必须构造期报错"""
        with pytest.raises(ValueError):
            OpenAIBackend(ModelConfig(type="openai"))

    def test_ollama_requires_base_url(self):
        """Ollama 必须有 base_url"""
        with pytest.raises(ValueError):
            OllamaBackend(ModelConfig(type="ollama"))

    def test_ernie_requires_both_keys(self):
        """ERNIE 需要 api_key 与 secret_key 两个凭据"""
        with pytest.raises(ValueError):
            ERNIEBackend(ModelConfig(type="baidu", api_key="only-one"))


class TestEndpointConstruction:
    """端点拼装"""

    def test_claude_url(self):
        """Claude 默认端点为 /v1/messages"""
        backend = ClaudeBackend(ModelConfig(type="claude", api_key="k", model="m"))
        assert backend.api_url == "https://api.anthropic.com/v1/messages"

    def test_claude_custom_base_url(self):
        """自定义 base_url 应被正确拼接"""
        backend = ClaudeBackend(ModelConfig(
            type="claude", api_key="k", model="m", base_url="https://proxy.example.com/"
        ))
        assert backend.api_url == "https://proxy.example.com/v1/messages"

    def test_gemini_url_contains_model(self):
        """Gemini 端点需包含模型名"""
        backend = GeminiBackend(ModelConfig(type="gemini", api_key="k", model="gemini-1.5-flash"))
        assert "gemini-1.5-flash" in backend.api_url
        assert backend.api_url.endswith(":generateContent")

    def test_ollama_url(self):
        """Ollama 端点应基于 base_url 拼接 /api/chat"""
        backend = OllamaBackend(ModelConfig(type="ollama", base_url="http://localhost:11434"))
        assert backend.api_url == "http://localhost:11434/api/chat"


class TestJsonExtraction:
    """JSON 提取"""

    def test_parses_pure_json(self):
        """纯 JSON 数组应被直接解析"""
        assert extract_json_array('[{"instruction": "a"}]') == [{"instruction": "a"}]

    def test_extracts_from_wrapped_text(self):
        """模型常在 JSON 外包裹说明文字，需能截取数组部分"""
        response = '好的，结果如下：[{"instruction": "a"}] 希望有帮助'
        assert extract_json_array(response) == [{"instruction": "a"}]

    def test_raises_when_no_array(self):
        """无数组时必须报错，避免上层拿到错误数据"""
        with pytest.raises(ValueError):
            extract_json_array("这里没有任何数组")

    def test_non_list_json_falls_back_to_bracket_search(self):
        """顶层是对象时应继续尝试从文本中截取数组"""
        assert extract_json_array('{"a": 1} [1, 2]') == [1, 2]


class FakeBackend(ModelBackend):
    """用于测试重试逻辑的假后端"""

    def __init__(self, failures: int = 0):
        """初始化

        Args:
            failures: 前 N 次调用失败
        """
        super().__init__(ModelConfig(type="fake"))
        self.failures = failures
        self.calls = 0

    def _call_api(self, prompt: str) -> str:
        """模拟调用"""
        self.calls += 1
        if self.calls <= self.failures:
            raise RuntimeError("模拟失败")
        return f"结果: {prompt}"


class TestRetryMechanism:
    """重试与统计"""

    def test_succeeds_without_retry(self):
        """正常情况一次成功"""
        backend = FakeBackend()
        assert backend.generate("hi") == "结果: hi"
        assert backend.request_count == 1
        assert backend.error_count == 0

    def test_retries_then_succeeds(self):
        """瞬时失败应通过重试恢复，这是多后端稳定性的关键"""
        backend = FakeBackend(failures=2)
        assert backend.generate("hi", max_retries=3, retry_delay=0) == "结果: hi"
        assert backend.error_count == 2

    def test_raises_after_exhausting_retries(self):
        """重试耗尽必须抛出 RuntimeError，不能返回空结果"""
        backend = FakeBackend(failures=10)
        with pytest.raises(RuntimeError):
            backend.generate("hi", max_retries=2, retry_delay=0)

    def test_reset_stats(self):
        """统计重置后计数归零"""
        backend = FakeBackend()
        backend.generate("hi")
        backend.reset_stats()

        assert backend.request_count == 0
        assert backend.error_count == 0


class TestModelBackendExtended:
    """ModelBackend 扩展测试"""

    def test_generate_returns_string(self):
        """generate 应返回字符串"""
        backend = FakeBackend()
        result = backend.generate("test prompt")
        assert isinstance(result, str)

    def test_generate_with_empty_prompt(self):
        """空 prompt 也应正常工作"""
        backend = FakeBackend()
        result = backend.generate("")
        assert isinstance(result, str)

    def test_stats_tracking(self):
        """统计信息应正确追踪"""
        backend = FakeBackend(failures=1)
        backend.generate("test", max_retries=2, retry_delay=0)
        assert backend.request_count == 2
        assert backend.error_count == 1

    def test_extract_json_array_complex(self):
        """复杂文本中的 JSON 提取"""
        text = "这是结果\n```json\n[{\"instruction\": \"q1\"}]\n```\n希望有帮助"
        result = extract_json_array(text)
        assert len(result) == 1

    def test_extract_json_array_multiple_matches(self):
        """多个 JSON 数组应返回第一个"""
        text = "结果: [1, 2, 3] 其他内容"
        result = extract_json_array(text)
        assert result == [1, 2, 3]

    def test_create_backend_with_all_types(self):
        """测试所有后端类型创建"""
        for model_type in ["baidu", "openai", "ollama", "claude", "gemini"]:
            config = ModelConfig(
                type=model_type,
                api_key="test_key",
                secret_key="test_secret",
                base_url="http://localhost:11434",
                model="test_model"
            )
            backend = create_model_backend(config)
            assert backend is not None
