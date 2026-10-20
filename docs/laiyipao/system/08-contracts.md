# 08 · 跨端契约

> 本文描述**客户端与服务端之间那些"两边对不上也不报错"的约定**：
> 字段命名、整数编码、随机数底座、回放哈希、S 段口径、规则下发、契约夹具。
>
> 为什么单独立一份：这类问题的共同特征是**两端各自自洽**。
> 客户端算一套、服务端算另一套，两端都"没 bug"，但真机玩家会莫名失败 ——
> 而且没有任何编译错误或测试会提前发现（见 `ENGINEERING.md` 第 4、6 条）。

---

## 1. 契约清单总览

| # | 契约 | 双端位置 | 守卫 |
|---|---|---|---|
| C1 | JSON 字段一律 `snake_case` | Go `json` tag ↔ TS 接口 | 契约覆盖测试 + 文档漂移测试 |
| C2 | 超过 2^53 的整数一律**字符串**下发 | `BattleTokenResp.Seed` ↔ `bt.seed` | 类型定义 |
| C3 | 确定性 PRNG（PCG-XSH-RR） | `domain.LCG` ↔ `game/lcg.ts` | 已知向量对拍 + `LCG_KNOWN_VECTORS` |
| C4 | 定点整数 `bigint` 存千分比 | `domain/*` ↔ `game/fixed.ts` | 一致性测试 |
| C5 | 攻方属性字段集（12 个） | `domain.Attacker` ↔ `damage.ts` | `attacker_fields_test.go` |
| C6 | 回放哈希前缀三段 | `battle.go` / Go 镜像 ↔ `engine.ts` | `replay_hash_contract_test.go` |
| C7 | S 段格式与**两个口径** | `domain/replayskills.go` ↔ `engine.ts` | `replay_skills_contract` + `formula_vectors.json` |
| C8 | `card_picks` 整数编码 | `battle.go` ↔ `engine.ts` | `card_picks_contract` |
| C9 | 分数规则 / 技能规则由 `/config` 下发 | `domain.DefaultScoreRules` ↔ `score.ts` / `skill.ts` | `TestSkillRulesMatchServerContract` |
| C10 | 关卡种子推导 | `domain.levelSeedOf` ↔ `levelgen` | `testdata/level_seeds.json` |
| C11 | 反应数值表 | `domain.AllReactionSpecs` ↔ `elements.ts` 的 `REACTIONS` | `reaction_specs.json` |
| C12 | 三端榜单类型白名单 | Go `leaderboardKinds` ↔ `rank.vue` 的 tabs | `TestLeaderboardKindIsValidated` |
| C13 | 战斗步长与槽位常量 | `domain.TickMs` / `BaseSkillSlots` ↔ `TICK_MS` / `ACTIVE_SLOTS` | `TestTickMsMatchesClient` 等 |
| C14 | 「今天」的日界 | Go `periodStart` ↔ 无客户端对应 | 单元测试 + 传日期字面量 |

C1–C14 每一条都对应一次真实故障。下面逐条展开。

---

## 2. 命名与编码约定

### 2.1 C1：字段一律 snake_case

Go 在没有 `json` tag 时按**字段名原文（PascalCase）**序列化。
客户端类型定义用的是 snake_case，于是对不上时：

- 客户端读到的每个字段都是 `undefined`；
- **不报任何错**；
- 症状是「战斗里一个敌人都没有」「章节下拉框没有选项」这类看起来像数据问题的现象。

已知踩过三次：

