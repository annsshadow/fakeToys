# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

r"""L242 守卫：进度声明面说「收了」，Backlog 表那一格就必须有标记

**为什么立这一条**：A265 那把尺子（账本进度一致性）管的是「一轮写两遍自己」——清单行与
循环日志块互为对账。但账本有**第三面**：Backlog 表里每个 `Axxx` / `Bxxx` 行的定级格。一轮收掉
一笔账，如果只在清单行与日志块里写了「Axxx 收口」，而表里那一格还挂着出厂的 `| S |`，后来人顺着
表读到的就是「这笔还没做」——声明面与账目面对不上，而且**没有任何机械通道会出声**。本轮开工前
实测这种欠账 **20 笔**（`MEASURED_GAP_BEFORE`），其中 17 笔是真漏标、3 笔是合法的「半程收口」。
这与 L241 那条「钉尺子必须点名它量的是哪一条路」同族：双写纪律加到第三条边，就得有尺子把三条边
同时钉住。

**口径（每一条都是现量选出来的，不是抄的）**：

1. **声明面 = 清单行（`- [x] **Lxxx** …`）+ 循环日志的标题行（`### Lxxx（…）— …`）**。这两个
   形状是「一轮的判决」被写下来的地方，也是 A265 那把尺子读的同两面，所以共用一个提取口。
   **日志块正文不进声明面**：实测宽口径（把正文 bullet 也收进来）多收到 6 个 ID，而这 6 个
   **全都已经标了关闭** ⇒ 窄口径没有漏掉任何一笔真欠账；反过来，正文里充满「未收口」「判负」
   这类带否定与条件的叙述句，收进来只会造出假欠账。这一条按「断言范围与禁令范围同族」拍定，
   并且由 `test_wide_scope_would_give_the_same_gap` 现场复验，不是写在纸上自证。
2. **对账只在「表里还有这一行」的面上做**。两卷 Backlog 表会裁撤已收的历史行：声明面里点到、
   但表里已经没有行的 ID 现量 46 个（`MEASURED_PHANTOM`，例如 L37 关的那批两位数账号），它们
   **不构成欠账**，由 `test_phantom_count_matches_measured` 记成现量而不是假装不存在。
3. **ID 认一位到三位**（`[AB]` + 1–3 位数字，尾部 `(?!\d)` 防贪心）：卷一的早轮有一位数与两位数
   账号，只认三位会让整个早轮面失明——本轮实测「只认三位」的名册是 **11 笔**，认到两位才涨到
   真实的 **20 笔**；再认到一位**不改变 gap 名册**（仍是那 20 笔），但账目面从 234 行涨到 247 行
   （多出的 13 行是 A1–A8、B1 / B2 / B5 / B7 / B8），少认一位就是让这 13 行在对账里永久隐身。
4. **一句里连点多个名字要展开**：`A95/A122`、`A149、A150`、`A128–A131`（en dash 与 ASCII
   hyphen 都算）⇒ 先按 `LIST` 抓整串，再 `expand()` 成集合；区间按**自然位数**展开，不补零——
   账本里从不写 `A08`，补零会造出表里不存在的假 ID，把 `MEASURED_PHANTOM` 污染成无法解释的数字。
5. **动词与名字的距离有上限**：`AFTER` 只允许中间夹 20 个非括号、非竖线、非反引号的字符，且
   **括号一律算断口**——否则「A328（一段很长的说明）… 收口」会把不相干的两半粘成一句判决。
6. **表行标记形状（`MARK`）两支**：动词 + `（L`（`已关闭（L` / `已收口（L` / `关闭（L` /
   `收口（L` / `结案（L` / `已修（L`），以及**被划掉的定级字母** `~~S~~`。两卷书写惯例不同：
   卷二早期行把「已关闭（Lxxx）」写在说明格开头，近期行把 `~~X~~ 已收口（Lxxx）` 写在定级格，
   两支都要认。
7. **`全清` / `清零` / `永久记档` 不算标记**，**划掉的数字也不算标记**（口径 7 由
   `TestWeakTokensAreNotMarks` 现场证伪）：现量证据是卷一的 A298 行内写着「按 id 全清」而那笔账
   至今待拍，A176 行内写着 `~~6 处~~`（划掉一个旧读数）而它同样待拍。若把这两种形状算成标记，
   待拍的账就在对账里永久隐身。

**判据是「精确相等」而不是「棘轮」**（承 L241 的同一条理由）：`declared ∩ rows − marked` 必须
**恰好等于** `PARTIAL`。写成「≤ 某数」的话，本轮补标的 17 格里任何一格回退成无标记，只要总数没
超上限就照样绿——那是把尺子调成看不见缺陷。名册变化的唯一正确姿势是「要么补标，要么给出限定词
证据并重钉名册」。

**`PARTIAL` 不是白名单，是被验的**：名册每一格都要满足三条——① 某条声明带着限定词
（`前半` / `一半` / `部分` / 只关 `①`），② 表行自己没有标记，③ 该格的叙事由各自的用例点名。
实测三格：A127「前半程收口」（后半转给 A128 到 A131）、A156「本进程那一半收口」（跨进程那一半
另立 A158，定级格写的是「半修（L88）」）、A45「L36 只关 ①」（`target_size` 取整损失那一档，
②③④ 按行内标注仍未办）。

**与既有尺子的分工**：表格列数、粗体成对、行尾不混用由 `test_docs_markdown_structure` 管；
清单行与日志块互为对账由 `test_ledger_progress_consistency_l203` 管；文档里的 `.py:NNN` 定位引用
由 `test_doc_line_refs_l79` 管。**本守卫只管「声明面 ⇒ 表行有标记」这一个方向**。

**本守卫的边界（如实声明，全部立进 A328）**：反方向（表行标了关闭 ⇒ 标记里的轮号能在**同一卷**
的进度声明面查到）**没有**本轮钉住。补标之后的现量：132 行有标记，标记词汇形状 **7 种**且没有
封闭清单（`已关闭（L` / `已收口（L` / `关闭（L` / `收口（L` / `已修（L` / `~~S~~` / `~~M~~`），
换一种新写法（例如未来的「已了结（Lxxx）」）对本守卫结构性失明；按严格式「（L+数字」解析轮号，
有 **10 行**解析不出（它们写成 `已关闭（第二本账 L185…）`，限定词插在括号与轮号之间）、另有
**3 行**（A46 / A130 / A138）解析得出轮号但那一轮不在本卷。放宽到行内任意 `L\d+` 则 132 行全部
「可查」——但那已经不是标记里的轮号，而是行内任一提及，不足以当判据。要把反方向钉住，先要给
「标记形状」与「轮号解析式」一份封闭清单，那是 A328 的活。
"""
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[2]

