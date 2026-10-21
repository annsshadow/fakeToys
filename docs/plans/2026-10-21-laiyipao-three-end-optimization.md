# 来一炮 · 三端全面优化 · 执行计划

> **日期**：2026-10-21
> **范围**：`laiyipao/` 三端 —— `server`（Go/Fiber/PG）、`miniapp`（uni-app 微信小程序 + H5）、`admin`（Vue 3 运营后台 + 玩家文档站）
> **状态**：计划已评审通过，待执行（本文件是执行的唯一依据，逐项销账）

---

## 0. 为什么要做这件事

三端审计（server / miniapp / admin 各一份，全量读码）得出一个共同结论：

> **这个项目最要紧的问题不是「功能少」，而是「**看起来实现了，其实没生效**」。**

具体表现为四类，每类都有实测证据：

| 类别 | 表现 | 证据 |
|---|---|---|
| **A. 契约错位** | 后台表格/抽屉的列**恒空**——字段名拼错，但没人报错 | 战报详情抽屉 14/16 字段空；防线「属主」「快照哈希」两列空；宝石「效果/说明」两列同绑 `descr`；皮肤「价格」绑了不存在的 `price` |
| **B. 事实错误** | 客户端文案与服务端事实**相反** | 背包页写「洗练券尚未接入、已从商城下架」，而 `seed.go:413` 明确「已接通，恢复上架」；且这句话被 `bag.test.ts:105-130` 锁成了断言 |
| **C. 静默降级** | 出错时返回「看起来合理」的值，而不是报错 | `/me` 把 DB 错误吞成 `points = 0`；结算路径 5 处 `marshalJSON` 错误被丢弃 → build 快照可能写空（回放哈希的锚点） |
| **D. 闭环缺口** | 后端有能力，客户端没有入口 | 18 件装备只有 2 件可达且**无装备/卸下入口**；`/config.reactions` 全客户端零消费；`POST /admin/levels/regenerate` 等写端点无服务端护栏 |

**根因只有一个**：后台的 15 个端点响应是**手写的 `map[string]any`**，形状由裸 SQL 与 map 字面量共同决定，**前后端两侧都没有事实源**。前端用 `Record<string, unknown>` 承接，字段拼错不报错；测试 fixture 又按前端形状写 —— 于是每一条缺陷都测得绿。

所以本轮的第一件事不是补功能，而是**先把事实源建起来**（P1），后面的修复才有靠得住的地基。

---

## 1. 基线（动手前实测，2026-10-21）

| 端 | 命令 | 结果 |
|---|---|---|
| server | `go test ./... -count=1` | **12/12 包通过**（service 203s / domain 18s / httpapi 19s） |
| admin | `pnpm test` | **26 文件 / 294 用例通过** |
| miniapp | `pnpm test` | **75 文件 / 1075 通过 / 2 跳过**（`i6.e2e.test.ts` 需要在跑的服务端，故意跳过） |

> ⚠️ 「2 跳过」是**故意**的（需 `LYP_E2E=1` + 运行中的服务端），不是缺依赖。
> 但按 AGENTS.md #11，跳过项必须**显式记账**，不能算进「全绿」。

**本地环境**（本轮自建，不入库）：`docker run -d --name lyp-pg -e POSTGRES_HOST_AUTH_METHOD=trust -p 5432:5432 postgres:17`，
`DATABASE_URL=postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable` → `cmd/migrate` → `cmd/seed`。

**收尾判据**：每一阶段结束时，三端测试与基线**持平或更好**，且 `git diff --cached` 无真值（AGENTS.md #12）。

---

## 2. 四项产品拍板（用户已授权「按推荐直接做」）

