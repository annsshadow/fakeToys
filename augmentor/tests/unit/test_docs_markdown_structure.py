# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""仓库 Markdown 结构守卫（L56，关闭 A93）

粗体成对、围栏闭合、表格列数一致、每表带表头分隔行、行尾不混用 —— 这五条口径从 L53 起
每轮靠 `Temp/` 里的一次性脚本手跑（L53 立粗体维、L54 立表格维与「数竖线先剥行内码」、
L55 立分隔行维），而 `Temp/` 不入库 ⇒ 判据随轮次流失，A93 因此拖了三轮。本文件把它们
冻成用例：文档坏一处当场变红。

每条判据各配一个「注入缺陷必被抓」的变异例，防止守卫在「其实一张表都没看到」时空转。
只读，不写任何文件；行尾按字节判，不经通用换行翻译。
"""

import re
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent

FENCE_RE = re.compile(r"^\s*```", re.M)
INLINE_CODE_RE = re.compile(r"`[^`]*`")
SEPARATOR_RE = re.compile(r"^\s*\|[\s:\-|]+\|\s*$")


def markdown_docs():
    """从目录派生待检文档，不抄清单 ⇒ 新增一份顶层或 `docs/` 文档自动进守卫

    只扫这两层的平铺 `.md`：递归 `rglob` 实测会把 `web/node_modules`（400+ 份）和
    `Temp/`、`.backups/` 一起拽进来，那是第三方与草稿的领地，不是本仓库的交付面。
    """
    found = set(AI_DIR.glob("*.md")) | set((AI_DIR / "docs").glob("*.md"))
    return sorted(found, key=lambda p: str(p.relative_to(AI_DIR)))


DOCS = markdown_docs()


def read_text(path):
    """按字节读，保留原始行尾（换行翻译会把 MIXED 判据抹平）"""
    return path.read_bytes().decode("utf-8")


def flatten(text):
    return text.replace("\r\n", "\n")


def blank_code(text):
    """把围栏内内容与行内码清空，行号与空行结构不变

    竖线必须在这之后才数：单元格里的 `` `a|b` `` 和示例代码里的表格不是列分隔符。
    """
    out = []
    in_fence = False
    for line in flatten(text).split("\n"):
        if FENCE_RE.match(line):
            in_fence = not in_fence
            out.append("")
            continue
        out.append("" if in_fence else INLINE_CODE_RE.sub("", line))
    return "\n".join(out)


def find_tables(text):
    """连续的 `|` 开头行 = 一张表；只有一行的不算表（那是引用式的竖线）"""
    tables, cur = [], []
    for line in text.split("\n"):
        if line.lstrip().startswith("|"):
            cur.append(line)
            continue
        if len(cur) >= 2:
            tables.append(cur)
        cur = []
    if len(cur) >= 2:
        tables.append(cur)
    return tables


def mixed_line_endings(text):
    crlf = text.count("\r\n")
    bare_lf = flatten(text).count("\n") - crlf
    return crlf, bare_lf


def odd_bold_blocks(text):
    blocks = [b for b in blank_code(text).split("\n\n") if b.strip()]
    return [(i, b.count("**"), b.strip().split("\n")[0][:60])
            for i, b in enumerate(blocks) if b.count("**") % 2]


def ragged_tables(text):
    return [(t[0][:60], sorted({line.count("|") for line in t}))
            for t in find_tables(blank_code(text))
            if len({line.count("|") for line in t}) > 1]


def tables_without_separator(text):
    return [t[0][:60] for t in find_tables(blank_code(text))
            if not SEPARATOR_RE.match(t[1])]


def relative(doc):
    return doc.relative_to(AI_DIR).as_posix()


DOC_IDS = [relative(d) for d in DOCS]


class TestGuardSeesSomething:
    """守卫自己先要成立：范围非空、且每一维都真的看到了被测对象"""

    def test_docs_discovered(self):
        assert DOCS, "一份 Markdown 都没发现 ⇒ 守卫失效，不是通过"

    def test_dimensions_are_not_vacuous(self):
        tables = sum(len(find_tables(blank_code(read_text(d)))) for d in DOCS)
        fences = sum(len(FENCE_RE.findall(flatten(read_text(d)))) for d in DOCS)
        bolds = sum(b.count("**") for d in DOCS
                    for b in blank_code(read_text(d)).split("\n\n") if b.strip())
        assert tables > 0, "没看到任何表格 ⇒ 表格两维在空转"
        assert fences > 0, "没看到任何围栏 ⇒ 围栏维在空转"
        assert bolds > 0, "没看到任何粗体标记 ⇒ 粗体维在空转"


@pytest.mark.parametrize("doc", DOCS, ids=DOC_IDS)
class TestRepoDocsStructure:
    """五条判据，逐份文档各跑一遍"""

    def test_line_endings_not_mixed(self, doc):
        crlf, bare_lf = mixed_line_endings(read_text(doc))
        assert crlf == 0 or bare_lf == 0, (
            f"{relative(doc)} 行尾混用：CRLF={crlf} 裸LF={bare_lf}")

    def test_code_fences_closed(self, doc):
        markers = FENCE_RE.findall(flatten(read_text(doc)))
        assert len(markers) % 2 == 0, (
            f"{relative(doc)} 有 {len(markers)} 个 ``` 行 ⇒ 围栏未闭合，"
            "此后所有正文都会被当代码渲染")

    def test_bold_markers_paired(self, doc):
        odd = odd_bold_blocks(read_text(doc))
        assert not odd, f"{relative(doc)} 有 {len(odd)} 个块粗体标记不成对: {odd[:3]}"

    def test_tables_have_consistent_column_count(self, doc):
        bad = ragged_tables(read_text(doc))
        assert not bad, f"{relative(doc)} 有 {len(bad)} 张表各行列数不一致: {bad[:3]}"

    def test_tables_have_header_separator(self, doc):
        orphan = tables_without_separator(read_text(doc))
        assert not orphan, (
            f"{relative(doc)} 有 {len(orphan)} 张表第二行不是分隔行（多半是被空行劈断，"
            f"GFM 会丢掉表头）: {orphan[:3]}")


class TestCriteriaCatchInjectedDefects:
    """注入必被抓 + 一处无害对照 ⇒ 证明判据不是「恒真」"""

    MIXED = "甲\r\n乙\n丙\r\n"
    PURE_LF = "甲\n乙\n丙\n"

    OPEN_FENCE = "正文\n\n```python\nx = 1\n"
    CLOSED_FENCE = "正文\n\n```python\nx = 1\n```\n"

    ODD_BOLD = "段落一 **加粗没有闭\n\n段落二 正常\n"
    PAIRED_BOLD = "段落一 **加粗** 正常\n\n段落二 **也** 正常\n"

    RAGGED = "| A | B |\n|---|---|\n| 1 |\n"
    SQUARE = "| A | B |\n|---|---|\n| 1 | 2 |\n"

    NO_SEPARATOR = "| A | B |\n| 1 | 2 |\n"

    # 单元格里的行内码带竖线：列数必须一致（先剥行内码这条口径的反例守卫）
    PIPE_IN_INLINE_CODE = "| A | B |\n|---|---|\n| `a|b` | 2 |\n"

    def test_line_ending_dimension(self):
        assert mixed_line_endings(self.MIXED) == (2, 1)
        assert mixed_line_endings(self.PURE_LF) == (0, 3)

    def test_fence_dimension(self):
        assert len(FENCE_RE.findall(self.OPEN_FENCE)) % 2 == 1
        assert len(FENCE_RE.findall(self.CLOSED_FENCE)) % 2 == 0

    def test_bold_dimension(self):
        assert len(odd_bold_blocks(self.ODD_BOLD)) == 1
        assert odd_bold_blocks(self.PAIRED_BOLD) == []

    def test_table_dimensions(self):
        assert len(ragged_tables(self.RAGGED)) == 1
        assert ragged_tables(self.SQUARE) == []
        assert len(tables_without_separator(self.NO_SEPARATOR)) == 1
        assert tables_without_separator(self.SQUARE) == []

    def test_inline_code_pipe_is_not_a_column_separator(self):
        assert ragged_tables(self.PIPE_IN_INLINE_CODE) == [], (
            "行内码里的竖线被当成列分隔符 ⇒ 「先剥后数」口径失效")

    def test_single_pipe_row_is_not_a_table(self):
        assert find_tables(blank_code("| 只有一行 |\n")) == []
