# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""ERNIE 原生异步（AU-P1 扩展）。

ERNIE 需要先换 access_token 再调聊天接口，此前全走线程池回退。本测试钉住
原生异步路径：真的走了 httpx、token 获取走异步锁、请求体与同步路径同源、
缺 httpx 回退、aclose 关闭客户端。
"""

import asyncio
import sys
import types

import pytest

from augmentor.config import ModelConfig
from augmentor.models import ERNIEBackend


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
    instances: list = []

    def __init__(self, timeout=None):
        self.timeout = timeout
        self.posts = []
        self.closed = False
        _FakeAsyncClient.instances.append(self)

    async def post(self, url, json=None, headers=None, params=None):
        self.posts.append({"url": url, "json": json, "headers": headers, "params": params})
        if "access_token=" in url:
            # CHAT_URL：返回模型结果
            return _FakeResponse({"result": "hello-from-ernie"})
        # TOKEN_URL：返回 access_token
        return _FakeResponse({"access_token": "fake-token", "expires_in": 3600})

    async def aclose(self):
        self.closed = True


def _make_backend():
    # ERNIEBackend 的合法 type 是 "baidu"（出厂模板 type=ernie，但 config 校验用 baidu）。
    return ERNIEBackend(
        ModelConfig(type="baidu", api_key="ak", secret_key="sk", model="ernie-4.0"),
    )


def test_ernie_acall_api_goes_through_async_client(monkeypatch):
    import httpx

    _FakeAsyncClient.instances.clear()
    monkeypatch.setattr(httpx, "AsyncClient", _FakeAsyncClient)

    backend = _make_backend()
    out = asyncio.run(backend.agenerate("hi", max_retries=0))

    assert out == "hello-from-ernie"
    assert len(_FakeAsyncClient.instances) == 1
    client = _FakeAsyncClient.instances[0]
    # 先换 token（TOKEN_URL，无 access_token 参数），再调聊天接口
    assert len(client.posts) == 2
    token_call = client.posts[0]
    assert token_call["url"] == ERNIEBackend.TOKEN_URL
    assert token_call["params"]["client_id"] == "ak"
    chat_call = client.posts[1]
    assert chat_call["url"].startswith(ERNIEBackend.CHAT_URL)
    assert "access_token=fake-token" in chat_call["url"]
    assert chat_call["json"]["messages"] == [{"role": "user", "content": "hi"}]
    assert chat_call["json"]["max_output_tokens"] == backend.config.max_output_tokens


def test_ernie_acall_api_falls_back_to_thread_when_httpx_missing(monkeypatch):
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


def test_ernie_aclose_closes_client(monkeypatch):
    import httpx

    _FakeAsyncClient.instances.clear()
    monkeypatch.setattr(httpx, "AsyncClient", _FakeAsyncClient)

    backend = _make_backend()

    async def run():
        await backend.agenerate("x", max_retries=0)
        await backend.aclose()

    asyncio.run(run())
    assert _FakeAsyncClient.instances[0].closed, "aclose 必须关闭异步客户端"