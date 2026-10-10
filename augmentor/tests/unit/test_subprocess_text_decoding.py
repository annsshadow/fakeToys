# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L195：子进程文本解码与 locale 解耦 —— 同族守门

**缺陷（实测复现）**：`tests/unit/test_partial_branches_l123.py` 的两处
`subprocess.run(..., text=True)` 没给 `encoding=`。父进程于是按
`locale.getpreferredencoding()` 解子进程输出 —— 中文 Windows 是 **cp936**。

子进程那侧的编码却取决于环境里有没有 `PYTHONIOENCODING`（本仓文档
`OPTIMIZATION_LOOP.md` 的基线明确要求测试命令带 `PYTHONIOENCODING=utf-8`）：

| 子进程写 | 父进程解 | 结果 |
| --- | --- | --- |
| UTF-8（文档约定的测试命令） | cp936 | 读线程 `UnicodeDecodeError` ⇒ `proc.stderr` 是 **None** |
| GBK（裸 locale） | cp936 | 碰巧对 |
| UTF-8（Linux CI） | UTF-8 | 碰巧对 |

⇒ 同一条用例在「项目自己文档写的那条测试命令」下必红，在 CI 上必绿。
探针实测字节：`b'... - WARNING - api.main - \\xe6\\x9c\\xaa\\xe8\\xae\\xbe...'`
（UTF-8 的「未设置」），`cp936` 解到第 49 字节的 `0xaa` 即
`illegal multibyte sequence`。同族已修先例：`test_config_unread_feedback.py`、
`test_ledger_hash_slots_l98.py`、`test_excel_write_native_l85.py` 三处早就写了
`encoding="utf-8", errors="replace"`，本轮把最后一处补齐并建守门。

**为什么 `errors="replace"` 单独不够**：它把解码失败从「抛异常」变成「塞
`\\ufffd`」，于是 `assert "未设置" not in stderr` 会**假绿**。父进程必须连编码
一起钉住，见 `TestApiMainImportsWithKeySet::test_decoded_warning_is_the_real_sentence_not_mojibake`。

**守门规则**（AST 推导，不抄清单 —— 承 L56「判据型脚本一律走 AST」的纪律）：

- **A**：`text=True`（含 legacy 别名 `universal_newlines=True`）却未传 `encoding=`
  ⇒ 直接红，**没有豁免**。这一条就是要堵的那个形状。
- **B**：字节模式的调用必须登记在 `BYTE_MODE_ALLOWLIST` 里并写明理由。新写一个
  不登记的字节模式子进程 ⇒ 红，逼人显式签字「我不解码文本，我用命名编解码器」。
- **C**（防空转）：每条登记必须仍指向真实存在、仍是字节模式、仍无 `encoding=`
  的调用点。改了实现忘了清登记 ⇒ 红。
- **D**：每个登记文件必须自己钉住子进程编码（源码里出现 `PYTHONIOENCODING`），
  否则「父进程不解码」不等于「输出确定」。
