# 运维手册

> 面向**部署与值班**。玩法设计见 [`GAME_DESIGN.md`](./GAME_DESIGN.md)，
> 工程约束见 [`ENGINEERING.md`](./ENGINEERING.md)，已知边界见 [`KNOWN_LIMITS.md`](./KNOWN_LIMITS.md)。
>
> 本手册里的每条命令都实际执行过；未验证的操作不写进来。

---

## 0. 一分钟速查

| 我要… | 做什么 |
|---|---|
| 首次部署 | §1 全流程 |
| 日常发布 | §5 发布检查清单 |
| 服务起不来 | §7.1 |
| 数据库连不上 | §7.2 |
| 结算接口报 400 | §7.3 |
| 历史战报验真失败 | §7.4 **（最常见且最容易被误判的故障）** |
| 加一条数据库迁移 | §3 |
| 改关卡/技能数值 | §4 **（会让存量战报失配）** |
| 备份 / 恢复 | §6 |
| 回滚一次发布 | §5.3 |

---

## 1. 首次部署

### 前置

| 依赖 | 版本 | 检查 |
|---|---|---|
| Go | 1.26.10 | `go version` |
| PostgreSQL | 15+ | `psql --version` |
| Node | 20+ | `node -v` |
| pnpm | 11 | `pnpm -v` |

> ⚠️ Go 的下限是 **1.26.10**，不是「大致 1.26」。1.26.9 及以下含
> `GO-2026-6809`（`crypto/x509` 不拒绝被攻破的中间证书），而本服务的调用链
> **代码可达**（`service.RecordRecompute → fmt.Fprintf → x509.Certificate.Verify`）——
> CI 的 govulncheck 闸门会直接红。想降版本之前先看
> `.github/workflows/laiyipao-ci.yml` 的 `dependency-audit` 一节：
> 那条闸门的判据是「**代码可达**的漏洞必须为 0」，不是「漏洞库里有几条」。

数据库默认用 `trust` 认证（仅本机开发）。**生产必须改**：
见 §6.1 的连接串与 `pg_hba.conf` 说明。

### 步骤

```bash
# 1) 建库
createdb laiyipao

# 2) 迁移（23 个 goose 迁移，50 张业务表）
cd laiyipao/server
export DATABASE_URL="postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable"
export BOOTSTRAP_ADMIN_PASS="<至少 10 位，缺省会拒绝启动>"
go run ./cmd/migrate

# 3) 写入游戏内容（敌人/技能/装备/关卡/初始状态）
go run ./cmd/seed

# 4) 启动
go run ./cmd/api          # 监听 :8080
```

### 验证

```bash
curl -s http://127.0.0.1:8080/readyz     # 期望 {"ok":true,"db":"up"}
curl -s http://127.0.0.1:8080/api/v1/config | head -c 200   # 期望能看到 levels 数组
```

- `/healthz` —— 返回 `{"ok":true,"time":"..."}`，**不查 DB**，用于 liveness 探针
- `/readyz` —— **会查 DB**，用于 readiness 探针

> 两个端点故意分开：DB 抖动时 liveness 仍应通过，
> 否则 K8s 会把一个只是暂时连不上库的 Pod 反复重启。

### 前端与后台

```bash
cd laiyipao/miniapp && pnpm install && pnpm dev:h5      # :5174
cd laiyipao/admin   && pnpm install && pnpm dev          # :5175
```

---

## 2. 环境变量（全部 39 个）

### 必填（生产）

| 变量 | 说明 |
|---|---|
| `DATABASE_URL` | PostgreSQL 连接串 |
| `JWT_SECRET` | **生产必须覆盖**。`APP_ENV=prod` 时若仍以 `dev-only` 开头，**进程直接 panic 拒绝启动** |
| `BOOTSTRAP_ADMIN_USER` | `admin` | 首次启动创建的后台**用户名** |
| `BOOTSTRAP_ADMIN_PASS` | `admin12345` | 首次启动创建的后台**密码**，**至少 10 位**，缺省则拒绝引导 | ⚠️ **第 67 轮**：`APP_ENV=prod` 时**必须显式设置**，未设直接 panic —— 默认值 `admin12345` 写在 README 与源码里是**公开的**，且恰好 10 位**正好通过**长度门槛

### 服务

| 变量 | 缺省 | 说明 |
|---|---|---|
| `ADDR` | `:8080` | 监听地址 |
| `APP_ENV` | `dev` | `prod` 会启用上面那条 JWT 校验 |
| `LOG_LEVEL` | `info` | — |
| `MIGRATIONS_ON_BOOT` | `true` | 见 §3.2 的取舍 |
| `ACCESS_TTL` | `2h` | 访问令牌有效期 |
| `REFRESH_TTL` | `720h` | 刷新令牌有效期 |
| `CORS_ALLOW_ORIGINS` | 空 | 跨域来源白名单（逗号分隔，逐项去空白）；dev 默认放行 `*`、prod 默认不发送任何 CORS 头（语义见 §2 末尾注释） |

### 数据库连接池

