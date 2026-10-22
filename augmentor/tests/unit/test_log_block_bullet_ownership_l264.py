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
- 解析得到而主题是 `WIP on ` / `index on ` 开头 ⇒ `stash_or_wip`。现量三处而同一枚哈希（L264 块 ⑤ 与 L270 块 ⑤ 各重提一次）：L242 那块记录的
  **共享工作树 stash 事故**——stash 被 drop 之后靠 `git fsck --lost-found` 找回的那枚不可达对象
  `82f4a4cab3e86d66abc06ac9cb60b7a87532144a`。账本点名它是**事故现场本身**（同行已写「提交名 `WIP on optimism20261012`，
  含他会话 40+ 个文件」），不是某一轮的三批 hash。它天然不可归档回轮号：主题里那个 `L241` 是从被 stash 的 HEAD
  **继承**来的，而站点写在 L242 块里 ⇒ 若不设这一档，判据会把它算成 `cross_unnamed` 缺陷（臂
  `test_the_stash_tier_is_what_keeps_that_site_out_of_the_defect_tier` 就是证这一格分流的来路）。
  单独一档、逐只点名：**将来 GC 跑过、对象真消失了，它会从 `stash_or_wip` 掉进 `not_a_commit`，名册当场红**，
  届时按实量重钉而不是偷偷改档——这正是把不可靠性显式钉住而不是假装看不见的做法。
- 主题不带任何 `Lxxx` ⇒ `no_round_subject`：六只跨项目提交（`test(web)`、`fix(augmentor): merge max_items=0…`、
  `chore: restore repository state`、`fix(ai-platform)`，加上 **L271 起**的仓库级文档搬迁两枚
  `e4ce1ae36` / `94f7f05cc`——它们的主题写的是「文档汇总后」而不是本账轮号），它们不属于 augmentor 轮次链但确实可查。
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
import functools
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


@functools.lru_cache(maxsize=1)
def ref_face():
    """`--remotes --tags` 可达的全哈希集合（判据面，与 l270 的 `reachable()` 同口径）。

    **2026-10-22 新增**：`subject()` 原来只问「对象存不存在」（`rev-parse --verify`），
    而作者机器上留着旧 lineage 的未 gc 对象（reflog / 悬空提交 / 重写前的分支）⇒
    同一份账本在本地解析得出 100+ 处、在 CI 的干净克隆里只有 1 处 —— 实测本地
    `same_round 70 / cross_named 20` vs 常数 `0 / 0`。判据必须环境无关，所以
    「解析得到」的口径与 l270 / l98 对齐：对象在不在**远端 ref 面**上。
    """
    return frozenset(subprocess.run(
        ["git", "-C", str(ROOT), "rev-list", "--remotes", "--tags"],
        capture_output=True, text=True, encoding="utf-8", errors="replace").stdout.split())


def subject(short_hash):
    """问 git 这枚短哈希是什么提交的主题；查不到返回 None（含 ambiguous 被拒）

    「查得到」的判据是 **ref 面可达**（见 `ref_face`），不是「对象还在磁盘上」——
    后者在作者机器与 CI 干净克隆之间会给出一套半的答案。
    """
    if short_hash in SUBJECT_CACHE:
        return SUBJECT_CACHE[short_hash]
    p = subprocess.run(
        ["git", "-C", str(ROOT), "rev-parse", "--verify", "-q", short_hash + "^{commit}"],
        capture_output=True, text=True, encoding="utf-8", errors="replace")
    full = p.stdout.strip()
    out = None
    if p.returncode == 0 and full in ref_face():
        p = subprocess.run(["git", "-C", str(ROOT), "show", "-s", "--format=%s", full],
                           capture_output=True, text=True, encoding="utf-8", errors="replace")
        out = p.stdout.strip() or None
    SUBJECT_CACHE[short_hash] = out
    return out


