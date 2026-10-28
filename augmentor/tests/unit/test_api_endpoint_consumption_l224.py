# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""后端 77 条操作与前端服务层调用面的消费对账（A279 立账轮）

**为什么建这把尺子**：`web/src/services/api.wiring.test.ts`（L211 那批）钉的是「`api.ts`
导出的每个函数都在页面上被引用」，它管的是**封装之后**那一环；而「后端有 75 条操作，
前端到底调了几条」从来没有机器读过。目标那句「消费能力达到 100%」要的是一个能数出来的
数，不是散文。本轮把数落出来并把缺口立成账（**A279**），不在同一轮里补接线：
实测缺口是 **32 条操作**，补它是一串功能改动，得按页面分组分批做（与 A138 的
②③ 档接线同一个节奏），一轮吞下去只会得到一份「加了三页但没人验证」的产物。

**L225 接了第一批**：A279 优先队列里「web 与 CLI 两边都没入口」的 6 条中先做 3 条
（`language-profile` / `evaluate` 进分析页，`vector-status` 进系统页），实测缺口
32 → **29**、已消费 43 → **46**。这把尺子在只改前端的那一步就先红了三次（形状数、
名单相等、名单规模），这正是它想要的反应。

**L226 接了第二批**：`search` / `validate` / `auto-config` 三条分别进分析页、质量中心与配置管理，
实测缺口 29 → **26**、已消费 46 → **49**，前端调用形状同涨到 49。`CLI_ENTRY` 少了 3 条：
26 → **23**，指向的不同子命令 21 → **18**；`NO_ENTRY` 仍是 **3**，本轮没碰。
另拒接两员并记账（**这条裁定已由用户在 L234 撤销**，两员都要接、目标 75/75，见下文）：
`GET /api/system/dependencies` 与 `/api/status` 里的 dependencies 是同一份数据的两面；
`POST /api/dataset/translate` 把整份数据集回传又没有 `output_file` 可落盘，属契约层问题，另立 **A283** 待拍。

**L227 接了第三批**：`impact` / `semantic-search` 两条进分析页，实测缺口 26 → **24**、
已消费 49 → **51**、前端调用形状同涨到 51。这一批的形状与 L226 不同：两条都**没有**
CLI 子命令（本轮探针在 `parser.py` 里 `grep impact|semantic` 只命中 `health-gate` 的
`--text-field`），所以它们一直住在 `NO_ENTRY` 里，接线之后从名单移出 —— `CLI_ENTRY`
一条没动（仍 **23** 条 / **18** 个不同子命令），`NO_ENTRY` 收到只剩 **1** 条
（`translate`，归 A283）。缺口 24 = 23 条有 CLI 入口 + 1 条没有，「连 CLI 名字都没有」
这一档收到只剩一员，判据 3 的验真继续逐条走（`NO_ENTRY` 每一员的末段仍要确认不在
`add_parser` 名单里，清零之前它都是可判负的形状）。

**L228 接了第四批**：`stats` / `features` / `compare` 三条进分析页（字段级统计、特征普查、
双集对比三张卡），实测缺口 24 → **21**、已消费 51 → **54**、前端调用形状同涨到 54。三条都
各有 CLI 子命令（`stats` / `features` / `compare`），所以这一批动的正是 `CLI_ENTRY`：
23 → **20** 条，指向的不同子命令 18 → **15** 个；`NO_ENTRY` 一条没动，仍是 **1** 条
（`translate`，归 A283）。缺口 21 = 20 条有 CLI 入口 + 1 条没有。

**L229 接了第五批**：`convert` / `merge` / `split` 三条写盘变换进数据管理页（格式转换、
合并数据集、训练 / 验证 / 测试分割三张卡），实测缺口 21 → **18**、已消费 54 → **57**、
前端调用形状同涨到 57。三条都各有同名 CLI 子命令，所以动的又是 `CLI_ENTRY`：20 → **17** 条，
不同子命令 15 → **12** 个；`NO_ENTRY` 仍 **1** 条（`translate`，归 A283）。缺口 18 = 17 条有
CLI 入口 + 1 条没有。这一批的**接线质量**判据不住在这里，住在
`tests/unit/test_transform_wiring_l229.py`（两张格式清单同源、页面初值等于契约默认、
UI 发得出的每一档在端点上的判决、回执覆盖全部响应键）。

**L230 接了第六批**：`sample` / `aggregate` / `rag` 三条进数据管理页（采样、多源聚合、
转成 RAG 检索格式三张卡），实测缺口 18 → **15**、已消费 57 → **60**、前端调用形状同涨到
60。三条各有同名 CLI 子命令，所以动的仍是 `CLI_ENTRY`：17 → **14** 条，不同子命令
12 → **9** 个；`NO_ENTRY` 仍 **1** 条（`translate`，归 A283）。缺口 15 = 14 条有 CLI 入口
+ 1 条没有。接线质量判据住在 `tests/unit/test_sample_aggregate_rag_wiring_l230.py`
（三张下拉清单同源于后端分发臂、页面初值等于契约默认、UI 发得出的每一档在端点上的判决、
回执覆盖全部响应键）；这一批同时量出六条后端判据缺口，立为 A292~A297。

**L231 接了第七批**：备份那一族四条（`GET` / `POST /api/system/backups`、
`POST …/{backup_id}/restore`、`DELETE …/{backup_id}`）进系统状态页的「数据备份」卡，实测
缺口 15 → **11**、已消费 60 → **64**、前端调用形状同涨到 64。四条都归 CLI 的 `backup`
一个子命令（`--action create|restore|list|delete`），所以 `CLI_ENTRY` 14 收到 **10** 条、
不同子命令 9 收到 **8** 个（`backup` 整名出名单）；`NO_ENTRY` 仍 **1** 条（`translate`，
归 A283）。缺口 11 = 10 条有 CLI 入口 + 1 条没有。接线质量判据住在
`tests/unit/test_backup_wiring_l231.py`（四条调用形状与契约面逐键对账、TS 响应面与
pydantic 模型逐键相等、页面口径与实测判决一致、SDK 那道备份名越界闸真的会红）；
这一批同时量出七条缺口立为 A298~A304，并当场收掉一条**安全缺陷**（`name="../../x"`
此前是 200 且真写到备份目录之外）。这批还第一次动到 query 那一维：四条都带
`backup_dir`，`params` 块从 4 条涨到 **8** 条。

**L232 接了第八批**：数据集血缘那一族三条（`GET` / `POST /api/system/dependency/datasets`、
`GET /api/system/dependency/graph`）进系统状态页的「数据集血缘」卡，实测缺口 11 → **8**、
已消费 64 → **67**、前端调用形状同涨到 67。三条都归 CLI 的 `dependency` 一个子命令
（`--action register|list|graph|validate`），所以 `CLI_ENTRY` 10 收到 **7** 条、不同子命令
8 收到 **7** 个（`dependency` 整名出名单，与 L231 的 `backup` 同形）；`NO_ENTRY` 仍 **1** 条
（`translate`，归 A283）。缺口 8 = 7 条有 CLI 入口 + 1 条没有。接线质量判据住在
`tests/unit/test_lineage_wiring_l232.py`；这一批同时量出八条缺口立为 A305~A312，并当场收掉
一条**静默丢字段**（`POST` 的 `description` / `tags` 此前只在响应里回显、随后 GET 与磁盘上
都是空，因为端点在 `register_dataset()` 落盘**之后**才改内存对象）。
query 那一维这批再添三条 `registry_path`，`params` 块从 8 条涨到 **11** 条、去重后键名 7 → **8**。