| 变量 | 缺省 | 说明 |
|---|---|---|
| `DB_MAX_CONNS` | `10` | ⚠️ 见 §7.2 的连接数计算 |
| `DB_MIN_CONNS` | `2` | — |
| `DB_CONN_LIFETIME` | `1h` | 连接最大存活时间 |
| `DB_CONN_IDLE_TIME` | `30m` | 空闲连接回收时间（R91 补登记）。回收的是「很久没被用过」的连接，不是活跃连接；托管 PG 有连接数上限时靠它把空闲连接还回去 |
| `DB_STATEMENT_TIMEOUT` | `30s` | **单条 SQL 的服务端执行上限**（R91 补登记），`0` = 不限。超时由 PG 以 `57014` 断开并释放连接，正常请求收到 503；默认值足以覆盖现有导出/看板聚合，又能在查询写坏时避免拖垮整池 |
| `DB_ACQUIRE_TIMEOUT` | `5s` | **池耗尽的等待上限**（R90）。排队超过该值 → 请求快速失败 `503 + Retry-After: 5`，而不是无限挂起（R89 实测曾挂起 15s 且无任何一层掐断）。⚠️ **不要设得太短**：正常负载的排队 p99 只有几十毫秒，设成 100ms 级会把合法突发打成 503 **假失败**，客户端重试还会放大流量——建议 `3s`–`10s`，且必须小于客户端超时。`0`/负值**启动直接失败** |
| `DB_LOCK_TIMEOUT` | `2s` | **等待任何一把锁的预算**（R94）。锁阻塞语句超过该值被 PG 取消（`55P03`）→ 503，而不是挂到锁释放（R89 实测 p95=15032ms 全挂起，客户端 10s 超时时 503 无人接收）。与 `DB_STATEMENT_TIMEOUT` 解耦：后者管整条语句（含正常慢查询/大导出），锁等待单独给短预算。⚠️ 迁移（goose）持锁远超 2s，迁移启动时**必须显式设 `0`**。`0` = 关闭（恢复 R89 旧行为，不推荐） |
| `DB_TIMEZONE` | `Asia/Shanghai` | **连接会话时区**（R90）。业务日界（R92）由 `config.BusinessTimeZone` 固定为 Asia/Shanghai，**不随此值或容器 `TZ` 漂移** |

> ⚠️ **R92 起：业务日界不再由这个变量决定。**
>
> 业务日界只有一个来源：`server/internal/config` 的 `BusinessTimeZone`
> （固定 `Asia/Shanghai`）。每日任务 / 签到 / 每日限购 / 防御挑战次数
> 都由 Go 侧 `periodStart` 按该时区截断自然日；运营看板的 SQL 则显式写
> `AT TIME ZONE 'Asia/Shanghai'` —— **都不依赖**本环境变量的会话时区，
> 也不依赖容器 `TZ`。以前「容器 TZ 未设（UTC）→ 早上 8 点才翻页」
> 的故障形态已从代码层面消除。
>
> 本变量现在只影响「SQL 没显式写时区」时的隐式折算（生产代码已无此类 SQL，
> 由 `TestServiceHasNoCurrentDateInBusinessDaySQL` 守卫）。保留它作为
> 连接默认值，是为了让人工排查的即席查询与业务口径一致。
> 改它是改部署细节，**不再**改业务日界；要改业务日界请改 `config.BusinessTimeZone`。
>
> 守卫：`server/internal/store/timezone_test.go` 3 条 + R92 的两条源码守卫
> （禁裸 `CURRENT_DATE` / `date_trunc('day', now())`，且 SQL 时区字面量必须等于
> `config.BusinessTimeZone`）。

### 导出

| 变量 | 缺省 | 说明 |
|---|---|---|
| `EXPORT_MAX_CONCURRENCY` | `2` | 同一时刻允许执行的 CSV 导出数（R91 补登记），必须 ≥ 1。单次导出独占一条池连接；默认 2 是给 10 条连接里的普通请求留余量 |
| `EXPORT_WAIT` | `3s` | 导出排队等槽位的上限（R91 补登记），`0` = 满槽立即 503。排队超时同样走快速失败，不占住请求 |
| `EXPORT_MAX_BYTES` | `64 MiB` | 单次导出允许物化的**字节**上限（R91 补登记，`67108864`；`0` = 不限）。超出直接 413 —— 筛得太宽或行内容异常时，413 比 OOM 好 |

### 玩法

