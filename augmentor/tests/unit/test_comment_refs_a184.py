# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A184 入库：**全部**注释与自有文档里的「定位引用」都要真的指得到东西

**为什么单独立一支，而不是把 L79 那把尺子调宽**：L79 的守卫只有两条正则，而它们都写死了
`.py` 后缀 ⇒ 它对「非 Python 载体上的引用」是**结构性失明**的。本轮普查里第一条真失效就是这么
漏的：出厂配置模板里那两个重试键的行号被两轮改动搬走之后，指向 YAML 的行号引用落在的是
**空行**，而仓库里任何一条已入库的判据都读不到它 —— 不是没跑，是那个形状压根不进正则
（这一格由 `TestCensusIsProven.test_the_rulers_own_regex_still_cannot_see_a_yaml_line_ref`
钉住，不是本文档的一句话主张）。所以这一支不是 L79 的副本，而是它看不见的**那一整个载体族**
（YAML、Markdown、ts/tsx/js/vue 注释）的补集守卫；两支的扫描面交集只有两份文档，而桶的判定
（名字怎么解析、区间怎么才算可证失效）全部从 L79 import，不留第二份实现。

**三条口径（A184 原文说「要先定这三条」，本轮拍定）**：

1. **扫描面** = 产品树里 Python 文件的**注释 + docstring**（tokenize 的 COMMENT 加 AST 的
   docstring 区间，正文代码不参与）、web 那侧 ts/tsx/js/jsx/vue 的注释行、以及全部 Markdown。
   跳过 Temp / .backups / bak / node_modules 与各类缓存目录 —— 草稿与依赖不是案面。
2. **索引** = 全部扩展名，不再只收 Python 文件。这一步是判据成立的前提：TS 源文件名在旧索引里
   压根不存在，于是「注释里写了一个不存在的 TS 文件」这件事今天没有任何东西会红。
3. **指向 Temp 的引用沿用「指向工件」而不是「失效」** —— 与 L79 同口径，单独成档、只当棘轮。
4. **符号主张档（`名字()` 这种形状）的「宇宙」只收代码面**，不收注释、docstring、字符串样本
   与 Markdown。这一条是注入实测逼出来的：第一版宇宙是「全文本扫标识符」，于是往一份真文件的
   docstring 里写一支根本不存在的 API 主张，守卫**全绿** —— 那句主张自己把名字喂给了宇宙，
   假主张只要被写下来就自动变成真名字。改成代码面之后同一支注入立刻红（报告见 Temp 下那支注入
   探针）。代价一并写清：点号右边那一截不验证，只按「点号左边打得开一个真模块」放行外部主张
   （`re._compile` 与 `str.isspace` 这类标准库形状本仓代码里不会调用，逐条 getattr 既慢又要
   在测试里 import 任意模块）。
5. **内容维（A331，本轮补）**：`dead_line` 只问「落点在不在代码跨度里」，问不到「落点对、内容错」。
   本轮普查现量 **12 条**这种引用（搜索那侧的 fuzzy 函数已经搬到 183 行之下、日志装配的入口搬到
   31 行之下，而两把尺子全程绿 ⇒ 见 `anchor_state`）。判法取本仓自己写主张的那一种形状：
   引用**紧跟的后括弧**、或**所在的「不含嵌套」外括弧**里，**第一个**反引号跨度必须是裸标识符，
   且它是目标文件里某个 def/class 的名字 ⇒ 落点跨度必须**完整落在**该符号的某一个跨度之内，
   否则进 `dead_anchor`（硬 0 档）。
   为什么把归因收成「括弧首那一个名」：探针逐档量出来的，不是先验。整行归因把 3 229 个反引号
   标识符平摊给同一行的每一条引用，产品注释面「全部命中」只剩 15 支；句内归因又把一个括弧里讲
   三件事的句子判成失配（点名 save_config / ValueError / grep 那 **6 条全是假红**）。
   一支引用只有一个主语，所以这里判得起硬 0。
   三条**判不到**的边界（不是「判错」，是压根不进主张档）：点号形状的主张（类名加点加方法名）、
   括弧首是散文的主张、不带文件名的裸行号。本轮按内容重钉时人眼撞见三处这类假句（架构文档里
   那两个类名归属行号、本账 A105 那条的 95、A76 那个裸调用点行号），一并按现量改了引用，
   但尺子仍然判不到它们 ⇒ 另立 A333。

**两条本轮新拍的口径**（L79 没面对过，因为它只读两份 augmentor 相对书写的文档）：

- **系别要问两次**：注释与文档同时被「augmentor 目录内」和「外层仓库根」两种读者读，所以把
  产品路径多包一层 augmentor 前缀的写法**不是缺陷**。判法是同一条引用按两系各问一次、再按
  优先级合并（见 `reference_state`），而不是硬选一系 —— 硬按 augmentor 相对判会把成百条真引用
  算成死链，硬按仓库根判则反过来把裸产品路径算成死链。**这一条让注释面比 L79 的两份文档更宽**
  （那边第 5 条口径按 augmentor 相对一系判），是刻意的不对称：那两份文档自己规定了书写系别，
  注释没有。两支守卫对同一批句子可以一严一宽，但**不存在洗白** —— 更严的那支会先把多系别的
  写法挡在文档门外。
- **父目录有没有被跟踪的文件**是「坏链接」与「示例路径」的分界：文档目录里全是真文件，所以
  指向该目录一个不存在的配置文档名**本该存在** ⇒ 判死；而示例路径的父目录一个被跟踪文件都
  没有 ⇒ 单独成档 runtime_ns，判它死等于把散文当契约。用 git 的跟踪集合而不是磁盘，因为磁盘
  上还站着不入仓的用户数据：按「目录里有文件」判，本轮第一版实测把 414 条示例路径全判成死链。

**判据一律「只降不升、精确相等」**（承 L76 / L77 / L79 的棘轮口径）：`CEILING` 每条都是等值断言，
所以「顺手清掉一条歧义」同样会让用例红 —— 那是设计，它逼着清账的人留一句「谁清的、清完剩多少」。
可证失效四档（dead path / dead line / dead symbol / dead anchor）硬 0，不进棘轮、不许混桶。

