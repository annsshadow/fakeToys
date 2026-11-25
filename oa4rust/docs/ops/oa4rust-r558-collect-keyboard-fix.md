# R558：H5 收藏页（collect.vue）6 类 @tap 伪按钮键盘可达修复

- 轮次：R558（台账 §548）；工单：t81；实现：page-core。
- 基线：HEAD `1f24c14408cd07d4c8e5cbff29847ddd09ac2e53`（真父，R557 判 P2，按真 bug 纪律 RED→GREEN 优先修复）。
- 目标：WCAG 2.1 SC 2.1.1（键盘）A 级，关联 SC 4.1.2（名称、角色、值）A 级。修复「我的收藏」页 6 类 @tap 伪按钮无法用键盘聚焦/激活、且无 role/可访问名的问题。

## 1. 方案取舍（Fail Loud）

- 方案 A（全局增强器）：排除。理由：① 动态可访问名（查看/删除 + 条目 title）无法从 DOM 可靠派生；② 全局扫描会越界他页，与已修页/后续页冲突；③ R537 增强器仅处理宿主表单控件透传，不覆盖 @tap 伪按钮。先例 R540/R544/R546/R548/R550/R552/R554/R556 均 Fail Loud 排除。
- 方案 B（逐页显式补 a11y）：采用。对每个目标 @tap 伪按钮追加 `role="button"`、`tabindex="0"`、`aria-label`（含动态绑定）、`@keydown.enter` / `@keydown.space.prevent`，复用原 handler；业务零改。

## 2. 写域与零改边界

写域精确 3 文件：`oa4rust/mobile/src/pages/collect/collect.vue`（M）、`oa4rust/mobile/src/pages/R558.collectKeyboardA11y.contract.test.ts`（A）、本文档（A）。证据在 gitignored `oa4rust/test-results/`，不入库。
零改：业务 handler（`load` / `openUrl` / `removeRow` / `goBrowse`）函数体、collectApi、@tap（含 :107 的 `.stop`）、样式、他页、main.ts / R537 全局增强器、calendar:295。

## 3. 源码修复（6 处，纯属性追加）

- :86 / :100 重试：`role=button tabindex=0 aria-label=重试 @keydown.enter=load @keydown.space.prevent=load`。
- :93 去浏览内容：`aria-label=去浏览内容` 复用 `goBrowse`。
- :94 刷新：`aria-label=刷新` 复用 `load`。
- :102 收藏项：`aria-label=查看 + r.title + 详情`，复用 `openUrl(r)`。
- :107 删除：`aria-label=删除 + r.title`，复用 `removeRow(r)`；键盘事件加 `.stop`（`@keydown.enter.stop` / `@keydown.space.stop.prevent`），与 `@tap.stop` 冒泡语义等价，防父项误触复制链接。
- 全部保留原 @tap / 文案，业务零改。修复前 `collect.vue` sha=`e627b885361751205cd2fa401d643e530da2dcf7f7db967e0c7f839e1282b46c`，修复后 sha=`9080f93fc02ce033026388cc37929ee8bf10fb2655c46865bf9340f091997356`（git diff 6 insertions / 6 deletions）。

## 4. 契约 RED→GREEN

契约 `R558.collectKeyboardA11y.contract.test.ts` 共 9 用例：5 核心（retry 两处合并 / 去浏览内容 / 刷新 / 收藏项 / 删除）+ 4 守卫（业务 handler + collectApi + @tap 含 .stop / main.ts R537 / 样式 + calendar:295）。`blockWith2(source,a,b)` 定位开标签并向下扫到 `trimEnd().endsWith(">")`；`a11yAttrs()` 断言 role/tabindex/aria-label/keydown 全到位且复用原 handler。
RED（git stash 回真父版 `e627b885…`）：EXIT=1，5 failed / 4 passed；GREEN（修复版）：EXIT=0，9 passed。

## 5. 三 verify（workdir `oa4rust/mobile`，字面 `cd oa4rust && pnpm vitest run mobile/src/pages/...` 不可跑，vitest 根为 `oa4rust/mobile`，用等价命令）

- V1：`pnpm test src/pages/R558.collectKeyboardA11y.contract.test.ts` → 9 passed，EXIT=0。
- V2：`pnpm test` → 90 files / 635 tests passed，EXIT=0。
- V3：`pnpm typecheck`（tsc --noEmit）→ 无错误，EXIT=0。

