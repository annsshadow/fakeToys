# R73 战场图例弹层焦点管理与键盘闭环 — 规范（test-first RED）

- 轮次：R73（round 73）
- 任务：t157 / attempt_id=becefdcc-0756-4463-9c59-36d65e7ed469
- 角色：same-route-lead（worker，规范设计）
- 性质：规范 + test-first RED —— 只写本规范与契约两文件；不改业务源码 / 令牌 / .gitignore；不写真值、不写 git
- 基线 HEAD：def111dd8（R72 收官）
- 候选来源：docs/laiyipao/recon/ROUND72_A_FOCUS_TRAP_RECON.md 候选 A（队长已独立读源码坐实）

## §0 候选来源与缺口

候选 A = laiyipao/miniapp/src/pages/battle/battle.vue 的战场图例弹层（helpVisible / .legend-overlay）。真 Chromium 三视口（390×844 / 768×1024 / 1280×800）一致复现六项客观缺陷：

1. openHelp()（battle.vue:982-999）打开后焦点未移入弹层：activeInOverlay=false，焦点停 BODY。
2. 无焦点陷阱：Tab 在弹层（DIV.uni-scroll-view）与背景 BODY 间反复横跳。
3. 无 ESC 关闭：全仓 0 处 keydown.esc；press Escape 后 overlayOpen 仍为 true。
4. 无 dialog 语义：legend-overlay 无 role=dialog / aria-modal（全仓 0 处）。
5. closeHelp()（battle.vue:1001-1007）关闭不归还焦点到 help-chip。
6. 关闭入口不可键盘操作：legend-close（battle.vue:181）与 help-chip（battle.vue:112-119）均无 tabindex / role / @keydown。

四条件裁定：①真浏览器可复现客观缺陷=满足；②展示层可修=满足；③改动面小（单页单点）=满足；④权威口径=部分满足（有 R60 键盘基线作正面对照，但无弹层焦点管理权威实现/契约），故本规范在 §1 显式新建口径以补齐第④条。

## §1 权威口径与修复决策

口径一（复用 R60 键盘基线）：交互元素统一 role="button" + tabindex="0" + @keydown.enter，且 @keydown.enter 逐字复用各自 @click 的处理函数。全仓 48 处既有正面对照；本轮只给 help-chip 与 legend-close 补点，不改其它元素。

口径二（新建弹层焦点管理最小闭环，参照 WAI-ARIA dialog pattern，仅前端交互层）：
- a. legend-overlay 加 role="dialog" + aria-modal="true" + aria-label="战场图例"；
- b. help-chip 与 legend-close 补 role="button" + tabindex="0" + @keydown.enter（分别复用 openHelp / closeHelp）；
- c. openHelp() 记录触发元素，nextTick 后把焦点移入弹层关闭按钮（ref 取 DOM + focus()）；
- d. closeHelp() 恢复焦点到触发元素（help-chip）；
- e. ESC 关闭：legend-overlay 加 @keydown.esc="closeHelp"（或弹层内 keydown 守卫判断 Escape）；
- f. 焦点陷阱：弹层内 Tab 循环（@keydown.tab 守卫或 keydown 判断 Tab，首尾元素间循环，不逃逸到背景）。

不新增视觉令牌：令牌锚点 tokens.css:14 / :15 / :16 保持，业务源码零 --r73-。不改 engine / replay / hash / score / settle / loot / tokens 与后端。

## §2 精确改点（实施轮照此落地，本轮不改业务源码）

模板（laiyipao/miniapp/src/pages/battle/battle.vue）：
- battle.vue:177 legend-overlay 开标签：在 class="legend-overlay" 后追加 role="dialog" aria-modal="true" aria-label="战场图例" @keydown.esc="closeHelp"（Tab 陷阱可加 @keydown.tab=守卫函数名）；保留 v-if="helpVisible" 与 @click="closeHelp"。
- battle.vue:112-119 help-chip：在 aria-label="战场图例" 同标签内追加 role="button" tabindex="0" @keydown.enter="openHelp"；保留 :class 绑定与 @click="openHelp"。
- battle.vue:181 legend-close：在 class="legend-close" 同标签追加 role="button" tabindex="0" @keydown.enter="closeHelp"；保留 @click="closeHelp" 与 × 文案。

