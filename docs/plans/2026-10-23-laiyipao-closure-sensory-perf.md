# 来一炮 · 闭环 / 体感 / 性能 三批落地 · 执行计划

> **日期**：2026-10-23
> **范围**：`laiyipao/` 三端（`server` Go/Fiber/PG、`miniapp` uni-app、`admin` Vue 3）
> **前序**：`2026-10-21-laiyipao-three-end-optimization.md`（两轮审计 55 + 33 条已销账）
> **状态**：执行中 —— 销账表在文末，未做完的显式记账，不冒充完成。

---

## 0. 为什么是这三批

前两轮审计消灭的是「**看起来实现了，其实没生效**」。本轮的前提判断是：
那类缺陷已经清完，**剩下的三类账性质不同**，必须分开做，不能混成一个「优化大礼包」：

| 批次 | 性质 | 判据形态 | 为什么不能混 |
|---|---|---|---|
| **P1 闭环** | 有写无读 / 有表无入口 | 「后台能发，玩家能看到」这类**端到端可达性** | 它是缺陷，不是优化；混进性能批会被当成"顺带做的" |
| **P2 体感** | 玩家能感知的缺失（声音 / 暂停 / 我的位置） | **行为断言**（tick 是否推进、AudioContext 是否创建） | 它改的是客户端交互路径，风险面与 SQL 完全无关 |
| **P3 性能** | 随数据量增长才暴露的债 | **可观测的读数**（索引存在、缓存命中、dry-run 行数） | 它不该动任何玩家可见行为，混在一起会让回归无法归因 |

---

## 1. 基线（动手前实测，2026-10-23）

| 项 | 结果 |
|---|---|
| Go | `go1.26.10 windows/amd64` |
| Node | `v22.22.2` |
| PostgreSQL | **本机不可用** —— Docker 未运行；`initdb` 被安全策略拦截；`postgresql-x64-15` 服务数据目录缺失，启动失败 |
| 因此 | **DB 依赖用例在本机全部 `t.Skip`**。这是本轮最大的验证缺口，见 §5「验证状态」的显式记账 |

> ⚠️ 按 AGENTS.md #11：**「Skip 掉一半用例」与「全绿」必须区分**。
> 本轮所有 DB 类新用例都会在有 PG 的环境里才真正执行，本机只验证到
> 「编译通过 / 类型正确 / 无 DB 的纯逻辑用例绿」。

---

## 2. 范围

### 做

**P1 · 闭环缺口**

| 编号 | 条目 | 落点 |
|---|---|---|
| C1 | 公告玩家端读端点 + 首页展示 | `service/announcement.go`、`routes.go`、`api/client.ts`、`pages/index/index.vue` |
| C2 | 小程序分享（首页 / 排行 / 战斗结算） | 三个页面的 `onShareAppMessage` |
| C3 | `KNOWN_LIMITS` 第 29 条销账（合成系统实际已接通） | `docs/laiyipao/KNOWN_LIMITS.md` |

**P2 · 体感**

| 编号 | 条目 | 落点 |
|---|---|---|
| S1 | 程序化音效（确定性生成 WAV + 播放器 + 静音持久化） | `miniapp/scripts/gen-sfx.mjs`、`miniapp/src/audio/sfx.ts` |
| S2 | 战斗暂停 + `onHide` 自动暂停 | `pages/battle/battle.vue`、`render/canvas.ts`（`pause/resume`） |
| S3 | 排行榜「我的排名」 | `service/economy.go`、`routes.go`、`pages/rank/rank.vue` |

**P3 · 性能**

| 编号 | 条目 | 落点 |
|---|---|---|
| F1 | 补 `battle_records(created_at)` / `wallet_flows(created_at)` 索引 | `migrations/00019_*.sql` |
| F2 | 看板 30s TTL 缓存（可配置、可 flush） | `service/stats.go`、`service/ttlcache.go`、`config.go` |
| F3 | 榜单 15s TTL 缓存 | `service/economy.go`、`config.go` |
| F4 | 数据保留策略：`cmd/prune`（默认 dry-run） | `cmd/prune/main.go`、`OPERATIONS.md` |

### 不做（明确记账）

| 项 | 原因 |
|---|---|
| 微信支付 / 激励广告 | 需 AppID 与商户号（KNOWN_LIMITS 第 1 条） |
| 微信登录接客户端 | 需真凭证，`KNOWN_UNSERVED` 已豁免 |
| 邮件/补偿系统 | 需要新的数据模型与后台页，属独立项目；本轮只做「公告」这一条最小闭环 |
| 真机帧率测量 | 需真机；本轮只做**不依赖真机**的音效与暂停 |
| `burn_cloud` 火区 / 速度进分数 / 服务端跑引擎 | 改 `replay_hash` 或推翻既有产品取舍（前序计划 §2 已拍板） |

