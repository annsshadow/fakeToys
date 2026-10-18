# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A341（L277）：`POST /api/config` 的目录值必须留在数据根白名单内

改前的链条是一句 `apply_section_update(section_config, updates[section])`：它只跑该节
`__post_init__` 里的数值判据，而 `vector.storage_dir` 是**路径**，没有任何读者判它落在
哪儿。实测后果两侧都坏：

- 写 `{"vector": {"storage_dir": "../outside"}}` 回 `200 配置已保存`，值进内存对象、
  也落盘；下一次持久化向量的时候就落在白名单根之外（`faiss.persist` 只判
  `storage_dir` **内部**的名字包含，不判它自己在哪儿）。
- 同一份字符串走数据端点时会被 `deps.resolve_within_roots` 判成 400/403 —— 也就是
  「客户端递路径」这条边有闸，「客户端改配置里的路径」那条边没有。

本文件钉三件事：坏值既不进内存也不落盘（回执 400/403）、好值照旧可写、
`_DIR_VALUED_KEYS` 这份名单与「可写节里的目录类键」逐格相等（多写或少写都红）。
"""

from dataclasses import fields

import pytest
from fastapi import FastAPI
from fastapi.testclient import TestClient

from api import deps
from api.routes import config as config_routes
from api.routes.config import _DIR_VALUED_KEYS, WRITABLE_SECTIONS

IN_ROOT = "vectors_ok"


def _dirish_keys():
    """从配置对象本身推出「可写节里所有目录/文件类键」，不抄名单"""
    from augmentor.config import AppConfig

    out = set()
    live = AppConfig()
    for section in WRITABLE_SECTIONS:
        sub = getattr(live, section)
        for f in fields(sub):
            if f.name.endswith(("_dir", "_path")) or f.name.endswith("file"):
                out.add((section, f.name))
    return out


@pytest.fixture
def served(tmp_path, monkeypatch):
    """把服务配置挪到 tmp_path 下的可写副本：仓库根那份 `config.yaml` 不许被测试改"""
    cfg = tmp_path / "config.yaml"
    cfg.write_text("vector:\n  storage_dir: data/vectors\n", encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_CONFIG_PATH", str(cfg))
    monkeypatch.delenv("AUGMENTOR_DATA_ROOTS", raising=False)
    monkeypatch.chdir(tmp_path)
    deps.invalidate_config_caches()
    deps.reset_pipeline()

    app = FastAPI()
    app.include_router(config_routes.router)
    app.dependency_overrides[deps.verify_api_key] = lambda: None
    client = TestClient(app)
    # 出厂默认把 `data` 当根；工作目录已是 tmp_path ⇒ 根为 tmp_path/data
    (tmp_path / "data").mkdir(exist_ok=True)
    yield client, cfg
    deps.reset_pipeline()
    deps.invalidate_config_caches()


class TestTheDirValueIsGated:
    @pytest.mark.parametrize("bad, expect", [
        ("../outside", 400),
        ("..", 400),
        ("data/../../outside", 400),
    ])
    def test_relative_escape_is_refused(self, served, bad, expect):
        client, cfg = served
        resp = client.post("/api/config", json={"vector": {"storage_dir": bad}})
        assert resp.status_code == expect
        assert "storage_dir" in resp.text or "路径" in resp.text
        # 判在写内存对象之前 ⇒ live 与磁盘都不许动
        assert deps.get_pipeline().config.vector.storage_dir != bad
        assert bad not in cfg.read_text(encoding="utf-8")
        assert not (cfg.parent / "outside").exists()

    def test_absolute_outside_is_refused(self, served, tmp_path):
        client, cfg = served
        outside = str((tmp_path.parent / "abs-outside-vector"))
        resp = client.post("/api/config", json={"vector": {"storage_dir": outside}})
        assert resp.status_code == 403
        assert outside not in cfg.read_text(encoding="utf-8")
        assert not (tmp_path.parent / "abs-outside-vector").exists()

    def test_in_root_value_still_writes(self, served):
        """零漂移臂：合法值必须照旧 200 且真的落盘，否则闸是恒真的"""
        client, cfg = served
        resp = client.post("/api/config", json={"vector": {"storage_dir": IN_ROOT}})
        assert resp.status_code == 200
        assert deps.get_pipeline().config.vector.storage_dir == IN_ROOT
        assert IN_ROOT in cfg.read_text(encoding="utf-8")

    @pytest.mark.parametrize("blank", [None, "", "   "])
    def test_blank_value_is_not_rejected_here(self, served, blank):
        """空值归该节自己的判据管，本闸不抢判：闸只看「非空路径落在哪」"""
        client, _ = served
        resp = client.post("/api/config", json={"vector": {"storage_dir": blank}})
        assert resp.status_code != 403


class TestTheRosterMatchesTheSurface:
    def test_roster_equals_derived_dir_keys(self):
        derived = _dirish_keys()
        assert set(_DIR_VALUED_KEYS.items()) == derived

    def test_roster_only_names_writable_sections(self):
        assert set(_DIR_VALUED_KEYS) <= set(WRITABLE_SECTIONS)


class TestInjectionProvesTheGateBites:
    def test_gate_removed_lets_the_escape_through(self, served, monkeypatch):
        """摘掉闸就复现原始缺陷：同一句请求 200 且坏值落盘"""
        client, cfg = served
        monkeypatch.setattr(
            config_routes, "_reject_escaping_dir_value", lambda *a: None)
        resp = client.post("/api/config", json={"vector": {"storage_dir": "../outside"}})
        assert resp.status_code == 200
        assert "../outside" in cfg.read_text(encoding="utf-8")
