"""L281：「判过存在再动手」那一族在 `atomic_write / backup / checkpoint / data.audio / data.image` 的十格。

L280 在 `augmentor/cache.py` 里收了这一族的第一批，并把口径立成三面（AST 名册 + 确定性消失
路径替身 + 反证）。本轮把同一把尺子推到**全仓产品语料**上：只读普查量到 133 只产品文件里还剩
10 格 / 5 只文件，逐格改完之后普查读数为 0，这一族从此由常驻守卫承重。

**L282 修的正是上面那句「133 只产品文件」**：那一次普查把 `cli` 当**目录**列进语料根，而它在磁盘
上的真形是仓库根的 `cli.py` ⇒ 旧 `product_sources()` 里 `if not base.is_dir(): continue` 把它
静默跳掉，面无声少一只、尺子照样全绿。修面后现量 **134**，`cli.py` 在这把尺上读数是 0 命中，
所以那十格的结论未推翻，被推翻的只有「133 就是全仓」这句话。补的两只尺
（`test_the_census_face_covers_the_cli_entry_module` / `test_the_corpus_is_the_same_face_as_l177`）
与 fail-loud 那一臂钉的是「语料面不许静默缩水」：**只钉规模常数挡不住这一族** —— 两把尺各数各的
时，本尺 133、l177 那只 134，两边都是自己那份常数上的绿，所以这里按**集合相等**跨尺对账。

同一轮把 A378 那一族（`finally` 里的裸清理步顶替真正的失败）也从全仓量了一遍：新尺
`finally_cleanup_hits` 吃源码吐读数，①面上按**集合相等**钉在 `MEASURED_FINALLY_EXEMPT` 那唯一一格，
四档自证臂（裸调用点名 / 自己罩 try 不杀 / `missing_ok` 不杀 / 换动词仍认）见
`test_the_finally_census_names_a_bare_cleanup`。那一格量到了却**未坐实成缺陷**，实测读数与不动产品码
的理由写在常数上方。

三面，逐面点名：

① 全仓普查尺（`TestTheWholeCorpusIsClean`）：把「同一只路径先过 `exists/is_file/is_dir`，再在
   任何能兜住缺文件的 handler 之外对它动手（`unlink/stat/rename/open/…`）」这一形状钉成语料级
   零命中。`unlink(missing_ok=True)` 算安全，它本就是「缺失即达成」的正解，不再需要预检。

② 逐格名册（`TestTheFixedSitesKeepTheirShape`）：名册管的是①看不见的那一半 —— 把预检摘掉之后
   「裸 `unlink()`」在语料尺上是**隐形**的（没有配对预检），所以十格按函数逐格判：`unlink` 必须
   带 `missing_ok=True`、`stat` 必须落在捕获 `FileNotFoundError` 的 `try` 之下、`finally` 体恰好
   是一只尽力清理的调用。尺子全部做成「吃源码文本、吐读数」的函数，好让③能拿合成语料自证。

③ 行为对账与反证（`TestWhatTheRaceActuallyReads` / `TestCheckpointFacesKeepTheirBehavior` /
   `TestTheRulerHasTeeth`）：确定性替身沿用 L280 那条口径——**先按真实结果返回值，再在调用方
   动手的那一步之前把文件删掉**，同一路径复用同一只替身（每调用新造会把 `fired` 重置、反证臂
   随之恒真，L280 实测踩过）。另附两支「改前同形复现」：把本轮摘掉的两句原样写一遍，喂同一枚
   替身必须当场抛穿 / 掩蔽原异常，否则上面所有「不抛」都是恒真。

本轮最大的一格不在 SDK 面而在**承诺面**：`DELETE /api/system/backups/{backup_id}` 把「索引有条目
而备份文件已丢不算 404，仍回 200」写进了对外 responses，而 `delete_backup()` 里那句先 `exists()`
再裸 `unlink()` 在并发窗口里抛的 `FileNotFoundError` 会跳过紧随其后的索引重排、并被路由的
`to_http_error` 折成非 200 —— 代码与它自己写定的契约相反。这一格由 HTTP 臂钉住。
"""

from __future__ import annotations

import ast
import json
import logging
from pathlib import Path

import pytest

import augmentor.atomic_write as atomic_write_mod
import augmentor.backup as backup_mod
import augmentor.data.audio as audio_mod
import augmentor.data.image as image_mod
from augmentor.atomic_write import atomic_write_json
from augmentor.backup import DatasetBackup
from augmentor.checkpoint import CheckpointManager
from augmentor.data.audio import AudioProcessor
from augmentor.data.image import ImageProcessor

ROOT = Path(__file__).resolve().parents[2]

#: 普查语料面：两棵产品子树 + CLI 入口模块，与 l177 那把尺同一面（`test_the_corpus_is_the_same_face_as_l177` 逐格对账）。
#: **每一根都要验真，不许静默跳过**：L281 原本把 `cli` 当目录列进来，而它在仓库根的真形是 `cli.py`，
#: `is_dir()` 为假 ⇒ 那一格 `continue` 把它悄悄跳掉，于是「全仓 133 只」少量一只、而缺的那只恰好不在
#: 任何子树里。这与 l177 docstring 警告的是同一族：扫描面缩水会把「量不到的 0」重新变成假绿。
PRODUCT_DIRS = ("augmentor", "api")
PRODUCT_FILES = ("cli.py",)

