# R304 h1 去重断言重对齐（t44 / C3）

## 背景
- 来源：队长台账 §219 裁决；t11 首轮设计评审与 t41 的建议曾被提出把 DeploymentApp 的 h1 改回 h2。
- 裁决：不改运行时代码。t38（commit a8cc1a56c）已实现「页面自持唯一 h1 + 注入 ADMIN_LAYOUT_HIDE_TITLE 隐藏 AdminLayout shell h1」的合法第二模式；把 h1 改回 h2 会失去页面唯一 h1，制造无障碍回归。
- 本轮只同步过时契约断言到两种合法模式，DeploymentApp.vue / components/admin/** 运行时实现零改动。

## 合法标题模式
1. shell-h1 模式：管理页内不写 h1，用 h2 分区标题，由 AdminLayout 提供唯一 shell h1。
2. 页面自持唯一 h1 模式：页内恰好 1 个 h1，并 inject(ADMIN_LAYOUT_HIDE_TITLE) 且置 .value = true 隐藏 shell h1（DeploymentApp 采用此模式）。

## 改动文件
- frontend/apps/desktop/src/views/EndpointFold.devGate.B302.contract.test.ts：部署管理页用例改为断言「ai-page-b5 标记存在 + (shell-h1 模式 或 自持唯一 h1 模式) + 无端点调试/无 class="mono"」。
- frontend/apps/desktop/src/views/flowPortalAdminTitleDedup.contract.test.ts：PageCase 新增 selfHeldTitle 标记；DeploymentApp 标注为 true，用例对其断言恰好 1 个 h1 + 注入并开启 ADMIN_LAYOUT_HIDE_TITLE，其他页面仍断言无本地 h1；端点/操作/keep 断言不变。

## RED → GREEN
- RED（同步前）：EndpointFold 用例 expect(tpl).toContain('<h2') 失败；flowPortal 用例 expect(src).not.toMatch(/<h1[\s>]/) 失败。2 failed / 26 passed（共 28）。
- GREEN（同步后）：两契约 2 files / 28 tests 全通过；pnpm run typecheck 六个 workspace 全通过。

## 真浏览器实测（Playwright Chromium headless，非物理真机）
- 目标环境：部署测试服（域名用 <YOUR_TEST_DOMAIN> 占位；真值不入库）。凭据运行时经只读脚本从测试服读取，不回显、不入库。
- 流程：真 UI 登录 → /oa4rust/app/admin/deployment（部署管理）→ 读取真实 DOM h1。
- 断言结果：整页 h1 总数 1、可见 h1 1，文本「部署管理」，testid=deployment-page-title，class=ai-gradient-text，shell admin-title h1 数 0，deployment-page-title 节点数 1，pageerror=[]。
- 截图与 evidence：test-results/r304-t44-h1-dedup/（gitignored），01-deployment-h1-dedup.png + evidence.json。来源标注：Playwright Chromium headless against deployed test server; not a physical device。

## 结论
断言重对齐完成：两套契约同时承认两种合法标题模式，DeploymentApp 自持唯一 h1 的裁决被契约显式保护；运行时代码零改动，无新增业务风险。本轮为持续迭代中的一环，项目未完结。
