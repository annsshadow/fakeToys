# R442 PC 6页内容/流程管理页 B5 标记补全

## 背景
R441 先例已为 7 个 CMS 缺口管理页根容器补 `modern-page ai-page-b5 ai-panel` 三标记（B5 统一视觉基线）。本轮为 6 个真实可达但缺标记的内容/流程管理页补齐三标记，统一 AI 风格视觉语言，业务功能与逻辑零改。

## 写域（精确 8 文件）
- 6 个 view：AppInfoApp / DocumentApp / HotpicApp / CollectApp / TemplateApp / ProcessApplicationApp（各改根 div class 1 行）
- 新契约：R442.contentB5.contract.test.ts
- 本文档
写域外（backend/mobile/main.ts/adminNav.ts/其它 view/构建配置）零改。

## 改动
AppInfoApp/DocumentApp/HotpicApp/ProcessApplicationApp 的 `<div class="mod-view">` → `<div class="mod-view modern-page ai-page-b5 ai-panel">`；CollectApp/TemplateApp 的 `<div class="crud-view">` → `<div class="crud-view modern-page ai-page-b5 ai-panel">`。仅追加标记 class，原 class 保留。

## 验证
- 契约：6 页 × 4 it = 24 it（三标记各 1 + 原 class 保留 1）；真父 6d6006bfd 上 RED 18 failed / 6 passed，修复后 GREEN 24/24。
- 三 verify 全 exit 0：定点 24/24；根 `pnpm test` 310 files / 3026 tests / 0 failed；`pnpm exec tsc -p frontend/apps/desktop/tsconfig.json --noEmit`。
- 构建入口 index-BP64jUqB.js；整目录原子部署测试服 DEPLOY_R442_PC_FULL_OK。
- 真浏览器 E2E：Playwright Chromium headless、1440x900、非物理真机（NOT a physical device），逐页 6 路由 31 断言全 true / failed=[] / pageErrors=[]。每页断言 modernPage/aiPageB5/aiPanel ≥1、根容器同时含三标记、H1 标题保留、业务按钮可见且无折叠、0 pageerror。
- 测试域名/主机：<YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST>（占位，真值不入库）。

## 截图（gitignored test-results/r442-pc-content-b5/，1440x900 非物理真机）
00-appinfo.png / 01-document.png / 02-hotpic.png / 03-collect.png / 04-template.png / 05-process-application.png 共 6 张，逐页对应 6 路由；6 张均 1440x900 headless 截图，非物理真机。

## 说明
标题 H1 由 AdminPageHeader 依导航配置归一化渲染；E2E 以真实 `h1.admin-title` 非空作为标题保留判据，未改任何业务代码。