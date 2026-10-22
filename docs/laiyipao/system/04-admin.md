# 04 · 运营后台（admin）

> Vue 3.5 + TypeScript 5.9 + Vite 8（Rolldown）+ Element Plus 2.14（按需引入）
> + ECharts 6 + vue-router 4（hash 模式）。
>
> 后台只有一个职责：**把服务端的管理端点组织成运营能用的界面**。
> 它不直连数据库、不持有业务规则、不做二次计算。

---

## 1. 目录结构

```
admin/
├─ index.html            入口 + CSP meta（含详细注释，改之前必读）
├─ vite.config.ts        base / 代理 / 按需引入 / manualChunks
├─ scripts/size-report.mjs  首屏体积测量（raw / gzip + 逐路由）
├─ src/
│  ├─ main.ts            应用装配
│  ├─ App.vue            根组件
│  ├─ router/index.ts    路由表 + 令牌守卫
│  ├─ layouts/AdminLayout.vue   侧边栏 / 顶栏 / 登出
│  ├─ api/client.ts      fetch 封装、令牌、401 处理、令牌验真缓存
│  ├─ api/index.ts       全部管理端点 + 类型定义（与 routes.go 一一对应）
│  ├─ api/types.gen.ts   由 `go run ./cmd/tygen` 从 Go 响应结构生成（`-check` 守漂移）
│  ├─ views/             12 个视图
│  ├─ reactions.ts       反应 key → 中文名（看板与文档站共用）
│  ├─ utils/format.ts    数字/时间格式化
│  ├─ utils/adminRole.ts 角色可写判定（与服务端 requireWritable 同源）
│  ├─ styles/main.css
│  └─ components.d.ts    由 unplugin-vue-components 生成
└─ tests/                27 个 Vitest 用例（含体积与 CSP 守卫）
```

---

## 2. 路由与视图

路由表与侧边栏**必须同步维护**，否则会出现「有路由没菜单」或「有菜单点 404」。

| 路径 | 视图 | 标题 | 职责 |
|---|---|---|---|
| `/admin/login` | `LoginView` | 登录 | 用户名/密码 → `POST /admin/login` |
| `/admin/dashboard` | `DashboardView` | 数据看板 | 用户/战斗/进度/经济/验真/反应分布/DAU/漏斗 |
| `/admin/levels` | `LevelsView` | 关卡配置 | 100 关列表、章节筛选、编辑、波次查看、重生成 |
| `/admin/skills` | `SkillsView` | 技能与配方 | 42 技能 + 18 配方，编辑数值 |
| `/admin/equipment` | `EquipmentView` | 装备宝石皮肤 | 三个目录表只读展示 |
| `/admin/users` | `UsersView` | 用户管理 | 搜索、封禁/解封、发币/回收 |
| `/admin/battles` | `BattlesView` | 战斗记录与验真 | 战报列表、守恒列、验真发起、详情 |
| `/admin/recomputes` | `RecomputesView` | 结算重算审计 | 台账列表、按关卡分布、闸门状态 |
| `/admin/defenses` | `DefensesView` | 防线值守 | 防线列表（战力/胜负/护盾） |
| `/admin/economy` | `EconomyView` | 经济与商城 | 货币流水、商品改价/上下架 |
| `/admin/content` | `ContentView` | 公告与兑换码 | 公告创建、兑换码创建 |
| `/wiki` | `WikiView` | 玩家玩法文档站 | 读 `/config` + `/admin/reactions` 渲染玩法说明 |

`/wiki` 在后台壳**之外**（全宽展示），但仍在路由守卫下：
`/config` 是公开端点，而 `/admin/dashboard` 需要令牌，混在一起会出现 401 报错。

侧边栏分组（`AdminLayout.vue`）：

| 分组 | 菜单 |
|---|---|
| 运营概览 | 数据看板 |
| 游戏内容 | 关卡配置 / 技能与配方 / 装备宝石皮肤 |
| 玩家与战斗 | 用户管理 / 战斗记录与验真 / 结算重算审计 / 防线值守 |
| 商业化与运营 | 经济与商城 / 公告与兑换码 |

> 「结算重算审计」刻意紧跟「战斗记录」：两者回答同一个问题的两层
> —— 战报页看「这一局长什么样」，重算页看「重算认为它对不对、拒没拒」。

---

## 3. API 层

### 3.1 客户端（`api/client.ts`）

```ts
export const API_BASE = import.meta.env.VITE_API_BASE || '/api/v1'
const TOKEN_KEY = 'lyp_admin_token'
```

