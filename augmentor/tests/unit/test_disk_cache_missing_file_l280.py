"""L280：`DiskCache` 的「判过存在再动手」那一族，与单条超上限时的自逐出。

三面，逐面点名：

① AST 名册（`TestTheMissingFileRoster`）：把 `augmentor/cache.py` 里 `DiskCache` 的
   `unlink` 调用是否带 `missing_ok`、`.exists()` 还剩哪几处、`.stat()` 是否落在
   `except FileNotFoundError` 的 `try` 之下、`set` 有没有把 `protect` 交下去、`get` 的
   `open` 落在哪个 handler 之下，全按「恰好 N 处且逐处点名」判，不写「不许出现 X」这种
   范围不定的断言。

② 行为对账（`TestMissingFilesDoNotBreakTheCache` / `TestAnOversizedEntryNoLongerWipesTheCache`）：
   真临时目录里把「文件已经不在了」这一格走通——`delete` 一个不存在的键、`clear` 撞上
   glob 之后消失的文件、`_enforce_size_limit` 撞上 stat 之前消失的文件、`stats` 同样撞上
   stat 之前消失的文件（这一格是尺子自己点出来的第四处，不是照着清单补的）、构造期撞上
   消失的索引，以及超大条目那条新口径（自己留着、别人也留着）。

③ 反证与尺子自证（`TestTheRulerHasTeeth`）：同一枚替身喂「改前的裸写形状」必须当场
   `FileNotFoundError`（否则替身是恒真的，上面那些「不抛」就什么都没证明）；同一段尺子
   喂 HEAD 形状的合成语料必须逐格点名；`_enforce_size_limit()` 不带 `protect` 那一臂必须
   重现「整本缓存连同刚写那条一起逐光」，用来证明 `protect` 是牙齿不是装饰。

取证口径沿用 L279：**概率锤不算证据**。这一族的窗口只有几条字节码宽，多线程去撞一万次
也可能一格不出；能作证据的是把路径对象换成「先按真实结果返回，再在调用方动手的那一步
之前把文件删掉」的替身。替身必须**先返回真值再改状态**，否则它否证的是自己而不是被测
代码——这条坑在 L280 的第一版探针上真踩过两次：一次把 drop 写在 `exists()` 判断之前
（两臂假绿），一次每调用新造一只替身、`fired` 被重置（同样假绿）。
"""

from __future__ import annotations

import ast
import json
import logging
from pathlib import Path

import pytest

from augmentor.cache import DiskCache

#: 被测源码。用 `__file__` 反推项目根，不依赖 pytest 的 cwd。
CACHE_PY = Path(__file__).resolve().parents[2] / "augmentor" / "cache.py"

#: 现量（L280）：`DiskCache` 里 `.unlink(...)` 的调用点数——三处全带 `missing_ok`
MEASURED_UNLINK_SITES = 3
#: 现量（L280）：`DiskCache` 里 `.exists()` 的调用点——收口后只剩 `get` 那一处判空
MEASURED_EXISTS_SITES = 1


# --------------------------------------------------------------------------- 替身

class VanishingPath:
    """把「调用方以为文件还在、动手时已经不在了」钉成必现

    `drop_on` 指定在哪一步之前把真文件删掉：

    - `"exists"`：`exists()` 先取**真实**结果再删，仍把那个真值返回给调用方（旧形状的窗口）。
    - `"stat"`：`stat()` 先删再问 ⇒ 真实 `FileNotFoundError`，不替被测代码吞异常。
    - `"unlink"`：`unlink()` 先删再按调用方给的 `missing_ok` 原样转发 ⇒ 这一格正是
      「改前抛穿、改后不抛」的分水岭。

    替身自己不吞异常，也不替被测代码补 `missing_ok`。
    """

    def __init__(self, real: Path, drop_on: str = "exists"):
        self._real = real
        self.drop_on = drop_on
        self.fired = False

    @property
    def name(self) -> str:
        return self._real.name

    @property
    def stem(self) -> str:
        return self._real.stem

    def __fspath__(self) -> str:
        return str(self._real)

    def _drop(self) -> None:
        if not self.fired:
            self.fired = True
            self._real.unlink(missing_ok=True)

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


