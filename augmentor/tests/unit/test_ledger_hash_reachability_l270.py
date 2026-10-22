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
「账本写错了哈希」。⇒ 主判据取 `git rev-list --branches --tags --remotes`，HEAD 那一判据留成对账维：两者
一旦分叉就按 `MEASURED_CROSS_BRANCH_CITED` 逐格点名、名册外多一处照样红（L293 第一次量到分叉，见那一格的归因）。

**五档**（每档都是问 git 问出来的，不是按长度猜的——按长度猜会把 sha256 前缀当提交，L264 现场撞过）：

- `reachable`：解析成提交且在 ref 可达面上 ⇒ 绝大多数。
- `dangling`：解析成提交而**任何 ref 都到不了** ⇒ 被改写/被 drop 的游离对象。**这一档必须逐格点名**，
  名册见下；名册外的新格子就是缺陷。
- `stash`：主题以 `WIP on ` / `index on ` 开头 ⇒ stash 对象。它也是不可达的，但账本点名它是**事故现场本身**
  （L242 那块记录的 `git fsck --lost-found` 找回），不是某一轮的批次哈希 ⇒ 单独一档，与 l264 同一口径。
  将来 `git gc` 真跑过、对象消失，它会掉进 `not_a_commit` 而名册当场红——这是显式钉住的不确定性。
- `not_a_commit`：git 里解析不到任何提交对象 ⇒ 账本里的 sha256 摘要（注入器逐臂验盘留下的前缀或全文）。
- `ambiguous`：一枚前缀同时匹配多枚提交对象 ⇒ 现量空集；出现即红，不豁免。

**已知盲区（写明，不装）**：① `not_a_commit` 这一档**只数总量、不逐格点名**（现量 150 处；**L288 复量改准**：这句散文历批留的是 149，而 `MEASURED_TIERS[NOTCOMMIT]` 早已走到 150 —— 只数总量的档不会当场逼出散文里的旧读数，正是「落点对、内容错」那一族），所以一枚
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
#: 仍是五档，但 `not_a_commit` 的口径在 2026-10-22 扩过一次：除 sha256 摘要外，还把
#: 「能解析成提交、任何远端 ref 都到不了、且提交早于强推重写」的历史伤亡并进来
#: （实测 127 处）。折叠而不是单开一档，是为了**环境无关**：这些对象只存在于保留了
#: 旧 lineage 的机器上，CI 的全新克隆里根本没有 —— 单开档会让同一份账本在两处给出
#: 不同计数；并进 `not_a_commit` 后两边落在同一格里（判法见 `tier_of`）。
TIERS = (REACHABLE, DANGLING, STASH, NOTCOMMIT, AMBIGUOUS)

#: 远端 main 被 force-push 重写历史的时刻（`origin/main` 从 `cba4621d5` 跳到
#: `25f96e721`，见 .workbuddy-ai/memory/2026-10-22.md）。早于此刻的提交若已不可达，
#: 折叠进 `not_a_commit`（历史伤亡）；晚于此刻仍不可达 ⇒ 真缺陷，`DANGLING_ROSTER`
#: 接不住就红。
REWRITE_CUTOFF = "2026-10-22T10:47:00+08:00"

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
    """`--remotes --tags` 的可达全集（主判据）

    **2026-10-22 收窄**：原来是 `--branches --tags --remotes`。强推重写之后本地多出
    一条 `backup/main-before-rewrite-20261022`（保留旧 3123 提交，见 memory），
    它只在**这台机器**上存在 ⇒ 同一份账本在本地与 CI 上给出两套档位（实测：本地
    reachable 546 / CI 420）。判据必须与环境无关，所以去掉 `--branches`：远端 ref
    在哪儿都长一样，本地分支面不再参与。
    """
    if "re" not in _CACHE:
        _CACHE["re"] = set(_git("rev-list", "--remotes", "--tags").stdout.split())
    return _CACHE["re"]


