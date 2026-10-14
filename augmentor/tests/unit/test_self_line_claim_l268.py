# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""账本里「本账第 N 行」那一形自指主张第一次有尺子（A333 的形状 ③ / L268）

**这一轮收的是哪一格**：A333 立账时点名三种 a184 内容维判不到的形状，其中 ③ 是**不带文件名的裸行号**
——账本散文写「所以本账第 835 行那条 `docs/augmentor/API.md:1139` 引用仍指得准」，主张的是**本卷自己的行号**。
a184 量的是 `file.py:NNN` / `FILE.md:NNN` 这类**带文件名**的落点，裸行号它结构性看不见，所以历轮每次插行
把那一格平移一次，全靠人眼重钉；L267 的账本批就当场错过一次（先钉成 833，复盘行号是 832，同句改回），
而本轮（L268）的账本批落下 3 行 ⇒ 名册里那一格由 832 走到 **835**，这是本守卫第一次咬到自己。
本守卫把这一形钉成三面闭合的现量名册。

**面怎么定义**：站点 = 匹配 `本账第 (\\d+) 行` 的物理行，一行多处按多处算。卷一的「本账第 6 次」那一族
是**序数**而不是行号主张（现量 8 处），由 `test_the_ordinal_shape_is_not_a_line_claim` 钉成「序数在场
而行号档为 0」，证明 0 是量出来的而不是短语根本不存在。

**站点身份为什么不用行号**：每一轮都会在清单区与 Backlog 区插行，用「第几行」当名册键会把尺子自己钉成
陈旧读数——那正是本尺子要抓的缺陷形。所以键是 `(卷名, 区段, 轮号, 主张的那一行)`：区段取该行自己的轮号
（清单行 `^- \\[x\\] \\*\\*L(\\d+)\\*\\*` 或所属 `^### L(\\d+)` 块标题），它随轮次固定、不随插行平移。
**代价写明**：这一格名册随账本变长而必须逐轮更新（A333 修法③ 自己就点名的代价），换到的牙齿是
「operative 那一格没重钉或钉错 ⇒ 当场红」。

**三档判据（全部可复算，不是读后印象）**：
- `LIVE`：同一物理行带 `文件名:行号` 形状的锚，且主张的那一行的内容里确实出现同一个锚。
- `STALE`：同上有锚，但主张的那一行里读不到它——历史读数（历轮当时的现量，按纪律不追改、不美化）。
- `BLIND`：那一行根本没有锚（「本账第 795 行」后面只跟着散文），机械无法判定，**这一档就是 A333
  剩下的那一半**，本守卫把它钉成逐格点名的现量而不是留给印象。

**牙齿**：`TestRulerBites` 五臂全部内存改写、零落盘——operative 的数字 off-by-one ⇒ `LIVE` 掉到 0；
在它之上插一行（就是「上一轮插行把自指平移」那一形）⇒ `LIVE` 掉到 0；把锚那一格的内容改掉 ⇒
`LIVE` 掉到 0；新增一条无锚自指 ⇒ 面 +1 而名册红；把序数那一形喂进来 ⇒ 面**不**动（判据不是「凡本账第都算」）。
**臂的锚与下标按内容定位器现读**：off-by-one 臂要改的数字、scrub 臂要改的行下标都由
`locate_operative_target` 现算，写死会让每一轮凭空多出两处重钉点（L268 记账批首跑三支臂红，两支就是
这个原因）——臂的牙齿是「改一位就掉档」，不是「旧数字恰好还在」。名册里 operative 那一格**仍然写死**：
那是刻意的每轮代价（见上一段），把它改成派生就等于把「散文没重钉」这一格从名册面上抹掉。

**为什么零 parametrize**：`test_parametrize_ids_l97`（438 / 317）与 `test_parametrize_blind_face_l98`
（盲面 116 / 四档 61-38-11-11 / 豁免 25）把全仓 parametrize 三面冻成现量数。本守卫的逐格断言走名册
字典循环，一支 parametrize 都不需要，那两把普查尺子一格未动。

