# oa4rust R552 H5 会议页 @tap 伪按钮键盘可达修复

## 1. 任务与范围

- 轮次：t69 / R552（WCAG 2.1 SC 2.1.1，关联 4.1.2）；甄别来源 R551（P2）。
- 真父：`16ed9784e9a2ed3ffddfb2247798bca892963bb9`（t68 入账后实际 HEAD；台账文字 `75cc96790` 为 R551 甄别 commit，meeting.vue 源码 BLOB 相同，不影响 RED replay）。
- 写域精确 3 文件：`M oa4rust/mobile/src/pages/meeting/meeting.vue`、`A oa4rust/mobile/src/pages/R552.meetingKeyboardA11y.contract.test.ts`、`A oa4rust/docs/ops/oa4rust-r552-meeting-keyboard-fix.md`。
- 严格禁改：业务 handler（`switchTab` / `load` / `loadMore` 函数体）、`meetingApi`、样式、其他页面、`main.ts`、R537 增强器、页内原生 `<button>`（`:195/:196/:199/:200/:203/:204/:222` uni-button 另一族）、既有修复页、calendar:295 正面对照；原有 `@tap` 全部保留。

## 2. 方案取舍（Fail Loud）

- 采用方案 B（逐页显式补 `role="button"` + `tabindex="0"` + `:aria-label` + `@keydown.enter` + `@keydown.space.prevent` 复用原 handler），仅修改 6 处伪按钮开标签属性。
- 方案 A（全局增强器）排除：越界他页 + 动态名派生不准；既有 R540/R544/R546/R548/R550 同口径先例。
- `@tap` 保留；键盘路径与鼠标路径调用同一 handler，业务语义等价。

## 3. 源码修改（6 类伪按钮，仅开标签属性追加）

| # | 位置 | 控件 | aria-label | 键盘等价 |
|---|------|------|------------|----------|
| 1 | meeting.vue:159-169 | 页签 v-for（待接受/待确认/我的会议） | `t.label` | Enter/Space → `switchTab(t.key)` |
| 2 | :175 | 错误重试 `.tip-error .retry-btn` | 重试 | Enter/Space → `load()` |
| 3 | :182 | 空态刷新 `.empty-guide-link` | 刷新 | Enter/Space → `load()` |
| 4 | :183 | 空态换页签 `.empty-guide-link` | 换个页签 | Enter/Space → `switchTab(tab==='mine'?'wait':'mine')` |
| 5 | :211 | footer 重试 `.footer .retry-btn` | 重试 | Enter/Space → `loadMore()` |
| 6 | :214 | 加载更多 `.load-more` | 加载更多 | Enter/Space → `loadMore()` |

- `git diff` 仅属性追加（10 insertions / 5 deletions），业务零改。
- 修复版 `meeting.vue` sha256：`ec31f4480d9471eacf74409281bce7f9563162620aaba5959376f0607fcb0e4f`；真父版 sha256：`260bb7dfbdefa8985472453a8c63143517c71308aa28f3d5812b1ef29f2625c2`。

## 4. 契约与 RED→GREEN

- 契约 `R552.meetingKeyboardA11y.contract.test.ts` 共 12 用例：6 核心（6 类控件 role/tabindex/aria-label/@keydown.enter/@keydown.space.prevent + 复用原 handler）+ 6 守卫（业务 handler 零改、meetingApi 零改、样式零改、原生 button 零改、他页零改、calendar:295 正面对照零改）。
- String.raw 写入避免反引号转义；回调显式类型 `(l: string)` 避免 TS7006。
- 多行开标签定位教训：初版单匹配 class 行导致页签 5 failed；改 `blockWith2`（class + handler 同行）仍剩页签跨行失败；最终加 `blockWith`（起始锚点向下扫描到首行 `trimEnd().endsWith('>')`）专供页签，其余 5 类用 `blockWith2` → GREEN 12 passed。
- 干净 RED：备份修复版，git show 真父版覆盖源码，`pnpm test` 定点 → exit 1（6 核心 failed / 6 守卫 passed）。
- 恢复修复版 → GREEN exit 0（12 passed）。

## 5. 三 verify（workdir: oa4rust/mobile）

1. `pnpm test src/pages/R552.meetingKeyboardA11y.contract.test.ts` → exit 0，12 passed。
2. `pnpm test` → exit 0，87 files / 609 tests passed。
3. `pnpm typecheck` → exit 0，`tsc --noEmit` 无报错。

## 6. 真浏览器 E2E（决定性判据）