---

## 3. 逐条设计

### C1 · 公告：玩家端读端点

**症状（已核实）**：`routes.go` 只有 `adm.Get/POST /announcements`；`miniapp/src` 全仓 grep `announcement` = **0 命中**。
即「后台能发公告，玩家永远看不到」。这正是前序计划 §0 定义的 **C 口径（闭环缺口）**。

**设计**

- `Service.ListAnnouncements(ctx, limit)`：
  - `WHERE published = TRUE`，`ORDER BY published_at DESC NULLS LAST, id DESC`；
  - `limit` 夹在 `[1, 20]`，缺省 5（与 `AdminListAnnouncements` 的夹法同口径）；
  - 返回 `[]AnnouncementView{id,title,body,published_at}` —— **不含** `published` 字段：
    玩家侧只可能拿到已发布的，回传一个恒为 `true` 的布尔是噪声。
- 路由：`v1.Get("/announcements", s.announcements)`（公开，与 `/config`、`/leaderboard` 同级）。
  **不挂 `requireUser`**：公告是公开内容，挂鉴权会让「首屏没登录就看不到公告」。
- 客户端：`fetchAnnouncements()`；首页顶部渲染最多 3 条，正文超 3 行折叠。

**判据（可执行）**

1. `go test ./internal/service -run TestListAnnouncements` —— 未发布的**不出现**、已发布的按 `published_at` 倒序、`limit` 越界被夹住。
2. `go test ./internal/httpapi -run TestEveryRouteHasClientCaller` —— 新路由必须有客户端调用方（否则红）。
3. `pnpm vitest run src/pages/index/index.test.ts` —— 有公告时渲染标题，无公告时**不渲染空卡片**。

**风险**：`/announcements` 是公开端点，无鉴权 → 需确认它不泄漏未发布内容（由 `WHERE published` 保证）。

---

### C2 · 小程序分享

**症状**：全仓 0 处 `onShareAppMessage`。微信小程序的**唯一免费获客通道**缺失。

**设计**

| 页面 | 分享标题 | 路径 |
|---|---|---|
| `index` | `来一炮 · 塔防 + 元素反应` | `/pages/index/index` |
| `rank` | `我在{榜名}排第 N` | `/pages/rank/rank?type=<当前 tab>` |
| `battle`（结算后） | `第 {level} 关 {stars} 星 · {score} 分` | `/pages/index/index` |

**判据**：`miniapp/src/test/share.test.ts` —— 用 `triggerUniHook(vm,'onShareAppMessage')` 捕获返回对象，
断言 `title` 非空、`path` 以 `/pages/` 开头；战斗页在**未结算时**分享文案不得包含分数（否则是假数据）。

**为什么用行为断言而不是扫源码**：扫 `onShareAppMessage` 字面量只能证明"写了这个名字"，
证明不了"点分享时真的返回一个能用的对象"。本仓库在 `Math.cos` 那次已经吃过这个亏。

---

### C3 · 文档销账

`KNOWN_LIMITS` 第 29 条写「18 个合成技能无获取途径」，实际 `bag.vue:63` 已有熔炼、
`routes.go:116` 已有 `POST /me/skills/synthesize`。**订正而不是删除**：保留编号（外部引用），
改写为「已接通，并注明接通时的判据」。

---

### S1 · 程序化音效

**为什么不用 Web Audio 合成**：微信小程序**没有 `AudioContext`**，只有
`uni.createInnerAudioContext()`（需 `src` 指向本地/网络音频）。
若只在 H5 用 Web Audio，小程序端就是**静音**——那正是本仓库反复要消灭的
「看起来实现了，其实没生效」。

**设计（两端同构）**

1. `miniapp/scripts/gen-sfx.mjs` —— **确定性**合成 8 个 WAV（22050Hz / 16-bit / 单声道）：
   `shot`（射击）、`hit`（命中）、`reaction`（反应触发）、`kill`（击杀）、
   `leak`（漏怪）、`win`、`lose`、`tap`（UI）。
   纯正弦/噪声 + 指数包络，无随机数（或固定 LCG），**同样输入必得同样字节**。
   写入 `miniapp/src/static/sfx/*.wav`；`--check` 模式逐字节比对，过期则退出码 1。
2. `miniapp/src/audio/sfx.ts` ——
   - 每种音效一个 `InnerAudioContext` 池（复用，不每帧新建 —— 与 `perf_frame.test.ts`
     同一条纪律：每帧 native 分配是主要开销）；
   - `play(name)` 在**未解锁**（无用户手势）时直接返回，不排队（排队会导致首次点击后
     一次性炸出一串音）；
   - `setMuted(bool)` 持久化到 storage（键 `lyp_sfx_muted`），`init()` 读回；
   - 单例。**不提供「页面卸载即销毁」**：本游戏是自动战斗，玩家可能全程不碰屏幕，
     销毁后下一局要重新等手势解锁 → 直接静音。context 最多 8 个且复用，
     不随时间增长（详见 §5「与计划的偏差」）。
