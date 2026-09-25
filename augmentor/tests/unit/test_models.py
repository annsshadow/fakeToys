# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""模型后端单元测试

多后端工厂与重试机制是平台可靠性的基础。
"""

import pytest

from augmentor.config import ModelConfig
from augmentor.exceptions import DataValidationError, ModelGenerateError
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


class TestModelBaseExtended:
    """ModelBackend 基类会话与重试扩展测试"""

    def test_get_session_reuses_instance(self):
        """会话应复用：二次获取返回同一实例"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        session = backend._get_session()
        assert backend._get_session() is session

    def test_get_session_concurrent_single_instance(self):
        """并发获取会话不应创建多个实例（双重检查锁）"""
        import threading
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        results = []
        barrier = threading.Barrier(4)

        def fetch():
            barrier.wait()
            results.append(backend._get_session())

        threads = [threading.Thread(target=fetch) for _ in range(4)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()
        assert len(set(id(s) for s in results)) == 1

    def test_get_session_with_preexisting_value(self):
        """已有会话对象时应直接返回"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        sentinel = object()
        backend._session = sentinel
        assert backend._get_session() is sentinel

    def test_generate_retries_then_succeeds(self):
        """前两次失败后成功，重试机制应返回结果并记录错误"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        calls = {"n": 0}

        def flaky(prompt):
            calls["n"] += 1
            if calls["n"] < 3:
                raise RuntimeError("transient")
            return "ok"

        backend._call_api = flaky
        result = backend.generate("p", max_retries=3, retry_delay=0)
        assert result == "ok"
        assert calls["n"] == 3
        assert backend._error_count == 2

    def test_generate_exhausts_retries_raises(self):
        """重试耗尽后应抛出最后一次异常"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))

        def always_fail(prompt):
            raise RuntimeError("down")

        backend._call_api = always_fail
        with pytest.raises(RuntimeError, match="down"):
            backend.generate("p", max_retries=2, retry_delay=0)
        assert backend._error_count == 2

    def test_counters_track_requests_and_errors(self):
        """成功与失败应分别计入计数器"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        backend._call_api = lambda prompt: "ok"
        backend.generate("ok", max_retries=1, retry_delay=0)
        assert backend._request_count == 1
        assert backend._error_count == 0

    def test_counters_initial_zero(self):
        """新建后端计数器应为 0"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        assert backend._request_count == 0
        assert backend._error_count == 0


class TestGenerateRetryKnobs:
    """generate 的重试次数旋钮"""

    def test_negative_attempts_rejected_at_entry(self):
        """总尝试次数为负没有读法：静默夹成 1 次会让调用方以为重试生效了"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        backend._call_api = lambda prompt: "ok"
        with pytest.raises(DataValidationError, match="max_retries"):
            backend.generate("p", max_retries=-1, retry_delay=0)
        assert backend._request_count == 0

    def test_zero_max_retries_still_calls_once(self):
        """max_retries=0 的既有读法（只调用一次）不能被收紧顺手改掉"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        backend._call_api = lambda prompt: "ok"
        assert backend.generate("p", max_retries=0, retry_delay=0) == "ok"
        assert backend._request_count == 1

    @staticmethod
    def _failing_backend(**kwargs):
        """总是失败的 OpenAI 后端，返回它被调用的次数容器"""
        backend = OpenAIBackend(
            ModelConfig(type="openai", api_key="k", model="m"), **kwargs
        )
        calls = []

        def boom(prompt):
            calls.append(prompt)
            raise RuntimeError("boom")

        backend._call_api = boom
        return backend, calls

    def test_backend_default_attempts_decides_the_call_count(self):
        """后端的默认档位必须真的决定调用次数

        这是 A64 的全部意义：`augmentation.max_retries` 在接线前是死旋钮，
        改它一次调用都不会变。
        """
        backend, calls = self._failing_backend(default_attempts=2, default_retry_delay=0)
        with pytest.raises(ModelGenerateError, match="共尝试 2 次"):
            backend.generate("p")
        assert len(calls) == 2

    def test_explicit_arg_beats_backend_default(self):
        """单次调用的显式参数优先于配置档位，否则「这一次别重试」无从表达"""
        backend, calls = self._failing_backend(default_attempts=5, default_retry_delay=0)
        with pytest.raises(ModelGenerateError):
            backend.generate("p", max_retries=1)
        assert len(calls) == 1

    def test_backend_defaults_reach_the_backoff_call(self, monkeypatch):
        """默认档位要落到 with_retries 的实际参数上（含「总尝试 − 1 = 额外重试」换算）"""
        captured = {}

        def fake_with_retries(func, **kwargs):
            captured.update(kwargs)
            return "ok", None

        monkeypatch.setattr("augmentor.models.base.with_retries", fake_with_retries)
        backend = OpenAIBackend(
            ModelConfig(type="openai", api_key="k", model="m"),
            default_attempts=5,
            default_retry_delay=2.5,
        )
        backend._call_api = lambda prompt: "ok"
        assert backend.generate("p") == "ok"
        assert captured["max_retries"] == 4
        assert captured["base_delay"] == 2.5

    @staticmethod
    def _capture_with_retries(monkeypatch):
        """截住 `generate()` 递给 `with_retries` 的实参"""
        captured = {}

        def fake_with_retries(func, **kwargs):
            captured.update(kwargs)
            return "ok", None

        monkeypatch.setattr("augmentor.models.base.with_retries", fake_with_retries)
        return captured

    def test_backend_forwards_both_ceilings_at_its_defaults(self, monkeypatch):
        """默认档必须把 §3.22 承诺的两个数原样送到 `with_retries`

        本条取代 L48 写的 `test_backend_does_not_shake_the_backoff_yet`。那条钉的是
        「后端**刻意不传** `jitter`」（当时抖动没有配置面，暴露面记在 A75），而 A75 与
        A73 已在 L49 同批接上 ⇒ 那条的前提没了，硬留着就是把「旋钮没接」这件事伪装成
        契约。改写后的主张：两副封顶都由配置决定，且默认档（300 s / 不抖）下每一档
        等待与接参前逐字相同。
        """
        captured = self._capture_with_retries(monkeypatch)
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        backend._call_api = lambda prompt: "ok"
        assert backend.generate("p") == "ok"
        assert captured["max_delay"] == backend._MAX_RETRY_DELAY
        assert captured["max_retry_wait"] == backend._DEFAULT_MAX_RETRY_WAIT
        assert captured["jitter"] == backend._DEFAULT_RETRY_JITTER

    def test_default_ceilings_match_the_documented_promises(self, monkeypatch):
        """后端默认值不许重抄数字：封顶跟着 `retry.MAX_RETRY_AFTER` 走"""
        from augmentor import MAX_RETRY_AFTER

        captured = self._capture_with_retries(monkeypatch)
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        backend._call_api = lambda prompt: "ok"
        backend.generate("p")
        assert (backend._DEFAULT_MAX_RETRY_WAIT, captured["max_retry_wait"]) == (
            MAX_RETRY_AFTER, MAX_RETRY_AFTER)
        assert captured["jitter"] == 0.0

    def test_shrunk_ceiling_and_configured_jitter_reach_the_call(self, monkeypatch):
        """旋钮真接进 `generate()`：45 s / 0.5 两档都要落到 `with_retries` 的实参上

        接线前实测（Temp `l49_probe.py`）：`max_retry_wait` 无处可传，429 + `Retry-After`
        一律睡 300 s；`jitter` 只有 `compute_delay` 的形参、无人调用。
        """
        captured = self._capture_with_retries(monkeypatch)
        backend = OpenAIBackend(
            ModelConfig(type="openai", api_key="k", model="m"),
            default_max_retry_wait=45.0,
            default_retry_jitter=0.5,
        )
        backend._call_api = lambda prompt: "ok"
        backend.generate("p")
        assert (captured["max_retry_wait"], captured["jitter"]) == (45.0, 0.5)

    @pytest.mark.parametrize("kwargs,name", [
        ({"default_max_retry_wait": 301.0}, "max_retry_wait"),
        ({"default_max_retry_wait": float("inf")}, "max_retry_wait"),
        ({"default_max_retry_wait": -1.0}, "max_retry_wait"),
        ({"default_max_retry_wait": True}, "max_retry_wait"),
        ({"default_retry_jitter": 1.5}, "jitter"),
        ({"default_retry_jitter": -0.1}, "jitter"),
        ({"default_retry_jitter": "0.5"}, "jitter"),
    ])
    def test_bad_wait_budget_defaults_rejected_at_construction(self, kwargs, name):
        """坏档位该在建后端时指名，而不是等第一次限流才表现为睡过头 / 抖不停"""
        with pytest.raises(DataValidationError, match=name):
            OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"), **kwargs)


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

    def test_retry_count_zero_without_retry(self):
        """A66：没重试时新出口停在 0（走满 `_request_count` 也不该虚增重试数）"""
        backend = FakeBackend()
        backend.generate("hi")
        assert backend.retry_count == 0
        assert backend.retry_wait_seconds == 0.0

    def test_retry_count_tracks_transient_failures(self):
        """A66：瞬时失败 2 次后成功 ⇒ 实际重试 2 回。`error_count` 只说「失败几次」，
        说不出「其中发生了几回带重发的重试」，这条出口补的正是后者。"""
        backend = FakeBackend(failures=2)
        assert backend.generate("hi", max_retries=3, retry_delay=0) == "结果: hi"
        assert backend.retry_count == 2
        assert backend.error_count == 2  # 既有语义：每次失败照计

    def test_retry_wait_seconds_accumulates_backoff_delays(self):
        """A66：等待时长是这条出口存在的理由——`request_count`/`error_count` 对「总共睡了
        多久」完全隐形。base=0.01、factor=2、无抖动 ⇒ 两回重试等 0.01 + 0.02。"""
        backend = FakeBackend(failures=2)
        backend.generate("hi", max_retries=3, retry_delay=0.01)
        assert backend.retry_count == 2
        assert backend.retry_wait_seconds == pytest.approx(0.03)

    def test_retry_stats_survive_an_exhausted_call(self):
        """A66 的关键判别：走 `on_retry` 而不是 `with_retries` 返回的 `RetryStats`——耗尽
        那条路会 `raise`、拿不到 stats，可等待其实已经付了。失败到弹尽仍须记下已发生的重试。"""
        backend = FakeBackend(failures=10)
        with pytest.raises(ModelGenerateError):
            backend.generate("hi", max_retries=3, retry_delay=0)
        # attempts=3 ⇒ 首次 + 2 回重试，全失败后抛 ModelGenerateError
        assert backend.retry_count == 2
        assert backend.request_count == 3
        assert backend.error_count == 3

    def test_reset_stats_clears_retry_counters(self):
        """A66：新计数纳入既有 `reset_stats` 家族，重置后一起归零"""
        backend = FakeBackend(failures=2)
        backend.generate("hi", max_retries=3, retry_delay=0.01)
        assert backend.retry_count == 2 and backend.retry_wait_seconds > 0
        backend.reset_stats()
        assert backend.retry_count == 0
        assert backend.retry_wait_seconds == 0.0


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

    def test_close_session(self):
        """关闭连接"""
        backend = FakeBackend()
        backend._get_session()
        backend.close()
        assert backend._session is None

    def test_get_session_creates_once(self):
        """会话只创建一次"""
        backend = FakeBackend()
        session1 = backend._get_session()
        session2 = backend._get_session()
        assert session1 is session2

    def test_request_count_thread_safe(self):
        """请求计数线程安全"""
        backend = FakeBackend()
        backend.generate("test1")
        backend.generate("test2")
        assert backend.request_count == 2

    def test_error_count_increments(self):
        """错误计数递增"""
        backend = FakeBackend(failures=1)
        try:
            backend.generate("test", max_retries=1, retry_delay=0)
        except RuntimeError:
            pass
        assert backend.error_count == 1


class TestOpenAIBackend:
    """OpenAI 后端测试"""

    def test_extract_json_from_response_pure_json(self):
        """提取纯 JSON 响应"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        result = backend.extract_json_from_response('[{"instruction": "q"}]')
        assert result == [{"instruction": "q"}]

    def test_extract_json_from_response_wrapped(self):
        """提取包裹在文本中的 JSON"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        result = backend.extract_json_from_response('结果: [{"instruction": "q"}] 完成')
        assert result == [{"instruction": "q"}]

    def test_extract_json_from_response_no_array(self):
        """无数组时抛出异常"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        with pytest.raises(ValueError):
            backend.extract_json_from_response("no array here")

    def test_extract_json_from_response_non_list_object(self):
        """顶层是对象时尝试从文本提取"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        result = backend.extract_json_from_response('{"a": 1} [1, 2]')
        assert result == [1, 2]


class _BareBackend(ModelBackend):
    """只实现抽象方法的最小后端

    ``ModelBackend`` 是 ABC，无法直接实例化；析构测试又必须绕过 ``__init__``
    （否则不会出现「没有 _session」的状态），因此用一个空壳子类承接 ``__new__``。
    """

    def _call_api(self, prompt: str) -> str:  # pragma: no cover - 析构测试不会调用
        return ""


class TestDestructorSafety:
    """析构安全

    __init__ 在赋值 _session 之前就抛异常时（子类校验密钥失败、测试用 __new__
    绕过初始化），对象仍会被 GC 回收并触发 close()/__del__。修复前会再抛
    AttributeError 并污染解释器退出流程（pytest 报 PytestUnraisableExceptionWarning）。
    """

    def test_close_tolerates_missing_session(self):
        """绕过 __init__ 构造的实例调用 close() 不得抛异常"""
        bare = _BareBackend.__new__(_BareBackend)
        assert not hasattr(bare, "_session")
        bare.close()  # 修复前：AttributeError: no attribute '_session'

    def test_del_tolerates_missing_session(self):
        """__del__ 必须整体兜底"""
        bare = _BareBackend.__new__(_BareBackend)
        bare.__del__()  # 修复前：AttributeError 冒泡

    def test_close_tolerates_broken_session(self):
        """会话对象自身 close() 抛错时不得影响回收"""

        class BrokenSession:
            def close(self):
                raise RuntimeError("连接已断开")

        bare = _BareBackend.__new__(_BareBackend)
        bare._session = BrokenSession()
        bare.close()
        assert bare._session is None
