"""L79 / A127 守卫：两份活文档里的「定位引用」必须真的指得到东西

**为什么立这一条**：本仓的架构文档与循环账本大量用反引号引用（`` `path/file.py:123` ``）给
读者指路。代码一改行号就漂，漂到极点是引用指向**空行 / 注释 / 已经不存在的文件** —— 读者照
引用跳过去什么也找不到，而这条失败**没有任何机械通道会出声**。L77 / L78 各抓到一次「账本里
写着的 file:line 指向别处」，两次都是人工回读撞上的（A87 的现场样本，见 §3.29 末尾那段勘误），
所以本轮把它变成用例。

**口径（本轮拍定）**：

1. 反引号里的「文件名 + 冒号 + 行号」= **定位引用**，本守卫只管这一种。
2. 要留**历史读数**（「当时在第 46 行」）就写成不带反引号的散文，或写成名字锚点。架构文档里
   原有的 7 处此类例子（§3.27 那段勘误的 4 个数字 + 同一句话在 §3.27 正文与 §6 表的 3 次出现）
   在 L79 就地改成散文；另有 30 处补了目录前缀、账本 22 处改指名字锚点。
3. 「一个名字对应仓里多个真实文件」= **歧义引用**，同样不许（`config.py` 有 `augmentor/config.py`
   与 `api/routes/config.py` 两支，读者无从判断）。
4. 指向**取证脚本**（`Temp/` 下、不入仓）的引用不算活引用：单独计数、只当棘轮。
5. 但 **Temp 里有同名副本不洗白产品路径**：`Temp/l*/head/` 那些 HEAD 快照让
   `` `augmentor/api/routes/dataset_tools.py` `` 这种**从未存在过的路径**也能"找得到文件"，
   所以先按顶层目录判产品路径，再考虑工件（`name_bucket` 的顺序即口径）。
6. **一个桶不许混两类**：可证失效（`dead_line` / `unresolved_source`，硬 0）与已记欠账
   （棘轮档）同住一个桶时，真 bug 会被棘轮的「只降不升」读成正常状态 —— 本轮 4 处产品
   路径写错就是这么藏住的（`scratch_missing` 里 111 个草稿名把它们盖住了）。

**判据一律「只降不升」，改进也得改常数**（承 L76 / L77 的棘轮口径）：`CEILING` 里每条都是
**精确相等**而不是「≤」。所以「顺手清掉一条歧义」同样会让用例红 —— 那是设计：它逼着清账的人
在账本上留一句「谁清的、清完剩多少」。

**残余项**：`code_but_no_name_match` 那一档（行号指着代码行、但被点名的标识符不在该行声明）
本轮**不清**、只量（现量以 `MEASURED` 为准，L82 之后是架构文档 38 处 / 账本 171 处）。它要逐条读原句
判断「那句话指的是哪件事」，属 A127 的后半程。**本守卫的边界（A129）**：散文里的行号它看不见，
而本轮实测到「连勘误段自己写下的那两个位移读数也位移了」⇒ 这一条不在这里修，只在 §3.33 声明。
"""
import ast
import collections
import functools
import os
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[2]

# 文件名段必须**至少含一个标识符字符**：否则散文里「以 `.py` 结尾」这种话会被当成引用。
FILE_TOKEN = re.compile(r"`([A-Za-z0-9_/\\.\-]*[A-Za-z0-9_]\.py)`")
LINE_REF = re.compile(r"`([A-Za-z0-9_/\\.\-]*[A-Za-z0-9_]\.py):(\d+)(?:-(\d+))?`")
BARE_REF = re.compile(r"`:(\d+)(?:-(\d+))?`")

ARCH = "docs/ARCHITECTURE.md"
LEDGER = "OPTIMIZATION_LOOP.md"
DOCS = (ARCH, LEDGER)

#: L79 就地清账后的基线。键是 `audit()` 的桶名；**只降不升**，见模块 docstring。
#: 现量读数见 `Temp/l79q/lineref_buckets.json` 里 `runs` 的最后一条（命令
#: `PYTHONIOENCODING=utf-8 C:/Python314/python.exe Temp/l79q/buckets.py <label>`）：
#: 架构文档 = 0 / 0 / 0 / 0 / 18 / 29，账本 = 0 / 0 / 69 / 145 / 151 / 111（L83 回填后）。
#: **L82 回填（两桶同时降，且降下来的原因不是「清了旧账」）**：`bare` 架构文档 20 → **18**
#: 的净 −2 = **删 3 加 1**（逐条命令现量：删 `` `:33` `` / `` `:34` `` / `` `:308` ``、加
#: `` `:250` ``），全部发生在同一处 —— L82 复原 A 级位移时重写 `docs/ARCHITECTURE.md` 里
#: 讲 `AugmentationConfig` 那一段，把两处续写行号改成散文（「第 308 行」）、一处换成新行号。
#: 账本 153 → **152** 同理是「本轮新写的活文本自己达标」：A140 / A123 / A141 三行用
#: `file.py:82` 起 + 「逐处见 Temp 工件」收尾，不再抄 `:83` / `:86` 那种续写（A141 立项的
#: 理由第一次用在自己身上）。同批把 A118 关闭后失效的 5 条 `dead_line` 清零，与本轮写新
#: 日志块时**新长出的那 1 条 `dead_line`**（一句散文里顺手写了 `test_config_validator.py` 加
#: 行号，而那个行号已被本轮自己的插行推成空行）互不相消 ⇒ 现场改散文、不动上限。
#: **L83 回填（账本 `bare` 152 → 151，降幅全部来自 A141 那一族的就地还税）**：L83 只动了
#: 三份产品码（`validation.py` +57/−8、`config_validator.py` +26/−4、`augmentor/config.py`
#: +12/−3），可证失效当场 2 条（A92 的 `validation.py:442` 被顶成空行 ⇒ 改指现量 637；
#: L47 日志块那条定位引用 + 它的裸行号 ⇒ 降散文，这一处正是 `bare` 那一桶净减 1 的唯一
#: 来源）。**同轮我自己还写坏了 2 条 `ambiguous_file`（145 → 147）**：日志块里两处裸
#: `` `config.py` `` 一名两指（仓里另有 `api/routes/config.py`，两个家），当场改成
#: `` `augmentor/config.py` `` 才回 145 —— 上限没被我抬上去，但这是「新写坏 → 就地改回」
#: 而不是清账，记在这里是为了
#: 让下一轮看见：`ambiguous_file` 的 145 是**被本轮写坏过又修平的数**，不是稳态。
#: `unresolved_source` = 「点名的路径第一段是仓内某个装 `.py` 的顶层目录（现量
#: `api` / `archive` / `augmentor` / `scripts` / `tests`），可这个文件不存在」—— 它与
#: 「指向不入仓的 Temp 草稿」必须分家：本轮 4 处**产品源码路径写错**原本被
#: `scratch_missing` 或 Temp 快照副本吞掉（判据太粗 ⇒ 把真失效藏起来），分家后当场可见。
#: **L87 回填（两格同时下降，且原因不是「清了旧账」）**：`ambiguous_line` 69 → **68**、
#: `bare` 151 → **149**、`ambiguous_file` 仍 **145**。降下来的两格全部来自本轮把 A153
#: 那一行里的数字引用改成名字锚点或占位形（`.py:NNN`）—— 也就是说**这两格是被「删掉引用」
#: 降的，不是被「消歧」降的**，把它读成「账本变干净了」是误读。`ambiguous_file` 那一格
#: 本轮涨过一次又回到 145，涨的原因与 L83 注释里那次**同族**（裸文件名一名两指），
#: 细节与教训记在账本 L87 日志块的「文档面与本轮还的税」条里。
CEILING = {
    ARCH: {"dead_line": 0, "unresolved_source": 0, "ambiguous_line": 0,
           "ambiguous_file": 0, "bare": 18, "scratch_missing": 29},
    LEDGER: {"dead_line": 0, "unresolved_source": 0, "ambiguous_line": 68,
             "ambiguous_file": 145, "bare": 149, "scratch_missing": 111},
}

#: 反空转下界：`line_refs` / `file_tokens` 是**两种语法各自的匹配总数**（不是桶的加和 ——
#: 上一版让两个总数共享 `ambiguous` 那一桶，账本那 214 条歧义引用同时顶在两个总数上）。
#: 下界取整留了余量 —— 这两个数是**好引用的规模**，不是缺陷计数，删引用是改进、不该被
#: 反空转下界卡住；它只负责证明「普查真的看见了成百条引用」，而不是钉住总数。
FLOOR = {
    ARCH: {"line_refs": 40, "file_tokens": 200},
    LEDGER: {"line_refs": 270, "file_tokens": 1050},
}