3. 接线：`battle.vue` 在 `handleEvents` 里按事件类型播放；`tap` 挂在主行动按钮。

**判据**

1. `node scripts/gen-sfx.mjs --check` 退出 0（CI 可跑，无需真机）。
2. `sfx.test.ts`（stub `uni.createInnerAudioContext`）：
   - 未 `unlock()` 前 `play()` **不创建**任何 context；
   - `unlock()` 后同一音效播放两次**复用同一个** context（断言 `createInnerAudioContext` 调用次数 = 音效种类数，不是播放次数）；
   - `setMuted(true)` 后 `play()` 不触发 `play()` 方法调用；
   - `setMuted` 写入 storage，新建实例 `init()` 后仍是静音。
3. 体积：8 个 WAV 合计 < 120 KB，`pnpm size:check` 仍绿（主包预算 1536 KB）。

**风险**：iOS 上 `InnerAudioContext` 需在用户手势后播放 —— 由 `unlock()` 承担，
未解锁时**静默返回**而不是报错（报错会污染控制台且无法处理）。

---

### S2 · 战斗暂停 / `onHide`

**症状（已核实）**：`battle.vue` 只有 `onLoad` / `onUnload`，无 `onHide`；无暂停按钮。
逻辑时钟是 `setInterval`（`canvas.ts:startLogicClock`），**与页面可见性无关** ——
这是当初为了「后台也能跑逻辑」刻意做的。副作用是：玩家切到微信聊天再回来，
**战斗时间是否继续推进取决于平台对后台定时器的处置**（小程序挂起 JS、H5 标签页节流），
即「切后台会怎样」是**平台相关且未定义**的。

**设计**

- `canvas.ts`：加 `pause()` / `resume()`：
  - `pause()` 停逻辑时钟 + 停 rAF（`paused=true`，`running` 保持 true 语义区分开）；
  - `resume()` 重建两者；**不重置** `lastTs` 之外的任何状态。
- `battle.vue`：
  - HUD 加「暂停」按钮；`onHide` → `pause()` 并置 `pausedByHide=true`；
  - `onShow`：若 `pausedByHide` 则弹「已暂停」遮罩，**不自动恢复**（自动恢复会让
    玩家回到游戏时已经被打死一半，这是最糟的体感）；
  - `onUnload` 保持现有清理。
- **不改** `TICK_MS`、不改 tick 序列：暂停期间 `engine.tick` 不前进，
  `elapsedMs` 因此也不前进 → `duration_ms` 与 `replay_hash` **天然不受影响**。

**判据**（全部为行为断言，不需要真机）

1. `battle.test.ts`：触发 `onHide` 后推进 10 个逻辑间隔，`engine.tick` **不变**（暂停生效）。
2. 触发 `onShow` 后仍不自动推进；点「继续」后 `tick` 恢复推进。
3. `settleInput().duration_ms` 在「暂停 5 秒再结算」与「不暂停直接结算」两种路径下**相等**
   —— 这条是本条的核心：它证明暂停**没有**污染回放契约。
4. `canvas.test.ts`：`pause()` 后 `setSpeed()` 不重建时钟（防止暂停期间换挡留下野定时器）。

---

### S3 · 排行榜「我的排名」

**症状（已核实）**：`economy.go:Leaderboard` 只返回前 N 条；`rank.vue` 的 `me` 高亮
**只在前 50 内有效**。未进前 50 的玩家在榜上找不到自己，也不知道总共有多少人。

**设计**

- 新端点 `GET /me/rank?type=power|stage|efficiency`（挂 `requireUser` 组）。
  **不复用** `/leaderboard`：那个端点是公开的、无用户身份；为它加可选鉴权会引入
  「有没有带 token 结果不同」的隐式分支，比多一个端点更难推理。
- 返回 `{rank, total, score, kind}`；`rank = 0` 表示「无有效成绩」（例如从未通关时的关卡榜）。
- SQL 与 `/leaderboard` **同口径**：用 `COUNT(*)` 数「严格优于我的行数」+1。
  ⚠️ 效率榜是 `min_power_clear` 升序，其余两个是降序 —— 排序方向写反会得到
  「所有人并列第 1」这种静默错误，必须用 `TestMyRankMatchesLeaderboardOrder` 钉住。

**判据**

1. `TestMyRankMatchesLeaderboardOrder`：造 3 个用户，断言 `/me/rank` 的 `rank`
   与 `/leaderboard` 列表里的下标一致（三个榜各一次）。
