# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L249 / A332 + A307：只读面**零磁盘副作用**——读盘那一趟不许留下目录

产品改动住在两处，同一个根因：

- `augmentor/backup.py:55-64`（`DatasetBackup.__init__` 不再 `mkdir`）与
  `augmentor/backup.py:66-75`（`_load_index` 索引不在场时交回内存空账，不再落默认 `index.json`）
- `augmentor/dependency.py:89-106`（`DependencyManager.__init__` 不再 `mkdir`）

**为什么本轮动的是库而不是路由与 CLI**：改前仓外探针量到 13 只读面里**只有 1 只**不建目录
（`GET /api/system/backups/verify`），而它干净的原因恰恰是路由里手写了
`if not resolved.exists()` 挡板——挡板是逐只打的，SDK 面与 CLI 面一只都没有。逐面挡板追不上
新增的面；摘掉副作用的来源，六只 GET、六只 SDK 读入口、五只 CLI 读档一起变干净（三面同源，
与 L244 把 verify / cleanup 接进 CLI 是同一路套路）。

**十组判据，全部是结构判据或磁盘事实判据，不量墙钟**：

1. 两只构造的函数体里**没有任何写调用**（`mkdir` / `atomic_write_json` / `open` / `write_text`），
   AST 数调用点而不是比文本——注释里提一句 `atomic_write_json` 喂不饱这条尺子。
2. 读边的 `open` 只许存在且模式是 `'r'`：把「读盘」与「顺手建一份」在语义上分开。
3. **只读名单是 AST 实推的，不是抄的**：`DatasetBackup` 与 `DependencyManager` 的公开方法按
   「体内有没有落索引 / 落文件 / 删文件的调用」二分为读侧与写侧（只按「有没有调 `_save_index`」
   分会把 `restore` 误判成读方法——它落输出文件而不动索引），`backup.py` 的模块级函数按
   「构造了管理器却不碰管理器的写方法」二分；四个集合各自与本文件名单**精确相等**，且并集
   覆盖全部成员——新加一只读方法而不进名单 ⇒ 当场红。
4. 逐面磁盘事实：SDK 的 `list_backups` / `verify_backup_integrity` / `clean_old_backups`（无债档）
   与注册表的读方法与读函数，跑之前跑之后**目录树名单逐字相同**，且返回值就是那只空判决。
5. 读两遍仍然没东西：幂等一档，防止「第一遍建、第二遍读」这种只测一遍量不到的形状。
6. **陷阱自证**：同一套快照在写面必须**变**——目录出现且里面是 `<name>.json` + `index.json`
   / `datasets.json`。少了这一支，「磁盘没变化」可以靠「写也写不下去」恒真。
7. 延迟建目录不等于不写盘：写完换新实例读得回那一行、校验和也照样匹配 ⇒ 副作用只是延后。
8. HTTP 面：四只读端点 + 默认档的列表 + cleanup 的无债档，白名单根下**没有新目录**。
9. CLI 面：五只读档（backup list / verify、dependency list / graph / validate）退出码 0、
   打印那句空判决、临时工作目录里**一个文件都没有**。
10. 判参先于构造（L44）原本靠「坏参数会留下目录」当证人，证人本轮被摘掉 ⇒ 口径改由 AST 比
    行号钉住，另配一支行为面：坏参数既报判决也不留目录。

