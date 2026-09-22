"""测试文件分诊：找出占位测试与空洞断言

用途：为 docs/plans/2026-09-21-001 的 T1.1 / T1.2 提供机器判据，
避免"凭感觉删测试"。

核心判据：一条 assert 若其真值**无需执行任何被测代码即可确定**，它就是空洞的
（永远为真 → 业务逻辑变化时不可能失败 → 不是测试）。实现方式是函数内的
常量传播：先把所有 `name = <字面量表达式>` 收集为常量环境，再看断言表达式
是否只由字面量与这些常量构成。

能识别的典型空洞断言：
    assert True
    assert 94.17 >= 80.0                      # 两个字面量互比
    assert "2.1.0" in "fakeToys ai v2.1.0"    # 字符串包含自身
    current_round = 62
    assert current_round <= 100               # 局部字面量的恒真比较
    total = 100
    assert total == 100

只读脚本，不修改任何文件。
"""

import ast
import json
import re
import sys
from pathlib import Path

TESTS_DIR = Path(__file__).resolve().parent.parent / "tests"


class _Unknown:
    """常量传播中表示"无法确定"的哨兵"""

    def __repr__(self):
        return "UNKNOWN"


UNKNOWN = _Unknown()


def _eval_const(node: ast.AST, consts: dict):
    """在常量环境下求值表达式；无法确定时返回 UNKNOWN

    Args:
        node: AST 节点
        consts: 变量名 → 已知常量值

    Returns:
        求值结果或 UNKNOWN
    """
    if isinstance(node, ast.Constant):
        return node.value

    if isinstance(node, ast.Name):
        return consts.get(node.id, UNKNOWN)

    if isinstance(node, ast.List):
        vals = [_eval_const(e, consts) for e in node.elts]
        return UNKNOWN if any(v is UNKNOWN for v in vals) else vals

    if isinstance(node, ast.Tuple):
        vals = [_eval_const(e, consts) for e in node.elts]
        return UNKNOWN if any(v is UNKNOWN for v in vals) else tuple(vals)

    if isinstance(node, ast.Set):
        vals = [_eval_const(e, consts) for e in node.elts]
        return UNKNOWN if any(v is UNKNOWN for v in vals) else set(vals)

    if isinstance(node, ast.Dict):
        keys = [_eval_const(k, consts) for k in node.keys if k is not None]
        vals = [_eval_const(v, consts) for v in node.values]
        if any(k is UNKNOWN for k in keys) or any(v is UNKNOWN for v in vals):
            return UNKNOWN
        return dict(zip(keys, vals))

    if isinstance(node, ast.UnaryOp):
        operand = _eval_const(node.operand, consts)
        if operand is UNKNOWN:
            return UNKNOWN
        if isinstance(node.op, ast.USub):
            return -operand
        if isinstance(node.op, ast.UAdd):
            return +operand
        if isinstance(node.op, ast.Not):
            return not operand
        return UNKNOWN

    if isinstance(node, ast.BinOp):
        left = _eval_const(node.left, consts)
        right = _eval_const(node.right, consts)
        if left is UNKNOWN or right is UNKNOWN:
            return UNKNOWN
        try:
            if isinstance(node.op, ast.Add):
                return left + right
            if isinstance(node.op, ast.Sub):
                return left - right
            if isinstance(node.op, ast.Mult):
                return left * right
            if isinstance(node.op, ast.Div):
                return left / right
            if isinstance(node.op, ast.Mod):
                return left % right
        except Exception:
            return UNKNOWN
        return UNKNOWN

    if isinstance(node, ast.Compare):
        left = _eval_const(node.left, consts)
        if left is UNKNOWN:
            return UNKNOWN
        for op, comparator in zip(node.ops, node.comparators):
            right = _eval_const(comparator, consts)
            if right is UNKNOWN:
                return UNKNOWN
            try:
                if isinstance(op, ast.Eq):
                    ok = left == right
                elif isinstance(op, ast.NotEq):
                    ok = left != right
                elif isinstance(op, ast.Lt):
                    ok = left < right
                elif isinstance(op, ast.LtE):
                    ok = left <= right
                elif isinstance(op, ast.Gt):
                    ok = left > right
                elif isinstance(op, ast.GtE):
                    ok = left >= right
                elif isinstance(op, ast.In):
                    ok = left in right
                elif isinstance(op, ast.NotIn):
                    ok = left not in right
                elif isinstance(op, ast.Is):
                    ok = left is right
                elif isinstance(op, ast.IsNot):
                    ok = left is not right
                else:
                    return UNKNOWN
            except Exception:
                return UNKNOWN
            if not ok:
                return False
            left = right
        return True

    if isinstance(node, ast.BoolOp):
        vals = [_eval_const(v, consts) for v in node.values]
        if isinstance(node.op, ast.Or):
            # `assert X or True` 恒真——这是很常见的假测试写法，
            # 任意一个分支为常量 True 即可判定整式空洞，无需知道 X。
            if any(v is True for v in vals):
                return True
            if all(v is not UNKNOWN for v in vals):
                return any(vals)
            return UNKNOWN
        # And
        if any(v is False for v in vals):
            return False
        if all(v is not UNKNOWN for v in vals):
            return all(vals)
        return UNKNOWN

    return UNKNOWN