| 变量 | 缺省 | 说明 |
|---|---|---|
| `BATTLE_TOKEN_TTL` | `30m` | 战斗凭证有效期 |
| `MIN_BATTLE_DURATION` | `15s` | 「清完全场」的最短合理时长，短于此的**胜利**上报会被拒 |
| `SETTLE_RECOMPUTE_MODE` | `enforce` | 服务端重算的处置档位，`off` / `shadow` / `enforce`。**不发版即可切换**，改环境变量重启生效 |
| `SETTLE_SLOW_MS` | `500` | 慢结算日志阈值（毫秒，R91 补登记）。达到该值打一条分段耗时日志；`0` = 关闭日志（聚合统计仍记）。低于它不打，避免噪声 |
| `OFFLINE_CAP_HOURS` | `12` | 离线收益累计上限（2026-10-31 新一轮第 7 轮起由 `GET/POST /me/offline-reward` 消费） |
| `OFFLINE_COIN_PER_HOUR` | `600` | 离线收益每小时金币速率（2026-10-31 新一轮第 7 轮）。收益按「距上次领取的时长 × 速率」结算，领取时清零；`0` = 无收益 |
| `DASHBOARD_CACHE_TTL` | `30s` | 运营看板的进程内缓存有效期。**设成 `0` 即关闭缓存**（每次都查库）—— 排查「数字为什么不动」时先关它，不需要发版 |
| `LEADERBOARD_CACHE_TTL` | `15s` | 三个榜单的进程内缓存有效期。`0` = 关闭。⚠️ `/me/rank`（我的排名）**不受它影响**，永远实时 |
| `SIGNIN_CALENDAR_CACHE_TTL` | `60s` | 签到日历预览的进程内缓存有效期。`0` = 关闭（日历是 seeder 静态配置，后台无修改入口，通常不用关） |
| `ANNOUNCEMENTS_CACHE_TTL` | `30s` | 公告列表的进程内缓存有效期。`0` = 关闭（公告是公开端点，首页首屏每次拉取；运营人工发布、变化极低频） |
| `SHOP_CACHE_TTL` | `30s` | 商城商品列表**静态部分**的进程内缓存有效期。`0` = 关闭（商品是 seeder 配置、仅低频运营修改；玩家今日已购次数是 per-user 数据，不缓存） |
| `TASKS_CACHE_TTL` | `60s` | 任务配置**静态部分**的进程内缓存有效期。`0` = 关闭（任务是 seeder 纯静态配置、无后台修改入口；玩家当日进度是 per-user 数据，不缓存） |
| `ECONOMY_CACHE_TTL` | `30s` | 后台经济流水页的进程内缓存有效期。`0` = 关闭（`wallet_flows` 是增长最快的表，每次打开经济页都全表聚合；运营看经济页是低频操作） |
| `ADMIN_USERS_COUNT_CACHE_TTL` | `5s` | 后台玩家列表 `total` 的进程内缓存有效期（R91 补登记），`0` = 关闭。列表分页的 total 是最贵的 COUNT，短 TTL 换吞吐；运营要「立刻看到人数变化」时设 0 |
| `METRICS_ENABLED` | `false` | 是否挂 `/metrics`（Prometheus 文本格式：TTL 缓存命中/未命中计数等）。**默认不挂**——未设口令的指标端点会泄露内部计数，确认采集端可达再开 |
| `PPROF_ADDR` | 空 | pprof 性能剖析监听地址（如 `127.0.0.1:6060`）。**默认不启动**；仅绑回环，排障时临时开 |
| `PLAYER_LOGIN_REQUIRED` | `false` | 玩家是否必须用账号口令登录。开启后 `/auth/guest` 一律 403，并引导一个**与超管同名同口令**的共享测试账号（测试环境用，详见 §2.4） |

#### `PLAYER_LOGIN_REQUIRED` 怎么用（测试环境）

**背景**：游客通道给**每个浏览器/设备**一个独立账号。测试环境里这意味着
测试数据散落在无数个游客号里、换浏览器就丢进度、多人测时互相看不见。

**开启**：在 `config.env` 里加 `PLAYER_LOGIN_REQUIRED=true`，重启服务。

| 项 | 行为 |
|---|---|
| 账号 | 启动时引导创建，用户名与口令**与运营超管相同**（复用 `BOOTSTRAP_ADMIN_USER` / `BOOTSTRAP_ADMIN_PASS`） |
| H5 | 出登录页（只回填用户名，**口令不预填** —— 前端产物是公开的） |
| `/auth/guest` | 一律 **403 `login_required`**（且**不建号**）—— 服务端这一道才是权威判据，缓存的旧客户端也一样被挡住 |
| 限流 | `POST /auth/login` **10/min/IP**（bcrypt cost 10，且这个口令同时是超管口令的爆破面） |
| 已存在的账号 | 启动只**同步口令**，**不碰**钱包/进度/专精/战报 |

**关掉**：把 `PLAYER_LOGIN_REQUIRED` 去掉或设为 `false` 重启 —— 游客通道恢复，
**已有的账号行不受影响**（账号只是多了一行玩家数据）。

**改口令**：改 `BOOTSTRAP_ADMIN_PASS` 后重启，共享测试账号的口令会跟着变
（两侧刻意保持一致，否则会出现「后台能登、H5 登不进」）。

#### `SETTLE_RECOMPUTE_MODE` 怎么用

当前**只有一条重算在跑**：`replay_skills`（`replay_hash` 的 S 段）。整局引擎尚未移植。

| 档位 | 行为 | 什么时候用 |
|---|---|---|
| `enforce`（缺省） | 重算并记录；不符即拒发奖励 | 常态。**不要**随意改 |
| `shadow` | 重算并记录；**不拒发** | 只在排查「校验误伤」时用 —— 它确实关掉了 S 段反作弊 |
| `off` | 完全不重算 | 已被证明误伤且来不及排查时的最后手段 |

**⚠️ 切到 `shadow` 或 `off` 就等于关掉 `replay_skills` 反作弊校验。** 这两档虽然仍写审计台账，但不再拦下任何伪造上报。

