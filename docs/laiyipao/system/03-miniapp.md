# 03 · 小程序端（miniapp）

> uni-app 3（微信小程序 + H5 双端）+ Vue 3.4 + Pinia 2 + TypeScript 5.9。
> 本文的重点是**分层边界**：哪些代码可以碰 `uni`，哪些绝对不能；
> 以及那条贯穿全系统的战斗闭环。

---

## 1. 技术栈与构建目标

| 项 | 值 | 备注 |
|---|---|---|
| 框架 | `@dcloudio/uni-app` 3.0.0-5020420260813003 | 全家桶版本统一锁定 |
| Vue | 3.4.21 | 与 `@vue/runtime-core` 同版本 |
| 状态 | Pinia 2.1.7 | 仅一个 store |
| 构建 | Vite **5.2.8**（硬锁） | `@dcloudio/vite-plugin-uni` 的 peer 硬锁 |
| 测试 | Vitest 2.1.9 + jsdom | 内核可无头跑 |
| 目标 | `es2020` | 定点数学用 BigInt 字面量，es2015 下 esbuild 直接报错 |

`vite` 必须锁 5.2.8。这条约束有可执行守卫 `src/test/toolchain_pin.test.ts`：
依赖机器人两次自动升级（5.2.8 → 6.4.3 → 8.3.4）同时打坏了 type-check、
build:h5 与 install。

脚本：

```bash
pnpm dev:h5            # http://127.0.0.1:5174
pnpm dev:mp-weixin     # 产物在 dist/dev/mp-weixin
pnpm build:h5          # dist/build/h5
pnpm build:mp-weixin
pnpm type-check        # vue-tsc --noEmit
pnpm test              # vitest run
```

H5 的 `router.mode = hash`、`base = './'`（`manifest.json`），
部署在 `/laiyipao-h5/` 下。

---

## 2. 目录结构与职责边界

```
miniapp/src/
├─ game/        纯 TS 战斗内核（零 uni API、零绘制、零网络）
├─ render/      Canvas 2D 渲染层（只读引擎状态）
├─ api/         请求封装（令牌、刷新、304、错误）
├─ store/       Pinia 全局状态（配置 / 钱包 / 进度 / 构筑）
├─ pages/       12 个页面
├─ utils/       通用工具（反应名映射、状态栏高度）
├─ test/        测试基础设施（uni stub、page helper、契约覆盖）
├─ testkit/     测试工具（注释剥离等）
├─ App.vue      应用壳
├─ main.ts      入口
├─ manifest.json / pages.json
└─ styles/common.css
```

**唯一的硬边界**：`game/` 不 import 任何 `uni` API，也不 import `render/` 与 `api/`。

这条边界带来三个直接收益：

1. 同一份内核能跑在微信 canvas、H5 canvas 和 Node（Vitest）里；
2. 100 关平衡扫描、确定性测试、变异测试都能在毫秒级完成；
3. 服务端镜像引擎可以与它逐位对拍（`engine_pure_vectors.json`）。

`render/` 是**只读**的：它读引擎状态画图，绝不修改引擎。
`api/` 不 import `store/`（避免循环依赖），令牌存取完全在 api 层内部。

---

## 3. 内核模块（`game/`，13 个文件）

| 文件 | 职责 | 关键导出 |
|---|---|---|
| `types.ts` | 内容与战斗的数据结构 | `EnemyDef` / `SkillDef` / `GeneratedLevel` / `LevelSummary` / `ReplayEvent` |
| `fixed.ts` | 定点整数工具 | `PERMILLE` / `mulDiv` / `applyArmor` / `MAX_ARMOR` / `MAX_REACTION_ATTACK_WEIGHT` |
| `lcg.ts` | 确定性 PRNG + FNV-1a | `LCG` / `BattleRng` / `fnv1a64` / `hex16` / `LCG_KNOWN_VECTORS` |
| `elements.ts` | 5 元素、7 反应链 | `REACTIONS` / `lookupReaction` / `reachableReactions` |
| `damage.ts` | 伤害结算 | `Attacker` / `Defender` / `resolveHit` / `resolveReaction` / `assertAntiInflation` |
| `heatmap.ts` | 热量与三选一手牌 | `HeatSystem` / `CardDeck` / `ACTIVE_SLOTS` / `HEAT_MAX` / `OVERHEAT_DURATION_MS` |
| `terrain.ts` | 5 类地形 | `Terrain` / `TerrainEffect` / `TERRAIN_NAME` / `TERRAIN_DESCR` |
| `skill.ts` | 技能等级与升级费用 | `SkillRules` / `skillBaseDamageAtLevel` / `upgradeCostFrom` |
| `score.ts` | 计分与星级 | `ScoreRules` / `DEFAULT_SCORE_RULES` / `scoreRulesFromServer` |
| `tuning.ts` | 战斗节奏参数 | `GameTuning` / `DEFAULT_TUNING` / `LEGACY_TUNING` |
| `engine.ts` | 引擎主体（2653 行） | `BattleEngine` / `BattleConfig` / `BattleEvent` / `replaySkillsSegmentOf` / `encodeDiscardCode` |
| `defense.ts` | 防线挑战本地模拟 | `runChallenge` / `validateSnapshot` / `snapshotDigest` |
| `replay.ts` | 回放与构筑映射 | `replay` / `equippedFromSnapshot` / `BuildSnapshot` / `ReplayInfo` |

