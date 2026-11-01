# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 55 轮：API 层不再使用弃用的 `asyncio.get_event_loop()`。

Python 3.12+ 在协程内调用它发 DeprecationWarning（未来版本移除），
且 `asyncio.to_thread` 语义等价（默认执行器 + kwargs 透传）。
本守卫防止 `loop = asyncio.get_event_loop()` 形态复活。
"""

from pathlib import Path

API_DIR = Path(__file__).resolve().parents[2] / "api"


def test_api_layer_has_no_get_event_loop():
    offenders = []
    for p in API_DIR.rglob("*.py"):
        if "get_event_loop" in p.read_text(encoding="utf-8"):
            offenders.append(str(p.relative_to(API_DIR)))
    assert not offenders, (
        f"api/ 下不应使用弃用的 asyncio.get_event_loop（用 asyncio.to_thread）：{offenders}"
    )