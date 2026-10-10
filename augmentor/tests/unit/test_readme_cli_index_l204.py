# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier:AGPL-3.0-or-later

"""L204：README 的 CLI 命令索引必须与 parser 逐条相等

**为什么有这一支**：实测 README 只以 `python cli.py <name>` 的形式覆盖了 **21/37**
个子命令，更糟的是**「功能特性」里承诺的三个功能**（隐私脱敏 / 泄漏检测 / 就绪审计）
在 CLI 一节**一个示例都没有** —— 承诺在 README 里、入口却查不到。这类缺口不会让任何
测试变红：docs 守卫只管形状与行引用，契约守卫只管 HTTP 端点。

本轮把 37 条命令全列成一张表（help 文案由 `augmentor/cli/parser.py` 现提，不手抄），
本守卫把「表 == parser」钉成集合相等：新增子命令忘了更新 README ⇒ 当场红；
README 里留着 parser 已删掉的命令 ⇒ 同样红。
"""

import re
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
README = ROOT / "README.md"
PARSER_SRC = ROOT / "augmentor" / "cli" / "parser.py"

#: `add_parser("name", help="…")` —— help 可能跨行，故用 \s* 吃掉换行
ADD_PARSER = re.compile(
    r'add_parser\(\s*"([a-z0-9\-]+)"\s*,\s*(?:help\s*=\s*)?"((?:[^"\\]|\\.)*)"',
    re.S)
#: README 表行：| `name` | help | （可选 ✅） |
TABLE_ROW = re.compile(r"^\|\s*`([a-z0-9\-]+)`\s*\|([^|]*)\|\s*(✅?)\s*\|\s*$")

#: 「功能特性」承诺过、CLI 一节必须有示例的三个命令
PROMISED = ("sanitize", "check-leakage", "audit")


def parser_commands():
    src = PARSER_SRC.read_text(encoding="utf-8")
    rows = [(m.group(1), m.group(2).replace('\\"', '"').strip())
            for m in ADD_PARSER.finditer(src)]
    names = [n for n, _ in rows]
    assert names, "parser.py 里一个 add_parser 都没解析到 ⇒ 正则坏了"
    assert len(set(names)) == len(names), f"子命令名重复: {sorted(names)}"
    return dict(rows)


def readme_rows():
    text = README.read_text(encoding="utf-8")
    rows = {}
    for line in text.splitlines():
        m = TABLE_ROW.match(line)
        if m:
            rows[m.group(1)] = (m.group(2).strip(), bool(m.group(3)))
    return rows


class TestTheIndexIsComplete:
    """表必须与 parser 逐条相等——两个方向都不许多或少"""

    def test_every_command_is_listed(self):
        missing = sorted(set(parser_commands()) - set(readme_rows()))
        assert not missing, (
            f"这些子命令 README 里没有: {missing} —— 功能加了、文档没跟上，"
            "而这是唯一不会让任何测试变红的缺口类型"
        )

    def test_no_stale_commands_are_listed(self):
        stale = sorted(set(readme_rows()) - set(parser_commands()))
        assert not stale, (
            f"README 里这些命令 parser 已经没有: {stale}"
        )

    def test_the_two_sets_are_exactly_equal(self):
        assert set(readme_rows()) == set(parser_commands())

    def test_help_text_matches_the_parser(self):
        """help 文案也是从 parser 现提的，不许手抄后漂掉"""
        parser = parser_commands()
        rows = readme_rows()
        drifted = [
            f"{name}: README {rows[name][0]!r} / parser {parser[name]!r}"
            for name in sorted(parser)
            if name in rows and rows[name][0] != parser[name]
        ]
        assert not drifted, "README 的 help 文案与 parser 不一致:\n  " + "\n  ".join(drifted)


class TestPromisedFeaturesHaveAnEntryPoint:
    """「功能特性」承诺过的三个功能，CLI 一节必须有示例"""

    @pytest.mark.parametrize("command", PROMISED)
    def test_command_has_a_runnable_example(self, command):
        text = README.read_text(encoding="utf-8")
        assert f"cli.py {command}" in text, (
            f"README 的 CLI 一节没有 `{command}` 的示例，而「功能特性」里承诺了对应能力"
        )

    @pytest.mark.parametrize("command", PROMISED)
    def test_command_is_in_the_index(self, command):
        assert command in readme_rows()

    def test_the_examples_use_real_flags(self):
        """示例里的旗标必须真的被该子命令接受（防止文档教一个不存在的参数）"""
        import sys
        sys.path.insert(0, str(ROOT))
        from augmentor.cli.parser import build_parser

        parser = build_parser()
        sub = None
        for action in parser._actions:
            choices = getattr(action, "choices", None)
            if isinstance(choices, dict) and choices and all(
                    hasattr(v, "format_usage") for v in choices.values()):
                sub = choices
                break
        assert sub is not None, "build_parser() 里找不到子命令解析器"

        text = README.read_text(encoding="utf-8")
        for command in PROMISED:
            for line in text.splitlines():
                stripped = line.strip()
                if not stripped.startswith("python cli.py " + command):
                    continue
                argv = stripped.split()[3:]                 # 去掉 python cli.py <cmd>
                argv = [a for a in argv if not a.startswith("$")
                        and not a.startswith("#")]
                try:
                    sub[command].parse_args(argv)
                except SystemExit as exc:                  # noqa: F841
                    pytest.fail(
                        f"README 示例解析不了：{stripped!r}"
                    )


class TestTheRulerSeesSomething:
    """防空转：两边都必须真的取到东西"""

    def test_census_is_not_empty(self):
        assert len(parser_commands()) >= 30, len(parser_commands())
        assert len(readme_rows()) >= 30, len(readme_rows())
