# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L289（A394）：全仓 `.py` 必须**过得了 compile** —— 模块 docstring 里的孤立代理会让整支守卫不可收集

**缺陷形**：`tests/unit/test_validation_finite_echo_l284.py` 的模块 docstring 里写着
`"\\ud800"`（想给读者展示那枚孤立代理的字面量），而它落在**非 raw** 的 docstring 上 ⇒ Python
把它解成真字符，随后 pytest 断言重写那一步 `compile(tree, ...)` 抛
`UnicodeEncodeError: 'utf-8' codec can't encode character '\\ud800' in position 678`。
后果不是「一条用例红」而是**整支守卫连收集都过不去**：全量定标当场 `Interrupted: 1 error
during collection`。

**为什么此前那份全量读数看不见它**：`__pycache__` 里同一只文件同时留着 312 与 313 两种标签的
重写缓存（现量 366 / 344 只），3.12 那条路上这个 compile 侥幸过得去，3.13 过不去 ⇒ 换解释器
（或干净克隆）才显形。本轮定标就是在 3.13.14 上撞出来的。

**族范围是量出来的**：仓外探针 `Temp/l289q/probe_collectable.py` 把整仓 541 只 `.py`（walk 全树，
只跳 `Temp` / `__pycache__` / `.venv` 那类噪声目录）逐只 `ast.parse` + `compile`，命中**恰好一格**
（就是上面那只），其余 540 只全过 ⇒ 不是「到处都有」，也不是「只有它一只文件写得歪」；判据落在
「模块级 compile 通不通」这一条机器可验的形状上。常驻尺自己的面现量 537 只（只数 `augmentor/`、
`api/`、`tests/` 三棵子树加根级 `cli.py`），与探针的差额逐只点名是五只：`scripts/` 四只
（`inject_defect_check.py` / `mutation/run.py` / `mutation/sample.py` / `triage_tests.py`）与
`archive/enhanceTXT.py` 一只 —— 这五只**也在探针面上且同样全过**，所以「零失败」那条主张在两处
口径下都成立，本尺不收它们只是把语料根钉在产品面与测试面这两族承重码上。

**修法**：那只文件的模块 docstring 加 `r` 前缀（一个字都不必改文档内容，字面量原样显示）。
本文件的注入臂就是把这一枚 `r` 摘掉，重现「尺子点名它」。

**常驻尺子四面**：一、语料面自证（逐根验真 + 产品/测试两侧都在场，防「扫了个空目录」的假绿）；
二、正判据（全仓零失败，失败名单逐只点名）；三、牙齿（内存里摘掉那枚 `r` ⇒ 恰一处红且点名到那只文件）；
四、修面钉（那只文件现量确实是 raw docstring、模块 docstring 里零代理）。

零落盘：三面全部读磁盘文本后在内存里改写，仓库文件一字不动。

