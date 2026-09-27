# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L98 守卫：账本「批哈希槽」与 git 真值对账（A180 同轮立同轮关）

**这一格为什么是缺陷而不是洁癖**（L98 实测，不是推理）：L97 批④ 回填上一轮哈希时，把
L97 **自己**的三批（`5655dd35f` + `4860e64a2` + `9dd0cfa6d`，主题各点着「L97 批①②③」）写进了
**L96** 那一行的槽里，而且进度行与文末日志块标题**两遍都写成了同一个错形状**。后果是双向的：

* L96 真正的三批（`e79538569` + `980cd313b` + `7da7190d8`）在整本账里一笔都不存在；
* 账上「L96 做了哪三笔」变成假事实，而它看起来完全合法 —— 两行都写着「三批」，所以
  「批数 = 槽内哈希数」这种只数数量的判据是绿的。**缺陷只在内容上，内容没有任何判据。**

本轮改造前全仓对哈希槽的读数能力是零：`test_doc_line_refs_l79` 数的是**占位符**有几处
（`PLACEHOLDER_CEILING`），占位被填掉之后那一格就交回散文，从此无人再看。⇒ 六档判据：

R1 槽内每笔哈希都能解析成 `git log --first-parent` 里的一个 commit；
R2 同一笔哈希不许被两轮同时记账（跨轮重复 ⇒ 复制粘贴漂移）；
R3 槽内提交顺序单调：阅读顺序按时间递增，而 rev-list 的 0 是最新 ⇒ **序号严格递减**；
R4 行文里声明的「三批 / 四批」与槽内哈希数量相等；
R5 槽的形状：每行要么全是哈希要么全是占位，占位指向的轮号必须大于本行轮号；
R6 每笔哈希的 commit 主题要点着它所属那一轮的轮号（白名单按**集合相等**钉，不是「≤」）。

