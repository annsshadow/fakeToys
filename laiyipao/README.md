# 来一炮

仿微信小游戏《向僵尸开炮》玩法内核自研的**塔防 + 元素反应**小程序游戏。美术 100% Canvas 程序化绘制，不使用任何原作名称、素材、UI 或文案。

```
laiyipao/
├─ server/     Go Fiber + PostgreSQL 后端
├─ miniapp/    uni-app（微信小程序 / H5 双端）
├─ admin/      Vue 3 运营后台 + 玩家玩法文档站
└─ scripts/    端到端验收脚本
```

> 注：设计文档已于 2026-10-14 迁出本目录（`docs/laiyipao/`）；
> 工程约束 / 已实现内容 / 已知边界三章已于 2026-10-20 迁至
> [`docs/laiyipao/ENGINEERING.md`](../docs/laiyipao/ENGINEERING.md) /
> [`IMPLEMENTATION.md`](../docs/laiyipao/IMPLEMENTATION.md) /
> [`KNOWN_LIMITS.md`](../docs/laiyipao/KNOWN_LIMITS.md)，
> 下方同名小节是转发锚，条目编号与拆分前一致。

---

## 快速开始

### 依赖

| 组件 | 版本 | 说明 |
|---|---|---|
| Go | 1.26.3 | 后端 |
| PostgreSQL | 15+ | 数据库，`trust` 认证即可 |
| Node | 20+ | 前端构建 |
| pnpm | 11 | 依赖管理 |

### 1. 起后端

```bash
cd laiyipao/server
export DATABASE_URL="postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable"
export BOOTSTRAP_ADMIN_PASS="admin12345"   # 少于 10 位会被拒绝引导

go run ./cmd/migrate   # 建表（19 个 goose 迁移，50 张业务表）
go run ./cmd/seed      # 写入游戏内容（首次必须执行）
go run ./cmd/api       # 监听 :8080
```

健康检查：`curl http://127.0.0.1:8080/readyz` → `{"ok":true,"db":"up"}`

### 2. 起小程序（H5 预览）

```bash
cd laiyipao/miniapp
pnpm install
pnpm dev:h5            # http://127.0.0.1:5174
pnpm dev:mp-weixin     # 产物在 dist/dev/mp-weixin，用微信开发者工具打开
```

API 地址在 `src/api/client.ts` 的 `BASE_URL`。真机调试需改成局域网 IP（H5 用了 `/api/v1` 相对路径时由 devServer 代理）。

### 3. 起运营后台

```bash
cd laiyipao/admin
pnpm install
pnpm dev               # http://127.0.0.1:5175
```

默认账号 `admin` / `admin12345`。

### 4. 端到端验收

```bash
cd laiyipao
pwsh -NoProfile -File scripts/e2e.ps1
```

> ⚠️ 脚本会**清空并重建** `laiyipao` 库。先停掉正在运行的服务端，否则 drop 会因连接占用失败。

覆盖 19 个步骤（`0`~`17`，含 `10b`）：重置数据、迁移+落库、游客登录、参考种子表、关卡内容（100 关 / 敌人 / 技能 / 反应契约）、钱包初始化、构筑评分、战斗凭证、资源守恒、**重放 422**、**超量击杀 422**、谎报星级被重算、未解锁关卡 403、签到/任务/商店/兑换码、专精每层 4 选 2、排行榜/防线/诊断、后台看板、**验真接口（I-6）**、**技能升级**、未验证上报 401。

### 5. 跑测试（含覆盖率）

在 `laiyipao/server` 目录下执行（目录进入方式见第 1 节）：

```bash
export TEST_DATABASE_URL="postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable"
go test ./... -timeout 1200s -coverprofile=coverage-go.out -covermode=atomic
go tool cover -func=coverage-go.out
```

`internal/httpapi` 的 e2e 与 `internal/service`/`store` 的集成用例按
`TEST_DATABASE_URL` → `DATABASE_URL` → 本机 `laiyipao` 库的顺序取 DSN。库必须已
`cmd/migrate` + `cmd/seed`；连不上时 e2e 会 **Skip 而不是失败**——报表只会显示
「几秒跑完、覆盖率很低」，看起来像测试写得浅，实际是一条都没跑。所以 CI 必须显式提供
可用的 DSN；Skip 数要在 `-v` 日志里人工复核（当前基线 2 条，均带显式理由）。
`internal/service` 全套实测 3–15 分钟（机器与并发差异大，
2026-10-28 单跑 179s，2026-10-27 夜跑 849s），默认 600s 在慢机上会假性超时，固定加
`-timeout 1200s`。

