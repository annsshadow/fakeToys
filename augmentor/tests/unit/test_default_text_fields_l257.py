# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A285 那一族「默认看哪三格」的权威对账（L257）

判的是一份字面量的**归属**，不是它的内容：改前 `["instruction", "input", "output"]`
在产品码里手写五遍——`PiiSanitizer` 与 `DatasetAuditor` 各一遍，CLI 的 `clean` /
`sanitize` / `audit` 三条 `--fields` 各一遍。现在权威住在 augmentor/privacy.py 的
模块常数 `DEFAULT_TEXT_FIELDS`，五处按各自面的既有口径就位：

- **SDK 两处真引用**：`privacy.py` 用得上（同模块），`audit.py` 把名字并进已有的
  `from .privacy import` 那一行（同行改写，净零行）。本文件第 1 面验「两个绑定是同一个
  对象」，抄第二份的写法在这一面必红。
- **CLI 三处保持字面量、由对账钉住**：`cli/parser.py` 有一份写进模块 docstring 的既有
  口径——它不 import 任何 augmentor 业务模块，`EXPORT_FORMATS` / `INPUT_FORMATS` 就是这个
  形状的现成先例，一致性由守卫比两份常量。本文件第 3 面因此**不**要求 parser 引用常数，
  而要求它的三档默认值与常数逐项相等，并且当场验那行里没有 augmentor 的 import。
- **同场两条 `--fields` 不收编**：`analyze` 的默认面是 `instruction` + `output` 两格，
  `check-leakage` 只比 `instruction` 一格。第 3 面把它们逐一点名，防止「把一切叫 fields
  的东西都并进来」这种假收口。