#: 两卷账本。卷一与卷二都是纯 LF（本轮摘掉了卷二里那一个历史游离 CR），读取仍按字节再归一。
VOLUMES = ("OPTIMIZATION_LOOP.md", "OPTIMIZATION_LOOP_2.md")

#: 口径 3：早轮有一位数与两位数账号（A1–A8、A45…），只认三位会让整个早轮面失明。
#: `(?!\d)` 是必需的：没有它，`A1234` 会被贪心切成 `A123`，凭空造出没行的 ID。
ID = r"(?:[AB]\d{1,3})(?!\d)"
#: 口径 4：一句里连点多个名字——斜杠 / 顿号 / 逗号 / dash 区间。
LIST = rf"{ID}(?:\s*[/、,]\s*\**{ID}\**|\s*[–-]\s*{ID})*"
#: 动词在前：「关闭 A48」「合轮收口：A95/A122」。
BEFORE = re.compile(rf"(?:收口|关闭|结案|已修|收)\s*[`*]*\s*({LIST})")
#: 名字在前：「A265 收口」「A155 自此关闭」。括号与竖线是断口，距离上限 20 字符。
AFTER = re.compile(rf"({LIST})(?:[^A-Za-z0-9|*`（(]{{0,20}})?(?:收口|结案|关闭|已修|清零)")
#: 口径 6：两支关闭标记。
MARK = re.compile(r"(?:已关闭|已收口|关闭|收口|结案|已修)（L|~~[ \t]*[A-Z]{1,2}[ \t]*~~")
#: 合法的「半程收口」限定词。
PARTIAL_QUALIFIER = re.compile(r"前半|一半|部分|①")
ROW = re.compile(rf"^\| ({ID}) \|")
CHECK_LINE = re.compile(r"^- \[x\] \*\*L\d+")
LOG_HEAD = re.compile(r"^### L\d+")