- 令牌存 `localStorage`（隐私模式下退化为内存态）。
- 非 2xx 一律抛 `ApiError(status, code, message)`，形状与 `httpapi.APIError` 对齐。
- **401 统一处理**：清令牌 + 跳登录页（`handleUnauthorized`）。
- **令牌验真缓存**（`confirmTokenValid`）：进入后台壳时验一次 `/admin/me`，
  结果只在**内存**缓存。不能持久化 —— 那等于把「已验过」写进磁盘，
  令牌过期后缓存反而成了绕过校验的后门。

为什么需要验真：「localStorage 里有字符串」不等于「令牌有效」。
只检查真值时，控制台执行 `localStorage.setItem('lyp_admin_token','x')`
即可进入全部 11 个视图，然后看到一整页 401 —— 恰恰是守卫本想避免的体验。

`confirmTokenValid` 的 `catch` 是**宽**的：连网络错误也算验真失败。
理由是「后端重启期间被弹回登录页」远好于「在一个全是错误的壳里找原因」。

### 3.2 端点封装（`api/index.ts`）

与 `server/internal/httpapi/routes.go` **一一对应**，每条都带类型定义。
响应类型的权威副本是 `api/types.gen.ts` —— 由 `go run ./cmd/tygen` 从 Go
结构生成，CI 跑 `-check`：改了 Go 结构忘了重新生成会是**红构建**而不是静默漂移。
几条必须知道的字段语义（都在源码注释里）：

| 字段 | 语义 | 误读后果 |
|---|---|---|
| `AdminUser.verify_checked` / `verify_mismatched` | `0/0` = **从没被验真过**（未知），不是「干净」 | 把「没验过」当成「没问题」 |
| `AdminBattle.total_enemies` | 该关总怪数（生成器权威值），供「守恒」列判断 | 无法发现击杀数伪造 |
| `AdminRecompute.battle_id === 0` | **没有战报**（S 段拒绝发生在写战报之前），不是「战报 #0」 | 去查一条不存在的战报 |
| `AdminRecompute.recomputed === null` | 引擎未接入 / 没算，不是「重算成 0 分」 | 把「没算」画成「0 分」（最强可疑信号） |
| `AdminRecompute.mode` vs `configured_mode` | 实际生效 vs 配置值，二者不同说明被熔断降级 | 误判「配置成 enforce 就一定在拒发」 |
| `AdminLevel.clear_rate` | **已是百分数**（`clears*100/attempts`） | 再乘 100 显示成 10000% |
| `AdminLevel.player_authority` | `generator` = 玩家侧由生成器产出，本页编辑只是规划记录 | 以为改完就生效 |
| `VerifyResult.seed` | 字符串（int64 可能超 2^53） | 用 number 会失精，重放必不符 |

---

## 4. 关键视图说明

### 4.1 数据看板

数据来自 `GET /admin/dashboard`，一次返回：

```
users{total,guests,wechat,banned,new_today}
battles{total,today,wins,avg_duration_ms}
progression{avg_max_stage,avg_power,stage_100_clears}
economy{coin_in,coin_out,gem_in,gem_out,orders_paid,orders_pending}
verification{checked,matched,mismatched}
reaction_usage[{reaction,count}]
daily_active[{date,dau}]
stage_funnel[{level_id,attempts,clears}]
```

图表用 ECharts（`BarChart/LineChart/RadarChart` + `CanvasRenderer`），
按需注册组件，不整包引入。

`reaction_usage` 的 key 是英文（`steam_burst`…），中文名来自
`GET /admin/reactions`（对应 `domain.AllReactionSpecs()`）。
**不复用 `/config`**：那个响应 167 KB，为了 7 个名字不值得。

### 4.2 关卡配置

- 列表：`GET /admin/levels?chapter=&keyword=&limit=&offset=`（筛选与分页都会传到服务端；`limit=0` 表示一次拉全部）。
- 编辑：`PUT /admin/levels/:id`，只提交改动字段。
- 波次：`GET /admin/levels/:id/waves`（只读）。
- 重生成：`POST /admin/levels/regenerate`（重跑生成器并落库）。
  ⚠️ 请求体必须带 `confirm: "regenerate"`（P3-6）—— 它一次覆盖全部 100 关且不可回滚，
  不能只靠前端确认框。

**必须理解的语义**（`DECISIONS D1`）：