**⚠️ 缺省值是 `enforce` 而不是 `off`，这是刻意的。** S 段的重算没有歧义：服务端与客户端读同一份 DB 数据，中间没有第二个实现，所以「不符」只可能是有人手改了上报值。整局引擎才有歧义（也可能是引擎算错了），那才需要先 `shadow` 观察。把两类混成一个默认值，会让新装部署出厂即关闭已在运行的防护。

**熔断只作用于整局引擎，永不作用于 S 段。** 引擎侧不符率超过 2%（且样本 ≥200）时自动降级回 `shadow` 并打印原因；**不会自动升回** `enforce`，必须人工改配置重启。S 段不参与熔断 —— 否则作弊脚本一开刷、不符率冲高就会把防护降级，等于在攻击发生时关掉刹车。

不符记录在 `settle_recomputes` 表（列：`reported_*` / `recomputed_*` / `mismatched_fields` / `mode` / `rejected` / `engine_version` / `created_at`）。后台页面：**结算重算审计**（`/admin/recomputes`）。

⚠️ `battle_id = 0` 表示「结算在写战报之前就被拒，没有战报 id」—— 目前只有 `replay_skills` 不符会这样，页面上显示为「—」。

#### 怎么用这个页面

**先看顶部的横幅。** 熔断触发时页面顶部会出现黄色横幅，此时**本该拒发的奖励正在放行**。台账数字此时可能完全正常（甚至因为不拒发而上升），所以不单独指出来的话容易误以为防护还开着。横幅消失的唯一方式是**人工**把 `SETTLE_RECOMPUTE_MODE` 改回 `enforce` 并重启。

**然后看「按关卡分布」那一栏**，判读方式：

| 观察到的形状 | 大概率是 | 该做什么 |
|---|---|---|
| 某几关不符率很高，且「单号最高不符」很小 | 引擎漂移（某段公式算错） | 查 `engine_version` 定位发布，回滚 |
| 某关不符率很高，且「单号最高不符」≈ 不符数 | 单个号在作弊 | 查该 `user_id`，封号 |
| 不符均匀散布在很多号上 | 引擎漂移 | 同第一行 |

页面上的「形态提示」只是**提示**，不是判定 —— 措辞刻意不给结论，因为给错的结论比不给更糟（可能照着「这是引擎 bug」去回滚，实际是有人在作弊）。

`mode` 与「处置」是两列，必须分别看：熔断可以在 enforce 档把单次处置降级，所以会看到 `enforce（本次降级）` —— 那一行的 `rejected` 是 `false`，**本该拒发的确实放行了**。



### 微信（未接通）

| 变量 | 缺省 | 说明 |
|---|---|---|
| `WECHAT_APP_ID` | 空 | 未配置时 `/auth/wechat` 返回 **501**，不静默降级成游客登录 |
| `WECHAT_APP_SECRET` | 空 | — |
| `WECHAT_ENDPOINT` | 微信官方地址 | 仅测试用 |

---

## 3. 数据库迁移

### 3.1 日常操作

```bash
cd laiyipao/server
go run ./cmd/migrate          # 应用
go run ./cmd/migrate -down    # 回滚**最后一个**
```

> ⚠️ **工具自己说 `-down` 是「仅本地调试用」**
> （`go run ./cmd/migrate -h` 的原文）。
> 这个警告要认真对待：回滚一个已经**写入生产数据**的迁移，
> 会连带删掉那张表里的数据。
>
> `-down` **只回滚一版**。连续回滚要重复执行。
> 且每一步都可能因为「该版有数据变换」而失败 —— 失败时**读它的 Down 段**再决定，
> 不要盲目重复。
>
> 生产环境回滚的正确姿势见 §5.3：**优先只回滚代码，不回滚迁移**。

### 3.2 `MIGRATIONS_ON_BOOT` 的取舍

缺省 `true`（服务启动时自动迁移）。**多副本部署时必须设为 `false`**：

> 多个副本同时跑 goose 会互相抢 `goose_db_version` 表的锁，
> 轻则报错重试，重则迁移执行两次。
> 生产应改为**部署流程里单独跑一次 `cmd/migrate`**，服务只读不写。

### 3.2.1 多副本就绪矩阵（R87 双实例实测）

**默认部署形态是单实例**（`cmd/api` 单进程）。上多副本前必须过这张表
——它是实测出来的（`node scripts/multi-instance-check.mjs`，
起两个真实实例逐项验证，可复跑），不是推断：

| 项 | 双实例实测 | 判定 | 部署动作 |
|---|---|---|---|
| 限流（per-IP / per-user / 全局，进程内计数） | 合计放行 **2×** 单实例额度（60→120） | ⚠️ 防护变弱（不产生资金错误，业务幂等兜底） | **必须** `RATE_LIMIT_SCALE=1/N`（心跳验证：0.5×2=60 ✓） |
| 幂等键（R86，PG 表） | 跨实例重放**逐字节相同**、余额只动一次 | ✅ 多副本就绪 | 无 |
| 看板/榜单增量表（R63/R64，PG 表） | 两实例读数一致 | ✅ 多副本就绪（各自缓存期内瞬时差异，到期收敛） | 无 |
| 缓存新鲜度（公告/任务/商城/看板/榜单，TTL 进程内） | A 写入后 B **最长 30s 不可见** | ⚠️ 可容忍（运营配置类） | 接受 TTL 窗口内陈旧；写侧实例自身立即生效 |
| singleflight（进程内） | 冷启动底层查询 ×N（每实例各一次） | ✅ 可忽略（TTL 边界多 N-1 次查询） | 无 |
| access/结算聚合（R80/R85，进程内） | 各实例各记一份 | ⚠️ 面板口径 | Prometheus `sum()` 聚合；单实例面板只见 1/N 流量 |