**L234 接了第九批**：系统那一族五条（monitor / auto-test / migrate / stream / validate-config）
进页面，实测缺口 8 → **3**、已消费 67 → **72**；五个名字整名出 `CLI_ENTRY`（7 收到 2）。
同轮用户拍板**撤销 L226 那条「指标涨而能力不涨的接线一律不做」的裁定**：
`GET /api/system/dependencies` 与 `POST /api/dataset/translate` 两员都要接，目标是 75/75
（`translate` 先收 A283 的契约层问题再接）。所以本节此前写的「仍拒接」不再是现行裁定。

**L235 接了第十批**：`POST /api/quality/health-gate` 进质量中心的「健康门禁」卡，实测缺口
3 → **2**、已消费 72 → **73**、前端调用形状同涨到 73。它自有同名 CLI 子命令，于是
`CLI_ENTRY` 2 收到 **1** 条 / **1** 个名字（只剩 `dependencies` → `doctor`），`NO_ENTRY` 仍 **1** 条
（`translate`，归 A283）。这一条**不带 query 参数**，所以 params 那一维一个数没动
（仍 11 条 / 8 个键名）。接线质量判据住在 `tests/unit/test_health_gate_wiring_l235.py`；
这一批同时量出三条新账：**A319**（权重不校验非负与上界，`health_score` 也不钳 [0,1]，
实测一套和为 1 的负权重把健康分顶到 1.1697 / healthy，而门禁三条规则根本不看权重 ⇒
「healthy 且 failed」同时为真）、**A320**（权重之和的容差写成 `abs(sum-1.0) > 0.01`，
恰好 0.01 那一档判不出来：0.99 与 1.01 都 400，1.005 才 200）、**A321**（`text_field`
送空串或送一个数据集里不存在的键，后端零判决，`duplicate_rate` 回 1.0 —— 与「真的全重复」
在响应里同形）。三条都写在页面上那句「读这一页之前要知道的六件事」里，不是只写在账本里。

**L236 收了最后两员，缺口到 ∅**：`GET /api/system/dependencies` 与 `POST /api/dataset/translate`
双双接进页面，实测缺口 2 → **0**、已消费 73 → **75**、前端调用形状同涨到 **75** ⇒
契约面 75 条操作**全数被前端调到**。`CLI_ENTRY` 与 `NO_ENTRY` 同时清空（前者最后一名
`doctor` 随 `dependencies` 出名单，后者唯一一名 `translate` 在 A283 收口后出名单）。
这一轮**动了后端契约**：A283 ① 给 `TranslateRequest` 加可选 `output_file`、`TranslateResponse`
加同名回显字段（给了落点才非空，值是已 resolve 的绝对路径），因为「整份译文只在响应体、
每一次点击都按 token 付费却留不下东西」是契约层缺口，不先收它就只会接出一个刷新即失的
假入口——用户拍板要接，所以 A279 那条「不改后端字段名或响应形状」的纪律在**这一员上**豁免，
豁免写在这里而不是悄悄做。`dependencies` 那一员按实测记账：它与 `/api/status` 里那份诊断
**逐字节相同**（L226 的拒接理由在数据面上是成立的），接它涨的是**取法**不是数据——它不碰
管道（把 `get_pipeline` 换成抛异常的替身仍 200），页面也因此能把依赖诊断从 `status &&`
那道整体门里摘出来单独刷新。两条都**不带 query 参数**，params 那一维一个数没动
（仍 11 条 / 8 个键名）。接线质量判据住在 `tests/unit/test_last_two_wiring_l236.py`；
这一批量出一条新账 **A322**（`text_key` 送空串或送数据集里不存在的键，后端零判决：200、
模型调用 **0** 次、`failed=0`，产物里凭空多出 `''` / `no_such_field` 两键——与 A321 同族，
但这一条在花钱路径上，而且会写进落盘产物）。

**L243 又抬高两员，两侧一起跟着走**：备份完整性那一族加了 `GET /api/system/backups/verify`
与 `POST /api/system/backups/cleanup`（A300 收口），同轮接进系统状态页 ⇒ 契约面 75 → **77**、
路径 71 → **73**、调用形状 75 → **77**、`UNCONSUMED` 仍 ∅（新两条一上线就被消费，没留缺口）。
params 那一维**动了**：11 → **13** 条绑定、去重键名 8 → **9**（新键 `backup_id`）——与 L236
那句「params 一个数没动」是两轮各自的现量，不是同一格的两种说法。

**名单空了不等于尺子钝了**：`UNCONSUMED` 收到 ∅ 之后，判据 3 那两条验真（子命令名存在、
末段确实不是子命令）的循环体不再执行 ⇒ 单看它们恒真。guard-integrity 自查正是点名这一族
（外加 `test_frontend_face_scale` 与 `test_corpus_excludes_test_files` 两处**构造式恒真**，
以及 `test_roster_entries_are_real_operations` 那处「空集减任何集合都是空集」）。修法不是
再添一支反证就完事，而是把判据搬成四只**收名单为参数**的纯函数
（`unverified_cli_claims` / `false_no_entry_claims` / `roster_phantoms` /
`roster_partition_problems`）：主判据读现量名单，`TestRulerIsNotBlind` 的五支注入臂读
**会撒谎的名单**（假子命令、把真子命令说成无入口、契约面上不存在的操作、划分三病、
`.test.` 排除档），函数不为空就点名 ⇒ 今天这些 0 是量出来的 0，不是「什么都没扫到」的 0。
`test_unwiring_one_operation_turns_the_empty_roster_red`（摘掉一条服务层调用形状的
monkeypatch 臂）继续钉主判据那一向。

**两侧权威各只有一份**：
* 后端面 = `app.openapi()` 里 `(方法, 路径模板)` 的集合（77 条操作 / 73 条路径，
  backups 与 dependency 有同路径多方法）；
* 前端面 = `web/src/services/api.ts` 里 `api.<verb>('路径')` 的调用形状（77 条），
  前缀从同一份源码里的 `baseURL` 读出来，不在测试里写死第二份。

**判据五向**：
1. 每一条前端调用形状都必须对得上契约（**幽灵方向**，能抓拼错的端点、已删的端点、
   方法用错、以及没声明的 query 参数）；
2. 每一条后端操作要么被前端调到，要么在 `UNCONSUMED` 名单里 —— 名单与实测集合
   **相等**，所以它只会随接线变短，新增一条没接的端点会当场红；
3. `UNCONSUMED` 每一员要么指到一个**真实存在**的 CLI 子命令（`cli_entry` 的每个名字由
   parser 的 `add_parser` 名单验真），要么落进 `NO_ENTRY`（L226 时 3 条，L227 接走其中
   2 条后剩 **1** 条，L236 收口后 **∅**）：判据是「路径末段确实不在 CLI 子命令清单里」。
   两张名单都空之后这一向的循环体不再执行 ⇒ 判据搬进纯函数，牙齿由 `TestRulerIsNotBlind`
   的注入臂①②④（假子命令 / 把真子命令说成无入口 / 划分三病）在名单非空时逐格点名；
