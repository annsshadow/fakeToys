# R560 H5 文档页（doc.vue）8 类 @tap 伪按钮键盘可达修复

> 缺陷：WCAG 2.1 SC 2.1.1（键盘）A 级，关联 SC 4.1.2（名称、角色、值）A 级。R559 甄别判 P2，按真 bug 纪律优先修复（RED → GREEN）。

## 1. 方案取舍（Fail Loud）

- 方案 A（全局增强器）排除：为 @tap 元素全局补 tabindex/role/keydown 会越界到本轮写域外页面，且动态 aria-label（`查看` + 条目名 + `附件`/`文档`）无法从 DOM 可靠派生，历史 R540/R544/R546/R548/R550/R552/R554/R556/R558 均据实 Fail Loud 排除全局增强器。
- 采用方案 B（逐页显式补）：在 `doc.vue` 模板的 8 处伪按钮开标签上就地追加 `role="button"` + `tabindex="0"` + 静态/动态 `aria-label` + `@keydown.enter` / `@keydown.space.prevent`，复用原有 handler，保留 `@tap`，业务零改。先例：R544 ~ R558 逐页修复全绿。

## 2. 写域（精确 3 文件）

- M `oa4rust/mobile/src/pages/doc/doc.vue`
- A `oa4rust/mobile/src/pages/R560.docKeyboardA11y.contract.test.ts`
- A `oa4rust/docs/ops/oa4rust-r560-doc-keyboard-fix.md`（本文件）

零改项：`load()` / `downloadAttachment()` / `downloadAndOpen()` 函数体、`fileApi`、`@tap`、样式、他页、`main.ts` R537 增强器、`calendar:295`。

★ 同页 5 处原生 `<button>`（模板编译为 `UNI-BUTTON` 自定义元素，属 uni-button 另一族、不同缺陷类）严禁纳入本轮改动：`:174` 拍照/相册、`:177` 上传文件（工具栏）、`:201` 拍照/相册、`:202` 上传文件（附件空态）、`:235` 上传文件（文档库空态）。契约守卫 + CDP 报告 `nativeBtns` 均钉死其 `role/tabindex/aria-label` 为 null。

## 3. 8 处修复明细（git diff 8 insertions / 8 deletions，纯属性追加）

| 行 | 控件 | 追加属性 | 复用 handler |
|---|---|---|---|
| :194 | 附件错误态 retry | role/tabindex/aria-label=重试/keydown | `load()` |
| :203 | 附件空态刷新 | role/tabindex/aria-label=刷新/keydown | `load()` |
| :208 | 附件 banner retry | role/tabindex/aria-label=重试/keydown | `load()` |
| :211 | 附件项（动态名） | role/tabindex/:aria-label=`'查看'+nameOf(row)+'附件'`/keydown | `downloadAttachment(row)` |
| :228 | 文档库错误态 retry | role/tabindex/aria-label=重试/keydown | `load()` |
| :236 | 文档库空态刷新 | role/tabindex/aria-label=刷新/keydown | `load()` |
| :241 | 文档库 banner retry | role/tabindex/aria-label=重试/keydown | `load()` |
| :244 | 文档项（动态名） | role/tabindex/:aria-label=`'查看'+nameOf(row)+'文档'`/keydown | `downloadAndOpen(row)` |

## 4. 契约 RED → GREEN

- 契约 `R560.docKeyboardA11y.contract.test.ts`：10 用例 = 8 核心（8 处标签各自 role/tabindex/aria-label/@keydown.enter/.space.prevent + 原 handler）+ 2 守卫（业务 handler + fileApi 零改；5 处原生 button 零改 + 无全局增强器/他页污染）。
- RED（真父 `656ec7936` 版 doc.vue，stash 覆盖取证）：EXIT=1，8 failed / 2 passed。
- GREEN（修复版）：EXIT=0，10 passed。最终契约校准后重跑 RED 仍为 8 failed / 2 passed，GREEN 10 passed。

## 5. 三 verify（workdir `oa4rust/mobile`，均 EXIT=0）

- V1 定点契约：10 passed。
- V2 全量 `pnpm test`：91 files / 645 tests 全绿。
- V3 `pnpm typecheck`（`tsc --noEmit`）：无错误。
- 说明：字面命令 `cd oa4rust && pnpm vitest run mobile/src/pages/...` 不可跑（vitest 根为 `oa4rust/mobile`，且 oa4rust 根 exclude `mobile/**` 会 No test files found），已用 `cd oa4rust/mobile && pnpm test ...` 等价命令完成并据实披露。

## 6. 真浏览器 CDP 决定性判据（8 组 light/dark × withData/error/errorBanner/empty）