> 部署与值班请看 **[`docs/laiyipao/OPERATIONS.md`](../docs/laiyipao/OPERATIONS.md)** ——
> 环境变量全表、迁移回滚、备份恢复、故障处置，
> 以及「改关卡/公式会让存量战报永久失去可重放性」这件事的处理流程。

---

## 核心创新点

完整设计见 [`docs/laiyipao/GAME_DESIGN.md`](../docs/laiyipao/GAME_DESIGN.md)。

| 编号 | 创新点 | 一句话 |
|---|---|---|
| **I-1** | 元素反应矩阵 | 5 元素 × 7 反应链。**反应伤害中攻击力权重结构性 ≤30%** —— 堆金币永远无法替代正确搭配 |
| **I-2** | 插槽与热量 | 「三选一」不再是无脑增益。给选择装上代价，弃牌回热量，热量满则 2s 无法释放 |
| **I-3** | 专精树 + 装备契合度 | 每层 4 选 2 且需先点亮前层；装备同系 ×3、异系 ×1 反应加成，逼玩家成套配 |
| **I-4** | 地形机制关 | 5 类可交互地形（油桶/潮汐闸/旋转风障/崩塌掩体/蓄能塔），25% 的关卡使用 |
| **I-5** | 防线值守 | 构筑固化成快照，他人**本地模拟**挑战。服务端不跑战斗引擎 |
| **I-6** | 回放哈希验真 | 战斗完全确定性。任何人可取回种子重放，**证伪**可疑分数 |
| **I-7** | 构筑评分 | 主指标不是「战力 9999」，而是元素覆盖/反应链/专精/契合度五维 + 短板建议 |
| L-1 | 卡关诊断 | 连续失败时给出针对性建议（无元素覆盖 / 抗性不匹配 / 专精未投入…） |
| L-2 | 效率榜 | 按**通关时最低战力**排名，给低养成玩家留位置 |
| L-3 | 阵型预设 | 一键套用他人已验证的构筑 |

---
---

## 关键工程约束

**完整 19 条见 [`docs/laiyipao/ENGINEERING.md`](../docs/laiyipao/ENGINEERING.md)** ——
每一条都对应一个真实发生过的故障，改动时必须同时满足；新增一条的判据是
「它是否对应一个真实发生过、且没被已有约束覆盖的故障」。

> 引用约定：本仓代码注释与测试里的「README 工程约束 N」即该文档第 N 条
> （编号与拆分前的 README 完全一致，未重排）。概要：
> 1 反通胀是结构性保证 · 2 定点整数+确定性 PRNG · 3 攻方属性服务端下发 ·
> 4 跨端字段名显式约定且双向核对 · 5 防作弊校验必有负向测试 ·
> 6 双端公式一致性是可执行检查 · 7 动数值前先建基准 ·
> 8 「看起来实现了」需专门判据 · 9 形态/分布断言结构上测不出须专门构造 ·
> 10 采样会掩盖分布问题 · 11 先怀疑观测本身 · 12 养成接线必须改变战斗结果 ·
> 13 千分比字段确认加法语义 · 14 升级不让排行变差是可测不变式 ·
> 15 养成输入烘进战斗数值 · 16 replay_skills S 段可校验 ·
> 17 名单型守卫必须结构完备 · 18 第 96 关难度墙卡护甲 ·
> 19 不能做的操作不渲染成可点入口。

---

## 已实现内容

**实现规模、平衡实测、验证状态与测试方法论复盘见
[`docs/laiyipao/IMPLEMENTATION.md`](../docs/laiyipao/IMPLEMENTATION.md)**。

> 概要：内容规模（42 技能 / 100 关 / 21 敌人 / 96 专精节点 / 5 类地形）·
> 平衡基准与技能数梯度 · 攻方加成实测（堆攻击买不到分数却压制反应 25%）·
> I-4 地形真实生效率 · /config 版本内容哈希 + 304 ·
> 验证状态（Go/TS/vet/e2e/变异测试）· 后台首屏 168 KB（预算 220 KB）。
> 文档里的数字都有测量工具对着真源核，改完请重测。

