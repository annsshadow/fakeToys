# R301 H5 骨架屏与按压微交互实质升级（t17 [C8]）

- 轮次：R301（≥1000 轮持续迭代中的一环，非终态）
- 依据：t11 设计评审 findings 与 H5 逐页批判——多页加载态仍是裸「加载中…」灰字，R300 AI 令牌中的 shimmer 未被多数页消费；缺真实触控微交互。
- 目标：用结构化骨架屏替代裸加载文字，并把按压微交互真实挂到可点击入口；不改业务逻辑与接口。

## 变更
1. `mobile/src/App.vue`：新增 `.ai-skeleton-list`（列表骨架容器）、`.ai-skeleton-line.short`（短行变体）、`.ai-pressable`（transform/opacity 弹簧过渡）及 `:active` 按压态（scale 0.97 / opacity 0.82），并由 `@media (prefers-reduced-motion: reduce)` 守卫关闭动效。
2. 10 个页面共 14 处裸加载文字替换为结构化骨架占位（仅占位，不伪造业务数据）：attendance、calendar、browse、collect、doc（3 处）、bbs/topic、recycle、search、process/start、chat。
3. 按压微交互落地：首页 12 宫格 + 2 统计卡 + 公告空态入口；browse/collect/doc/meeting/recycle 空态引导链接。message 页因既有 R302 契约钉死 `class="empty-guide-link"` 字面量而未强加，保持兼容。

## 验证（test-first RED→GREEN）
- RED：新增 `mobile/src/pages/h5Skeleton.emptyState.test.ts`，12 用例断言骨架/按压类与逐页加载态替换；未实现时 12 failed（真实红）。
- GREEN：实现后契约 13 passed（含按压落地断言）；`pnpm --filter @oa4rust/mobile exec vitest run src/pages --reporter=dot` 46 files / 202 tests passed；typecheck 与 build 通过。
- 产物：`mobile/dist/build/h5/assets/index-CdSwBuzB.css` 含 `.ai-skeleton-list` 等新类。

## 测试服与真浏览器实测
- 部署口径：本地构建 → scp 至测试服 `<YOUR_TEST_HOST>` 的 `/usr/local/oa4rust/h5`（数字端口由环境注入）→ 时间戳备份 → 原子切换 → 远端 grep 新哈希（`index-CdSwBuzB.css`）。
- 真浏览器：headless Chromium 390×844 移动上下文，真实 UI 登录 → 逐页捕获骨架态 → search 真实交互。为使短暂加载态可见，仅对真实 API 响应做 2200ms 延迟（响应体不变），未伪造数据。
- 结论：`H5_SKELETON_OK`；9 步流程 bad=0 / pageErrors=0 / passed=true；7 页捕获骨架、首页 15 个按压入口且 computed 过渡生效、search 交互捕获骨架。
- 证据：`test-results/r301-h5-skeleton/`（evidence.json + 逐页 PNG + manifest.md，gitignored，不入库）；截图为真实浏览器渲染，非物理真机。

## 凭据与合规
- 账号/口令仅运行时从本机凭据脚本读取，不回显、不入库；host 用 `<YOUR_TEST_DOMAIN>` 占位。
