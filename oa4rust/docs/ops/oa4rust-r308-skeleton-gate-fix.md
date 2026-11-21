# R308 静态骨架条无条件渲染修正（t64）

## 背景与依据

t62 基线复审（docs/ops/oa4rust-r308-baseline-reaudit.md §3/§6）坐实：
PC 端 57-admin-portal-script-designer 页（oa4rust/frontend/apps/desktop/src/views/PortalScriptDesignerApp.vue）
模板中 `<div class="script-loading ai-skeleton-line" aria-hidden="true"></div>` 无条件渲染，
数据加载完成后骨架条仍常驻 DOM，构成视觉误导与骨架探针假阳性。

## 根因与方案

该页面 script setup 仅持有 api adapter（list/load/create/save/remove/versions），
无 loading 状态引用；ScriptWorkbench 子组件自管 loading（列表内 `v-if="loading"` 显示“加载中…”），
父页面无法绑定其加载态。据此选择直接移除常驻装饰骨架节点，
保留 .ai-skeleton-line / @keyframes ai-shimmer 令牌样式（满足 R306C 契约对未来真实骨架的样式约定，不产生 DOM 假阳性）。

## RED → GREEN

- 新契约 oa4rust/frontend/apps/desktop/src/views/SkeletonGate.R308.contract.test.ts：
  断言模板无无条件骨架节点、若保留须绑定真实 loading、ai-page/ai-panel 与业务端点不回退。
- RED：2 failed / 1 passed（骨架条常驻且无 v-if）。
- GREEN：3 passed；连同 AdminR306C 共 2 files / 9 tests passed；pnpm run typecheck 通过。

## 真浏览器实测（非物理真机）

构建 desktop 产物后原子部署测试服（SSH 别名 oa4rust-test，真值不入库）。
Playwright Chromium headless 真登录逐页复验 /app/admin/portal-script-designer：
skeletonBar=0、aiPage=2、workbench=true、scriptWorkbench=true、pageerror=0、API 失败=0，pass=true。
截图与 evidence：oa4rust/test-results/r308-skeleton-gate/（gitignored）。
来源标注：Playwright Chromium headless against deployed test server; not a physical device。

## 服务地址与凭据

测试服域名、SSH 连接与登录凭据均不入库，运行时从 <YOUR_TEST_DOMAIN> 对应的环境变量/本地凭据文件读取。