4. query 参数逐键对到该操作的 `in=query` 声明。两处来源都算：写在路径串里的
   `` `/versions/create?filename=${filename}` ``（1 条）与 axios 的 `params: {…}` 对象
   （L232 之后 11 条 / 去重后 8 个键名）。后者是量出来才补进去的一维 —— `data/load` 的分页三键与
   `page_size` 这个「TS 侧改名后的对外名」只有读 params 块才看得见。L231 那四条备份调用
   第一次让这一维撞上归属窗口：`createBackup` 原本把请求体写在调用里、`params` 块离调用
   超过 60 字符，于是它的 `backup_dir` **绑不上**（实测：四条里唯一漏的一员）。修法是改
   源码（把请求体提到上一行，间隔收到 14 字符），不是放宽窗口 —— 尺子看不见一格就是看不见。
5. 一个形状同时落在字面量路由和参数化路由上时（实测 1 例：`GET /api/versions/history`），
   绑定只认占位符最少的那一档，并拿运行时的 200 + 响应体形状证明这一档确实是赢的那条。
   不做这档收敛，字面量调用会把参数化路由也算成已消费，缺口往少里报错一格。

**没做的一条判据，理由按实测写**：想再加「CLI 子命令与端点做的是同一件事」这一维，做法是
比两边的旋钮命名。**立账轮 L224 当时**实测 26 条有 CLI 入口的映射里只有 **15** 条存在 dest/字段名交集
（CLI 侧叫 `--input` / `--output`，API 侧字段是 `input_file` / `output_file`），硬做交集
会把另外 11 条明明有入口的操作错报成无入口 —— 那是一把会撒谎的尺子，所以本轮只验
「名字存在」，把「语义等同」留在 A279 的散文里，不当判据。
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))

API_TS = ROOT / "web" / "src" / "services" / "api.ts"
FRONT_SRC = ROOT / "web" / "src"

#: `api.get('...')` / `` api.post(`/x/${id}`, ...) `` —— 只认显式的路径字面量或模板串首段
CALL = re.compile(r"\bapi\.(get|post|put|delete|patch)\(\s*[`'\"]([^`'\"]*)[`'\"]")
BASE_URL = re.compile(r"""baseURL:\s*['"](/api)['"]""")
SUBCOMMAND = re.compile(r"add_parser\(\s*[`'\"]([a-z0-9_\-]+)[`'\"]")

#: axios 的查询对象 `params: { page, page_size: pageSize }`；块内不许再套花括号
PARAMS_BLOCK = re.compile(r"params:\s*\{([^{}]*)\}")

#: params 块到所属调用的字符间隔上限。现量 4~41（L231 之前是 4~14；`restoreBackup` 那条
#: 把请求体写在调用里，间隔 41）。这一维今天不咬人：把上限放到无穷，
#: 归属仍是 14 条 —— 语料里每个 `params:` 块之前都站着另一条调用，「最近前一员」自己就
#: 收对了。上限守的是另一种还没出现的形状：一个不属于任何调用的 `params:` 块（比如最后
#: 一条调用之后的配置对象）没有上限就会被记到最后一员头上。下界有牙齿：收到 3 就把
#: 14 条真归属全判没（见 `TestTheParamsWindowIsScoped`）。
#: 60 这个数也**真的会咬人**：L231 的 `createBackup` 原本间隔超限 ⇒ `backup_dir`
#: 绑不上，处置是改源码把请求体提上去，不是放宽这里。
PARAMS_MAX_GAP = 60

#: 前端调用形状里的 `${...}` 归一化成的占位符
PLACEHOLDER = "{}"

#: 实测未被前端消费的操作（L236：75 条操作**全数被调到**，缺口 ∅；立账轮 L224 是 32，
#: L225 第一批后是 29，L226 第二批后是 26，L227 第三批后是 24，L228 第四批后是 21，
#: L229 第五批后是 18，L230 第六批后是 15，L231 第七批后是 11，L232 第八批后是 8，
#: L234 第九批后是 3，L235 第十批后是 2）。
#: 每员要能在 `CLI_ENTRY` 或 `NO_ENTRY` 里落位；接线一轮就少一格，回填只降不升。
#: **空名单不是「不用管了」**：判据是集合相等，所以新接一条没接的端点仍会当场红
#: （实测由 `TestRulerIsNotBlind` 那支反证钉住，见其 docstring）。
UNCONSUMED = frozenset()

#: 缺口里「CLI 那条路有入口」的成员 → 子命令名。名字由 `TestCliEntryClaimsAreReal` 验真。
#: L234 把 monitor / auto-test / migrate / stream / validate-config 五员接进页面后收到 **2**；
#: L235 把 health-gate 接进质量中心后收到 **1**（只剩 `dependencies` → `doctor`）；
#: L236 把 `dependencies` 接进系统状态页后**清空**——那一名 `doctor` 是这条名单上最后
#: 一员，它的接法按用户拍板做（数据与 `/api/status` 里那份逐字节相同，涨的是取法不是数据，
#: 实测记账见模块 docstring 的 L236 段）。
CLI_ENTRY = {}

#: 两边都没有入口的操作。L225 把 evaluate / language-profile / vector-status 接进前端，
#: 队列从 6 收到 3；L227 把 impact / semantic-search 接进分析页，收到 1；L236 收掉最后
#: 一员 `translate`——它不是「再等一轮接线」，而是先做 A283 ①（给请求加可选 `output_file`、
#: 响应回显已 resolve 的绝对落点）让这次按 token 付费的请求留下东西，再接进页面。
#: 判据取「路径末段确实不是 CLI 子命令」；名单空了之后这一向由反证那一支提供牙齿。
NO_ENTRY = frozenset()


def openapi():
    from api.main import app
    return app.openapi()


def operations():
    """`{(方法, 路径模板)}` —— 后端契约的完整操作面"""
    return {
        (m.upper(), p)
        for p, ops in openapi()["paths"].items()
        for m in ops
    }


def base_prefix():
    """服务层的 baseURL，从源码读而不是在测试里写第二份"""
    matches = BASE_URL.findall(API_TS.read_text(encoding="utf-8"))
    assert len(matches) == 1, f"baseURL 应恰好出现一次，实得 {len(matches)} 次：{matches}"
    return matches[0]


def normalize(raw):
    """`` `/versions/${versionId}/data?limit=${n}` `` → `('/versions/{}/data', ('limit',))`"""
    path, _, query = raw.partition("?")
    path = re.sub(r"\$\{[^}]*\}", PLACEHOLDER, path)
    keys = tuple(sorted(
        k.split("=")[0] for k in query.split("&") if k and not k.startswith("{")))
    return "/api" + path if not path.startswith("/api/") else path, keys


