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

**L292（A404）：尺子自己也有同一形缺陷 —— 牙齿臂原只认 3.13 的读数**
上面那份修法把产品面修好了，但把判据的**牙齿**压在了「`compile()` 抛不抛」这一格上，而那一格本身
是解释器依赖的：`compile(tree, rel, "exec", dont_inherit=True)` 对同一份非 raw 语料只在 3.13.14 抛
`UnicodeEncodeError`，在 3.12.14 **侥幸过得去**（现量两版跑同一支：3.13 四支全绿、3.12 的牙齿臂逐字
`AssertionError`，失败名单为空而右侧多出的正是那只靶子）。后果不是「少一条证据」而是**换解释器时
牙齿会空转** —— 臂红得像尺子有牙，实际红在被测缺陷的反面。归因：判据的输入（那枚孤立代理）在两版
`ast.parse` 之后是**同一个东西**，版本差异只出现在 `compile` 那一步，所以判据该钉在 AST 面上。

**修正后的判据（版本无关，两通道）**：
- 通道一（承重）：`ast.parse` 之后只看**会被写成 `__doc__` 的那些常量**（模块 / 函数 / 异步函数 /
  类的第一条语句），其中出现 **孤立代理码点**（U+D800–U+DFFF，Python 的 utf-8 编码器对全部代理一律
  拒绝）就判不可收集，判决原文点名 `行/列/U+码点`。这一通道在两版上读数逐字相同。
- 边界由仓外探针量出，不是猜的：同一个代理放 docstring 位 ⇒ 3.13 抛、3.12 不抛；放**普通常量**位
  （`VALUE = "\\ud800"` 或列表元素）⇒ **两版都不抛**。所以判据收在 docstring 位；若写成「任何常量」，
  仓里 `test_log_forging_l285.py:113` 与 `test_validation_finite_echo_l284.py:78/439` 那三处运行时
  字面量会被误判，正判据那一臂当场红。
- 通道二（第二只眼）：原样保留 `compile(..., dont_inherit=True)` 捕 `UnicodeEncodeError`。
  它只在部分解释器上有反应，所以**不再单独担牙齿**，只作通道一漏量时的兜底。
- 反过宽控制：`U+1F600` 那类合法星面字符不是代理 ⇒ 不判红；docstring 位之外的代理 ⇒ 不判红。
  判据不是「源文件不许有非 ASCII」，也不是「任何字符串不许有代理」。

**常驻尺子五面**：一、语料面自证（逐根验真 + 产品/测试两侧都在场，防「扫了个空目录」的假绿）；
二、正判据（全仓零失败，失败名单逐只点名）；三、牙齿（内存里摘掉那枚 `r` ⇒ 恰一处红且点名到那只文件）；
四、修面钉（那只文件现量确实是 raw docstring、模块 docstring 里零代理）；五、尺子自证（合成语料四臂：
docstring 位代理必点名、加回 `r` 必复绿、星面字符不误判、普通常量位的代理不误判 —— 四臂只喂内存字符串，
不碰仓库文件）。

零落盘：三面全部读磁盘文本后在内存里改写，仓库文件一字不动；五面只喂合成串。

跑法：`python -m pytest tests/unit/test_module_collectable_l289.py -q --no-cov`
版本主张：判据版本无关 ⇒ 3.12 与 3.13 两版应当给出同一份读数（这就是 A404 的收口证据）。
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


def docstring_constants(tree: ast.AST) -> List[ast.Constant]:
    """所有「会被写成 `__doc__`」的字符串常量：模块 / 函数 / 异步函数 / 类的第一条语句

    这就是 compile 那一步唯一会碰代理的位置 —— 边界由仓外探针 `Temp/l292q/probe_a404_boundary.py`
    在两版逐字同读钉住：docstring 位（模块 / 裸串首句 / 函数 / 异步函数 / 类 / 嵌套函数）放孤立代理
    ⇒ 3.13 的 `compile` 全数抛、3.12 一个不抛；普通常量（`VALUE = "\\ud800"`、列表里的元素）
    放同样的代理 ⇒ **两版都不抛**。
    ⇒ 判据钉在 docstring 位而不是「任何常量」，否则会把 `test_log_forging_l285.py:113`
    与 `test_validation_finite_echo_l284.py:78/439` 那三处**运行时字面量**误判成不可收集。
    """
    out: List[ast.Constant] = []

    def head(node) -> None:
        body = getattr(node, "body", None)
        if body and isinstance(body[0], ast.Expr):
            value = body[0].value
            if isinstance(value, ast.Constant) and isinstance(value.value, str):
                out.append(value)

    head(tree)
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            head(node)
    return out