**当前没有「必须单实例」的资金级项**：唯一状态共享硬约束（幂等键、
增量计数表）都在 PostgreSQL 里，天然多副本就绪。其余进程内状态
的后果都已列明且可控。这张表由脚本守——改动其中任何一类机制时，
重跑 `multi-instance-check.mjs`，FAIL 即先修再上副本。

### 3.3 加一条迁移的检查清单

1. 文件名递增：`migrations/000NN_描述.sql`
2. **Up 段必须可重复执行**（用 `IF NOT EXISTS`）——
   迁移失败后重跑是常见操作，不可重复执行的迁移会把人卡住
3. **Down 段必须真的能删**（`DROP ... IF EXISTS`）
4. 加列时给 `DEFAULT` 且 `NOT NULL` —— 存量行也要有合法值
5. 跑 `go run ./cmd/migrate` + `go run ./cmd/migrate -down` + 再 `-` 一次，确认可升可降

### 3.4 滚动发布兼容性（R88 四组合实测）

滚动发布时新旧二进制共存、schema 只有一份。R88 用 git 旧提交构建真实旧二进制
（不手工改代码），对两个方向做了实测。结论先行：

> **00025–00027 的迁移是 expand-only，「先迁移后换二进制」滚动发布安全；
> 反向（先换二进制后跑迁移）是降级运行——玩家链路无损，新特性读侧明确 500。**

#### 实测矩阵

旧二进制起点：旧A = R63 父提交（00024 时代，触发器引入前）；
旧B = R80（00026 时代，结算重排前）。关键路径 = 建号/签到/领奖/兑换/结算/看板。

| 组合 | 结果 | 判定 |
|---|---|---|
| 旧A + 00027 | 全 200；结算 60×200 无 5xx；看板走旧全表聚合口径、与触发器累加一致 | ✅ **安全** |
| 旧B + 00027 | 全 200；结算 240×200 无 5xx | ✅ **安全** |
| 新 HEAD + 00024 | 玩家链路全 200；结算 240 行正常落库；**看板/幂等 500 明确报错** | ✅ **安全（降级）** |
| 新 HEAD + 00026 | 玩家链路+看板 200、触发器累加一致；**幂等带 key 500**、无 key 200 | ✅ **安全（降级）** |

依据（为什么安全）：

- **新表对旧二进制透明**：00025（dashboard_*）、00027（admin_idempotency_keys）
  只新建表，旧代码不读它们；旧代码 INSERT 的列集一列未变。
- **触发器在 DB 侧，对调用方透明**：00025 引入触发器、00026 把 4 参函数换成
  5 参——二进制从不直接调这些函数，新旧二进制写 `battle_records` 都经同一个
  DB 触发器。goose 单事务内 DROP+CREATE 原子完成，无「函数不存在」窗口。
- **新二进制缺表时是明确 500，不是静默错算**：新 HEAD 对 00024 跑看板/幂等，
  SQL 报「表不存在」→ 500 内部错误，运维能立刻看到「该先跑迁移」。
- **写路径不依赖新 schema 对象**：新 HEAD 对 00024 结算，240 行全部正常落库
  （`battle_records` 从 00004 起列集稳定）。00025 的 Up 段含**全量回填**
  （`INSERT INTO dashboard_battle_stats SELECT ... FROM battle_records`），
  迁移补跑后窗口期的战报会被回填治愈，无永久漂移。

#### ⚠️ 滚动窗口的两个灰度语义（不是故障，但要知道）

1. **旧二进制静默忽略 `Idempotency-Key`**（R86 引入该头，旧代码不认识 →
   直接执行）。窗口期内运营重试发币/公告/批量码，**旧实例上会重复执行**。
   发布顺序上：迁移先行（00027 是纯新表，随时可跑），换二进制期间管理端
   写操作**只打新实例**（或在窗口期不携带幂等键的重试视为可能重复）。
2. **迁移执行瞬间的锁**：00010/00013/00014 含存量行 UPDATE/DELETE，
   大表执行期有行锁与 WAL 膨胀——迁移应在低峰跑，且先 `SELECT count(*)`
   估量级（`battle_records` 百万行的 UPDATE 是分钟级）。

#### expand-contract 违规登记：00013_achievement_epoch（已发布，不再改）

00013 把成就从「按天分行」改为「哨兵行终身累计」：迁移合并历史行进
`1970-01-01` 哨兵行并 DELETE，**同时**新二进制把 `periodStart(achievement)`
改为返回哨兵日期。**若先跑迁移后换二进制**，窗口期内旧实例仍读「今天零点」
那行——历史行已被删，旧实例会新建当天行、进度归零重算、**已领过的成就
可再领一次（资金级）**。

