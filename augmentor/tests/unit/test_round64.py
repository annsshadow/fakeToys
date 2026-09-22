# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第64轮: cache缓存管理"""
import pytest
from augmentor.cache import MemoryCache, DiskCache


class TestMemoryCache:
    def test_create(self):
        c = MemoryCache()
        assert c is not None

    def test_set_get(self):
        c = MemoryCache()
        c.set("key1", "value1")
        result = c.get("key1")
        assert result == "value1"

    def test_get_missing(self):
        c = MemoryCache()
        result = c.get("nonexistent")
        assert result is None

    def test_clear(self):
        c = MemoryCache()
        c.set("k", "v")
        c.clear()
        assert c.get("k") is None


class TestDiskCache:
    def test_create(self, tmp_path):
        c = DiskCache(str(tmp_path / "cache"))
        assert c is not None

    def test_set_get(self, tmp_path):
        c = DiskCache(str(tmp_path / "cache"))
        c.set("key1", "value1")
        result = c.get("key1")
        assert result == "value1"