**边界**：本守卫不重测 L243 那族的行为矩阵（校验和闸、幽灵 id、鉴权分档），也不重测 L231 的
路径穿越门；它只量「只读的一趟留不留东西」这一格。写面**留什么**由各写侧守卫钉。
"""
import ast
import io
import json
import pathlib
import sys
from contextlib import redirect_stderr, redirect_stdout

import pytest

from augmentor.backup import (
    DatasetBackup,
    clean_old_backups,
    create_backup,
    list_backups,
    verify_backup_integrity,
)
from augmentor.dependency import DependencyManager, get_dependency_graph, register_dataset
from augmentor.exceptions import DataValidationError

from cli import main

ROOT = pathlib.Path(__file__).resolve().parents[2]

#: 构造与读边不许出现的调用名（写调用的全集，AST 层判定）
WRITE_CALLEES = {"mkdir", "atomic_write_json", "open", "write_text", "write_bytes", "touch"}

#: 实推出来的名单（`TestTheRosterIsDerivedNotCopied` 逐格核对，改名单必须同时改产品码）
BACKUP_MANAGER_READ_METHODS = {"get_backup_info", "list_backups"}
BACKUP_MANAGER_WRITE_METHODS = {"backup", "delete_backup", "restore"}
REGISTRY_READ_METHODS = {"get_dataset", "get_dependencies", "get_dependency_graph",
                         "list_datasets", "validate_dependencies"}
REGISTRY_WRITE_METHODS = {"add_dependency", "register_dataset", "remove_dependency"}
SDK_BACKUP_READ_FUNCTIONS = {"list_backups", "verify_backup_integrity"}
SDK_BACKUP_WRITE_FUNCTIONS = {"clean_old_backups", "create_backup", "delete_backup",
                              "restore_backup"}

#: 沙箱里除了语料什么都不该有——所有「没留东西」断言的共同右端
CLEAN_ROOT_LISTING = ["p.json"]


#: 判「这只方法会不会写盘」的标记：落索引、落文件、删文件都算写（`restore` 就是那只
#: 只落输出文件而不动索引的方法——只按「有没有调 _save_index」分会把它误判成读方法）
WRITING_MARKERS = {"_save_index", "_save_datasets", "_save_dependencies",
                   "atomic_write_json", "unlink", "mkdir"}


def _class_method_nodes(source, class_name):
    tree = ast.parse(source)
    cls = next(n for n in tree.body
               if isinstance(n, ast.ClassDef) and n.name == class_name)
    return {n.name: n for n in cls.body if isinstance(n, ast.FunctionDef)}


def _callee_names(node):
    """函数体里全部调用的末段名字（`self.x.mkdir` → mkdir，`Path(...)` → Path）"""
    out = []
    for call in ast.walk(node):
        if isinstance(call, ast.Call):
            fn = call.func
            out.append(fn.attr if isinstance(fn, ast.Attribute)
                       else getattr(fn, "id", type(fn).__name__))
    return out


def _module_function(source, name):
    tree = ast.parse(source)
    return next(n for n in tree.body
                if isinstance(n, ast.FunctionDef) and n.name == name)


def _module_functions(source):
    tree = ast.parse(source)
    return {n.name: n for n in tree.body if isinstance(n, ast.FunctionDef)}


def _listing(root):
    """root 之下的相对路径全名单，目录带 `/` 后缀——磁盘事实的快照"""
    return sorted(p.relative_to(root).as_posix() + ("/" if p.is_dir() else "")
                  for p in root.rglob("*"))


def _sandbox(tmp_path, monkeypatch):
    """临时数据根 + 一份 1 条的源文件，回 (根, 源文件)。被测目录一律是**不存在**那一档"""
    root = tmp_path / "ds"
    root.mkdir()
    src = root / "p.json"
    src.write_text(json.dumps([{"instruction": "甲", "output": "乙"}], ensure_ascii=False),
                   encoding="utf-8")
    monkeypatch.chdir(tmp_path)
    return root, src


def _assert_leaves_nothing(root, target, call, expect):
    """跑一趟只读：磁盘名单逐字未变**且**返回值就是那只空判决（两半都必须在场）"""
    assert not target.exists(), f"夹具自己就把 {target.name} 建出来了，本用例是空跑"
    before = _listing(root)
    got = call()
    after = _listing(root)
    assert after == before, f"只读的一趟留下了东西：{sorted(set(after) - set(before))}"
    assert got == expect, f"空判决的形状变了：{got!r}"


class TestConstructorsHaveNoWriteCalls:
    """结构那一半：构造与读边在 AST 里一个写调用都不许有"""

    @staticmethod
    def _open_modes(source_path, class_name, method_name):
        node = _class_method_nodes(source_path.read_text(encoding="utf-8"),
                                   class_name)[method_name]
        opens = [c for c in ast.walk(node) if isinstance(c, ast.Call)
                 and getattr(c.func, "id", "") == "open"]
        modes = []
        for call in opens:
            positional = call.args[1] if len(call.args) > 1 else None
            keyword = next((k.value for k in call.keywords if k.arg == "mode"), None)
            node_mode = positional if positional is not None else keyword
            modes.append(node_mode.value if node_mode is not None else None)
        return opens, modes

    def test_the_two_constructors_touch_no_disk(self):
        backup_init = _class_method_nodes(
            (ROOT / "augmentor" / "backup.py").read_text(encoding="utf-8"),
            "DatasetBackup")["__init__"]
        registry_init = _class_method_nodes(
            (ROOT / "augmentor" / "dependency.py").read_text(encoding="utf-8"),
            "DependencyManager")["__init__"]

        for node, label in ((backup_init, "DatasetBackup.__init__"),
                            (registry_init, "DependencyManager.__init__")):
            made = set(_callee_names(node)) & WRITE_CALLEES
            assert made == set(), f"{label} 里出现了写调用：{made}"

    def test_the_read_edges_still_open_for_reading_only(self):
        """读边的 `open` 只许存在、只许是 `'r'`：「读到没有就建一份」不算读"""
        pkg = ROOT / "augmentor"
        opens, modes = self._open_modes(pkg / "backup.py", "DatasetBackup", "_load_index")
        assert len(opens) == 1 and modes == ["r"], f"索引读边：{modes}"
        opens, modes = self._open_modes(pkg / "dependency.py", "DependencyManager", "_load_file")
        assert len(opens) == 1 and modes == ["r"], f"注册表读边：{modes}"

    def test_the_loader_returns_the_empty_verdict_without_writing(self, tmp_path):
        """行为那一半：索引不在场时读出空账，而磁盘照旧空——构造不炸也不留"""
        target = tmp_path / "__l249_absent__"
        manager = DatasetBackup(str(target))
        assert manager._index == {"backups": []}
        assert not target.exists()


class TestTheRosterIsDerivedNotCopied:
    """只读名单从「体内是否调用写方法」实推，与本文件名单精确相等

    抄一份清单当真相只会跟着代码腐坏：新增一只公开读方法而名单没跟上要红；把某只读方法
    改成写方法（那它就不该再被「只读面不建目录」保护）同样要红。
    """

    @staticmethod
    def _split(class_name, module_path, writers):
        methods = _class_method_nodes(module_path.read_text(encoding="utf-8"), class_name)
        public = {name for name in methods if not name.startswith("_")}
        made_writing = {name for name in public
                        if set(_callee_names(methods[name])) & writers}
        return public - made_writing, made_writing, public

    def test_dataset_backup_splits_into_the_measured_two_sets(self):
        read, write, public = self._split(
            "DatasetBackup", ROOT / "augmentor" / "backup.py", WRITING_MARKERS)

        assert read == BACKUP_MANAGER_READ_METHODS, f"读侧名单不等：{sorted(read)}"
        assert write == BACKUP_MANAGER_WRITE_METHODS, f"写侧名单不等：{sorted(write)}"
        assert public == read | write, "有公开方法既不属于读也不属于写 ⇒ 二分没盖住它"

    def test_dependency_manager_splits_into_the_measured_two_sets(self):
        read, write, public = self._split(
            "DependencyManager", ROOT / "augmentor" / "dependency.py", WRITING_MARKERS)

        assert read == REGISTRY_READ_METHODS, f"注册表读侧名单不等：{sorted(read)}"
        assert write == REGISTRY_WRITE_METHODS, f"注册表写侧名单不等：{sorted(write)}"
        assert public == read | write

    def test_the_module_level_backup_functions_split_both_ways(self):
        """`backup.py` 的模块级函数：构造了管理器却不碰写方法的，恰好是那两只读函数"""
        source = (ROOT / "augmentor" / "backup.py").read_text(encoding="utf-8")
        read, write = set(), set()
        for name, node in _module_functions(source).items():
            names = set(_callee_names(node))
            if "DatasetBackup" not in names:
                continue
            (write if names & BACKUP_MANAGER_WRITE_METHODS else read).add(name)

        assert read == SDK_BACKUP_READ_FUNCTIONS, f"SDK 只读函数名单不等：{sorted(read)}"
        assert write == SDK_BACKUP_WRITE_FUNCTIONS, f"SDK 写函数名单不等：{sorted(write)}"


class TestSdkReadOnlyFacesLeaveNothing:
    """逐面磁盘事实：SDK 上每一只只读函数与只读方法，跑之前跑之后目录树一致"""

    def test_list_backups_on_a_missing_dir(self, tmp_path, monkeypatch):
        root, _src = _sandbox(tmp_path, monkeypatch)
        _assert_leaves_nothing(root, root / "nb",
                               lambda: list_backups(str(root / "nb")), [])

    def test_verify_backup_integrity_on_a_missing_dir(self, tmp_path, monkeypatch):
        root, _src = _sandbox(tmp_path, monkeypatch)
        _assert_leaves_nothing(root, root / "nb",
                               lambda: verify_backup_integrity(str(root / "nb")), {})

    def test_clean_old_backups_with_no_debt_is_not_an_excuse_to_create(self, tmp_path, monkeypatch):
        """无债可清的「写」端点同样不许留垃圾：这一趟它什么都没删"""
        root, _src = _sandbox(tmp_path, monkeypatch)
        _assert_leaves_nothing(root, root / "cb",
                               lambda: clean_old_backups(str(root / "cb"), 5), 0)

    def test_the_registry_module_level_read_function(self, tmp_path, monkeypatch):
        root, _src = _sandbox(tmp_path, monkeypatch)
        _assert_leaves_nothing(root, root / "nr",
                               lambda: get_dependency_graph(registry_path=str(root / "nr")),
                               {"nodes": [], "edges": []})

    def test_the_five_registry_read_methods_on_a_missing_dir(self, tmp_path, monkeypatch):
        root, _src = _sandbox(tmp_path, monkeypatch)
        target = root / "nr"
        assert _listing(root) == CLEAN_ROOT_LISTING

        manager = DependencyManager(str(target))
        assert _listing(root) == CLEAN_ROOT_LISTING, "光是构造就把注册表目录建出来了"

        assert manager.list_datasets() == []
        assert manager.get_dependency_graph() == {"nodes": [], "edges": []}
        assert manager.validate_dependencies() == []
        assert manager.get_dataset("ghost") is None
        assert manager.get_dependencies("ghost") == []
        assert not target.exists(), "五只读方法跑完把注册表目录建出来了 ⇒ A307 又犯"
        assert _listing(root) == CLEAN_ROOT_LISTING

    def test_reading_twice_still_leaves_nothing(self, tmp_path, monkeypatch):
        """幂等一档：「第一遍建、第二遍读」的形状只有跑两遍才量得到"""
        root, _src = _sandbox(tmp_path, monkeypatch)
        for _ in range(3):
            assert list_backups(str(root / "nb")) == []
            assert verify_backup_integrity(str(root / "nb")) == {}
            assert get_dependency_graph(registry_path=str(root / "nr")) == {
                "nodes": [], "edges": []}
        assert _listing(root) == CLEAN_ROOT_LISTING, f"三遍之后留下了：{_listing(root)}"


class TestTheWitnessFiresOnWrites:
    """陷阱自证：同一套快照在写面必须变，否则上面那些「没变化」是恒真"""

    def test_create_backup_still_builds_the_dir_lazily(self, tmp_path, monkeypatch):
        root, src = _sandbox(tmp_path, monkeypatch)
        target = root / "wb"

        info = create_backup(str(src), str(target), name="w1")

        assert info.backup_id == "w1"
        assert _listing(root) == ["p.json", "wb/", "wb/index.json", "wb/w1.json"], (
            "写面的落点变了：延迟建目录被做成了不建")

    def test_register_dataset_still_builds_the_registry_dir(self, tmp_path, monkeypatch):
        root, src = _sandbox(tmp_path, monkeypatch)
        target = root / "wr"

        info = register_dataset("d1", str(src), 1, registry_path=str(target))

        assert info.name == "d1"
        assert _listing(root) == ["p.json", "wr/", "wr/datasets.json"]

    def test_a_written_index_is_readable_by_a_fresh_instance(self, tmp_path, monkeypatch):
        """延迟建目录不等于不写：换新实例读得回那一行，校验和也照样匹配"""
        root, src = _sandbox(tmp_path, monkeypatch)
        target = root / "rt"
        create_backup(str(src), str(target), name="r1")
        assert (target / "index.json").exists()

        assert [b["backup_id"] for b in DatasetBackup(str(target)).list_backups()] == ["r1"]
        assert verify_backup_integrity(str(target))["r1"]["checksum_match"] is True


class TestHttpReadOnlyFacesLeaveNothing:
    """HTTP 面：只读端点与「无债可清的 cleanup」都不许在白名单根下建目录"""

    @pytest.fixture
    def client_with_root(self, tmp_path, monkeypatch):
        root, _src = _sandbox(tmp_path, monkeypatch)
        monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
        monkeypatch.setenv("AUGMENTOR_API_KEY", "")
        from fastapi.testclient import TestClient

        from api.main import app

        return TestClient(app), root

    def test_the_four_read_endpoints_leave_no_dir(self, client_with_root):
        client, root = client_with_root
        before = _listing(root)
        calls = (
            ("/api/system/backups?backup_dir=nb", {"backups": []}),
            ("/api/system/backups/verify?backup_dir=nb", {"results": []}),
            ("/api/system/dependency/datasets?registry_path=nr", {"datasets": []}),
            ("/api/system/dependency/graph?registry_path=nr",
             {"nodes": [], "edges": [], "issues": []}),
        )

        for path, expect in calls:
            r = client.get(path)
            assert r.status_code == 200, f"{path} ⇒ {r.status_code} {r.text}"
            assert r.json() == expect, f"{path} ⇒ {r.json()}"

        assert _listing(root) == before, f"四只只读端点留下：{sorted(set(_listing(root)) - set(before))}"

    def test_cleanup_with_nothing_to_do_creates_no_dir(self, client_with_root):
        client, root = client_with_root
        before = _listing(root)

        r = client.post("/api/system/backups/cleanup", json={"max_backups": 5},
                        params={"backup_dir": "cd"})

        assert r.status_code == 200, r.text
        assert r.json() == {"deleted_count": 0, "max_backups": 5}
        assert _listing(root) == before, "无债可清的清理建出了目录"

    def test_the_default_backup_dir_is_not_created_by_listing(self, client_with_root):
        """省略 `backup_dir` 那一档走 `default_backup_dir()`：出厂根下也不凭空出目录"""
        client, root = client_with_root

        r = client.get("/api/system/backups")

        assert r.status_code == 200 and r.json() == {"backups": []}
        assert not (root / ".backups").exists(), "默认档的列表把 data/.backups 建出来了"

    def test_the_default_registry_dir_is_not_created_by_listing(self, client_with_root):
        client, root = client_with_root

        for path in ("/api/system/dependency/datasets", "/api/system/dependency/graph"):
            assert client.get(path).status_code == 200
        assert not (root / ".dependency_registry").exists()


class TestCliReadOnlyFacesLeaveNothing:
    """CLI 面：五只读档跑完，数据根里一个额外文件都不许有"""

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

    def test_the_five_read_actions_leave_no_dir(self, tmp_path, monkeypatch):
        root, _src = _sandbox(tmp_path, monkeypatch)
        faces = (
            (["cli", "backup", "--action", "list", "--backup-dir", str(root / "nb")],
             "没有找到备份"),
            (["cli", "backup", "--action", "verify", "--backup-dir", str(root / "nb2")],
             "没有找到备份"),
            (["cli", "dependency", "--action", "list", "--registry-path", str(root / "nr")],
             "没有注册的数据集"),
            (["cli", "dependency", "--action", "graph", "--registry-path", str(root / "nr2")],
             "依赖图"),
            (["cli", "dependency", "--action", "validate", "--registry-path", str(root / "nr3")],
             "依赖关系验证通过"),
        )

        for argv, expect in faces:
            out, err, code = self._run_cli(argv)
            assert code is None, f"{argv[1:3]} 退出码 {code}，stderr={err}"
            assert expect in out, f"{argv[1:3]} 没打印那句空判决：{out!r}"

        assert _listing(root) == CLEAN_ROOT_LISTING, f"CLI 读档留下了：{_listing(root)}"


class TestValidationStillPrecedesConstruction:
    """证人是「坏参数会留下目录」，而证人本轮被摘掉 ⇒ 口径改由 AST 的先后钉"""

    def test_require_count_is_called_before_the_manager_is_constructed(self):
        node = _module_function(
            (ROOT / "augmentor" / "backup.py").read_text(encoding="utf-8"),
            "clean_old_backups")
        calls = {}
        for name in ("require_count", "DatasetBackup"):
            lines = [c.lineno for c in ast.walk(node) if isinstance(c, ast.Call)
                     and (getattr(c.func, "id", "") == name
                          or getattr(c.func, "attr", "") == name)]
            assert lines, f"{name} 在 clean_old_backups 里不存在，判据失去对象"
            calls[name] = min(lines)

        assert calls["require_count"] < calls["DatasetBackup"], (
            f"判参 ({calls['require_count']}) 没有排在构造 ({calls['DatasetBackup']}) 之前")

    def test_a_bad_retention_still_raises_without_touching_disk(self, tmp_path, monkeypatch):
        """行为那一半仍然要响：坏参数既报判决也不留目录"""
        root, _src = _sandbox(tmp_path, monkeypatch)
        target = root / "bad"

        with pytest.raises(DataValidationError, match="max_backups"):
            clean_old_backups(str(target), 0)

        assert not target.exists()
        assert _listing(root) == CLEAN_ROOT_LISTING
