# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""Ollama 原生异步（AU-P1 扩展）。

OpenAI 已原生异步（httpx.AsyncClient），其余后端此前走线程池回退——等待时仍占
线程。本测试钉住 Ollama 的原生异步路径：真的走了 httpx（而不是悄悄退化成线程
池）、请求体与同步路径同源、缺 httpx 时回退到线程池实现、aclose 关闭客户端。
"""

import asyncio
import sys
import types

import pytest

from augmentor.config import ModelConfig
from augmentor.models import OllamaBackend


class _StatusError(Exception):
    def __init__(self, status: int):
        super().__init__(f"HTTP {status}")
        self.response = types.SimpleNamespace(status_code=status)


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
    """httpx.AsyncClient 替身：记录请求体，返回 Ollama 形状的响应。"""

    instances: list = []

    def __init__(self, timeout=None):
        self.timeout = timeout
        self.posts = []
        self.closed = False
        _FakeAsyncClient.instances.append(self)

    async def post(self, url, json=None, headers=None):
        self.posts.append({"url": url, "json": json, "headers": headers})
        return _FakeResponse({"message": {"content": "hello-from-ollama"}})

    async def aclose(self):
        self.closed = True


def _make_backend():
    return OllamaBackend(
        ModelConfig(type="ollama", base_url="http://localhost:11434", model="llama3"),
    )


def test_ollama_acall_api_goes_through_async_client(monkeypatch):
    import httpx

    _FakeAsyncClient.instances.clear()
    monkeypatch.setattr(httpx, "AsyncClient", _FakeAsyncClient)

    backend = _make_backend()
    out = asyncio.run(backend.agenerate("hi", max_retries=0))

    assert out == "hello-from-ollama"
    assert len(_FakeAsyncClient.instances) == 1
    sent = _FakeAsyncClient.instances[0].posts[0]
    # 请求体与同步路径同源（_build_request）
    assert sent["url"] == backend.api_url
    assert sent["json"]["model"] == "llama3"
    assert sent["json"]["messages"] == [{"role": "user", "content": "hi"}]
    assert sent["json"]["stream"] is False
    assert sent["json"]["options"]["num_predict"] == backend.config.max_output_tokens


def test_ollama_acall_api_falls_back_to_thread_when_httpx_missing(monkeypatch):
    # `sys.modules['httpx'] = None` 会让 `import httpx` 抛 ImportError
    monkeypatch.setitem(sys.modules, "httpx", None)

    backend = _make_backend()
    seen = []

    def fake_call(prompt):
        seen.append(prompt)
        return "sync-path"

    monkeypatch.setattr(backend, "_call_api", fake_call)

    out = asyncio.run(backend.agenerate("hi", max_retries=0))
    assert out == "sync-path"
    assert seen == ["hi"], "缺 httpx 时必须真的退回同步实现，而不是静默返回空"


def test_ollama_aclose_closes_client(monkeypatch):
    import httpx

    _FakeAsyncClient.instances.clear()
    monkeypatch.setattr(httpx, "AsyncClient", _FakeAsyncClient)

    backend = _make_backend()

    async def run():
        await backend.agenerate("x", max_retries=0)
        await backend.aclose()

    asyncio.run(run())
    assert _FakeAsyncClient.instances[0].closed, "aclose 必须关闭异步客户端"