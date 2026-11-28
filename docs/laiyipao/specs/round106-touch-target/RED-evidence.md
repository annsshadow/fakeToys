# R106 契约 RED 证据（逐字）

- 轮次：R106 ｜ 任务：t297 ｜ attempt_id：34537dac-ec80-4e06-aac1-f0881fd72bf1
- 命令：`cd laiyipao/miniapp && npx vitest run src/styles/r106TouchTarget.contract.test.ts --reporter=verbose > ../../docs/laiyipao/specs/round106-touch-target/RED-raw.log 2>&1; echo RED_EXIT=$?`
- vitest 版本：v4.1.12 ｜ Start at 14:56:11 ｜ Duration 256ms（transform 25ms, import 38ms, tests 11ms）
- **RED_EXIT=1**（实施前预期失败）；Test Files 1 failed (1)；Tests **5 failed | 6 passed (11)**。

## 逐 it 结果

| # | it | 结果 | 根因 / 说明 |
| --- | --- | --- | --- |
| 1 | 缺口：.mg-row CSS 块含 min-height:88rpx | FAIL | 收到 `{display:flex;...padding:10rpx0;border-bottom:...}`，无 88rpx（index.vue:828） |
| 2 | 缺口：.elem-chip CSS 块含 min-height:88rpx | FAIL | 收到 `{display:flex;...gap:8rpx;}`，无 88rpx（index.vue:1147） |
| 3 | 缺口：.ann-all CSS 块含 min-height:88rpx | FAIL | 收到 `{font-size:22rpx;color:var(--fire);padding:4rpx04rpx16rpx;}`，无 88rpx（index.vue:1219） |
| 4 | 缺口：.dim-hint CSS 块含 min-height:88rpx | FAIL | 收到 `{font-size:22rpx;padding:0;min-height:auto;}`，无 88rpx（index.vue:934） |
| 5 | 缺口：.dim-hint 原 min-height:auto 须被替换 | FAIL | `.dim-hint` 仍保留 `min-height:auto`（index.vue:937），实施轮须改写为 88rpx |
| 6 | 不变量：四类键盘三件套齐备且同 handler | PASS | ann-all/dim-hint/mg-row/elem-chip 均 role=button+tabindex=0+@keydown.enter 同 @click |
| 7 | 不变量：common.css 两层 :focus-visible 焦点环兜底 | PASS | :where(...) + 通用 :focus-visible 各 1 处，outline 3rpx + offset 2rpx |
| 8 | 不变量：令牌三锚点原样 | PASS | tokens.css:14-16 三锚点齐全 |
| 9 | 不变量：业务 index.vue 零 --r106- | PASS | occurrences=0 |
| 10 | 不变量：surgical .mg-row/.elem-chip 保留 | PASS | display:flex / padding:10rpx0 / gap:8rpx 均在 |
| 11 | 不变量：surgical .ann-all/.dim-hint 保留 | PASS | font-size:22rpx / padding 声明均在 |

## 结论

- 5 个 FAIL 全为真实缺口（四类 CSS 缺 88rpx + .dim-hint 的 auto 未替换），无解析错误、无模块解析失败、无锚点定位失败（cssBlock/tagBefore 均成功定位）。
- 6 个不变量全 PASS，证明契约不误伤现有键盘语义、焦点环、令牌、surgical 声明。
- 原始逐字 stdout/stderr 见同目录 RED-raw.log（108 行）。
