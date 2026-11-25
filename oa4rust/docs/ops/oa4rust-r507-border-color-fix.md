# R507 QSD --border-color 未定义致边界消失修复（RED→GREEN）

## 1. 缺陷依据
- 来源：R506 甄别（oa4rust-r506-nontext-contrast-audit.md §4/§5），双人终审通过。
- 根因：SQL 语句设计器（QSD）参数绑定/可视化浮层多处 CSS 用 `border:1px solid var(--border-color)`，但全前端 grep `--border-color` 定义命中 0，声明整条失效；真浏览器 computedStyle `border=0px none`、boxShadow/outline/backgroundImage 全 none，28/30 控件功能性边界完全消失且无替代指示器，bgRatio 最低 1.02，违反 WCAG 2.1 AA 1.4.11。
- 真父：03b8200f52df43d74cd431489bb435ee56e2aaa8（台账 §501）。

## 2. 修复（仅新增变量定义，业务零改）
- 文件：`frontend/apps/desktop/src/views/QueryStatementDesigner.vue` `<style scoped>` 域内新增：
  `/* R507 WCAG1.4.11 ... */` 与 `.mod-view{--border-color: var(--border-subtle)}`。
- `.mod-view` 是 QSD 模板根元素（模板第 4 行 `<div class="mod-view">`），参数浮层/可视化浮层均在其子树内，局部 CSS 变量由继承覆盖全部 `var(--border-color)` 引用，不泄漏其他页面。
- `--border-subtle` 为 dark.css 双主题令牌（深色 rgba(0,212,255,0.5)/浅色 rgba(0,0,0,0.44)），已由 nonTextContrast.contract.test.ts 验证合成到三种背景 ≥3:1；未新造硬编码色值、未改全局 token 文件、未改任何选择器其他声明、模板、@click、v-for、data-testid、业务处理器。源码 diff 仅 2 行新增（注释 + 变量定义）。

## 3. 契约 RED→GREEN
- 新契约：`frontend/apps/desktop/src/views/R507.borderColorFix.contract.test.ts`（5 it，源码字符串断言）。
- 真父定点 RED：`npx vitest run .../R507...` → 2 failed（核心：scoped 内 --border-color 定义缺失、未挂根选择器）| 3 passed（守卫：原 border 声明保留、模板事件零改、全局 token 未新增变量），无 ReferenceError/TypeError 桩错。
- 修复版 GREEN：5/5 passed。

## 4. 三 verify（全 exit 0）
- V1 定点 vitest：5 passed (5)。
- V2 `pnpm test`：330 files / 3198 tests，0 failed。
- V3 `pnpm typecheck`：6 项目全 Done。

## 5. 真浏览器 E2E（Playwright Chromium headless 1440x900 DPR1，非物理真机）
- 双主题 light/dark 登录 <YOUR_TEST_DOMAIN> 后进入 `/oa4rust/app/admin/query-statement-designer/`，真实交互打开参数绑定浮层（button[title=参数绑定] → +添加参数）与 SQL 可视化编辑器浮层（button[title=SQL可视化编辑器] → +添加条件）。
- CDP/浏览器 getComputedStyle 取 borderColor/Width/Style 与祖先背景层，按 WCAG 相对亮度公式合成对比度；R506 补采同款选择器与判据。
- 终轮 `R507_VERIFY_DONE groups=2 shots=4 errs=0`；summary：`controlsTotal=30, borderMissing=0, notIdentifiable=0, minRatio=3.18`（≥3:1）。全局 `--border-color` 仍为空字符串（坐实修复为组件局部而非全局污染），组件根 `.mod-view` 继承值生效。
- 组件根局部变量探针（CDP getComputedStyle）：light 根 --border-color=#00000070、--border-subtle=#00000070，.param-input border=1px solid rgba(0,0,0,0.44)；dark 根 --border-color=#00d4ff80、--border-subtle=#00d4ff80，.param-input border=1px solid rgba(0,212,255,0.5)；全局 documentElement --border-color 为空串。坐实变量仅在 QSD 组件根定义并继承到浮层控件，未污染全局。
- 主轮对照：R506 同脚本同选择器补采 `borderMissing=28, notIdentifiable=28, minRatio=1.02`；修复后 28 个失效控件边框全部恢复 `1px solid` 且最低 3.18:1，前后差异坐实。
- 4 张截图（1440x900）目视确认真 OA4RUST admin QSD 参数/可视化浮层，边框清晰可辨；0 pageerror。

## 6. 证据 sha256（命令行实存值，gitignored `test-results/r507-border-color-fix/`）
- pc-qsd-param-fixed-light-r507.png 724c1c793c2cbda47e1301e4c138e43e7ffd0cecbb26a13f3a162bb3ebf4e4d1
- pc-qsd-param-fixed-dark-r507.png c3ae73cf278e0574c3ba38ee2ea46827cf278f7fe454d87ad6f17a24c1b2d0a3
- pc-qsd-visual-fixed-light-r507.png c2fcb23dc6cd4b86c6b521a5787c93b7f96d469c88e2ce151067ced5f9ef4503
- pc-qsd-visual-fixed-dark-r507.png 21922906e663b103ffde53a4bc47dc9045c308ed19c4978a741b144d883fbaaa
- verify.json 9353d9693bab478a4457d1ec26c4cedea1f2fcc5aa0895e02e6418df32e8592f
- local-token.json 4149d7d9ce77e41d40002221b602345bfd313b251d7da87582fe06ec38fbf955
- red.log d1e7d166853efa5ba06d25f53656a217872554fe7efadc896522b5aa1ad19cba；v1.log 5e46daacd8ffeb9dc522b2d69f2d46f79c9976daea8d0262cf72257857fdea1c；v2-full.log 608135d26e5ed0ab1a5f3d03bbf2e14888657df4f0a9d6f58911127a3c7b7138；v3-typecheck.log f1d438d95cb7702b21ec49e10aa04a5c5c8c298980ffa43ff0e9289c42e53a62。

## 7. 脱敏与结论
- 测试域名/主机仅用 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>` 占位；口令由 runner 运行时从测试服凭据文件读取注入环境变量，不入库；截图为 headless 浏览器实测、非物理真机。
- 结论：R507 修复成立，QSD 30 控件双主题边界全部可见且 ≥3:1，业务零改，三 verify 全绿，0 pageerror。