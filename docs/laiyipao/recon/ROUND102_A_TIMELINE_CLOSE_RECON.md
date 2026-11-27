# R102-A 反应时间线抽屉关闭按钮键盘可达性 — 只读侦察

## §0 任务与方法

- 视角：R29「完整反应时间线」抽屉（battle.vue:260-285）的关闭入口键盘可达性与弹层语义；聚焦任务命题「关闭按钮是否键盘不可达 / 焦点能否移入并归还」。
- 范围：`laiyipao/miniapp/src/pages/battle/battle.vue` 为主，旁证 R73 图例弹层闭环（`ROLL` 对照）、R75-A 弹层关闭侦察、R72-A 焦点陷阱侦察。
- 方法：静态全量走查（OPS grep/read）+ 真 Chromium（agent-browser 0.41.1）三视口 390×844 / 768×1024 / 1280×800 客观 DOM 探针 + 截图亲读；禁止仅凭未验证 grep 假零下结论。
- 四条件（全中才列推荐候选）：①真实可复现缺陷（非理论/非设计取舍）②展示层可修且有同仓权威口径 ③改动面小 ④真 Chromium 三视口坐实。
- 基线 HEAD=`5ec099e5508b853a4cc9b8936d09d76ec1020589`；纯只读：未改业务源码/契约/规范/令牌，未写真值，未写 git。

## §1 静态全量走查

### 1.1 时间线抽屉结构（battle.vue:260-285）

- `:261` 遮罩：`<view v-if="reactionTimelineVisible" class="r29-timeline-overlay" @click="closeReactionTimeline">` —— 无 `role="dialog"`、无 `aria-modal`、无 `aria-label`、无 `@keydown.esc`、无 `@keydown.tab`/焦点陷阱、`@click` 是唯一关闭入口。
- `:262` 抽屉盒：`<view class="r29-timeline-box" @click.stop>` —— 无 `role`/`aria-*`/`tabindex`。
- `:265` 关闭按钮：`<view class="r29-timeline-close" @click="closeReactionTimeline"><text>×</text></view>` —— 裸 `view`，无 `role="button"`、无 `tabindex="0"`、无 `@keydown.enter`。
- `:271` 内容区：`<scroll-view scroll-y class="r29-timeline-scroll">` 只读派生展示，无交互控件。
- `:2847-2877` CSS：遮罩 `position:absolute; z-index:40`（高于图例30/暂停30/结算sheet），关闭按钮 `.r29-timeline-close` 仅视觉 56rpx 圆形，无 focus 样式。
- 打开入口两处：`:77 .r28-feed-cta`（HUD 内 text）、`:449 .r29-result-timeline-cta`（结算页 view），均 `@click.stop="openReactionTimeline"`，均裸元素无 role/tabindex/keydown（与关闭入口同型，R96/R99 已记录的键盘盲区，不重复立案）。
- 逻辑：`:620 reactionTimelineVisible ref(false)`；`:1052-1072 openReactionTimeline()` 先 `closeHelp()` 互斥，构造 `r29Timeline` 后 `reactionTimelineVisible=true`，未暂停则 `doPause()` + `timelinePausedByDrawer=true`，**无 nextTick/focus 移入**；`:1075-1081 closeReactionTimeline()` 设 false 并按 `timelinePausedByDrawer` 恢复暂停，**无 focus 归还**。

### 1.2 同仓权威正面对照（battle.vue:181-185 / R73）