### 3.1 定点整数

所有比例、速度、坐标用 `bigint` 存千分比（`×1000`）。
不用 `number` 的理由不是精度洁癖，而是**跨端一致性**：

- 浮点在不同引擎（V8 / JavaScriptCore / 小程序运行时）上可能产生末位差异；
- 一次差异会沿着 tick 累积，最终让回放哈希对不上；
- 而哈希对不上的症状是「验真把正常玩家判成作弊」，排查方向会完全跑偏。

`fixed.ts` 还定义了几条硬上界（`MAX_ARMOR = 750‰`、`MAX_RESIST = 500‰`、
`MAX_REACTION_ATTACK_WEIGHT = 300‰`），它们是反通胀的结构性保证，
不是可调参数。

### 3.2 确定性 PRNG

`LCG` 是 64 位线性同余，参数固定；`BattleRng` 在其上做区间取值。
`LCG_KNOWN_VECTORS` 是**手写期望值**的已知向量 —— 用实现生成期望值就是自证，
测试会永远绿。

种子在客户端一律以 `bigint` 处理，因为服务端下发的 int64 超过 2^53 后
JSON number 会失精。

### 3.3 引擎

`BattleEngine` 的核心契约：

```ts
interface BattleConfig {
  level: GeneratedLevel          // 含 waves
  enemies: Map<number, EnemyDef>
  skills: Map<number, SkillDef>
  equipped: EquippedSkill[]      // 槽位来自 build.skills[].slot
  attacker: Attacker             // 必须来自服务端 build.attacker
  seed: number | bigint
  extraBaseHp?: bigint           // 复活额外血量（进 hash 前缀 X 段）
  activeSlots?: number           // 缺省 4，不得改变
  scoreRules?: ScoreRules        // 来自 /config
  tuning?: GameTuning            // 缺省 DEFAULT_TUNING
}
```

阶段机：

```
prepare ──start()──▶ wave ──波次清空──▶ card_select ──选牌/跳过──▶ wave …
                        │                                        │
                        └──────────── 防线血量 ≤ 0 ──▶ lost      └──最后一波──▶ won
```

关键不变量：

| 不变量 | 违反后果 |
|---|---|
| `TICK_MS = 50` 与服务端 `domain.TickMs` 一致 | 回放 tick 节奏不同 → 哈希必不同 |
| `activeSlots` 缺省 4 | 历史战报全部重放不出原哈希 |
| `extraBaseHp` 只加当前血量、不动 `baseHpMax` | 加成被漏怪公式整个抵掉 |
| 空关卡不是胜利 | 0 波次关卡会立刻判胜并领奖 |
| 达到 `MAX_BATTLE_TICKS` 强制结束并记 `stalemate` | 引擎停滞导致页面永久卡死 |

### 3.4 回放哈希

```ts
prefix =
  `L${level.id}` +
  `|A${attack}.${critPermille}.${critMultiplierPermille}` +
     `.${reactionMultPermille}.${elementCap}.${reactionTier}.${elementCoefPermille}` +
  `|S${replaySkillsSegment()}` +
  `|X${extraBaseHp ?? 0n}`

hash = hex16(fnv1a64([prefix, ...events.map(e => `${e.t}|${e.type}|${e.a}|${e.b}|${e.c}`)].join(';')))
```

三段各自的用途：

- `L` 段：关卡身份。同一关卡 id 决定 `base_hp`、波次、地形，所以不必重复写入。
- `A` 段：攻方系数（**含局内 buff**），服务端算不出来 —— 这是 I-6 的已知边界。
- `S` 段：技能槽配置（**不含 buff**），服务端能独立重算 —— 这是 D4/第 56 轮补上的校验面。
- `X` 段：复活额外血量。它是唯一「能撑血又不属于关卡」的量，
  不进前缀就能改本地这一行白扛无数次漏怪。

