# 07 · 配置与部署

> 本文覆盖：依赖版本、**全部环境变量**、首次部署、构建、nginx / systemd、
> 迁移策略、发布检查清单、回滚、备份恢复、故障速查。
>
> 值班用的「一分钟速查」在 `OPERATIONS.md` §0；本文是它的**实现依据**。

---

## 1. 依赖与版本

| 组件 | 版本 | 用途 |
|---|---|---|
| Go | **1.26.3** | 后端编译 |
| PostgreSQL | **15+** | 数据库（需要 `num_nonnulls`、`GENERATED … AS IDENTITY`） |
| Node | **20+** | 前端构建 |
| pnpm | 11 | 依赖管理 |
| nginx | 任意稳定版 | 反代 + 静态托管 |
| systemd | 任意 | 服务托管 |

前端工具链版本有**硬锁**，不要随手升级：

| 包 | 版本 | 原因 |
|---|---|---|
| `vite`（miniapp） | **5.2.8** | `@dcloudio/vite-plugin-uni` 的 peer 硬锁 |
| `vitest`（miniapp） | 2.1.9 | 与 vite 5 配套 |
| `vite`（admin） | 8.x | Rolldown 构建；`manualChunks` 只接受函数形式 |

`miniapp/src/test/toolchain_pin.test.ts` 会在依赖被改动后先红。

---

## 2. 环境变量全表

### 2.1 后端（23 个）

| 变量 | 默认值 | 必填（prod） | 说明 |
|---|---|---|---|
| `ADDR` | `:8080` | 否 | 监听地址 |
| `APP_ENV` | `dev` | **是**（设 `prod`） | 决定 CORS 默认值与 prod 守卫 |
| `LOG_LEVEL` | `info` | 否 | ⚠️ 已解析但当前未被消费（无日志分级实现） |
| `DATABASE_URL` | `postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable` | **是** | 连接串；prod 必须显式含 `sslmode=` |
| `DB_MAX_CONNS` | `10` | 否 | 连接池上限（越界回退默认） |
| `DB_MIN_CONNS` | `2` | 否 | 连接池下限 |
| `DB_CONN_LIFETIME` | `1h` | 否 | 连接最大存活 |
| `DB_TIMEZONE` | `Asia/Shanghai` | 否 | **钉在每条连接上的会话时区** |
| `MIGRATIONS_ON_BOOT` | `true` | 否 | 启动时自动迁移；生产建议 `false` + 独立执行 |
| `JWT_SECRET` | `dev-only-insecure-secret-change-me` | **是** | prod 下以 `dev-only` 开头会 panic |
| `ACCESS_TTL` | `2h` | 否 | 玩家/管理员 access token 有效期 |
| `REFRESH_TTL` | `720h`（30d） | 否 | 玩家 refresh token 有效期 |
| `WECHAT_APP_ID` | 空 | 否 | 与 Secret 同时存在才启用微信登录 |
| `WECHAT_APP_SECRET` | 空 | 否 | |
| `WECHAT_ENDPOINT` | `https://api.weixin.qq.com/sns/jscode2session` | 否 | 可指向 mock |
| `CORS_ALLOW_ORIGINS` | 空 | 否 | 逗号分隔白名单；**prod 为空 = 不发任何 CORS 头** |
| `BATTLE_TOKEN_TTL` | `30m` | 否 | 开局凭证有效期 |
| `MIN_BATTLE_DURATION` | `15s` | 否 | 最短可信战斗时长 |
| `SETTLE_RECOMPUTE_MODE` | `enforce` | 否 | `off` / `shadow` / `enforce` |
| `OFFLINE_CAP_HOURS` | `12` | 否 | ⚠️ 已解析但当前无离线收益端点消费 |
| `BOOTSTRAP_ADMIN_USER` | `admin` | 否 | 首次引导管理员用户名 |
| `BOOTSTRAP_ADMIN_PASS` | `admin12345` | **是** | prod 下未显式设置会 panic；长度 < 10 时拒绝创建 |

> **没有速率上限配置项，这是刻意的。** 曾经的 `MAX_SCORE_PER_SECOND`
> （硬编码 50000）已在 2026-10-20 删除：它远高于第 1 关满分（19539），
> `min()` 永远取满分那侧，限制从未真正生效。现在由 `domain.scoreCapFor`
> 按关卡推导。**不要再把它加回来** —— 锚点必须与 `scoreCapFor`
> 用同一个值，做成可配置项就多了一个「两处可能不同步」的面。

