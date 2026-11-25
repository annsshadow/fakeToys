# R554 H5 回收站页 @tap 伪按钮键盘可达修复

- 编号：R554（台账 §544；R553 甄别判 P2，按真 bug 优先走 RED→GREEN）
- 执行：page-core；真父 HEAD b134a06b486f2de58a7572017dc131954175d637
- 规范：WCAG 2.1 SC 2.1.1（键盘）A，关联 SC 4.1.2（名称/角色/值）A
- 服务地址：仅用占位符 `<TEST_HOST>:<PORT>`；本仓公开，真实回环端口与造数不入库。

## 一、问题与方案取舍（Fail Loud）

回收站页 6 类 `@tap` 伪按钮仅有鼠标点击，无键盘可达与可访问名：错误态重试、有行错误 banner 重试、空态刷新、空态去文档、列表项查看、详情关闭。
- 方案 A（全局增强器）：Fail Loud 排除——历史 R540/R544 证实全局扫描会越界他页，且动态名（如「查看 + 条目名 + 详情」）无法从 DOM 可靠派生；main.ts 的 R537 增强器仅处理宿主表单控件，不覆盖此类动态伪按钮。
- 方案 B（逐页显式补）：采纳。对 6 类锚点就地追加 `role="button"` + `tabindex="0"` + `aria-label`（含动态绑定）+ `@keydown.enter` / `@keydown.space.prevent`，复用原 `@tap` handler，业务零改。先例 R542/R544/R546/R548/R550/R552 一致。

## 二、写域（精确 3 文件，禁止越界）

- M `oa4rust/mobile/src/pages/recycle/recycle.vue`
- A `oa4rust/mobile/src/pages/R554.recycleKeyboardA11y.contract.test.ts`
- A `oa4rust/docs/ops/oa4rust-r554-recycle-keyboard-fix.md`（本文）

零改约束：业务 handler（`load` / `showDetail` / `goDoc`）函数体、`recycleApi`、原 `@tap`、样式、他页、原生 button 族（彻底删除 :172 / 清空回收站 :183，属 uni-button 另一族，按 R545 排除）、main.ts 与 R537 增强器、calendar:295 正面对照，均未改动；`@tap` 全部保留，鼠标零回归。

## 三、源码修复（6 处，属性追加）

- :153 错误态 retry、:167 有行错误 banner retry：`role=button tabindex=0 aria-label=重试`，Enter/Space 复用 `load()`。
- :160 空态刷新：`aria-label=刷新`，复用 `load()`。
- :161 空态去文档：`aria-label=去文档`，复用 `goDoc`（无括号）。
- :170 列表项 card-title：`:aria-label="'查看' + nameOf(r) + '详情'"`，复用 `showDetail(r)`。
- :178 详情关闭 detail-close：`aria-label=关闭详情`，复用 `detail = null`。

六处均保留 `@tap` 与原文案，仅追加键盘与语义属性。

## 四、RED→GREEN 契约

契约 10 用例 = 5 核心（6 类锚点的 role/tabindex/aria-label/@keydown.enter/@keydown.space.prevent 复用原 handler）+ 5 守卫（业务 handler 零改 / `@tap`+recycleApi / 原生 button 零改 / main.ts 增强器零改 / 样式+calendar:295 零改）。
- RED：真父版 recycle.vue（sha 73cb67ba…）覆盖后 `5 failed | 5 passed`，exit 1。
- GREEN：修复版（sha b5968f1c…）恢复后 `10 passed`，exit 0。

## 五、三 verify（workdir `oa4rust/mobile`）

- V1 定点：`pnpm test src/pages/R554.recycleKeyboardA11y.contract.test.ts` → 10 passed，exit 0。
- V2 全量：`pnpm test` → 88 files / 619 tests passed，exit 0。
- V3 类型：`pnpm typecheck`（`tsc --noEmit`）→ 无错，exit 0。

说明：字面量命令 `cd oa4rust && pnpm vitest run mobile/src/pages/...` 在本仓不可直接执行（vitest 以 `oa4rust/mobile` 为根），已按实使用等价命令并在本文披露。

## 六、E2E 决定性判据（真浏览器 Playwright）