def call_shapes():
    """服务层发出的调用：`{(方法, 路径, query 键)}`

    query 键有两处来源，都要算：写在路径串里的 `?a=${x}`，和 axios 的 `params: {…}` 对象。
    只认前一种会漏掉现量 4 条调用 / 5 个不同键 —— 那批键里 `page_size` 恰好是 TS 侧改名
    （`page_size: pageSize`）后对外的真名，也就是契约里 `in=query` 声明的那个。
    """
    text = API_TS.read_text(encoding="utf-8")
    extra = {(m, p): keys for m, p, keys, _ in params_bindings()}
    out = set()
    for verb, raw in CALL.findall(text):
        path, keys = normalize(raw)
        merged = set(keys) | extra.get((verb.upper(), path), set())
        out.add((verb.upper(), path, tuple(sorted(merged))))
    return out


def params_bindings():
    """把每个 `params: {…}` 块绑到**最近的前一个** `api.<verb>(` 调用上

    返回 `[(方法, 路径, 键集合, 与所属调用的字符间隔)]`。为什么不看调用之后固定 N 个
    字符：实测那种窗口（400）会把 `/versions/history` 的 `limit` 同时算进 rollback 与
    DELETE version 两条根本没有 params 的调用 —— 候选从 4 涨到 7，多出的是三桩冤案。
    换成「最近的前一个调用」之后实测收回 4 条，间隔 4~14 字符。
    """
    text = API_TS.read_text(encoding="utf-8")
    calls = list(CALL.finditer(text))
    merged = {}
    for block in PARAMS_BLOCK.finditer(text):
        prev = [c for c in calls if c.end() <= block.start()]
        if not prev:
            continue
        call = prev[-1]
        gap = block.start() - call.end()
        if gap > PARAMS_MAX_GAP:
            continue
        keys = {k.split(":")[0].strip() for k in block.group(1).split(",") if k.strip()}
        if not keys:
            continue
        shape = (call.group(1).upper(), normalize(call.group(2))[0])
        merged.setdefault(shape, [set(), 0])
        merged[shape][0].update(keys)
        merged[shape][1] = max(merged[shape][1], gap)
    return [
        (method, path, keys, gap)
        for (method, path), (keys, gap) in sorted(merged.items())
    ]


def front_source_files(root=None):
    """前端源码语料（不含测试替身）——接线判据不吃测试替身，L211 的教训

    `root` 这一格是自查之后添的：排除档若只能读 `web/src`，「测试替身被排掉了」这件事就
    永远只有空读数。注入臂⑤拿它读一份临时目录（`Panel.tsx` 收、`Panel.test.tsx` 排），
    当场证明这道过滤真的在排东西。

    测试替身有两种命名：`*.test.*`，以及 `web/src/test/` 下的共享夹具
    （`setup.ts` / `valid-response.ts`）。只按 `.test.` 过滤会把夹具算成产品文件，
    让「产品语料规模」这个 ratchet 虚涨 —— Z-FULL 加 `test/valid-response.ts` 时
    就出现过这种假位移。
    """
    out = []
    base = root or FRONT_SRC
    for path in sorted(base.rglob("*")):
        if not path.is_file() or path.suffix not in {".ts", ".tsx", ".vue", ".js"}:
            continue
        rel = path.relative_to(base)
        if "__pycache__" in path.parts or "test" in rel.parts or "__tests__" in rel.parts:
            continue
        if ".test." in path.name:
            continue
        out.append(path)
    return out


def cli_subcommands():
    src = (ROOT / "augmentor" / "cli" / "parser.py").read_text(encoding="utf-8")
    return set(SUBCOMMAND.findall(src))


def match_operation(template, actual):
    """路径模板 ↔ 前端实际路径（`{param}` 与 `${...}` 逐段对上）

    契约里是 `{param}` 的那一段允许任意非空实段；反过来，前端放了变量而契约那一段是
    常量，就是拼错 —— 拦的正是「`/analyze/${f}` 写成了 `/analyze/x`」这一类。
    """
    t_parts, a_parts = template.split("/"), actual.split("/")
    if len(t_parts) != len(a_parts):
        return False
    for t, a in zip(t_parts, a_parts):
        if t.startswith("{") and t.endswith("}"):
            if not a:
                return False
        elif a == PLACEHOLDER or t != a:
            return False
    return True


def query_keys_of(method, path):
    spec = openapi()
    op = spec["paths"][path][method.lower()]
    return {p["name"] for p in op.get("parameters") or [] if p.get("in") == "query"}


def raw_matches(method, path):
    """逐段匹配命中的全部契约操作（不做去歧义）—— 只给歧义判据用"""
    return {(om, op) for om, op in operations()
            if om == method and match_operation(op, path)}


def bind(method, path):
    """前端一条调用形状绑到的契约操作：**占位符最少的那一档**才算数

    `GET /api/versions/history` 逐段匹配时同时符合 `GET /api/versions/{version_id}`，
    但运行时字面量路由赢（`api/routes/version.py` 把 history 声明在第 157 行、参数化
    那条在 171 行，Starlette 按声明顺序取先命中的路由，实测 200 且响应体是
    `{"history": […]}`）。不做这档收敛，字面量调用会把参数化路由也算成「已消费」，
    缺口少报一格 —— 尺子往松的方向错，正是最不能接受的方向。
    """
    hits = raw_matches(method, path)
    if not hits:
        return set()
    fewest = min(op.count("{") for _, op in hits)
    return {(om, op) for om, op in hits if op.count("{") == fewest}


def consumed_operations():
    return {op for m, p, _ in call_shapes() for op in bind(m, p)}


def unconsumed_operations():
    return {f"{m} {p}" for m, p in operations() - consumed_operations()}


def ghosts():
    """对不上契约的前端调用（拼错/已删/方法错）"""
    return sorted(
        (m, p, keys) for m, p, keys in call_shapes()
        if not bind(m, p)
    )


def undeclared_queries():
    """前端塞了 query 键但契约没声明的调用"""
    out = []
    for method, path, keys in sorted(call_shapes()):
        if not keys:
            continue
        bound = bind(method, path)
        if len(bound) != 1:
            out.append((method, path, keys, f"绑定到 {len(bound)} 条操作"))
            continue
        declared = query_keys_of(*next(iter(bound)))
        extra = set(keys) - declared
        if extra:
            out.append((method, path, sorted(extra), f"该操作声明的是 {sorted(declared)}"))
    return out


#: ---------------------------------------------------------------------------
#: 名单判据（guard-integrity 自查之后重写的这一族）
#: 三张名单在 L236 之后同时为空集，于是「遍历名单逐项验真」那几支的循环体一次都不执行，
#: `assert not <空集>` 从此恒真。判据搬进下面的纯函数，函数**收到什么名单就验什么名单**，
#: 于是同一只函数既能读现量（空 ⇒ 空问题清单）也能读注入（非空 ⇒ 逐格点名喊红）。
#: ---------------------------------------------------------------------------

def unverified_cli_claims(cli_entry, names=None):
    """`CLI_ENTRY` 那句「CLI 那条路有入口」的验真：回点名清单（子命令名并不存在的那些）"""
    names = cli_subcommands() if names is None else set(names)
    return sorted({cmd for cmd in cli_entry.values()} - names)


def false_no_entry_claims(no_entry, names=None):
    """`NO_ENTRY` 那句「两边都没入口」的验真：路径末段其实是真子命令的成员"""
    names = cli_subcommands() if names is None else set(names)
    return sorted(entry for entry in no_entry
                  if entry.split("/")[-1].split("{")[0].rstrip("-") in names)