| 项 | 决定 | 依据 | 落地形态 |
|---|---|---|---|
| ① 装备获取途径 | **做** | 装备的 `base_armor`/`base_bonus_pct` **已有真实战斗效果**（护甲 430、攻击加成 1875）。18 件里 16 件不可达是纯粹的闭环缺口，正是 KNOWN_LIMITS 第 25 条的教训「服务端有扣减逻辑 ≠ 玩家能用」 | 商城上架（定价走 `shop_items.payload`）+ 等级解锁发放（对齐 `skills.unlock_level` 的既有惯例）+ `POST /me/equipment/:id/equip` / `unequip` + 背包 UI |
| ② `burn_cloud` 火区 | **不做**，但把规格写死留档 | 任何实现都改战斗演化 → 改 `replay_hash` → **存量战报永久失去可重放性**（`OPERATIONS.md` 有专门流程），且要重跑 100 关平衡基线。这是独立项目，不该混进「三端功能优化」 | KNOWN_LIMITS 第 19 条补一段「可执行规格」：火区怎么算 / 存在哪 / 是否计分 / 要不要服务端重算，四个问题各自给出**默认答案与代价**，标明「待拍板」 |
| ③ `levels` 表权威性 | **走 (b) 变体**：生成器继续权威，但让分歧**在后台可见** | (a) 需要缓存+失效机制，且与 `LoadGameConfig`「绝不失败」的签名约定冲突；做成**可见的分歧**比藏起来更诚实 | 列表行与单关回读都带 `player_authority`/`applied_to_players`；后台顶部常驻横幅「本页编辑记录在案，不影响线上玩家」，并把**玩家实际生效值**（`domain.GenerateLevel`）一并下发做对照 |
| ④ 合成技能材料 | **熔炼**（现状即如此），并把 `unlock_level` 作**合成前置门槛** | `SynthesizeSkills` 已是熔炼语义（销毁材料 + 清槽位 + 发产物），本轮把这条从「隐含」变成「有守卫钉住」 | 新增守卫测试：材料被销毁、产物唯一、等级不足时 403 且**不消耗材料** |

---

## 3. 执行阶段总览

| 阶段 | 主题 | 条目数 | 依赖 |
|---|---|---|---|
| **P1** | 契约根修：Go struct → TS 类型生成 | 6 | — |
| **P2** | 静默失效 P0 | 10 | P1（类型先落地才能改前端） |
| **P3** | 后端健壮性 | 8 | — |
| **P4** | 玩家玩法闭环 | 7 | P1/P2/P3 |
| **P5** | 运营后台能力 | 10 | P1/P2/P3 |
| **P6** | 体验与卫生 | 12 | P1~P5 |

**阶段内的执行纪律**
1. 每条改动**先写守卫测试**，再改实现（AGENTS.md #8：测试要能因业务逻辑变化而失败）。
2. 每阶段结束跑一次三端全量 + `gofmt -l`（判据：**无输出**）+ `go vet`。
3. 任何一条跳过/无法验证，**显式记进本文件的销账表**，不算「完成」（AGENTS.md #11）。

---

## P1 · 契约根修：Go struct → TS 类型生成

> **目标**：消灭「前后端各自手写形状」这一类缺陷的**根**。
> 判据不是「加了很多测试」，而是：**改一个字段名，两端同时红**。

### P1-1 `server/internal/service/adminviews.go`（新建）

15 个后台端点的响应视图 struct。**唯一事实源**。
关键约定：
- json tag = 线上 JSON 键名，handler 不得再包别的名字；
- 可空字段用指针（`*T`）→ 生成 `T | null`；可缺省字段用 `omitempty` → 生成 `?`；
- 时间用 `time.Time`（RFC3339 字符串），可空时间用 `*time.Time`；
- `seed` 必须是**字符串**（int64 LCG 值超 2^53，JS Number 会失真）。

### P1-2 `server/internal/tygen/tygen.go`（新建）

反射式 TS 生成器核心：
- 输入：一个 `(TS 名 → Go 类型)` 注册表；
- 输出：TS interface 文本；
- 类型映射：`string→string`、数值→`number`、`bool→boolean`、`time.Time→string`、
  `*T→T | null`、`[]T→T[]`、`map[string]T→{ [key: string]: T }`、`any→unknown`、
  `[]byte→string`（Go JSON 编码成 base64 字符串）、具名 struct→引用注册表名、
  匿名 struct→内联展开；
- 字段规则：json tag 优先、`-` 跳过、`omitempty` → 可选、匿名内嵌 struct → 字段平铺。

### P1-3 `server/cmd/tygen/main.go`（新建）

CLI：`go run ./cmd/tygen` 写出 `admin/src/api/types.gen.ts`（后续可扩到 miniapp）。

### P1-4 `server/internal/service/*.go`（改造 15 个方法）

