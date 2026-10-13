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
   两支都要认。**L261 起「认哪两支」与「在哪一格认」分家**：词形仍是两支（口径 6 未松），
   但**判据只读定级格**——见口径 9。
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
   自己的证据。过滤后各格现量：表行 **216**、留在册的声明面 **95**、定级格有标记 **110**、phantom **84**
   = 历史裁撤 46 + 日志名册 38（两族各有专支点名）。
   **L261 起这一条从「省事」升成「承重」**：口径 9 把标记判据收到定级格之后，不过滤那一面的欠账名册
   多出全部 38 只日志名册 ID（它们的正文带着那一轮的闭合语而定级格没有），由
   `test_the_filter_is_load_bearing_on_the_narrow_face` 逐格点名 ⇒ 区段过滤不再是「让读数含义唯一」的
   整洁要求，而是判据不自我循环的前提。
9. **定级格是权威面（L261 / A338 修法②）**：`marked()` 只读表格最后一格。立这一条的理由来自
   `test_backlog_mark_vocab_l253` 量到的两格差——**真欠账是「正文写着判决而定级格还是裸字母」那一族**：
   旧判据按整行匹配，正文里的闭合语把这一族喂饱，于是它们永远进不了欠账名册。两面值之差
   （`TWO_FACE_ROSTER`）在 L261 补标之前是 44 行，补标之后是 **3** 行（A21 / B2 / B266：正文那句
   只管本行的一半或只点别的 ID）。收紧之后 `PARTIAL` 从三格走到**四格**（新增 B2，「前半关闭」那一句
   由 `test_the_excel_ingest_half_row_is_qualified` 点名），宽面一格未删、只降级成对照面，由
   `test_the_other_ruler_counts_the_same_wide_face` 继续钉住两面之差等于名册。

**判据是「精确相等」而不是「棘轮」**（承 L241 的同一条理由）：`declared ∩ rows − marked` 必须
**恰好等于** `PARTIAL`。写成「≤ 某数」的话，L242 补标的 17 格与 L261 补标的 41 格里任何一格回退成无标记，只要总数没
超上限就照样绿——那是把尺子调成看不见缺陷。名册变化的唯一正确姿势是「要么补标，要么给出限定词
证据并重钉名册」。

**`PARTIAL` 不是白名单，是被验的**：名册每一格都要满足三条——① 某条声明带着限定词
（`前半` / `一半` / `部分` / 只关 `①`），② 表行自己没有标记，③ 该格的叙事由各自的用例点名。
实测四格：A127「前半程收口」（后半转给 A128 到 A131）、A156「本进程那一半收口」（跨进程那一半
另立 A158，定级格写的是「半修（L88）」）、A45「L36 只关 ①」（`target_size` 取整损失那一档，
②③④ 按行内标注仍未办）、**B2**「前半关闭（L80）」（写边那一半 L84 已订正为关闭，剩下上传那一半
在 A132——L261 把标记判据收到定级格之后这一格第一次现形，此前宽面被正文喂饱所以永不红，
正是 A338 立账时写下的那句「这一族永远不会红」）。

**与既有尺子的分工**：表格列数、粗体成对、行尾不混用由 `test_docs_markdown_structure` 管；
清单行与日志块互为对账由 `test_ledger_progress_consistency_l203` 管；文档里的 `.py:NNN` 定位引用
由 `test_doc_line_refs_l79` 管。**本守卫只管「声明面 ⇒ 定级格有标记」这一个方向**。