跑法：`python -m pytest tests/unit/test_self_line_claim_l268.py -q --no-cov`
"""
import re

from tests.unit import test_backlog_mark_vocab_l253 as other

VOLUMES = other.VOLUMES

#: 主张形状：本卷自己的行号，不带文件名。`\d+` 而不是 `[0-9]{1,4}`：账本行号会随卷变长，
#: 位数上限是一种「以后不用再改」的假设，而这一格要的恰恰是每次都能被量到。
CLAIM = re.compile(r"本账第 (\d+) 行")
#: 同物理行的锚：带扩展名的文件名 + 冒号 + 行号（可带 `-范围`）。锚必须落在同一行，
#: 否则「第 N 行」与「哪个引用」的配对关系就要靠跨行语料推断，那是印象而不是判据。
FILE_REF = re.compile(r"([A-Za-z0-9_./\\-]+\.(?:py|md|tsx|ts|yaml|json)):(\d+)(?:-\d+)?")
#: 区段身份：清单行自己的轮号。
CHECK = re.compile(r"^- \[[ x]\] \*\*L(\d+)\*\*")
#: 区段身份：日志块标题的轮号。
TITLE = re.compile(r"^### L(\d+)")
#: 序数形状（卷一的「本账第 6 次」「本账第 **5** 次」「本账第三次」三种写法），必须**不**进面。
#: `[^行、\\n]` 是必需的：同一物理行既写着「第 832 行」又出现「次」字时，宽松窗口会把行号主张切成假序数。
ORDINAL = re.compile(r"本账第[^行、\n]{0,12}?次")

LIVE = "LIVE"
STALE = "STALE"
BLIND = "BLIND"

#: ---- 现量（L268 立尺）----
#: 站点总数：全部落在卷二，卷一 0（`test_volume_one_has_no_line_claims_at_all`）。
MEASURED_SITES = 13
#: 三档闭合：1 + 5 + 7 = 13。`LIVE` 那一格就是 operative 自指，其余是历史读数与无锚主张。
MEASURED_TIERS = {LIVE: 1, STALE: 5, BLIND: 7}
#: 名册：`(卷, 区段, 轮号, 主张的那一行)`。`("OPTIMIZATION_LOOP_2.md", "checklist", 232, 862)` 是
#: operative 那一格——L267 复盘现量 832（先钉成 833 的那一格同批改回，见 L267 块 ⑮），L268 的账本批在它之上
#: 落 3 行 ⇒ 重钉 835，L269 的账本批又在其上落 3 行（清单行 +1 与 A347 连带分隔空行 +2）⇒ 838，
#: **L270 的账本批在其上落 7 行（清单行 +1 与新立 A348 / A349 / A350 三笔连带它们的分隔空行 +6）⇒ 按 ⑤ 的内容
#: 定位器重钉为 845**，**L271 的账本批在其上落 21 行（清单行 +1、A351 / A352 连带分隔空行 +5、日志块 +15）
#: ⇒ 第一次写完落 850、补回表格那一格分隔空行后按定位器重钉为 851**（见 L271 块 ⑨ 的返工记录），
#: **L272 的账本批在其上落 3 行（清单行 +1 与新立 A353 连带它的分隔空行 +2；本轮日志块那 16 行落在
#: 被指向的那一行之下 ⇒ 不参与平移）⇒ operative 与链尾同批落到 854**，**L272 修正账本批在其上落 3 行（新立 A354 连带它的分隔空行 +2 与要点 ⑭ +1；清单行与要点 ③④ 都是同行改写 ⇒ 净 0，要点 ⑮ 落在被指向那一行之下 ⇒ 不参与平移）⇒ operative 与链尾同批落到 856**，**L273 的账本批在其上落 6 行（清单行 +1 与新立 A355 / A356 / A357 三笔连带其中两笔的分隔空行 +5；本轮日志块 ⑭⑮ 落在被指向那一行之下 ⇒ 不参与平移）⇒ operative 与链尾同批落到 862**，
#: 这一格每次都由 `test_the_roster_is_exact` 的红读数当场逼出
#: （L272 那轮是 `assert ('...', 232, 856) == ('...', 232, 854)`；本轮红读数同时点名两格：案面里有 `('...', 232, 862)` 而名册里还是 `('...', 232, 856)`），不是算术推的。
ROSTER = frozenset(
    {
        ("OPTIMIZATION_LOOP_2.md", "checklist", 232, 862),
        ("OPTIMIZATION_LOOP_2.md", "checklist", 243, 734),
        ("OPTIMIZATION_LOOP_2.md", "checklist", 252, 795),
        ("OPTIMIZATION_LOOP_2.md", "log", 232, 773),
        ("OPTIMIZATION_LOOP_2.md", "log", 236, 339),
        ("OPTIMIZATION_LOOP_2.md", "log", 244, 773),
        ("OPTIMIZATION_LOOP_2.md", "log", 245, 781),
        ("OPTIMIZATION_LOOP_2.md", "log", 252, 795),
        ("OPTIMIZATION_LOOP_2.md", "log", 257, 810),
        ("OPTIMIZATION_LOOP_2.md", "log", 259, 813),
        ("OPTIMIZATION_LOOP_2.md", "log", 260, 818),
        ("OPTIMIZATION_LOOP_2.md", "log", 261, 819),
        ("OPTIMIZATION_LOOP_2.md", "log", 262, 820),
    }
)
#: operative 那一格的身份：卷二清单区 L232 行的自指。历轮每次插行都重钉它，只有它是「现在必须为真」的。
OPERATIVE = ("OPTIMIZATION_LOOP_2.md", "checklist", 232)
#: 卷一的序数形状在场量（证明「面 0」不是短语缺失）。
MEASURED_ORDINALS = 8


def self_claims(volumes=None):
    """扫两卷，返回 `[{volume, kind, round, line, target, refs, tier}]`

    Args:
        volumes: `{卷名: [行]}`，默认按字节读两卷并归一成 LF

    Returns:
        list，按文件与行号顺序
    """
    if volumes is None:
        volumes = other.load_volumes()
    out = []
    for name, lines in volumes.items():
        kind = ("outside", 0)
        for i, line in enumerate(lines, 1):
            hit = CHECK.match(line)
            if hit:
                kind = ("checklist", int(hit.group(1)))
            else:
                hit = TITLE.match(line)
                if hit:
                    kind = ("log", int(hit.group(1)))
            for claim in CLAIM.finditer(line):
                target = int(claim.group(1))
                refs = [f"{f}:{n}" for f, n in FILE_REF.findall(line)]
                in_range = 1 <= target <= len(lines) - 1
                if not refs:
                    tier = BLIND
                elif in_range and any(r in lines[target - 1] for r in refs):
                    tier = LIVE
                else:
                    tier = STALE
                out.append(
                    {
                        "volume": name,
                        "kind": kind[0],
                        "round": kind[1],
                        "line": i,
                        "target": target,
                        "refs": refs,
                        "tier": tier,
                    }
                )
    return out


def key_of(site):
    """名册键：区段身份而不是物理行号（见模块 docstring）"""
    return (site["volume"], site["kind"], site["round"], site["target"])


def tier_counts(sites):
    """三档计数，恒返回三面齐全（缺档补 0，让「掉档」是数字差而不是 KeyError）"""
    got = {LIVE: 0, STALE: 0, BLIND: 0}
    for site in sites:
        got[site["tier"]] += 1
    return got


#: operative 那一格的**独立定位器**：不读散文里的数字，而是按内容找出那条引用真正住在第几行——
#: 全卷同时含 `DiffResult` 与 `docs/augmentor/API.md:1139` 的物理行现量唯一。这样 `test_...` 才是「按内容重算」
#: 而不是「把散文写的数抄一遍」。
OPERATIVE_MARKER = ("DiffResult", "docs/augmentor/API.md:1139")


def locate_operative_target(lines):
    """按内容独立定位 operative 自指应当写的那一行号

    Args:
        lines: 卷二的行表（0 起索引，末元素是文件尾换行后的空串）

    Returns:
        行号（1 起）；找不到或多于一个即 `AssertionError`——定位器本身也要有牙齿
    """
    hits = [
        i for i, ln in enumerate(lines, 1) if all(marker in ln for marker in OPERATIVE_MARKER)
    ]
    assert len(hits) == 1, (OPERATIVE_MARKER, hits)
    return hits[0]


def patched(volumes, name, edit):
    """在内存里改写某一卷，返回新 dict——零落盘，绝不动仓库文件

    Args:
        volumes: `load_volumes()` 的返回值
        name: 卷名
        edit: 接受行表、返回新行表的函数

    Returns:
        新的 `{卷名: [行]}`
    """
    out = dict(volumes)
    out[name] = edit(list(volumes[name]))
    return out


class TestTheFace:
    """面、名册与三档闭合"""

    def test_the_face_is_the_measured_sites(self):
        """现量 13 处「本账第 N 行」，全部在卷二"""
        sites = self_claims()
        assert len(sites) == MEASURED_SITES, [(s["volume"], s["line"]) for s in sites]

    def test_the_roster_is_exact(self):
        """名册精确相等：多一处（新一轮写了新的自指而没记账）或少一处都红"""
        got = {key_of(s) for s in self_claims()}
        assert got == ROSTER, sorted(got ^ ROSTER, key=str)

    def test_the_three_tiers_add_up_to_the_face(self):
        """三档之和恰好等于面，且逐档现量 1 / 5 / 7"""
        sites = self_claims()
        got = tier_counts(sites)
        assert got == MEASURED_TIERS, got
        assert sum(got.values()) == len(sites) == MEASURED_SITES

    def test_volume_one_has_no_line_claims_at_all(self):
        """卷一这一形 0 处：它用的是序数（下一条测试证明短语在场）"""
        volumes = other.load_volumes()
        sites = self_claims({"OPTIMIZATION_LOOP.md": volumes["OPTIMIZATION_LOOP.md"]})
        assert sites == [], sites

    def test_the_ordinal_shape_is_not_a_line_claim(self):
        """判据不是「凡 `本账第` 都算」：卷一 8 处序数在场而行号档为 0"""
        lines = other.load_volumes()["OPTIMIZATION_LOOP.md"]
        assert len(ORDINAL.findall("\n".join(lines))) == MEASURED_ORDINALS

    def test_the_blind_tier_is_derived_not_asserted(self):
        """BLIND 的 7 格逐格验「同物理行确实没有 `文件:行号` 锚」；STALE 的 5 格反之"""
        sites = self_claims()
        for site in sites:
            if site["tier"] == BLIND:
                assert site["refs"] == [], site
            else:
                assert site["refs"], site

    def test_every_claim_targets_a_real_line(self):
        """主张的行号必须落在卷内：越界是「行号写飞」而不是「历史陈旧」"""
        volumes = other.load_volumes()
        for site in self_claims():
            n = len(volumes[site["volume"]]) - 1
            assert 1 <= site["target"] <= n, site


class TestTheOperative:
    """唯一那一格「现在必须为真」的自指"""

    def test_exactly_one_live_self_claim_and_it_is_the_operative(self):
        """`LIVE` 档恰好一格，且它是卷二清单区 L232 那一行——历轮重钉的就是它"""
        live = {key_of(s)[:3] for s in self_claims() if s["tier"] == LIVE}
        assert live == {OPERATIVE}, live

    def test_the_operative_number_matches_the_independent_locator(self):
        """散文写的行号 == 按内容独立定位的行号（L273 账本批之后现量 862）"""
        volumes = other.load_volumes()
        site = next(
            s
            for s in self_claims()
            if (s["volume"], s["kind"], s["round"]) == OPERATIVE
        )
        assert site["target"] == locate_operative_target(volumes[site["volume"]]), site


class TestRulerBites:
    """五臂自证：全部内存改写，仓库文件一格未动"""

    def test_arm_off_by_one_drops_the_live_tier(self):
        """L267 真实撞过的那一格（832 写成 833）：数字挪一位 ⇒ `LIVE` 掉到 0。
        锚文字按 `locate_operative_target` 现读而不是写死旧值——臂的牙齿是「挪一位就掉档」，
        写死数字只会让每一轮多两处要重钉（L268 记账批第一次红就红在写死的锚与下标上）。"""
        volumes = other.load_volumes()
        op = locate_operative_target(volumes["OPTIMIZATION_LOOP_2.md"])
        old, new = f"所以本账第 {op} 行", f"所以本账第 {op + 1} 行"
        volumes = patched(
            volumes,
            "OPTIMIZATION_LOOP_2.md",
            lambda ls: [ln.replace(old, new) for ln in ls],
        )
        got = tier_counts(self_claims(volumes))
        assert got[LIVE] == 0, got
        assert got[STALE] == MEASURED_TIERS[STALE] + 1, got

    def test_arm_inserting_a_line_above_shifts_the_claim(self):
        """「上一轮在清单区插一行」那一形：目标行被平移而散文没改 ⇒ `LIVE` 掉到 0"""
        def insert(lines):
            lines.insert(5, "")
            return lines

        got = tier_counts(self_claims(patched(other.load_volumes(), "OPTIMIZATION_LOOP_2.md", insert)))
        assert got[LIVE] == 0, got

    def test_arm_removing_the_anchor_from_the_target_breaks_the_live_tier(self):
        """锚那一格的内容被改写（引用被删）⇒ 同一数字也不再为真，判据读的是内容不是数字。
        下标同样按内容定位器现读，不写死物理行号。"""
        volumes = other.load_volumes()
        idx = locate_operative_target(volumes["OPTIMIZATION_LOOP_2.md"]) - 1

        def scrub(lines):
            lines[idx] = lines[idx].replace("docs/augmentor/API.md:1139", "另一条引用")
            return lines

        got = tier_counts(self_claims(patched(volumes, "OPTIMIZATION_LOOP_2.md", scrub)))
        assert got[LIVE] == 0, got

    def test_arm_a_new_blind_self_claim_grows_the_face(self):
        """新一轮写一条无锚自指 ⇒ 面 +1 而名册红（这一格的更新代价是刻意要的，不是误伤）"""
        def append(lines):
            lines.insert(106, "- [x] **L268** `deadbeef`：散文里写「本账第 900 行」而没有锚。")
            return lines

        sites = self_claims(patched(other.load_volumes(), "OPTIMIZATION_LOOP_2.md", append))
        assert len(sites) == MEASURED_SITES + 1, len(sites)
        assert {key_of(s) for s in sites} ^ ROSTER == {
            ("OPTIMIZATION_LOOP_2.md", "checklist", 268, 900)
        }

    def test_arm_an_ordinal_line_does_not_enter_the_face(self):
        """负向臂：新增的是「本账第 7 次」那种序数 ⇒ 面一格不动，判据不是无差别前缀匹配"""
        def append(lines):
            lines.insert(106, "- [x] **L268** `deadbeef`：本账第 7 次记这一类插行缺陷。")
            return lines

        sites = self_claims(patched(other.load_volumes(), "OPTIMIZATION_LOOP_2.md", append))
        assert {key_of(s) for s in sites} == ROSTER, sorted({key_of(s) for s in sites}, key=str)