#: 现量（L282 修面）：语料规模 133 → **134**（补回仓库根的 `cli.py`）。
#: 红读数逐字 `assert 134 == 133`（由仓外探针 `Temp/probe_l282_face.py` 把常数拨回 133 在新语料面上
#: 实跑取出，非推断），且 `cli.py` 进面后普查读数仍是 **0**（它不含任何动手类调用），
#: ⇒ L281 那十格的结论没有被这次修面推翻，被推翻的只有「133 就是全仓」这句话。
MEASURED_PRODUCT_FILES = 134

#: 现量（L281）：`atomic_write.py` 里 `.exists()` 预检的剩余调用点（两格随裸 unlink 一起摘掉）
MEASURED_ATOMIC_WRITE_PRECHECKS = 0

#: 动手类调用：判过存在之后真的会对文件系统起作用的那些
OPS = {"unlink", "remove", "stat", "rename", "replace", "rmdir", "mkdir", "touch",
       "rmtree", "copy", "copy2", "copyfile", "copytree", "move"}
OPEN_OP = "open"

#: 预检类调用：与上面配成「先判再动」这一形状的那一侧
TESTS = {"exists", "is_file", "is_dir"}

#: 兜得住「文件在窗口里消失」的 handler 名。`Exception` / `BaseException` 也算：这一族的病灶是
#: 异常抛穿到调用方，不是捕获得够不够窄。
SAFE_EXC = {"FileNotFoundError", "OSError", "IOError", "Exception", "BaseException"}


# --------------------------------------------------------------------- 尺子（吃文本，吐读数）

def _root_name(node: ast.AST) -> str | None:
    n = node
    while isinstance(n, ast.Attribute):
        n = n.value
    return n.id if isinstance(n, ast.Name) else None


def _op_of(call: ast.Call) -> str | None:
    f = call.func
    if isinstance(f, ast.Attribute) and f.attr in OPS:
        return f.attr
    if isinstance(f, ast.Name) and f.id == OPEN_OP:
        return OPEN_OP
    if isinstance(f, ast.Attribute) and f.attr == OPEN_OP and _root_name(f.value) == "io":
        return OPEN_OP
    return None


def _declares_missing_ok(call: ast.Call) -> bool:
    """`missing_ok` 要**取值为 True** 才算声明：`missing_ok=False` 就是改前的行为"""
    return any(
        k.arg == "missing_ok" and isinstance(k.value, ast.Constant) and k.value.value is True
        for k in call.keywords
    )


def _caught_names(try_node: ast.Try) -> set[str]:
    caught: set[str] = set()
    for handler in try_node.handlers:
        if handler.type is None:
            caught |= SAFE_EXC
            continue
        for nm in ast.walk(handler.type):
            if isinstance(nm, ast.Name):
                caught.add(nm.id)
            elif isinstance(nm, ast.Attribute):
                caught.add(nm.attr)
    return caught


def _covered_lines(node: ast.AST) -> set[int]:
    """落在「handler 捕获到 file-ish 异常」的 try 体 / finalbody 里的行号（按语句行跨度取）"""
    covered: set[int] = set()
    for try_node in (n for n in ast.walk(node) if isinstance(n, ast.Try)):
        if not (_caught_names(try_node) & SAFE_EXC):
            continue
        for stmt in list(try_node.body) + list(try_node.finalbody):
            covered.update(range(stmt.lineno, stmt.end_lineno + 1))
    return covered


def _attr_calls(node: ast.AST, *attrs: str) -> list[ast.Call]:
    return [
        c for c in ast.walk(node)
        if isinstance(c, ast.Call) and isinstance(c.func, ast.Attribute)
        and c.func.attr in attrs
    ]


def _funcs(tree: ast.AST) -> dict[str, ast.AST]:
    return {
        n.name: n for n in ast.walk(tree)
        if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))
    }


def _func(source: str, func_name: str) -> ast.AST:
    fn = _funcs(ast.parse(source)).get(func_name)
    assert fn is not None, f"这份源码里没有函数 {func_name} ⇒ 名册与代码对不上"
    return fn


def check_then_act_hits(source: str, relpath: str) -> list[dict]:
    """语料级尺子：返回「预检 + 未罩住的动手」这一族的命中格"""
    hits: list[dict] = []
    for fn in _funcs(ast.parse(source)).values():
        covered = _covered_lines(fn)
        tests = [(c.func.attr, c.lineno, ast.dump(c)) for c in _attr_calls(fn, *TESTS)]
        for call in ast.walk(fn):
            if not isinstance(call, ast.Call):
                continue
            op = _op_of(call)
            if op is None or call.lineno in covered:
                continue
            if op in {"unlink", "remove"} and _declares_missing_ok(call):
                continue
            recv = _root_name(call.func.value) if isinstance(call.func, ast.Attribute) else None
            paired = next(((attr, ln) for attr, ln, dump in tests
                           if recv and recv in dump), None)
            if paired:
                hits.append({"file": relpath, "func": fn.name, "op": op, "line": call.lineno,
                             "test": paired[0], "test_line": paired[1]})
    return hits


def bare_unlink_lines(source: str, func_name: str) -> list[int]:
    """名册尺：这一族里「删档不带 `missing_ok=True`」的行号"""
    return [c.lineno for c in _attr_calls(_func(source, func_name), "unlink", "remove")
            if not _declares_missing_ok(c)]


def precheck_lines(source: str, func_name: str) -> list[int]:
    """名册尺：预检回来了没有"""
    return [c.lineno for c in _attr_calls(_func(source, func_name), *TESTS)]


def unguarded_stat_lines(source: str, func_name: str) -> list[int]:
    """名册尺：`stat()` 落在缺文件 handler 之外的行号"""
    fn = _func(source, func_name)
    covered = _covered_lines(fn)
    return [c.lineno for c in _attr_calls(fn, "stat") if c.lineno not in covered]


