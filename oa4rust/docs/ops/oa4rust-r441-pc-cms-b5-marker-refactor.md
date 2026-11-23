# R441 PC 7页 CMS 缺口管理页 B5 标记补全

## 背景
R303 先例已为部分页面补 `modern-page ai-page-b5 ai-panel` 三标记（B5 视觉基线）。本轮为 7 个真实可达但缺标记的 CMS 管理页根容器补齐三标记，统一视觉语言，业务功能与业务逻辑零改。

## 写域（精确 9 文件）
- 7 个 view：CmsColumnApp / CmsModuleApp / CmsColumnManagerApp / CmsFormDesignerApp / CmsScriptDesignerApp / CmsViewDesignerApp / CmsXformApp（各改根 `div` class 1 行）
- 新契约：R441.cmsB5.contract.test.ts
- 本文档
写域外（backend/mobile/main.ts/adminNav.ts/其它 view/构建配置）零改。

## 改动
6 页 `<div class="crud-view">` → `<div class="crud-view modern-page ai-page-b5 ai-panel">`；CmsScriptDesignerApp `<div class="mod-view">` → `<div class="mod-view modern-page ai-page-b5 ai-panel">`。仅追加标记 class，原 class 保留。

## 验证
- 契约：7 页 × 4 it = 28 it（三标记各 1 + 原 class 保留 1）；真父 5eb87acdf 上 RED 21 failed / 7 passed，修复后 GREEN 28/28。
- 三 verify 全 exit 0：定点 28/28；根 `pnpm test` 309 files / 3002 tests / 0 failed；`pnpm exec tsc -p frontend/apps/desktop/tsconfig.json --noEmit`。
- 构建入口 index-DS4LhGM2.js；整目录原子部署测试服 DEPLOY_R441_PC_FULL_OK。
- 真浏览器 E2E：Playwright Chromium headless、1440x900、非物理真机（NOT a physical device），逐页 7 路由 36 断言全 true / failed=[] / pageErrors=[]。每页断言 modernPage/aiPageB5/aiPanel ≥1、根容器同时含三标记、标题 H1 保留、业务按钮可见且无折叠、0 pageerror。
- 测试域名/主机：<YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST>（占位，真值不入库）。

## 截图（gitignored test-results/r441-pc-cms-b5/，1440x900 非物理真机）
| 路由 | 截图 | sha256 |
| --- | --- | --- |
| cms-column | 00-cms-column.png | d76eca9cd9878a126f880ea8c7e7f076ea642729eb6f9c903485753bab430305 |
| cms-script-designer | 04-cms-script-designer.png | 43b9f0fb5cf6a4de4263703d78e46a11e17b6298ab95332c710ea50a4acca7af |
其余 5 张截图同目录，逐页对应 7 路由。

## 说明
标题 H1 由 AdminPageHeader 依导航配置归一化渲染（源码 prop「CMS 栏目管理」→ H1「CMS列管理」），属既有组件行为；E2E 以真实 `h1.admin-title` 非空且含 CMS 作为标题保留判据，未改任何业务代码。