该迁移已随第 93 轮发布（当时单实例停机发布，风险未兑现），**不改历史**。
但它定义了一类必须遵守的顺序约束：

> **改「周期/口径语义」的迁移（periodStart、统计口径、状态机枚举）属于
> expand-contract：必须先全部换完新二进制、验证新口径写入正常，再跑迁移。**
> 发布顺序步骤：
> 1. 发布新二进制到全部实例（`MIGRATIONS_ON_BOOT=false`，手动控迁移）；
> 2. 验证新口径已生效（抽查写入落在新哨兵/新列上）；
> 3. 跑迁移（合并/回填历史数据）；
> 4. 观察一个业务周期确认无双重领取/双重计数。

#### 迁移兼容性检查清单（R88 核对 27 个迁移后固化）

新迁移评审时逐条过：

- [ ] **新增 NOT NULL 列必须带 DEFAULT**（存量行立即合法；无 DEFAULT 的
      `ADD COLUMN ... NOT NULL` 在 PG 上会重写全表且旧代码 INSERT 必炸）
- [ ] **新增列优先可空**，用代码填默认值，比 DB DEFAULT 更灵活
- [ ] **DROP 列/表必须晚一个版本**：版本 N 先让所有代码不再引用，
      版本 N+1 才 DROP（本仓库 27 个迁移的 DROP 全部只在 Down 段，合规）
- [ ] **触发器/函数变更必须在同一事务内 DROP+CREATE**（goose 默认行为，
      不要在迁移里加 `NO TRANSACTION` 破坏它）
- [ ] **索引只增不减**；换索引写法 = 新建一个再 DROP 旧的（旧索引在窗口期
      仍服务旧二进制）
- [ ] **存量数据 UPDATE/DELETE（回填型）**：评估锁量级，进 §3.4 的
      灰度语义第 2 条；纯配置行（百行级）随意，用户数据行（万行级）要低峰
- [ ] **周期/口径语义变更 = expand-contract**：按上面 00013 的四步顺序发布，
      并在迁移头部注释里写明「必须先换二进制」

27 个存量迁移核对结果：26 个合规；00013 违规（已发布，登记如上，不再改）；
00010/00014 回填型合规但需注意执行期锁。

#### 滚动发布演练纪律（R88 布线事故教训）

本地多实例演练时：进程绑定 `127.0.0.1` 明确地址（Windows 上 `0.0.0.0:P`
与 `127.0.0.1:P` 可共存，请求优先命中更具体绑定——R88 曾被一个 R58 残留的
影子进程截胡全部请求，对着错误的库跑出整套假阴性）；起进程后先建一个号
核对 id 落在目标库的序列区间，再开始测。

---

## 4. 改关卡 / 技能 / 公式 —— 会让存量战报失配

> ⚠️ **本节是整个运维手册里唯一会造成不可逆数据损失的地方。**

### 4.1 为什么危险

I-6 的重放哈希覆盖**关卡 id + 攻方系数 + 技能槽 + 整条事件流**。
改动下列任一项，历史战报的 `replay_hash` 就**永远算不出来**：

- 关卡生成器（波次/敌种/血量/地形 → 事件序列变了）
- 伤害公式（权重、系数、反通胀上限）
- 分数规则（记进 `hit` 事件的分数值变了）
- 技能基础数值
- 元素系数 / 反应倍率
- **槽位数**（`activeSlots` 缺省必须是 4）

症状是**验真失败** —— 而验真失败看起来像作弊。
运维会误判成「有人刷分」，实际上是发版造成的。

### 4.2 正确流程

```bash
# 1) 改完代码/数据，先重生成契约夹具
cd laiyipao/server
go run ./cmd/vectors            # ← 忘了这步，后面全错
go run ./cmd/vectors -check     # CI 里用这个验证「夹具与源码是否同步」

# 2) 跑全部测试
go test ./... -count=1 -timeout 1200s

# 3) 跑端到端验收
cd .. && pwsh -NoProfile -File scripts/e2e.ps1
```

### 4.3 行为测试「全绿」不代表没破坏可重放性

`no_deadlock.test.ts` / `difficulty.test.ts` 这类行为测试
只断言「能通关 / 难度递增」，**不校验哈希**。

> 实测教训：改动关卡生成后，行为测试仍然全绿，
> 而**每一个存量战报都已经静默失配**。
> 判断是否破坏可重放性，唯一可靠的办法是重跑 `cmd/vectors`
> 并检查 diff 里是否包含 `star_targets` / 波次 / 地形的变化。

---

## 5. 发布

### 5.1 检查清单

- [ ] `go test ./... -count=1 -timeout 1200s` 全绿，**且输出里没有 SKIP**
      （SKIP 被计入 PASS，「全绿」里可能藏着没执行的测试。
      **有 PostgreSQL 在途中**：DB 依赖的测试会自检跳过；CI 无 PG 服务时
      259 个 DB 测试自然跳过，这属于已知行为，不是回归。
      本地需 `go test ./... -count=1 -timeout 1200s` 在**有 PG** 的前提下跑，
      方能进入「零 SKIP」的真门槛。）
