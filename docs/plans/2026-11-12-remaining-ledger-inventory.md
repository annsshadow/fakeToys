# 各项目剩余账本 · 盘点（2026-11-12）

> **用途**：把五个子项目各自的「账本 / 台账」里**还没闭**的条目收敛成一份可执行清单，
> 逐条写清「账本位置 / 原文说了什么 / 本轮实测到哪一步 / 谁才能推进」。
> **口径**：**账本记载只作线索，结论以本轮复跑现量为准** —— 承
> `docs/plans/2026-10-29-remaining-work-plan.md` §四 那条教训
> 「账本过时的一个反例（提醒：引用前先复核）」：本轮就撞上**三处**这样的过期条目，
> 已按现量收口（见 §二「本轮清掉的账」）。
>
> **本轮做过的实测（逐项可复跑）**：
> augmentor 全量门禁 `python -m pytest tests -q -p no:cacheprovider` ⇒ **0 failed / 10551 passed / 6 skipped**
> （548.53 s；覆盖率 **99.53%**，门槛 99%）；
> augmentor 前端 `npx vitest run` ⇒ **24 文件 / 427 支全绿**（`tsc --noEmit` / `eslint --max-warnings 0` / `prettier --check src` 同绿）；
> laiyipao 归属守卫 `go test ./internal/service/ -run "TestUserScoped…|TestOwnerScoped…|TestOwnership…"` ⇒ **3 支全绿**；
> laiyipao 依赖闸门 `govulncheck -show verbose ./...`（Go **1.26.11**）⇒ **No vulnerabilities found**；
> llm-lab `python -m pytest tests` ⇒ **130 支全绿**（本机装了 torch ⇒ 无 skip）；
> auto-checkin `npx tsx --test test/*.test.ts` ⇒ **163 支全绿**；`npm run test:ratchet` ⇒ **九档全绿**；
> 跨子项目告警门禁 `node scripts/check-warnings.mjs --only rust-lints,llm-lab,llm-lab-tests,augmentor-web,license`
> ⇒ 修一处后 **5 项检查 5 通过**（见 §三）。

---

## 一、逐项目盘点

### 1. augmentor · 优化循环账本（两卷）

| 账本 | 位置 | 剩余 | 本轮状态 |
|---|---|---|---|
| 卷一 `OPTIMIZATION_LOOP.md` | 卷一 Backlog B（功能增强） | **6 格里 2 笔本轮收口**（B1 / B8），余 B2（前半关闭）/ B3 / B5 / B7（B5 与 B7 = 卷二 A271 / A273 两族） | 本轮还动了 6 处引用落点（歧义清账）与卷一 Backlog B 的 B1 / B8 |
| 卷二 `OPTIMIZATION_LOOP_2.md` | Backlog A（质量缺口） | **32 笔**（A267…A385；**A311 / A272 / A278 已收口**） | 本轮 **L322 / L323 / L324** 三轮把全量门禁从 28 支红收到 **0 支红**；**L325 收掉 A311**（422 的错误体翻成中文同一形状）；**L327 收掉 A272 与 A278**（`multilingual` 那一节的口径写实 + A277 那一把尺子的「自由子对象白名单」一档）⇒ 32 笔 |
| 卷二 | Backlog B（功能增强） | **0 笔**（28 笔全闭） | — |

**剩余 32 笔的形状**（按定级）：

| 定级 | 笔数 | 名单 | 能不能本仓自己推进 |
|---|---|---|---|
| **M** | 27 | A267 A268 A269 A271 A273 A289 A290 A293 A296 A306 A309 A345 A346 A349 A351 A353 A354 A355 A356 A357 A358 A364 A371 A376 A381 A384 A385 | 多为「**口径要拍**」或「**要动 SDK 公开契约**」 |
| **S** | 5 | A287 A301 A308 A335 A370 | **A311 已收（L325）**；**A272 / A278 已收（L327）**；余下这 5 笔里 A308 / A370 是「要产品拍板」、A335 要动 SDK 公开契约 ⇒ **真正能自证的只剩 A287 / A301** |

