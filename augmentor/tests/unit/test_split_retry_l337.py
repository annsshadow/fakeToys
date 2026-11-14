# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L337 波 1b-2 守卫：A289 两支 shuffle 同口径、A356 读侧有限重试与 503。

零 parametrize；判据都是行为面，产品码改回旧形状就红。
"""

import builtins
import json

import pytest
from fastapi import HTTPException

CORPUS = [{"instruction": "问题%d" % i, "intent": "c%d" % (i % 3), "pos": i}
          for i in range(30)]


# ------------------------------------------------------------------ A289
def _members(parts):
    return {item["pos"] for seg in parts for item in seg}


def test_shuffle_only_changes_order_not_membership():
    """两支统一后：shuffle 只管段内顺序，成员集合由 seed 决定且不随 shuffle 变"""
    from augmentor.dataset_ops import DatasetOperations, SplitConfig

    plain = DatasetOperations().split(CORPUS, SplitConfig(ratios=(0.5, 0.25, 0.25),
                                                          seed=7, shuffle=False))
    shuffled = DatasetOperations().split(CORPUS, SplitConfig(ratios=(0.5, 0.25, 0.25),
                                                             seed=7, shuffle=True))
    assert _members(plain) == _members(shuffled) == set(range(30))
    assert [len(x) for x in plain] == [len(x) for x in shuffled]


def test_shuffle_false_restores_original_relative_order_within_each_segment():
    """shuffle=False 把每段按输入原序回排，不再等于输入的前 50%/25%/25%"""
    from augmentor.dataset_ops import DatasetOperations, SplitConfig

    parts = DatasetOperations().split(CORPUS, SplitConfig(ratios=(0.5, 0.25, 0.25),
                                                          seed=7, shuffle=False))
    for seg in parts:
        positions = [item["pos"] for item in seg]
        assert positions == sorted(positions), positions
    assert [item["pos"] for item in parts[0]][:3] != [0, 1, 2], "改前的「前三段前 N 条」回潮了"


def test_seed_still_drives_membership_when_shuffle_is_false():
    """反证臂：改前 shuffle=False 时 seed 完全空转，现在必须真的影响谁进哪段"""
    from augmentor.dataset_ops import DatasetOperations, SplitConfig

    a = DatasetOperations().split(CORPUS, SplitConfig(ratios=(0.5, 0.25, 0.25),
                                                      seed=1, shuffle=False))
    b = DatasetOperations().split(CORPUS, SplitConfig(ratios=(0.5, 0.25, 0.25),
                                                      seed=99, shuffle=False))
    assert {item["pos"] for item in a[0]} != {item["pos"] for item in b[0]}


# ------------------------------------------------------------------ A356
def test_transient_permission_error_is_retried_and_then_succeeds(tmp_path, monkeypatch):
    """瞬时 PermissionError 重试后读到原数据，不把一次撞车变成 500"""
    from api import deps

    path = tmp_path / "d.json"
    path.write_text(json.dumps([{"instruction": "甲"}]), encoding="utf-8")
    real_open = builtins.open
    calls = {"n": 0}

    def flaky_open(*args, **kwargs):
        if args[0] == path and calls["n"] == 0:
            calls["n"] += 1
            raise PermissionError(13, "denied")
        return real_open(*args, **kwargs)

    monkeypatch.setattr(builtins, "open", flaky_open)
    monkeypatch.setattr(deps.time, "sleep", lambda _s: None) if hasattr(deps, "time") else None
    got = deps._read_json_with_retry(path, attempts=3, delay=0)
    assert got == [{"instruction": "甲"}]
    assert calls["n"] == 1


def test_permanent_permission_error_is_503_with_a_fixed_chinese_detail(tmp_path, monkeypatch):
    """重试耗尽回 503，响应体不带 Permission / Errno / 部署路径"""
    from api import deps

    path = tmp_path / "d.json"
    path.write_text("[]", encoding="utf-8")
    real_open = builtins.open

    def always_denied(*args, **kwargs):
        if args[0] == path:
            raise PermissionError(13, "denied")
        return real_open(*args, **kwargs)

    monkeypatch.setattr(builtins, "open", always_denied)
    with pytest.raises(HTTPException) as exc:
        deps._read_json_with_retry(path, attempts=3, delay=0)
    assert exc.value.status_code == 503
    assert "Permission" not in exc.value.detail and "Errno" not in exc.value.detail
    assert str(tmp_path) not in exc.value.detail


def test_normal_read_is_untouched(tmp_path):
    """反证臂：没有撞车时读一次就返回，内容一字不改"""
    from api import deps

    path = tmp_path / "d.json"
    path.write_text(json.dumps([{"v": 1.5}], ensure_ascii=False), encoding="utf-8")
    assert deps._read_json_with_retry(path) == [{"v": 1.5}]