**S 段有两个口径，不能混用**（这是全客户端最容易写错的一处）：

| 用途 | 方法 | 是否含局内升格 |
|---|---|---|
| 计算 `replayHash` | `replaySkillsSegment()` | **含**（取牌会升格技能） |
| 上报给服务端 | `buildSnapshotSegment()` | **不含**（构造时冻结） |

混用的后果是：取过技能卡的正常对局上报 `0:1:120:2:20`，服务端按 DB 重算出
`0:1:100:1:20` → 422。而 `rollWaveCards` 固定把技能卡放在手牌下标 0，
5 波各取 1 张时命中技能卡的概率约 86% —— 也就是说**绝大多数玩家会被判作弊**。

### 3.5 选牌编码

`card_picks` 是一个整数序列，每波一个：

| 取值 | 含义 |
|---|---|
| `0 / 1 / 2` | 普通取牌（无弃牌），下标是取牌当次的手牌下标 |
| `-1` | 整波跳过（无弃牌） |
| `-(3 + code)` | 本波弃过牌，`code` 编码「弃牌位掩码 + 取剩余第几张 / 跳过」 |

旧编码（`-3..-5` 弃+取、`-6..-8` 弃+跳）表达不了「弃的是哪一张」，
也表达不了「弃多于一张」—— 而「快速装填」卡让每波弃牌次数 > 1 成为正常玩法。
新编码把弃牌集合写成手牌位掩码，两端由 `card_picks_contract` 测试锁死。

---

## 4. 战斗闭环（最完整的一条链路）

```
pages/battle/battle.vue
  │
  ├─ onLoad(query.level)
  ├─ 未登录 → store.login()（单飞）
  ├─ api.startBattle(level)
  │     ├ tokenId / level(含 waves) / seed(str) / build / extra_base_hp / revived
  │
  ├─ equipped = equippedFromSnapshot(bt.build, skillMap)   ← 唯一构造点
  │     为空 → 提示「先去背包配置出战技能」并退出
  │
  ├─ store.setBuild(bt.build)      ← attacker 以服务端为准
  │
  ├─ new BattleEngine({
  │     level, enemies, skills, equipped,
  │     attacker: store.attacker,
  │     seed: BigInt(bt.seed),
  │     extraBaseHp: BigInt(bt.extra_base_hp ?? 0),
  │     activeSlots: bt.build?.active_slots,
  │     scoreRules: scoreRulesFromServer(store.config.score_rules),
  │   })
  ├─ eng.start()
  │
  ├─ renderer = new BattleRenderer(canvas, eng)
  │     setViewport(cssW, cssH)     ← CSS 像素；canvas 内部按 dpr 放大
  │     start(rAF 循环)
  │
  ├─ 事件订阅 → 手牌 UI / 结算 UI / 飘字
  │     波次间隙 phase=card_select → 展示 3 张牌
  │     玩家 takeCard / discardCard / skipCards
  │
  └─ phase ∈ {won, lost} → settle(eng)
        ├ settled 幂等标志（防重复上报）
        ├ 停渲染 + 清定时器
        ├ payload = eng.settleInput(tokenId)
        └ api.settleBattle(payload)
              ├ 成功 → 展示星级/掉落/裁剪提示 → store.setMaxStage → refreshProfile
              └ 失败 → 展示错误 + 本地 replay_hash（供人工比对）

战斗中还可「放弃」（P4-5）：api.abandonBattle(tokenId) 退回体力（与复活币），
服务端把该凭证置为已用 —— 放弃后不能再结算这一局，那是刻意的。
```

页面生命周期有三个 **await 窗口**需要显式防退出
（`pageUnloaded` 标志）：`login()`、`startBattle()`、`getCanvas()`。
任一窗口内用户退出页面，都必须停止后续创建（否则会留下无人 stop 的
rAF / 定时器 + 引擎步进）。

### 4.1 渲染层

```
render/canvas.ts
  ├ makeViewport(width, height)   逻辑坐标 1000×1000 → 屏幕
  ├ toCanvasX / toCanvasY         定点坐标 → CSS 像素
  └ class BattleRenderer
        start(cb)                 优先 rAF，不可用则 setInterval 兜底
        stop()
        draw()                    读 engine 状态 → 清屏 → 地形 → 敌人 → 弹道 → 防线 → HUD
```