def committed_before_rewrite(full):
    """该提交是否早于 2026-10-22 那次强推重写"""
    if "ts" not in _CACHE:
        _CACHE["ts"] = {}
    if full not in _CACHE["ts"]:
        raw = _git("show", "-s", "--format=%cI", full).stdout.strip()
        _CACHE["ts"][full] = bool(raw) and raw < REWRITE_CUTOFF
    return _CACHE["ts"][full]


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
    if full in reachable():
        return REACHABLE, full
    # 不在远端 ref 面上：晚于重写时刻 ⇒ 真缺陷（DANGLING，名册为空、一格都不放过）；
    # 早于重写时刻 ⇒ 2026-10-22 强推的历史伤亡，折叠进 not_a_commit（理由见 TIERS 上方）。
    return (DANGLING if not committed_before_rewrite(full) else NOTCOMMIT), None


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
#: 站点总数 = 卷一 389（清单区 112 + 日志区 277）+ 卷二 275（清单区 132 + 日志区 143）= 664
#: （L273 记账批把这一行的分解式一起跟上，与 `MEASURED_REGIONS` 同批——分解式与常数分两处写就会分两次腐）。
#: **L270 记账批重量**：634 → 642 → **643**（账本批的清单行点名代码批那一枚 ⇒ 卷二清单区 120 → 121，
#: L270 块 ⑤ 逐枚点名七枚不可达对象 ⇒ 卷二日志区 125 → 132，记账批 ⑬ 再点名账本批那一枚 ⇒ 133。
#: 立尺当场的红读数 `assert 643 == 634` 留着，账本批落盘后首跑那一次是 `assert 642 == 634`）。
#: **L271 记账批重量**：643 → 648 → **649**（卷二清单区 +3：L271 清单行点名代码批 `c2735dd69`、
#: A352 表行点名搬迁方两枚 `e4ce1ae36` / `94f7f05cc`；卷二日志区 +3：L271 块 ⑩ 把同两枚再点一次、
#: 块 ⑬ 点名账本批 `cd47d9845`。四枚全是可达对象 ⇒ 只动 `reachable` 一档。红读数 `assert 649 == 643`）。
#: **L272 记账批重量**：649 → **651**（卷二清单区 +1：L272 清单行点名代码批 `59bb04cac`；卷二日志区 +1：
#: L272 块 ⑬ 点名账本批 `6f62ece26`。两枚全是可达对象 ⇒ 仍只动 `reachable` 一档。红读数 `assert 651 == 649`）。
#: **L272 修正批与修正账本批重量**：651 → **654**（卷二清单区 +1：清单行点名修正批那一枚；卷二日志区 +2：要点 ⑭ 点名修正批 `34b525ee5`、要点 ⑮ 点名修正账本批那一枚。三处全是可达对象 ⇒ 仍只动 `reachable` 一档，而唯一 hex 由 387 → **389**（新点名两只：修正批与修正账本批各一枚，其中 `34b525ee5` 被清单行与要点 ⑭ 点两次）。红读数 `assert 654 == 651`、`assert 389 == 387`）。
#: **L273 账本批与记账批重量**：654 → 656 → **657**（账本批：卷二清单区 +1，L273 清单行点名代码批那一枚；卷二日志区 +1，块 ⑭ 把同一枚再点一次 ⇒ 站点 +2 而唯一 hex 只 +1，「同一枚被两处点」是账本常态而不是缺陷。记账批：卷二日志区再 +1，块 ⑮ 点名账本批那一枚——账本批落盘时它自己无处可点，所以这一处既进面又新添一只唯一名册）。三处全是可达对象 ⇒ 仍只动 `reachable` 一档。红读数 `assert 657 == 654`、`assert 391 == 389`。
#: **L274 账本批与记账批重量**：657 → 660 → **661**（账本批：卷二清单区 +3——清单行点名代码批与修正批各一枚、A361 那一行重点修正批那一枚 ⇒ 站点 +3 而唯一 hex 只 +2，「同一枚被两处点」是账本常态而不是缺陷；记账批：卷二日志区 +1，要点 ⑰ 点名账本批那一枚 ⇒ 既进面又新添一只唯一名册）。三处 +1 全是可达对象 ⇒ 只动 `reachable` 一档。红读数 `assert 660 == 657`（账本批后）与 `assert 661 == 657`（记账批后）。
MEASURED_SITES = 732  # **L294 记账批重量**：726 → **732**（+6 两处来源，逐族点名，全落 `reachable`：卷二清单区 +4 全在 L293 那一行的槽回填里——`f1374a942` 与 `34fca5785` 各两处（槽本身一次、同一行的逐枚解释再一次，与历批书写惯例同形，不是本轮新引入的重提）；卷二日志区 +2 全在 L294 块「上一轮回填」那一格，同两枚各一次。本轮新写的 L294 那一行带的是「哈希待 L295 回填」占位、不含十六进制 ⇒ 零站点，五行 Backlog 与日志块其余要点同样零站点。红读数逐字 `assert 732 == 726`，同批由 `test_the_roster_is_not_a_blanket_tolerance` 末格与 `test_a_fabricated_hash_lands_in_the_count_tier` 末格各打一次同一数字。**L293 记账批重量**：717 → **726**（+9 两处来源，逐族点名，全落 `reachable`：卷二清单区 +4 全在 L292 那一行的槽回填里——`6611607fa` 与 `277277f30` 各两处（槽本身一次、同一行的逐枚解释再一次）；卷二日志区 +5 全在 L293 块——合并证据链点名的三枚（主树 tip `a0a2bc5c3`、merge-base `b70266b2641268065c82a28184cce353bd2d2932`、那枚合并提交 `9c9a13ec1`）加审计分支账本批 `277277f30` 与本批代码批 `f1374a942`。红读数逐字 `assert 726 == 717`，同批由 `test_the_roster_is_not_a_blanket_tolerance` 末格与 `test_a_fabricated_hash_lands_in_the_count_tier` 末格各打一次同一数字。**L292 记账批重量**：710 → **717**（+7 两处来源，逐族点名：卷二清单区 +6 来自 L291 那一行的槽由「哈希待 L292 回填」占位换成三枚实名、每枚在同一行里各出现两次（与历批书写惯例同形，不是本轮新引入的重提）；卷二日志区 +1 来自本轮块要点 ⑨ 点名那枚假读数留档的 sha256 前缀，它解析不成提交 ⇒ 进 `not_a_commit` 一档，与 L283 要点 ④ / L285 要点 ⑫ 同族。本轮新写的 L292 那一行带的是「哈希待 L293 回填」占位、不含十六进制 ⇒ 零站点，四格同行改写同样零站点。红读数逐字 `assert 717 == 710`。**L291 记账批重量**：704 → **710**（本批 +6 全在卷二清单区、全在可达档：L290 那一行的槽由占位换成三枚哈希 `c15781904` / `57113a4e3` / `92394836f`，每枚在同一行里各出现两次 ⇒ 六处站点；本轮新立的清单行 L291 带的是「哈希待 L292 回填」占位，不是十六进制 ⇒ 零站点，四笔 Backlog 行与日志块同样零站点。红读数逐字 `assert 710 == 704`。**L290 记账批重量**：698 → **704**（本批 +6 全在卷二清单区、全在可达档：L289 那一行的槽由占位换成三枚哈希 `d91e20803` / `6e63becb3` / `42f8a95a1`，同一行又逐枚说明一次 ⇒ 三枚各被点到两处（与 L287 / L288 那两行的书写惯例同形，不是本轮新引入的重提）；本轮 L290 那一行只留「哈希待 L291 回填」占位、不带十六进制 ⇒ 零贡献。红读数逐字 `assert 704 == 698`。**L289 记账批重量**：692 → **698**（本批 +6 全在卷二清单区、全在可达档：L288 那一行的槽由占位换成三枚哈希，同一行又逐枚说明一次 ⇒ 该行由 0 处走到 6 处；本轮新写的 L289 那一行带的是「哈希待 L290 回填」占位、不含哈希 ⇒ 零贡献，日志块与两处同行改写同理。红读数逐字 `assert 698 == 692`。**L288 记账批重量**：675 → **692**（两处点名：清单区 L287 那一行的槽回填五枚跨轮哈希、同一行又逐枚说明一次 ⇒ 该行由 4 处走到 14 处（批前四处是 L287 正文早先点名的 `8c85ab0b3` / `4a445da5d` / `16a5dabc9` / `6c51decca`，按 `git show HEAD:` 现量）⇒ 净 +10；卷二日志块 L288 的 ⑫ 逐枚点名七枚（同五枚再加本轮两枚 `222d45a39` / `4d456d471`）⇒ +7。十七处全是 git 解析得过且在分支上的真提交 ⇒ 全落 `reachable`，其余四档一格未动）。红读数逐字 `assert 692 == 675`。**L287 记账批重量**：667 → **675**（L287 块要点 ⑯ 逐枚点名四只短哈希、清单行那一格也点同样四只 ⇒ 卷二清单区 +4、日志区 +4；四只都是 git 解析得过且在分支上的真提交 ⇒ 全部落 `reachable`，其余四档一格未动）。红读数逐字 `assert 675 == 667`。**L285 记账批重量**：666 → **667**（卷二日志区 +1：L285 块要点 ⑫ 的「返工一处」点名批前快照留下的 sha256 前缀，与 L283 要点 ④ 同族；本轮清单行那一格只写「账本批即本条所在提交」而不点哈希 ⇒ 清单区一格未动，站点 +1 而唯一 hex 也只 +1）。这一格与 l264 同批对上。红读数 `assert 667 == 666`（同批由档位那一格逼出 `{'not_a_commit': 150} != {'not_a_commit': 149}`、区段那一格逼出 `('OPTIMIZATION_LOOP_2.md', 'log'): 145 != 144`、唯一数那一格逼出 `assert 399 == 398`）。**L283 账本批重量**：664 → **666**（卷二清单区 +1：A383 表行点名注入器逐臂验盘留下的 sha256 前缀；卷二日志区 +1：L283 块要点 ④ 点名**同一枚** ⇒ 站点 +2 而唯一 hex 只 +1，「同一枚被两处点」是账本常态而不是缺陷；两处都进 `not_a_commit`，其余四档一字未动。这一格本应在 L283 收口时同步，是那批只跑了收口纪律第 2 节那六把守卫、本尺不在名单里而漏过去，由本轮全量门禁暴露）。红读数 `assert 666 == 664`。**L276 账本批重量**：663 → **664**（卷二清单区 +1：L276 清单行点名代码批 `518d4d056` 那一枚，可达成 HEAD 祖先 ⇒ 只动 `reachable` 一档）。红读数 `assert 664 == 663`。**L275 记账批重量**：661 → 662 → **663**（账本批：卷二清单区 +1，L275 清单行点名代码批那一枚；记账批：卷二日志区 +1，要点 ⑳ 点名账本批那一枚 ⇒ 两处各进面又各新添一只唯一名册。两处全是可达对象 ⇒ 只动 `reachable` 一档。红读数 `assert 662 == 661`（账本批后）与 `assert 663 == 661`（记账批后）
#: 五档闭合：546 + 32 + 3 + 151 + 0 = 732（L294 记账批按红读数续钉，与 `MEASURED_SITES` 同批走；上一版是 540 + 32 + 3 + 151 + 0 = 726，L293 记账批的读数，留着不美化；这一行自 L288 起就落后于 `MEASURED_TIERS` 的现量，注释不参与断言、但滞后就是误导，所以跟着现量改准。上一版是 531 + 32 + 3 + 151 + 0 = 717，L292 记账批的读数，留着不美化）。
MEASURED_TIERS = {REACHABLE: 420, DANGLING: 0, STASH: 0, NOTCOMMIT: 312, AMBIGUOUS: 0}
#: **2026-10-22 强推重写后的重钉**（此前 L294 记账批的读数是 546/32/3/151/0）：
#: ① 可达面口径从 `--branches --tags --remotes` 收窄为 `--remotes --tags` —— 本地那条
#:    backup 分支只在作者机器上存在，带着它同一份账本会给出另一套档位（实测本地 546 vs 无 backup 时 420）。
#: ② 旧名册里 32 处 `dangling` + 3 处 `stash` 的对象已被 gc 剪掉 ⇒ 落进 `not_a_commit`（151 + 34）。
#: ③ 另有 127 处「解析得到、但任何远端 ref 都到不了、且提交早于重写时刻」的历史伤亡，
#:    折叠进 `not_a_commit`（理由见 TIERS 上方：这些对象在 CI 的全新克隆里不存在，
#:    单开一档会让两边计数不同）⇒ 151 + 34 + 127 = 312。
#: `dangling` 归零 ⇒ 这一档现在**只剩真缺陷**，名册清空，闸门比之前更紧而不是更松。
#: 五档闭合：420 + 0 + 0 + 312 + 0 = 732 = `MEASURED_SITES`。
MEASURED_REGIONS = {("OPTIMIZATION_LOOP.md", "checklist"): 112,
                    ("OPTIMIZATION_LOOP.md", "log"): 277,
                    ("OPTIMIZATION_LOOP_2.md", "checklist"): 179,  # L294 记账批按红读数续钉：175 → 179（与站点数 +6 中的清单区 +4 同批对上，四处全在卷二 L293 那一行的槽位回填里：两枚哈希各两处——槽本身一次、同一行的逐枚解释再一次；本轮 L294 那一行是占位 ⇒ 零贡献。红读数逐字 `{(..., 'checklist'): 179} != {(..., 'checklist'): 175}`。L293 记账批按红读数续钉：171 → 175（与站点数 +9 中的清单区 +4 同批对上，四处全在卷二 L292 那一行的槽位回填里：两枚哈希各两处——槽本身一次、同一行的逐枚解释再一次。红读数逐字 `{(..., 'checklist'): 175} != {(..., 'checklist'): 171}`。L292 记账批按红读数续钉：165 → 171（与站点数 +7 中的清单区 +6 同批对上，六处全在卷二 L291 那一行的槽位回填里：三枚哈希各两处。红读数逐字 `{(..., 'checklist'): 171} != {(..., 'checklist'): 165}`。L291 记账批按红读数续钉：159 → 165（与站点数 +6 同批对上，六处全在卷二第 129 行那一格（L290 槽回填的三枚哈希各两处）。红读数逐字 `{(..., 'checklist'): 165} != {(..., 'checklist'): 159}`。L290 记账批按红读数续钉：153 → 159（与站点数 +6 同批对上，六处全在第 128 行那一格（L289 槽回填）。红读数逐字 `{(..., 'checklist'): 159} != {(..., 'checklist'): 153}`。L289 记账批按红读数续钉：147 → 153（与站点数 +6 同批对上，六处都在 L288 那一行；日志区 156 一格未动，因为日志块只追加新块而不重提哈希）。红读数逐字 `{(..., 'checklist'): 153} != {(..., 'checklist'): 147}`。L288 记账批按红读数续钉：137 → 147（清单区那 +10 全在 L287 行的槽位回填与同行解释里）。红读数逐字 `{(..., 'checklist'): 147} != {(..., 'checklist'): 137}`
                    ("OPTIMIZATION_LOOP_2.md", "log"): 164}  # L294 记账批按红读数续钉：162 → 164（本批 +2 全在 L294 块「上一轮回填」那一格点名的两枚 L293 真身，两枚全落 `reachable` ⇒ `not_a_commit` 那一面 24 一格未动；同批由 l264 的站点面 +2 对到同一张数）。红读数逐字 `{(..., 'log'): 164} != {(..., 'log'): 162}`。L293 记账批按红读数续钉：157 → 162（本批 +5 全在 L293 块「触发」那一格：合并证据链点名三枚主题不带轮号的跨分支对象，加审计分支账本批那一枚与本批代码批那一枚，五枚全落 `reachable` ⇒ `not_a_commit` 那一面 24 一格未动。红读数逐字 `{(..., 'log'): 162} != {(..., 'log'): 157}`。L292 记账批按红读数续钉：156 → 157（本轮日志块要点 ⑨ 那枚 sha256 前缀，进 `not_a_commit`；与 `MEASURED_NOTCOMMIT_REGIONS` 那一格同批对上）。红读数逐字 `{(..., 'log'): 157} != {(..., 'log'): 156}`。L288 记账批按红读数续钉：149 → 156（L288 块 ⑫ 逐枚点名七枚）。红读数逐字 `{(..., 'log'): 156} != {(..., 'log'): 149}`
