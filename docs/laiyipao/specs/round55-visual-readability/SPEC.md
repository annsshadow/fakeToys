# R55 规范：768 反应图鉴溢出与背包占位对比度修复

- 版本标记：r55-visual-readability-v1（仅文档/测试标记；禁止新增 --r55-* 视觉令牌）
- 性质：test-first 规范 + 真实源码守卫；本轮不改业务源码，实施留待下一轮
- 来源：docs/laiyipao/recon/ROUND54_C_VISUAL_RECON.md（真 Chromium，12 页 × 3 视口，36 次探针）
- 审计数据（只读引用；仅相对路径，不含任何真值/URL）：
  - docs/laiyipao/audits/round54-visual-walk/data/loc2-768-reactions.json
  - docs/laiyipao/audits/round54-visual-walk/data/probe-768-reactions-reactions.json
  - docs/laiyipao/audits/round54-visual-walk/data/ph-bag.json
  - docs/laiyipao/audits/round54-visual-walk/data/ph-gems.json
  - 截图 r54-768-reactions-reactions.png（审计 data 目录，gitignored，按名引用）

## 0. 判定标准

四条件全中才立候选：①客观可测（具体数值/截图）②真浏览器可复现③修复不碰 engine/replay/hash/score/settle/loot/tokens/后端结算④非 R31–R54 已收口项。两候选均满足（来源报告 §3、§5）。本规范只规定展示层守卫，不做功能改动。

## 1. 候选1：768 视口反应图鉴 R14 空态块横向溢出 77px

### 1.1 客观证据（逐字数值）

- probe-768-reactions-reactions.json：innerW=768、docW=bodyW=845、hOverflow=77、truncCount=0；同页 390/1280 视口 hOverflow=0。
- loc2-768-reactions.json：overflowCount=10；最右 .r14-advice.r14-enter right=845（x=691,w=154）；其内 .r14-advice-badge / .r14-advice-empty right=822（w=102,x=720）；.r14-advice-empty-title/-msg/-cta right=804（w=68,x=737）。
- 同文件 r14[0]：CTA「去打一场战斗 →」display=inline-block、whiteSpace=pre-line、wordBreak=normal、rect h=633 w=68、parentW=102 —— 单行 CTA 被折成竖列；共 7 个同类 CTA（每反应一个）。
- 影响：768 视口下「AI · 可执行建议」空态与 CTA 被挤出可视区，运营看不到完整提示与「去打一场战斗 →」。

### 1.2 根因链（真实源码，带行号）

- laiyipao/miniapp/src/styles/common.css:776-780：uni-page-body { display:block; max-width:560px; margin:0 auto; min-height:100vh; } —— 桌面 H5 限宽居中。
- laiyipao/miniapp/src/styles/common.css:1259：r3-data-grid { display:grid; grid-template-columns:minmax(0,1fr); ... }；:1260-1270 .r3-data-grid .list-row, .r3-data-grid .r3-row { display:flex; align-items:center; gap:12rpx; ... } —— 无 flex-wrap、无 flex-direction:column。
- laiyipao/miniapp/src/pages/reactions/reactions.vue:195-196：view.r3-data-grid 内 v-for 的 view.rxn.list-row；:232-245 第 5 个子项是 view.r14-advice.r14-enter（空态 :241 .r14-advice-empty、:244 .r14-advice-empty-cta）。
- laiyipao/miniapp/src/styles/common.css:1737-1745：.r14-advice { margin:16rpx 0; padding:20rpx 22rpx; ... } —— 无 min-width:0、无 max-width:100%、无 box-sizing:border-box，在 flex 行内不能收缩。
- laiyipao/miniapp/src/styles/common.css:1772：.r14-advice-empty { margin-top:12rpx; padding:16rpx; ... } —— 无 min-width:0、无 overflow-wrap。
- laiyipao/miniapp/src/styles/common.css:1775：.r14-advice-empty-cta { display:inline-block; min-height:88rpx; line-height:88rpx; padding:0 20rpx; margin-top:10rpx; ... } —— inline-block 且无 white-space:nowrap，父容器仅 102px 时把 CTA 逐字折成 h=633 竖列。
- 机制：flex 行容器不折行且第五子项不可收缩，768 视口 rpx 放大后子项最小内容宽之和 = 845px > 768px，产生 77px 横向溢出；空态 CTA 在 68px 内容宽内竖排。

### 1.3 修复判据（GREEN 必须全部满足）

1. .r14-advice 规则含 min-width: 0 与 max-width: 100%，允许在 grid/flex 容器内收缩且不越卡片。
2. .r14-advice-empty 规则含 overflow-wrap: anywhere（或 break-word / word-break），长消息不再撑宽父容器。
3. .r14-advice-empty-cta 规则含 white-space: nowrap 与 max-width: 100%，CTA 保持单行横向、不再出现 h=633 竖排。
4. .r3-data-grid .list-row 规则含 flex-wrap: wrap 或 flex-direction: column，第五子项可折行/纵向堆叠。
5. 修复只在 laiyipao/miniapp/src/styles/common.css 的 R14 展示层规则块（LYP-R14-ADVICE-ACTION-V1）内进行；不改 template、不改 r14GoEmpty / r14AdviceOf。

## 2. 候选2：背包预设名 placeholder 对比度 4.03:1 且 768 截断

### 2.1 客观证据