**R3 的方向是被自证逼出来的**：本探针第一版按「序号递增」判，于是把 21 格正确形状报成
破口。方向写反的判据永远红，属会自曝的一类；但本轮在**写判据之前**就撞到了它，说明
「让判据自己解释自己」不够，必须**喂已知答案**（见 `TestRulerProvesItself`）。
"""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Tuple

import pytest

from tests.unit.test_doc_line_refs_l79 import (
    HASH_PLACEHOLDER,
    LEDGER,
    LOG_BLOCK,
    PLACEHOLDER_CEILING,
    PROGRESS_LINE,
)
from tests.unit.test_measurement_rulers_l94 import RulerNotProven

SELF = sys.modules[__name__]
REPO_ROOT = Path(__file__).resolve().parents[2]
#: 与 `test_doc_line_refs_l79` 读同一份账：那边 `LEDGER` 是**相对**字符串路径，这里按仓库根
#: 拼一次，免得两支守卫因为 cwd 不同而各自读到一份账还都觉得自己绿。
LEDGER_PATH = REPO_ROOT / LEDGER

#: 槽 = `**L<n>**` 与下一个全角 `（` 之间那段反引号清单。整行扫会把**文件 sha256 前缀**
#: （L45 五笔逐字节还原、L63 一笔）和**缓存键前缀**（L72 两笔）当成 commit，还会把进度行
#: 正文里谈论占位的那几句散文当成占位（L60～L65、L75 各一处）⇒ 先定位槽，再在槽内解析。
HASH_TOKEN = re.compile(r"^`([0-9a-f]{7,12})`$")
SLOT_PLACEHOLDER = re.compile(r"^`哈希待 (L\d+) 回填`$")
BATCH_DECLARED = re.compile(r"([一二三四五六])批按显式路径提交")
CN_NUM = {"一": 1, "二": 2, "三": 3, "四": 4, "五": 5, "六": 6}
ROUND_TOKEN = re.compile(r"(?<!\d)L\d+(?!\d)")

#: 现量（L98 批④ 重量于「回填 L97 四笔 + 落下 L98 自己的占位」之后：进度行多一行带槽、
#: 日志块多两行带槽（L97 由只块变两侧、L98 新行），唯一哈希 +4）。这些是**读数**不是预算：
#: 下一轮写了新块就会动，动了就要重量 —— 与下面那几档精确相等断言是同一条纪律。
MEASURED_SLOT_ROWS = {"prog": 13, "log": 22}
MEASURED_BOTH_SIDES = 13
MEASURED_UNIQUE_HASHES = 57
MEASURED_LOG_ONLY_ROUNDS = frozenset(
    {"L45", "L57", "L79", "L80", "L81", "L82", "L83", "L84", "L85"})
MEASURED_PROGRESS_PLACEHOLDERS = 1

#: R6 的白名单：**账本没错、commit 主题自己写错或压根没写轮号**的四格。
#: 逐格理由（都按 `git log --first-parent` 的位置与该行声明的批数核过）：
#: `L83 / 3b0d513a6` 主题写着「L82 批②」，但它紧跟 L83 批① `ee3b12404`、且 L82 自己那行
#: 已有三笔并声明「三批」⇒ 账本对、主题里的轮号错；其余三格主题压根不写轮号。
MEASURED_SUBJECT_WHITELIST = frozenset({
    ("L83", "3b0d513a6"),
    ("L86", "b52b39780"),
    ("L86", "1e590559b"),
    ("L87", "c2290f1eb"),
})


# --------------------------------------------------------------------------- 提取口

def slot_tokens(line: str, rnd: str) -> List[str]:
    """本行「批哈希槽」里的反引号 token，按 `+` 分列"""
    head = line.split(f"**{rnd}**", 1)[1] if f"**{rnd}**" in line else line
    head = head.split("（", 1)[0]
    return [t.strip() for t in head.split("+") if t.strip().startswith("`")]


def read_slots(text: str, pattern: re.Pattern) -> Dict[str, Tuple[List[str], List[str]]]:
    """按**行首形状**提取每轮的槽：`{轮号: (哈希列表, 占位目标列表)}`

    复用 `PROGRESS_LINE` / `LOG_BLOCK` 而不是自造正则 —— 与本仓另外两支守卫同一口径
    （两侧早就各有提取口，缺的从来不是提取而是**对账**）。
    """
    out: Dict[str, Tuple[List[str], List[str]]] = {}
    for line in text.splitlines():
        m = pattern.match(line)
        if not m:
            continue
        rnd = "L" + m.group(1)
        toks = slot_tokens(line, rnd)
        hashes = [h.group(1) for h in (HASH_TOKEN.match(t) for t in toks) if h]
        phs = [p.group(1) for p in (SLOT_PLACEHOLDER.match(t) for t in toks) if p]
        if hashes or phs:
            out[rnd] = (hashes, phs)
    return out


def read_ledger_slots(text: str) -> Dict[str, Dict[str, Tuple[List[str], List[str]]]]:
    return {"prog": read_slots(text, PROGRESS_LINE), "log": read_slots(text, LOG_BLOCK)}


def round_of(rnd: str) -> int:
    return int(rnd[1:])


def ordered(d: Dict) -> List:
    """轮号按数值序（不是字典序）排，`L9` 不许排在 `L86` 后面"""
    return sorted(d, key=round_of)


# --------------------------------------------------------------------------- git 真值

def git_history() -> List[Tuple[str, str]]:
    """`[(完整 sha, 主题)]`，按 `--first-parent` 从新到旧；读不到就抛，不给读数

    单次 `git log` 而不是每笔哈希一次 `git show`：53 笔逐个起进程会把这支守卫拖成
    整套里最慢的一支（Windows 上每次 spawn 都是几十毫秒）。
    """
    try:
        cp = subprocess.run(["git", "log", "--first-parent", "--format=%H\t%s", "HEAD"],
                            cwd=str(REPO_ROOT), capture_output=True, text=True,
                            encoding="utf-8", errors="replace", timeout=120)
    except OSError as exc:
        raise RulerNotProven(f"读不到 git：{exc}") from exc
    if cp.returncode != 0 or not cp.stdout.strip():
        raise RulerNotProven(f"git log 不可用（退出码 {cp.returncode}）：{cp.stderr[:200]}")
    out = [(sha, subj) for sha, _, subj
           in (raw.partition("\t") for raw in cp.stdout.splitlines())
           if re.fullmatch(r"[0-9a-f]{40}", sha)]
    if len(out) < 500:
        raise RulerNotProven(f"first-parent 历史只有 {len(out)} 笔，不像本仓的真值")
    return out


def _prefixes(sha: str) -> Tuple[str, ...]:
    return tuple(sha[:n] for n in (7, 9, 12) if n <= len(sha))


def position_table(history: List[Tuple[str, str]]) -> Dict[str, int]:
    """`{前缀: 序号}`，序号 0 = HEAD = 最新；顺带钉住「7 位前缀在本仓唯一」

    不唯一 ⇒ 抛而不是挑一个：账本里的哈希现量是 9 位，但将来可能有人写 7 位，那一格
    现在就要响亮，而不是等某个 7 位前缀撞车时给出一个看起来合法的错序号。
    """
    table: Dict[str, int] = {}
    for i, (sha, _subj) in enumerate(history):
        for key in _prefixes(sha):
            if len(key) == 7 and key in table:
                raise RulerNotProven(f"7 位前缀撞车：{key} 同时是第 {table[key]} 与第 {i} 笔")
            table.setdefault(key, i)
    return table


def subject_table(history: List[Tuple[str, str]]) -> Dict[str, str]:
    table: Dict[str, str] = {}
    for sha, subj in history:
        for key in _prefixes(sha):
            table.setdefault(key, subj)
    return table


# --------------------------------------------------------------------------- 判据（纯函数）

def unresolvable(slots: Dict[str, Tuple[List[str], List[str]]],
                 table: Dict[str, int]) -> List[Tuple[str, str]]:
    """R1：槽内解析不到的哈希（任何前缀长度都追不到 = 不是本仓历史里的 commit）"""
    return [(rnd, h) for rnd, (hashes, _ph) in ((k, slots[k]) for k in ordered(slots))
            for h in hashes if h not in table]


def cross_round_dupes(*slot_maps: Dict[str, Tuple[List[str], List[str]]]
                      ) -> Dict[str, Tuple[str, ...]]:
    """R2：同一笔哈希被**不同轮**记账（同一轮在两侧各写一遍是既有形状，不算重复）"""
    seen: Dict[str, set] = {}
    for slots in slot_maps:
        for rnd, (hashes, _ph) in slots.items():
            for h in hashes:
                seen.setdefault(h, set()).add(rnd)
    return {h: tuple(sorted(r, key=round_of)) for h, r in seen.items() if len(r) > 1}


def monotonic_breaches(slots: Dict[str, Tuple[List[str], List[str]]],
                       table: Dict[str, int]) -> List[tuple]:
    """R3：按阅读顺序，槽内序号必须**严格递减**（rev-list 的 0 是最新 ⇒ 批① 序号最大）"""
    broken: List[tuple] = []
    prev_last = None
    for rnd in ordered(slots):
        hashes = slots[rnd][0]
        if not hashes:
            continue  # 占位行没有内容可对账
        pos = [table.get(h) for h in hashes]
        if any(p is None for p in pos):
            broken.append((rnd, "未解析"))  # R1 负责点名，这里只声明「无从判序」
            continue
        if pos != sorted(set(pos), reverse=True):
            broken.append((rnd, "行内逆序", pos))
        if prev_last is not None and pos[0] >= prev_last:
            broken.append((rnd, "跨行逆序", pos[0], prev_last))
        prev_last = pos[-1]
    return broken


def numeral_mismatches(text: str, slots: Dict[str, Tuple[List[str], List[str]]],
                       pattern: re.Pattern) -> List[Tuple[str, int, int]]:
    """R4：行文说「三批」而槽里躺着两笔（或四笔）"""
    out = []
    for line in text.splitlines():
        m = pattern.match(line)
        if not m:
            continue
        rnd = "L" + m.group(1)
        declared = BATCH_DECLARED.search(line)
        if declared is None or rnd not in slots:
            continue
        n = len(slots[rnd][0])
        want = CN_NUM[declared.group(1)]
        if n and want != n:
            out.append((rnd, want, n))
    return out


def side_mismatches(slots: Dict[str, Tuple[List[str], List[str]]]) -> List[str]:
    """R5：一行同时有哈希与占位 = 半填"""
    return [rnd for rnd in ordered(slots) if slots[rnd][0] and slots[rnd][1]]


def impossible_placeholders(slots: Dict[str, Tuple[List[str], List[str]]]) -> List[str]:
    """R5：占位说「待 L<n> 回填」而 n 不大于本行 ⇒ 指向过去或指向自己"""
    return [rnd for rnd in ordered(slots)
            for p in slots[rnd][1] if round_of(p) <= round_of(rnd)]


def subject_flags(slots: Dict[str, Tuple[List[str], List[str]]],
                  subjects: Dict[str, str]) -> List[Tuple[str, str]]:
    """R6：commit 主题里没有本行轮号（数字边界匹配，`L9` 不算命中 `L97`）"""
    out = []
    for rnd in ordered(slots):
        for h in slots[rnd][0]:
            found = set(ROUND_TOKEN.findall(subjects.get(h, "")))
            if rnd not in found:
                out.append((rnd, h))
    return out


def open_placeholders(slots: Dict[str, Tuple[List[str], List[str]]]) -> Dict[str, List[str]]:
    return {rnd: v[1] for rnd, v in slots.items() if v[1]}


# --------------------------------------------------------------------------- 尺子自证

#: 已知答案的合成样本。三档形状各有专门用途：`L1` 只有占位，`L2` 两笔真哈希（**批① 在前**
#: ⇒ 序号从大到小），中间那行散文里埋着一笔**形状与 commit 一字不差**的 12 位 sha 前缀、
#: 一句「谈论占位」和一个 `文件:行号` 引用。`L3` 那一行是「占位指向过去」的已知红。
def _sample_text(first: str, second: str) -> str:
    return (
        "- [x] **L1** `哈希待 L2 回填`（两批按显式路径提交、未 push）假轮\n"
        "- [x] **L2** `%s` + `%s`（两批按显式路径提交、未 push）`fix(a)`：\n"
        "  正文里谈 `哈希待 L3 回填`，还原 sha `d5a990e6ad`，还有 `foo.py:466`\n"
        "- **L2** `%s` + `%s`（两批按显式路径提交、未 push）`fix(a)`\n"
        "- **L3** `哈希待 L2 回填`（一批按显式路径提交、未 push）`fix(b)`\n"
        % (first, second, first, second)
    )


def prove_ruler(history: List[Tuple[str, str]]) -> None:
    """判据入库前先在同一份脚本里编译一段带已知答案的样本，答错即抛、不给读数

    样本答的五道题：
      1. 槽内解析：`L2` 恰好两笔，且顺序是「批① 在前」；
      2. 埋进散文的那笔 12 位 sha 前缀**不算**槽内哈希（它解析不到 git，若被读进来 R1 会
         当场假红 —— 这正是本仓 L45 / L63 那几笔的形状）；
      3. 散文里那句「谈论占位」不算占位：进度行占位只有一处；
      4. 单调性方向：**正确形状 0 破口，故意换序必须恰好 1 格** —— 方向写反的判据会在
         这一步自曝（L98 的探针正是在这里先错过一次）；
      5. R5 / R6 各点名一次（半填行、指向过去的占位、主题不写本轮）。
    """
    table = position_table(history)
    if len(history) < 2:
        raise RulerNotProven("样本需要两笔真哈希")
    first, second = history[1][0][:9], history[0][0][:9]  # 批① 更早 ⇒ 序号更大
    text = _sample_text(first, second)
    slots = read_ledger_slots(text)
    if slots["prog"].get("L2", ([], []))[0] != [first, second]:
        raise RulerNotProven(f"槽解析答案不符：{slots['prog'].get('L2')}")
    if any("d5a990e6ad" in h for v in slots["prog"].values() for h in v[0]):
        raise RulerNotProven("散文里的文件 sha 被当成槽内哈希")
    if open_placeholders(slots["prog"]) != {"L1": ["L2"]}:
        raise RulerNotProven(f"进度行占位答案不符：{open_placeholders(slots['prog'])}")
    if monotonic_breaches(slots["prog"], table):
        raise RulerNotProven("正确形状被判成破口 ⇒ 方向写反了")
    swapped = monotonic_breaches({"L2": ([second, first], [])}, table)
    if len(swapped) != 1 or swapped[0][0] != "L2":
        raise RulerNotProven(f"故意换序没被判出：{swapped}")
    if side_mismatches({"L4": ([first], ["L5"])}) != ["L4"]:
        raise RulerNotProven("半填形状没被 R5 判出")
    if impossible_placeholders({"L3": ([], ["L2"])}) != ["L3"]:
        raise RulerNotProven("指向过去的占位没被 R5 判出")
    if not subject_flags({"L9": ([first], [])}, subject_table(history)):
        raise RulerNotProven("主题不点本轮没被 R6 判出")


# --------------------------------------------------------------------------- fixtures

@pytest.fixture(scope="session")
def history() -> List[Tuple[str, str]]:
    return git_history()


@pytest.fixture(scope="session")
def proven(history) -> None:
    prove_ruler(history)


@pytest.fixture(scope="session")
def text() -> str:
    if not LEDGER_PATH.is_file():
        raise RulerNotProven(f"读不到账本：{LEDGER_PATH}")
    return LEDGER_PATH.read_text(encoding="utf-8")


@pytest.fixture(scope="session")
def slots(text, proven) -> Dict[str, Dict[str, Tuple[List[str], List[str]]]]:
    return read_ledger_slots(text)


@pytest.fixture(scope="session")
def table(history) -> Dict[str, int]:
    return position_table(history)


@pytest.fixture(scope="session")
def subjects(history) -> Dict[str, str]:
    return subject_table(history)


# --------------------------------------------------------------------------- 用例

class TestRulerProvesItself:
    """尺子先自证：答错即抛，判据连读数都不给（与 L97 的 param_sites 同一口径）"""

    def test_the_sample_is_answered_correctly(self, history):
        prove_ruler(history)

    def test_the_proof_actually_depends_on_the_ordering_ruler(self, history, monkeypatch):
        """把 R3 换成恒不报破口的版本后，自证必须抛 —— 证明自证在管事

        变异打在**本模块运行时绑定**上（monkeypatch 负责还原），不动仓库任何文件。
        """
        monkeypatch.setattr(SELF, "monotonic_breaches", lambda slots_arg, table_arg: [])
        with pytest.raises(RulerNotProven):
            prove_ruler(history)


class TestScopeIsPinned:
    """规模档：带槽的行有几行、两侧交集几行、唯一哈希几笔

    **为什么钉精确相等而不是「≥」**：这一族读数一旦被当成下限，「少了一行槽」这种腐坏
    （正是本轮撞到的那一格）就只能靠人眼。
    """

    def test_slot_row_counts_per_side(self, slots):
        assert {k: len(v) for k, v in slots.items()} == MEASURED_SLOT_ROWS

    def test_both_sides_set(self, slots):
        both = set(slots["prog"]) & set(slots["log"])
        assert len(both) == MEASURED_BOTH_SIDES, ordered({r: 1 for r in both})

    def test_log_only_rounds_are_the_legacy_ones(self, slots):
        assert set(slots["log"]) - set(slots["prog"]) == set(MEASURED_LOG_ONLY_ROUNDS)

    def test_unique_hash_count(self, slots):
        seen = {h for side in slots.values() for v in side.values() for h in v[0]}
        assert len(seen) == MEASURED_UNIQUE_HASHES, len(seen)


class TestTwoSidesTellTheSameStory:
    """A135（每轮写两遍自己）在哈希槽上的兑现：既然写两遍，那就把两遍对账

    L97 的错位**同时**写进了两遍，所以这一档抓不到那一格（抓到它的是 R6 + 人工重读）。
    它的用处是下一次只填一遍：改进度行忘改块标题，或反过来 —— 本轮批② 与批③ 之间
    就真实存在这个状态，这一档在那段时间里是红的，直到块标题那一侧也订正完才转绿。
    """

    def test_slots_agree_for_every_round_on_both_sides(self, slots):
        both = set(slots["prog"]) & set(slots["log"])
        diff = {r: (slots["prog"][r], slots["log"][r]) for r in ordered({k: 1 for k in both})
                if slots["prog"][r] != slots["log"][r]}
        assert diff == {}, diff


class TestAgainstGitTruth:
    def test_every_slot_hash_resolves_to_a_commit(self, slots, table):
        assert unresolvable(slots["prog"], table) == []
        assert unresolvable(slots["log"], table) == []

    def test_no_hash_is_credited_to_two_rounds(self, slots):
        assert cross_round_dupes(slots["prog"], slots["log"]) == {}

    def test_commit_order_is_monotonic_on_both_sides(self, slots, table):
        assert monotonic_breaches(slots["prog"], table) == []
        assert monotonic_breaches(slots["log"], table) == []


class TestProseMatchesNumbers:
    def test_declared_batch_count_equals_slot_length(self, text, slots):
        assert numeral_mismatches(text, slots["prog"], PROGRESS_LINE) == []
        assert numeral_mismatches(text, slots["log"], LOG_BLOCK) == []

    def test_no_row_is_half_filled(self, slots):
        assert side_mismatches(slots["prog"]) == []
        assert side_mismatches(slots["log"]) == []

    def test_placeholders_point_forward(self, slots):
        assert impossible_placeholders(slots["prog"]) == []
        assert impossible_placeholders(slots["log"]) == []

    def test_progress_side_has_exactly_one_open_slot_and_it_is_the_newest(self, slots):
        """进度行那一侧此前**没有任何**占位判据（`PLACEHOLDER_CEILING` 只数块标题那一侧）

        钉成「唯一一处占位，且它正是带槽最大轮号；它等待的那一轮 = 本仓最新一轮」，
        这样「填了上一轮却忘了给自己留占位」与「忘了填上一轮」两种漏法都落在同一断言上。
        """
        open_ = open_placeholders(slots["prog"])
        assert len(open_) == MEASURED_PROGRESS_PLACEHOLDERS, open_
        newest_with_slot = max((r for r, v in slots["prog"].items() if v[0] or v[1]),
                               key=round_of)
        assert list(open_) == [newest_with_slot], (open_, newest_with_slot)

    def test_block_side_placeholder_count_is_the_l79_ceiling(self, text):
        """同一格不重复计数：块标题那一侧沿用 `test_doc_line_refs_l79` 的常量与提取口"""
        assert len(HASH_PLACEHOLDER.findall(text)) == PLACEHOLDER_CEILING


class TestSubjectsNameTheirRound:
    def test_flag_set_equals_the_whitelist(self, slots, subjects):
        flagged = {(r, h) for r, h in subject_flags(slots["prog"], subjects)}
        flagged |= {(r, h) for r, h in subject_flags(slots["log"], subjects)}
        assert flagged == set(MEASURED_SUBJECT_WHITELIST), sorted(
            flagged - set(MEASURED_SUBJECT_WHITELIST))

    def test_the_ruler_that_nominates_a_round_is_not_loose(self):
        """`L9` 不许因为主题里写着 `L97` 就算命中 —— 数字边界不是装饰

        这一档直接决定 R6 白名单的可信度：宽松匹配会把「错轮」读成「对轮」，而 R6 的全部
        用处就在于认出**错轮**（本轮撞的就是这一格）。
        """
        assert set(ROUND_TOKEN.findall("启动面把 web 节三键读成行为（B5 / L97 批①）")) == {"L97"}
        assert set(ROUND_TOKEN.findall("承 L96 那一格，另立 A176 / A177 两行")) == {"L96"}


class TestTheDefectShapeGoesRed:
    """能红性：把本轮真实撞到的错位**原样**复现，判据里至少三档必须点名

    不是造一个假缺陷：样本用的是账本里两轮的**真**哈希，形状是「后一行装着前一行的提交，
    后一行自己的真提交零出现」，与 L97 批④ 造成的那一格逐字同形。
    """

    def test_a_mis_slot_is_caught_by_three_judges(self, text, table, subjects):
        real = read_ledger_slots(text)
        victim, thief = "L95", "L96"
        stolen = real["prog"][victim][0][0]
        bad_text = text
        for h in real["prog"][thief][0]:
            bad_text = bad_text.replace(f"`{h}`", f"`{stolen}`", 1)
        bad = read_ledger_slots(bad_text)
        assert bad["prog"][thief][0] and stolen in bad["prog"][thief][0]
        assert unresolvable(bad["prog"], table) == [], "真哈希 ⇒ R1 看不见这一格"
        assert cross_round_dupes(bad["prog"]), "同一笔被两轮记账，R2 必须点名"
        assert (thief, stolen) in subject_flags(bad["prog"], subjects), "R6 必须点名"
        assert monotonic_breaches(bad["prog"], table), "塞进别的轮的提交，序也一定错"