- [ ] `pnpm vitest run --config vitest.config.ts` 通过
      （miniapp）；`cd ../admin && pnpm vitest run` 亦需全绿
- [ ] 双端 `npx tsc --noEmit` 无错误
- [ ] `gofmt -l server` 输出为空、`go vet ./...` 无输出
- [ ] `go run ./cmd/vectors -check` 全 OK
- [ ] `pwsh -NoProfile -File scripts/e2e.ps1` 全部通过
- [ ] **若改过关卡/公式**：`go run ./cmd/vectors` 已重跑，且 diff 已人工确认
- [ ] `JWT_SECRET` / `BOOTSTRAP_ADMIN_PASS` 已在生产环境覆盖
- [ ] `MIGRATIONS_ON_BOOT=false`，迁移已在部署流程里单独执行

### 5.2 测试超时的一个坑

```bash
go test ./... -count=1 -timeout 300s     # ← internal/service 会超时失败
```

`internal/service` 单独跑只要 **77s**，但 11 个包**并行抢同一个 PostgreSQL** 时
会涨到 **340s**，撞上 300s 超时。

- 症状：`FAIL internal/service`，但单独重跑该包**通过** —— 这是**假性超时**，不是真的失败
- **不要**因此去改被测代码
- 正确做法：用 `-timeout 1200s`，或把依赖 DB 的包串行化
  （`go test -p 1 ./...`）

### 5.3 回滚

```bash
# 1) 回滚代码到上一个 tag
# 2) 数据库迁移：不要盲目 -down
```

> ⚠️ **如果新版本的迁移是加列/加表**，旧代码不认识新列但能正常工作
> （多出的列有默认值）→ **无需回滚迁移**，只回滚代码。
>
> ⚠️ **如果新版本改动了关卡生成或伤害公式**（§4），
> 回滚代码**不能**恢复历史战报的可重放性 ——
> 只有把数据库也回滚到对应时点才行，而那会丢掉那之后的所有玩家数据。
> **这种改动本质上不可回滚**，发布前必须慎重。

---

## 6. 备份与恢复

### 6.1 备份

```bash
pg_dump -Fc -d laiyipao -f laiyipao-$(date +%F-%H%M).dump
```

**必须包含的表**（丢了就无法运营）：

| 表 | 丢了会怎样 |
|---|---|
| `battle_records` | 全部战报与验真凭证消失；排行榜与星级历史归零 |
| `replay_verifications` | 验真留痕消失 |
| `user_*`（钱包/进度/专精/装备/技能/关卡进度） | 玩家资产归零 |
| `defenses` | 玩家防线配置消失 |
| `shop_offers` / 内容表 | 后台改过的数值归零 |

> 内容表（敌人/技能/装备/关卡）**可以用 `cmd/seed` 重建**，
> 但**后台的手工修改会丢** —— 所以还是得备份。

### 6.2 恢复

```bash
# 1) 停服
# 2) 恢复
pg_restore -d laiyipao -c laiyipao-YYYY-MM-DD-HHMM.dump
# 3) 校验版本
cd laiyipao/server && go run ./cmd/migrate   # 幂等，补齐备份之后的迁移
# 4) 起服并验证
curl -s http://127.0.0.1:8080/readyz
```

### 6.3 生产数据库认证

开发用 `trust`（本机免密）。生产必须：

```
# pg_hba.conf
host    laiyipao   laiyipao_app   10.0.0.0/8    scram-sha-256
```

并用 `scram-sha-256` 而非 `md5`。

---

## 6.4 数据保留与清理

> 2026-10-23 新增。此前仓库里**没有任何**清理/归档/分区机制，
> 而下面三张表都是「每局一条」量级的只增表。

### 先看会删多少（默认 dry-run）

```bash
cd server
go run ./cmd/prune --days=30
```

输出每张表的待删行数与「为什么这张表可以删」。**这一步不会删任何东西。**

### 真的删

```bash
go run ./cmd/prune --days=30 --apply
```

### 四张表的处置

| 表 | 默认保留 | 删了会怎样 |
|---|---|---|
| `battle_records` | 建议 ≥ 30 天 | **旧战报的回放（`/battle/:id/replay`）与验真永久失效**；`replay_verifications` 会因外键 CASCADE 一起被删 |
| `replay_verifications` | 建议 ≥ 30 天 | 只影响「谁在什么时候验过哪一场」的排查记录，不参与结算 |
| `admin_audit_logs` | 建议 ≥ 90 天 | 后台操作审计消失。**保留期应不短于内部合规要求**，默认 30 天对多数场景偏短 |
| `wallet_flows` | **永不删** | 它是经济系统的原始凭证。删了之后「这个玩家的金币怎么来的」永远无法追 |

`wallet_flows` 要压缩只能**归档到冷表**（2026-10-30 起已命令化，见
`cmd/prune` 的 `--archive-wallet-flows`）：

```bash
# 先看会归档多少行（不搬任何数据）
go run ./cmd/prune --days=180 --archive-wallet-flows

# 确认后真的搬：老流水进入 wallet_flows_archive，主表对应行被删除
go run ./cmd/prune --days=180 --archive-wallet-flows --apply
```