脚本（同文件）：
- openHelp()（:982-999）：在现有 closeReactionTimeline() / 暂停代劳 / helpSeen 逻辑之后，记录触发元素，import nextTick 并在 nextTick(() => 关闭按钮 ref.focus()) 把焦点移入弹层；既有 doPause() / helpPausedByPage = true 语义逐字不动。
- closeHelp()（:1001-1007）：在既有 helpVisible=false 与 if(helpPausedByPage){...doResume()} 之后（或之前保存引用后）恢复焦点到触发元素；既有暂停恢复语义逐字不动。
- 新增守卫函数：ESC（若不在模板用 @keydown.esc）与 Tab 焦点陷阱（弹层内首尾可聚焦元素循环），仅操作 DOM 焦点，不碰引擎与计时。

## §3 不变量（实施轮不得回退）

- doPause / doResume 调用与 helpPausedByPage 记账逐字保持：手动暂停不被图例误恢复（battle.test.ts:1089-1095）、阅读期间暂停与关闭后自动继续（:1062-1087）。
- openHelp() 先 closeReactionTimeline()：图例与 R29 时间线抽屉互斥（battle.test.ts:1030-1047 之外的 R29 语义）。
- help-chip / legend-close 的 @click 处理函数与图例显隐、helpSeen 首次记忆（uni.setStorageSync battle_help_seen）不变。
- 令牌三锚点 tokens.css:14 --lyp-token-version: lyp-tokens-v1、:15 --lyp-r3-version: lyp-tokens-v3-r3、:16 --lyp-r6-version: lyp-tokens-r6 不动。
- 前序 R55 / R60 / R61 / R71 / R72 已锁口径不回退（视觉可读性、键盘可达、表单可达、记录时间本地化、搜索空态溢出）。

## §4 契约清单

laiyipao/miniapp/src/styles/r73LegendFocus.contract.test.ts（真实 node:fs readFileSync 读 battle.vue 与 tokens.css，禁止内嵌实现副本；blockAfter 大括号配对提取函数块，语义定位而非纯文本顺序）：
- it1：legend-overlay 开标签含 role="dialog" + aria-modal="true"。
- it2：help-chip 与 legend-close 开标签均含 tabindex="0" + role="button" + @keydown.enter 且逐字复用 @click 处理函数。
- it3：openHelp 块含 nextTick 与 focus 调用（焦点移入），closeHelp 块含 focus 调用（焦点恢复）。
- it4：ESC 守卫（@keydown.esc 或 Escape 判断）+ Tab 焦点陷阱守卫（@keydown.tab 或 Tab/keyCode 判断或 trapFocus 函数）存在。
- it5：不变量 doPause / doResume / helpPausedByPage / openHelp 内 closeReactionTimeline() 互斥 / 令牌三锚点保留（当前即通过）。
- it6：业务源码零 --r73-（当前即通过）。

## §5 RED 预期

命令：cd laiyipao/miniapp && pnpm exec vitest run src/styles/r73LegendFocus.contract.test.ts --reporter=verbose
预期：exit=1，Tests 4 failed | 2 passed（it1-it4 为缺口失败，it5/it6 不变量通过）；失败均为缺口断言，无解析 / 模块 / 超时错误。

## §6 验证命令

- RED（本轮）：cd laiyipao/miniapp && pnpm exec vitest run src/styles/r73LegendFocus.contract.test.ts --reporter=verbose
- 实施轮 GREEN 后回归：cd laiyipao/miniapp && pnpm exec vitest run && pnpm exec vue-tsc --noEmit && pnpm run build:h5
- 实施轮真浏览器复查：三视口打开图例后焦点在关闭按钮 / Tab 不逃逸背景 / ESC 关闭 / role=dialog 与 aria-modal=true 就位 / 关闭后焦点回到 help-chip；并确认暂停恢复语义不回退。

## §7 诚实边界

- 本轮只写规范与契约两文件，未改 battle.vue 业务源码；契约断言的是实施后的目标形态，RED 阶段失败是预期信号，非业务故障。
- 第④条口径为本规范新建（同仓无弹层焦点管理权威实现），以 WAI-ARIA dialog pattern 最小闭环为据，并要求真浏览器复查坐实，不靠静态断言替代交互验证。
- 探针数据源自 t150 只读侦察（无后端开局 request:fail 不影响纯前端弹层焦点语义）；报告中的回环地址为本地回环，无真值。
- 契约不锁死具体变量名 / ref 名，只锁语义（nextTick+focus、Escape/Tab 守卫、ARIA 属性、键盘复用），避免因合理实现差异假失败；但语义缺失时必须失败。