三条 prod 守卫（`config.Load`）：

```
JWT_SECRET 以 "dev-only" 开头          → panic
DATABASE_URL 不含 "sslmode="           → panic
BOOTSTRAP_ADMIN_PASS 环境变量为空      → panic
```

第三条的理由：默认口令 `admin12345` 恰好 10 位，**能通过**
`EnsureBootstrapAdmin` 的长度门槛。而它写在 README 与源码里，等于公开。
拿到 admin token 后可给任意用户发币、封禁任意玩家、改商城价格 ——
即完整经济与封禁体系的接管。

非 prod 环境不会 panic，但启动日志会打印 `DevDefaultWarnings()` 的醒目告警，
让「正在用公开默认凭证」这件事不再隐形。

### 2.2 前端

| 变量 | 作用域 | 默认 | 说明 |
|---|---|---|---|
| `VITE_API_BASE` | miniapp + admin | miniapp: `http://127.0.0.1:8080/api/v1`；admin: `/api/v1` | 构建时烘焙进产物 |
| `VITE_API_TARGET` | admin dev server | `http://127.0.0.1:8080` | 仅开发代理目标，不进产物 |

⚠️ miniapp 的 `VITE_API_BASE` 若是**公网域名 + `http://`**，应用启动即抛错
（令牌 30 天有效，明文过网等于长期裸奔）。局域网 IP 只告警，回环地址放行。

### 2.3 密钥管理

| 文件 | 内容 | 是否入库 |
|---|---|---|
| `laiyipao/deploy/.local.keys` | `JWT_SECRET` / `ADMIN_PASS` / `DB_DSN` / `DOMAIN` | **否**（`.gitignore`） |
| `laiyipao/deploy/remote-config.env` | 上传到服务器的 `config.env` 副本 | **否** |
| `/usr/local/laiyipao/etc/config.env` | 远端实际配置（`chmod 600`） | 只在服务器 |

仓库里**不含**任何真实域名 / IP / 口令。`deploy/README.md` 里的
`<YOUR_TEST_DOMAIN>` 是占位符。

---

## 3. 首次部署

### 3.1 数据库

```bash
# 建库与账号（示例）
sudo -u postgres psql -c "CREATE USER laiyipao_app WITH PASSWORD '<PASSWORD>';"
sudo -u postgres psql -c "CREATE DATABASE laiyipao OWNER laiyipao_app;"
```

要求：PostgreSQL 15+；时区由应用钉死（`DB_TIMEZONE`），不依赖服务器配置。

### 3.2 迁移与种子

```bash
cd laiyipao/server
export DATABASE_URL="postgres://laiyipao_app:<PASSWORD>@127.0.0.1:5432/laiyipao?sslmode=disable"

go run ./cmd/migrate      # 15 个迁移，50 张表
go run ./cmd/seed         # 首次必须执行：敌人/技能/装备/关卡/签到/任务/商城/兑换码
```

`cmd/seed` 是**幂等 upsert**：

- 内容表会被覆盖为当前代码里的值；
- `shop_items.enabled` **不在 upsert 分支里**，后台手动上下架能扛住重灌；
- 玩家数据一律不碰。

> 只跑迁移不跑种子，`/config` 仍能工作（内容是 `domain` 静态生成的），
> 但后台看板、商城、任务、签到会读到空表。

### 3.3 启动

```bash
export APP_ENV=prod
export JWT_SECRET="$(openssl rand -hex 32)"
export BOOTSTRAP_ADMIN_PASS='<强口令>'
./lyp-server-amd64
```

启动日志会打印：

```
[来一炮 server] 启动配置: addr=:8080 env=prod db=postgres://laiyipao_app:***@…/laiyipao wechat_enabled=false access_ttl=2h0m0s refresh_ttl=720h0m0s
[来一炮 server] 已创建初始管理员：admin（密码来自 BOOTSTRAP_ADMIN_PASS，请尽快修改）
[来一炮 server] 监听 :8080（微信登录通道：未启用（游客登录））
```

### 3.4 验证

```bash
curl -s localhost:8080/healthz           # {"ok":true,"time":"…"}
curl -s localhost:8080/readyz            # {"ok":true,"db":"up"}
curl -s localhost:8080/api/v1/config | head -c 200
curl -s -X POST localhost:8080/api/v1/auth/guest \
     -H 'Content-Type: application/json' -d '{"guest_token":"","nickname":""}'
```

