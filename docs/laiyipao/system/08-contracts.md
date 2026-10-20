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

联盟规则：**任何 int64 字段在跨端 JSON 里都必须是字符串**。
