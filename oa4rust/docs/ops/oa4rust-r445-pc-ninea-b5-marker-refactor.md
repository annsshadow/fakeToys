# R445 PC 九页裸 ai-page 补 B5 标记修复（t296）

## 1. 背景与依据
R444 已对 ViewApp / FileInfoApp / JPushApp 三页把根 `ai-page` 升级为 `ai-page-b5 ai-panel`。只读审计发现另有 9 个管理页仍停留在裸 `ai-page modern-page`，未进入 B5 设计语言。本轮把这 9 页对齐 B5 标记，业务结构与接口零改。

## 2. 改动范围（硬边界）
仅改以下 9 个 .vue 的根容器 `class` 一行：`ai-page` → `ai-page-b5 ai-panel`，保留原基类和 `modern-page`。
- oa4rust/frontend/apps/desktop/src/views/FormApp.vue（mod-view，data-testid=form-page 保留）
- oa4rust/frontend/apps/desktop/src/views/QueryManager.vue（query-view）
- oa4rust/frontend/apps/desktop/src/views/QueryViewApp.vue（mod-view；另有 mod-view ai-page 非目标，勿改）
- oa4rust/frontend/apps/desktop/src/views/QueryDesigner.vue（query-designer）
- oa4rust/frontend/apps/desktop/src/views/QueryExplorerApp.vue（crud-view）
- oa4rust/frontend/apps/desktop/src/views/PortalDesigner.vue（designer-shell）
- oa4rust/frontend/apps/desktop/src/views/PortalDictDesignerApp.vue（crud-view）
- oa4rust/frontend/apps/desktop/src/views/PortalPageDesignerApp.vue（crud-view）
- oa4rust/frontend/apps/desktop/src/views/QueryViewDesignerApp.vue（designer-shell）

硬边界：不改业务结构/接口/data-testid/header 内层 ai-page ai-panel/端点/后端/SDK/main.ts/导航/其它契约。

## 3. RED → GREEN
新增 `R445.b5.contract.test.ts`（36 it）：全文正则锚定 `class="[^"]*<base>[^"]*modern-page ai-page-b5[^"]*ai-panel[^"]*"`（QueryViewApp/QueryExplorerApp/QueryViewDesignerApp 首个 div 非目标，故不用取首个 div）。真父 92a45397b RED=18 failed/18 passed；GREEN=36/36。

## 4. 门禁
- 定点：36/36 passed
- 全量：pnpm test → 313 files/3086 tests 全过
- typecheck：npx tsc -p frontend/apps/desktop/tsconfig.json --noEmit → exit 0
- build：pnpm build → exit 0，入口 assets/index-DmnvoVwz.js

## 5. 测试服真浏览器实测
原子部署：tar 打包 → scp → sha256 本地远端比对（5118a93806f2c4a00ef8730791fe13a21b76bda2a13ff7186bc83d76b8e5ea3a）→ 远端原子替换（备份 web.old-r445）；远端 index.html 已含 index-DmnvoVwz.js。
Playwright Chromium headless 1440x900 登录测试服（域名用占位符 `<YOUR_TEST_DOMAIN>`），逐一访问 9 个 admin 路由，逐页断言：根容器 `.<base>.modern-page.ai-page-b5.ai-panel` 的 `rootCount=1 / hasB5=true / hasPanel=true / hasBase=true`，H1 标题保留 `h1Count=1`，业务按钮可见 `buttonCount>0`，`foldCount=0`，0 pageerror。9 页全部通过。
证据目录（gitignored，NOT a physical device 非物理真机）：`oa4rust/test-results/r445-pc-nine-b5/`（9 张路由截图 + report.json）。来源标注：Playwright Chromium headless 1440x900 -> 测试服真实页面（非物理真机）。

9 截图 sha256 命令行原始值（`cd oa4rust/test-results/r445-pc-nine-b5 && sha256sum *.png`）：
```
4089d655b044a8633618261643013e4969f2a704470f3016c29a0b3952d8c892  form.png
178032d2e1a3188cadee58c8e8e6c2d1f799f37a33d2b90d93ac0d4f8415f383  portal-designer.png
5bb504a16f4dece5da916e211a17b697cecc83bc14ae1076020df0cee8726f83  portal-dict-designer.png
dfad7181a7ae67e8e11f423e4eff922e3e1852f9139574635a2f5abfd28f63b2  portal-page-designer.png
6940a4a70665066be6624ad5c7ddc0b57e1b7a83dd0eb84bc36766b042eeb76e  query-designer.png
1a1c0bcf27134cf51f7fa4685222ef09c82ce2398f11fff7f4310f4ef06c965b  query-explorer.png
29a4fc719577e7f97e33a37087c1bae4b0f4d0b1694a34d8c223ec8a5755dab1  query-view-designer.png
cde643aa35aed180883d2124ebc545cb2db8c22abcb4718c6c459adbf0f7488a  query.png
d1e6845ff4efc59ad444a01d9cf571bf6d28f0215334ebdac0a9443beac1b75c  queryview.png
```

## 6. 真值纪律
文档用占位域名 `<YOUR_TEST_DOMAIN>`；口令/私钥/会话数据不入库；截图与临时脚本留在 gitignored 证据目录。

本轮为 ≥1000 轮持续迭代中的一环，非终态。