def finally_call_names(source: str, func_name: str) -> list[str]:
    """名册尺：`finally` 体里那一句调用的是什么"""
    tries = [n for n in ast.walk(_func(source, func_name)) if isinstance(n, ast.Try)]
    assert len(tries) == 1, f"{func_name} 的 try 形状不是「一整段写 + 一段清理」"
    out: list[str] = []
    for stmt in tries[0].finalbody:
        expr = stmt.value if isinstance(stmt, ast.Expr) else None
        if isinstance(expr, ast.Call):
            out.append(expr.func.id if isinstance(expr.func, ast.Name)
                       else getattr(expr.func, "attr", "?"))
        else:
            out.append(type(stmt).__name__)
    return out


#: 「自己会抛」的清理动词名单：判的是这一族，不是全文扫 `finally`。
CLEANUP_CALLS = frozenset({"unlink", "remove", "rmtree", "close", "rename", "replace"})

#: 现量（L282 立尺）：134 只产品文件的 `finally` 体里，裸清理步**恰好一格** ——
#: `augmentor/excel_write.py::export_table_to_excel` 的 `wb.close()`。
#: 这一格**量到了，但没坐实成缺陷**：openpyxl 的 `Workbook.close()` 写成
#: `if hasattr(self, "_archive"): self._archive.close()`，仓外探针 `Temp/probe_finally_close.py`
#: 照产品骨架（`try: …wb.save()… finally: wb.close()`）实测两档 ⇒ `wb.save()` 因「目标是一枚目录」
#: 抛 `PermissionError` 时 `close()` **静默**（读数 `ARM_SAVE_FAIL_DIR: close_silent
#: top:PermissionError ctx:None`，真正的写失败没被顶替），重复 close 与 save 成功后的 close 也静默。
#: 于是按「不为不会发生的情形加兜底」这条原话**不动产品码**，把它钉成下面这份名册里唯一的一格，
#: 判据逐格写死（文件 + 函数 + 调用文本）：产品码一改，这一格就红；长出第二格，同样红。
MEASURED_FINALLY_EXEMPT = (("augmentor/excel_write.py", "export_table_to_excel", "wb.close"),)


def finally_cleanup_hits(source: str, relpath: str) -> list[dict]:
    """语料尺：`finally` 体里那些「自己会抛、又没被罩住」的裸清理调用，逐格点名

    与 A378 同一族：`finally` 里抛出的异常**顶替**上面那次真正的失败，于是调用方按「清理失败」
    判决，真正的失败凭空消失。两道豁免各有实测理由 —— `missing_ok=True` 是「缺失即达成」的正解
    （A377 口径），调用体自己罩了 `try` 的是尽力清理（`atomic_write._discard_tmp` 就长这样）；
    只调用一只尽力清理 helper 的格子落在 `CLEANUP_CALLS` 之外，天然不命中。
    """
    tree = ast.parse(source)
    parent: dict[int, ast.AST] = {}
    for node in ast.walk(tree):
        for child in ast.iter_child_nodes(node):
            parent[id(child)] = node

    def enclosing_func(node: ast.AST) -> str:
        current = parent.get(id(node))
        while current is not None:
            if isinstance(current, (ast.FunctionDef, ast.AsyncFunctionDef)):
                return current.name
            current = parent.get(id(current))
        return "<module>"

    hits: list[dict] = []
    for node in ast.walk(tree):
        if not isinstance(node, ast.Try) or not node.finalbody:
            continue
        shielded: set[int] = set()
        for stmt in node.finalbody:
            for inner in ast.walk(stmt):
                if isinstance(inner, ast.Try):
                    shielded.update(id(c) for c in ast.walk(inner))
        for stmt in node.finalbody:
            for call in (c for c in ast.walk(stmt) if isinstance(c, ast.Call)):
                if id(call) in shielded or any(k.arg == "missing_ok" for k in call.keywords):
                    continue
                text = ast.unparse(call.func)
                if text.rsplit(".", 1)[-1] not in CLEANUP_CALLS:
                    continue
                hits.append({"rel": relpath, "func": enclosing_func(node),
                             "call": text, "line": call.lineno})
    return hits


def product_sources(dirs: tuple[str, ...] = PRODUCT_DIRS,
                    files: tuple[str, ...] = PRODUCT_FILES) -> list[tuple[str, str]]:
    """按声明的语料面读源码，**任何一根不存在就报错**

    参数化是为了让③能喂合成面自证：把一根假名字传进来必须响，否则「面缩水」这一族缺陷
    在本守卫里就又是恒绿的 —— L281 那一版正是用 `if not base.is_dir(): continue` 把
    `cli.py` 静默跳掉的。
    """
    out: list[tuple[str, str]] = []
    for sub in dirs:
        base = ROOT / sub
        if not base.is_dir():
            raise AssertionError(f"语料根 {sub} 不是目录（真形可能是仓库根的单个模块文件）")
        for p in sorted(base.rglob("*.py")):
            if "__pycache__" in p.parts or ".venv" in p.parts:
                continue
            out.append((p.relative_to(ROOT).as_posix(), p.read_text(encoding="utf-8")))
    for name in files:
        one = ROOT / name
        if not one.is_file():
            raise AssertionError(f"语料文件 {name} 不在磁盘上")
        out.append((one.relative_to(ROOT).as_posix(), one.read_text(encoding="utf-8")))
    return out