def _collect_consts(func: ast.AST) -> dict:
    """收集函数体内的常量绑定（严格模式）

    只有同时满足以下条件，名字才会被视为常量：

    1. 在整个函数（含嵌套函数）内**只被绑定一次**；
    2. 绑定形式是普通 `Assign`——不是 `AugAssign`、`for` 目标、`with ... as`、
       `except ... as` 或海象运算符；
    3. 没有被 `nonlocal` / `global` 声明；
    4. 没有作为可变方法的接收者（`x.append(...)` 等）；
    5. **没有作为参数传给任何调用**——被调用方可能原地修改它。

    条件 3~5 是保守的过度排除。早期版本只做"出现过 `name = <字面量>` 就当常量"，
    把 `call_count = 0`（后续被闭包 `+=`）、`missing = []`（后续被 `append`）误判为常量，
    进而把 `assert call_count == 1` 这类**有效断言**误报成空洞。宁可漏判不可误判。

    Args:
        func: 函数 AST 节点

    Returns:
        变量名 → 常量值
    """
    nodes = sorted(ast.walk(func), key=lambda n: getattr(n, "lineno", 0))

    # 可变方法的接收者：这些名字即使只赋值一次也不能当常量
    _MUTATORS = {
        "append", "extend", "insert", "remove", "pop", "clear", "sort",
        "reverse", "add", "discard", "update", "setdefault", "write",
        "close", "__setitem__", "__delitem__",
    }

    store_count: dict = {}
    tainted = set()

    for node in nodes:
        # 各类绑定形式都计入 store
        if isinstance(node, ast.Name) and isinstance(node.ctx, (ast.Store, ast.Del)):
            store_count[node.id] = store_count.get(node.id, 0) + 1
        if isinstance(node, ast.AugAssign) and isinstance(node.target, ast.Name):
            tainted.add(node.target.id)
        if isinstance(node, ast.For) and isinstance(node.target, ast.Name):
            tainted.add(node.target.id)
        if isinstance(node, ast.NamedExpr) and isinstance(node.target, ast.Name):
            tainted.add(node.target.id)
        if isinstance(node, (ast.Nonlocal, ast.Global)):
            tainted.update(node.names)
        if isinstance(node, ast.ExceptHandler) and node.name:
            tainted.add(node.name)
        if isinstance(node, ast.withitem) and isinstance(node.optional_vars, ast.Name):
            tainted.add(node.optional_vars.id)
        # x.append(...) 形式
        if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute):
            if node.func.attr in _MUTATORS and isinstance(node.func.value, ast.Name):
                tainted.add(node.func.value.id)
        # 作为参数传出（可能被原地修改）
        if isinstance(node, ast.Call):
            for arg in list(node.args) + [kw.value for kw in node.keywords]:
                if isinstance(arg, ast.Name):
                    tainted.add(arg.id)

    consts: dict = {}
    for node in nodes:
        if not isinstance(node, ast.Assign):
            continue
        if len(node.targets) != 1 or not isinstance(node.targets[0], ast.Name):
            continue
        name = node.targets[0].id
        if name in tainted or store_count.get(name, 0) != 1:
            continue
        value = _eval_const(node.value, consts)
        if value is not UNKNOWN:
            consts[name] = value

    return consts