class VanishingDir:
    """只把 `glob` 吐出的路径包成替身，其余转给真目录

    `clear()` 不判 `exists()`，它是 glob 之后直接动手，所以这一族只能从 glob 那侧挂替身。
    每只文件配一枚**持久**替身（挂在 `wrappers` 上），否则 `fired` 每次重置、反证臂变恒真。
    """

    def __init__(self, real: Path, drop_on: str = "unlink"):
        self._real = real
        self.drop_on = drop_on
        self.wrappers: dict[str, VanishingPath] = {}

    def wrap(self, path: Path) -> VanishingPath:
        return self.wrappers.setdefault(str(path), VanishingPath(path, self.drop_on))

    def glob(self, pattern: str):
        return [self.wrap(p) for p in self._real.glob(pattern)]


def _hook_key_paths(cache: DiskCache, drop_on: str) -> dict[str, VanishingPath]:
    """把 `cache._get_cache_path` 换成按 key 缓存的替身表（同 key 同替身）"""
    original = DiskCache._get_cache_path.__get__(cache)
    table: dict[str, VanishingPath] = {}

    def hooked(key: str) -> VanishingPath:
        if key not in table:
            table[key] = VanishingPath(original(key), drop_on)
        return table[key]

    cache._get_cache_path = hooked  # type: ignore[method-assign]
    return table


# --------------------------------------------------------------------------- 尺子

def _class_node(tree: ast.AST, name: str) -> ast.ClassDef:
    for node in ast.walk(tree):
        if isinstance(node, ast.ClassDef) and node.name == name:
            return node
    raise AssertionError(f"这份源码里没有 class {name}")


def _attr_calls(node: ast.AST, attr: str) -> list[ast.Call]:
    return [
        call
        for call in ast.walk(node)
        if isinstance(call, ast.Call)
        and isinstance(call.func, ast.Attribute)
        and call.func.attr == attr
    ]


def _name_calls(node: ast.AST, name: str) -> list[ast.Call]:
    return [
        call
        for call in ast.walk(node)
        if isinstance(call, ast.Call) and isinstance(call.func, ast.Name)
        and call.func.id == name
    ]


def _declares_missing_ok(call: ast.Call) -> bool:
    """`missing_ok` 要**取值为 True** 才算声明：`missing_ok=False` 就是改前的行为"""
    return any(
        k.arg == "missing_ok" and isinstance(k.value, ast.Constant) and k.value.value is True
        for k in call.keywords
    )


def _guarded_lines(func: ast.FunctionDef, exception_names: set[str]) -> set[int]:
    """落在「handler 捕获指定异常」的 try 体里的行号（按该 try 体的行跨度取）"""
    lines: set[int] = set()
    for try_node in (n for n in ast.walk(func) if isinstance(n, ast.Try)):
        caught: set[str] = set()
        for handler in try_node.handlers:
            if handler.type is None:
                caught.add("BaseException")
            elif isinstance(handler.type, ast.Name):
                caught.add(handler.type.id)
            elif isinstance(handler.type, ast.Tuple):
                caught.update(elt.id for elt in handler.type.elts if isinstance(elt, ast.Name))
        if not (caught & exception_names):
            continue
        body_lines = [s.lineno for s in try_node.body if hasattr(s, "lineno")]
        if body_lines:
            lines.update(range(min(body_lines), max(body_lines) + 1))
    return lines


