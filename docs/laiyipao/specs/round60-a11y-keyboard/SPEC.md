# R60 规范：role=button 键盘可达性（WCAG 2.1.1）

## 0. 判定与依据

- 来源：docs/laiyipao/recon/ROUND60_A11Y_RECON.md（t100，真 Chromium + axe-core 坐实）。
- 权威依据：WCAG 2.1.1 Keyboard（所有功能必须可经键盘操作）；WCAG 2.4.7 Focus Visible（键盘焦点可见，现有 common.css:907 已覆盖 [tabindex]）。
- 本轮性质：规范 + test-first 契约（RED）；**不实施修复**，修复由下一轮 t102 完成。

## 1. 缺口机制（真实、可复现）

- miniapp H5 全站 48 处 role="button" 的 `<text>/<view>` 元素（分布于 announcements/battles/equipment/gems/mastery/reactions/shop/signin/skills/stage/verify 等页面）只有 @click、无 tabindex、无键盘激活处理、cursor=auto。
- 真浏览器实测（公告页 1280×800）：目标元素 tag=UNI-TEXT、class=r3-ai-cta、tabIndex=-1、hasAttribute(tabindex)=false、cursor=auto；a11y 树暴露为 `button「去看变更影响 →」`，但连续 12 次真实 Tab，activeElement 仅在 BODY 与 INPUT.uni-input-input 间循环，0 次落到该 CTA。
- 结论：a11y 语义声称是按钮、视觉是按钮样式，键盘却完全不可达——典型 WCAG 2.1.1 失败；cursor:auto 连鼠标可点暗示都缺失。
- 源码锚点示例：laiyipao/miniapp/src/pages/announcements/announcements.vue:36/71、reactions.vue:81/238/244/273-275、equipment.vue:64/74/76/117/127/133/243-245、battles.vue:151/282/288/342-344、skills/gems/mastery/stage/shop/signin/verify 同类写法。

## 2. 修复口径（供 t102 实施）

采用**逐元素显式属性**方案（非共享指令/composable）：每个 role="button" 的 `<text>/<view>` 元素同期补齐以下三项——

1. `tabindex="0"`：使其进入正常 Tab 序列（WCAG 2.1.1）。
2. `@keydown.enter`（或等价 Enter/Space 键盘激活）：键盘触发其原 @click 对应行为，不改行为语义。
3. `cursor:pointer`：在 common.css 增加 `[role="button"] { cursor: pointer; }` 全局规则，一处覆盖全站（勿逐元素重复）。
- focus-visible：复用现有 common.css:907 `:where(a, button, input, textarea, select, [tabindex]):focus-visible` 规则，**不重复声明**；补 tabindex 后焦点环自动生效（WCAG 2.4.7）。
- 契约 laiyipao/miniapp/src/styles/r60A11yKeyboard.contract.test.ts 按此口径校验：逐元素 tabindex + @keydown，全局 cursor:pointer，focus-visible 不重复。

## 3. 保护面（不可触碰）

- 不碰 engine/replay/hash/score/settle/loot/tokens/后端结算；不新增 --r60-* 样式令牌；令牌三锚点原样。
- 不改 CTA 文案与路由行为：R43/R44/R46/R47/R48 的言行一致性锚点须保持（契约已加不变量：battles r20 CTA @click=goStage、battle r29 CTA @click.stop=openReactionTimeline、index ai-cta @click=go(aiInsight.hint)、去战斗 @click=goStage）。
- 禁改 focus-visible 既有规则内容（只复用不重复造）；禁删任何原 @click。

## 4. 诚实边界

- axe-core 5 项结构性 violation（label critical 1、landmark-one-main、meta-viewport、page-has-heading-one、region 9 节点）属 H5 框架层/全站结构，修复需改框架模板与全局 meta，风险高，**不在本轮范围**，仅披露。
- color-contrast 55 节点为 axe incomplete（伪元素致背景无法确定），非确定 violation，不据此判定。
- r6-why-toggle 触摸目标 28~29px 属辅助展开文字观察项，非主 CTA，本轮不改布局。
- QA 库只读、无真值入库；本规范与契约不含任何密钥/令牌/真实服务器地址。

## 5. RED 门禁实测（test-first）

- 命令：`cd laiyipao/miniapp && pnpm exec vitest run src/styles/r60A11yKeyboard.contract.test.ts --reporter=verbose`，vitest v4.1.12。
- 首轮曾因契约注释内 `pages/**/*.vue` 的 `*/` 提前闭合块注释导致 esbuild `Unexpected "*"` 解析错误（非正当 RED）；已修正注释写法后重跑。
- 终轮退出码 exit=1，Test Files 1 failed(1)，Tests 3 failed | 3 passed (6)，Duration 263ms，无解析错误/模块缺失/超时。
- 三条失败（均为真实缺口断言）：
  1. AssertionError: 以下 role=button 缺 tabindex="0"，键盘 Tab 不可达: pages\announcements\announcements.vue:36 ...（共 48 处）: expected [ …(48) ] to deeply equal []
  2. AssertionError: 以下 role=button 缺 @keydown 键盘激活: pages\announcements\announcements.vue:36 ...（共 48 处）: expected [ …(48) ] to deeply equal []
  3. AssertionError: common.css 需含 [role="button"] 全局规则: expected -1 to be greater than or equal to 0
- 三条通过（不变量）：focus-visible 覆盖 [tabindex] 且未重复声明、已收口 CTA 言行锚点未改、令牌三锚点不变且无 --r60-。
- RED 完全来自三个真实缺口，假阳性/解析失败已清零。

## 6. 验收交接（t102 实施后应转 GREEN）

- 逐处补齐 tabindex=0 + 键盘激活后，it1/it2 转绿；common.css 加全局 cursor:pointer 后 it3 转绿；三条不变量须始终绿。
- 真浏览器复查：对 11 页各视口相同 Tab 序列，每个可见 role=button 均可被 Tab 聚焦且 Enter/Space 触发其 @click 行为；焦点环可见。
