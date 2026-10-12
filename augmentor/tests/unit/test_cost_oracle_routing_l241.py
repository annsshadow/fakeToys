# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L241 / §3.34：按代价选路的预言机家族守卫（文档同名单 + 判据纯整数 + 真的在分流）

立账出处：`Deduplicator` 与 `LeakageDetector` 各有一条「同一件事、两种算法」的岔路，选路判据
住在 `augmentor/dedup.py` 的 `_inverted_blocks_win` 与 `augmentor/leakage.py` 的
`_dense_route_wins` 里，架构文档 §3.34 是唯一面向读者的说明。本轮把「那段说明还成立吗」变成
可失败用例——判据本体不量墙钟、不看负载，只数结构与名字（承 A286 那条「性能判据一律计数」纪律）。

**五组判据**：

1. **同名单，两个方向**：文档 §3.34 里反引号点名的预言机名字 == AST 从产品码实推的集合。
   新增一条路线而忘了写文档 ⇒ 红；文档写了一个不存在的名字 ⇒ 同样红。所以这里刻意用
   **精确相等**而不是 L76 / L77 那套「只降不升」棘轮：棘轮恰好把「新增却不写文档」这一半方向放过。
2. **三条独立通道互相咬合**：名字形状（`_win` / `_wins` 结尾）、「比较式里用到 `*_COST` 单价常数」、
   「声明返回 `bool` + 比较式有一侧是乘法 + 在同模块被 `if` 的 test 位消费」各自实推一次，三边必须给出
   同一个集合。抄一份字面清单当真相是禁止的——那份清单只会跟着代码腐坏。
3. **判据本体是纯整数**：函数体内**零个浮点常数、零次调用、零个属性访问**。这一条把「选路这一步
   自己不碰时钟、也不为选路真跑一遍实现」从散文变成结构断言；放行属性访问等于让它能写
   `time.monotonic()`，那条纪律当场退化成一句空话。
4. **真的在分流**：每支预言机在**自己的模块**里被调用、至少一处调用点直接坐在某条 `if` 的 test 位，
   且承载它的那个函数同时调用 `MEASURED_PRODUCERS` 里点名的两侧生产者。**判负的一侧不许变装饰品**。
5. **每条自动选路都要有把它焊死的用例**：tests 树里必须存在把该预言机写进 `setattr` / `patch`
   目标位的用例（现量 dedup 在 `test_dedup.py`、leakage 在 `test_leakage_dense_route_l240.py`
   与 `test_leakage_counts_l95.py` 各有一处）。L240 那条「加了第二条路线之后，钉第一条路线的尺子
   必须点名是哪一条」只有配上这一条才闭环：能焊死，才谈得上逐条同判。