390x844 DPR2、isMobile/hasTouch、自起 uni H5；CDP `getFullAXTree` 取 `properties[]` 中 `focused===true` 且非 RootWebArea 的节点；`focusProbe` 用 `page.evaluate el.focus()`（shadow DOM 内层 `DOM.focus` 会报 Element is not focusable）。
- 8 组（light/dark × withData/error/errorBanner/empty），names=12，namesBad=0，fatals=0，pageErrors=0。
- 6 类焦点节点 AX `role=button` 且 name 达标：重试 / 刷新 / 去文档 / 查看R554回收项0详情 / 关闭详情；`document.activeElement=UNI-VIEW|UNI-TEXT[role=button]`，isTarget=true、isBody=false（未探宿主 wrapper 假阳性，遵 R535 教训）。
- 注入对照：移除 aria-label 后 name 失配（R554回收项0），注入 R554-injected-card 后恢复；removedOk/injectedOk 均 true。
- 等价：Enter 开详情、Space 关详情、Empty Enter 触发刷新计数 +1；Tab 命中 card-title 10、detail-close 6、retry 10/6、empty-link 16。
- 正面对照：calendar:295 伪可达仍 AX role=button/name=新建日历，零回归（verdictPos 只验 AX 语义，因其无 tabindex 不进 Tab，遵 R549/R550 口径）。
- 鼠标零回归：点击 card 开详情、点击 retry 触发重载计数增加。

取证瑕疵（Fail Loud）：`errorBanner` 截图拍在经 Vue 实例触发第二次 load 之前，故其 png 与 `withData` 同 sha（列表态，未显示 banner）；retry167 的 AX focused 判据在触发后才采集并通过（namesBad=0）。截图不承载决定性判据，特此披露。

## 七、截图（780x1687，light/dark 同 sha，H5 无主题机制）

- withData：回收站标题 + R554回收项0 卡 + 彻底删除 + 清空回收站。
- error：回收站加载失败 banner + 重试。
- empty：空态卡片 + 刷新 / 去文档。

## 八、SHA256 表（0 真值，仅证据指纹）

| 文件 | sha256 |
| --- | --- |
| src/pages/recycle/recycle.vue | b5968f1cde0d45c7d2040c75d5a1ec79b111cf700e867f8253234c1fb27fe212 |
| src/pages/R554.recycleKeyboardA11y.contract.test.ts | f952f46b2d3d53cdf65926579f47209f80fee0a6d97876f22341518df51d780c |
| test-results/r554-e2e.cjs | 7ad6f047769e53dd938d4a585f82689cb53f9254c0f0b81c1a55aec1f21031e3 |
| test-results/r554/r554-ax-report.json | 5cfbd6890b7cad8c52a37877206d23a4db7ee155b974aef3f4fb27a760a1cd02 |
| test-results/r554-red.log | 59e92918c1ec5edf0003ed6d1df29b1d481add687dc410e584a442f149e71c0d |
| test-results/r554-green.log | d15ee384fb3d47e47ef8713651205e03c1647ddb1a9400981ca2c335e7faa7aa |
| test-results/r554-verify1.log | 84857cf660c5922a5d756501a0cde7bdfa0c398f912a77a7b6d573c2e20649d1 |
| test-results/r554-verify2.log | f67287dc87a0e8ad519dbe909b1b56de03ecdf51353679961b125747ff873463 |
| test-results/r554-verify3.log | 8366207267355d3e3d5bf3bf6e8c94c5f93f6078c34f08973fa2b38cdda6cc92 |
| test-results/r554/r554-withData-light.png | 7c6a5a783caf0482b0c7e005d713d86a83c04c758e5b191d8194d348569779c5 |
| test-results/r554/r554-error-light.png | c42c25a2995de2960959a9a782129765273393695eabf2e47a476fcb31c929ae |
| test-results/r554/r554-errorBanner-light.png | 7c6a5a783caf0482b0c7e005d713d86a83c04c758e5b191d8194d348569779c5 |
| test-results/r554/r554-empty-light.png | a703179b8bb4e1678a850492680ff4d95841a3c10165fbec7bd8eef14f821f1e |
| 真父 recycle.vue（RED 基线） | 73cb67bafa686c750807554fed7856ed4edd84cbf046ce4b23b510e8d9ba75b0 |

## 九、脱敏自检

- 全文无真实域名/IP/主机名/端口（端口用 `<PORT>`）、无密钥 token、无个人环境路径；证据与造数位于 gitignored `oa4rust/test-results/`，不入库。