把 `map[string]any` 返回改成对应 struct 返回：
`AdminListLevels` / `AdminLevelRow` / `AdminLevelWaves` / `AdminSkills` / `fetchSkills` /
`AdminEquipment` / `AdminListDefenses` / `AdminEconomy` / `AdminUpdateShopItem` /
`AdminListAnnouncements` / `AdminCreateAnnouncement` / `AdminListRedeemCodes` /
`AdminCreateRedeemCode` / `AdminListAuditLogs` / `AdminUpdateLevel` / `AdminUpdateSkill` / `skillRow`。

**顺带修掉 3 个已确认缺陷**（本就是 P0/P3 的条目，见 P2-10、P3-7、P3-8）：
- `AdminUpdateSkill` 补 `default: reject` 分支（与 levels/shop 同口径）；
- `AdminListAuditLogs` 的 `_ = json.Unmarshal(detail, &d)` 改 fail-loud；
- `AdminListDefenses` 补 `snapshot_hash` 列。

### P1-5 `admin/src/api/types.gen.ts`（生成物，入库）

生成物**入库**而不是构建时生成：理由是 CI 里 admin 只做 `type-check`，
生成物入库才能让「类型漂移」在 PR 上立刻可见。

### P1-6 守卫：`server/internal/tygen/tygen_test.go` + CI 步骤

- Go 测试：重新生成一遍，与入库文件**逐字节比对**，不一致 → 红。
  判据要能失败：把某个 struct 的 json tag 改掉而不重新生成，测试必红。
- CI：新增一步 `go run ./cmd/tygen && git diff --exit-code -- admin/src/api/types.gen.ts`。

### P1-7 `admin/src/api/index.ts` 改造

- 删掉手写的 `AdminLevel` / `AdminSkill` / `AdminUser` / `AdminBattle` 等 interface，
  改为 `import type { ... } from './types.gen'`；
- **`Record<string, unknown>` 全部消灭**（现状 39 处）；
- API 函数签名与生成类型对齐。

### P1-8 `admin/src/api/index.ts` 的 envelope 也要类型化

`{items,total}` / `{waves}` / `{flows,shop}` / `{equipment,...}` 等包装也要有生成类型，
否则前端照样得手写包装形状 —— 那是同一个根因。

**验收判据**
```bash
go run ./cmd/tygen && git diff --exit-code      # 无输出 = 生成物与源同步
cd admin && pnpm type-check                     # 0 错误
cd admin && pnpm test                           # 全绿
grep -rn "Record<string, unknown>" admin/src    # 0 命中
```

---

## P2 · 静默失效 P0（10 条）