def measure(source: str) -> dict:
    """对一份 cache.py 源码文本量 `DiskCache` 的缺文件语义面（守卫的尺子本体）"""
    cls = _class_node(ast.parse(source), "DiskCache")
    methods = {n.name: n for n in cls.body if isinstance(n, ast.FunctionDef)}

    unlinks = _attr_calls(cls, "unlink")
    bare_unlinks = sorted(c.lineno for c in unlinks if not _declares_missing_ok(c))
    exists_sites = sorted(
        f"{name}:{c.lineno}"
        for name, func in methods.items()
        for c in _attr_calls(func, "exists")
    )
    stat_unprotected = sorted(
        f"{name}:{c.lineno}"
        for name, func in methods.items()
        for c in _attr_calls(func, "stat")
        if c.lineno not in _guarded_lines(func, {"FileNotFoundError"})
    )

    enforce = methods.get("_enforce_size_limit")
    enforce_has_protect = bool(
        enforce is not None
        and "protect" in [a.arg for a in enforce.args.args]
        and len(enforce.args.defaults) == 1
        and isinstance(enforce.args.defaults[0], ast.Constant)
        and enforce.args.defaults[0].value is None
    )
    set_call = next(
        iter(_attr_calls(methods["set"], "_enforce_size_limit")), None
    ) if "set" in methods else None
    set_passes_protect = bool(
        set_call is not None
        and any(
            k.arg == "protect" and isinstance(k.value, ast.Name) and k.value.id == "key"
            for k in set_call.keywords
        )
    )

    get_opens = _name_calls(methods["get"], "open") if "get" in methods else []
    get_guard = _guarded_lines(methods["get"], {"IOError", "OSError"}) if "get" in methods else set()
    get_open_guarded = bool(get_opens) and all(c.lineno in get_guard for c in get_opens)

    return {
        "unlink_sites": len(unlinks),
        "bare_unlink_sites": bare_unlinks,
        "exists_sites": exists_sites,
        "stat_unprotected": stat_unprotected,
        "enforce_has_protect": enforce_has_protect,
        "set_passes_protect": set_passes_protect,
        "get_open_guarded": get_open_guarded,
        "load_metadata_exists": any(s.startswith("_load_metadata:") for s in exists_sites),
    }


def production() -> dict:
    return measure(CACHE_PY.read_text(encoding="utf-8"))


# ------------------------------------------------------------------ ① AST 名册

class TestTheMissingFileRoster:
    """名册按「恰好几处、且逐处点名」判，不收成「不许出现 X」这种范围不定的断言"""

    def test_the_source_file_is_the_one_under_test(self):
        assert CACHE_PY.is_file(), CACHE_PY
        assert "class DiskCache" in CACHE_PY.read_text(encoding="utf-8")

    def test_every_unlink_in_diskcache_declares_missing_ok(self):
        m = production()
        assert m["unlink_sites"] == MEASURED_UNLINK_SITES, m
        assert m["bare_unlink_sites"] == [], m

    def test_the_exists_calls_are_only_the_named_get_one(self):
        m = production()
        assert len(m["exists_sites"]) == MEASURED_EXISTS_SITES, m["exists_sites"]
        assert all(s.startswith("get:") for s in m["exists_sites"]), m["exists_sites"]

    def test_metadata_delete_clear_and_enforce_check_nothing_first(self):
        m = production()
        for method in ("_load_metadata", "delete", "clear", "_enforce_size_limit"):
            assert not any(s.startswith(f"{method}:") for s in m["exists_sites"]), (
                f"{method} 又写回「判过存在再动手」了：{m['exists_sites']}"
            )
        assert m["load_metadata_exists"] is False, m

    def test_every_stat_sits_under_a_file_not_found_handler(self):
        m = production()
        assert m["stat_unprotected"] == [], m

    def test_set_hands_the_just_written_key_to_the_eviction_pass(self):
        m = production()
        assert m["enforce_has_protect"] is True, m
        assert m["set_passes_protect"] is True, m

    def test_gets_read_stays_under_the_io_error_handler(self):
        """`get` 那处 `.exists()` 是**被证明**的豁免，不是又一句「最坏只是少统计一次」"""
        m = production()
        assert m["get_open_guarded"] is True, m


# ---------------------------------------------------------- ② 缺文件时的行为对账

