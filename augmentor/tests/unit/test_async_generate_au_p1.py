# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""AU-P1：异步生成链路（async_with_retries / agenerate / OpenAI 原生异步）。

# 为什么这些用例必须有

异步版的价值主张只有一条：**退避等待期间不占线程**。这条主张无法用"跑完没崩"
证明，只能靠三条判据：① 重试策略与同步版逐位一致（含参数校验发生在发请求之前）；
② 缓存与统计口径与同步版一致（否则"同步慢还是异步慢"无法对比）；
③ 原生异步路径真的走了 httpx，而不是悄悄退化成线程池。
"""

import asyncio
import sys
import types

import pytest

from augmentor.config import ModelConfig
from augmentor.exceptions import DataValidationError, ModelGenerateError
from augmentor.models import OpenAIBackend
from augmentor.models.base import ModelBackend
from augmentor.retry import async_with_retries


class _StatusError(Exception):
    """带 response.status_code 的异常 —— classify_error 靠它判「不可重试」。"""

    def __init__(self, status: int):
        super().__init__(f"HTTP {status}")
        self.response = types.SimpleNamespace(status_code=status)


class _DummyBackend(ModelBackend):
    """最小可用的后端替身：`script` 决定每次调用的结果。"""

    def __init__(self, **kwargs):
        super().__init__(ModelConfig(type="openai", api_key="k", model="m"), **kwargs)
        self.calls = 0
        self.script: list = []

    def _call_api(self, prompt: str) -> str:
        self.calls += 1
        action = self.script.pop(0) if self.script else "ok"
        if isinstance(action, BaseException):
            raise action
        return action


class _FakeResponse:
    def __init__(self, payload, status_code=200):
        self._payload = payload
        self.status_code = status_code

    def json(self):
        return self._payload

    def raise_for_status(self):
        if self.status_code >= 400:
            raise _StatusError(self.status_code)


class _FakeAsyncClient:
    """httpx.AsyncClient 的替身：记录请求体，不联网。"""

    instances: list = []

    def __init__(self, timeout=None):
        self.timeout = timeout
        self.posts = []
        self.closed = False
        _FakeAsyncClient.instances.append(self)

    async def post(self, url, json=None, headers=None):
        self.posts.append({"url": url, "json": json, "headers": headers})
        return _FakeResponse({"choices": [{"message": {"content": "hello"}}]})

    async def aclose(self):
        self.closed = True


# ── async_with_retries ────────────────────────────────────────────────────────

def test_async_retry_succeeds_on_first_try():
    async def run():
        calls = []

        async def fn():
            calls.append(1)
            return "ok"

        return await async_with_retries(fn, max_retries=2, base_delay=0.0), calls

    (result, stats), calls = asyncio.run(run())
    assert result == "ok"
    assert stats.attempts == 1
    assert stats.succeeded is True
    assert calls == [1]


def test_async_retry_retries_then_succeeds_and_records_delay():
    slept = []

    async def fake_sleep(delay):
        slept.append(delay)

    async def run():
        attempts = []

        async def fn():
            attempts.append(1)
            if len(attempts) < 3:
                raise RuntimeError("boom")
            return "done"

        return await async_with_retries(
            fn, max_retries=3, base_delay=0.5, factor=2.0,
            sleeper=fake_sleep, classify=lambda exc: (True, None),
        )

    result, stats = asyncio.run(run())
    assert result == "done"
    assert stats.attempts == 3
    # 退避由 compute_delay 算：0.5, 1.0（与同步版同一函数）
    assert slept == [0.5, 1.0]
    assert stats.delays == [0.5, 1.0]


def test_async_retry_stops_immediately_when_classify_says_no():
    slept = []

    async def fake_sleep(delay):
        slept.append(delay)

    async def run():
        async def fn():
            raise _StatusError(401)

        await async_with_retries(
            fn, max_retries=5, base_delay=0.1, sleeper=fake_sleep,
            classify=lambda exc: (False, None),
        )

    with pytest.raises(_StatusError):
        asyncio.run(run())
    # 不可重试 ⇒ 一次都不等（重试只会浪费配额）
    assert slept == []


def test_async_retry_validates_before_any_call():
    called = []

    async def run():
        async def fn():
            called.append(1)
            return "never"

        await async_with_retries(fn, max_retries=-1)

    with pytest.raises(DataValidationError):
        asyncio.run(run())
    assert called == [], "坏参数必须在发出任何调用之前判掉"


def test_async_retry_raises_last_error_when_exhausted():
    async def fake_sleep(delay):
        return None

    async def run():
        async def fn():
            raise RuntimeError("always")

        await async_with_retries(fn, max_retries=2, base_delay=0.0, sleeper=fake_sleep)

    with pytest.raises(RuntimeError, match="always"):
        asyncio.run(run())


# ── BaseModel.agenerate ──────────────────────────────────────────────────────

def test_agenerate_uses_cache_and_skips_api():
    backend = _DummyBackend()

    first = asyncio.run(backend.agenerate("p", max_retries=0))
    second = asyncio.run(backend.agenerate("p", max_retries=0))

    assert first == second == "ok"
    assert backend.calls == 1, "第二次必须命中内存缓存，不能再打一次 API"
    assert backend.request_count == 2, "缓存命中仍计一次 request（与同步版同口径）"


def test_agenerate_counts_retries_and_errors_like_sync():
    backend = _DummyBackend()
    backend.script = [RuntimeError("transient"), "finally"]

    result = asyncio.run(backend.agenerate("p2", max_retries=2, retry_delay=0.0))

    assert result == "finally"
    assert backend.request_count == 2
    assert backend.error_count == 1
    assert backend.retry_count == 1
    assert backend.retry_wait_seconds == pytest.approx(0.0)


def test_agenerate_wraps_exhausted_error():
    backend = _DummyBackend()
    backend.script = [RuntimeError("a"), RuntimeError("b")]

    with pytest.raises(ModelGenerateError, match="共尝试 2 次"):
        asyncio.run(backend.agenerate("p3", max_retries=2, retry_delay=0.0))


# ── OpenAI 原生异步 ──────────────────────────────────────────────────────────

def test_openai_acall_api_goes_through_async_client(monkeypatch):
    import httpx

    _FakeAsyncClient.instances.clear()
    monkeypatch.setattr(httpx, "AsyncClient", _FakeAsyncClient)

    backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="gpt-4o"))
    out = asyncio.run(backend.agenerate("hi", max_retries=0))

    assert out == "hello"
    assert len(_FakeAsyncClient.instances) == 1
    sent = _FakeAsyncClient.instances[0].posts[0]
    assert sent["url"] == OpenAIBackend.API_URL
    # 请求体与同步路径同源（_build_request），这是「切异步不改模型行为」的判据
    assert sent["json"]["model"] == "gpt-4o"
    assert sent["json"]["messages"] == [{"role": "user", "content": "hi"}]
    assert sent["headers"]["Authorization"] == "Bearer k"


def test_openai_acall_api_falls_back_to_thread_when_httpx_missing(monkeypatch):
    # `sys.modules['httpx'] = None` 会让 `import httpx` 抛 ImportError
    monkeypatch.setitem(sys.modules, "httpx", None)

    backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="gpt"))
    seen = []

    def fake_call(prompt):
        seen.append(prompt)
        return "sync-path"

    monkeypatch.setattr(backend, "_call_api", fake_call)

    out = asyncio.run(backend.agenerate("hi", max_retries=0))
    assert out == "sync-path"
    assert seen == ["hi"], "缺 httpx 时必须真的退回同步实现，而不是静默返回空"


def test_openai_aclose_closes_client(monkeypatch):
    import httpx

    _FakeAsyncClient.instances.clear()
    monkeypatch.setattr(httpx, "AsyncClient", _FakeAsyncClient)

    backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="gpt"))

    async def run():
        await backend.agenerate("x", max_retries=0)
        await backend.aclose()

    asyncio.run(run())
    assert _FakeAsyncClient.instances[0].closed is True