def _is_vacuous(assert_node: ast.Assert, consts: dict) -> bool:
    """判断一条 assert 是否空洞（真值无需执行被测代码即可确定）"""
    return _eval_const(assert_node.test, consts) is not UNKNOWN


def _count_real_checks(func: ast.AST, consts: dict) -> int:
    """统计函数体内的有效检查点

    有效检查点包括：

    - 非空洞的 `assert`；
    - `pytest.raises` / `pytest.fail` / `pytest.warns`；
    - `np.testing.assert_*`（如 `assert_allclose`）与 `unittest` 的 `self.assert*`。

    Args:
        func: 函数 AST 节点
        consts: 常量环境

    Returns:
        有效检查点个数
    """
    count = 0
    for node in ast.walk(func):
        if isinstance(node, ast.Assert):
            if not _is_vacuous(node, consts):
                count += 1
        elif isinstance(node, ast.Call):
            name = ast.unparse(node.func)
            if name.endswith(("pytest.raises", "pytest.fail", "pytest.warns")):
                count += 1
            elif ".testing.assert_" in name or name.startswith("assert_"):
                count += 1
            elif re.search(r"\.assert[A-Z_]", name):
                count += 1
    return count


# 纯粹的测试辅助调用：出现在测试里不能说明"这条测试有可能失败"
_TRIVIAL_CALLS = {
    "print", "len", "range", "sorted", "list", "dict", "set", "tuple",
    "str", "int", "float", "bool", "enumerate", "zip", "sum", "min", "max",
    "isinstance", "hasattr", "getattr", "repr", "id", "type",
}


def can_fail(func: ast.AST, consts: dict) -> bool:
    """判断这条测试是否**有可能失败**

    一个测试若既没有有效检查点、也没有任何"未被 try 完整包裹的调用"，
    那么无论被测代码怎么改它都会通过——它不是测试，是装饰。

    这条判据比"必须有 assert"更准确：`writer.close()` 这种"不抛异常"测试
    是合法的（`close` 若开始抛异常，测试就会失败），不应被误报。

    Args:
        func: 函数 AST 节点
        consts: 常量环境

    Returns:
        是否有可能失败
    """
    if _count_real_checks(func, consts) > 0:
        return True

    # 被 try 块整体包裹的调用：异常会被 except 吞掉，因此不算"可能失败"
    guarded = set()
    for node in ast.walk(func):
        if isinstance(node, ast.Try):
            for stmt in node.body:
                for inner in ast.walk(stmt):
                    guarded.add(id(inner))

    for node in ast.walk(func):
        if not isinstance(node, ast.Call):
            continue
        if id(node) in guarded:
            continue
        if ast.unparse(node.func).split(".")[-1] in _TRIVIAL_CALLS:
            continue
        return True
    return False


def _is_test_function(node: ast.AST) -> bool:
    """判断节点是否为 pytest 会收集的测试函数

    pytest 只收集名为 `test_*` 的函数，但**带 `@pytest.fixture` 装饰的同名函数
    是 fixture 而非测试**（`tests/unit/test_dataset_ops.py::test_data` 就是这样），
    必须排除，否则会被误判成"不可能失败的空壳测试"。

    Args:
        node: AST 节点

    Returns:
        是否为真正的测试函数
    """
    if not isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
        return False
    if not node.name.startswith("test_"):
        return False
    for decorator in node.decorator_list:
        if "fixture" in ast.unparse(decorator):
            return False
    return True


def analyse_file(path: Path) -> dict:
    """分析单个测试文件

    Args:
        path: 测试文件路径

    Returns:
        含统计字段的字典
    """
    source = path.read_text(encoding="utf-8")
    try:
        tree = ast.parse(source)
    except SyntaxError as exc:
        return {"file": str(path), "error": f"SyntaxError: {exc}"}

    test_funcs = [
        node for node in ast.walk(tree) if _is_test_function(node)
    ]

    vacuous = 0
    real = 0
    details = []
    for fn in test_funcs:
        consts = _collect_consts(fn)
        fn_vacuous = sum(
            1
            for n in ast.walk(fn)
            if isinstance(n, ast.Assert) and _is_vacuous(n, consts)
        )
        fn_real = _count_real_checks(fn, consts)
        vacuous += fn_vacuous
        real += fn_real
        details.append({
            "name": fn.name,
            "lineno": fn.lineno,
            "real_checks": fn_real,
            "vacuous_asserts": fn_vacuous,
            "can_fail": can_fail(fn, consts),
        })

    imports = [ast.unparse(n) for n in ast.walk(tree)
               if isinstance(n, (ast.Import, ast.ImportFrom))]

    return {
        "file": str(path.relative_to(TESTS_DIR.parent)),
        "tests": len(test_funcs),
        "real_checks": real,
        "vacuous_asserts": vacuous,
        "lines": len(source.splitlines()),
        "imports": imports,
        # 整文件无任何有效检查点 → 占位文件
        "placeholder": len(test_funcs) > 0 and real == 0,
        "tests_detail": details,
    }