class TestMissingFilesDoNotBreakTheCache:
    """「文件已经不在了」这一格必须走通，而不是把异常抛穿到调用方"""

    def test_delete_of_a_key_whose_file_was_never_written(self, tmp_path):
        cache = DiskCache(tmp_path)
        assert cache.delete("never_written") is True
        assert cache.delete("never_written") is True

    def test_delete_survives_the_file_vanishing_at_the_unlink_step(self, tmp_path):
        cache = DiskCache(tmp_path)
        cache.set("k", {"a": 1})
        table = _hook_key_paths(cache, "unlink")
        assert cache.delete("k") is True, "文件在动手那一刻消失，delete 不该把它变成异常"
        assert table["k"].fired is True, "替身没开火 ⇒ 这一臂什么都没测到"
        assert "k" not in cache._metadata

    def test_clear_survives_files_that_vanish_at_the_unlink_step(self, tmp_path):
        cache = DiskCache(tmp_path)
        cache.set("a", 1)
        cache.set("b", 2)
        real_dir = cache._cache_dir
        hook = VanishingDir(real_dir)
        cache._cache_dir = hook  # type: ignore[assignment]
        cache.clear()
        assert len(hook.wrappers) == 3, hook.wrappers  # 两只owned + 索引本身也在 glob 结果里
        index_wrapper = next(w for w in hook.wrappers.values()
                             if w.name == "_metadata.json")
        owned_wrappers = [w for w in hook.wrappers.values() if w is not index_wrapper]
        assert len(owned_wrappers) == 2, hook.wrappers
        assert all(w.fired for w in owned_wrappers), hook.wrappers
        assert index_wrapper.fired is False, "clear 按 owned 名单跳过索引，这一格也要被看着"
        assert cache._metadata == {}
        cache._cache_dir = real_dir
        left = [p.name for p in real_dir.glob("*.json") if p.name != "_metadata.json"]
        assert left == [], left

    def test_enforce_size_survives_the_file_vanishing_before_stat(self, tmp_path):
        cache = DiskCache(tmp_path, max_bytes=10 ** 6)
        cache.set("a", "x" * 50)
        cache.set("b", "y" * 50)
        table = _hook_key_paths(cache, "stat")
        cache._enforce_size_limit()
        assert table["a"].fired is True and table["b"].fired is True, table
        assert cache._metadata == {}, "消失的文件按孤儿元数据清掉，而不是抛穿调用方"

    def test_a_missing_index_reads_as_an_empty_ledger(self, tmp_path):
        """`_load_metadata` 改后不再判存在：索引在不在都得给出答案，不许抛穿构造

        断言里那一格 `probe.fired is False` 是有牙齿的：新形状如果偷偷写回了 `exists()`，
        替身就会开火，这一臂当场红。
        """
        index = tmp_path / "_metadata.json"
        ledger = {"k": {"created_at": 1.0, "ttl": None}}
        index.write_text(json.dumps(ledger), encoding="utf-8")
        probe = VanishingPath(index, "exists")
        fresh = DiskCache.__new__(DiskCache)  # 只借这一格，不跑 __init__
        fresh._metadata_file = probe
        assert fresh._load_metadata() == ledger, "前置条件：替身指向的是真档案"
        index.unlink()
        assert fresh._load_metadata() == {}, "消失的索引该读成空账，而不是 FileNotFoundError"
        assert probe.fired is False, "新形状里没有 exists()，替身不该开火"

    def test_stats_survives_a_file_vanishing_before_the_stat(self, tmp_path):
        """诊断口径同一族：`stats` 原先是 `sum(f.stat() for f in files)`，一次外部删除就炸读数"""
        cache = DiskCache(tmp_path, max_bytes=10 ** 6)
        cache.set("a", 1)
        real_dir = cache._cache_dir
        hook = VanishingDir(real_dir, "stat")
        cache._cache_dir = hook  # type: ignore[assignment]
        stats = cache.stats
        assert stats["entries"] == 1, stats
        assert stats["total_size_bytes"] == 0, "消失的那只按已经不在目录里计，不该炸掉整份读数"
        owned = [w for w in hook.wrappers.values() if w.name != "_metadata.json"]
        assert owned and all(w.fired for w in owned), hook.wrappers
        cache._cache_dir = real_dir

    def test_orphan_metadata_is_still_cleaned_up(self, tmp_path):
        """既有口径不动：文件被外部删掉 ⇒ 元数据跟着清，且这一格不抛"""
        cache = DiskCache(tmp_path, max_bytes=10 ** 6)
        cache.set("dead", "v")
        cache.set("live", "w")
        cache._get_cache_path("dead").unlink()
        cache._enforce_size_limit()
        assert "dead" not in cache._metadata
        assert "live" in cache._metadata


