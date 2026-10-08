# -*- coding: utf-8 -*-
"""L160 / B230：DiskCache 元数据写边收原子写（A182 逐面收路线第二面）。

判据口径（L99 / atomic_write 模块明文）：读侧把坏 JSON 变成**错误答案**的写边
必须收；读侧「按未命中重算」的可恢复代价不必动。`DiskCache` 两处写边据此分判：

- `_save_metadata`（元数据）**收**：读侧 `_load_metadata` 无 except 且在
  `__init__` 必调——旧写法的写窗口里，共享同一 cache_dir 的第二个实例构造
  当场崩（`JSONDecodeError` 冒泡），缓存整体不可用 = 错误答案。
- `set` 的缓存值写边**刻意不收**：读侧 `get` 把坏 JSON 按未命中重算
  （`return None`），重算即可恢复。这条豁免是**条件性的**——它挂在 get 的
  读侧行为上，所以本文件把它钉成机器：`get` 哪天改成抛异常，豁免即失效，
  测试当场红提示重新对账（A182 的「逐面收」口径就靠这类守卫传递）。
"""
import io
import json
import os
import pathlib

import pytest

from augmentor import cache as cache_mod
from augmentor.cache import DiskCache

_CACHE = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "cache.py"


def _cache(tmp_path):
    return DiskCache(cache_dir=str(tmp_path))


def test_write_failure_leaves_the_previous_metadata_intact(tmp_path, monkeypatch):
    """元数据写中断：已入账条目不丢、构造不炸——旧写法会把半份留在盘上。"""
    c = _cache(tmp_path)
    c.set("k1", {"v": 1})

    def boom(*args, **kwargs):
        raise RuntimeError("写中断")

    monkeypatch.setattr(cache_mod, "atomic_write_json", boom)
    with pytest.raises(RuntimeError):
        c.set("k2", {"v": 2})
    monkeypatch.undo()

    c2 = DiskCache(cache_dir=str(tmp_path))
    assert "k1" in c2._metadata, "旧元数据必须完好"
    assert "k2" not in c2._metadata, "写失败的条目不得以半份状态入账"


def test_no_temp_file_survives_a_successful_metadata_write(tmp_path):
    c = _cache(tmp_path)
    c.set("k1", {"v": 1})
    c.set("k2", {"v": 2})
    leftovers = [f for f in os.listdir(tmp_path) if f.endswith(".tmp")]
    assert leftovers == [], leftovers
    on_disk = json.loads(
        (tmp_path / "_metadata.json").read_text(encoding="utf-8")
    )
    assert set(on_disk) == {"k1", "k2"}


def test_the_value_write_edge_stays_exempt_because_get_treats_garbage_as_a_miss(tmp_path):
    """`set` 写边的豁免条件机器化：get 的读侧行为变了 ⇒ 本条红 ⇒ 重新对账。"""
    c = _cache(tmp_path)
    target = c._get_cache_path("k1")
    target.write_text('{"v": 1, "截断', encoding="utf-8")
    assert c.get("k1") is None, (
        "get 必须按未命中处理坏 JSON——这是缓存值写边免收的唯一依据"
    )


def test_the_metadata_edge_is_no_longer_an_open_w_plus_dump_pair():
    """静态面：元数据写边不再命中 l99 棘轮的「open(w) 窗口内 json.dump」形状。"""
    src = io.open(_CACHE, "r", encoding="utf-8", newline="").read()
    in_save = src.split("def _save_metadata", 1)[1].split("\n    def ", 1)[0]
    assert "json.dump(" not in in_save
    assert "atomic_write_json" in in_save
