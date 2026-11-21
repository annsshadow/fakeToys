# R333-A AppInfoApp.vue 默认插槽致入口缺失修复（方案 A：具名插槽）

## 1. 背景

R332 只读甄别（文档 `oa4rust-r332-default-slot-audit.md`，commit 948b0fb03）坐实 P1 真缺陷：PC 应用管理页 `AppInfoApp.vue` 把 6 个工具栏按钮和每行 5 个行操作按钮写在 `AdminToolbar` / `AdminTable` 的默认插槽中，而 R298 原语组件只渲染具名插槽（`AdminToolbar`: `#filters`/`#actions`，`AdminTable`: `cell-*`），Vue 静默丢弃默认插槽内容，页面 `pageerror=0` 但全部写入口和行操作在真 DOM 中缺失。

## 2. 方案 A（fail-loud，写入文档）

只改 `AppInfoApp.vue`（单文件方案 A），将入口全部迁移到具名插槽，组件本身不改：
- 删除自造 `<input class="search-input">`，使用 `AdminToolbar` 自带搜索框（`v-model="keyword"` + `@search="doSearch"` + `@refresh="doSearch"`）；
- 搜索按钮进 `<template #filters>`，其余 5 个按钮（新建应用/管理视图/发布/受控栏目/类型管理）进 `<template #actions>`，全部 `@click` 与文案原样保留，一个不删；
- `AdminTable` 只新增 `<template #cell-actions="{ row: item }">` 放行 4 个 `btn-act2`（权限/设权限/建文件/图标，均 `@click.stop`）+ 1 个 `btn-del`（删除）；`name`/`appFlag`/`appType` 三列不加 `cell-*`，让组件回退渲染 `row[col.key]`；
- `ListLoadError` / loading / 空态由 `AdminTable` 内置接管；所有 `load*` 方法、列定义 `:columns="appInfoColumns"`、`:loading`/`:error`/`empty-text` 绑定不变；
- `AdminToolbar.vue` / `AdminTable.vue`（R298，14 页复用）、SDK、后端、`HotpicApp.vue`、`DocumentApp.vue` 全部 outOfScope。

## 3. RED → GREEN

- 契约测试 `AppInfoApp.slots.contract.test.ts`（`@vitest-environment node`，静态源码模板断言，不引入 `@vue/test-utils`；所有自定义函数参数带 `:string` 类型标注，避免 strict tsconfig 的 TS7006）：
  - RED：`pnpm exec vitest run <契约文件>` = 5 failed / 1 passed（默认插槽仍裸放按钮、缺 `#cell-actions`、缺 4 btn-act2）。
  - GREEN：6 passed / 6 passed。
- 全量：`cd oa4rust && pnpm exec vitest run` = 270 files / 2176 tests 全绿，exit 0；`cd oa4rust && pnpm --filter @oa4rust/desktop typecheck` exit 0。

## 4. 部署与真浏览器实测

- 前端 `pnpm build` exit 0，新入口 hash `index-DVL-dQiF.js`（CSS 沿用 `index-BMFkaD73.css`）；测试服单 web 目录串行原子替换：`tar` → `scp` → 远端 `web.incoming-<STAMP>` → `mv web web.old-<STAMP>` → `mv web.incoming-<STAMP> web`，hash 校验通过（`WEB_ATOMIC_SWAP_OK` / `DEPLOY_R333_APPINFO_HASH_OK`）。静态资源替换无需重启后端。
- 真浏览器复验：`1440x900` 无头 Chromium（**本地无头浏览器，非物理真机**），凭据经 SSH 在运行期从测试服读取（不入库、不回显）；证据落 gitignored 目录 `oa4rust/test-results/r333-appinfo-slots-fix/`（2 PNG + `evidence.json`）。
- 结论：`verdict=APPINFO_SLOTS_VERIFIED`。工具栏 7 按钮文本 `搜索 | + 新建应用 | 管理/视图/类型 | 发布/全部视图/含文档 | 受控栏目/带流程/视图数据 | 类型管理/含文档类型/输出 | 刷新` 全部可见；组件自带搜索框 1；表格 1 行，行操作 `btn-act2=4` + `btn-del=1`，操作列文本 `权限设权限建文件图标删除`（不再为 `—`）；点「管理/视图/类型」触发只读入口 `GET /api/appinfo/list/manage` 等 4 个 GET 全 200；`pageerror=0`、API 失败 0。
- 取证限制：破坏性写（新建/删除应用）本轮未授权，未执行真 UI 写闭环，仅验证入口可见性与只读入口 200。

## 5. 脱敏

文档仅使用 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位符；未写入真实域名、IP、SSH 端口、口令或私人路径。凭据仅在复验脚本运行期经 SSH 从测试服读取，不进入仓库。

## 6. 已知边界

本任务只修 `AppInfoApp.vue` 的入口可见性；其他页面的同类默认插槽问题（如 R332 甄别中提及的页面）不在本任务范围，按队长分派另案处理。`HotpicApp.vue` 由并行任务另修，其提交已在 HEAD，本提交不触碰。

## 7. 证据索引

- 契约：`oa4rust/frontend/apps/desktop/src/views/AppInfoApp.slots.contract.test.ts`
- 实现：`oa4rust/frontend/apps/desktop/src/views/AppInfoApp.vue`
- 真浏览器：`oa4rust/test-results/r333-appinfo-slots-fix/evidence.json`（gitignored，非物理真机）
- 结论：R333-A 方案 A 落地，真 UI 入口全部恢复，0 pageerror，验收通过；项目 ≥1000 轮持续迭代，非终态。
