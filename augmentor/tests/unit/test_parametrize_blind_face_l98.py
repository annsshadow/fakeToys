# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L98：把 L97 留下的 93 格盲面**逐格定性**，并且用两把互相独立的尺子做同一件事

**这一轮还的是 L97 账上明写的那笔残项**：批③ 的普查把全仓 parametrize 分成「可判 262 / 判不出 93」，
而账上如实记下「本轮只钉规模与形状，没数这 93 格里有多少其实也喂了类型对象 —— 那是下一轮的活」。
本轮就数这一格。

两把尺子，同一问题：
  静态二阶解析（本文件的 `resolve`）：名字能追到赋值右侧的，就把元素表达式读出来；追不到的**记成盲，
    不记成干净**。这一档撞出三格形状，都是 L97 那把尺子看不见的：
      ① `out = []` 再 `out.append(...)` 装配出来的清单 —— 解析「成功」了，元素却是零个，
        读起来像一份全常量的干净名单。所以本尺子定死一条：**解析到 0 个元素一律不算 plain**。
      ② `ids=` 不总是字面量。`tests/unit/test_exceptions.py` 有四处写成
        `ids=lambda e: e.__name__` / `ids=lambda v: getattr(v, "__name__", str(v))` —— L97 只记
        「有没有 `ids=`」，于是这一类被读成「作者已把 id 写死」，而**它恰恰是从对象现造的**：
        与 L96 那格同源的风险搬进了 ids 自己。所以本文件把 ids 分成 `literal` / `derived` / `none`
        三档，只有 `literal` 才给豁免。
      ③ 值参数是 `sorted(NAME)` / `list(DICT)` / `NAME[:-1]` / 推导式 —— unwrap 之后元素可见性不同，
        派生档单列，不与「常量看得见」同档。
  动态面（`TestDynamicIdMakerFace`）：直接读 pytest 真正拿去造 id 的**对象**。`_pytest/python.py` 里
    `_idval_from_value` 的顺序是：str / 数字 / 正则 / enum，然后「anything with a `__name__`」，
    最后才是 `<参数名><序号>`。⇒ 唯一会随解释器版本劈叉的分支就是 `__name__` 那一支（3.14 给
    PEP 604 联合挂了 `__name__ == "Union"`，3.13 没有 ⇒ 同一个值两侧渲染成两个节点名，这正是 L96
    撞出的事故机制）。所以判据可以贴着机制写：**值对象的 `__name__` 若来自非本仓模块，就必须被
    字面 `ids=` 挡住**，否则红。

动态面为什么不算「第二套正则」：它不复用静态判据的任何一条 —— 静态看表达式形状，动态看运行期对象；
两者对同一问题的读数必须能对上（`TestTwoRulersAgree`），对不上就是其中一把在说谎。L97 那把尺子的
函数（`param_sites` / `_value_shapes` / `TYPE_VALUE_NAMES` / 三档常数）在本文件里是**直接 import**
的，没有抄第二份。

