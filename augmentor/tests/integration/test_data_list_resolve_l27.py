# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 27 轮：`/api/data/list` 只读优化 —— 单根不再逐文件 `Path.resolve()`。

去重只在**多个根互相嵌套**时才有意义（同一文件可能被两个根的 glob 各命中一次）；
单根是默认部署（AUGMENTOR_DATA_ROOTS 通常只配一个目录），glob 天然无重复，
此前每请求对每个文件付一次 resolve（实测每文件 ~0.2ms，占扫描时间 99%）。

本测试钉住两半边：
- 单根：返回全部文件、无重复（性能形态：不付 resolve）；
- 多根嵌套：同一文件只出现一次（去重语义没有被优化掉）。
"""

import json
import os
from pathlib import Path

import pytest
from fastapi.testclient import TestClient


@pytest.fixture
def api(tmp_path, monkeypatch):
    from api.main import app

    monkeypatch.delenv("AUGMENTOR_API_KEY", raising=False)
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    monkeypatch.chdir(tmp_path)
    for i in range(3):
        (tmp_path / f"train_data_{i}.json").write_text(json.dumps([{}]), encoding="utf-8")
    return TestClient(app), tmp_path


def test_single_root_lists_all_without_duplicates(api):
    client, tmp = api
    resp = client.get("/api/data/list")
    assert resp.status_code == 200, resp.text
    names = [f["name"] for f in resp.json()["files"]]
    assert names == ["train_data_0.json", "train_data_1.json", "train_data_2.json"]
    assert len(names) == len(set(names))


def test_nested_roots_dedup_still_works(api):
    client, tmp = api
    # 两个根互相嵌套：根 A = tmp，根 B = tmp（同一目录）。
    # 同一文件会被两个根的 glob 各命中一次 —— 去重必须保证只出现一次。
    monkeypatch_data_roots = os.pathsep.join([str(tmp), str(tmp)])
    import api.routes.data as route

    old = os.environ.get("AUGMENTOR_DATA_ROOTS")
    os.environ["AUGMENTOR_DATA_ROOTS"] = monkeypatch_data_roots
    try:
        resp = client.get("/api/data/list")
    finally:
        if old is None:
            os.environ.pop("AUGMENTOR_DATA_ROOTS", None)
        else:
            os.environ["AUGMENTOR_DATA_ROOTS"] = old
    assert resp.status_code == 200, resp.text
    names = [f["name"] for f in resp.json()["files"]]
    assert names == ["train_data_0.json", "train_data_1.json", "train_data_2.json"]
    assert len(names) == len(set(names)), "嵌套根去重失效：同一文件出现多次"