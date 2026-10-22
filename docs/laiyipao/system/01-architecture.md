# 01 · 系统架构与端间交互

> 本文回答三个问题：**系统由哪些进程组成**、**一次请求经过哪些层**、
> **同一条数据到底谁说了算**。模块内部的细节分别见 02/03/04。

---

## 1. 架构定位与三条设计前提

整个系统的形态由一个取舍决定：**服务端不跑战斗引擎**。

这个取舍不是省事，而是三件事的连锁结果：

1. 战斗必须完全确定性（定点整数 + LCG），否则回放无法复现（I-6）；
2. 若服务端也跑一份引擎，就必须保证 Go 与 TS **逐位一致**，
   而任何一次数值调整都会让存量战报失去可重放性；
3. 因此服务端只做**可判定的校验**：凭证一次性、数值上界、
   交叉约束（击杀 ≤ 总怪数、命中率 ≤ 100%）、S 段逐字比对、
   以及（R1 起）可开关的整局重算。

由此产生三条贯穿全系统的前提，后文所有设计都建立在它们之上：

| 前提 | 含义 | 后果 |
|---|---|---|
| **P1 服务端是权威** | 关卡内容、种子、构筑属性、余额、进度、结算结论都由服务端裁定 | 客户端上报的一切都要能被服务端独立验证或封顶 |
| **P2 客户端是唯一执行体** | 战斗、防线挑战的模拟只在客户端发生 | 客户端必须拿到**完整且冻结**的输入（关卡 + 种子 + 构筑 + 规则） |
| **P3 所有跨端数据必须有唯一真相源** | 内容表在 `domain/content.go`，规则在 `domain/*.go`，契约夹具由生成器产出 | 任何「两边各写一份」的实现都被视为缺陷 |

---

## 2. 部署拓扑

### 2.1 开发环境（本机三进程）

```
┌──────────────┐   :5174   ┌──────────────┐   :5175
│ miniapp H5   │──────────▶│ admin        │
│ (vite dev)   │           │ (vite dev)   │
└──────┬───────┘           └──────┬───────┘
       │ /api/v1 相对路径          │ /api 代理
       │ （devServer 代理）        │ （vite proxy）
       └───────────┬───────────────┘
                   ▼
            ┌──────────────┐        ┌──────────────┐
            │ server :8080 │───────▶│ PostgreSQL   │
            │ go run ./... │        │ :5432        │
            └──────────────┘        └──────────────┘
```

- miniapp 的 `BASE_URL` 来自 `VITE_API_BASE`，缺省 `http://127.0.0.1:8080/api/v1`；
  非回环地址若用明文 HTTP 会**启动即抛**（`api/client.ts` 的 `resolveBaseURL`）。
- admin 的 `API_BASE` 来自 `VITE_API_BASE`，缺省 `/api/v1`（同源 + 代理）。
- 数据库可用 `DATABASE_URL` 指向任意实例；本地 Docker 常见做法是把容器
  `5432` 发布到 `15432`（Windows 上原生端口代理常不通）。

### 2.2 测试 / 生产环境（单机反代）

```
                    ┌────────────────────────────────────────────┐
   玩家 / 运营 ────▶│ nginx（HTTPS 终结）                        │
                    │  /laiyipao-api/    → 127.0.0.1:9020/       │
                    │  /laiyipao-admin/  → 静态 admin/dist       │
                    │  /laiyipao-h5/     → 静态 miniapp h5 产物  │
                    └───────────────┬────────────────────────────┘
                                    │
                    ┌───────────────▼──────────────┐
                    │ systemd: laiyipao.service    │
                    │ /usr/local/laiyipao/bin/…    │
                    │ EnvironmentFile=etc/config.env│
                    └───────────────┬──────────────┘
                                    ▼
                             PostgreSQL `laiyipao`
```

要点（详见 07）：

- **前端与 API 同源**（都在同一域名下不同路径），因此 `prod` 默认
  **不发任何 CORS 头**即可正常工作；需要跨域时才配 `CORS_ALLOW_ORIGINS`。
- nginx 对 `/laiyipao-api/` 做**前缀剥离**（`proxy_pass .../` 末尾带斜杠），
  所以服务端路由仍然是 `/api/v1/...`。
- SPA 必须 `try_files ... /index.html` 兜底；`assets/` 走 1 年 immutable 缓存，
  `index.html` 走 `no-cache`。
- 后台的 `base` 必须是 `/laiyipao-admin/`（`vite.config.ts` 已固定），
  否则资源打到根路径直接白屏。