2. `TestMyRankWithoutScore`：无成绩时 `rank=0` 且 `total` 仍是真实总数。
3. `rank.vue`：列表下方常驻「我的排名」条；`rank=0` 时显示「暂无成绩」而不是「第 0 名」。

---

### F1 · 补 created_at 索引

**症状（已核实）**：`battle_records` 只有 `(user_id, created_at DESC)` 与 `(level_id, created_at DESC)`；
`wallet_flows` 只有 `(user_id, created_at DESC)` 与 `(reason, created_at DESC)`。
而看板的 DAU 与「今日新增」按 `created_at` 做**范围扫描**，走不到这两个复合索引的首列。

**设计**：`migrations/00019_dashboard_indexes.sql`
```sql
CREATE INDEX IF NOT EXISTS idx_battle_records_created ON battle_records (created_at, user_id);
CREATE INDEX IF NOT EXISTS idx_wallet_flows_created ON wallet_flows (created_at);
```
- `battle_records` 用 `(created_at, user_id)` 而不是单列：DAU 是
  `COUNT(DISTINCT user_id)`，两列同在一个索引里可以走 index-only scan。
- 回滚文件 `00019_*_rollback.sql`（本仓惯例：`_rollback.sql` 不被运行器自动应用）。

**判据**：`TestDashboardIndexesExist` 查 `pg_indexes`，断言两个索引名存在且列序正确。
（无 DB 时 Skip —— 这是本轮唯一无法在本机验证的判据，见 §5。）

**为什么索引名带 `dashboard_` 前缀的取舍**：不采用。索引名按「表_列」命名，
与既有 35 个索引同风格；用 `dashboard_` 会让人误以为它只服务看板，
而它同时服务「今日新增用户 / 今日战报」等多处。

---

### F2 · 看板缓存

**症状**：`AdminDashboard` 串 9 段查询，其中 5 段是全表 `COUNT(*)`/`AVG`，
DAU 是 14 桶范围扫描，漏斗是全表 `GROUP BY level_id`。**全站唯一缓存是 `/config` 静态快照**，
Dashboard 每次进页面全跑一遍。运营看板对 30 秒延迟不敏感。

**设计**

- `service/ttlcache.go`：泛型 `ttlCache[T]`，`get(now, ttl)` / `put` / `flush`，
  内部 `sync.Mutex`；`ttl <= 0` 时**恒不命中**（等价关闭）。
- `Service` 加 `dashboardCache ttlCache[Dashboard]`；`AdminDashboard` 先查缓存，
  命中直接返回（**不查库**）。
- 配置：`DASHBOARD_CACHE_TTL`（默认 `30s`，`0` 关闭）。
- 公开 `FlushStatsCache()` 供测试与「运营点刷新」用（本轮不接后台按钮，避免扩大范围）。

**判据**

1. `ttlcache_test.go`（纯逻辑，**无 DB 也能跑**）：TTL 内命中、过期不命中、`ttl<=0` 恒不命中、`flush` 后不命中。
2. `TestDashboardServedFromCache`：第一次 `AdminDashboard` 成功后**把 `users` 表改名**，
   第二次调用仍成功 → 证明第二次**没有查库**（查了必然报错）。
   ⚠️ 这条同时证明了「缓存会让运营看到 30 秒前的数据」——这是**有意的取舍**，
   写进函数注释，不藏着。
3. 回归：`TestAdminDashboardSectionFailures` 仍须绿（每个 Service 实例冷缓存，改表后首次调用必报错）。

---

### F3 · 榜单缓存

**症状（已核实）**：`economy.go:100-118` 三个榜都是 `level_stars JOIN users` 全表
`GROUP BY` + `ORDER BY ... LIMIT 50`，玩家每进一次排行页算一次。

**设计**：`leaderboardCache map[string]ttlCache[[]LeaderboardEntry]`（按 kind 分键），
TTL `LEADERBOARD_CACHE_TTL` 默认 `15s`。

**判据**：`TestLeaderboardCached`（改名 `level_stars` 后第二次调用仍成功）。
⚠️ 与 F2 不同：**`/me/rank` 不进缓存** —— 它是"我的"，15 秒陈旧会让玩家刚打完一局
看到旧名次，那是最容易被当成 bug 的陈旧。

---

### F4 · 数据保留策略

**症状（已核实）**：`battle_records` / `wallet_flows` / `replay_verifications` /
`admin_audit_logs` 无限增长；`deploy/` 与 `scripts/` 里**无任何** retention / archive / VACUUM 相关脚本。

