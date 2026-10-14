# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""正文把判决写成「轮号在动词之前」时，定级格必须跟着落（A176 家族的一个子形 / L263）

**这一轮收的是哪一格**：`test_backlog_closure_l242` 与 `test_backlog_mark_vocab_l253` 共用同一支
标记判据 `MARK`，它只认**闭合词紧贴 `（L`** 的那一形（或划掉字母）。账本里另有两种写法把判决说了
出去而宽面与定级格两面都读不到：① 轮号在动词之前（`A274` 的「**· L220 已收口**：两只字段名的请求
默认改成 `Optional[str] = None` 哨兵……」），② 动词后面跟的是限定词而不是轮号（`A71` 的
「**L46 关闭（第三种修法：按依赖等级拆 try）**」，括号里点的是**用哪条修法**，不是**收了哪一半**）。
本轮按这两个形状普查两卷 Backlog 的 217 行，量到 **7 格候选**，逐行读完正文后定性：
**2 格是整行判决而定级格仍是裸字母**（`A274` / `A71`——正是 A176 描述的「欠账看着还在、债其实已经
还了」的机械形状，本轮把两格收成划掉字母加轮号），余 5 格合法留裸字母：3 格半程（`A45` 只关 ①、
`A127`「前半程关闭，后半仍开放」、`A136`「关闭（假臂那一侧）」且同行明写「留一格开放」），
2 格引用别物（`A121`「L75 新记（关闭 A114 时……）」、`A143`「L84 新记（关闭 A133 时……）」，
那句判决管的是别的行，本行自己写着「要连着端点表一起拍」「两格都要先拍」）。

**与 l253 的分工**：l253 钉「MARK 读得到的那面」（宽面与定级格面，之差由 `TWO_FACE_ROSTER` 逐格
点名），本守卫钉「MARK 读不到、而散文确实带着轮号说了判决」那面。两面**按构造互斥**（候选行先要
求整行不带 MARK），由 `test_the_three_faces_partition_the_table` 现场验三档闭合。

**分类为什么敢机械做**：三档的判据都是可复算的字面形状，不是读后印象——`引用别物` = 动词之后 8
字符内紧跟另一个 ID；`半程` = 「轮号 → 动词 → 动词后 24 字符」这一窗口里出现封闭限定词表
（前半 / 后半 / 一半 / 部分 / 那一半 / 那一侧 / 仍开放 / 仍在案 / 留一格 / 未关闭 / 带圈序号）里的
某一个；两者都不成立才算 `整行判决`。封闭表这一维**确实有风险**：某行真收了而正文恰好带一个限定词，
会被误读成半程而静默放行。所以 `MEASURED_HITS` 把每一格**靠哪个词豁免**逐格点名（换词就红），
`test_arm_dropping_a_scope_token_promotes_the_row_to_a_verdict` 再证明那个词真的承重。

**反向的牙齿**：`FULL_VERDICT` 现量是**空集**，且断言写成精确相等而不是「≤ 某数」——写成上界就允许
一格回退。两支补标回退臂把 `A274` / `A71` 的定级格收回裸字母，两格立刻重新进盲区并被分类成整行判决
⇒ 空集断言变红。`test_arm_verb_then_round_without_brackets_is_caught` 补第二种形状（`已收口，见 Lxxx`，
动词在前且没有 `（`，MARK 同样读不到），现量 0 格 ⇒ 那一档是量出来的空集。
`test_arm_a_negative_prose_shape_is_not_a_verdict` 证明「未关闭」不会被当成判决语（负向前瞻承重）。

**为什么零 parametrize**：`test_parametrize_ids_l97` 与 `test_parametrize_blind_face_l98` 把全仓
parametrize 三面冻成现量数。本守卫的逐格断言走名册字典循环，一支 parametrize 都不需要，那两把普查
尺子一格未动。