四项全通 = 账号、配置、数据库三条主链路就绪。

---

## 4. 构建

### 4.1 后端（Linux amd64，CGO 关闭）

```bash
cd laiyipao/server
CGO_ENABLED=0 GOOS=linux GOARCH=amd64 \
  go build -ldflags "-s -w" -o ../deploy/bin/lyp-server-amd64 ./cmd/api
CGO_ENABLED=0 GOOS=linux GOARCH=amd64 \
  go build -ldflags "-s -w" -o ../deploy/bin/lyp-migrate ./cmd/migrate
CGO_ENABLED=0 GOOS=linux GOARCH=amd64 \
  go build -ldflags "-s -w" -o ../deploy/bin/lyp-seed ./cmd/seed
```

### 4.2 运营后台

```bash
cd laiyipao/admin
pnpm install
VITE_API_BASE="https://<DOMAIN>/laiyipao-api/api/v1" pnpm build
# 产物：admin/dist（base 已是 /laiyipao-admin/）
```

`pnpm build` 会先跑 `vue-tsc --noEmit`，类型错误会阻断构建。
只想快速出包用 `pnpm build:only`。

体积检查（可选但推荐）：

```bash
pnpm size          # 报告首屏 raw / gzip 与逐路由代价
pnpm size:check    # 超预算退出码 1（raw 220 KB / gzip 80 KB）
```

### 4.3 小程序 H5

```bash
cd laiyipao/miniapp
pnpm install
VITE_API_BASE="https://<DOMAIN>/laiyipao-api/api/v1" pnpm build:h5
# 产物：miniapp/dist/build/h5
```

### 4.4 微信小程序包

```bash
pnpm build:mp-weixin
# 产物：miniapp/dist/build/mp-weixin → 用微信开发者工具上传
```

注意：

- `manifest.json` 的 `mp-weixin.appid` 为空，需填入真实 AppID；
- 真机调试要把 `VITE_API_BASE` 改成局域网地址（会打印明文告警），
  或配置微信「合法域名」；
- `manifest.json` 的 `urlCheck: false` 仅用于开发。

---

## 5. nginx

```nginx
# API → 127.0.0.1:9020（注意 proxy_pass 末尾的斜杠：剥离 /laiyipao-api/ 前缀）
location /laiyipao-api/ {
    proxy_pass http://127.0.0.1:9020/;
    proxy_http_version 1.1;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
    proxy_read_timeout 120s;
    client_max_body_size 64m;
}

# 后台（SPA）
location = /laiyipao-admin { return 301 /laiyipao-admin/; }
location ^~ /laiyipao-admin/assets/ {
    alias /usr/local/laiyipao/admin/assets/;
    expires 1y; add_header Cache-Control "public, immutable"; access_log off;
}
location /laiyipao-admin/ {
    alias /usr/local/laiyipao/admin/;
    index index.html;
    try_files $uri $uri/ /laiyipao-admin/index.html;
    add_header Cache-Control "no-cache, must-revalidate" always;
}

# H5（uni-app build:h5 产物）
location = /laiyipao-h5 { return 301 /laiyipao-h5/; }
location ^~ /laiyipao-h5/assets/ {
    alias /usr/local/laiyipao/h5/assets/;
    expires 1y; add_header Cache-Control "public, immutable"; access_log off;
}
location ^~ /laiyipao-h5/static/ {
    alias /usr/local/laiyipao/h5/static/;
    try_files $uri =404;
    expires 1y; add_header Cache-Control "public, immutable";
}
location /laiyipao-h5/ {
    alias /usr/local/laiyipao/h5/;
    index index.html;
    try_files $uri $uri/ /laiyipao-h5/index.html;
    add_header Cache-Control "no-cache, must-revalidate" always;
}
```

要点：

- `proxy_pass` **必须**以 `/` 结尾，否则 `/laiyipao-api/api/v1/...` 会原样转发，
  服务端收到 `/laiyipao-api/api/v1/...` → 404。
- `assets/` 用 immutable 长缓存（文件名带哈希）；`index.html` 必须 no-cache，
  否则发版后用户仍加载旧入口。
- 若把前端与 API 部署到**不同域名**，必须配 `CORS_ALLOW_ORIGINS`，
  否则 `prod` 默认不发 CORS 头，浏览器会拒绝读取响应。

---

## 6. systemd