| # | 端 | 症状（证据） | 改动 | 验收判据 |
|---|---|---|---|---|
| **P2-1** | admin | `BattlesView.vue:245-333` 详情抽屉 **14/16 字段恒空**。服务端 `GET /admin/battles/:id` 返回的是 `ReplayInfo`（`economy.go:628-648`：`battle_id/level_id/seed/level/expected_hash/created_at/build/card_picks`），前端按 `AdminBattle` 读 | ① 服务端新增 `GET /admin/battles/:id` 的**战报详情**响应（`AdminBattleDetailView`），字段与玩家侧战报一致；② 前端类型改为该结构，抽屉逐字段渲染 | 抽屉显示非空；`BattlesView.test.ts` 的 fixture 改为**服务端真实形状**并断言字段可见 |
| **P2-2** | admin | `DefensesView.vue:101`「属主」绑 `user_id`（服务端给 `owner_id`）→ 恒空；`:124-128`「快照哈希」服务端**根本不返回** → 恒空 | ① 服务端补 `snapshot_hash`（P1-4 已含）；② 前端改绑 `owner_id` | 两列非空；`DefensesView.test.ts` fixture 改服务端形状 |
| **P2-3** | admin | `EquipmentView.vue:102` 宝石「效果」与「说明」**两列同绑 `descr`**（宝石真实字段是 `attr`） | 前端：宝石表列改为 `attr`（效果）+ `descr`（说明） | 两列内容不同；测试断言 `attr` 出现 |
| **P2-4** | admin | `EquipmentView.vue:112` 皮肤「价格」绑 `price`，但 `domain.SeedSkin` **无 price 字段** | 前端：价格列改为 `rarity`/`passive`（真实字段） | 该列非空且有值 |
| **P2-5** | admin | 匿名访问 `/wiki` 被 `handleUnauthorized` 踢去后台登录页（`WikiView.vue:132` 调 admin-only 的 `fetchDashboard` → 401 → `client.ts:121` 改 hash） | ① `WikiView` 的「实战分布」Tab 改为**可选加载**：未登录就不请求，而不是请求后被踢；② `client.ts` 的 401 跳转对**公开路由**豁免 | 匿名打开 `/wiki` 不跳转，实战分布 Tab 显示「登录后可见」 |
| **P2-6** | miniapp | `bag.vue:381-384` 洗练券文案「尚未接入、已从商城下架」与 `seed.go:413` **相反**；且 `bag.test.ts:105-130` 把错话写成断言 | 文案改为与事实一致；**同时改测试**（测试锁错话比没测试更坏） | 文案与 seed 一致；测试断言改为正向事实 |
| **P2-7** | miniapp | `stage.vue:172` `diagnose(id, 2)` 的 `failed_times` **硬编码 2** → 第一次进关的玩家也看到「卡关诊断」 | 服务端 `GET /me/diagnose`（或从 `user_progress` 读真实失败次数）下发 `failed_times`；客户端不再自己编 | 新号不显示诊断卡；连败 2 次后显示 |
| **P2-8** | miniapp | `signin.vue:216-219` 签到失败后 `canSign=false` → 按钮永久禁用，且文案变成「七日签到已完成」**说谎** | catch 分支改为「保留可点 + 提示错误」；文案只在**服务端确认已签**时才显示完成 | 失败后可重试；文案不撒谎 |
| **P2-9** | server | `game.go:343-345,365-366` 结算路径 5 处 `marshalJSON` 错误被丢弃 → `buildRaw` 可能为空 → **战报存下一份空的构筑快照**（回放哈希与验真的锚点） | 改 fail-loud：`buildSnapshot` 序列化失败必须让结算失败（与 `adminapi.go:124` 同口径） | 注入序列化失败 → 结算返回 5xx，且**没有**写进半截战报 |
| **P2-10** | server | `stats.go:828-844` `AdminUpdateSkill` 的 switch **无 default 拒绝分支** → 非法键静默丢弃、返回 200「已保存」 | 补 `reject` 机制，与 `AdminUpdateLevel` / `AdminUpdateShopItem` 完全同口径 | 混合 patch（1 合法 + 1 非法）→ 400 + 零副作用；新增负向测试 |

**阶段验收**：`go test ./...`、admin/miniapp `pnpm test` 全绿；每条新增守卫都能被变异打红。

---

## P3 · 后端健壮性（8 条）

| # | 症状（证据） | 改动 | 验收判据 |
|---|---|---|---|
| **P3-1** | `/me`：`progression.go:18-19` 丢 ctx（用 `context.Background()`）、`game.go:732-735` 把 DB 错误吞成 `points = 0` → `build_rating` 静默变成一个**看起来合理的错值** | ① `ComputeRatingFor` 接 ctx；② DB 错误向上抛，接口返回 5xx 而不是假值 | DB 故障时 `/me` 返回 5xx，不返回 `points=0` 的成功响应 |
| **P3-2** | `AdminGrantCurrency`（`stats.go:442`）不预检用户存在 → `grantWallet`（`service.go:560`）报「coin 不足（需要 100）」，真实原因是**用户不存在** | 先查用户是否存在，不存在 → 404「用户不存在」 | 对不存在的 user_id → 404 且消息正确 |
| **P3-3** | 重算闸门档位（off/shadow/enforce）**没有任何 HTTP 端点可读可改** → 运营无法在线切档 | 新增 `GET /admin/recomputes/mode` + `PUT /admin/recomputes/mode`；后台「重算审计」页加档位切换（带二次确认） | 能读、能切、有审计记录 |
| **P3-4** | `GET /admin/levels` 无 `limit/offset`，`total` = `len(out)`（**不是真实总数**）→ 分页语义不成立 | 加 `limit/offset`（带真实 `COUNT(*)`），保持默认全量兼容 | `total` 与 DB 真实数一致；能翻页 |
| **P3-5** | `replay_verifications` **无幂等键** → 同一战报重复验真 200 次写 200 行（`verification.go:59-74`；`middleware.go:519` 已记录此事实） | 加唯一约束（battle_id + verifier + hash）+ `ON CONFLICT DO NOTHING` | 重复验真不新增行 |
| **P3-6** | `POST /admin/levels/regenerate` **无限流、无二次确认的服务端护栏**（单次即重建全部 100 关） | 服务端加「同参确认令牌」或冷却窗口；UI 已有 `ElMessageBox.confirm`，服务端补硬护栏 | 连续调用被拒（或需令牌） |
| **P3-7** | `POST /mastery/reset` 响应**缺 `families`**（`routes.go:680-687` vs `:640-647`）→ 客户端重铸后要再拉一次 `/mastery` | 补齐字段 | 两个响应形状一致；加对拍守卫 |
| **P3-8** | 审计 detail 读取侧 `_ = json.Unmarshal`（`adminapi.go:717`）→ 坏数据与「无明细」无法区分（写入侧已有兜底，只修了一半） | 改 fail-loud | 坏 detail → 报错；正常 detail 不受影响 |

