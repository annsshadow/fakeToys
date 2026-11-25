# R531 抽屉焦点遮挡修复（WCAG 2.2 SC 2.4.11）

- 任务：t27（台账 §522，真父 `2610c1cc2b22cdd3efe24eb83d97546b1718ea1b`）
- 性质：实现修复，写域精确 3 文件、业务零改
- 环境问题：本地 Vite dev server + Playwright chromium headless，1440×900、DPR 1、light/dark 双主题
- 测试地址：`<TEST_HOST>`（占位；实际使用本地回环，真值不入文档）

## 一、缺陷与修复

R530 已判定程序中心详情抽屉（`ProgramCenterApp.vue:384` `.detail-drawer`）打开后不移焦、无 Tab 陷阱，键盘焦点可被右侧抽屉完全覆盖，违反 WCAG 2.2 SC 2.4.11 Focus Not Obscured (Minimum)（AA）。仓库已有 `modalFocus.ts` 全局覆盖层管理器（移焦/Tab 陷阱/Escape/关闭焦点回归），但该抽屉不在 `OVERLAY_SELECTOR` 中。

本次唯一源码改动：`oa4rust/frontend/apps/desktop/src/utils/modalFocus.ts:10`

```diff
-const OVERLAY_SELECTOR = '.modal-overlay,.upload-overlay,.paging-overlay,.modal-mask,.preview-overlay,.mo'
+const OVERLAY_SELECTOR = '.modal-overlay,.upload-overlay,.paging-overlay,.modal-mask,.preview-overlay,.mo,.detail-drawer'
```

该行末尾追加 `.detail-drawer`，使 `installViewModalManager` 自动纳管抽屉。抽屉自带 `role=dialog` 与 `:aria-label`，`findTitle` 因已有 aria-label 跳过命名，不破坏现有可访问名称；模板结构、@click、v-if、文案、样式、ProgramCenterApp 与其他页面均零改。

## 二、RED → GREEN 契约

契约：`oa4rust/frontend/apps/desktop/src/utils/R531.focusObscuredFix.contract.test.ts`，6 用例：
- 核心 1：抽屉获 `data-oa4-modal-managed`、`aria-modal=true`，原生 aria-label 保留（aria-labelledby=null）。
- 核心 2：打开后焦点移入抽屉、背景 `aria-hidden=true`、focusin/Tab 不落到后方按钮。
- 核心 3：源码 `OVERLAY_SELECTOR` 末尾追加且唯一一行。
- 守卫 4–6：ProgramCenter 抽屉模板/openDetail 业务零改；`modal-overlay` 既有家族 dialog/aria-modal/标题命名语义仍在；关闭回归链接与背景 aria-hidden 恢复不回归。

干净实证：将源码精确回退一行至真父 sha256=`df63f96767e6bf25f537257a2e33dd7e56cd808702f0c86640c21d562f42ed57` → `pnpm exec vitest run frontend/apps/desktop/src/utils/R531.focusObscuredFix.contract.test.ts` exit=1，3 核心 failed / 3 守卫 passed；重施修复 sha256=`f35c7b2dbae4694251178de27ce7398903eafc5bd990cb4e2d78b89eda38b9ea` → exit=0，6 passed。

Fail Loud：早期 RED 重放脚本因相对路径 `git show` 失误产生无效 RED，已废弃；上述结论以回退 sha 精确等于真父的日志为准。现有 modalFocus 契约族无一处钉死 OVERLAY_SELECTOR 精确字符串（仅注释文字），故未改任何现有契约；全量测试见下。

## 三、三 verify（均在 oa4rust/ 目录执行，全 exit 0）

| 命令 | 结果 |
|---|---|
| `pnpm exec vitest run frontend/apps/desktop/src/utils/R531.focusObscuredFix.contract.test.ts` | 1 file / 6 tests passed |
| `pnpm test` | 全量通过 |
| `pnpm typecheck` | `pnpm --recursive` 6 项目全 Done |

## 四、真浏览器双主题 CDP 实证