> 玩家侧的关卡权威是**生成器**，不是 `levels` 表。
> 因此本页的编辑只影响运营规划与统计，不影响线上玩家。
> 响应里的 `player_authority="generator"` 与 `applied_to_players=false`
> 就是这件事的机器可读表达，界面必须显示出来。

### 4.3 战斗记录与验真

- 列表：`GET /admin/battles?user_id=&level_id=&only_mismatched=1&limit=&offset=`
- 详情：`GET /admin/battles/:id`（P2-1 起返回战报详情本身，不再是重放信息）
- 验真：`POST /admin/battles/:id/verify {replay_hash}`

**验真端点不重算哈希** —— 服务端没有引擎参与这条路径。
调用方需要自己在能跑引擎的地方用种子重放，把算出的 hash 贴进来。
它防的是「改了数据却没改凭证」，不是「从一开始就伪造」。

运营侧验真会同时写 `admin_audit_logs`（`battle_verify`），
因为这是一次管理动作，审计轨迹要独立于统计表存在。

### 4.4 结算重算审计

两个端点成对：

| 端点 | 用途 |
|---|---|
| `GET /admin/recomputes?user_id=&level_id=&only_mismatch=1` | 单条台账 |
| `GET /admin/recomputes/stats?hours=24` | 总账 + 按关卡分布 + 闸门状态 |
| `GET /admin/recomputes/mode` | 读当前档位（P3-3） |
| `PUT /admin/recomputes/mode` | 切档（P3-3）：`{mode, confirm:"confirm"}`，写审计；`readonly` 一律 403 |

`stats` 的 `engine` 段是**唯一能回答「校验到底有没有在跑」**的读数：

```
verified / mismatch / cannot_run / skipped / settled_total / verified_permille / status
```

它与 `totals` 口径不同，不能混用：`totals` 只含不符的局（台账只记不符），
所以「100% 通过」与「根本没跑」在 `totals` 上都是 0 行。
只看 `totals` 会把「校验没跑」误判成「没问题」，而那是最坏的误判。

`demoted=true` 表示 `enforce` 已被熔断降级 —— 此时**本该拒发的奖励正在放行**，
界面必须显式标红。

### 4.5 用户管理

- 搜索：`GET /admin/users?keyword=&limit=&offset=`
- 详情：`GET /admin/users/:id`（P5-7：构筑、钱包、最近战报、验真明细）
- 封禁/解封：`POST /admin/users/:id/ban|unban`
- 发币/回收：`POST /admin/users/:id/grant {currency, amount}`

`amount` 为负即回收，审计事件名会变成 `revoke_currency`。
`currency` 可以是四种货币，也可以是三种道具（`revive_token` / `mastery_reset` /
`gem_wash_token`），服务端有显式白名单。

### 4.6 经济与商城

- `GET /admin/economy`：货币流水 + 商品列表
- `PUT /admin/shop/:id`：改价 / 改 payload / 上下架

服务端对 `price` / `payload` 的 key 做白名单校验：必须是货币名或道具名，
否则返回「未知货币」。这条校验是刻意的摩擦 —— 打错字应当报错，
而不是静默发放一个没人看得见的道具。

### 4.7 玩法文档站（`/wiki`）

读 `GET /config` 与 `GET /admin/reactions`，因此技能数值、抗性、反应链说明
**永远与线上版本一致**。这是它放在后台里的唯一理由：
不会出现「文档写的和游戏里不一样」这种最伤玩家信任的问题。

它也是跨端字段命名的一个活体检查点：`reactions` / `chapters` 两组曾写成
PascalCase，而服务端一直下发 snake_case，于是反应表整列为空、
章节下拉框没有选项，且不报任何错。

---

## 5. 权限模型与界面的关系

服务端三档角色（`admin` / `ops` / `readonly`）由 `requireWritable` 强制。
**后台界面不按角色隐藏任何按钮** —— 这是当前的已知行为：

- 好处：界面逻辑简单，不存在「前端隐藏了但后端没拦」的假安全感；
- 代价：`readonly` 账号能看见全部写操作，点了才 403。

因此 403 的提示必须清晰（服务端返回 `role_not_writable` + 当前角色），
否则运营会以为「系统坏了」而不是「我没有权限」。

登出对 `readonly` 也放行：会话生命周期不是数据写。

---

## 6. 构建与体积

### 6.1 关键配置（`vite.config.ts`）