# ------------------------------------------------------ ②b 自逐出那格（新口径）

class TestAnOversizedEntryNoLongerWipesTheCache:
    """单条超过 `max_bytes` 的写入：改前毁掉整本缓存，且自己也没存下"""

    def test_the_oversized_write_keeps_every_other_entry_and_itself(self, tmp_path):
        cache = DiskCache(tmp_path, max_bytes=400)
        for i in range(3):
            cache.set(f"small_{i}", "x" * 20)
        payload = "y" * 3000
        cache.set("big", payload)

        assert cache.get("big") == payload, "刚写的那一条不该把自己也逐掉"
        for i in range(3):
            assert f"small_{i}" in cache._metadata, (
                f"为了放下一条超大的就把别人的缓存清空：small_{i} 被牵连"
            )
        assert "big" in cache._metadata

    def test_it_says_so_on_the_log(self, tmp_path, caplog):
        cache = DiskCache(tmp_path, max_bytes=400)
        with caplog.at_level(logging.WARNING, logger="augmentor.cache"):
            cache.set("big", "z" * 3000)
        assert any("已超过上限" in r.getMessage() for r in caplog.records), [
            r.getMessage() for r in caplog.records
        ]

    def test_a_normal_write_still_evicts_the_oldest_and_only_the_oldest(self, tmp_path):
        cache = DiskCache(tmp_path, max_bytes=600)
        for i in range(20):
            cache.set(f"key{i}", "x" * 100)
        assert cache.stats["total_size_bytes"] <= 600, "普通写入仍须守住上限"
        assert cache.get("key19") is not None
        assert cache.get("key0") is None


# ------------------------------------------------------- ③ 反证与尺子自证