- **E**（防空转）：扫描到的调用点总数必须 > 0 且 ≥ 一个下限，防止 glob 静默扫空。
"""

import ast
from pathlib import Path

import pytest

TESTS_DIR = Path(__file__).resolve().parent.parent

#: 允许「不传 encoding」的调用点，键是相对 tests/ 的路径，值是 {行号: 理由}
#:
#: 只收**字节模式**调用：父进程不碰文本解码，且都用一个**具名**编解码器显式解
#: （utf-8 / gbk），同时子进程侧由 `PYTHONIOENCODING` 钉死（规则 D 验这一条）。
BYTE_MODE_ALLOWLIST = {
    "integration/test_api_logging_wiring.py": {
        76: "字节模式：只做 ASCII 哨兵断言（b\"HANDLERS=\" / SENTINEL.encode()），"
            "不解码文本；子进程 env 里钉了 PYTHONIOENCODING=utf-8",
    },
    "integration/test_cli_logging_wiring.py": {
        78: "字节模式：后面用 err.decode(\"utf-8\") 具名解码；子进程 env 里钉了 "
            "PYTHONIOENCODING=utf-8",
    },
    "integration/test_cli_stdio_encoding.py": {
        49: "字节模式且**故意的**：本文件测的就是 GBK 管道，用 "
            "proc.stdout.decode(\"gbk\", \"replace\") 具名解码；子进程 env 里钉了 "
            "PYTHONIOENCODING=gbk",
        89: "字节模式：同上，.decode(\"gbk\", \"replace\") 具名解码，"
            "子进程 env 里钉了 PYTHONIOENCODING=gbk",
    },
    "integration/test_cli_verdict_wiring.py": {
        102: "字节模式：proc.stdout.decode(\"gbk\", \"replace\") 具名解码；"
             "子进程 env 里钉了 PYTHONIOENCODING=gbk",
    },
    "unit/test_repo_residue_l94.py": {
        227: "字节模式：(run.stdout + run.stderr).decode(\"utf-8\", \"replace\") "
             "具名解码",
    },
}

#: subprocess 家族里会捕获输出的入口
SUBPROCESS_FUNCS = frozenset({"run", "Popen", "check_output", "check_call"})

#: 扫描下限：防 glob 静默扫空（ Rule E ）
MIN_SITES = 13

_SKIP_DIRS = frozenset({"__pycache__", ".backups", ".pytest_cache"})

#: 本文件自己（判据的 docstring 里必然写出 `subprocess.run(…)` 这个形状）
_SELF = "unit/test_subprocess_text_decoding.py"


def _iter_test_files():
    for path in sorted(TESTS_DIR.rglob("*.py")):
        if _SKIP_DIRS & set(path.parts):
            continue
        yield path


def _is_subprocess_call(node: ast.Call) -> bool:
    func = node.func
    if not isinstance(func, ast.Attribute) or func.attr not in SUBPROCESS_FUNCS:
        return False
    base = func.value
    # `subprocess.run(...)` 与 `from subprocess import run; run(...)` 都算
    if isinstance(base, ast.Name):
        return base.id in ("subprocess", "sp")
    return isinstance(base, ast.Attribute) and base.attr == "subprocess"


def _keyword(call: ast.Call, name: str):
    for kw in call.keywords:
        if kw.arg == name:
            return kw
    return None


def _is_true_const(node) -> bool:
    return isinstance(node, ast.Constant) and node.value is True


def _discover():
    """扫出全部子进程调用点

    Returns:
        [(相对 tests/ 的路径, 行号, 是否文本模式, 是否传了 encoding)]
    """
    sites = []
    for path in _iter_test_files():
        rel = path.relative_to(TESTS_DIR).as_posix()
        tree = ast.parse(path.read_text(encoding="utf-8"))
        for node in ast.walk(tree):
            if not isinstance(node, ast.Call) or not _is_subprocess_call(node):
                continue
            text_kw = _keyword(node, "text")
            universal_kw = _keyword(node, "universal_newlines")
            is_text = (text_kw is not None and _is_true_const(text_kw.value)) or (
                universal_kw is not None and _is_true_const(universal_kw.value)
            )
            has_encoding = _keyword(node, "encoding") is not None
            sites.append((rel, node.lineno, is_text, has_encoding))
    return sites


@pytest.fixture(scope="module")
def sites():
    return _discover()


class TestScanIsAlive:
    """规则 E：扫空了就意味着守门失效"""

    def test_scan_finds_call_sites(self, sites):
        assert sites, "tests/ 下一个 subprocess 调用都没扫到 ⇒ glob 或 AST 判据坏了"
        assert len(sites) >= MIN_SITES, (
            f"只扫到 {len(sites)} 个调用点（下限 {MIN_SITES}）—— 不是文件被删了，"
            "就是判据漏认了一种写法"
        )

    def test_discovery_matches_a_direct_grep_of_the_same_pattern(self, sites):
        """AST 判据与一次独立的文本 grep 互相印证（不同源，避免同源自证）

        排除本文件：判据自己的 docstring 与注释里必须写出 `subprocess.run(…)` 这个
        形状，grep 必然命中它 —— 那是自我引用，不是漏认。
        """
        found = set()
        for path in _iter_test_files():
            rel = path.relative_to(TESTS_DIR).as_posix()
            if rel == _SELF:
                continue
            for lineno, line in enumerate(
                path.read_text(encoding="utf-8").splitlines(), start=1
            ):
                if "subprocess." in line and any(
                    f"subprocess.{fn}" in line for fn in sorted(SUBPROCESS_FUNCS)
                ):
                    found.add((rel, lineno))
        scanned = {(rel, line) for rel, line, _, _ in sites}
        missing = found - scanned
        assert not missing, (
            f"grep 看见而 AST 漏掉的调用点: {sorted(missing)} ⇒ AST 判据漏了一种写法"
        )
        extra = scanned - found
        assert not extra, (
            f"AST 看见而 grep 漏掉的调用点: {sorted(extra)} ⇒ AST 判据多认了一种写法"
        )


class TestTextModeAlwaysPinsEncoding:
    """规则 A：文本模式的子进程必须显式给编码，无豁免"""

    def test_no_text_mode_call_left_unpinned(self, sites):
        offenders = [
            (rel, line) for rel, line, is_text, has_encoding in sites
            if is_text and not has_encoding
        ]
        assert not offenders, (
            "这些调用点用 text=True 却没传 encoding=，父进程会按 "
            "locale.getpreferredencoding() 解码（中文 Windows = cp936）：\n"
            + "\n".join(f"  {rel}:{line}" for rel, line in offenders)
            + "\n修法：两侧同时钉住 —— env 里 PYTHONIOENCODING=utf-8，"
            "调用点 encoding=\"utf-8\", errors=\"replace\""
        )


class TestByteModeSitesAreSignedOff:
    """规则 B/C：字节模式调用必须登记，且登记不许过期"""

    def test_every_byte_mode_site_is_registered(self, sites):
        unregistered = {
            (rel, line) for rel, line, is_text, has_encoding in sites
            if not has_encoding and (rel, line) not in _registered()
        }
        assert not unregistered, (
            "新的未登记子进程调用点（不传 encoding）：\n"
            + "\n".join(f"  {rel}:{line}" for rel, line in sorted(unregistered))
            + "\n要么按规则 A 补 encoding=，要么在 BYTE_MODE_ALLOWLIST 登记并写明"
            "「父进程不解码文本 + 用哪个具名编解码器 + 子进程编码怎么钉」"
        )

    def test_registration_still_points_at_a_byte_mode_call(self, sites):
        """规则 C：改了实现忘了清登记 ⇒ 红"""
        indexed = {(rel, line) for rel, line, _, _ in sites}
        stale = [
            f"{rel}:{line}" for rel, entries in BYTE_MODE_ALLOWLIST.items()
            for line in entries
            if (rel, line) not in indexed
        ]
        assert not stale, (
            f"登记表指向的调用点已不存在/已挪位: {stale} ⇒ 同步 BYTE_MODE_ALLOWLIST"
        )
        pinned = {(rel, line) for rel, line, _, has_encoding in sites if has_encoding}
        now_pinned = [
            f"{rel}:{line}" for rel, entries in BYTE_MODE_ALLOWLIST.items()
            for line in entries if (rel, line) in pinned
        ]
        assert not now_pinned, (
            f"这些点已经传了 encoding=，登记可以删了: {now_pinned}"
        )

    def test_every_registration_has_a_reason(self):
        for rel, entries in BYTE_MODE_ALLOWLIST.items():
            for line, reason in entries.items():
                assert reason and reason.strip(), f"{rel}:{line} 的登记理由为空"

    def test_registered_files_pin_the_child_encoding(self):
        """规则 D：父进程不解码 ≠ 输出确定，子进程侧必须自己钉死"""
        for rel in BYTE_MODE_ALLOWLIST:
            source = (TESTS_DIR / rel).read_text(encoding="utf-8")
            assert "PYTHONIOENCODING" in source, (
                f"{rel} 的字节模式子进程没有在任何地方钉 PYTHONIOENCODING ⇒ "
                "子进程按 locale 编码，输出形态随机器而变"
            )


def _registered():
    return {
        (rel, line) for rel, entries in BYTE_MODE_ALLOWLIST.items() for line in entries
    }
