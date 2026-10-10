# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L203（A265）：账本「已完成」清单与循环日志的进度一致性

**为什么有这一支**：同一型事故发生了两次，都靠人眼发现 —— L194 只写了循环日志块、
漏写「已完成」清单行（L197 才补记）；更早的 L50 也漏过一次。而「每轮回看循环机制」
是用户定下的硬要求，那一整段可以消失、全量测试照样绿。

**判据**（全部从账本推导，不抄清单）：
- `LOG` = 循环日志里所有 `### L<n>` / `### L<a>–L<b>` 标题解析出的轮次集合；
- `LIST` = 「已完成」段里所有 `- [x] **L<n>**` / `**L<a>–L<b>**` 解析出的轮次集合；
- `BOUNDARY` = 指针句「本清单只到 L<m>」里的 m（**从文档现读**，不抄第二份）。

三条不变量，逐条对应一种真实失效形状：
1. **`max(LOG) ∈ LIST`** —— 最新一轮必须两边都在。L194 那次的形状（日志写到 L194、
   清单停在 L113）当场红。
2. **`LIST ⊆ LOG`** —— 清单里不许出现日志里没有的轮次（防清单行指向一段不存在的日志）。
3. **`LIST` 中边界以上的那一段必须是从 `max(LIST)` 一路连续到 `max(LOG)` 的一整条尾巴**
   —— 既抓「漏了最新一轮」，也抓「最近这几轮中间被跳掉一轮」。L194 漏写时这条也红
   （尾巴从 L194 起，而 LOG 已到 L194 但 LIST 的尾巴起点高于它）。