**本守卫的边界（如实声明）**：反方向（定级格标了关闭 ⇒ 标记里的轮号查得到）**不在本守卫**，
由 `test_backlog_mark_vocab_l253` 钉住——那是 A328（L253 立尺、L261 复跑）的射程：闭合词 **5** 种、
字母形 **7** 种、组合名册 **18** 格三张封闭清单，加同卷可查 **97** / 跨卷 **13** 且全部写明「第二本账」
/ 括号里无轮号 **0** / 轮号解析不出 **0**。所以本守卫对「换一种新写法（例如未来的『已了结（Lxxx）』）」
仍结构性失明，那一格由 l253 的注入臂 A 当场点名，不由本守卫兜。**两面之差同样不在本守卫**：
定级格有标记 **110**、整行有标记 **113**，那 **3** 行差集（A21 / B2 / B266）由口径 9 与
`test_the_other_ruler_counts_the_same_wide_face` 各自点名——本守卫的判据用窄面，宽面只作对照。
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
#: 用 `### L` 当边界会让卷一整个日志区留在「Backlog」里——现量：卷一的这一形恰 1 处，卷二也恰 1 处，
#: 由 `test_the_boundary_is_unique_per_volume` 现场验。**不在这条注释里钉行号**：Backlog 区每长一行
#: 它就漂一格，而「本账第 NNN 行」那一形没有尺子量（A333 / A338 的账）。
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
MEASURED_ROWS = 218  # **L265 记账批二轮重量**：217 → **218**（新立 A344：日志块点名的哈希只要 git 解析得过就算归档，「这枚提交在不在分支上」全账无人量。红读数 `assert 218 == 217`）｜原 **L262 重量**：216 → **217**（新立 A343 一行：`x = x or DEFAULT` 把调用方显式传入的空容器读成「没传」，全仓普查 38 格分四族、四条 HTTP 可达路逐条点名，独立待拍、定级格是裸 `M` 且整行不带闭合词 ⇒ `MEASURED_DECLARED` 95 与 `MEASURED_MARKED` 110 两格未动，L262 清单行与日志标题一律不用收口那一族词，`gap` 仍是 `PARTIAL` 四格）。红读数 `assert 217 == 216`，同批由区段对照那一格逼出 `assert 263 == (217 + 46)` 同一张数。**L261 一格未动**：本轮零 Backlog 新行、零裁撤（41 处定级格与 A338 行全是同行改写），表行三面 262 / 216 / 46 逐格与 L259 相同。**L259 重量**：214 → **216**（新立 A341 与 A342 两行，都是待拍、定级格裸字母、整行不带闭合词 ⇒ `MEASURED_DECLARED` 94 与 `MEASURED_MARKED` 112 两格未动，L259 清单行与日志标题一律不用收口那一族词）。红读数 `assert 216 == 214`，同批另有 `assert 262 == (214 + 46)` 从区段对照那一格逼出同一张数。**L258 重量**：213 → **214**（新立 A340 一行：并行支路把 SDK 判决咽成 `success: true` 的假回执，独立待拍、不带标记；日志块那一面一格未动）。**同一次首跑另有五红，全部来自清单行末句「本轮零收口」**——`AFTER` 的 20 字窗口里 `）` 与汉字都不是断口，于是整串待拍名册（17 只，区间按自然位数展开）一次粘成假声明：声明面 94 → 108、gap 名册从三格涨到十七格。处置是**同行改写那句散文**（「零收口」换成不带动词的写法）而不是重钉 94 与 112：红的是案面，尺子一格未松；改写后声明面回到 **94**、gap 回到 `PARTIAL` 三格、`MEASURED_ROWS + 46 = 260` 与「不过滤声明面 = 94 + 38 = 132」两式同时闭合。**L257 重量**：212 → **213**（新立 A339 一行：A285 收口时同场量到的 `fields=[]` 假零不对称，独立待拍、不带标记；日志块那一面一格未动）。红读数 `assert 213 == 212`。**L254 重量（口径变了，不是数字变了）**：257 → **212**。这一格从「两卷所有行首 `| Axxx |`」收成「Backlog 区的行首 `| Axxx |`」——`## 循环日志` 之下的 46 行名册表格不再算表行（A337 修法③）。46 = A1–A8（注入臂名册）+ B226–B263（某一轮收口清单），由 `test_the_filtered_out_rows_are_the_named_forty_six` 逐格点名。**L253 重量**：256 → **257**（新立 A337 一行：收 A328 时把标记判据从整行收到定级格，同场量到的「定级格裸字母而正文写着闭合语」44 行两格差 + 日志块那 46 行污染表行面，独立待拍、不带标记）。红读数 `assert 257 == 256`。**L252 重量**：255 → **256**（新立 A336 一行：收 A330 时同场量到的反方向缺口——不带请求体的三员被框架自动声明 422 而构造不出，独立待拍、不带标记）。红读数 `assert 256 == 255`。**L250 重量**：254 → **255**（新立 A335 一行：A303 的修法② 那一半拆出来独立待拍，不带标记）。红读数 `assert 255 == 254`。**L251 一格未动**：本轮只收 A304 那一格的原地改写，不收新账
MEASURED_DECLARED = 95  # **L261 重量**：94 → **95**（A338 第一次进声明面：L261 日志标题「收 A338」点一次；清单行里 41 行的名册一律写作反引号包裹 + 「（）」括住的形状，口径 5 的断口挡住 ⇒ 没有造出假声明，`gap` 只多出 B2 那一格真半程）。**L257 重量**：93 → **94**（A285 第一次进声明面：L257 清单行开头的「收 A285——」与日志标题「收 A285：」各点一次，同一只 ID 只算一笔）。本轮新立 A339 一行 ⇒ `MEASURED_ROWS` 212 → **213**（另钉，见上），phantom 84 与日志名册 38 两格未动（A339 自己就有 Backlog 行，而清单行里它写作「并立 A339 待拍」、全账没有一处「收 A339」⇒ `test_gap_equals_partial_roster` 没有多出假欠账）⇒ 不过滤那一面由配对那支从 131 自动走到 **132**（= 94 + 38，等式自己推导，没有另钉）。红读数 `assert 94 == 93`。**L256 重量**：92 → **93**（A324 第一次进声明面：L256 清单行开头的「收 A324——」与日志标题「收 A324：」各点一次，同一只 ID 只算一笔）。本轮零新 Backlog 行 ⇒ `MEASURED_ROWS` 212、phantom 84、日志名册 38 三格未动，不过滤那一面由配对那支从 130 自动走到 **131**（= 93 + 38，等式自己推导，没有另钉）。红读数 `assert 93 == 92`。**L255 重量**：91 → **92**（A316 第一次进声明面：L255 清单行开头的「收 A316——」与日志标题各点一次，同一只 ID 只算一笔）。本轮零新 Backlog 行 ⇒ `MEASURED_ROWS` 212、phantom 84、日志名册 38 三格未动，不过滤那一面由配对那支从 129 自动走到 **130**（= 92 + 38，等式自己推导，没有另钉）。红读数 `assert 92 == 91`。**L254 重量**：128 → **91**。两件事叠在这一格：区段过滤把 B226–B263 那 38 只「只在日志块名册里出现、从未有 Backlog 行」的 ID 请出留在册的声明面（128 → 90），本轮 A337 第一次进声明面（90 → 91）。**不过滤的那一面由 `test_the_unfiltered_face_still_pairs_with_the_filtered_one` 继续钉**（现量 129），所以下界从 100 降到 85 不是把尺子放松，是面本身缩了而两面都有数。**L253 重量**：127 → **128**（A328 第一次进声明面：L253 清单行开头的「收 A328——」与日志标题「收 A328：」各点一次，同一只 ID 只算一笔；A337 按口径 5 写成「新立 **A337**（……）」的括号形状，不进声明面 ⇒ `test_gap_equals_partial_roster` 没有多出假欠账，那一支本轮实测未红）。红读数 `assert 128 == 127`。**L252 重量**：126 → **127**（A330 第一次进声明面：L252 清单行开头的「收 A330——」与日志标题「收 A330：」各点一次，同一只 ID 只算一笔；A336 按口径 5 写成「新立 **A336**（……）」的括号形状，不进声明面 ⇒ `test_gap_equals_partial_roster` 没有多出假欠账，那一支本轮实测未红）。红读数 `assert 127 == 126`。**L251 重量**：125 → **126**（A304 第一次进声明面：L251 清单行开头的「收 A304——」与日志标题「收 A304：」各点一次，同一只 ID 只算一笔）。红读数 `assert 126 == 125`。**L250 重量**：123 → 125（+2：A298 与 A303 双双第一次进声明面——L250 清单行开头的「收 A298 + A303——」与句中的「同轮 A303 收口」、日志标题各点一次，两个 ID 各算一笔；A335 按口径 5 写成「新立 **A335**（……）」的括号形状，不进声明面）。红读数 `assert 125 == 123`
MEASURED_MARKED = 112  # **L263 重量**：110 → **112**（`A274` / `A71` 两格：正文早就写着整行判决，但那句的形状是「轮号在动词之前」或「动词后面跟限定词」，`MARK` 两面都读不到，所以定级格一直停在裸 `S` —— 本轮由 `test_round_first_verdict_l263` 普查量到并落格，两格都不进声明面（`declared()` 只读清单行与日志标题）⇒ `MEASURED_DECLARED` 95 一格未动、`gap` 仍是 `PARTIAL` 四格）。红读数 `assert 112 == 110`。**L261 重量（口径 9 落地，这一格从此是「定级格有标记」而不是「整行有标记」）**：112 → **110**。三件事叠在同一格：① 判据从整行收到定级格 ⇒ 同一时刻读数从宽面 112 掉到 68；② 同批补标 41 行 ⇒ 68 + 41 = 109；③ A338 自己收成 `~~S~~ 已收口（L261）` ⇒ **110**。宽面那一面一格未删，仍由 l253 独立量到 **113**（= 110 + 3），两把尺子的同面对账改由 `test_the_other_ruler_counts_the_same_wide_face` 比 `narrow`。**L257 重量**：111 → **112**（A285 那一格收成划掉字母加轮号，形状走 `(~~S~~ / 已收口 / 括号 L / 不粗体` 那一档，没有造出新形状；它与 DECLARED 同批各 +1 ⇒ 缺口仍是 `{A127, A156, A45}`，`test_gap_equals_partial_roster` 未红在标记上）。这一格现在仍与 l253 那把尺子的「整行带闭合标记的 Backlog 行数」同面同数（112 = 111 + 1），两把独立尺子在同一只面上第四次对上 ⇒ 由 `test_the_other_ruler_counts_the_same_wide_face` 现场钉。新立 A339 那一行裸字母不带标记 ⇒ 本格未被它推动。**L256 重量**：110 → **111**（A324 那一格收成划掉字母加轮号，形状走 `(~~M~~ / 已收口 / 括号 L / 不粗体` 那一档，没有造出新形状；它与 DECLARED 同批各 +1 ⇒ 缺口仍是 `{A127, A156, A45}`，`test_gap_equals_partial_roster` 未红在标记上）。这一格现在仍与 l253 那把尺子的「整行带闭合标记的 Backlog 行数」同面同数（111 = 110 + 1），两把独立尺子在同一只面上第三次对上 ⇒ 由 `test_the_other_ruler_counts_the_same_wide_face` 现场钉。**L255 重量**：109 → **110**（A316 那一格收成划掉字母加轮号；它与 DECLARED 同批各 +1 ⇒ 缺口仍是 `{A127, A156, A45}`，`test_gap_equals_partial_roster` 未红在标记上）。这一格现在仍与 l253 那把尺子的「整行带闭合标记的 Backlog 行数」同面同数（110 = 109 + 1），两把独立尺子在同一只面上再次对上 ⇒ 由 `test_the_other_ruler_counts_the_same_wide_face` 现场钉。**L254 重量**：146 → **109**。过滤请掉 38 格（B226–B263 那 38 行日志块名册行**带着标记**——它们是某一轮收口清单里的表格，标记写的是那一轮的账，不是 Backlog 的账），本轮 A337 补上一格 ⇒ 146 − 38 + 1 = **109**。**L253 重量**：145 → **146**（A328 那一格收成 `~~M~~` 已收口（L253）；A337 是新账不带标记。卷一 A31 那一格本轮补的是**轮号**，而它早就带划掉字母 ⇒ 宽面计数一格未动，这一笔只由 A328 贡献）。红读数 `assert 146 == 145`。**L252 重量**：144 → **145**（A330 那一格收成 `~~M~~` 已收口（L252）；A336 是新账不带标记）。phantom 46、`PARTIAL` 与表行那一格本轮跟着新账走（见上），弱词档未动。红读数 `assert 145 == 144`。**L251 重量**：143 → **144**（A304 那一格收成 `~~S~~` 已收口（L251）；A280 那一行原地补的是现量，定级格未动所以不算标记）。红读数 `assert 144 == 143`。**L250 重量**：141 → 143（A298 / A303 各收成 `~~M~~` / `~~S~~` 已收口（L250））
#: 定级格这一面在**未过滤**那面（含日志块 46 行）多出来的格数：L261 收到定级格之后现量 **0**——
#: 日志块里的名册表格最后一格不是定级格（那一轮的账写在正文里），所以一格也贡献不进权威面。
#: 这一格由 `test_the_unfiltered_face_still_pairs_with_the_filtered_one` 钉住，它同时是
#: 「口径 9 的收紧没有把日志块偷偷收进权威面」的证据。
MEASURED_LOG_ROW_MARKS = 0
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

