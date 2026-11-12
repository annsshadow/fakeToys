# 各项目剩余账本 · 盘点（2026-11-12）

> **用途**：把五个子项目各自的「账本 / 台账」里**还没闭**的条目收敛成一份可执行清单，
> 逐条写清「账本位置 / 原文说了什么 / 本轮实测到哪一步 / 谁才能推进」。
> **口径**：**账本记载只作线索，结论以本轮复跑现量为准** —— 承
> `docs/plans/2026-10-29-remaining-work-plan.md` §四 那条教训
> 「账本过时的一个反例（提醒：引用前先复核）」：本轮就撞上两处这样的过期条目，
> 已按现量收口（见 §二「本轮清掉的账」）。
>
> **本轮做过的实测（逐项可复跑）**：
> augmentor 全量门禁 `python -m pytest tests -q -p no:cacheprovider` ⇒ **0 failed / 10551 passed / 6 skipped**；
> augmentor 前端 `npx vitest run` ⇒ **24 文件 / 427 支全绿**（`tsc --noEmit` / `eslint --max-warnings 0` / `prettier --check src` 同绿）；
> laiyipao 归属守卫 `go test ./internal/service/ -run "TestUserScoped…|TestOwnerScoped…|TestOwnership…"` ⇒ **3 支全绿**；
> llm-lab `python -m pytest tests` ⇒ **130 支全绿**（本机装了 torch ⇒ 无 skip）；
> auto-checkin `npx tsx --test test/*.test.ts` ⇒ **163 支全绿**；`npm run test:ratchet` ⇒ **九档全绿**。

---

## 一、逐项目盘点

### 1. augmentor · 优化循环账本（两卷）

| 账本 | 位置 | 剩余 | 本轮状态 |
|---|---|---|---|
| 卷一 `OPTIMIZATION_LOOP.md` | 卷一 Backlog B（功能增强） | **6 格里 2 笔本轮收口**（B1 / B8），余 B2（前半关闭）/ B3 / B5 / B7（B5 与 B7 = 卷二 A271 / A273 两族） | 本轮还动了 6 处引用落点（歧义清账）与卷一 Backlog B 的 B1 / B8 |
| 卷二 `OPTIMIZATION_LOOP_2.md` | Backlog A（质量缺口） | **35 笔**（A267…A385） | 本轮 **L322 / L323** 两轮把全量门禁从 28 支红收到 **0 支红** |
| 卷二 | Backlog B（功能增强） | **0 笔**（28 笔全闭） | — |

**剩余 35 笔的形状**（按定级）：

| 定级 | 笔数 | 名单 | 能不能本仓自己推进 |
|---|---|---|---|
| **M** | 25 | A267 A268 A269 A271 A273 A289 A290 A293 A296 A306 A309 A345 A346 A349 A351 A353 A354 A355 A356 A357 A358 A364 A371 A376 A384 A385 | 多为「**口径要拍**」或「**要动 SDK 公开契约**」 |
| **S** | 10 | A272 A278 A287 A301 A308 A311 A335 A370 | A311 是本轮 L321/L322 块里点名的下一轮候选 |

**三类「本仓自己推不动」的形状**（逐条读过原文，不是印象）：

- **要产品拍板**（拍的是对外语义，自决就要改契约）：A272（`multilingual` 节要不要开成第 5 个可写节）、
  A287（`similarity_to_a` / `similarity_to_b` 两个恒零键：接计算还是删键 = 破坏性变更）、
  A278（OpenAPI 自由子对象要不要给最小形状）、A306（两处 `created_at` 覆盖 / 空值抹旧值）、
  A308（`{param}` 豁免把写路由一并匿名放行）、A309（默认注册表目录三份、端点那份与另两份不是同一个地方）、
  A273（`self.expander` / `self.tracker`：接消费者 = 新端点，删构造 = 缩公开属性面）、
  A271（十只 `config.*.enabled` 接进阶段入口 = **对外破坏面**）。
- **要环境/前置**（不是写代码能补的）：A268（真模型嵌入要一个装了 `sentence-transformers` 的环境）、
  A269（chromadb 后端要一个装了它的环境来定标）。
- **要动 SDK 公开契约**（⇒ 单独一轮）：A357（落点对、内容错那一族）、A335、A370。

**本轮已经推进掉的**：A311 的邻居（A295 / A297 / A286 / A287 / A282 / A317 / A323 / A387 / A388 / A386 / A280 那一族）
在 L315–L323 连续九轮里逐笔收口 ⇒ 卷二 Backlog A 的**红笔**从 28 支降到 0 支。

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
| P1-3 | CI `sccache` 基础设施红 | 外部基础设施抖动 | 不能（观察即可） |
| P2-1 | 本地环境债 | Swagger zip / 本地 DB 的 bam 用例 / github.com:443 断网 | 不能（环境债） |

**另有一处本轮量到的规模差**（与 P1-2 同族，值得记下）：交接文档 §10.3 写
「读侧 701 条，其中 514 条静态初筛无门禁标记、251 条是 GET + 路径参数」，
并**自陈**「静态初筛不能证明『SQL 无归属谓词』，只能证明『没有明显的门禁调用』」
⇒ 那 251 条**不能直接当缺陷清单**，逐条定性要真请求 + 既有 kind ⇒
每一批 20–25 条、一批一 PR、带 sabotage 点名。

