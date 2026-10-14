# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""两卷 Backlog 表「定级格」的标记词汇封闭清单与轮号反向可追溯性（A328 / L253 收口轮）

**这一轮收的是哪一格**：A328 立账时记的是两件事——① 关闭标记的词汇形状没有封闭清单，
② 表行有标记 ⇒ 标记里的轮号能在进度声明面查到（反方向）这一维没钉。本轮把这两维都落成判据，
并把 L242 那把守卫只认的方向（声明面 → 表行有标记）与本轮的方向（表行有标记 → 轮号可查）
接成一条闭环。

**判据面为什么选「定级格」而不是整行**：`test_backlog_closure_l242` 的标记判据原本扫的是**整行**，
而 Backlog 那一格的正文里满是「A95 已关闭（L185）」「**ngram 已修（L24）**」这一类引用**别的东西**
的闭合语。两面之差在 L253 立账时是 **44 行**（`TWO_FACE_ROSTER` 逐格点名），A337 / A338 把它写成了
「36 只正文说了收了而定级格还停在裸字母 + 8 只连声明面都没进过（`A21` / `A201`–`A206` / `B266`）」。
**L261 收 A338 的 ① 时逐行读了那八行的正文，归因错了**：`A201`–`A206` 六行的第一句就是**本行整行**
的判决（`A201`「16 条缺数 + 3 条隐藏偏支弧全清」、`A206`「10 页各建组件测试，112 例全绿」），不是
引用别物也不是半程 ⇒ 本轮连同原点名的 35 行一起补标，实做 **41 行补标 / 3 行留裸字母**。留下的三行
各有专格：`A21` 那句只管 ngram 那一半（本行自己写着 contains / regex 两格待办）、`B2` 写着前半而
剩下的在 A132、`B266` 写着部分关闭且其余仍在案。`TWO_FACE_ROSTER` 因此从 44 格缩到 **3 格**，
同一批改写把 l242 的 `marked()` 也从整行收到定级格（l242 口径 9）——两面之差从此只剩这三格，
而 `test_arm_a_prose_closure_phrase_moves_the_wide_face_only` 继续证明「正文写一句闭合语而定级格不动」
这一族仍被宽面数进去，两面之差不是恒零。

**封闭清单怎么立**：把定级格拆成三件结构件——字母形（`S` / `M` / `L` / `XS` 裸字母 vs
`~~S~~` / `~~M~~` / `~~L~~` 划掉）、闭合词（括号紧跟轮号的那 2–3 个字）、括号头（`（L` 开头
还是 `（第二本账 L` 开头）——再拼上「有没有粗体包裹」。词汇表由现量导出并逐词点名，
组合名册（`MEASURED_COMBOS`）精确相等 ⇒ 换新写法（例如「已了结（Lxxx）」）当场红，
而不是静默地不算标记。`test_arm_a_new_verb_shape_fires_the_roster` 用改写现量的一格证明
这把尺子不是恒真。

**轮号反向可追溯**：每一格的闭合语必须从它的括号里读出**至少一个**轮号（`（` 与 `L` 之间
允许限定词，例如「第二本账」「L184，B254」），且该轮号必须能在**某一卷**的进度声明面
（`- [x] **LNNN**` 清单行或 `### LNNN` 日志标题）查到。跨卷那一族单独钉：现量 **13 行**
全部写在卷一、全部指向卷二、全部在括号头自我声明「第二本账」，两个方向同一把尺子——
① 跨卷而不写明（把 A46 那一格的「第二本账」摘掉）红，② 写明跨卷而轮号其实本卷可查也红。
第 13 格是 L261 补标 `A130` 时加进来的：它的判决轮号 L183 只躺在卷二声明面，照直写
`已关闭（L183）` 就会撞上「跨卷而不写明」那一支 ⇒ 按既有同族写法带上限定词。
本轮开工前该维的现量是一条真缺陷：卷一 A31 那一格写作划掉字母而**没有任何轮号**
（`~~S~~`  alone），本轮按声明面现量补成 `~~S~~ 已收口（L29）`，此后「有标记而无轮号」
的名册为空集。