**设计**：`cmd/prune`：
- 默认 **dry-run**：只打印每张表「按 `--days` 会删多少行」，**不删**。
- `--apply` 才执行，且**必须**同时给 `--days`（无默认值）—— 防止「顺手跑一下」清空生产。
- 只清四张表，且各自有硬下限：`--days` 不得小于 7（少于此值直接拒绝）。
- 删除走 `DELETE ... WHERE created_at < now() - interval`，分批（`LIMIT`）避免长事务锁表。
- `wallet_flows` **不删**：它是经济审计的原始凭证，删了就无法追账。
  改为在文档里给出「归档到冷表」的手工步骤。
- `battle_records` 删除会破坏旧战报的 `/battle/:id/replay`（回放需要 token 与战报），
  文档里必须写清这条代价。

**判据**

1. `cmd/prune` 的参数解析单测（无 DB）：无 `--days` 时退出码 2；`--days=3` 退出码 2；
   默认 dry-run 时 `apply=false`。
2. 文档：`OPERATIONS.md` 新增「数据保留」小节，含四张表的保留建议与**删除代价**。

---

## 3.5 审计复核（写完后逐条回查代码，2026-10-23）

审计口径：**不把"我以为的"当判据**。每条都要在代码里找到对应事实，找不到就改设计。

| 条目 | 复核结论 | 处置 |
|---|---|---|
| C1 公告 | `announcements` 表有 `published`/`published_at`（`00006_economy.sql:123`）；`v1` 组无鉴权中间件（`routes.go:81-82` 同级）；`TestEveryRouteHasClientCaller` 会剥掉 `/api/v1` 前缀再匹配（`route_consumption_test.go:213`），客户端写 `'/announcements'` 可命中 | 原设计成立 |
| C2 分享 | `src/test/uni-app-stub.ts` **已导出** `onShareAppMessage` 与 `triggerUniHook` → 行为断言可行 | 原设计成立 |
| S1 音效 | ⚠️ **`miniapp/src/static` 目录不存在**；且 `manifest.json` 的 H5 `router.base = "./"` | **修正设计**：新建 `src/static/sfx/`；路径**运行时**拼（见下） |
| S1 音效 | ⚠️ `vitest.config.ts` **不加载** `@dcloudio/vite-plugin-uni` → `// #ifdef H5` 条件编译在测试里**不被处理**，两分支会同时生效导致重复声明 | **禁止使用条件编译宏**，改用运行时判断 |
| S2 暂停 | `canvas.ts` 逻辑时钟是 `setInterval`，与可见性无关；`elapsedMs` 按 tick 累加 → 停 tick 即停时长 | 原设计成立，且**天然不污染 `replay_hash`** |
| S3 我的排名 | `Leaderboard` 返回 `[]LeaderboardEntry`，无 total；`rank.vue` 的 `me` 类只在前 50 内有效 | 原设计成立 |
| F1 索引 | 确认 `battle_records` / `wallet_flows` **均无** `created_at` 单列或首列索引（35 个索引逐个核过） | 原设计成立 |
| F2 缓存 | 逐个核对会二次读看板的用例：`TestAdminDashboard`（1 次）、`TestAdminDashboardStageFunnel`（1 次）、`TestAdminDashboardSectionFailures`（每个子用例新建 Service，冷缓存）→ **per-Service 缓存不会让任何既有用例变绿** | 原设计成立；另加"改名后仍成功"证明命中 |
| F2 缓存 | `scripts/e2e.ps1:269` 只调一次看板 | 不影响 e2e |
| F4 保留 | `replay_verifications` / `admin_audit_logs` 均有 `created_at`；`wallet_flows` 决定**不删**（经济审计凭证） | 原设计成立，范围由 4 张表收窄为 3 张 |

**S1 的路径修正（审计后定稿）**

```ts
// 不用 #ifdef：vitest 不跑 uni 预处理器，两分支会同时生效。
// 也不用裸 '/static/...'：H5 的 base 是 "./"，部署在子路径时会 404。
const BASE = (import.meta as { env?: { BASE_URL?: string } }).env?.BASE_URL ?? '/'
const PREFIX = BASE === './' || BASE === '' ? '/' : BASE
// → 小程序得 '/static/sfx/x.wav'（包根绝对路径），H5 dev 得 '/static/...'
```

**新增验证项（原计划漏了）**：`pnpm build:mp-weixin` 后必须确认
`dist/build/mp-weixin/static/sfx/*.wav` **真的存在** —— 否则"音效写完了但包里没有"
就是本条最典型的失效方式（源码里 import 一个不存在的资源，vite 不会报错）。

---

## 4. 执行纪律

1. 每条**先写守卫测试**，再改实现（AGENTS.md #8）。
2. 每批结束跑：`gofmt -l .`（无输出）、`go vet ./...`、`go build ./...`、
   `go test ./... -count=1`、`pnpm type-check` + `pnpm test`（两端）、`pnpm size:check`。