def analyse_all() -> list:
    """分析全部测试文件

    Returns:
        每个测试文件的统计字典列表
    """
    return [analyse_file(p) for p in sorted(TESTS_DIR.rglob("test_*.py"))]


def _empty_shell_tests(rows: list) -> list:
    """找出**不可能失败**的测试函数

    这类"空壳测试"与空洞断言同样有害：无论被测代码做什么它都会通过。
    典型写法是函数体只有一行 docstring、`pass`，或 `try/except: pass` 吞掉一切。

    注意不要用"没有 assert"当判据——`writer.close()` 这种"不抛异常"测试是合法的。

    Args:
        rows: analyse_all() 的输出

    Returns:
        "文件::测试名:行号" 字符串列表
    """
    return [
        f"{r['file']}::{d['name']}:{d['lineno']}"
        for r in rows
        if "error" not in r
        for d in r.get("tests_detail", [])
        if not d["can_fail"]
    ]


def main() -> int:
    rows = analyse_all()
    ok = [r for r in rows if "error" not in r]

    placeholders = [r for r in ok if r["placeholder"]]
    total_tests = sum(r["tests"] for r in ok)
    vacuous_total = sum(r["vacuous_asserts"] for r in ok)
    shells = _empty_shell_tests(rows)

    print("=" * 74)
    print(f"测试文件 {len(ok)} 个，测试函数 {total_tests} 个")
    print(f"占位文件（无任何有效检查点）: {len(placeholders)} 个，"
          f"含测试函数 {sum(r['tests'] for r in placeholders)} 个")
    print(f"空洞断言总数: {vacuous_total}")
    print(f"有效检查点总数: {sum(r['real_checks'] for r in ok)}")
    print(f"无有效检查点的单个测试: {len(shells)} 个")
    print("=" * 74)

    print("\n--- 占位文件明细 ---")
    for r in sorted(placeholders, key=lambda x: (-x["tests"], x["file"])):
        prod_imports = [i for i in r["imports"] if "augmentor" in i or "api" in i]
        print(f"  {r['tests']:3d} tests  {r['vacuous_asserts']:3d} vacuous  "
              f"{r['file']}"
              + (f"   [引入被测模块: {prod_imports}]" if prod_imports else ""))

    print("\n--- 部分空洞（有有效检查点，但含空洞断言）---")
    partial = [r for r in ok if not r["placeholder"] and r["vacuous_asserts"] > 0]
    for r in sorted(partial, key=lambda x: -x["vacuous_asserts"]):
        print(f"  {r['vacuous_asserts']:3d} vacuous / {r['real_checks']:3d} real / "
              f"{r['tests']:3d} tests  {r['file']}")

    if shells:
        print(f"\n--- 无有效检查点的单个测试（前 40 条，共 {len(shells)} 条）---")
        for s in shells[:40]:
            print(f"  {s}")

    if "--json" in sys.argv:
        out = Path(sys.argv[sys.argv.index("--json") + 1])
        out.write_text(json.dumps(rows, ensure_ascii=False, indent=2), encoding="utf-8")
        print(f"\n明细已写入 {out}")

    if "--check" in sys.argv:
        problems = len(placeholders) + vacuous_total + len(shells)
        if problems:
            print(f"\n[FAIL] 测试卫生门禁未通过：{problems} 处问题")
            return 1
        print("\n[OK] 测试卫生门禁通过：无占位文件、无空洞断言、无空壳测试")
    return 0


if __name__ == "__main__":
    sys.exit(main())
