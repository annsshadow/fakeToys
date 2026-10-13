# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""日志块要点里点名的提交哈希，必须能被归档回它所在块的轮号（L264）

**这一轮收的是哪一格**：L262 的记账批把「记账批读数」那条要点插进了 **L261 块尾**（`### L262` 标题在它下面），
九把账本与文档守卫全程绿——`test_ledger_hash_slots_l98` 判的是**清单区**那一格的哈希槽与 git 真值对账，
它扫不到日志块内部的要点归属。L263 把它搬回 L262 块时顺手在案上记了一句「这一形没有尺子」，本轮就是那把尺子。

**判据怎么落到 git 上**：站点 = 日志块（`## 循环日志` 之后、`^### L(\\d+)` 开块）行内被反引号包住的
`[0-9a-f]{6,40}`，允许尾随一个省略号（`` `cc1a80b8…` `` 那一形）。每只站点问一次
`git rev-parse --verify <h>^{commit}` 与 `git show -s --format=%s`，按答案分**六档**：

- 解析不到提交 ⇒ `not_a_commit`。账本里这一档全是 **sha256 摘要**（注入器逐臂验盘用的前缀，
  例如 `8b17a662`、`da10bb97b864`），它们不是提交，本来就不该拿轮号去问。
- 解析得到而主题是 `WIP on ` / `index on ` 开头 ⇒ `stash_or_wip`。本轮量到唯一一只：L242 那块记录的
  **共享工作树 stash 事故**——stash 被 drop 之后靠 `git fsck --lost-found` 找回的那枚不可达对象
  `82f4a4cab3e86d66abc06ac9cb60b7a87532144a`。账本点名它是**事故现场本身**（同行已写「提交名 `WIP on optimism20261012`，
  含他会话 40+ 个文件」），不是某一轮的三批 hash。它天然不可归档回轮号：主题里那个 `L241` 是从被 stash 的 HEAD
  **继承**来的，而站点写在 L242 块里 ⇒ 若不设这一档，判据会把它算成 `cross_unnamed` 缺陷（臂
  `test_the_stash_tier_is_what_keeps_that_site_out_of_the_defect_tier` 就是证这一格分流的来路）。
  单独一档、逐只点名：**将来 GC 跑过、对象真消失了，它会从 `stash_or_wip` 掉进 `not_a_commit`，名册当场红**，
  届时按实量重钉而不是偷偷改档——这正是把不可靠性显式钉住而不是假装看不见的做法。
- 主题不带任何 `Lxxx` ⇒ `no_round_subject`：四只跨项目提交（`test(web)`、`fix(augmentor): merge max_items=0…`、
  `chore: restore repository state`、`fix(ai-platform)`），它们不属于 augmentor 轮次链但确实可查。
- 主题轮号集合含本块轮号 ⇒ `same_round`（绝大多数）。
- 不含而**同一行显式写着被引轮的轮号** ⇒ `cross_named`（合法的跨轮叙述，如 `L117（4c979939e）` 那种标签形）。
- 不含而行内也没点名 ⇒ `cross_unnamed` = **缺陷**：读者拿到一枚哈希却不知道它属于哪一轮，
  更不知道它为什么出现在这一块里。本轮修法把两格补上轮号（同行改写、净 0 行），
  此档现量 **空集**，断言写成精确相等而不是上界。

**行内点名的口径为什么敢用整行**：日志 bullet 会折行续排，而站点与它需要归档的那句话在同一物理行上
（L215 的 `4d177e3f3` 与本轮补的「（L212 至 L214 那一批账本补写）」就是）。用整行而不是整段，
是为了让臂 2（删掉点名）能精确复现缺陷形状——臂删的就那几个字符。

**与 l98 的分工**：l98 管清单区那一格的哈希槽与 git 真值对账（R1 到 R6），本守卫管日志块内部的要点归属；
`test_the_whole_face_sits_below_the_log_section` 现场验两面互斥，否则两把尺子会重复判同一格。