def roster_phantoms(roster, ops=None):
    """名单里躺着契约面上并不存在的操作（端点改名或删除而名单没同步）"""
    ops = {f"{m} {p}" for m, p in operations()} if ops is None else set(ops)
    return sorted(set(roster) - set(ops))


def roster_partition_problems(measured, cli_entry, no_entry):
    """三张名单与**实测缺口**的对账：漏分类、双分类、名单过期，三类问题逐个点名

    读的是 `unconsumed_operations()`（实测）而不是 `UNCONSUMED` 常量本身 ⇒ 契约面新增
    一条没接的端点时，这一支会立刻报「既不在 CLI_ENTRY 也不在 NO_ENTRY」，而不是继续
    对三个空集做集合运算。
    """
    measured, claimed_cli, claimed_no = set(measured), set(cli_entry), set(no_entry)
    problems = []
    for label, group in (
            ("漏分类（缺口没落进任何一档）", measured - claimed_cli - claimed_no),
            ("双分类（同一员既算有入口又算无入口）", claimed_cli & claimed_no),
            ("名单过期（名单里但实测缺口没有：已接好或已消失）",
             (claimed_cli | claimed_no) - measured)):
        if group:
            problems.append(f"{label}: {sorted(group)}")
    return problems


class TestRulerSeesTheRealSurface:
    """先确认尺子读到了两侧的真实现场"""

    def test_backend_face_scale(self):
        ops = operations()
        assert len(ops) == 77, f"契约操作数现量 77，实得 {len(ops)}"
        assert len({p for _, p in ops}) == 73, "路径数现量 73"

    def test_frontend_face_scale(self):
        """形状数现量 **78**：L251 起它第一次比操作数（77）多一格

        多出来的那一格是 `GET /api/data/list` 的两档调用（默认档裸路径 + 宽档把
        `all_files=true` 写在路径串里）。两侧的对账判据走的是「每条形状能不能绑到一条操作」
        与「每条操作有没有被绑到」，都不要求形状数与操作数相等，所以这一格差 1 不是缺口，
        而是这张表第一次出现「一只操作、两种吃法」。它钉在这里是因为不钉的话，下一个加档的人
        会把 78 当成漂移改回来。
        """
        shapes = call_shapes()
        assert len(shapes) == 78, f"服务层调用形状现量 78，实得 {len(shapes)}"
        # 旧写法是 `<= {"GET","POST","PUT","DELETE","PATCH"}`，而 CALL 那条正则本身只可能
        # 匹到这五个动词 ⇒ 构造式恒真。换成与契约面（另一侧权威）对账，才有牙齿。
        assert {m for m, _, _ in shapes} <= {m for m, _ in operations()}, \
            "前端发了契约面上没有的方法"

    def test_http_paths_live_only_in_the_service_layer(self):
        """页面与组件里不许再出现第二处 `api.<verb>('/...')` 形状

        路径只住在 `api.ts` 一个文件里，这把尺子的语料才只有一份；哪天页面直接拼路径，
        消费面就会变成两处，本判据当场红。
        """
        outside = {}
        for path in front_source_files():
            if path == API_TS:
                continue
            found = CALL.findall(path.read_text(encoding="utf-8"))
            if found:
                outside[str(path.relative_to(ROOT))] = len(found)
        assert not outside, f"这些文件绕过服务层直接拼路径：{outside}"

    def test_corpus_excludes_test_files(self):
        """语料口径本身：23 只产品文件在场（含 `api.ts`），26 只测试替身被排除在外

        旧写法里的第二支 `assert not any(p == API_TS for p in tests)` 是构造式恒真 ——
        `tests` 是按 `.test.` 筛出来的，而 `API_TS` 叫 `api.ts`，两条名单永不相交。
        改成读**排除是否真的排掉了东西**：两侧规模各自钉住，且在场判据直接点名服务层。

        22 → **23** 是 Z 批次（zod 运行时校验）的连带位移：产品侧多一只
        `web/src/schemas/main-schemas.ts`。Z-FULL 又添了 `schemas/full-schemas.ts`
        （产品）与 `test/valid-response.ts` + `test/valid-response.test.ts`（测试替身）。
        产品数仍是 **23**：`test/` 目录下的共享夹具不是产品文件，不能拿来抬 ratchet。
        测试替身 23 → **26** = 24 只 `*.test.*` + `test/setup.ts` + `test/valid-response.ts`。
        """
        assert len(front_source_files()) == 23, \
            f"前端产品语料现量 23 只，实得 {len(front_source_files())} 只 ⇒ 口径变了"
        assert API_TS in front_source_files(), "服务层那一份必须在这把尺子的语料里"
        excluded = [p for p in FRONT_SRC.rglob("*")
                    if p.is_file() and p.suffix in {".ts", ".tsx", ".vue", ".js"}
                    and (".test." in p.name or "test" in p.relative_to(FRONT_SRC).parts)]
        assert len(excluded) == 26, f"被排除的测试替身现量 26 只，实得 {len(excluded)} 只"


class TestNoGhostCalls:
    """幽灵方向：前端发出的每一条调用都必须在契约里，且 query 键要声明过"""

    def test_every_call_matches_an_operation(self):
        bad = ghosts()
        assert not bad, f"这些前端调用对不上任何契约操作（拼错/已删/方法用错）：{bad}"

    def test_every_query_key_is_declared(self):
        bad = undeclared_queries()
        assert not bad, f"这些调用带了契约里没声明的 query 键：{bad}"

    def test_at_least_one_call_carries_a_query_key(self):
        """query 那一支判据不能空转：两处来源合起来现量 **15** 条

        **1 条写在路径串里**（L251 的 `/data/list?all_files=true`；L277 把原先另一条
        `/versions/create?filename=…` 改走了 `params`，含 `&` / `=` / `#` 的文件名拼串会拆坏
        query），14 条走 `params: {…}` 对象（`data/load` 3 键、`data/update` 与 `data/delete`
        各 `index`、
        `versions/history` 各 `limit`，L231 添备份那一族四条各 `backup_dir`，L232 再添数据集血缘
        那一族三条各 `registry_path`，**L243 添备份完整性的两员**：`cleanup` 一只 `backup_dir`、
        `verify` 一只 `{backup_id, backup_dir}`）；去重后 **10** 个键名（L251 添 `all_files`）。
        """
        with_query = [s for s in call_shapes() if s[2]]
        assert len(with_query) == 15, f"带 query 的调用现量 15 条，实得 {len(with_query)} 条"
        assert len({k for _, _, keys in with_query for k in keys}) == 10
        assert len(params_bindings()) == 14, "params 对象那一支现量 14 条"
        assert sum(1 for _, _, keys, _ in params_bindings() if keys == {"backup_dir"}) == 5
        # L232 那三条：两条 GET 的间隔是 8，POST 那条因为请求体先提成 `const body` 是 14
        assert sum(1 for _, _, keys, _ in params_bindings() if keys == {"registry_path"}) == 3