---

## P4 · 玩家玩法闭环（7 条）

> 这一段是「玩家能不能真的玩到」的缺口。

| # | 症状（证据） | 改动 | 验收判据 |
|---|---|---|---|
| **P4-1** | 18 件装备只有 **2 件可达**（`service.go:395` 注册时赠送），且**没有装备/卸下端点**，背包页装备区**纯只读**（`bag.vue:121-128`） | ① 新增 `POST /me/equipment/:id/equip` / `POST /me/equipment/unequip`；② 商城上架装备（`shop_items.payload` 走既有发放机制）；③ 等级解锁发放（对齐 `skills.unlock_level` 惯例）；④ 背包页装备/卸下按钮 | 新号之外能通过商城/等级拿到装备；装备后护甲/攻击加成**在战斗里生效**（端到端断言） |
| **P4-2** | `GameConfig.reactions` **全客户端零消费**（`client.ts:99-108`）→ 玩家查不到「冰+雷=什么反应、伤害多少、持续多久」 | 新增「反应图鉴」页（或背包/首页的图鉴 Tab），消费 `reactions` 全部字段 | 页面上能查到 7 条反应的完整参数 |
| **P4-3** | `reachableReactions(elements)`（`elements.ts:333`）**无任何 UI 调用** → 玩家看不到「我这个构筑能打出哪些反应链」 | 接到首页短板建议与背包页 | 构筑改变时能实时看到可达反应变化 |
| **P4-4** | `TERRAIN_DESCR`（`terrain.ts:432`）**零消费** → 选关页只显示地形**名字**，不显示机制 | 选关页显示地形机制说明 | 5 种地形都能看到机制 |
| **P4-5** | 战斗中**退出不结算**（`battle.vue:408-414` 的 `onUnload` 只停渲染器）→ 体力已扣、无战报、无奖励 | 战斗中提供「放弃战斗」入口，服务端支持**放弃并退还体力**（或明确写死「体力不可退」并在 UI 说明——但那样必须在开战前告知） | 放弃后体力处理明确且前后端一致 |
| **P4-6** | 体力**净消耗为负**：`EnergyOnWin=5` < 第 1 关 `EnergyCost=6` → 赢一局净亏 1 体力，而客户端只显示「消耗 6」，从不显示净成本 | 结算后展示净消耗（`energy_cost - energy_on_win`），并在选关页标注 | 玩家看到的是净成本 |
| **P4-7** | 防线快照恒传 `equipment: []` / `mastery_nodes: []`（`me.vue:314-323`）→ 别人挑战你的防线时，你的装备与专精**根本没进快照**；`works` 是硬编码的 `['slow_belt','block_wall','tesla_grid']`（`me.vue:230`），玩家无法选 | 快照带上真实装备/专精/宝石；`works` 改为玩家可选 | 快照内容与玩家实际构筑一致（断言） |

---

## P5 · 运营后台能力（10 条）

