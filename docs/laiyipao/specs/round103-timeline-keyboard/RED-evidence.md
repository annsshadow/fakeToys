# R103 规范契约 RED 证据（逐字日志）

- 轮次：R103（t275 / attempt_id=45f4c7c2-3927-45b3-90d8-326ec1b59d56）
- 命令：`cd laiyipao/miniapp && pnpm exec vitest run src/styles/r103TimelineKeyboard.contract.test.ts --reporter=verbose`
- vitest v4.1.12；实测 exit=1；Test Files 1 failed (1)；Tests 8 failed | 4 passed (12)；Duration 249ms。
- 8 失败全为 §2 八处真实缺口（两处 CTA 三件套 / 抽屉 dialog+ESC+Tab / 关闭按钮 ref+三件套 / 三个 setup ref / open 焦点移入 / 触发源记录+close 焦点归还 / trapTimelineTab）；4 不变量（互斥 / R73 闭环 / 暂停记账 / 令牌+文案+@click.stop）全通过。
- 无解析/模块/超时错误；完整逐字 stdout（含各 Received 全量源片段）落 `docs/laiyipao/specs/round103-timeline-keyboard/RED-raw.log`。

## 契约自纠说明（如实披露）

- 首轮编写时 it6/it7 的焦点断言误写为 `toMatch(/focuss*(/)`，触发 `Error: Invalid regular expression: /focuss*(/: Unterminated group`（契约自身语法错误，非目标缺口），Tests no tests。
- 已将两处改为 `toContain('focus(')` 后复跑，得到下述干净的 8 failed | 4 passed RED；最终契约无解析错误。

## 逐字关键日志（超长 Received 行已截断，完整见 RED-raw.log）

```
 RUN  v4.1.12 D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/laiyipao/miniapp
 × src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it1 R28 feed CTA 补键盘三件套且保留 @click.stop 5ms
   → CTA1 须有 role="button": expected '<text class="r28-feed-cta" @click.sto…' to contain 'role="button"'
 × src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it2 结算页 timeline CTA 补键盘三件套且保留 @click.stop 1ms
   → CTA2 须有 role="button": expected '<view class="r29-result-timeline-cta"…' to contain 'role="button"'
 × src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it3 时间线抽屉补 dialog 语义 + ESC/Tab 守卫且保留 v-if/@click 1ms
   → 抽屉须有 role="dialog": expected '<view v-if="reactionTimelineVisible" …' to contain 'role="dialog"'
 × src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it4 关闭按钮补 ref + 键盘三件套且保留 @click 1ms
   → 关闭按钮须有 ref="timelineCloseRef": expected '<view class="r29-timeline-close" @cli…' to contain 'ref="timelineCloseRef"'
 × src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it5 setup 区新增三个 timeline ref 1ms
   → timelineOverlayRef 须以 ref(null) 声明: expected '<template>\n  <view class="page-root …' to match /timelineOverlayRef\s*=\s*ref(?:<[^>]*…/
 × src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it6 openReactionTimeline 末尾 nextTick 移入焦点到关闭按钮 1ms
   → openReactionTimeline 必须 nextTick 后移入焦点: expected '{\n  closeHelp()\n  if (reactionTimel…' to contain 'nextTick'
 × src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it7 触发源记录（currentTarget）与 close 焦点归还 3ms
   → openReactionTimeline 须接收事件参数 e: expected '<template>\n  <view class="page-root …' to contain 'function openReactionTimeline(e'
 × src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it8 新增 trapTimelineTab 复用 R73 模式并绑定 timelineOverlayRef 0ms
   → trapTimelineTab 必须存在: expected '' not to be '' // Object.is equality
 ✓ src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it9 不变量：openHelp/openReactionTimeline 双向互斥保留 0ms
 ✓ src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it10 不变量：R73 图例闭环不回退 0ms
 ✓ src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it11 不变量：timelinePausedByDrawer 暂停记账与 doPause/doResume 逐字不动 0ms
 ✓ src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it12 不变量：令牌三锚点、零 --r103-、@click.stop 与文案逐字不动 0ms
⎯⎯⎯⎯⎯⎯⎯ Failed Tests 8 ⎯⎯⎯⎯⎯⎯⎯
 FAIL  src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it1 R28 feed CTA 补键盘三件套且保留 @click.stop
AssertionError: CTA1 须有 role="button": expected '<text class="r28-feed-cta" @click.sto…' to contain 'role="button"'
Expected: "role="button""
Received: "<text class="r28-feed-cta" @click.stop="openReactionTimeline">"
 ❯ src/styles/r103TimelineKeyboard.contract.test.ts:59:42
 FAIL  src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it2 结算页 timeline CTA 补键盘三件套且保留 @click.stop
AssertionError: CTA2 须有 role="button": expected '<view class="r29-result-timeline-cta"…' to contain 'role="button"'
Received: "<view class="r29-result-timeline-cta" @click.stop="openReactionTimeline">"
 FAIL  src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it3 时间线抽屉补 dialog 语义 + ESC/Tab 守卫且保留 v-if/@click
AssertionError: 抽屉须有 role="dialog": expected '<view v-if="reactionTimelineVisible" …' to contain 'role="dialog"'
Received: "<view v-if="reactionTimelineVisible" class="r29-timeline-overlay" @click="closeReactionTimeline">"
 FAIL  src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it4 关闭按钮补 ref + 键盘三件套且保留 @click
AssertionError: 关闭按钮须有 ref="timelineCloseRef": expected '<view class="r29-timeline-close" @cli…' to contain 'ref="timelineCloseRef"'
Received: "<view class="r29-timeline-close" @click="closeReactionTimeline">"
 FAIL  src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it5 setup 区新增三个 timeline ref
AssertionError: timelineOverlayRef 须以 ref(null) 声明: expected '<template>\n  <view class="page-root …' to match /timelineOverlayRef\s*=\s*ref(?:<[^>]*…/
 FAIL  src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it6 openReactionTimeline 末尾 nextTick 移入焦点到关闭按钮
AssertionError: openReactionTimeline 必须 nextTick 后移入焦点: expected '{\n  closeHelp()\n  if (reactionTimel…' to contain 'nextTick'
 FAIL  src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it7 触发源记录（currentTarget）与 close 焦点归还
AssertionError: openReactionTimeline 须接收事件参数 e: expected '<template>\n  <view class="page-root …' to contain 'function openReactionTimeline(e'
 FAIL  src/styles/r103TimelineKeyboard.contract.test.ts > R103 时间线抽屉键盘闭环契约 > it8 新增 trapTimelineTab 复用 R73 模式并绑定 timelineOverlayRef
AssertionError: trapTimelineTab 必须存在: expected '' not to be '' // Object.is equality
 Test Files  1 failed (1)
      Tests  8 failed | 4 passed (12)
   Start at  05:28:32
   Duration  249ms (transform 24ms, setup 0ms, import 37ms, tests 16ms, environment 0ms)
```

## 收尾卫生

- HEAD=91e776228925d40df75ac87af4e1d71c8c4662d3（本轮只读未改）。
- 交付仅 3 文件：SPEC.md / r103TimelineKeyboard.contract.test.ts / RED-evidence.md（均 UNTRACKED）；RED-raw.log 为原始日志落同目录。
- 未改 battle.vue / tokens.css / 其它契约 / QA / .gitignore；业务源码零 --r103-；零真值。