规则：

- 渲染层**只读**引擎，绝不改状态。任何「画的时候顺手算一下」都会让
  画面与逻辑分叉，而分叉在回放时表现为哈希不符。
- 视口用 **CSS 像素**，canvas 内部按 dpr 放大。混用会把画面缩小 dpr 倍
  （表现为画布空白 / 元素挤在角落）。
- `raf` 不可用（部分小程序基础库）时降级到 `setInterval`，`renderMode` 会记录当前模式。

---

## 5. API 层（`api/client.ts`）

### 5.1 地址解析

```ts
BASE_URL = VITE_API_BASE || 'http://127.0.0.1:8080/api/v1'
```

- 回环地址（`127.0.0.1` / `localhost` / `[::1]`）允许明文；
- 局域网 IP（`192.168.` / `10.` / `172.16-31.`）明文只告警（真机调试场景）；
- **公网域名 + 明文 → 启动即抛**。

这条判据的理由：令牌有效期 30 天，明文过网等于长期裸奔。
而「把写死的 127.0.0.1 换成真实域名」是最自然的修补方式，
没有任何东西会阻止人保留 `http://`。

### 5.2 令牌与刷新

| 存储键 | 内容 |
|---|---|
| `lyp_token` | access token |
| `lyp_refresh` | refresh token |
| `lyp_token_exp` | access 过期时刻（RFC3339） |
| `lyp_guest` | 游客凭证 |
| `lyp_config_cache_v1` | 配置缓存 `{version, config}` |

请求流程：

```
request(path, opts)
  ├ ensureFreshToken()      已知快过期（30s 提前量）→ 先刷新
  ├ uni.request（20s 超时，Bearer）
  ├ 2xx → resolve
  ├ 401 且需要认证 → refreshAccessToken() → 重试一次
  │     刷新失败 → clearTokens() → 抛 401「登录已过期」
  └ 其他 → 抛 ApiError(status, code, message)
```

**刷新必须单飞**（`refreshInflight`）。服务端轮换 refresh token 是原子的
（条件 UPDATE + RETURNING），N 个并发刷新里恰好 1 次成功、N-1 次 401。
若每个 401 各自处理，失败分支会 `clearTokens()` 把刚写入的新令牌删掉 →
用户被静默登出。触发场景不需要攻击者：access token 2h 过期后回到小程序，
某页面并发拉 wallet + me + tasks + mastery 就是 4 个 401。

`tokenExpiringSoon()` 用**本地时钟**且刻意不做时钟校正：
钟快 → 多刷一次（无害）；钟慢 → 退回 401 兜底路径（无害）。
做校正反而会引入「钟错导致拒绝登录」的新风险。

### 5.3 配置条件请求

```ts
fetchConfigIfVersion(haveVersion)
  → { status: 'modified',   config, serverTimeMs }
  | { status: 'unmodified', version, serverTimeMs }
```

它**不走通用 `request()`**，因为 304 在本场景是成功而非失败。
304 没有响应体，`serverTimeMs` 取 HTTP `Date` 头（秒粒度），
解析失败再回退响应体 `server_time`，再失败回退 0（由 store 决定如何兜底）。

### 5.4 登出的顺序陷阱

```ts
const res = await request('/me/logout', { method: 'POST' })
if (res?.guest_token) setGuestToken(res.guest_token)   // ← 必须先存
clearTokens()                                          // ← 再清
```

服务端登出会**轮换** `guest_token`，旧的那枚立即作废。
不存新的 → 下次冷启动带旧凭证 → 匹配不到账号 → **建出新号** → 进度消失。
网络失败时：本地照清 access/refresh，但**不动 guest_token**
（服务端没轮换它，清掉反而会丢进度）。

---

## 6. 状态层（`store/game.ts`）

单 store，`defineStore('game', setup)`。状态分组：

| 组 | 字段 | 说明 |
|---|---|---|
| 账号 | `userId` / `nickname` / `isGuest` / `loggedIn` / `loginError` | `userId` 以 `/me` 为权威回填 |
| 配置 | `config` / `configLoading` | 一次全量 + 本地持久化 |
| 钱包 | `wallet{coin,gem,energy,keys,tokens}` | `tokens` 初值 `{}` |
| 进度 | `maxStage` / `power` / `buildRating` / `build` | 只增不减 |
| 出战 | `equippedSkillIds` / `loadout` | 服务端是权威 |
| 派生 | `enemyMap` / `skillMap` / `levelMap` / `unlockedLevel` / `attacker` / `equippedElements` | computed |