### 2.3 进程模型与扩展边界

| 组件 | 进程模型 | 可否水平扩展 |
|---|---|---|
| server | 单进程，Fiber 默认多 goroutine | **当前不建议**：限流是**进程内计数**（`perIPLimiter`），
多副本会让实际额度变成 `N × 配置值`；且重算闸门的熔断统计也是进程内 |
| miniapp | 客户端 | — |
| admin | 静态资源 | 可任意副本（无状态） |
| PostgreSQL | 单实例 | 可做主从，但当前代码无读写分离 |

> 若将来要多副本，必须先做两件事：把限流计数器换成 Redis（或网关层限流），
> 把熔断统计改成共享存储。这一点在 `middleware.go` 的注释里已写明。

---

## 3. 一次请求经过哪些层

### 3.1 中间件顺序（顺序本身是契约）

`httpapi.Register` 的挂载顺序：

```
1. recoverPanic      兜底 panic → 500 + 完整栈进日志
2. securityHeaders   X-Frame-Options / CSP / nosniff / Referrer-Policy
3. compress          gzip（必须在任何写响应体的中间件之前）
4. requestLogger     记录 X-Request-Duration
5. cors              按 Env 决定是否发 CORS 头
6. perIPLimiter      （仅未认证端点：guest / wechat / refresh / login / redeem / admin-login）
7. requireUser       解析 Bearer → Locals["user_id"]（每次都查 users.status）
   requireAdmin      解析 Bearer → Locals["admin_id","admin_role"]（查会话 + status）
8. requireWritable   （仅管理端写操作）角色白名单 admin/ops
9. authRateLimit     （仅 settle / verify / redeem）按 user_id 计数
10. handler          业务处理
11. failErr          统一错误映射
```

三条容易踩的规则：

- **`securityHeaders` 与 `compress` 必须在所有会产生响应的中间件之前**，
  否则它们拿不到后续中间件写出的响应体/头。
- **限流挂在 `requireUser` 之后**，键才能用 `user_id`；挂在前面只能按 IP。
- **路由分组必须用显式前缀**。Fiber v2 非严格路由下，空前缀分组会与
  兄弟分组产生前缀歧义 —— 历史上曾导致 `/admin/login` 被误套 `requireUser`。

### 3.2 统一响应形状

成功：直接返回业务 JSON（各端点形状见 05）。

失败：**唯一形状**

```json
{ "error": { "code": "bad_input", "message": "……" } }
```

`failErr` 的映射表：

| 服务层错误 | HTTP | `code` |
|---|---|---|
| `ErrUnauthorized` | 401 | `unauthorized` |
| `ErrForbidden` | 403 | `forbidden` |
| `ErrNotFound` | 404 | `not_found` |
| `ErrBadInput` | 400 | `bad_input` |
| 结算拒绝族（见 05 §6） | 422 | `settle_rejected` |
| 限流命中 | 429 | `rate_limited` |
| 角色不可写 | 403 | `role_not_writable` |
| 其余 | 500 | `internal_error`（真实原因只进日志） |

> 422 与 500 的边界是**刻意维护**的：`isSettleRejection` 里那份清单必须与
> `domain` 的结算错误同步，`TestEverySentinelErrorIsClassified` 会打印差集。
> 漏一个的后果不是「少个提示」，而是「玩家作弊」和「服务端坏了」在客户端
> 长得一模一样，同时把真故障淹进假 500 里。

---

## 4. 鉴权模型

系统里有**三套凭证**，互不通用：

| 凭证 | 载体 | 签发 | 校验 | 失效方式 |
|---|---|---|---|---|
| **玩家 access token** | `Authorization: Bearer` | `auth.Signer` HS256 JWT，`kind="user"` | 验签 + 查 `users.status` | 到期（2h） |
| **玩家 refresh token** | 请求体 | 随机串，SHA-256 摘要落 `refresh_tokens` | 条件 UPDATE（未吊销 + 未过期），**轮换** | 登出吊销全部 / 封禁 |
| **游客凭证 guest_token** | 请求体 + 本地存储 | 24 字节随机串，存 `users.guest_token` | 直接查表 | **登出时轮换** |
| **管理员 access token** | `Authorization: Bearer` | HS256 JWT，`kind="admin"`，载荷含 `sid` | 验签 + 查 `admin_tokens`（摘要、吊销、过期）+ 查 `admin_users.status` | 到期（2h）/ 后台登出吊销全部 |