**本文件的注释与文档里不写反引号路径引用**：它自己就属于扫描面，写一条等于给自己造一条要维护
的引用（本轮普查数出来的 15 条失效里有 4 条正是守卫自己的案面写的）。需要引用形状的测试数据
一律用 `tick()` 在运行期拼，字面量在源码里永远不成形。
"""
import ast
import builtins
import collections
import functools
import importlib.util
import io
import os
import pathlib
import re
import subprocess
import sys
import tokenize

import pytest

from tests.unit.test_doc_line_refs_l79 import LINE_REF, name_bucket, require_authoring_tree

ROOT = pathlib.Path(__file__).resolve().parents[2]
#: 仓库根与「汇总到仓库根的子项目文档」目录（2026-10-14 从 `augmentor/docs/` 迁出）。
#: 索引要伸到这一层，否则产品码与账本里的 docs/augmentor/<DOC>.md 会集体翻档。
#: （本行不写反引号：本文件属于扫描面，成形的引用一律走 `tick()`，见其 docstring）
REPO = ROOT.parent
DOCS_DIR = REPO / "docs" / "augmentor"

IDX_SKIP = frozenset({".git", "__pycache__", ".pytest_cache", "node_modules",
                        ".backups", "dist", "coverage"})
# `.backups` = 套件自写的运行时快照目录（gitignored，不入仓）：索引收它会让测试套件
# 每跑一次全量门禁就为下一次普查造出新的文件名候选（`index.json`/`snap.json`），
# 引用被翻进 ambiguous 欠账档、棘轮自伤（L145，B215）。SCAN_SKIP 一直有它，索引侧对齐。
# `dist` = 前端构建产物（同样 gitignored）：跑没跑过 npm run build 会让文件数与引用数
# 跟着变，而 CI 克隆里没有它——与 `.backups` 同一条「读数不能被本机状态改写」理由。
# `coverage` = 覆盖率报告目录（`npm run test:coverage` 生成，gitignored）：同一理由。
# 2026-10-27 实测：作者树跑过一次覆盖率后有 3 只 `web/coverage/*.js` 被算进语料，
# `files_seen` 627 而干净克隆 624 —— 读数跟着本机跑没跑过覆盖率变，就是这条要挡的。
SCAN_SKIP = IDX_SKIP | {"bak", "htmlcov", ".venv", "venv", ".mypy_cache"}
#: 顶层（ROOT 直接子级）的未入仓用户数据目录，仅按「恰好是 ROOT 的子目录」逐层判，
#: 不误伤嵌套的 `augmentor/data` 包（包里 `image.py`/`cleaner.py` 是入仓产品码，
#: 被 `web/src/types/api.ts` 等注释引用，必须留在索引里才能解析成 live）。
#: 若用 IDX_SKIP 那种「任意深度的同名目录」跳过法，会把 `augmentor/data` 包也抹掉，
#: 于是 `augmentor/data/image.py` 翻成 dead_path（L178 首版因此红 2 条，已回退改此法）。
TOPLEVEL_SKIP = frozenset({"data", "bak"})

#: 运行期输出目录：这些名字属于「训练/推理跑出来的东西」，不是仓内命名空间。
#: 引用它们是散文里的示例路径（`checkpoints/…`），既不是死链、也不该落进「不入仓草稿」
#: 两档 —— 按**名字**判，不看磁盘上有没有这个目录。理由与 `.backups` / `dist` / `coverage`
#: 同源：作者树跑过一次训练就会多出 `checkpoints/`，而干净克隆没有；若按磁盘判，
#: 同一条引用在 CI 落 `scratch_missing`、在作者树落 `runtime_ns`，`runtime_ns` 这条棘轮
#: 就永远只能在作者树上跑（2026-10-27 CI 首轮红正是这个形状：1 vs 0）。
RUNTIME_DIRS = frozenset({"checkpoints"})
SRC_EXT = frozenset({".py", ".ts", ".tsx", ".js", ".jsx", ".vue"})
DOC_EXT = frozenset({".md", ".yaml", ".yml", ".toml", ".sql", ".ini", ".cfg",
                     ".json", ".txt"})
ALL_EXT = SRC_EXT | DOC_EXT
#: 进扫描面的扩展名：注释族全收，文档只收 Markdown —— YAML/JSON 里那些「像路径的串」绝大多数
#: 是数据而不是给读者指路的话，收进来只会淹掉信号
SCAN_EXT = SRC_EXT | {".md"}
#: 账本与架构文档的散文里全是「当年那条 API 主张后来作废」这类句子，符号档只在产品注释与
#: README 上用（案面与判面分开量，承 A183 那条读法）
NO_SYM_SCAN = frozenset({"OPTIMIZATION_LOOP.md", "docs/augmentor/ARCHITECTURE.md",
                         "OPTIMIZATION_PLAN.md"})

_EXT_ALT = "|".join(sorted(e[1:] for e in ALL_EXT))
#: 定位引用：反引号里「带扩展名的路径」，可跟 `:行号` 或 `:起-止`
REF = re.compile(r"`([A-Za-z0-9_/\\.\-]*[A-Za-z0-9_]\.(?:" + _EXT_ALT +
                 r"))(?::(\d+)(?:-(\d+))?)?`")
_SYM = r"[A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*"
#: 符号档只认「反引号闭合后紧跟左括号」这一种形状 —— 那是 API 主张。第一版把散文里的名字也
#: 当符号，普查当场数出 220 条假缺陷：逻辑运算符、文件名、键名全进了死档。
SYM_CALL = re.compile(r"`(" + _SYM + r")\(\)`")
SYM_PAREN = re.compile(r"`(" + _SYM + r")`\(")
#: 内容维（A331 / L245）：引用**紧跟的后括弧**——`\s*[（(] … [）)]`，中间不许再套括弧
TAIL_PAREN = re.compile(r"\s*[（(]([^（()）]*)[）)]")
#: 主张括弧的**正文开头**必须正好是一个裸标识符的反引号跨度才算点了名。用 `match` 而不是
#: `search`：括弧里先写散文再写名字（「新增 X，见某行」那种）不算主张，见口径 5。
BARE_TICK = re.compile(r"`([A-Za-z_][A-Za-z0-9_]*)`")
CLOSE_PAREN = re.compile(r"[）)]")
#: 注释标记按**被指向文件**的语言取：`#` 在 Markdown/YAML 里是内容，只有 Python 里才是注释。
#: 这一条是本轮 selftest 抓出来的（第一版照抄 L79 的 `#`，于是把 README 的标题行读成注释）。
COMMENT_MARKERS = {".py": ("#",), ".ts": ("//", "*"), ".tsx": ("//", "*"),
                   ".js": ("//", "*"), ".vue": ("//", "*")}

#: 现量基线（扫描面 = 476 份文件的注释与自有文档、3 794 条定位引用、326 条符号主张；
#: 逐档读数就是下面这份常量，动文件布局或动案面文字都要重跑普查再改这里）。**只降不升**。
#: 可证失效四档硬 0（dead_path / dead_line / symbol_dead 与 **L245 新加的 dead_anchor**）；
#: 其余四档 = 已记账的歧义欠账 + 运行期示例路径 + 指向不入仓工件
#: （scratch 两档合起来就是 A128 那块地盘的注释面读数）。
#: **2026-09-28 数据集搬家那一趟的三格位移**（四种「正文 × 布局」组合都现量跑过，逐格归因）：
#: `ambiguous` 328 → **327** = 案面改动 —— 删掉的那条指向工作目录根的整路径引用，在旧布局下
#: 因「工作目录根 / 嵌套包目录」两个候选而落在歧义档；`runtime_ns` 31 → **30** 与
#: `scratch` 728 → **729** 是同一件事的两面 —— 那份套件产物从工作目录根移进 Temp 之后，
#: 它的引用从「运行期命名空间」翻成「指向不入仓工件」。`live`、`scratch_missing` 与三档硬 0
#: 在四种组合里一字未动（搬家没有造出任何新失效引用，这是本轮要的正面证据）。
CEILING = {
    "dead_path": 0,
    "dead_line": 0,
    # 内容维（A331 / L245）：立档时案面现量 12 条红（逐条按内容重钉之后回 0），见本账 L245 块
    "dead_anchor": 0,
    "symbol_dead": 0,
    # 2026-10-22 重钉 300 → **463**（+163）。新增成员几乎全在两卷账本的反引号形状里：
    # 实测 OPTIMIZATION_LOOP.md 240 条 / OPTIMIZATION_LOOP_2.md 128 条 / 其余散在测试与
    # 产品码注释。主流是**短哈希**（`82f4a4cab3e86…` 这类 9~10 位）与账目编号散名
    # （`A283` 等）：一个短哈希若与两个以上文件同名（不同长度的哈希共享前缀），
    # `resolve()` 就交回多条 live ⇒ 落歧义档。属判据的已知粗糙处（拿哈希当文件名问索引），
    # 修法要动 `REF` 形状而不是改数字，另立待办。
    # 已用「stash 全部本地改动后重跑普查」对照：HEAD 读数同为 463 ⇒ 这次位移不是本轮
    # 改动造成的，是更早的账本批累积的存量（棘轮档语义要求涨也记账，记的就是这一条）。
    # 2026-10-27 A184 增量重钉 463 → **497**（+34）。扫描面接上 docs/augmentor 的 14 份
    # Markdown 后先量到 +35：config.py / pipeline.py / api.ts 这类裸文件名在 augmentor 与
    # docs 两份语料里都能数出多份候选，按既有口径落「歧义」而不是「死链」；修 15 条死引用
    # 时其中 1 条（原 docs/ARCHITECTURE.md）改写后仍有多候选，故净 +34。逐条判定见
    # 10-27 计划 §2 的增量探针。
    # 2026-10-30 重钉 497 → **500**（+3）。AU-P1 异步引擎等提交（124e3d759 / b81354d13 /
    # 6ffdedc79）给 OPTIMIZATION_LOOP.md 补记轮次账目时新增的短哈希/账目编号散名在
    # augmentor 与 docs 双语料里有多候选，按既有口径落歧义档。已用「本地仅未入语料的
    # 台账文档 + 重跑普查」核对：位移来自 HEAD 存量（这些提交后未重钉），非本轮工作区改动。
    "ambiguous": 500,
    # 2026-10-22 重钉 0 → **1**：唯一一条是 OPTIMIZATION_LOOP_2.md A315 那段散文举例的
    # 运行期产物路径（checkpoints/…，反引号形状）——它正是本档的语义（"没有 ⇒ 单独成档
    # runtime_ns，判它死等于把散文当契约"）。
    # 2026-10-27 订正判据：原先按「父段在源码目录集里」推，而那个目录只有跑过训练的作者树
    # 才存在 ⇒ CI 干净克隆把同一条引用记成 `scratch_missing`（首轮 CI 红：1 vs 0）。
    # 现在改由 `RUNTIME_DIRS` 按**名字**判，作者树与干净克隆都读到 1。
    # 同族先例：data/* 示例路径曾在本档积过 30 条。
    # （本注释刻意不用反引号复述那条路径 —— 反引号形状会被普查收成新引用，
    #  第一版就这么自己给自己造了第 2 条。）
    "runtime_ns": 1,
    #: `scratch` / `scratch_missing` **不进 CEILING**：两格的分界线是 Temp/ 里有没有同名
    #: 未入仓工件，而 Temp/ 是取证草稿目录、内容随各轮脚本增删变化 —— 同一批引用在作者树
    #: 与干净克隆里会落进不同的格（本机实测 0 / 1365，作者树 731 / 634），单格钉死就是拿
    #: 环境噪声撞常数。判据改为只钉**两档之和**（见 SCRATCH_PAIR），与 L79 的
    #: authoring_tree 口径同源。历史上这两格曾经单独钉过（2026-10-22 的 731 / 634），
    #: 那份读数保留在 git 历史里，不再作为可执行常量。
}

#: 「指向不入仓工件」两档之和（2026-10-27 重钉 1365 → **1484**；2026-10-30 → **1483**）
#:
#: 为什么只钉和：`scratch` 与 `scratch_missing` 的分界线是「Temp/ 里有没有同名工件」，
#: 而 Temp/ 不入仓、内容随取证脚本增删变化。作者树里 731 / 634，干净克隆里 0 / 1365，
#: 单格读数不可移植；两档之和 1365 才是可移植的案面读数。
#:
#: 本轮两笔：docs/augmentor 的 14 份 Markdown 接进扫描面 ⇒ +120（data/* 示例路径、
#: 短哈希、代码标识符形状）；上面那段 L178 历史归因收成本注时删掉了一条含反引号
#: train_data.json 的旧引用 ⇒ −1。净 1365 → 1484。
#: 2026-10-30 重钉 1484 → **1483**（−1）：AU-P1 提交（b81354d13 / 124e3d759）后
#: OPTIMIZATION_LOOP.md 的账目改写删除/合并了一条指向不入仓工件的旧引用，与 ambiguous
#: 同批漂移的存量（见 CEILING 注），一并记账。
SCRATCH_PAIR = 1483

#: 反空转下界：这几个数**不是缺陷计数**，而是「普查真的看见了多大一片」的证据。
#: 删引用是改进，不该被下界卡住；它只负责证明扫描没空转 —— 路径拼错、跳过清单写歪、
#: git ls-files 返回空，三种事故都会让「全部死档为 0」靠「什么都没扫到」拿到手。
#: `anchor_claims`（L245）是内容维的同一格保险：现量 24 条主张，归因正则写歪、括弧形状改成
#: 别的写法，都会让 `dead_anchor` 靠「一条主张都没看见」伪造成绿。
FLOOR = {"files": 400, "refs": 3000, "symbol_live": 300, "anchor_claims": 40}

#: 判据能产出的**全部桶名**（词汇表，与 `reference_state` 的返回集 + 两档符号 + dead_line 对齐）。
#: `live` 与 `symbol_live` 是好引用，不进 `CEILING`；`dead_line` 只能由 `span_state` 产出、
#: `dead_anchor` 只能由 `anchor_state` 产出，所以这两档不在 `_LIVE_ORDER` 里却必须在词汇表里。
#: `anchor_claims` **不是桶**（它是「看见多少条主张」的反空转计数），所以它不进这张词汇表。
BUCKETS = frozenset({"live", "ambiguous", "dead_path", "dead_line", "dead_anchor",
                     "runtime_ns", "scratch", "scratch_missing", "symbol_live",
                     "symbol_dead"})

#: 两系别合并的优先级就是口径，顺序即结论（每一档为什么排在这个位置见 `reference_state`）。
#: `dead_path` 排在工件两档之前：Temp 里站着 HEAD 快照与历史副本，排在后面等于允许一份不入仓
#: 的拷贝把产品路径写错洗白 —— 这正是 L79 第 5 条口径的理由。`ambiguous` 更靠前，因为「这个名字
#: 有两条候选」说的是引用指得到东西、只是不唯一，它归欠账档而不该抢硬 0 档的红。
_LIVE_ORDER = ("live", "ambiguous", "dead_path", "scratch", "scratch_missing",
               "runtime_ns")


def tick(*parts):
    """运行期拼一个带反引号的引用：本文件属于扫描面，源码里的字面量永远不成形"""
    return "`" + "".join(parts) + "`"


def _walks(include_temp, skip, top_skip=frozenset()):
    for base, dirs, files in os.walk(ROOT):
        if pathlib.Path(base) == ROOT:
            dirs[:] = [d for d in dirs if d not in top_skip]
        if not include_temp and "Temp" in set(pathlib.PurePath(base).parts):
            dirs[:] = []
            continue
        dirs[:] = [d for d in dirs if d not in skip]
        for name in files:
            path = pathlib.Path(base) / name
            yield path.relative_to(ROOT).as_posix(), path


def _repo_docs_walks():
    """汇出到仓库根的子项目文档（2026-10-14 从 `augmentor/docs/` 迁到 `docs/augmentor/`）

    产出与 `_walks` 同形，但相对坐标取**仓库根** ⇒ 首段是 `docs`，与产品码和账本里的
    引用写法（docs/augmentor/ 加文件名）对齐。不收它，那些引用会集体从 `live` 翻成
    `scratch_missing`（实测见本账 L14 文档汇总那一轮）。
    """
    if not DOCS_DIR.is_dir():
        return
    for path in sorted(DOCS_DIR.rglob("*")):
        if path.is_file() and path.suffix in ALL_EXT:
            yield path.name, ("docs", "augmentor", *path.relative_to(DOCS_DIR).parts)


@functools.lru_cache(maxsize=1)
def full_index():
    """`{文件名: (仓内路径的段元组, ...)}` —— 与 L79 的 `build_index` 同式，但**收全部扩展名**

    口径 2 的落点。只在这里放宽索引，桶的判定仍由 L79 的 `name_bucket` 出，判据不留第二份。
    """
    index = collections.defaultdict(list)
    for rel, _ in _walks(include_temp=True, skip=IDX_SKIP, top_skip=TOPLEVEL_SKIP):
        parts = pathlib.PurePath(rel).parts
        index[parts[-1]].append(parts)
    for name, parts in _repo_docs_walks():
        index[name].append(parts)
    return {k: tuple(sorted(v)) for k, v in index.items()}


@functools.lru_cache(maxsize=1)
def source_dirs():
    """装源码的顶层目录（Temp 除外）—— 与 L79 同式，只是喂的是全扩展名那一份索引"""
    return frozenset(p[0] for paths in full_index().values() for p in paths
                     if len(p) > 1 and p[0] != "Temp")


@functools.lru_cache(maxsize=None)
def lines_of(rel):
    """按**两套坐标**读同一份案面：augmentor 相对（ROOT）与仓库根相对（REPO）

    `docs/augmentor/*.md` 2026-10-14 从 `augmentor/docs/` 迁出后只存在于仓库根一侧；
    扫描面接上它们之后，行号/内容两维（`span_state` / `anchor_state`）要读的就是那份。
    先问 ROOT 再问 REPO：`docs/...` 在 ROOT 下不存在，不会与产品文件撞名；反过来若哪天
    仓库根也放一份同名产品文件，仍以 augmentor 内那份为准（历史坐标不变）。
    """
    for base in (ROOT, REPO):
        path = base / rel
        if not path.is_file():
            continue
        try:
            return path.read_text(encoding="utf-8").splitlines()
        except (UnicodeDecodeError, OSError):
            return None
    return None


def python_comments(text):
    """Python 文件的**注释 + docstring**（逐行带行号）；正文代码不参与引用普查"""
    out, docs = [], set()
    try:
        tree = ast.parse(text)
    except SyntaxError:
        tree = None
    if tree is not None:
        for node in ast.walk(tree):
            if not isinstance(node, (ast.Module, ast.ClassDef, ast.FunctionDef,
                                     ast.AsyncFunctionDef)):
                continue
            body = getattr(node, "body", None)
            if (body and isinstance(body[0], ast.Expr)
                    and isinstance(body[0].value, ast.Constant)
                    and isinstance(body[0].value.value, str)):
                docs.update(range(body[0].lineno, body[0].end_lineno + 1))
    try:
        for tok in tokenize.generate_tokens(io.StringIO(text).readline):
            if tok.type == tokenize.COMMENT:
                out.append((tok.start[0], tok.string))
            elif tok.type == tokenize.STRING and tok.start[0] in docs:
                out.append((tok.start[0], tok.string))
    except (tokenize.TokenError, IndentationError, SyntaxError):
        pass
    return out


def script_comments(text):
    """ts / tsx / js / vue 的注释行（行首 `//`、块注释起始与 `*` 续行）"""
    return [(lineno, stripped)
            for lineno, line in enumerate(text.splitlines(), 1)
            if (stripped := line.strip()).startswith(("//", "/*", "*"))]


@functools.lru_cache(maxsize=1)
def corpus():
    """扫描面全文：`((相对路径, ((行号, 文本), ...)), ...)`

    **两套坐标都在场**：产品树按 augmentor 相对（`_walks`），汇出到仓库根的
    docs/augmentor 下 Markdown 按仓库根相对（`_repo_docs_walks`）。后者此前只喂索引、不喂语料，
    于是 FAQ 与 ARCHITECTURE/API 三卷里的引用**没有任何守卫在读**——
    10-26 台账 §8 第 4 条把这条记成已知缺口，本轮（A184 增量）补齐。
    """
    rows = []
    for rel, path in _walks(include_temp=False, skip=SCAN_SKIP):
        ext = pathlib.PurePath(rel).suffix
        if ext not in SCAN_EXT:
            continue
        text = lines_of(rel)
        if text is None:
            continue
        if ext == ".py":
            notes = python_comments("\n".join(text))
        elif ext == ".md":
            notes = list(enumerate(text, 1))
        else:
            notes = script_comments("\n".join(text))
        rows.append((rel, tuple(notes)))
    for _name, parts in _repo_docs_walks():
        # 坐标直接用 `_repo_docs_walks` 交回的段元组 —— 用文件名拼会丢掉 system/ 子目录，
        # 而 `lines_of` 读不到时只会静默跳过（那正是「扫描面缩了却仍全绿」的成形方式）。
        full_rel = "/".join(parts)
        if pathlib.PurePath(full_rel).suffix not in SCAN_EXT:
            continue
        text = lines_of(full_rel)
        if text is None:
            continue
        rows.append((full_rel, tuple(enumerate(text, 1))))
    return tuple(rows)


@functools.lru_cache(maxsize=1)
def symbol_universe():
    """产品树里**代码面**在场的标识符 —— 符号档「活着」的定义

    为什么宇宙不许包含注释与文档：本轮注入实测直接量出来的（Temp 下那支注入探针的
    symbol_dead 一档）—— 往一份真实文件的 docstring 里写一支根本不存在的 API 主张，守卫
    **全绿**。第一版的宇宙是「全文本扫标识符」，于是那句主张自己把名字喂给了宇宙：一条假
    主张只要被写下来就自动变成真名字，判据自证能力为 0。改成只收代码面之后，同一支注入
    立刻红。

    代价说清楚：代码面宇宙仍然只判「名字在场」，不解析归属，也不验证点号右边那一截。
    标准库那一类主张由 `declared_or_external` 单独放行（`re._compile`、`str.isspace` 这种
    末端属性本仓代码里压根不会调用，逐条 getattr 既慢又要在测试里 import 任意模块）。
    """
    names, ident = set(), re.compile(r"[A-Za-z_][A-Za-z0-9_]{2,}")
    for rel, path in _walks(include_temp=False, skip=SCAN_SKIP):
        ext = pathlib.PurePath(rel).suffix
        if ext not in ALL_EXT or ext == ".md":
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        if ext == ".py":
            try:
                tree = ast.parse(text)
            except SyntaxError:
                tree = None
            if tree is not None:
                for node in ast.walk(tree):
                    if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef,
                                         ast.ClassDef)):
                        names.add(node.name)
                    elif isinstance(node, ast.Assign):
                        names.update(t.id for t in node.targets
                                     if isinstance(t, ast.Name))
                    elif isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name):
                        names.add(node.target.id)
                    elif isinstance(node, ast.Attribute):
                        names.add(node.attr)
            try:
                # tokenize 的 NAME 天然不含注释与字符串内容 —— 自洗白就是这么挡掉的
                names.update(tok.string for tok in
                             tokenize.generate_tokens(io.StringIO(text).readline)
                             if tok.type == tokenize.NAME)
            except (tokenize.TokenError, IndentationError, SyntaxError):
                pass
        elif ext in SRC_EXT:
            # 脚本文件只取非注释行（块注释按整块跳过：续行的 `*` 里也可能带名字主张）
            for line in text.splitlines():
                stripped = line.strip()
                if stripped.startswith(("//", "/*", "*")):
                    continue
                names.update(ident.findall(stripped))
        else:
            names.update(ident.findall(text))
    return names


@functools.lru_cache(maxsize=None)
def external_head(head):
    """点号主张的**头**是不是一个打得开的模块（标准库或已装依赖）"""
    if head in dir(builtins) or head in sys.builtin_module_names:
        return True
    try:
        return importlib.util.find_spec(head) is not None
    except (ImportError, ValueError):
        return False


def declared_or_external(name, universe):
    """符号主张的放行判据：末端名字在代码面宇宙里，或那是一个「外部模块.成员」形状的主张"""
    leaf = name.split(".")[-1]
    if leaf in universe:
        return True
    # 只有带点号、且点号左边打得开一个真模块的形状才走外部档（`re._compile`、`str.isspace`）；
    # 裸名不存在就是不存在，这里不许有第二种解释。
    return "." in name and external_head(name.split(".")[0])


@functools.lru_cache(maxsize=1)
def tracked_dirs():
    """被跟踪路径的**全部父目录** —— 「坏链接」与「运行期示例路径」的那条分界线

    只要父目录，因为判据问的是「这个目录里本来有没有东西」；被跟踪文件本身由索引答。
    用 git 的跟踪集合而不是磁盘，因为磁盘上还站着不入仓的用户数据 —— 按「目录里有文件」判，
    本轮第一版实测把 414 条示例路径全判成了死链。

    2026-10-14 文档汇总：`docs/` 不再是 augmentor 内的命名空间，改由仓库根
    `docs/augmentor/` 承接 ⇒ 跟踪集合要把 `docs/augmentor` 一并取回，并保留
    `docs` 与 `docs/augmentor` 两级父目录。不取的话 docs/CONFIG.md 这类
    已知坏链接会从 `dead_path` 滑进 `runtime_ns`，`TestRulerProvesItself`
    两格当场红（判据自己不再认得已知答案）。
    """
    out = subprocess.run(["git", "ls-files", "--", "augmentor", "docs/augmentor"],
                         cwd=ROOT.parent, capture_output=True, text=True,
                         encoding="utf-8", errors="replace")
    dirs = set()
    for line in out.stdout.splitlines():
        parts = pathlib.PurePath(line).parts
        if parts[:1] == ("augmentor",):
            #: 原有坐标：augmentor 相对（augmentor/api/routes/x.py → api、api/routes）
            parts = parts[1:]
        elif parts[:2] != ("docs", "augmentor"):
            continue
        dirs.update("/".join(parts[:i]) for i in range(1, len(parts)))
    return frozenset(dirs)


def span_state(rel, start, end):
    """区间引用只有**整段都不是代码**才算可证失效 —— 这一条口径抄自 L79 的 `classify`

    单行的语义是「跳到那一行」⇒ 必须是代码行；区间的语义是「这件事在这段里」⇒ 起点空、
    终点是代码的那档属「区间报歪一行」，归 L79 的漂移档（A127 后半程），不该由硬门禁开火。
    """
    lines = lines_of(rel)
    if lines is None:
        return "unreadable"
    markers = COMMENT_MARKERS.get(pathlib.PurePath(rel).suffix, ())
    last = start if end is None else max(end, start)
    kinds = set()
    for lineno in range(max(start, 1), last + 1):
        if lineno > len(lines):
            kinds.add("out-of-range")
            continue
        raw = lines[lineno - 1].strip()
        kinds.add("blank" if not raw
                  else "comment" if markers and raw.startswith(markers) else "code")
    return "dead" if "code" not in kinds else "live"


@functools.lru_cache(maxsize=None)
def def_spans(rel):
    """目标文件里 def/class 的跨度名册 `{名字: ((起, 止), ...)}` —— 内容维的坐标系

    与 `symbol_universe` 同源但**带行号**：符号档只问「名字在不在代码宇宙里」，内容维要问
    「这一行在不在它的身子里」，所以必须回到 AST 跨度。非 Python、读不出、语法错都回空表，
    那些文件里的引用压根不进主张档（判不到，不是判绿）。
    """
    if pathlib.PurePath(rel).suffix != ".py":
        return {}
    lines = lines_of(rel)
    if lines is None:
        return {}
    try:
        tree = ast.parse("\n".join(lines))
    except SyntaxError:
        return {}
    out = collections.defaultdict(list)
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            out[node.name].append((node.lineno, node.end_lineno or node.lineno))
    return {k: tuple(sorted(v)) for k, v in out.items()}


def claim_paren(text, start, end):
    """引用（字符跨度 `[start, end)`）的主张括弧正文；两种形状都没有则 None

    先问**后括弧**（本仓写主张的正形：路径带行号之后紧跟一个括弧点名），再问**外括弧**
    （把名字写在前面、引用写在同一个括弧里的那种）。外括弧只认不含嵌套的最内层，否则一句里
    几个括弧会互相冒名。
    """
    tail = TAIL_PAREN.match(text[end:end + 200])
    if tail:
        return tail.group(1)
    open_at = max(text.rfind("（", 0, start), text.rfind("(", 0, start))
    if open_at == -1:
        return None
    close = CLOSE_PAREN.search(text, open_at)
    if close is None or close.start() < end:
        return None
    inner = text[open_at + 1:close.start()]
    return None if ("（" in inner or "(" in inner) else inner


def anchor_state(target, start, end, text, tick_start, tick_end):
    """一条「文件活着、落点是代码」的引用 → 内容维读数 `(主张型, 结论, 点名, 现量跨度)`

    三种读数（口径 5）：
    - `(False, None, None, ())` —— **判不到**：没有主张括弧、括弧首不是裸标识符、或那个名
      不是目标文件里的 def/class（键名、属性名、别的文件的名都落这一档）。
    - `(True, "ok", 名, 跨度)` —— 主张成立：落点确实整个在那个符号的身子里。
    - `(True, "red", 名, 跨度)` —— 可证失效：落点是代码，但**不在**所点名的那个 def/class 里
      ⇒ 这就是 `dead_line` 量不到的那一格「落点对、内容错」。
    """
    inner = claim_paren(text, tick_start, tick_end)
    if inner is None:
        return False, None, None, ()
    named = BARE_TICK.match(inner)
    if named is None:
        return False, None, None, ()
    name = named.group(1)
    spans = def_spans(target).get(name, ())
    if not spans:
        return False, None, None, ()
    inside = any(lo <= start and end <= hi for lo, hi in spans)
    return True, "ok" if inside else "red", name, spans


def reference_state(ref, dirs, index, src):
    """一条引用 → `(结论, 唯一活路径)`；两系别各判一次，合并优先级就是口径

    优先级（先命中者说了算）与理由：

    1. 任一系指到唯一活文件 ⇒ `live`；
    2. 任一系说「名字有多条候选」⇒ `ambiguous`（欠账档）：引用指得到东西、只是不唯一，
       不该抢硬 0 档的红；
    3. 「父目录是被跟踪的命名空间、却没有这个文件」⇒ `dead_path`，**必须排在工件与示例档之前**
       （Temp 里的 HEAD 快照与历史副本会把产品路径写错洗白）；
    4. 最后才是不入仓工件（`scratch` / `scratch_missing`）与运行期示例路径（`runtime_ns`）。

    **只摘前缀、绝不加前缀**：索引取自 augmentor 内部，所以仓库根系别的写法要把 augmentor 前缀
    摘掉再问一次。反方向给裸名**加**前缀是没用的 —— 那个层级压根没有条目，它只会凭空造出
    「父目录被跟踪」的假证据（第二版就这么把 414 条示例路径全判成了死链）。
    """
    cands = [ref]
    if ref.startswith("augmentor/"):
        cands.append(ref[len("augmentor/"):])
    states = []
    for cand in cands:
        bucket, live = name_bucket(cand, index, src)
        if bucket is None:
            states.append(("live", live))
        elif bucket == "unresolved_source":
            # 父目录判据必须用 augmentor 相对坐标：`dirs` 就是从 augmentor 里算出来的。
            head = pathlib.PurePath(cand).parts[0] if pathlib.PurePath(cand).parts else ""
            parent = "/".join(pathlib.PurePath(cand).parts[:-1])
            if head in RUNTIME_DIRS:
                # 运行期输出目录：按**名字**判，不赌磁盘上有没有（否则同一条示例路径
                # 在作者树与干净克隆里会落进两个不同的档，见 RUNTIME_DIRS 注释）。
                states.append(("runtime_ns", None))
            else:
                states.append(("runtime_ns" if parent not in dirs else "dead_path", None))
        else:
            states.append((bucket, None))
    for want in _LIVE_ORDER:
        for state, live in states:
            if state == want:
                return want, live
    # 走到这里说明 `name_bucket` 交回了一个本判据没登记过的桶名。宁可当场炸，也不许把未知
    # 桶并进某个棘轮档 —— 那正是「真 bug 被记成欠账、没有任何东西会红」的成形方式。
    raise AssertionError("未知桶：%r → %r" % (ref, states))


def audit(items, index, src, dirs, universe):
    """跑一遍普查 → `(档位计数, 可证失效行, 符号失效行)`；行里带上下文，失败信息直接点名

    计数键**就是桶名**（不带前缀）：`CEILING` 的键与这里的键一一对应，于是「桶名拼错」会当场
    红在 `test_every_ceiling_bucket_has_its_own_readout` 上，而不是让 `Counter` 的默认 0
    把硬 0 档伪造成绿。
    """
    counts = collections.Counter({"files_seen": len(items)})
    dead, sym_dead = [], []
    for rel, notes in items:
        for lineno, text in notes:
            for match in REF.finditer(text):
                ref, start, end = match.group(1), match.group(2), match.group(3)
                state, live = reference_state(ref, dirs, index, src)
                counts[state] += 1
                counts["refs_total"] += 1
                note = text.strip()[:64]
                if state == "dead_path":
                    dead.append((rel, lineno, match.group(0), "dead_path", note))
                elif start is not None and state == "live":
                    span_end = int(end) if end else int(start)
                    if span_state(live, int(start), span_end) == "dead":
                        counts["dead_line"] += 1
                        dead.append((rel, lineno, match.group(0), "dead_line", note))
                    else:
                        # 落点是代码 ⇒ 接着问内容维（口径 5）：这一行是不是它自称的那个符号
                        claimed, verdict, name, spans = anchor_state(
                            live, int(start), span_end, text, match.start(), match.end())
                        if claimed:
                            counts["anchor_claims"] += 1
                            if verdict == "red":
                                counts["dead_anchor"] += 1
                                dead.append((rel, lineno, match.group(0), "dead_anchor",
                                             "%s 点名 %s 现量跨度 %s"
                                             % (note, name, list(spans))))
            if rel in NO_SYM_SCAN:
                continue
            seen = set()
            for pattern in (SYM_CALL, SYM_PAREN):
                for sym in pattern.finditer(text):
                    name = sym.group(1)
                    if name in seen:
                        continue
                    seen.add(name)
                    if declared_or_external(name, universe):
                        counts["symbol_live"] += 1
                    else:
                        counts["symbol_dead"] += 1
                        sym_dead.append((rel, lineno, name))
    return counts, dead, sym_dead


@functools.lru_cache(maxsize=1)
def census():
    """真身普查：整套判据只跑一次（全量 ~6 s），下面所有用例读同一份结果"""
    return audit(corpus(), full_index(), source_dirs(), tracked_dirs(),
                 symbol_universe())


def probe(items):
    """把合成语料并进真身普查：注入用例据此证明「判据会红」，而不是「数字恰好是 0」"""
    counts, rows, sym = _probe_raw(items)
    real_counts, real_rows, real_sym = census()
    return counts, rows, sym, real_counts, real_rows, real_sym


@functools.lru_cache(maxsize=1)
def _probe_raw(items):
    return audit(tuple(corpus()) + items, full_index(), source_dirs(),
                 tracked_dirs(), symbol_universe())


def ask_state(ref):
    """单条引用的结论档 —— selftest 的入口，也是「判据换方向时先红在这里」的那一格"""
    return reference_state(ref, tracked_dirs(), full_index(), source_dirs())[0]


def first_line_of(rel, want_blank):
    """现量取 `rel` 里第一个空白 / 非空白行的行号 —— 注入的靶子不许硬写行号（行号会搬家）"""
    lines = lines_of(rel)
    assert lines, rel
    for lineno, raw in enumerate(lines, 1):
        if (not raw.strip()) == want_blank:
            return lineno
    raise AssertionError("%s 里没有%s行" % (rel, "空白" if want_blank else "非空白"))


class TestCensusIsProven:
    """扫描面本身必须被证实在场，否则「全部死档为 0」可以靠「什么都没扫到」拿到"""

    def test_the_corpus_is_big_enough_to_be_the_claim(self):
        counts, _, _ = census()
        assert counts["files_seen"] >= FLOOR["files"], counts["files_seen"]
        assert counts["refs_total"] >= FLOOR["refs"], counts["refs_total"]
        assert counts["symbol_live"] >= FLOOR["symbol_live"], counts["symbol_live"]
        # 内容维的反空转：主张条数掉到下界之下 = 归因那一跳断了，硬 0 就成了假绿
        assert counts["anchor_claims"] >= FLOOR["anchor_claims"], counts["anchor_claims"]

    def test_the_index_is_not_python_only(self):
        """A184 的失明点就在这三格：YAML / TS / Markdown 都要能被解析成唯一活路径

        三个靶子是现量选的，不是随手写的：README 这个名字在仓里有**多份**（根上一份、
        子项目各一份），它进不了这一格 —— 它该进的是歧义档，硬写在这里只会让用例红在
        「我以为它唯一」上。
        """
        index, src = full_index(), source_dirs()
        for rel in ("config.yaml", "web/src/services/api.ts", "docs/augmentor/ARCHITECTURE.md"):
            bucket, live = name_bucket(rel, index, src)
            assert bucket is None and live == rel, (rel, bucket, live)

    def test_repo_level_docs_are_in_the_corpus(self):
        """仓库根的 docs/augmentor 必须在**语料**里，不只是索引里

        只进索引不进语料，等于「能解析但没人读」——FAQ 的 31 条代码引用、ARCHITECTURE
        与 API 三卷的成百条定位引用会一起退回 10-26 台账 §8 第 4 条那个缺口。
        三个靶子覆盖三种布局：顶层单文件、顶层卷、system/ 子目录（子目录用文件名拼路径会丢层）。
        """
        rels = {rel for rel, _ in corpus()}
        for rel in ("docs/augmentor/FAQ.md", "docs/augmentor/API.md",
                    "docs/augmentor/system/01-overview-and-terminology.md"):
            assert rel in rels, rel

    def test_the_rulers_own_regex_still_cannot_see_a_yaml_line_ref(self):
        """钉住「为什么需要这一支」：L79 的行号正则写死 Python 后缀，YAML 引用它读不到

        哪天 L79 把正则放宽了，这一格会红 ⇒ 届时该做的是**合并两支守卫**，而不是把这条断言
        删了事（红在这里是好消息，不是噪声）。
        """
        token = tick("config.yaml:", str(first_line_of("config.yaml", True)))
        assert LINE_REF.fullmatch(token) is None, token
        # 而本守卫读得到：同一个名字喂进判据，落点是那两份 YAML 里唯一的出厂模板
        state, live = reference_state("config.yaml", tracked_dirs(),
                                      full_index(), source_dirs())
        assert (state, live) == ("live", "config.yaml")

    def test_every_ceiling_bucket_is_a_bucket_the_census_can_produce(self):
        """桶名拼错 = 硬 0 永远绿：`CEILING` 的每个键都必须在普查的桶词汇表里

        这一格不吃读数，只吃词表 —— 因为 `Counter` 对不存在的键默认给 0，「拼错的桶名」与
        「真的 clean」在读数上完全同形。词汇表 `BUCKETS` 是判据能产出的全部结论名，写死在
        模块顶部，`reference_state` 的返回集变了就必须同时改它。
        """
        assert set(CEILING) <= BUCKETS, set(CEILING) - BUCKETS
        # 好引用与「symbol_live」不进 CEILING；工件的两档由 SCRATCH_PAIR 合钉（单格不可移植）
        assert BUCKETS - set(CEILING) == {
            "live", "symbol_live", "scratch", "scratch_missing"}, BUCKETS - set(CEILING)
        # 合钉的那两档不许被漏掉：词汇表里在、CEILING 里不在的只允许这四格
        assert {"scratch", "scratch_missing"} <= BUCKETS
        # 合并优先级的表不许写出词汇表之外的档名
        assert set(_LIVE_ORDER) <= BUCKETS, set(_LIVE_ORDER) - BUCKETS


class TestHardZero:
    """可证失效 = 0：四档各自硬判，失败信息点名到文件与行"""

    @staticmethod
    def _named(rows, kind):
        return "\n".join("  %s:%d %s | %s" % (r[0], r[1], r[2], r[4])
                         for r in rows if r[3] == kind)

    def test_no_reference_points_at_a_file_that_should_exist(self):
        counts, rows, _ = census()
        assert counts["dead_path"] == CEILING["dead_path"], self._named(rows, "dead_path")

    def test_no_line_reference_lands_on_a_non_code_span(self):
        counts, rows, _ = census()
        assert counts["dead_line"] == CEILING["dead_line"], self._named(rows, "dead_line")

    def test_no_line_reference_claims_a_symbol_it_does_not_land_in(self):
        """内容维（A331 / L245）：落点是代码但不是所点名那个 def/class 的家里 ⇒ 红"""
        counts, rows, _ = census()
        assert counts["dead_anchor"] == CEILING["dead_anchor"], self._named(rows, "dead_anchor")

    def test_no_symbol_claim_points_at_an_absent_name(self):
        counts, _, rows = census()
        assert counts["symbol_dead"] == CEILING["symbol_dead"], \
            "\n".join("  %s:%d %s" % r for r in rows)


class TestRatchetsAreExact:
    """四档欠账/工件读数精确相等（只降不升；改进也得同时改常数并留一句「谁清的」）"""

    @pytest.mark.parametrize("bucket", ["ambiguous", "runtime_ns"])
    def test_bucket_matches_its_ceiling(self, bucket):
        counts, _, _ = census()
        assert counts[bucket] == CEILING[bucket], \
            "%s 现量 %d，常数是 %d" % (bucket, counts[bucket], CEILING[bucket])

    def test_scratch_pair_matches_its_ceiling(self):
        """工件两档只钉**和**：Temp/ 不入仓，作者树与干净克隆的单格读数必然不同

        这条在两种树上都跑（不像 `test_bucket_matches_its_ceiling` 依赖作者工作树），
        因为「和」恰恰是两边共有的那一格；若哪天有人把 docs/augmentor 从扫描面摘掉，
        和会掉回 1365，当场红。
        """
        counts, _, _ = census()
        pair = counts["scratch"] + counts["scratch_missing"]
        assert pair == SCRATCH_PAIR, \
            "scratch + scratch_missing 现量 %d（%d + %d），常数是 %d" % (
                pair, counts["scratch"], counts["scratch_missing"], SCRATCH_PAIR)

    def test_the_good_references_outweigh_the_debt(self):
        """好引用必须压过全部欠账：这条比例是「普查在量案面而不是在量噪声」的读数"""
        counts, _, _ = census()
        debt = sum(counts[b] for b in ("ambiguous", "runtime_ns", "scratch",
                                       "scratch_missing"))
        assert counts["live"] > debt, (counts["live"], debt)

    def test_runtime_backup_dir_is_invisible_to_the_index(self):
        """`tests/.backups` 是测试套件自写、gitignore 的运行时快照目录：进索引会让
        `index.json`/`snap.json` 这类裸文件名在多候选化，下一次全量门禁的普查把一批
        本活的引用翻进 `ambiguous` 欠账档（L145，B215，实测 336 vs 钉值 327）。"""
        leaked = [parts for parts_list in full_index().values()
                  for parts in parts_list if ".backups" in parts]
        assert not leaked, leaked[:5]


class TestRedCapability:
    """判据必须能红：三种合成失效喂进同一条判据，要求逐档命中、且真身一格不多"""

    def _synthetic(self):
        # 靶子行号现量：硬写的行号下一轮就不是空行了，那时「红能力」会退化成永远绿
        blank = first_line_of("config.yaml", True)
        code = first_line_of("config.yaml", False)
        self.control = tick("config.yaml:", str(code))
        notes = ((1, "broken link " + tick("docs/NOT_A_REAL_FILE_", "a184.md")),
                 (2, "blank line " + tick("config.yaml:", str(blank))),
                 (3, "dead claim " + tick("no_such_symbol_", "a184", "()")),
                 (4, "control " + self.control))
        return (("tests/unit/_a184_injected_probe.py", notes),)

    def test_injection_turns_each_hard_bucket_red_exactly_once(self):
        counts, rows, sym, real, real_rows, real_sym = probe(self._synthetic())
        assert counts["dead_path"] - real["dead_path"] == 1, rows
        assert counts["dead_line"] - real["dead_line"] == 1, rows
        assert len([r for r in rows if r[3] == "dead_path"]) == 1, rows
        assert len([r for r in rows if r[3] == "dead_line"]) == 1, rows
        assert len(rows) == len(real_rows) + 2, rows
        assert len(sym) - len(real_sym) == 1, sym
        # 两条 YAML 引用都指得到那个文件（一条只是行是空的）⇒ live 档各 +1，
        # 而 symbol_live 一格不动 ⇒ 「红」来自判据，不是把一切判死。
        assert counts["live"] - real["live"] == 2, counts["live"]
        assert counts["symbol_live"] == real["symbol_live"]
        assert not any(r[2] == self.control for r in rows), self.control

    def test_the_injection_leaves_the_ratchet_buckets_untouched(self):
        counts, _, _, real, _, _ = probe(self._synthetic())
        for bucket in ("ambiguous", "runtime_ns", "scratch", "scratch_missing"):
            assert counts[bucket] == real[bucket], bucket


class TestContentDimension:
    """内容维（A331 / L245）：括弧里点名的 def/class 必须真的把落点包住

    五档合起来才证明这把尺子不是恒真：① 落点在符号里 ⇒ 绿；② 落点在符号外 ⇒ 红（这一格正是
    `dead_line` 量不到的「落点对、内容错」）；③ 点号形状**不判**；④ 括弧首是散文**不判**；
    ⑤ 落点压根不是代码时只归 `dead_line`，内容维**不重复开火**（两档不许抢同一个红）。
    靶子的行号全部由 AST 与 `first_line_of` 现量取，不硬写 —— 硬写的行号下一轮就不是那一行。
    这四支都不叫 `require_authoring_tree()`：判据是**增量**对账（合成语料并进真身普查再比差），
    与 Temp 底料在不在场无关，加了反而在干净克隆里把自己跳过、注入取证白做。
    """

    #: 靶子按现量选：文件名唯一、且里面有跨多行的 def/class（本文件属于扫描面，所以这个名字
    #: 只能写成不带反引号的字符串，运行期由 `tick()` 拼成形）
    ANCHOR_TARGET = "augmentor/search_enhanced.py"

    @classmethod
    def arms(cls):
        """`(点名符号, 符号里那一行, 符号外那一行)` —— 全从目标文件的 AST 现量取"""
        single = {n: s[0] for n, s in def_spans(cls.ANCHOR_TARGET).items() if len(s) == 1}
        assert len(single) >= 3, single
        name, (lo, hi) = max(single.items(), key=lambda kv: kv[1][1] - kv[1][0])
        markers = COMMENT_MARKERS[".py"]
        outside = next(i for i, raw in enumerate(lines_of(cls.ANCHOR_TARGET), 1)
                       if raw.strip() and not raw.strip().startswith(markers)
                       and not lo <= i <= hi)
        return name, lo, outside

    def _notes(self, name, inside_line, outside_line):
        ref = lambda line: tick(self.ANCHOR_TARGET + ":", str(line))
        claim = lambda line, token: ref(line) + "（" + token + "）"
        return (("tests/unit/_a184_anchor_probe.py", (
            (1, "in " + claim(inside_line, tick(name))),
            (2, "out " + claim(outside_line, tick(name))),
            (3, "dotted " + claim(outside_line, tick(name + ".method"))),
            (4, "prose " + ref(outside_line) + "（散文 " + tick(name) + "）"),
        )),)

    def _notes_blank(self, name, blank_line):
        return (("tests/unit/_a184_anchor_probe.py",
                 ((1, "blank " + tick(self.ANCHOR_TARGET + ":", str(blank_line))
                   + "（" + tick(name) + "）"),)),)

    def test_only_the_two_landing_lines_are_claims_and_exactly_one_is_red(self):
        name, lo, outside = self.arms()
        counts, rows, _sym, real, real_rows, _rsym = probe(self._notes(name, lo, outside))
        assert counts["anchor_claims"] - real["anchor_claims"] == 2, counts["anchor_claims"]
        assert counts["dead_anchor"] - real["dead_anchor"] == 1, rows
        # 名册按**增量**比：案面本来就该是 0 格（硬 0 档），但注入判据不许依赖这一点——
        # 别轮若留下 12 格真红，这一格仍要精确交出「合成语料只造了 1 格」
        real_anchor = len([r for r in real_rows if r[3] == "dead_anchor"])
        anchor_rows = [r for r in rows if r[3] == "dead_anchor"]
        assert len(anchor_rows) - real_anchor == 1, anchor_rows
        assert anchor_rows[0][2] == tick(self.ANCHOR_TARGET + ":", str(outside)), anchor_rows
        # 落点在同一符号里的那一支不但不红，连「红」的邻居都不当：dead_line 一格不多
        assert counts["dead_line"] == real["dead_line"], real["dead_line"]
        assert not any(r[2] == tick(self.ANCHOR_TARGET + ":", str(lo)) for r in rows), lo

    def test_the_two_excluded_shapes_are_not_claims_at_all(self):
        """③④ 两档证明「判不到」是真判不到，而不是碰巧也绿：主张数一格都不多"""
        name, lo, outside = self.arms()
        notes = self._notes(name, lo, outside)[0][1]
        counts, _, _, real, _, _ = probe(self._notes(name, lo, outside))
        assert counts["anchor_claims"] - real["anchor_claims"] == 2, counts["anchor_claims"]
        # 单独喂 ③④ 这两支 ⇒ 主张增量为 0（混在 ①② 一批里看不出这一格）
        c2, _, _, _, _, _ = probe((("tests/unit/_a184_anchor_probe.py", notes[2:]),))
        assert c2["anchor_claims"] == real["anchor_claims"], c2["anchor_claims"]
        assert c2["dead_anchor"] == real["dead_anchor"], c2["dead_anchor"]

    def test_a_blank_landing_fires_dead_line_only_never_twice(self):
        """⑤ 同一支引用不许同时开火两档：落点是空行 ⇒ `dead_line` +1、内容维 +0"""
        name, _lo, _outside = self.arms()
        blank = first_line_of(self.ANCHOR_TARGET, True)
        counts, rows, _, real, _rr, _ = probe(self._notes_blank(name, blank))
        assert counts["dead_line"] - real["dead_line"] == 1, rows
        assert counts["anchor_claims"] == real["anchor_claims"], counts["anchor_claims"]
        assert counts["dead_anchor"] == real["dead_anchor"], counts["dead_anchor"]


class TestRulerProvesItself:
    """判据自己得认得已知答案 —— 已知形状逐格验；方向写反的判据永远红（承 L98 的教训）"""

    def test_broken_link_inside_a_tracked_namespace_is_dead(self):
        assert ask_state("docs/NOT_A_REAL_FILE_xyz.md") == "dead_path"

    def test_the_historically_broken_config_pointer_is_dead(self):
        # 账本曾七次引用过一个从未存在的配置文档；本轮把它摘成散文，指针本身仍是坏链接
        assert ask_state("docs/CONFIG.md") == "dead_path"

    def test_example_path_in_a_user_data_dir_is_not_dead(self):
        # data/ 自 L178 起从索引顶层移出（未入仓的用户数据目录，同 .backups 一档）：
        # 指向其中不存在文件的路径判 scratch_missing（「指向不入仓工件」欠账档），
        # 不是硬 0 的坏链接 —— 这一格守住「用户数据里的示例路径不被当死链开火」。
        require_authoring_tree()
        assert ask_state("data/xxx.json") == "scratch_missing"

    def test_a_live_augmentor_relative_path_is_recognised(self):
        assert ask_state("api/deps.py") == "live"

    def test_the_same_file_written_from_the_repo_root_is_also_live(self):
        assert ask_state("augmentor/api/deps.py") == "live"

    def test_a_temp_draft_counts_as_an_artifact_not_a_dead_link(self):
        assert ask_state("Temp/nope/probe_l99.py") in ("scratch", "scratch_missing")

    def test_span_judgement_follows_the_target_files_language(self):
        assert span_state("README.md", 1, 1) == "live"          # Markdown 的 `#` 是内容
        assert span_state("README.md", 99999, 99999) == "dead"  # 越界不是代码

    def test_the_content_dimension_answers_the_two_known_landing_lines(self):
        """已知答案逐格验（承 L98 那条「方向写反的判据永远红」）：家里 ok、门外 red、散文不判"""
        name, lo, outside = TestContentDimension.arms()
        target = TestContentDimension.ANCHOR_TARGET
        for line, want in ((lo, "ok"), (outside, "red")):
            ref = tick(target + ":", str(line))
            text = "落点 " + ref + "（" + tick(name) + "）"
            at = text.index(ref)
            claimed, verdict, got, spans = anchor_state(target, line, line, text,
                                                        at, at + len(ref))
            assert (claimed, verdict, got) == (True, want, name), (line, claimed, verdict, got)
            assert spans == def_spans(target)[name], spans
        # 括弧首是散文 ⇒ 整条不进主张档。「判不到」与「判绿」在读数上同形，所以这一格必须点名
        ref = tick(target + ":", str(outside))
        text = ref + "（散文 " + tick(name) + "）"
        assert anchor_state(target, outside, outside, text, 0, len(ref))[0] is False
