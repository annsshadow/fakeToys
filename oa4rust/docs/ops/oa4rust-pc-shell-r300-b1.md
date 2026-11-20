# R300 B1 PC 应用外壳与导航实测记录

## 范围
- 修改 PC 应用外壳 `frontend/packages/ui/src/components/AppShell.vue`，基于 t1 的 `--ai-*` 双主题令牌落地：
  - 侧栏品牌/激活态使用 AI 渐变令牌；主题切换按钮具名并采用 spring/快速动效令牌。
  - 顶栏增加多段面包屑（根 / 分组 / 当前页）、活动页签条（可关闭）、命令面板入口。
  - Ctrl/⌘+K 或入口按钮打开命令面板，支持过滤、↑↓ 选择、Enter 打开/搜索、Esc 关闭。
  - 顶栏全局搜索回车跳转真实 `/app/search?q=` 页面。
- 新增契约 `frontend/packages/ui/src/components/AppShell.aiShell.contract.test.ts`，先 RED（5 失败）后 GREEN。
- 原有 58 条管理导航、15 条侧栏项、键盘可达、布局/偏移/搜索标记契约均保持通过。

## 验证与证据
- 契约测试：10 个文件 / 54 项通过（含新增 5 项、原有外壳与导航契约）。
- 类型检查：`pnpm --filter @oa4rust/desktop typecheck` 通过。
- 构建：`pnpm --filter @oa4rust/desktop build` 通过，产物入口 `index-UfuUBydA.js` / `index-CcsYol9C.css`。
- 部署：将构建产物部署到测试服 PC 静态目录（SSH 别名与真值不写入仓库），远端资源核对通过；从本机直接 curl 远端回环失败属预期，不作部署失败判据。
- 真浏览器实测：Playwright 无头 Chromium 对已部署测试服执行真实 UI 登录→导航→命令面板→主题切换→全局搜索→关闭页签流程，非物理真机、非静态截图；截图与机器证据位于 `test-results/r300-b1-shell/`（本地 gitignored）。
  - `01-dashboard.png`：登录后工作台。
  - `02-directory-navigation.png`：点击侧栏「通讯录」后页签/面包屑。
  - `03-command-palette.png`：命令面板打开并过滤。
  - `04-theme-switched.png`：主题切换到深色。
  - `05-global-search-result.png`：全局搜索提交后跳转。
- 关键 DOM 断言（浏览器实测）：登录进入工作台；点击侧栏后 URL 到通讯录、面包屑为「工作台 / 工作台 / 组织通讯录」、页签数 2；命令面板 15 项、过滤后 5 项、回车跳回工作台且面板关闭；主题 light→dark；搜索跳转 `/app/search?q=...`；关闭页签 2→1；侧栏 15 项含系统管理。

## 注记
- 截图来源如实标注：Playwright Chromium headless + 测试服真实页面，不是物理真机。
- 真域名/SSH 别名/凭据仅从本地环境读取，仓库与提交信息不写真值。
- 项目持续 ≥1000 轮迭代，本轮为其中一环，非完成态。