> **口径说明**：这份名单是**按现量数出来的**，不是按印象（A311 收口前 S 级 8 笔、M 级 27 笔；
> A311 收口后 S 级 7 笔、M 级 27 笔 ⇒ 余 34 笔；**L327 收掉 A272 / A278 后** S 级 5 笔、
> M 级 27 笔 ⇒ 余 **32** 笔）。

**三类「本仓自己推不动」的形状**（逐条读过原文，不是印象）：

- **要产品拍板**（拍的是对外语义，自决就要改契约）：A308（`multilingual` 那一节的**写**权限：L327 只把它的口径写实、**没有**开成可写节 ⇒ 真要开是另一笔对外行为变更）、
  A287（`similarity_to_a` / `similarity_to_b` 两个恒零键：接计算还是删键 = 破坏性变更）、
  A278（OpenAPI 自由子对象要不要给最小形状）、A306（两处 `created_at` 覆盖 / 空值抹旧值）、
  A308（`{param}` 豁免把写路由一并匿名放行）、A309（默认注册表目录三份、端点那份与另两份不是同一个地方）、
  A273（`self.expander` / `self.tracker`：接消费者 = 新端点，删构造 = 缩公开属性面）、
  A271（十只 `config.*.enabled` 接进阶段入口 = **对外破坏面**）。
- **要环境/前置**（不是写代码能补的）：A268（真模型嵌入要一个装了 `sentence-transformers` 的环境）、
  A269（chromadb 后端要一个装了它的环境来定标）。
- **要动 SDK 公开契约**（⇒ 单独一轮）：A357（落点对、内容错那一族）、A335、A370。

**本轮已经推进掉的**（按账本原文逐条核过、确已闭上的那些）：A280 / A282 / A286 / A287 / A295 / A297 /
A317 / A323 / A386 / A387 / A388 那一族在 L315–L323 连续九轮里逐笔收口
⇒ 卷二 Backlog A 的**红笔**从 28 支降到 **0 支**（A311 仍是开着的，它被 L321 / L322 两轮点名为下一轮候选）。

### 2. oa4rust · 台账族

| 台账 | 位置 | 剩余 | 本轮状态 |
|---|---|---|---|
| F2-E 越权读端点台账 | `oa4rust/docs/plans/f2e-endpoint-tracker.md` | 「仍未解决的假绿项」**已清空** | 本轮按代码层复核收口（§二） |
| 2026-11-08 交接文档 | `oa4rust/docs/plans/handoff-2026-11-08.md` | §4 剩余待办 6 条 | 见下表 |
| 优化台账（Go 段 + 通用） | `docs/plans/optimization-1000-oa4rust-laiyipao.md` | B1 候选池 8 项 | 轮次槽位 R5 起认领，已连续认领到 R152 |

**交接文档 §4 的六条，逐条的前置条件**：

| 编号 | 项 | 前置条件（原文） | 本轮能不能推 |
|---|---|---|---|
| P0-1 | `x_portal` 共享读 2 条 | **已判定不可补**（§10.2）⇒ 等业务提新需求 | 不能（要产品提新需求） |
| P0-2 | `x_message_mass.creator` 从不写入 | 门禁缺「管理员攻击者档」⇒ **先设计一个能翻转门禁的断言**，再改 `mass_create` | 不能（本轮无 Postgres + Rust 全量构建窗口） |
| P1-1 | attendance 五张配置表写侧只写 `creator_person` | 先构造「写后读一致性」用例，再补写 | 同上 |
| P1-2 | Go 侧未复跑门禁 / sabotage | 需要真库 | 同上 |
| P1-3 | CI `sccache` 基础设施红 | 外部基础设施抖动 | 不能（观察即可）—— **但本轮实测：2026-11-12 main 上的红项已经不是「基础设施抖动」那一族**（见 §三） |
| P2-1 | 本地环境债 | Swagger zip / 本地 DB 的 bam 用例 / github.com:443 断网 | 不能（环境债）—— 本轮实测：github.com:443 **确凿**间歇断网（同一分钟内两次 `Invoke-RestMethod` 报「unexpected EOF」，重试可过）⇒ 该条**现量成立**；本机另**无 Postgres / 无 Docker** ⇒ P0-2 / P1-1 / P1-2 三条在本机**无法复跑**（不是「没做」，是「没有真库」） |

