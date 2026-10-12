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
   `TestWeakTokensAreNotMarks` 现场证伪）：现量证据原是卷二的 A298 行内写着「按 id 全清」而那笔账
   当时待拍，A176 行内写着 `~~6 处~~`（划掉一个旧读数）而它同样待拍。若把这两种形状算成标记，
   待拍的账就在对账里永久隐身。**L250 重量**：A298 本轮收了 ⇒ live 样本换成 A328（那一行引用标记
   词汇本身而没有轮号括弧），「全清」那一形由同支的 `test_grade_strikethrough_and_verb_both_mark` 继续钉。
   **L253 重量**：A328 本轮收了，那一行现在自己就是标记 ⇒ 样本换成 A274（卷二，定级格是裸 S、正文写着
   「L220 已收口」但不带轮号括弧）。换代由该支自己的红读数逼出（`MARK.search` 在 A328 那行开了火），
   与 L250 那次同一处置：改样本不改尺子。
8. **表行面按区段过滤（L254 / A337 修法③）**：`## 循环日志` 之下的 `| Axxx |` **不算** Backlog 表行。
   日志块里的名册表格（46 行：A1–A8 是注入臂名册、B226–B263 是某一轮收口清单）此前混在这一格里，
   于是「表行数」的含义不唯一，而那 38 只从未有 Backlog 行的 ID 还在「声明 ⇒ 表行有标记」里充当
   自己的证据。过滤后各格现量：表行 **212**、留在册的声明面 **91**、有标记 **109**（与 l253 的整行
   那一面同数，两把尺子对上）、phantom **84** = 历史裁撤 46 + 日志名册 38（两族各有专支点名）。
   **判据本身一格未动**：过滤前后 `declared ∩ rows − marked` 都是 `PARTIAL` 三格，所以这次改的是
   「哪一行算表行」这一面的定义，不是松紧。

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

from tests.unit import test_backlog_mark_vocab_l253 as other

ROOT = pathlib.Path(__file__).resolve().parents[2]

#: 两卷账本。**无 CRLF 行尾**（L253 订正：不是「纯 LF」——卷二正文里有一处字面 CR 字节，那一格在讲
#: 行尾守卫写的是反引号包的 CR，是内容不是行尾）。这一格由 `test_line_endings_are_the_documented_shape`
#: 按字节现场验（CRLF 恒 0、卷二的游离 CR 恒恰 1 处），不写在注释里当自述。
VOLUMES = ("OPTIMIZATION_LOOP.md", "OPTIMIZATION_LOOP_2.md")