口径 5 之外还钉了一格**现量不对称**（本轮只核对未修，已立 **A339**）：显式空列表在
`PiiSanitizer` 那侧是「零字段」的合法意图（L188 改的正是这一刀），在 `DatasetAuditor`
那侧仍被 `fields or 默认` 读成「没传」而套上三格。本文件把它钉成现量而不是顺手改掉——
改掉要让空值统计面变窄，属行为变更，待拍。
"""

import ast
import pathlib
import re

import pytest

from augmentor.audit import DatasetAuditor
from augmentor.cli.parser import build_parser
from augmentor.privacy import DEFAULT_TEXT_FIELDS, PiiSanitizer

ROOT = pathlib.Path(__file__).resolve().parents[2]

#: 那份字面量的两种写法（方括号 / 圆括号）都算——权威用元组，抄写用列表，
#: 尺子必须同时认，否则「把定义抄成列表」这种漂移会在本面静默通过。
TRIO = re.compile(r'[\[(]\s*"instruction",\s*"input",\s*"output"\s*[\])]')
DEFINITION = re.compile(r'^DEFAULT_TEXT_FIELDS = \("instruction", "input", "output"\)$')
#: 五处就位的形状：`list(权威)`（SDK 两处）与 `default=[…]`（CLI 三处）。
SDK_USE = re.compile(r"list\(DEFAULT_TEXT_FIELDS\)")

CLI_TRIO_SUBCOMMANDS = ("clean", "sanitize", "audit")


def _src(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


def cli_fields_default(subcommand: str, required: list) -> object:
    """真解析一次 argv，取那一族 `--fields` 的默认值（不走 argparse 私有面）"""
    return vars(build_parser().parse_args([subcommand, *required])).get("fields")


class TestTheAuthorityIsOneObject:
    """第 1 面：SDK 两面共引同一份，抄第二份在这一面红"""

    def test_audit_and_privacy_bind_the_same_object(self):
        import augmentor.audit as audit_module
        import augmentor.privacy as privacy_module

        assert audit_module.DEFAULT_TEXT_FIELDS is privacy_module.DEFAULT_TEXT_FIELDS
        assert audit_module.DEFAULT_TEXT_FIELDS is DEFAULT_TEXT_FIELDS

    def test_the_two_sdk_sites_read_the_authority_instead_of_a_literal(self):
        """两处各自的那一句：`audit` 用 `or`、`privacy` 用 `is not None`（这一格不对称是
        A339 的内容，本面只验「两边都从权威取值」）"""
        for rel, sentence in (
            ("augmentor/audit.py", "fields or list(DEFAULT_TEXT_FIELDS)"),
            ("augmentor/privacy.py",
             "fields if fields is not None else list(DEFAULT_TEXT_FIELDS)"),
        ):
            src = _src(rel)
            assert SDK_USE.search(src), rel
            assert sentence in src, (rel, sentence)

    def test_privacy_holds_exactly_one_trio_and_it_is_the_definition(self):
        """权威文件里那一遍必须是定义本身，逐行点名落点"""
        src = _src("augmentor/privacy.py")
        hits = [line for line in src.splitlines() if TRIO.search(line)]
        assert len(hits) == 1, hits
        assert DEFINITION.match(hits[0]), hits[0]

    def test_audit_holds_no_trio_literal_at_all(self):
        assert not TRIO.search(_src("augmentor/audit.py"))


class TestTheValueIsTheHistoricThree:
    """第 2 面：常量的内容与形状——漂移一格就红"""

    def test_the_authority_names_exactly_the_three_default_faces(self):
        assert DEFAULT_TEXT_FIELDS == ("instruction", "input", "output")

    def test_the_authority_is_immutable_so_five_readers_cannot_share_one_mutant(self):
        """取元组而不是取列表的理由：模块级可变列表被多处共引，一处 `append` 改掉五处默认面"""
        assert isinstance(DEFAULT_TEXT_FIELDS, tuple)
        sanitizer, auditor = PiiSanitizer(), DatasetAuditor()
        sanitizer.fields.append("injected")
        auditor.fields.append("injected")
        assert DEFAULT_TEXT_FIELDS == ("instruction", "input", "output")
        assert PiiSanitizer().fields == ["instruction", "input", "output"]
        assert DatasetAuditor().fields == ["instruction", "input", "output"]


class TestRuntimeDefaultsFollowTheAuthority:
    """第 3 面：SDK 的默认值确实来自常量，且新旧行为逐格相同"""

    def test_both_sdk_defaults_are_lists_of_the_same_three_names(self):
        """类型仍是列表——账本与 l227 / l228 的 docstring 钉的就是这个形状"""
        for instance in (PiiSanitizer(), DatasetAuditor()):
            assert isinstance(instance.fields, list)
            assert instance.fields == list(DEFAULT_TEXT_FIELDS)

    def test_explicit_fields_still_win_on_both_faces(self):
        assert PiiSanitizer(fields=["output"]).fields == ["output"]
        assert DatasetAuditor(fields=["output"]).fields == ["output"]

    def test_the_empty_list_is_honest_on_one_face_and_swallowed_on_the_other(self):
        """A339 的现量：显式空列表在 privacy 侧是合法意图，在 audit 侧仍被 `or` 读成没传

        本轮**只核对未修**。把这一格钉成断言而不是注释，是为了让「顺手改掉」或「悄悄修好」
        都必须在 A339 那一行留下账——它现在红着的是行为，不是尺子。
        """
        assert PiiSanitizer(fields=[]).fields == []
        assert DatasetAuditor(fields=[]).fields == list(DEFAULT_TEXT_FIELDS)


class TestTheCliFaceIsPinnedNotImported:
    """第 4 面：CLI 三档默认逐项对账，并按 parser 自己的口径验它没有第二份 import"""

    REQUIRED = {
        "clean": ["--input", "a.json", "--output", "b.json"],
        "sanitize": ["--input", "a.json", "--output", "b.json"],
        "audit": ["--input", "a.json"],
    }

    @pytest.mark.parametrize("subcommand", CLI_TRIO_SUBCOMMANDS)
    def test_the_three_trio_defaults_equal_the_authority(self, subcommand):
        got = cli_fields_default(subcommand, self.REQUIRED[subcommand])
        assert got == list(DEFAULT_TEXT_FIELDS), subcommand

    def test_parser_keeps_three_literals_and_every_one_is_a_default(self):
        """三处逐格点名：多一处（抄回 SDK 侧的写法）或少一处（悄悄改成 import）都红"""
        src = _src("augmentor/cli/parser.py")
        lines = [line for line in src.splitlines() if TRIO.search(line)]
        assert len(lines) == 3, lines
        for line in lines:
            assert "default=" in line, line

    def test_parser_still_imports_no_augmentor_business_module(self):
        """`parser.py` 的既有口径（模块 docstring 里 `EXPORT_FORMATS` / `INPUT_FORMATS`
        两处注释即先例）：CLI 面用字面量 + 守卫对账，而不是 import 权威。
        这一条钉住的是「为什么第 4 面不比对象同一性」——口径若改了，本条先红，
        届时应把这三处一并收成引用而不是让两条形状并存。

        判的是 **AST 里的真实 import**而不是行首正则：注入臂 A5 实测过，
        `import argparse; from augmentor.privacy import DEFAULT_TEXT_FIELDS` 这种
        「分号偷带」在行首正则下静默通过（`from` 不在行首），而 AST 一定看得见。
        散文里提到 augmentor 不算——禁令管的是导入，不是措辞。
        """
        tree = ast.parse(_src("augmentor/cli/parser.py"))
        imported = []
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                imported += [alias.name for alias in node.names]
            elif isinstance(node, ast.ImportFrom):
                imported += ["." * (node.level or 0) + (node.module or "")]
        offenders = [name for name in imported
                     if name.startswith(".") or name.split(".")[0] == "augmentor"]
        assert offenders == [], offenders

    @pytest.mark.parametrize("subcommand,expected", [
        ("analyze", ["instruction", "output"]),
        ("check-leakage", ["instruction"]),
    ])
    def test_the_two_out_of_family_field_defaults_are_named_not_absorbed(self, subcommand, expected):
        """`--fields` 这个名字下还有两条别的默认面，A285 不收编它们"""
        required = {
            "analyze": ["--input", "a.json"],
            "check-leakage": ["--train", "a.json", "--test", "b.json"],
        }[subcommand]
        assert cli_fields_default(subcommand, required) == expected, subcommand