#: **现量**读数（`Temp/l79q/buckets.py` 的最后一个 run）。凡是在注释、`docs/ARCHITECTURE.md`
#: §3.33 或账本里写「实测 N 处」，那个 N 只许从这份常量取 —— A77「一份权威只住一处」用在
#: **测试自己的注释**上：本轮 §3.33 里「40 处 / 114 处 / 10 例」三个数就是在同一轮内过期的，
#: 因为文档段落写在了判据改完之前。
#:
#: **这一份每轮都要重量**（L79 收尾现场兑现）：账本日志块写完之后，正文自己引用的 7 个仓内文件
#: 与 7 个 Temp 草稿名把 `file_tokens` 从 1075 顶到 1089 —— 与 `CEILING` 里那些「随代码变坏而涨」
#: 的缺陷档不同，这里是**对账位**：它红说的是「文档里的数靠了回忆」，不是「代码变坏了」。
#:
#: **L80 回填（两条都是文档面，不是代码变坏）**：`line_refs` 281 → 280 是把 Backlog B 的 B3①
#: 那一格从定位引用改成散文行号（漂移档那条由产品码插行造成的新增同时消掉，仍 165）；
#: `file_tokens` 1089 → 1123 来自「收尾实测」小句与 A136 行点名的 6 份仓内文件。
#:
#: **L81 回填（同样是文档面，且第一次写的归因是错的）**：`line_refs` 280 → **285** 的 +5 **全部**
#: 来自新立的 A137 行（那一格点名了 5 处窄/宽档 CJK 分叉的源码位置），L81 日志块 / 进度行 /
#: A136 补字三处的定位引用增量都是 **0**；`file_tokens` 1123 → **1149** 的 +26 =
#: 「日志块 21 + A136 补字 2 + A137 行 1 + 进度行 1 + 回看 ⑪ 的探针名 1」五笔。**上一版注释把 +5
#: 写成「日志块 +7、A137 +5」**，因为第一版探针 `Temp/l81q/attr.py` 用 `^- **L\d+** ` 当块尾锚点，
#: 而本账最后一节（`- 全量：L4 后 …` 那条散文清单）不是日志块标题 ⇒ 正则一路吞到文件末尾，把
#: 51,249 字符的整个文件尾部当成 11,639 字符的 L81 块删掉。修法 `Temp/l81q/attr2.py` 改成按
#: 「块标题 → 下一条非块行」切，并**要求逐块减法之后残余为 0**（`residual = 现量(还原后) − 现量(HEAD)`
#: 必须三桶全 0，否则归因不成立）—— 归因探针必须自带闭合断言 ⇒ 本轮升为纪律 **(s)**。
#: **终态那一次改用更强的闭合口径**：按 token 多重集对 HEAD 求差 ⇒ 新增 20 个不同文件名合计 +26、
#: **删除 0** ⇒ 纯增可加，不需要逐块减法就能闭合（逐块减法只在「同一轮里既有增又有删」时才是必需的）。
#: `code_but_no_name_match` 165 = 165 未动（本轮 0 处新增「指向不存在的名字」）。
#: **`file_tokens` 在本轮内被重估四次**（1146 → 1148 → 1149 → 本常数，每次给日志块补回看就多一两个反引号
#: 文件名）⇒ 纪律 **(h)**（文档里的计数句最后写）本轮第四次兑现。
#:
#: **L82 回填（口径升级一次：差集必须在「同一棵树」上求）**：架构文档 47 / 213 / 37 →
#: **48 / 217 / 38**，账本 285 / 1149 / 165 → **287 / 1196 / 171**。架构文档那格的 `file_tokens`
#: 在本轮内被**自己**顶高两次（215 → 217 = §3.33 新写的「现量只住常数」那句指针补了 2 个反引号
#: 文件名），于是这一格第一次撞上一个新形状：**归因句写完之后又改了一次常数** ⇒ 217 的读数不是
#: 事后补的，是那次改动之后重跑全量时由本守卫报出来的（`('docs/ARCHITECTURE.md', 'file_tokens',
#: 217, 215)` —— 守卫的失败信息直接给出三元组，这就是它存在的理由）。这里的被减数是**在现树上重跑
#: `audit()` 得到的 HEAD 文档态**，不是照抄 L81 写下的那三个终态常数 —— 本轮给 `augmentor/config.py`
#: 等文件插了行，于是同一份文档在同一树上的分桶自己就会位移（实测：账本 `code_but_no_name_match`
#: 的 HEAD 态在现树上仍是 **165**，架构文档那一条却从 L81 记的 **37** 变成 **36**，差的那 1 条是
#: §3.33 里一条引用因源码插行恰好撞上了名字）。**这是纪律 (t) 的分桶版**（旧快照只能对照、不得
#: 订正终态）：不绑树版本的读数不能当被减数。归因走 L81 终态那次的**更强闭合口径**（token 多重集
#: 对 HEAD 求差）：两份文档的 `code_but_no_name_match` 分别是**加 2 / 删 0**（架构文档）与
#: **加 6 / 删 0**（账本）⇒ 纯增，不需要逐块减法即闭合；账本 `file_tokens` 的 +47（先 +49、收尾
#: 句里把两份测试文件名改成散文名又 −2，A141 的口径第一次用在自己的收尾句上）全部来自 L82
#: 日志块、A138–A142 五行新格与进度行（本轮 0 处删引用，实测「删 0」）。**「同轮读数过期」在本轮
#: 只被压住了一半**：账本那三格确实是「先落文档终态、再量桶、再回填」（纪律 (h) 第一次按顺序执行），
#: 架构文档那格没有 —— 写完 §3.33 的指针句之后又补了 2 个文件名，而那份句子本身就是「别抄散文里的数」
#: 的口径，绕不开 ⇒ 结论修正为 **(h) 只能对「一份文档」成立，跨文档的互引（本文件引用另一文件的
#: 计数）仍会过期，因为写指针句的动作本身就在增加 token**。无论哪一侧，代价都一样：`MEASURED`
#: 自己不许出现在散文里当数（A129 的老边界，本轮在 §3.33 新写的那句指针里又复述了一次）。
#:
#: **L83 回填（三格位移，全部可对账到「本轮写了哪两段文档」，无一是代码变坏）**：架构文档
#: `file_tokens` 217 → **219**（§6 错误处理表新增的那一行点名了 `validation.py` 与
#: `config_validator.py`）、`code_but_no_name_match` 38 → **37**（净 −1；本轮**没有**为这一格
#: 跑逐条归因探针，只验到「桶的净差 = −1 且 `dead_line` 双侧仍为 0」，具体是哪一条从漂移档搬
#: 回名字命中档要 A141 候选 ③ 那种固定 refshift 才能说 —— 按纪律 (u) 不把没量过的因果写进
#: 注释。可证的是：L82 说过「同一棵树上的 HEAD 态自己会漂」，被减数因此必须是本轮重跑的桶，
#: 不是照抄 L82 常数）。账本
#: `file_tokens` 1196 → **1218**（+22：A142 关闭段、L83 日志块十条子弹与进度行；本轮对引用的
#: 唯一一次删改是 A141 口径下把 2 条失效引用**改成散文/改指新号**，净效果已含在这 +22 里）、
#: `code_but_no_name_match` 171 → **170**（A92 那一格从 442 改指现量 637 之后不再命中空行）、
#: `line_refs` 287 → **287 未动**（一进一出：新写的定位引用与降为散文的那条相互抵平 —— 这句
#: 是对账后回写的，不是先写的：补「收尾实测」那一节之前量到 286、之后 287，纪律 (h) 在跨文档
#: 互引上仍然只能对「一份文档」成立）。
#:
#: **L84 回填（本轮全部是「新增引用恰好都带了名字锚点」，无一是代码变坏，也无一处删改）**：
#: 架构文档 `file_tokens` 219 → **221**、`file_unique` 175 → 177（§6 新增那一格点名两份产品
#: 文件），`line_refs` 48 与 `code_but_no_name_match` 37 **未动**。账本 `line_refs` 287 →
#: **292**（+5 全部来自 A143 那一行，且**五条全落 `name_hit`**：`api/routes/export.py:153` /
#: `augmentor/cli/parser.py:50` / `augmentor/export.py:235` / `augmentor/export_enhanced.py:27` /
#: `tests/integration/test_cli_merged_commands.py:1081`）、`file_tokens` 1218 → **1236**（+18：
#: 进度行、L84 日志块、A143 与 B2 两处注解；其中 5 个是 `Temp/l84q/` 探针名，落 `scratch_artifact`
#: 而不进缺陷档）、`code_but_no_name_match` 170 → **170**。**这一格本轮是被判据抓过一次才对的**：
#: A143 初稿写了两条「真代码行但没有名字锚点」的引用（`api/routes/export.py:158` 与
#: `parser.py:141`），按 token 多重集对 HEAD 求差时它们是**新增 2 / 删除 0**，即漂移档要涨到 172 ——
#: 改指 `list_export_formats` 那一行与那份常数本身之后回到 170（同一把差集探针第二次兑现，
#: 第一次是 L81 终态）。纯增 ⇒ 不需要逐块减法即闭合。
#:
#: **L84 终态复跑回填（第 25 次 census，`buckets.py "L84 终态复跑后"`）**：`file_tokens` 1236 →
#: **1239**（+3 = 「终态复跑」那一小节点名了 `config.yaml` / `augmentor/config.yaml` 与两份
#: `Temp/l84q/full_*_terminal.txt` 工件；其余反引号串是 `--cov` / `--collect-only` 这类开关，
#: 不是文件名形状）；`line_refs` **292**、`code_but_no_name_match` **170**、架构文档整格
#: （48 / 221 / 37）**逐位未动** —— 终态复跑只往账本加了散文，没加任何行号引用，所以漂移档
#: 不涨是**预期**而不是运气；如果哪一轮的「只补数字」让小节写出了新的裸行号，这里会当场红。
#: **L85 终态回填（第 28 次 census `buckets.py "L85-c28"`；c26/c27 是本轮回填过程中的两次中间量）**：
#: 架构文档 `line_refs` 48 → **46**、`code_but_no_name_match` 37 → **35**、`file_tokens` 221 →
#: **223**。三条的归因**用归因房现量**（`Temp/l85q/attr_l85.py`：同一份索引下分别审计 HEAD 版与
#: 工作区版的本文，取桶计数差 + 明细集合对称差）：`line_refs` −2 与 `dead_line` −2 **同一件事**
#: —— 那两条就是被 L84 之后 `test_micro_branches_l61.py` 的行号位移打成 dead_line 的 `:99` 引用，
#: 本轮按 A141 候选 ① 降为散文；`code_but_no_name_match` 37 → 35 是**同两条引用在 dead_line 与
#: 本档之间换了桶**（L84 census 时它们还在 37 里，本轮重测该文件后落到 dead_line），归因房给出
#: 「HEAD 版与工作区版这一档**同数同集合**（35 / 35）」⇒ **本档本轮没有新增漂移，是还了 2 条旧债**；
#: `file_tokens` +2 = `file_unique` +2，正是那两处散文新点名的文件名。
#: 账本 `line_refs` 292 → **299**（+7 = **新增 13 条 − 降级 6 条**，逐桶闭合：`name_hit` 53 →
#: 66（+13 全部带名字锚点）、`dead_line` 6 → 0、`code_but_no_name_match` 净 0）、`file_tokens`
#: 1239 → **1293**（+54：A144/A145/A146 三行、L85 进度行与 L85 日志块 35 行；**这一格当场演示了本文件
#: 开头写的那条「写指针句的动作本身就在增加 token」，而且是两次** —— c28/c29 量到 1291，把「用了归因房
#: `attr_l85.py`」写进收尾小节 ⇒ 第 1292 个 token；再把「那处降级」写成带冒号行号的引用形状 ⇒
#: 1293 并在 c31 额外量出 1 条 dead_line（同一句话既抬数又写坏，是这条口径最硬的一个实例），改成
#: 不带引用形状的散文后 c32 落定 1293 且 `dead_line` 回到 0 ⇒ **被引用的终量是 c32**）。
#: `code_but_no_name_match` 170 → **164**。**这一档的 −6 不是本轮写的**：归因房量到「HEAD 版本文
#: 在今天的索引下就是 164」，也就是**本轮自己**给那份测试补了 L85 的注释（它是本轮工作区里被改过的文件之一），第 99 行从代码
#: 移成了注释 ⇒ 账本里那 6 条 `:99` 引用从「真代码行无名字锚点」降成「dead_line」，**代码动在了文档之前**（而且动它的是写文档的人自己）。本轮既没新增
#: 漂移也没补旧漂移（164 → 164），A144 / A146 初稿那三处无锚点引用（`pipeline.py:125` 两处与
#: `comparison.py:237-246`）在回填时改指 `_init_components` / `compare` 两个 def 行后归零 —— 这是
#: A141 口径第四次兑现，也是第一次**在同一次回填里就把新写的引用全部落成名字锚点**。
#: `scratch_missing` 曾在本轮回填中涨到 112（我在日志块里引了一个当时不存在的探针名
#: `Temp/l85q/import_tax.py`）⇒ 修法是把那个探针**写成文件**并当场重量一次（新值 393.6 / 241.4 ms，
#: 与第一次量到的 371.8 / 242.2 同形状、差在缓存冷热），涨上去的那 1 格随之回到 **111**。
#: **L86 回填（账本三格同时动；`code_but_no_name_match` 那 +5 是本轮改代码改出来的，不是写文档写坏的）**：
#: 账本 `line_refs` 299 → **308**（+9）、`file_tokens` 1293 → **1321**（+28，其中 +5 是「写下
#: 1316 这个数字」这句话自己抬的 —— c2/c3 两次复量停在 1321 ⇒ 被引用的终量取 c3）、
#: `code_but_no_name_match` 164 → **169**（+5）。归因房（`Temp/l85q/attr_l85.py`，HEAD 版账本 vs
#: 工作区版同一索引）量到 **HEAD 版账本在今天的代码索引下就是 169** —— 也就是说这 5 格与 L86
#: 写的任何一句话无关，而是 **L86 自己改动的两份产品文件把行号挪了位**：明细逐条只有
#: `checkpoint.py` 三格（184 / 218 / 253）与 `streaming.py` 两格（317 / 328），**全部位于 A144 /
#: A145 两行行首那段划掉的「L85 立案时落点」**（每段后面都紧跟一段「L86 后的现量落点」，后者
#: 逐格命中名字锚点）。⇒ 结构性发现：**「已关闭行的历史定位引用」这一族必然随代码改动而腐坏**，
#: 机器分不清「历史」与「现行」，只能靠 `MEASURED` 的精确相等把它逼到台面上（本轮全量那唯一 1 条红
#: 就是它）。这一族要不要单独立一个不计入棘轮的桶，属对外口径扩张 ⇒ 见 A148。
#: 新写的 9 条 `line_refs` **全部带名字锚点**（`name_hit` 61 → 70，+9 与 +9 闭合），
#: **四个棘轮桶本轮一格未抬**：`ambiguous_line` 69 = 上限、`ambiguous_file` 145 = 上限、
#: `bare` 151 = 上限、`scratch_missing` 111 = 上限 ⇒ L86 日志块没有引入一条歧义/裸/失效引用。
#: **L87 回填（三格两降一升，升的那格与降的两格是同一个动作开的）**：本轮为 A149 在
#: `api/deps.py` 与 `augmentor/config.py` 插了码 ⇒ 两份文档里指向它们的数字引用先成片失效
#: （`dead_line` 一度 5 格），我按 A127 口径逐格改成**函数名锚点**之后 `line_refs` 与
#: `code_but_no_name_match` 同时下降；**而 A153 那一行（它就是来记这个盲区的）在描述失效
#: 引用时把被描述的那两条原样抄进了账本，于是记账行自己变成两条新的 `dead_line`，被硬 0
#: 档当场拦下** —— 这是本程序第一次「立案的那一行当场触发被立案的缺陷」。改成占位形后
#: 收敛读数（`Temp/l79q/lineref_buckets.json` 的 run `L87 记账 c5`）：架构文档 46 → **42**
#: / 223 → **229** / 35 → **37**，账本 308 → **297** / 1321 → **1360** / 169 → **165**。
#: **架构文档那两格反向移动（`line_refs` 降 4 而漂移档升 2）本轮未做逐块归因**，
#: 只报差不编因；要还这笔先跑 L81 那类带闭合断言的 attr 探针。
#: **L88 回填（三格全是文档面，且第一次把 L87 欠的那笔归因还掉）**：探针
#: `Temp/l88q/bucket_attr.py` 按 L82 终态那次的**更强闭合口径**跑（HEAD 文本与现文本在
#: **同一棵树**上各跑一次 `audit()`，再按 token 多重集求差）——
#: 架构文档 42 / 229 / 37 → **42 / 230 / 37**：净 +1 且**删 0**，那 1 个 token 就是新写那段
#: 失效边界里点名的 `test_cache_invalidation_l88.py`（`file_unique` 184 → 185 同向闭合）。
#: **L89 回填（1367 → 1383，+16）**：本轮进度行 + 日志块 + A152 / A157 两行状态里的反引号文件记号。
#: 三格读数的移动方向本身就是口径：**`line_refs` 与 `code_but_no_name_match` 一格未动**（本轮刻意全部用
#: 名字锚点、不写行号），涨的只有 `file_tokens`；四个棘轮桶（68 / 145 / 149 / 111）与 `dead_line` 两侧
#: 硬 0 全部停在原读数 ⇒ 新增的 16 格记号**逐条指向真实存在的文件**（`scratch_artifact` 一桶同步吸收）。
#: 账本 297 / 1360 / 165 → **296 / 1367 / 168**，其中 **+7 纯增**（`api/routes/config.py`、
#: `api/deps.py`、`augmentor/config.py`、两份 `Temp/l88q/` 探针 ×2 与 `test_upload_ceiling_l87.py`，
#: 删 0 ⇒ 纯增可加，不需要逐块减法即闭合）、**−1 条 `line_refs`** 是本轮把 A138 那格的
#: 「文件:行号」降成名字锚点（同一动作让 `dead_line` 1 → 0，硬 0 档当场抓住的那一条就是它）。
#: **`code_but_no_name_match` 那 +3 与本轮文本无关**：HEAD 版账本在**现树**上重跑就是 168
#: （`dead_line` 也同态给出 1，正是 A153 预言的形状），所以漂移全部由本轮给 `api/deps.py`
#: 插码 + 路由 import 摊成六行这两处**源码位移**解释 ⇒ **L87 那句「要还这笔先跑带闭合断言的
#: attr 探针」在本轮兑现**（还的是「同树 HEAD 态」这一半；跨文档那一半仍未还）。
#: **四个棘轮桶本轮一格未抬**：账本 `ambiguous_line` 68、`ambiguous_file` 145、`bare` 149、
#: `scratch_missing` 111 全部等于上限，`dead_line` 与 `unresolved_source` 两侧仍为 0。
MEASURED = {
    ARCH: {"line_refs": 42, "file_tokens": 231, "code_but_no_name_match": 37},
    LEDGER: {"line_refs": 296, "file_tokens": 1383, "code_but_no_name_match": 168},
}

