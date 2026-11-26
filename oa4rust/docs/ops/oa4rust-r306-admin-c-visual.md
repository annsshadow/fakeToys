# R306 C 组 admin 设计器 AI 令牌深度消费实测记录

> 本记录对应 t55（C3）。证据来源为测试服真实页面流程与 Playwright Chromium headless 截图，非物理真机。

## 范围与结论

- 目标六页：`/app/admin/portal-script-designer`、`portal-widget-designer`、`service-invoke-designer`、`find-designer`、`pdf-viewer`、`common`。
- 结论：六页由双未消费（源码 `var(--ai-*)=0` 且 DOM `aiPage=0`）转为令牌深度消费，真浏览器逐页 `aiPage=2`，`aiPanel=1~2`，页面错误 0。

## 变更摘要

- 六页根元素统一加 `modern-page ai-page`，头部 `view-header/script-hero` 加 `ai-page ai-panel`。
- 头部 `--ai-gradient-surface` + `--ai-glass-border`，面板 `--ai-glass-bg`，主按钮 `--ai-gradient-brand`，8px 栅格 `--ai-grid-1/2/3`，动效 `--ai-motion-fast` + `--ai-ease-spring`。
- 裸 Loading/spinner 骨架下沉 `--ai-skeleton-base` + `ai-shimmer`；空态补业务向 AI 引导文案。
- 仅改 template class/CSS/空态文案，未改 script、业务接口、路由、h1（标题去重契约保持通过）。

## 验证

- `AdminR306C.contract.test.ts`：RED 6 failed → 修正骨架选择器后 GREEN 6 passed（逐选择器 hasAiRule 真断言 + aiVarCount≥5）。
- 相关 7 个契约（标题去重/CRUD 守卫/liveQuery/autoquery/t4-admin-restyle）共 112 tests passed，无回归。
- `pnpm run typecheck` 通过；`pnpm --filter @oa4rust/desktop build` 通过。
- 测试服原子部署（备份 web.bak → web.incoming 原子替换 → 新哈希 grep 校验）后，真浏览器登录逐页 1440x900 截图，证据见 gitignored `test-results/r306c-admin-c-visual/` 的 6 张 PNG 与 `evidence.json`。

## 安全与来源标注

- 测试服地址占位：`<YOUR_TEST_DOMAIN>`；凭据在测试运行时由远端凭据文件读取，不入库、不回显。
- 截图来源：Playwright Chromium headless 对测试服真实页面截图，非物理真机。