runner：`oa4rust/test-results/r531-e2e.cjs`（gitignored），本地 Vite dev 加载仓库源码，API stub 占位数据；1440×900、DPR 1，light/dark 双主题，0 pageerror、0 fatal。

| 观测点 | light / dark 一致结果 |
|---|---|
| 打开前 | 焦点在 `.detail-link`「R531智能体甲」，right=977.656 与抽屉 left=959 仅几何交叠 18.656px（coverage=0.045），非完全覆盖 |
| 打开后 | `activeElement` 为抽屉内「关闭」按钮；`insideDrawer=true`、`stillOnLink=false`、`obscured=false`（`document.elementFromPoint` 命中测试），焦点可见 |
| 抽屉语义 | DOM `role=dialog`、`aria-label=Agent · R531智能体甲 详情`、`aria-modal=true`、`data-oa4-modal-managed=""`；CDP partial+full AXTree 均 `role=dialog / name=... / ignored=false / properties.modal=true` |
| Tab 陷阱 | 连续 4 次 Tab 均停在抽屉内「关闭」按钮，`insideDrawer=true / visible=true`，不再落到后方被覆盖操作按钮（R530 时 100% 覆盖） |
| 关闭回归 | 抽屉数 0，焦点回到原触发 `.detail-link`（`isDetailLink=true`、`bodyFocus=false`） |
| 现有家族零回归 | 同页注入 `.modal-overlay` 仍自动 `role=dialog / aria-modal=true / aria-labelledby=oa4-modal-title-1 / marker=true / focusInside=true`，AX name「R531对照对话框」、`modal=true` |
| 截图目视 | light/dark 均 1440×900，右侧详情抽屉、关闭按钮焦点环、表格与侧栏布局正常 |

## 五、证据清单（sha256）

| 证据 | sha256 | 备注 |
|---|---|---|
| `oa4rust/test-results/r531-red-final.log` | `34d23fac82f9adaa422886d7f3bb824923fa4e9856f925c9bdeadea239831f06` | 真父 RED：3 failed / 3 passed |
| `oa4rust/test-results/r531-green-final.log` | `681248758b74866d76a47c9ae318ffba9b30eabdb9ac9c4b39a38ad6d5626692` | 修复后 GREEN：6 passed |
| `oa4rust/test-results/r531-verify{1,2,3}.log` | `772037f42ff25d3426e74425abf8812ac1793f62a499b5c41fb6b9212107f642` / `325cdca83ee6502d22dbbdc2620a38a461a99d734c296876a945e0e586182e98` / `d56dc7b2275bd732c01461b53e9205dff794b80f52f3f19c2c7cb140ad043075` | 三 verify |
| `oa4rust/test-results/r531-e2e.cjs` | `cbeffaebcab523eb844976453af7d26f417601c5d8ea3e97dae6e4fdd04ffe7b` | 双主题 CDP runner（gitignored） |
| `oa4rust/test-results/r531/r531-ax-report.json` | `07d99e215f4bf70a752bc15452a0576fcf7b8bc7e7f8a26c85519eb849fe9f8c` | 双主题主报告 |
| `oa4rust/test-results/r531/r531-detail-light.png` | `5c04a81f39e840720fba7f6bda435511d2e30123871dd179d4d79384c65e62e6` | 1440×900，155916 B |
| `oa4rust/test-results/r531/r531-detail-dark.png` | `75b1cf8fc8f9795fafbf0129198c8837a519fd9ad7093f96a4270bf3b10ac317` | 1440×900，178354 B |
| 修复后 `modalFocus.ts` | `f35c7b2dbae4694251178de27ce7398903eafc5bd990cb4e2d78b89eda38b9ea` | 唯一源码改动 |
| `R531.focusObscuredFix.contract.test.ts` | `a037f89e32f1d77a5ec69bf7f9d39fc295ab26b0f0d77533b94a58a649737f00` | 新增契约 |

脱敏自检：0 真值。文档与报告均用 `<TEST_HOST>`、占位用户/智能体名与本地回环，无真实域名/IP/凭据/个人信息；证据目录 `oa4rust/test-results/` 已被 `.gitignore:28` 排除。