# R305 P0B 搜索/文件/消息页 AI 令牌深度消费实测记录

> 本记录对应 t49（C3）。来源为测试服真实页面流程与 Playwright Chromium headless 截图，非物理真机。

## 范围与结论

- 目标页：`/app/search`、`/app/file`、`/app/im`。
- 校正口径：`/app/file` 与 `/app/im` 源码已有 `--ai-*` 令牌，但 DOM 无 `ai-page` 锚点；`/app/search` 为双重未消费。
- 结果：三页 DOM `[class*="ai-page"]` 数分别为 3 / 2 / 2；会话列表 11 项，原始 UUID 泄漏 0，t29 的「单聊/群聊·尾号」与 `data-testid="im-conv-N"` 未回退。

## 变更摘要

- `SearchApp.vue`：根与 header 加 `ai-page/ai-panel`，加载骨架 `ai-skeleton-line`，空态加业务示例引导；`var(--ai-*)` 覆盖背景、边框、阴影、按钮渐变、输入过渡。
- `FileManager.vue`：根与 hero 加 `ai-page`，骨架行复用 `ai-skeleton-line` shimmer。
- `IMChat.vue`：侧栏与主区各加 `ai-page`，骨架行 shimmer、主区渐变面板与 spring 过渡；保留 t29 UUID 遮蔽。
- `Pages.aiStyle.R305b.contract.test.ts`：三页 RED→GREEN 结构/令牌契约；真实缺口先行验证。

## 验证

- `pnpm exec vitest run frontend/apps/desktop/src/views/Pages.aiStyle.R305b.contract.test.ts`：RED 3 failed → GREEN 3 passed；连同 B2/B3 共 13 passed。
- `pnpm run typecheck`：通过。
- `pnpm --filter @oa4rust/desktop build`：通过，测试服原子部署成功。
- 真浏览器：登录测试服后逐页访问三页，页面错误 0；证据见 gitignored `test-results/r305-p0b-visual/evidence.json` 与 `01-search.png`、`02-file.png`、`03-im.png`。

## 安全与来源标注

- 测试服地址占位：`<YOUR_TEST_DOMAIN>`；凭据在测试运行时由远端凭据文件读取，不入库、不回显。
- 截图来源：Playwright Chromium headless 对测试服真实页面的截图，非物理真机。
