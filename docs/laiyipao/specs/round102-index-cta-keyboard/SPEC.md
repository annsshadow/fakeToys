# R102-B 首页 CTA 键盘可达性补全 — 规范（test-first RED）

- 轮次：R102-B（round 102-B）
- 任务：t270 / attempt_id=899cfeb0-3455-4a8e-9d52-39e96ca2ef0e
- 角色：design-reviewer-wb（worker，规范设计）
- 性质：规范 + test-first RED —— 只写本规范与契约；不改业务源码 / 令牌 / .gitignore；不写真值、不写 git
- 基线 HEAD=5ec099e5508b853a4cc9b8936d09d76ec1020589（全程未变）
- 候选来源：docs/laiyipao/recon/ROUND102_B_INDEX_CTA_KEYBOARD_RECON.md（四条件全中，1 推荐候选）

## §0 候选来源与缺口

侦察报告坐实 index.vue 首页 5 处用户可感知的导航/恢复入口仍为裸 view / text，缺键盘三件套（role=button + tabindex=0 + @keydown.enter 同 handler），键盘用户（仅 Tab/Enter）完全无法触达。真 Chromium 三视口 390×844 / 768×1024 / 1280×800 DOM/AX/真实 Tab 三重坐实：三处 CTA role=null/tabIndex=null/cursor=auto、AX 树缺席、14 次真实 Tab 全部跳过；错误态重试（注入 guest 500 合成）同样 role=null/tabIndex=null。

四条件：①真实可复现用户可见缺陷=成立（错误态重试为唯一恢复路径，危害最大）；②展示层小改=成立（每处各补一行三属性，零新增 DOM/逻辑，不动 handler）；③权威口径明确=成立（R96/R99/R100/R101 同仓范式 + WAI-ARIA APG）；④改动面可控=成立（仅 index.vue 单文件 5 处）。

## §1 权威口径与修复决策

口径一（复用 R60/R95/R96/R99 键盘基线）：交互入口统一 role="button" + tabindex="0" + @keydown.enter，且 @keydown.enter 逐字复用各自 @click 的 handler。同仓正面对照：r3-empty-action 在 6+ 页统一补同款三件套（common.css:1346）；r99LinkKeyboard.contract.test.ts 19 处 link；本文件 :189-192 go(d.hint)（R99 已修）、:115-119 ann-item（R100/R101 已修）。焦点环 common.css:914/918 已覆盖 [tabindex]/:focus-visible。

口径二（WAI-ARIA APG）：交互入口应暴露 button 角色并可键盘操作；错误态重试作为唯一恢复路径尤其不可键盘不可达。

不新增视觉令牌：令牌锚点 tokens.css:14 --lyp-token-version: lyp-tokens-v1、:15 --lyp-r3-version: lyp-tokens-v3-r3、:16 --lyp-r6-version: lyp-tokens-r6 保持；业务源码零 --r102-。不改 handler 逻辑、store、engine、后端。

## §2 精确改点（实施轮照此落地，本轮不改业务源码）

全部位于 laiyipao/miniapp/src/pages/index/index.vue，仅补属性、不动 @click 与文案：

1. :40 错误态重试（最高优先）—— `<view class="btn btn-primary" style="margin-top: 20rpx" @click="retry">重试</view>` 开标签补 `role="button" tabindex="0" @keydown.enter="retry"`。
2. :53 新手引导卡 —— `<view v-if="store.maxStage === 0" class="card first-run-guide tap" @click="goStage">` 开标签补 `role="button" tabindex="0" @keydown.enter="goStage"`。
3. :78 AI 洞察去补强 —— `<view class="btn ai-cta" @click="go(aiInsight.hint)">去补强 →</view>` 开标签补 `role="button" tabindex="0" @keydown.enter="go(aiInsight.hint)"`。
4. :92 AI 洞察去战斗 —— `<view class="btn ai-cta" @click="goStage">去战斗 →</view>` 开标签补 `role="button" tabindex="0" @keydown.enter="goStage"`。注意 :240 action-card 已有同款三件套（R100 已修），不重复改。
5. :109 公告全部入口 —— `<text class="ann-all" @click="goAnnouncements">全部 {{ announcements.length }} 条 →</text>` 开标签补 `role="button" tabindex="0" @keydown.enter="goAnnouncements"`。