#: 现量读数（本轮补标之后）。**精确相等**，理由见模块 docstring。
MEASURED_ROWS = 248
MEASURED_DECLARED = 116
MEASURED_MARKED = 132
#: 声明面点到、但表里已经没有行的 ID 数（口径 2，历史行被裁撤）。
MEASURED_PHANTOM = 46
#: 本轮开工前实测的欠账笔数：17 笔补标 + 3 笔半程。
MEASURED_GAP_BEFORE = 20
#: 反空转下界：解析器退化到看不见东西时先在这里响，不等集合断言。
MIN_DECLARED = 100

#: 允许「宣称收口而表行不标」的 ID——必须是半程收口，且每格有各自的点名用例。
PARTIAL = frozenset({"A127", "A156", "A45"})
#: 本轮补标的十七格——既是判据对象，也是注入臂的取样面。
PATCHED = frozenset({
    "A122", "A138", "A147", "A155", "A172", "A182", "A265", "A266", "B265",
    "A48", "A51", "A53", "A64", "A67", "A69", "A72", "A95",
})


def read_volume(name):
    """按字节读一份账本并归一成 LF 行表，不经通用换行翻译"""
    data = (ROOT / name).read_bytes().decode("utf-8")
    return data.replace("\r\n", "\n").split("\n")


def load_volumes():
    return {name: read_volume(name) for name in VOLUMES}


def expand(blob):
    """把 `A95/A122` 与 `A128–A131` 这类连写摊成 ID 集合，区间按自然位数展开（口径 4）"""
    blob = blob.replace("*", "").replace("`", "")
    out = set(re.findall(ID, blob))
    for lo, hi in re.findall(rf"({ID})\s*[–-]\s*({ID})", blob):
        out |= {lo[0] + str(n) for n in range(int(lo[1:]), int(hi[1:]) + 1)}
    return out


def table_rows(volumes):
    """`{ID: [(卷, 行号, 整行)]}` —— 只有行首的 `| Axxx |` 才算表行，散文里点名不算"""
    rows = {}
    for name, lines in volumes.items():
        for i, line in enumerate(lines, 1):
            hit = ROW.match(line)
            if hit:
                rows.setdefault(hit.group(1), []).append((name, i, line))
    return rows


def declared(volumes, rows=None):
    """声明面：`{ID: [(卷, 行号, 命中上下文)]}`；`rows` 给了就只留在账的 ID（口径 2）"""
    out = {}
    for name, lines in volumes.items():
        for i, line in enumerate(lines, 1):
            if not (CHECK_LINE.match(line) or LOG_HEAD.match(line)):
                continue
            for pattern in (BEFORE, AFTER):
                for hit in pattern.finditer(line):
                    for ident in expand(hit.group(1)):
                        if rows is not None and ident not in rows:
                            continue
                        out.setdefault(ident, []).append(
                            (name, i, line[max(0, hit.start() - 30): hit.end() + 15])
                        )
    return out


def marked(rows):
    return {
        ident
        for ident, entries in rows.items()
        if any(MARK.search(line) for _, _, line in entries)
    }


def gap_of(volumes):
    rows = table_rows(volumes)
    return set(declared(volumes, rows)) - marked(rows)