第一版判据想当然写成「`LOG - LIST` 必须是边界及以前的轮次」，实测当场红并把设计
错在哪暴露出来：**边界之前根本就没有「只在日志里」的轮次**（清单完整覆盖 L101–L113），
而**边界之后**那一大片 L114–L193 才是刻意不回填的。所以「认边界」这个设计在当前
账本形态下不起作用，已按上面三条重写。
"""

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "OPTIMIZATION_LOOP_2.md"

#: 「已完成」清单行：- [x] **L101** …  /  - [x] **L104–L113** …
CHECK_LINE = re.compile(r"^- \[x\] \*\*L(\d+)(?:[–-]L(\d+))?\*\*")
#: 循环日志块标题：### L101（… / ### L104–L113（… / ### L114 补记（…
LOG_HEAD = re.compile(r"^### L(\d+)(?:[–-]L(\d+))?")
#: 指针句里的边界：本清单只到 L113
BOUNDARY = re.compile(r"本清单只到 L(\d+)")

UPPER = 10 ** 6


def _text() -> str:
    return LEDGER.read_text(encoding="utf-8")


def _section(text: str, heading: str) -> str:
    """取 heading 之后、下一个标题之前的那段"""
    out, on = [], False
    for line in text.splitlines():
        if line.startswith("#"):
            if on:
                break
            on = line.strip() == heading
            continue
        if on:
            out.append(line)
    return "\n".join(out)


def _expand(a: str, b) -> set:
    start = int(a)
    end = int(b) if b else start
    return set(range(start, min(end, UPPER) + 1))


def logged_rounds() -> set:
    text = _text()
    assert "## 循环日志" in text, "账本里找不到 `## 循环日志` 段"
    rounds = set()
    for line in text.split("## 循环日志", 1)[1].splitlines():
        m = LOG_HEAD.match(line)
        if m:
            rounds |= _expand(m.group(1), m.group(2))
    assert rounds, "循环日志一段都没解析到 ⇒ 标题形状变了，判据失效"
    return rounds


def checklist_rounds() -> set:
    section = _section(_text(), "## 已完成")
    assert section.strip(), "账本里找不到 `## 已完成` 段，或它是空的"
    rounds = set()
    for line in section.splitlines():
        m = CHECK_LINE.match(line.strip())
        if m:
            rounds |= _expand(m.group(1), m.group(2))
    assert rounds, "已完成清单一行都没解析到 ⇒ 行形状变了，判据失效"
    return rounds


def declared_boundary() -> int:
    m = BOUNDARY.search(_text())
    assert m, "账本里找不到「本清单只到 L<n>」这句指针 ⇒ 边界的唯一产地没了"
    return int(m.group(1))


class TestTheRulersSeeSomething:
    """防空转：三个量都必须真的取到东西，否则 0 == 0 也能绿"""

    def test_census_sees_both_faces(self):
        logged = logged_rounds()
        listed = checklist_rounds()
        assert len(logged) >= 50, f"只解析到 {len(logged)} 轮日志"
        assert len(listed) >= 5, f"只解析到 {len(listed)} 行清单"
        assert declared_boundary() >= 1

    def test_the_log_only_region_is_nonempty(self):
        """证明「刻意不回填」那片区域真被解析到了

        这一条是防「判据恒真」的关键：如果普查把 L114–L193 全漏掉了，
        `LOG - LIST` 会是空集，上面几条不变量会退化成「什么都没比就绿」。
        """
        log_only = logged_rounds() - checklist_rounds()
        assert log_only, (
            "「只在日志里」的轮次一个都没有 ⇒ 要么历史已全部回填（那把指针句的边界"
            "往前挪），要么普查漏掉了日志的大半边"
        )
        assert len(log_only) >= 50, f"只解析到 {len(log_only)} 轮「只在日志里」"

    def test_the_boundary_is_below_the_newest_round(self):
        """指针句必须还没过期：边界旧于最新一轮，这句「只到 L<m>」才是有信息量的"""
        assert declared_boundary() < max(logged_rounds()), (
            f"指针说只到 L{declared_boundary()}，而日志最新一轮是 "
            f"L{max(logged_rounds())} ⇒ 该把边界往前挪了"
        )


class TestEveryRecentRoundIsListed:
    """核心不变量：清单在边界以上那一段，必须是一路连到最新轮的完整尾巴"""

    def test_the_newest_round_is_listed(self):
        assert max(logged_rounds()) in checklist_rounds(), (
            f"最新一轮 L{max(logged_rounds())} 没有「已完成」清单行 —— "
            "这正是 L194 与 L50 两次事故的形状（只写日志块、漏写清单行）"
        )

    def test_the_tail_above_the_boundary_is_contiguous(self):
        """既抓「漏了最新一轮」，也抓「最近几轮中间被跳掉一轮」"""
        boundary = declared_boundary()
        logged = logged_rounds()
        listed = checklist_rounds()
        tail = sorted(r for r in listed if r > boundary)
        assert tail, "边界以上一个清单行都没有 ⇒ 指针句与清单对不上"
        newest = max(logged)
        assert tail[-1] == newest, (
            f"清单尾巴止于 L{tail[-1]}，而日志最新一轮是 L{newest} ⇒ 漏了靠前的若干轮"
        )
        expected = list(range(tail[0], newest + 1))
        assert tail == expected, (
            f"清单在边界以上不是连续尾巴：缺 {sorted(set(expected) - set(tail))}。"
            "回填时跳过一轮、或只补了其中几轮，都会落进这一格"
        )

    def test_the_checklist_does_not_invent_rounds(self):
        """反向：清单里不许出现日志里没有的轮次"""
        listed_only = sorted(checklist_rounds() - logged_rounds())
        assert not listed_only, (
            f"清单里有这些轮次、循环日志里却没有：L{listed_only}"
        )

    def test_the_boundary_itself_is_listed(self):
        """指针说「只到 L<m>」，那 L<m> 必须真的在清单里"""
        boundary = declared_boundary()
        assert boundary in checklist_rounds(), (
            f"指针句说清单只到 L{boundary}，但 L{boundary} 不在清单里 ⇒ 指针是假的"
        )