class TestTheParamsWindowIsScoped:
    """`params: {…}` 的归属判据本身：绑到最近的前一个调用，而不是往后看固定字符"""

    def test_no_params_block_is_claimed_by_a_bare_call(self):
        """`api.get('/data/list')` 后面紧跟 load 的 params —— 不许记到它头上

        这条是「窗口收紧」的牙齿：按「调用之后 400 字符」那种取法实测会多出 3 桩冤案
        （`/data/list` 白得 3 个键、rollback 与 DELETE version 各白得一个 `limit`），
        候选从 4 涨到 7。收紧到「最近前一员 + 间隔 ≤ 60 字符」后实测 4 条，最大间隔 14。

        **L251 把判据从合并面挪到归属面**：宽档那次改动让 `/api/data/list` 这一条路径上
        同时存在两员调用（默认档裸调用 + 宽档把 query 写在路径串里），`call_shapes()` 是集合，
        `{(m, p): keys for …}` 这一格取到 `()` 还是 `('all_files',)` 取决于集合迭代顺序，而顺序
        带 hash 随机化 ⇒ 同一份代码在 `PYTHONHASHSEED=0/1` 绿、`2/3/7/11` 红（六档实测）。
        冤案判据量的本来是「`params` 块有没有被错绑」，那就直接读归属来源，不读合并后的字典。
        """
        bound = {(m, p) for m, p, _, _ in params_bindings()}
        for shape in (("GET", "/api/data/list"), ("POST", "/api/versions/{}/rollback"),
                      ("DELETE", "/api/versions/{}")):
            assert shape not in bound, f"这条裸调用白得了别人的 params 块：{shape}"
        # 合并面继续钉，但钉成集合而不是「同一路径的最后一格」：两档都在，各回各的键
        listed = {keys for m, p, keys in call_shapes() if (m, p) == ("GET", "/api/data/list")}
        assert listed == {(), ("all_files",)}, listed

    def test_the_gap_budget_is_load_bearing(self):
        """上限的两端都量过：收到 3 判没 14 条，放到无穷今天不多收一条

        现量最大间隔 41 字符（`restoreBackup`：请求体写在调用里，把 `params` 块推远了），
        所以 60 这个数留的是余量而不是判据。真正在收归属的是
        「最近的前一个调用」；上限守的是尚未出现的形状（不属于任何调用的 `params:` 块）。
        这句写在这里是因为不写就会让人以为放宽上限等于放宽判据 —— 实测不是。
        反方向倒是咬过一次人：`createBackup` 原本间隔超过 60 ⇒ 那条 `backup_dir` 绑不上，
        处置是改源码（把请求体提到上一行，间隔 14），不是把这里放宽。L232 的
        `registerDependencyDataset` 请求体有四个键、比 `createBackup` 更长，所以照同一处方
        先把 body 提成 `const body`，实测间隔 14 —— 这次是**先躲开**而不是先踩再修。
        **L243 的 `cleanupBackups` 走同一条处方**（请求体先提成 `const body`，间隔 14），
        没有重踩一遍 `createBackup` 那只坑。
        **归属那一维现量 13 → 14**：L277 把 `createVersion` 的 `filename` 从路径串改走
        `params`（含 `&` / `=` / `#` 的文件名会把 query 拆开），于是 params 对象多收一条、
        写在路径串里的那一档从 2 条收到 1 条。本轮只按实测回填数字，判据一格没松。
        """
        gaps = [gap for _, _, _, gap in params_bindings()]
        assert len(gaps) == 14 and max(gaps) <= 41, f"归属间隔现量 ≤41，实得 {gaps}"
        original = PARAMS_MAX_GAP
        try:
            globals()["PARAMS_MAX_GAP"] = 3
            assert params_bindings() == [], "收到 3 应把 14 条真归属全判没"
            globals()["PARAMS_MAX_GAP"] = 10 ** 6
            assert len(params_bindings()) == 14, "放宽上限今天不多收：语料里每块前面都有调用"
        finally:
            globals()["PARAMS_MAX_GAP"] = original

    def test_params_keys_are_checked_against_the_declaration(self):
        """7 个键逐个能在契约 `in=query` 里找到，一个都不许多"""
        for method, path, keys, _ in params_bindings():
            bound = bind(method, path)
            assert len(bound) == 1, f"{method} {path} 绑定到 {bound}"
            declared = query_keys_of(*next(iter(bound)))
            assert set(keys) <= declared, f"{method} {path}: {keys} 不在 {declared} 里"
        assert "page_size" in query_keys_of("GET", "/api/data/load/{filename}")


class TestShadowedRoutes:
    """字面量路由与参数化路由同时能吃下同一形状时，绑定只认占位符最少的那一档"""

    def test_exactly_one_shape_is_shadowed(self):
        """67 条形状里实测只有 1 条落在两条操作上：`GET /api/versions/history`

        L231 那四条备份调用没有再添歧义：`/api/system/backups` 与
        `/api/system/backups/{backup_id}` 段数不同，逐段匹配撞不上（实测 shadowed 仍是 1 条）。
        """
        shadowed = [(m, p) for m, p, _ in call_shapes() if len(raw_matches(m, p)) > 1]
        assert shadowed == [("GET", "/api/versions/history")], shadowed

    def test_binding_prefers_the_literal_route(self):
        assert raw_matches("GET", "/api/versions/history") == {
            ("GET", "/api/versions/history"), ("GET", "/api/versions/{version_id}")}
        assert bind("GET", "/api/versions/history") == {("GET", "/api/versions/history")}

    def test_the_literal_route_actually_wins_at_runtime(self):
        """绑定的依据必须是运行时行为，不是我的匹配规则：实测 200 且响应体是历史清单

        判据住在这里的原因：`api/routes/version.py` 里 history 声明在第 157 行、
        `/api/versions/{version_id}` 在 171 行，Starlette 按声明顺序取先命中的路由。
        哪天有人把两条调换，`getVersionHistory()` 会被参数化路由吃掉并拿 4xx ——
        这条当场红，而不是等用户在版本页看到空列表。
        """
        from fastapi.testclient import TestClient
        from api.main import app
        from api.routes.version import router

        # 注：`app.routes` 在这版 Starlette 里不展平（included router 是 `_IncludedRouter`
        # 占位），所以声明顺序从路由对象自己的 routes 列表读。
        order = [getattr(r, "path", None) for r in router.routes]
        assert order.index("/api/versions/history") < order.index(
            "/api/versions/{version_id}"), "声明顺序是本轮判据的前提"
        response = TestClient(app).get("/api/versions/history")
        assert response.status_code == 200, response.text
        assert "history" in response.json()

    def test_the_param_route_is_consumed_on_its_own_terms(self):
        """去歧义没把参数化路由判成未消费：它由 `/versions/${versionId}` 那条形状自己认领"""
        assert bind("GET", "/api/versions/{}") == {("GET", "/api/versions/{version_id}")}
        assert "GET /api/versions/{version_id}" not in set(UNCONSUMED)


