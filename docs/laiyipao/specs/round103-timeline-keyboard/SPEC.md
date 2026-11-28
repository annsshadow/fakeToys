# R103 规范：R29 完整反应时间线抽屉键盘闭环（test-first RED）

## §0 候选来源与四条件裁定
- 候选来源：`docs/laiyipao/recon/ROUND102_A_TIMELINE_CLOSE_RECON.md`（t268，99 行，四条件全中）；队长 R102 裁定待办今收口。
- ①真实可感知缺陷：成立。真 Chromium 768/390 坐实打开态 close role=null/tabIndex=null、overlay 无 dialog、ESC 不关闭、AX 无关闭按钮。
- ②权威口径：成立。同页 :181-185 / :990-1043 的 R73 图例弹层已有完整 dialog+ESC+Tab+焦点闭环，契约 `r73LegendFocus.contract.test.ts` 锁死；R103 复刻同构口径。
- ③改动面：成立。仅 `laiyipao/miniapp/src/pages/battle/battle.vue` 单文件，8 处精确改点，不加状态不改业务逻辑。
- ④不破坏既有：成立。互斥、暂停记账、R73 闭环、令牌与文案全部列入不变量并由契约锁定。
- 裁定：采纳完整键盘闭环（dialog 语义 + ESC 关闭 + Tab 陷阱 + 焦点移入/归还 + 两处 CTA 三件套）。

## §1 权威口径
- R73 图例：`battle.vue:181` 遮罩 `role="dialog" aria-modal="true" aria-label="战场图例" @click="closeHelp" @keydown.esc="closeHelp" @keydown.tab="trapLegendTab"`；`:185` 关闭按钮 `ref="legendCloseRef" role="button" tabindex="0" @click/@keydown.enter="closeHelp"`；`:990-1043` openHelp 焦点移入 / closeHelp 焦点归还 / trapLegendTab 首尾循环。
- R60/R95/R99/R100 键盘三件套：`role="button" tabindex="0" @keydown.enter="同名handler"`，@click 保留。
- WAI-ARIA：展示型模态抽屉用 dialog + aria-modal；打开时焦点移入、关闭时归还触发源；Tab 不得逃逸。

## §2 精确改点（8 处，全部 battle.vue 单文件）
1. `:77` `<text class="r28-feed-cta" @click.stop="openReactionTimeline">` 补 `role="button" tabindex="0" @keydown.enter="openReactionTimeline"`；`@click.stop` 与文案「查看完整反应时间线 →」逐字不动。
2. `:449` `<view class="r29-result-timeline-cta" @click.stop="openReactionTimeline">` 补同款三件套；`@click.stop` 与文案逐字不动。
3. `:261` `<view v-if="reactionTimelineVisible" class="r29-timeline-overlay" @click="closeReactionTimeline">` 补 `ref="timelineOverlayRef" role="dialog" aria-modal="true" aria-label="完整反应时间线" @keydown.esc="closeReactionTimeline" @keydown.tab="trapTimelineTab"`；`v-if`/`@click` 保留。
4. `:265` `<view class="r29-timeline-close" @click="closeReactionTimeline">` 补 `ref="timelineCloseRef" role="button" tabindex="0" @keydown.enter="closeReactionTimeline"`；`@click` 保留。
5. setup 区（`legendCloseRef` 旁，约 `:984` 后）新增 `const timelineOverlayRef = ref<any>(null)`、`const timelineCloseRef = ref<any>(null)`、`const timelineTriggerRef = ref<any>(null)`。
6. `openReactionTimeline()` 末尾（`:1071` 后）加 `nextTick(() => { const el: any = timelineCloseRef.value; const node = (el && el.$el) || el; if (node && typeof node.focus === 'function') node.focus() })`。
7. 触发源记录与焦点归还：`function openReactionTimeline()` 改为 `function openReactionTimeline(e?: Event)`（模板 `@click.stop="openReactionTimeline"` 与新增 `@keydown.enter="openReactionTimeline"` 由 Vue 自动把事件作为首参传入，模板表达式无需改动），函数体内记 `timelineTriggerRef.value = (e && e.currentTarget) || (typeof document !== 'undefined' ? document.activeElement : null)`；`closeReactionTimeline()` 末尾在暂停记账之后从 `timelineTriggerRef.value` 取节点（`$el` 回退）并 `focus()` 归还。不动 `timelinePausedByDrawer` / `doPause` / `doResume`。
8. 新增 `function trapTimelineTab(e: KeyboardEvent)` 复刻 `trapLegendTab`（`:1026-1043`）：以 `timelineOverlayRef` 取根节点，`querySelectorAll('[tabindex="0"], [role="button"]')`，首尾循环，绑定见改点 3。

> 实现说明：改点 7 选「事件自动传参」方案，是为了同时满足 `@click.stop` 逐字不动与「接收事件记 currentTarget」的可测要求；两处 CTA 均为 focusable（tabindex=0），鼠标/键盘两路都能记录触发源。

## §3 不变量
- I1 `openHelp()` 首行 `closeReactionTimeline()`、`openReactionTimeline()` 首行 `closeHelp()` 双向互斥逐字保留。
- I2 R73 图例闭环不回退：`legend-overlay` dialog/ESC/Tab、`legend-close` 与 `help-chip` 三件套、`trapLegendTab`、`openHelp`/`closeHelp` 焦点管理全保留。
- I3 `timelinePausedByDrawer` 暂停记账与 `doPause`/`doResume` 逐字不动；`if (!paused.value && !settled.value)` 守卫保留。
- I4 `tokens.css:14-16` 三锚点不动；业务源码零 `--r103-`；两处 `@click.stop="openReactionTimeline"` 与文案「查看完整反应时间线 →」逐字不动。

## §4 契约清单（12 it = 8 缺口 + 4 不变量）
- it1 CTA1 三件套 + @click.stop；it2 CTA2 三件套 + @click.stop；it3 抽屉 dialog 语义 + ESC/Tab + v-if/@click；it4 关闭按钮三件套 + ref；it5 三个 setup ref；it6 open 焦点移入；it7 触发源记录 + close 焦点归还；it8 trapTimelineTab 复用 R73。
- it9 互斥不变量；it10 R73 闭环不变量；it11 暂停记账不变量；it12 令牌/文案/@click.stop 不变量。
- 契约 `r103TimelineKeyboard.contract.test.ts` 真实 `fs.readFileSync` 读 `pages/battle/battle.vue` 与 `styles/tokens.css`；`tagBefore` 按唯一 class 属性串锚点定位开标签、`blockAfter` 括号配对取函数体，均不依赖行号。

## §5 RED 预期
- 命令：`cd laiyipao/miniapp && pnpm exec vitest run src/styles/r103TimelineKeyboard.contract.test.ts --reporter=verbose`
- 期望 exit=1；8 个缺口 it 失败（缺 role/tabindex/@keydown.enter/ref/dialog/focus/trap 等），4 个不变量 it 通过（现网已满足）；无解析/模块错误。

## §6 验证命令与诚实边界
- GREEN 后复跑：R103 契约 12/12、R73 契约 6/6、R60/R99/R100/R101 契约、miniapp 全量 vitest、vue-tsc、build:h5、`git diff --check`；QA 三视口真 Chromium 验 dialog/AX/ESC/Tab/焦点闭环。
- 诚实边界：本规范只写 SPEC/契约/RED 证据，不改 battle.vue；实现轮须保真可测且不动暂停记账。游客态 0 战报时结果 CTA 可能不自然渲染，由契约静态保证并如实披露。
- 零真值：产物不得含任何密钥/域名/IP/个人路径。