def l177_corpus_face() -> set[str]:
    """另一把普查尺（l177 导出死符号普查）自己的语料面，按相对路径集合取回。

    跨尺对账必须读**它那份**定义，不能在这里抄一份同样的清单：抄了就等于两把尺各数各的，
    L281 漏掉 `cli.py` 那一格（本尺 133、l177 一直 134）在抄本上照样两边各自绿。
    """
    import importlib.util

    spec = importlib.util.spec_from_file_location(
        "_l177_face_probe",
        Path(__file__).resolve().parent / "test_export_census_sentinel_l177.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return {p.relative_to(ROOT).as_posix() for p in module._PRODUCTS}


def source_of(relpath: str) -> str:
    return (ROOT / relpath).read_text(encoding="utf-8")


# --------------------------------------------------------------------- 替身

class VanishingPath:
    """把「调用方以为文件还在、动手时已经不在了」钉成必现

    `drop_on` 指定在哪一步之前把真文件删掉。替身自己**不吞异常、也不替被测代码补
    `missing_ok`**：

    - `"exists"`：`exists()` 先取真实结果再删，仍把那个真值交回调用方（预检那一格的窗口）。
    - `"stat"`：`stat()` 先删再问 ⇒ 真实 `FileNotFoundError`。
    - `"unlink"`：`unlink()` 先删，再按调用方给的 `missing_ok` 原样转发 ⇒ 这一格正是
      「改前抛穿、改后不抛」的分水岭。
    """

    def __init__(self, real: Path, drop_on: str):
        self._real = real
        self.drop_on = drop_on
        self.fired = False

    def _drop(self) -> None:
        if not self.fired:
            self.fired = True
            self._real.unlink(missing_ok=True)

    def __str__(self) -> str:
        return str(self._real)

    def __fspath__(self) -> str:
        return str(self._real)

    @property
    def name(self) -> str:
        return self._real.name

    @property
    def stem(self) -> str:
        return self._real.stem

    @property
    def suffix(self) -> str:
        return self._real.suffix

    @property
    def parent(self) -> Path:
        return self._real.parent

    def exists(self) -> bool:
        got = self._real.exists()
        if got and self.drop_on == "exists":
            self._drop()
        return got

    def stat(self):
        if self.drop_on == "stat":
            self._drop()
        return self._real.stat()

    def unlink(self, missing_ok: bool = False) -> None:
        if self.drop_on == "unlink":
            self._drop()
        self._real.unlink(missing_ok=missing_ok)


def _table_factory(table: dict[str, VanishingPath]):
    """只把名单里的路径换成替身，其余原样交回真 `Path`

    模块级 `Path` 在这些文件里还被别处用着（索引、构造、校验和），整颗换成替身会连坐；
    按「字符串化之后是否命中」挑。
    """

    def factory(*args, **kwargs):
        real = Path(*args, **kwargs)
        return table.get(str(real), real)

    return factory


def _wrapper_for(table: dict[str, VanishingPath], real: Path, drop_on: str) -> VanishingPath:
    return table.setdefault(str(real), VanishingPath(real, drop_on))


# --------------------------------------------------------------------- ① 全仓普查

class TestTheWholeCorpusIsClean:
    def test_corpus_face_is_the_measured_size(self):
        srcs = product_sources()
        assert len(srcs) == MEASURED_PRODUCT_FILES, (
            f"语料面从 {MEASURED_PRODUCT_FILES} 变成 {len(srcs)}：先归因（新增/删除产品文件？"
            "venv 落进语料？）再按红读数重钉，不许顺手放宽"
        )

    def test_no_check_then_act_sites_remain(self):
        hits: list[dict] = []
        for relpath, src in product_sources():
            hits.extend(check_then_act_hits(src, relpath))
        assert hits == [], "「判过存在再动手」又长出格子：" + json.dumps(
            hits, ensure_ascii=False, indent=1
        )

    def test_the_census_face_actually_covers_the_five_fixed_files(self):
        """语料尺不是空转：这五只文件必须真的在扫描面上"""
        seen = {rel for rel, _ in product_sources()}
        for rel in ("augmentor/atomic_write.py", "augmentor/backup.py",
                    "augmentor/checkpoint.py", "augmentor/data/audio.py",
                    "augmentor/data/image.py"):
            assert rel in seen, f"{rel} 不在语料面上 ⇒ 上面那条零命中是假的"

    def test_the_census_face_covers_the_cli_entry_module(self):
        """仓库根的 `cli.py` 也在面上——L281 那一版恰恰漏了它

        旧实现把 `cli` 当**目录**列进语料根，而它在磁盘上的真形是单文件 `cli.py` ⇒
        `if not base.is_dir(): continue` 静默跳掉，「全仓 133 只」少了这一只。
        这一格有牙齿：把 `files` 传空就复现旧面，读数 133（`MEASURED_PRODUCT_FILES - 1`），
        尺子当场指出少的那一只。
        """
        seen = {rel for rel, _ in product_sources()}
        assert "cli.py" in seen, "CLI 入口模块不在语料面上 ⇒ 全仓普查读数不可信"
        assert len(product_sources(files=())) == MEASURED_PRODUCT_FILES - 1

    def test_the_corpus_is_the_same_face_as_l177(self):
        """两把普查尺量的是同一个面，按**集合相等**对账

        只各自钉一个规模数挡不住这一族缺陷：本尺 133、l177 那只 134 时，两把尺在自己那份
        常数上都是绿的。漏的那一只恰好不在任何子树里，所以必须逐格比集合。
        """
        mine = {rel for rel, _ in product_sources()}
        theirs = l177_corpus_face()
        assert mine == theirs, (
            "两把全仓普查尺的语料面分叉 ⇒ 其中一把在缩量。"
            f"只在本尺: {sorted(mine - theirs)} 只在那把: {sorted(theirs - mine)}"
        )

    def test_bare_finally_cleanups_are_exactly_the_named_cell(self):
        """`finally` 体里的裸清理步（A378 那一族）在全仓只许住在名册点名的那一格

        判据写成**集合相等**而不是「不许出现」：豁免名单逐格点名（文件 + 函数 + 调用文本），
        产品码把那一句改形、或别处新长出一格，都当场红。为什么这一格豁免而不是修掉，理由与
        实测读数写在 `MEASURED_FINALLY_EXEMPT` 上方。
        """
        found = {(h["rel"], h["func"], h["call"])
                 for rel, src in product_sources()
                 for h in finally_cleanup_hits(src, rel)}
        assert found == set(MEASURED_FINALLY_EXEMPT), (
            f"finally 裸清理面与名册分叉：实测 {sorted(found)}，名册 {sorted(MEASURED_FINALLY_EXEMPT)}"
        )


# --------------------------------------------------------------------- ② 逐格名册

class TestTheFixedSitesKeepTheirShape:
    def test_atomic_write_finally_is_one_best_effort_call(self):
        src = source_of("augmentor/atomic_write.py")
        for name in ("atomic_write_json", "atomic_write_text"):
            assert finally_call_names(src, name) == ["_discard_tmp"], (
                f"{name} 的 finally 不再是「一只尽力清理」的调用：裸 unlink 会把真正的写失败顶替掉"
            )
        assert bare_unlink_lines(src, "_discard_tmp") == []
        assert precheck_lines(src, "atomic_write_json") == []
        tree = ast.parse(src)
        assert len(_attr_calls(tree, *TESTS)) == MEASURED_ATOMIC_WRITE_PRECHECKS

    def test_discard_tmp_catches_oserror_and_says_so(self):
        src = source_of("augmentor/atomic_write.py")
        fn = _func(src, "_discard_tmp")
        unlinks = _attr_calls(fn, "unlink")
        assert len(unlinks) == 1, f"清理步应当只有一只 unlink，实际 {len(unlinks)}"
        tries = [n for n in ast.walk(fn) if isinstance(n, ast.Try)]
        assert len(tries) == 1, "清理步必须罩在 try 之下，否则它在 finally 里抛穿"
        caught = _caught_names(tries[0])
        assert "OSError" in caught, f"清理失败的捕获面退化成 {caught}"
        warns = [c for c in _attr_calls(fn, "warning")]
        assert warns, "吞掉清理异常又不登记 ⇒ 失败被藏起来了"

    def test_delete_backup_has_no_precheck_and_declares_missing_ok(self):
        src = source_of("augmentor/backup.py")
        assert precheck_lines(src, "delete_backup") == [], (
            "预检回来了：窗口里的 FileNotFoundError 会跳过索引重排并把 200 变成非 200"
        )
        assert bare_unlink_lines(src, "delete_backup") == []
        fn = _func(src, "delete_backup")
        assert len(_attr_calls(fn, "unlink")) == 1

    def test_verify_wraps_both_stat_and_checksum(self):
        src = source_of("augmentor/backup.py")
        fn = _func(src, "verify_backup_integrity")
        assert precheck_lines(src, "verify_backup_integrity") == []
        assert unguarded_stat_lines(src, "verify_backup_integrity") == []
        tries = [n for n in ast.walk(fn) if isinstance(n, ast.Try)]
        assert len(tries) == 1, f"巡检体里应当只有一段 try，实际 {len(tries)}"
        assert "FileNotFoundError" in _caught_names(tries[0])
        covered = _covered_lines(fn)
        checksums = _attr_calls(fn, "_calculate_checksum")
        assert len(checksums) == 1
        assert checksums[0].lineno in covered, (
            "校验和那一步同样有「判过存在再动手」的窗口，必须落在同一个 try 之内"
        )

    @pytest.mark.parametrize(
        "func_name",
        ["_remove_delta", "delete_checkpoint"],
        ids=["增量文件那一族", "断点本体那一格"],
    )
    def test_checkpoint_unlink_sites_declare_missing_ok(self, func_name):
        src = source_of("augmentor/checkpoint.py")
        assert precheck_lines(src, func_name) == [], f"{func_name} 的预检回来了"
        assert bare_unlink_lines(src, func_name) == [], f"{func_name} 有裸 unlink"

    @pytest.mark.parametrize(
        "relpath,func_name",
        [("augmentor/data/image.py", "process_image"),
         ("augmentor/data/audio.py", "process_audio")],
        ids=["图像", "音频"],
    )
    def test_processors_stat_the_size_under_file_not_found(self, relpath, func_name):
        src = source_of(relpath)
        assert unguarded_stat_lines(src, func_name) == [], (
            f"{relpath} 的取尺寸步落在 try 之外：一份在扫描队列里消失的文件会让整次 "
            "POST /api/multimodal/scan 变成 500"
        )
        fn = _func(src, func_name)
        honest = [n.value for n in ast.walk(fn)
                  if isinstance(n, ast.Constant) and n.value == "文件不存在"]
        assert len(honest) == 2, (
            f"「文件不存在」该出现两次（早退 + 窗口），实际 {len(honest)} 次：改后的诚实读数"
        )


# --------------------------------------------------------------------- ③ 行为对账

ITEMS = [{"text": "alpha"}, {"text": "beta"}]


@pytest.fixture
def dataset_root(tmp_path, monkeypatch):
    """仓外白名单根：`ds/p.json` 是数据集，`ds/bs` 是备份目录（不预先建索引）"""
    root = tmp_path / "ds"
    root.mkdir()
    (root / "p.json").write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.setenv("AUGMENTOR_CONFIG_PATH", str(tmp_path / "config.yaml"))
    monkeypatch.setenv("AUGMENTOR_API_KEY", "")
    monkeypatch.chdir(tmp_path)
    return root


def _make_backup(root: Path) -> tuple[DatasetBackup, Path]:
    bdir = root / "bs"
    bdir.mkdir(parents=True, exist_ok=True)
    manager = DatasetBackup(str(bdir))
    info = manager.backup(str(root / "p.json"), name="raceprobe")
    return manager, Path(info.backup_path)


def _index_rows(bdir: Path) -> list[str]:
    index = json.loads((bdir / "index.json").read_text(encoding="utf-8"))
    return [b["backup_id"] for b in index.get("backups", [])]


class TestWhatTheRaceActuallyReads:
    def test_delete_backup_survives_the_file_gone_at_unlink(self, dataset_root, monkeypatch):
        manager, backup_file = _make_backup(dataset_root)
        assert backup_file.is_file()
        table: dict[str, VanishingPath] = {}
        _wrapper_for(table, backup_file, "unlink")
        monkeypatch.setattr(backup_mod, "Path", _table_factory(table))

        assert manager.delete_backup("raceprobe") is True
        assert table[str(backup_file)].fired is True, "替身没开火 ⇒ 这一臂在测空气"
        assert not backup_file.exists()
        assert _index_rows(dataset_root / "bs") == [], "删了档但索引行留在盘上 ⇒ 下次删除再撞同一格"

    def test_http_delete_still_returns_200_when_the_file_vanishes(
        self, dataset_root, monkeypatch
    ):
        """承诺面：`DELETE /api/system/backups/{id}` 的 responses 明文写着「仍回 200」"""
        from fastapi.testclient import TestClient

        _, backup_file = _make_backup(dataset_root)
        bdir = dataset_root / "bs"
        client = TestClient(__import__("api.main", fromlist=["app"]).app)
        listed = client.get("/api/system/backups", params={"backup_dir": str(bdir)})
        assert listed.status_code == 200, listed.text
        assert [b["backup_id"] for b in listed.json()["backups"]] == ["raceprobe"]

        table: dict[str, VanishingPath] = {}
        _wrapper_for(table, backup_file, "unlink")
        monkeypatch.setattr(backup_mod, "Path", _table_factory(table))

        dropped = client.delete("/api/system/backups/raceprobe",
                                params={"backup_dir": str(bdir)})
        assert dropped.status_code == 200, (
            f"窗口里的 FileNotFoundError 被折成 {dropped.status_code}，与这个端点自己写定的"
            f"「仍回 200」相反：{dropped.text}"
        )
        monkeypatch.undo()
        assert not backup_file.exists()
        assert _index_rows(bdir) == []

    def test_verify_reports_missing_instead_of_raising_at_stat(self, dataset_root, monkeypatch):
        _manager, backup_file = _make_backup(dataset_root)
        table: dict[str, VanishingPath] = {}
        _wrapper_for(table, backup_file, "stat")
        monkeypatch.setattr(backup_mod, "Path", _table_factory(table))

        results = backup_mod.verify_backup_integrity(str(dataset_root / "bs"), "raceprobe")
        assert results["raceprobe"]["exists"] is False, (
            "档在 stat 那一步消失 ⇒ 诚实读数是 exists=False，不是整次巡检失败"
        )
        assert results["raceprobe"]["checksum_match"] is False

    def test_verify_reports_missing_when_the_file_gone_after_stat(self, dataset_root, monkeypatch):
        """第二个窗口：stat 之后、整份读之前被删（`_calculate_checksum` 直接 open）"""
        _manager, backup_file = _make_backup(dataset_root)
        real_checksum = DatasetBackup._calculate_checksum

        def dropping(self, file_path):
            Path(file_path).unlink(missing_ok=True)
            return real_checksum(self, file_path)

        monkeypatch.setattr(DatasetBackup, "_calculate_checksum", dropping)
        results = backup_mod.verify_backup_integrity(str(dataset_root / "bs"), "raceprobe")
        assert results["raceprobe"]["exists"] is False, (
            "校验和那一步撞上并发删除也要按缺档计，而不是把 FileNotFoundError 递给调用方"
        )
        assert not backup_file.exists()

    def test_verify_still_reads_a_present_backup_as_present(self, dataset_root):
        """兜住反向：上面那些「按缺档计」的臂都得有这一臂证明没把在册的档也读成缺"""
        _make_backup(dataset_root)
        results = backup_mod.verify_backup_integrity(str(dataset_root / "bs"), "raceprobe")
        assert results["raceprobe"]["exists"] is True
        assert results["raceprobe"]["checksum_match"] is True

    @pytest.mark.parametrize(
        "module,cls,ext,call",
        [(image_mod, ImageProcessor, "png", "process_image"),
         (audio_mod, AudioProcessor, "wav", "process_audio")],
        ids=["图像", "音频"],
    )
    def test_processors_report_the_honest_missing_reading(
        self, module, cls, ext, call, tmp_path, monkeypatch
    ):
        """扫描队列里的一只在轮到它之前被删 ⇒ 诚实的 `valid=False`，而不是打断整次扫描"""
        real = tmp_path / f"one.{ext}"
        real.write_bytes(b"\x89PNG\r\n\x1a\n" + b"\x00" * 40 if ext == "png"
                         else b"RIFF\x00\x00\x00\x00WAVE" + b"\x00" * 40)
        table: dict[str, VanishingPath] = {}
        wrapper = _wrapper_for(table, real, "exists")
        monkeypatch.setattr(module, "Path", _table_factory(table))

        got = getattr(cls(), call)(str(real))
        assert wrapper.fired is True, "替身没开火 ⇒ 这一臂什么都没测"
        assert got.valid is False
        assert got.error == "文件不存在", f"实际 error={got.error!r}"
        assert got.format == ""

    def test_cleanup_failure_does_not_mask_the_write_failure(self, tmp_path, monkeypatch, caplog):
        """`finally` 里的清理失败不许顶替真正的写失败（模块 docstring 承诺「异常原样上抛」）"""
        monkeypatch.setattr(atomic_write_mod.secrets, "token_hex", lambda n: "deadbeef")
        target = tmp_path / "out.json"
        tmp = target.parent / f".{target.name}.tmp-deadbeef"
        tmp.mkdir()  # 临时件的位置被一只目录占住 ⇒ 写失败，且清理必然抛 OSError

        with caplog.at_level(logging.WARNING, logger="augmentor.atomic_write"):
            with pytest.raises(OSError) as excinfo:
                atomic_write_json(target, {"a": 1})
        assert excinfo.value.__context__ is None, (
            "清理异常成了上下文 ⇒ 报出去的是清理失败而不是那次写失败，承诺被顶替"
        )
        assert not target.exists(), "写失败却落了目标 ⇒ 上面那读数不可信"
        assert "清理临时件失败" in caplog.text

    def test_a_normal_write_leaves_no_tmp_and_lands_the_content(self, tmp_path):
        target = tmp_path / "out.json"
        atomic_write_json(target, {"a": 1})
        assert json.loads(target.read_text(encoding="utf-8")) == {"a": 1}
        assert [p.name for p in tmp_path.iterdir()] == ["out.json"], "尽力清理不该留垃圾"


class TestCheckpointFacesKeepTheirBehavior:
    def test_remove_delta_survives_the_file_gone_at_unlink(self, tmp_path, monkeypatch):
        mgr = CheckpointManager(str(tmp_path / "ck"))
        real = mgr._get_delta_path("t1")
        real.parent.mkdir(parents=True, exist_ok=True)
        real.write_text("[]", encoding="utf-8")
        wrapper = VanishingPath(real, "unlink")
        monkeypatch.setattr(mgr, "_get_delta_path", lambda task_id: wrapper)

        mgr._remove_delta("t1")
        assert wrapper.fired is True, "替身没开火 ⇒ 这一臂什么都没测"
        assert not real.exists()

    def test_delete_checkpoint_survives_the_file_gone_at_unlink(self, tmp_path, monkeypatch):
        mgr = CheckpointManager(str(tmp_path / "ck"))
        mgr.create_checkpoint("t2", len(ITEMS))
        real = mgr._get_checkpoint_path("t2")
        assert real.is_file()
        wrapper = VanishingPath(real, "unlink")
        monkeypatch.setattr(mgr, "_get_checkpoint_path", lambda task_id: wrapper)

        mgr.delete_checkpoint("t2")
        assert wrapper.fired is True, "替身没开火 ⇒ 这一臂什么都没测"


# --------------------------------------------------------------------- 反证与尺子自证

class TestTheRulerHasTeeth:
    def test_the_surrogate_reproduces_the_pre_fix_crash(self, tmp_path):
        """同形复现：改前那两句喂同一枚替身必须抛穿，否则「不抛」全是恒真"""
        real = tmp_path / "gone.json"
        real.write_text("{}", encoding="utf-8")
        wrapper = VanishingPath(real, "unlink")

        def pre_fix(path) -> None:
            if path.exists():
                path.unlink()

        with pytest.raises(FileNotFoundError):
            pre_fix(wrapper)

    def test_the_pre_fix_finally_masks_the_original_error(self, tmp_path):
        """同形复现：改前的 finally 裸 unlink 会把写失败顶替成清理失败"""
        tmp = tmp_path / ".out.json.tmp-cafebabe"
        tmp.mkdir()

        def pre_fix() -> None:
            try:
                with open(tmp, "w", encoding="utf-8") as f:
                    json.dump({"a": 1}, f)
            finally:
                if tmp.exists():
                    tmp.unlink()

        with pytest.raises(OSError) as excinfo:
            pre_fix()
        assert excinfo.value.__context__ is not None, (
            "同形复现没重现出顶替 ⇒ 上面那支「不顶替」的臂没有对照"
        )

    @pytest.mark.parametrize(
        "relpath,needle,replacement,expect_func",
        [("augmentor/backup.py",
          "backup_path.unlink(missing_ok=True)",
          "if backup_path.exists():\n                backup_path.unlink()",
          "delete_backup"),
         ("augmentor/checkpoint.py",
          "delta_path.unlink(missing_ok=True)",
          "if delta_path.exists():\n                delta_path.unlink()",
          "_remove_delta"),
         ("augmentor/atomic_write.py",
          "        _discard_tmp(tmp)\n\n\ndef atomic_write_text",
          "        if tmp.exists():\n            tmp.unlink()\n\n\ndef atomic_write_text",
          "atomic_write_json")],
        ids=["备份删除回到预检", "增量清理回到预检", "临时件清理回到裸 unlink"],
    )
    def test_the_corpus_ruler_names_the_pre_fix_shape(
        self, relpath, needle, replacement, expect_func
    ):
        src = source_of(relpath)
        assert src.count(needle) == 1, f"锚点在这份源码里不是唯一命中：{relpath}"
        hits = check_then_act_hits(src.replace(needle, replacement), relpath)
        assert expect_func in {h["func"] for h in hits}, (
            f"普查尺对 HEAD 形状失明，读数={hits}"
        )
        assert check_then_act_hits(src, relpath) == [], "现状这一份就该是零命中"

    @pytest.mark.parametrize(
        "relpath,func_name",
        [("augmentor/backup.py", "delete_backup"),
         ("augmentor/checkpoint.py", "_remove_delta"),
         ("augmentor/atomic_write.py", "_discard_tmp")],
        ids=["备份", "增量", "临时件"],
    )
    def test_the_roster_names_a_flipped_missing_ok(self, relpath, func_name):
        """名册尺的牙齿：把 `missing_ok=True` 翻回 False，语料尺是瞎的（没有配对预检），名册必须点得出"""
        src = source_of(relpath)
        flipped = src.replace("missing_ok=True", "missing_ok=False")
        assert flipped != src, f"{relpath} 里没有 missing_ok=True ⇒ 注入落空"
        assert bare_unlink_lines(src, func_name) == []
        assert bare_unlink_lines(flipped, func_name), "翻了 missing_ok 而名册没反应 ⇒ 尺子恒真"

    @pytest.mark.parametrize(
        "relpath,func_name",
        [("augmentor/backup.py", "verify_backup_integrity"),
         ("augmentor/data/image.py", "process_image"),
         ("augmentor/data/audio.py", "process_audio")],
        ids=["完整性巡检", "图像", "音频"],
    )
    def test_the_roster_names_a_widened_except_face(self, relpath, func_name):
        src = source_of(relpath)
        assert "except FileNotFoundError:" in src, relpath
        assert unguarded_stat_lines(src, func_name) == []
        assert unguarded_stat_lines(
            src.replace("except FileNotFoundError:", "except PermissionError:"), func_name
        ), "把缺文件的 handler 换成不相关的异常 ⇒ 取尺寸步重新裸抛，名册必须点名"

    def test_the_roster_names_a_cleanup_called_bare(self):
        src = source_of("augmentor/atomic_write.py")
        assert finally_call_names(src, "atomic_write_json") == ["_discard_tmp"]
        assert finally_call_names(
            src.replace("_discard_tmp(tmp)\n\n\ndef atomic_write_text",
                        "tmp.unlink(missing_ok=True)\n\n\ndef atomic_write_text"),
            "atomic_write_json",
        ) == ["unlink"], "清理步退回裸 unlink 而名册没反应 ⇒ 这一臂恒真"

    def test_a_misdeclared_corpus_root_is_loud_not_silently_skipped(self):
        """语料根对不上磁盘形状时必须当场喊红，不许把面缩掉后继续返回读数

        L281 那一版的缺陷形状就是这里的第一档：`cli` 列成了目录、真形是仓库根的 `cli.py`，
        `is_dir()` 为假 ⇒ 旧代码 `continue` 跳掉，普查面无声少一只文件、尺子照样全绿。
        """
        with pytest.raises(AssertionError, match="语料根 cli 不是目录"):
            product_sources(dirs=("augmentor", "api", "cli"))
        with pytest.raises(AssertionError, match="语料文件 no_such_module.py 不在磁盘上"):
            product_sources(files=("no_such_module.py",))
        with pytest.raises(AssertionError, match="语料根 augmentor_old 不是目录"):
            product_sources(dirs=("augmentor_old", "api"))

    def test_the_finally_census_names_a_bare_cleanup(self):
        """finally 裸清理尺的四档自证：能点名裸调用，且不把三种正解误判成缺陷

        最后一档是防「豁免名单收成恒真空集」：真面上那一格必须仍然量得到，
        否则上面那条集合相等其实是在跟一份没人读的名单比。
        """
        bare = (
            "def make_report(out, wb):\n"
            "    try:\n"
            "        wb.save(out)\n"
            "    finally:\n"
            "        wb.close()\n"
        )
        assert [(h["func"], h["call"]) for h in finally_cleanup_hits(bare, "syn.py")] == [
            ("make_report", "wb.close")
        ], "裸的 finally 清理步没被点名 ⇒ 这一族尺子恒真"

        wrapped = bare.replace(
            "        wb.close()\n",
            "        try:\n            wb.close()\n        except OSError:\n            pass\n",
        )
        assert finally_cleanup_hits(wrapped, "syn.py") == [], "自己罩了 try 的尽力清理仍被判裸 ⇒ 误杀"

        unlinked = bare.replace("        wb.close()\n", "        out.unlink()\n")
        assert [(h["call"], h["line"]) for h in finally_cleanup_hits(unlinked, "syn.py")] == [
            ("out.unlink", 5)
        ], "同一族换个动词就不认 ⇒ 名单写死在 close 上"
        polite = unlinked.replace("out.unlink()", "out.unlink(missing_ok=True)")
        assert finally_cleanup_hits(polite, "syn.py") == [], "missing_ok 是正解，不该判裸"

        real = finally_cleanup_hits(source_of("augmentor/excel_write.py"),
                                    "augmentor/excel_write.py")
        assert [(h["func"], h["call"]) for h in real] == [
            ("export_table_to_excel", "wb.close")
        ], "豁免那一格在尺子上已经量不到了 ⇒ 名册是在跟空集对账"
