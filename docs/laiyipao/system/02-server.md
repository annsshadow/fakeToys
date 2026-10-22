# 02 · 服务端（server）

> Go 1.26.10 + Fiber v2 + pgx v5 + PostgreSQL 15+。
> 本文按「目录 → 分层规则 → 各包职责 → 核心流程 → 事务约定 → 测试体系」展开。
> 接口形状见 [05](./05-api-reference.md)，表结构见 [06](./06-data-model.md)。

---

## 1. 运行入口与启动序列

五个可执行入口，都在 `server/cmd/` 下：

| 命令 | 作用 | 典型用法 |
|---|---|---|
| `cmd/api` | 常驻服务 | `go run ./cmd/api` |
| `cmd/migrate` | 独立迁移，支持 `-down` | `go run ./cmd/migrate` / `-down` |
| `cmd/seed` | 写入游戏内容（幂等 upsert） | 首次部署必须执行 |
| `cmd/vectors` | 从 Go 真相源导出跨端契约夹具 | `-check` 供 CI 校验 |
| `cmd/tygen` | 从 Go 响应结构生成后台 TS 类型（`admin/src/api/types.gen.ts`） | `-check` 供 CI 校验漂移 |

`cmd/api` 的启动顺序是**硬性**的，任一步失败直接退出并打印原因：

```
1. config.Load()                 读环境变量；APP_ENV=prod 时缺关键项直接 panic
2. 打印 Redacted() 配置 + DevDefaultWarnings()
3. store.Open()                  建连接池 + 探测；失败即退出
4. db.Migrate()                  仅当 MIGRATIONS_ON_BOOT=true
5. service.New()                 装配 Service（含重算闸门）
6. EnsureBootstrapAdmin()        admin_users 为空时创建初始管理员
7. EnsureContentSeeded()         全新库自动补种子；内容偏少时只告警不覆盖
8. buildApp()                    构造 Fiber + 挂路由
9. Listen(ADDR)                  收到 SIGINT/SIGTERM 后 10s 优雅关闭
```

设计取舍：**不做「先起来再重试」**。容器编排与本地开发都需要立刻知道失败在哪一步，
而不是在健康检查里反复超时。

第 6 步的密码门槛是 `len >= 10`，且 `APP_ENV=prod` 时必须显式设置
`BOOTSTRAP_ADMIN_PASS`（默认值 `admin12345` 恰好 10 位，能通过长度门槛，
所以必须用「环境变量是否被设置」来判定，而不是「够不够长」）。

第 7 步的语义需要区分清楚：

- 全新库（内容表为空）→ 自动灌种子，保证「拉下来就能跑」；
- 已有库但内容比当前版本少 → **只打警示，不覆盖**，避免冲掉运营自定义行；
- 运营改过的行 → 由 `cmd/seed` 显式重灌（它是 upsert，会覆盖内容字段，
  但刻意不碰 `enabled` 之类的运营开关）。

---

## 2. 目录结构

```
server/
├─ cmd/
│  ├─ api/          HTTP 服务入口
│  ├─ migrate/      迁移命令
│  ├─ seed/         内容灌入
│  ├─ tygen/        后台 TS 类型生成器
│  └─ vectors/      契约夹具生成器
├─ internal/
│  ├─ apitypes/     后台响应结构注册表（tygen 的输入）
│  ├─ auth/         JWT 签发校验、refresh/guest token 生成与摘要
│  ├─ config/       环境变量 → Config，prod 守卫，开发默认告警
│  ├─ domain/       纯逻辑：数值、元素、关卡生成、结算校验、引擎镜像
│  ├─ httpapi/      路由、中间件、handler、错误映射
│  ├─ seeder/       内容表写入（敌人/技能/装备/宝石/皮肤/专精/关卡/签到/任务/商城/兑换码）
│  ├─ service/      业务编排：事务边界、跨表一致性、权限
│  ├─ store/        pgx 连接池、goose 迁移、时区钉死
│  ├─ tygen/        TS 类型生成器实现（读 apitypes 注册表）
│  └─ testutil/     测试辅助（scratchdb：真实 PG 临时库）
├─ migrations/      18 个 goose 迁移（00001…00018）+ embed.go
├─ testdata/        跨端契约夹具（由 cmd/vectors 生成）
└─ test/            端到端辅助
```

非测试代码约 25.3k 行；`domain` 与 `service` 占其中约 2/3。

---

## 3. 分层规则

依赖方向是**单向**的，任何反向引用都视为缺陷：

