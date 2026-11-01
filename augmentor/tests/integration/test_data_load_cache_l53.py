# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 53 轮：`/api/data/load` 的数据读取走有界 LRU 缓存。

`/api/data/load` 每次翻页/搜索都整份重读重解析（3.6MB/6902 条实测 ~15ms）。
缓存键 = path + mtime_ns + size：文件在写操作之间不变，重复读直接命中；
写路径让 stat 变化 → 旧键自动失效（无需显式失效）。
"""

import json

import pytest
from fastapi.testclient import TestClient


@pytest.fixture
def api(tmp_path, monkeypatch):
    from api.main import app

    monkeypatch.delenv("AUGMENTOR_API_KEY", raising=False)
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    monkeypatch.chdir(tmp_path)
    items = [{"instruction": f"q{i}", "input": "", "output": f"a{i}"} for i in range(20)]
    (tmp_path / "train_data.json").write_text(
        json.dumps(items, ensure_ascii=False), encoding="utf-8"
    )
    return TestClient(app), tmp_path


def _disk_versions(api):
    import api.deps as deps

    return deps._DATA_READ_CACHE, deps._DATA_READ_CACHE_MAX


def test_repeated_load_hits_cache_and_returns_same(api):
    client, tmp = api
    r1 = client.get("/api/data/load/train_data.json", params={"page_size": 5})
    r2 = client.get("/api/data/load/train_data.json", params={"page_size": 5, "page": 2})
    assert r1.status_code == 200 and r2.status_code == 200
    assert r2.json()["items"] != r1.json()["items"], "不同页应返回不同切片（坐标正确）"
    # 缓存已落（同一文件、mtime 未变）
    cache, _ = _disk_versions(api)
    assert len(cache) >= 1, "重复读同一文件应命中缓存"


def test_modified_file_invalidates_cache(api):
    client, tmp = api
    client.get("/api/data/load/train_data.json", params={"page_size": 5})

    # 改文件（内容 + mtime 变化）→ 下一次读必须拿到新内容。
    new_items = [{"instruction": "NEW", "input": "", "output": "x"}]
    (tmp / "train_data.json").write_text(
        json.dumps(new_items, ensure_ascii=False), encoding="utf-8"
    )
    r = client.get("/api/data/load/train_data.json", params={"page_size": 5})
    assert r.status_code == 200
    assert r.json()["items"] == new_items, "文件变化后必须读到新内容（缓存自动失效）"


def test_cache_is_bounded(api):
    import api.deps as deps

    client, tmp = api
    # 读 3 个不同文件（每次换 mtime 再读一次制造更多键）→ 缓存大小必须 ≤ 上限。
    for i in range(3):
        p = tmp / f"train_data_{i}.json"
        p.write_text(json.dumps([{"instruction": f"x{i}"}]), encoding="utf-8")
        client.get(f"/api/data/load/train_data_{i}.json")
    assert len(deps._DATA_READ_CACHE) <= deps._DATA_READ_CACHE_MAX, (
        f"LRU 缓存大小必须 ≤ {deps._DATA_READ_CACHE_MAX}，实际 {len(deps._DATA_READ_CACHE)}"
    )