# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""账本每一处点名的提交哈希，必须还能被某个 ref 归档（L270 立尺，收 A344 的 ①）

**这一轮收的是哪一格**：A344 的表行写着——「日志块点名的提交哈希只要 `git rev-parse --verify`
解析得过就算归档 ⇒ 『这枚提交在不在分支上』全账无人量」，并记下 L265 记账批的现量（面上 60 格里 6 格已被
并行会话的 reset/rebase 改写成游离对象）。那一格的「当前处置」写着尺子一格未动、定级 M、待拍。
本轮按 A344 自己写定的修法 ① 立这把尺子：**零产品代码**，只量「可达」这一维。
剩下的 ②（那批游离点名到底重钉成什么、`git gc` 之后怎么办）**不在本轮决定**，拆成 **A348** 继续待拍，
免得「① 已立尺」被读成「A344 整条清了」。

**面**：两卷账本里**所有**被反引号包住的 `[0-9a-f]{6,40}`（形状尺直接复用 l264 的 `HEX`，两把尺子不许各写一份），
按 `## 循环日志` 边界分成清单区与日志区，区段身份（`round`）从所属的 `- [x] **Lnnn**` 清单行**或** `### Lnnn`
块标题继承，两者都还没出现过就是 `None`。**这与 l264 的面不同**：那把尺子的块号只由 `### Lnnn` 开块，
且标题行本身 `continue` 掉不算站点 ⇒ 卷一日志区那 277 处（那卷没有 `### Lnnn` 形状的标题）与卷二五个块标题行
上的 5 处它一处也看不见；本守卫按边界分区，把那 282 处一起收进来，并另存 `block` / `on_head` 两格专供
与 l264 对账（`test_l264s_face_is_a_strict_subset_of_mine` 现场证包含关系，差集里卷二那 5 枚逐枚点名、
卷一那 277 处按区钉数）。

**判据用 ref 可达而不是 HEAD 祖先**：A344 原文写的是「必须是 HEAD 的祖先」。本轮实测两判据在现面上
**给出同一批游离对象**（`test_the_two_criteria_agree_on_the_current_face`），但共享工作树里并行会话会把 HEAD
切到别的分支（L265 与 L266 各被咬过一次，见 L266 块 ⑮），拿 HEAD 当判据就会把「别人切了分支」误报成
「账本写错了哈希」。⇒ 主判据取 `git rev-list --branches --tags --remotes`，HEAD 那一判据留成对账维，
两者一旦分叉就红，逼下一轮重新看这一格，而不是悄悄换掉一把尺子的语义。

**五档**（每档都是问 git 问出来的，不是按长度猜的——按长度猜会把 sha256 前缀当提交，L264 现场撞过）：

- `reachable`：解析成提交且在 ref 可达面上 ⇒ 绝大多数。
- `dangling`：解析成提交而**任何 ref 都到不了** ⇒ 被改写/被 drop 的游离对象。**这一档必须逐格点名**，
  名册见下；名册外的新格子就是缺陷。
- `stash`：主题以 `WIP on ` / `index on ` 开头 ⇒ stash 对象。它也是不可达的，但账本点名它是**事故现场本身**
  （L242 那块记录的 `git fsck --lost-found` 找回），不是某一轮的批次哈希 ⇒ 单独一档，与 l264 同一口径。
  将来 `git gc` 真跑过、对象消失，它会掉进 `not_a_commit` 而名册当场红——这是显式钉住的不确定性。
- `not_a_commit`：git 里解析不到任何提交对象 ⇒ 账本里的 sha256 摘要（注入器逐臂验盘留下的前缀或全文）。
- `ambiguous`：一枚前缀同时匹配多枚提交对象 ⇒ 现量空集；出现即红，不豁免。