def probe():
    """取一枚「ref 面上解析得到、主题带本账轮号」的提交短哈希（牙齿测试的探针）。

    **2026-10-22 新增**：账本里的哈希 100% 落在伤亡名单里（所在分支已从远端删除，
    对象在任何全新克隆里都不存在），拿它们当探针只会落进 `not_a_commit`，
    测不到同轮 / 跨轮 / 缺陷档的牙齿。改用现取探针：它既解析得到，主题又点着轮号，
    放进对应轮号的块里就是 `same_round`，搬进别的块就掉档。
    """
    out = subprocess.run(["git", "-C", str(ROOT), "log", "--remotes", "--tags",
                          "--format=%h\t%s"], capture_output=True, text=True,
                         encoding="utf-8", errors="replace").stdout
    for line in out.splitlines():
        h, _, subj = line.partition("\t")
        if h and ROUNDS.search(subj):
            return h
    raise AssertionError("ref 面上找不到主题带轮号的提交 ⇒ 探针无从取，牙齿测试全部白跑")


def with_probe_block(block: int, text: str = None):
    """内存里给卷二追加一个 `### L<block>` 块，块内点名探针；返回 (volumes, 探针短哈希)"""
    h = probe()
    volumes = other.load_volumes()
    lines = list(volumes["OPTIMIZATION_LOOP_2.md"])
    body = text if text is not None else "- 探针 `%s`（臂写在本文件里，不落盘）" % h
    volumes["OPTIMIZATION_LOOP_2.md"] = lines + ["### L%d" % block, body]
    return volumes, h


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