- ph-bag.json：UNI-INPUT 占位色 rgb(128, 128, 128)、背景 rgb(26, 34, 51)、ratio=4.03；普通 INPUT 值色 rgb(230, 237, 243)、ratio=13.46。
- ph-gems.json：同结构 UNI-INPUT 占位色 rgb(150, 160, 173)、ratio=6（未告警）。
- 来源报告 §2：390/768/1280 三视口 bag 均 lowContrast=[{cls: uni-input-placeholder input-placeholder, ratio:4.03, size:12.48px/24.576px, t: 预设名（最多 20 字）}]；768 额外 trunc=[{cw:237, sw:240}]。
- 源码：laiyipao/miniapp/src/pages/bag/bag.vue:404-411 的 input（v-model=presetName、class=preset-input、placeholder=预设名（最多 20 字）、maxlength=20、:focus=presetFocus）—— 缺 placeholder-class 属性。

### 2.2 权威惯例对照

- laiyipao/miniapp/src/styles/common.css:383-385：.ph { color: var(--dim); }。
- laiyipao/miniapp/src/styles/tokens.css:32：--dim: #96a0ad = rgb(150,160,173)；背景 --panel-2: #1a2233（:21）= rgb(26,34,51)。真实 WCAG 复算 ratio=6.00 ≥ 4.5。
- 同仓 16 处已用 placeholder-class=ph：announcements.vue:91、enemies.vue:38、equipment.vue:151、gems.vue:87、login.vue:39/50、me.vue:91/574/583/620/629、reactions.vue:189、signin.vue:208、skills.vue:158、skins.vue:43、verify.vue:59。bag.vue 是唯一遗漏。

### 2.3 修复判据

1. 仅给 bag.vue:404-411 该 input 补 placeholder-class 属性值为 ph；v-model / maxlength / :focus / placeholder 文案逐字保留。
2. 不改 .ph 规则与 --dim 令牌（避免全局漂移）；不新增自定义占位色。
3. 768 下末字截断（cw237<sw240）由占位色改正后同轮复查；若仍存在，另立视觉轮次，不在本轮扩大改动面。

## 3. 不变量（GREEN 后必须保持）

- R14 空态行为锚点：reactions.vue:232-245 的 .r14-advice / .r14-advice-empty / data-r14-source=reactions / @click=r14GoEmpty(r14AdviceOf(r)) / 模板插值 empty?.headline 与 empty?.cta 保留（空态文案由服务端权威下发，源码不内嵌字面量）。
- R14 ready 态 #r14-advice-action-btn.r14-advice-cta[data-r14-source=reactions] 与两步确认浮层不受影响。
- 令牌三锚点：tokens.css:14-16 lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6 原样；全仓不出现 --r55-*。
- 服务端权威口径与 engine/replay/hash/score/settle/loot/tokens/后端结算零改动。

## 4. QA 三视端口径（GREEN 后真浏览器复查）

- 视口 390×844 / 768×1024 / 1280×800，页面 hash 路由 #/pages/reactions/reactions 与 #/pages/bag/bag。
- reactions：documentElement.scrollWidth === innerWidth（768 下 docW 由 845 降到 768、hOverflow=0）；.r14-advice-empty-cta 的 rect.h < 88px、rect.w > 100px 且单行；空态文案与 CTA 在可视区内可读；390/1280 回归 hOverflow=0。
- bag：三视口 placeholder 占色为 rgb(150,160,173)、WCAG 比值 ≥4.5；768 下 scrollWidth <= clientWidth 不再截断；预设保存/切换交互不变。
- 对比基线（不得被实现改动）：审计 JSON 内 768 hOverflow=77 / loc2.overflow[0].right=845 / bag ratio=4.03 保持原值，供回归对照。

## 5. 红线

- 禁改 engine / replay / replay_hash / score / settle / loot / tokens / 后端结算；禁新增路由或裸 hash。
- 禁新增 --r55-* 令牌；禁改三锚点版本字符串；禁写真值（域名/IP/口令/个人路径）；禁 git 写。
- 本轮只新增本 SPEC 与 laiyipao/miniapp/src/styles/r55VisualReadability.contract.test.ts；common.css / bag.vue / reactions.vue 的实现修复留待下一轮。

## 6. test-first RED 门禁

- 契约：laiyipao/miniapp/src/styles/r55VisualReadability.contract.test.ts（真实读源码 + 真实读审计 JSON + 真实 WCAG 数学，无内嵌参考实现）。
- 命令：cd laiyipao/miniapp && pnpm exec vitest run src/styles/r55VisualReadability.contract.test.ts。
- 期望：exit≠0，且失败仅来自「bag 预设 input 缺 placeholder-class」与「common.css 缺 R14 溢出守卫」两组真实缺口断言；不得为解析失败、模块缺失或超时。
- 实测结果：待本轮契约写盘后补录于 §7。

## 7. RED 实测记录

- 命令：cd laiyipao/miniapp && pnpm exec vitest run src/styles/r55VisualReadability.contract.test.ts --reporter=dot
- 首轮：exit=1，Test Files 1 failed(1)，Tests 3 failed | 1 passed(4)。第③条失败为假阳性——源码里空态文案是模板插值 empty?.headline / empty?.cta，由服务端权威下发、无字面量，故已把该断言改为校验模板绑定，并同步修正 §3 表述。
- 终轮：exit=1，Test Files 1 failed(1)，Tests 2 failed | 2 passed(4)。两条失败均为真实缺口断言（it2 WCAG 与 it4 不变量已通过）：
  - ① AssertionError: 预设名 input 缺少 placeholder-class="ph"（对比度 4.03 < AA 4.5）——断言预设名 input 块含 placeholder-class="ph" 失败。
  - ② AssertionError: .r14-advice 需 min-width:0 才能在 flex 行内收缩——断言 .r14-advice 规则含 min-width:0 失败。
- 无解析失败、模块缺失或超时；假阳性已消除，RED 完全来自两候选真实缺口。

## 8. 修订轨迹

- v1（本轮）：按 ROUND54_C 两候选立规范；只写 SPEC + 契约，实现留待下一轮。