**另有一处本轮量到的规模差**（与 P1-2 同族，值得记下）：交接文档 §10.3 写
「读侧 701 条，其中 514 条静态初筛无门禁标记、251 条是 GET + 路径参数」，
并**自陈**「静态初筛不能证明『SQL 无归属谓词』，只能证明『没有明显的门禁调用』」
⇒ 那 251 条**不能直接当缺陷清单**，逐条定性要真请求 + 既有 kind ⇒
每一批 20–25 条、一批一 PR、带 sabotage 点名。

### 3. laiyipao · 已知边界 + 优化台账 Go 段

| 账本 | 位置 | 剩余 | 本轮状态 |
|---|---|---|---|
| 已知边界（诚实声明） | `docs/laiyipao/KNOWN_LIMITS.md` | 30 条编号里 **15 条仍开着**，其余已闭或已被判「不该做」 | 本轮收口第 17 条（§二）；**笔数按现量重数**（原文写 12 条、名单列了 14 条 ⇒ 实际 15 条，见下） |
| 优化台账 Go 段 | `docs/plans/optimization-1000-oa4rust-laiyipao.md` | R 轮次已认领到 R152 | — |

**仍开着的 15 条，按「能不能本仓自己推进」分**：

- **要凭证**（拿不到就动不了）：第 1 条（微信支付内购 / 流量主广告 —— 要 AppID 与商户号）、
  第 2 条（微信登录通道 —— `jscode2session` 已实现但客户端未接；服务端没配 AppID/Secret 时回 501）。
- **要产品拍板**（语义不是工程能定的）：第 3 条（防线挑战的胜负由客户端裁决 —— 彻底修要在服务端跑战斗引擎）、
  第 12 条（装备 Element 字段纯风味 —— 要加反应系数就是伤害公式改动，会动重放哈希）、
  第 13 条（默认构筑元素多样性 —— 要产品定「元素收益」怎么给）、
  第 23 条（`levels` 表不是玩家侧关卡来源 —— 两条候选路径都要拍板）。
- **要人工/真机**：第 4 条（真机像素级视觉未验证）、第 6 条与第 8 条（逐关平衡调优）、
  第 5 条（奖励「打得快」—— 原文已判「不做」）。
- **要引擎级改动**（双端 TS + Go）：第 15 条附带的「按波次轮换地形种类」。
- **设计意图、不是缺陷**（原文已说明）：第 7 条（油桶「集火才点燃」是有意保留）、
  第 9 条（槽位上限缺省值是重放哈希的锚点）、第 10 条（装备 `level` / `star` 实测后确认「不该做」）。
- **本轮补上的一格**（原文名单漏了它）：第 30 条（2026-10-23 那一轮**明确决定不做**的三件事 ——
  邮件 / 补偿系统、BGM、真机帧率测量，另带两个「已接通但仍有边界」的点）⇒ 它**整条开着**，
  原文的名单把它漏在外面 ⇒ 笔数从名单的 14 条改成 **15 条**。

> **口径订正（按现量，不按印象）**：这份盘点原稿写的是「30 条编号里 **12** 条仍开着」，
> 而它自己的名单列了 **14** 条 —— 两个数互相对不上。本轮把 `KNOWN_LIMITS.md` 的 30 个编号
> 逐条数过一遍：**已闭或已被判「不该做」**的是第 11 / 14 / 17 / 18 / 19 / 20 / 21 / 22 / 24 / 25 /
> 26 / 27 / 28 / 29 条（其中第 29 条自己写着「上面『仍无获取途径』这句**已经过时**」），
> **仍开着**的是 15 条（上列）⇒ 笔数取 **15**，并把漏掉的第 30 条补进名单。
> 承 `docs/plans/2026-10-29-remaining-work-plan.md` §四 那条教训：
> **账本记载只作线索，结论以本轮复跑现量为准** —— 这一格连「账本自己说的那条数」都不作数。

### 4. llm-lab