#: 日志块标题行上「下一轮才填得出自己哈希」的占位符。精确相等 = **只许有该填的那几条**。
#: 实测命中 15 = L58..L71 那 14 条从未回填的历史欠账 + 上一轮（L78）待回填那 1 条；
#: 本轮把 L78 填掉、加上 L79 自己写的，仍是 15 ⇒ 谁忘了回填上一轮，用例当场红。
#: **不能写成 `text.count("哈希待")`**：账本里那个串出现 26 次，多出的 11 次是散文在
#: **谈论**「哈希待回填」这件事（如 L64 那句「本账自身 numstat 待 L65 量」），不是欠账本身。
#: 补那 14 处历史正文要不要做属 A130（待用户拍）。
#: **L86 回填侧（15 → 14，本常数每一轮要动两次，这是第一次）**：L85 的三批哈希落地后它自己的
#: 占位符被填掉 ⇒ 现量只剩 14 条历史欠账。本轮写 L86 日志块标题时会重新加回 1 条 ⇒ 那时要改回
#: 15。**L86 兑现了那半句**：日志块与进度行落笔时把常数改回 **15**，L87 填掉 L86 哈希时再改回 14。**为什么不留 14..15 的区间**：区间判据会让「忘了回填上一轮」和「忘了给自己留占位」两种
#: 相反的失误互相抵消（都落在区间内），而精确相等时两者各红一次 —— 这正是它每轮必须动两次的原因。
#: **L89 回填侧（15 → 14）**：填掉 L88 的三批哈希后只剩 14 条历史欠账；本轮写 L89 块标题时改回 15。
HASH_PLACEHOLDER = re.compile(r"^- \*\*L\d+\*\* `哈希待 L\d+ 回填`", re.MULTILINE)
PLACEHOLDER_CEILING = 15

