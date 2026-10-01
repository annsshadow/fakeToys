# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L97：A177 的普查与判据 —— parametrize 的默认 id 不许随解释器劈叉

**为什么这一支该住在仓库里**：pytest 的参数化 id 默认由**参数对象现造**，而对象的渲染
随解释器变。L96 收尾做名册对账时实测撞出：3.14 给 PEP 604 联合（`int | str`）挂了
`__name__` 等于 `Union`，3.13 没有这个属性 ⇒ 同一条 `test_label` 在两侧长成两个节点名，
于是「两侧新增集合相同」这条对账判据被一次解释器差异无声劈开。L96 就地修了撞出的那一支
（七档显式命名 id）并写下「**下一轮第一件事：先普查再定修法**」。本轮先兑现普查，再把修法
冻成判据。

普查读数（由本文件的尺子在终态树上现量，四格都被 `test_census_shape_is_pinned` 钉住）：
  - 全仓 `parametrize(...)` 调用 **355** 支；
  - 可判面 **262** 支：值写成字面 list ⇒ 能静态看到喂了什么对象；
  - 盲面 **93** 支：值来自变量 / 生成器 / 函数调用 ⇒ 本尺子判不出，**记成盲而不是记成安全**
    （A171 那一族的第三种形状：尺子看不见的那一格最容易读成「干净」）；
  - 可判面里值位含类型对象 / 联合 / 命名空间属性的 **1** 支，正是 L96 已显式命名 id 的
    `test_schema_type_spec_l96.py` 那一支 ⇒ **违规集为空**。