**已知盲区（写明，不装）**：① `not_a_commit` 这一档**只数总量、不逐格点名**（现量 147 处），所以一枚
**写错一位的 commit 哈希**会安静地落进这一档而不红——本轮用 `test_a_fabricated_hash_lands_in_the_count_tier`
把这一格量成案面而不是辩解，并**立 A349 待拍**（要不要逐格点名，与 A333 说的「名册型守卫随账本变长的代价」
同批处置）；② 不带反引号的裸哈希不进面（与 l264 同一形状判据，`test_a_bare_hash_without_backticks_stays_out`
双向证明）；③ sha256 前缀若与某枚提交的前缀撞车会被当提交——概率极低，且一旦撞车就会落进名册外的
`dangling` 格而红。

**为什么零 parametrize**：`test_parametrize_ids_l97` 与 `test_parametrize_blind_face_l98` 把全仓 parametrize
三面冻成现量数，本守卫走名册字典循环，那两把普查尺子一格未动。

跑法：`python -m pytest tests/unit/test_ledger_hash_reachability_l270.py -q --no-cov`
"""
import re
import subprocess

from tests.unit import test_backlog_mark_vocab_l253 as other
from tests.unit import test_log_block_bullet_ownership_l264 as l264

ROOT = other.ROOT
VOLUMES = other.VOLUMES
#: 形状尺与 l264 共用一把：本守卫只加「可达」这一维，不另起一套 hex 形状口径
HEX = l264.HEX
BLOCK_HEAD = l264.BLOCK_HEAD
CHECK = re.compile(r"^- \[[ x]\] \*\*L(\d{1,3})\*\*")

REACHABLE = "reachable"
DANGLING = "dangling"
STASH = "stash"
NOTCOMMIT = "not_a_commit"
AMBIGUOUS = "ambiguous"
TIERS = (REACHABLE, DANGLING, STASH, NOTCOMMIT, AMBIGUOUS)

#: stash 提交的主题格式（与 l264 同一档口径）
WIP_HEADS = ("WIP on ", "index on ")
#: L242 记录的那枚 `git fsck` 找回的不可达 stash 对象（40 位全文）
STASH_HEX = "82f4a4cab3e86d66abc06ac9cb60b7a87532144a"

_CACHE = {}


def _git(*args):
    return subprocess.run(["git", "-C", str(ROOT), *args], capture_output=True,
                          text=True, encoding="utf-8", errors="replace")


def commit_objects():
    """库里全部 commit 对象的全哈希集合（一次 `--batch-check`，不逐格起子进程）"""
    if "co" not in _CACHE:
        rows = _git("cat-file", "--batch-all-objects", "--batch-check").stdout.splitlines()
        _CACHE["co"] = {x.split()[0] for x in rows if len(x.split()) >= 3 and x.split()[1] == "commit"}
    return _CACHE["co"]


def reachable():
    """`--branches --tags --remotes` 的可达全集（主判据）"""
    if "re" not in _CACHE:
        _CACHE["re"] = set(_git("rev-list", "--branches", "--tags", "--remotes").stdout.split())
    return _CACHE["re"]


def head_ancestors():
    """HEAD 的祖先集（对账维，专门用来抓两判据分叉）"""
    if "hd" not in _CACHE:
        _CACHE["hd"] = set(_git("rev-list", "HEAD").stdout.split())
    return _CACHE["hd"]


def resolve(short_hash):
    """短哈希 ⇒ (全哈希 or None, 命中的提交对象数)；>1 即歧义"""
    hits = [c for c in commit_objects() if c.startswith(short_hash)]
    return (hits[0] if len(hits) == 1 else None), len(hits)


def subject(full):
    """提交主题；可达面上的走一次 `git log` 全量取，面外（含游离）逐枚问"""
    if "log" not in _CACHE:
        rows = _git("log", "--format=%H%x09%s").stdout.splitlines()
        _CACHE["log"] = {k: v for k, v in (x.split("\t", 1) for x in rows if "\t" in x)}
    if full in _CACHE["log"]:
        return _CACHE["log"][full]
    if full not in _CACHE.setdefault("subject", {}):
        _CACHE["subject"][full] = (_git("show", "-s", "--format=%s", full).stdout or "").strip()
    return _CACHE["subject"][full]


def tier_of(short_hash):
    """(档, 全哈希 or None)"""
    full, hits = resolve(short_hash)
    if hits > 1:
        return AMBIGUOUS, None
    if full is None:
        return NOTCOMMIT, None
    if subject(full).startswith(WIP_HEADS):
        return STASH, full
    return (REACHABLE if full in reachable() else DANGLING), full


def sites(volumes=None):
    """两卷每一处反引号 hex ⇒ `{volume, region, round, line, hex, tier, full}`

    Args:
        volumes: `{卷名: [行]}`，默认按 l253 的 `load_volumes()` 读字节并归一成 LF

    Returns:
        list，按卷与行号顺序
    """
    volumes = volumes if volumes is not None else other.load_volumes()
    out = []
    for name, lines in volumes.items():
        bound = other.log_boundary(lines)
        blk = None
        block = None
        for i, line in enumerate(lines, 1):
            hit = CHECK.match(line)
            if hit:
                blk = int(hit.group(1))
            hit = BLOCK_HEAD.match(line)
            on_head = bool(hit)
            if hit:
                block = blk = int(hit.group(1))
            for m in HEX.finditer(line):
                tier, full = tier_of(m.group(1))
                out.append({"volume": name, "region": "log" if i - 1 > bound else "checklist",
                            "round": blk, "line": i, "hex": m.group(1),
                            "tier": tier, "full": full,
                            #: `block` / `on_head` 只为与 l264 对账而留：那把尺子的块号**只**由 `### Lnnn` 开块
                            #: 且标题行本身不算站点，我的 `round` 还会从清单行继承，所以两面不等。
                            "block": block if i - 1 > bound else None, "on_head": on_head})
    return out


def tally(rows):
    got = {k: 0 for k in TIERS}
    for r in rows:
        got[r["tier"]] += 1
    return got


def cells(rows, tier):
    """档内逐格点名：`{(卷, 区段, 轮号, hash): 处数}`"""
    got = {}
    for r in rows:
        if r["tier"] == tier:
            key = (r["volume"], r["region"], r["round"], r["hex"])
            got[key] = got.get(key, 0) + 1
    return got


def defect_cells(rows):
    """名册外的游离格 = 缺陷：新出现一处「解析得过但任何 ref 都到不了」的点名就落在这里"""
    got = cells(rows, DANGLING)
    return {k: v for k, v in got.items() if k not in DANGLING_ROSTER or DANGLING_ROSTER[k] != v}


#: ---- 现量（L270 立尺；记账批按账本批与 ⑬ 落盘后的红读数重钉）----
#: 站点总数 = 卷一 389（清单区 112 + 日志区 277）+ 卷二 260（清单区 124 + 日志区 136）。
#: **L270 记账批重量**：634 → 642 → **643**（账本批的清单行点名代码批那一枚 ⇒ 卷二清单区 120 → 121，
#: L270 块 ⑤ 逐枚点名七枚不可达对象 ⇒ 卷二日志区 125 → 132，记账批 ⑬ 再点名账本批那一枚 ⇒ 133。
#: 立尺当场的红读数 `assert 643 == 634` 留着，账本批落盘后首跑那一次是 `assert 642 == 634`）。
#: **L271 记账批重量**：643 → 648 → **649**（卷二清单区 +3：L271 清单行点名代码批 `c2735dd69`、
#: A352 表行点名搬迁方两枚 `e4ce1ae36` / `94f7f05cc`；卷二日志区 +3：L271 块 ⑩ 把同两枚再点一次、
#: 块 ⑬ 点名账本批 `cd47d9845`。四枚全是可达对象 ⇒ 只动 `reachable` 一档。红读数 `assert 649 == 643`）。
#: **L272 记账批重量**：649 → **651**（卷二清单区 +1：L272 清单行点名代码批 `59bb04cac`；卷二日志区 +1：
#: L272 块 ⑬ 点名账本批 `6f62ece26`。两枚全是可达对象 ⇒ 仍只动 `reachable` 一档。红读数 `assert 651 == 649`）。
MEASURED_SITES = 651
#: 五档闭合：469 + 32 + 3 + 147 + 0 = 651。
MEASURED_TIERS = {REACHABLE: 469, DANGLING: 32, STASH: 3, NOTCOMMIT: 147, AMBIGUOUS: 0}
MEASURED_REGIONS = {("OPTIMIZATION_LOOP.md", "checklist"): 112,
                    ("OPTIMIZATION_LOOP.md", "log"): 277,
                    ("OPTIMIZATION_LOOP_2.md", "checklist"): 125,
                    ("OPTIMIZATION_LOOP_2.md", "log"): 137}
#: 唯一 hex 只数（387 枚），与站点数（651 处）是两个维：同一枚被多行重提是账本常态。
#: **L271 记账批重量**：381 → **385**（新点名四只：本账代码批与账本批各一枚 + 搬迁方两枚）。红读数 `assert 385 == 381`。
#: **L272 记账批重量**：385 → **387**（新点名两只：本账代码批与账本批各一枚）。红读数 `assert 387 == 385`。
MEASURED_UNIQUE_HEX = 387
#: 不可达对象只数：6 枚游离 + 1 枚 stash = 7 枚（L265 记账批 ⑱ 量的那批，本轮按全账两卷重量）
MEASURED_UNIQUE_DANGLING = 6
#: 名册外的游离格必须恰好为空集——多一处就红，这一条是整把尺子的牙齿所在
MEASURED_DEFECTS = {}
#: `not_a_commit` 逐格不点名，只按四个区数总量（已知盲区 ①）
MEASURED_NOTCOMMIT_REGIONS = {("OPTIMIZATION_LOOP.md", "checklist"): 15,
                              ("OPTIMIZATION_LOOP.md", "log"): 95,
                              ("OPTIMIZATION_LOOP_2.md", "checklist"): 16,
                              ("OPTIMIZATION_LOOP_2.md", "log"): 21}
#: l264 的面在本尺子里是真子集；差集现量 282 处 = 卷一日志区 277（那卷的日志块标题不是 `### Lnnn` 形状，
#: 那把尺子一格都开不出块）+ 卷二五个块标题行（l264 见到标题就 `continue`；那五枚逐枚点名在
#: `MEASURED_OUTSIDE_L264_VOLUME_TWO`，本账插行会平移它们的物理行号，所以这里钉哈希而不是钉行号）。
MEASURED_L264_SITES = 132  # **L272 记账批重量**：131 → **132**（L272 块 ⑬ 点名账本批那一枚，与 l264 同批对上）。红读数 `assert 132 == 131`。**L271 记账批重量**：128 → 130 → **131**（L271 块 ⑩ 那两枚 + 块 ⑬ 那枚，与 l264 同批对上）。红读数 `assert 131 == 128`。
MEASURED_OUTSIDE_L264 = 282
MEASURED_OUTSIDE_L264_VOLUME_ONE = 277
MEASURED_OUTSIDE_L264_VOLUME_TWO = frozenset(
    {"9af6ded88", "61d075bf3", "925610098", "0c61ba3dd", "94f7f09a2"})

#: 游离名册：新点名的格全部落在 L265 那一轮（清单行 3 处 + 日志块 23 处），成因是 L265 期间并行会话对本仓暂存区的
#: 两次 amend/reset/rebase——账本在 ⑰⑱ 两格里逐枚点名叫破了这件事，所以这些格**是记录，不是错账**。
#: `1094b0696` → `b9d3ad7b1` → `b2af89396` 是同一次账本批的三代形态，`ee133fc40` 是被 reset 重做掉的主守卫，
#: `930e31f74` / `adc0df075` / `b9d3ad7b1` 是 oa4rust 那三枚被并行会话吞走的提交（主题不带 augmentor 轮号）。
#: **L270 记账批重量**：9 格 → 15 格——L270 块 ⑤ 为「7 枚不可达对象的逐枚归因」把这六枚各重提一次，
#: 名册键是 `(卷, 区段, 轮号, hash)`，历轮重提就是新格 ⇒ 这正是 A333 点名的「名册随账本变长」那笔代价，
#: 也是 A348 那一格要不要把历轮重提压回单点的理由（本轮不替用户拍）。
DANGLING_ROSTER = {
    ("OPTIMIZATION_LOOP_2.md", "checklist", 265, "1094b0696"): 1,
    ("OPTIMIZATION_LOOP_2.md", "log", 265, "1094b0696"): 6,
    ("OPTIMIZATION_LOOP_2.md", "checklist", 265, "b2af89396"): 1,
    ("OPTIMIZATION_LOOP_2.md", "log", 265, "b2af89396"): 5,
    ("OPTIMIZATION_LOOP_2.md", "checklist", 265, "ee133fc40"): 1,
    ("OPTIMIZATION_LOOP_2.md", "log", 265, "ee133fc40"): 4,
    ("OPTIMIZATION_LOOP_2.md", "log", 265, "930e31f74"): 3,
    ("OPTIMIZATION_LOOP_2.md", "log", 265, "b9d3ad7b1"): 3,
    ("OPTIMIZATION_LOOP_2.md", "log", 265, "adc0df075"): 2,
    ("OPTIMIZATION_LOOP_2.md", "log", 270, "1094b0696"): 1,
    ("OPTIMIZATION_LOOP_2.md", "log", 270, "b2af89396"): 1,
    ("OPTIMIZATION_LOOP_2.md", "log", 270, "ee133fc40"): 1,
    ("OPTIMIZATION_LOOP_2.md", "log", 270, "930e31f74"): 1,
    ("OPTIMIZATION_LOOP_2.md", "log", 270, "b9d3ad7b1"): 1,
    ("OPTIMIZATION_LOOP_2.md", "log", 270, "adc0df075"): 1,
}
#: stash 名册与 l264 同一批格（L242、L264 与 L270 三个块点名同一枚 40 位对象）
STASH_ROSTER = {("OPTIMIZATION_LOOP_2.md", "log", 242, STASH_HEX): 1,
                ("OPTIMIZATION_LOOP_2.md", "log", 264, STASH_HEX): 1,
                ("OPTIMIZATION_LOOP_2.md", "log", 270, STASH_HEX): 1}


class TestTheReachabilityFace:
    def test_the_face_is_the_measured_sites(self):
        assert len(sites()) == MEASURED_SITES

    def test_each_tier_is_the_measured_count(self):
        assert tally(sites()) == MEASURED_TIERS

    def test_the_five_tiers_partition_the_face(self):
        """五档闭合：分档之和等于总数 ⇒ 没有一处站点被漏归档"""
        got = tally(sites())
        assert sum(got.values()) == len(sites()) == MEASURED_SITES

    def test_the_region_split_is_the_measured_four_numbers(self):
        got = {}
        for r in sites():
            got[(r["volume"], r["region"])] = got.get((r["volume"], r["region"]), 0) + 1
        assert got == MEASURED_REGIONS

    def test_the_unique_hex_count_is_the_measured_number(self):
        assert len({r["hex"] for r in sites()}) == MEASURED_UNIQUE_HEX

    def test_no_cited_prefix_is_ambiguous(self):
        """歧义前缀现量空集：一枚短哈希同时指两枚提交就红，不放过"""
        assert sorted({r["hex"] for r in sites() if r["tier"] == AMBIGUOUS}) == []

    def test_the_dangling_roster_is_exact(self):
        assert cells(sites(), DANGLING) == DANGLING_ROSTER

    def test_the_dangling_roster_object_count_is_the_measured_number(self):
        assert len({k[3] for k in cells(sites(), DANGLING)}) == MEASURED_UNIQUE_DANGLING

    def test_there_is_no_defect_cell_outside_the_roster(self):
        """整把尺子的判决格：名册外的游离点名一处都不许有"""
        assert defect_cells(sites()) == MEASURED_DEFECTS

    def test_the_stash_cells_are_exact(self):
        assert cells(sites(), STASH) == STASH_ROSTER

    def test_volume_one_names_no_unreachable_commit_at_all(self):
        """卷一 389 处点名全在可达面上 ⇒ 卷二那 32 处不是全账通病，是 L265 一轮的事故现场（量出来的，不是推的）"""
        assert [r for r in sites() if r["volume"] == VOLUMES[0] and r["tier"] in (DANGLING, STASH)] == []

    def test_the_not_a_commit_tier_is_counted_by_region(self):
        got = {}
        for r in sites():
            if r["tier"] == NOTCOMMIT:
                got[(r["volume"], r["region"])] = got.get((r["volume"], r["region"]), 0) + 1
        assert got == MEASURED_NOTCOMMIT_REGIONS

    def test_the_two_criteria_agree_on_the_current_face(self):
        """ref 可达与 HEAD 祖先这两判据在现面上**给同一批对象** ⇒ 换判据没有放松任何一格；一旦分叉就红"""
        ref_dangling = {r["hex"] for r in sites() if r["tier"] == DANGLING}
        hd = head_ancestors()
        by_head = {r["hex"] for r in sites() if r["full"] is not None and r["tier"] != STASH
                   and r["full"] not in hd}
        assert ref_dangling == by_head

    def test_l98_reads_only_volume_one(self):
        """A344 说「全账无人量」的前提：管哈希槽与 git 真值对账那把尺子只读卷一，所以卷二清单区这 125 处一直没人问
        （**L272 记账批重量**：124 → **125**，随 `MEASURED_REGIONS` 同批，红读数 `assert 125 == 124`；
        **L271 记账批重量**：121 → **124**，红读数 `assert 124 == 121`）"""
        from tests.unit import test_ledger_hash_slots_l98 as l98
        assert l98.LEDGER_PATH.name == other.VOLUMES[0]
        assert MEASURED_REGIONS[("OPTIMIZATION_LOOP_2.md", "checklist")] == 125

    def test_l264s_face_is_a_strict_subset_of_mine(self):
        """与 l264 的分工：它那 132 处全在我的面里，我多出的 282 处是它按「`### Lnnn` 开块 + 标题行不算站点」
        的分区口径看不见的（卷一整片日志区 + 卷二五个标题行）。对账按**处**算而不是按去重后的格算，
        同一行重提同一枚哈希在两把尺子里都各算两处。"""
        rows = [r for r in sites() if r["region"] == "log"]
        theirs = [(r["volume"], r["line"], r["hex"]) for r in rows if r["block"] is not None and not r["on_head"]]
        mine = [(r["volume"], r["line"], r["hex"]) for r in rows]
        assert len(theirs) == MEASURED_L264_SITES == l264.MEASURED_SITES
        assert set(theirs) <= set(mine), sorted(set(theirs) - set(mine))
        extra = [t for t in mine if t not in set(theirs)]
        assert len(extra) == MEASURED_OUTSIDE_L264
        assert sum(1 for v, _, _ in extra if v == VOLUMES[0]) == MEASURED_OUTSIDE_L264_VOLUME_ONE
        assert {h for v, _, h in extra if v == VOLUMES[1]} == set(MEASURED_OUTSIDE_L264_VOLUME_TWO)


class TestTheRulerHasTeeth:
    def test_dropping_an_ancestor_moves_its_cited_sites_into_the_dangling_tier(self):
        """可达判据不是摆设：内存里把 L269 代码批那枚从可达面摘掉 ⇒ 它立刻从 `reachable` 走到 `dangling`、
        缺陷格恰好多出它那一格；复原后重跑与基线逐格相同。零落盘。"""
        target = "e3baad45a"
        full, hits = resolve(target)
        assert hits == 1 and full in reachable()
        base = sites()
        assert [r["tier"] for r in base if r["hex"] == target] == [REACHABLE]
        saved = _CACHE["re"]
        _CACHE["re"] = saved - {full}
        try:
            got = sites()
            assert [r["tier"] for r in got if r["hex"] == target] == [DANGLING]
            assert tally(got)[DANGLING] == MEASURED_TIERS[DANGLING] + 1
            assert tally(got)[REACHABLE] == MEASURED_TIERS[REACHABLE] - 1
            assert defect_cells(got) == {("OPTIMIZATION_LOOP_2.md", "checklist", 269, target): 1}
        finally:
            _CACHE["re"] = saved
        assert sites() == base

    def test_the_roster_is_not_a_blanket_tolerance(self):
        """名册不是「游离的一律放过」：合成一处**新**的点名——同一枚已知游离对象，但落在名册里没有的
        区段身份（L270 那一行）上 ⇒ 缺陷格当场非空且恰好是那一格。只在内存里改面，不落盘。"""
        volumes = other.load_volumes()
        lines = volumes[VOLUMES[1]]
        probe = "- [x] **L270** 假造的游离点名 `930e31f74`（臂写在本文件里，不落进账本）\n"
        volumes[VOLUMES[1]] = lines[:2] + [probe.rstrip("\n")] + lines[2:]
        try:
            got = defect_cells(sites(volumes))
            assert got == {("OPTIMIZATION_LOOP_2.md", "checklist", 270, "930e31f74"): 1}
        finally:
            volumes[VOLUMES[1]] = lines
        assert len(sites()) == MEASURED_SITES

    def test_a_bare_hash_without_backticks_stays_out(self):
        """形状判据双向有效：不带反引号不进面（加了就 +1）⇒ 面不是「凡是 hex 都算」"""
        base = len(sites())
        volumes = other.load_volumes()
        lines = volumes[VOLUMES[1]]
        volumes[VOLUMES[1]] = lines[:2] + ["散文里的裸哈希 930e31f74 不算站点"] + lines[2:]
        try:
            assert len(sites(volumes)) == base
            volumes[VOLUMES[1]] = lines[:2] + ["散文里的反引号哈希 `930e31f74` 算站点"] + lines[2:]
            assert len(sites(volumes)) == base + 1
        finally:
            volumes[VOLUMES[1]] = lines
        assert len(sites()) == base

    def test_a_fabricated_hash_lands_in_the_count_tier(self):
        """已知盲区 ① 的量法：一枚编造的哈希落进 `not_a_commit`（只数总量的那一档）而**不**进缺陷格 ⇒
        「写错一位的 commit 哈希不会被这把尺子抓到」是量出来的事实，因此立 A349 而不是装作没有。"""
        volumes = other.load_volumes()
        lines = volumes[VOLUMES[1]]
        volumes[VOLUMES[1]] = lines[:2] + ["- [x] **L900** 编造 `fffffffffff` 一枚"] + lines[2:]
        try:
            got = sites(volumes)
            fresh = [r for r in got if r["hex"] == "fffffffffff"]
            assert [r["tier"] for r in fresh] == [NOTCOMMIT]
            assert defect_cells(got) == MEASURED_DEFECTS
        finally:
            volumes[VOLUMES[1]] = lines
        assert len(sites()) == MEASURED_SITES
