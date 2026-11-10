# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""Backlog 编号的日志说法名册：A344 只准这样写（A350① / L303）

**这一轮收的是哪一格**：A350 立账时量到同一个编号在日志要点里被写成三件不同的事——A344
的表行说「点名的提交哈希只验解析、没量可达性」，L266 块 ⑮ 写成「共享工作树吞提交 / 换机器
部署」、L268 块 ⑫ 写成「`api/routes/*` 的写盘端点失败形状」、L269 块 ⑫ 沿用后者。L300 做掉
了候选②（三处同行订正），本轮做候选①：把「编号 → 允许的说法」钉成逐格名册。

**判据面**：只扫 `## 循环日志` 之后的日志 bullet（`- ⑮ ...` 那种），只看两种「说法形状」：

1. 编号后紧跟全角括号：`A344 那一格（...）`、`**A344**（...）`；
2. 编号后紧跟「的」：`A344 的可达性缺口`。

括号若是定级 / 轮号元数据（`M，待拍`、`L270 立`），不进内容面；`A344 的 ②` 这种候选序号
引用也不进面。除此之外的叙述性点名（「A350 = ... A344 实测三种互斥说法」）不在这一把尺的
量面里——那是 A350 的立账记录，不是对 A344 内容的断言。

**为什么排除 L300 块**：L300 块 ⑫ 是那次三处订正的记录，按 A348① 同一口径「记录不改写」，
它必须引用旧说法才能说明改了什么。`QUOTE_BLOCKS = {300}` 把它排除在活说法面外；
`test_the_correction_record_block_is_not_a_blanket_tolerance` 把「排除是承重的」钉住：
把排除摘掉，块里那三条旧括号立刻成为缺陷格。

**第一遍就咬到第四格（本轮真实返工，不美化）**：L300 只订正了 L269 块 ⑫，块 ② 里同一句
「A344 的「失败形状」」还在。本轮先按守卫读数把它同行订正成「A344 的可达性缺口」，再落
名册——这正是 A350① 要的牙齿：L300 的三处名册漏了第四处，靠人读没发现，靠形状判据发现。

**为什么零 parametrize**：l97 / l98 把全仓 parametrize 三面冻成现量数，本守卫走字典循环，
两支普查尺子一格未动。