> 玩家侧还有第四种**入门方式**（不是新令牌类型）：`POST /auth/login`
> 用户名 + 口令（迁移 00016 给 `users` 加 `username` / `password_hash`，
> 部分唯一索引）。它与游客登录签发**同一种** `kind=user` 令牌，权限面完全一样，
> 存在的理由是测试环境的共享账号（`PLAYER_LOGIN_REQUIRED`，见 07 §2.4）。

### 4.1 为什么玩家侧要三个凭证

- access token 短命（2h），泄露窗口有限，但**无法单条吊销**（JWT 固有性质）。
- refresh token 长命（30d），落库可吊销、可轮换 —— 它才是真正需要切断的东西。
- guest_token 永久有效，等价于口令。它此前**永不变更**，意味着「登出」
  对持有旧凭证的人毫无意义。现在登出会在同一事务里轮换它。

> ⚠️ 客户端必须保存登出响应里的新 `guest_token`。不存的话下一次冷启动
> 会带着已作废的旧凭证去 `GuestLogin` → 匹配不到 → **建出新号**，
> 玩家的金币与进度看起来凭空消失。这是全系统最容易写错的一处顺序，
> `miniapp/src/api/client.ts` 的 `logout()` 有完整注释。

### 4.2 为什么管理员要多一个 sid

管理员令牌若只验签，则「离职 / 降权 / 登出」在 2h 内都不生效 ——
而管理端能发币、封号、改价。现在 `AdminLogin` 每次签发都会写一行
`admin_tokens`（只存 sid 的 SHA-256），`ResolveAdmin` **每次都查这一行**。
登出 = 把该管理员全部会话置 `revoked_at`，签名仍合法的 JWT 立刻失效。

### 4.3 角色模型

`admin_users.role` 三值：`admin` / `ops` / `readonly`。

`requireWritable` 是 **fail-closed 白名单**：只有 `admin` / `ops` 可写，
其余（含空串、拼错的值）一律 403。此前是「只拒 readonly，未知放行」——
那意味着把 role 手改成 `readonly `（尾随空格）就会变成可写，
一个数据录入错误直接升级成越权。

读端点对 `readonly` 保持放行；唯一不挂 `requireWritable` 的写操作是
`POST /admin/logout`（会话生命周期不是数据写，`readonly` 更应该能切断自己）。

---

## 5. 数据所有权矩阵

「谁是权威」这件事必须逐字段明确，否则就会出现「客户端显示 30 体力、
开战报体力不足」这类无法归因的问题。

| 数据 | 权威 | 客户端角色 | 同步方式 |
|---|---|---|---|
| 关卡内容（100 关 / 波次 / 地形） | `domain.GenerateLevel`（纯函数） | 只读 | `/config`（摘要）+ `/levels/:id`（含 waves） |
| 敌人 / 技能 / 装备 / 宝石 / 皮肤 / 专精树 / 反应 / 分数规则 | `domain/content.go` + `domain/*` | 只读 | `/config` 一次全量 |
| 配置版本 | 服务端内容哈希（SHA-256 前 16 位） | 缓存 key | `?if_version=` 条件请求 → 304 |
| 随机种子 | 服务端 `random()` | 只读（字符串） | `/battle/token`、`/battle/:id/replay` |
| 攻方属性（attacker） | 服务端 `computeAttacker` | **只读，禁止本地推算** | `/me`、`/battle/token` 的 `build.attacker` |
| 出战槽位 | `user_skill_slots` | 乐观 UI + 提交 | `PUT /me/loadout` |
| 技能等级 | `user_skills.level` | 只读（费用/上限由服务端裁定） | `POST /me/skills/:id/upgrade` |
| 钱包余额 | `user_wallets` + `user_tokens` | 只读 | `/wallet`、各写操作回执 |
| 进度（max_stage） | `user_progress` | 只读 | `/me` 的 `max_stage` |
| 关卡星级 | `level_stars`（GREATEST） | 只读 | `/me/stars` |
| 结算结论（星级/掉落/裁剪） | 服务端 `ValidateSettle` + `ComputeLoot` | **只读** | `/battle/settle` 回执 |
| 回放哈希 | 客户端计算，服务端**只存** | 计算方 | settle 上报 |
| S 段（replay_skills） | 客户端计算，服务端**独立重算比对** | 计算方 | settle / challenge 上报 |
| 防线快照 | 服务端存，内容来自玩家构筑 | 提交方 | `POST /defenses/save` |
| 防线挑战胜负 | 客户端裁决（I-5 取舍） | 裁决方 | `POST /defenses/:id/challenge` |
| 每日「今天」 | Go 的 `periodStart(time.Now(), "daily")` | 无关 | 传日期字面量给 SQL |
| 服务端时钟 | `/config` 的 `server_time` / `Date` 头 | 校准本地偏移 | `estimateServerNowMs()` |

