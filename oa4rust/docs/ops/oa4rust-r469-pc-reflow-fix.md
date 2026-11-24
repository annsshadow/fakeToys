# R469 PC 320px 移动端侧栏抽屉态回流修复报告

- 任务：t330（Attempt 6f2a79eb-b20b-40f2-984b-fecdaffdee60），实现页 page-core；测试服域名仅以 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，0 真值入库。
- 证据来源：Playwright Chromium headless **非物理真机**。PC 1280×1024 baseline + 320×256（等价 400% 缩放 1280×1024）双主题；H5 390×844 DPR2 isMobile/hasTouch。0 pageerror。
- 真父基线：1c1ceb448981c8df0536750b253f05191f262d70（HEAD=UP，AB=0/0，DIRTY=0，分支 opt1000/oa4rust-pc-ai-refactor）。

## 一、缺陷根因（依据 R468 甄别）

R468 甄别（oa4rust/docs/ops/oa4rust-r468-reflow-audit.md）坐实 1 项 P2 可达缺陷：AppShell.vue:446 `isMobile.value = window.innerWidth < 768`，但移动端 `@media (max-width: 768px)` 内两条规则互相作用：

1. `.app-shell.mobile .sidebar { transform: translateX(0) }` 覆盖基线 `translateX(-100%)`，280px 侧栏无条件常驻；
2. `.topbar, .content { margin-left: var(--sidebar-width) }` 又叠加 240px 偏移。

320px 视口下主内容仅剩约 80px 文字碎片，PC 8 页双主题 CDP 几何与冷启动截图双证；WCAG 1.4.10 Reflow 不达标。

## 二、修复方案（业务零改）

- AppShell.vue:5 根 div 增加 `'drawer-open': mobileDrawerOpen` 状态类；
- AppShell.vue:257 新增 `const mobileDrawerOpen = ref(false)`；
- `toggleSidebar()` 移动端切换抽屉（切完 return），桌面端保留原 `sidebarCollapsed` 折叠逻辑；
- 移动端 CSS：常驻规则收紧为 `.app-shell.mobile.drawer-open .sidebar { transform: translateX(0) }`；`.topbar, .content` 偏移归零 `margin-left: 0`，内容回流占满视口，侧栏改为按需滑出抽屉。
- 过时契约 AppShell.offsetGeometry.contract.test.ts:41-45 原断言锁死缺陷（要求移动端补 --sidebar-width 偏移），改为断言“不得有该偏移且必须 margin-left:0”——更新为新正确语义，非删除削弱。

新契约 R469.reflow.contract.test.ts（5 it）真断言：根容器绑定 drawer-open 与移动抽屉 ref、toggle 分支、侧栏基线隐藏仅抽屉态滑出、topbar/content margin 归零、R293 补偿移除。

## 三、RED → GREEN 与三 verify

- 真父版定点运行：`npx vitest run R469.reflow.contract.test.ts AppShell.offsetGeometry.contract.test.ts` → 6 failed / 3 passed（9），失败全为缺陷断言（AssertionError），无桩错（ReferenceError/TypeError=0）；日志 test-results/r469-red.log。
- 修复版 V1 定点：9/9 passed，exit 0（日志 r469-v1.log）。
- V2 全量：`pnpm test` → 319 files / 3143 tests 全过，0 failed，exit 0（日志 r469-v2-full.log）。
- V3 类型：`pnpm typecheck` → 6 项目全部 Done，exit 0（日志 r469-v3-typecheck.log）。

## 四、构建与部署

`pnpm run build` exit 0，入口 index-g2Ub0YLo.js / CSS index-COmAqEyf.css；部署脚本 test-results/deploy-pc-r469.sh 打包 → scp → sha256 比对 → 远端 web.incoming-r469 → grep 入口 → mv 原子替换，输出 DEPLOY_R469_PC_FULL_OK，测试服入口 hash 已复核一致。

## 五、真浏览器实证（320px 窄视口专项）

脚本 test-results/r469-pc-reflow-check.cjs + runner r469-pc-reflow-run.sh；证据目录 test-results/r469-pc-reflow-fix/（gitignored，含 report.json 与 23 张截图）。终轮 R469_DONE checks=140 passed=140 failed=0 shots=23 errors=0。

关键几何（dashboard，浅色）：

| 状态 | sidebar right | sidebar transform | topbar/content margin | content x / w |
| --- | --- | --- | --- | --- |
| 1280 baseline | 240 | none | 0px | 240 / 1040 |
| 320 默认 | 0 | matrix(-280px) | 0px / 0px | 0 / 320 |
| 320 抽屉打开 | 280 | matrix(0px) | 0px / 0px | 0 / 320 |
| 320 抽屉关闭 | 0 | matrix(-280px) | 0px / 0px | 0 / 320 |

覆盖 PC 8 路由（dashboard / admin/common / bbs / bam / org / file / attendance / process）× 双主题：baseline 侧栏可见与内容偏移、320 默认侧栏隐藏、无 drawer-open 类、margin 归零、内容 x≤2 且宽≥290、无横向溢出，均逐页通过；☰（.collapse-btn）点击抽屉开→滑入→再点关闭→滑出，双主题通过；H5 5 页零回归。

截图 sha256（命令行 sha256sum，来源 Playwright Chromium headless 非物理真机，完整 23 张见证据目录 report.json）：

| 截图 | sha256 |
| --- | --- |
| narrow-dashboard-light.png | c58eb6d8d39e22598dd6e5ceddec9c2ac47b98a04793d0075f5cdb4d4891ad0d |
| drawer-dashboard-light.png | fc568c664d923ba10fe047c4980ab6840f5b8e7f98eed22d26ae3308dd0e8b5a |
| narrow-dashboard-dark.png | 4453502ab4e850600e2fe3b8ef8c2df78add0cb2217b3d2f9cb7fa55656035b8 |
| drawer-dashboard-dark.png | d2237829f416bfa2cdf8a9c72a89c411936a2e004140a9ad3cd1627068ae510c |
| narrow-common-light.png | 93e7429e496b4ac05fb30fd3e4518be3b9cd1c67d7f86511aa6ce61faa86c601 |
| narrow-common-dark.png | 6746649ab60078abd3a441065cd7346d1677369950fef654f44f8cf014f9fe99 |
| narrow-bbs-dark.png | ac9eb53ce1e2108e90207160c748eb9f543efcad4657c0ab0e61f0288b579f92 |
| narrow-file-light.png | 78fb30a89a06c071081bc0e2e4ccb094fff117b640e817f723c82b3b8c61252d |
| narrow-process-work-dark.png | 18a4b254e781b2ff82d29d2b1fbe1ae71bef1de3756aa2cfd2d462eaba8e62d3 |
| h5-index.png | c531e964b3ac9378572318159b21821f8d155dda18b916a6efabcc503f068c96 |

narrow-dashboard-light.png（320×256，侧栏隐藏、欢迎语与时钟完整回流）与 drawer-dashboard-light.png（抽屉滑出、侧栏导航可见）已 read_image 目视确认，为同一 headless 会话真实截图。

## 六、真值纪律与写域

精确写域 4 文件：AppShell.vue、AppShell.offsetGeometry.contract.test.ts、R469.reflow.contract.test.ts、本文档。业务模板/接口/data-testid/后端/SDK/H5/其它 view 零改。测试服域名与账号口令仅从环境与 gitignored runner 注入，仓库与文档仅留 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，0 真值。
