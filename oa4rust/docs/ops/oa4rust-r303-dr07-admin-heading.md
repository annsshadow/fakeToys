# R303 DR07-R 修复记录：部署页标题层级与管理页 B5 标记

## 来源与设计依据
- 来源：t30 次轮设计评审 finding R302-DR07-R（low），源码级复核坐实。
- 问题①：DeploymentApp.vue 原页面标题与描述均为 h2，语义层级重叠。
- 问题②：CmsIndexApp.vue / ProcessManagerApp.vue 根容器缺 modern-page/ai-page-b5 标记，未纳入 B5 管理区统一视觉。
- 修复原则：最小改动；不修改接口、data-testid 与业务行为；先写回归契约 RED，再 GREEN。

## 改动摘要
1. `oa4rust/frontend/apps/desktop/src/views/DeploymentApp.vue`
   - 标题 h2 → 唯一 h1（保留 `ai-gradient-text` 与 `data-testid="deployment-page-title"`），描述 h2 → `<p class="subtitle">`。
   - inject `ADMIN_LAYOUT_HIDE_TITLE` 并将 `hideTitle.value` 置 true，隐藏 AdminLayout 的同名 shell h1，确保渲染后只有 1 个 h1。
   - 样式选择器同步为 `.view-header h1.ai-gradient-text` 与 `.view-header p.subtitle`。
2. `oa4rust/frontend/apps/desktop/src/views/CmsIndexApp.vue` / `ProcessManagerApp.vue`
   - 根容器 class 增补 `modern-page ai-page-b5 ai-panel`，保留既有 AdminPageHeader/AdminFormSection/AdminToolbar 业务结构。
3. 回归契约 `oa4rust/frontend/apps/desktop/src/views/R303Dr07.contract.test.ts`（verify 包装：DeploymentApp.heading.contract.test.ts / AdminZone.modernPage.contract.test.ts）。

## RED → GREEN
- RED：新增契约在部署页标题层级/现代页面标记/隐藏 shell 标题断言下失败，verify 命令退出 1（Tests 4 failed | 2 passed）。
- GREEN：apply 最小改动后，verify 命令退出 0（2 files / 6 tests passed）；`pnpm run typecheck` 退出 0；`pnpm --filter @oa4rust/desktop build` 退出 0。

## 测试服真浏览器验证
- 部署地址：`https://<YOUR_TEST_DOMAIN>/oa4rust`（占位；真值仅本地 gitignored harness 运行时由环境注入）。
- 部署方式：按队长权威步骤，静态包 tar → scp → 远端备份 web → web.incoming 解包 → 原子 mv 替换，无服务重启；远端 index.html 校验构建哈希 `index-Br4G57bM.js` / `index-BOp8VUI6.css`。
- 浏览器：Playwright Chromium headless 1440×900，来源声明：`Playwright Chromium headless against deployed test server; not a physical device`（非物理真机）。
- 流程：登录 → 依次访问 /app/admin/deployment、/app/admin/cms-index、/app/admin/process-manager。
- 结果：三页 h1Count=1；部署页 data-testid="deployment-page-title" 为 H1 且 `p.subtitle=1 / h2.subtitle=0`，无端点调试文案与展开端点入口；CMS/流程实例页 `modernPage=1 / aiPageB5>0`，标题 testid 保留；0 pageerror。
- 截图与 evidence（gitignored，未入库）：`oa4rust/test-results/r303-dr07/01-deployment.png`、`02-cms-index.png`、`03-process-manager.png`、`evidence.json`。截图为用户本地 Chromium 对测试服的页面渲染，不是物理真机截图。

## 安全与范围
- 未改接口地址、会话/权限逻辑、data-testid 与业务操作；未提交任何真实域名、账号口令、私钥、个人路径；文档仅用 `<YOUR_TEST_DOMAIN>` 占位。
- 项目持续迭代，本记录仅代表本轮修复，不代表项目完成。
