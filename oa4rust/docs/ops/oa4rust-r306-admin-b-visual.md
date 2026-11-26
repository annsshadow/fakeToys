# R306-B 六页 admin 设计器视觉深化（t54）

## 背景

R306-A 审计发现六个 admin 设计器页处于「双未消费」状态：源码 `var(--ai-*)` 计数为 0，真浏览器 DOM `.ai-page` 计数为 0，AI 设计令牌完全未落地。
本轮按 t11 findings 做信息层级与视觉实质升级，而非仅换颜色。

## 改造范围（六页）

- QueryDesigner.vue / PortalDesigner.vue
- QueryExplorerApp.vue / QueryViewDesignerApp.vue
- PortalDictDesignerApp.vue / PortalPageDesignerApp.vue

每页：根容器加 `modern-page ai-page`，页头加 `ai-page ai-panel`；
style 注入令牌规则——8px 栅格 `var(--ai-grid-2)`、渐变表面 `var(--ai-gradient-surface)`、玻璃背景 `var(--ai-glass-bg)` + `var(--ai-glass-blur)`、品牌渐变 `var(--ai-gradient-brand)`、骨架屏 `var(--ai-skeleton-base)` + `var(--ai-skeleton-shine)` + `ai-shimmer` 动画、微交互 `var(--ai-motion-fast)` + `var(--ai-ease-spring)`。
PortalDesigner 裸「加载中…」段落替换为 `.sk-wrap` 骨架。

## 预存真 bug：组件漏 import（RED→GREEN）

`QueryExplorerApp.vue` / `QueryViewDesignerApp.vue` 模板使用了 `<AdminPageHeader>` / `<AdminFormSection>` / `<AdminToolbar>`，但仅以 `void` 引用规避 lint，未真正 import。
运行时整页 setup 抛 `ReferenceError: AdminXxx is not defined`，被全局 errorHandler 吞成「页面发生意外错误，请重试」toast，`pageerror` 事件抓不到。
先写契约断言（模板使用的每个 `Admin*` 组件都必须在 `../components/admin/` import、且不得保留 `void` 规避）——RED 1 failed/4 passed，精确复现；
再补 import 并删除 `void` 引用——GREEN 5 passed/5。对照正规写法 CmsScriptDesignerApp.vue / FormDesigner.vue。

## 验证

- 新契约 `AdminR306B.contract.test.ts`：RED 4 failed/1 passed（令牌断言）→ 追加 import 断言 RED 1 failed/4 passed → GREEN 5 passed/5。hasAiRule 逐选择器断言真实令牌，aiVarCount>=5、aiPageCount>=2，非假断言。
- `pnpm run typecheck`：exit 0（desktop/mobile/sdk/apis/ui/locales 全 Done）。
- 全量 desktop vitest：2 failed/1078 passed，两条失败均在队友写域（tabStateA11y 日历选中态、AdminR306C PortalScriptDesigner/FindDesigner 骨架令牌），本任务六页与契约无回归。
- 构建入口：`index-BiyhjNmp.js` / `index-BMFkaD73.css`；测试服原子部署（备份 web.bak → scp → web.incoming → 原子 mv）后远端哈希一致。

## 真浏览器实测

Playwright Chromium headless 1440×900 登录测试服（`<YOUR_TEST_DOMAIN>`），逐页访问六个 `/app/admin/*` 路由。
结果：6/6 页 `aiPage=2`、`aiPanel=1`、`gradientOnPanel=true`、`rootGap=16px`，`pageErrors=[]`；query-explorer 显示真实数据行，无错误 toast。
证据：`test-results/r306-admin-b/evidence.json` 与六张 fullPage 截图；source 标注「Playwright Chromium headless against deployed test server; not a physical device」，非物理真机。

## 备注

修复漏 import 后，预存的外层 `Admin*` 组件层与 AI 改造层同时呈现，功能完整（搜索框经 `v-model` 同源联动）。保留全部原有业务逻辑，未删减任何操作入口。
