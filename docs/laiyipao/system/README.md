# 来一炮 · 系统文档（索引）

> 本目录描述**系统是怎么被构建出来的**：三端的架构、模块边界、业务流程、
> 接口与数据结构、配置与部署、端与端之间的契约。
>
> 与既有文档的分工见下方「文档地图」。玩法意图看 `GAME_DESIGN.md`，
> 工程红线看 `ENGINEERING.md`，没做什么看 `KNOWN_LIMITS.md`，
> 上线值班看 `OPERATIONS.md`。本目录不重复它们，只做**实现层的对照表**。

代码基线：`laiyipao/` 工作区当前提交（`optimism20261012` 之后的
D1–D5 落地轮次，含结算重算闸门 R1、宝石洗练、道具消费闭环、防线 S 段校验；
2026-10-21 P1–P6 轮次：tygen 生成后台 TS 类型、装备获取/装配、战斗放弃退款、
管理端分页/详情/重算档位切换、验真幂等、玩家账号登录、诊断字段修复）。

---

## 1. 文档地图

| 文档 | 回答的问题 | 主要读者 |
|---|---|---|
| **README.md**（本文） | 从哪读起、术语是什么、系统长什么样 | 所有人 |
| [01-architecture.md](./01-architecture.md) | 三端怎么连起来、谁是谁的权威、一次请求怎么走 | 架构 / 新人 |
| [02-server.md](./02-server.md) | Go 后端怎么分层、每个包管什么、核心流程怎么编排 | 后端 / 测试 |
| [03-miniapp.md](./03-miniapp.md) | 小程序端的引擎 / 渲染 / 页面 / 状态怎么切分 | 客户端 / 测试 |
| [04-admin.md](./04-admin.md) | 运营后台有哪些页、每页打哪些接口、权限怎么收口 | 前端 / 运营 |
| [05-api-reference.md](./05-api-reference.md) | 75 个端点的入参、出参、错误码、限流 | 前后端 / 联调 |
| [06-data-model.md](./06-data-model.md) | 50 张表的字段、索引、JSONB 结构、不变量 | 后端 / 数据 |
| [07-deployment.md](./07-deployment.md) | 环境变量、构建、nginx / systemd、迁移、回滚 | 运维 / 发布 |
| [08-contracts.md](./08-contracts.md) | 跨端字段命名、种子、回放哈希、S 段、卡牌编码 | 全体 |

既有文档（同级目录）：

| 文档 | 内容 |
|---|---|
| `GAME_DESIGN.md` | 玩法设计、7 个创新点（I-1…I-7）、100 关章节、原创性声明 |
| `ENGINEERING.md` | 19 条关键工程约束，每条对应一次真实故障 |
| `IMPLEMENTATION.md` | 内容规模、平衡实测、验证状态、测试方法论复盘 |
| `KNOWN_LIMITS.md` | 29+ 条「没做且不假装做了」的诚实声明 |
| `OPERATIONS.md` | 部署步骤、环境变量全表、迁移回滚、备份恢复、故障处置 |
| `DECISIONS.md` | D1–D5 待拍板决策单与批注 |

---

## 2. 系统全景

```
                       ┌──────────────────────────────────────────┐
                       │            玩家（微信小程序 / H5）        │
                       └───────────────────┬──────────────────────┘
                                           │ HTTPS  Bearer
                       ┌───────────────────▼──────────────────────┐
                       │  miniapp（uni-app + Vue 3 + Pinia）       │
                       │  ├ game/    纯 TS 战斗内核（零 uni / 零绘制）│
                       │  ├ render/  Canvas 2D 只读渲染            │
                       │  ├ api/     请求封装（401 刷新 / 304 缓存）│
                       │  ├ store/   全局状态（配置 / 钱包 / 进度） │
                       │  └ pages/   12 个页面                     │
                       └───────────────────┬──────────────────────┘
                                           │
        ┌──────────────────────────────────▼──────────────────────────────────┐
        │                    server（Go 1.26 + Fiber v2 + pgx）                 │
        │  httpapi ── service ── domain（纯函数 / 双端镜像）                    │
        │      │          │                                                     │
        │      │          └── store（pgx 连接池 / goose 迁移）                  │
        │      └── auth（HS256 JWT / refresh token / 游客凭证）                 │
        └───────────┬──────────────────────────────────┬──────────────────────┘
                    │                                  │
        ┌───────────▼───────────┐          ┌───────────▼──────────────────┐
        │ PostgreSQL 15+        │          │ admin（Vue 3 + Element Plus）│
        │ 50 张表 / 19 个迁移   │◀─────────│ 11 个运营视图 + 玩法文档站   │
        └───────────────────────┘   Bearer └──────────────────────────────┘
```

