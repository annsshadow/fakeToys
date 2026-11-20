# R304 业务视图契约同步 B（t43）

## 背景

依据 t41 审计与 §219 裁决：业务视图实现已正确演进，但 7 条源码级契约断言停留在旧形态，导致定点 vitest 误报。本轮仅同步过时断言至语义等价的新形态，实现零改动，未弱化为空断言、未使用 expect(true)、未跳过用例。

## 7 条断言同步明细

- 用例1 BusinessWork.aiStyle.B4.contract.test.ts:51-52：任务中心已由单一 h1 演进为带 data-testid 的 h2 主标题；断言改为「h1>0 时仍须唯一，h1=0 时须存在 data-testid 匹配的 h2 主标题」。
- 用例2 destructiveGuards.test.ts:16-18：回收站破坏性动作由 2 个演进为 4 个（永久删除/批量恢复/批量删除/清空）；守卫计数 2→>=2，并逐函数校验 if(busy.value) 守卫、busy.value=true 上锁与 finally 复位。
- 用例5 liveQueryItems.test.ts:126：统计失败口径由 `s.value ?? '—'` 演进为 stat-placeholder + `{{ s.value ?? null }}`；断言改校验占位类、null 回退与 statCount 失败返回 null。
- 用例6 liveQueryItems.test.ts:137：待办空态文案由「暂无待办事项」演进为「暂无待办，去流程中心发起」；断言改用正则提取真实文案并校验错误态分支排在空态之前、失败分支返回 null。
- 用例7 ModernBusinessB.contract.test.ts:47：会议页 class 由 `meeting-view modern-page` 演进为追加 ai-page；整串断言改分项 toContain(meeting-view) + toContain(modern-page)。
- 用例8 ModernBusinessB.contract.test.ts:76：工作流页 class 同理演进为追加 ai-page/ai-panel；改分项 toContain(work-view) + toContain(modern-page)。
- 用例12 ProcessFold.contract.test.ts:44：displayTitle 对 work- 原始键的识别由正则 `\^work-` 演进为 startsWith('work-') 与「工作实例」映射；断言接受三种等价形态。

## 实现零改动范围

仅修改 5 个契约测试文件 + 本文档。未触碰任何 .vue 实现；未触碰 DeploymentApp.vue、EndpointFold.devGate/flowPortal 契约、AdminToolbar/manifest（分属 t44/t42/t46）。

## 验证结果

- RED 基线：定点 5 契约 vitest，`Test Files 5 failed / Tests 7 failed | 22 passed (29)`，精确复现 7 条过时断言。
- GREEN：同步后同命令 `Test Files 5 passed / Tests 29 passed (29)`，无跳过。
- 全仓 `pnpm run typecheck` 通过（desktop/mobile/locales/sdk/apis/ui 六项目 Done）。
- 本轮仅改断言与文档、未改运行时代码，测试服沿用已部署构建，无需重建与原子部署。

## 测试服真浏览器冒烟

通过 Playwright Chromium headless（1440×900）登录 <YOUR_TEST_DOMAIN> 测试服，逐页访问工作台/工作流/会议管理/回收站/流程任务中心：五页均真实渲染，页面错误 0；任务中心呈现 h2[data-testid=process-task-center-page-title] 主标题与业务名「S3 运行闭环」，工作流呈现可读业务名「S3 运行闭环」「工作实例」，均未暴露原始 work- 键。
截图与证据：test-results/r304-b-smoke/ 下 dashboard.png、process.png、meeting.png、recycle.png、process-task-center.png 与 evidence.json。截图来源为本地 Playwright headless 对部署测试服的真实页面渲染，非物理真机。凭据仅运行时从测试服本机读取，未入库。