跑法：`python -m pytest tests/unit/test_round_first_verdict_l263.py -q --no-cov`
"""
import re

from tests.unit import test_backlog_closure_l242 as l242
from tests.unit import test_backlog_mark_vocab_l253 as other

VOLUMES = other.VOLUMES
ID = other.ID
ROW = other.ROW
MARK = other.MARK
BARE = other.BARE

#: 判决动词：与 `MARK` 同一组字，再收 l242 的 `AFTER` 也认的「清零」。
#: `(?<![未不没])` 是必需的：账本里「未关闭」是常态描述（`A127` 就写着「后半仍开放」那一类），
#: 把它读成判决会让普查面凭空胖一圈假候选。
VERB = r"(?<![未不没])(?:已关闭|已收口|已结案|结案|已修|清零|关闭|收口)"
#: 形状 ①（A274 / A71 那一形）：轮号在动词之前，中间最多 12 个字符且不断句、不跨竖线。
PRE = re.compile(rf"L(\d{{1,3}})([^。|\n]{{0,12}}?)({VERB})")
#: 形状 ②：动词在轮号之前，但中间没有 `（`——带 `（L` 的那一形是 `MARK` 的刀口，归 l253 管。
POST = re.compile(rf"({VERB})([^。|（(\n]{{0,12}}?)L(\d{{1,3}})")
#: 半程限定词封闭表：出现任何一个 ⇒ 那句判决只管本行的一部分，不算整行判决。
SCOPE = re.compile(
    r"前半|后半|一半|部分|那一半|那一侧|仍开放|仍在案|留一格|未关闭|[①②③④⑤⑥⑦⑧⑨]"
)
#: 引用别物：动词之后 8 字符内紧跟另一个 ID。
REF_TAIL = re.compile(rf"^\s*[`*]?\s*({ID})")

FULL = "整行判决"
HALF = "半程"
REF = "引用别物"

#: ---- 现量（L263 补标之后）----
#: 与 l242 的 `MEASURED_ROWS` 同面同数（现量 218 行 Backlog 表行）。
MEASURED_BACKLOG_ROWS = 224  # **L270 重量**：221 → **224**（新立 A348 / A349 / A350 三行入表，与 l242 的 `MEASURED_ROWS` 同面同数；三行都是裸字母待拍 ⇒ 三面里只走「无判决语」档，`MEASURED_BLIND` 5 一格未动，宽面那一档由 A344 带上标记的同一格 +1）。红读数 `assert 224 == 221`｜原 **L269 重量**：220 → **221**（A347 入表，与 l242 的 `MEASURED_ROWS` 同面同数；那一格三面里只走「无判决语」档 ⇒ `MEASURED_BLIND` 5 与宽面 116 一格未动）。红读数 `assert 221 == 220`｜原 **L268 重量**：219 → **220**（A346 入表，与 l242 的 `MEASURED_ROWS` 同面同数）。红读数 `assert 220 == 219`｜原 **L267 重量**：218 → **219**（A345 入表，与 l242 的 `MEASURED_ROWS` 同面同数）。红读数 `assert 219 == 218`｜原 **L265 记账批二轮重量**：217 → 218（新立 A344 一行入表）。红读数 `assert 218 == 217`
#: 盲区候选：补标 `A274` / `A71` 之前的首跑读数是 **7**，两格落进定级格之后剩 **5**。
MEASURED_BLIND = 5
#: 三档闭合：宽面（l253）+ 盲区（本守卫）+ 正文里根本没有带轮号的判决语 = 表行总数（L270 现量 117 + 5 + 102 = 224：A344 带着标记走进宽面就离开「无判决语」那一档，三行新账带着裸字母走进同一档 ⇒ 净 +2；L268 那一轮的 116 + 5 + 99 = 220 留着）。
MEASURED_NO_VERDICT_ROWS = 102  # **L270 重量**：100 → **102**（三行新账各入这一档，A344 那一格因为定级格带上判决语而离开这一档 ⇒ +3 −1；闭合式 `117 + 5 + 102 == 224` 与 l253 的宽面 117、本守卫的盲区 5 两格同批对上）。红读数由 `test_the_three_faces_partition_the_table` 在 `MEASURED_BACKLOG_ROWS` 那一步之后逼出｜原 **L269 重量**：99 → **100**（A347 那一格是裸 M 待拍、定级格与整行都没有 `MARK` 读得到的闭合语、也没有「轮号在动词之前」的盲区形状 ⇒ 三面里只落「无判决语」这一档；闭合式 `116 + 5 + 100 == 221` 与 l253 的宽面 116、本守卫的盲区 5 两格同批对上）。红读数由 `test_the_three_faces_partition_the_table` 在 `MEASURED_BACKLOG_ROWS` 那一步之后逼出｜原 **L267 重量**：98 → **99**（A345 那一格是裸字母待拍、正文也没有带轮号的判决语 ⇒ 落「无判决语」档）。红读数 `assert (115 + 5) + 98 == 219`｜原 **L265 记账批二轮重量**：97 → 98（A344 那一格是裸 `M` 待拍、正文也没有带轮号的判决语 ⇒ 落「无判决语」档）。红读数 `assert (115 + 5) + 97 == 218`
#: 本轮从「正文说了而定级格没说」收成「两边都说」的两格：(所在卷, ID, 补标后的定级格, 判决轮号)。
#: 轮号取自正文那句判决（`A274` 的 L220、`A71` 的 L46），不是本轮轮号——本轮只是把别人的判决落格。
CLOSED_AT_L263 = (
    ("OPTIMIZATION_LOOP_2.md", "A274", "~~S~~ 已收口（L220）", 220),
    ("OPTIMIZATION_LOOP.md", "A71", "~~S~~ 已关闭（L46）", 46),
)
#: 余下五格的逐命中名册：`{ID: [(档, 命中串, 依据)]}`，依据是那一格被分到该档所凭的字面片段。
#: 精确相等 ⇒ 换措辞（例如把「关闭 A114」写成「收掉 A114」）也要跟着重钉，不许静默通过。
MEASURED_HITS = {
    "A45": [(HALF, "L36 更新（①已修", "①")],
    "A121": [(REF, "L75 新记（关闭", "A114")],
    "A127": [(HALF, "L79 前半程关闭", "前半")],
    "A136": [(HALF, "L81 关闭", "那一侧")],
    "A143": [(REF, "L84 新记（关闭", "A133")],
}
#: 整行判决档的现量名册：补标两格之后是**空集**，这一格是这把尺子的牙齿。
FULL_VERDICT = frozenset()


def load_volumes():
    return other.load_volumes()


def verdicts(text):
    """一行里所有「带轮号的判决语」命中：`[{shape, text, start, verb_end}]`"""
    out = []
    for m in PRE.finditer(text):
        out.append({"shape": "pre", "text": m.group(0), "start": m.start(), "verb_end": m.end()})
    for m in POST.finditer(text):
        out.append({"shape": "post", "text": m.group(0), "start": m.start(),
                    "verb_end": m.end(1)})
    return out


def hits_of(text):
    """`[(档, 命中串, 依据)]`——引用别物优先（动词后面跟着的是别的行），其次半程，剩下才算整行判决"""
    out = []
    for h in verdicts(text):
        ref = REF_TAIL.match(text[h["verb_end"]:h["verb_end"] + 8])
        window = text[h["start"]:h["verb_end"] + 24]
        scope = SCOPE.search(window)
        if ref:
            out.append((REF, h["text"], ref.group(1)))
        elif scope:
            out.append((HALF, h["text"], scope.group(0)))
        else:
            out.append((FULL, h["text"], ""))
    return out


def row_grade(hits):
    """一行的档取「最强的那句判决」：只要有一命中既不是别物也不是半程，这一行就算写了整行判决"""
    kinds = {k for k, *_ in hits}
    for kind in (FULL, HALF, REF):
        if kind in kinds:
            return kind
    raise AssertionError(f"空命中名册 {hits}")


def blind_face(volumes=None):
    """`MARK` 两面都读不到、定级格仍是裸字母、而正文带「轮号 + 判决语」的 Backlog 行"""
    volumes = load_volumes() if volumes is None else volumes
    out = {}
    for row in other.backlog_rows(volumes):
        if MARK.search(row["text"]) or not BARE.match(other.grade_cell(row["text"])):
            continue
        got = hits_of(row["text"])
        if got:
            out[row["id"]] = {"volume": row["volume"], "line": row["line"],
                              "cell": other.grade_cell(row["text"]), "hits": got,
                              "grade": row_grade(got)}
    return out


def rows_of(volumes, ident):
    return [r for r in other.backlog_rows(volumes) if r["id"] == ident]


def _patch_cell(volumes, volume, ident, new_cell):
    """在内存里改写某一行的定级格，磁盘一格不动（与 l253 的注入臂同一通道）"""
    lines = volumes[volume]
    bound = other.log_boundary(lines)
    hit = [i for i, ln in enumerate(lines[:bound]) if ROW.match(ln) and ROW.match(ln).group(1) == ident]
    assert len(hit) == 1, (volume, ident, hit)
    parts = lines[hit[0]].split("|")
    parts[-2] = f" {new_cell} "
    lines[hit[0]] = "|".join(parts)
    return volumes


def _patch_text(volumes, volume, ident, old, new):
    """在内存里替换某一行的一段正文，先验锚点唯一命中再改"""
    lines = volumes[volume]
    bound = other.log_boundary(lines)
    hit = [i for i, ln in enumerate(lines[:bound]) if ROW.match(ln) and ROW.match(ln).group(1) == ident]
    assert len(hit) == 1, (volume, ident, hit)
    text = lines[hit[0]]
    assert text.count(old) == 1, (volume, ident, old, text.count(old))
    lines[hit[0]] = text.replace(old, new)
    return volumes


class TestTheBlindFace:
    """盲区名册先钉住：哪些行把判决写在 MARK 读不到的那一形里，每一格凭哪个词留下"""

    def test_the_face_is_the_measured_five(self):
        got = blind_face()
        assert len(got) == MEASURED_BLIND, sorted(got)
        assert set(got) == set(MEASURED_HITS), sorted(set(got) ^ set(MEASURED_HITS))

    def test_each_row_says_exactly_the_measured_phrase(self):
        got = {k: sorted(v["hits"]) for k, v in blind_face().items()}
        assert got == {k: sorted(v) for k, v in MEASURED_HITS.items()}, got

    def test_no_candidate_is_a_whole_row_verdict(self):
        full = {k for k, v in blind_face().items() if v["grade"] == FULL}
        assert full == set(FULL_VERDICT), sorted(full)

    def test_the_three_faces_partition_the_table(self):
        """三档闭合：宽面（含定级格面）+ 盲区 + 无判决语 = 表行总数，且盲区与两面互斥按构造成立"""
        volumes = load_volumes()
        rows = other.backlog_rows(volumes)
        assert len(rows) == MEASURED_BACKLOG_ROWS
        wide, narrow = other.faces(volumes)
        got = blind_face(volumes)
        assert set(got) & wide == set(), sorted(set(got) & wide)
        assert set(got) & narrow == set(), sorted(set(got) & narrow)
        assert len(wide) + len(got) + MEASURED_NO_VERDICT_ROWS == MEASURED_BACKLOG_ROWS, len(wide)

    def test_two_of_the_half_rows_are_the_ones_l242_nominates(self):
        """半程那一档与 l242 的 `PARTIAL` 不是同面（`A136` 从未进声明面，所以 l242 看不见它），
        但重叠的两格必须真重叠——两把尺子在同一批行上各读各的刀口"""
        half = {k for k, v in blind_face().items() if v["grade"] == HALF}
        assert {"A45", "A127"} <= half & l242.PARTIAL, sorted(half & l242.PARTIAL)
        assert "A136" in half and "A136" not in l242.PARTIAL

    def test_the_reference_rows_point_at_ids_that_are_not_open(self):
        """引用别物那两格点名的 ID（A114 / A133）自己不是「还开着的行」：实测两只是 phantom——
        判决进过声明面而表行早已随裁撤离场（l242 的 `MEASURED_PHANTOM` 那一族）。
        这一格不是装饰：如果 `A114` 其实仍是一行开着的账，那「本行没标是因为那句管别的东西」就是借口，
        真正的欠账会被这个档位静默吞掉。"""
        volumes = load_volumes()
        narrow = other.faces(volumes)[1]
        phantoms = set(l242.declared(volumes)) - set(l242.table_rows(volumes))
        targets = {d for v in MEASURED_HITS.values() for _k, _p, d in v if re.fullmatch(ID, d)}
        assert targets == {"A114", "A133"}, targets
        assert targets <= phantoms, sorted(targets - phantoms)
        assert targets & narrow == set(), sorted(targets & narrow)


class TestWhatL263Closed:
    """本轮补掉的两格：正文那句判决从此在定级格里也说得出"""

    def test_both_cells_carry_the_verdict(self):
        volumes = load_volumes()
        for volume, ident, cell, _round in CLOSED_AT_L263:
            row = rows_of(volumes, ident)
            assert len(row) == 1 and row[0]["volume"] == volume, row
            assert other.grade_cell(row[0]["text"]) == cell, (ident, other.grade_cell(row[0]["text"]))
            assert MARK.search(other.grade_cell(row[0]["text"])), ident

    def test_both_rounds_resolve_in_their_own_volume(self):
        """判决轮号取自正文（L220 / L46），且都在**同一卷**的声明面查得到 ⇒ 不需要「第二本账」限定词"""
        faces = other.declared_rounds(load_volumes())
        for volume, ident, _cell, rnd in CLOSED_AT_L263:
            assert rnd in faces[volume], (ident, rnd, volume)
            assert MARK.search(f"{other.grade_cell(rows_of(load_volumes(), ident)[0]['text'])}")

    def test_neither_row_is_a_candidate_anymore(self):
        got = blind_face()
        for _v, ident, _c, _r in CLOSED_AT_L263:
            assert ident not in got, ident


class TestRulerBites:
    """注入臂全部在内存里改行，不碰磁盘：每臂点名喊红，证明上面几张名册不是恒真"""

    def test_arm_unmarking_a274_makes_it_a_whole_row_verdict(self):
        """把本轮补上的定级格收回裸字母 ⇒ 该格立刻重新进盲区，且档是「整行判决」⇒ 空集断言红"""
        volumes = _patch_cell(load_volumes(), "OPTIMIZATION_LOOP_2.md", "A274", "S")
        got = blind_face(volumes)
        assert "A274" in got and got["A274"]["grade"] == FULL, got.get("A274")
        assert {k for k, v in got.items() if v["grade"] == FULL} == {"A274"}

    def test_arm_unmarking_a71_makes_it_a_whole_row_verdict(self):
        volumes = _patch_cell(load_volumes(), "OPTIMIZATION_LOOP.md", "A71", "S")
        got = blind_face(volumes)
        assert "A71" in got and got["A71"]["grade"] == FULL, got.get("A71")

    def test_arm_dropping_a_scope_token_promotes_the_row_to_a_verdict(self):
        """限定词表不是「有散文就放行」：摘掉 `A136` 那句「假臂那一侧」，本行升成整行判决"""
        volumes = _patch_text(load_volumes(), "OPTIMIZATION_LOOP.md", "A136", "（假臂那一侧）", "（假臂）")
        got = blind_face(volumes)
        assert got["A136"]["grade"] == FULL, got["A136"]["hits"]

    def test_arm_verb_then_round_without_brackets_is_caught(self):
        """形状②（`已收口，见 Lxxx`）现量 0 格，但确实读得到：注入一臂当场进盲区且算整行判决"""
        volumes = _patch_text(load_volumes(), "OPTIMIZATION_LOOP_2.md", "A343",
                              "定级 M，待拍。", "定级 M，待拍。本轮已收口，见 L263")
        got = blind_face(volumes)
        assert "A343" in got and got["A343"]["grade"] == FULL, got.get("A343")
        assert [h for _k, h, _b in got["A343"]["hits"]][0] == "已收口，见 L263", got["A343"]["hits"]

    def test_arm_a_negative_prose_shape_is_not_a_verdict(self):
        """反方向的牙齿：写「未关闭」不算判决，否则普查面会把一堆仍开放的行读成收了"""
        volumes = _patch_text(load_volumes(), "OPTIMIZATION_LOOP_2.md", "A343",
                              "定级 M，待拍。", "定级 M，待拍。但那一半 L263 未关闭")
        got = blind_face(volumes)
        assert "A343" not in got
        assert verdicts("本轮 L263 未关闭，那一半仍在案") == [], verdicts("本轮 L263 未关闭")

    def test_arm_marking_a_still_open_row_moves_the_blind_face_out(self):
        """给仍待拍的 `A121` 补一格 ⇒ 它退出盲区（盲区 −1），证明盲区数不是靠正文措辞撑着"""
        volumes = _patch_cell(load_volumes(), "OPTIMIZATION_LOOP.md", "A121", "~~S~~ 已关闭（L75）")
        got = blind_face(volumes)
        assert "A121" not in got and len(got) == MEASURED_BLIND - 1, sorted(got)


class TestRulerIsProven:
    """反空转：判据必须真收到东西，否则「相等」可能是恒真"""

    def test_the_census_actually_reads_rows(self):
        volumes = load_volumes()
        rows = other.backlog_rows(volumes)
        hits = [r for r in rows if verdicts(r["text"])]
        assert len(hits) >= MEASURED_BLIND, len(hits)
        assert len({r["id"] for r in hits}) >= MEASURED_BLIND

    def test_the_blind_face_sits_in_volume_one_only(self):
        """按实量钉两卷分布：残余 5 格全在卷一（补标前的 7 格里卷二只占 `A274` 一格）。
        卷二这一档是**量出来的 0**，不是不可达——`test_arm_unmarking_a274...` 把那一格收回裸字母时，
        盲区立刻多出卷二的一行"""
        got = blind_face()
        by_volume = {name: {k for k, v in got.items() if v["volume"] == name} for name in VOLUMES}
        assert by_volume == {
            "OPTIMIZATION_LOOP.md": {"A45", "A121", "A127", "A136", "A143"},
            "OPTIMIZATION_LOOP_2.md": set(),
        }, by_volume
        revived = blind_face(_patch_cell(load_volumes(), "OPTIMIZATION_LOOP_2.md", "A274", "S"))
        vol2 = [k for k, v in revived.items() if v["volume"] == "OPTIMIZATION_LOOP_2.md"]
        assert vol2 == ["A274"], vol2

    def test_the_pre_shape_is_the_whole_measured_face(self):
        """两个形状分开量：形状①（轮号在动词之前）现量 5 格、形状②（动词在轮号之前而没有 `（`）
        现量 0 格 ⇒ 第二档是量出来的空集，不是没写"""
        volumes = load_volumes()
        pre = post = 0
        for ident, entry in blind_face(volumes).items():
            got = {h["shape"] for h in verdicts(rows_of(volumes, ident)[0]["text"])}
            pre += "pre" in got
            post += "post" in got
        assert (pre, post) == (MEASURED_BLIND, 0), (pre, post)