#: 区段边界（L254 / A337 修法③）。**只认这一形**：卷一的日志块标题不是 `### LNNN` 而是别的写法，
#: 用 `### L` 当边界会让卷一整个日志区留在「Backlog」里——现量核对过：卷一的 `## 循环日志` 在第 349 行、
#: 卷二在第 225 行，两卷的这一形各只出现一次（由 `test_the_boundary_is_unique_per_volume` 钉住）。
LOG_SECTION = "## 循环日志"

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
#: **L243 重量**：表行 248 → **251**（新立 A329 / A330 / A331 三行，全是待拍、不带标记），
#: 有标记 132 → **134**（A299 与 A300 各带一格 `~~X~~ 已收口（L243）`）。声明面 **116 一格未动**
#: ——本轮清单行刻意把三条新账写成「Axxx（限定词）」的括号形状（口径 5 的断口），所以既没有新增
#: 声明、也没有造出假欠账；这一格由 `test_gap_equals_partial_roster` 现场证明，不靠自述。
#: **L244 重量**：表行 251 → **252**（新立 A332 一行，待拍、不带标记）、有标记 134 → **135**
#: （A329 那一格收成 `~~S~~ 已收口（L244）`）、声明面 116 → **117**（L244 清单行第一次点名判决
#: A329——A329 立账时 L243 那格写的是「A329（CLI 只四档）」的括号形状，不进声明面）。 phantom 46
#: 一格未动：新声明的 A329 表行在场，不是历史被裁撤的那种 ID。三格都由本轮红读数当场逼出来的
#: （`assert 252 == 251` / `assert 117 == 116` / `assert 135 == 134`），不是推的。
#: **L245 重量**：表行 252 → **253**（新立 A333 一行，待拍、不带标记）、有标记 135 → **136**
#: （A331 那一格收成 `~~M~~ 已收口（L245）`）、声明面 117 → **118**（L245 清单行开头的「收 A331——」
#: 是这一格的第一次判决；A333 按口径 5 写成「新立 **A333**（内容维判不到的三种形状）」的括号形状，
#: 既不进声明面也不造假欠账）。phantom 46 与 `PARTIAL` 三格均未动。三格同样由本轮红读数当场逼出
#: （`assert 253 == 252` / `assert 118 == 117` / `assert 136 == 135`）。
#: **L246 重量**：表行 253 → **254**（新立 A334 一行，待拍、不带标记）、有标记 136 → **137**
#: （A327 那一格收成 `~~L~~ 已收口（L246）`）、声明面 118 → **119**（L246 清单行开头的「收 A327——」
#: 与日志标题同名一次；A334 按同一口径写成「新立 **A334**（`_embedding_cache` 无界驻留）」的括号
#: 形状，不进声明面，故 `test_gap_equals_partial_roster` 这一格**没有**多出假欠账）。phantom 46 与
#: `PARTIAL` 三格均未动。三格由本轮红读数当场逼出（`assert 254 == 253` / `assert 119 == 118` /
#: `assert 137 == 136`），写在账本里的预测值 254 / 137 / 119 与红读数逐格相同，仍以红读数为准。
#: **L247 重量**：表行 254 **未动**（本轮不收新账，A334 那一行已在册）、有标记 137 → **138**
#: （A334 收成 `~~M~~ 已收口（L247）`）、声明面 119 → **120**（L247 清单行开头的「收 A334——」与
#: 日志标题同名一次）。phantom 46 与 `PARTIAL` 三格均未动。两格由本轮红读数当场逼出
#: （`assert 120 == 119` / `assert 138 == 137`），与账本里写的预测值逐格相同，仍以红读数为准。
MEASURED_ROWS = 212  # **L254 重量（口径变了，不是数字变了）**：257 → **212**。这一格从「两卷所有行首 `| Axxx |`」收成「Backlog 区的行首 `| Axxx |`」——`## 循环日志` 之下的 46 行名册表格不再算表行（A337 修法③）。46 = A1–A8（注入臂名册）+ B226–B263（某一轮收口清单），由 `test_the_filtered_out_rows_are_the_named_forty_six` 逐格点名。**L253 重量**：256 → **257**（新立 A337 一行：收 A328 时把标记判据从整行收到定级格，同场量到的「定级格裸字母而正文写着闭合语」44 行两格差 + 日志块那 46 行污染表行面，独立待拍、不带标记）。红读数 `assert 257 == 256`。**L252 重量**：255 → **256**（新立 A336 一行：收 A330 时同场量到的反方向缺口——不带请求体的三员被框架自动声明 422 而构造不出，独立待拍、不带标记）。红读数 `assert 256 == 255`。**L250 重量**：254 → **255**（新立 A335 一行：A303 的修法② 那一半拆出来独立待拍，不带标记）。红读数 `assert 255 == 254`。**L251 一格未动**：本轮只收 A304 那一格的原地改写，不收新账
MEASURED_DECLARED = 91  # **L254 重量**：128 → **91**。两件事叠在这一格：区段过滤把 B226–B263 那 38 只「只在日志块名册里出现、从未有 Backlog 行」的 ID 请出留在册的声明面（128 → 90），本轮 A337 第一次进声明面（90 → 91）。**不过滤的那一面由 `test_the_unfiltered_face_still_pairs_with_the_filtered_one` 继续钉**（现量 129），所以下界从 100 降到 85 不是把尺子放松，是面本身缩了而两面都有数。**L253 重量**：127 → **128**（A328 第一次进声明面：L253 清单行开头的「收 A328——」与日志标题「收 A328：」各点一次，同一只 ID 只算一笔；A337 按口径 5 写成「新立 **A337**（……）」的括号形状，不进声明面 ⇒ `test_gap_equals_partial_roster` 没有多出假欠账，那一支本轮实测未红）。红读数 `assert 128 == 127`。**L252 重量**：126 → **127**（A330 第一次进声明面：L252 清单行开头的「收 A330——」与日志标题「收 A330：」各点一次，同一只 ID 只算一笔；A336 按口径 5 写成「新立 **A336**（……）」的括号形状，不进声明面 ⇒ `test_gap_equals_partial_roster` 没有多出假欠账，那一支本轮实测未红）。红读数 `assert 127 == 126`。**L251 重量**：125 → **126**（A304 第一次进声明面：L251 清单行开头的「收 A304——」与日志标题「收 A304：」各点一次，同一只 ID 只算一笔）。红读数 `assert 126 == 125`。**L250 重量**：123 → 125（+2：A298 与 A303 双双第一次进声明面——L250 清单行开头的「收 A298 + A303——」与句中的「同轮 A303 收口」、日志标题各点一次，两个 ID 各算一笔；A335 按口径 5 写成「新立 **A335**（……）」的括号形状，不进声明面）。红读数 `assert 125 == 123`
MEASURED_MARKED = 109  # **L254 重量**：146 → **109**。过滤请掉 38 格（B226–B263 那 38 行日志块名册行**带着标记**——它们是某一轮收口清单里的表格，标记写的是那一轮的账，不是 Backlog 的账），本轮 A337 补上一格 ⇒ 146 − 38 + 1 = **109**。这一格现在与 l253 那把尺子的「整行带闭合标记的 Backlog 行数」同面同数（108 + 1 = 109），两把独立尺子在同一只面上对上数 ⇒ 由 `test_the_other_ruler_counts_the_same_wide_face` 现场钉。**L253 重量**：145 → **146**（A328 那一格收成 `~~M~~` 已收口（L253）；A337 是新账不带标记。卷一 A31 那一格本轮补的是**轮号**，而它早就带划掉字母 ⇒ 宽面计数一格未动，这一笔只由 A328 贡献）。红读数 `assert 146 == 145`。**L252 重量**：144 → **145**（A330 那一格收成 `~~M~~` 已收口（L252）；A336 是新账不带标记）。phantom 46、`PARTIAL` 与表行那一格本轮跟着新账走（见上），弱词档未动。红读数 `assert 145 == 144`。**L251 重量**：143 → **144**（A304 那一格收成 `~~S~~` 已收口（L251）；A280 那一行原地补的是现量，定级格未动所以不算标记）。红读数 `assert 144 == 143`。**L250 重量**：141 → 143（A298 / A303 各收成 `~~M~~` / `~~S~~` 已收口（L250））。三格由本轮红读数当场逼出（`assert 256 == 255` / `assert 127 == 126` / `assert 145 == 144`），与清单行里写的预测值逐格相同，仍以红读数为准
#: 声明面点到、但 Backlog 表里没有行的 ID 数（口径 2）。**L254 起这一格有两种归因**：46 只是
#: 「历史行被裁撤」那一族，区段过滤之后另有 **38** 只是「日志块名册里的 ID 从来没有 Backlog 行」
#: （B226–B263，本轮请出表行面之后第一次现形）。两族名册分别由 `test_phantom_count_matches_measured`
#: 与 `test_the_thirty_eight_never_had_a_backlog_row` 点名。
MEASURED_PHANTOM = 84
MEASURED_PHANTOM_LOG_ROSTER = 38
#: 本轮开工前实测的欠账笔数：17 笔补标 + 3 笔半程。
MEASURED_GAP_BEFORE = 20
#: 反空转下界：解析器退化到看不见东西时先在这里响，不等集合断言。
#: **L254 重量**：100 → **85**。区段过滤把留在册的声明面从 128 缩到 91，下界必须跟着面走，
#: 否则这一格会在口径变更当场变成「假阳性红」。放松的担心由另一面挡着：不过滤的那一面
#: （现量 129）与日志块正文那一面各有独立断言，解析器一旦退化到看不见东西，三面里至少两面响。
MIN_DECLARED = 85

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