#: 允许「宣称收口而定级格不标」的 ID——必须是半程收口，且每格有各自的点名用例。
#: **L261 从三格走到四格**：口径 9 把标记判据收到定级格之后，B2 那一格第一次现形（正文写着
#: 「前半关闭（L80）」而定级格仍是裸 `M`），它此前一直被宽面喂饱。
PARTIAL = frozenset({"A127", "A156", "A45", "B2"})
#: L242 当轮的半程名册——只服务 `test_round_arithmetic_closes` 那一式，不让它随后续轮的名册漂。
PARTIAL_AT_L242 = frozenset({"A127", "A156", "A45"})
#: 本轮补标的十七格——既是判据对象，也是注入臂的取样面。
PATCHED = frozenset({
    "A122", "A138", "A147", "A155", "A172", "A182", "A265", "A266", "B265",
    "A48", "A51", "A53", "A64", "A67", "A69", "A72", "A95",
})
#: L261（A338 修法①）补标的 41 行——逐行核对「正文那句闭合语说的确实是本行整行」之后补上定级格。
#: 其中 35 行是 A338 原格点名的那一批，6 行（`A201` 至 `A206`）是 A337 当年误归入「引用别物」档、
#: 本轮按正文现量订正的；留裸字母的三行是 A21 / B2 / B266，见 `TWO_FACE_ROSTER`。
PATCHED_L261 = frozenset({
    "A49", "A55", "A130",
    "A201", "A202", "A203", "A204", "A205", "A206", "A207", "A208",
    "A270", "A275", "A277", "A302", "A312",
    "B201", "B202", "B203", "B204", "B205", "B206", "B207", "B208", "B209", "B210",
    "B211", "B212", "B213", "B214", "B215", "B216", "B217", "B218", "B219", "B220",
    "B221", "B222", "B223", "B224", "B225",
})