行号仅为定位参考，实施轮须按行内唯一属性串 / 文案锚点定位（上游插行不漂移）。

## §3 不变量（实施轮不得回退）

- :189-192 空维补导 go(d.hint) 已由 R99 覆盖三件套（role=button/tabindex=0/@keydown.enter="go(d.hint)"），本契约不得重复锁定也不得回退；实施轮不得改动该处。
- :115-119 ann-item 已由 R100/R101 覆盖三件套 + :aria-expanded="String(expanded.has(a.id))"，不得回退。
- 四处 handler 的 @click 必须保留：`@click="retry"` / `@click="goStage"`（:53 与 :92 两处）/ `@click="go(aiInsight.hint)"` / `@click="goAnnouncements"`；@keydown.enter 逐字复用同名 handler。
- 令牌三锚点 tokens.css:14-16 不动；业务源码零 --r102-。
- handler 逻辑：go(path)→uni.navigateTo、goAnnouncements()→公告页、goStage()→关卡页、retry()→await store.login()（index.vue:686-701）逐字不动。

## §4 契约清单

laiyipao/miniapp/src/styles/r102IndexCtaKeyboard.contract.test.ts（真实 node:fs readFileSync 读 index.vue 与 tokens.css，禁止内嵌实现副本；tagBefore 按唯一属性串/文案锚点向前找开标签、向后找最近的 '>'；不依赖行号）：

- it1-it5：§2 的 5 处 CTA 各自开标签含 role="button" + tabindex="0" + 保留 @click + @keydown.enter 同名 handler。
- it6：不变量——R99 go(d.hint) 三件套不回退。
- it7：不变量——R100/R101 ann-item 三件套 + aria-expanded 不回退。
- it8：不变量——四个 @click handler 在 index.vue 保留。
- it9：不变量——令牌三锚点 + 业务源码零 --r102-。

共 9 it（5 缺口 + 4 不变量）。

## §5 RED 预期

命令：cd laiyipao/miniapp && pnpm exec vitest run src/styles/r102IndexCtaKeyboard.contract.test.ts --reporter=verbose

预期：exit=1，Tests 5 failed | 4 passed (9)；5 失败全为 it1-it5 缺口断言（role="button" 缺失），4 不变量全通过；无解析 / 模块 / 超时错误。逐字日志落 docs/laiyipao/audits/round102-b-index-cta/RED-evidence.md。

## §6 验证命令与诚实边界

- RED（本轮）：cd laiyipao/miniapp && pnpm exec vitest run src/styles/r102IndexCtaKeyboard.contract.test.ts --reporter=verbose
- 实施轮 GREEN 后回归：cd laiyipao/miniapp && pnpm exec vitest run && pnpm exec vue-tsc --noEmit && pnpm run build:h5；并复跑 r60A11yKeyboard / r99LinkKeyboard / r100ClickableView / r101SemanticA11y 契约 GREEN。
- 实施轮真浏览器复查：三视口点 Tab 序列命中 5 处 CTA（错误态重试需注入 guest 500 或等价错误态）、Enter 触发同名 handler、焦点环可见；确认 :189-192 与 :115-119 既有三件套不回退。
- 诚实边界：本轮只写规范与契约两文件 + RED 证据，未改 index.vue 业务源码；契约断言的是实施后目标形态，RED 失败是预期信号非业务故障。错误态重试的真浏览器证据来自 t269 注入合成 guest 500（非真实服务故障），坐实的是客户端 DOM 属性与键盘不可达因果。行号为侦察时快照，契约按属性串锚点定位，容忍上游插行漂移。