## 6. E2E 决定性判据（真浏览器）

runner：`oa4rust/test-results/r558-e2e.cjs`，Playwright 390x844 DPR2、isMobile/hasTouch，自起 uni H5 于 `<TEST_HOST>:<PORT>`；CDP `getFullAXTree` 中 `prop(n,"focused")===true` 且非 RootWebArea 节点为判据；focusProbe 用 `page.evaluate` 调 `el.focus()`（shadow DOM 内层 DOM.focus 报 Element is not focusable）。stub 五态：withData（3 条 R558 收藏项）/ error / errorBanner / empty。
最终结果 `R558_E2E_DONE`：`names=12 namesBad=0 fatals=0 pageErrors=0`，8 组（light/dark × 4 态）。
- 6 类聚焦节点 AX `role=button`、name 全达标（重试 / 去浏览内容 / 刷新 / 查看 R558收藏项0 详情 / 删除 R558收藏项0）；`document.activeElement=UNI-VIEW[role=button]`、`isTarget=true`、`isBody=false`（未探宿主 wrapper，R535 教训）。
- 注入对照：removed 失配 / injected=R558-injected-card 恢复；正面对照 :296（`.compose-btn` tabindex=0 focusable=true name=新建事件）；:295（`.calendar-create-btn` tabindex=null focusable=false）据实披露为伪可达，不作为判据。
- 等价：Enter 打开收藏项 / Space 重试 / Space 刷新 / Enter 去浏览跳 browse / Enter 删除 .stop 只删不复制链接；Tab 命中收藏项 21 / 删除 10 / 重试 12 / 空态 16。
- 鼠标零回归：纯鼠标点击收藏项复制链接；纯鼠标点击删除先 `page.reload()` 真重载回 3 条干净状态再点 :107，light/dark 均精确 **before=3 → after=2**（`ok=true`）。
- 注：:107 的 Enter 验等价于同 case 内先删一条，随后注入对照及鼠标回归必须真重载（hash-only `page.goto` 为同文档导航不会重置 Vue 状态，已据此修正并坐实）。H5 无主题机制，light/dark 截图同 sha。

## 7. 截图目视

4 图 780x1687 已目视正常：withData（3 条收藏项+删除）/ error（重试）/ errorBanner（横幅+列表）/ empty（空态双按钮）。aria-label 不影响视觉。

## 8. sha256 表（脱敏，证据路径不含真值）

- collect.vue (修复版)：`9080f93fc02ce033026388cc37929ee8bf10fb2655c46865bf9340f091997356`。
- collect.vue (真父版)：`e627b885361751205cd2fa401d643e530da2dcf7f7db967e0c7f839e1282b46c`。
- R558 契约：`ce97acc22589b72b08be5ff8fc333d904c993df43a3d91824f20d656e08d59fa`。
- E2E runner：`ca8d22ad53539521455b9c690c69a1e3811e1976cb54ab43aab565b8f0dcb0e0`。
- E2E 报告：`c7d11d682fe58bd32eaa4ebe779d921c9a5f1d1364ec657399230361ad13d204`。
- RED 日志：`d67df4fd646b315040793e1a1df44fac76c7b0dbee6c2692e5d74f53144d61ed`。
- GREEN 日志：`7dc64c142ed870c40930e53d2726b4467c9fbf7457f5c54ed167755b1faf3fb6`。
- V1 日志：`c6b1ef9421d4147a5c8a71760abc8a76f6c5549d1b707e54e4d88cac9b7b70c1`。
- V2 日志：`8f4846718ac42a1ecf7fbce166b52092d600c1d4da3d03a95936facadd4a6a50`。
- V3 日志：`8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92`。
- withData 图：`ee4ab6c38c8dec44ab66aa7f9bbf5d4c64c062bb8f2239cc81cc94aac74e7860`。
- error 图：`6ccc72b01748d93c2de1a3f6607c027b1815a2c04458e85c0be2dcf404973f3a`。
- errorBanner 图：`d4bf8785560f05490f4a03ab1bdb587ba3b19027eac875e4533edde169a9b3f9`。
- empty 图：`cbae5e3d08c81c9b2276a9f3c103d6b0094494f6dbda229b8a1f66a2276f0ed6`。

所有地址均用 `<TEST_HOST>:<PORT>` 占位，0 真值入库。