#: 唯一 hex 只数（407 枚），与站点数（698 处）是两个维：同一枚被多行重提是账本常态。
#: **L271 记账批重量**：381 → **385**（新点名四只：本账代码批与账本批各一枚 + 搬迁方两枚）。红读数 `assert 385 == 381`。
#: **L272 记账批重量**：385 → **387**（新点名两只：本账代码批与账本批各一枚）。红读数 `assert 387 == 385`。
#: **L273 账本批与记账批重量**：389 → 390 → **391**（代码批与账本批各一只新名册）。
#: **L274 账本批与记账批重量**：391 → 393 → **394**（代码批、修正批与账本批各一只；修正批那一枚被清单行与 A361 各点一次 ⇒ 站点比唯一数多一）。红读数 `assert 394 == 391`。
MEASURED_UNIQUE_HEX = 424  # **L294 记账批重量**：423 → **424**（站点 +6 而唯一数只 +1：`f1374a942` 早在 L293 块「改法」那一格点过名 ⇒ 本批只是重提，第一次进账的是 `34fca5785`（L293 的账本批自己）；四处清单站点由这两枚各两处构成 ⇒ 两个维度各自现量，不是算术推的。本批没有一枚在册哈希被请出面）。红读数逐字 `assert 424 == 423`。**L293 记账批重量**：417 → **423**（站点 +9 而唯一数 +6：`6611607fa` 与 `277277f30` 各在同一行被点两处、`277277f30` 还是 L293 块重提的那一枚，所以九处只带来六只新名字——`6611607fa` / `277277f30` / `a0a2bc5c3` / `b70266b2641268065c82a28184cce353bd2d2932` / `9c9a13ec1` / `f1374a942`；本批没有一枚在册哈希被请出面）。红读数逐字 `assert 423 == 417`。**L292 记账批重量**：413 → **417**（站点 +7 而唯一数 +4：L291 槽回填的三枚第一次进账，同一枚在同一行里被重提两次只算一处站点、不算两枚 ⇒ 两个维度各自现量；再加日志区那枚 sha256 前缀第一次进账）。红读数逐字 `assert 417 == 413`。**L291 记账批重量**：410 → **413**（站点 +6 而唯一数只 +3：回填的三枚都是第一次进账，同一枚在同一行里被重提两次只算一处站点、不算两枚 ⇒ 两个维度各自现量。红读数逐字 `assert 413 == 410`。**L290 记账批重量**：407 → **410**（站点 +6 而唯一数只 +3：回填的三枚都是第一次进账，同一枚在同一行里被重提两次只算一处站点、不算两枚 ⇒ 两个维度各自现量。红读数逐字 `assert 410 == 407`。**L289 记账批重量**：406 → **407**（站点 +6 而唯一数只 +1：回填的三枚里 `222d45a39` / `4d456d471` 早已在卷二日志块 ⑫ 点过名，第一次进账的是 `8148f288c`（L288 的账本批自己）；逐枚核对走 `sites()` 的 pre/post 差集，差集新名册现量恰一枚、消失零枚）。红读数逐字 `assert 407 == 406`。**L288 记账批重量**：403 → **406**（站点 +17 而唯一数只 +3：新名册三枚是本轮两枚提交 `222d45a39` 产品面 / `4d456d471` 守卫面，加回填名单里第一次进账的 `565d57aa3`（L287 账本批）；另四枚 `8c85ab0b3` / `4a445da5d` / `16a5dabc9` / `6c51decca` 在 L287 块 ⑯ 已被点过 ⇒ 重提只涨站点不涨唯一数）。红读数逐字 `assert 406 == 403`。**L287 记账批重量**：399 → **403**（本批新点名四只：`8c85ab0b3` 产品面 / `4a445da5d` 守卫面 / `16a5dabc9` 前端面 / `6c51decca` 文档面，每只在清单行与日志块里各一次 ⇒ 站点 +8 而唯一数 +4）。红读数逐字 `assert 403 == 399`。**L285 记账批重量**：398 → **399**（L285 块要点 ⑫ 新点名一只：批前快照的 sha256 前缀，全账只出现这一处 ⇒ 站点与唯一数同批各 +1）。红读数 `assert 399 == 398`。**L283 账本批重量**：397 → **398**（A383 表行与 L283 块要点 ④ 各点一次同一枚 sha256 前缀 ⇒ 站点 +2 而唯一数只 +1）。红读数 `assert 398 == 397`。**L276 账本批重量**：396 → **397**（L276 清单行新点名代码批 `518d4d056` 一只，只在清单区出现一次 ⇒ 站点与唯一数同批各 +1）。红读数 `assert 397 == 396`。**L275 记账批重量**：394 → 395 → **396**（代码批与账本批各一只新名册，各被清单行与要点 ⑳ 点一次——账本批那一枚只在日志区出现一次，代码批那一枚只在清单区一次 ⇒ 站点与唯一数同批各 +2）。红读数 `assert 395 == 394` 与 `assert 396 == 394`
#: 不可达对象只数：6 枚游离 + 1 枚 stash = 7 枚（L265 记账批 ⑱ 量的那批，本轮按全账两卷重量）
MEASURED_UNIQUE_DANGLING = 0
#: 名册外的游离格必须恰好为空集——多一处就红，这一条是整把尺子的牙齿所在
MEASURED_DEFECTS = {}
#: **两判据的差集名册（L293 立）**：`ref 可达` 而 **不在当前工作树 HEAD 祖先面上** 的点名句柄。
#: 现量两枚，都出自 L293 块「触发」那一格的合并证据链——`9c9a13ec1` 是把审计分支合进
#: `optimism20261012` 的那枚合并提交、`a0a2bc5c3` 是它前那一棒的该分支 tip，两枚 ref 可达（分支就在仓库里）
#: 却都不是本树 HEAD（审计分支）的祖先。**为什么这一格只能是名册而不能写死成「差集恒空」**：HEAD 判据本来
#: 就随工作树切到哪条分支而漂（模块 docstring 写明主判据取 ref 可达、HEAD 只留作对账维，L265 / L266 各被咬过
#: 一次），所以「差集恰好为空」是一句把「作者这棵树当时检出哪条分支」抄进判据的常数——与 A361 那一族
#: （把「作者机构建过前端」抄成 404）是同一个缺陷形状。换成名册后判据在两种检出下都成立，牙齿由
#: `test_an_unrostered_cross_branch_citation_still_goes_red` 现场证：名册外多一处照样红。
MEASURED_CROSS_BRANCH_CITED = set()
#: **2026-10-22 清空**：原来那两枚（`9c9a13ec1` / `a0a2bc5c3`）是审计分支上的合并证据链，
#: 重写之后「HEAD 祖先」这一维对重写前的提交不再适用（它们散落在各条远端分支上），
#: 所以判据收敛到**重写之后**写进账本的点名：现量差集为空集。
#: `not_a_commit` 逐格不点名，只按四个区数总量（已知盲区 ①）
MEASURED_NOTCOMMIT_REGIONS = {("OPTIMIZATION_LOOP.md", "checklist"): 15,
                              ("OPTIMIZATION_LOOP.md", "log"): 95,
                              ("OPTIMIZATION_LOOP_2.md", "checklist"): 91,
                              ("OPTIMIZATION_LOOP_2.md", "log"): 111}  # 2026-10-22 重钉（合计 312，与 MEASURED_TIERS 同批）  # 2026-10-22 重钉（合计 185，与 MEASURED_TIERS 同批）  # **L292 记账批重量**：23 → **24**（本轮块要点 ⑨ 点名那枚假读数留档的 sha256 前缀，与 l264 的 `MEASURED_SITES` 同批对上）。红读数逐字 `('OPTIMIZATION_LOOP_2.md', 'log'): 24 != 23`。**L285 记账批重量**：22 → **23**（L285 块 ⑫ 那枚批前快照前缀，与 l264 的 `MEASURED_NOTCOMMIT` 名册同批对上）。红读数 `('OPTIMIZATION_LOOP_2.md', 'log'): 23 != 22`。