3. 任何一条跳过/无法验证，**显式记进 §5**，不算完成（AGENTS.md #11）。
4. 新增路由必须同步客户端调用方，否则 `TestEveryRouteHasClientCaller` 红。
5. 新增 `cmd` 必须在 `docs/laiyipao/system/` 有对应说明，否则运维不知道它存在。

---

## 5. 验证状态

### 本机环境限制（必须先说）

| 项 | 状态 |
|---|---|
| PostgreSQL | **不可用**：Docker 未运行；`initdb` 被安全策略拦截；`postgresql-x64-15` 数据目录缺失 |
| 后果 | `internal/service`、`internal/httpapi`、`internal/store`、`internal/seeder`、`cmd/seed`、`cmd/migrate` **6 个包的 DB 用例失败或跳过** |
| 失败原因归类 | 逐条核对过：**全部**是 `建一次性库失败：dial tcp 127.0.0.1:5432: connectex: No connection could be made`，**没有一条**是逻辑失败 |

> 这不是"测试全绿"。按 AGENTS.md #11 记账：**8 个包绿、6 个包因无 PG 红**，
> 其中 C1 / S3 / F1 / F2 / F3 的 SQL 与集成判据**未经运行时验证**。
> 它们已写好，会在有 PG 的环境（CI / 本地 docker）第一次执行。

### 最终门禁（2026-10-23 实测）

| 端 | 门禁 | 结果 |
|---|---|---|
| server | `go build ./...` | ✅ |
| server | `go vet ./...` | ✅ 无输出 |
| server | `gofmt -l .` | ✅ 无输出 |
| server | `go run ./cmd/tygen -check` | ✅ `types.gen.ts is up to date` |
| server | `go test ./...` | **8 包通过**（config / auth / domain / tygen / testutil/scratchdb / cmd/api / cmd/vectors / cmd/prune）· **6 包红，全部因无 PG** |
| server | `cmd/prune` 参数单测 | ✅ 8 条（不需要 DB） |
| miniapp | `vue-tsc --noEmit`（含 `tsconfig.vitest.json`） | ✅ 0 error |
| miniapp | `vitest run` | **78 文件 / 1130 通过 / 2 跳过**（跳过的是 `i6.e2e.test.ts`，需运行中的服务端） |
| miniapp | `build:mp-weixin` | ✅ 且 `dist/build/mp-weixin/static/sfx/*.wav` **8 个文件都在** |
| miniapp | `build:h5` | ✅ 且 `dist/build/h5/static/sfx/*.wav` 在 |
| miniapp | `size:check` | ✅ 主包 **329 KB**（预算 1536 KB；音效占 65.5 KB） |
| miniapp | `node scripts/gen-sfx.mjs --check` | ✅ 音效与生成脚本一致 |
| admin | `vue-tsc --noEmit` + `tsc -p tsconfig.vitest.json` | ✅ 0 error |
| admin | `vitest run` | ✅ **321 通过**（本轮未改后台，回归确认） |
| admin | `size:check` | ✅ 首屏 **173 KB**（预算 220 KB） |
| 端到端 | `scripts/e2e.ps1`（19 步） | ⛔ **未跑**：需要运行中的服务端 + PG |

### 被文档守卫抓到的两处漂移（本轮真实收益）

加迁移 00019 之后，`internal/domain` 的两条**既有**守卫当场变红 ——
这正是它们存在的意义：

| 守卫 | 报错 |
|---|---|
| `TestReadmeMigrationCountMatchesEmbed` | 「README 写 18 个 goose 迁移，实际 19 个」 |
| `TestOpsDocMatchesReality` | 「运维手册漏了 2 个环境变量：`DASHBOARD_CACHE_TTL`, `LEADERBOARD_CACHE_TTL`」 |

两处已修，并顺带同步了 `docs/laiyipao/system/` 里三处「18 个迁移」的引用
（那三处没有守卫，是人工找的）。

### 销账表

| 批次 | 条目 | 状态 | 验证 |
|---|---|---|---|
| P1 | C1 公告玩家端 | ✅ 完成 | service 3 条 + httpapi 形状 1 条（本地 Skip）+ 首页 3 条（绿） |
| P1 | C2 分享 | ✅ 完成 | `share.ts` 纯函数 + 排行 2 条 + 战斗 3 条（绿） |
| P1 | C3 文档销账 | ✅ 完成 | `KNOWN_LIMITS` 第 29 条订正；新增第 30 条记账本轮不做项 |
| P2 | S1 程序化音效 | ✅ 完成 | sfx 9 条 + battle-sfx 5 条 + 接线 2 条；`gen-sfx --check` 绿；两端产物已确认 |
| P2 | S2 暂停 / onHide | ✅ 完成 | canvas 4 条 + battle 4 条；含 `elapsedMs === tick × TICK_MS` 不变式 |
| P2 | S3 我的排名 | ✅ 完成 | service 5 条（本地 Skip）+ 排行 5 条（绿） |
| P3 | F1 created_at 索引 | ✅ 完成 | 迁移 00019；`TestDashboardIndexesExist`（本地 Skip） |
| P3 | F2 看板缓存 | ✅ 完成 | `ttlcache` 6 条（**绿，不需 DB**）+ 集成 4 条（本地 Skip） |
| P3 | F3 榜单缓存 | ✅ 完成 | 同上 + 名单对拍 1 条（绿） |
| P3 | F4 保留策略 | ✅ 完成 | `cmd/prune` 8 条（绿）+ `OPERATIONS.md` §6.4 |