#: 账本的章节骨架，**逐字且按序**（L80 现量）。判据是「提取出的标题行序列 == 这份元组」，
#: 所以标题被吃掉、被改名、被挪序、被多插一节都会红。
#:
#: **这一档为什么必须存在**：`## Backlog A — 性能（含 file:line 与实测线索）` 在
#: `9ba59b493`（L78）之后就不在文件里了 —— `git show 0101470c5:./OPTIMIZATION_LOOP.md` 里
#: 那行还在，`git show 9ba59b493` 里它已被删（`git log -S"## Backlog A"` 只点名这两个提交）。
#: 根因是本账已用散文记过三次的同一形状：拿**紧跟表格行的节标题**当 Edit 锚点，
#: 新字符串里忘了把锚点带回去。散文不是判据 ⇒ 第二次吃掉时没有任何东西会红。
#:
#: **为什么 L79 的普查抓不到它**：那台机器只看反引号引用，A 表的 130+ 行一条没少，
#: 少的是它头上那行字。发现过程本身就是证据：L80 想往「已完成」列表尾部插进度行，
#: 拿 `## Backlog A — 缺陷清单…` 当锚点，Edit 报 0 occurrences，回读才看见整节标题不存在。
LEDGER_SECTIONS = (
    "# augmentor 优化循环进度追踪（2026-09-23 启动，目标 100 轮）",
    "## 已完成",
    "## Backlog A — 性能（含 file:line 与实测线索）",
    "## Backlog B — 功能增强（价值 ÷ 工作量）",
    "## 循环日志",
)
HEADING = re.compile(r"^#{1,2} \S.*$", re.MULTILINE)


def headings(text):
    """账本里的一二级标题行，按出现顺序 —— 骨架判据的唯一提取口"""
    return HEADING.findall(text)


@functools.lru_cache(maxsize=1)
def build_index():
    """`{文件名: [仓内相对路径的段元组]}` —— 按名字分桶，查找代价与全仓规模无关

    排除 `.git` / `__pycache__` / `.pytest_cache` / `node_modules`；`Temp/` **保留**，因为
    账本里的取证脚本引用要能被认成「指向工件」而不是「指向不存在的文件」。
    """
    index = collections.defaultdict(list)
    for base, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in (".git", "__pycache__",
                                                ".pytest_cache", "node_modules")]
        for name in files:
            if name.endswith(".py"):
                rel = os.path.relpath(os.path.join(base, name), ROOT)
                index[name].append(tuple(pathlib.PurePath(rel).parts))
    return {k: tuple(sorted(v)) for k, v in index.items()}


def source_dirs(index):
    """仓内**装 `.py` 的顶层目录**（`Temp` 除外）

    用它区分「这条未解析引用是**写错了的产品源码路径**」还是「一个不入仓的草稿名」。
    不硬编目录清单：下一轮新增 `tools/` 时这里自动认得，而写错的 `tolus/foo.py`
    会因为第一段不存在而被归进草稿那一档（宁可漏判成棘轮，不误判成硬门禁）。
    只取 `len(parts) > 1` 的路径 —— 根目录的 `cli.py` 自身会给出 `('cli.py',)`，
    它不是目录，把它当目录会让「`cli.py` 的草稿邻居」这类引用错误地升格成硬失效。
    """
    return frozenset(p[0] for paths in index.values() for p in paths
                     if len(p) > 1 and p[0] != "Temp")


def name_bucket(rel, index, src):
    """引用名 → `(结论桶, 唯一活路径)`；桶为 `None` 才表示「这一名指得到唯一文件」

    **顺序就是口径**（L79 实测教训）：先判「第一段是真实源码目录」，再考虑 Temp 里的同名
    尾巴。反过来的话 `` `augmentor/api/routes/dataset_tools.py` `` 会被 Temp 快照副本
    「洗白」成工件引用 —— 可那个前缀从来不存在（真实路径是 `api/routes/dataset_tools.py`），
    一条**可证失效**就此隐形。

    同理，未解析也不能只留一档：`scratch_missing` 若同时吞「不入仓草稿名」和「产品路径写错」，
    而它是只降不升的棘轮 ⇒ 真 bug 被记成欠账、当场没有任何东西会红（本轮实测到 4 条）。
    """
    live, scratch = resolve(rel, index)
    if len(live) == 1:
        return None, live[0]
    if len(live) > 1:
        return "ambiguous", None
    if rel.replace("\\", "/").split("/")[0] in src:
        return "unresolved_source", None
    return ("scratch" if scratch else "scratch_missing"), None


def resolve(rel, index):
    """文档里写的相对名 → 仓内真实路径，返回 `(活文件, Temp 工件)` 两组；歧义**不猜**"""
    parts = tuple(p for p in rel.replace("\\", "/").split("/") if p)
    live, scratch = [], []
    for rp in index.get(parts[-1], ()):
        if rp[-len(parts):] == parts:
            (scratch if rp[0] == "Temp" else live).append("/".join(rp))
    return sorted(live), sorted(scratch)


@functools.lru_cache(maxsize=None)
def declared_lines(rel):
    """`{行号: 以该行为声明起点的标识符}`，只认 def / class / 赋值（含带注解的赋值）"""
    table = collections.defaultdict(set)
    tree = ast.parse((ROOT / rel).read_text(encoding="utf-8"))
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            names = [node.name]
        elif isinstance(node, ast.Assign):
            names = [t.id for t in node.targets if isinstance(t, ast.Name)]
        elif isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name):
            names = [node.target.id]
        else:
            continue
        for n in names:
            table[node.lineno].add(n)
    return dict(table)