两条最容易违反的：

1. **攻方属性不能本地推算**。回放哈希由「关卡 + 攻方属性 + 技能槽」共同决定；
   两端各算一套，哈希必然不同，I-6 会把所有正常对局判成伪造。
   客户端的本地推算只作为服务端未下发时的占位，并会在控制台明确告警。
2. **「今天」不能交给数据库**。`challenge_date` / `sign_date` / 任务周期
   全部由 Go 侧算好日期字面量再传 SQL，否则容器里 `time.Local=UTC` 会让
   每日任务与签到在不同时刻翻页。

---

## 6. 端间交互矩阵

| 交互 | 发起方 | 接收方 | 协议 | 频率 | 关键约束 |
|---|---|---|---|---|---|
| 游客登录 / 恢复 | miniapp | server | `POST /auth/guest` | 冷启动 1 次 | 单飞（single-flight），避免并发建两个号 |
| 配置拉取 | miniapp | server | `GET /config?if_version=` | 冷启动 1 次 | 304 零载荷；本地持久化副本先点亮界面 |
| 开局 | miniapp | server | `POST /battle/token` | 每局 1 次 | 扣体力 + 扣复活币 + 发种子，同一事务 |
| 结算 | miniapp | server | `POST /battle/settle` | 每局 1 次 | token 一次性；422 表示上报不可信 |
| 验真 | miniapp / admin | server | `POST /battle/verify`、`POST /admin/battles/:id/verify` | 按需 | 服务端**不重算**，只比对提交值与记录值 |
| 防线列表 / 挑战 | miniapp | server | `GET /defenses`、`POST /defenses/:id/challenge` | 按需 | 挑战胜负客户端裁决；S 段可校验 |
| 运营读写 | admin | server | `/api/v1/admin/*` | 人工 | 全部写操作进 `admin_audit_logs` |
| 玩法文档站 | admin | server | `GET /config`、`GET /admin/reactions` | 打开页面 | 与线上同源，避免文档与游戏不一致 |
| 契约校验 | CI / 本地 | 双向 | `go run ./cmd/vectors -check` | 每次改动 | Go 与 TS 共读同一份 `testdata/*.json` |

---

## 7. 关键时序

### 7.1 冷启动 → 可玩

```
miniapp                      server                      PostgreSQL
   │                            │                            │
   ├─ guestLogin() ────────────▶│ upsert users ─────────────▶│
   │                            │ initNewUser（钱包/进度/装备/技能/槽位）
   │◀──── access+refresh+user ──┤                            │
   │                            │                            │
   ├─ loadConfig() ────────────▶│ staticConfig()（进程内 once）
   │  带本地 version            │ version 相同 → 304         │
   │◀──── 304 / 200 全量 ───────┤                            │
   │                            │                            │
   ├─ refreshProfile() ────────▶│ /wallet（含体力恢复）      │
   │                            │ /me（build + power + max_stage）
   │◀──── 钱包 / 构筑 ──────────┤                            │
   │                            │                            │
   └─ 首页可交互（配置已就绪）  │                            │
```

要点：

- 配置**先用本地持久化副本点亮界面**，再条件请求确认；304 时零字节载荷。
- 304 也要校准时钟（用 `Date` 头），否则长时间不更新配置的老玩家会出现
  「过期护盾显示为生效中」。
- `refreshProfile` 用 `/me` 的 `user_id` 回填身份，而不是依赖「登录过一次」。

### 7.2 一局战斗（最关键的一条链路）