**已知盲区（写明，不装）**：① 不带反引号的裸哈希不进面——臂 5 证明这一维确实是形状判据而不是「反正都会红」；
② `ambiguous` 短哈希会被 `--verify` 拒 ⇒ 落 `not_a_commit`，名册是精确集合相等，撞车即红，这是想要的反应；
③ sha256 前缀若与某提交前缀撞车会被当成提交，概率极低且一旦撞车就落进 `same_round` 之外的某一档而红。

**为什么零 parametrize**：`test_parametrize_ids_l97` 与 `test_parametrize_blind_face_l98` 把全仓
parametrize 三面冻成现量数，本守卫走名册字典循环，那两把普查尺子一格未动。

跑法：`python -m pytest tests/unit/test_log_block_bullet_ownership_l264.py -q --no-cov`
"""
import re
import subprocess

from tests.unit import test_backlog_mark_vocab_l253 as other

VOLUMES = other.VOLUMES
ROOT = other.ROOT

#: 反引号内的 hex，允许一个尾随省略号（账本里 sha256 缩写就写成那一形）
HEX = re.compile(r"`([0-9a-f]{6,40})(…)?`")
BLOCK_HEAD = re.compile(r"^### L(\d{1,3})")
ROUNDS = re.compile(r"L(\d{1,3})")

SAME = "same_round"
NAMED = "cross_named"
UNNAMED = "cross_unnamed"
NOROUND = "no_round_subject"
NOTCOMMIT = "not_a_commit"
STASH = "stash_or_wip"
KINDS = (SAME, NAMED, UNNAMED, NOROUND, NOTCOMMIT, STASH)

#: stash 提交的主题格式：`WIP on <branch>: <sha> <被 stash 时 HEAD 的主题>`
WIP_HEADS = ("WIP on ", "index on ")

SUBJECT_CACHE = {}


def subject(short_hash):
    """问 git 这枚短哈希是什么提交的主题；查不到返回 None（含 ambiguous 被拒）"""
    if short_hash in SUBJECT_CACHE:
        return SUBJECT_CACHE[short_hash]
    got = subprocess.run(
        ["git", "-C", str(ROOT), "rev-parse", "--verify", "-q", short_hash + "^{commit}"],
        capture_output=True, text=True, encoding="utf-8", errors="replace").returncode == 0
    out = None
    if got:
        p = subprocess.run(["git", "-C", str(ROOT), "show", "-s", "--format=%s", short_hash],
                           capture_output=True, text=True, encoding="utf-8", errors="replace")
        out = p.stdout.strip() or None
    SUBJECT_CACHE[short_hash] = out
    return out


def sites(volumes=None):
    """日志块内每处反引号 hex ⇒ `{volume, block, line, hash, kind, rounds, text}`（清单区不在面内）"""
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
            if block is None or i - 1 <= bound:
                continue
            for hit in HEX.finditer(line):
                h = hit.group(1)
                subj = subject(h)
                if subj is None:
                    kind, rounds = NOTCOMMIT, frozenset()
                elif subj.startswith(WIP_HEADS):
                    kind, rounds = STASH, frozenset()
                else:
                    rounds = frozenset(int(x) for x in ROUNDS.findall(subj))
                    if block in rounds:
                        kind = SAME
                    elif not rounds:
                        kind = NOROUND
                    elif rounds & ({int(x) for x in ROUNDS.findall(line)} - {block}):
                        kind = NAMED
                    else:
                        kind = UNNAMED
                out.append({"volume": name, "block": block, "line": i, "hash": h,
                            "kind": kind, "rounds": rounds, "text": line})
    return out


def tally(rows):
    got = {k: 0 for k in KINDS}
    for r in rows:
        got[r["kind"]] += 1
    return got


def names(rows, kind):
    return {(r["block"], r["hash"]) for r in rows if r["kind"] == kind}


#: ---- 现量（L264 补注两处轮号、并按实量分出 stash 档之后）----
#: 站点面只有一卷出数：卷一的日志块里一处反引号 hex 都没有（`test_volume_one_contributes_no_site_at_all` 钉住），
#: 所以名册一律写成 `(块轮号, hash)` 而不带卷名。
MEASURED_SITES = 76
MEASURED_COUNTS = {SAME: 41, NAMED: 7, UNNAMED: 0, NOROUND: 6, NOTCOMMIT: 21, STASH: 1}
#: 合法的跨轮叙述：`L117（4c979939e）` 那一族标签形，与本轮补注的两格。
MEASURED_NAMED = {(114, "87174e26b"), (116, "4c979939e"), (116, "0bf96fbec"), (116, "2fd83c9b2"),
                  (215, "4d177e3f3"), (238, "29b715600"), (263, "486c4f6ec")}
#: 主题不带轮号的提交：四只跨项目（`test(web)` / `fix(augmentor): merge…` / `chore: restore…` / `fix(ai-platform)`）。
#: `8041df804` 在同一行里被点名三次 ⇒ 处数 6 而唯一名册 4 只，那一维由 `test_the_repeat_is_a_single_line_triple_mention` 闭合。
MEASURED_NOROUND = {(104, "284d89e29"), (144, "cf35b5984"), (212, "4f0cf03d1"), (236, "8041df804")}
MEASURED_NOROUND_UNIQUE = 4
#: git 里查不到的 hex 全是 sha256 摘要（注入器逐臂验盘用的前缀或全文），不是提交
MEASURED_NOTCOMMIT = {(229, "da10bb97b864"), (229, "b0d77707eb37"), (229, "89877d7bdbbf"),
                      (230, "8b17a662"), (234, "cc1a80b8"),
                      (234, "c17d122404c30688de4f69a767f6be3e84262152"),
                      (235, "8b457088"), (235, "2020a211"), (235, "6856f28a"),
                      (236, "bfa38c4f"), (236, "27e15fea"), (239, "13e78b589e8d"),
                      (240, "df54edca9a61"), (246, "798206eed79354a5"), (246, "21745677c1d0"),
                      (247, "6f405bf0099ef1e9"), (248, "e9722a89d092ebf6"), (248, "be6c8ac9275a4737"),
                      (248, "76aeace0d4096f26"), (248, "58b568468a3c4978"), (260, "51ca546db")}
#: 唯一的 stash 引用：L242 那块记录共享工作树 stash 事故时点名的那枚 `git fsck` 不可达对象。
#: 它不是任何一轮的三批 hash，也不该被归档回轮号——单独一档钉住，将来对象消失就掉进 `not_a_commit` 而红。
MEASURED_STASH = {(242, "82f4a4cab3e86d66abc06ac9cb60b7a87532144a")}
#: 卷一的日志块里一处反引号 hex 都没有 ⇒ 这一档是量出来的空集，不是没写
MEASURED_VOLUME_ONE_SITES = 0


class TestTheOwnershipFace:
    def test_the_face_is_the_measured_sites(self):
        assert len(sites()) == MEASURED_SITES

    def test_each_kind_is_the_measured_count(self):
        got = tally(sites())
        assert got == MEASURED_COUNTS, got

    def test_the_six_kinds_add_up_to_the_face(self):
        """六档闭合：总数等于分档之和——漏一档就加不回去"""
        got = tally(sites())
        assert sum(got.values()) == len(sites()) == MEASURED_SITES

    def test_the_cross_round_named_roster_is_exactly_this(self):
        assert names(sites(), NAMED) == MEASURED_NAMED

    def test_the_no_round_subject_roster_is_exactly_this(self):
        assert names(sites(), NOROUND) == MEASURED_NOROUND

    def test_the_not_a_commit_roster_is_exactly_this(self):
        assert names(sites(), NOTCOMMIT) == MEASURED_NOTCOMMIT

    def test_the_stash_reference_is_the_measured_one(self):
        """账本引用一枚 stash 是本轮量到的事实：恰好一只，多一只少一只都红"""
        assert names(sites(), STASH) == MEASURED_STASH

    def test_the_stash_tier_is_what_keeps_that_site_out_of_the_defect_tier(self):
        """档间分流不是笔误：`stash_or_wip` 这一档存在的**唯一后果**就是把 L242 那枚 40 位从
        `cross_unnamed` 里摘出去。收掉判据（内存里把 `WIP_HEADS` 清空，零落盘）⇒ 缺陷档立刻 1 格、
        stash 档归 0 —— 所以「那一格不算缺陷」是有承重判据的判决，不是我为了变绿临时加的豁免。"""
        import tests.unit.test_log_block_bullet_ownership_l264 as me
        saved = me.WIP_HEADS
        me.WIP_HEADS = ()
        try:
            got = tally(sites())
        finally:
            me.WIP_HEADS = saved
        assert got[STASH] == 0 and got[UNNAMED] == 1, got
        assert got[SAME] == MEASURED_COUNTS[SAME], "摘 stash 档不该动到本块引用"

    def test_the_repeat_is_a_single_line_triple_mention(self):
        """处数 6 而唯一名册 4 只，差的那两笔必须是同一行点名同一枚跨项目提交"""
        rows = [r for r in sites() if r["kind"] == NOROUND]
        assert len(rows) == MEASURED_COUNTS[NOROUND]
        assert len({(r["block"], r["hash"]) for r in rows}) == MEASURED_NOROUND_UNIQUE
        triple = [r for r in rows if sum(1 for x in rows if x["line"] == r["line"]) == 3]
        assert len(triple) == 3 and len({r["hash"] for r in triple}) == 1

    def test_volume_one_contributes_no_site_at_all(self):
        got = {s["volume"] for s in sites()}
        assert got == {"OPTIMIZATION_LOOP_2.md"}, got
        assert sum(1 for s in sites() if s["volume"] == "OPTIMIZATION_LOOP.md") == MEASURED_VOLUME_ONE_SITES

    def test_the_whole_face_sits_below_the_log_section(self):
        """清单区那一格的哈希槽归 l98 管：本面必须与它互斥，否则两把尺子重复判同一格"""
        volumes = other.load_volumes()
        for s in sites(volumes):
            assert s["line"] - 1 > other.log_boundary(volumes[s["volume"]]), s


class TestTheTwoCellsL264Closed:
    """本轮补注的两格：改前它们是 cross_unnamed，改后必须被点名判据接住"""

    def test_the_unnamed_reference_is_now_empty(self):
        assert names(sites(), UNNAMED) == set()
        assert MEASURED_COUNTS[UNNAMED] == 0

    def test_the_two_rows_name_the_round_they_point_at(self):
        for block, h in ((215, "4d177e3f3"), (238, "29b715600")):
            row = [s for s in sites() if s["hash"] == h and s["block"] == block]
            assert len(row) == 1, (h, len(row))
            assert row[0]["kind"] == NAMED, row[0]["kind"]
            named = {int(x) for x in ROUNDS.findall(row[0]["text"])}
            assert named & row[0]["rounds"], "点名维没吃到这一格"


class TestRulerBites:
    """注入臂：全部在内存里改文本，零落盘——落错块那一族的正主必须能被打红"""

    def _patch(self, old, new):
        volumes = other.load_volumes()
        for name, lines in volumes.items():
            try:
                i = next(k for k, s in enumerate(lines) if old in s)
            except StopIteration:
                continue
            lines[i] = lines[i].replace(old, new, 1)
            volumes[name] = lines
            break
        else:
            raise AssertionError("锚点无命中: " + old[:30])
        got = sites(volumes)
        return got, tally(got), names(got, UNNAMED)

    def test_arm_moving_a_bullet_into_the_wrong_block_breaks_the_same_round_count(self):
        """L262 事故的形状：把归位后的 ⑫ 再搬进 L261 块 ⇒ 该处引用从 same_round 掉出，并落进未点名缺陷"""
        volumes = other.load_volumes()
        lines = volumes["OPTIMIZATION_LOOP_2.md"]
        i = next(k for k, s in enumerate(lines) if s.startswith("- ⑫ **记账批（本条所在提交）读数**"))
        bullet = lines.pop(i)
        j = next(k for k, s in enumerate(lines) if s.startswith("### L262"))
        lines.insert(j - 1, bullet)
        got = tally(sites(volumes))
        assert got[SAME] == MEASURED_COUNTS[SAME] - 1, got
        assert got[UNNAMED] == 1, got

    def test_arm_dropping_the_named_round_falls_into_the_unnamed_defect(self):
        _, got, unnamed = self._patch("（`29b715600`，L194 那一族先例", "（`29b715600` 那一族先例")
        assert (238, "29b715600") in unnamed, got
        assert got[NAMED] == MEASURED_COUNTS[NAMED] - 1, got

    def test_arm_inventing_a_hash_that_is_not_in_git_moves_a_site_into_the_sha_tier(self):
        _, got, _ = self._patch("`486c4f6ec`（L262 账本批）", "`0f0f0f0f0`（L262 账本批）")
        assert got[NOTCOMMIT] == MEASURED_COUNTS[NOTCOMMIT] + 1, got
        assert got[NAMED] == MEASURED_COUNTS[NAMED] - 1, got

    def test_arm_pointing_at_another_rounds_commit_without_naming_it(self):
        """把 L116 块里的标签轮号删掉 ⇒ 同处从 cross_named 掉进 cross_unnamed"""
        _, got, unnamed = self._patch("**L117（`4c979939e`）六模块退化输入", "**（`4c979939e`）六模块退化输入")
        assert (116, "4c979939e") in unnamed, got
        assert got[UNNAMED] == 1, got

    def test_arm_a_pointing_at_the_stash_from_a_clean_site_lands_the_stash_tier(self):
        """stash 档有牙齿：把一处本块引用换成那枚 40 位 stash ⇒ STASH +1、same_round −1"""
        _, got, _ = self._patch("账本批（`486c4f6ec`）", "账本批（`82f4a4cab3e86d66abc06ac9cb60b7a87532144a`）")
        assert got[STASH] == MEASURED_COUNTS[STASH] + 1, got
        assert got[SAME] == MEASURED_COUNTS[SAME] - 1, got

    def test_arm_a_negative_prose_shape_is_not_a_site(self):
        """不带反引号的裸 hex 不进面（已知盲区，钉成断言而不是靠人记住）"""
        volumes = other.load_volumes()
        before = len(sites(volumes))
        lines = volumes["OPTIMIZATION_LOOP_2.md"]
        i = next(k for k, s in enumerate(lines) if "`486c4f6ec`（L262 账本批）" in s)
        lines[i] = lines[i].replace("`486c4f6ec`（L262 账本批）", "486c4f6ec（L262 账本批）", 1)
        assert len(sites(volumes)) == before - 1


class TestRulerIsProven:
    """反空转：判据必须真的问到了 git，否则「same_round 41 处」可能是恒真"""

    def test_the_git_side_is_alive(self):
        head = subprocess.run(["git", "-C", str(ROOT), "rev-parse", "--short", "HEAD"],
                              capture_output=True, text=True,
                              encoding="utf-8", errors="replace").stdout.strip()
        assert subject(head), "git 解析不了 HEAD ⇒ 整把尺子在空转"
        assert int(ROUNDS.findall(subject(head))[0]) >= 200, subject(head)

    def test_the_code_commit_of_this_round_is_resolvable(self):
        """L263 代码批在案：它必须解析得到且主题点着 L263——名册里的历史证据不靠记忆"""
        assert "L263" in (subject("be06c5dfe") or ""), subject("be06c5dfe")

    def test_the_same_round_tier_is_not_an_empty_acquisition(self):
        got = tally(sites())
        assert got[SAME] >= 40, got

    def test_abbreviated_sha_prefixes_are_resolved_as_not_commits(self):
        """`8b17a662` 与 `cc1a80b8…` 两形：一形不带省略号、一形带，都必须落 sha 档而不是被当成提交"""
        got = names(sites(), NOTCOMMIT)
        assert (230, "8b17a662") in got
        assert (234, "cc1a80b8") in got

    def test_the_census_reads_both_volumes(self):
        """反空转：语料面真的把两卷都读进来了，只是卷一那一面本来为空"""
        volumes = other.load_volumes()
        assert set(volumes) == set(VOLUMES)
        assert any(len(lines) > 2000 for lines in volumes.values())