几个关键设计：

- **`login()` 单飞**。`onLoad` 与 `onMounted` 都会调它，并发执行会建两个游客号、
  两个 token 互相覆盖（症状：钱包显示体力 30，但开战报体力不足）。
- **配置持久化 + 条件请求**。冷启动先用本地副本点亮界面，再带 `version` 问服务端；
  304 时不替换、不重新解析。`CONFIG_CACHE_KEY` 带 `_v1` 后缀，
  将来改缓存结构时换后缀即可让旧缓存自然失效。
- **服务端时钟校准**。`serverTimeMs + (Date.now() - receivedAt)` 抵消本地偏移；
  没有基准时诚实回退本地时钟。所有拿 `shielded_until` 之类绝对时间比较的地方
  都必须用 `estimateServerNowMs()`。
- **`attacker` 只能来自 `build.attacker`**。缺失时回退 `defaultAttacker()` 并
  `console.warn` 明确说明「此时回放哈希会与服务端记录不符」。

---

## 7. 页面（`pages/`，12 个）

| 页面 | 路径 | 职责 | 调用接口 |
|---|---|---|---|
| 首页 | `pages/index/index` | 资源栏、入口导航、当前进度 | 无（读 store） |
| 关卡 | `pages/stage/stage` | 章节切换、关卡列表、星级、诊断 | `fetchMyStars` / `diagnose` |
| 战斗 | `pages/battle/battle` | Canvas 战斗、手牌、结算、放弃 | `startBattle` / `settleBattle` / `abandonBattle` |
| 背包 | `pages/bag/bag` | 出战槽位、技能升级、熔炼、宝石洗练、装备装配 | `fetchGems` / `upgradeSkill` / `synthesizeSkills` / `washGem` / `fetchMyEquipment` / `saveEquippedEquipment` |
| 专精 | `pages/mastery/mastery` | 三层树、分配、重铸 | `fetchMastery` / `allocateMastery` / `resetMastery` |
| 反应 | `pages/reactions/reactions` | 反应图鉴、当前构筑可触发的反应 | 无（读 `/config` 的 `reactions`） |
| 商城 | `pages/shop/shop` | 商品分组、购买、售罄 | `fetchShop` / `buyItem` / `fetchWallet` |
| 签到 | `pages/signin/signin` | 七日签到、任务、兑换码 | `fetchSignInCalendar` / `fetchSignInStatus` / `signIn` / `fetchTasks` / `claimTask` / `redeem` |
| 排行榜 | `pages/rank/rank` | 战力 / 关卡 / 效率三榜 | `fetchLeaderboard` |
| 战绩验真 | `pages/verify/verify` | 输入 battle_id → 本地重放比对 | `getReplay` / `verifyReplay` |
| 我的 | `pages/me/me` | 构筑概览、防线保存/挑战、登出 | `fetchDefenses` / `saveDefense` / `challengeDefense` / `fetchLevel` / `logout` |
| 登录 | `pages/login/login` | 账号口令登录（`PLAYER_LOGIN_REQUIRED` 时进入） | `login`（`POST /auth/login`） |

### 7.1 关卡页

- 章节来自 `/config` 的 `chapters`（**字段必须 snake_case**：
  `start_level` / `end_level` / `terrain_kind` / `boss_enemy_id`）。
  曾写成 PascalCase，服务端一直下发 snake_case，于是章节名空白、
  点击与高亮彻底失效，且不报任何错。
- 星级来自 `/me/stars`（`{stars: {"<id>": n}}`），未结算的关卡不在 map 里，
  客户端按 0 处理 —— 服务端不补 0。
- 解锁判定：`unlockedLevel = min(100, max(1, maxStage + 1))`。

### 7.2 背包页

- 出战槽位最多 4 个主动槽，保存走 `PUT /me/loadout`（必须落服务端，
  槽位参与回放哈希）。
- 技能升级：费用与上限由服务端裁定，客户端**不预判**能不能升；
  回执带 `wallet`，直接刷新余额（避免连点看到中间态）。
- 熔炼：材料不齐时**不渲染入口**（`ENGINEERING.md` 第 19 条）。
- 宝石：`GET /me/gems` 列出实例，`POST /me/gems/:id/wash` 消耗 1 枚洗练券
  重 roll 词条（品质不变）。