```
cmd/*  ──▶  httpapi  ──▶  service  ──▶  domain
                │            │
                └────────────┴──▶  store（连接池 / 事务）
                └───────────────▶  auth / config
```

| 层 | 允许做 | 禁止做 |
|---|---|---|
| `domain` | 纯函数、常量、数据结构、错误哨兵 | 查库、读环境变量、打日志、依赖 `time.Now()` 以外的外部状态 |
| `service` | 事务、跨表编排、权限校验、把 domain 结果落库 | 写 HTTP 状态码、解析请求体 |
| `httpapi` | 解析参数、调 service、映射错误、写响应 | 直接写 SQL、包含业务规则 |
| `store` | 连接池、迁移、事务包装 | 业务查询（业务 SQL 在 service 里） |

`domain` 的纯度不是洁癖，它有三个可观测的好处：

1. 100 关生成、结算校验、引擎镜像都能在**无数据库**环境下单测；
2. 同一份逻辑可以被 `cmd/vectors` 导出成跨端夹具；
3. Go 侧镜像与 TS 引擎能逐位对拍（`engine_pure_vectors.json`）。

---

## 4. 各包职责

### 4.1 `config`

- 全部配置来自环境变量，敏感项（DB 口令、JWT 密钥、微信 AppSecret）**不入库**。
- `envInt32` 带范围裁剪：越界值（如 `DB_MAX_CONNS=99999999999`）回退默认，
  而不是静默截断成负数。
- `envList` 逐项 `TrimSpace` 并丢空项：`"https://a.com, https://b.com"` 的第二项
  带前导空格会永远匹配不上 CORS 白名单，而这种失败极难归因。
- `prod` 守卫三条：`JWT_SECRET` 不能是 `dev-only…`、`DATABASE_URL` 必须显式声明
  `sslmode`、`BOOTSTRAP_ADMIN_PASS` 必须显式设置。
- `DevDefaultWarnings()` 与 prod 守卫**同源判据**（同一个 `HasPrefix` 与同一个
  `os.Getenv(...)==""`），避免两套判据分叉后出现「prod 会 panic 但这里不告警」。

