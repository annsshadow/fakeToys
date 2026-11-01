# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 28 轮：`generate` 未命中路径只对 prompt 做一次 sha256。

改前 `_cache_lookup` 与 `_cache_store` 各调一次 `_cache_key(prompt)`——
长 prompt（数 KB～数十 KB）每请求双份哈希。改后 lookup 把 key 带回，
store 直接复用。判据用计数器点名 `_cache_key` 的调用次数：一次生成
（未命中 → 调 API → 写缓存）应恰好 1 次；第二次相同 prompt 命中缓存，
连 1 次都不用付（命中路径仍走 lookup 的 key）。
"""

from augmentor.config import ModelConfig
from augmentor.models.base import ModelBackend


class _CountingBackend(ModelBackend):
    """覆写 `_cache_key` 计数；`_call_api` 固定返回文本，不联网。"""

    def __init__(self):
        super().__init__(ModelConfig(type="openai", api_key="k", model="m"))
        self.key_calls = 0
        self.calls = 0

    def _cache_key(self, prompt: str) -> str:
        self.key_calls += 1
        return super()._cache_key(prompt)

    def _call_api(self, prompt: str) -> str:
        self.calls += 1
        return "generated"


def test_generate_hashes_prompt_once_when_miss():
    backend = _CountingBackend()
    result = backend.generate("A" * 4096)  # 未命中：lookup 算一次 key
    assert result == "generated"
    assert backend.calls == 1
    assert backend.key_calls == 1, (
        f"未命中生成应只做 1 次 sha256（lookup 带回 key 给 store 复用），"
        f"实际 {backend.key_calls}"
    )

    # 第二次相同 prompt：内存缓存命中，仍只付 lookup 的一次 key。
    result2 = backend.generate("A" * 4096)
    assert result2 == "generated"
    assert backend.key_calls == 2
    assert backend.calls == 1, "命中不应再调 API"


def test_agenerate_hashes_prompt_once_when_miss():
    import asyncio

    backend = _CountingBackend()

    async def run():
        return await backend.agenerate("B" * 4096)

    result = asyncio.run(run())
    assert result == "generated"
    assert backend.calls == 1
    assert backend.key_calls == 1, (
        f"异步未命中生成应只做 1 次 sha256，实际 {backend.key_calls}"
    )