# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L338 A309 守卫：SDK 注销、DELETE / 建边两条 HTTP 写入口。

零 parametrize；产品码改回旧形状（没有 remove_dataset / 没有两条端点）就红。
"""

import pathlib

from fastapi.testclient import TestClient


def _manager(tmp_path):
    from augmentor.dependency import DependencyManager

    return DependencyManager(str(tmp_path / "registry"))


def test_remove_dataset_deletes_the_entry_and_its_edges(tmp_path):
    """注销必须连边一起清，否则图里会留下悬空端点"""
    m = _manager(tmp_path)
    a = m.register_dataset("A", str(tmp_path / "a.json"), 1)
    b = m.register_dataset("B", str(tmp_path / "b.json"), 1)
    m.add_dependency(a.dataset_id, b.dataset_id, "derived")

    assert m.remove_dataset(a.dataset_id) is True
    assert a.dataset_id not in {d.dataset_id for d in m.list_datasets()}
    assert m.get_dependencies(b.dataset_id) == [], "指向已注销节点的边没被清掉"
    # 重新构造一个 manager（从磁盘读），确认真的落盘而不是只改内存
    assert _manager(tmp_path).get_dependencies(b.dataset_id) == []


def test_remove_dataset_is_idempotent(tmp_path):
    """反证臂：不存在的 id 返回 False 且不抛异常（DELETE 幂等）"""
    m = _manager(tmp_path)
    assert m.remove_dataset("no-such-id") is False


def test_delete_endpoint_is_reachable_and_idempotent(tmp_path, monkeypatch):
    """HTTP 面上第一次真的删掉、第二次 deleted=False"""
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    from api.main import app

    m = _manager(tmp_path)
    info = m.register_dataset("A", str(tmp_path / "a.json"), 1)
    client = TestClient(app)
    params = {"registry_path": "registry"}

    first = client.delete(f"/api/system/dependency/datasets/{info.dataset_id}",
                          params=params)
    assert first.status_code == 200, first.text
    assert first.json() == {"deleted": True, "dataset_id": info.dataset_id}
    second = client.delete(f"/api/system/dependency/datasets/{info.dataset_id}",
                           params=params)
    assert second.status_code == 200 and second.json()["deleted"] is False


def test_edge_endpoint_creates_a_real_edge(tmp_path, monkeypatch):
    """建边端点不再让 edges 恒为空：回执带新边，随后图里能读到它"""
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    from api.main import app

    m = _manager(tmp_path)
    a = m.register_dataset("A", str(tmp_path / "a.json"), 1)
    b = m.register_dataset("B", str(tmp_path / "b.json"), 1)
    client = TestClient(app)
    params = {"registry_path": "registry"}

    created = client.post("/api/system/dependency/edges", params=params, json={
        "source_dataset": a.dataset_id, "target_dataset": b.dataset_id,
        "dependency_type": "derived", "description": "L338/A309"})
    assert created.status_code == 200, created.text
    edge = created.json()["edge"]
    assert edge["source_dataset"] == a.dataset_id
    assert edge["target_dataset"] == b.dataset_id
    assert edge["dependency_type"] == "derived"

    graph = client.get("/api/system/dependency/graph", params=params)
    assert graph.status_code == 200
    assert any(e["source"] == a.dataset_id and e["target"] == b.dataset_id
               for e in graph.json()["edges"]), graph.json()["edges"]
