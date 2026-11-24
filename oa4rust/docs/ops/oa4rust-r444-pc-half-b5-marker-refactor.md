# R444 PC 半B5页视觉统一 3 页（ViewApp/FileInfoApp/JPushApp）

## 背景
R441/R442/R443 已为 16 个管理页根容器补齐 B5 统一视觉标记（`modern-page ai-page-b5 ai-panel`）。本轮对 3 个"已有 `modern-page ai-page` 但缺 B5 标记"的半成品管理页做同样处理，纳入 B5 管理区视觉体系。3 页均真实可达、无 scoped `.ai-page` 选择器依赖、既有 R306A 契约用宽松正则（`ai-page-b5` 含 `ai-page` 子串不破坏）、t3/t4 契约只查业务串；R306C 的 6 页（精确 class 断言）在本轮排除。

## 改动（精确 5 文件）
- `ViewApp.vue` / `FileInfoApp.vue` / `JPushApp.vue`：根容器 `<div class="mod-view modern-page ai-page">` → `<div class="mod-view modern-page ai-page-b5 ai-panel">`，各仅 1 行；保留原 `mod-view` + `modern-page`，`ai-page` 替换为 `ai-page-b5` 并追加 `ai-panel`。
- `R444.halfB5.contract.test.ts`：新建 12 it 静态契约。
- 本文档。
业务结构/接口/data-testid/header 已有 `ai-page ai-panel`/端点/payload/method/后端/SDK/main.ts/adminNav.ts/R306A/R306C 契约/其它 view/构建配置全部零改。git diff 仅 3 行新增 + 3 行删除（每页 1 行）。

## 验证
- 契约：3 页 × 4 it（根含 modern-page[父版有]/根含 ai-page-b5[父版无]/根含 ai-panel[父版无]/原 mod-view 保留[父版有]）= 12 it；真父 `d516935de` 上 RED 6 failed / 6 passed，修复后 GREEN 12/12。
- 三 verify 全 exit 0：`npx vitest run frontend/apps/desktop/src/views/R444.halfB5.contract.test.ts`（12/12）、`pnpm test`（312 files / 3050 tests / 0 failed）、`npx tsc -p frontend/apps/desktop/tsconfig.json --noEmit`。
- 构建入口 `index-BzSBbcf0.js`；整目录原子部署测试服 `DEPLOY_R444_PC_FULL_OK`。
- 真浏览器 E2E：Playwright Chromium headless、1440x900、非物理真机（NOT a physical device），3 路由（`/app/admin/view`、`/app/admin/fileinfo`、`/app/admin/jpush`）共 19 断言全 true / failed=[] / pageErrors=[]。每页断言：根同时含 modern-page/ai-page-b5/ai-panel 且 rootFound、原 mod-view 保留、H1 标题保留、业务按钮可见、fold=0、0 pageerror。
- 测试域名/主机占位：`<YOUR_TEST_DOMAIN>` / `<TEST_HOST>`；真值不入库，evidence 真域名仅留 gitignored 测试产物目录，不回显。

## 截图 SHA256（gitignored `test-results/r444-pc-half-b5/`，1440x900 非物理真机，命令行原始值）
- `00-view.png`：`03f5ccb6b891e64a15fb72d9bb6fa924c55dac86c06136d712601f88840f6307`
- `01-fileinfo.png`：`4ea4da5cd72f095a2cdd03b458c410c50d17c55ada014340a6507bb63b3d7ef4`
- `02-jpush.png`：`81ea00a8a696a20199f805046a9a3590fee3e9afb1003ae9e0671c13a1936f0d`
3 张截图均经 read_image 目视确认为真 OA4RUST admin 页（视图管理/文件信息/消息推送），非物理真机一致。