冷表 `wallet_flows_archive` 由命令自动创建（`LIKE wallet_flows INCLUDING ALL`，
不含外键）。归档与删除共用同一个 `--days` 截止时间，同样默认 dry-run、
`--apply` 才真搬，且**永不进入 `--table` 删除白名单**。

⚠️ 归档表**不进任何查询**，所以它只解决体积，不解决「查得到」。
需要长期可查的话，导出到对象存储而不是留在同一个库里。

### 两条硬约束（写死在命令里，不可绕过）

1. **`--days` 没有默认值**，且不得小于 **7**。战报要支撑验真 / 回放 / 客服申诉
   三条业务，短于 7 天会删掉仍在用的数据。
2. **`--apply` 才真删**。默认只统计 —— 这个命令最可能的用法是
   `ssh prod` 之后顺手跑一下，所以默认必须是无害的那一侧。

### 建议的节奏

- 先连续两周只跑 dry-run，观察「30 天前的战报有多少」的实际曲线。
- 曲线平稳后再开 `--apply`，并把它放进 systemd timer（每周一次）。
- `admin_audit_logs` 单独用更大的 `--days` 跑，不要和战报同一个值。

---

## 7. 故障处置

### 7.1 服务起不来

| 现象 | 原因 | 处理 |
|---|---|---|
| `panic: config: APP_ENV=prod 时必须显式设置 JWT_SECRET` | 生产没覆盖 `JWT_SECRET` | 设置一个非 `dev-only` 开头的值 |
| `BOOTSTRAP_ADMIN_PASS` 缺失即拒绝 | 缺省密码太弱 | 设一个 ≥10 位的值 |
| 启动卡在迁移 | 见 §3.2 | 多副本设 `MIGRATIONS_ON_BOOT=false`，部署流程里单独跑 |
| `database "laiyipao" does not exist` | 没建库 | `createdb laiyipao` |

### 7.2 数据库连不上 / 连接池耗尽

先算连接数：

```
实例数 × DB_MAX_CONNS 必须 < PostgreSQL 的 max_connections
```

缺省 `DB_MAX_CONNS=10`。4 副本就是 40 条连接。
若 `max_connections=100` 且还想跑测试与备份工具，**必须调低 `DB_MAX_CONNS`**。

```sql
SHOW max_connections;
SELECT count(*), state FROM pg_stat_activity GROUP BY state;
```

### 7.3 结算接口报 400

400 意味着上报被服务端拒了。按错误码定位：

| 错误 | 含义 |
|---|---|
| `剩余血量超过该关初始血量` | 上报了不可能的血量 |
| `发射数超过该时长的物理上限` | 时长与发射数不自洽 |
| `命中率越界` | `hits > shots` |
| `发射数超过该关总怪数` | 击杀/漏怪数超过该关总怪数 |
| `元素使用合计超过…` | `elements_used` 越界 |
| `未知反应 "xxx"` | `reactions_used` 里有不存在的键 |
| `得分速率超过上限` | 时长远短于清场所需 |
| `波次超过该关总波数` | — |

> **若大量玩家同时报 400，且代码刚发过版** → 优先怀疑 §4 的公式改动，
> 而不是玩家作弊。

### 7.4 历史战报验真失败 ⚠️

**这是最容易被误判的故障。**

**先分清是「玩家刷分」还是「发版造成」**：

```sql
SELECT date_trunc('day', created_at) AS d, count(*)
FROM battle_records GROUP BY 1 ORDER BY 1;
```

| 特征 | 判断 |
|---|---|
| 失败集中在**发版之后**，且**跨所有关卡** | **发版造成**（改了哈希锚点，见 §4） |
| 失败集中在**少数关卡 / 少数玩家** | 可能是真的篡改 |
| 失败集中在一段历史时间段 | 那一版代码有问题 |

**发版造成时的处理**：

1. **不要**给玩家发「作弊」通知 —— 这是我方问题
2. 若能定位到具体改动，**回滚代码**（注意 §5.3 的不可回滚警告）
3. 记入事故：写明「哪些 commit 改了哈希锚点」

> 预防：把 `cmd/vectors -check` 放进 CI —— 它能在发版前发现
> 「关卡/公式改动没有同步重生成夹具」。

---

## 8. 值班备忘

```bash
# 活着吗
curl -s localhost:8080/healthz && curl -s localhost:8080/readyz

# 最近的错误率 / 战报量
psql -d laiyipao -c "
SELECT date_trunc('hour', created_at) h, result, count(*)
FROM battle_records WHERE created_at > now() - interval '24 hours'
GROUP BY 1,2 ORDER BY 1 DESC LIMIT 48;"

# 异常胜率（防线值守的作弊信号）
psql -d laiyipao -c "
SELECT round(avg(CASE WHEN won THEN 1.0 ELSE 0 END)::numeric,3) AS win_rate, count(*)
FROM battle_records WHERE level_id IN (SELECT DISTINCT level_id FROM battle_records
  WHERE result='lose' LIMIT 5) AND created_at > now() - interval '7 days';"
```

> 防线值守的胜负由**客户端裁决**（服务端不跑引擎），
> 所以胜率是最重要的作弊信号。接近 100% 或 0% 都需要人工看。