**没有独立账本**（它是教学项目，不是优化循环账本那套）。可数的只有两样：
讲义 `docs/llm-lab/`（10 篇）与测试 `llm-lab/tests/`（**130 支**，本轮实测全绿）。
交接文档 §D1 记的那条「llm-lab pytest 门禁 SKIP」已于 2026-10-30 消解（本机默认 `python` 已带 pytest）。
⇒ **本轮实测**：130 支全绿、torch 已装 ⇒ **无 skip**（README 里那行「62 passed / 10 skipped」是未装 torch 时的旧读数）。

### 5. auto-checkin

**没有独立账本**，它的「账本」是**覆盖率棘轮** `auto-checkin/coverage-baseline.json` +
`scripts/coverage-ratchet.mjs`（九档，每档只升不降）。
交接文档记的读数是「102 passed；覆盖率棘轮九档全绿」；
**本轮实测**：`npx tsx --test test/*.test.ts` ⇒ **163 支全绿**，
`npm run test:ratchet` ⇒ 九档逐档「持平」（justwoker 100% / vyceai 99.58% / agentrouter 95.51% /
newapi-client 94.58% / server 95.8% / lock 91.3% / scheduler 91.16% / runner 87.18% / login 36.76%）
⇒ **棘轮一格未掉**（棘轮只升不降 ⇒ 「持平」即达标）。

---

## 二、本轮清掉的账（两处「账本记载已过时」的收口）

三处都是同一族：**账本写下的「仍未做 / 仍未解决」，而那件事其实早就做完了（守卫早就建起来了）** ——
与 `docs/plans/2026-10-29-remaining-work-plan.md` §四 的提醒同一族。

> **教训（值得写进规矩）**：本轮三处过期条目的共同成因是「**后来那一轮做完了，账本没回写**」——做的那一轮只在自己的日志块里记了「已做」，没回头把早先那条待办改成闭合形状。⇒ **待办条目的闭合必须与「做它的那一轮」同批落笔**，否则账本会长期停在「仍未做」；本轮这三处都是靠**按现量复跑**才发现的（l224 的尺子读数、go/ast 归属守卫的读数、Rust 侧那两道门的落点），**不是靠人眼扫出来的**。

| 项目 | 账本位置 | 原文写的 | 本轮实测 | 处置 |
|---|---|---|---|---|
| **laiyipao** | `docs/laiyipao/KNOWN_LIMITS.md` 第 17 条 | 「`battle_records` 里除战报外的玩家数据没有归属校验 …… 按 `user_id` 过滤的完整性**没有结构性守卫**」 | `server/internal/service/ownership_scan_test.go`（第 78 轮）就是那道守卫：`battle_records` 登记在 `ownerScopedTables`（归属列 `user_id`），`TestOwnerScopedTableRegistryCoversSchema` 直接从 migrations 核对集合完整性；复跑读数「扫到 **106** 条按用户分区的语句，**全部**带归属过滤（豁免 **7** 处）」「schema 里 **29** 张按用户分区的表：登记 **12** 张、豁免 **3**」，7 处豁免逐条点名全是运营侧聚合 | **整条关闭**（`831094f8c`） |
| **augmentor** | 卷二 Backlog A 的 **A311** | 「缺必填字段走 FastAPI 英文 422、其余坏参数走中文 4xx」那一族双轨 | **L325 收口（自决取修法①）**：全局 `RequestValidationError` 处理器**自己渲染一句中文**（状态码仍 422，形如「参数校验失败：input_file 缺少必填字段」）；理由按 pydantic 的 `type` 查表、**不抄英文的 `msg`** ⇒ 句子恒为中文；**原值不回显** ⇒ 请求体里的非有限浮点与孤立代理对**结构上进不去响应体** ⇒ L272 量到的那一格「整个响应没了」从机制上根除。八处认 422 的用例同批重钉（含 l284 那支「合法回执逐字节不改」被本条的行为变更**作废**） | **整条关闭**（本批） |
| **augmentor** | `augmentor/OPTIMIZATION_LOOP.md` 卷一 Backlog B 的 **B1 / B8** | B1：「把 `/api/system/*`(13) 与 `/api/dataset/*`(12) 接入 UI —— `api.ts` 对二者零引用」；B8：「前端 5 个未接线服务函数 + 运行时 schema 校验」 | ① 接线队列 A279 分**十一批**（L225–L236）全部走完 ⇒ 契约面 **79** 条操作**全数**被前端调到，`test_api_endpoint_consumption_l224.py` 的 `test_roster_scale_is_pinned` 现在钉的是「已消费 79 = 契约面 79」、`test_unconsumed_matches_the_roster` 钉的是「实测缺口 == 空名单」⇒ 再有一条端点没接当场红；② 那 5 个未接线服务函数早已在对应页面真实接线，`api.wiring.test.ts` 的 `KNOWN_UNWIRED` 白名单**已清空**（那把守门仍在）；运行时 schema 校验也已落地（zod 面 + `main-schemas.conformance.test.ts` 把键集与后端契约面逐格对着） | **两格按现量收口**（本批） || **oa4rust** | `oa4rust/docs/plans/f2e-endpoint-tracker.md`「仍未解决的假绿项」 | ① `x_portal` 共享读 2 条「谓词未加」；② admin-only 表那一族「未闭环暴露」 | ① `portal_assemble_designer` 的 `designer_owned_read_gate` 就是 `creator = 会话人 OR is_admin`、NULL fail-closed，两个调用点（`portal_id_icon` / `portal_id_permission`）都走它 —— 与 2026-11-08 交接文档 §10.2 的结论一致；② 原文自己就写着「已由下节『读侧无谓词收口批』处理」，下一节已落地（`cases=345 pass=345`，sabotage 点名 12 个端点） | **本节清空**（本批） |