```ini
[Unit]
Description=LaiYiPao Game Platform API
After=network.target postgresql.service
Wants=postgresql.service

[Service]
Type=simple
User=root
WorkingDirectory=/usr/local/laiyipao
EnvironmentFile=/usr/local/laiyipao/etc/config.env
ExecStart=/usr/local/laiyipao/bin/lyp-server-amd64
Restart=on-failure
RestartSec=5
LimitNOFILE=65535
StandardOutput=append:/usr/local/laiyipao/logs/stdout.log
StandardError=append:/usr/local/laiyipao/logs/stderr.log

[Install]
WantedBy=multi-user.target
```

`config.env`（`chmod 600`）示例：

```bash
APP_ENV=prod
ADDR=127.0.0.1:9020
DATABASE_URL=postgres://laiyipao_app:<PASSWORD>@127.0.0.1:5432/laiyipao?sslmode=disable
JWT_SECRET=<64 字节随机>
BOOTSTRAP_ADMIN_PASS=<强口令>
SETTLE_RECOMPUTE_MODE=enforce
MIGRATIONS_ON_BOOT=false
```

目录布局：

```
/usr/local/laiyipao/
├─ bin/    lyp-server-amd64 / lyp-migrate / lyp-seed
├─ admin/  后台静态产物
├─ h5/     H5 静态产物
├─ etc/    config.env（600）
└─ logs/   stdout.log / stderr.log
```

---

## 7. 迁移策略

### 7.1 生产推荐

```bash
# 发版前单独执行（不在启动时自动迁移）
DATABASE_URL=… ./bin/lyp-migrate
```

`MIGRATIONS_ON_BOOT=false` 的理由：启动时迁移失败会让服务起不来，
而回滚代码后数据库已经前进了 —— 两件事应该分开、可控。

### 7.2 加一条迁移的检查清单

1. 文件名 `000NN_desc.sql`，含 `+goose Up` / `+goose Down`；
2. Down **必须能跑通**，并写明是否**有损**（有损要写原因）；
3. 大表加列优先 `ADD COLUMN … DEFAULT`（PG 11+ 不重写表）；
4. 加唯一约束前先确认存量数据无重复；
5. 改 `redeem_codes` / `user_tasks` 这类有哨兵语义的表时，
   同步检查相关查询；
6. 更新 [06-data-model.md](./06-data-model.md) §12。

### 7.3 不要盲目 `-down`

`00011` / `00013` / `00014` / `00015` 的 Down 都是**有损**的
（删运营验真记录 / 丢历史周期 / 丢已洗练词条 / 丢重算证据）。
回滚前先看迁移文件头部的说明，并做备份。

---

## 8. 发布检查清单

### 8.1 后端

- [ ] `go test ./...` 全绿（含真实库测试）
- [ ] `go vet ./...` 无告警
- [ ] 若改了内容表/公式：`go run ./cmd/vectors` 重生成夹具，再跑一次测试
- [ ] `cmd/vectors -check` 无差异
- [ ] 迁移已在预发库验证（含 Down）
- [ ] `SETTLE_RECOMPUTE_MODE` 取值确认（默认 `enforce`）
- [ ] 启动日志无 `⚠️ 正在使用开发默认凭证`

### 8.2 前端

- [ ] `pnpm type-check` 通过
- [ ] `pnpm test` 全绿
- [ ] `VITE_API_BASE` 是 **https** 且指向正确环境
- [ ] admin `pnpm build` 成功；`pnpm size:check` 未超预算
- [ ] 产物路径与 nginx `alias` 一致

### 8.3 端到端

- [ ] `pwsh -File scripts/e2e.ps1`（会**清空重建** `laiyipao` 库，只能在测试环境跑）
- [ ] 手工验证：游客登录 → 配置加载 → 开一局 → 结算 → 验真 → 防线保存/挑战
- [ ] 后台：登录 → 看板 → 改关卡 → 发币 → 审计日志有记录

> ⚠️ 改关卡/技能/公式会让**存量战报永久失去可重放性**。
> 处理流程见 `OPERATIONS.md` §4：先重生成契约夹具 → 跑全量测试 → 跑 e2e →
> 在公告里说明。行为测试全绿**不**代表可重放性没被破坏。

---

## 9. 回滚

### 9.1 后端

```bash
systemctl stop laiyipao
mv /usr/local/laiyipao/bin/lyp-server-amd64{,.old}
# 放入上一版二进制
systemctl start laiyipao
```