class TestRosterIsReconciled:
    """判据本体：声明面与账目面的差集必须恰好等于半程名册"""

    def test_gap_equals_partial_roster(self):
        volumes = load_volumes()
        actual = gap_of(volumes)
        assert actual == PARTIAL, (
            f"宣称收口却没标的名册 = {sorted(actual)}；本守卫的名册 = {sorted(PARTIAL)}。"
            "唯一正确姿势：给缺标的补上标记，或给出限定词证据并重钉名册"
        )

    def test_front_half_exception_is_qualified(self):
        """A127：声明带「前半程」，后半转给 A128 到 A131，表行自己没有标记"""
        volumes = load_volumes()
        sites = declared(volumes, table_rows(volumes))["A127"]
        assert any("前半" in ctx for _, _, ctx in sites), [c for _, _, c in sites]
        assert not MARK.search(table_rows(volumes)["A127"][0][2])

    def test_half_fixed_exception_is_qualified(self):
        """A156：声明带「那一半」，定级格写的是「半修（L88）」而不是「已修」"""
        volumes = load_volumes()
        sites = declared(volumes, table_rows(volumes))["A156"]
        assert any("一半" in ctx for _, _, ctx in sites), [c for _, _, c in sites]
        row = table_rows(volumes)["A156"][0][2]
        assert "半修（" in row, "定级格不再是「半修」形状：要么全收（那就从名册删掉），要么口径变了"
        assert not MARK.search(row)

    def test_first_slot_only_exception_is_qualified(self):
        """A45：L36 只关 ①（`target_size` 取整损失），②③④ 按行内标注仍未办"""
        volumes = load_volumes()
        sites = declared(volumes, table_rows(volumes))["A45"]
        assert any("A45 ①" in ctx for _, _, ctx in sites), [c for _, _, c in sites]
        row = table_rows(volumes)["A45"][0][2]
        assert "④" in row, "A45 行内不再有「④」那档未办的标注，本用例的样本失效"
        assert not MARK.search(row)

    def test_patched_rows_are_now_marked(self):
        """本轮补标的十七格逐格验：不认「大概都标了」"""
        volumes = load_volumes()
        rows = table_rows(volumes)
        unmarked = {k for k in PATCHED if k not in marked(rows)}
        assert not unmarked, f"这些格仍然没标：{sorted(unmarked)}"

    def test_round_arithmetic_closes(self):
        """补标十七格 + 半程三格 = 开工前实测的二十笔欠账（数字必须自己闭合）"""
        assert len(PATCHED) + len(PARTIAL) == MEASURED_GAP_BEFORE


class TestTableIsAPureFunction:
    """ID → 表行必须是函数：重复行会让对账结论取决于「撞到哪一行」"""

    def test_backlog_ids_are_unique(self):
        volumes = load_volumes()
        rows = table_rows(volumes)
        dupes = {k: [(n, i) for n, i, _ in v] for k, v in rows.items() if len(v) > 1}
        assert not dupes, f"同一 ID 有多行：{sorted(dupes.items())}"
        assert len(rows) == MEASURED_ROWS

    def test_rows_are_table_lines_with_a_grade_cell(self):
        """结构自检：每行以 `| ID | ` 起、以 ` |` 止。列数与粗体由 markdown 结构守卫管，不重复"""
        volumes = load_volumes()
        bad = []
        for ident, entries in table_rows(volumes).items():
            for name, i, line in entries:
                if not (line.startswith(f"| {ident} | ") and line.endswith(" |")):
                    bad.append((name, i, line[:50]))
        assert not bad, f"表行形状不合：{bad[:4]}"


class TestRulerIsNotIdle:
    """反空转：解析器必须真的收到东西，否则「相等」可能是恒真"""

    def test_declaration_face_is_populated(self):
        volumes = load_volumes()
        dec = declared(volumes, table_rows(volumes))
        assert len(dec) >= MIN_DECLARED, f"声明面只收到 {len(dec)} 个 ID，判据已失去意义"
        assert len(dec) == MEASURED_DECLARED

    def test_marked_count_matches_measured(self):
        volumes = load_volumes()
        assert len(marked(table_rows(volumes))) == MEASURED_MARKED

    def test_both_volumes_contribute_to_both_faces(self):
        volumes = load_volumes()
        rows = table_rows(volumes)
        dec = declared(volumes, rows)
        for name in VOLUMES:
            in_rows = sum(1 for e in rows.values() if e[0][0] == name)
            in_decl = sum(1 for sites in dec.values() if any(s[0] == name for s in sites))
            assert in_rows and in_decl, f"{name} 的一面对空转（表行 {in_rows} / 声明 {in_decl}）"

    def test_phantom_count_matches_measured(self):
        """口径 2 的现量：声明面点到、表里没行的 ID——裁撤历史行的结果，不是欠账"""
        volumes = load_volumes()
        rows = table_rows(volumes)
        phantoms = set(declared(volumes)) - set(rows)
        assert len(phantoms) == MEASURED_PHANTOM, sorted(phantoms)[:12]

    def test_wide_scope_would_give_the_same_gap(self):
        """窄口径不是「少看」：把日志块正文也收进来，gap 名册一字不变（本轮实测）"""
        volumes = load_volumes()
        rows = table_rows(volumes)
        mk = marked(rows)
        wide = set()
        for name, lines in volumes.items():
            in_log = False
            for line in lines:
                if LOG_HEAD.match(line):
                    in_log = True
                elif line.startswith("## "):
                    in_log = False
                is_head = CHECK_LINE.match(line) or LOG_HEAD.match(line)
                if not is_log_body(line, in_log) and not is_head:
                    continue
                for pattern in (BEFORE, AFTER):
                    for hit in pattern.finditer(line):
                        wide |= {i for i in expand(hit.group(1)) if i in rows}
        assert wide - mk == gap_of(volumes)
        assert len(wide) > len(declared(volumes, rows)), "宽口径并没有多收 ID ⇒ 这条对照空转了"


