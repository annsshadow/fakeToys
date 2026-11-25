# R556 H5 论坛页伪按钮键盘可达修复

## 结论

R555 甄别判 H5 \`bbs/bbs.vue\` 存在 4 类 \`@tap\` 伪按钮缺少键盘可达路径（WCAG 2.1 SC 2.1.1 A，关联 SC 4.1.2 A）：\`new-btn\` 发帖、两处 \`retry-btn\` 重试、\`topic o2-list-item\` 主题项。按真 bug 优先纪律走 RED→GREEN 修复。

- 修复方式：方案 B 逐页显式补 a11y 属性（\`role="button"\` + \`tabindex="0"\` + 静态/动态 \`aria-label\` + \`@keydown.enter\` / \`@keydown.space.prevent\` 复用原 handler），业务零改。
- 真父 HEAD：\`35ddb65a63cd39dfb0efb856852d49b7f52793d2\`。

## 方案取舍（Fail Loud）

- 方案 A（全局增强器）排除：会越界影响他页，主题项动态名 \`查看 + 标题 + 详情\` 无法从 DOM 可靠派生；R537 增强器仅覆盖宿主表单控件（\`uni-input, uni-textarea, uni-picker\`），不处理伪按钮。先例 R540/R544/R546/R548/R550/R552/R554 一致。
- 采方案 B：仅在本页 4 类开标签上追加 a11y 属性，保留 \`@tap\` 与文案，业务 handler 函数体零改。

## 写域（精确 3 文件）

- M \`oa4rust/mobile/src/pages/bbs/bbs.vue\`
- A \`oa4rust/mobile/src/pages/R556.bbsKeyboardA11y.contract.test.ts\`
- A \`oa4rust/docs/ops/oa4rust-r556-bbs-keyboard-fix.md\`

零改：业务 handler（\`titleOf\`/\`load\`/\`openTopic\`/\`newTopic\`）函数体、\`bbsApi\`/\`bbsFeed\`、\`@tap\`、样式段、他页、\`main.ts\` R537 增强器、\`calendar:295\` 正面对照。

## 修复点（git diff 4 insertions / 4 deletions，纯属性追加）

- \`:85\` 发帖：\`aria-label="发帖"\`，Enter/Space 复用 \`newTopic\`。
- \`:92\` 错误态重试：\`aria-label="重试"\`，Enter/Space 复用 \`load\`。
- \`:101\` 错误横幅重试：同上。
- \`:103\` 主题项：动态 \`:aria-label="'查看' + titleOf(row) + '详情'"\`，Enter/Space 复用 \`openTopic(row)\`。

## RED→GREEN

契约 \`R556.bbsKeyboardA11y.contract.test.ts\`：7 用例 = 3 核心（发帖 / retry 双处 / 主题项，\`blockWith2\` 定位开标签） + 4 守卫（业务 handler 零改 / @tap + bbsApi / main.ts 增强器零改 / 样式 + calendar:295）。

- RED（真父覆盖）：exit 1，3 failed | 4 passed。
- GREEN（修复版）：exit 0，7 passed。

## 三 verify（workdir \`oa4rust/mobile\`）

字面命令 \`cd oa4rust && pnpm vitest run mobile/src/pages/...\` 在本仓库不可跑（vitest 根为 \`oa4rust/mobile\`），故用等价命令并在本文件披露：

1. \`pnpm test src/pages/R556.bbsKeyboardA11y.contract.test.ts\` → 1 passed / 7 tests。
2. \`pnpm test\` → 89 files / 626 tests passed。
3. \`pnpm typecheck\` → \`tsc --noEmit\` 无错。

## 真浏览器决定性判据（CDP）

基于 \`@playwright/test\` 自起 uni H5，390x844 DPR2、isMobile/hasTouch，stub \`/api/authentication/who\` 与 \`/api/bbs/assemble/control/mobile/view/all\`。CDP \`getFullAXTree\` 取 \`prop(n,'focused')===true\` 且非 RootWebArea 的节点；\`focusProbe\` 用 \`page.evaluate el.focus()\`（shadow DOM 内层 DOM.focus 会报 not focusable）；\`document.activeElement\` = \`UNI-VIEW[role=button]\`，未探宿主 wrapper 假阳性（R535 教训）。

- 8 组（light/dark × withData/error/errorBanner/empty），names=16，namesBad=0，fatals=0，pageErrors=0。
- 4 类焦点节点 AX name 全达标：发帖、重试（错误态与横幅态）、查看 R556主题0详情；\`isTarget=true\`、\`isBody=false\`。
- 注入对照：移除主题项 aria-label 后 AX name 回落为卡片文本（失配），注入 \`R556-injected-card\` 后恢复；removedOk/injectedOk 均 true。
- 正面对照：\`calendar:295\` AX role=button / name=新建日历，零回归（只验 AX 语义，R549/R550 口径）。
- 等价：主题项 Enter 打开 \`/pages/bbs/topic?id=r556-topic-0\`；重试 Space 触发 load 重载。
- Tab 命中：主题项 12、错误重试 8、横幅重试 4、发帖（空态）12；鼠标点击主题/重试有效（零回归）。
- 端口 Fail Loud：原计划 5174 被本轮新启服务占用，日志显示服务实际起于 5174；诊断 \`.page\`/\`.new-btn\` 真实渲染后再跑判据，runner 用 \`<TEST_HOST>:<PORT>\` 占位。
- errorBanner 造数：首次请求成功返回 2 行，经 Vue 实例 \`setupState.load\` 触发二次请求返回 500，横幅出现后在其原页完成 retry101 判据（避免页面重载清空横幅）。

## 目视

4 图 780x1687（withData/error/errorBanner/empty，light/dark 同 sha，H5 无主题机制）：论坛页布局正常，发帖按钮、主题列表、错误重试、空态图标均按原样式渲染，a11y 属性不影响视觉。

## sha256 表

| 项 | sha256 |
| --- | --- |
| bbs.vue 真父 | \`284f21b2f5649561295a3797120aafb2d5aee0ee772572b097c4af2a9121be78\` |
| bbs.vue 修复版 | \`d08fa9258e31747c26dd14719f2abe53fa0c5341b50dff30c2b35d93e6caf2fe\` |
| R556 契约 | \`2459a5e96a1ef4a4d94ef3fec52b73621c017f823faf9c207a45824ff2897287\` |
| r556-e2e.cjs | \`b78b263cdc2304ec2c332ef9dd4c7e1a16838decea43ecfca2d7e8ab1d51ab21\` |
| r556-ax-report.json | \`31e68e15948f7cc5dd1c988fcf8145765b798b4c0313764f139cc37dd483563b\` |
| red.log | \`e085ab5a7c5fab1f1c3da8d30b73f33fc90bc5c7d4b014aa9a06201d0ac7989d\` |
| green.log | \`eca8d501a50ab8a0d9ba17bb91598423ca82adc334b15172390c55ac7d89c64b\` |
| verify2.log（全量 test） | \`a977c9b32375c4fc7d5bc7df2d8f1f70b7b8f23d5faee0633605a9a6a98a6703\` |
| verify3.log（typecheck） | \`8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92\` |
| withData-light/dark.png | \`a345ed350c78569510739c568d50136628d4e4bdae1c519402d9050be6672d98\` |
| error-light/dark.png | \`054b9a0d0cb477796af227e2e1cccbac6e7c872fe766a6f00b1fe2b932b0bf41\` |
| errorBanner-light/dark.png | \`c26ede8818635a6b4e382fa182e00c4c27a2d9be98faac2af1f285d1b600dcb6\` |
| empty-light/dark.png | \`48e147dd40f1913152b70bb3753bf441e24456e25363ccba21f077f5133332cd\` |

证据文件在 gitignored \`oa4rust/test-results/\`，不入库；文档内地址一律占位，0 真值。