### 9.2 前端

```bash
mv /usr/local/laiyipao/admin{,.old}     # 或 h5
# 放入上一版产物
```

静态资源文件名带哈希，回滚后浏览器会自然拉到旧文件；
`index.html` 是 no-cache，不会卡住。

### 9.3 数据库

**先判断迁移是否可逆**：

- 纯加列/加表 → 可保留（旧代码不认识新列，不影响）；
- 改了语义（如 00013 成就合并）→ 回滚代码**不会**恢复旧行为，
  需要按迁移文件的 Down 处理，且会丢数据。

宁可「代码回滚、数据库保留」，也不要盲目 `-down`。

---

## 10. 备份与恢复

### 10.1 备份

```bash
pg_dump -Fc -d laiyipao -f /backup/laiyipao_$(date +%F).dump
```

建议：每日全量 + 发布前手动一次；保留 ≥ 7 天。

### 10.2 恢复

```bash
systemctl stop laiyipao
dropdb laiyipao && createdb -O laiyipao_app laiyipao
pg_restore -d laiyipao /backup/laiyipao_2026-10-20.dump
DATABASE_URL=… ./bin/lyp-migrate      # 确认 schema 版本
systemctl start laiyipao
```

恢复后必须验证：`/readyz`、`/config` 的 `version`、随机抽一条战报能读回。

---

## 11. 故障速查

| 现象 | 最可能原因 | 处置 |
|---|---|---|
| 启动即 `FATAL: 连接数据库` | DSN 错 / 库未起 / 端口不通 | 用 `psql` 直连验证 |
| `config: APP_ENV=prod 时必须显式设置 JWT_SECRET` | 漏设环境变量 | 补 `JWT_SECRET` |
| `config: … 必须显式设置 BOOTSTRAP_ADMIN_PASS` | 同上 | 补口令（≥10 位） |
| `/readyz` 503 | 数据库不可用 | 查 PG 与连接数 |
| 后台白屏 | `base` 不是 `/laiyipao-admin/`，或 nginx alias 不匹配 | 查产物路径与 network 面板 |
| 后台所有请求 404 | `proxy_pass` 末尾漏了 `/` | 补斜杠 |
| 浏览器报 CORS | prod 未配 `CORS_ALLOW_ORIGINS` 且前后端不同源 | 配白名单或改同源 |
| 登录后立刻 401 | `JWT_SECRET` 在多次发布间变了 | 固定密钥 |
| 玩家「体力够但开不了局」 | 客户端并发登录建了两个号 | 检查 `login()` 单飞 |
| 玩家「进度凭空消失」 | 登出后没保存新 `guest_token` | 检查客户端 `logout()` 顺序 |
| 结算大量 422 | S 段校验误伤 / 引擎漂移 | 查 `/admin/recomputes/stats`；必要时 `SETTLE_RECOMPUTE_MODE=shadow` |
| `demoted=true` | 熔断已降级，奖励在放行 | 查不符率与关卡分布，定位漂移 |
| 兑换码创建 500 | `redeem_codes` 序列未对齐 | 跑 `cmd/seed`（含 setval） |
| 每日任务/签到在早上 8 点翻页 | Go 与 PG 时区不一致 | 设 `DB_TIMEZONE` 并确认容器 `TZ` |
| 后台图标全空 | 图标写成字符串名 | 传组件引用 |
| 构建报 `Big integer literals are not available` | 构建 target < es2020 | 保持 `es2020` |

---

## 12. 安全清单

- [ ] `APP_ENV=prod`，三条守卫都通过
- [ ] `JWT_SECRET` 64 字节随机，且**不随发布变化**
- [ ] `BOOTSTRAP_ADMIN_PASS` 强口令，首登后修改
- [ ] `config.env` 权限 600，不在仓库里
- [ ] `DATABASE_URL` 含 `sslmode=require`（跨网时）
- [ ] 前端 `VITE_API_BASE` 是 https
- [ ] nginx 下发 `X-Frame-Options: DENY`（服务端也会发，双保险）
- [ ] 前后端不同源时配 `CORS_ALLOW_ORIGINS`，不要用 `*`
- [ ] 后台账号按最小权限分配（只读用 `readonly`）
- [ ] 定期检查 `admin_audit_logs` 的 `grant_currency` / `revoke_currency`
- [ ] 单实例部署（限流是进程内计数）；若上多副本，先换共享计数器