| 位置 | 写错的字段 |
|---|---|
| `miniapp/src/api/client.ts` 的 `GameConfig` | `Output/A/B/Key/Name/StartLevel/EndLevel/BossEnemyId/TerrainKind` |
| `admin/src/views/WikiView.vue` 的本地类型 | `reactions` / `chapters` 两组 |
| `admin/src/api/index.ts` / 服务端 `domain.Diagnose` | 见 [05 §3.8](./05-api-reference.md#38-诊断) 的不一致记录 |

判据：**新增接口字段时，Go 侧 tag 与 TS 侧接口必须逐字写在同一处**（写在同一轮提交里），
否则就必须有契约测试覆盖。

### 2.2 C2：超 2^53 的整数用字符串

| 字段 | 类型 | 位置 |
|---|---|---|
| `BattleTokenResp.seed` | string | `/battle/token` |
| `VerifyResult.seed` | string | `/battle/verify`、`/admin/battles/:id/verify` |
| `ReplayInfo.seed` | string | `/battle/:id/replay` |
| `ChallengeInput.seed` | string | `/defenses/:id/challenge`（上报） |
| `LevelResp.seed_str` | string | 关卡的字符串种子 |
| `battle_records.replay_hash` | string | 16 位 hex |

`int64` 超过 `2^53` 后 JSON number 在 JS 侧解析即失精 ——
客户端拿到的种子与库里那个不是同一个数，回放哈希必然对不上（I-6 失效）。

全局规则：**任何 int64 字段在跨端 JSON 里都必须是字符串**。

---

## 3. C3–C4：确定性底座

### 3.1 PCG-XSH-RR

两端用同一个 LCG 变体，**不用 `Math.random()`**：

```
state = uint64(seed) ^ 0x2545F4914F6CDD1D
next():
    old = state
    state = (old * 6364136223846793005 + 1442695040888963407) mod 2^64
    xorshifted = uint32(((old >> 18) ^ old) >> 27)
    rot = old >> 59
    return (xorshifted >> rot) | (xorshifted << ((64 - rot) mod 64))
```

- Go：`domain/levelgen.go` 的 `LCG`（`newLCG` / `Next` / `Intn`）；
- TS：`game/lcg.ts` 的 `LCG`（`constructor` / `next` / `intn` / `permille`）。

选它而不是 mulberry32 的理由：只用 64 位整数运算，
Go 用 `uint64`、TS 用 `BigInt`（每次运算后 `BigInt.asUintN(64, …)`），
两端能产出**逐位相同**的序列。

> `LCG_KNOWN_VECTORS`（`seed=12345` 的前三个输出）是**手写期望值**。
> 用实现生成期望值就是自证，测试会永远绿。

### 3.2 定点整数

所有比例/速度/坐标用 `bigint` 存千分比（`×1000`）。
不用 `number` 的理由见 [03 §3.1](./03-miniapp.md#31-定点整数)。

硬上界（既是反通胀红线，也是契约）：

| 常量 | Go | TS | 值 |
|---|---|---|---|
| 反应攻击力权重上限 | `MAX_REACTION_ATTACK_WEIGHT` | `fixed.ts` | `300n`（30%） |
| 抗性上限 | `MAX_RESIST` | `fixed.ts` | `500n` |
| 护甲上限 | `MAX_ARMOR` | `fixed.ts` | `750n` |
| 千分比基数 | `permille` | `PERMILLE` | `1000n` |

---

## 4. C5 与 C13：常量与字段集双端对照

### 4.1 常量

| 常量 | Go | TS | 值 |
|---|---|---|---|
| 战斗步长 | `domain.TickMs` | `TICK_MS` | 50 ms |
| 逻辑频率 | — | `TICK_HZ` | 20 Hz |
| 单局最大 tick | Go 镜像 | `MAX_BATTLE_TICKS` | 30000（25 min） |
| 基础插槽 | `domain.BaseSkillSlots` | `TOTAL_SLOTS` | 5（4 主动 + 1 被动） |
| 主动槽 | Go 镜像 | `ACTIVE_SLOTS` | 4 |
| 被动槽 | — | `PASSIVE_SLOT` | 4 |
| 每层专精上限 | `PerLayerPickLimit` | mastery 页 | 2（4 选 2） |
| 热量上限 | `domain.HeatMax` | `HEAT_MAX` | 100 |
| 过热时长 | `OverheatDurationMs` | `OVERHEAT_DURATION_MS` | 2000 ms |
| 每秒衰减 | `HeatDecayPerSec` | `HEAT_DECAY_PER_SEC` | 10 |
| 每波弃牌 | — | `DISCARD_PER_WAVE` | 1 |
| 标准关卡总数 | `domain.TotalLevels` | 100 | 100 |

> `TICK_MS` 与 `TickMs` 由 `TestTickMsMatchesClient` 锁住 ——
> 只改一边，要么所有已存战报的哈希对不上，要么上界放得比预想的松。

### 4.2 攻方属性字段集（C5）

`build.attacker` 的 10 个字段，两端**逐字一致**：

| JSON 字段 | 含义 |
|---|---|
| `attack` | 攻击力 |
| `crit_permille` | 暴击率 |
| `crit_multiplier_permille` | 暴击倍率 |
| `reaction_mult_permille` | 反应倍率加成 |
| `element_cap` | 元素层数上限 |
| `reaction_tier` | 反应等级 |
| `element_coef_permille` | 元素系数 |
| `heat_cap_permille` | 热量上限加成（第 150 轮新增） |
| `armor_permille` | 防线护甲（新增） |
| `mechanic_permille` | 机制卡强度（新增） |

后三个用 `?? 0` 兜底：老版本服务端的 `build_snapshot` 里没有它们，
`BigInt(undefined)` 会抛 TypeError，而缺省 0 正好等价于「没有装备/宝石/专精」。

`build.active_slots` 同属这一组，缺省 4（见 03 §3.3）。

---

## 5. C6：回放哈希（I-6 的核心）

```ts
prefix =
  `L${level.id}` +
  `|A${attack}.${critPermille}.${critMultiplierPermille}` +
     `.${reactionMultPermille}.${elementCap}.${reactionTier}.${elementCoefPermille}` +
  `|S${replaySkillsSegment()}` +
  `|X${extraBaseHp ?? 0n}`

hash = hex16(fnv1a64([prefix, ...events.map(e => `${e.t}|${e.type}|${e.a}|${e.b}|${e.c}`)].join(';')))
```

### 5.1 三段各自的用途

| 段 | 内容 | 服务端能否独立重算 |
|---|---|---|
| `L` | 关卡 id | 能（`level.id` 唯一确定 `base_hp`、波次、地形） |
| `A` | 攻方系数（**含局内 buff**） | **不能** —— 含战斗选卡加成，服务端结算时不知道 |
| `S` | 技能槽配置（**不含 buff**） | **能** —— 完全由构筑决定 |
| `X` | 复活额外血量 | 能（服务端在 `StartBattle` 里算出并下发） |

把 `A` / `S` / `X` 放进前缀是为了堵住一类作弊：
若只哈希事件流，一场全是漏怪的战斗（`kills=0`）事件里没有任何伤害数值，
此时改攻击力、元素系数都不改变哈希。前缀把这类改动纳入证伪范围。

种子**不进**前缀：它的影响已完整体现在事件流里（刷怪洗牌、暴击滚点、命中次序），
重复计入反而让两端更难对齐。

`X` 段未复活时写 `|X0` 而不是省略整段：省略会让「有前缀」与「无前缀」
两种长度不同，将来加字段时容易混淆。

### 5.2 FNV-1a 64

双端用**规范实现**，不手写循环：

- Go：`domain/hash.go` 用 `hash/fnv.New64a`；
- TS：`game/lcg.ts` 的 `fnv1a64`。

手写版本一旦在「是否含终止字节」或移位顺序上与标准差一位，
两端会静默失配，症状是「快照摘要永远对不上」—— 看起来像数据坏了。

`SnapshotHash`（防线快照）与 `replay_hash` 用同一个 FNV-1a + 16 位 hex 格式，
方便前端统一处理。

---

## 6. C7：S 段口径（最容易写错的一条）

### 6.1 格式

```
每槽一条 slot:skillId:baseDamage:applyStacks:heatCost
按**字符串**升序，逗号连接
```

例：`0:1:120:2:20,1:4:100:1:18`

两端必须逐字一致：

| 侧 | 定义 |
|---|---|
| 服务端 | `domain/replayskills.go`（`ReplaySkillsMaxLen = 1024`） |
| 客户端 | `engine.ts` 的 `replaySkillsSegmentOf(skills)` |

排序用**字符串**比较，不是槽位数值：`.sort()` 无参数时按 UTF-16 码元，
所以 `10:...` 排在 `2:...` 前面。Go 侧用 `sort.Strings` 才等价 ——
写成按槽位数值排会锁死 10 槽以上的玩家。

### 6.2 两个口径绝不能混用

| 用途 | 方法 | 是否含局内升格 |
|---|---|---|
| 计算 `replayHash()` | `engine.replaySkillsSegment()` | **含**（取牌会升格技能） |
| 上报给服务端 | `engine.buildSnapshotSegment()` | **不含**（构造时冻结） |

混用的后果（实测）：取过技能卡的正常对局上报 `0:1:120:2:20`，
服务端按 DB 重算出 `0:1:100:1:20` → `ErrReplaySkillsMismatch` → 422。
`rollWaveCards` 固定把技能卡放在手牌下标 0，5 波各取 1 张时
命中技能卡的概率约 **86%** —— 也就是绝大多数玩家会被判作弊。

`buildSnapshotSegment()` 读的是 `this.buildSnapshotSkills`，
它必须在**构造那一刻**算出字符串：`applyCard` 会 `this.skills = this.skills.map(...)`
整体替换数组元素，持有旧数组引用并不能免疫升格。

### 6.3 它能抓住什么、抓不住什么

| 能抓住 | 抓不住 |
|---|---|
| 伪造 `base_damage`（等价于假报技能等级） | 伪造 `A` 段（攻方系数）—— 需要模拟，已记入已知边界 |
| 上报一套与 `user_skill_slots` 不同的技能 | 从始至终伪造一整套自洽数据（R1 整局重算的目标） |
| 槽位错位 | 防线挑战的胜负本身（I-5 取舍） |

空串**放行**：「没上报」与「上报了但不对」要分开 ——
后者才是作弊，前者只是老客户端。

---

## 7. C8：`card_picks` 编码

`battle_records.card_picks` 是逗号分隔的紧凑文本（迁移 00008），
每波一个整数。为什么不用 JSONB 数组：数组极小（100 关最多 800 个整数），
紧凑文本比 JSONB 省空间也省解析开销，且便于直接比对。

| 取值 | 含义 |
|---|---|
| `0` / `1` / `2` | 普通取牌（无弃牌），下标是取牌当次的手牌下标 |
| `-1` | 整波跳过（无弃牌） |
| `-(3 + code)` | 本波弃过牌；`code ∈ [0, PICK_CODE_MAX]` |

常量：

| 常量 | Go | TS | 值 |
|---|---|---|---|
| 编码基址 | — | `PICK_CODE_BASE` | 3 |
| 弃牌码上限 | — | `PICK_CODE_MAX` | 19 |
| 最小值 | — | `PICK_MIN` | -22 |

**为什么重写过一次**（第 159 轮）：旧编码（`-3..-5` 弃+取、`-6..-8` 弃+跳）
有两个致命缺陷：

1. 表达不了「弃的是哪一张」—— 重放侧固定弃 `hand[0]`，
   而 UI 允许弃任意一张，弃了非首张的正常玩家必然失配；
2. 表达不了「弃多于一张」——「快速装填」卡（每波弃牌次数 +1）
   让 `discardsLeft > 1` 成为正常玩法。

新编码把弃牌集合写成**手牌位掩码**：一个 `code` 同时携带
「弃了原手牌的哪几张」+「（不弃完时）取剩余第几张 / 跳过」。
两端由 `card_picks_contract` 测试锁死，定义位置：

| 侧 | 函数 |
|---|---|
| 客户端 | `engine.ts` 的 `encodeDiscardCode` / `decodeDiscardCode` |
| 服务端 | `domain/battle.go` 的对拍实现 |

> 这一环是 I-6 重放闭环的最后一环：选牌会改变后续战斗，
> 缺了它第三方无法复现原局，验真会把正常对局判成伪造。

---

## 8. C9：由 `/config` 下发的规则

**客户端不得硬编码**下面任何一组数值 —— 必须从 `/config` 读。
硬编码的后果不是「不一致」，而是「两端各自自洽、没有任何行为测试会发现」。

### 8.1 `score_rules`

| 字段 | Go 定义 | TS 转换 |
|---|---|---|
| `per_damage_unit` | `domain.scorePerDamageUnit = 100` | `scoreRulesFromServer` |
| `on_kill_normal` | `500` | 同上 |
| `on_kill_boss` | `5000` | 同上 |
| `star_target_ratio` | `[600, 850, 980]`（千分比） | 同上 |
| `score_full_at_sec` | `ScoreFullAtSec = 60` | 同上 |

唯一真相源是 `domain.DefaultScoreRules()`。`levels.star_targets` 就是服务端
用这份规则算出来的 —— 客户端用自己的一份就会与门槛算法漂移。

`DEFAULT_SCORE_RULES` 与 `DEFAULT_SKILL_RULES` 只是**离线兜底**，
由 `skill.test.ts` 的 `TestSkillRulesMatchServerContract` 断言不漂移。

### 8.2 `skill_rules`

| 字段 | 说明 |
|---|---|
| `max_level` | 等级上限 |
| `coef_permille` | 每级伤害系数 |
| `base_cost` | 升级费用基数 |

客户端 `skill.ts` 的 `upgradeCostFrom` / `coefPermilleAt` / `isMaxLevel`
全部以这份为准，**不预判**能不能升 —— 余额、满级、并发推满都由服务端说了算。

### 8.3 其他下发项

| 项 | 用途 |
|---|---|
| `reactions` | 7 条反应链的全部数值（C11） |
| `rating_weights` | 构筑评分五维权重 |
| `chapters` | 6 章参数（含 `terrain_kind` / `boss_enemy_id`） |
| `gem_qualities` | 品质 → 词条数 / 倍率 |
| `mastery_families` | 8 系 × 12 节点的种类与数值 |
| `level_seeds`（夹具，非接口） | 100 关种子推导 |

### 8.4 C10 / C11：两份「手工同步的副本」

这两处**刻意**在两端各有一份定义，靠契约文件锁住：

| 副本 | Go | TS | 夹具 |
|---|---|---|---|
| 反应数值表 | `domain.elements.go` 的 `reactionSpecs` | `game/elements.ts` 的 `REACTIONS` | `testdata/reaction_specs.json` |
| 关卡种子推导 | `domain.levelSeedOf` | `levelgen`（镜像） | `testdata/level_seeds.json` |

`level_seeds.json` 存在的理由：客户端曾用
`expect(levelSeed(1)).toBe(levelSeed(1))` 来"验证与 Go 侧一致" ——
那是恒等式，且 PHI 常量在测试里本地复制了一份。所以夹具里放的是
Go 侧算出的**具体数值**，客户端对拍的也是具体数值。

---

## 9. 契约夹具（`server/testdata/`）

夹具由 `cmd/vectors` 从 **Go 侧真相源**生成，Go 与 TS 两侧的测试读同一份文件。

```bash
go run ./cmd/vectors          # 写入全部文件
go run ./cmd/vectors -check   # 只校验是否为最新，不写盘（CI 用）
```

| 文件 | 内容 | 大小 | 消费者 |
|---|---|---|---|
| `formula_vectors.json` | 跨端公式的**字面期望值**（人工推导） | 15 KB | Go + TS 对拍 |
| `reaction_specs.json` | 7 条反应链数值 | 2.6 KB | Go + TS |
| `level_seeds.json` | 100 关种子 | 2.5 KB | TS |
| `replay_vectors.json` | 回放向量 | 26 KB | Go + TS |
| `smoke_levels.json` | 真实关卡（暴露引擎停摆的关键夹具） | 344 KB | TS 引擎冒烟 |
| `engine_pure_vectors.json` | Go 引擎镜像的纯输出向量 | 19.6 MB | Go |

两条铁律：

1. **夹具只导数据，不导期望值**。期望值必须人工推导后写进
   `formula_vectors.json` —— 用实现生成期望值就是自证，测试会永远绿。
2. **不要手工编辑**。`vectors -check` 会发现手改并报错。

`smoke_levels.json` 的存在理由是具体教训：引擎冒烟测试此前全用手工构造的关卡，
于是「热量吸收态导致战斗永久停摆」与「刷怪 delay 单位错 50 倍」
这两个致命缺陷一路活到端到端试玩才暴露。**真实内容表是唯一能暴露这类问题的夹具。**

---

## 10. 一致性守卫清单

改任何契约前，先确认这些守卫都会跑：

| 契约 | Go 守卫 | TS 守卫 |
|---|---|---|
| C1 字段名 | `consistency_test.go`、`docs_drift_test.go` | `client.test.ts`、`contract_coverage.test.ts` |
| C3 PRNG | `levelgen_test.go` 的已知向量 | `lcg.test.ts` |
| C4 定点数 | `crit_test.go`、`damage_test.go` | `fixed.test.ts`、`damage.test.ts` |
| C5 攻方字段 | `attacker_fields_test.go` | `loadout_attacker.test.ts` |
| C6 回放哈希 | `replay_hash_contract_test.go` | `replay.test.ts`、`i6.e2e.test.ts` |
| C7 S 段 | `replay_skills_contract_test.go` | `replay_skills_contract.test.ts`、`replay_skills_build_snapshot.test.ts` |
| C8 卡牌 | `card_picks_contract_test.go` | `card_picks_contract.test.ts` |
| C9 规则 | `config_verify_test.go` | `skill.test.ts`、`score.test.ts` |
| C11 反应表 | `reaction_contract_test.go` | `reaction_contract.test.ts`、`reaction_descr.test.ts` |
| C12 榜单 | `TestLeaderboardKindIsValidated` | `rank.test.ts` |
| C13 常量 | `TestTickMsMatchesClient` | `settle_bounds_parity.test.ts` |
| 引擎镜像 | `engine_divergence_test.go`、`engine_pure_vectors_test.go` | `engine.test.ts` |

四条结构性守卫（AST / 扫描类，不依赖具体行为）：

| 守卫 | 检查什么 |
|---|---|
| `ownership_scan_test.go` | 所有 UPDATE/DELETE 是否带归属条件 |
| `readonly_guard_test.go` | 管理端写端点是否挂 `requireWritable` |
| `prod_guard_coverage_test.go` | 敏感默认值是否都在 prod 守卫清单里 |
| `unused_param_test.go` | 函数参数是否被忽略 |

---

## 11. 改一条契约的流程

以「新增一个攻方字段」为例：

```
1. 改 domain.Attacker（加字段 + json tag）
2. 改 miniapp damage.ts 的 Attacker 接口（同名 + 相同语义）
3. 改 loadout_attacker.go / build_parts.go（产出侧）
4. 改 store/game.ts 的 attacker computed（?? 0 兜底老数据）
5. 若它进回放哈希前缀 → 同步改 engine.ts 的 replayHash()
      ⚠️ 这会让所有存量战报失去可重放性！
6. go run ./cmd/vectors          重生成夹具
7. go test ./... && pnpm test    两端必须同时绿
8. 跑 scripts/e2e.ps1
9. 更新本文档对应小节
```

第 5 步是最危险的一步。**回放哈希前缀的任何改动都会让存量战报失配**，
处理流程见 `OPERATIONS.md` §4（先重生成夹具 → 全量测试 → e2e → 公告说明）。
行为测试全绿**不**代表可重放性没被破坏 —— 那是两种不同的不变式。

### 11.1 快速自检问题

改完契约后问自己四个问题，有一个答不上来就停下：

1. 旧客户端/旧战报还能正常工作吗？（兼容层在哪里）
2. 两端各自由一份定义的地方，是否都改了？
3. 夹具需要重生成吗？重生成之后期望值要不要人工重推？
4. 有没有测试能在两端漂移时**变红**？如果没有，先补测试再改代码。

---

## 12. 已知的契约缺口

诚实记录当前仍然存在的、KNOWN_LIMITS 未覆盖或已覆盖的契约问题：

| # | 缺口 | 状态 |
|---|---|---|
| 1 | `/diagnose` 下发 PascalCase（`domain.Diagnose` 无 json tag），客户端读小写 → 诊断卡空白 | **未修**，见 [05 §3.8](./05-api-reference.md#38-诊断) |
| 2 | 反应数值表、关卡种子仍是「手工同步的两份副本」，靠夹具锁 | 已接受（夹具足够） |
| 3 | `A` 段（攻方系数）服务端无法独立重算 | 已知边界，R1 整局重算的目标 |
| 4 | 防线挑战胜负客户端裁决 | 已知边界第 3 条（DECISIONS D4 已补 S 段） |
| 5 | `card_picks` 对旧战报缺失时只能「看哈希是否巧合相符」 | 已声明 |
| 6 | `game_configs` 表当前不被读取路径消费 | 预留 |

第 1 条是唯一的**未修缺陷**，其余都是经过论证的取舍。
