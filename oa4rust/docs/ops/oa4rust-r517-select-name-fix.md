<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->
# R517 ConfigDesignerApp 双 select 可访问名称修复（WCAG 2.1 SC 4.1.2 Name）

## 1. 缺陷与依据
R516 甄别（t426/t427 + 队长复核）判定 P2 真缺陷：`ConfigDesignerApp.vue` 工具栏两个 `.tb-select`
（配置格式 `v-model=editorLang`、编辑器主题 `v-model=editorTheme`）在双主题下 CDP AXTree
computed `axName` 均为空字符串，违反 WCAG 2.1 SC 4.1.2（Name）——控件有 role=combobox、
focusable=true、ignored=false，但无可访问名称，读屏用户无法辨识控件用途。
注入 aria-label 后 name 立即变化，坐实可通过命名属性修复；R516 注入前空名对照与 Settings
页 `select#cfgTz`（`axName=时区`）正面对照均留档。

## 2. 最小修复（业务零改）
仅给两个 select 各新增一行命名属性：
- `ConfigDesignerApp.vue:21` `<select v-model="editorLang" class="tb-select" aria-label="配置格式">`
- `ConfigDesignerApp.vue:26` `<select v-model="editorTheme" class="tb-select" aria-label="编辑器主题">`

完整 `git diff` 仅上述两行：`v-model` 绑定、`class=tb-select`、五个 `<option>` 与 value、
事件处理器、模板结构、插值、样式、其他页面一律零改；复用原生 aria-label，零新增运行时逻辑。

## 3. RED → GREEN（test-first 留档）
- 契约：`R517.selectNameA11y.contract.test.ts`，5 it（2 核心：两 select 各有非空可访问名；3 守卫：
  v-model/class/option 五值保留、toolbar/editor-layout 结构保留、@click 事件与 ref 默认值保留）。
- 真父 `e1549b858a140f53de0a5a4b64218d332f124fde` 定点 RED：2 failed（核心）| 3 passed（守卫），
  `VITEST_EXIT=1`（日志 `r517-red.log`，sha 见 §6）；失败正为两核心 hasName=false，守卫全绿无桩错。
- 修复后 GREEN：定点 5/5（日志 `r517-v1.log`）。

## 4. 三 verify（全 exit 0）
- V1 `cd oa4rust && pnpm vitest run frontend/apps/desktop/src/views/R517.selectNameA11y.contract.test.ts`：5 passed (5)。
- V2 `cd oa4rust && pnpm test`：333 files / 3213 tests 全 passed，0 failed。
- V3 `cd oa4rust && pnpm typecheck`：6 projects tsc --noEmit 全 Done。
- 构建 `cd oa4rust && pnpm run build`：exit 0，入口 `index-BYKbMMpH.js`。

## 5. 真浏览器实测（1440x900 DPR1 headless，非物理真机）
部署：本地 `frontend/dist/web` 打包 → scp → 同一归档 sha256 比对一致 → 远端解压原子替换；
`SHA_OK=0127ba76826d30373612aeb9b64a087d50490b8608bcc40a39b4e0617352d860`，`DEPLOY_R517_PC_FULL_OK`。
测试域名由环境变量 `<YOUR_TEST_DOMAIN>` 注入，真值不入库。

双主题（light/dark）真实登录态，进入 `<YOUR_TEST_DOMAIN>/oa4rust/app/admin/config-designer/`，
CDP `Accessibility.getPartialAXTree` 实测：
- 两 `.tb-select` axRole=combobox、axName 分别为 `配置格式` / `编辑器主题`（非空）、
  axIgnored=false、props.focusable=true，role/focusable 与修复前一致；
- DOM `aria-label` 属性存在，select.value 保持 json / dark（业务默认值零改）；
- 首 select 程序化 focus 命中 `select.tb-select|配置格式`；
- Settings 页 `select#cfgTz` 对照 axName=时区（零回归）；
- 汇总 `R517_DONE groups=2 checks=24 passed=24 failed=0 fatals=0 errs=0`，0 pageerror。
对照 R516 甄别报告：修复前两 select axName=空字符串；本报告坐实修复后非空。
截图：`pc-config-selector-light-r517.png`、`pc-config-selector-dark-r517.png`、
`pc-settings-cfgtz-light-r517.png`、`pc-settings-cfgtz-dark-r517.png`（均为 PC 1440x900 headless，非物理真机）。

## 6. 证据 SHA-256（gitignored `oa4rust/test-results/r517-*`）
| 文件 | sha256 |
| --- | --- |
| r517-select-name/report.json | 54cb92f11924551a037e986778f1ef4d71ece7cf9c9ae89bba719d1a550ad07c |
| pc-config-selector-light-r517.png | 15f63a5e4372f420561f03147dac4a6621bea746d8a5fa62e3091fe73aa5253a |
| pc-config-selector-dark-r517.png | 00c431cdd19ae515adba74912c89cb085925fac4d2534393e69f795f3d4b0c15 |
| pc-settings-cfgtz-light-r517.png | 582a93adfc297717a3203beee129ff4267fef358ba2798028addb27a88d9eb5a |
| pc-settings-cfgtz-dark-r517.png | 60ae08c41a77fec982795d98241478d20acc2eee1b01198976ecbe3b16fd7b48 |
| r517-red.log | fc93ae9e6086be22f9c64a86c9f407ce7a98ab6c6b483312f26e8e53fb92406c |
| r517-v1.log | 7bb3ce1f103bb3b1ead043e2519e98de6047c8b5730431badc00c7d9435c3f9e |
| r517-v2-full.log | 9647f3379e0dbd7c8c8868673057e3e546a06dec9cedf913ee82a86fb9a57600 |
| r517-v3-typecheck.log | d56dc7b2275bd732c01461b53e9205dff794b80f52f3f19c2c7cb140ad043075 |
| r517-build.log | 3dd4040fc6cd27ccdc4f25f4da0ec7c125243f2e60d9fd3a17984644dcf7a9d4 |

## 7. 交付与脱敏
精确 3 文件：`ConfigDesignerApp.vue`（M，仅两行 aria-label）、`R517.selectNameA11y.contract.test.ts`（A）、
本文档（A）。证据与 runner 脚本入 gitignored `oa4rust/test-results/r517-*`，不入库。
文档仅用 `<YOUR_TEST_DOMAIN>` 占位，域名/账号/口令真值全部走环境变量或 `.local.keys`，0 真值入库。