跑法：`python -m pytest tests/unit/test_backlog_claim_roster_l303.py -q --no-cov`
"""
import re

from tests.unit import test_backlog_mark_vocab_l253 as other

VOLUMES = other.VOLUMES
ROOT = other.ROOT
BLOCK_HEAD = re.compile(r"^### L(\d{1,3})")
BULLET = re.compile(r"^\s*-\s+(?:([①-⑳])\s+)?")

A344 = "A344"

#: 记录块：L300 块 ⑫ 是「三处订正」的现场，必须引用旧说法说明改了什么。
#: 按 A348① 的口径，记录不改写，也不进活说法面。
QUOTE_BLOCKS = frozenset({300})

#: 编号 -> 允许的内容说法。`parenthetical` 是编号后括号的精确允许集；
#: `noun` 是「编号 + 的」后面允许起头的名词（长的放前面）；`forbidden` 是 A350
#: 量到的三种互斥旧说法，任何人把它们写回活说法面都当场红。
CLAIMS = {
    A344: {
        "parenthetical": frozenset({"点名的提交哈希只验解析、没量可达性"}),
        "noun": ("可达性缺口", "可达性", "提交哈希"),
        "forbidden": ("共享工作树吞提交", "换机器部署", "写盘端点失败", "失败形状"),
    },
}

#: 定级 / 轮号这类元数据括号不是对编号内容的说法。
METADATA = re.compile(r"^(?:[MSXL]{1,3}(?:，待拍)?|L\d+ 立|L\d+)$")
#: `A344 的 ②` 是候选序号引用，不是内容断言。
ORDINAL = re.compile(r"^[①-⑳]")

_TRACKED = "|".join(sorted(map(re.escape, CLAIMS), key=len, reverse=True))
PAREN = re.compile(
    rf"\*{{0,2}}(?P<id>{_TRACKED})\*{{0,2}}(?: 那一格)?（(?P<claim>[^）]*)）")
NOUN = re.compile(
    rf"(?P<id>{_TRACKED}) 的\s*(?P<claim>[^，。；、）」\s]{{0,16}})")

#: 现量四格：L266 ⑮、L268 ⑫、L269 ②⑫。三条旧说法被订正后，L269 的 ② 与 ⑫ 同形。
MEASURED_CONTENT_CELLS = frozenset({
    ("OPTIMIZATION_LOOP_2.md", 266, "⑮", "parenthetical", "点名的提交哈希只验解析、没量可达性"),
    ("OPTIMIZATION_LOOP_2.md", 268, "⑫", "parenthetical", "点名的提交哈希只验解析、没量可达性"),
    ("OPTIMIZATION_LOOP_2.md", 269, "②", "noun", "可达性缺口"),
    ("OPTIMIZATION_LOOP_2.md", 269, "⑫", "noun", "可达性缺口"),
})


def bullets(volumes=None):
    """日志区里带圈号的 bullet ⇒ `{volume, block, line, ordinal, text}`"""
    volumes = volumes if volumes is not None else other.load_volumes()
    out = []
    for name, lines in volumes.items():
        bound = other.log_boundary(lines)
        block = None
        for i, line in enumerate(lines, 1):
            head = BLOCK_HEAD.match(line)
            if head:
                block = int(head.group(1))
                continue
            if block is None or i - 1 <= bound or block in QUOTE_BLOCKS:
                continue
            hit = BULLET.match(line)
            if not hit:
                continue
            out.append({"volume": name, "block": block, "line": i,
                        "ordinal": hit.group(1) or "", "text": line})
    return out


def cells(volumes=None):
    """两种「说法形状」的逐格名册；元数据括号与候选序号引用不进面"""
    out = []
    for bullet in bullets(volumes):
        for hit in PAREN.finditer(bullet["text"]):
            claim = hit.group("claim")
            if METADATA.match(claim):
                continue
            out.append({**bullet, "id": hit.group("id"),
                        "shape": "parenthetical", "claim": claim})
        for hit in NOUN.finditer(bullet["text"]):
            claim = hit.group("claim")
            if ORDINAL.match(claim):
                continue
            out.append({**bullet, "id": hit.group("id"),
                        "shape": "noun", "claim": claim})
    return out


def canonical(cell):
    """把名词格的长捕获归一到允许的那一名词（只对合格格调用）"""
    if cell["shape"] == "parenthetical":
        return cell["claim"]
    return next(n for n in CLAIMS[cell["id"]]["noun"]
                if cell["claim"].startswith(n))


def defects(volumes=None):
    """允许集外的说法，或带禁止词的格，逐格返回"""
    out = []
    for cell in cells(volumes):
        spec = CLAIMS[cell["id"]]
        claim = cell["claim"]
        if cell["shape"] == "parenthetical":
            allowed = claim in spec["parenthetical"]
        else:
            allowed = claim.startswith(spec["noun"])
        dirty = any(bad in claim for bad in spec["forbidden"])
        if not allowed or dirty:
            out.append(cell)
    return out


def live_roster(volumes=None):
    """`(卷, 块, 圈号, 形状, 归一后的说法)` 的精确集合"""
    return frozenset(
        (cell["volume"], cell["block"], cell["ordinal"],
         cell["shape"], canonical(cell))
        for cell in cells(volumes))


def rewrite(block, ordinal, old, new):
    """内存里改写卷二某个块某条 bullet 的一处文字；返回 volumes，不落盘"""
    volumes = other.load_volumes()
    lines = list(volumes["OPTIMIZATION_LOOP_2.md"])
    current = None
    for i, line in enumerate(lines):
        head = BLOCK_HEAD.match(line)
        if head:
            current = int(head.group(1))
            continue
        if current == block and line.startswith("- %s " % ordinal):
            assert old in line, (block, ordinal, old)
            lines[i] = line.replace(old, new, 1)
            volumes["OPTIMIZATION_LOOP_2.md"] = lines
            return volumes
    raise AssertionError("找不到 L%d 的 %s 那一条 bullet" % (block, ordinal))


class TestTheClaimRoster:
    def test_the_content_cells_are_the_measured_roster(self):
        assert live_roster() == MEASURED_CONTENT_CELLS

    def test_no_live_cell_carries_a_forbidden_phrasing(self):
        assert defects() == []

    def test_the_authoritative_row_is_what_the_roster_paraphrases(self):
        rows = {row["id"]: row for row in other.table_rows(other.load_volumes())
                if row["id"] == A344}
        assert set(rows) == {A344}
        text = rows[A344]["text"]
        for token in ("提交哈希", "解析", "分支"):
            assert token in text, token
        claim = next(iter(CLAIMS[A344]["parenthetical"]))
        for token in ("提交哈希", "解析", "可达性"):
            assert token in claim, token

    def test_the_two_shapes_are_both_in_the_live_face(self):
        assert {cell["shape"] for cell in cells()} == {"parenthetical", "noun"}


class TestRulerBites:
    def test_a_wrong_parenthetical_on_l266_goes_red(self):
        volumes = rewrite(266, "⑮", "点名的提交哈希只验解析、没量可达性",
                          "共享工作树吞提交 / 换机器部署")
        got = defects(volumes)
        assert len(got) == 1
        assert got[0]["block"] == 266
        assert got[0]["claim"] == "共享工作树吞提交 / 换机器部署"

    def test_a_wrong_noun_on_l269_goes_red(self):
        volumes = rewrite(269, "⑫", "可达性缺口", "失败形状")
        got = defects(volumes)
        assert len(got) == 1
        assert got[0]["block"] == 269
        assert got[0]["claim"].startswith("失败形状")

    def test_a_wrong_parenthetical_on_l268_goes_red(self):
        volumes = rewrite(268, "⑫", "点名的提交哈希只验解析、没量可达性",
                          "`api/routes/*` 的写盘端点失败形状")
        got = defects(volumes)
        assert len(got) == 1
        assert got[0]["block"] == 268

    def test_a_fifth_content_cell_is_outside_the_roster(self):
        volumes = other.load_volumes()
        lines = list(volumes["OPTIMIZATION_LOOP_2.md"])
        lines += ["### L999", "- ㉑ **A344**（点名的提交哈希只验解析、没量可达性）"]
        volumes["OPTIMIZATION_LOOP_2.md"] = lines
        assert defects(volumes) == []
        assert live_roster(volumes) != MEASURED_CONTENT_CELLS


class TestTheCorrectionRecordIsNotTolerance:
    def test_the_correction_record_block_is_out_of_the_live_face(self):
        assert not any(cell["block"] == 300 for cell in cells())

    def test_the_correction_record_block_is_not_a_blanket_tolerance(self, monkeypatch):
        import tests.unit.test_backlog_claim_roster_l303 as me
        monkeypatch.setattr(me, "QUOTE_BLOCKS", frozenset())
        got = defects()
        assert got
        assert {cell["block"] for cell in got} == {300}

    def test_an_untracked_id_with_the_same_shape_is_out_of_scope(self):
        volumes = other.load_volumes()
        lines = list(volumes["OPTIMIZATION_LOOP_2.md"])
        lines += ["### L999",
                  "- ㉑ **A999**（写盘端点失败形状）",
                  "- ㉒ **A999** 的失败形状"]
        volumes["OPTIMIZATION_LOOP_2.md"] = lines
        assert not any(cell["block"] == 999 for cell in cells(volumes))