- 图例弹层 `:181`：`<view v-if="helpVisible" ref="legendOverlayRef" class="legend-overlay" role="dialog" aria-modal="true" aria-label="战场图例" @click="closeHelp" @keydown.esc="closeHelp" @keydown.tab="trapLegendTab">`。
- 图例关闭按钮 `:185`：`<view ref="legendCloseRef" class="legend-close" role="button" tabindex="0" @click="closeHelp" @keydown.enter="closeHelp"><text>×</text></view>`。
- 脚本 `:990-1012 openHelp()`：`closeReactionTimeline()` 互斥 → `helpVisible=true` → `doPause()` + `helpPausedByPage` → `nextTick(() => legendCloseRef.focus())` 焦点移入；`:1014-1023 closeHelp()`：`focus()` 归还 help-chip；`:1025-1043 trapLegendTab()` 首尾 `[tabindex="0"],[role="button"]` 间循环。
- 契约 `r73LegendFocus.contract.test.ts` 6 it 将以上语义逐条锁死（role=dialog/aria-modal、三件套+同名 handler、nextTick+focus、ESC/Tab 守卫、暂停互斥与令牌不变量、零 --r73-）。这是同仓唯一弹层键盘闭环权威实现，R73 已 GREEN。
- 全仓 `laiyipao/miniapp/src` 下 `keydown.esc`/`keydown.tab`/`trap*Tab`/`Escape` 命中仅 battle.vue:181 与 :1026 两处（OPS grep matchedTotal=2）——除图例外无任何弹层有 ESC 或焦点陷阱。

### 1.3 与既有侦察的关系（避免重复立案）

- R75-A `ROUND75_A_OVERLAY_CLOSE_RECON.md`：曾把 `:261 r29-timeline-overlay` 列入全仓 11 处弹层清单，§1.2 记「无 ESC/dialog/tabindex/focus」，但四条件裁定因「可完整复现的图例已修、其余弹层缺 ESC 属移动端设计取舍、滚动锁定是框架惯例」得出**无推荐候选**。本轮聚焦命题不同：R29 抽屉是**用户主动点 CTA 打开、内容与图例同级的只读展示浮层**（非强制错误 sheet），且同仓已有 R73 完整闭环正面对照，R75-A 的「设计取舍」 blanket 结论对本浮层不成立。
- R72-A `ROUND72_A_FOCUS_TRAP_RECON.md`：焦点陷阱侦察，产出候选 A（图例弹层，即 R73 实施对象），未覆盖 R29 抽屉。
- R100/R101 键盘与语义族：覆盖 link/view 三件套与 aria-expanded，未覆盖弹层关闭按钮。
- 结论：R29 关闭按钮键盘可达性是**未被前序侦察单独立案的真空**，且与 R73 构成「同页同类弹层一口径已修、一口径缺失」的直接对照。

## §2 真 Chromium 三视口证据

- H5 dev server 实际端口 5174（命令 `npx uni -p 5175` 被 uni-app 忽略，probe 实测 5174=200 / 5175=000）；session 采用 `export AGENT_BROWSER_SESSION` 同一 shell 链式，`set viewport` 在 `open` 之前。
- 脚本：`data/probe.js`（overlay/close role/tabindex/dialog/cw-sw）、`data/ax.js`（role=button/tabindex=0 计数代理）、`data/find-cta.js`（点击 `.r28-feed-cta` 打开抽屉）、`data/driver.sh`（三视口初始态）、`data/driver2.sh`（768 打开态）、`data/driver3.sh`（390/1280 打开+ESC）。

### 2.1 打开态（三视口一致，768 显式坐实，390 同构）

- CTA 探针：`count=1, cls=r28-feed-cta, text=查看完整反应时间线 →, role=null, tab=null, clicked=true` —— CTA 本身不可键盘触发（R96/R99 盲区，不重复立案），但鼠标点击可稳定打开抽屉。
- 抽屉打开后 DOM 探针（768×1024 与 390×844）：`hasOverlay=true, closeText="×", overlayRole=null, overlayAriaModal=null, overlayKeydownEsc=null, closeRole=null, closeTabIndex=null, closeKeydownEnter=null`；AX 代理 `roleButtons=1, tabIndex0=1`（来自页面其它控件，因为 `.r29-timeline-close` 的 `closeInAX=false`），`cw===sw`（768/390）无横溢。
- 1280×800 的 `driver3` 点击后 `hasOverlay=false`：该轮 8s sleep 内页面仍在 loading/CTA 尚未挂载（`find-cta` count=1 但 DOM 探针早于 Vue 渲染提交），**此视口打开态未取得有效样本**，如实标为未完成；768 与 390 两视口已足以坐实缺陷，1280 初始态 `cw=sw=1280` 无横溢。