跑法：`python -m pytest tests/unit/test_backlog_mark_vocab_l253.py -q --no-cov`
"""
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[2]

#: 与 `test_backlog_closure_l242` 同一对面：两卷账本，按字节读再归一成 LF 行表。
VOLUMES = ("OPTIMIZATION_LOOP.md", "OPTIMIZATION_LOOP_2.md")

ID = r"(?:[AB]\d{1,3})(?!\d)"
ROW = re.compile(rf"^\| ({ID}) \|")
#: 区段边界：`## 循环日志` 之后的 `| Axxx |` 是日志块里的表格，不是 Backlog 行（现量 46 行，
#: 含注入臂名册 A1–A8 与 B226–B263）。**L261 起这一过滤从「省事」升成「承重」**：l242 的判据改读
#: 定级格之后，未过滤那面（含日志块）的欠账名册会多出全部 38 只 B226–B263——那些行是某一轮收口
#: 清单里的表格，它们的「定级格」不是 Backlog 的定级格。由 l242 的
#: `test_the_filter_is_load_bearing_on_the_narrow_face` 点名（见 A337 / A338）。
LOG_SECTION = "## 循环日志"

#: 与 l242 同一支标记判据，本轮只把它用在**定级格**上。
MARK = re.compile(r"(?:已关闭|已收口|关闭|收口|结案|已修)（L|~~[ \t]*[A-Z]{1,2}[ \t]*~~")
#: 划掉的定级字母（划掉 = 这行的处置结论已定）。
STRIKE = re.compile(r"~~[ \t]*([A-Z]{1,2})[ \t]*~~")
#: 裸定级字母（未划掉 = 仍待拍）。行首要连续字母，`XS` 是现量里的最小档。
BARE = re.compile(r"^([A-Z]{1,2})(?![A-Za-z])")
#: 候选闭合词：紧贴着「（ + 轮号（本卷或他卷）」的 2–3 个汉字，且前面不是汉字/字母/数字。
CLOSURE_WORD = re.compile(r"(?<![一-龥A-Za-z0-9])([一-龥]{2,3})(?=（(?:L|第[一二]本账))")
#: MARK 认账的那几个动词，用来把候选词分成「算收口」与「不算收口」（`半修` 属后者）。
COUNTED = ("已关闭", "已收口", "关闭", "收口", "结案", "已修")
#: 括号里的轮号：`（` 与 `L` 之间允许最多 40 个非括号字符（限定词、逗号、B 号）。
ROUNDS = re.compile(r"（[^（）]{0,40}?L(\d+)")
CROSS_HEAD = "（第二本账"
CHECK_LINE = re.compile(r"^- \[x\] \*\*L(\d+)")
LOG_HEAD = re.compile(r"^### L(\d+)")

#: ---- 现量（L261 收 A338 之后）----
#: **L261 动 42 格，表行三面一格未动**：A338 的 ① 逐行核对之后补标 41 行定级格（卷一 3 行
#: `A49` / `A55` / `A130`，卷二 38 行 `A201`–`A208`、`A270` / `A275` / `A277` / `A302` / `A312`、
#: `B201`–`B225`），另加 A338 自己那一格收成 `~~S~~ 已收口（L261）` ⇒ 权威面 68 → **110**、
#: 宽面 112 → **113**（只 +1：那 41 行此前就被整行的正文闭合语数进去了，这正是两面之差 44 的来源；
#: 唯一新进宽面的是 A338 那一行）、两面之差 44 → **3**（`A21` / `B2` / `B266`，`TWO_FACE_ROSTER` 逐格
#: 缩）、同卷可查 56 → **97**、跨卷 12 → **13**（`A130` 的轮号 L183 在卷二，按同族写法带「第二本账」）。
#: 封闭清单长出新形状：`A204` 的 XS 档此前从未被划掉过、`A206` 的 L 档此前只与「已收口」搭配 ⇒
#: 组合名册 16 → **18** 格，是新形状落进清单并被逐格点名，清单本身一格未松（精确相等当场证明）。
#: 本轮零新 Backlog 行、零裁撤 ⇒ 262 / 216 / 46 未动。红读数逐格见 l242 的口径 9 与本文件各常量的
#: L261 注。**L257 那轮的旧记录**：账本批之前该守卫跑出 **9 红 / 11 绿**（同一支只报第一处，所以
#: Backlog 区那一格的读数由「表行先红的那一支在同批从 210 推到 211」这一事实给出，而不是由一次
#: 独立可见的 assert 给出——这一点如实记下，不假称每格都单独被逼出过）。
#: **L257 动六格，另加表行三面里的两格**：A285 那一格从裸 `S` 收成划掉字母加轮号 ⇒ 权威面 67 → 68、
#: 宽面 111 → 112（与 l242 过滤后的「有标记」同面同数，两把尺子第四次对上）、同卷可查 55 → 56、
#: 字母形裸 S 92 净 0（A285 离格、新立 A339 入格）与划掉 ~~S~~ 34 → 35、闭合词已收口 35 → 36、
#: 组合名册 `(~~S~~ / 已收口 / 括号 L / 不粗体` 那一档 9 → 10（**落进已有形状，没有造出第 17 种**，
#: 由组合名册的精确相等当场证明）。新立 A339 一行（裸字母待拍、两面无标记）⇒ 表行 258 → 259、
#: Backlog 区 212 → 213（与 l242 的 `MEASURED_ROWS` 同面同数），日志块 46 未动。红读数 `assert 68 == 67`。
#: **L256 只动六格，表行三面一格未动**：A324 那一格从裸 `M` 收成划掉字母加轮号 ⇒ 权威面 66 → 67、
#: 宽面 110 → 111（与 l242 过滤后的「有标记」同面同数，两把尺子第三次对上）、同卷可查 54 → 55、
#: 字母形裸 M 56 → 55 与划掉 ~~M~~ 25 → 26、闭合词已收口 34 → 35、组合名册 `(~~M~~ / 已收口 / 括号 L /
#: 不粗体` 那一档 7 → 8（**落进已有形状，没有造出第 17 种**，由组合名册的精确相等当场证明）。
#: 本轮零新 Backlog 行、零裁撤 ⇒ 258 / 212 / 46 未动。红读数 `assert 67 == 66`。
#: **L255 只动六格，表行三面一格未动**：A316 那一格从裸 `S` 收成划掉字母加轮号 ⇒ 权威面 65 → 66、
#: 宽面 109 → 110、同卷可查 53 → 54、字母形裸 S 93 → 92 与划掉 ~~S~~ 33 → 34、闭合词已收口 33 → 34、
#: 组合名册 `(~~S~~ / 已收口 / 括号 L / 不粗体` 那一档 8 → 9（走的是同一族已有形状）。
#: **L254 只动三格**：A337 收成划掉字母加轮号（权威面、宽面、同卷可查各 +1）、新立 A338 一行
#: （裸字母待拍、两面无标记 ⇒ 只有表行面 +1）。字母形那一档 S 净 0（A337 离格、A338 入格）。
MEASURED_ALL_ROWS = 266  # **L268 重量**：265 → **266**（A346 一行入面）。红读数 `assert 266 == 265`｜原 **L267 重量**：264 → **265**（A345 一行入面）。红读数 `assert 265 == 264`｜原 **L265 记账批二轮重量**：263 → **264**（A344 一行入面，红读数 `assert 264 == 263`）｜原 **L262 重量**：262 → **263**（新立 A343 一行，待拍、定级格裸 M、整行不带闭合词）。红读数 `assert 263 == 262`。**L259 重量**：260 → **262**（新立 A341、A342 两行，待拍、不带标记）。红读数 `assert 262 == 260`。**L258 重量**：259 → **260**（新立 A340 一行：并行支路咽下判决那一格，独立待拍、不带标记）。红读数 `assert 260 == 259`。**L257 重量**：258 → **259**（新立 A339 一行：A285 收口时量到的 `fields=[]` 在两个类里语义相反，独立待拍、不带标记）。红读数 `assert 259 == 258`。**L254 重量**：257 → **258**（新立 A338 一行：A337 的 ① 那一半——44 行两格差要逐行核对才能补标——拆出来独立待拍、不带标记）。**L253 重量**：256 → **257**（新立 A337 一行，待拍、不带标记）。红读数 `assert 257 == 256`
MEASURED_BACKLOG_ROWS = 220  # **L268 重量**：219 → **220**（同一行落在 Backlog 区，与 l242 的 `MEASURED_ROWS` 同面同数；日志块那一面一格未动）。红读数 `assert 220 == 219`｜原 **L267 重量**：218 → **219**（同一行落在 Backlog 区，与 l242 的 `MEASURED_ROWS` 同面同数；日志块那一面一格未动）。红读数 `assert 219 == 218`｜原 **L265 记账批二轮重量**：217 → **218**（A344 一行，红读数同 `assert 218 == 217`）｜原 **L262 重量**：216 → **217**（那一行落在 Backlog 区，与 l242 的 `MEASURED_ROWS` 同面同数；日志块那一面一格未动）。红读数与 l242 那格独立闭合。**L259 重量**：214 → **216**（那两行落在 Backlog 区，与 l242 的 `MEASURED_ROWS` 同面同数；日志块那一面一格未动）。红读数由 `test_the_two_faces_are_measured_as_they_are` 在 ALL_ROWS 之后那一步逼出。**L258 重量**：213 → **214**（同一行落在 Backlog 区，与 l242 的 `MEASURED_ROWS` 同面同数；日志块那一面一格未动）。**L257 重量**：212 → **213**（同一行落在 Backlog 区，与 l242 的 `MEASURED_ROWS` 同面同数；日志块那一面一格未动）。**L254 重量**：211 → **212**（同一行落在 Backlog 区；日志块那一面一格未动，A338 写在 Backlog 区）。**L253 重量**：210 → **211**（同一行落在 Backlog 区）
MEASURED_LOG_ROWS = 46  # 日志块里的 `| Axxx |` / `| Bxxx |` 行数（A1–A8 与 B226–B263），L254 给 l242 加的区段过滤量到的正是这 46 行 ⇒ 本守卫自己按 `## 循环日志` 分区的读法与那一改动作数一致（同卷同数由 l242 的 `test_the_filtered_out_rows_are_the_named_forty_six` 点名）
MEASURED_CLOSURE_CELLS = 113  # **L268 重量**：112 → **113**（A333 那一格入权威面）。红读数 `assert 113 == 112`。**L263 重量**：110 → **112**（`A274` / `A71` 两格把正文早就写下的整行判决落进定级格，两格都走清单里已有的形状、没有造出第 19 种）。红读数 `assert 112 == 110`。**L261 重量**：68 → **110**（A338 的 ① 补标 41 行 + A338 自己那一格收成 `~~S~~ 已收口（L261）`）。**L257 重量**：67 → **68**（A285 那一格从裸 S 收成划掉字母加轮号）。**L256 重量**：66 → **67**（A324 那一格从裸 M 收成划掉字母加轮号）。**L255 重量**：65 → **66**（A316 那一格从裸 S 收成划掉字母加轮号）。**L254 重量**：64 → **65**（A337 那一格收成划掉字母加轮号）。**L253 重量**：63 → **64**（A328 那一格收成划掉字母加轮号）。红读数 `assert 64 == 63`
MEASURED_WIDE_ROWS = 116  # **L268 重量**：115 → **116**（A333 那一格带上 `MARK` 读得到的闭合语 ⇒ 宽面与权威面同步各 +1，两面之差仍是 **3**、`TWO_FACE_ROSTER` 一字未动 ⇒ 两把尺子第六次对上）。红读数 `assert 116 == 115`。**L263 重量**：113 → **115**（那两格的定级格落进标记之后，整行也就带上了 MARK 读得到的闭合语 ⇒ 宽面与权威面同步各 +2；两面之差仍是 **3**、`TWO_FACE_ROSTER` 一字未动 ⇒ 两把尺子第五次对上）。红读数 `assert 115 == 113`。**L261 重量**：112 → **113**（只 +1：那 41 行早就被整行的正文闭合语数进宽面，本轮补的是定级格；宽面唯一新进的是 A338 那一行）。宽面与权威面之差 44 → **3**，由 `TWO_FACE_ROSTER` 逐格点名。**L257 重量**：111 → **112**（A285 那一格此前整行也没有闭合标记）——两把尺子第四次对上。**L256 重量**：110 → **111**（A324）——第三次对上。**L255 重量**：109 → **110**（A316），同面同数第二次。**L254 重量**：108 → **109**。红读数 `assert 108 == 107`
MEASURED_SAME_VOLUME = 100  # **L268 重量**：99 → **100**（A333 那一格读的轮号 L268 在卷二自己的声明面查得到 ⇒ 走本卷档；跨卷那 13 格一格未动）。红读数 `assert 100 == 99`。**L263 重量**：97 → **99**（`A274` 那一格读 L220、卷二自己的声明面查得到；`A71` 那一格读 L46、卷一自己的声明面查得到 ⇒ 两格都走本卷档，跨卷那 13 格一格未动）。红读数 `assert 99 == 97`。**L261 重量**：56 → **97**（补标那 41 行读出的轮号全部在本卷声明面查得到，另加 A338 自己那一格读 L261；`A130` 走跨卷那一档，不在本格）。**L257 重量**：55 → **56**（A285 那一格读出的 L257 在卷二自己的声明面查得到）。**L256 重量**：54 → **55**（A324）。**L255 重量**：53 → **54**（A316）。**L254 重量**：52 → **53**（A337）。**L253 重量**：50 → **52**（A31 补上 L29 那一轮、A328 补上 L253 那一轮）。红读数 `assert 52 == 50`
MEASURED_CROSS_VOLUME = 13  # 与 `CROSS_ROSTER` 同数；A328 立账时写的是 3 行，L253 按声明面逐卷核对成 12，**L261 补标卷一 `A130`（轮号 L183 在卷二）走到 13**

#: 定级格字母形封闭清单（划掉四种 + 裸三种），值为行数。
#: **L261 之后 XS 档只以划掉形出现**（裸 `XS` 那一格是 `A204`，本轮被划掉），所以字典里没有 `XS` 这一键。
MEASURED_LETTERS = {
    "S": 68,  # **L263 重量**：70 → **68**（`A274` / `A71` 两格离这一格）。**L261 重量**：94 → **70**（补标那 41 行里 24 行的定级格是裸 S，全部离这一格）
    "M": 43,  # **L268 记这一格未动**：A346 带进一格裸 M、A333 交出一格，两数恰好同抵 ⇒ "只收一格又只立一格"的证据就在这格不动上（红读数里没有它，`Omitting 6 identical items` 点名它同）。**L267 重量**：42 → **43**（A345 那一格留裸 M 待拍，与 A343 / A344 同形；红读数 `{'M': 43, ...} != {'M': 42, ...}`）。**L265 记账批二轮重量**：41 → **42**（A344 那一格留裸 M 待拍，与 A343 同形；红读数 `{'M': 42, ...} != {'M': 41, ...}`）。**L262 重量**：40 → **41**（A343 那一格按「三修法对外语义互不相同」留裸 M 待拍，与 L259 的 A341 同形）。红读数 `assert 41 == 40`。**L261 重量**：56 → **40**（同上，16 行走 M 档）
    "L": 1,  # **L261 重量**：2 → **1**（`A206` 的 L 档被划掉）
    "~~S~~": 61,  # **L263 重量**：59 → **61**（上面那两格入这一格）。**L261 重量**：35 → **59**（+24：卷一 `A49` / `A130`、卷二 `A201`–`A203`、`A275` / `A277`、`B201`–`B205`、`B207`–`B210`、`B212`–`B214`、`B217`、`B219`、`B222`、`B225` 与 A338 自己）
    "~~M~~": 43,  # **L268 重量**：42 → **43**（A333 那一格入这一格）。红读数 `{'~~M~~': 43, ...} != {'~~M~~': 42, ...}`。**L261 重量**：26 → **42**（+16：卷一 `A55`、`A205` / `A207` / `A208` / `A270` / `A302` / `A312`、`B206` / `B211` / `B215` / `B216` / `B218` / `B220` / `B221` / `B223` / `B224`）
    "~~L~~": 3,  # **L261 重量**：2 → **3**（`A206` 收成 `~~L~~ 已关闭（L104–L113）`）
    "~~XS~~": 1,  # **L261 新档**：0 → **1**（`A204` 收成 `~~XS~~ 已关闭（L103）`，XS 档此前从未被划掉过）
}
#: 定级格闭合词封闭清单，值为行数。`半修` 在场而 MARK 不认（半程不算收口），由
#: `test_the_uncounted_word_is_named` 逐格点名。
MEASURED_WORDS = {"已收口": 39, "已关闭": 53, "已修": 20, "关闭": 1, "半修": 1}  # **L268 重量**：已收口 38 → **39**（A333 定级格的用词；本轮清单行与日志要点刻意只在名册形状字面里用闭合词，不进这一面，故恰好 +1）。红读数 `assert 39 == 38`。原 **L263 重量**：已收口 37 → **38**（`A274` 照抄正文那句「L220 已收口」的用词）、已关闭 52 → **53**（`A71` 正文写的是「L46 关闭（第三种修法…）」，定级格按清单里已有的「已关闭」形收，不新造第 6 个词）。红读数 `assert 53 == 52`。**L261 重量**：已关闭 13 → **52**（补标那 41 行照抄正文判决的用词，39 行是「已关闭」，`A49` / `A55` 是「已修」）、已修 18 → **20**、已收口 36 → **37**（A338 自己那一格），`关闭` / `半修` 两档未动。**L257 重量**：已收口 35 → **36**（A285 用这个词收口）。**L256 重量**：34 → **35**（A324）。**L255 重量**：33 → **34**（A316）
#: (字母形, 闭合词, 括号头, 是否粗体) 的组合名册，值为行数——闭合清单的实体。
#: **L261：16 格 → 18 格**，新增的两格是 `~~XS~~ / 已关闭 / 括号 L / 不粗体`（`A204`）与
#: `~~L~~ / 已关闭 / 括号 L / 不粗体`（`A206`）——L 档此前只与「已收口」搭配过。
#: 这是新形状**落进清单并被逐格点名**，不是清单被改松：`test_the_combo_roster_is_exact` 是精确相等，
#: 少一格或多一格都当场红。
MEASURED_COMBOS = {
    ("~~S~~", "已关闭", "L", False): 25,  # **L263 重量**：24 → 25（`A71` 那一格走这一形）。L261 重量：3 → 24（补标那一批 B 档与 A201–A203 等走这一形）
    ("~~M~~", "已关闭", "L", False): 19,  # L261 重量：4 → 19
    ("~~S~~", "已修", "L", False): 12,  # L261 重量：11 → 12（`A49` 走这一形）
    ("~~S~~", "已收口", "L", False): 12,  # **L263 重量**：11 → 12（`A274` 那一格走这一形）。L261 重量：10 → 11（A338 自己那一格走这一形）
    ("~~M~~", "已收口", "L", False): 9,  # **L268 重量**：8 → **9**（A333 那一格落进已有组合形状，红读数 `assert 9 == 8`）
    ("~~S~~", "已收口", "L", True): 5,
    ("~~M~~", "已收口", "L", True): 5,
    ("~~M~~", "已修", "L", False): 4,  # L261 重量：3 → 4（`A55` 走这一形）
    ("~~S~~", "已关闭", "第二本账", False): 4,  # L261 重量：3 → 4（`A130` 走这一形，跨卷那 13 格里的一格）
    ("~~M~~", "已关闭", "第二本账", False): 3,
    ("~~M~~", "已收口", "第二本账", False): 3,
    ("~~S~~", "已收口", "第二本账", False): 3,
    ("S", "已修", "L", False): 3,
    ("~~L~~", "已收口", "L", False): 2,
    ("~~L~~", "已关闭", "L", False): 1,  # L261 新档：`A206`
    ("~~XS~~", "已关闭", "L", False): 1,  # L261 新档：`A204`
    ("M", "关闭", "L", False): 1,
    ("M", "已修", "L", False): 1,
}
#: 跨卷点名：(所在卷, ID, 括号里的轮号)。全部在卷一、全部指向卷二、全部写明「第二本账」。
CROSS_ROSTER = (
    ("OPTIMIZATION_LOOP.md", "A46", (156,)),
    ("OPTIMIZATION_LOOP.md", "A95", (185,)),
    ("OPTIMIZATION_LOOP.md", "A122", (185,)),
    ("OPTIMIZATION_LOOP.md", "A124", (153,)),
    ("OPTIMIZATION_LOOP.md", "A125", (151,)),
    ("OPTIMIZATION_LOOP.md", "A130", (183,)),  # L261 补标 `~~S~~ 已关闭（第二本账 L183）`
    ("OPTIMIZATION_LOOP.md", "A138", (184,)),
    ("OPTIMIZATION_LOOP.md", "A139", (157,)),
    ("OPTIMIZATION_LOOP.md", "A140", (154,)),
    ("OPTIMIZATION_LOOP.md", "A147", (182,)),
    ("OPTIMIZATION_LOOP.md", "A155", (179,)),
    ("OPTIMIZATION_LOOP.md", "A172", (181,)),
    ("OPTIMIZATION_LOOP.md", "A182", (170,)),
)
#: 宽面算标记而定级格不算的三格（A337 / A338 的案面收完之后的余集）。逐格点名 ⇒ 谁补了标这一格要
#: 跟着缩，而每一格都必须是「正文那句闭合语并不覆盖本行整行」，由 l242 的专格用例逐格验：
#: `A21`（那句只管 ngram 那一半，本行自己写着 contains / regex 两格待办）、
#: `B2`（写着「前半关闭」，剩下的另一半在 A132）、
#: `B266`（写着「部分关闭（L201 收 StreamWriter 那一处；其余仍在案）」）。
#: **L261 重量**：44 → **3**（A338 的 ① 逐行核对补标 41 行；`A201`–`A206` 六行本轮订正——
#: A337 / A338 原把它们归入「引用别物或只覆盖一半」，实测那六句第一格就是本行整行的判决）。
TWO_FACE_ROSTER = frozenset({"A21", "B2", "B266"})


def load_volumes():
    """`{卷名: [行]}`，按字节读后归一 LF（两卷都是纯 LF，历史游离 CR 已在 L242 摘掉）"""
    return {
        name: (ROOT / name).read_bytes().decode("utf-8").replace("\r\n", "\n").split("\n")
        for name in VOLUMES
    }


def log_boundary(lines):
    """`## 循环日志` 的行号（0 起）；它之前的 `| Axxx |` 才算 Backlog 行"""
    return next(i for i, ln in enumerate(lines) if ln.startswith(LOG_SECTION))


def table_rows(volumes):
    """全区表行 `[{volume, line, id, text, backlog}]`，行首才是表行，散文里点名不算"""
    out = []
    for name, lines in volumes.items():
        bound = log_boundary(lines)
        for i, line in enumerate(lines, 1):
            hit = ROW.match(line)
            if hit:
                out.append({"volume": name, "line": i, "id": hit.group(1),
                            "text": line, "backlog": i - 1 < bound})
    return out


def backlog_rows(volumes):
    return [r for r in table_rows(volumes) if r["backlog"]]


def grade_cell(text):
    """定级格 = 表格行的最后一格（正文里出现的竖线只让格数变多，不改末格的位置）"""
    return text.split("|")[-2].strip()


def declared_rounds(volumes):
    """`{卷: {本轮号}}`：清单行与日志标题声明过的轮号"""
    out = {}
    for name, lines in volumes.items():
        found = set()
        for line in lines:
            hit = CHECK_LINE.match(line) or LOG_HEAD.match(line)
            if hit:
                found.add(int(hit.group(1)))
        out[name] = found
    return out


def letter_form(cell):
    hit = STRIKE.search(cell)
    if hit:
        return f"~~{hit.group(1)}~~"
    hit = BARE.match(cell)
    return hit.group(1) if hit else "无字母"


def closure_words(cell):
    return set(CLOSURE_WORD.findall(cell))


def bracket_head(cell):
    if CROSS_HEAD in cell:
        return "第二本账"
    return "L" if "（L" in cell else "无括号"


def combos(volumes):
    """`{(字母形, 闭合词, 括号头, 粗体): [ID]}` —— 只收定级格里真的有闭合标记的那些行"""
    out = {}
    for row in backlog_rows(volumes):
        cell = grade_cell(row["text"])
        if not MARK.search(cell):
            continue
        words = sorted(w for w in closure_words(cell) if w in COUNTED)
        key = (letter_form(cell), words[0] if words else "无动词",
               bracket_head(cell), "**" in cell)
        out.setdefault(key, []).append(row["id"])
    return out


def rounds_of(cell):
    return tuple(int(x) for x in ROUNDS.findall(cell))


def traceability(volumes):
    """把每一格的轮号按声明面分档：本卷可查 / 只在他卷可查 / 两卷都查不到 / 根本没有轮号"""
    faces = declared_rounds(volumes)
    no_round, unresolvable, cross, same = [], [], [], []
    for row in backlog_rows(volumes):
        cell = grade_cell(row["text"])
        if not MARK.search(cell):
            continue
        rounds = rounds_of(cell)
        if not rounds:
            no_round.append(row["id"])
            continue
        homes = {r: [v for v in VOLUMES if r in faces[v]] for r in rounds}
        if any(not h for h in homes.values()):
            unresolvable.append((row["id"], rounds))
        elif any(row["volume"] not in h for h in homes.values()):
            cross.append((row["volume"], row["id"], rounds, bracket_head(cell)))
        else:
            same.append(row["id"])
    return no_round, unresolvable, cross, same


def faces(volumes):
    """宽面（整行有标记）与权威面（定级格有标记）的 ID 名册"""
    wide, narrow = set(), set()
    for row in backlog_rows(volumes):
        if MARK.search(row["text"]):
            wide.add(row["id"])
        if MARK.search(grade_cell(row["text"])):
            narrow.add(row["id"])
    return wide, narrow


class TestTheGradeCellIsTheAuthoritativeFace:
    """区段与面的定义先钉住：哪一格算这行的处置结论，以及日志块里的同名表格不是 Backlog 行"""

    def test_the_two_faces_are_measured_as_they_are(self):
        volumes = load_volumes()
        rows = table_rows(volumes)
        assert len(rows) == MEASURED_ALL_ROWS
        assert len(backlog_rows(volumes)) == MEASURED_BACKLOG_ROWS
        assert len([r for r in rows if not r["backlog"]]) == MEASURED_LOG_ROWS

    def test_the_log_region_rows_are_named_and_are_not_backlog(self):
        """日志块里躺着 46 行 `| Axxx |` / `| Bxxx |`，其中 A1–A8 是注入臂名册不是欠账"""
        volumes = load_volumes()
        log_ids = sorted({r["id"] for r in table_rows(volumes) if not r["backlog"]})
        backlog_ids = {r["id"] for r in backlog_rows(volumes)}
        assert len(log_ids) == MEASURED_LOG_ROWS, log_ids
        assert {"A1", "A8", "B263"} <= set(log_ids)
        assert log_ids and not (set(log_ids) & backlog_ids), sorted(set(log_ids) & backlog_ids)

    def test_the_wide_face_over_counts_by_the_named_roster(self):
        """宽面把「正文有闭合语而定级格是裸字母」的行也算成已收——L261 补标之后余三格，逐格点名"""
        wide, narrow = faces(load_volumes())
        assert len(wide) == MEASURED_WIDE_ROWS
        assert len(narrow) == MEASURED_CLOSURE_CELLS
        assert narrow <= wide
        assert wide - narrow == TWO_FACE_ROSTER, sorted(wide - narrow)

    def test_the_grade_cell_is_always_the_last_column(self):
        """末格定位不靠格数：正文里有竖线的行（现量 8 行格数 ≠ 6，A344 那一行的正文里有竖线）末格仍是定级格"""
        volumes = load_volumes()
        odd = [r for r in backlog_rows(volumes) if len(r["text"].split("|")) != 6]
        assert len(odd) == 8, [r["id"] for r in odd]
        bad = [(r["id"], grade_cell(r["text"])) for r in odd
               if letter_form(grade_cell(r["text"])) == "无字母"]
        assert bad == [], bad


class TestTheMarkVocabularyIsAClosedList:
    """A328 的 ③：标记词汇要有封闭清单——字母形、闭合词、组合名册各钉一张"""

    def test_the_letter_forms_are_the_measured_set(self):
        cells = [grade_cell(r["text"]) for r in backlog_rows(load_volumes())]
        got = {}
        for cell in cells:
            got[letter_form(cell)] = got.get(letter_form(cell), 0) + 1
        assert got == MEASURED_LETTERS, got

    def test_the_closure_words_are_the_measured_set(self):
        volumes = load_volumes()
        got = {}
        for row in backlog_rows(volumes):
            for word in closure_words(grade_cell(row["text"])):
                got[word] = got.get(word, 0) + 1
        assert got == MEASURED_WORDS, got

    def test_the_uncounted_word_is_named(self):
        """`M 半修（L88）` 那一格进词汇表而不进收口计数：半程不算收，A156 因此在 PARTIAL 名册里"""
        volumes = load_volumes()
        half = [r["id"] for r in backlog_rows(volumes)
                if "半修" in closure_words(grade_cell(r["text"]))]
        assert half == ["A156"], half
        assert all("半修" not in w for w in COUNTED)

    def test_the_combo_roster_is_exact(self):
        got = {k: len(v) for k, v in combos(load_volumes()).items()}
        assert got == MEASURED_COMBOS, sorted(
            set(got.items()) ^ set(MEASURED_COMBOS.items()), key=str)

    def test_the_combo_roster_accounts_for_every_closure_cell(self):
        got = combos(load_volumes())
        assert sum(len(v) for v in got.values()) == MEASURED_CLOSURE_CELLS
        assert len({i for v in got.values() for i in v}) == MEASURED_CLOSURE_CELLS


class TestEveryClosureCellCitesAResolvableRound:
    """A328 的 ①②：反向可追溯——划掉也好、带动词也好，括号里必须有轮号，轮号必须查得到"""

    def test_no_closure_cell_lacks_a_round_number(self):
        """本轮开工前的现量是 1（卷一 A31 那一格只有划掉的字母），补标之后为空集"""
        no_round, *_ = traceability(load_volumes())
        assert no_round == [], no_round

    def test_every_cited_round_resolves_in_some_volume(self):
        _, unresolvable, _, _ = traceability(load_volumes())
        assert unresolvable == [], unresolvable

    def test_the_same_volume_claims_are_the_measured_count(self):
        *_, same = traceability(load_volumes())
        assert len(same) == MEASURED_SAME_VOLUME, sorted(same)

    def test_cross_volume_claims_are_the_measured_twelve_and_all_say_so(self):
        cross = traceability(load_volumes())[2]
        assert sorted((v, i, r) for v, i, r, _ in cross) == sorted(CROSS_ROSTER), cross
        assert all(h == "第二本账" for *_x, h in cross), [c for c in cross if c[3] != "第二本账"]

    def test_no_cell_says_the_other_volume_when_it_is_this_ones_own(self):
        """反方向：写了「第二本账」而那一轮本卷也查得到的格不存在（A138 那种格同时点两卷）"""
        volumes = load_volumes()
        faces_map = declared_rounds(volumes)
        liars = []
        for row in backlog_rows(volumes):
            cell = grade_cell(row["text"])
            if not (MARK.search(cell) and CROSS_HEAD in cell):
                continue
            if any(r in faces_map[row["volume"]] for r in rounds_of(cell)):
                liars.append(row["id"])
        assert liars == [], liars


class TestTheRulerBites:
    """注入臂全部在内存里改行，不碰磁盘：每臂点名喊红，证明上面两张名册不是恒真"""

    def _arm(self, volume, ident, new_cell):
        volumes = load_volumes()
        lines = volumes[volume]
        bound = log_boundary(lines)
        hit = [i for i, ln in enumerate(lines[:bound])
               if ROW.match(ln) and ROW.match(ln).group(1) == ident]
        assert len(hit) == 1, (volume, ident, hit)
        parts = lines[hit[0]].split("|")
        parts[-2] = f" {new_cell} "
        lines[hit[0]] = "|".join(parts)
        return volumes

    def test_arm_a_new_verb_shape_fires_the_roster(self):
        """换成清单外的「已了结（L253）」⇒ 组合名册与词汇表同时变，而不是静默不算标记"""
        volumes = self._arm("OPTIMIZATION_LOOP_2.md", "A330", "~~M~~ 已了结（L253）")
        got = {k: len(v) for k, v in combos(volumes).items()}
        assert got != MEASURED_COMBOS
        assert got.get(("~~M~~", "无动词", "L", False), 0) + \
            got.get(("~~M~~", "已了结", "L", False), 0) == 1, got
        words = closure_counts(volumes)
        assert words.get("已了结") == 1 and words != MEASURED_WORDS, words

    def test_arm_dropping_the_round_leaves_an_untraceable_cell(self):
        volumes = self._arm("OPTIMIZATION_LOOP_2.md", "A330", "~~M~~ 已收口")
        no_round, _, _, _ = traceability(volumes)
        assert no_round == ["A330"], no_round

    def test_arm_lying_about_a_round_is_unresolvable(self):
        volumes = self._arm("OPTIMIZATION_LOOP_2.md", "A330", "~~M~~ 已收口（L999）")
        _, unresolvable, _, _ = traceability(volumes)
        assert unresolvable == [("A330", (999,))], unresolvable

    def test_arm_dropping_the_cross_volume_qualifier_breaks_the_pairing(self):
        volumes = self._arm("OPTIMIZATION_LOOP.md", "A46", "~~S~~ 已收口（L156，B226）")
        cross = traceability(volumes)[2]
        assert [c for c in cross if c[1] == "A46"][0][3] == "L", cross
        assert any(h != "第二本账" for *_x, h in cross)

    def test_arm_marking_an_open_row_moves_the_authoritative_face_only(self):
        """给待拍的 A335 补一格 ⇒ 权威面 +1、字母形与组合名册同时变"""
        before_wide, before_narrow = faces(load_volumes())
        volumes = self._arm("OPTIMIZATION_LOOP_2.md", "A335", "~~S~~ 已收口（L253）")
        wide, narrow = faces(volumes)
        assert len(narrow) == len(before_narrow) + 1 and "A335" in narrow
        assert sum(len(v) for v in combos(volumes).values()) == len(before_narrow) + 1
        assert closure_counts(load_volumes()) != closure_counts(volumes)

    def test_arm_a_prose_closure_phrase_moves_the_wide_face_only(self):
        """A328 立账时那格自己撞上过的形状：正文里写一句闭合语，定级格不动 ⇒ 宽面多一行"""
        before_wide, before_narrow = faces(load_volumes())
        victim = [r for r in backlog_rows(load_volumes())
                  if not MARK.search(r["text"]) and BARE.match(grade_cell(r["text"]))][0]
        volumes = load_volumes()
        lines = volumes[victim["volume"]]
        parts = lines[victim["line"] - 1].split("|")
        parts[-3] = " 待拍，正文里补一句 `已收口（L253）` 的引用 "
        lines[victim["line"] - 1] = "|".join(parts)
        wide, narrow = faces(volumes)
        assert victim["id"] in wide and victim["id"] not in narrow
        assert len(wide) == len(before_wide) + 1 and len(narrow) == len(before_narrow)


def closure_counts(volumes):
    """定级格里出现过的闭合词及其行数（词汇封闭清单的现量侧）"""
    out = {}
    for row in backlog_rows(volumes):
        for word in closure_words(grade_cell(row["text"])):
            out[word] = out.get(word, 0) + 1
    return out
