# R443 PC 3页纯裸管理页 B5 标记补全

## 背景
R441/R442 先例已为 13 个管理页根容器补 `modern-page ai-page-b5 ai-panel` 三标记（B5 统一视觉基线）。本轮为 3 个单根纯裸 `mod-view`、真实可达但缺标记的管理页补齐三标记，统一 AI 风格视觉语言，业务功能与逻辑零改。

## 写域（精确 7 文件）
- 3 个 view：CategoryApp（分类管理）/ ProgramCenterApp（程序中心）/ ServerApp（服务器管理），各改根 div class 1 行
- 新契约：R443.contentB5.contract.test.ts
- 本文档
写域外（backend/mobile/main.ts/adminNav.ts/其它 view/构建配置/CategoryApp.writeEnvelope.contract.test.ts）零改。CategoryApp 已有的 R439/R440 静默写守卫完全不动。

## 改动
3 页 `<div class="mod-view">` → `<div class="mod-view modern-page ai-page-b5 ai-panel">`，仅追加标记 class，原 `mod-view` 保留。git diff --stat 3 files 各 1+/1-。

## 验证
- 契约：3 页 × 4 it = 12 it（三标记各 1 + 原 mod-view 保留 1）；真父 bcd29adc2 上 RED 9 failed / 3 passed，修复后 GREEN 12/12。
- 三 verify 全 exit 0：定点 12/12；根 `pnpm test` 311 files / 3038 tests / 0 failed；`pnpm exec tsc -p frontend/apps/desktop/tsconfig.json --noEmit`。
- 构建入口 index-Bym71w5b.js；整目录原子部署测试服 DEPLOY_R443_PC_FULL_OK。
- 真浏览器 E2E：Playwright Chromium headless、1440x900、非物理真机（NOT a physical device），逐页 3 路由 16 断言全 true / failed=[] / pageErrors=[]。每页断言 modernPage/aiPageB5/aiPanel ≥1、根容器同时含三标记、H1 标题保留、业务按钮可见、0 pageerror。
- 测试域名/主机：<YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST>（占位，真值不入库）。

## 截图（gitignored test-results/r443-pc-content-b5/，1440x900 非物理真机）
00-category.png / 01-program.png / 02-server.png 共 3 张，逐页对应 3 路由；均 1440x900 headless 截图，非物理真机。

## 说明
ServerApp 页面含 3 个 `<details>` 折叠分组（状态与信息/诊断端点/运维操作），为该页既有业务结构，非本轮标记改动引入；E2E 按钮可见断言仅要求存在可见业务按钮，不以无折叠为判据。标题 H1 由 AdminPageHeader 依导航配置归一化渲染，E2E 以真实 `h1.admin-title` 非空作为标题保留判据。未改任何业务代码。