| # | 症状（证据） | 改动 | 验收判据 |
|---|---|---|---|
| **P5-1** | 表格**无分页**：战报（服务端也缺 `offset`）、重算台账（服务端已支持却不用）、审计日志、兑换码、公告、商城、用户（手写分页无每页条数） | 统一分页组件 + 服务端补 `offset`（战报） | 每个列表能翻页且 `total` 正确 |
| **P5-2** | 破坏性操作**无二次确认**：解封用户（`UsersView.vue:108`）、发/回收资源（`:114`，一次误输 `coin -1000000` 直接回收百万金币）、停用关卡 | 统一加确认；发/回收资源**必须**二次确认并显示净变化 | 每个破坏性操作都有确认 |
| **P5-3** | 关卡 **7/9** 个可写字段无入口（`name/wave_count/difficulty/energy_cost/star_targets/terrain_config/is_boss`） | 补齐编辑入口 | 每个白名单键都能从 UI 改 |
| **P5-4** | 技能 **8/11** 个可写字段无入口（`cooldown_ms/pierce/aoe_radius/apply_stacks/projectile_speed/chain/element/family/kind`） | 补齐编辑入口 | 同上 |
| **P5-5** | 商城**无上架/下架开关**（服务端 `enabled` 已支持）、`sort_order`/`payload` 无入口 | 补齐 | 能下架一个商品 |
| **P5-6** | `adminMe` **从未被调用** → `role` 从未读取 → readonly 管理员看得见全部写按钮（`router/index.ts:60-62` 注释已自认） | 接进布局：readonly 隐藏/禁用写按钮 | readonly 登录看不到写入口 |
| **P5-7** | **无用户详情抽屉**（服务端也没有该端点）→ 看不到该玩家的战报历史、钱包流水、验真明细 | 新增 `GET /admin/users/:id` 返回聚合详情 | 用户页能点开详情 |
| **P5-8** | 全仓 **0 个 `el-empty`**、0 个 `el-skeleton`；错误只是一条几秒后消失的 toast | 空态 + 持久错误面板（带重试） | 空表/加载失败/筛选无结果三种状态可区分 |
| **P5-9** | `keep-alive` + 全部视图只用 `onMounted(load)`、**0 处 `onActivated`** → 封禁用户后切页回来还是旧数据 | 加 `onActivated` 刷新 | 切页回来数据是最新的 |
| **P5-10** | 公告**看不到正文**（服务端返回 `body`，UI 只有标题）、`published_at` 未展示、已发布不能撤回；兑换码 `enabled` 未展示、不能停用 | 展示完整字段 | — |

---

## P6 · 体验与卫生（12 条）

| # | 项 | 证据 | 改动 |
|---|---|---|---|
| **P6-1** | `stage/mastery/bag/me` **无 loading、无空态**（`shop.vue` 是全项目唯一做对的） | `stage.vue:101-106`、`mastery.vue:88-93`、`bag.vue:250-254`、`me.vue:308-311` | 统一补 loading / 空态 / 错误态 |
| **P6-2** | 3 个 `navigationStyle:'custom'` 页面**无状态栏/胶囊避让**、全项目 0 处 safe-area；`App.vue:2` 注释承诺统一处理但只 `console.log` | `pages.json:9,22,64`；`App.vue:2-8` | 统一做胶囊避让 + `env(safe-area-inset-bottom)` |
| **P6-3** | `stage.vue` **无 `onShow`** → 打完回选关页星级不刷新 | `stage.vue:153-177` | 加 `onShow` 刷新 |
| **P6-4** | 死代码：`.debug-tag`、`.radar-box`、`.row-between`、`--panel-3`、死 ref `renderMode`/`equippedElements`、死分支 `rank.vue:60` | — | 清理 + 相应测试同步 |
| **P6-5** | 三份重复的 `currencyName`（`battle.vue`/`me.vue`/`signin.vue`）；多处服务端英文 key 直接上屏（`bag.vue:303-310`） | — | 抽公共映射 + 未知值兜底中文 |
| **P6-6** | `bag.vue`/`mastery.vue` 的 **O(n²)** 查找（`skillNameOf`/`skillOf` 线性查找、`nodesInLayer` 模板内重算） | `bag.vue:277-279,560-564`；`mastery.vue:48-67` | 改 Map + computed 缓存 |
| **P6-7** | `me.vue`/`verify.vue` 的 `setTimeout` **从不清除** → 页面卸载后仍发写请求 | `me.vue:433`；`verify.vue:202` | 加卸载守卫 |
| **P6-8** | `contract_coverage.test.ts` 的字段名匹配**会假绿**（`reactions` 因别处同名字符串而「通过」，实际零消费） | `contract_coverage.test.ts:26-35`（它自己文档里承认） | 改结构化匹配（按消费点而非同名字符串） |
| **P6-9** | miniapp **无体积闸门**（admin 有 `size:check`，miniapp 没有），CI 也不检查产物大小 | `miniapp/package.json`；`laiyipao-ci.yml:200-203` | 加 `size:check` + CI 步骤（对齐 admin 的预算写法） |
| **P6-10** | `manifest.json`：`mp-weixin.appid` 为空（**不能真机预览/上传**）、`urlCheck:false` 未文档化、未声明最低基础库、`uploadWithSourceMap:true` | `manifest.json:22,24,39` | 占位符 + 文档说明 + 关掉 sourcemap 上传 |
| **P6-11** | 后台**零响应式**（0 个 `@media`）、侧栏固定 216px、`el-col :span="6"` 固定四列 | `AdminLayout.vue:128`；`DashboardView.vue:184+` | 加断点 + 侧栏折叠 |
| **P6-12** | 文档站缺：侧边栏目录（用 `el-tabs` 横排代替）、搜索、深链、`equipment/gems/skins/recipes` 未展示、关卡地形/BOSS 字段未展示、移动端横向溢出 | `WikiView.vue` | 补目录 + 搜索 + 深链 + 补字段 + 移动端布局 |