### 3. laiyipao · 已知边界 + 优化台账 Go 段

| 账本 | 位置 | 剩余 | 本轮状态 |
|---|---|---|---|
| 已知边界（诚实声明） | `docs/laiyipao/KNOWN_LIMITS.md` | 30 条编号里 **12 条仍开着**，其余已闭或已被判「不该做」 | 本轮收口第 17 条（§二） |
| 优化台账 Go 段 | `docs/plans/optimization-1000-oa4rust-laiyipao.md` | R 轮次已认领到 R152 | — |

**仍开着的 12 条，按「能不能本仓自己推进」分**：

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
| **augmentor** | `augmentor/OPTIMIZATION_LOOP.md` 卷一 Backlog B 的 **B1 / B8** | B1：「把 `/api/system/*`(13) 与 `/api/dataset/*`(12) 接入 UI —— `api.ts` 对二者零引用」；B8：「前端 5 个未接线服务函数 + 运行时 schema 校验」 | ① 接线队列 A279 分**十一批**（L225–L236）全部走完 ⇒ 契约面 **79** 条操作**全数**被前端调到，`test_api_endpoint_consumption_l224.py` 的 `test_roster_scale_is_pinned` 现在钉的是「已消费 79 = 契约面 79」、`test_unconsumed_matches_the_roster` 钉的是「实测缺口 == 空名单」⇒ 再有一条端点没接当场红；② 那 5 个未接线服务函数早已在对应页面真实接线，`api.wiring.test.ts` 的 `KNOWN_UNWIRED` 白名单**已清空**（那把守门仍在）；运行时 schema 校验也已落地（zod 面 + `main-schemas.conformance.test.ts` 把键集与后端契约面逐格对着） | **两格按现量收口**（本批） || **oa4rust** | `oa4rust/docs/plans/f2e-endpoint-tracker.md`「仍未解决的假绿项」 | ① `x_portal` 共享读 2 条「谓词未加」；② admin-only 表那一族「未闭环暴露」 | ① `portal_assemble_designer` 的 `designer_owned_read_gate` 就是 `creator = 会话人 OR is_admin`、NULL fail-closed，两个调用点（`portal_id_icon` / `portal_id_permission`）都走它 —— 与 2026-11-08 交接文档 §10.2 的结论一致；② 原文自己就写着「已由下节『读侧无谓词收口批』处理」，下一节已落地（`cases=345 pass=345`，sabotage 点名 12 个端点） | **本节清空**（本批） |

---

## 三、下一轮候选（按「能不能自证」排序）

| 优先 | 项 | 归属 | 判据 |
|---|---|---|---|
| 1 | 卷二 Backlog A 的 10 笔 **S** 级（A272 A278 A287 A301 A308 A311 A335 A370） | augmentor | 本仓可自证；其中 A311（全局 `RequestValidationError` 处理器把 422 翻成中文同一形状）已被 L321 / L322 两轮点名为下一轮方向 |
| 2 | 卷二 Backlog A 的 25 笔 **M** 级里**已拍口径**的那些 | augmentor | 口径已定 ⇒ 只差实现；**要动 SDK 公开契约**的另起一轮 |
| 3 | `x_message_mass.creator`（P0-2）与 attendance 五表（oa4rust P1-1） | oa4rust | 需要真库 + Rust 构建窗口；第一步都是「先设计一个能翻转门禁的断言」 |
| 4 | 第 2 条微信登录通道 | laiyipao | 需要 AppID/Secret + 客户端接线 |
| 5 | 第 23 条 `levels` 表权威 | laiyipao | 两条候选路径都要产品拍板 |

## 四、复跑命令

```bash
# augmentor 全量门禁（本轮：**0 failed / 10551 passed / 6 skipped**）
cd augmentor && python -m pytest tests -q -p no:cacheprovider
cd augmentor/web && npx vitest run && npx tsc --noEmit && npx eslint . --max-warnings 0 && npx prettier --check src

# laiyipao 归属守卫（本轮：3 支全绿，读数见 §二）
cd laiyipao/server && go test ./internal/service/ -run "TestUserScopedQueriesMustFilterByOwner|TestOwnerScopedTableRegistryCoversSchema|TestOwnershipScannerSeesTheRealPackage" -count=1

# llm-lab（本轮：130 支全绿）
cd llm-lab && python -m pytest tests -q

# auto-checkin（本轮：163 支全绿 + 棘轮九档全绿）
cd auto-checkin && npx tsx --test test/*.test.ts && npm run test:ratchet

# oa4rust 门禁（需真库 + Rust 构建窗口；本轮未跑）
cd oa4rust/backend && cargo test --test security_regression -- --nocapture
```

> ⚠️ **运维要点**（沿用 `docs/plans/2026-10-29-remaining-work-plan.md` §D4，本轮再次撞到）：
> augmentor 全量 pytest 运行期间**不要做 git 提交 / 推送** —— `test_ledger_hash_reachability_l270` 与
> `test_log_block_bullet_ownership_l264` 依赖远端 ref 面，运行中改 HEAD/refs 会让它们随机红
> （本轮实测到一次，23 支红全属这一族，与产品面无关）。