def classify(lines, start, end, names_at):
    """`dead_line` 按引用形状分两档 —— 这是**口径**，不是把判据放宽

    单行 `` `f.py:42` `` 的语义是「跳到 42 行」⇒ 那一行必须是代码行。
    区间 `` `f.py:76-78` `` 的语义是「这件事在 76..78 之间」⇒ 只有**整段都不是代码**
    （全空行 / 全注释 / 越界）才算可证失效；起点空、终点是代码的那档属「区间报歪一行」，
    归漂移档（A127 后半程），不该由硬门禁开火。
    """
    last = start if end is None else max(end, start)
    span = []
    for ln in range(max(start, 1), last + 1):
        if ln > len(lines):
            span.append("out-of-range")
            continue
        raw = lines[ln - 1].strip()
        span.append("blank" if not raw
                    else "comment" if raw.startswith("#") else "code")
    if "code" not in span:
        return "dead_line"
    return "name_hit" if names_at.get(start) else "code_but_no_name_match"


def audit(text, index):
    """扫一份文档，返回 `(counts, rows)`；`rows` 是「桶 → 引用原文」，供失败信息直接抄

    桶的名字就是**结论**：`dead_line` / `unresolved_source` = 可证失效（硬 0），
    `ambiguous_line` / `ambiguous_file` / `bare` / `scratch_missing` = 已清账的残余（棘轮），
    `name_hit` / `file_unique` = 好引用，`code_but_no_name_match` = 漂移档（A127 后半程），
    `scratch_line_ref` / `scratch_artifact` = 指向不入仓工件（A128 的地盘）。
    """
    src = source_dirs(index)
    rows = collections.defaultdict(list)
    for m in LINE_REF.finditer(text):
        bucket, rel = name_bucket(m.group(1), index, src)
        if bucket is None:
            lines = (ROOT / rel).read_text(encoding="utf-8").split("\n")
            bucket = classify(lines, int(m.group(2)),
                              int(m.group(3)) if m.group(3) else None,
                              declared_lines(rel))
        elif bucket == "ambiguous":
            bucket = "ambiguous_line"
        elif bucket == "scratch":
            bucket = "scratch_line_ref"
        rows[bucket].append(m.group(0))
    for m in FILE_TOKEN.finditer(text):
        bucket, _ = name_bucket(m.group(1), index, src)
        if bucket is None:
            bucket = "file_unique"
        elif bucket == "ambiguous":
            bucket = "ambiguous_file"
        elif bucket == "scratch":
            bucket = "scratch_artifact"
        rows[bucket].append(m.group(0))
    counts = {key: len(val) for key, val in rows.items()}
    # 总数 = **该种语法自身的匹配次数**，不由桶加和：上一版让 `line_refs` 与 `file_tokens`
    # 共享 `ambiguous` 那一桶，同一批引用被同时计入两个总数，账本里的 303 被读成了 427。
    counts["line_refs"] = len(LINE_REF.findall(text))
    counts["file_tokens"] = len(FILE_TOKEN.findall(text))
    counts["bare"] = len(BARE_REF.findall(text))
    return counts, {key: sorted(val) for key, val in rows.items()}


class TestDeadAndAmbiguousReferences:
    """硬门禁：可证失效 = 0；架构文档的歧义引用 = 0（账本的歧义在 `CEILING` 里挂着）"""

    def test_no_reference_points_at_a_blank_comment_or_missing_line(self):
        index = build_index()
        bad = {}
        for doc in DOCS:
            _, rows = audit((ROOT / doc).read_text(encoding="utf-8"), index)
            dead = rows.get("dead_line", []) + rows.get("unresolved_source", [])
            if dead:
                bad[doc] = dead
        assert bad == {}, bad

    def test_architecture_doc_has_no_ambiguous_reference(self):
        """歧义分两桶（带行号 / 只有文件名），但架构文档两边都必须是 0"""
        _, rows = audit((ROOT / ARCH).read_text(encoding="utf-8"), build_index())
        assert rows.get("ambiguous_line", []) == []
        assert rows.get("ambiguous_file", []) == []


class TestL79CleanupActuallyHappened:
    """本轮清掉的具体例子逐条钉住（行为面而不是计数面：计数由 `TestRatchetsAreExact` 管）"""

    def test_the_erratum_note_keeps_its_numbers_as_prose_not_as_pointers(self):
        text = (ROOT / ARCH).read_text(encoding="utf-8")
        assert "**行号勘误（L53 关掉 A89 之后补，原文不删）**" in text
        for pointer in ("`cli.py:46`", "`cli.py:49`", "`cli.py:49-51`", "`cli.py:51-53`",
                        "`cli.py:46-53`"):
            assert pointer not in text, pointer
        assert "第 49-51 行" in text          # 历史读数还在，只是不再冒充定位引用
        # 口径立了就得一处不留：同一句话在 §3.27 正文与 §6 表的两次出现也在本轮改了形状
        assert "`cli.py` 当时第 46 行" in text
        # A129：勘误段自己那两个读数也位移了，文档必须明说「散文数字没有机械通道」
        assert "这就是 A129" in text

    def test_the_module_names_this_round_qualified(self):
        """`config.py` 一名两指在架构文档里已彻底消失，但**账本**里还挂着（棘轮分开记）"""
        index = build_index()
        _, arch_rows = audit((ROOT / ARCH).read_text(encoding="utf-8"), index)
        _, ledger_rows = audit((ROOT / LEDGER).read_text(encoding="utf-8"), index)
        assert "`config.py`" not in arch_rows.get("ambiguous_file", [])
        assert "`config.py`" in ledger_rows.get("ambiguous_file", [])

    def test_the_three_wrong_source_paths_the_old_bucket_was_hiding_are_fixed(self):
        """`unresolved_source` 这一档**为什么存在**：它上一版被 `scratch_missing` 吞掉

        三条都是**产品源码路径写错**（不是草稿名），改前不会让任何东西红。逐条钉住现在的
        正确形状，而不是只钉「计数为 0」—— 计数为 0 也能靠「把那几句整个删掉」达成。
        """
        text = (ROOT / LEDGER).read_text(encoding="utf-8")
        # ERNIE 后端住在 ernie.py，仓里从来没有 baidu.py（`baidu` 只是配置里的后端 id）
        assert "`augmentor/models/baidu.py`" not in text
        assert "`augmentor/models/ernie.py`" in text
        # 两处「订正 A86 文件引用」的历史记录保留原错引，但**不再用反引号冒充定位引用**
        assert text.count("`api/routes/config_ops.py`") == 0
        assert "api/routes/config_ops.py" in text
        assert "`api/routes/config.py`" in text


class TestRatchetsAreExact:
    """每条残余按**精确值**记账：变差要改回来，变好也要改常数并在账本留一句"""

    def test_each_document_matches_its_own_ceiling(self):
        index = build_index()
        for doc in DOCS:
            counts, rows = audit((ROOT / doc).read_text(encoding="utf-8"), index)
            for bucket, ceiling in CEILING[doc].items():
                assert counts.get(bucket, 0) == ceiling, (
                    f"{doc} 的 {bucket} 实测 {counts.get(bucket, 0)}，基线写着 {ceiling}。"
                    f"清账清出来的就改小 `CEILING` 并在 OPTIMIZATION_LOOP.md 的 L 行留一句；"
                    f"新写坏的就把它改回去。\n明细：{rows.get(bucket, [])[:12]}")

    def test_only_the_current_round_leaves_a_hash_placeholder(self):
        """**每一轮的提交哈希必须当轮回填**：忘了就当场红

        日志块开头写「`哈希待 L{n+1} 回填`」是允许的（自己那一轮的哈希还不存在），
        但下一轮必须把它填掉 —— 所以非 0 的只许有本轮这一条。实测 L58..L71 共 14 条
        从未回填（那是 A130 的历史欠账，补正文要用户拍），加本轮 = 15。
        """
        text = (ROOT / LEDGER).read_text(encoding="utf-8")
        found = HASH_PLACEHOLDER.findall(text)
        assert len(found) == PLACEHOLDER_CEILING, found[-3:]