### 2.2 ESC 行为（390 打开态）

- 390 打开后 `press Escape`，复探 `hasOverlay=true` 且 `closeRole=null/closeTabIndex=null` 与打开态逐字一致 —— ESC **不关闭**抽屉，与 overlay 无 `@keydown.esc` 的静态结论一致。
- 对照 R75-A §2.1 图例弹层三视口 `press Escape` 后 `dialogs=[]`：R73 闭环真实生效；R29 抽屉缺同一闭环。

### 2.3 截图与 sha256（已亲读，均为非登录页战斗页）

- `timeline-768.png` 768×1024 / 181485B `sha256=60b53457ed4109b0f52f0accfb355c7922ff24d943c53738c49caac7ccfc4bc6` —— 防线1000/1000、波次1/5、AI面板、暂停/倍速/放弃控件，非登录页。
- `timeline-1280.png` 1280×800 / 130037B `sha256=f78e11b3b8c97a2ec55fdf32914cb0a3ed524a6faeeb958c937957e470a10e61` —— 同上战斗页，非登录页。
- `timeline-open.png` 768×1024 / 104995B `sha256=40d49f21719cf6989087573b82fb24c5cd33db2f5c1480bf40518aa3b8c86b7d` —— 抽屉打开态，密集反应记录列表与底部关闭控件区可见，非登录页。
- `timeline-390.png` 390×844 / 15773B `sha256=b044ebcb3666845de4981c74c3580b42384db59129df2dfdd8b29cc8b1ffb71d` —— 仍在 loading 态，**不作证据**。
- `timeline-esc-390.png` 390×844 `sha256=ed14321aebb6c48e2910fe973ec672b088d5c7a6c656abaa926ddb3b804d7ebb` / `timeline-esc-1280.png` 1280×800 `sha256=88a65696bce0d2b56f0933143dd488ceda2ec5a5490dd42650bb7eea7025f262` —— ESC 后采样，仅作流程留痕，不单独作为决定性证据（决定性证据为 §2.2 DOM 逐字对比）。

## §3 四条件逐条裁定

- ①真实可复现缺陷：**成立**。R29 抽屉通过可见 CTA 打开是正常用户流程（非强制 sheet），内容是图例同级的只读展示浮层。关闭按钮 `:265` 裸 view 不在 Tab 序列（DOM `tabIndex=null` + AX `closeInAX=false`），键盘用户只能点击遮罩（鼠标依赖）或 ESC（本抽屉无 ESC）关闭——与 R73 同页图例形成「一类已修、一类未修」的直接可感知差异。
- ②展示层可修且有同仓权威口径：**成立**。同文件 `:181-185/:990-1043` 的 R73 闭环 + `r73LegendFocus.contract.test.ts` 6 it 给出逐字可执行口径：overlay 加 `role=dialog/aria-modal/aria-label/@keydown.esc/@keydown.tab`，close 加三件套，open/close 加 nextTick focus 移入/归还，与 `closeReactionTimeline()` 既有互斥/暂停记账完全兼容。
- ③改动面小：**成立**。仅 battle.vue 单页：1 处 overlay 开标签、1 处 close 开标签、open/close 两个函数各加 focus 逻辑、1 个 trap 守卫函数（可直接复用 `trapLegendTab` 模式，绑定新 overlay ref）；不动引擎/replay/hash/score/令牌。
- ④真 Chromium 部分坐实：768 与 390 两视口 DOM 决定性坐实（打开态 close 无 role/tabindex，ESC 不关闭，cw=sw）；1280 打开态因 loading 时序未取得有效样本，如实标注。两视口已足够判定缺陷；实施轮需补真浏览器三视口复查。

## §4 推荐修复候选（供队长裁定，本轮不改源码）