---

## 三、本轮查出来的三处 red CI 门禁（两处已修，一处待查）

> **为什么这一节在这里**：上面五节的盘点都是**账本里写着的**条目；本节是**账本没写、
> 但 CI 天天在报**的那一族 —— 它们同样是「账本」的一种（门禁即账本：红了就是欠账）。
> 本轮的入口是 GitHub API（`/actions/runs`，无需 token）：**2026-11-12 main 的 HEAD 上
> 三个 workflow 红**，逐个 job / step 读出来是哪一格红。

| # | workflow | 红的那一格 | 定性 | 处置 |
|---|---|---|---|---|
| 1 | `laiyipao CI` | `Dependency vulnerability gate` → **govulncheck (must find no reachable vulnerability)** | **代码可达的标准库漏洞** `GO-2026-6859` / CVE-2026-97054（`net/http` 的 `MaxBytesReader` 头部竞态 ⇒ HTTP/1 服务端在「读请求体」与「写响应头」并发时可能 panic）。本地复跑逐字：`Found in: net/http@go1.26.10` / `Fixed in: net/http@go1.26.11` / `Your code is affected by 1 vulnerability from the Go standard library`；调用链点名 `service.code2Session → http.Client.Do`、`api.startPprof → http.ListenAndServe` ⇒ **不是第三方模块，是工具链下限** | **已修**（`765489466`）：`go.mod` 一行 `go 1.26.10` → `go 1.26.11`；复跑 `govulncheck` ⇒ **No vulnerabilities found**；四份写着下限的文档同步（含两条漏洞各自的调用链） |
| 2 | `laiyipao CI` | 同一个 job 的**第二格** → **Audit gates must not regress** | 第一格修好之后，红的**挪到了第二格**（同一个 job 的下一步）⇒ 逐条读出来：`audit-gate.cjs check-overrides` 逐字点名 **9 只**「override 不在 pnpm-workspace.yaml 里了」（vue / `@vue/compiler-*` / `@vue/reactivity` / `@vue/runtime-*` / `@vue/shared`）—— **根因**：`f7d15a76e`（vue 运行时家族回 3.4.21 修 uni-h5-vue 兼容）改了 `pnpm-workspace.yaml`，**没同步** `audit-baseline.ini` 的 `[miniapp.overrides]` 名册 ⇒ **账本没回写那一族**（改的是闸门查的东西，没回头改闸门记的东西） | **已修**（`b4ed7a9e7`）：那 9 只按现值改成 **3.4.21**（`@vue/server-renderer` 保持 3.5.42）⇒ **不是放宽闸门**：闸门查「override 在不在、版本对不对」，名册记的应是**现值**；漏洞计数两格一格未动（miniapp 1 / admin 0）⇒ 复跑三格全绿（overrides 齐全 33 条 / miniapp 1 条持平 / admin 0 条持平） |
| 3 | `warnings-gate` | `Cross-subproject warning gates` → **license header（全仓源码 SPDX 标识符）** | `node scripts/check-license-headers.mjs` 读数逐字：**2448 个源码文件，缺失 1** ⇒ 点名 `augmentor/tests/unit/test_checkpoint_total_items_l295.py`（L295 那一轮新立的守卫落笔时漏了两行文件头） | **已修**（`3e351e48d`）：补上仓里通行的两行（`# Copyright (C) 2026 annsshadow` + `# SPDX-License-Identifier: AGPL-3.0-or-later`）⇒ 复跑 **缺失 0**；守卫**一字未改**，且补的是文件头 ⇒ 全仓在册定位引用零位移。**CI 侧已验证**：该 workflow 在 `765489466` 之后的每一枚都是 **success** |
| 4 | `AI platform CI` | `Backend tests & coverage` → **Run tests with coverage gate** | **长期红**（2026-11-02 起连续红 11 天，最后一次 success 是 **2026-11-01 09:40**，而**账本里没有任何一处记过它红在哪**）。本轮先铺「怎么拿到答案」（pytest 输出写进 `$GITHUB_STEP_SUMMARY` 与 **`::error::` 注解** ⇒ **check-run API（公开仓库无需 token 就能读）**）⇒ 随后那 9 支失败名单与覆盖率读数自己说出来了 ⇒ **三处根因**（**没有一处是「基础设施抖动」**）：① **依赖清单漏进仓**（`augmentor/requirements-legacy.txt` 被 `.gitignore` 的 `*.txt` 挡住 ⇒ 从未入库 ⇒ 干净检出上那支守卫读不到文件 ⇒ **只有 CI 看得见**）；② **冒号/盘符那一档按平台判**（`Path(…).drive` 在 POSIX 上是空的 ⇒ 同一个名字在两个平台上两种判决 ⇒ 6 支红 ⇒ 改成按 **Windows 形状**判 ⇒ **Windows 上行为一格未动**、POSIX 上从放行收成拒 ⇒ 两边同形）；③ **数版本目录时把 `current` 那条符号链接也算进去了**（`Path.is_dir()` **会跟随符号链接** ⇒ POSIX 上 25 而 Windows 上 24 ⇒ **两支红是数法的漏项、不是并发缺陷** ⇒ 改按产品自己的口径数（`list_versions()` 也是 `name != "current"` 才收））。**两个否证读数**（faiss 假设 / 依赖漂移假设）也留在案上 —— 免得下一个人再猜一遍 | **已修**（三批：`823bca62e` 改产品面与守卫、`7fe58a3fb` 改正归因与数法、`f01fffde5` 修诊断那几条 `grep` 自己把这一步打红的那一处）⇒ **CI 侧已复验绿**：`f01fffde5` 上 **AI platform CI / warnings-gate / laiyipao CI / auto-checkin CI / OA4Rust supply-chain 五个 workflow 全 success**；后端那一步的读数逐字 **10547 passed / 14 skipped / 0 failed**（445.58 s）、**覆盖率 99.53%**（`TOTAL 15823 56 4380 38 99%`） |