```
miniapp                                    server
   │                                          │
   ├─ POST /battle/token {level_id} ─────────▶│ ① 校验关卡已解锁
   │                                          │ ② 生成 seed
   │                                          │ ③ 事务：插 token + 扣体力 + 扣复活币
   │◀── token_id, seed(str), level(含waves), │
   │    build(含 attacker), extra_base_hp ────┤
   │                                          │
   ├─ new BattleEngine({level, seed, attacker,│
   │    equipped(来自 build.skills[].slot),   │
   │    activeSlots, scoreRules, extraBaseHp})│
   │                                          │
   ├─ 本地 tick 推进（20Hz）                  │
   │   ├ 刷怪 / 自动开火 / 命中 / 反应 / 漏怪 │
   │   ├ 波次间隙：三选一（技能/属性/机制）   │
   │   └ 记录 replay 事件 + card_picks        │
   │                                          │
   ├─ POST /battle/settle {token_id, …} ────▶│ ④ ValidateSettle（上界/交叉/时长/速率）
   │                                          │ ⑤ replay_skills 重算比对（S 段）
   │                                          │ ⑥ 整局重算（若启用）→ 台账
   │                                          │ ⑦ 事务：抢 token → 写战报 → 发掉落
   │                                          │    → 推进进度 → 更新星级 → 任务进度
   │◀── battle_id, stars, score, loot, wallet,│
   │    build_rating, power, new_max_stage ───┤
   │                                          │
   └─ refreshProfile()                        │
```

失败分支：

| 阶段 | 失败 | 返回 | 客户端表现 |
|---|---|---|---|
| 开局 | 关卡未解锁 | 403 `forbidden` | 提示「第 N 关未解锁」 |
| 开局 | 体力不足 | 400 `bad_input` | 提示余额不足 |
| 结算 | token 已用/过期/关卡不符 | 422 `settle_rejected` | 「上报不可信」 |
| 结算 | 数值越界（击杀/反应/发射数…） | 422 `settle_rejected` | 同上 |
| 结算 | S 段不符且 `enforce` | 422 `settle_rejected` | 同上 |
| 结算 | S 段不符但 `shadow` | 200（照常发奖） | 玩家无感，台账留痕 |
| 结算 | 服务内部错误 | 500 `internal_error` | 「服务内部错误」 |

### 7.3 验真（I-6）

```
玩家 / 运营                    server                      本地
   │                             │                          │
   ├─ GET /battle/:id/replay ───▶│ 校验归属（玩家）           │
   │◀─ level + seed + build +    │                          │
   │   card_picks + expected_hash│                          │
   │                             │                          │
   ├─────────────────────────────┼─────────────────────────▶│ replay() 重跑
   │                             │                          │ 算出 actual_hash
   ├─ POST /battle/verify ──────▶│ 比对 expected vs actual   │
   │   {battle_id, replay_hash}  │ 写 replay_verifications   │
   │◀─ matched / expected / actual│                         │
```

**这个机制证明了什么、没证明什么**（必须在评审时说清楚）：

- 证明：战报记录与「用同一关卡+种子+构筑重放出来的结果」一致。
  即「改了数据却没改凭证」会被发现。
- 不证明：如果作弊者从一开始就伪造一整套自洽的数据（含 hash），
  服务端在没有引擎时无法识破 —— 这正是 R1 整局重算要补的那一段。

### 7.4 结算重算闸门（R1）

```
SettleBattle
   │
   ├─ replay_skills 不符？──是──▶ Gate.Decide(ScopeSkills)
   │                                ├ enforce → 台账(rejected=true) → 422
   │                                └ shadow/off/熔断 → 台账(rejected=false) → 继续
   │
   ├─ 整局重算（recomputeEngine）
   │     ├ 引擎不可用 → cannotRun++（不阻断）
   │     ├ 一致 → verified++
   │     └ 不符 → mismatch++ → Gate.Decide(ScopeEngine)
   │                ├ enforce → 台账 → 422
   │                └ 其余 → 台账 → 继续
   │
   └─ 正常结算事务
```

三档语义：

| 模式 | 行为 | 适用 |
|---|---|---|
| `off` | 完全不校验（S 段也不拒） | 应急回滚 |
| `shadow` | 只记录、不拒发 | 引擎刚接入时的观察期 |
| `enforce`（默认） | 不符即拒发 | 正常生产 |

**熔断器**：`enforce` 下若最近 10 分钟窗口的不符率超过阈值且样本量足够，
自动降级为 `shadow` 并记录原因 —— 理由是「引擎漂移是系统性的，
批量误伤诚实玩家比零星作弊更糟」。台账里 `mode` 与 `rejected` 分开存，
因为熔断可以让 `enforce` 下的单次处置变成「不拒发」。

### 7.5 防线挑战（I-5）