class TestTheAuditIsNotSpinning:
    """反空转 + 判据可失败：这台机器必须证明它既能看见好引用，也能抓住坏引用"""

    def test_both_docs_yield_a_known_amount_of_refs(self):
        index = build_index()
        for doc in DOCS:
            counts, _ = audit((ROOT / doc).read_text(encoding="utf-8"), index)
            for bucket, floor in FLOOR[doc].items():
                assert counts[bucket] >= floor, (doc, bucket, counts[bucket], floor)

    def test_the_index_sees_both_config_modules_and_full_paths_are_unique(self):
        live, _ = resolve("config.py", build_index())
        assert live == ["api/routes/config.py", "augmentor/config.py"], live
        for one in live:
            solo, _ = resolve(one, build_index())
            assert solo == [one], (one, solo)

    def test_a_reference_to_a_blank_line_is_reported_dead(self):
        """造一份含死引用的假文档，普查必须抓到它（L76 立的规矩：判据不能永不开火）

        明细存的是**引用原文**（含反引号），因为失败信息要能被读者原样 grep 回文档。
        """
        lines = (ROOT / "augmentor/logging_setup.py").read_text(
            encoding="utf-8").split("\n")
        blank = next(str(i + 1) for i, line in enumerate(lines) if not line.strip())
        fake = f"举例：见 `logging_setup.py:{blank}` 与 `config.py` 两份。\n"
        counts, rows = audit(fake, build_index())
        assert rows["dead_line"] == [f"`logging_setup.py:{blank}`"], rows
        assert counts["ambiguous_file"] == 1, rows.get("ambiguous_file")

    def test_an_out_of_range_reference_is_reported_dead(self):
        _, rows = audit("见 `augmentor/logging_setup.py:999999`\n", build_index())
        assert rows["dead_line"] == ["`augmentor/logging_setup.py:999999`"], rows

    def test_a_wrong_source_path_is_hard_failure_and_a_bogus_top_dir_is_only_a_draft(self):
        """`unresolved_source` 这一档**两个方向**都要能红 —— 分桶判据不偏不倚

        只钉「写错的产品路径要红」会纵容另一种错：把任何未解析名字都判成硬失效，于是
        111 个 Temp 草稿名当场把门禁撑成永红。两半一起钉。
        """
        index = build_index()
        _, rows = audit("见 `augmentor/models/baidu.py` 与 `api/routes/config_ops.py`\n", index)
        assert rows["unresolved_source"] == ["`api/routes/config_ops.py`",
                                            "`augmentor/models/baidu.py`"], rows
        assert rows.get("scratch_missing", []) == [], rows

        _, rows = audit("见 `l99q/probe.py` 与 `tolus/foo.py`\n", index)
        assert rows["scratch_missing"] == ["`l99q/probe.py`", "`tolus/foo.py`"], rows
        assert rows.get("unresolved_source", []) == [], rows

    def test_a_temp_snapshot_copy_does_not_launder_a_wrong_source_prefix(self):
        """分桶**顺序**的用例：这一条在账本里真的存在，且改前藏在工件档

        `` `augmentor/api/routes/dataset_tools.py` `` 指的是产品路径，可产品树里根本没有
        `augmentor/api/`（真实文件是 `api/routes/dataset_tools.py`）；Temp 下有 HEAD 快照副本
        恰好同尾 ⇒ 先查工件会把它判成「指向不入仓工件」，永远不红。
        """
        live, scratch = resolve("augmentor/api/routes/dataset_tools.py", build_index())
        assert live == [], live                       # 产品树里没有
        assert len(scratch) >= 1, scratch             # Temp 快照里有：先查就会被它骗过
        _, rows = audit("见 `augmentor/api/routes/dataset_tools.py:117-127`\n", build_index())
        assert rows["unresolved_source"] == ["`augmentor/api/routes/dataset_tools.py:117-127`"]
        assert rows.get("scratch_line_ref", []) == []

    def test_the_hash_placeholder_regex_counts_debt_not_prose(self):
        """正则 vs `count("哈希待")`：账本里 26 : 15，散文**谈论**这件事的 11 次不是欠账"""
        sample = ("- **L80** `哈希待 L81 回填`\n"
                  "- **L79** `deadbee0`（当轮已回填）\n"
                  "  - 顺手记一句：本行的哈希待 L81 回填，numstat 也待量。\n")
        assert HASH_PLACEHOLDER.findall(sample) == ["- **L80** `哈希待 L81 回填`"]
        assert sample.count("哈希待") == 2      # 宽松口径会多算一条 ⇒ 判据必须按形状

    def test_the_totals_quoted_in_the_prose_are_the_current_measurement(self):
        """`MEASURED` 与真实现量必须相等：**注释与文档里的数不许靠回忆**

        这条是本轮回看段抓到的病：`docs/ARCHITECTURE.md` §3.33 写完后，同一轮内的判据改动
        让「40 处 / 114 处 / 10 例」三个数当场过期，而**没有任何用例能出声**（散文数字在
        守卫的可见范围之外，即 A129）。修法是把数字搬进常量、让用例去比对现量。
        """
        index = build_index()
        for doc in DOCS:
            counts, _ = audit((ROOT / doc).read_text(encoding="utf-8"), index)
            for bucket, want in MEASURED[doc].items():
                assert counts[bucket] == want, (doc, bucket, counts[bucket], want)

    def test_a_range_whose_start_is_blank_but_holds_code_is_not_dead(self):
        """区间档的**反向用例**：全段皆非代码才算死，起点歪一行不算（口径必须两边都钉）"""
        lines = (ROOT / "augmentor/logging_setup.py").read_text(
            encoding="utf-8").split("\n")
        blank = next(i for i, line in enumerate(lines) if not line.strip())
        while lines[blank + 1].strip().startswith("#") or not lines[blank + 1].strip():
            blank += 1
        fake = f"见 `augmentor/logging_setup.py:{blank + 1}-{blank + 2}`\n"
        _, rows = audit(fake, build_index())
        assert rows.get("dead_line", []) == [], rows
        assert rows["code_but_no_name_match"] == [
            f"`augmentor/logging_setup.py:{blank + 1}-{blank + 2}`"], rows


class TestLedgerSectionSkeleton:
    """账本的章节骨架是**逐字按序**的：标题被吃掉 / 改名 / 挪序 / 多插一节都当场红

    这一档补的是本账用散文记过三次、却始终没有机械通道的那个形状（L45 丢「；A63」、
    L46 丢 `## Backlog B` 标题、L78 丢 `## Backlog A` 标题）。前两次靠回读相邻行才发现，
    第三次一直躺到 L80 —— 因为「插一条进度行」的锚点换成 `0 occurrences` 才把它暴露出来。

    **为什么引用普查看不见它**：`audit()` 判的是反引号引用，A 表的 130+ 行一条没少，
    少的是它头上那一行 —— 一条标题既不是引用也不是表格，粗判据全绿。
    """

    def test_the_ledger_headings_are_the_frozen_five_in_order(self):
        text = (ROOT / LEDGER).read_text(encoding="utf-8")
        assert headings(text) == list(LEDGER_SECTIONS), headings(text)

    def test_the_eaten_backlog_a_heading_is_back(self):
        """L78 那次提交吃掉的具体一行，逐字钉回原位（不是「有个标题就行」）"""
        text = (ROOT / LEDGER).read_text(encoding="utf-8")
        assert "## Backlog A — 性能（含 file:line 与实测线索）" in text
        # 恢复后的形状：标题之后先是表格头，中间不插别的段落。
        # **锚点必须取整行标题**：账本里 `## Backlog A` 这个短串另有散文提及（L80 的进度行），
        # 按短串切会切到散文那一处，测出来的就不是章节正文。
        body = text.split("## Backlog A — 性能（含 file:line 与实测线索）", 1)[1]
        assert body.startswith("\n\n| # | 位置 | 问题 | 量级 |"), repr(body[:60])

    def test_the_skeleton_guard_fires_when_a_heading_is_eaten_or_added(self):
        """判据可失败性的两个方向（承 L79 纪律 (f)）：少一行要红，多一行也要红

        只钉「少标题会红」等于把判据写成「标题集合的子集检查」，下一轮谁插一节
        `## 附录` 把它撑歪，这条守卫照样绿。
        """
        text = (ROOT / LEDGER).read_text(encoding="utf-8")
        assert headings(text) == list(LEDGER_SECTIONS)

        dropped = text.replace("## Backlog A — 性能（含 file:line 与实测线索）\n", "")
        assert headings(dropped) != list(LEDGER_SECTIONS)
        assert "## Backlog A — 性能（含 file:line 与实测线索）" not in headings(dropped)

        added = text + "\n## 附录 — 临时\n"
        assert headings(added) != list(LEDGER_SECTIONS)
        assert len(headings(added)) == len(LEDGER_SECTIONS) + 1

    def test_a_third_level_heading_is_not_part_of_the_skeleton(self):
        """提取口的边界：`###` 属正文，不参与骨架判据（架构文档那种层级不该把这里撑红）"""
        sample = "## 已完成\n\n### 小节\n\n#### 更小的节\n"
        assert headings(sample) == ["## 已完成"], headings(sample)