三端职责的一句话版本：

- **server** 是唯一的权威。它决定关卡内容、战斗凭证、结算是否可信、
  经济余额、进度解锁、验真结论。它**不跑战斗引擎**（I-5 / I-6 的取舍）。
- **miniapp** 是唯一跑战斗的地方。它用服务端下发的
  「关卡 + 种子 + 构筑 + 分数规则」在本地确定性模拟，并把结果上报。
- **admin** 是运营面。它只读写服务端的 `/api/v1/admin/*`，
  不直连数据库，也不持有任何业务逻辑。

---

## 3. 三端一览

| 维度 | server | miniapp | admin |
|---|---|---|---|
| 语言 / 框架 | Go 1.26.10 / Fiber v2 | TypeScript 5.9 / uni-app 3 / Vue 3.4 | TypeScript 5.9 / Vue 3.5 / Vite 8 |
| 状态管理 | 无（请求级） | Pinia 2 | Pinia 3（仅登录态） |
| UI | — | 自绘 Canvas + 原生组件 | Element Plus 2.14（按需引入） |
| 数据访问 | pgx v5 + goose | `uni.request` | `fetch` |
| 鉴权 | HS256 JWT + 可吊销 refresh / 管理员 sid 会话 | localStorage（`uni.*Storage`） | localStorage |
| 端口（dev） | `:8080` | `:5174`（H5） | `:5175` |
| 产物 | 单二进制 `lyp-server-amd64` | `dist/build/h5` 或 `dist/dev/mp-weixin` | `dist/`（`base=/laiyipao-admin/`） |
| 测试 | `go test ./...`（267 个 Go 文件，含大量契约/守卫测试） | Vitest（内核可无头跑） | Vitest + 体积预算 |
| 代码量（非测试） | ≈ 25.3k 行 Go | ≈ 14.2k 行 TS/Vue | ≈ 6.4k 行 TS/Vue |

内容规模（服务端 `domain/content.go` 为唯一真相源，`/config` 下发）：

| 项 | 数量 | 说明 |
|---|---|---|
| 关卡 | 100（6 章） | 生成器产出，落库为可逐关手改的配置行 |
| 敌人 | 22 条定义 / 21 种可用 | 5 类抗性 + BOSS 高抗/负抗组合 |
| 技能 | 24 基础 + 18 合成 = 42 | 8 系，每系 3 个基础技 |
| 合成配方 | 18 | A+B → 高阶技能 |
| 装备 | 18 | 6 部位 × 3 阶 |
| 宝石 | 8 属性 | 品质是实例属性，词条走洗练池 |
| 皮肤 | 6 | 机制型被动 |
| 专精 | 8 系 × 12 节点 = 96 | 每层 4 选 2，前置层约束 |
| 反应链 | 7 | 5 元素两两组合（动能只作后手） |
| 地形 | 5 类 | 油桶 / 潮汐闸 / 旋转风障 / 崩塌掩体 / 蓄能塔 |
| 数据库表 | 50 | 19 个 goose 迁移 |
| HTTP 端点 | 75（玩家 44 + 管理 31） | 见 05 |

---

## 4. 术语表

阅读代码与文档时会反复遇到这些词。**同一个词在三端必须是同一个意思**，
否则就会出现「客户端算一套、服务端算另一套」这类最难排查的漂移。

### 4.1 战斗与数值

