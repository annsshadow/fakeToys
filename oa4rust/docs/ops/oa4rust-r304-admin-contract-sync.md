# R304 管理区契约同步 A（t42）

## 背景

R304 基线契约审计（docs/ops/oa4rust-baseline-contract-mismatch-audit-r304.md）列出桌面 admin 目录 8 个失败用例，编号 9/10/14/15/16/17/18/19。经队长 §219 裁决逐条复核：其中 7 条为「断言写法过时、生产实现语义正确」的契约漂移，需做语义等价同步；1 条为真实死代码缺陷，需真修复。本轮只改契约断言与 1 处样式死代码，不改任何业务行为。

## 逐条裁决与改动

- 用例 9（ModernUI scoped）：ModernUI.vue 的 scoped 样式仅作用于组件私有类 .ai-b5-surface，属合法用法。断言由「禁止任何 scoped」改为「允许 scoped，但 scoped 块内不得定义 .modern-*/.glass-card/.admin-* 全局类」，保留 modern-page 根容器、默认插槽、.ai-b5-surface 真断言。
- 用例 10/14/16/19（h1 class 顺序）：AdminLayout.vue:38 现为 `<h1 class="admin-title ai-gradient-text">`，原断言 `/<h1[^>]*class="admin-title"/` 要求 class 恰好以 admin-title 结尾，与多类名实现冲突。四处统一放宽为 `/<h1[^>]*class="[^"]*admin-title/`，仍强制断言 h1 上确有 admin-title 语义类。
- 用例 17（admin-page-title）：AdminPageHeader 现以普通 class 承载标题，原正则 `/class="admin-page-title"/` 对引号边界过严。改为 `toContain('admin-page-title')`，语义等价。
- 用例 18（面包屑容器）：AppShell 面包屑已由 div 升级为 nav landmark（可访问性改进）。断言由 `/<div .../breadcrumb/` 改为 `/<(div|nav).../breadcrumb/`，语义等价。
- 用例 15（真修复）：AdminToolbar.vue `.admin-toolbar-btn.primary` 块内存在两处 color 声明，前一处 `color: #fff` 被后一处 `color: var(--bg-base)` 覆盖，属不可达死代码。删除该行，保留设计令牌 color: var(--bg-base)。这是本轮唯一的生产代码改动。

## 验证

- RED 基线：4 个契约文件 8 failed / 15 passed（23），8 失败正好对应审计 8 用例。
- GREEN：4 个契约文件 23 passed（23），exit 0。
- pnpm run typecheck：exit 0。
- pnpm run build：exit 0。
- 真浏览器：测试服 <YOUR_TEST_DOMAIN> 部署后，Playwright Chromium headless 真实页面冒烟，逐页截图，证据存 gitignored 测试产物目录，source 标注为非物理真机。

## 说明

本轮属契约同步与死代码清理，无新增业务功能、无接口变更。测试服地址、凭据、SSH 端口等环境真值仅从部署环境注入的 .local.keys / 环境变量读取，不入库。