### 与计划的偏差（如实记录）

| 计划写的 | 实际做的 | 原因 |
|---|---|---|
| S1 在页面卸载时 `destroySfx()` | **不做** | 销毁后新战斗需要重新解锁音频，而玩家可能全程不碰屏幕（自动战斗）——那会让下一局静音。context 是模块级单例（最多 8 个），留着不构成泄漏 |
| F3 榜单缓存按 `(kind, limit)` 分键 | 按 `kind` 分键，查询固定取 200 行后切片 | 一个榜只有一个缓存条目，键更简单；多取 150 行的代价可忽略 |
| S1 计划里写「8 个音效 < 120 KB」 | 实测 **65.5 KB** | — |
| 计划外新增 | `Service.FlushStatsCache()` / `LeaderboardCacheKinds()` / `triggerUniHook` 返回值 | 前两个是测试与运维需要的最小口子；第三个让分享断言变成**行为**断言而不是源码扫描 |

---

## 6. 独立审计与修复（2026-10-23 晚）

代码全部落地、5 个提交推送之后，做了一轮**独立审计**：两个 agent 分别对抗性审查
服务端与小程序端，口径是「找出**看起来完成、其实没生效**的地方」，每条必须给
文件:行号 + 症状 + 判据失效点。审计报 28 条，逐条复核后确认 **8 条真缺陷 + 6 条测试缺口**，
其余 14 条是重复或已在文档记账。

### 6.1 真缺陷（已修）

| # | 缺陷 | 症状 | 修法 |
|---|---|---|---|
| A1 | **`MyRank` 与 `Leaderboard` 并列口径不一致** | 同分两人在榜上是第 1/第 2，而 `/me/rank` 给两人都算第 1。玩家看到「我的排名 第 1」而榜上自己高亮在第 2 条。效率榜（`MIN(min_power_clear)`）并列极常见 | 名次判据从「只比分数」改成**双键**（分数 + `user_id`），与 `ORDER BY score, u.id` 逐字对应 |
| A2 | **第一版的测试把错误口径固定成了期望** | `TestMyRankTiesCountOnce` 断言「同分两人都是第 1」——谁按计划把实现改对，它反而变红 | 改为 `TestMyRankTieBreaksByUserID`：断言 a=1、b=2，且与列表第 1/2 条逐条对齐 |
| A3 | **榜单缓存与 `/me/rank` 不一致** | 列表 15 秒陈旧、名次实时，同一屏两个数字来自不同快照 | `SettleBattle` 成功后 `flushLeaderboards()`（只榜单，不动看板）。玩家打完 → 打开排行页 → 两者都新 |
| A4 | **`sliceLeaderboard` 泄漏 cap** | `all[:limit]` 的 cap 仍是 200，调用方 `append` 会写进缓存底层数组 —— 榜单内容随机变化且无报错 | 三索引切片 `all[:limit:limit]`，把口头契约变成结构约束；加 `cap(got)==limit` 断言 |
| A5 | **看板缓存测试的断言恒真** | `if d.Users.Total < 0`（`COUNT(*)` 永不为负）—— 「缓存返回快照而不是零值」这半条完全没验 | 先造一个用户，断言 `second.Total == first.Total && first.Total >= 1`，并加 `len(DailyActive)==14` |
| A6 | **公告 limit 越界退回缺省而不是夹到上限** | `?limit=100` 返回 5 条：比请求少、也比上限少，且不报错。原断言 `len <= max` 区分不出 | 拆成两段式（`<=0` 回缺省、`>max` 夹到 max），断言**恰好等于 20** |
| A7 | **结算在途时分享拼出「0 星 0 分」** | `settled` 在 `settle()` 第一行置位，`settleResult` 要等接口返回才赋值。弱网数秒内转发，卡片是「第 3 关 0 星 · 0 分」 | 谓词改成 `!!settleResult.value && !settleResult.value.error` |
| A8 | **公告请求无在途去重，且失败会清空已展示内容** | 首次请求在途时再 `onShow` 会并发第二次；一次抖动把公告抹掉（看起来像运营撤了公告） | 加 `annInflight` 标志；失败分支保留旧值 |