def surrogate_hits(tree: ast.AST) -> List[str]:
    """docstring 位常量携带的孤立代理码点（通道一，版本无关）

    Python 的 utf-8 编码器对 U+D800–U+DFFF **全部**拒绝（连合法配对代理也不放过），所以
    「`__doc__` 里出现代理码点」就等于「这份源在 3.13 那一步编译会炸」⇒ 整支守卫不可收集。
    判据取**支持区间里最严那一版**的行为：3.12 侥幸能 compile，但它随时会因换解释器（或干净
    克隆）把整份定标读数变成 `Interrupted: 1 error during collection`，所以这里一律判红。
    """
    out: List[str] = []
    for node in docstring_constants(tree):
        for ch in node.value:
            if 0xD800 <= ord(ch) <= 0xDFFF:
                out.append("line %d col %d U+%04X" % (node.lineno, node.col_offset, ord(ch)))
    return sorted(set(out))


def uncollectable(items: Tuple[Tuple[str, str], ...]) -> List[Tuple[str, str]]:
    """逐只判可收集性，返回 `[(相对路径, 判决原文), ...]`

    两通道（A404 之后）：通道一 `surrogate_hits` 是承重判据，两版读数逐字相同；通道二
    `compile(..., dont_inherit=True)` 只在部分解释器上抛 `UnicodeEncodeError`，留作兜底。
    通道一命中就不再走通道二 —— 同一只文件不重复记账。
    """
    bad: List[Tuple[str, str]] = []
    for rel, text in items:
        try:
            tree = ast.parse(text)
        except SyntaxError as exc:  # 语法错与编码错同一档：都进不了收集
            bad.append((rel, f"SyntaxError: {exc}"))
            continue
        hits = surrogate_hits(tree)
        if hits:
            bad.append((rel, "LoneSurrogateInDocstring: " + "; ".join(hits)))
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

    A404 之后判决原文只依赖通道一（docstring 位的孤立代理码点），所以这一臂在 3.12 与 3.13 上
    必须给出**同一份**读数 —— 旧写法断言 `"UnicodeEncodeError" in bad[0][1]` 正是把牙齿
    钉在了解释器版本上。
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
    assert "LoneSurrogateInDocstring" in bad[0][1], bad
    assert "U+D800" in bad[0][1], bad
    assert "\\ud800" in items[L284_PY], "靶子文件的字面量被改没了 ⇒ 注入臂没东西可摘"


# ============ 五、尺子自证（合成语料四臂，版本无关判据自己的红绿） ============


def _synthetic(raw: bool, body: str) -> str:
    """造一份内存语料：raw ⇒ `\\ud800` 保持字面量，非 raw ⇒ 解成孤立代理"""
    return ('r"""展示一枚孤立代理：%s"""\n' % body) if raw else ('"""展示一枚孤立代理：%s"""\n' % body)


def test_synthetic_lone_surrogate_source_is_named_on_the_docstring_channel():
    """有牙：非 raw 的孤立代理字面量落在 docstring 位 ⇒ 恰一处红，且判决由通道一给出"""
    bad = uncollectable((("synthetic/lone.py", _synthetic(False, "\\ud800")),))
    assert [rel for rel, _ in bad] == ["synthetic/lone.py"], bad
    assert bad[0][1].startswith("LoneSurrogateInDocstring"), bad
    assert "U+D800" in bad[0][1], bad


def test_synthetic_raw_source_compiles_again():
    """复绿：同一份文本加回 `r` ⇒ 零失败（证明红是被那枚前缀推动的，不是合成串本身）"""
    assert uncollectable((("synthetic/raw.py", _synthetic(True, "\\ud800")),)) == []


def test_astral_but_not_surrogate_is_not_a_hit():
    """反过宽一：合法星面字符（U+1F600）不是代理 ⇒ 不判红

    没有这一臂，通道一可以写成「源里不许有非 ASCII」照样全绿 —— 那是一条比禁令宽得多的尺子。
    """
    src = '"""星面字符不是代理：\U0001F600"""\n'
    assert uncollectable((("synthetic/astral.py", src),)) == []


def test_surrogate_outside_docstring_position_is_not_a_hit():
    """反过宽二：同一个代理放在**普通常量**位 ⇒ 不判红

    边界由两版探针逐字同读钉住：`VALUE = "\\ud800"` 在两版 compile 都不抛 ⇒ 它不是「不可收集」。
    仓里真有两格这形状（`test_log_forging_l285.py:113` 与 `test_validation_finite_echo_l284.py:78/439`
    的运行时字面量），判据收在 docstring 位正是为了让正判据那一臂在真实语料上是绿的。
    """
    src = '"""正常的模块 docstring"""\n\nVALUE = "\\ud800"\nOTHER = ["\\ud800", "\\udbff"]\n'
    tree = ast.parse(src)
    assert docstring_constants(tree) == [tree.body[0].value], len(tree.body)
    assert surrogate_hits(tree) == []
    assert uncollectable((("synthetic/plain_const.py", src),)) == []


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