| 术语 | 定义 | 代码位置 |
|---|---|---|
| **定点整数** | 所有比例/速度/坐标用整数存千分比（`×1000`），避免浮点跨端不一致 | `domain/*`、`game/fixed.ts` |
| **千分比** | 值 `300` = 30%。字段名常带 `_permille` / `_pct`，**语义必须逐字段确认是加法还是乘法** | `ENGINEERING.md` 第 13 条 |
| **攻方（Attacker）** | 玩家侧的战斗属性集合：攻击、暴击、反应倍率、元素上限、护甲、热量上限、机制强度 | `domain/attacker.go`、`game/damage.ts` |
| **守方（Defender）** | 敌人侧属性：血量、护甲、护盾、五维抗性、元素层数 | `game/damage.ts` |
| **元素层数（stacks）** | 敌人身上附着的元素计数，受 `element_cap` 限制 | `domain/elements.go` |
| **反应链（reaction）** | 已附着元素 A + 新施加元素 B 触发的效果，**不可交换** | `domain/elements.go` 的 `reactionTable` |
| **反应等级（tier）** | 反应强度档位，受关卡 `max_reaction_tier` 限制 | `domain/elements.go` |
| **热量（heat）** | 释放技能累积的资源，满 100 触发 2s 过热（禁手） | `game/heatmap.ts` |
| **弃牌回热量** | 每波可弃 1 张手牌，弃牌返还 1 点热量 | `heatmap.ts` 的 `DISCARD_*` |
| **插槽（slot）** | 主动 0–3 共 4 槽 + 被动 4 号槽，基础 5 槽；专精可 +N | `domain/power.go` 的 `BaseSkillSlots` |
| **专精（mastery）** | 8 系 × 3 层 × 4 槽的点数树，每层最多选 2 个 | `domain/power.go` |
| **构筑评分（rating）** | 五维评分：元素覆盖 / 反应覆盖 / 专精 / 装备契合 / 机制深度 | `domain/power.go` 的 `DefaultRatingWeights` |
| **战力（power）** | 单一标量，用于防线排序与效率榜，**不是**评分 | `service/game.go` 的 `computePower` |
| **地形（terrain）** | 关卡内可交互物件，5 类，按章节主导 + 概率生成 | `domain/terrain.go`、`game/terrain.ts` |
| **机制卡 / 属性卡 / 技能卡** | 波次间隙三选一的三类手牌 | `game/heatmap.ts` |

### 4.2 战斗生命周期

| 术语 | 定义 | 代码位置 |
|---|---|---|
| **battle_token** | 开局一次性凭证：绑定 user + level + seed + 过期时刻，结算时置 `used_at` | `battle_tokens` 表、`service.StartBattle` |
| **seed** | 服务端生成的 63 位随机种子，决定整局随机序列。**JSON 中一律字符串下发** | `StartBattle`、`BattleTokenResp.Seed` |
| **build / build_snapshot** | 结算那一刻冻结的玩家构筑（技能、槽位、装备、宝石、专精、attacker） | `battle_records.build_snapshot` |
| **replay_hash** | 整局回放哈希：`fnv1a64(前缀;事件…)` 的 16 位十六进制 | `game/engine.ts` 的 `replayHash()` |
| **S 段（replay_skills）** | 回放前缀里**可被服务端独立重算**的那一段：`slot:skillId:baseDamage:applyStacks:heatCost` | `domain/replayskills.go` |
| **A 段** | 回放前缀里的攻方系数段，含局内 buff，**服务端算不出来** | `replayHash()` 的 `|A…` |
| **X 段** | 回放前缀里的复活额外血量 `|X<n>` | 同上 |
| **card_picks** | 每波选牌决策序列，`-1` 表示跳过；负数编码「弃牌掩码 + 取第几张」 | `engine.ts` 的 `encodeDiscardCode` |
| **结算重算（recompute）** | 服务端用 Go 引擎复算并比对上报值，受 `SETTLE_RECOMPUTE_MODE` 闸门控制 | `service/recompute_*.go` |
| **熔断（breaker）** | 不符率超阈值时把 `enforce` 临时降级为 `shadow`，避免批量误伤 | `service/recompute_gate.go` |
| **验真（verify）** | 任何人拿 battle_id 取回种子/关卡/构筑，本地重放后比对哈希 | `service/verification.go` |

### 4.3 经济与社交

| 术语 | 定义 |
|---|---|
| **coin / gem / energy / keys** | 四种货币，各只有一个主要用途（见 `00001_core.sql` 注释） |
| **token（道具）** | `revive_token` / `mastery_reset` / `gem_wash_token`，走 `user_tokens` 表，与货币列分离 |
| **wallet_flows** | 每一笔货币/道具变动的流水，含变动后余额 |
| **效率榜（efficiency）** | 按「通关时使用的最低战力」升序排名（L-2） |
| **防线（defense）** | 玩家固化的构筑快照，他人本地模拟挑战；服务端只存快照与结算 |
| **偷取（steal）** | 防线挑战胜利后按固定 10% 比例夺取资源，受每日上限与掉落封顶约束 |
| **护盾（shield）** | 24h 内不可被挑战 |
| **诊断（diagnose）** | 连续失败时给出针对性建议（无元素覆盖 / 抗性不匹配 / 专精未投入…） |

### 4.4 工程术语