**附加清理**
- `server/internal/service/content_seeding_test.go.bak`、`fixture_test.go.bak` —— **已被 git 跟踪**的备份文件，容易误导后来者。删除并确认 `.gitignore` 拦住 `*.bak`。
- `POST /api/v1/auth/wechat` —— 服务端完整但客户端无 `wechatLogin` 函数。**本轮不接**（需 AppID/Secret 凭证），保持官方豁免。

---

## 4. 验证矩阵

| 阶段 | 验证动作 |
|---|---|
| 每个条目 | 该条目对应的守卫测试（**必须能因业务逻辑变化而失败**） |
| 每个阶段末 | `go build ./... && go vet ./... && gofmt -l .`（**`gofmt -l` 必须无输出**）<br>`go test ./... -count=1`（需 PG）<br>admin `pnpm type-check && pnpm test && pnpm build && pnpm size:check`<br>miniapp `pnpm type-check && pnpm test && pnpm build:h5 && pnpm build:mp-weixin` |
| 收尾 | `git status --porcelain` + `git diff --cached` 逐个新文件确认无真值（AGENTS.md #12）<br>`laiyipao/scripts/e2e.ps1` 端到端验收 |

> ⚠️ `gofmt` 的判据是 **`gofmt -l` 无输出**，不是「跑过 `gofmt -w`」。
> （KNOWN_LIMITS 记了「`gofmt -C` 是无效 flag + `| Out-Null` 吞掉 stderr」的误判，本轮不再犯。）

---

## 5. 明确不做的事（及原因）

| 不做 | 原因 |
|---|---|
| `burn_cloud` 火区实现 | 见 §2 ②：改 `replay_hash` → 存量战报永久失去可重放性 + 要重跑 100 关平衡基线。本轮只把「缺什么」写成可执行规格 |
| 让 `levels` 表成为玩家侧权威 | 见 §2 ③：与 `LoadGameConfig`「绝不失败」的签名约定冲突；本轮改为让分歧**可见** |
| 微信支付 / 激励广告接入 | 需 AppID 与商户号；`orders`/`ad_rewards` 表零写入是**已知边界第 1 条**，不假装做了 |
| 微信登录接入客户端 | 需 AppID/Secret；服务端已完整并有官方豁免 |
| 装备 `level`/`star` 缩放 | KNOWN_LIMITS 第 10 条：实测**不该做**（攻击侧越过封顶、护甲侧吃满只多买 0.68% 血量）。本轮做的是**让装备可达**，不是给装备加新维度 |
| 服务端跑完整战斗引擎 | KNOWN_LIMITS 第 3 条的取舍：防线胜负由客户端裁决 + 胜率异常告警兜底 |
| 后台从 Go struct 生成 **miniapp** 类型 | 本轮先做 admin（漂移集中地）；miniapp 已有 `client.ts` 手写类型且与后端对得上，扩到 miniapp 属于 P1 的后续增量 |

---

## 6. 销账表（执行时逐条勾）

> 每完成一条，在本表勾上并写上**证据**（测试名 / 命令输出）。
> 「已完成」= 有守卫 + 有证据。**跳过任何一条都要写明原因。**

