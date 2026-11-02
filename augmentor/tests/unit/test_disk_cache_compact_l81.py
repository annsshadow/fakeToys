# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 81 轮：磁盘缓存值改为紧凑 JSON 落盘。

`DiskCache.set` 旧实现用 `indent=2` 写缓存值——读者只有 `get()` 的
`json.load`（对缩进无感），而值是模型响应这类大块文本，缩进把文件体积与
写入 I/O 白白放大。本文件锁三件事：

1. 紧凑格式仍然逐字段读回同一份值（往返等价）；
2. 文件体积显著小于 `indent=2`（改动确实省了字节，不是空转）；
3. 非 ASCII / 中文不被转义成 `\\uXXXX`（`ensure_ascii=False` 保持）。
"""

import json
import sys
from pathlib import Path

AI_DIR = Path(__file__).resolve().parents[2]
if str(AI_DIR) not in sys.path:
    sys.path.insert(0, str(AI_DIR))

from augmentor.cache import DiskCache  # noqa: E402


def _payload():
    return {
        "id": "resp-1",
        "choices": [
            {"index": i, "text": "这是第 %d 条带缩进会变大的中文响应内容" % i,
             "meta": {"score": i / 7.0, "tags": ["a", "b", "c"]}}
            for i in range(120)
        ],
    }


def test_compact_round_trip_keeps_value(tmp_path):
    cache = DiskCache(str(tmp_path / "c"))
    value = _payload()
    cache.set("k1", value)
    assert cache.get("k1") == value


def test_compact_file_is_smaller_than_pretty(tmp_path):
    cache = DiskCache(str(tmp_path / "c"))
    value = _payload()
    cache.set("k1", value)
    written = cache._get_cache_path("k1").read_bytes()
    pretty = json.dumps(value, ensure_ascii=False, indent=2).encode("utf-8")
    assert len(written) < len(pretty) * 0.75, (
        "紧凑写入没有显著省体积：%d vs %d" % (len(written), len(pretty)))
    assert len(written) > 0


def test_compact_keeps_non_ascii_unescaped(tmp_path):
    cache = DiskCache(str(tmp_path / "c"))
    cache.set("k2", {"text": "中文响应"})
    raw = cache._get_cache_path("k2").read_bytes().decode("utf-8")
    assert "中文响应" in raw
    assert "\\u" not in raw


def test_stats_size_matches_compact_file(tmp_path):
    cache = DiskCache(str(tmp_path / "c"))
    cache.set("k3", _payload())
    stats = cache.stats
    assert stats["entries"] == 1
    actual = cache._get_cache_path("k3").stat().st_size
    assert stats["total_size_bytes"] == actual
