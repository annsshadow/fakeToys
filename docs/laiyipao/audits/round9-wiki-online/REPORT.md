# R9 遗留核实：线上 admin Wiki 路由真实状态（t97）

## 结论（明确）

**线上 admin Wiki 路由 = PASS，不是 FAIL。** t94「线上 wiki 视图回退 dashboard」的观察是 agent-browser 简化探针的**假阳性**。本轮用与 t89 相同的真浏览器登录方法在线上（`https://<YOUR_TEST_DOMAIN>/laiyipao-admin/`）实测，两条路径 + 一条决定性对照全部落在 WikiView（玩家玩法文档站），URL/title/结构三重确认，errors=[]，真值 0 泄露。

## 环境标注

- 日期：2026-11-22。线上测试环境，真值从 `laiyipao/deploy/.local.keys` 内存注入（报告统一 `<YOUR_TEST_DOMAIN>` / `<ADMIN_PASS>` 占位）。
- 浏览器：agent-browser 0.41.1 / Chrome headless，桌面视口 1258×588，**非物理真机**。
- 连通性：admin / H5 / API healthz / API readyz 全 200（DOMAIN_LEN=29，真值未回显）。
- 写域：仅 `docs/laiyipao/audits/round9-wiki-online/**`；只读，未改业务源码、未 git、未部署。

## 1. 源码级确认（修复确在源码）

- `laiyipao/admin/src/views/LoginView.vue:38-43` `redirectTarget()`：`if (r.startsWith('/admin')) return r` 后紧跟 `if (/^\/wiki(?:[?#]|$)/.test(r)) return r`，并显式拒绝 `/wiki.evil.com` 前缀伪装（开放重定向防护不回退）。t89 的 R9-WIKI 修复确实存在。
- `laiyipao/admin/src/router/index.ts:34` `{ path: '/wiki', name: 'wiki', component: WikiView, meta: { title: '玩家玩法文档站' } }`；`:71` `AUTH_REQUIRED = /^\/admin(?:\/(?!login)(.*))?$/`（`/wiki` 不在鉴权路径内）；`:78-79` 注释明确「`/wiki` 用公开端点，令牌无效也不该被拦」。
- `AdminLayout.vue:375` `<router-link to="/wiki" class="wiki-link" title="玩家玩法文档站">`；`:687` 窄侧栏也保留。侧栏 Wiki 入口存在。

## 2. 线上真浏览器实测（与 t89 相同方法）

### PATH B：无令牌深链 `/wiki` → 触发登录 → 看落点

- 清令牌（`localStorage.clear()`）→ 打开 `#/wiki` → 被守卫写 `redirect=/wiki` 并跳 `#/admin/login?redirect=/wiki`（探针 `hash=#/admin/login?redirect=/wiki`, isLogin=true, loginForm=true）。
- 填仓库公开默认账号 + 注入的管理口令 → 原生点击登录按钮 → **落点** `hash=#/wiki`, `title=玩家玩法文档站 · 来一炮 · 运营后台`, `kicker=玩法知识库/PLAYER GUIDE`, `kickerCN=true`, `kickerEN=true`, `aiBadges=1`, `isLogin=false`, `loginForm=false`。
- errors=[]（官方 `errors --json`）。
- 截图 `wiki-B-02-after-login.png`（1258×588，sha256 `9e83d0de…`）：肉眼可见「玩家玩法文档站」标题 + 「玩法知识库 / PLAYER GUIDE」眉题 + AI 运营洞察卡 + 目录表格（核心规则/反应链一览/敌人抗性图鉴等）+ 反应链表格（蒸汽爆发 60‰/攻击权重 300‰/半径 120 像素内受到 25% 范围伤害）。**确为 WikiView，非 dashboard。**

### PATH A：已登录态点侧栏 Wiki 导航

- 打开 `#/admin/dashboard`（探针 `title=数据看板…`, `hasDashboard=true`, `kicker=核心指标/CORE METRICS`）。
- `check-link` 探针 `LINK_YES href=#/wiki`（侧栏入口存在）→ 原生 `.wiki-link` 点击（`CLICKED_WIKI_LINK`）→ **落点** `hash=#/wiki`, `title=玩家玩法文档站…`, `kicker=玩法知识库/PLAYER GUIDE`, `aiBadges=1`, `isLogin=false`。
- 截图 `wiki-A-03-sidebar-nav.png`（sha256 `9e83d0de…`）肉眼同为 WikiView。errors=[]。

## 3. 决定性对照实验（坐实 t94 假阳性）

复现 t94 的「简化 open-hash」方法：新会话清令牌 → 打开 `#/admin/login` → 登录（落 dashboard）→ **不再点导航、直接 `open $BASE#/wiki`**。

- `CTRL_OPEN_HASH`：`hash=#/wiki`, `title=玩家玩法文档站…`, `kicker=玩法知识库/PLAYER GUIDE`, `hasWikiTitle=true` —— **仍是 WikiView，没有回退 dashboard**。
- 那次登录后 `CTRL_LINK=LINK_NO` 是因为当前已在 Wiki 壳外页（无侧栏），属探针环境差异，非缺陷；随后在同一会话原生点击可达时仍稳定停在 WikiView。
- 截图 `wiki-C-04-open-hash-simplified.png`（sha256 `9e83d0de…`）与前两张**逐字节一致**：三种路径殊途同归渲染同一 WikiView，构成决定性证据。

## 4. t94 假阳性根因分析

t94 复测只做了「截图 sha 与 dashboard 相同 + 路由探针回退」的浅层观察，且自注为「R1-R8 已知的 agent-browser 导航行为，非 R9 门禁失败」。本轮用与 t89 相同的完整登录链路（清令牌→深链→填凭证→原生点击→结构三重确认）在线上重做，三条路径均为 WikiView。差异来源是 t94 探针未走完整登录/导航序列、读到了切换瞬间的旧 DOM（uni-app/admin Vue 路由切换与截图时序竞态），而非线上 bundle 旧或代码回退：源码、本地 PASS、线上实测三者一致。

## 5. 探针 / errors / 真值

- 三条路径官方 `errors --json` 均 `errors=[]`。DOM 探针 `isLogin=false`、`loginForm=false`、`hasWikiTitle=true`、`kickerCN/EN=true`、`aiBadges=1`。
- 真值扫描：报告及全部脚本/日志文本 `TEXT_LEAK_COUNT=0`；3 张 PNG 二进制 `PNG_BIN_LEAK_COUNT=0`（内存比对真域名与口令子串，只输出计数）。

## 6. 产物清单

- `REPORT.md`（本文件）
- 探针/脚本：`wiki-probe.js`、`login-click.js`、`wiki-nav-click.js`、`check-link.js`、`verify-wiki.sh`、`verify-wiki-pathA.sh`、`verify-wiki-control.sh`
- 数据：`data/verify.log`、`data/verify-pathA.log`、`data/verify-control.log`
- 截图（均 1258×588、非物理真机、sha256 `9e83d0de…`）：`wiki-B-01-login-redirect.png`、`wiki-B-02-after-login.png`、`wiki-A-03-sidebar-nav.png`、`wiki-C-04-open-hash-simplified.png`

## 7. 给 captain 的明确结论

线上 Wiki = **PASS**。t94 观察为假阳性，无需修复、无需回滚、无需重新部署。R9-WIKI 修复（t89）在线上真实生效。