> **为什么第 3 条不许含糊**：交接文档 §4 的 P1-3 把 CI 红项定性成「**基础设施抖动**（非代码）」——
> 本轮实测**那不成立**：红的是**代码级**的门禁（govulncheck 的工具链下限、license 门禁少一个文件头），
> 两处已经各自收掉；剩下第 3 条的红**仍然不能用「抖动」两个字盖过去** ⇒ 记成待查，
> 并把「下一次拿到 CI 日志要做的第一件事」写清楚（见上表第 3 行的「下一步」）。

---

## 四、下一轮候选（按「能不能自证」排序）

| 优先 | 项 | 归属 | 判据 |
|---|---|---|---|
| 1 | 卷二 Backlog A 余下的 **5 笔 S 级**（A287 A301 A308 A335 A370） | augmentor | **A311 已于 L325 收口**、**A272 / A278 已于 L327 收口**（`multilingual` 那一节的口径写实 + A277 那一把尺子的「自由子对象白名单」一档）。余下 5 笔里 A308 / A370 是「要产品拍板」、A335 要动 SDK 公开契约 ⇒ 真正能自证的只剩 **A287 / A301** |
| 2 | 卷二 Backlog A 的 27 笔 **M** 级里**已拍口径**的那些 | augmentor | 口径已定 ⇒ 只差实现；**要动 SDK 公开契约**的另起一轮 |
| 3 | `x_message_mass.creator`（P0-2）与 attendance 五表（oa4rust P1-1） | oa4rust | 需要真库 + Rust 构建窗口；第一步都是「先设计一个能翻转门禁的断言」 |
| 4 | 第 2 条微信登录通道 | laiyipao | 需要 AppID/Secret + 客户端接线 |
| 5 | 第 23 条 `levels` 表权威 | laiyipao | 两条候选路径都要产品拍板 |
| 6 | **A287**（`similarity_to_a` / `similarity_to_b` 两个恒零键）与 **A301**（「默认备份目录」三份那一族，与 A308 同根） | augmentor | **本仓可自证的前提是先量语义**：A287 先量「接计算」需要拍哪一条语义（B 相对 A 还是 A 相对 B）、「删键」是破坏性变更；A301 / A308 先量「三份默认」各自的现值与收敛点，再决定是「改 CLI/SDK 的相对默认」还是「三面收敛到一个函数」 |

