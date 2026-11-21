# R330 PC 文档中心 DocumentApp.vue 默认插槽修复

- 任务：t127（R330）｜分支：opt1000/oa4rust-pc-ai-refactor
- 截图/证据来源：本地无头 Chromium 渲染，非物理真机

## §1 背景

R329 t126 只读深度审计（oa4rust/docs/ops/oa4rust-r329-pc-document-audit.md，commit 6545b6e39）坐实 P1 真缺陷：DocumentApp.vue 把工具栏 8 个按钮与每行行操作按钮写在 AdminToolbar/AdminTable 的默认插槽内，而两组件（R298 复用原语，14 页共用）只渲染具名插槽（#filters/#actions、cell-*），没有默认 <slot/>。Vue 对未声明的默认插槽内容静默丢弃，导致 pageerror=0 但所有写入口（新建/发布/删除/字段/视图/探针）全部消失，文档中心实质不可用。

## §2 裁决方案 A（fail-loud）

队长裁决方案 A：只改 DocumentApp.vue 用具名插槽承载全部内容；AdminToolbar.vue / AdminTable.vue（14 页复用）列入 outOfScope，不为本页单独改通用组件。要求：①搜索用 AdminToolbar 自带 v-model（已绑 keyword）、@search 保留；②工具栏 8 按钮（搜索/新建/字段批量状态/管理视图/孪生A/孪生B/深度C/密文列表）与 doc-meta-note 全部进 #actions（搜索放 #filters），一个不删；③表格四列 #cell-title/#cell-status/#cell-createdAt/#cell-actions，标题 item.title||item.name||未命名、statusLabel+docStatusCls、创建时间、每行 14 btn-act + 1 btn-del 及全部 @click/:disabled 保留；④ListLoadError/loading/空态由 AdminTable 内置接管，:loading/:error 仍绑定；⑤tab/新建弹窗/load* 方法/loadDocTwin3 探针语义不动。

fail-loud 说明：本轮明确不改通用组件；其他页面若存在同类默认插槽误用，另立任务处理，不在本轮静默顺手修。删除后 manager 列表残留（§6）经队长裁决确认为后端独立缺陷，剥离为 R331，不在本轮越界修后端/SDK。

## §3 test-first RED→GREEN

契约测试 oa4rust/frontend/apps/desktop/src/views/DocumentApp.slots.contract.test.ts（6 条，静态源码断言，无新增依赖）：RED=5 failed / 1 passed（断言 #actions/#cell-* 缺失、默认插槽内仍裸放按钮）→ GREEN=6/6 passed。全量 vitest 268 files / 2165 tests，0 failed。

DocumentApp.vue 改造：AdminToolbar 默认插槽拆 #filters（搜索按钮）+ #actions（新建/字段批量状态/管理视图/孪生A/孪生B/深度C/密文列表/doc-meta-note），删除自造 input 改用组件自带 v-model；AdminTable 默认插槽拆四个 #cell-* 作用域插槽，cell-actions 保留 14 btn-act + 1 btn-del 及全部 @click/:disabled；script 加 void ListLoadError 保持原导入语义。

## §4 真浏览器实测（本地无头 Chromium，非物理真机）

部署：pnpm build 产物 js=index-DE68PPne.js / css=index-BMFkaD73.css，经 web.incoming 原子替换部署，hash 校验通过（WEB_ATOMIC_SWAP_OK / DEPLOY_R330_DOCUMENT_HASH_OK）。t124（同 web 目录，Personal.vue 修复）已先行提交并 t125 终审通过，无并发。

1440x900 真浏览器断言：btnCreate=1、每行 btnAct=14+btnDel=1、toolbarHTML 含「+ 新建文档」、行操作列含「发布」「删除」、0 API 失败 / 0 pageerror——R329 P1「写入口全缺失」已修复。

闭环前三步走通：真 UI 新建 STAMP 文档 POST /api/document 200 → 列表回显（tr.admin-table-row 含 STAMP）→ 点「发布」PUT /api/document/publish/{id} 200 → 刷新后仍在列表（状态已发布）。

## §5 脱敏声明

本文档及证据不含真实域名/IP/口令/私钥/个人路径；base 与凭据仅运行期从环境/本地工具读取，不入库；截图标注来源为本地无头浏览器渲染、非物理真机。占位符 <YOUR_TEST_DOMAIN>/<YOUR_TEST_HOST>。

## §6 已知边界（独立缺陷，队长裁决剥离为 R331）

删除后列表不消失：真 UI 点「删除」→ 自定义确认框确认 → DELETE /api/document/{id} 200，后端 document_u2_delete（cms_assemble_control/src/lib.rs:8772-8795）走 soft_delete_by_id 软删，数据库 deleted_at 已置值（诊断坐实 2026-11-22 02:52:34）；但 manager 列表 document_filter_paging_manager_u3（lib.rs:11606-11618，route routes.rs:645 /api/document/filter/list/{page}/size/{size}/manager）查询条件仅 ORDER BY create_time DESC LIMIT 200，缺 deleted_at IS NULL，而其余三变体（:11570/:11584/:11598）都带，导致刷新后仍返回该软删文档。

队长裁决：t127 前端职责（含 DELETE 调用成功）全部正常，删除后列表不消失为后端独立真缺陷，剥离为 R331 后端任务 test-first 修复，不算 t127 阻塞，不属本任务范围，不越界改后端/SDK。诊断证据（DELETE 200 + deleted_at 已置值 + manager 列表仍含该文档）保留在 gitignored oa4rust/test-results/r330-document-slots-fix/，作为 R331 立项依据。

## §7 证据索引与结论

- 证据目录：oa4rust/test-results/r330-document-slots-fix/（4 PNG + evidence.json，gitignored，非物理真机）。
- 诊断脚本：oa4rust/test-results/r330-del-diag.cjs、r330-diag.cjs（gitignored）。
- 结论：R330 P1「写入口全缺失」已完整闭环，t127 前端交付达标；R331 后端 manager 列表软删过滤另立任务；项目 ≥1000 轮非终态。