def is_log_body(line, in_log):
    """日志块正文里的 bullet——宽口径专用，窄口径不收"""
    return in_log and line.lstrip().startswith(("- ", "* "))


class TestWeakTokensAreNotMarks:
    """口径 7 的可证伪面：弱词与划掉的数字都不算关闭标记"""

    def test_quanqing_alone_does_not_mark_a_row(self):
        volumes = load_volumes()
        row = table_rows(volumes)["A298"][0][2]
        assert "全清" in row, "A298 那行不再含「全清」，本用例的样本失效，换一个真实形状"
        assert not MARK.search(row), "「全清」被当成关闭标记了——待拍的账会永久隐身"

    def test_strikethrough_numbers_do_not_mark_a_row(self):
        volumes = load_volumes()
        row = table_rows(volumes)["A176"][0][2]
        assert "~~" in row, "A176 那行不再有划掉的数字，本用例的样本失效"
        assert not MARK.search(row), "划掉的数字被当成定级作废——A176 仍在待拍面上"

    def test_grade_strikethrough_and_verb_both_mark(self):
        assert MARK.search("| A999 | x | y | ~~S~~ 已修（L1） |")
        assert MARK.search("| A999 | x | **已关闭（L1）**：细节 | S |")
        assert MARK.search("| A999 | 位置 | 说明 | ~~M~~ 已收口（L1） |")
        assert not MARK.search("| A999 | x | 说明里写着 ~~6 处~~ 已作废 | M |")
        assert not MARK.search("| A999 | x | 删除的粒度按 id 全清 | M |")
        assert not MARK.search("| A999 | x | 只写着 半修（L1） 的半程格 | M |")