def table_rows(volumes, backlog_only=True):
    """`{ID: [(卷, 行号, 整行)]}` —— 只有行首的 `| Axxx |` 才算表行，散文里点名不算。

    **L254 加的区段过滤（A337 修法③）**：`## 循环日志` 之下的同名行**不算 Backlog 表行**——
    日志块里的名册表格（卷二 46 行：A1–A8 是注入臂名册、B226–B263 是某一轮收口清单）此前也被
    这一格数进去，于是「表行数」的含义不唯一，而 B226–B263 那 38 只从未有过 Backlog 行却在
    「声明 ⇒ 表行有标记」这一判据里充当自己的证据。`backlog_only=False` 只留给对照臂用。
    """
    rows = {}
    for name, lines in volumes.items():
        in_log = False
        for i, line in enumerate(lines, 1):
            if line.startswith(LOG_SECTION):
                in_log = True
            hit = ROW.match(line)
            if hit and not (backlog_only and in_log):
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


class TestTheSectionBoundary:
    """口径 8（L254 / A337 修法③）：`## 循环日志` 之下的名册表格不算 Backlog 表行"""

    def test_the_boundary_is_unique_per_volume(self):
        """边界只认 `## 循环日志` 这一形：卷一的日志块标题不是 `### LNNN`，拿它当边界会让
        卷一整个日志区留在「Backlog」里——那正是 A337 说含义不唯一的来源之一"""
        for name, lines in load_volumes().items():
            hits = [i for i, ln in enumerate(lines, 1) if ln.startswith(LOG_SECTION)]
            assert len(hits) == 1, (name, hits)

    def test_the_filtered_out_rows_are_the_named_forty_six(self):
        volumes = load_volumes()
        off = table_rows(volumes, backlog_only=False)
        on = table_rows(volumes)
        dropped = sorted(set(off) - set(on))
        assert dropped == sorted(
            [f"A{n}" for n in range(1, 9)] + [f"B{n}" for n in range(226, 264)]
        ), dropped
        assert len(dropped) == 46
        assert len(off) == MEASURED_ROWS + 46
        assert len(on) == MEASURED_ROWS
        # 被过滤掉的每一格都必须在边界之下，不是靠 ID 形状挑出来的
        for name, lines in load_volumes().items():
            edge = next(i for i, ln in enumerate(lines, 1) if ln.startswith(LOG_SECTION))
            for ident in dropped:
                for vol, line_no, _ in off.get(ident, []):
                    if vol == name:
                        assert line_no > edge, (name, ident, line_no, edge)

    def test_the_gap_is_the_same_on_both_faces(self):
        """核心判据没被这次过滤改松：开与关两侧的欠账名册都是 PARTIAL 三格"""
        volumes = load_volumes()
        on = table_rows(volumes)
        off = table_rows(volumes, backlog_only=False)
        assert set(declared(volumes, on)) - marked(on) == PARTIAL
        assert set(declared(volumes, off)) - marked(off) == PARTIAL

    def test_the_unfiltered_face_still_pairs_with_the_filtered_one(self):
        """反空转的另一半：留在册的声明面 91，不过滤那一面 91 + 38 = 129 也照样有数"""
        volumes = load_volumes()
        off = table_rows(volumes, backlog_only=False)
        assert len(declared(volumes, off)) == (
            MEASURED_DECLARED + MEASURED_PHANTOM_LOG_ROSTER
        )
        assert len(marked(off)) == MEASURED_MARKED + MEASURED_PHANTOM_LOG_ROSTER

    def test_the_thirty_eight_never_had_a_backlog_row(self):
        """B226–B263 是某一轮收口清单里的表格行，从未进过 Backlog 表——过滤之后它们改归
        phantom 那一族，且另一族（历史裁撤 46）的读数一格未动"""
        volumes = load_volumes()
        rows = table_rows(volumes)
        phantoms = set(declared(volumes)) - set(rows)
        roster = {f"B{n}" for n in range(226, 264)}
        assert phantoms & roster == roster, sorted(roster - phantoms)
        assert len(phantoms) == MEASURED_PHANTOM
        assert len(phantoms) - len(roster) == 46

    def test_the_other_ruler_counts_the_same_wide_face(self):
        """两把独立尺子在同一只面上对上数：l253 的「整行带闭合标记的 Backlog 行」= 本守卫
        过滤后的有标记名册。定级格那一面（权威面）严格小于它——那 44 行差是 A338 的账"""
        wide, narrow = other.faces(other.load_volumes())
        mine = marked(table_rows(load_volumes()))
        assert mine == wide, sorted(mine ^ wide)
        assert narrow < wide, sorted(narrow - wide)

    def test_line_endings_are_the_documented_shape(self):
        """订正历轮自述：两卷是「无 CRLF 行尾」而不是「纯 LF」——卷二正文里有一处字面 CR
        字节（讲行尾守卫那一格反引号包的那个），它是内容不是行尾，须原样留着"""
        for name in VOLUMES:
            data = (ROOT / name).read_bytes()
            assert b"\r\n" not in data, name
        assert (ROOT / VOLUMES[0]).read_bytes().count(b"\r") == 0
        assert (ROOT / VOLUMES[1]).read_bytes().count(b"\r") == 1


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

    def test_mark_verb_quoted_as_prose_does_not_mark_a_row(self):
        volumes = load_volumes()
        row = table_rows(volumes)["A274"][0][2]
        assert "已收口" in row, "A274 那行不再引用标记词汇本身，本用例的样本失效，换一个真实形状"
        assert not MARK.search(row), "行内引用标记词汇被当成关闭标记了——待拍的账会永久隐身（L250 之前的 live 样本是 A298 的「全清」、L253 之前是 A328，两形改由下面的合成断言与本行新样本继续钉）"

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