判据口径沿用 L96 立行时写下的候选：「凡参数含类型对象或联合就必须显式 `ids`」。它比「只认
联合」严一档（裸 `int` 当值的 id 其实两侧一致），严出来的代价是后来人多写一个 `ids=`，
换来的是不必再逐对象论证「这个类型的渲染稳不稳定」。
"""

import ast
from pathlib import Path
from typing import Dict, List, Tuple

import pytest

from tests.unit.test_measurement_rulers_l94 import RulerNotProven

TESTS_ROOT = Path(__file__).resolve().parent.parent
AI_ROOT = TESTS_ROOT.parent

#: 作为**值**喂进 parametrize 的内置类型名。判定看的是「名字出现在值位」，
#: 所以 `float("nan")` 里那个被调用的 `float` 不算（v1 探针正是栽在这里，见下面自证样本）。
TYPE_VALUE_NAMES = frozenset({
    "int", "str", "float", "bool", "bytes", "bytearray", "complex",
    "list", "dict", "tuple", "set", "frozenset", "object", "type", "memoryview", "range",
})

#: 命名空间前缀：`typing.Any` / `types.FunctionType` 这类值同样由对象现造 id。
NS_PREFIXES = frozenset({
    "typing", "types", "collections", "datetime", "decimal", "enum",
    "numbers", "abc", "dataclasses", "functools", "itertools", "re",
})

# 现量普查读数（改动全仓 parametrize 形状就要重跑本文件并改这里，不许照抄上一轮）
# **收官后 A184 入库回填（总数 +1、可读 +1，盲面一格未动）**：新守卫里那支按桶名参数化的用例
# 是「argvalues 全是字符串字面量」的可读形状 ⇒ 只抬两格读数，93 格盲面与 L98 的六档分布一字未动。
# 这里刻意不用反引号点名那支新守卫文件：本文件的注释也在 A184 普查的扫描面里，多写一条断言形状
# 就要多改一处账（A141「写账本身还税」那一族的第 N 次复现，本轮选择少欠一笔）。
# **第二本账 L114 回填（总数 +1、可读 +1，盲面不动）**：test_export_enhanced 新增一支
# TestConversationEmptyFieldBranches 的对话式格式空字段假支参数化，桶=plain（可读）⇒
# MEASURED_CALLS 356→357、MEASURED_READABLE 263→264，MEASURED_BLIND 93 与 L98 六档分布一字未动。
#: 第二本账 L151 回填（总数 +2、可读 +2，盲面不动）：test_quality_dedup_gates_l76 新增两支
#: QualityScorer 阈值参数化（坏值 7 档 / 合法 4 档，全字面量列表）⇒ CALLS 357→359、
#: READABLE 264→266，BLIND 93 一格未动。
MEASURED_CALLS = 359
MEASURED_READABLE = 266
MEASURED_BLIND = 93
#: 值位含类型对象的那些档（必须全部自带 ids，否则就是本判据的违规）
KNOWN_TYPED_SITES = frozenset({("tests/unit/test_schema_type_spec_l96.py", "test_label")})


def _value_shapes(node, hits: set) -> None:
    """把「处在**值位**上的类型形状」记进 `hits`

    三条下钻规则：`f(x)` 不进 `f`（那是被调用的函数，不是被喂的参数，所以
    `float("nan")` 与 `set()` 都不算类型值）；`a.b` 当叶子（`a` 是命名空间前缀，整条属性链
    才是那个值）；`X | Y` 记一次 `union` 后**不下钻**（联合的两边必然是类型，再记一遍
    `int` / `str` 只是把同一格数成两格）。
    """
    if isinstance(node, ast.BinOp) and isinstance(node.op, ast.BitOr):
        hits.add("union")
        return
    if isinstance(node, ast.Name):
        if node.id in TYPE_VALUE_NAMES:
            hits.add(node.id)
        return
    if isinstance(node, ast.Attribute):
        if isinstance(node.value, ast.Name) and node.value.id in NS_PREFIXES:
            hits.add(f"{node.value.id}.{node.attr}")
        return
    if isinstance(node, ast.Call):
        for arg in node.args:
            _value_shapes(arg, hits)
        for kw in node.keywords:
            _value_shapes(kw.value, hits)
        return
    for child in ast.iter_child_nodes(node):
        _value_shapes(child, hits)


def _hits_for(value_arg) -> Tuple[str, Tuple[str, ...]]:
    """判一支 parametrize 的值参数：返回 `(桶, 命中的类型值形状)`

    桶有三态，**没有「判不出就算安全」那一支**：字面 list 之外一律 `no-literal-list`。
    """
    if not isinstance(value_arg, ast.List):
        return "no-literal-list", ()
    hits: set = set()
    for elem in value_arg.elts:
        _value_shapes(elem, hits)
    return ("type-valued" if hits else "plain"), tuple(sorted(hits))


#: 已知答案的样本。`plain_case` 里 `float` / `set` 是**被调用**的函数，必须读成 plain
#: （v1 探针把这类当类型值收了 24 支假阳性）；`violation_case` 必须读成「类型值且无 ids」，
#: 所以这份样本自己就证明了判据能红 —— 一份读不出任何违规的样本无法证明检出能力。
SAMPLE_SRC = '''
import pytest
VALUES = [1, 2]

@pytest.mark.parametrize("a,b", [(float("nan"), 1), (set(), 2)])
def plain_case(a, b): pass

@pytest.mark.parametrize("t,want", [(int | str, "u"), (str, "s")], ids=["union", "str"])
def named_case(t, want): pass

@pytest.mark.parametrize("t", [typing.Any])
def violation_case(t): pass

@pytest.mark.parametrize("v", VALUES)
def blind_case(v): pass
'''

SAMPLE_ANSWER = {
    "plain_case": ("plain", ()),
    "named_case": ("type-valued", ("str", "union")),
    "violation_case": ("type-valued", ("typing.Any",)),
    "blind_case": ("no-literal-list", ()),
}


def _sample_sites() -> Dict[str, dict]:
    """跑样本源码，得到 `用例名 → 读数`，用于每次读数前自证"""
    return {r["test"]: r for r in _sites_from(ast.parse(SAMPLE_SRC), "<sample>")}


def _sites_from(tree: ast.AST, rel: str) -> List[dict]:
    out = []
    for node in ast.walk(tree):
        if not isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            continue
        for dec in node.decorator_list:
            call = dec.value if isinstance(dec, ast.Attribute) else dec
            if not (isinstance(call, ast.Call) and isinstance(call.func, ast.Attribute)):
                continue
            if call.func.attr != "parametrize":
                continue
            value_arg = None
            for a in call.args[1:]:
                value_arg = a
                break
            if value_arg is None:
                for k in call.keywords:
                    if k.arg == "argvalues":
                        value_arg = k.value
            if value_arg is None:
                continue
            bucket, hits = _hits_for(value_arg)
            out.append({
                "file": rel, "line": node.lineno, "test": node.name,
                "ids": any(k.arg == "ids" for k in call.keywords),
                "bucket": bucket, "hits": hits,
            })
    return out


def _rel(p: Path, root: Path) -> str:
    """仓内文件给仓库相对路径（判据里的名册要能读），临时注入给裸名（不为它编一套前缀）"""
    try:
        return p.relative_to(AI_ROOT).as_posix()
    except ValueError:
        return p.name


def param_sites(root: Path) -> List[dict]:
    """普查 `root` 下所有 `@pytest.mark.parametrize` 调用及其值形状

    每次读数前先跑 `SAMPLE_SRC` 自证（A171 口径：自检写在读数函数里，不是另一支可选用例），
    对不上就抛 `RulerNotProven`。

    Args:
        root: 要普查的目录（通常是 `tests/`，注入档会指向临时目录）

    Returns:
        每支 parametrize 一条 dict：`file` / `line` / `test` / `ids` / `bucket` / `hits`

    Raises:
        RulerNotProven: 样本读数与已知答案不同，或 root 下一支 parametrize 都没读到
    """
    sample = _sample_sites()
    for name, expected in SAMPLE_ANSWER.items():
        got = sample[name]["bucket"], sample[name]["hits"]
        if got != expected:
            raise RulerNotProven(f"parametrize 尺子读错样本 {name}：{got} != {expected}")
    if not any(r["bucket"] == "type-valued" and not r["ids"] for r in sample.values()):
        raise RulerNotProven("样本里没有「类型值且无 ids」那一档，这份样本证明不了判据能红")

    files = sorted(p for p in Path(root).rglob("*.py") if "__pycache__" not in p.parts)
    rows: List[dict] = []
    for p in files:
        rows.extend(_sites_from(ast.parse(p.read_text(encoding="utf-8", errors="replace")),
                                _rel(p, Path(root))))
    if not rows:
        raise RulerNotProven(f"{root} 里一支 parametrize 都没读到：这是数不到，不是干净")
    return rows


def violations(rows: List[dict]) -> set:
    """值位含类型对象 / 联合却没写 `ids=` 的那些档，键为 `(文件, 用例名)`"""
    return {(r["file"], r["test"]) for r in rows if r["bucket"] == "type-valued" and not r["ids"]}


def blind_sites(rows: List[dict]) -> set:
    """静态判不出值形状的那些档（值来自变量 / 生成器 / 调用）"""
    return {(r["file"], r["test"]) for r in rows if r["bucket"] == "no-literal-list"}


@pytest.fixture(scope="module")
def rows() -> List[dict]:
    return param_sites(TESTS_ROOT)


class TestTheRulerProvesItself:
    """尺子自证：先证明它读得对，再拿它读数"""

    def test_sample_readings_match_known_answers(self):
        sample = _sample_sites()
        for name, expected in SAMPLE_ANSWER.items():
            assert (sample[name]["bucket"], sample[name]["hits"]) == expected, name

    def test_called_builtin_is_not_a_type_value(self):
        """v1 假阳性的那一格：`float("nan")` / `set()` 是被调用的函数，不是被喂的类型"""
        assert _sample_sites()["plain_case"]["bucket"] == "plain"

    def test_sample_contains_a_detectable_violation(self):
        """样本必须能让判据红一次 —— 读不出违规的样本证明不了检出能力"""
        assert violations(list(_sample_sites().values())) == {("<sample>", "violation_case")}

    def test_ruler_refuses_a_silence_shaped_like_success(self, tmp_path):
        """数不到东西要抛，而不是返回空集让「干净」看起来成立（A171 本体）"""
        empty = tmp_path / "t.py"
        empty.write_text("def nothing(): pass\n", encoding="utf-8")
        with pytest.raises(RulerNotProven):
            param_sites(tmp_path)


class TestCensusShapeIsPinned:
    """普查规模钉死：动到全仓 parametrize 形状就要重量并改常量"""

    def test_census_totals_match_this_rounds_measurement(self, rows):
        assert len(rows) == MEASURED_CALLS
        blind = sum(1 for r in rows if r["bucket"] == "no-literal-list")
        assert blind == MEASURED_BLIND
        assert len(rows) - blind == MEASURED_READABLE
        assert MEASURED_READABLE + MEASURED_BLIND == MEASURED_CALLS

    def test_typed_face_is_exactly_the_known_sites(self, rows):
        """可判面里喂了类型对象的就那几支：多出未知的一支必须被看见，而不是默默进违规集"""
        typed = [r for r in rows if r["bucket"] == "type-valued"]
        assert len(typed) == len(KNOWN_TYPED_SITES)
        assert {(r["file"], r["test"]) for r in typed} == KNOWN_TYPED_SITES


class TestNoInterpreterDependentIds:
    """判据本体"""

    def test_every_type_valued_parametrize_names_its_ids(self, rows):
        assert violations(rows) == set()

    def test_known_typed_site_still_carries_explicit_ids(self, rows):
        """删掉那支用例或摘掉它的 ids 都会让上一条判据变成空转 ⇒ 这里单独钉住"""
        site = ("tests/unit/test_schema_type_spec_l96.py", "test_label")
        assert site in KNOWN_TYPED_SITES
        hits = [r for r in rows if (r["file"], r["test"]) == site]
        assert len(hits) == 1, f"那一支的定位读到 {len(hits)} 份，名册已漂"
        assert hits[0]["ids"] is True
        assert "union" in hits[0]["hits"]


class TestInjectionMakesItRed:
    """反向证据：往临时目录写一支「联合当值且不写 ids」的用例，判据必须点名它"""

    def test_union_without_ids_is_flagged(self, tmp_path):
        bad = tmp_path / "injected.py"
        bad.write_text(
            "import pytest\n\n\n"
            '@pytest.mark.parametrize("t", [int | str])\n'
            "def injected_union(t):\n    pass\n",
            encoding="utf-8",
        )
        injected = param_sites(tmp_path)
        assert violations(injected) == {("injected.py", "injected_union")}

    def test_the_same_site_with_ids_is_clean(self, tmp_path):
        ok = tmp_path / "injected.py"
        ok.write_text(
            "import pytest\n\n\n"
            '@pytest.mark.parametrize("t", [int | str], ids=["int-or-str"])\n'
            "def injected_union(t):\n    pass\n",
            encoding="utf-8",
        )
        assert violations(param_sites(tmp_path)) == set()

    def test_blind_face_is_recorded_as_blind_not_as_safe(self, tmp_path):
        """值来自变量 ⇒ 落进盲桶，绝不落进「已判过且干净」"""
        src = tmp_path / "injected.py"
        src.write_text(
            "import pytest\nVALUES = [1]\n\n\n"
            '@pytest.mark.parametrize("v", VALUES)\n'
            "def blind(v):\n    pass\n",
            encoding="utf-8",
        )
        got = param_sites(tmp_path)
        assert got[0]["bucket"] == "no-literal-list"
        assert blind_sites(got) == {("injected.py", "blind")}
        assert violations(got) == set()