- 装备（P4-1）：`GET /me/equipment` 返回目录 + 拥有/装配状态，
  `PUT /me/equipment` 整组替换（`equipment_ids`）—— 不用「装一件/卸一件」
  的增量端点，避免出现「同部位两件都 equipped」的中间态。

### 7.3 专精页

- 每层 4 选 2，第 2/3 层必须先点亮前一层。
- 点数有限（初始 3，每 10 级 +1），所以必须做方向选择。
- 重铸：消耗 1 件 `mastery_reset`，清空全部节点，点数预算不变。
  没有它，这件道具就是「买了不知道干嘛」。

### 7.4 我的页（防线）

```
saveDefense()
  ├ 取当前构筑（技能/装备/专精/工事）
  ├ 名称、works 白名单
  └ POST /defenses/save

challengeDefense(view)
  ├ fetchLevel(1)                  ← 标准战场（含 waves）
  ├ runChallenge(view, deps)       ← 本地模拟
  │     种子：view.id + 当前分钟桶（可注入 seedOverride 供测试）
  │     attacker = applyWorks(我的 attacker, 防守方 works)
  │     跑满 MAX_TICKS → won / hp_left_pct / replay_hash / replay_skills
  └ POST /defenses/:id/challenge
```

挑战胜负由客户端裁决（`KNOWN_LIMITS` 第 3 条），服务端守住的是次数、
护盾、被偷上限、字段边界与**挑战者构筑的 S 段**。

### 7.5 验真页

```
输入 battle_id
  → GET /battle/:id/replay   {level, seed, build, card_picks, expected_hash}
  → replay(info, {level, enemies, skills, scoreRules})
        equipped = equippedFromSnapshot(build, skills)
        attacker = attackerFromSnapshot(build)
        seed = BigInt(info.seed)
        activeSlots = build.active_slots
        重跑到结束 → computedHash
  → POST /battle/verify {battle_id, replay_hash: computedHash}
```

重放失败的四类前置错误都会显式提示：缺关卡、缺构筑、构筑无技能、
种子非法 —— 而不是笼统地报「不匹配」。

---

## 8. 测试体系

| 类型 | 代表 | 说明 |
|---|---|---|
| 确定性 | `determinism.test.ts`、`lcg.test.ts` | 同种子同输入必须同输出 |
| 反通胀 | `anti_inflation.test.ts` | 反应伤害中攻击力权重 ≤ 30% |
| 平衡扫描 | `balance.probe.test.ts` | 100 关 × 三档加成，改动数值前后对照 |
| 契约 | `reaction_contract` / `card_picks_contract` / `replay_skills_contract` | 与 Go 共读 `testdata/*.json` |
| 形态/分布 | `armor_shape` / `elem_coef_shape` / `settle_bounds_parity` | 结构上测不出的那类断言 |
| 端到端（无头） | `i6.e2e.test.ts` | 开局 → 战斗 → 结算 → 重放全链路 |
| 页面 | `pages/*/*.test.ts` | 组件行为 + 接口调用 |
| 工具链 | `toolchain_pin.test.ts` | 锁 vite / vitest 版本 |

页面测试用 `src/test/uni-mock.ts` 与 `src/test/page.ts` 提供 `uni` 替身；
`src/test/uni-app-stub.ts` 处理 `onLoad` / `onUnload` 等生命周期钩子。

---

## 9. 常见陷阱清单

| 陷阱 | 症状 | 正确做法 |
|---|---|---|
| 本地拼 `EquippedSkill` | 实战哈希与重放哈希永远不符 | 只用 `equippedFromSnapshot` |
| S 段用 `replaySkillsSegment()` 上报 | 86% 的正常对局被判 422 | 上报用 `buildSnapshotSegment()` |
| 改 `level.base_hp` 做复活 | 加成完全无效 | 用 `extraBaseHp` |
| 用编译期 `ACTIVE_SLOTS` | 专精「额外插槽」点了没用 | 读 `build.active_slots` |
| 用 `DEFAULT_SCORE_RULES` | 星级与服务端漂移 | 读 `/config` 的 `score_rules` |
| 本地推算 attacker | I-6 验真误判 | 读 `build.attacker` |
| 并发登录 / 并发刷新 | 建两个号 / 静默登出 | 单飞 |
| 视口混用 CSS 像素与 dpr 像素 | 画布空白 | `setViewport(cssW, cssH)` |
| 登出不存新 guest_token | 进度凭空消失 | 先存后清 |
| 用 `Date.now()` 比服务端绝对时间 | 护盾状态判断错误 | `estimateServerNowMs()` |