- 自起 uni-app H5 开发服务，Playwright 390×844 DPR2 isMobile/hasTouch 真浏览器；CDP `Accessibility.getFullAXTree` 取 `properties[focused]===true` 且非 RootWebArea/BODY 的焦点节点。
- 6 类控件判据全绿：AX role=button / name=重试（附件顶/文档库顶/banner 3 处）、刷新（附件空态/文档库空态 2 处）、查看R560附件0附件、查看R560文档0文档；`document.activeElement` 为 `UNI-VIEW[role=button]`，`isTarget=true` / `isBody=false`（未探宿主 wrapper 假阳性，R535 教训）。共 18 条名字判据 namesBad=0。
- Tab 可达：withData item 命中 24、error retry 命中 20、errorBanner retry 5 + item 20、empty 引导链接 20。
- Enter/Space 等价：Enter 触发 top retry / 附件项 downloadAttachment，Space 触发 banner retry / 文档项 downloadAndOpen；0 pageerror。
- 注入对照：移除 aria-label 后焦点节点 name 失配（裸文本 `📎 R560附件0 1.0 KB · u`），注入 `R560-injected-att` 后恢复，证明判据通道有效。
- 正面对照：`calendar:296 .compose-btn` tabindex=0 focusable=true AX role=button name=新建事件；`:295 .calendar-create-btn` tabindex=null focusable=false 仅 AX 语义 role=button name=新建日历（伪可达，据实披露不夸大）。
- 5 处原生 `<button>`/`UNI-BUTTON` 零改：empty 态采到 5 个，role/tabindex/aria-label 均 null，未被本轮属性污染。
- 鼠标零回归：withData 态鼠标点击附件项后仍触发原 handler（4→4，下载请求被 stub），视觉 4 态截图目视正常，light/dark 同 sha（H5 无主题机制）。

## 7. SHA-256 证据表（工作树相对路径；证据文件位于 gitignored `oa4rust/test-results/`）

| 对象 | SHA-256 |
|---|---|
| 真父 HEAD `656ec7936...` | 656ec7936790b284c99d25cfd6f9c7b96bacaae0 |
| 真父 doc.vue | 3e17484e11d2757a77cc7607231cd4f6061436695dec5bee3d89bf356f890370 |
| 修复版 doc.vue | 863e26b926e1bc67a638a8d064aa5376bf964b258ebc03279077f02ca5a82426 |
| R560 契约 | e4b906587ebeba5f9faeecbe1524258319777d522c4f51b56ace46d22fbb0475 |
| RED 日志 | a289af3c879f9cbb93ee90236d641c6e0443ce634341f30fb4b78d8f52fe9758 |
| GREEN 日志（最终校准后同 V1） | d2d2fbef412d6d9a7301e656cc2f97bb96311dafc32cc0549551f15a9fc96f27 |
| V1 定点日志 | d2d2fbef412d6d9a7301e656cc2f97bb96311dafc32cc0549551f15a9fc96f27 |
| V2 全量日志 | 685c4da00eaebbb69db6136b3eccaafa1f3c6e043e6c84fb01e9ceb2aaf6bf07 |
| V3 typecheck 日志 | 8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92 |
| E2E runner `r560-e2e.cjs` | 17437b19760de56e8ea6dff0eecf8823672ecce261a2f0c369a8402fc45fb757 |
| E2E 报告 `r560-ax-report.json` | 14e04e549d3049b464e34fbaae659f1d5b33276f6577ad2afc0131433f0b83c0 |
| withData 截图（light/dark 同） | 1c2e1be29260d64356366d5526c8f774a7a0ed8e7902e70b7312998b45ed8e31 |
| error 截图（light/dark 同） | 850ca609c20abc7a0372bf50554212bc2a17045ce2cb4642c68a8be31fbd1284 |
| errorBanner 截图（light/dark 同） | 5bb71c616013f6da78d3bfac30fc560dcf8708d8f25dbd5fe8c0a9a0d2c40083 |
| empty 截图（light/dark 同） | e2e5b866dfa15e7a56d32e9640b18fed1752241116aee69e704e5766b2c5c0b7 |

## 8. 脱敏

- 本文件 0 真值：所有测试地址/端口用 `<TEST_HOST>:<PORT>` 占位。E2E 报告 `base` 字段亦为占位符。
- 截图与证据文件在 gitignored `oa4rust/test-results/r560/`，不入库；入库仅上述 3 文件。

## 9. Fail Loud 披露

- 端口事实：本次 `<PORT>` 端口初始探测 `HTTP=000 CONNECT_FAIL`（空闲），自起服务后 `HTTP=200` 且页面 `#app` 有内容；无他组占用，未执行 Stop-Process。
- 契约迭代：初版因动态项标签选择器跨行匹配失败（GREEN 3 fail）与函数签名精确文本不匹配（1 fail），均据实修正断言定位（按行定位 + 精确 TS 签名），非放宽业务判据；校准后最终 RED 8 fail / 2 pass、GREEN 10 pass 复验成立。
- E2E 迭代：初版 `nativeBtns` 选择器用 `button` 且采集发生在跳 calendar 后导致 0 假阴性，修正为 `.page uni-button` 并在 doc 页采集后为 5（空态）/ 2（其余态），属性全 null；初版 error 态只 judge 1 个 retry，补 `retryFilesTop` 后 8 类全覆盖。迭代日志与最终报告 sha 均已列明。
- 注入对照移除 aria-label 后焦点节点 name 为裸文本 `📎 R560附件0 1.0 KB · u`（动态 name 由 aria-label 提供，符合预期），非本轮缺陷。

## 10. 结论

8 类 @tap 伪按钮均键盘可达（role/tabindex/aria-label/Enter/Space），业务零改，5 处原生 button 零污染，真浏览器 CDP 决定性判据 18/18 全绿，三 verify 全 exit 0，契约 RED→GREEN 证据链闭合。待 t86 终审。