| 配置 | 值 | 原因 |
|---|---|---|
| `base` | `/laiyipao-admin/` | 部署在子路径；缺省 `/` 会让资源打到 `<host>/assets/…` → 404 白屏 |
| `plugins` 顺序 | `vue()` **在** `Components()` **之前** | Components 靠分析编译后的 render 函数收集组件，顺序反了会「构建成功但组件全丢」 |
| `Components` | `ElementPlusResolver({ importStyle: 'css' })` | 按需引入，只打进模板真正用到的 `el-*` |
| `manualChunks` | 函数形式（Vite 8 = Rolldown） | 对象映射在 Rolldown 类型里已不支持 |
| `manualChunks` 分组 | `charts`（echarts）/ `vue`（vue/router/pinia/@vue） | **不得**把 `element-plus` 圈成一组，那会绕过按需裁剪 |

### 6.2 体积

| 指标 | 值 |
|---|---|
| 改动前（整包注册） | 1981 KB，element-plus 单 chunk 901 KB（45.5%） |
| 改动后（按需） | 1101 KB（−44.4%），element-plus 286 KB（15 chunk，−68.2%） |
| 首屏预算 | raw **220 KB** / gzip **80 KB**（实测 161 / 62） |

两个守卫分工不同：

- `tests/bundle_size.test.ts`（秒级）：源码层检查是否又出现
  `app.use(ElementPlus)` 或把 element-plus 圈进 manualChunks。
  **必须先剥注释再匹配** —— 否则会匹配到自己写的解释性注释，
  于是「把代码改对」反而判失败。
- `scripts/size-report.mjs`（需先 build）：量首屏真实代价。
  它从 `index.html` 沿**静态 import** 求闭包算首屏，再逐路由算懒加载代价。
  用 `dist` 总量会把 echarts 的 534 KB 算成「占 48%」，而它只在打开看板时才下载。

---

## 7. 安全

`index.html` 里的 CSP meta：

```
default-src 'self';
script-src 'self';
style-src 'self' 'unsafe-inline';
img-src 'self' data: blob:;
font-src 'self' data:;
connect-src 'self' https:
```

三条必须知道的事实（都写在 meta 的注释里）：

1. `connect-src 'self'` 已覆盖 Vite HMR（同端口 WebSocket）。不要为「HMR 连不上」
   加 `ws:`，先检查端口。
2. `style-src 'unsafe-inline'` 是**必需**的：Vite 开发模式用
   `document.createElement('style')` 注入样式。
3. `frame-ancestors` **不能**用 `<meta>` 下发（CSP3 明确移除）。
   点击劫持这条唯一「不需要服务端漏洞」的路径必须由**部署侧**补
   `X-Frame-Options: DENY`（服务端已全局下发，见 02 §8）。

---

## 8. 测试

27 个用例，按职责分四类：

| 类型 | 代表 | 说明 |
|---|---|---|
| 视图行为 | `DashboardView` / `LevelsView` / `BattlesView` / `RecomputesView`… | 挂载 + mock API + 断言渲染与交互 |
| API 契约 | `api-index` / `api-client` | 路径、方法、请求体形状 |
| 守卫 | `bundle_size` / `csp_meta` / `readme_structure` | 结构性回归 |
| 布局 | `AdminLayout` / `AdminLayout.logout` | 导航、登出 |

`tests/helpers.ts` 提供统一挂载与 Element Plus 注册；
`tests/setup.ts` 注册全局 stub。

---

## 9. 常见陷阱

| 陷阱 | 症状 | 正确做法 |
|---|---|---|
| 忘记改 `base` | 部署到子路径后白屏 | 保持 `/laiyipao-admin/` |
| `Components()` 排在 `vue()` 前 | 构建成功但组件全丢 | 顺序不能反 |
| 把 element-plus 圈进 manualChunks | 体积回到 900 KB | 交给打包器自行分块 |
| 用字符串做图标名 | 侧边栏图标全空 | 传组件引用（写错即编译错误） |
| 把 `verify_checked=0` 当干净 | 漏掉可疑账号 | 两个字段一起看 |
| 把 `battle_id=0` 当战报 0 | 去查不存在的战报 | 显示成「—」 |
| 把 `recomputed=null` 当 0 分 | 误判最强可疑信号 | 显示成「未重算」 |
| `clear_rate` 再乘 100 | 显示 10000% | 服务端已是百分数 |
| 只验「有令牌」 | 无效令牌进壳 → 满页 401 | `confirmTokenValid()` |
| 持久化令牌验真结果 | 过期后成为后门 | 只存内存 |