## 五、复跑命令

```bash
# augmentor 全量门禁（本轮：**0 failed / 10551 passed / 6 skipped**，覆盖率 99.53%）
cd augmentor && python -m pytest tests -q -p no:cacheprovider
cd augmentor/web && npx vitest run && npx tsc --noEmit && npx eslint . --max-warnings 0 && npx prettier --check src

# 跨子项目告警门禁（本轮：**5 项 5 通过**；§三 第 3 行那道 license 门禁就在其中）
node scripts/check-warnings.mjs --only rust-lints,llm-lab,llm-lab-tests,augmentor-web,license
node scripts/check-license-headers.mjs   # 单跑那道：读数「2448 个源码文件，缺失 0」

# laiyipao 归属守卫（本轮：3 支全绿，读数见 §二）
cd laiyipao/server && go test ./internal/service/ -run "TestUserScopedQueriesMustFilterByOwner|TestOwnerScopedTableRegistryCoversSchema|TestOwnershipScannerSeesTheRealPackage" -count=1

# laiyipao 依赖漏洞门禁（本轮：**No vulnerabilities found** —— Go 1.26.11 之后）
cd laiyipao/server && go install golang.org/x/vuln/cmd/govulncheck@v1.9.0 && govulncheck -show verbose ./...

# llm-lab（本轮：130 支全绿）
cd llm-lab && python -m pytest tests -q

# auto-checkin（本轮：163 支全绿 + 棘轮九档全绿）
cd auto-checkin && npx tsx --test test/*.test.ts && npm run test:ratchet

# oa4rust 门禁（需真库 + Rust 构建窗口；本轮未跑 —— 本机无 Postgres、无 Docker）
cd oa4rust/backend && cargo test --test security_regression -- --nocapture
```

> ⚠️ **运维要点**（沿用 `docs/plans/2026-10-29-remaining-work-plan.md` §D4，本轮再次撞到）：
> augmentor 全量 pytest 运行期间**不要做 git 提交 / 推送** —— `test_ledger_hash_reachability_l270` 与
> `test_log_block_bullet_ownership_l264` 依赖远端 ref 面，运行中改 HEAD/refs 会让它们随机红
> （本轮实测到一次，23 支红全属这一族，与产品面无关）。
>
> ⚠️ **本轮新增的一条**：**augmentor 全量 pytest 期间也不要编辑 `augmentor/tests/**`**
> —— 本轮那 1 支 ERROR（`test_http_exception_passthrough_l265`）就是并行跑第二轮时撞出来的，
> 单独复跑全绿 ⇒ **同一棵树里同时跑两轮 pytest 会互相踩**（本轮那两次重叠的全量跑就是这个教训）。