class TestRulerIsProven:
    """尺子自证：八臂合成注入，零写入磁盘，每臂都指定它要红的那一支判据"""

    @staticmethod
    def _replace_line(volumes, name, index, new_line):
        src = volumes[name]
        patched = dict(volumes)
        patched[name] = src[:index] + [new_line] + src[index + 1:]
        return patched

    def test_arm_unmarking_a_row_reopens_the_gap(self):
        """把某一格已标的表行还原成出厂形状 ⇒ 它必须重新进 gap（尺子抓得住回退）"""
        volumes = load_volumes()
        rows = table_rows(volumes)
        ident = next(k for k in sorted(PATCHED) if MARK.search(rows[k][0][2]))
        name, index, line = rows[ident][0]
        stripped = re.sub(r"~~[ \t]*[A-Z]{1,2}[ \t]*~~[ \t]*", "", line)
        stripped = re.sub(r"(?:已关闭|已收口|已修|关闭|收口|结案)（L", "（L", stripped)
        assert stripped != line, f"{ident} 的标记拆不掉，样本失效"
        assert stripped.count("|") == line.count("|")
        patched = self._replace_line(volumes, name, index - 1, stripped)
        assert ident not in marked(table_rows(patched))
        assert ident in gap_of(patched), f"摘掉 {ident} 的标记却不进 gap ⇒ 尺子看不见回退"

    @staticmethod
    def _pick_open_row(volumes, rows):
        """挑一格「既没标、也没被声明过」的表行——两臂共用的取样口"""
        dec = set(declared(volumes, rows))
        mk = marked(rows)
        for ident in sorted(rows):
            if ident not in dec and ident not in mk:
                return ident, rows[ident][0]
        raise AssertionError("找不到一格未开账的表行，取样面空转")

    def test_arm_unmarked_row_can_be_marked(self):
        """反向臂：把标记**加**给一个本来没标的待拍行 ⇒ 它必须进 marked（尺子不靠运气）"""
        volumes = load_volumes()
        rows = table_rows(volumes)
        ident, (name, index, line) = self._pick_open_row(volumes, rows)
        assert ident not in marked(rows)
        marked_line = line[:-3].rstrip() + " ~~M~~ 已关闭（L1） |"
        patched = self._replace_line(volumes, name, index - 1, marked_line)
        assert ident in marked(table_rows(patched))
        assert ident not in gap_of(patched)

    def test_arm_deleting_a_declaration_line_shrinks_declared(self):
        """删掉一条清单行 ⇒ 它点名的 ID 从声明面消失（提取口真的在读清单行）"""
        volumes = load_volumes()
        rows = table_rows(volumes)
        before = set(declared(volumes, rows))
        name, index, line = next(
            (n, i, l)
            for n, lines in volumes.items()
            for i, l in enumerate(lines)
            if CHECK_LINE.match(l) and declared({n: [l]}, rows)
        )
        patched = self._replace_line(volumes, name, index, "- [x] **L999** 占位：删掉一条清单行")
        shrunk = before - set(declared(patched, table_rows(patched)))
        assert shrunk, "删掉一条声明行，声明面纹丝不动 ⇒ 提取口不认清单行"

    @staticmethod
    def _pick_declaration_line(volumes):
        """取一条真清单行做落点——声明臂改的是声明面，不是账目面"""
        for name in VOLUMES:
            hits = [(i, l) for i, l in enumerate(volumes[name]) if CHECK_LINE.match(l)]
            if hits:
                return name, hits[-1]
        raise AssertionError("清单行一面空转，声明臂没有落点")

    def test_arm_adding_a_declaration_reopens_the_reconciliation(self):
        """给一条清单行加上「关闭 X」⇒ X 立刻进 gap（证明判据会**变红**，不是只会被动变小）"""
        volumes = load_volumes()
        rows = table_rows(volumes)
        ident, _ = self._pick_open_row(volumes, rows)
        assert ident not in gap_of(volumes)
        name, (index, line) = self._pick_declaration_line(volumes)
        patched = self._replace_line(volumes, name, index, line + f"；关闭 {ident}")
        assert ident in gap_of(patched), f"声明面收了 {ident} 却没进 gap ⇒ 对账不会为新增欠账而红"

    def test_arm_slash_list_expands(self):
        assert expand("A95/A122") == {"A95", "A122"}
        assert expand("**A138** 与 A147") == {"A138", "A147"}
        assert expand("A149、A150") == {"A149", "A150"}

    def test_arm_range_expands(self):
        assert expand("A128–A131") == {"A128", "A129", "A130", "A131"}
        assert expand("A149-A151") == {"A149", "A150", "A151"}
        assert expand("A8–A11") == {"A8", "A9", "A10", "A11"}

    def test_arm_parenthesis_blocks_a_false_adjacency(self):
        """口径 5：括号是断口——「A328（很长的说明）… 收口」不能被粘成判决"""
        line = "- [x] **L999** 新立 A328（一段足够长的说明文字，中间提到别的账），下一轮再收口"
        hits = set()
        for pattern in (BEFORE, AFTER):
            for hit in pattern.finditer(line):
                hits |= expand(hit.group(1))
        assert hits == set(), f"粘上了：{sorted(hits)}"

    def test_arm_row_start_is_required(self):
        """散文里点名不算表行：只有行首 `| Axxx |` 才进账目面"""
        fake = {"OPTIMIZATION_LOOP.md": ["正文里说 A122 已收口", "| A122 | 位置 | 说明 | ~~S~~ 已修（L1） |"]}
        rows = table_rows(fake)
        assert set(rows) == {"A122"}
        assert len(rows["A122"]) == 1