| 术语 | 定义 |
|---|---|
| **契约夹具（vectors）** | `server/testdata/*.json`，由 `cmd/vectors` 从 Go 真相源生成，供 Go 与 TS 两侧测试共读 |
| **影子模式（shadow）** | 只记录不符、不拒发奖励 |
| **强制模式（enforce）** | 不符即拒发（422） |
| **写放大路径** | 每次调用都落库一行的端点（settle / verify / redeem），单独按身份限流 |
| **fail-closed** | 未知/异常取值一律朝拒绝的一侧倒（如 `requireWritable` 的角色白名单） |

---

## 5. 关键数字速查

排查与评审时最常被问到的数字，全部可在代码或测试中找到出处：

| 数字 | 值 | 出处 |
|---|---|---|
| 玩家 access token TTL | 2h | `ACCESS_TTL` |
| 玩家 refresh token TTL | 30d | `REFRESH_TTL` |
| 管理员会话 TTL | 2h（与 access 同） | `AdminLogin` 写 `admin_tokens.expires_at` |
| 战斗凭证 TTL | 30min | `BATTLE_TOKEN_TTL` |
| 最短可信战斗时长 | 15s | `MIN_BATTLE_DURATION` |
| 战斗逻辑步长 | 50ms（20Hz） | `domain.TickMs` / `TICK_MS` |
| 单局最大 tick | 30000（25 分钟） | `MAX_BATTLE_TICKS` |
| 体力恢复 | 6min / 点，上限 120 | `service.LoadWallet` |
| 每日防线挑战 | 3 次 | `DefenseAttemptLimit` |
| 每日被偷 | 2 次 | `DefenseStolenLimit` |
| 偷取比例 | 10% | `ChallengeDefense` |
| 专精初始点 | 3，每 10 级 +1 | `00003_progression.sql` / `applyProgress` |
| 每层专精上限 | 2（4 选 2） | `PerLayerPickLimit` |
| 配置包体积 | 167,124 B 未压缩 / 23,964 B gzip | `routes.go` 注释实测 |
| 后台首屏体积预算 | 220 KB（实测 161 KB） | `admin/tests/bundle_size.test.ts` |
| 反应攻击力权重上限 | ≤ 300‰（30%） | `MAX_REACTION_ATTACK_WEIGHT` |

---

## 6. 阅读路径建议

**第一次接触项目（半天）**

1. 本文 §2 全景 + §4 术语；
2. `GAME_DESIGN.md` §1 核心循环、§2 创新点（只读 I-1/I-5/I-6 三节）；
3. [01-architecture.md](./01-architecture.md) §4 端间交互矩阵；
4. [03-miniapp.md](./03-miniapp.md) §4 战斗闭环 —— 这是全系统最复杂的一条链路；
5. 跑一遍 `scripts/e2e.ps1`，对照 [05-api-reference.md](./05-api-reference.md)。

**只改后端**

[02-server.md](./02-server.md) §3 分层规则 → §5 核心流程 → [06-data-model.md](./06-data-model.md)
→ [08-contracts.md](./08-contracts.md)（改公式/字段前必读）。

**只改客户端**

[03-miniapp.md](./03-miniapp.md) 全文 → `ENGINEERING.md` 第 2/3/4/6 条
→ [08-contracts.md](./08-contracts.md) §3–§5。

**值班 / 排障**

[07-deployment.md](./07-deployment.md) §7 故障速查 → `OPERATIONS.md` §0 一分钟速查
→ [05-api-reference.md](./05-api-reference.md) §6 错误码表。

**写测试**

`ENGINEERING.md` 第 5/8/9/10/17 条 → [02-server.md](./02-server.md) §9 测试体系
→ [08-contracts.md](./08-contracts.md) §8 契约夹具。

---

## 7. 文档维护规则

这些文档是**实现层的对照表**，因此：

1. **改接口必须同时改 05**。新增/删除端点、改字段名、改错误码，都算。
2. **改表结构必须同时改 06**，并在 §9 迁移清单追加一行。
3. **改环境变量必须同时改 07 §2**，并与 `config.go` 的 `env(...)` 调用逐项核对。
4. **改跨端字段/公式必须同时改 08**，并重生成 `server/testdata/*.json`
   （`go run ./cmd/vectors`），否则 `cmd/vectors -check` 会红。
5. **不要在这里复制玩法数值**。数值的真相源是 `domain/content.go`，
   文档只写「去哪里看」，不写「现在是多少」—— 抄一份必然会漂。
6. 已知边界**只写在 `KNOWN_LIMITS.md`**，本文档提到时给编号引用，不另立清单。

最后一条判据：**如果一段描述在代码改掉之后仍然读起来是对的，它就不该写在这里。**
文档的价值在于「代码变了，它会变红」，而不是「读起来很顺」。