class TestConsumptionRoster:
    """主判据：缺口只认这份名单，而且名单只许变短"""

    def test_unconsumed_matches_the_roster(self):
        measured = unconsumed_operations()
        assert measured == set(UNCONSUMED), (
            f"实测未消费与名单不符 —— 新增 {sorted(measured - set(UNCONSUMED))}，"
            f"已接好或已消失 {sorted(set(UNCONSUMED) - measured)}")

    def test_roster_scale_is_pinned(self):
        """缺口 0 / 已消费 75 / 总操作 75：接线一轮就要回填这三个数（缺口只降不升）

        L236 收到 ∅ 之后这一格的意义变了：从前它钉的是「还剩几条没接」，现在它钉的是
        **「不许再退回」**——契约面新增一条端点而服务层没接，`consumed_operations()` 就
        少于 77、`unconsumed_operations()` 就非空，两个数一起红。
        """
        assert len(UNCONSUMED) == 0, len(UNCONSUMED)
        assert len(consumed_operations()) == 77, len(consumed_operations())
        assert len(consumed_operations()) == len(operations()), "契约面全数被消费才算收口"

    def test_roster_entries_are_real_operations(self):
        """名单与实测缺口都不许躺着契约面上不存在的操作

        旧写法 `ghost = set(UNCONSUMED) - ops` 在名单为空时是恒真的（空集减任何集合都还是
        空集）。现在把**实测缺口**也喂进同一只判据：契约面新增一条没接的操作，它先进
        `unconsumed_operations()`，这一格与名单相等那一格一起红。
        """
        assert roster_phantoms(UNCONSUMED) == [], roster_phantoms(UNCONSUMED)
        assert roster_phantoms(unconsumed_operations()) == [], \
            "实测缺口里出现了契约面上不存在的操作（绑定或归一化口径变了）"


class TestCliEntryClaimsAreReal:
    """缺口名单每一员的「CLI 那条路有入口」这句话都得验真

    **guard-integrity 自查点名了这一族的三处恒真**：L236 之后 `CLI_ENTRY` 与 `NO_ENTRY`
    同时为空集，于是「遍历名单逐名验真」的循环体一次都不执行，`assert not <空集>` 与
    `空集 | 空集 == 空集` 从此永远绿。判据搬进三只纯函数（`unverified_cli_claims` /
    `false_no_entry_claims` / `roster_partition_problems`），主判据继续读现量名单，
    `TestRulerIsNotBlind` 里那几支注入臂把**会撒谎的名单**喂进同一只函数——函数不为空
    就说谎，这一族的牙齿不再依赖名单恰好是空的。
    """

    def test_every_mapped_subcommand_exists(self):
        names = cli_subcommands()
        assert len(names) == 37, f"parser 的 add_parser 名单现量 37，实得 {len(names)} ⇒ 判据坏了"
        bad = unverified_cli_claims(CLI_ENTRY)
        assert not bad, f"这些 CLI 子命令名并不存在：{bad}"

    def test_no_entry_members_really_have_no_subcommand(self):
        """`NO_ENTRY` 的牙齿：路径末段确实不在子命令清单里"""
        bad = false_no_entry_claims(NO_ENTRY)
        assert not bad, f"这些成员声称无 CLI 入口，但末段是个真子命令：{bad}"

    def test_roster_partitions_into_two_claims(self):
        """两档划分读的是**实测缺口**，不是三张空名单互相对账"""
        problems = roster_partition_problems(unconsumed_operations(), CLI_ENTRY, NO_ENTRY)
        assert problems == [], problems

    def test_cli_roster_scale(self):
        """37 个子命令是 L204 钉过的现量；名单 7 条指向其中 7 个不同子命令

        L225 只动前端，`CLI_ENTRY` 那一半一个数没变（26 条 / 21 个名字）；变的只有
        `NO_ENTRY`：接好 3 条后从 6 收到 3。L226 接的三条都各有 CLI 入口（`search` /
        `validate` / `auto-config`），所以这轮动的正是 `CLI_ENTRY`：26 收到 23，
        不同子命令 21 收到 18；`NO_ENTRY` 这轮一条没动。
        L227 接的两条（`impact` / `semantic-search`）经探针确认**没有** CLI 子命令
        （`grep impact|semantic` 在 `parser.py` 只命中 health-gate 的 `--text-field`），
        所以这轮动的只有 `NO_ENTRY`：3 收到 1，`CLI_ENTRY` 两个数一个没改。
        L228 接的三条（`stats` / `features` / `compare`）三条都各有同名子命令，动的又是
        `CLI_ENTRY`：23 收到 20，不同子命令 18 收到 15（三个名字各自在名单里只出现一次，
        所以条目数与名字数同减 3）；`NO_ENTRY` 与子命令现量都没动。
        L229 接的三条（`convert` / `merge` / `split`）与 L228 同形：各有同名子命令且各自
        只出现一次，`CLI_ENTRY` 20 收到 17，不同子命令 15 收到 12；`NO_ENTRY`（1）与
        子命令现量（37）都没动。
        L230 接的三条（`sample` / `aggregate` / `rag`）仍是这一形：`CLI_ENTRY` 17 收到 14，
        不同子命令 12 收到 9；`NO_ENTRY`（1）与子命令现量（37）都没动。
        L231 接的四条（备份那一族）都归 `backup` 一个子命令（`--action create|restore|list|delete`），
        于是这一个名字整名出名单：条目数 14 收到 10（减 4）、不同子命令 9 收到 8（减 1）——
        两个数**不同减**此前也发生过（`dependency` 一名占三条，当时仍在名单里；L232 整名出名单），
        但一次减掉 4 条是现量最大的一批；`NO_ENTRY`（1）与子命令现量（37）都没动。
        L232 接的三条（数据集血缘那一族）都归 `dependency` 一个子命令（`--action
        register|list|graph|validate`），于是与 L231 的 `backup` 同形：条目数 10 收到 **7**（减 3）、
        不同子命令 8 收到 **7**（减 1），减完之后条目数与名字数**又相等了**（每名各占一条）；
        `NO_ENTRY`（1）与子命令现量（37）都没动。
        L234 接的五条（系统那一族）各归**一个自己的子命令**（monitor / auto-test / migrate /
        stream / validate-config），于是五个名字整名出名单：条目数 7 收到 **2**（减 5）、
        不同子命令 7 收到 **2**（减 5）—— 一次减掉 5 个名字是现量最大的一批，且条目数与名字数
        仍然相等（每名各占一条）；`NO_ENTRY`（1）与子命令现量（37）都没动。
        L235 接的一条（`health-gate`）自有同名子命令，于是条目数 2 收到 **1**、不同子命令
        2 收到 **1**；`NO_ENTRY`（1）与子命令现量（37）都没动。名单只剩两员：
        `dependencies`（→ `doctor`，用户已拍板 L236 接掉）与 `translate`（无入口，归 A283）。
        L236 把这两员一起收掉：`dependencies` 是 `CLI_ENTRY` 的最后一名（`doctor` 整名
        出名单 ⇒ 条目数 1 收到 **0**、不同子命令 1 收到 **0**），`translate` 是 `NO_ENTRY`
        的最后一名（先做 A283 ① 的契约再接线 ⇒ 1 收到 **0**）。两张名单同时空掉，
        而子命令现量（37）一个没动——本轮没给 CLI 加子命令，`translate` 在 CLI 侧仍然
        没有入口，它出名单是因为**前端接上了**，不是因为多了一条 CLI 路。

        **guard-integrity 自查之后这一格换了形状**：两张名单同时空掉之后，
        `len(CLI_ENTRY) == 0` 这种常数钉子只钉得住「不许退回」，量不到现场。现在这一格
        读的是**实测缺口规模**——名单规模必须等于缺口规模：契约面新增一条没接的操作而
        名单没跟上，当场红；名单自己长回去（把已接好的操作留在册上）同样红。上面那段
        历史保留不动，是因为回填这三个数的人需要知道每一批为什么动 `CLI_ENTRY` 或
        `NO_ENTRY`。
        """
        assert len(cli_subcommands()) == 37
        assert (len(CLI_ENTRY), len(NO_ENTRY)) == (0, 0), "名单回填只降不升"
        assert len(CLI_ENTRY) + len(NO_ENTRY) == len(unconsumed_operations()), \
            "名单规模与实测缺口脱钩了（接线一轮就同步回填这三个数）"

    def test_the_empty_claims_are_not_a_blind_ruler(self):
        """两张名单空了之后，上面两条验真的循环体不再执行 ⇒ 这一支替它们保留牙齿

        判据不是「名单为空」，而是「**实测**为空且尺子仍能看见东西」：
        ① parser 那 37 个子命令仍被逐名解析出来（尺子没瞎）；
        ② 契约面 75 条操作**每一条**都在服务层有对应调用形状（缺口是量出来的 0）；
        ③ 反证：把任一员的调用形状摘掉，`unconsumed_operations()` 立刻非空 ——
           空集相等这条判据仍会响，不是恒真。摘法走 monkeypatch 换掉 `call_shapes`
           的返回值，不改磁盘上一个字节。
        """
        assert len(cli_subcommands()) == 37, "parser 一个子命令都读不到 ⇒ 判据坏了"
        assert unconsumed_operations() == set()
        assert len(consumed_operations()) == len(operations()) == 77

    def test_unwiring_one_operation_turns_the_empty_roster_red(self, monkeypatch):
        """③ 的反证臂：摘掉 `translate` 的调用形状，缺口必须立刻点名它"""
        real = call_shapes()
        dropped = {s for s in real if s[1] != "/api/dataset/translate"}
        assert len(dropped) == len(real) - 1, "摘的那一条不在语料里 ⇒ 反证无效"
        monkeypatch.setattr(sys.modules[__name__], "call_shapes", lambda: dropped)
        assert unconsumed_operations() == {"POST /api/dataset/translate"}
        assert set(UNCONSUMED) != unconsumed_operations(), "相等判据在缺口非空时必须红"