- 环境：Playwright Chromium headless，视口 390×844 DPR2，isMobile/hasTouch；自起 uni H5 dev server `pnpm dev:h5`（BASE `<TEST_HOST>:<PORT>`）。
- stub 四模式：`withData`（2 行）/ `error`（HTTP 500）/ `empty`（0 行）/ `mineMore`（50 行使 load-more 出现；第 2 页 500 触发 footer retry）。
- CDP `Accessibility.getFullAXTree`：取 `prop(node, 'focused') === true` 且非 RootWebArea/document/BODY 的节点，断言 `role=button` 且 `name` = 预期文案；同时校验 `document.activeElement` 为 `UNI-VIEW[role=button]`、`isTarget=true`、`isBody=false`。
- ★ 未探宿主 wrapper（uni-input 假阳性教训）；`focused` 在 `node.properties[]`，`name` 在 `node.name.value`。
- 注入对照：移除页签 `aria-label` → focused AX name 失配；注入 `R552-injected-tab` → 恢复；证实判据通道有效。
- 正面对照：跨页 calendar:295 `role=button` / `name=新建日历`（`verdictPos` 只验 AX 语义，不要求 focused，因该对照无 tabindex 不进 Tab）。
- 等价性：Enter 切页签（on 0→1）、Space 切空态页签（0→2）、Enter 触发 load-more 出现 footer retry；鼠标点击页签 0→1 零回归。
- 结果：`R552_E2E_DONE` names=22 / namesBad=0 / fatals=0 / pageErrors=0；8 组（light/dark × 4 模式）全绿。
- 6 类控件 name 覆盖：待接受 / 待确认 / 我的会议 / 重试 / 刷新 / 换个页签 / 加载更多 —— AX name 全非空且等于预期。
- 4 图 780×1687 目视正常（withData/error/empty/mineMore）；H5 无主题切换机制，light/dark 截图同 sha，据实披露不伪造暗色差异。
- Fail Loud：`mineMore` 截图在切换页签前拍摄，故与 `withData` 同 sha；load-more 与 footer retry 的存在与判据由切换后的 CDP `viewMine`（cards=50 / load-more=1）与 AX 判据坐实。

## 7. 证据 sha256 表

| 项 | sha256 |
|----|--------|
| meeting.vue（修复版） | `ec31f4480d9471eacf74409281bce7f9563162620aaba5959376f0607fcb0e4f` |
| meeting.vue（真父版） | `260bb7dfbdefa8985472453a8c63143517c71308aa28f3d5812b1ef29f2625c2` |
| R552 契约 | `128c2ced50f72bff6de3f8dc7f521171088e268a215ec5eb023791eea5529c3d` |
| r552-e2e.cjs | `83fe67091147a4cc3085b87cf92fe2a0f1052b13ca7a189707ee5c4222b51f7c` |
| r552-ax-report.json | `8bcd78cc14094877c416e3728e088373dd50122d42045865e892f8d42f289ea5` |
| r552-red.log | `fb7be1314d2f7aa2d39b0c4c1cf37219b73c394383c59144cc88dbf77bc61948` |
| r552-green.log | `7297c2cd03bd2dec3dfed1870fc3cda0384afca4c2ef5446988ef13af7da9ff7` |
| r552-verify1.log | `7d49feb237be2b1283e7c7a63c732efd506021f87d6490521af58b937705cca3` |
| r552-verify2.log | `6b5afa74f407c317ea30480e324f37c82e9033f05716d8d3e6ce473a2f013a7d` |
| r552-verify3.log | `8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92` |
| r552-withData-light.png | `dc946402fc99d8876cd8a042900ee1b3b8966c2bb6ec2dfe32fe31e4e60c04c3` |
| r552-error-light.png | `a2fdb57ba04b5d5a0808741a28fe102ed0dd6b2fdcc8ae2c807dd0d58055a6fb` |
| r552-empty-light.png | `5faf382dd5c4c9f3205155fe393338d87177ed09e6ec06a515aab01c9d7ab51c` |
| r552-mineMore-light.png | `dc946402fc99d8876cd8a042900ee1b3b8966c2bb6ec2dfe32fe31e4e60c04c3`（与 withData 同 sha，见 Fail Loud） |
| r552-e2e-run.log | `5233a469c8b10a3a87eb86437face72753d67f70dc79edcbb8fb684992bf1884` |

## 8. 脱敏声明

- 文档全程使用占位符 `<TEST_HOST>:<PORT>`，不含真实域名 / IP / 端口 / 凭据 / 个人目录 / 本机绝对路径。
- 证据文件位于 gitignored `oa4rust/test-results/`，不入库。