---

## 已知边界（诚实声明）

**完整 29+ 条见 [`docs/laiyipao/KNOWN_LIMITS.md`](../docs/laiyipao/KNOWN_LIMITS.md)** ——
以下没有做，且不会假装做了。

> 引用约定：本仓注释里的「README 已知边界第 N 条」即该文档第 N 条。
> ⚠️ 编号是历史沉积的锚，**未重排**（存在重复编号与乱序，是有意的）；
> 新增条目只追加新编号。要点：支付/广告未接 · 微信登录未启用 ·
> 防线胜负客户端裁决 · 逐关调优未做 · 合成技能的获取（二期合成系统）·
> 三条被撤回的误判留档。

---

## 目录约定

```
server/internal/
├─ domain/     纯逻辑（伤害、元素、关卡生成、专精、战力）—— 与存储无关，可独立测试
├─ service/    业务编排
├─ httpapi/    路由与鉴权中间件
├─ store/      pgx 数据访问
└─ auth/       令牌签发与校验

miniapp/src/
├─ game/       纯 TS 内核（零绘制、零网络）—— 可在 Node 里无头跑
├─ render/     Canvas 2D 渲染层（只读引擎状态，绝不修改）
├─ api/        请求封装（401 自动刷新一次）
└─ store/      Pinia 全局状态
```

**职责边界**：`game/` 不 import 任何 `uni` API，因此同一份内核能同时跑在微信 canvas、H5 canvas 和 Node 测试里。

---

## 环境变量

| 变量 | 默认 | 说明 |
|---|---|---|
| `ADDR` | `:8080` | 监听地址 |
| `DATABASE_URL` | — | PG 连接串 |
| `TEST_DATABASE_URL` | — | 测试专用 DSN；优先于 `DATABASE_URL`，需指向已 migrate+seed 的 `laiyipao` 库 |
| `BOOTSTRAP_ADMIN_PASS` | `admin12345` | 首次引导的管理员密码，**<10 位会 panic 拒绝**；**`APP_ENV=prod` 时必须显式设置**，否则 panic（默认值是公开的，第 67 轮） |
| `WECHAT_ENABLED` | `false` | 是否启用微信登录 |
| `ACCESS_TTL` / `REFRESH_TTL` | `2h` / `720h` | 令牌有效期 |
| `DASHBOARD_CACHE_TTL` / `LEADERBOARD_CACHE_TTL` | `30s` / `15s` | 看板与榜单的进程内缓存有效期；**支持热加载**（见下） |
| `CONFIG_FILE` | — | 扁平 JSON 配置文件（键名与上表同名）。出现即覆盖同名环境变量；**`SIGHUP` 会重读它** |

### 配置热加载（2026-10-29）

进程改不了自己的环境变量，所以「重读配置」必须有一个**能重读的文件**来源 ——
只重读环境变量会得到与启动时逐字节相同的结果，热加载就成了表演。

```json
// /etc/laiyipao/config.json
{ "DASHBOARD_CACHE_TTL": "60s", "LEADERBOARD_CACHE_TTL": "30s", "SETTLE_RECOMPUTE_MODE": "shadow" }
```

```bash
CONFIG_FILE=/etc/laiyipao/config.json ./api          # 启动时读一次
kill -HUP $(pgrep -f '/api$')                        # 改完文件后重载
```

- 文件里的空字符串视为「未设置」，回退到环境变量。
- 覆盖层只接受标量；嵌套对象/数组会**启动失败**（猜一个序列化形式会让
  `CORS_ALLOW_ORIGINS` 静默变成「一个元素」）。
- 重载失败（文件不存在 / JSON 坏 / prod 校验不过）**保留原配置**并在 stderr 报错，
  不会打死正在服务的进程。
- **生效范围**：看板/榜单 TTL、结算重算模式、微信端点与凭据。
  **仍需重启**：数据库连接池、JWT 签名器、监听地址、路由级限流桶（构造期固化）。
- 结算重算模式只有在**文件里显式改了这一项**时才重建闸门；只改 TTL 的重载
  不会覆盖运维用 `PUT /admin/recomputes/mode` 切过的档。