```
挑战者 miniapp                server                    防守者
   │                            │                         │
   ├─ GET /defenses ───────────▶│ 候选（按战力降序，排除自己）
   │◀─ mine + candidates ───────┤ 含 shielded_until / 剩余次数
   │                            │
   ├─ GET /levels/1 ───────────▶│ 标准战场（含 waves）
   │                            │
   ├─ runChallenge(本地模拟)     │
   │   （用防守方 snapshot）     │
   │                            │
   ├─ POST /defenses/:id/challenge
   │   {seed, won, duration_ms, │ ① 字段边界校验（事务外）
   │    hp_left_pct, replay_hash│ ② 事务：锁双方计数 → 检查次数/护盾/被偷上限
   │    replay_skills} ────────▶│ ③ 胜则按 10% 偷取（受掉落封顶）
   │                            │ ④ 写 defense_challenges（settled 幂等）
   │◀─ stolen / owner / 计数 ───┤
```

胜负由客户端裁决（`KNOWN_LIMITS` 第 3 条），服务端能守住的是：
次数上限、护盾、被偷上限、字段边界、**挑战者构筑的 S 段**（D4 落地）。

### 7.6 运营改内容 → 玩家可见

```
admin                    server                        miniapp
  │                        │                             │
  ├─ PUT /admin/levels/1 ─▶│ 写 levels 表（玩家侧权威=生成器，见 D1）
  │                        │ 写 admin_audit_logs          │
  │                        │                             │
  │                        │   ⚠️ 玩家路径读的是 GenerateLevel，
  │                        │      不是 levels 表 → 本次编辑不生效
  │                        │                             │
  ├─ POST /admin/levels/regenerate ▶ 重生成 100 关并落库（需 confirm 口令，同样不影响玩家路径）
  │                        │                             │
  │                        │  /config 的 version 是内容哈希 → 代码没变就不变
  │                        │                             │
  └─ 改 domain 代码 + 重新部署 ──▶ 进程重启后 staticConfig 重建
                                                          │
                                          冷启动带旧 version → 200 全量
```

这条时序解释了一个常见疑问：**为什么后台改关卡对玩家没影响**。
原因是 `DECISIONS D1` 已拍板「玩家侧关卡权威 = 生成器」，
`levels` 表是运营规划与统计的载体，不是玩家路径的输入
（`service/level_authority_test.go` 钉住这一点）。

---

## 8. 失败模式与降级策略

| 场景 | 系统行为 | 设计理由 |
|---|---|---|
| 数据库不可用 | 启动直接退出；运行中 `/readyz` 503 | 不做「先起来再重试」，编排层必须立刻知道 |
| 微信登录未配置 | `/auth/wechat` 返回 403（消息明确说未启用） | 静默降级成游客会让「登录失败」变成「登录成功但数据不对应」 |
| 配置拉取失败 | 用本地持久化副本继续（若有）；无副本则页面提示 | 宁可展示旧内容，也不要白屏 |
| access token 过期 | 发请求前预判（30s 提前量）→ 刷新；401 再兜底刷新一次 | 省一次必然 401 的往返 |
| 并发 401（多页面同时请求） | 刷新单飞（single-flight），只发一次 | N 个 401 各自刷新会互相删掉新令牌，导致静默登出 |
| 结算重算引擎不可用 | `cannotRun++`，不阻断结算 | 引擎故障不应变成「所有玩家领不到奖励」 |
| 重算不符率异常升高 | 熔断降级 `enforce → shadow` | 系统性漂移时，批量误伤比零星作弊更糟 |
| 限流命中 | 429 + 中文提示 | 按 IP 与按身份分工，避免运营商 NAT 误伤 |
| panic | 500 + 栈进日志 | 单个请求不能打挂进程 |

---

## 9. 端与端的「禁止事项」

这些不是风格建议，每一条都对应一次真实故障（详见 `ENGINEERING.md`）：

1. **客户端不得硬编码分数规则、技能升级规则、星级比例** —— 必须读 `/config`。
2. **客户端不得本地推算攻方属性** —— 必须读 `build.attacker`。
3. **服务端不得把「没传」与「传了空」当成同一件事** —— 查询参数用
   `QueryArgs().Has`，不用 `c.Query(k, default)`（后者会把 `?type=` 吞成默认值）。
4. **服务端不得把非法输入静默重解释成另一个合法值** —— 未知榜单类型必须 400，
   不能回落成战力榜。
5. **不得用 `CURRENT_DATE` 表达业务日** —— 由 Go 传日期字面量。
6. **不得在客户端与服务端各写一份同一公式** —— 要么服务端下发，要么进契约夹具。
7. **不得在 GET 端点里写库** —— 防线计数曾因此在浏览时落行、并与挑战路径抢锁。
8. **不得让「看起来实现了」通过验收** —— 每个机制必须有能红的判据（见 `ENGINEERING.md` 第 8 条）。
