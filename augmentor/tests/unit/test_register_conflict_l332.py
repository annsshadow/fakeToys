# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""L332 / A306：同名再登记必须回 409，且原有登记一字不动。

为什么这不是「加个 409」就完事：改前同名再登记是**静默覆盖** ——
`created_at` 被刷新成第二次时间、第二次送空 description/tags 会把第一次
写进去的值抹成空，而前端「登记」按钮看起来是幂等的。守卫两侧都钉：
SDK 面（锁内判据）与 HTTP 面（409 具名捕获，不落 500 兜底）。
"""
import json
import pathlib

import pytest

from augmentor.dependency import DependencyManager
from augmentor.exceptions import DatasetAlreadyRegistered


def _manager(tmp_path):
    return DependencyManager(registry_path=str(tmp_path / "reg"))


def test_second_registration_is_rejected_without_touching_the_first(tmp_path):
    mgr = _manager(tmp_path)
    first = mgr.register_dataset("dsA", "/fake/a.json", 3, description="初版", tags=["x"])
    with pytest.raises(DatasetAlreadyRegistered) as exc:
        mgr.register_dataset("dsA", "/fake/b.json", 99, description="", tags=[])
    assert "dsA" in str(exc.value)

    fresh = _manager(tmp_path)
    rows = fresh._datasets
    assert len(rows) == 1
    kept = list(rows.values())[0]
    assert kept.created_at == first.created_at, "created_at 不得被刷新"
    assert kept.description == "初版" and kept.tags == ["x"], "空值不得抹掉旧值"
    assert kept.item_count == 3 and kept.path == "/fake/a.json"


def test_different_names_still_register(tmp_path):
    mgr = _manager(tmp_path)
    mgr.register_dataset("dsA", "/fake/a.json", 1)
    mgr.register_dataset("dsB", "/fake/b.json", 2)
    assert len(_manager(tmp_path)._datasets) == 2


def test_conflict_gate_is_inside_register_dataset_and_after_reload():
    """判据与『锁内重读』同一段：摘掉判据，第二次登记就会覆盖整条。"""
    import augmentor.dependency as dep

    src = pathlib.Path(dep.__file__).read_text(encoding="utf-8")
    body = src.split("def register_dataset", 1)[1].split("\n    def ", 1)[0]
    assert "self._datasets = self._load_datasets()" in body
    assert "raise DatasetAlreadyRegistered(dataset_id, name)" in body
    assert body.index("self._datasets = self._load_datasets()") < body.index(
        "raise DatasetAlreadyRegistered(dataset_id, name)"
    ), "判据必须在锁内重读之后、写回之前"


def test_http_face_declares_named_409_capture():
    """路由必须具名捕获冲突类：摘掉就会落 to_http_error 的 500 兜底。"""
    src = pathlib.Path(__file__).resolve().parents[2] / "api" / "routes" / "system_ops.py"
    text = src.read_text(encoding="utf-8")
    assert "except DatasetAlreadyRegistered as e:" in text
    assert "status_code=409" in text
    fn = text.split("async def dependency_register", 1)[1].split("\n@router.", 1)[0]
    assert fn.index("except DatasetAlreadyRegistered as e:") < fn.index(
        "except Exception as e:"
    ), "具名捕获必须在通用兜底之前"
