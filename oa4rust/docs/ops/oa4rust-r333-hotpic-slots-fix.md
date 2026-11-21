# R333-B PC 热帖管理默认插槽入口缺失修复

## 1. 范围与环境

- 轮次：R333-B，任务 t134；性质：真缺陷修复（方案 A，具名插槽改造）。
- 对象：PC 端 `HotpicApp.vue`（路由 /app/admin/hotpic），修复 R332/t132 审计 §4.2 坐实的 P1 缺陷。
- 环境：测试服 <YOUR_TEST_HOST>，本地 Playwright Chromium headless，视口 1440x900。**截图与 evidence 均为本地无头浏览器实测，非物理真机。**
- 凭据：运行期经 gitignored 脚本与 ssh 通道读取，未入库、未回显；域名/主机统一用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位。
- 写域：仅 `HotpicApp.vue`、`HotpicApp.slots.contract.test.ts`、本文；不动 R298 组件 AdminToolbar/AdminTable、SDK、后端、AppInfoApp（t133）、DocumentApp（R330 已修）。

## 2. 根因与方案 A（fail-loud）

- 根因：`AdminToolbar.vue` 仅消费 `#filters`/`#actions` 具名插槽 + 内置刷新/createLabel，`AdminTable.vue` 仅消费 `cell-*` 具名插槽并回退 `row[col.key]`，两者**均无默认 \<slot/>**。写在标签对之间的默认插槽内容被 Vue 静默丢弃，编译期与运行期均无告警，pageerror=0。
- 审计实测（R332 §4.2）：21 个热图工具栏入口 + 行删除全在真 DOM 缺失，`toolbarBtn=1`（仅内置刷新），btnDel=0，表头「操作」列 td 为 —，列表接口 GET /api/hotpic/core/entity/list = 200。
- **方案 A（队长裁决，照 R330 DocumentApp 范式）**：① 搜索 input 与「搜索」按钮移入 `<template #filters>`，保留 `v-model="keyword"`/`@keyup.enter="doSearch"`，AdminToolbar 自带 `v-model`/@search/@refresh 不变；② 其余 19 个业务按钮全部移入 `<template #actions>`，一个不删，保留全部文案与 @click；③ AdminTable 只加 `<template #cell-actions="{ row: item }">` 放行行删除按钮（保留 `@click.stop="onDelete(item)"`）；标题/应用/状态列不加 cell-* 插槽，让组件回退 `row[col.key]` 保持原展示。
- **fail-loud**：契约测试断言 AdminToolbar/AdminTable 默认插槽内不再有裸 `<button`/`class="search-input"`，回退即刻 RED，防止缺陷复发。
- 不采用方案 B（在组件内补默认 `<slot/>`）：改动面覆盖所有使用方，回归成本高，且超出本任务写域。

## 3. 修复内容（HotpicApp.vue 模板，仅此文件源码改动）

- :15-18 `#filters`：搜索 input（保留 v-model/@keyup.enter）+ 搜索按钮。
- :19-39 `#actions`：热图/面板、热图2/面板2/应用2、深度读、孪生端点、建热图、改标题、存配置、建用户热图、删用户热图、清BBS密文、清CMS密文、建实体热图、删实体热图、存在校验、按应用热图、按ID热图、密文热图列表、用户热图列表、删用户热图2（共 19 个 btn-primary）。
- :43-45 `#cell-actions`：`btn-del` 删除按钮，@click.stop="onDelete(item)" 不变；原 item-grid 内的卡片删除按钮亦保持不动（展示层兼容）。
- **未动**：ListLoadError 的 banner/empty 分支、loading 骨架、空态、所有 load*/hotpicWrite/hotpicMore/doSearch/onDelete 方法、columns 定义、<script> 与 <style>。

## 4. RED→GREEN 台账（静态）

- 契约：`HotpicApp.slots.contract.test.ts`，5 it，读源码模板静态断言（不引入 @vue/test-utils、无新增依赖）；所有自定义函数参数带 `: string` 显式类型标注（遵 R330 TS7006 教训）。
- RED（修复前）：`pnpm exec vitest run frontend/apps/desktop/src/views/HotpicApp.slots.contract.test.ts` exit=1，4 failed / 1 passed——缺 #filters/#actions/#cell-actions、默认插槽仍有 search-input/裸按钮。
- GREEN（修复后）：定点 5/5 passed。
- 全量：`cd oa4rust && pnpm exec vitest run` exit=0，269 files / 2170 tests 0 failed。
- 类型：`cd oa4rust && pnpm --filter @oa4rust/desktop typecheck` exit=0（tsc --noEmit）。

## 5. 真浏览器部署复验（待队长放行）

- 硬时序：测试服单 web 目录，与 t133（AppInfoApp）串行 build + 原子替换；队长放行后执行，本文提交时**尚未放行，故本节不声称真浏览器结果**。
- 待验断言（1440x900 真浏览器）：21 个工具栏入口 + 行删除 btn-del 全部可见；表头「操作」列 td 不再为 —；0 pageerror；点只读入口（如热图/面板）返回 200。
- 证据落 gitignored 目录 `test-results/r333-hotpic-slots-fix/`（截图 + evidence.json，source 标注非物理真机），完成后以 append-only evidence_note 补记，不回改本文提交。

## 6. 取证限制与合规

- 破坏性写（建热图/删热图/清密文/删除等）本轮未点击执行，遵守未授权不写原则，端点写入行为不在结论内。
- 未声称项目完成：≥1000 轮持续迭代，本轮仅为其中一环。

## 7. 结论

- 方案 A 静态修复完成，契约与全量测试/类型检查转 GREEN；真浏览器复验待放行后据实补证。
- 与 R329 DocumentApp（已修）、t133 AppInfoApp（待修）合并看，系统管理页群插槽契约错配按同一范式逐页治理。