现量读数与盲面六档分布见 `MEASURED_TIERS` 那一族常量的注释；取证与逐格名册由本文件自己跑出来。
"""

import ast
import sys
import types
from pathlib import Path
from typing import Dict, List, Optional, Tuple

import pytest

from tests.unit.test_measurement_rulers_l94 import RulerNotProven
from tests.unit.test_parametrize_ids_l97 import (
    MEASURED_BLIND, MEASURED_CALLS, MEASURED_READABLE, _value_shapes, param_sites, violations,
)

TESTS_ROOT = Path(__file__).resolve().parent.parent

#: `sorted(x)` / `list(x)` 这类只换容器不改对象的包装 —— unwrap 后元素就是同一批对象
UNWRAP_CALLS = frozenset({"sorted", "list", "tuple", "set", "frozenset", "reversed"})

#: 只有这些常量种类算「对象看得见」（bytes 也算：pytest 直接把它转义进 id，不读 `__name__`）
VISIBLE_CONSTANTS = (str, bytes, bool, int, float, complex, type(None))

#: 作者自己命名的对象：类、函数、方法、模块 —— 它们的名字由源码写死，不会「因为解释器版本而忽然
#: 出现或消失」，所以走 pytest 的 `__name__` 分支是稳的。
AUTHORED_NAME_KINDS = (type, types.FunctionType, types.MethodType, types.ModuleType,
                       types.BuiltinFunctionType, types.BuiltinMethodType)

# ---------------------------------------------------------------- 六档
TIER_IDS_LITERAL = "ids-literal"    # ids 是字符串字面量列表 ⇒ id 与对象无关
TIER_IDS_DERIVED = "ids-derived"    # 写了 ids，但值本身从对象现造（lambda / 推导式 / 名字）
TIER_PLAIN = "plain"                # 解析得到，且元素全是看得见的常量
TIER_TYPED = "typed"                # 解析得到，且值位读到类型形状
TIER_DERIVED = "derived"            # 解析得到表达式，但元素对象看不见（含「元素为 0」）
TIER_OPAQUE = "opaque"              # 压根解析不到

# L98 现量（Temp/l98q/static_tiers.py 与 Temp/l98q/l98_dyn.py 各跑一遍；改形状就要重量）
# 六档之和 = MEASURED_BLIND。`typed` 与 `ids-literal` 现量为 **0**，所以不在这张表里 ——
# 它们各由一条「名册等于空集」的判据钉住（`test_typed_blind_sites_are_exactly_the_known_ones`
# 与 `test_no_blind_site_carries_literal_ids`），而不是靠这张表缺席来表达。
MEASURED_TIERS: Dict[str, int] = {
    TIER_PLAIN: 46,
    TIER_DERIVED: 30,
    TIER_IDS_DERIVED: 8,
    TIER_OPAQUE: 9,
}
#: 盲面里静态读到类型形状的格（无字面 ids 即违规）：**0 格** —— 这就是 L97 残项的答案
MEASURED_TYPED_SITES: frozenset = frozenset()
#: 静态彻底解析不到的 9 格，键为 `文件::用例#该用例上第几个 parametrize`
MEASURED_OPAQUE_SITES: frozenset = frozenset({
    "integration/test_cli_dataset_tools.py::test_every_supported_target_really_converts#0",
    "integration/test_cli_merged_commands.py::test_every_cli_choice_actually_exports#0",
    "unit/test_converter_excel_input_l80.py::test_three_spellings_are_the_same_edge#0",
    "unit/test_logging_wiring_a97.py::test_every_allowed_name_is_a_real_stdlib_level#0",
    "unit/test_logging_wiring_a97.py::test_every_allowed_name_maps_to_the_stdlib_constant#0",
    "unit/test_model_type_choices_l74.py::test_every_listed_type_builds_its_backend#0",
    "unit/test_model_type_choices_l74.py::test_every_listed_type_is_accepted_by_both_faces#0",
    "unit/test_model_type_choices_l74.py::test_table_values_are_backends_not_strings#0",
    "unit/test_rag.py::test_supported_formats_dispatch#0",
})
#: `ids=` 写成非字面量的 8 格（同上键形）—— L97 只记「有没有 ids」，这一档是它看不见的
MEASURED_IDS_DERIVED_SITES: frozenset = frozenset({
    "integration/test_api_dataset_system_tools.py::test_rejected_with_400#1",
    "unit/test_exceptions.py::test_builtin_compat_contract#0",
    "unit/test_exceptions.py::test_every_domain_exception_is_augmentor_error#0",
    "unit/test_exceptions.py::test_malformed_bracket_payload_raises_model_response_error#0",
    "unit/test_exceptions.py::test_malformed_payload_is_still_a_value_error#0",
    "unit/test_model_entry_defaults_l75.py::test_direct_construction_keeps_the_placeholder#0",
    "unit/test_model_entry_defaults_l75.py::test_loader_message_names_the_real_entry#0",
    "unit/test_model_entry_defaults_l75.py::test_none_and_empty_string_reach_the_same_verdict#0",
})
#: 动态面**不做豁免**时读到的风险名册，按解释器分档钉住（同一份源码在两版上是两种对象，
#: 把两版读数写进同一张表才是这条判据的全部信息量；未知解释器直接抛，不给读数）
MEASURED_HAZARD_ROSTER: Dict[Tuple[int, int], frozenset] = {
    (3, 13): frozenset({
        "unit/test_schema_type_spec_l96.py::test_label<-value:typing._UnionGenericAlias:Union"}),
    (3, 14): frozenset({
        "unit/test_schema_type_spec_l96.py::test_label<-value:typing.Union:Union"}),
}
#: 现量：盲面用例 90 支（93 格调用压成 90 个名字），动态面两支解释器都全数采到
MEASURED_BLIND_TESTS = 90
MEASURED_BLIND_SEEN = 90
#: 现量：整支豁免（该用例的每一支 parametrize 都写了字面 ids）的用数
MEASURED_EXEMPT_SITES = 17
#: 动态面至少要采到这么多条，否则「没有风险对象」只是没采到（A171 口径：数不到不判干净）
MIN_COLLECTED_ITEMS = 3000

# ------------------------------------------------------------------ 静态：作用域与解析


class Scope:
    """一个文件里的 `名字 → 值表达式` 与 `函数名 → 唯一 return 表达式`"""

    def __init__(self, tree: ast.AST):
        self.assign: Dict[str, ast.expr] = {}
        self.funcs: Dict[str, ast.expr] = {}
        for node in ast.walk(tree):
            if isinstance(node, ast.Assign):
                for target in node.targets:
                    if isinstance(target, ast.Name):
                        self.assign.setdefault(target.id, node.value)
            elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                rets = [n for n in ast.walk(node) if isinstance(n, ast.Return) and n.value is not None]
                if len(rets) == 1:
                    self.funcs.setdefault(node.name, rets[0].value)


def resolve(value: ast.expr, scope: Scope, depth: int = 0) -> Tuple[bool, List[ast.expr]]:
    """把 parametrize 的值表达式解析成「元素表达式列表」

    Returns:
        `(是否解析到容器, 元素列表)`。解析到**零个元素**不是错误，但上层不许把它读成干净 ——
        `out = []` 再 `out.append(...)` 装配出来的清单正是这一档。
    """
    if depth > 6:
        return False, []
    if isinstance(value, ast.Call):
        fname = value.func.id if isinstance(value.func, ast.Name) else None
        if fname in UNWRAP_CALLS and value.args:
            return resolve(value.args[0], scope, depth + 1)
        if fname is not None and fname in scope.funcs:
            return resolve(scope.funcs[fname], scope, depth + 1)
        return False, []
    if isinstance(value, ast.Name):
        return _follow(value.id, scope, depth)
    if isinstance(value, ast.Attribute):
        return _follow(value.attr, scope, depth)
    if isinstance(value, ast.Subscript):
        return resolve(value.value, scope, depth + 1)
    if isinstance(value, ast.BinOp) and isinstance(value.op, ast.Add):
        lok, left = resolve(value.left, scope, depth + 1)
        rok, right = resolve(value.right, scope, depth + 1)
        return (lok or rok), (left + right if (lok or rok) else [])
    if isinstance(value, (ast.List, ast.Tuple, ast.Set)):
        return True, list(value.elts)
    if isinstance(value, ast.Dict):
        return True, list(value.values) + list(value.keys)
    if isinstance(value, (ast.ListComp, ast.SetComp, ast.GeneratorExp)):
        return True, [value.elt]
    if isinstance(value, ast.DictComp):
        return True, [value.key, value.value]
    return False, []


def _follow(name: str, scope: Scope, depth: int) -> Tuple[bool, List[ast.expr]]:
    if name in scope.assign:
        return resolve(scope.assign[name], scope, depth + 1)
    return False, []


def visible_constant(node: ast.expr) -> bool:
    """表达式是否**只**由常量组成（`-1` 只在一元号下接数字常量时才算）"""
    if isinstance(node, ast.Constant):
        return isinstance(node.value, VISIBLE_CONSTANTS)
    if isinstance(node, ast.UnaryOp) and isinstance(node.operand, ast.Constant):
        return isinstance(node.operand.value, (int, float, complex))
    if isinstance(node, (ast.List, ast.Tuple, ast.Set)):
        return all(visible_constant(e) for e in node.elts)
    if isinstance(node, ast.Dict):
        pairs = list(zip(node.keys, node.values))
        return all(k is not None and visible_constant(k) and visible_constant(v)
                   for k, v in pairs)
    return False


def ids_form(call: ast.Call) -> str:
    """`ids=` 的写法：只有字符串字面量列表算 `literal`，写了但不是字面量算 `derived`

    这一档是本轮新长出来的判据：L97 只记「有没有 `ids=`」，于是
    `ids=lambda e: e.__name__` 被读成安全，而它恰恰是从对象现造的。
    """
    kw = next((k for k in call.keywords if k.arg == "ids"), None)
    if kw is None:
        return "none"
    if (isinstance(kw.value, ast.List)
            and all(isinstance(e, ast.Constant) and isinstance(e.value, str) for e in kw.value.elts)):
        return TIER_IDS_LITERAL
    return TIER_IDS_DERIVED


def param_calls(tree: ast.AST) -> List[Tuple[ast.FunctionDef, ast.Call, int]]:
    """`[(函数节点, parametrize 调用, 该函数上第几个装饰器)]`，按源码顺序"""
    out = []
    for node in ast.walk(tree):
        if not isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            continue
        idx = 0
        for dec in node.decorator_list:
            call = dec.value if isinstance(dec, ast.Attribute) else dec
            if not (isinstance(call, ast.Call) and isinstance(call.func, ast.Attribute)):
                continue
            if call.func.attr != "parametrize":
                continue
            out.append((node, call, idx))
            idx += 1
    return out


def sites(tree: ast.AST, rel: str, scope: Scope) -> List[dict]:
    """扫一棵树里的 parametrize，产出 `(用例, 序号, 档, 值形状, ids 写法, 元素数)`

    序号是**同一支用例上第几个 parametrize 装饰器**：L97 的 `(文件, 用例)` 键形会把
    「一支用例挂两支装饰器」的 14 个名字压成 1 格（355 支调用只对应 340 个名字），名册判据
    要是照那个键形写，就会被自己压扁。
    """
    out: List[dict] = []
    for node, call, idx in param_calls(tree):
        value_arg = call.args[1] if len(call.args) > 1 else next(
            (k.value for k in call.keywords if k.arg == "argvalues"), None)
        if value_arg is None:
            continue
        form = ids_form(call)
        literal = isinstance(value_arg, ast.List)
        ok, elts = (True, list(value_arg.elts)) if literal else resolve(value_arg, scope)
        hits: set = set()
        if ok:
            for el in elts:
                _value_shapes(el, hits)
        if form == TIER_IDS_LITERAL:
            tier = TIER_IDS_LITERAL
        elif form == TIER_IDS_DERIVED:
            tier = TIER_IDS_DERIVED
        elif not ok:
            tier = TIER_OPAQUE
        elif hits:
            tier = TIER_TYPED
        elif not elts or not all(visible_constant(e) for e in elts):
            tier = TIER_DERIVED
        else:
            tier = TIER_PLAIN
        out.append({"file": rel, "test": node.name, "idx": idx, "tier": tier,
                    "hits": tuple(sorted(hits)), "ids": form, "literal": literal,
                    "n_elts": len(elts) if ok else -1})
    return out


# ------------------------------------------------------------------ 静态：尺子的自证样本
#: 已知答案的样本，六档各有其位。样本里那支靠 append 累积参数的 parametrize 源专钉
#: 「解析到 0 元素不许算干净」，`lambda_ids_case` 专钉「`ids=` 非字面量不给豁免」。
#: （这里刻意不给那支 helper 写成「名字 + 括号」的主张形状：它只住在下面那份字符串样本里，
#: 代码面没有它的声明 —— A184 的符号档据此会判它失效，而那句话本意是描述样本，不是指路。）
SAMPLE_SRC = '''
import pytest

STRINGS = ["a", "b"]
PAIRS = [(1, "x"), (2, "y")]
TYPES = [int | str]
DERIVED = [f.name for f in range(3)]


def made():
    out = []
    out.append(("a", 1))
    return out


@pytest.mark.parametrize("v", STRINGS)
def plain_case(v): pass


@pytest.mark.parametrize("pair", PAIRS)
def tuple_plain_case(pair): pass


@pytest.mark.parametrize("t", TYPES)
def typed_case(t): pass


@pytest.mark.parametrize("v", DERIVED)
def derived_case(v): pass


@pytest.mark.parametrize("v", made())
def append_case(v): pass


@pytest.mark.parametrize("v", [1], ids=lambda x: str(x))
def lambda_ids_case(v): pass


@pytest.mark.parametrize("v", [1], ids=["one"])
def literal_ids_case(v): pass


@pytest.mark.parametrize("v", NOPE)
def opaque_case(v): pass
'''

#: 每支样本用例的期望读数：`(档, 值形状命中, ids 写法)`
SAMPLE_ANSWER = {
    "plain_case": (TIER_PLAIN, (), "none"),
    "tuple_plain_case": (TIER_PLAIN, (), "none"),
    "typed_case": (TIER_TYPED, ("union",), "none"),
    "derived_case": (TIER_DERIVED, (), "none"),
    "append_case": (TIER_DERIVED, (), "none"),
    "lambda_ids_case": (TIER_IDS_DERIVED, (), TIER_IDS_DERIVED),
    "literal_ids_case": (TIER_IDS_LITERAL, (), TIER_IDS_LITERAL),
    "opaque_case": (TIER_OPAQUE, (), "none"),
}


def sample_rows() -> Dict[str, dict]:
    tree = ast.parse(SAMPLE_SRC)
    return {r["test"]: r for r in sites(tree, "<sample>", Scope(tree))}


def prove_ruler() -> None:
    """每次读数前先自证（A171 口径：自检写在读数函数里，不是另一支可选用例）"""
    got = sample_rows()
    for name, expected in SAMPLE_ANSWER.items():
        row = got[name]
        found = (row["tier"], row["hits"], row["ids"])
        if found != expected:
            raise RulerNotProven(f"盲面尺子读错样本 {name}：{found} != {expected}")
    if not any(r["n_elts"] == 0 for r in got.values()):
        raise RulerNotProven("样本里没有「解析得到而元素为 0」那一档，这份样本证明不了 append 陷阱被拦住")


def static_rows(root: Path) -> List[dict]:
    """普查 `root` 下每个 .py 的 parametrize（含二阶解析后的档位）

    Raises:
        RulerNotProven: 样本自证失败，或 `root` 下一支 parametrize 都没读到
    """
    prove_ruler()
    paths = sorted(p for p in Path(root).rglob("*.py") if "__pycache__" not in p.parts)
    trees = {p: ast.parse(p.read_text(encoding="utf-8", errors="replace")) for p in paths}
    rows: List[dict] = []
    for p in paths:
        rel = p.relative_to(root).as_posix()
        rows.extend(sites(trees[p], rel, Scope(trees[p])))
    if not rows:
        raise RulerNotProven(f"{root} 里一支 parametrize 都没读到：这是数不到，不是干净")
    return rows


def key(row: dict) -> str:
    return f"{row['file']}::{row['test']}#{row['idx']}"


def blind_rows(rows: List[dict]) -> List[dict]:
    """L97 口径下的盲面（值参数不是字面 list）在本文件里的读数"""
    return [r for r in rows if not r["literal"]]


def tier_counts(rows: List[dict]) -> Dict[str, int]:
    counts: Dict[str, int] = {}
    for row in blind_rows(rows):
        counts[row["tier"]] = counts.get(row["tier"], 0) + 1
    return counts


# ------------------------------------------------------------------ 动态：pytest 真正拿去造 id 的对象
def hazardous(val: object) -> Optional[str]:
    """这个值会不会被 pytest 的 `__name__` 分支渲染成 id，而它又不是「作者自己命名的对象」

    判据贴着 `_pytest/python.py` 的 `_idval_from_value`：它在 str / 数字 / 正则 / enum 之后试
    `getattr(val, "__name__", None)`。⇒ 危险的不是「有名字」，而是**一个本来没有名字的对象在某版
    解释器上长出了名字**：3.14 给 PEP 604 联合挂了 `__name__ == "Union"`，3.13 没有 ⇒ 同一个值两侧
    渲染成两个节点名（L96 撞出的事故机制）。所以类 / 函数 / 模块这些「名字由作者写死」的对象不算，
    `X | Y`、`typing.*` 别名这类「名字由解释器决定」的对象才算。
    """
    if isinstance(val, AUTHORED_NAME_KINDS):
        return None
    name = getattr(val, "__name__", None)
    if not isinstance(name, str):
        return None
    kind = type(val)
    return f"{kind.__module__}.{kind.__name__}:{name}"


def item_site(item) -> str:
    """采集项 → `相对 tests/ 的路径::用例名`，与静态名册同一键形（不另起一套）"""
    path = Path(str(item.location[0])).resolve()
    return f"{path.relative_to(TESTS_ROOT).as_posix()}::{item.originalname}"


def dynamic_risky(items, exempt: set) -> set:
    """逐条采集项读参数对象，返回 `文件::用例<-参数名:类型:名` 的风险集

    Args:
        items: `session.items`
        exempt: 可以整支豁免的 `文件::用例` 集合（只允许是**所有**装饰器都写字面 ids 的那些）
    """
    out = set()
    for item in items:
        spec = getattr(item, "callspec", None)
        if spec is None:
            continue
        site = item_site(item)
        if site in exempt:
            continue
        for argname, value in spec.params.items():
            got = hazardous(value)
            if got:
                out.add(f"{site}<-{argname}:{got}")
    return out


def exempt_sites(rows: List[dict]) -> set:
    """所有 parametrize 装饰器都写了字面 ids 的用例 ⇒ 动态面才整支放过（少一个都不放）"""
    per: Dict[str, List[str]] = {}
    for row in rows:
        per.setdefault(f"{row['file']}::{row['test']}", []).append(row["ids"])
    return {site for site, forms in per.items() if all(f == TIER_IDS_LITERAL for f in forms)}


@pytest.fixture(scope="session")
def collected(request) -> list:
    items = request.session.items
    if len(items) < MIN_COLLECTED_ITEMS:
        raise RulerNotProven(
            f"本次只采到 {len(items)} 条用例（下限 {MIN_COLLECTED_ITEMS}）：动态面数不到东西，"
            "「没有风险对象」在这一跑里不成立")
    return items


@pytest.fixture(scope="session")
def rows() -> List[dict]:
    return static_rows(TESTS_ROOT)


class TestTheStaticRulerProvesItself:
    def test_sample_readings_match_known_answers(self):
        got = sample_rows()
        for name, expected in SAMPLE_ANSWER.items():
            row = got[name]
            assert (row["tier"], row["hits"], row["ids"]) == expected, name

    def test_empty_resolution_is_not_clean(self):
        """`out = []` 再 append 的清单：解析「成功」而元素为零 ⇒ 不许读成 plain（A171 的零形状）"""
        row = sample_rows()["append_case"]
        assert row["n_elts"] == 0
        assert row["tier"] == TIER_DERIVED

    def test_callable_ids_is_not_an_exemption(self):
        """`ids=lambda ...` 不进字面量档 —— L97 只记有无，这里补上「写的是什么」"""
        got = sample_rows()
        assert got["lambda_ids_case"]["tier"] == TIER_IDS_DERIVED
        assert got["literal_ids_case"]["tier"] == TIER_IDS_LITERAL

    def test_ruler_refuses_a_silence_shaped_like_success(self, tmp_path):
        empty = tmp_path / "t.py"
        empty.write_text("def nothing(): pass\n", encoding="utf-8")
        with pytest.raises(RulerNotProven):
            static_rows(tmp_path)


class TestBlindFaceIsQualifiedPerSite:
    """还 L97 残项：93 格盲面逐格定性，六档之和等于盲面数"""

    def test_census_agrees_with_the_l97_ruler(self, rows):
        assert len(rows) == len(param_sites(TESTS_ROOT)) == MEASURED_CALLS
        assert len(blind_rows(rows)) == MEASURED_BLIND
        assert len(rows) - len(blind_rows(rows)) == MEASURED_READABLE

    def test_tier_counts_are_pinned(self, rows):
        counts = tier_counts(rows)
        assert counts == MEASURED_TIERS, counts
        assert sum(counts.values()) == MEASURED_BLIND

    def test_typed_blind_sites_are_exactly_the_known_ones(self, rows):
        got = {key(r) for r in blind_rows(rows) if r["tier"] == TIER_TYPED}
        assert got == MEASURED_TYPED_SITES, sorted(got ^ MEASURED_TYPED_SITES)

    def test_unreadable_sites_are_named_not_counted(self, rows):
        """`opaque` 是名册而不是一个数：多一格必须点名，少一格同样"""
        got = {key(r) for r in blind_rows(rows) if r["tier"] == TIER_OPAQUE}
        assert got == MEASURED_OPAQUE_SITES, sorted(got ^ MEASURED_OPAQUE_SITES)

    def test_derived_ids_sites_are_named(self, rows):
        got = {key(r) for r in blind_rows(rows) if r["tier"] == TIER_IDS_DERIVED}
        assert got == MEASURED_IDS_DERIVED_SITES, sorted(got ^ MEASURED_IDS_DERIVED_SITES)

    def test_no_blind_site_carries_literal_ids(self, rows):
        """盲面里现量 **0** 格写了字面 ids ⇒ 这条判据钉的是「零」，不是「缺席」

        为什么值得单独钉：字面 ids 是盲面唯一的豁免通道，一旦有人给某支盲面用例补上 `ids=[...]`，
        它在六档里的归属就会从 plain / derived / opaque 变成 `ids-literal`，那五档的数会同时漂。
        """
        got = {key(r) for r in blind_rows(rows) if r["tier"] == TIER_IDS_LITERAL}
        assert got == set(), sorted(got)

    def test_every_blind_read_is_explainable_by_its_own_fields(self, rows):
        """档位不是自由变量：每一行的档必须能由同一行的 `ids` / `hits` / `n_elts` 解释

        钉的是判定级联的顺序本身 —— 若有人把 `opaque` 那一支挪到 `plain` 之后（或删掉），
        解析不到的行就会漏进干净档，这里立刻红。
        """
        for r in blind_rows(rows):
            if r["tier"] == TIER_IDS_LITERAL:
                assert r["ids"] == TIER_IDS_LITERAL, key(r)
            elif r["tier"] == TIER_IDS_DERIVED:
                assert r["ids"] == TIER_IDS_DERIVED, key(r)
            elif r["tier"] == TIER_OPAQUE:
                assert r["n_elts"] == -1 and r["ids"] == "none", key(r)
            elif r["tier"] == TIER_TYPED:
                assert r["hits"] and r["ids"] == "none", key(r)
            else:
                assert r["ids"] == "none" and not r["hits"], key(r)


class _GrewAName:
    """一个「实例却带 `__name__`」的对象：pytest 会走 `__name__` 分支去渲染它

    本判据的自证用的就是它 —— 它不依赖任何解释器版本，所以能在两版上给出同一个期望答案。
    """

    __name__ = "grown"


class TestTheDynamicRulerProvesItself:
    """分类器自证：什么算危险、什么不算，得在版本无关的样本上先说清楚"""

    def test_authored_names_are_not_hazards(self):
        for val in (int, str, _GrewAName, hazardous, Path("x").absolute, TESTS_ROOT.joinpath):
            assert hazardous(val) is None, val

    def test_an_instance_that_grew_a_name_is_a_hazard(self):
        assert hazardous(_GrewAName()) == f"{__name__}._GrewAName:grown"

    def test_an_ordinary_instance_is_not_a_hazard(self):
        for val in (object(), 1, "s", (1, 2), ["a"], {"k": "v"}, 3.5, None, b"x"):
            assert hazardous(val) is None, val

    def test_the_hazard_branch_is_really_the_pytest_branch(self):
        """判据必须贴着 `_pytest.python` 的实现，而不是贴着我的想象

        取 `_idval_from_value` 的源码，确认它确实有「试 `__name__`」那一支；没有就说明 pytest
        换了机制，本判据的根据不再成立 ⇒ 抛，而不是继续给一个已经失去根据的读数。
        """
        import inspect
        import _pytest.python as py_mod

        src = inspect.getsource(py_mod)
        assert '"__name__"' in src, "pytest 的 id 机制里找不到 `__name__` 那一支：判据要跟版本重定"


class TestDynamicIdMakerFace:
    """动态面：读 pytest 真正拿去造 id 的对象，不看表达式形状"""

    def test_hazard_roster_is_pinned_per_interpreter(self, collected):
        """同一份源码在两版解释器上是两种对象 ⇒ 两版读数分别钉死，未知版本直接抛"""
        version = sys.version_info[:2]
        if version not in MEASURED_HAZARD_ROSTER:
            raise RulerNotProven(f"解释器 {version} 没有现量读数：这张表要按新解释器重量，不许猜")
        assert dynamic_risky(collected, set()) == MEASURED_HAZARD_ROSTER[version]

    def test_the_two_interpreters_do_not_share_the_same_roster_text(self):
        """这张表本身就是 L96 那一格的机制化记忆：两版读数不同是**记录下来的事实**

        3.13 上它是 `typing._UnionGenericAlias:Union`，3.14 上它是 `typing.Union:Union`，
        而 PEP 604 的 `int | str` 在 3.13 上根本没有 `__name__` ⇒ 同一行源码在两版上长出的
        是不同类型名的对象。两版读数一模一样反而说明有一把尺子失效了。
        """
        assert MEASURED_HAZARD_ROSTER[(3, 13)] != MEASURED_HAZARD_ROSTER[(3, 14)]

    def test_no_param_is_rendered_from_an_unauthored_name(self, collected, rows):
        """判据本体：字面 ids 挡住的格之外，风险集必须为空（两版同一口径）"""
        assert dynamic_risky(collected, exempt_sites(rows)) == set()

    def test_the_dynamic_face_sees_every_blind_site_by_name(self, collected, rows):
        """动态面必须真覆盖到静态盲面，否则上一条的绿是「没采到」而不是「判过」"""
        blind_tests = {f"{r['file']}::{r['test']}" for r in blind_rows(rows)}
        seen = {item_site(i) for i in collected if getattr(i, "callspec", None) is not None}
        assert len(blind_tests) == MEASURED_BLIND_TESTS, len(blind_tests)
        assert len(blind_tests & seen) == MEASURED_BLIND_SEEN
        assert len(exempt_sites(rows)) == MEASURED_EXEMPT_SITES

    def test_the_l96_union_site_is_blocked_by_literal_ids_not_by_luck(self, collected, rows):
        """L96 那支联合值用例：动态面要**真的**看到它的风险对象，并且确认挡住它的是字面 ids"""
        site = "unit/test_schema_type_spec_l96.py::test_label"
        typed = [r for r in rows if f"{r['file']}::{r['test']}" == site]
        assert len(typed) == 1 and typed[0]["ids"] == TIER_IDS_LITERAL
        vals = [v for it in collected if item_site(it) == site
                for v in it.callspec.params.values()]
        risky = [hazardous(v) for v in vals]
        assert any(risky), f"这一跑没看到动态面判为危险的对象：{[type(v).__name__ for v in vals]}"
        assert site in exempt_sites(rows)


class TestTwoRulersAgree:
    """两把尺子对同一问题的读数必须咬合：动态发现危险对象的那些格，静态必须已经归进「写了字面 ids」"""

    def test_static_violations_are_empty(self):
        assert violations(param_sites(TESTS_ROOT)) == set()

    def test_dynamic_risk_is_explained_by_literal_ids_sites(self, collected, rows):
        """互证的那一格：动态读出危险对象的每一支用例，静态侧必须能看到它写了字面 ids

        这一条是**不豁免**地跑动态面的 ⇒ 若动态在某格上发现非本仓带名对象，而静态看不出那格写了
        字面 ids，就是其中一把尺子在说谎（而不是那格恰好被放过）。
        """
        risky_sites = {s.split("<-")[0] for s in dynamic_risky(collected, set())}
        literal_any = {f"{r['file']}::{r['test']}" for r in rows if r["ids"] == TIER_IDS_LITERAL}
        assert risky_sites <= literal_any, sorted(risky_sites - literal_any)


class TestInjectionQualifiesBlindSites:
    """反向证据：三种「本该被点名」的形状写进临时树，尺子必须当场读出对应档位

    没有这一族，「`MEASURED_TYPED_SITES` 为空」和「盲面里 0 格写了字面 ids」这两格都可能只是
    尺子看不见，而不是判过（A171 的零形状：空集最容易读成成功）。
    """

    def _row(self, tmp_path, src: str, name: str = "injected") -> dict:
        (tmp_path / "injected.py").write_text(src, encoding="utf-8")
        got = [r for r in static_rows(tmp_path) if r["test"] == name]
        assert len(got) == 1, f"注入的用例读到 {len(got)} 份，键形或解析漂了"
        return got[0]

    def test_blind_site_with_literal_ids_moves_to_the_exemption_tier(self, tmp_path):
        row = self._row(
            tmp_path,
            "import pytest\nV = [1, 2]\n\n\n"
            '@pytest.mark.parametrize("v", V, ids=["one", "two"])\n'
            "def injected(v):\n    pass\n",
        )
        assert row["tier"] == TIER_IDS_LITERAL
        assert row["literal"] is False, "值仍来自变量：豁免只可能来自 ids 写法"

    def test_blind_site_resolving_to_a_union_is_named_not_counted_clean(self, tmp_path):
        row = self._row(
            tmp_path,
            "import pytest\nT = [int | str]\n\n\n"
            '@pytest.mark.parametrize("t", T)\n'
            "def injected(t):\n    pass\n",
        )
        assert row["tier"] == TIER_TYPED
        assert row["hits"] == ("union",)

    def test_derived_ids_never_becomes_an_exemption(self, tmp_path):
        row = self._row(
            tmp_path,
            "import pytest\nV = [1, 2]\n\n\n"
            '@pytest.mark.parametrize("v", V, ids=sorted(V))\n'
            "def injected(v):\n    pass\n",
        )
        assert row["tier"] == TIER_IDS_DERIVED
        assert "injected.py::injected" not in exempt_sites([row])