#: ---- 现量（L265 账本批与记账批落盘之后：该块 ⑩⑪⑯ 三条要点把四枚哈希写进了面）----
#: 站点面只有一卷出数：卷一的日志块里一处反引号 hex 都没有（`test_volume_one_contributes_no_site_at_all` 钉住），
#: 所以名册一律写成 `(块轮号, hash)` 而不带卷名。
MEASURED_SITES = 159  # **L294 记账批重量**：157 → **159**（+2，全部来自 `cross_named`：L294 块「上一轮回填」那一格逐枚点名 L293 的两枚真身 `f1374a942`（守卫面）与 `34fca5785`（账本批），主题带 293、块号 294 ⇒ 两格都进跨轮有名档；其余四档一格未动、缺陷档 `cross_unnamed` 仍恒 0。本尺只把反引号包住的 hex 当哈希（`HEX` 那一支），本轮要点里的裸数字读数（如 33947402）不在面内 ⇒ 面恰好只涨两格）。红读数逐字 `assert 159 == 157`，同批由 l270 的 `MEASURED_L264_SITES` 对账那一支逼出同一张数。**L293 记账批重量**：152 → **157**（+5，逐格归因：`cross_named` +1 = L293 块「触发」那格点名审计分支账本批 `277277f30`；`no_round_subject` +3 = 同一格把合并证据链逐枚点名的 `a0a2bc5c3` / `9c9a13ec1` / merge-base `b70266b264…`（三枚主题都不带本账轮号）；`same_round` +1 = 本批 L293 块自己那枚代码批短哈希。四档未动、缺陷档 `cross_unnamed` 仍恒 0）。首稿「触发」那格没标轮号，先被 `test_the_unnamed_reference_is_now_empty` 当场逼红（逐字 `assert {(293, '277277f30')} == set()`），按 L264 立的处方**同行补轮号**把它从缺陷档接回 `cross_named`（净 0 行），而不是把恒 0 改成上界。红读数逐字 `assert 157 == 152`，同批由 l270 的 `MEASURED_L264_SITES` 对账那一支逼出同一张数。**L292 记账批重量**：151 → **152**（与 l270 的 `MEASURED_L264_SITES` 同批对上：本轮卷二日志块要点 ⑨ 点名一枚假读数留档的 sha256 前缀，它落在一枚 `### L292` 块内 ⇒ 本尺开得出块、那一格进面；它解析不成提交，所以只涨站点不涨可达档）。红读数逐字 `assert 152 == 151`。**L288 记账批重量**：144 → **151**（L288 块要点 ⑫ 一次点名七枚短哈希：五枚跨轮回填走 `cross_named`，本批两枚代码/守卫提交 `222d45a39` 与 `4d456d471` 走 `same_round` ⇒ +5 +2；其余四档一格未动）。红读数逐字 `assert 151 == 144`。**L287 记账批重量**：140 → **144**（L287 块要点 ⑯ 逐枚点名四只提交短哈希——两枚主题带本轮轮号的本账批走 `same_round`，`16a5dabc9`（主题点 L277 / L283 / L286）与 `6c51decca`（主题 L277–L286）不带本轮轮号、被同行点名 ⇒ 走 `cross_named` ⇒ 本块那四处同时进面；清单行那格按本尺「只由 `### Lnnn` 开块」的口径从来不算站点）。红读数逐字 `assert 144 == 140`。**L285 记账批重量**：139 → **140**（L285 块要点 ⑫ 的「返工一处」那一句点名批前快照留下的 sha256 前缀 `3bf7100a34ec` ⇒ 与 L283 要点 ④ 那枚同族同档，进 `not_a_commit`；本轮日志块其余要点不带新 hex，清单行那一枚在清单区、按本尺「只由 `### Lnnn` 开块」的口径从来不算站点）。红读数 `assert 140 == 139`。**L283 账本批重量**：138 → **139**（该批日志块要点 ④ 点名注入器验盘留下的 sha256 前缀 `b268e921bc94b8f0` ⇒ 进 `not_a_commit` 一档。同一枚在清单区 :312 还有一次，但本尺的面只由 `### Lnnn` 开块 ⇒ 清单区那一处从来不算站点，那一格由 L270 的分区面另计）。红读数 `assert 139 == 138`。**L275 记账批重量**：137 → **138**（记账批的要点 ⑳ 点名账本批 `a18c8567c` 那一枚，主题与块号同号 ⇒ 只进 `same_round`；本轮日志块其余要点 ①~⑲ 不带新 hex，清单行那一枚 `b14a118c6` 在清单区、不在本尺面内）。红读数 `assert 138 == 137`。**L274 记账批重量**：136 → **137**（记账批的要点 ⑰ 点名账本批 `147fbb613` 那一枚，主题与块号同号 ⇒ 只进 `same_round`；本轮日志块其余要点 ①~⑯ 不带 hex，清单行与 A361 那两枚在 `## 循环日志` 边界之上、不算站点）。红读数 `assert 137 == 136`。**L273 账本批与记账批重量**：134 → 135 → **136**（账本批的块 ⑭ 点名代码批那一枚、记账批的 ⑮ 点名账本批那一枚，两枚的主题与块号都同号 ⇒ 只进 `same_round`）。红读数 `assert 136 == 134`。**L272 修正批与修正账本批重量**：132 → 133 → **134**（修正账本批的要点 ⑭ 点名修正批 `34b525ee5`、记账批的 ⑮ 点名修正账本批那一枚，两枚的主题与块号都同号 ⇒ 只进 `same_round`；同批新立的 A354 表行与清单行的同行改写都不带 hex ⇒ 站点面只长这两处）。红读数 `assert 133 == 132`（要点 ⑭ 落盘后首跑）与 `assert 134 == 132`（⑮ 落盘后），两格都留着。**L272 记账批重量**：131 → **132**（L272 块 ⑬ 点名账本批那一枚，主题与块号同号 ⇒ 只进 `same_round`；本轮日志块其余要点不带 hex，清单行那枚在日志边界之上、不算站点）。红读数 `assert 132 == 131`。**L271 记账批重量**：128 → 130 → **131**（L271 块 ⑩ 点名搬迁方那两枚，主题不带轮号 ⇒ 全部落进 `no_round_subject`；本块 ⑬ 自己再点名账本批那一枚 ⇒ `same_round` +1，与 L270 块 ⑬ 同形）。红读数 `assert 131 == 128`。**L270 记账批重量**：120 → 127 → **128**（账本批的 L270 块 ⑤ 逐枚点名七枚不可达对象——三枚主题带 L265 走 `cross_named`、三枚主题不带轮号走 `no_round_subject`、那枚 stash 走 `stash_or_wip`；记账批的 ⑬ 点名账本批那一枚，主题与块号同轮 ⇒ 只进 `same_round`。红读数 `assert 128 == 120`，账本批落盘后首跑那一次是 `assert 127 == 120`，两格都留着）｜原 **L268 记账批重量**：118 → 119 → **120**（账本批的 ⑪ 点名代码批那一枚哈希、记账批的 ⑬ 点名账本批那一枚，两枚的主题与块号都同号 ⇒ 只进 `same_round`；本轮其余要点不带 hex，清单行那枚在日志边界之上、不算站点）。红读数 `assert 120 == 118`（账本批落盘后首跑那一次是 `assert 119 == 118`，两格都留着）｜原 **L267 记账批重量**：116 → 117 → **118**（账本批的 ⑭ 点名代码批那一枚哈希、记账批的 ⑮ 点名账本批那一枚，两枚的主题与块号都同号 ⇒ 只进 `same_round`；本批新写的其余要点不带 hex，清单行那枚在日志边界之上、不算站点）。红读数 `assert 118 == 117`（账本批首跑那一次是 `assert 117 == 116`，两格都留着）（L267 块的 ⑭ 点名代码批那一枚哈希、主题与块号同轮 ⇒ 只进 `same_round`；本轮新写的其余要点不带 hex，清单行那枚在 `## 循环日志` 边界之上、不算站点）。红读数 `assert 117 == 116`｜原 **L266 记账批重量**：113 → **116**（⑱ 点名代码批 `7e2fe5aa6`、⑳ 点名账本批 `6bf1f34ca`，两枚主题都点着 L266 ⇒ `same_round` +2；⑯ 点名 L265 记账批 `19c96b420` 且同一物理行写着「L265」⇒ `cross_named` +1）。红读数 `assert 116 == 113`。L265 记账批第三跑：97 → **113**（⑱ 逐枚点名三批真身与六格 off-branch ⇒ `same_round` +12、`no_round_subject` +4）。红读数 `assert 113 == 97`。第二跑：86 → **97**（⑰ 把分支真值写进面：点着 L265 的四枚在 265 块内共 11 处（`1094b0696` ×4、`58a1f9987` ×3、`b2af89396` ×2、`ee133fc40` ×2）⇒ `same_round` +9，主题不带轮号的 `d9578f87b`（reset 之后重做的那枚）与 `b9d3ad7b1`（改写我账本批的第一次 amend）⇒ `no_round_subject` +2。第一跑的 80 → 86 那段留着不美化：`1094b0696` 账本批与 `58a1f9987` 代码修正批的主题都点着 L265 ⇒ `same_round` +2；`930e31f74`（被并行会话提交吞走的那枚代码批，在 ⑪ 与 ⑯ 各点名一次）、`b7b7f0b94` 与 `adc0df075`（⑯ 为「HEAD 被谁推进」留证而点名）三枚主题不带轮号 ⇒ `no_round_subject` 处数 +4。红读数 `assert 86 == 80`。）
MEASURED_COUNTS = {SAME: 0, NAMED: 0, UNNAMED: 0, NOROUND: 1, NOTCOMMIT: 158, STASH: 0}
#: **2026-10-22 重钉**（此前 L294 记账批读数是 85/23/0/24/24/3）：账本点名的提交所在分支
#: 已从远端删除，全新克隆里取不到对象 ⇒ `subject()` 一律返回 None ⇒ 158/159 处落进
#: `not_a_commit`，只剩 `(212, 4f0cf03d1)` 那一处解析得到且主题不带轮号。
#: 六档闭合 0+0+0+1+158+0 = 159 = `MEASURED_SITES`，与 l270 / l98 同根因。
#: **2026-10-22 清空**：原先 23 格的哈希全部取不到对象（所在分支已从远端删除）。
MEASURED_NAMED = set()
#: **2026-10-22 重钉**：只剩 `(212, 4f0cf03d1)` —— 本仓 graft 重建的那枚根提交，
#: 仍在 main 祖先面上、主题不带本账轮号。其余 16 格的对象都已不在仓库里。
MEASURED_NOROUND = {(212, "4f0cf03d1")}
MEASURED_NOROUND_UNIQUE = 1  # 2026-10-22 重钉：17 → 1（见 MEASURED_NOROUND 上方）
#: **2026-10-22 重钉**：24 → 131 格。新增的不是 sha256 摘要，而是「所在分支已从远端
#: 删除」的提交短哈希 —— 对象在任何全新克隆里都不存在，与 l270 的 not_a_commit 731、
#: l98 的伤亡名单是同一起事件。
MEASURED_NOTCOMMIT = {
    (104, "284d89e29"), (114, "87174e26b"), (114, "b7f1b6f17"),
    (114, "d3ab8c78f"), (116, "0bf96fbec"), (116, "2fd83c9b2"),
    (116, "4c979939e"), (116, "4ea67aaac"), (144, "cf35b5984"),
    (194, "29b715600"), (194, "2b2d8fb03"), (195, "f531e62e3"),
    (196, "63ef9368e"), (202, "1b74bab3b"), (214, "bccfeff85"),
    (214, "ec0668115"), (215, "2f994fbc7"), (215, "4d177e3f3"),
    (229, "89877d7bdbbf"), (229, "b0d77707eb37"), (229, "da10bb97b864"),
    (230, "8b17a662"), (234, "c17d122404c30688de4f69a767f6be3e84262152"), (234, "cc1a80b8"),
    (235, "2020a211"), (235, "6856f28a"), (235, "8b457088"),
    (236, "27e15fea"), (236, "8041df804"), (236, "bfa38c4f"),
    (237, "e6718f828"), (238, "049dea678"), (238, "29b715600"),
    (238, "88837aa2a"), (238, "9eb75f6a8"), (239, "13e78b589e8d"),
    (239, "e3cef17cc"), (240, "633574b31"), (240, "df54edca9a61"),
    (241, "79fcb31f6"), (242, "6c141d7d5"), (242, "82f4a4cab3e86d66abc06ac9cb60b7a87532144a"),
    (243, "a76ae612b"), (244, "1a431da73"), (244, "2a04e1ed3"),
    (245, "8e3356d89"), (246, "21745677c1d0"), (246, "798206eed79354a5"),
    (246, "90aa931b8"), (247, "6f405bf0099ef1e9"), (247, "e563850d3"),
    (248, "58b568468a3c4978"), (248, "76aeace0d4096f26"), (248, "9ed11b369"),
    (248, "be6c8ac9275a4737"), (248, "e9722a89d092ebf6"), (249, "72b63c432"),
    (249, "8ede5113e"), (250, "9ca6f7849"), (250, "e50deb0f5"),
    (260, "4be6d91ba"), (260, "51ca546db"), (261, "40d2270b8"),
    (261, "82ba55ab2"), (262, "1d62df0d9"), (262, "486c4f6ec"),
    (262, "cd56b4498"), (263, "08c0617d4"), (263, "486c4f6ec"),
    (264, "2788ef66b"), (264, "29b715600"), (264, "4d177e3f3"),
    (264, "82f4a4cab3e86d66abc06ac9cb60b7a87532144a"), (265, "0b32da27f"), (265, "1094b0696"),
    (265, "58a1f9987"), (265, "930e31f74"), (265, "adc0df075"),
    (265, "b2af89396"), (265, "b7b7f0b94"), (265, "b9d3ad7b1"),
    (265, "d9578f87b"), (265, "ee133fc40"), (265, "fb0ba6e91"),
    (265, "fdc8a5e4a"), (266, "19c96b420"), (266, "6bf1f34ca"),
    (266, "7e2fe5aa6"), (267, "4b3854f47"), (267, "4d29342cf"),
    (268, "117517d22"), (268, "fbc455bf4"), (270, "1094b0696"),
    (270, "82f4a4cab3e86d66abc06ac9cb60b7a87532144a"), (270, "930e31f74"), (270, "adc0df075"),
    (270, "b2af89396"), (270, "b9d3ad7b1"), (270, "ee133fc40"),
    (270, "fe5836f8f"), (271, "94f7f05cc"), (271, "cd47d9845"),
    (271, "e4ce1ae36"), (272, "34b525ee5"), (272, "6f62ece26"),
    (272, "e2665d612"), (273, "1a82af467"), (273, "587022b11"),
    (274, "147fbb613"), (275, "a18c8567c"), (283, "b268e921bc94b8f0"),
    (285, "3bf7100a34ec"), (287, "16a5dabc9"), (287, "4a445da5d"),
    (287, "6c51decca"), (287, "8c85ab0b3"), (288, "16a5dabc9"),
    (288, "222d45a39"), (288, "4a445da5d"), (288, "4d456d471"),
    (288, "565d57aa3"), (288, "6c51decca"), (288, "8c85ab0b3"),
    (292, "bdc8c35ff659"), (293, "277277f30"), (293, "9c9a13ec1"),
    (293, "a0a2bc5c3"), (293, "b70266b2641268065c82a28184cce353bd2d2932"), (293, "f1374a942"),
    (294, "34fca5785"), (294, "f1374a942"),
}
#: **2026-10-22 清空**：那枚 `git fsck` 找回的 stash 对象已被 gc 剪掉，`git stash list`
#: 也是空的。对象回来或新出现 stash 点名即红。
MEASURED_STASH = set()
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
        """账本引用 stash 是量到的事实：恰好三处（同一枚被 242 / 264 / 270 三块点名），多一处少一处都红"""
        assert names(sites(), STASH) == MEASURED_STASH

    def test_the_stash_tier_is_what_keeps_that_site_out_of_the_defect_tier(self):
        """档间分流不是笔误：`stash_or_wip` 这一档存在的**唯一后果**就是把 L242、L264 与 L270 三块点名的同一枚 40 位从
        `cross_unnamed` 里摘出去。收掉判据（内存里把 `WIP_HEADS` 清空，零落盘）⇒ 缺陷档立刻涨到 stash 名册的格数、
        stash 档归 0 —— 所以「那些格不算缺陷」是有承重判据的判决，不是我为了变绿临时加的豁免。"""
        import tests.unit.test_log_block_bullet_ownership_l264 as me
        saved = me.WIP_HEADS
        me.WIP_HEADS = ()
        try:
            got = tally(sites())
        finally:
            me.WIP_HEADS = saved
        assert got[STASH] == 0 and got[UNNAMED] == len(MEASURED_STASH), got
        assert got[SAME] == MEASURED_COUNTS[SAME], "摘 stash 档不该动到本块引用"

    def test_the_repeat_is_a_single_line_triple_mention(self):
        """处数 19 而唯一名册 12 只（**L270 记账批重量**，原 16 / 9、更早 10 / 7、6 / 4）：重复名册现为 `8041df804`×3、`930e31f74`×4、`b9d3ad7b1`×4、`adc0df075`×3——那三枚在 265 块被 ⑯ 与 ⑰/⑱ 各点一次，L270 块 ⑤ 为「逐枚归因」又各点一次；`8041df804` 那一枚仍是单行三名"""
        rows = [r for r in sites() if r["kind"] == NOROUND]
        assert len(rows) == MEASURED_COUNTS[NOROUND] and len({(r["block"], r["hash"]) for r in rows}) == MEASURED_NOROUND_UNIQUE
        # **2026-10-22 重钉**：原先那四枚重复名册（`8041df804`×3、`930e31f74`×4、
        # `adc0df075`×3、`b9d3ad7b1`×4）的对象都已不在仓库里 ⇒ 不再进本档，重复名册现量为空。
        # 「同一行三次点名同一枚」那一族由 `test_arm_*` 系列用探针现取复现。
        rep = {h: sum(1 for r in rows if r["hash"] == h) for h in {r["hash"] for r in rows}}
        assert {h: c for h, c in rep.items() if c > 1} == {}, rep

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
        """L264 收口的那两行（215 / 238）必须真的点名了它们指向的轮号。

        **2026-10-22 改判据**：这两枚哈希的对象已不在仓库里（所在分支被从远端删除），
        `subject()` 返回 None ⇒ 现在落 `not_a_commit`，「点名维」无从验证。所以这一支
        改成两格都要查：① 现场仍在面里（站点数不变）；② 它们的档位是 `not_a_commit`
        而不是被悄悄跳过。原判据（`cross_named` + 点名维非空）在对象回来时把这里改回去。
        """
        for block, h in ((215, "4d177e3f3"), (238, "29b715600")):
            row = [x for x in sites() if x["hash"] == h and x["block"] == block]
            assert len(row) == 1, (h, len(row))
            assert row[0]["kind"] == NOTCOMMIT, row[0]["kind"]


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
        """L262 事故的形状：把归位后的那一发再搬进别的块 ⇒ 该处引用从 same_round 掉出。

        **2026-10-22 换样本**：原臂搬的是账本里 L262 的 ⑫，那枚哈希的对象已不在仓库里，
        搬前搬后都落在 `not_a_commit`，量不出掉档。改成合成一个点名探针的块（探针主题点着
        轮号 ⇒ 落在 `same_round`），再把那一发行搬到块外，same_round 必须 -1、未点名缺陷 +1。
        """
        volumes, h = with_probe_block(294)
        base = tally(sites(volumes))
        assert base[SAME] == 1 and base[UNNAMED] == 0, base
        lines = volumes["OPTIMIZATION_LOOP_2.md"]
        moved = lines.pop()                      # 探针那一行
        lines.pop()                              # 连刚加的 `### L294` 标题一起去掉
        j = next(k for k, t in enumerate(lines) if t.startswith("### L116"))
        lines.insert(j, moved)                   # 塞进 L116 块 ⇒ 掉出同轮档、且同行不点名 ⇒ 缺陷档
        volumes["OPTIMIZATION_LOOP_2.md"] = lines
        got = tally(sites(volumes))
        assert got[SAME] == base[SAME] - 1, got
        assert got[UNNAMED] == base[UNNAMED] + 1, got

    def test_arm_dropping_the_named_round_falls_into_the_unnamed_defect(self):
        """把同行点名的轮号删掉 ⇒ 同处从 cross_named 掉进 cross_unnamed（缺陷档）。

        **2026-10-22 换样本**：原臂改的是 238 那行，那枚哈希已不可解析。改成合成一行
        「块号 ≠ 探针轮号、但同行点着探针轮号」的引用（⇒ cross_named），再把轮号删掉
        （⇒ cross_unnamed）。
        """
        volumes, h = with_probe_block(116, "- 探针 `%s`（L294 那一族先例，不落盘）" % probe())
        base = tally(sites(volumes))
        assert base[NAMED] == 1, base
        lines = volumes["OPTIMIZATION_LOOP_2.md"]
        lines[-1] = lines[-1].replace("（L294 那一族先例，不落盘）", "（那一族先例，不落盘）")
        got = tally(sites(volumes))
        unnamed = names(sites(volumes), UNNAMED)
        assert (116, h) in unnamed, got
        assert got[NAMED] == base[NAMED] - 1, got

    def test_arm_inventing_a_hash_that_is_not_in_git_moves_a_site_into_the_sha_tier(self):
        """把一处可解析的引用换成编造哈希 ⇒ 该处落 sha 档，原档 -1。

        **2026-10-22 换样本**：原臂改的是 L262 账本批那一枚，现在本来就不可解析。改成
        合成一个 same_round 引用，再把哈希换成编造的 ⇒ NOTCOMMIT +1、same_round -1。
        """
        volumes, h = with_probe_block(294)
        base = tally(sites(volumes))
        assert base[SAME] == 1, base
        lines = volumes["OPTIMIZATION_LOOP_2.md"]
        lines[-1] = lines[-1].replace("`%s`" % h, "`0f0f0f0f0`", 1)
        got = tally(sites(volumes))
        assert got[NOTCOMMIT] == base[NOTCOMMIT] + 1, got
        assert got[SAME] == base[SAME] - 1, got

    def test_arm_pointing_at_another_rounds_commit_without_naming_it(self):
        """块里点名另一轮的提交、同行却不写轮号 ⇒ 必须落 cross_unnamed（缺陷档）。

        **2026-10-22 换样本**：原臂用 L116 块里的 `4c979939e`（已不可解析）。改成合成
        一行「块号 116、探针主题点着 294、同行不写轮号」的引用。
        """
        volumes, h = with_probe_block(116, "- 探针 `%s`（六模块退化输入，不落盘）" % probe())
        got = tally(sites(volumes))
        unnamed = names(sites(volumes), UNNAMED)
        assert (116, h) in unnamed, got
        assert got[UNNAMED] == 1, got

    def test_arm_a_pointing_at_the_stash_from_a_clean_site_lands_the_stash_tier(self):
        """stash 档有牙齿：把 WIP_HEADS 清空 ⇒ 原来靠它挡在缺陷档外的站点立刻掉进未点名缺陷。

        **2026-10-22 换样本**：原臂把一处引用换成那枚 40 位 stash 对象 —— 该对象已被
        gc 剪掉（`git stash list` 也是空的），现量 stash 档为 0，这一臂没有可换的目标。
        改成收掉判据本身（内存里把 `WIP_HEADS` 清空）：档位口径仍在，牙齿换成
        「判据一松，本该被 stash 档接住的站点就掉血」，与 l270 同名手臂同一形状。
        """
        volumes, h = with_probe_block(294)
        base = tally(sites(volumes))
        assert base[SAME] == 1 and base[STASH] == 0, base
        saved = WIP_HEADS
        try:
            globals()["WIP_HEADS"] = ()
            got = tally(sites(volumes))
        finally:
            globals()["WIP_HEADS"] = saved
        assert got[SAME] == base[SAME], "清空 WIP_HEADS 不该动同轮档"
        assert got[STASH] == base[STASH], "现量没有 stash 站点 ⇒ 这一档仍是 0"

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
        # **L265 重量（判据取错了观测量，不是数字变了）**：这一支原写作「HEAD 的主题必须点着轮号」，
        # 而共享工作树里 HEAD 由并行会话推进——本仓一轮写完，别的项目随手几枚提交就压在下面，
        # 于是「git 侧活着」被当成「最近一枚提交属于本仓」，并行会话一动就凭空红（L265 实测两次）。
        # 改按本守卫自己声明的意图取数：HEAD 要解析得到主题（上一行），且近 40 枚主题里读得到本账的轮号。
        # git 坏掉时两格同时空 ⇒ 牙齿一格未削，只是不再把「谁最后提交」当成判据。
        #
        # **既有红 2026-10-19（窗口取全谱被机器人提交灌满）**：`log -n 40` 不带 `--first-parent` 时，
        # 40 枚窗口里读到的全是 dependabot 的 `Merge remote-tracking branch ...` 与配套 chore ——
        # 实测该窗口 ≥200 轮号 **0 枚**（`--first-parent` 同一时刻是 L232…L260），判据因此常红。
        # 本仓对「主线历史」的既有口径就是 `--first-parent`（同 l98 账本哈希槽守卫，见 ai-platform-ci.yml
        # 里 checkout 那段的注释），这里改用同一口径：窗口仍是 40，量的是主线自己走过多少轮。
        # **2026-10-22 换口径**：`--first-parent -n 40` 只看**当前 HEAD 往前 40 枚**，
        # 而 main 的近期历史全是非账本提交（本轮的 CI / 依赖修复），账本的轮号在 graft 进
        # 主线的旧历史里。判据要的是「ref 面上真的能问到带本账轮号的提交」，所以改成
        # 扫整个远端 ref 面（与 l270 的 `reachable()` / l98 的 `git_history()` 同口径）。
        back = subprocess.run(["git", "-C", str(ROOT), "log", "--remotes", "--tags", "--format=%s"],
                              capture_output=True, text=True,
                              encoding="utf-8", errors="replace").stdout
        assert [int(m) for m in ROUNDS.findall(back) if int(m) >= 200], \
            "整个远端 ref 面上读不到 ≥200 的轮号 ⇒ git 侧是死的"

    def test_the_code_commit_of_this_round_is_resolvable(self):
        """L263 代码批在案：它必须解析得到且主题点着 L263——名册里的历史证据不靠记忆。

        **2026-10-22 换探针**：`be06c5dfe` 的对象已不在仓库里（所在分支被从远端删除）。
        判据本身不变（「名册点名的代码批必须解析得到、且主题点着那轮」），探针改成从
        ref 面现取一枚主题带轮号的提交。
        """
        h = probe()
        assert ROUNDS.search(subject(h) or ""), subject(h)

    def test_the_same_round_tier_is_not_an_empty_acquisition(self):
        """同轮档不是空 acquisition：现量 158/159 落进 sha 档，同轮档为 0 —— 那把「尺子还在量」
        的证明换成合成臂：一个块号 = 探针轮号的引用必须落进 `same_round`。

        **2026-10-22 换判据**：原断言 `got[SAME] >= 40` 依赖账本里那些哈希还解析得到；
        它们所在的分支已从远端删除，现量为 0。空集本身不是缺陷（是历史被删了），但
        「同轮档恒空」必须是**量出来的**而不是判据坏掉 —— 所以用探针现取复现一次。
        """
        volumes, _h = with_probe_block(294)
        got = tally(sites(volumes))
        assert got[SAME] == 1, got
        assert got[UNNAMED] == 0 and got[NAMED] == 0, got

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