跑法：`python -m pytest tests/unit/test_module_collectable_l289.py -q --no-cov`
"""

import ast
from pathlib import Path
from typing import List, Tuple

ROOT = Path(__file__).resolve().parents[2]

#: 语料根（**逐根验真**，形状对不上就响，不许静默 `continue`——承 L282 那条「把 cli 当目录列进
#: 语料根，于是全仓少一只」的教训）。前端面 `web/` 是 ts/tsx，不在本尺的扩展名里 ⇒ 不列。
PRODUCT_DIRS = ("augmentor", "api")
PRODUCT_FILES = ("cli.py",)
TEST_DIRS = ("tests",)

SKIP_DIRS = frozenset({".git", "Temp", "__pycache__", ".venv", "venv",
                       "node_modules", "htmlcov", ".mypy_cache", "data"})

#: A394 那一格的靶子：本轮唯一命中者
L284_PY = "tests/unit/test_validation_finite_echo_l284.py"


def _roots() -> List[Path]:
    """语料根现量验真：目录必须是目录、文件必须是文件（不许靠 `continue` 吞掉形状不符的根）"""
    out: List[Path] = []
    for name in PRODUCT_DIRS + TEST_DIRS:
        base = ROOT / name
        assert base.is_dir(), f"语料根 {base} 不是目录"
        out.append(base)
    for name in PRODUCT_FILES:
        base = ROOT / name
        assert base.is_file(), f"语料根 {base} 不是文件（cli 在仓库根的真形是 cli.py）"
        out.append(base)
    return out


def corpus() -> Tuple[Tuple[str, str], ...]:
    """`((相对路径, 磁盘文本), ...)` —— 语料面上每一只 `.py`"""
    rows: List[Tuple[str, str]] = []
    for base in _roots():
        walk = [base] if base.is_file() else sorted(base.rglob("*.py"))
        for path in walk:
            parts = set(path.relative_to(ROOT).parts)
            if parts & SKIP_DIRS:
                continue
            rows.append((path.relative_to(ROOT).as_posix(),
                         path.read_text(encoding="utf-8")))
    return tuple(rows)


def uncollectable(items: Tuple[Tuple[str, str], ...]) -> List[Tuple[str, str]]:
    """逐只 `ast.parse` + `compile`，返回 `[(相对路径, 判决原文), ...]`（过不了 compile 的那批）"""
    bad: List[Tuple[str, str]] = []
    for rel, text in items:
        try:
            tree = ast.parse(text)
        except SyntaxError as exc:  # 语法错与编码错同一档：都进不了收集
            bad.append((rel, f"SyntaxError: {exc}"))
            continue
        try:
            compile(tree, rel, "exec", dont_inherit=True)
        except UnicodeEncodeError as exc:
            bad.append((rel, f"{type(exc).__name__}: {exc}"))
    return bad


# ============ 一、语料面自证 ============


def test_the_corpus_is_big_enough_to_be_the_claim():
    """「零失败」只有建立在一份够大的语料上才算主张：两侧都在场 + 总量有下界

    下界而不是精确相等：本尺的反空转判据只需「面还在、两侧都有成员」，规模主张由
    `test_export_census_sentinel_l177.py` 那只语料规模尺钉（口径不同 —— 那把把 web 之外的
    产品根全数进去，本尺只数 `augmentor/` + `api/` + `tests/` + 根级 `cli.py` 四根的 `.py`）。
    下界出自现量：L289 落盘时 409 / 127 / 1。
    """
    items = corpus()
    assert len(items) >= 500, len(items)
    tests = [rel for rel, _ in items if rel.startswith("tests/")]
    products = [rel for rel, _ in items if rel.startswith(("augmentor/", "api/"))]
    assert len(tests) >= 400, len(tests)
    assert len(products) >= 125, len(products)
    assert "cli.py" in dict(items), "根级 cli.py 不在面 ⇒ 语料根那一格被静默吞掉了"
    assert L284_PY in dict(items), "靶子文件不在语料面上 ⇒ 后面的判据全是空的"


# ============ 二、正判据 ============


def test_every_module_on_the_corpus_compiles():
    """全仓 `.py` 逐只过 compile：失败名单必须为空，且逐只点名（不是一句「跑不通」）"""
    bad = uncollectable(corpus())
    assert bad == [], bad


# ============ 三、牙齿（内存摘掉那枚 r） ============


def test_injection_removing_the_raw_prefix_lands_exactly_one_hit():
    """把 A394 的修法（docstring 的 `r` 前缀）在内存里摘掉 ⇒ 尺子必须恰点名那一格

    这一臂同时证明两件事：判据不是恒真（有东西能让它红），以及红的**归因**落在那只文件自己身上
    （不是「语料面变了所以数不对」）。仓库文件一字未改：改动只发生在读进内存的那份文本上。
    """
    items = dict(corpus())
    text = items[L284_PY]
    anchor = 'r"""L284'
    assert text.count(anchor) == 1, text.count(anchor)
    weakened = text.replace(anchor, '"""L284', 1)
    assert weakened != text
    bad = uncollectable(tuple((rel, weakened if rel == L284_PY else txt)
                              for rel, txt in items.items()))
    assert [rel for rel, _ in bad] == [L284_PY], bad
    assert "UnicodeEncodeError" in bad[0][1] and "\\ud800" in items[L284_PY], bad


# ============ 四、修面钉 ============


def test_the_fixed_module_docstring_carries_no_lone_surrogate():
    """A394 的修法本身被钉住：那只文件的模块 docstring 是 raw，且现量零个代理码点"""
    text = (ROOT / L284_PY).read_text(encoding="utf-8")
    doc = ast.get_docstring(ast.parse(text), clean=False) or ""
    assert doc, "读不到模块 docstring ⇒ 这一臂没牙"
    assert not [c for c in doc if 0xD800 <= ord(c) <= 0xDFFF], len(doc)
    assert "\\ud800" in doc, "docstring 里那句字面量被改没了 ⇒ 读者看不见这一形"
    # raw 前缀在磁盘上（不在内存里）：这条是「修面已落盘」的正控制
    assert text.splitlines()[3].startswith('r"""'), text.splitlines()[3][:40]
