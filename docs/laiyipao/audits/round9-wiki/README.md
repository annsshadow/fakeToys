# R9-WIKI 审计：admin Wiki 路由回退 dashboard（t89）

## 环境标注

- 截图环境：非物理真机；Windows + Chrome headless（agent-browser 0.41.1）；桌面视口 1258×588；本地回环 127.0.0.1:5176（admin dev）/ 127.0.0.1:8080（server）。
- 无真实域名 / IP / 口令入库；dev 默认凭证为仓库公开值。

## 缺陷与根因

- 现象：未登录访问 `#/wiki` → 登录成功 → 被送回 `#/admin/dashboard`，文档站打不开；已登录态从侧栏点文档链接则正常。
- 根因：`src/views/LoginView.vue` 的 `redirectTarget()` 只放行 `r.startsWith('/admin')`，把 `/wiki` 归入「壳外路径落看板」。而 `src/router/index.ts` 的 `AUTH_REQUIRED` 只匹配 `/admin`，无令牌访问 `/wiki` 时会写 `redirect=/wiki` 并跳登录页——登录成功后 redirect 被 redirectTarget 拒绝，回落 dashboard。
- 修复：`redirectTarget()` 放行 `/^/wiki(?:[?#]|$)/`（带 query / 深链一并保留），并显式拒绝 `/wiki.evil.com` 这类前缀伪装。

## test-first RED→GREEN

- RED：`tests/LoginView.test.ts` 新增 3 用例（/wiki redirect 跳回、带 query 深链保留 tab/focus、/wiki.evil.com 前缀伪装仍拒），失败 2 例（expected '/admin/dashboard' to be '/wiki'）。
- GREEN：`LoginView.vue` redirectTarget 判据修复 + 注释更新，LoginView/router/AdminLayout/commandPalette 共 4 files/59 tests 全绿。
- 眉题补做（§1B.2）：`tests/WikiView.test.ts` 新增 2 用例（内容区 section-kicker 中文眉题 + 英文小标；AI 四判据齐全）。RED 1 failed（眉题缺）→ GREEN：`WikiView.vue` 在 page-title 前插 `.section-kicker`（玩法知识库 / PLAYER GUIDE），WikiView 27/27 全绿。

## 真浏览器验证（修复后）

- 链路：清 localStorage 令牌 → 打开 `?v=t89clear#/wiki` → 跳 `#/admin/login?redirect=/wiki` → 填仓库公开默认凭证登录 → 落 `#/wiki`。
- 三重确认：URL `#/wiki`；`document.title` = 「玩家玩法文档站 · 来一炮 · 运营后台」；结构探针 `wiki=1, kicker=1, ai=1, kickerCN=玩法知识库, kickerEN=PLAYER GUIDE`。
- AI 四判据：可见汉字徽章「AI」+ 真实数据结论「当前配置版本 6237e0b6…反应条目 7 类」+ 依据行「累计战报 168count；累计胜场 131count…」+ 可点 CTA「查看内容 →」（跳 #/admin/content）。
- errors=[]（无 pageerror / console.error）。

## 截图

- `screenshots/after-01-wiki-fixed.png`：修复后 wiki 页，登录成功 + 眉题 + AI 卡 + 目录表格全貌。sha256 `89c2e041fdc4d8e15ff55659696f5eae49e9ac99c061086262539ae484ceaa1a`。
- 前态（缺陷现象）以根因调查中的代码级证据与本文「缺陷与根因」节为准（旧 5175 进程为旧代码，落 dashboard 已在调查阶段复现确认）。

## 门禁

- admin 全量 npm test：694 passed / 694（59 files）。
- vue-tsc --noEmit：EXIT 0。
- npm run build：EXIT 0（built 3.66s，charts chunk 545KB）。
- 未破坏：其余 11 视图 / 命令面板 10 项 / 顶栏 API 脱敏 / 推荐接口 / F-1 兜底 / 图表令牌化 / AI 洞察卡。
- 全程未执行 git；无真值入库。