**边界（与 A129 同类的残余，如实声明）**：一支三条通道都不占的新预言机仍然对本守卫失明。第三通道
是按「会不会误伤」实测之后才加进来的：全包现量只命中那两支、零误伤，所以它不是一把会自己变红的宽尺子。
本守卫抓的是「改名/新增而漏文档」与「文档说了一件事而代码没做」这两类已经发生过的形状。
"""
import ast
import functools
import pathlib
import re

import pytest

ROOT = pathlib.Path(__file__).resolve().parents[2]
PKG = ROOT / "augmentor"
TESTS = ROOT / "tests"
ARCH = (ROOT / "docs" / "ARCHITECTURE.md").relative_to(ROOT).as_posix()

#: 预言机的名字形状：dedup 那支是单数 `_win`，leakage 那支是复数 `_wins`
NAME_SHAPE = re.compile(r"^_\w*_wins?$")
#: 第二条通道：比较式里出现的单价常数后缀
COST_SUFFIX = "_COST"
#: 架构文档里那节说明的标题前缀
SECTION = "### 3.34"
INLINE_CODE = re.compile(r"`([^`\n]+)`")

#: 每支预言机两侧的路线生产者（键必须与实推集合相等，见 `test_producer_table_...`）
MEASURED_PRODUCERS = {
    "_inverted_blocks_win": ("_batch_encode", "_iter_inverted_similarity_row_blocks"),
    "_dense_route_wins": ("_inverted_flags", "_dense_flags"),
}

#: 现量：三条通道与文档点名各自独立给出的集合大小（反空转下界，不是缺陷计数）
MIN_ORACLES = 2


def _bodies(tree):
    """AST 里所有函数/方法定义节点（类内的也算，dedup 那支就是静态方法）"""
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            yield node


def _stmt_nodes(node):
    """只走函数体（装饰器与形参标注不算判据本体）"""
    for stmt in node.body:
        yield from ast.walk(stmt)


def oracles_by_name(tree) -> set:
    """通道一：名字形状"""
    return {n.name for n in _bodies(tree) if NAME_SHAPE.match(n.name)}


def oracles_by_cost(tree) -> set:
    """通道二：函数体里有一条比较，其某一侧用到 `*_COST` 常数"""
    out = set()
    for node in _bodies(tree):
        for sub in ast.walk(node):
            if isinstance(sub, ast.Compare) and any(
                isinstance(x, ast.Name) and x.id.endswith(COST_SUFFIX)
                for x in ast.walk(sub)
            ):
                out.add(node.name)
                break
    return out


def _has_mult_compare(node) -> bool:
    """比较式的两侧里至少有一处是乘法（代价账的形状：次数 × 单价 vs 格数）"""
    for sub in ast.walk(node):
        if isinstance(sub, ast.Compare):
            for side in [sub.left, *sub.comparators]:
                if any(isinstance(v, ast.BinOp) and isinstance(v.op, ast.Mult)
                       for v in ast.walk(side)):
                    return True
    return False


def oracles_by_shape(tree) -> set:
    """通道三：声明返回 `bool` + 比较式有一侧是乘法 + 在同模块被 `if` 的 test 位消费

    这条是按「会不会误伤」实测之后才采用的：全包现量只命中那两支预言机、零误伤，
    所以它既补上「改名而漏两条通道」的那一类，也不是一把会自己变红的宽尺子。
    """
    consumed = set(if_tests(tree))
    out = set()
    for node in _bodies(tree):
        if node.name not in consumed or node.returns is None:
            continue
        if ast.unparse(node.returns) != "bool" or not _has_mult_compare(node):
            continue
        out.add(node.name)
    return out


def called_names(tree) -> set:
    return {c.func.id for c in ast.walk(tree) if isinstance(c, ast.Call)
            and isinstance(c.func, ast.Name)} | {
            c.func.attr for c in ast.walk(tree) if isinstance(c, ast.Call)
            and isinstance(c.func, ast.Attribute)}


def if_tests(tree):
    """每处直接坐在 `if` 的 test 位上的调用名"""
    for node in ast.walk(tree):
        if isinstance(node, ast.If) and isinstance(node.test, ast.Call):
            func = node.test.func
            yield func.id if isinstance(func, ast.Name) else func.attr


def product_oracles():
    """实推：名字 -> 定义它的模块（仓根相对 posix 路径），三条通道的并集"""
    where = {}
    for path in sorted(PKG.rglob("*.py")):
        tree = ast.parse(path.read_text(encoding="utf-8"))
        for name in oracles_by_name(tree) | oracles_by_cost(tree) | oracles_by_shape(tree):
            where.setdefault(name, set()).add(path.relative_to(ROOT).as_posix())
    return where


@functools.lru_cache(maxsize=1)
def patch_targets() -> dict:
    """tests 树里被 `setattr` / `patch` 点名的名字 -> 出处文件（「把选路焊死」的接缝）"""
    out: dict = {}
    for path in sorted(TESTS.rglob("*.py")):
        if "__pycache__" in path.parts:
            continue
        rel = path.relative_to(ROOT).as_posix()
        tree = ast.parse(path.read_text(encoding="utf-8", errors="replace"))
        for name in _patched_names(tree):
            out.setdefault(name, set()).add(rel)
    return out


def _patched_names(tree) -> set:
    out = set()
    for node in ast.walk(tree):
        if not isinstance(node, ast.Call):
            continue
        func = node.func
        called = func.id if isinstance(func, ast.Name) else getattr(func, "attr", "")
        if called not in {"setattr", "patch"}:
            continue
        for arg in [*node.args, *[kw.value for kw in node.keywords]]:
            if isinstance(arg, ast.Constant) and isinstance(arg.value, str):
                out.add(arg.value)
                out.add(arg.value.rsplit(".", 1)[-1])
    return out


@functools.lru_cache(maxsize=1)
def module_trees():
    """模块路径 -> AST，供结构与纯度判据复用（一次全解析，避免每支用例重跑）"""
    return {
        path.relative_to(ROOT).as_posix(): ast.parse(path.read_text(encoding="utf-8"))
        for path in sorted(PKG.rglob("*.py"))
    }


def arch_text() -> str:
    return (ROOT / ARCH).read_text(encoding="utf-8")


def section_body(text: str) -> str:
    """取 §3.34 那一节的正文：从该标题行到下一个 `### ` 或 `## ` 标题"""
    lines = text.replace("\r\n", "\n").split("\n")
    start = [i for i, line in enumerate(lines) if line.startswith(SECTION)]
    assert len(start) == 1, f"{SECTION} 标题在 {ARCH} 应当恰好一处，现量 {len(start)} 处"
    rest = lines[start[0] + 1:]
    stop = [i for i, line in enumerate(rest)
            if line.startswith("### ") or line.startswith("## ")]
    return "\n".join(rest[:stop[0]] if stop else rest)


def doc_names(body: str) -> set:
    """文档点名：节内反引号里、形状是预言机名字的那些 token"""
    return {tok.strip() for tok in INLINE_CODE.findall(body) if NAME_SHAPE.match(tok.strip())}


def doc_paths(body: str) -> set:
    return {tok.strip() for tok in INLINE_CODE.findall(body) if tok.strip().endswith(".py")}


class TestSameRosterBothDirections:
    """判据 1 与 2：名单与代码同名字，且三条独立通道互相咬合"""

    def test_doc_roster_equals_ast_roster(self):
        body = section_body(arch_text())
        documented = doc_names(body)
        derived = set(product_oracles())
        assert derived, "AST 在产品树里一支预言机都没找到——尺子在空转"
        assert len(derived) >= MIN_ORACLES, f"现量 {len(derived)} 支，低于反空转下界 {MIN_ORACLES}"
        assert documented == derived, (
            f"文档 §3.34 点名 {sorted(documented)} 与 AST 实推 {sorted(derived)} 不一致："
            f"只在文档 {sorted(documented - derived)}，只在代码 {sorted(derived - documented)}")

    def test_three_channels_agree(self):
        by_name, by_cost, by_shape = set(), set(), set()
        for tree in module_trees().values():
            by_name |= oracles_by_name(tree)
            by_cost |= oracles_by_cost(tree)
            by_shape |= oracles_by_shape(tree)
        assert by_name == by_cost, (
            f"通道一与通道二给出不同集合：名字形状 {sorted(by_name)}，代价常数 {sorted(by_cost)}")
        assert by_name == by_shape, (
            f"通道一与通道三给出不同集合：名字形状 {sorted(by_name)}，"
            f"`-> bool` + 乘法比较 + if-test 消费 {sorted(by_shape)}")

    def test_module_named_in_the_doc(self):
        body = section_body(arch_text())
        paths = doc_paths(body)
        for name, modules in sorted(product_oracles().items()):
            assert modules <= paths, f"{name} 定义在 {sorted(modules)}，文档那节没点名这一模块"

    def test_producer_table_is_not_a_second_roster(self):
        assert set(MEASURED_PRODUCERS) == set(product_oracles()), (
            "两侧生产者表的键与实推预言机集合不等——那张表要么过期，要么漏了新路线")

    def test_every_auto_route_has_a_welded_case(self):
        """判据 5：每条自动选路都能在 tests 树里找到把它焊死的用例"""
        targets = patch_targets()
        for oracle in sorted(product_oracles()):
            files = targets.get(oracle)
            assert files, (
                f"{oracle} 没有任何用例把它写进 setattr / patch 的目标位：一旦加了第二条路线，"
                f"就没人能钉住「这一支走的是哪条路」")


class TestJudgeBodyIsPureInteger:
    """判据 3：函数体内零浮点、零调用、零属性访问（选路不碰时钟，也不为选路跑实现）"""

    @pytest.mark.parametrize("oracle", ["_inverted_blocks_win", "_dense_route_wins"],
                             ids=["dedup", "leakage"])
    def test_no_float_no_call_no_attribute(self, oracle):
        node = self._find(oracle)
        floats = [x.value for x in _stmt_nodes(node)
                  if isinstance(x, ast.Constant) and isinstance(x.value, float)]
        attrs = [ast.unparse(x) for x in _stmt_nodes(node) if isinstance(x, ast.Attribute)]
        calls = [ast.unparse(x.func) for x in _stmt_nodes(node) if isinstance(x, ast.Call)]
        assert floats == [] and attrs == [] and calls == [], (
            f"{oracle} 不再是纯整数判据：浮点 {floats} 属性 {attrs} 调用 {calls}")

    @staticmethod
    def _find(oracle):
        for tree in module_trees().values():
            for node in _bodies(tree):
                if node.name == oracle:
                    return node
        raise AssertionError(f"产品树里没有 {oracle}")


class TestRoutingIsReal:
    """判据 4：预言机真在分流，而且两侧生产者都住在同一个函数里"""

    def _callers(self, oracle):
        """(模块, 承载它的函数节点) —— 直接给出节点，不按名字回找，免得同名误伤"""
        out = []
        for mod, tree in module_trees().items():
            for node in _bodies(tree):
                for sub in ast.walk(node):
                    if not isinstance(sub, ast.Call):
                        continue
                    func = sub.func
                    name = func.id if isinstance(func, ast.Name) else getattr(func, "attr", None)
                    if name == oracle:
                        out.append((mod, node))
        return out

    def test_called_from_its_own_module(self):
        for oracle, modules in sorted(product_oracles().items()):
            calls = self._callers(oracle)
            assert calls, f"{oracle} 一个调用点都没有：它已经是死代码，而文档还在描述它"
            assert {mod for mod, _ in calls} == modules, (
                f"{oracle} 的定义模块 {sorted(modules)} 与调用模块不符")

    def test_at_least_one_call_sits_in_an_if_test(self):
        for oracle in sorted(product_oracles()):
            tests = set()
            for tree in module_trees().values():
                tests |= set(if_tests(tree))
            assert oracle in tests, (
                f"{oracle} 的调用点没有一个直接坐在 if 的 test 位——现量两处都是 "
                f"`if <oracle>(...)`；改成先存变量再判断也要同步本判据")

    def test_both_route_producers_live_in_the_calling_function(self):
        for oracle, producers in sorted(MEASURED_PRODUCERS.items()):
            hosts = {(mod, node) for mod, node in self._callers(oracle)}
            assert hosts, f"{oracle} 没有承载函数"
            for mod, node in sorted(hosts, key=lambda x: (x[0], x[1].lineno)):
                inside = called_names(node)
                missing = [p for p in producers if p not in inside]
                assert not missing, (
                    f"{mod} 的 {node.name} 调用 {oracle} 却不再调用 {missing}："
                    f"选路的一侧变成了装饰品")


class TestRulerIsProven:
    """尺子自证：每判据各配一支合成注入（全在 AST/文本上做，不落任何仓库文件）"""

    def test_extra_route_oracle_would_be_found_by_both_channels(self):
        tree = ast.parse("import time\n\n\ndef _third_route_wins(a, b):\n"
                        "    return a * _OTHER_COST >= b\n")
        assert oracles_by_name(tree) == {"_third_route_wins"}
        assert oracles_by_cost(tree) == {"_third_route_wins"}

    def test_cost_channel_also_sees_a_renamed_oracle(self):
        tree = ast.parse("def pick(a, b):\n    return a * _OTHER_COST >= b\n")
        assert oracles_by_cost(tree) == {"pick"}
        assert oracles_by_name(tree) == set()

    def test_shape_channel_sees_what_the_other_two_miss(self):
        """通道三的反空转臂：既不改名、又不用单价常数的选路判据仍被结构抓住"""
        tree = ast.parse(
            "def route(a, b) -> bool:\n    return a * 7 >= b * 3\n"
            "\n\ndef use(a, b):\n    if route(a, b):\n        return 1\n    return 2\n")
        assert oracles_by_shape(tree) == {"route"}
        assert oracles_by_name(tree) == set()
        assert oracles_by_cost(tree) == set()

    def test_shape_channel_does_not_fire_on_a_plain_predicate(self):
        """通道三的收窄臂：没有乘法、没被 if-test 消费、没标 bool 的普通函数不算预言机"""
        tree = ast.parse(
            "def _plain_wins(a, b):\n    return a > b\n"
            "\n\ndef other(a, b) -> bool:\n    return a > b\n"
            "\n\ndef use(a, b):\n    if other(a, b):\n        return 1\n    return 2\n")
        assert oracles_by_shape(tree) == set()

    def test_singular_name_shape_is_matched(self):
        # 反向臂：形状不许太窄——dedup 那支用的是单数 `_win`
        assert NAME_SHAPE.match("_inverted_blocks_win")
        assert NAME_SHAPE.match("_dense_route_wins")
        assert not NAME_SHAPE.match("detect")

    def test_ghost_name_in_the_doc_would_be_collected(self):
        fake = "### 3.34 标题\n\n- `augmentor/dedup.py` 的 `_ghost_wins`\n\n## 4. 下一节\n"
        assert doc_names(fake) == {"_ghost_wins"}
        assert doc_paths(fake) == {"augmentor/dedup.py"}

    def test_section_slice_stops_at_the_next_heading(self):
        fake = SECTION + " 选路\n\n甲\n\n### 3.35 别的\n\n乙\n\n## 4. 还别的\n\n丙\n"
        body = section_body(fake)
        assert "甲" in body and "乙" not in body and "丙" not in body

    def test_wall_clock_attribute_is_not_pure(self):
        node = ast.parse("def f(a):\n    import time\n    return a > time.monotonic()\n").body[0]
        assert [ast.unparse(x) for x in _stmt_nodes(node) if isinstance(x, ast.Attribute)]

    def test_float_threshold_is_not_pure(self):
        node = ast.parse("def f(a):\n    return a * 0.5 >= 1\n").body[0]
        assert [x.value for x in _stmt_nodes(node)
                if isinstance(x, ast.Constant) and isinstance(x.value, float)]

    def test_uncalled_oracle_is_reported_as_dead(self):
        tree = ast.parse("def _ghost_wins(a, b):\n    return a > b * _X_COST\n")
        assert oracles_by_name(tree) == {"_ghost_wins"}
        assert list(if_tests(tree)) == []

    def test_assignment_then_if_is_not_an_if_test_call(self):
        tree = ast.parse("def g(a, b):\n    pick = _dense_route_wins(a, b)\n    if pick:\n"
                         "        return 1\n    return 2\n")
        assert any(isinstance(n, ast.If) for n in ast.walk(tree)), "注入样本本身没长出 if"
        assert set(if_tests(tree)) == set()

    def test_missing_producer_is_reported(self):
        tree = ast.parse("def h(items):\n    if _dense_route_wins(1, 2, 3, 4):\n"
                         "        return _dense_flags(items)\n    return items\n")
        host = [n for n in _bodies(tree) if n.name == "h"][0]
        assert "_inverted_flags" not in called_names(host)

    def test_patch_scanner_reads_attribute_and_dotted_targets(self):
        tree = ast.parse(
            "mp.setattr(Mod, \"_dense_route_wins\", lambda *a: True)\n"
            "patch(\"augmentor.dedup._ghost_wins\", return_value=False)\n"
            "mp.setattr(x, \"not_a_route\", 1)\n")
        names = _patched_names(tree)
        assert {"_dense_route_wins", "_ghost_wins", "not_a_route"} <= names
        assert "augmentor.dedup._ghost_wins" in names, "整串点号目标位也要收，否则同串两种写法会漏"

    def test_patch_scanner_ignores_a_plain_string_assert(self):
        tree = ast.parse("def t():\n    assert got == \"_dense_route_wins\"\n")
        assert _patched_names(tree) == set()
