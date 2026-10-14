# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L250 / A298：备份索引不再对同名备份谎报份数——写边按 id 收敛，读边照原样交回

**产品改动只有一处**：`augmentor/backup.py` 的 `DatasetBackup.backup()` 落索引时，先摘掉同
`backup_id` 的旧行再写新行（旧写法是无条件 `append`）。A303（同一只钟）住在 `test_backup_wiring_l231.py`，
本文件不重测。

**为什么「索引两行」不是形状偏好而是死账**：同名再备份时 `backup_path` 逐字相同，第二次那份
`atomic_write_json` 把第一次的内容**覆盖掉了**。旧行仍然挂在索引里，而它带的 `checksum` /
`item_count` / `file_size` 描述的是那份**已经不存在的内容** ⇒ 索引从此合法地含有一条指向被覆盖
数据的记录。更糟的是删除粒度与创建粒度不是同一把尺子（按 id 全清 vs 按次追加），于是「备份有哪些」
这张唯一权威表里的条数与磁盘上的份数可以长期不等，`clean_old_backups` 的「保留几份」也跟着算错。
按 id 收敛之后：磁盘一份 ⇒ 索引一行，写侧与删除侧的粒度对上，三面（SDK / HTTP / CLI）同源。

**修法选的是 A298 的 ①（创建时按 name 覆盖索引行）**，另外两案未做：
② 让 `backup_id` 真的唯一（同名追加后缀）——那会把「同名再备份」的语义从覆盖改成留档，磁盘落点
也得跟着变，是一次更大的行为变更；③ 维持现状只在回执里报「本 name 已有 N 行」——那等于把死账
留在表里只多报一个数。①的代价记在下面 `TestUnnamedBackupsInsideOneSecond`：同一秒内的两只
**匿名**备份仍然共享 id（收敛之后是「一行 + 一份文件」，自洽但只留一份）。

**十一组判据，写侧与读侧分开钉**：

1. 写边收敛：同名连打三次 ⇒ 索引 **1** 行，且那一行是**最后一次**的读数（条数与校验和跟着内容走）。
2. 不误伤：不同名各留一行——收敛的键是 id，不是「整张表只留一条」。
3. **行数 == 磁盘份数**的不变量，跨创建与删除两种操作都成立（本轮立的那把尺子的正面）。
4. 读边不改：存量 `index.json` 里的同名两行**照原样回两行**。少了这一支，「列表只有一行」可以靠
   读时去重恒真——而那是把数据藏起来，不是修写边。
5. 存量多行 + 再备份一次 ⇒ 写边把遗留的旧行一起摘掉（证明过滤读的是**加载来的**索引，不只是本轮
   会话内存里的那一行）。
6. 删一次的「全清」现量证人活在存量档上：手工塞的两行同名 ⇒ 一次删除两行全没、文件也没。
7. 匿名同秒共享 id 的边界当场钉住（改法 ② 若落地，这一支必须红并逼一次重新拍板）。
8. HTTP 面：`POST` 同名两次 ⇒ `GET` 回一行，且列表那一行与创建回执逐键相同。
9. CLI 面：`backup --action create` 同名两次 ⇒ `--action list` 打印「找到 1 个备份」。
10. 结构那一半：`backup()` 体内**没有**对 `_index` 的 `append`（全类零处），且有一条以
    `backup_id` 为条件的推导式——判据走 AST，注释里提一句 `append` 喂不饱这把尺子。
11. 陷阱自证：把写边退回旧的 `append`（注入臂），上面第 1 / 3 / 5 / 8 / 9 / 10 组逐臂点名喊红。

**边界**：本守卫不重测 L243 的校验和闸与 `clean_old_backups` 的删除算术（那两族量的是「坏备份
拦不拦」与「按时间挑哪几只删」），也不重测 L231 的四条调用形状与那只越界闸，更不重测 L249 的
「只读面零磁盘副作用」；它只量**索引里行数的算术**这一格。

