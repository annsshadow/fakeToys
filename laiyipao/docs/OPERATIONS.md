# 运维手册

> 面向**部署与值班**。玩法设计见 [`GAME_DESIGN.md`](./GAME_DESIGN.md)，
> 工程约束与已知边界见 [`../README.md`](../README.md)。
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
| Go | 1.26.3 | `go version` |
| PostgreSQL | 15+ | `psql --version` |
| Node | 20+ | `node -v` |
| pnpm | 11 | `pnpm -v` |

数据库默认用 `trust` 认证（仅本机开发）。**生产必须改**：
见 §6.1 的连接串与 `pg_hba.conf` 说明。

### 步骤

```bash
# 1) 建库
createdb laiyipao

# 2) 迁移（13 个 goose 迁移，49 张业务表）
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

## 2. 环境变量（全部 23 个）

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

### 数据库连接池

| 变量 | 缺省 | 说明 |
|---|---|---|
| `DB_MAX_CONNS` | `10` | ⚠️ 见 §7.2 的连接数计算 |
| `DB_MIN_CONNS` | `2` | — |
| `DB_CONN_LIFETIME` | `1h` | 连接最大存活时间 |
| `DB_TIMEZONE` | `Asia/Shanghai` | **连接会话时区**，第 90 轮新增 |

> ⚠️ **不要把它设成 UTC。**
>
> 这个值决定 PG 把 `TIMESTAMPTZ` 折成 `DATE` 时用哪个日界，
> 进而决定「今天几号」。业务是中文小程序，
> 「今天」必须等于**玩家所在时区的自然日** ——
> 设成 UTC 会让每日任务与每日签到在北京时间 **08:00** 翻页，
> 而玩家手机上显示的还是同一天。
>
> 服务端已在连接握手上钉死这个值（`RuntimeParams["TimeZone"]`），
> 所以它**不依赖** `postgresql.conf` 或托管实例的默认时区。
> 改这个环境变量是改**业务日界**，不是改部署细节。
>
> 守卫：`server/internal/store/timezone_test.go` 3 条
> （含「5 条物理连接各自都必须是该值」与「赋值必须无条件执行」）。

### 玩法

| 变量 | 缺省 | 说明 |
|---|---|---|
| `BATTLE_TOKEN_TTL` | `30m` | 战斗凭证有效期 |
| `MIN_BATTLE_DURATION` | `15s` | 「清完全场」的最短合理时长，短于此的**胜利**上报会被拒 |
| `MAX_SCORE_PER_SECOND` | `40000` | ⚠️ **已废弃，不参与判定**。历史上它被当速率上限，但值远高于第 1 关满分，`min()` 永远取满分那侧，限制实际失效。现由 `scoreCapFor` 按关卡动态计算 |
| `OFFLINE_CAP_HOURS` | `12` | 离线收益累计上限 |

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

### 3.3 加一条迁移的检查清单

1. 文件名递增：`migrations/000NN_描述.sql`
2. **Up 段必须可重复执行**（用 `IF NOT EXISTS`）——
   迁移失败后重跑是常见操作，不可重复执行的迁移会把人卡住
3. **Down 段必须真的能删**（`DROP ... IF EXISTS`）
4. 加列时给 `DEFAULT` 且 `NOT NULL` —— 存量行也要有合法值
5. 跑 `go run ./cmd/migrate` + `go run ./cmd/migrate -down` + 再 `-` 一次，确认可升可降

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
- [ ] `npx vitest run --config vitest.config.ts` 通过
      （miniapp）；`cd ../admin && npx vitest run` 亦需全绿
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