class TestRulerIsNotBlind:
    """反空转：这几条判据必须能失败，而且失败得对"""

    def test_invented_path_is_a_ghost(self):
        assert match_operation("/api/dataset/convert", "/api/dataset/conver") is False
        assert match_operation("/api/data/load/{filename}", "/api/data/load/x.json")
        assert match_operation("/api/versions/history", "/api/versions/{}/data") is False

    def test_query_judgment_catches_an_invented_key(self):
        method, path = "POST", "/api/versions/create"
        declared = query_keys_of(method, path)
        assert "filename" in declared, sorted(declared)
        assert "nonsense_key" not in declared

    def test_query_judgment_uses_the_contract_name_not_the_ts_name(self):
        """`params: { page_size: pageSize }` 里对外的名字是冒号左边那半

        假如有人把键写成 `{ pageSize }`，前端发出去的就不再是契约声明的那个键，而这条
        判据必须当场抓住 —— 所以两个名字都要量：`page_size` 在声明里，`pageSize` 不在。
        """
        declared = query_keys_of("GET", "/api/data/load/{filename}")
        assert "page_size" in declared, sorted(declared)
        assert "pageSize" not in declared, sorted(declared)

    def test_consumed_and_unconsumed_partition_the_face(self):
        """两侧并起来正好是契约面，且互不相交 —— 缺一边或多一边都红"""
        ops = operations()
        cons = consumed_operations()
        uncon = {(m, p) for m, p in (o.split(" ", 1) for o in unconsumed_operations())}
        assert cons | uncon == ops, f"并集不等于契约面：{sorted(ops ^ (cons | uncon))}"
        assert not cons & uncon, f"同一员既算已消费又算缺口：{sorted(cons & uncon)}"

    def test_base_prefix_is_read_not_hardcoded(self):
        assert base_prefix() == "/api"
        assert API_TS.read_text(encoding="utf-8").count("baseURL") == 1

    def test_a_lying_cli_entry_roster_is_named(self):
        """注入臂①：`CLI_ENTRY` 现为空集，主判据那一格读不到东西 ⇒ 喂一份撒谎的名单

        同一只函数必须做到「真名放行、假名点名」，否则 `assert not unverified_cli_claims(
        CLI_ENTRY)` 就还是对空集求像的恒真判据。
        """
        names = cli_subcommands()
        assert "monitor" in names and "translate" not in names, "尺子的子命令读数面变了"
        assert unverified_cli_claims({"GET /api/system/monitor": "monitor"}, names) == []
        assert unverified_cli_claims({"GET /api/system/monitor": "monitors"}, names) == \
            ["monitors"]

    def test_a_lying_no_entry_roster_is_named(self):
        """注入臂②：把「两边都没入口」说给一个**真有**子命令的操作 ⇒ 必须反驳"""
        names = cli_subcommands()
        assert false_no_entry_claims({"POST /api/dataset/translate"}, names) == []
        assert false_no_entry_claims({"POST /api/dataset/convert"}, names) == \
            ["POST /api/dataset/convert"]

    def test_a_phantom_roster_entry_is_named(self):
        """注入臂③：名单里塞一条契约面上不存在的操作 ⇒ 必须点名"""
        assert roster_phantoms({"POST /api/dataset/translate"}) == []
        assert roster_phantoms({"GET /api/no/such/operation"}) == ["GET /api/no/such/operation"]

    def test_the_three_partition_faults_are_each_named(self):
        """注入臂④：划分的三种病各喂一次 —— 漏分类、双分类、名单过期"""
        gap = {"POST /api/system/monitor"}
        assert roster_partition_problems(gap, {"POST /api/system/monitor": "monitor"}, set()) == []
        missing = roster_partition_problems(gap, {}, set())
        assert len(missing) == 1 and "漏分类" in missing[0], missing
        stale = roster_partition_problems(set(), gap, gap)
        assert any("双分类" in p for p in stale), stale
        assert any("名单过期" in p for p in stale), stale

    def test_the_test_file_filter_actually_excludes_something(self, tmp_path):
        """注入臂⑤：临时目录里造两只同形文件，产品面读得到、测试替身必须读不到"""
        ghost = "api.get('/api/nonsense/only-in-injection')"
        (tmp_path / "Panel.test.tsx").write_text(ghost, encoding="utf-8")
        (tmp_path / "Panel.tsx").write_text(ghost, encoding="utf-8")
        corpus = front_source_files(tmp_path)
        assert [p.name for p in corpus] == ["Panel.tsx"], \
            "`.test.` 那一档没被排除 ⇒ 语料口径退化，接线判据会吃到测试替身（L211 的教训）"
        # 另一半牙齿：没被排除的那只文件确实会被读出来（不是靠「什么都读不到」绿的）
        assert CALL.findall((tmp_path / "Panel.tsx").read_text(encoding="utf-8"))