class TestTheRulerHasTeeth:
    """每条断言都要能红：同一替身喂旧形状必抛、尺子喂旧语料必逐格点名"""

    def test_the_head_shaped_delete_raises_under_the_same_substitute(self, tmp_path):
        """改前的 `if exists(): unlink()`：替身开火后当场 `FileNotFoundError`

        这一臂证明替身本身有效——它若不响，上面那些「不抛」的守卫就是恒真。
        """
        cache = DiskCache(tmp_path)
        cache.set("k", {"a": 1})
        real = cache._get_cache_path("k")

        def head_shaped_delete(path: VanishingPath) -> None:
            if path.exists():      # 真值 True，随后文件已不在
                path.unlink()       # 改前不带 missing_ok ⇒ 这里炸

        with pytest.raises(FileNotFoundError):
            head_shaped_delete(VanishingPath(real, "exists"))

    def test_the_head_shaped_stat_pass_raises_under_the_same_substitute(self, tmp_path):
        cache = DiskCache(tmp_path, max_bytes=10 ** 6)
        cache.set("k", "x" * 50)
        real = cache._get_cache_path("k")

        def head_shaped_size(path: VanishingPath) -> int:
            if not path.exists():
                return -1
            return path.stat().st_size

        with pytest.raises(FileNotFoundError):
            head_shaped_size(VanishingPath(real, "exists"))

    def test_the_head_shaped_stats_pass_raises_under_the_same_substitute(self, tmp_path):
        cache = DiskCache(tmp_path)
        cache.set("k", "x" * 40)
        files = [
            VanishingPath(p, "stat")
            for p in Path(tmp_path).glob("*.json")
            if p.name != "_metadata.json"
        ]
        assert len(files) == 1, files

        def head_shaped_total(paths: list[VanishingPath]) -> int:
            return sum(f.stat().st_size for f in paths)

        with pytest.raises(FileNotFoundError):
            head_shaped_total(files)

    def test_the_head_shaped_load_metadata_raises_under_the_same_substitute(self, tmp_path):
        index = tmp_path / "_metadata.json"
        index.write_text("{}", encoding="utf-8")
        probe = VanishingPath(index, "exists")

        def head_shaped_load(path: VanishingPath):
            if path.exists():
                with open(path, "r", encoding="utf-8") as handle:
                    return json.load(handle)
            return {}

        with pytest.raises(FileNotFoundError):
            head_shaped_load(probe)

    def test_dropping_protect_reproduces_the_wipe(self, tmp_path):
        """不带 `protect` 调同一只函数 ⇒ 整本缓存连同刚写那条一起清零（改前的真实形状）"""
        cache = DiskCache(tmp_path, max_bytes=400)
        for i in range(3):
            cache.set(f"small_{i}", "x" * 20)
        cache.set("big", "y" * 3000)
        assert cache.get("big") == "y" * 3000, "前置条件：带 protect 时大条目留着"

        cache._enforce_size_limit()  # 与改前逐字同形：没有要保护的名单成员
        assert cache._metadata == {}, (
            "`protect` 不是装饰：摘掉它就该重现「三条小条目 + 刚写的大条目同时消失」"
        )
        assert cache.get("big") is None

    def test_the_ruler_flags_the_head_shaped_corpus(self):
        """同一段尺子喂 HEAD 形状的合成语料 ⇒ 逐格点名，证明它量的不是恒真"""
        head_shaped = '''
class DiskCache:
    def _load_metadata(self):
        if self._metadata_file.exists():
            with open(self._metadata_file) as handle:
                return json.load(handle)
        return {}

    def get(self, key):
        path = self._get_cache_path(key)
        if not path.exists():
            return None
        try:
            with open(path) as handle:
                return json.load(handle)
        except (json.JSONDecodeError, IOError):
            return None

    def set(self, key, value):
        with open(self._get_cache_path(key), "w") as handle:
            json.dump(value, handle)
        self._enforce_size_limit()

    def _enforce_size_limit(self):
        for key in list(self._metadata):
            path = self._get_cache_path(key)
            if not path.exists():
                del self._metadata[key]
                continue
            size = path.stat().st_size
        for _, key, path, size in []:
            path.unlink(missing_ok=True)
            del self._metadata[key]

    def delete(self, key):
        path = self._get_cache_path(key)
        if path.exists():
            path.unlink()
        return True

    def clear(self):
        for file in self._cache_dir.glob("*.json"):
            file.unlink()
'''
        m = measure(head_shaped)
        assert len(m["bare_unlink_sites"]) == 2, m["bare_unlink_sites"]
        assert len(m["exists_sites"]) == 4, m["exists_sites"]
        assert {s.split(":")[0] for s in m["stat_unprotected"]} == {"_enforce_size_limit"}, m
        assert m["enforce_has_protect"] is False, m
        assert m["set_passes_protect"] is False, m
        assert m["get_open_guarded"] is True, "get 那一处是真豁免，两版都该判它绿"
        assert m["load_metadata_exists"] is True, m

    def test_swapping_the_fix_out_turns_the_roster_red(self):
        """把修法摘掉（`missing_ok=True` → `missing_ok=False`）必须被名册点名"""
        aliased = CACHE_PY.read_text(encoding="utf-8").replace(
            "cache_path.unlink(missing_ok=True)", "cache_path.unlink(missing_ok=False)"
        )
        assert aliased != CACHE_PY.read_text(encoding="utf-8")
        m = measure(aliased)
        assert len(m["bare_unlink_sites"]) == 1, m["bare_unlink_sites"]