### P1
- [x] P1-1 `adminviews.go`（15 个响应视图 struct）—— `server/internal/service/adminviews.go`
- [x] P1-2 `internal/tygen` 生成器核心 —— `server/internal/tygen/tygen.go`
- [x] P1-3 `cmd/tygen` CLI（`-check` 模式）—— `server/cmd/tygen/main.go`
- [x] P1-4 service 层 17 个方法改为 typed —— `adminapi.go` / `stats.go`
- [x] P1-5 `admin/src/api/types.gen.ts`（入库）
- [x] P1-6 守卫测试 —— `server/internal/tygen/tygen_test.go`（3 条）+ CI 步骤
- [x] P1-7 `admin/src/api/index.ts` 改用生成类型（`Record<string, unknown>` 全清）
- [x] P1-8 envelope 也要类型化 —— `server/internal/service/adminresponses.go`（22 个信封）

### P2
- [x] P2-1 战报详情抽屉 —— 服务端新增 `AdminBattleDetailView` + 端点改返回战报详情
- [x] P2-2 防线属主 + 快照哈希 —— 服务端补 `snapshot_hash`，前端改绑 `owner_id`/`owner_name`
- [x] P2-3 宝石效果列 —— 改绑 `attr`
- [x] P2-4 皮肤列 —— `price`（不存在）换成 `rarity`/`passive`
- [ ] P2-5 wiki 匿名 401
- [ ] P2-6 洗练券文案
- [ ] P2-7 卡关诊断真实失败次数
- [ ] P2-8 签到失败不永久禁用
- [ ] P2-9 结算 marshalJSON fail-loud
- [x] P2-10 `AdminUpdateSkill` 整单拒绝（补 `default: reject` 分支）

### P3
- [ ] P3-1 `/me` ctx + 吞错
- [ ] P3-2 grant 用户不存在
- [ ] P3-3 重算档位 HTTP 读写
- [ ] P3-4 levels 分页 + 真实 total
- [ ] P3-5 验真幂等
- [ ] P3-6 regenerate 护栏
- [ ] P3-7 `/mastery/reset` 响应形状
- [x] P3-8 审计 detail fail-loud（`adminapi.go` 的 `AdminListAuditLogs`）

### P4
- [ ] P4-1 装备获取 + 装备/卸下
- [ ] P4-2 反应图鉴
- [ ] P4-3 `reachableReactions` 接线
- [ ] P4-4 地形机制
- [ ] P4-5 战斗退出
- [ ] P4-6 体力净消耗
- [ ] P4-7 防线快照真实内容

### P5
- [ ] P5-1 分页
- [ ] P5-2 破坏性操作确认
- [ ] P5-3 关卡字段
- [ ] P5-4 技能字段
- [x] P5-5 商城字段 —— 补「上架/下架」入口（含二次确认）
- [ ] P5-6 角色只读
- [ ] P5-7 用户详情抽屉
- [ ] P5-8 空态/错误态
- [ ] P5-9 `onActivated` 刷新
- [x] P5-10 公告/兑换码字段 —— 公告正文 + 发布时间 + 兑换码 enabled

### P6
- [ ] P6-1 ~ P6-12（见上表）

---

## 附：P1/P2 已落地部分的「恒空列」销账

| 原症状 | 现在 |
|---|---|
| 战报详情抽屉 14/16 字段空 | 服务端返回 `AdminBattleDetailView`（含 shots/hits/hp_left/元素/反应/地形/种子），前端逐字段渲染；`clamped`/`clamp_note`（库里不存在）被删除而不是留着当摆设 |
| 防线「属主」「快照哈希」两列空 | 服务端补 `snapshot_hash` 列；前端改绑 `owner_id` + 新增 `owner_name`/`mastery_done` |
| 宝石「效果」与「说明」同列 | 效果列改绑 `attr`；装备补 `base_armor`/`base_bonus_pct`/`unlock_level` |
| 皮肤「价格」恒空 | 换成 `rarity`/`passive`（`price` 在 `domain.SeedSkin` 里根本不存在） |
| 商城「上架/下架」无入口 | 新增按钮 + 二次确认（服务端 `enabled` 早已支持） |
| 公告看不到正文 | 新增「正文」「发布时间」两列；兑换码新增「状态」列 |
| `GET /admin/equipment` 的 8 个 key 只有 3 个被前端声明 | 全部进类型；敌人/反应/专精树/构筑权重不再被丢弃 |