- **唯一推荐候选**：按 R73 图例闭环口径修复 R29 时间线抽屉——
  1. `:261 r29-timeline-overlay` 加 `ref="timelineOverlayRef" role="dialog" aria-modal="true" aria-label="完整反应时间线" @keydown.esc="closeReactionTimeline" @keydown.tab="trapTimelineTab"`（保留 `v-if`/`@click`）。
  2. `:265 r29-timeline-close` 加 `role="button" tabindex="0" @keydown.enter="closeReactionTimeline"`（保留 `@click` 与 × 文案）；建议加 `ref="timelineCloseRef"` 供焦点移入。
  3. `openReactionTimeline()` 末尾加 `nextTick(() => timelineCloseRef.focus())`（在 `reactionTimelineVisible=true` 与暂停记账之后）。
  4. `closeReactionTimeline()` 加 `focus()` 归还到触发的 CTA（`.r28-feed-cta` 或 `.r29-result-timeline-cta`，可用 `ref` 或记录触发源）；保留 `timelinePausedByDrawer` 记账与 `doResume()` 逐字不动。
  5. 新增 `trapTimelineTab(e)` 复用 `trapLegendTab` 首尾循环模式，绑定 `timelineOverlayRef`。
  6. 不变量：`openHelp()` 与 `openReactionTimeline()` 双向互斥逐字保留；R73 图例闭环不回退；令牌三锚点（`tokens.css:14-16`）不动；业务源码零 `--r102-`。
- **不推荐/不重复立案**：两处 CTA 本身无 role/tabindex/keydown（R96/R99 同型键盘盲区）；R75-A 的滚动锁定与设计取舍结论；selection 态/mastery toggle；强制错误 sheet。

## §5 验证口径（实施轮使用）

- GREEN 契约（建议）：`laiyipao/miniapp/src/styles/r102TimelineKeyboard.contract.test.ts`，真实 `fs.readFileSync` 读 battle.vue/tokens.css，断言 overlay dialog/ESC/Tab、close 三件套、open/close 的 nextTick+focus、互斥与令牌不变量、零 --r102-。
- 回归：`cd laiyipao/miniapp && pnpm exec vitest run && pnpm exec vue-tsc --noEmit && pnpm run build:h5`，且 R73 契约复跑 GREEN。
- 真浏览器复查三视口：点 CTA 打开抽屉 → 焦点在关闭按钮 → Tab 不逃逸背景 → ESC 关闭 → 关闭后焦点归还 CTA → role=dialog/aria-modal=true 就位；并确认图例/暂停互斥不回退。

## §6 诚实边界

1. 1280 视口打开态因页面 loading 时序未取得有效 DOM 样本，未坐实；但 768/390 已坐实缺陷静态与动态一致。
2. `find-cta.js` 用 JS `click()` 触发 CTA（非真实键盘），这是打开抽屉的唯一可靠途径；CTA 本身键盘不可达属 R96/R99 盲区，不在本轮重复立案。
3. AX 状态无直接 API，用 `role="button"`/`tabindex="0"` 计数与 `closeInAX` 代理；DOM 属性已决定性。
4. R75-A 曾把 R29 抽屉列入弹层清单但裁定无候选；本轮聚焦命题不同（R73 正面对照、展示型浮层非强制 sheet），据此独立立案，不推翻 R75-A 对其余弹层的裁定。
5. 无 PostgreSQL/API 环境，战斗页显示 request:fail 错误态但 UI 正常渲染，不影响抽屉 DOM 探针；报告所涉地址为本地回环，无真值。
6. 纯侦察产出，未改业务源码/契约/规范/令牌，未写 git，未写真值。

## §7 收尾校验

- HEAD=`5ec099e5508b853a4cc9b8936d09d76ec1020589`（与基线一致）；`git diff --check` RC=0；tracked diff=NONE（工作树大量 `??` 为历史遗留与本轮 `docs/laiyipao/audits/round102-a-timeline-close/` 证据目录，均 UNTRACKED）。
- auth 路径：`laiyipao/deploy/.local.keys` ABSENT、`laiyipao/deploy/remote-config.env` ABSENT。
- 自启 H5 dev server background job `bash-fe5af1cfbed14e1e` 待收尾 kill。