def read_volume(name):
    """按字节读一份账本并归一成 LF 行表，不经通用换行翻译"""
    data = (ROOT / name).read_bytes().decode("utf-8")
    return data.replace("\r\n", "\n").split("\n")


def load_volumes():
    return {name: read_volume(name) for name in VOLUMES}


def grade_cell(line):
    """定级格 = 表格行的最后一格（口径 9）。正文里的竖线只让格数变多，不改末格位置——
    这一条与 `test_backlog_mark_vocab_l253.grade_cell` 同式，两把尺子读同一格由
    `test_the_other_ruler_counts_the_same_wide_face` 现场对账。"""
    return line.split("|")[-2].strip()


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
    """口径 9：只有**定级格**里的标记才算这行收了（正文那句闭合语不算）"""
    return {
        ident
        for ident, entries in rows.items()
        if any(MARK.search(grade_cell(line)) for _, _, line in entries)
    }


def marked_wide(rows):
    """对照面（L261 之前判据读的那一面）：整行任一格里有闭合标记就算"""
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

    def test_the_excel_ingest_half_row_is_qualified(self):
        """B2（L261 新增的那一格）：声明带「前半关闭」，定级格仍是裸字母，剩下的是 A132 那一半"""
        volumes = load_volumes()
        sites = declared(volumes, table_rows(volumes))["B2"]
        assert any("前半" in ctx for _, _, ctx in sites), [c for _, _, c in sites]
        row = table_rows(volumes)["B2"][0][2]
        assert "A132" in row, "B2 行内不再点名剩下那一半的去向，本格的半程证据失效"
        assert "只剩" in row or "剩余" in row, "B2 行内不再有「还剩一半」的标注：要么全收（补定级格并从名册删掉），要么口径变了"
        assert not MARK.search(grade_cell(row)), "定级格被标上了——那这一格就不是半程，该从 PARTIAL 删掉"
        assert MARK.search(row), "B2 的正文不再带闭合语：宽面这一格失效，本格的存在理由要重述"

    def test_patched_rows_are_now_marked(self):
        """L242 当轮补标的十七格逐格验：不认「大概都标了」"""
        volumes = load_volumes()
        rows = table_rows(volumes)
        unmarked = {k for k in PATCHED if k not in marked(rows)}
        assert not unmarked, f"这些格仍然没标：{sorted(unmarked)}"

    def test_patched_rows_from_l261_are_marked_in_the_grade_cell(self):
        """A338 修法① 的 41 行逐格验「定级格里真有标记」——正文那句闭合语不算数"""
        volumes = load_volumes()
        rows = table_rows(volumes)
        loose = {k for k in PATCHED_L261 if k not in marked(rows)}
        assert not loose, f"这些格补标没落到定级格：{sorted(loose)}"

    def test_the_two_faces_differ_by_the_remaining_three(self):
        """补标之后两面之差只剩三格，且每一格都是「正文那句不管整行」的形状"""
        volumes = load_volumes()
        rows = table_rows(volumes)
        diff = marked_wide(rows) - marked(rows)
        assert diff == {"A21", "B2", "B266"}, sorted(diff)
        # 三格里只有 B2 进过声明面，而它正是靠「前半」这一限定词留在 PARTIAL 名册的那一格
        assert diff & set(declared(volumes, rows)) == {"B2"}
        assert {"B2"} <= PARTIAL

    def test_round_arithmetic_closes(self):
        """补标十七格 + 半程三格 = 开工前实测的二十笔欠账（数字必须自己闭合）"""
        assert len(PATCHED) + len(PARTIAL_AT_L242) == MEASURED_GAP_BEFORE

    def test_the_l261_patch_closes_the_measured_forty_one(self):
        """L261 自己那一式：补标 41 + 留裸字母 3 = 补标前量到的两格差 44"""
        assert len(PATCHED_L261) + 3 == 44


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

    def test_the_filter_is_load_bearing_on_the_narrow_face(self):
        """口径 8 与口径 9 的关系（L261 现形）：判据收到定级格之后，不开区段过滤那一面会多出
        **全部 38 只日志名册 ID** 的欠账——它们的正文带着那一轮的闭合语而最后一格不是定级格。
        过滤不是「读数整洁」，是判据不自我循环的前提"""
        volumes = load_volumes()
        on = table_rows(volumes)
        off = table_rows(volumes, backlog_only=False)
        assert set(declared(volumes, on)) - marked(on) == PARTIAL
        off_gap = set(declared(volumes, off)) - marked(off)
        roster = {f"B{n}" for n in range(226, 264)}
        assert off_gap == PARTIAL | roster, sorted(off_gap ^ (PARTIAL | roster))
        assert len(off_gap) == len(PARTIAL) + 38

    def test_the_unfiltered_face_still_pairs_with_the_filtered_one(self):
        """反空转的另一半：留在册的声明面 95，不过滤那一面 95 + 38 = 133 也照样有数；
        而定级格那一面在两侧同数（日志块 46 行的末格不是定级格 ⇒ 贡献 `MEASURED_LOG_ROW_MARKS` 格）"""
        volumes = load_volumes()
        off = table_rows(volumes, backlog_only=False)
        assert len(declared(volumes, off)) == (
            MEASURED_DECLARED + MEASURED_PHANTOM_LOG_ROSTER
        )
        assert len(marked(off)) == MEASURED_MARKED + MEASURED_LOG_ROW_MARKS

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
        """两把独立尺子在同一只面上对上数（L261 起这一格是**定级格**那一面）：l253 的
        「定级格带闭合标记的 Backlog 行」= 本守卫过滤后的有标记名册，而宽面比它多的正是
        `TWO_FACE_ROSTER` 那三格（A338 补标之后剩下的「正文管一半或管别物」档）"""
        wide, narrow = other.faces(other.load_volumes())
        mine = marked(table_rows(load_volumes()))
        assert mine == narrow, sorted(mine ^ narrow)
        assert len(mine) == MEASURED_MARKED
        assert wide - narrow == other.TWO_FACE_ROSTER, sorted(wide - narrow)
        assert len(wide) == len(mine) + len(other.TWO_FACE_ROSTER)

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
        row = table_rows(volumes)["A136"][0][2]  # **L263 换样本**：原样本 `A274` 那句「L220 已收口」的判决本轮落进了定级格（`~~S~~ 已收口（L220）`），不再是「正文引用标记词而定级格不动」的 live 样本；接棒的 `A136` 是「**L81 关闭（假臂那一侧）**」——动词紧贴全角括号，但括号头是「假」而不是 `L`，正是 MARK 必须**不**认的那一形
        assert "关闭（假臂那一侧）" in row, "A136 那行不再引用标记词汇本身，本用例的样本失效，换一个真实形状（L263 之前用的是 A274 的「已收口」、L253 之前是 A328、L250 之前是 A298 的「全清」）"
        assert not MARK.search(row), "行内引用标记词汇被当成关闭标记了——待拍的账会永久隐身（半程的那一格「关闭（假臂那一侧）」从此不算收了）"

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
