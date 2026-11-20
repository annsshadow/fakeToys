# R301 C301 业务工作域实质升级（回收站 / 笔记 / 文件管理）

> 截图来源：Playwright Chromium headless 对已部署测试服的真实页面流程实测，非物理真机；证据与截图位于 `oa4rust/test-results/r301-c301-jsoncreate/`（gitignored）。

## 目标与依据

按 R301 设计评审 t11 的 DR-04/DR-05/DR-06 findings，对业务工作域做信息层级、筛选与批量操作的实质优化（非仅换色）：

- DR-04：业务页保持唯一可见 h1。
- DR-05：空态给出说明文案 + 一个主行动按钮，复用 `.ai-panel`。
- DR-06：原始技术标识符不再暴露为业务字段标签，改为可读文案。
- 任务附加要求：筛选与批量操作实质优化。

## 改动文件

- `oa4rust/frontend/apps/desktop/src/views/RecycleApp.vue`：重写为「原始列表 allItems + items 计算属性」结构，新增关键词 + 类型双筛选、全选/逐项勾选、批量恢复/批量永久删除（Promise.allSettled）、两种空态引导（空回收站 / 无筛选结果 + 主行动按钮）。保留 `recycle-*` 既有 testid、`v-for="item in items"`、孪生端点调试入口与「刷新列表/清空回收站」以兼容既有契约。
- `oa4rust/frontend/apps/desktop/src/views/NoteApp.vue`：成员字段从「成员 ID / personId」改为可读「成员 / 成员姓名 / 工号（可选）」，表格成员列加 `data-testid="note-member-cell"`；保留 `form.personId` 与 create payload 兼容。
- `oa4rust/frontend/apps/desktop/src/views/FileManager.vue`：新增「新建可清理文件」UI 入口（`data-testid="file-create-json"`），通过 JSON 端点 `POST /api/file/core/entity/file` 以会话本人身份创建真实 FILE_FILE 记录，并在「可清理文件」列表（`data-testid="files-recyclable-list"`）即时回显，复用现役软删端点 `DELETE /api/file/assemble/control/file/{id}` 使其进入回收站。修复归属人取自 `session.state.user?.unique`（SDK 会话实际结构），避免空 person 导致后端 400。
- 契约：`BusinessWorkUpgrade.C301.contract.test.ts`（新）、`FileManagerJsonCreate.C301.contract.test.ts`（新）。

## 为何这样设计

回收站数据源为 FILE_FILE 的软删行（`deleted_at IS NOT NULL`），而「我的文件」列表读 x_file，两表不同。为使「数据不够走真实 UI 流程添加」可行且不新增后端接口，采用最小 UI 方案：用现成 JSON 建文件端点写 FILE_FILE，本地 ref 数组即时回显新项，删除沿用现役软删端点直接进入回收站——全程真实 API、真实落库，非静态假数据。

## RED → GREEN

1. 新增 C301 两份契约先跑 RED：BusinessWorkUpgrade 4 failed/5 passed；FileManagerJsonCreate 3 failed/1 passed。
2. 实现后 GREEN：`pnpm exec vitest run src/views/BusinessWorkUpgrade.C301.contract.test.ts src/views/RecycleApp.business.contract.test.ts src/views/FileManagerJsonCreate.C301.contract.test.ts --reporter=dot` → 3 files / 13 tests passed。
3. `pnpm --filter @oa4rust/desktop build` → exit 0（dist hash index-C49Ng-hj.js / index-BMFkaD73.css）。
4. `pnpm run typecheck` 的报错来自队友在途的 `OrgViewer.loadingFallback.contract.test.ts(33-36)` TS1005，与本次改动无关，未触碰该文件。

## 真浏览器实测

脚本：`oa4rust/test-results/r301-c301-jsoncreate-check.cjs`（gitignored）。测试服地址由 `.local.keys` / 环境变量读取，凭据经 `ssh <TEST_HOST> python3 -` 运行时注入，均未入库。

实测步骤与结果：

- 登录 → /app/file：点击「新建可清理文件」两次（prompt 输入文件名），列表回显 2 项。
- 逐项点击删除（自定义确认弹窗「确认」），本地列表 remaining=0。
- /app/recycle：两个新文件名均出现在回收站（inRecycle=2），筛选/类型/全选控件存在，0 pageerror。
- 关键词筛选 `JSON-A`：只显示 A、排除 B；无结果时引导文案 + 主行动按钮出现。
- 全选 + 批量恢复（确认弹窗）：回收站 6 项 → 0 项，恢复成功提示。
- /app/note：新建弹窗成员字段为可读文案，`成员 ID` 标签为 0。

截图 01~07 与 `evidence.json` 位于 `oa4rust/test-results/r301-c301-jsoncreate/`，`source` 字段标注「Playwright Chromium headless against deployed test server; not a physical device」，并保留 `urlHost` 仅作证据定位（不入版本库）。

## 提交前对齐

历史台账重写导致本地与远端分叉，提交前执行 `git fetch && git pull --rebase` 对齐后精确 `git add` 本次文件、单提交推送；不暂存队友在途文件。