#: 「已完成」里的进度行与文末的日志块标题（L80 现量）。两侧都按**行首形状**提取，
#: 所以正文里谈论它们的散文不算数（与 `HASH_PLACEHOLDER` 同一条教训）。
PROGRESS_LINE = re.compile(r"^- \[[ x]\] \*\*L(\d+)\*\*", re.MULTILINE)
LOG_BLOCK = re.compile(r"^- \*\*L(\d+)\*\* ", re.MULTILINE)


class TestProgressLineAndLogBlockArePeeled:
    """A135 的机械答案：**每轮写两遍自己**（「已完成」一行 + 文末一块日志），那就两侧对账

    **为什么立这一条**（L80 现量）：L79 只写了文末日志块、漏了「已完成」的进度行，而这件事
    当时没有任何机械通道能发现 —— 块标题那一侧早有 `PLACEHOLDER_CEILING` 数着（15 条精确相等），
    进度行那一侧**一个数都没有**。补写 L79 那行时才发现：两副名单是同一条纪律的两半。

    **口径**：判据是**号集合的包含**而不是行数相等 —— 一轮可以分批提交（实测 L12 在「已完成」
    里有两条：`perf(api)` 与 `perf(statistics)`），按行数比会把这条合法形状判成缺陷。
    反向那一维由 `test_a_second_batch_in_one_round_is_legal` 钉住。
    """

    def _both(self):
        text = (ROOT / LEDGER).read_text(encoding="utf-8")
        return PROGRESS_LINE.findall(text), LOG_BLOCK.findall(text)

    def test_every_log_block_has_a_progress_line(self):
        """硬门禁（A135 的病理本体）：文末有块、「已完成」没行 ⇒ 当场红"""
        prog, log = self._both()
        assert sorted(set(log) - set(prog), key=int) == [], (
            "这些轮次写了文末日志块却没有「已完成」进度行："
            f"{sorted(set(log) - set(prog), key=int)}")

    def test_the_progress_only_rounds_are_the_measured_three(self):
        """反向残额精确相等：L1 / L2 / **L12** 只有进度行、没有文末日志块

        L1/L2 是因为 `## 循环日志` 从 L3 起；**L12 是 A135 的另一种形状** —— 那一轮在「已完成」
        里写了两条（分批提交），却整轮没写日志块，本守卫建立之前这件事无声。
        写成 `{1, 2, 12}` 而不是「≤」：下一轮谁给 L12 补了日志块，这里同样红 ⇒ 逼他在账上留一句。
        """
        prog, log = self._both()
        assert set(prog) - set(log) == {"1", "2", "12"}, sorted(
            set(prog) - set(log), key=int)

    def test_the_guard_fires_in_both_directions(self):
        """判据可失败性（承 L79 纪律 (f)）：摘掉进度行要红，摘掉日志块也要红

        **本轮号从现量取，不手抄**（L81 改）：上一版把 `- [x] **L80**` 写死在断言里，
        于是下一轮它测的是**别人那一轮** —— 与 `PLACEHOLDER_CEILING` 每轮必改同族，但那
        个常数改漏会当场红，这个改漏只会悄悄测错对象（真事实配假因果，纪律 (k)）。
        """
        text = (ROOT / LEDGER).read_text(encoding="utf-8")
        last = max(PROGRESS_LINE.findall(text), key=int)
        prev = str(int(last) - 1)
        head = text.split(f"- [x] **L{last}**", 1)
        assert len(head) == 2, f"L{last} 进度行不在场，这一档就没有取证对象"
        without_line = head[0] + head[1].split("\n\n", 1)[1]
        prog, log = PROGRESS_LINE.findall(without_line), LOG_BLOCK.findall(text)
        assert last not in prog and last in log           # 摘进度行 ⇒ 硬门禁那一侧红

        block = f"- **L{prev}** "
        marked = [ln for ln in text.splitlines() if ln.startswith(block)]
        assert len(marked) == 1, marked
        without_block = text.replace(marked[0], f"- **L{prev}x** ", 1)
        prog, log = PROGRESS_LINE.findall(text), LOG_BLOCK.findall(without_block)
        assert prev in prog and prev not in log           # 摘日志块 ⇒ 反向那一侧看到差集

    def test_a_second_batch_in_one_round_is_legal(self):
        """反向钉住形状：**重复的进度行不是缺陷**（一轮分批提交是本仓常态）

        这一档防的是「下一轮有人把集合比较改成行数比较」：改完之后 L12 那两条
        （`perf(api)` + `perf(statistics)`）会当场红，而那是一次正当的交付。
        """
        prog, _ = self._both()
        dupes = sorted({n for n in prog if prog.count(n) > 1}, key=int)
        assert dupes == ["12"], dupes


#: ---- L81：账本的「表行形状」与「轮次顺序」两副骨架 ------------------------------
#:
#: **为什么这一族值得机械化**：L80 一轮里同一形状犯了两次（吃掉 A124 行的行首、把 L80 块
#: 插进 L79 块肚子里），两次都不是靠回读发现的 —— 第一次靠 Edit 报 `0 occurrences`，第二次
#: 靠 `grep -n` 顺手撞破。散文纪律「不许拿邻行前缀当锚点」拦不住下一次，而这两次的后果
#: **都能被行首形状抓到**：被吃掉行首的表行不再匹配竖线行的号形状，插错位置的块让轮号
#: 序列不再递增。
#:
#: **为什么不做成「对 HEAD 比删除数」**（L80 回看 ① 的原提法）：那种判据要看 git 历史 ——
#: 提交之后它永久成立、未提交时又会把同分支**其他 agent 的提交**算进来 ⇒ 一个只在「我刚
#: 写完」那一刻有意义的断言不是守卫（不可复现）。替代档 = 现量常数 + 两条注入档，见
#: `test_a_dropped_row_prefix_is_caught`；本轮那笔删除数对账改由收尾时的 `git diff --numstat`
#: 现量并在账上留字（一次性的取证，不伪装成常驻守卫）。
BACKLOG_ROW = re.compile(r"^\| (?:~~)?([AB]\d+)(?:~~)? \|")
BACKLOG_HEADS = ("| # | 位置 | 问题 | 量级 |", "|---|------|------|------|",
                 "| # | 内容 | 证据 | 量级 |", "|---|------|------|------|")
#: Backlog A 现量（`Temp/l81q/skeleton2.py` + L81 新立 A137 之后）：138 条竖线行 =
#: 2 行表头 + 136 条数据行，号集 = 1..137 减 `{52}`（A52 从未存在，是本仓唯一一次跳号）。
#: **L82 回填**：**143 条竖线行** = 2 表头 + 141 条数据行，号集 = 1..142 减 `{52}` —— 新增的
#: 5 条数据行就是本轮新立的 A138–A142（跳号集合未变 ⇒ 本轮没有吞行、也没有补回 A52）。
#: **L84 回填**：**144 条竖线行** = 2 表头 + 142 条数据行，号集 = 1..143 减 `{52}` —— 只增
#: A143 一行（关闭 A133 时按「本行不假装做完」那句立的），跳号集合仍未变。
#: **L85 回填**：**147 条竖线行** = 2 表头 + 145 条数据行，号集 = 1..146 减 `{52}` —— 新增
#: 三行全部由本轮三把普查尺子产出（A144 断点增量的 O(N²)、A145 流式一次请求两趟解析、
#: A146 `compare` 外溢的相似度代价），跳号集合第三次未变。
#: **L86 轮内（未回填）**：**149 条竖线行** = 2 表头 + 147 条数据行，号集 = 1..148 减 `{52}`
#: —— 增两行：A147（单条 JSON 值形状 `total_input=0` 而 `processed=1` 的既有分叉，本轮只立案
#: 不动手）与 A148（「已关闭行的历史定位引用必随代码腐坏」这一族，见 `MEASURED` 上方那段），
#: 跳号集合第五次未变。
#: **L87 轮内（未回填）**：**158 条竖线行** = 2 表头 + 156 条数据行，号集 = 1..157 减 `{52}`
#: —— 新增九行 A149–A157（上传体字节闸、starlette spool 的管不到那一层、写面节级漏网、
#: `save_config` 吃注释、`dead_line` 桶的盲区、默认线程池形状、500 档回显服务器状态、
#: (路径, mtime_ns) 缓存的一 tick 失效窗口、一条绝对墙钟预算在并发负载下假红），
#: 跳号集合第六次未变。**本轮这两条判据各红过一次，都是排版级自伤**：插 A149–A153 时把一条
#: 空行留在了 A148 与 A149 之间 ⇒ GFM 表格被劈成两张，`test_tables_have_header_separator`
#: 与这里三条（行数 / 号集 / 掉前缀）同时报；另一条是写完文末日志块才想起「已完成」进度行
#: 还没写 ⇒ `test_every_log_block_has_a_progress_line` 红，那正是 A135 的病理本体。
#: **第三次自伤是本轮最新的一种形状，也是本文件第一次被「行内前缀锚点」伤到**：往 A155 之后
#: 插两行时，我的 `old_string` 只取到 A155 那一行的**前缀**（按字面截断在 120 字符处），于是
#: Edit 把行头换成两整行、A155 的行身被粘到新写的 A157 末尾 ⇒ 表里少一行、多一条畸形行，
#: 而 Edit 只报「1 replacement」。**发现方式不是门禁**：是我打印行首行尾时看见 A157 的尾巴读起来
#: 不像 A157。修法是带闭合断言的脚本切片（按唯一分界串拆开、复原行头、断言行数 +1、断言三个
#: 行号各出现一次），不是再来一次手写 Edit。**升为口径候选**：插入型 Edit 的 `old_string`
#: **必须以整行为单位**（含行首 `| A\d+ |` 与行尾 `| 定级 |`），截断到半行的锚点在竖线表格里
#: 一定吃掉行头 —— 这是 (w) 那一族「工具不报错的损害」里最安静的一种。
#: **L88 回填（+2 行，(w) 口径第一次落在脚本里而不是 Edit 里）**：A158（保住 mtime 的写方让
#: 陈旧窗口无界）与 A159（`save_config` 的双趟 YAML 解析，判负不动手）两行由
#: `Temp/l88q/ledger_fill_1.py` 按**整行锚点 + 行数断言**插入，本轮三条骨架判据一次未因形状
#: 破损而红（红的是「常数尚未回填」那一类，与本档无关）。跳号集合第七次未变。
BACKLOG_A_LINES = 160
BACKLOG_A_MAX = 159
BACKLOG_A_MISSING = {52}
#: Backlog B 现量：11 条竖线行 = 2 表头 + 7 条可编号 + 2 条带角标（B3① / ~~B3②~~）。
BACKLOG_B_LINES = 11
BACKLOG_B_SUBSCRIPT = ("B3①", "B3②")
#: `## 循环日志` 从 L3 起（L1/L2 只有进度行），L12 也只有进度行 ⇒ 块号 3..本轮 减 `{12}`。
LOG_BLOCK_FIRST = 3
LOG_BLOCK_HOLES = {12}


