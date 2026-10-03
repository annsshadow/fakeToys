# -*- coding: utf-8 -*-
"""L177 / B247：SDK 导出面死符号哨兵（`__all__` 逐条必须可核到消费方）。

L177 普查把 `__all__` 里 220 个导出符号逐个对全仓消费面扫了一遍，撞出 1 个
零消费方的死别名（`get_supported_export_formats`，已删）。本哨兵把普查**机器
化**成常驻守卫：`__all__` 的每条目必须在「产品代码（非 `__init__` 自身）或
测试面」有至少一处引用——将来再往 `__all__` 塞死符号当场红，而不是等下一
轮普查人工扫。

普查口径（与 L177 脚本同款，防止哨兵自己误报）：
- 产品面出现 = 非 import/定义行的 `符号` 词边界命中（`__init__` 的 import 行与
  `__all__` 清单行本身不算消费）；
- 测试面出现 = 非 import 行的命中；
- 两个面**至少一个**有命中才算活。
"""
import re
from pathlib import Path

_ROOT = Path(__file__).resolve().parents[2]
_PRODUCTS = (list(_ROOT.joinpath("augmentor").glob("*.py"))
             + list(_ROOT.joinpath("augmentor/cli").rglob("*.py"))
             + [p for p in _ROOT.joinpath("api").rglob("*.py")]
             + [_ROOT / "cli.py"])
_TESTS = list(_ROOT.joinpath("tests").rglob("*.py"))


def _all_symbols() -> list:
    import augmentor
    return list(augmentor.__all__)


def _corpus(files, exclude_init=False):
    parts = []
    for p in files:
        if exclude_init and p.name == "__init__.py":
            continue
        parts.append(p.read_text(encoding="utf-8", errors="replace"))
    return "\n".join(parts)


_PRODUCT_CORPUS = _corpus(_PRODUCTS, exclude_init=True)
_TEST_CORPUS = _corpus(_TESTS)
_PRODUCT_LINES = [l for l in _PRODUCT_CORPUS.splitlines()
                  if not (l.strip().startswith("from ") or l.strip().startswith("import "))
                  and not re.match(r"^['\"][\w]+['\"],?$", l.strip())]
_TEST_LINES = [l for l in _TEST_CORPUS.splitlines()
               if not (l.strip().startswith("from ") or l.strip().startswith("import "))]
_PRODUCT_LINE_TEXT = "\n".join(_PRODUCT_LINES)
_TEST_LINE_TEXT = "\n".join(_TEST_LINES)


def _refs(symbol: str) -> int:
    """产品面或测试面任一有非 import 行命中即算活（词边界匹配，语料库模块级缓存）。"""
    return (
        len(re.findall(r"\b" + re.escape(symbol) + r"\b", _PRODUCT_LINE_TEXT))
        + len(re.findall(r"\b" + re.escape(symbol) + r"\b", _TEST_LINE_TEXT))
    )


def test_every_all_symbol_has_a_consumer():
    """`__all__` 逐条有消费方（产品面或测试面）——死导出禁入库。"""
    dead = []
    for sym in _all_symbols():
        if _refs(sym) == 0:
            dead.append(sym)
    assert dead == [], (
        f"__all__ 里的 {dead} 全仓零消费方（产品代码与测试面都没有引用）——"
        "死导出按 L177 纪律删除或接消费方，二选一。"
    )


def test_the_dead_alias_stays_out():
    """`get_supported_export_formats` 不许以别名形态回流（L177 删的是别名，
    原版 `export_enhanced.get_supported_formats` 由测试面直接 import 保活）。"""
    import augmentor
    assert "get_supported_export_formats" not in augmentor.__all__