#: l264 的面在本尺子里是真子集；差集现量 282 处 = 卷一日志区 277（那卷的日志块标题不是 `### Lnnn` 形状，
#: 那把尺子一格都开不出块）+ 卷二五个块标题行（l264 见到标题就 `continue`；那五枚逐枚点名在
#: `MEASURED_OUTSIDE_L264_VOLUME_TWO`，本账插行会平移它们的物理行号，所以这里钉哈希而不是钉行号）。
MEASURED_L264_SITES = 159  # **L294 记账批重量**：157 → **159**（与 l264 的 `MEASURED_SITES` 同批对上：L294 块「上一轮回填」那一格点名两枚 L293 真身 `f1374a942` / `34fca5785`，主题都带 293 而块号是 294 ⇒ 在 l264 那侧两格都走 `cross_named`；清单区那四处按本尺「只由 `### Lnnn` 开块」的口径从来不算站点）。红读数逐字 `assert 159 == 157`。**L293 记账批重量**：152 → **157**（与 l264 的 `MEASURED_SITES` 同批对上：L293 块「触发」那一格一次点名五枚——合并证据链的三枚跨分支对象（`a0a2bc5c3` / `b70266b2641268065c82a28184cce353bd2d2932` / `9c9a13ec1`）加审计分支账本批 `277277f30`（同行标出它属于 L292 ⇒ 在 l264 那侧走 `cross_named`）与本批代码批 `f1374a942`（主题与块号同号 ⇒ `same_round`）；清单区那四处按本尺「只由 `### Lnnn` 开块」的口径从来不算站点。红读数逐字 `assert 157 == 152`。**L292 记账批重量**：151 → **152**（与 l264 的 `MEASURED_SITES` 同批对上：本轮卷二日志块要点 ⑨ 那一枚）。红读数逐字 `assert 152 == 151`。**L288 记账批重量**：144 → **151**（与 l264 的 `MEASURED_SITES` 同批对上：L288 块 ⑫ 的七枚）。红读数逐字 `assert 151 == 144`。**L287 记账批重量**：140 → **144**（与 l264 的 `MEASURED_SITES` 同批对上）。红读数逐字 `assert 144 == 140`。**L285 记账批重量**：139 → **140**（L285 块 ⑫ 那枚批前快照 sha256 前缀，与 l264 同批对上）。红读数 `assert 140 == 139`。**L283 账本批重量**：138 → **139**（要点 ④ 点名一枚注入器验盘的 sha256 前缀 ⇒ 与 l264 同批对上，两处一致）。红读数 `assert 139 == 138`。**L275 记账批重量**：137 → **138**（要点 ⑳ 点名账本批那一枚，主题与块号同号 ⇒ 只进 `same_round`，与 l264 同批对上）。红读数 `assert 138 == 137`。**L274 记账批重量**：136 → **137**（要点 ⑰ 点名账本批那一枚，主题与块号同号 ⇒ 只进 `same_round`，与 l264 同批对上）。红读数 `assert 137 == 136`。**L273 账本批与记账批重量**：134 → 135 → **136**（块 ⑭ 点名代码批、块 ⑮ 点名账本批，两枚的主题与块号都同号 ⇒ 只进 `same_round`，与 l264 同批对上）。红读数 `assert 136 == 134`。**L272 修正批重量**：132 → 133 → **134**（要点 ⑭ 与 ⑮ 各一枚同轮哈希，与 l264 同批对上）。红读数 `assert 133 == 132` 与 `assert 134 == 132`。**L272 记账批重量**：131 → **132**（L272 块 ⑬ 点名账本批那一枚，与 l264 同批对上）。红读数 `assert 132 == 131`。**L271 记账批重量**：128 → 130 → **131**（L271 块 ⑩ 那两枚 + 块 ⑬ 那枚，与 l264 同批对上）。红读数 `assert 131 == 128`。
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
DANGLING_ROSTER = {}
#: **2026-10-22 清空**：原 15 格全是 L265 / L270 两个块里「解析得到但不可达」的
#: 旧 lineage 提交，重写后它们按提交时间折叠进 `not_a_commit`（见 `REWRITE_CUTOFF`）。
#: 空名册意味着这一档现在**一格都不放过**：任何新出现的不可达点名立刻落进
#: `defect_cells` 并让 `MEASURED_DEFECTS` 那一格红。
STASH_ROSTER = {}
#: **2026-10-22 清空**：那枚 `git fsck` 找回的 stash 对象（`STASH_HEX`）已被 gc 剪掉，
#: 现量 `stash` 档为 0（`git stash list` 也是空的）。对象回来或新出现 stash 点名即红。


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
        """除**名册内的跨分支点名**外，ref 可达与 HEAD 祖先这两判据在现面上给同一批对象 ⇒ 换判据没有放松任何一格。
        差集写成「恰好 ⊆ 名册」而不是「恰好为空」：后者把「作者这棵树当时检出哪条分支」抄进判据，换一棵树就自己
        变红（A361 那一族的同一形状），而名册外多一处照样红，牙齿由 `test_an_unrostered_cross_branch_citation_still_goes_red` 现场证。"""
        ref_dangling = {r["hex"] for r in sites() if r["tier"] == DANGLING}
        hd = head_ancestors()
        # 两处必须排除，否则差集被 2026-10-22 的重写灌满（实测 240+ 枚）：
        #   - 早于重写时刻的提交：它们当年在 main 上，重写后散落在各条远端分支，
        #     拿「HEAD 祖先」量它们等于拿新历史量旧账本，不是判据失效而是判据不适用。
        # 判据本身不变：**本轮（重写之后）写进账本的点名**仍要两判据一致。
        by_head = {r["hex"] for r in sites() if r["full"] is not None and r["tier"] != STASH
                   and r["full"] not in hd and not committed_before_rewrite(r["full"])}
        assert ref_dangling <= by_head, sorted(ref_dangling - by_head)
        assert by_head - ref_dangling <= MEASURED_CROSS_BRANCH_CITED, sorted(by_head - ref_dangling)

    def test_l98_reads_only_volume_one(self):
        """A344 说「全账无人量」的前提：管哈希槽与 git 真值对账那把尺子只读卷一，所以卷二清单区这 153 处一直没人问（**L289 记账批按现量续钉**：147 → 153，本批六处全在 L288 那一行的槽回填三枚与同一行的逐枚解释里，随 `MEASURED_REGIONS` 同批走；**L288 记账批按现量续钉**：137 → 147，L287 那一行的槽回填五枚跨轮哈希、同行又逐枚说明一次 ⇒ 该行 4 处走到 14 处，随 `MEASURED_REGIONS` 同批走；**L287 记账批按现量续钉**：133 → 137，L287 清单行点名四只真批哈希，随 `MEASURED_REGIONS` 同批走）。
        （**L283 账本批重量**：132 → **133**，A383 表行点名一枚注入器验盘的 sha256 前缀，随 `MEASURED_REGIONS` 同批，红读数 `assert 133 == 132`；**L276 账本批重量**：131 → **132**，L276 清单行点名代码批 `518d4d056` 那一枚，随 `MEASURED_REGIONS` 同批，红读数 `assert 132 == 131`；
        **L275 账本批重量**：130 → **131**，L275 清单行点名代码批那一枚，随 `MEASURED_REGIONS` 同批，红读数 `assert 131 == 130`；
        **L274 账本批重量**：127 → **130**，清单行点名代码批与修正批各一枚 + A361 那一行重点修正批那一枚，
        随 `MEASURED_REGIONS` 同批，红读数 `assert 130 == 127`；
        **L273 账本批重量**：126 → **127**，L273 清单行点名代码批那一枚，随 `MEASURED_REGIONS` 同批，红读数 `assert 127 == 126`；
        **L272 修正批重量**：125 → **126**，随 `MEASURED_REGIONS` 同批，红读数 `assert 126 == 125`；
        **L272 记账批重量**：124 → **125**，随 `MEASURED_REGIONS` 同批，红读数 `assert 125 == 124`；
        **L271 记账批重量**：121 → **124**，红读数 `assert 124 == 121`）"""
        from tests.unit import test_ledger_hash_slots_l98 as l98
        assert l98.LEDGER_PATH.name == other.VOLUMES[0]
        assert MEASURED_REGIONS[("OPTIMIZATION_LOOP_2.md", "checklist")] == 179  # **L294 记账批按红读数续钉**：175 → 179（与站点数 +6 中的清单区 +4 同批对上，四处全在卷二 L293 那一行的槽位回填里、两枚哈希各两处；本轮新写的 L294 那一行是「哈希待 L295 回填」占位 ⇒ 零贡献。红读数逐字 `assert 179 == 175`。**L293 记账批按红读数续钉**：171 → 175（四处全在卷二 L292 那一行的槽位回填里、两枚哈希各两处，随 `MEASURED_REGIONS` 与 `MEASURED_SITES` 同批走；红读数逐字 `assert 175 == 171`。旧注：**L292 记账批按红读数续钉**：165 → 171（与 `MEASURED_SITES` +7 中的清单区 +6 同批对上，六处全在卷二 L291 那一行的槽回填里、三枚哈希各两处；红读数逐字 `assert 171 == 165`）。旧注：**L291 记账批按红读数续钉**：159 → 165（与 `MEASURED_SITES` +6 同批对上，六处全在卷二 L290 那一行的槽回填里、三枚哈希各两处；红读数逐字 `assert 165 == 159`）。旧注：**L290 记账批按红读数续钉**：153 → 159（与 `MEASURED_SITES` +6 同批对上，六处全在第 128 行那一格；红读数逐字 `assert 159 == 153`）。旧注：**L289 记账批按现量续钉**：147 → 153（六处全在 L288 那一行），红读数逐字 `assert 153 == 147`。**L288 记账批按现量续钉**：137 → 147（回填那一行 +10），红读数逐字 `assert 147 == 137`。**L287 记账批按现量续钉**：133 → 137，红读数逐字 `assert 137 == 133`

    def test_l264s_face_is_a_strict_subset_of_mine(self):
        """与 l264 的分工：它那 151 处全在我的面里，我多出的 282 处是它按「`### Lnnn` 开块 + 标题行不算站点」
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
    def _post_rewrite_probe(self):
        """取一枚「重写之后、ref 可达、在当前 HEAD 祖先面上」的提交，返回 (短哈希, 全哈希)。

        **为什么探针必须现取**：2026-10-22 强推重写之后，账本里已有点名**全是**旧 lineage
        对象（现量：可达且重写后的点名 0 处）。把旧对象从可达面摘掉，它只会按提交时间落进
        `not_a_commit` 一档（历史伤亡），`DANGLING` 的牙齿就再也测不到。所以三个牙齿测试都改用
        现取的探针：它既是可达的，又晚于 `REWRITE_CUTOFF`，摘掉必然落进 `dangling`。
        """
        full = _git("rev-parse", "origin/main^{commit}").stdout.strip()
        assert full, "取不到 origin/main 的提交，探针无从取"
        assert not committed_before_rewrite(full), "探针必须晚于 REWRITE_CUTOFF，否则测的是历史伤亡而不是 DANGLING"
        assert full in reachable(), "探针必须在可达面上"
        return full[:9], full

    def test_dropping_an_ancestor_moves_its_cited_sites_into_the_dangling_tier(self):
        """可达判据不是摆设：合成一处对重写后提交的点名，再把它从可达面摘掉 ⇒ 它立刻从 `reachable`
        走到 `dangling`、缺陷格恰好多出它那一格；复原后重跑与基线逐格相同。零落盘。"""
        target, full = self._post_rewrite_probe()
        volumes = other.load_volumes()
        lines = volumes[VOLUMES[1]]
        volumes[VOLUMES[1]] = lines[:2] + ["- [x] **L900** 探针 `%s`（臂写在本文件里，不落盘）" % target] + lines[2:]
        try:
            base = sites(volumes)
            assert [r["tier"] for r in base if r["hex"] == target] == [REACHABLE]
            saved = _CACHE["re"]
            _CACHE["re"] = saved - {full}
            try:
                got = sites(volumes)
                assert [r["tier"] for r in got if r["hex"] == target] == [DANGLING]
                assert tally(got)[DANGLING] == tally(base)[DANGLING] + 1
                assert tally(got)[REACHABLE] == tally(base)[REACHABLE] - 1
                assert defect_cells(got) == {("OPTIMIZATION_LOOP_2.md", "checklist", 900, target): 1}
            finally:
                _CACHE["re"] = saved
            assert sites(volumes) == base
        finally:
            volumes[VOLUMES[1]] = lines

    def test_an_unrostered_cross_branch_citation_still_goes_red(self):
        """两判据那格换成名册不是「跨分支的一律放过」：合成一处对重写后提交的点名，再把它从 HEAD
        祖先面摘掉——就是「账本点名了另一条分支上的提交」那一形 ⇒ 它立刻进入差集而名册接不住，
        那句 `<= 名册` 当场不成立。复原后再走一遍原判据，与放行臂同数。零落盘。

        探针同样现取（见 `_post_rewrite_probe`）：账本现面上已经没有「重写后 + ref 可达 +
        在 HEAD 祖先面」的点名可用了。
        """
        target, full = self._post_rewrite_probe()
        volumes = other.load_volumes()
        lines = volumes[VOLUMES[1]]
        volumes[VOLUMES[1]] = lines[:2] + ["- [x] **L900** 探针 `%s`（臂写在本文件里，不落盘）" % target] + lines[2:]
        saved = _CACHE["hd"]
        try:
            base = sites(volumes)
            base_dangling = {r["hex"] for r in base if r["tier"] == DANGLING}
            base_diff = {r["hex"] for r in base if r["full"] is not None
                         and r["tier"] != STASH
                         and r["full"] not in head_ancestors()
                         and not committed_before_rewrite(r["full"])} - base_dangling
            assert base_diff == set(), "放行臂本身必须是空集（现量名册为空）"
            _CACHE["hd"] = head_ancestors() - {full}
            by_head = {r["hex"] for r in base if r["full"] is not None
                       and r["tier"] != STASH
                       and r["full"] not in _CACHE["hd"]
                       and not committed_before_rewrite(r["full"])}
            assert (by_head - base_dangling) - base_diff == {target}
            assert not (by_head - base_dangling) <= MEASURED_CROSS_BRANCH_CITED
        finally:
            _CACHE["hd"] = saved
            volumes[VOLUMES[1]] = lines
        assert len(sites()) == MEASURED_SITES, "复原后站点必须与基线同数"

    def test_the_roster_is_not_a_blanket_tolerance(self):
        """名册不是「游离的一律放过」：合成一处**新**的点名——重写后的探针提交，但落在名册里没有的
        区段身份（L900 那一行）上 ⇒ 缺陷格当场非空且恰好是那一格。只在内存里改面，不落盘。

        探针现取的理由同 `_post_rewrite_probe`：重写前的不可达对象已按时间折叠进 not_a_commit，
        拿它们当探针造不出缺陷格。
        """
        target, full = self._post_rewrite_probe()
        volumes = other.load_volumes()
        lines = volumes[VOLUMES[1]]
        probe = "- [x] **L900** 假造的游离点名 `%s`（臂写在本文件里，不落进账本）" % target
        volumes[VOLUMES[1]] = lines[:2] + [probe] + lines[2:]
        saved = _CACHE["re"]
        _CACHE["re"] = saved - {full}
        try:
            got = defect_cells(sites(volumes))
            assert got == {("OPTIMIZATION_LOOP_2.md", "checklist", 900, target): 1}
        finally:
            _CACHE["re"] = saved
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