def section_body(text, title):
    """按**行首**取某节正文。

    用 `str.index` 取会拿到错的区间：账本里 `## Backlog A` 这个短串另有散文提及
    （L80 进度行、A 节标题自身），第一版探针就是这么把 B 表读成了 A 表。
    """
    marks = [(m.start(), m.group(0)) for m in re.finditer(r"^## \S.*$", text, re.MULTILINE)]
    i = next(k for k, (_, t) in enumerate(marks) if t.startswith(title))
    return text[marks[i][0]:(marks[i + 1][0] if i + 1 < len(marks) else len(text))]


class TestBacklogAndRoundSkeleton:
    """A135 的孪生：**同一轮里第二次「吃掉邻行行首」之后，这一族不再靠人眼**"""

    def test_every_pipe_line_in_backlog_a_is_a_numbered_row(self):
        """硬 0：A 表里除两行表头外，每一条竖线行都必须带 `A<号>`（行首被吃 ⇒ 当场红）"""
        body = section_body((ROOT / LEDGER).read_text(encoding="utf-8"), "## Backlog A")
        rows = [ln for ln in body.splitlines() if ln.startswith("|")]
        assert len(rows) == BACKLOG_A_LINES, len(rows)
        unparsed = [ln[:70] for ln in rows
                    if ln not in BACKLOG_HEADS and not BACKLOG_ROW.match(ln)]
        assert unparsed == [], unparsed

    def test_backlog_a_ids_are_the_measured_set_without_dupes(self):
        """号集合精确相等：跳号、重号、被吃掉一整行都到这里就红"""
        body = section_body((ROOT / LEDGER).read_text(encoding="utf-8"), "## Backlog A")
        ids = [int(m.group(1)[1:]) for m in map(BACKLOG_ROW.match, body.splitlines()) if m]
        assert len(ids) == len(set(ids)), sorted({n for n in ids if ids.count(n) > 1})
        assert set(ids) == set(range(1, BACKLOG_A_MAX + 1)) - BACKLOG_A_MISSING, \
            sorted(set(range(1, BACKLOG_A_MAX + 1)) - set(ids))

    def test_backlog_b_keeps_its_numbered_rows_and_the_two_subscripted_ones(self):
        """B 表两档分开数：可编号的 7 行 + 带角标的 2 行（角标行**不该**被号判据吞掉）

        这一档顺手钉住「`B3①`/`B3②` 是两行而不是一行」：把它们合并成一行会同时改掉两个
        交付口径（反向转换边 vs tsv 双边），本守卫会因两条形状同时消失而红。
        """
        body = section_body((ROOT / LEDGER).read_text(encoding="utf-8"), "## Backlog B")
        rows = [ln for ln in body.splitlines() if ln.startswith("|")]
        assert len(rows) == BACKLOG_B_LINES, len(rows)
        ids = [m.group(1) for m in map(BACKLOG_ROW.match, rows) if m]
        assert ids == ["B1", "B2", "B4", "B5", "B6", "B7", "B8"], ids
        others = [ln for ln in rows if ln not in BACKLOG_HEADS
                  and not BACKLOG_ROW.match(ln)]
        assert len(others) == 2, others
        assert [s for s in BACKLOG_B_SUBSCRIPT
                if not any(s in ln for ln in others)] == [], others

    def test_round_numbers_increase_by_one_with_the_measured_hole(self):
        """轮次顺序：块号必须 3..本轮 **逐号递增且缺号只有 L12**（插错位置 ⇒ 红）

        L80 那次「L80 块插进 L79 块肚子里」如果被这条抓到，靠的是**同一轮号出现两次**
        或**号序逆排**；两种形状都在这里，不需要读散文。
        """
        text = (ROOT / LEDGER).read_text(encoding="utf-8")
        nums = [int(n) for n in LOG_BLOCK.findall(text)]
        assert nums == sorted(nums), [
            (a, b) for a, b in zip(nums, nums[1:]) if b <= a]
        assert len(nums) == len(set(nums)), sorted(
            {n for n in nums if nums.count(n) > 1})
        assert nums[0] == LOG_BLOCK_FIRST
        assert set(range(LOG_BLOCK_FIRST, nums[-1] + 1)) - set(nums) == LOG_BLOCK_HOLES

    def test_a_dropped_row_prefix_is_caught(self):
        """判据可失败性（承 L79 纪律 (f)）：把某条表行的行首吃掉，判据必须红

        注入的是 L80 现场那一次的形状：`| A136 | …` 变成 `A136 | …`（前缀被上一行的插入
        吞掉）。另一维（块插错位置）由 `test_the_guard_fires_when_a_block_moves_ahead` 钉。
        """
        text = (ROOT / LEDGER).read_text(encoding="utf-8")
        mangled = text.replace("\n| A136 | ", "\nA136 | ", 1)
        assert mangled != text, "A136 行不在预期形状，注入无效 ⇒ 这一档没有取证对象"
        rows = [ln for ln in section_body(mangled, "## Backlog A").splitlines()
                if ln.startswith("|")]
        assert len(rows) == BACKLOG_A_LINES - 1        # 少一条竖线行：它已不成行
        survivor = [ln for ln in rows if "A136" in ln[:12]]
        assert survivor == []                          # 且没有任何行还带着那个号

    def test_the_guard_fires_when_a_block_moves_ahead(self):
        """反向注入：把 L79 的块标题挪到 L80 之后 ⇒ 递增那一档必须抓到逆排

        L80 现场的第二犯就是这个形状（新块插进了上一轮的块肚子里，事后靠 `grep -n` 撞破）。
        这里只搬**标题行**而不搬整块 —— 判据读的就是标题行上的号，搬标题足够复现病理。
        """
        text = (ROOT / LEDGER).read_text(encoding="utf-8")
        base = [int(n) for n in LOG_BLOCK.findall(text)]
        assert base == sorted(base), "现量本身就不递增 ⇒ 这一档没有对照"

        cut = re.sub(r"^- \*\*L79\*\* .*$\n", "", text, count=1, flags=re.MULTILINE)
        assert len(LOG_BLOCK.findall(cut)) == len(base) - 1
        blocks = list(re.finditer(r"^- \*\*L\d+\*\* .*$", cut, re.MULTILINE))
        tail = blocks[-1]
        after = [int(n) for n in LOG_BLOCK.findall(
            cut[:tail.end()] + "\n\n- **L79** （被挪到最后一块之后）" + cut[tail.end():])]
        assert len(after) == len(base), (len(after), len(base))  # 号没丢，只是序变了
        assert after != sorted(after), after
        assert [a for a, b in zip(after, after[1:]) if b <= a] == [base[-1]]
