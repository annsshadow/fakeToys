# -*- coding: utf-8 -*-
"""L162 / B232：BackupManager 索引写边收原子写（A182 逐面收路线第四面）。

与 L160（DiskCache）/ L161（VersionControl）完全同构：`_load_index` 无 except
且在 `__init__` 必调 ⇒ 半份索引窗口 = 新实例构造当场崩 = 错误答案档，索引两边
必收；快照（`backup` 的备份文件写）与恢复输出（`restore` 的 output 写）两边
读侧响亮抛异常 = 可恢复档不收——它们的「同名覆盖写中途崩毁既有」窗口是判据
口径未覆盖的另一档，记档待裁，不在本轮擅自扩判据。
"""
import json
import os

import pytest

from augmentor import backup as backup_mod
from augmentor.backup import DatasetBackup


def _manager(tmp_path):
    return DatasetBackup(backup_dir=str(tmp_path / "bk"))


def _source(tmp_path):
    src = tmp_path / "src.json"
    src.write_text(json.dumps([{"a": 1}]), encoding="utf-8")
    return str(src)


def test_index_write_failure_leaves_the_previous_index_intact(tmp_path, monkeypatch):
    """索引写中断：已入账备份不丢、新构造不炸——旧写法会把半份索引留在盘上。"""
    mgr = _manager(tmp_path)
    mgr.backup(_source(tmp_path), name="b1")

    def boom(*args, **kwargs):
        raise RuntimeError("写中断")

    monkeypatch.setattr(backup_mod, "atomic_write_json", boom)
    with pytest.raises(RuntimeError):
        mgr.backup(_source(tmp_path), name="b2")
    monkeypatch.undo()

    fresh = DatasetBackup(backup_dir=str(tmp_path / "bk"))
    assert len(fresh._index["backups"]) == 1, "旧索引必须完好"


def test_no_temp_file_survives_a_successful_index_write(tmp_path):
    mgr = _manager(tmp_path)
    mgr.backup(_source(tmp_path), name="b1")
    leftovers = [f for f in os.listdir(tmp_path / "bk") if f.endswith(".tmp")]
    assert leftovers == [], leftovers
    on_disk = json.loads((tmp_path / "bk" / "index.json").read_text(encoding="utf-8"))
    assert len(on_disk["backups"]) == 1


def test_snapshot_edge_stays_exempt_because_reads_fail_loud(tmp_path):
    """快照写边的豁免条件机器化：校验和不匹配要出声 + 坏内容读侧响亮失败。"""
    import pathlib

    from augmentor.exceptions import BackupIntegrityError

    mgr = _manager(tmp_path)
    mgr.backup(_source(tmp_path), name="b1")
    backup_file = tmp_path / "bk" / "b1.json"
    backup_file.write_text('{"a": 1, "截断', encoding="utf-8")
    # 校验和哨兵先出声
    info = mgr.get_backup_info("b1")
    real = pathlib.Path(tmp_path / "bk" / "b1.json").read_bytes()
    assert len(real) > 0
    # 读侧响亮失败，不被吞成空数据。L243（A299）把校验和闸接回来之后，冒泡的那一只换成
    # `BackupIntegrityError`：它同样是 `ValueError` 的子类，且排在解析与写盘**之前**，
    # 所以「响亮失败」与「可恢复档不收」这两条判据一字未动，只是哨兵换了个名字。
    with pytest.raises(BackupIntegrityError):
        mgr.restore("b1", str(tmp_path / "out.json"))
    assert not (tmp_path / "out.json").exists(), "闸若排在写盘之后，豁免条件当场作废"


def test_the_index_edges_are_no_longer_open_w_plus_dump_pairs():
    """静态面：索引的**唯一**写边是 `_save_index` 的原子写，而构造与读边现在是零写

    改前这一条是按源码文本切函数体，查「没有 `json.dump(` / 有 `atomic_write_json`」。
    L249 收 A332 时把 `_load_index` 里那份默认索引的落盘摘掉了（只读面不留磁盘副作用），
    写边因此从两条收成一条——文本判据在这里会走样：注释里提一句 `atomic_write_json`
    就能把「有原子写」喂饱。所以判据换成 AST：**只有调用点算数**，注释不算。
    """
    import ast
    import pathlib

    src_path = (
        pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "backup.py"
    )
    tree = ast.parse(src_path.read_text(encoding="utf-8"))
    cls = next(n for n in tree.body
               if isinstance(n, ast.ClassDef) and n.name == "DatasetBackup")
    bodies = {n.name: n for n in cls.body if isinstance(n, ast.FunctionDef)}

    def callees(fn_name):
        """函数体里所有调用的名字（属性调用取末段：`self._backup_dir.mkdir` → mkdir）"""
        return [n.func.attr if isinstance(n.func, ast.Attribute) else getattr(n.func, "id", "")
                for n in ast.walk(bodies[fn_name]) if isinstance(n, ast.Call)]

    assert callees("_save_index").count("atomic_write_json") == 1
    assert "open" not in callees("_save_index")
    for fn_name in ("__init__", "_load_index"):
        made = callees(fn_name)
        assert "atomic_write_json" not in made and "mkdir" not in made, fn_name
    # 读边仍然要 open，但只许读：`open(path, 'r', encoding=...)`
    reads = [n for n in ast.walk(bodies["_load_index"])
             if isinstance(n, ast.Call) and getattr(n.func, "id", "") == "open"]
    assert len(reads) == 1
    mode = reads[0].args[1] if len(reads[0].args) > 1 else reads[0].keywords and next(
        (k.value for k in reads[0].keywords if k.arg == "mode"), None)
    assert isinstance(mode, ast.Constant) and mode.value == "r", "读边出现写模式 ⇒ 只读面又落盘"
