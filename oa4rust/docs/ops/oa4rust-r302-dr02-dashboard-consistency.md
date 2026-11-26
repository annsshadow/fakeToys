# R302 DR-02 工作台待审批统计与待办列表同源修复

- 任务：t26 [R302 DR-02 修复]（设计评审 high finding）
- 工作分支：opt1000/oa4rust-pc-ai-refactor
- 本轮为 ≥1000 轮持续迭代中的一环，非完成态

## 问题
工作台首屏「待审批」统计卡取自进程计数接口 `.../work/count/currentperson`，返回 0；
而同屏「我的待办」列表取自真实分页接口 `.../task/list/my/paging/1/size/20`，实测 5 条。
同一屏出现「待审批 0 / 待办 5 条」的自相矛盾，属真 bug（DR-02）。

## 根因
统计卡与列表是两条独立数据源：进程计数接口在该账号下恒为 0，未与分页待办列表对齐。
失败兜底 `?? 0` 还会把「取数失败」误渲染为真实的 0。

## 修复（保留全部业务逻辑与接口）
- 新增 `approvalCount = computed<number | null>`，直接由 `pendingItems.value.length` 派生：
  - 分页待办首次加载完成前、加载失败时返回 `null`，模板渲染占位（不显示误导性的 0）；
  - 加载成功后返回真实分页待办条数。
- 「待审批」统计卡 `value` 改为 `approvalCount.value`，不再使用 `statCount(0, procFailed, procCounts)`。
- 待办列表接口、分页大小、切片条数、其余三张统计卡（未读消息/已办结/今日会议）均保持不变。
- 新增 `pendingLoaded` 标志区分「尚未加载」与「加载完成但为空」。

## test-first RED → GREEN
- 新增 `Dashboard.dr02.contract.test.ts` 源码契约：断言待审批卡来自 `approvalCount`/`pendingItems` 同源、
  不得再用 `statCount(0,...)`、存在失败占位派生计数、列表仍来自真实分页接口。
- RED：`pnpm --filter @oa4rust/desktop exec vitest run src/views/Dashboard.dr02.contract.test.ts --reporter=dot`
  → 1 failed file / 2 failed tests（精确复现不同源缺陷）。
- GREEN：`pnpm --filter @oa4rust/desktop exec vitest run src/views/Dashboard --reporter=dot`
  → 3 files / 13 tests 全部通过。
- `pnpm --filter @oa4rust/desktop typecheck` 通过；`pnpm --filter @oa4rust/desktop build` 通过。

## 测试服真浏览器实测（每轮硬性要求）
- 部署：测试服 `<YOUR_TEST_DOMAIN>` 的 `<YOUR_TEST_HOST>` 目录，原子替换 web/web.old，
  远端 index.html 校验哈希 `index-CAtVICEH.js` / `index-BOp8VUI6.css` 与本轮构建一致。
- 真浏览器：Playwright Chromium headless 登录后进入工作台，读取统计卡与列表 DOM：
  - `approvalStatValue = "5"`，`approvalStatPlaceholder = false`
  - `todoRows = 5`，`statMatchesList = true`
  - 修复前为「统计 0 / 列表 5」，修复后同屏一致为 5。
- 截图与证据：`oa4rust/test-results/r302-dr02/01-dashboard-consistency.png`、
  `oa4rust/test-results/r302-dr02/evidence.json`（本地 gitignored，未入库）。
- 来源标注：截图来自本地 Playwright Chromium headless 对已部署测试服真实页面的操作，
  `evidence.json` 的 source 为「Playwright Chromium headless against deployed test server; not a physical device」，
  **为本地渲染证据，非物理真机**。
- 说明：截图中待办标题为测试服随机流程实例数据，仅用于证明列表真实有 5 条，非本任务构造的假数据，文档不记录具体标题。

## 变更文件
- `oa4rust/frontend/apps/desktop/src/views/Dashboard.vue`
- `oa4rust/frontend/apps/desktop/src/views/Dashboard.dr02.contract.test.ts`
- `oa4rust/docs/ops/oa4rust-r302-dr02-dashboard-consistency.md`（本文）