完整变量表见 [07 §2](./07-deployment.md#2-环境变量全表)。

### 4.2 `auth`

| 能力 | 实现 |
|---|---|
| access token | 自实现 HS256 JWT（不引第三方库），载荷 `{sub, knd, sid, iat, exp}` |
| refresh token | 24 字节随机串，只存 SHA-256 摘要；刷新时**轮换**（条件 UPDATE + RETURNING） |
| guest token | 24 字节随机串，明文存 `users.guest_token`（它本身就是凭证） |
| 管理员会话 | 明文 `sid` 进 JWT，摘要落 `admin_tokens`；每次请求查行 |

`Issue(userID, kind, sid)` 的三个参数是**刻意的破坏性变更**：不显式写
「这个会话没有 sid」，下一个签发点就会静默拿到无会话令牌。

### 4.3 `store`

- `Open` 建 `pgxpool`，`MaxConns/MinConns/MaxConnLifetime` 来自配置。
- **会话时区钉死在连接上**（`RuntimeParams["TimeZone"]`，默认 `Asia/Shanghai`）。
  理由：`sign_date DATE`、`::date` 折算、「今天几号」这些业务概念不能由
  数据库服务器配置决定。用 `RuntimeParams` 而不是 `AfterConnect + SET`，
  是为了不存在「漏掉某条连接」的可能。
- `Tx(ctx, fn)` 是统一事务包装：`Begin → fn → Commit`，`fn` 返回错误即回滚。
- `Migrate` 用 goose，迁移文件通过 `embed.go` 嵌入二进制。

### 4.4 `domain`

按职责分四组：

**（a）内容与生成**

| 文件 | 职责 |
|---|---|
| `content.go` | 敌人 / 技能 / 合成 / 装备 / 宝石 / 皮肤 / 品质的种子表；`ScaleEnemy/Skill` 应用平衡缩放（血量 ×8、弹速 ×4） |
| `levelgen.go` | 6 章参数 + LCG 生成 100 关（波次、地形、星级门槛、理论满分）；`ScoreRules` 唯一定义处 |
| `gems.go` | 宝石品质、词条池、洗练 roll |
| `playerlevel.go` | 等级与专精点曲线 |

**（b）战斗数值**

| 文件 | 职责 |
|---|---|
| `damage.go` | 伤害公式：护甲、抗性、元素层数、暴击 |
| `elements.go` | 5 元素、7 反应链、`reactionTable`（不可交换）、`ReactionSpec` |
| `heat.go` | 热量、过热、弃牌返还 |
| `terrain.go` | 5 类地形的效果与触发 |
| `power.go` | 专精树生成（8×12）、构筑评分五维、战力 |
| `skillrules.go` | 技能等级上限 / 每级系数 / 升级费用 |
| `revive.go` | 复活币的额外血量语义 |
| `defense.go` | 防线工事折算、挑战掉落 |

**（c）战斗校验**

| 文件 | 职责 |
|---|---|
| `battle.go` | `ValidateSettle`：token 可用性、击杀/反应/发射数上界、命中率、时长、速率、HPLeft、掉落封顶、星级重算 |
| `battle_collections.go` | 上报集合字段的规模与元素校验（防 JSONB 膨胀） |
| `replayskills.go` | S 段格式与重算（与 TS 逐字一致） |
| `replay.go` | 重放数据结构、事件编码 |

**（d）Go 引擎镜像（R1）**

| 文件 | 职责 |
|---|---|
| `engine.go` | 引擎主体（tick 推进、阶段机） |
| `engine_combat.go` | 命中、伤害、反应、死亡 |
| `engine_card.go` | 三选一卡牌与 buff |
| `engine_enemy.go` / `engine_wave.go` / `engine_terrain.go` | 敌人行为、波次、地形 |
| `engine_tuning.go` | 与 TS `tuning.ts` 对齐的节奏参数 |

镜像引擎的唯一用途是**结算重算**，它不参与正常结算的奖励计算。

### 4.5 `service`

| 文件 | 职责 | 关键点 |
|---|---|---|
| `service.go` | `Service` 装配、账号、钱包、`/config` | 静态配置进程内只组装一次 |
| `game.go` | 开局、结算、build 快照、任务进度、S 段重算 | 结算主流程 |
| `progression.go` | 专精分配/重铸、诊断、防线列表/保存/挑战 | 每日计数、锁顺序 |
| `economy.go` | 排行榜、签到、兑换、商城、回放读取 | 「今天」由 Go 决定 |
| `loadout.go` | 槽位保存、技能升级、等级读取 | 槽位预算校验 |
| `loadout_attacker.go` / `attacker.go` / `build_parts.go` | 攻方属性推导（装备/宝石/专精/皮肤） | 与 TS `damage.ts` 对齐 |
| `synthesis.go` | 技能熔炼 | 材料消耗 + 配方匹配 |
| `gems.go` | 宝石铸造、列出、洗练 | 词条 roll 与品质缩放 |
| `equipment.go` | 装备目录 + 拥有/装配状态、整组替换装配 | P4-1：装备从「只有数值」变成可获取可换 |
| `battleabandon.go` | 放弃战斗并退回体力（复活币） | P4-5：中途退出不再白扣体力 |
| `player_account.go` | 玩家账号登录（用户名 + 口令） | 与游客登录共用 `issueTokens` |
| `adminuserdetail.go` / `adminbattledetail.go` | 后台玩家详情 / 战报详情 | P5-7 / P2-1 |
| `verification.go` | 玩家/运营验真 | 两条路径共用 `compareAndRecord`；00017 起同人同哈希幂等 |
| `admin.go` / `adminutil.go` | 管理员登录、会话、审计、引导 | bcrypt + sid 会话 |
| `adminapi.go` | 后台内容管理（关卡/技能/装备/经济/公告/兑换码） | 白名单字段更新 |
| `stats.go` | 看板、用户列表、封禁、发币、战报列表 | 聚合 SQL |
| `recompute_engine.go` / `recompute_gate.go` / `recompute_mode_setter.go` / `recompute_engine_report.go` | 结算重算、熔断、台账、覆盖率读数 | R1 |

### 4.6 `httpapi`

- `routes.go`：全部路由注册 + 玩家侧 handler。
- `handlers_admin.go`：管理端 handler（含审计写入）。
- `middleware.go`：panic 兜底、安全头、CORS、限流、鉴权、错误映射、`queryInt`。

路由组织约定（写在 `routes.go` 顶部）：

1. 一律用**显式前缀**的 Group，不用 `Group("")`；
2. 每个需要认证的路径前缀挂一次 `requireUser`，一一对应，不做嵌套；
3. 读端点对 `readonly` 放行，写端点逐个挂 `requireWritable()`。

### 4.7 `seeder`

把 `domain` 的内容写库，全部 upsert、可重复执行、不碰玩家数据。
写入顺序有依赖：敌人 → 技能 → 装备/宝石 → 皮肤 → 专精 → 关卡/波次 → 签到 → 任务 → 商城 → 兑换码。

两个容易忽略的细节：

- 敌人落库的是**缩放后**的血量（×8），与客户端 `/config` 拿到的数值一致；
  否则后台看到的血量是实际游戏的 1/8。
- `redeem_codes.id` 是 `GENERATED BY DEFAULT AS IDENTITY`，种子显式插入固定 id
  后必须 `setval` 对齐序列，否则管理端创建第一条兑换码会撞主键。

---

## 5. 核心流程

### 5.1 游客登录 / 令牌签发

```
GuestLogin(guest_token, nickname)
  ├ 空 guest_token → 生成新的 24 字节随机串
  ├ 昵称空 → "玩家" + 4 位数字；长度 > 64 → 400（不静默截断）
  ├ INSERT users … ON CONFLICT (guest_token) DO UPDATE SET last_login_at
  ├ initNewUser：钱包(2000/50/30/1) + 进度 + 离线 + 2 件装备 + 3 个技能 + 3 个槽位
  ├ loadUser → status != 1 → 403
  └ issueTokens：签发 JWT + 写 refresh_tokens
```

`initNewUser` 全部用 `ON CONFLICT DO NOTHING`，因此重复调用不会重置进度。

新号初始配置是**玩法可用性**问题，不是数值问题：三个技能分属焰/冰/电，
三个槽位与之对应，保证新号一进关就能触发反应链。若只发技能不给槽位，
新玩家进游戏就是死局（引擎没有任何可用技能）。

`WechatLogin` 与 `Refresh` 共用 `issueTokens`，而封禁检查下沉在这个唯一出口 ——
否则「封禁一个微信用户 → 他重新登录拿新令牌」就能绕过封禁。

### 5.2 开局（`POST /battle/token`）

```
StartBattle(userID, levelID)
  ├ levelID ∈ [1,100]，否则 400
  ├ gl = GenerateLevel(levelID)
  ├ levelID > max_stage + 1 → 403（关卡门禁）
  ├ seed = SELECT abs(FLOOR(random()*2^63-1))
  └ Tx:
       ├ INSERT battle_tokens(user, level, seed, expires_at) RETURNING id
       ├ grantWallet(energy: -gl.EnergyCost, reason="battle_start")
       └ 读 revive_token 余额；>=1 → spendToken（静默跳过表示「没买复活」）
  └ 返回 token_id / seed(str) / expires_at / level(含 waves) / revived / extra_base_hp / build
```

三个关键决策：

- **种子由服务端生成**。客户端自选种子等于可以刷出想要的随机序列。
- **复活币在开战扣，不在结算扣**。「有没有用到」只有客户端知道，
  放到结算就成了伪造入口；代价是「进战斗页又退出也扣」，与体力模型一致。
- **复活给的是额外当前血量，不是 `base_hp` 倍率**。漏怪伤害以 `baseHpMax`
  为分母，抬高 `base_hp` 会同步抬高漏怪伤害，加成被整个抵掉（实测 100 关 0 关翻盘）。

### 5.3 结算（`POST /battle/settle`）—— 全系统最长的单条流程

```
SettleBattle(userID, tokenID, in)
 │
 ├ ① 读 battle_tokens（限定 user_id），不存在 → 404
 ├ ② gl = GenerateLevel(tok.LevelID)
 ├ ③ ValidateSettle(tok, gl, in, limits, now)   ← domain 纯函数
 │     ├ token 已用/过期/关卡不符
 │     ├ kills ≤ 总怪数、leaked ≤ 总怪数
 │     ├ reactions ≤ 理论上限、shots ≤ 时长物理上限
 │     ├ hits/shots 命中率 ≤ 100%
 │     ├ duration ∈ [MinBattleDuration, 24h]
 │     ├ score ≤ scoreCapFor(关卡, 时长)  ← 速率锚点按关卡推导
 │     ├ hp_left ≤ 关卡初始血量
 │     └ 星级由分数重算（不信任上报）
 │
 ├ ④ replay_skills 比对（若非空）
 │     expect = replaySkillsSegment(userID)   ← 用 user_skill_slots + user_skills.level 重算
 │     不符 → Gate.Decide(ScopeSkills)
 │              ├ enforce → 台账(battle_id=0) → 422
 │              └ 其他   → 台账 + stderr 日志 → 继续
 │
 ├ ⑤ 归一化集合字段（nil → {} / []），避免 jsonb null
 ├ ⑥ 组装 build_snapshot = 当前构筑 + settle_input 原文
 ├ ⑦ recomputeEngine(...)  ← 整局重算，见 5.4
 ├ ⑧ Tx:
 │     ├ UPDATE battle_tokens SET used_at WHERE id AND user_id AND used_at IS NULL
 │     │     RowsAffected=0 → 403（已被结算）
 │     ├ INSERT battle_records（含 card_picks、build_snapshot、replay_hash、loot）
 │     ├ grantWallet(loot, reason="battle_reward", ref=battleID)
 │     ├ applyProgress（max_stage / level_exp / total_battles / total_kills）
 │     ├ upsert level_stars（GREATEST 星级 / best_score / clears / LEAST min_power_clear）
 │     └ bumpTasks（kills / clears / reactions / max_stage）
 └ ⑨ 读回钱包、build、评分、战力，组装响应
```

为什么幂等靠 `UPDATE … WHERE used_at IS NULL` 而不是「先查后写」：
后者在并发下是经典的重复领奖漏洞。条件更新的 `RowsAffected` 是原子判据。

为什么 `build_snapshot` 存的是**当时的玩家构筑**而不是上报原文：
回放哈希由「关卡 + 种子 + 构筑」共同决定，只存上报数据的话，
验真方拿不到技能与养成属性，算出的哈希必然对不上 ——
验真会把所有正常对局判成伪造。

### 5.4 结算重算（R1）

```
recomputeEngine(userID, tok, gl, buildRaw, in)
  ├ 引擎不可用（缺数据/未移植分支） → cannotRun++ → 返回 nil（不阻断）
  ├ 重算结果与上报一致            → verified++
  └ 不符 → mismatch++
        ├ Gate.Decide(ScopeEngine)
        ├ RecordRecompute（台账：上报值 + 重算值 + mismatched_fields + mode + rejected）
        └ enforce → 返回 422；其余 → 继续发奖
```

台账 `settle_recomputes` 的设计要点（迁移 00015 的注释写得很完整）：

- **上报值与重算值并列存放**，`mismatched_fields` 是可聚合维度而不是一个布尔。
  理由：「分数不符」无法区分「玩家作弊」与「Go 引擎算错」，而处置方式完全相反
  （封号 vs 回滚引擎）。区分办法是按关卡/按字段聚合。
- `recomputed_*` 允许 NULL：引擎未接入时 shadow 模式记的就是「还没算」。
- `mode` 与 `rejected` 分开存：熔断可以让 `enforce` 下的单次处置变成「不拒发」，
  而「这一局到底有没有被拒」是运营问的第一个问题。
- 刻意不建「只索引不符行」的部分索引：熔断要算的是「不符数 / 总数」，
  只有不符行就没有分母了。

进程内计数器（`verified/mismatch/cannotRun/skipped`）是**独立读数**：
台账只记不符，所以「校验 100% 通过」与「校验根本没跑」在台账上都是 0 行。
`/admin/recomputes/stats` 的 `engine` 段是唯一能区分两者的地方。

### 5.5 验真

```
VerifyReplay / AdminVerifyReplay
  └ compareAndRecord(verifier, battleID, actualHash)
       ├ actualHash 长度 ≤ 64，否则 400
       ├ 查 battle_records（玩家侧额外 AND user_id = 自己）
       │    查不到 → 404（「不存在」与「不是你的」返回同一错误，避免探测预言机）
       ├ 比对 expected vs actual
       └ 写 replay_verifications（verifier_id 与 admin_verifier_id 恰有一个非空）
```

**它不重算哈希**。服务端没有引擎参与这条路径（R1 的重算走另一条）。
它防的是「改了数据却没改凭证」，不防「从一开始就伪造」。
迁移 00017 给 `replay_verifications` 加了唯一索引
`(battle_id, COALESCE(verifier_id, admin_verifier_id), actual_hash)`：
同一场战报 + 同一个验真者 + 同一个哈希只留一条（建索引前先删重复行，保留最早）。
运营侧验真额外写 `admin_audit_logs`（管理动作的审计轨迹独立于统计表）。

### 5.6 经济

**体力恢复**（`LoadWallet`）：

```
k = FLOOR((now - energy_updated_at) / 6min)
energy = LEAST(120, energy + k)
energy_updated_at += k * 6min        ← 关键：按整格推进，不是推到 now()
```

按整格推进是为了**不吞余数**。旧写法 `energy_updated_at = now()` 会丢掉
不足一格的时间：每 5 分钟读一次、6 分钟回一点，一小时后只回 6 点而不是 10 点
（少回 40%），而读取间隔恰好短于回复间隔正是最常见的形态。

**签到**：靠 `user_sign_ins (user_id, sign_date)` 主键保证一天一次，
不做「先查后写」。七天签完后返回 `already=true`。

**任务**：

- `scope` 白名单 `daily/weekly/achievement`，非法值 400（不能静默返回空列表 ——
  那会让玩家看到「今天没有任务」这个看起来完全合理的错误答案）。
- 周期起点由 `periodStart(now, scope)` 决定：daily 今天、weekly 本周一、
  achievement 固定 epoch（终身累计，迁移 00013）。
- `bumpTasks` 对 achievement 用 `GREATEST(progress, value)`，所以 `max_stage`
  类任务的 progress 语义是「历史最高关号」。

**商城**：`price` / `payload` 都是 JSONB。`payload` 的 key 必须是货币名
（`coin/gem/energy/keys`）或道具名（`revive_token/mastery_reset/gem_wash_token`），
否则 `grantWallet` 报「未知货币」。这个显式白名单是刻意的摩擦：
打错字会报错，而不是静默发放一个没人看得见的道具。

**兑换码**：`redeem_codes.used_count <= max_uses` 有 CHECK；
`redeem_usages (code_id, user_id)` 唯一，保证一人一次。
并发下用条件 UPDATE 抢占名额，而不是「先查再写」。

### 5.7 专精

```
AllocateMastery(userID, nodeID)
  ├ 节点存在；点数 > 0
  ├ 同层已选 < 2（PerLayerPickLimit）
  ├ 前置层已点亮（layer-1 至少 1 个）
  └ Tx: 扣 1 点 + INSERT user_mastery_nodes
```

`ResetMastery` 消耗 1 件 `mastery_reset`，清空全部节点，**点数预算不变**。
它存在的理由：`AllocateMastery` 只插不删，点错方向后没有任何改向途径 ——
没有它，`mastery_reset` 就只是商店里一件「买了不知道干嘛」的商品。

`GET /mastery` 必须同时下发 `base_slots` 与 `total_slots`：
专精第 3 层有 8 个「额外插槽」节点，若客户端用编译期常量 `ACTIVE_SLOTS=4`，
玩家花点数点出来的槽位在战斗里不存在。

### 5.8 防线（I-5）

```
SaveDefense
  ├ 名称 ≤ 32 字；works ⊆ {slow_belt, block_wall, tesla_grid}
  ├ validateDefenseOwnership：技能/装备/专精必须真实属于该用户
  ├ 计算 power / element_coverage / mastery_done
  ├ snapshot = {skills, equipment, gems, mastery, works}
  └ snapshot_hash = SnapshotHash(snapshot)   ← FNV-1a 64，16 位 hex
  └ upsert defenses（一人一条，UNIQUE(owner_id)）

ChallengeDefense
  ├ ValidateChallengeInput（事务外）：seed 字符串、duration ∈ (0,24h]、
  │   hp_left_pct ∈ [0,100]、replay_hash ≤ 64、replay_skills ≤ 1024
  ├ Tx:
  │   ├ 按固定顺序锁双方 daily 计数（避免死锁）
  │   ├ 挑战者 attempts < 3；防守方 stolen < 2；护盾未生效
  │   ├ 胜 → 按 10% 偷取（受掉落封顶）
  │   ├ 写 defense_challenges（settled 幂等）
  │   └ 更新双方计数与 wins/losses
  └ 返回 stolen / owner / 双方计数
```

**锁顺序**必须固定（`lockChallengeStateOrdered` 对 user_id 排序后逐个锁），
否则两个玩家互打对方防线时会形成经典 ABBA 死锁。

`GET /defenses` 是**只读**的：它曾经是一条 `INSERT … ON CONFLICT DO UPDATE`，
于是每次浏览都落一行、并与挑战路径抢行锁。现在行不存在即视为 0。

### 5.9 管理端

**登录**：bcrypt（cost 10）→ 生成 sid → 签发 JWT → 写 `admin_tokens`。
用户不存在与密码错误返回**同一错误**（避免用户名枚举）。

**看板**（`AdminDashboard`）聚合：用户数（总/游客/微信/封禁/今日新增）、
战斗（总/今日/胜/平均时长）、进度（平均最高关/平均战力/100 关通关人数）、
经济（四币进出、订单状态）、验真健康度、反应使用分布、DAU、关卡漏斗。

**改关卡 / 技能**：字段白名单更新，写 `admin_audit_logs`。
返回体带 `player_authority` 与 `applied_to_players` ——
`generator` 路线下 `applied_to_players` 恒为 false，明确告诉运营「这次编辑不影响线上」。

**封禁**：`users.status=2` + `ban_reason`，同一事务吊销该用户全部 refresh token。
封禁即时生效，因为 `ResolveUser` 每次请求都查 `status`。

**发币**：`AdminGrantCurrency` 支持负数（回收）。审计事件名区分
`grant_currency` / `revoke_currency` —— 否则审计里「发放」和「回收」长得一模一样。

**重算审计**：`GET /admin/recomputes`（列表）与 `/recomputes/stats`（总账 +
按关卡分布 + 闸门状态）。`/stats` 必须注册在 `/recomputes` 之后且不能写成
`/recomputes/:id`，否则会被当成 id 匹配掉。

### 5.10 配置下发与版本

```
staticConfig = sync.OnceValue(func() GameConfig {
    Levels:     SummarizeAll(GenerateAllLevels())   ← 摘要，无 waves
    Enemies:    ScaleAllEnemies()                   ← 缩放后
    Skills:     ScaleAllSkills()
    Composite:  ScaleAllCompositeSkills()
    …
    Version:    sha256(json)[:8] → 16 位 hex
})
LoadGameConfig() = staticConfig() + ServerTime=now()
```

- **`Version` 是内容哈希**，只在内容真的变了才变。它曾经是
  `time.Now().Unix() % 100000`（每秒都变），于是「版本没变就不用刷新」
  这个判断从来没被实现过。
- **静态部分进程内只组装一次**。原来每个请求都重跑 `GenerateAllLevels()`，
  而 `/config` 是最热的端点。
- **`levels` 只下发摘要**。`waves` 占整个配置包的 56%，而客户端只有
  「防线挑战的标准战场」一处需要它，那一处走 `GET /levels/:id`。
- 条件请求用查询参数 `?if_version=` 而不是 ETag：自定义请求头会触发 CORS 预检，
  而 ETag 会让浏览器自作主张缓存响应体，里面的 `server_time` 会被当成
  「服务端当前时间」。

---

## 6. 事务与并发约定

| 约定 | 原因 | 例子 |
|---|---|---|
| **先校验、后开事务** | 请求校验不是业务规则；PG 的 22003 在事务中炸出来就是真 500 | `ValidateChallengeInput`、`ValidateSettle` |
| **抢占用条件 UPDATE + RowsAffected** | 「先查后写」在并发下必然重复 | `battle_tokens.used_at`、兑换码名额 |
| **扣减用 `WHERE balance >= need`** | 靠 CHECK 兜底会整批失败且无法给可读错误 | `grantWallet`、`grantToken`、`spendToken` |
| **锁顺序固定** | 双人互打防线会 ABBA 死锁 | `lockChallengeStateOrdered` |
| **幂等键优先于去重查询** | 主键/唯一索引是数据库层的原子保证 | `user_sign_ins`、`redeem_usages`、`defense_challenges`、`replay_verifications` |
| **同一语义只写一份实现** | 两份实现必然漂移，且症状指向错误方向 | 玩家/运营验真共用 `compareAndRecord` |
| **归属写进语句本身** | 「调用点保证」在新增调用点时会静默失效 | `UPDATE … WHERE id=$1 AND user_id=$2` |

`ownership_scan_test.go` 会扫描所有 UPDATE/DELETE 语句，检查是否带归属条件。
这条守卫的价值在于：归属校验是**安全属性**，不能靠人记得传。

---

## 7. 错误模型

哨兵错误集中在 `service`：

```go
ErrNotFound      // 资源不存在
ErrUnauthorized  // 未认证 / 令牌失效
ErrForbidden     // 无权 / 封禁 / 未解锁
ErrBadInput      // 输入非法
```

`domain` 另有一组**结算拒绝**错误（`ErrTooManyKills`、`ErrTooShort`、
`ErrScoreRateExceeded`、`ErrInvalidHitRate`、`ErrWaveExceeded`、
`ErrTooManyReactions/Shots/Leaked`、`ErrInvalidField`、
`ErrHPLeftExceedsBase`、`ErrToken*`、`ErrLevelMismatch`），
由 `isSettleRejection` 统一映射到 422。

维护规则：

- 新增 domain 结算错误 → 必须同步 `isSettleRejection`；
  `TestEverySentinelErrorIsClassified` 会把差集打出来。
- 500 只返回通用文案，真实原因进日志（`log.Printf`）。
- panic 由 `recoverPanic` 兜底，**panic 值直接落日志** ——
  曾经写进 `Locals` 等 `failErr` 读，但 panic 展开后根本走不到那里。

---

## 8. 安全与限流

| 措施 | 实现 | 说明 |
|---|---|---|
| 安全响应头 | `X-Frame-Options: DENY`、CSP `default-src 'self'; frame-ancestors 'none'`、`nosniff`、`no-referrer` | 后台所有写操作单击即生效，点击劫持是唯一「不需要服务端漏洞」的路径 |
| CORS | dev 未配置 → `*`；prod 未配置 → 不发任何头；配了白名单 → 回显具体 Origin | 默认取安全一侧 |
| IP 限流 | guest/wechat/refresh 60/min；admin login 5/min；redeem 5/min | 按 IP 挡「一台机器能打多猛」 |
| 身份限流 | settle 60/min；verify 20/min；redeem 20/min | 按 user_id 挡「一个账号能打多猛」 |
| 请求体上限 | 1MB | 昵称、replay_hash、集合字段另有更严的业务上界 |
| 归属校验 | 每个读写用户数据的语句都带 `user_id` | 由扫描测试守住 |

限流额度给得比直觉宽松，理由是小程序玩家大量走运营商 NAT，
同一出口 IP 后可能有成百上千真实用户。把每 IP 每分钟卡到个位数会误伤整片网段，
而那种误伤比不限流更难排查。

---

## 9. 测试体系

服务端测试不是「覆盖率高」，而是**分层设防**：

| 类型 | 代表 | 作用 |
|---|---|---|
| 纯逻辑单测 | `damage_test.go`、`levelgen_test.go`、`settle_*_test.go` | domain 无需数据库 |
| 真实库集成 | `service/*_test.go` + `testutil/scratchdb` | 临时 PG 库，跑真实 SQL / 事务 / 并发 |
| 契约测试 | `reaction_contract_test.go`、`replay_skills_contract_test.go`、`card_picks_contract_test.go` | 与 TS 共读 `testdata/*.json` |
| 守卫测试 | `ownership_scan_test.go`、`readonly_guard_test.go`、`unused_param_test.go`、`prod_guard_coverage_test.go` | 结构上禁止某类缺陷 |
| 文档漂移 | `docs_drift_test.go`、`readme_drift_test.go` | 文档与代码不一致就红 |
| 端到端 | `scripts/e2e.ps1`（19 步） | 真实 HTTP + 真实库 |
| 变异测试 | 见 `IMPLEMENTATION.md` | 验证「测试能不能红」 |

守卫测试是这套体系里最有价值的一类。它们的判据不是「行为对不对」，
而是「某种写法是否还存在」：

- `ownership_scan_test.go`：所有 UPDATE/DELETE 必须带归属条件；
- `readonly_guard_test.go`：管理端写端点必须挂 `requireWritable`（AST 扫描）；
- `prod_guard_coverage_test.go`：所有敏感默认值必须被 prod 守卫覆盖（AST 扫描）；
- `unused_param_test.go`：函数参数不得被忽略（抓到过 `reactions` 从未使用）。

---

## 10. 扩展指南

### 加一个玩家端点

1. `service` 写方法，返回哨兵错误；
2. `httpapi/routes.go` 挂到正确的 Group（需要认证就挂 `requireUser` 组下）；
3. 若是写放大路径，加 `authRateLimit`；
4. 更新 [05-api-reference.md](./05-api-reference.md)；
5. 补负向测试（未认证 401 / 越权 403 / 非法输入 400）。

### 加一张表

1. 新建 `migrations/000NN_xxx.sql`，含 `+goose Up` / `+goose Down`；
2. Down 必须能跑通，且注释里写明是否**有损**；
3. 更新 [06-data-model.md](./06-data-model.md) §3 与 §9；
4. 若表含 `user_id`，所有查询必须带归属条件（否则扫描测试红）。

### 加一个内容项（技能/敌人/装备）

1. 改 `domain/content.go`；
2. `go run ./cmd/seed`；
3. `go run ./cmd/vectors` 重生成夹具；
4. 跑 `go test ./...` 与 `pnpm test`（两端契约必须同时绿）；
5. 注意：**改动数值会让存量战报失去可重放性**，流程见 `OPERATIONS.md` §4。

### 加一个管理端写端点

1. handler 里调 service；
2. 路由挂 `requireWritable()`；
3. 写 `admin_audit_logs`（事件名、target、detail）；
4. `readonly_guard_test.go` 会检查你是否漏了第 2 步。