另外 4 条小修：

- `MyRank` 未知 kind 现在**规范化**为 `power` 再回显（与 `routes.go`「响应的 type 必须与数据同一件事」同一条纪律）。
- `pausedByHide` 从 `let` 改成 `ref`（模板依赖它；非响应式时「手动暂停后又切后台」的提示不会更新）。
- `onHide` 的守卫从 `engine.value` 改成 `!settled && !startError`：前者会漏掉 `eng.start()` 到 `r.start()` 之间那次 `await getCanvas()` 的窗口；并在 `r.start()` 后补一次 `if (paused) r.pause()`。
- 音效资源路径 `'./'` 不再归一成 `'/'`：H5 部署在子路径时那样会 404。抽成 `assetPrefixFor(base)` 并对三种 base 各断言一次。

### 6.2 测试缺口（已补）

| 缺口 | 为什么重要 | 补法 |
|---|---|---|
| `share.ts` 纯函数零单测（计划点名 `src/test/share.test.ts` 但没建） | 删掉首页 `onShareAppMessage` 整行、或让 `indexShare()` 返回空 path，**1130 条测试全绿**，而微信卡片点进去是空白页 | 新建 `utils/share.test.ts` 8 条 + 首页分享断言 1 条 |
| `unlockSfx` 接线零覆盖 | 删掉 `stage.vue` 那行，`playSfx` 永远在第一个 `if` 返回 —— 无报错、无视觉变化、全绿，线上整局静音 | `stage.test.ts` 加「开始战斗后 `isSfxUnlocked()` 为 true」 |
| `pause()` 只测了逻辑时钟那半 | 删掉 `cancelAnimationFrame` / `clearInterval(drawTimer)`，4 条既有暂停用例全绿，而画面冻住却仍在每帧重绘 | 假 rAF：`pause` 断言 `cancelAnimationFrame` 被调、`resume` 断言重新注册 |
| 暂停中换挡是否真的生效 | 既有断言只要求「步数变多」，用旧档位也成立 | spy `setInterval`，断言 `resume` 后间隔 == `logicIntervalMs(4)` |
| fallback（无 canvas）路径的暂停零覆盖 | 删掉 `doResume` 里的 `startFallbackLoop()`，页面永远停在「已暂停」遮罩上 | 加一条 `mockQueryNode(null)` 的暂停/恢复用例 |
| `ttlCache` 无并发用例 | 去掉 `mu.Lock` 时 `-race` 也抓不到（没有用例并发进入临界区） | 1 写 4 读 + `flush`，靠 `-race` 兜底 |
| `prune` 退出码只测到 `parseOpts` | 把 `os.Exit(2)` 改成 1，测试全绿而脚本 `$? -eq 2` 静默失效 | `main` 拆成 `runCLI(args, out, err) int`，直接断言返回码；并断言「参数校验先于连库」 |
| 深链测试抓不到「两次请求」 | `toHaveBeenCalledWith('stage')` 在「先按 power 拉一次、再补 stage」时照样绿 | 加 `toHaveBeenCalledTimes(1)` |

### 6.3 无法单测覆盖的一处（显式记账）

**战斗页根节点的 `@click="unlockSfx()"`**（音效解锁的第二道保险）。

- 它在 uni 的构建产物里是 `bindtap="{{Q}}"`（已用
  `dist/build/mp-weixin/pages/battle/battle.wxml` 核对），**真实环境有效**；
- 但 jsdom 下 `<view>` 是自定义元素，VTU 的 `trigger('click')` 触发不到它 ——
  试过 `trigger('touchstart')`、`trigger('click')`、`dispatchEvent(new Event(...))`，
  元素上都没有 `_vei` 监听器。

所以这条**不写测试，也不假装测了**。主链路（`stage.vue` → `unlockSfx`）有单测，
兜底靠构建产物 + 真机。这与「用源码扫描冒充行为断言」是两回事：前者是承认测不到，
后者是假装测到了。

### 6.4 审计后的门禁

| 端 | 结果 |
|---|---|
| server `gofmt -l` / `go vet` / `go build` | 无输出 / 无输出 / ✅ |
| server `go test` | 9 包通过（含 `cmd/prune` 的 10 条、`internal/domain` 的文档守卫）；其余 6 包仍因**无 PG** 红，失败原因全部是连接失败 |
| miniapp `vitest` | **79 文件 / 1150 通过 / 2 跳过** |
| miniapp `vue-tsc` | 0 error |
| miniapp 构建 | `build:mp-weixin` ✅（音效路径实测 `/static/sfx/*.wav`）· `build:h5` ✅（实测 `./static/sfx/*.wav`，子路径部署正确） |