跑法：`python -m pytest tests/unit/test_backup_index_collapse_l250.py -q --no-cov`
"""
import ast
import io
import json
import pathlib
import sys
from contextlib import redirect_stderr, redirect_stdout

import pytest

from augmentor.backup import DatasetBackup, verify_backup_integrity

from cli import main

ROOT = pathlib.Path(__file__).resolve().parents[2]
BACKUP_PY = ROOT / "augmentor" / "backup.py"

#: 一份 5 条的语料（与 l231 的探针语料同尺寸，`item_count 5` 那个读数来自它）
ITEMS = [{"instruction": f"q{i}", "input": "", "output": f"a{i}"} for i in range(5)]


def _write_source(path, items):
    path.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    return path


def _files_on_disk(backup_dir):
    """备份目录里真正落了盘的备份份数（`index.json` 是账本，不算一份备份）"""
    return sorted(p.relative_to(backup_dir).as_posix()
                  for p in backup_dir.rglob("*.json") if p.name != "index.json")


def _dup_rows(backup_dir, backup_id, older_timestamp):
    """把索引里那一行复制成**存量形状的两行**（L250 之前写出的 `index.json` 就是这个样子）"""
    index = json.loads((backup_dir / "index.json").read_text(encoding="utf-8"))
    row = next(b for b in index["backups"] if b["backup_id"] == backup_id)
    index["backups"].append({**row, "timestamp": older_timestamp})
    (backup_dir / "index.json").write_text(json.dumps(index, ensure_ascii=False), encoding="utf-8")
    return index["backups"]


class TestTheWriteEdgeCollapses:
    """1 / 2 / 3：同名收敛成一行、不同名不误伤、行数与磁盘份数对上"""

    def test_three_creates_with_one_name_leave_one_row_describing_the_last_file(self, tmp_path):
        src = _write_source(tmp_path / "s.json", ITEMS)
        big = _write_source(tmp_path / "big.json", ITEMS * 2)
        target = tmp_path / "bk"
        manager = DatasetBackup(str(target))

        for _ in range(2):
            manager.backup(str(src), name="dup")
        newest = manager.backup(str(big), name="dup")

        rows = manager.list_backups()
        assert [r["backup_id"] for r in rows] == ["dup"], f"同名没收敛：{rows}"
        assert rows[0] == newest.to_dict(), "留下的那一行不是最新那次备份"
        assert rows[0]["item_count"] == 10
        assert _files_on_disk(target) == ["dup.json"]
        # 唯一那一行对得上磁盘上的文件——改前那条指向被覆盖内容的死账不复存在
        assert verify_backup_integrity(str(target))["dup"]["checksum_match"] is True

    def test_different_names_keep_their_own_rows(self, tmp_path):
        """收敛的键是 `backup_id`，不是「整张表塌成一条」"""
        src = _write_source(tmp_path / "s.json", ITEMS)
        target = tmp_path / "bk"
        manager = DatasetBackup(str(target))
        for name in ("a", "b", "c"):
            manager.backup(str(src), name=name)

        assert [r["backup_id"] for r in manager.list_backups()] == ["a", "b", "c"]
        assert _files_on_disk(target) == ["a.json", "b.json", "c.json"]

    def test_the_row_count_tracks_the_disk_across_create_and_delete(self, tmp_path):
        """不变量：索引行数 == 磁盘份数，覆盖写、子目录名、删除之后都成立"""
        src = _write_source(tmp_path / "s.json", ITEMS)
        target = tmp_path / "bk"
        manager = DatasetBackup(str(target))
        manager.backup(str(src), name="a")
        manager.backup(str(src), name="a")
        manager.backup(str(src), name="sub/deep")

        assert len(manager.list_backups()) == len(_files_on_disk(target)) == 2
        assert manager.delete_backup("a") is True
        assert len(manager.list_backups()) == len(_files_on_disk(target)) == 1
        assert _files_on_disk(target) == ["sub/deep.json"]

    def test_a_fresh_instance_sees_the_collapsed_row(self, tmp_path):
        """收敛落在磁盘上，不是只在同一实例的内存里——换新实例读得回那一行"""
        src = _write_source(tmp_path / "s.json", ITEMS)
        target = tmp_path / "bk"
        DatasetBackup(str(target)).backup(str(src), name="dup")
        DatasetBackup(str(target)).backup(str(src), name="dup")

        rows = DatasetBackup(str(target)).list_backups()
        assert len(rows) == 1
        assert json.loads((target / "index.json").read_text(encoding="utf-8"))["backups"] == rows


class TestTheReadEdgeStillsHandsBackWhatIsThere:
    """4 / 5 / 6：存量索引里的同名多行——读时不藏、再写时一起摘、删除仍然全清"""

    LEGACY = ("2026-10-11T17:58:33.991804", "2026-10-11T17:58:33.997820")

    def _legacy_manager(self, tmp_path):
        src = _write_source(tmp_path / "s.json", ITEMS)
        target = tmp_path / "bk"
        DatasetBackup(str(target)).backup(str(src), name="dup")
        _dup_rows(target, "dup", self.LEGACY[0])
        return DatasetBackup(str(target)), src, target

    def test_a_legacy_two_row_index_is_still_listed_as_two_rows(self, tmp_path):
        """读边不许「顺手去重」：写侧收敛不等于把存量数据藏起来"""
        manager, _src, _target = self._legacy_manager(tmp_path)
        rows = manager.list_backups()
        assert len(rows) == 2 and [r["backup_id"] for r in rows] == ["dup", "dup"]

    def test_the_next_backup_clears_the_legacy_rows_too(self, tmp_path):
        """过滤读的是**加载来的**索引：遗留的旧行也会被新行一起顶掉"""
        manager, src, target = self._legacy_manager(tmp_path)
        manager.backup(str(_write_source(tmp_path / "big.json", ITEMS * 2)), name="dup")

        rows = DatasetBackup(str(target)).list_backups()
        assert len(rows) == 1
        assert rows[0]["item_count"] == 10
        assert _files_on_disk(target) == ["dup.json"]

    def test_one_delete_clears_every_legacy_row_and_the_file(self, tmp_path):
        """「按 id 全清」这一格的证人现在是存量档：两行一起没，文件也删"""
        manager, _src, target = self._legacy_manager(tmp_path)
        assert manager.delete_backup("dup") is True

        assert manager.list_backups() == []
        assert not (target / "dup.json").exists()


class TestUnnamedBackupsInsideOneSecond:
    """7：A298 的 ②（让 id 真的唯一）没做 ⇒ 同一秒内的两只匿名备份仍然共享 id，当场钉住"""

    def test_two_unnamed_backups_in_one_second_share_one_id_and_one_file(self, tmp_path, monkeypatch):
        import augmentor.backup as bk

        real = bk.datetime
        frozen = real(2026, 10, 11, 17, 58, 33, 991804)

        class Frozen:
            @classmethod
            def now(cls):
                return frozen

        src = _write_source(tmp_path / "s.json", ITEMS)
        target = tmp_path / "bk"
        monkeypatch.setattr(bk, "datetime", Frozen)
        first = DatasetBackup(str(target)).backup(str(src))
        second = DatasetBackup(str(target)).backup(str(src))

        assert first.backup_id == second.backup_id == "backup_20261011_175833"
        # 收敛之后的自洽形状：一行 + 一份文件（改前是「两行 + 一份」，旧行指向被覆盖的内容）
        assert DatasetBackup(str(target)).list_backups() == [second.to_dict()]
        assert _files_on_disk(target) == ["backup_20261011_175833.json"]


class TestHttpAndCliFacesAgree:
    """8 / 9：同一条产品改动在另外两面上给同一个答案"""

    @pytest.fixture
    def client_with_root(self, tmp_path, monkeypatch):
        root = tmp_path / "ds"
        root.mkdir()
        _write_source(root / "p.json", ITEMS)
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
        monkeypatch.setenv("AUGMENTOR_API_KEY", "")
        monkeypatch.chdir(tmp_path)
        from fastapi.testclient import TestClient

        from api.main import app

        return TestClient(app), root

    @staticmethod
    def _run_cli(argv):
        old_argv = sys.argv
        sys.argv = argv
        out, err = io.StringIO(), io.StringIO()
        code = None
        try:
            with redirect_stdout(out), redirect_stderr(err):
                main()
        except SystemExit as exc:
            code = exc.code
        finally:
            sys.argv = old_argv
        return out.getvalue(), err.getvalue(), code

    def test_two_creates_with_one_name_list_one_row(self, client_with_root):
        client, root = client_with_root
        first = client.post("/api/system/backups", json={"input_file": "p.json", "name": "dup"})
        second = client.post("/api/system/backups", json={"input_file": "p.json", "name": "dup"})

        assert first.status_code == second.status_code == 200
        listed = client.get("/api/system/backups").json()["backups"]
        assert len(listed) == 1
        assert listed[0] == second.json(), "列表那一行与创建回执不等 ⇒ 索引留的是旧读数"
        assert _files_on_disk(root / ".backups") == ["dup.json"]

    def test_the_cli_list_reports_one_backup_after_two_creates(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        src = _write_source(tmp_path / "s.json", ITEMS)
        target = str(tmp_path / "bk")
        for _ in range(2):
            out, err, code = self._run_cli([
                "cli", "backup", "--action", "create", "--input", str(src),
                "--name", "dup", "--backup-dir", target])
            assert code is None, f"create 档崩了：{err}"

        out, err, code = self._run_cli([
            "cli", "backup", "--action", "list", "--backup-dir", target])
        assert code is None, f"list 档崩了：{err}"
        assert "找到 1 个备份" in out, out


class TestTheWriteEdgeIsAFilterNotAnAppend:
    """10：结构那一半，走 AST 而不是比文本"""

    @staticmethod
    def _backup_body():
        tree = ast.parse(BACKUP_PY.read_text(encoding="utf-8"))
        cls = next(n for n in tree.body
                   if isinstance(n, ast.ClassDef) and n.name == "DatasetBackup")
        return next(n for n in cls.body
                    if isinstance(n, ast.FunctionDef) and n.name == "backup"), cls

    def test_no_append_to_the_index_left_anywhere_in_the_manager(self):
        """全类零处「往 `self._index[...]` 上 append」：旧写法只有 `backup()` 一处，收成过滤后
        整类都不该有它；留着 append 就等于给下一次「按次追加」留门"""
        _body, cls = self._backup_body()
        hits = []
        for call in ast.walk(cls):
            if isinstance(call, ast.Call) and isinstance(call.func, ast.Attribute) \
                    and call.func.attr == "append":
                base = call.func.value
                if isinstance(base, ast.Subscript) and isinstance(base.value, ast.Attribute) \
                        and base.value.attr == "_index":
                    hits.append(call.lineno)
        assert hits == [], f"`backup.py` 里又出现按次追加：{hits}"

    def test_the_index_write_in_backup_is_guarded_by_a_backup_id_condition(self):
        body, _cls = self._backup_body()
        # 推导式的过滤住在 `generator.ifs` 里而不是节点自己身上
        conditions = [cond for comp in ast.walk(body) if isinstance(comp, ast.ListComp)
                      for gen in comp.generators for cond in gen.ifs]
        guarded = [cond for cond in conditions if any(
            isinstance(n, ast.Name) and n.id == "backup_id" for n in ast.walk(cond))]
        assert guarded, "backup() 里没有「以 backup_id 为条件」的过滤 ⇒ 收敛不在写边上"
        # L273 之后写点不再躺在函数体顶层：整段读-改-写罩在 `with path_lock(...)` 里，
        # 所以按 `body.body` 只取顶层语句会数到 0。判据本身一字未动——**恰好一处**对
        # `self._index[...]` 的赋值，多一处（例如又按次追加）照样红。
        assigns = [n for n in ast.walk(body) if isinstance(n, (ast.Assign, ast.AugAssign))]
        written = [n for n in assigns if any(
            isinstance(t, ast.Subscript) and isinstance(t.value, ast.Attribute)
            and t.value.attr == "_index" for t in getattr(n, "targets", []))]
        assert len(written) == 1, f"`backup()` 里对索引的写点应当只有一处，实得 {len(written)}"
