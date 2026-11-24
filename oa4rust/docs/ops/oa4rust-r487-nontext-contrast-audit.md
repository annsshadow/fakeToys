# R487 WCAG 2.1 AA 1.4.11 非文本对比度（≥3:1）甄别报告

## 1. 概况

- 任务：t365（R487），只读甄别，不改业务源码；唯一交付本文档。
- 判据：WCAG 2.1 AA 1.4.11 Non-text Contrast，功能性组件边界与图形对象对相邻背景对比度 ≥ 3:1；严格区别于 1.4.3 文字对比度（4.5:1）与 R479 焦点环（不重复甄别）。
- 来源：Playwright Chromium headless，PC 1440×900 DPR1，**非物理真机**；light/dark 双主题，登录页 + dashboard/calendar/form/bbs/attendance/file 六页，共 14 组、216 个采样元素。
- 真值纪律：<YOUR_TEST_DOMAIN>/<TEST_HOST> 占位；测试服真域名仅存在于 gitignored runner，运行时从受控凭据源注入进程环境，不回显、不入库。

## 2. 判据与口径

- 功能性边界（须 ≥3:1）：输入框可见边框、按钮边界或实心填充、可交互控件边界；半透明边框须与其下所有背景层自底向上合成后再算对比度。
- 装饰性边界（豁免）：卡片/容器的 glass-card 装饰描边（--border-card 系 rgba(15,23,42,0.1) 等）不承担组件识别功能，1.4.11 明确豁免，仅记录不判缺陷。
- 合成算法：composite = channel×alpha + bg×(1-alpha)；相对亮度按 WCAG sRGB 线性化；ratio = (L亮+0.05)/(L暗+0.05)。比较对象为**元素边界视觉色 vs 外部相邻背景**（跳过自身填充层），不是边框色 vs 自身填充。
- 完全透明且无边界、靠文字内容传达的按钮（calendar 的 today-btn/cll-tab on/cal-create-entry 等 6 个）不属边界组件，记为 NA。

## 3. 令牌层与契约（静态依据）

- 契约测试 oa4rust/frontend/apps/desktop/src/utils/nonTextContrast.contract.test.ts（R189，130 行）：AA_NON_TEXT=3；themeBlock 解析 [data-theme] 块；BACKGROUNDS=[--bg-base,--bg-surface,--bg-elevated]；4 it 钉住 dark/light × subtle/active 各在三背景合成 ≥3:1，2 it 钉住 active 强于 subtle。
- 旧值（dark subtle rgba(0,212,255,.12)/active .35、light subtle rgba(0,0,0,.08)/active .4）合成仅 1.19–2.30:1，R189 已抬升为现行值。
- dark.css：--border-subtle rgba(0,212,255,0.5)（:40）、--border-active rgba(0,212,255,0.7)（:41）、--shadow-card 含 0 0 0 1px var(--border-subtle)（:58），背景 --bg-base #060913/:33、--bg-surface #0d1525/:34、--bg-elevated #131e33/:35。
- 浅色区：--border-subtle rgba(0,0,0,0.44)（:110）、--border-active rgba(37,99,235,0.85)（:111）、--bg-surface #ffffff/:100、--bg-elevated #f8fafc/:101。
- modern.css .glass-card border:1px solid var(--border-subtle)（:63）；LoginScreen.vue .form-input border:1px solid var(--border-subtle)（:420）、:focus 用 var(--color-primary)（:429）。

## 4. 方法与三处踩坑

1. page.evaluate 仅接受单参数：初版传 (kind,theme,key) 报 FATAL Too many arguments，改为 evaluate(({kind,theme,key})=>{...},{kind,theme,key})。
2. **transition 中间态假象（决定性）**：主题切换后仅等 250/300ms 采样，落在 CSS transition(0.15s) 中途，dark 被误读为浅色令牌 rgba(0,0,0,0.44)，v1/v2 出现 14 个假 fail。改等 1800ms 稳定后重跑（job bash-882830f2063f4581，R487_DONE groups=14 shots=14 items=216 errors=0）全部归零。
3. 判据修正：v1 把边框色与元素自身填充比较致实心主按钮 1:1 误判；v3 改为边界视觉色对外部背景，并剔除装饰卡边框。
- CDP getComputedStyleForNode / getMatchedStylesForNode 交叉验证：主题稳定后 dark 密码框 border=rgba(0,212,255,0.5)、light=rgba(0,0,0,0.44)，根令牌 login light subtle=#00000070、dark=#00d4ff80，命中 .form-input[data-v-d722fe53] border:1px solid var(--border-subtle)。

## 5. 逐组实测（v3 判据，外部背景合成后最小对比度）

| 主题 | 页面 | 输入框 min | 输入框代表 | 按钮数 | 按钮 min | 按钮代表 |
|---|---|---|---|---|---|---|
| light | login | 3.23 | form-input/请输入密码 | 4 | 3.23 | oauth-btn |
| light | dashboard | — | 无输入框 | 6 | 4.94 | handle-btn/去处理 |
| light | calendar | — | 无输入框 | 13 | 3.23 | nav-btn/‹ |
| light | form | — | 无输入框 | 18 | 3.22 | btn-sm/编辑 |
| light | bbs | — | 无输入框 | 3 | 3.25 | add-section-btn/+ |
| light | attendance | 3.19 | mi 搜索框 | 18 | 3.23 | cfg-mini/详情 |
| light | file | 4.34 | fi 搜索框 | 14 | 3.22 | modern-btn ghost |
| dark | login | 3.46 | form-input/请输入密码 | 4 | 3.46 | oauth-btn |
| dark | dashboard | — | 无输入框 | 6 | 9.40 | handle-btn/去处理 |
| dark | calendar | — | 无输入框 | 13 | 3.19 | cal-chip |
| dark | form | — | 无输入框 | 18 | 3.34 | btn-sm/编辑 |
| dark | bbs | — | 无输入框 | 3 | 3.46 | add-section-btn/+ |
| dark | attendance | 3.45 | mi 搜索框 | 18 | 3.45 | cfg-mini/详情 |
| dark | file | 4.51 | fi 搜索框 | 14 | 3.34 | modern-btn ghost |

- 汇总：INPUT_BORDER total=8 ok=8 fail=0；BUTTON_EDGE total=152 ok=152 fail=0；BUTTON_NA=6；整体最小值 3.19（>3:1）。
- 输入框对比度分布：3.19 / 3.23 / 3.45 / 3.46 / 4.34 / 4.51 / 5.08 / 10.65。

## 6. 技术观察（非缺陷）

- 卡片玻璃装饰描边：rgba(0,0,0,0.08) / rgba(15,23,42,0.1) 合成后约 1.19–1.23:1，低于 3:1，但属 WCAG 1.4.11 明确豁免的装饰性边界（不影响组件识别，内部文字与实心填充另按 1.4.3 达标），据实记录不判 P 级。CARD_DECOR total=42（ok 8 / 低对比 34）。
- 6 个透明无边界按钮靠文字传达语义，非边界组件，不计入失败。

## 7. 三分判定

- 可达真缺陷：0。功能性边界（输入框 8/8、按钮 152/152）合成后全部 ≥3:1（最小 3.19），无低视力难辨实证；transition 中间态假象已排除。
- 不可达 L1：0。
- 正常规范预期：若干（功能性边界全达标；装饰边框豁免；6 个文字按钮 NA）；另 1 项技术观察（装饰描边低对比）见第 6 节。

## 8. 截图与证据 sha256（命令行原始值，与 report.json 一致）

证据目录 oa4rust/test-results/r487-nontext-contrast/（14 PNG + report.json + cdp-detail.json，全部 gitignored）。

| 截图 | sha256 |
|---|---|
| pc-attendance-dark-r487.png | 85e665fc7e5fad10eae3b95d83cf41381eb3dc24858cdf2ee5c4cb972f582df7 |
| pc-attendance-light-r487.png | 678bf7a97db9f909f44902582cb3cc4a9dcaa670b3e68032f41dfc16ac91144b |
| pc-bbs-dark-r487.png | 6971592b8637568d54e01261e7a0b99ac5127ef79091e3d63d55ea5724d8082c |
| pc-bbs-light-r487.png | 888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105 |
| pc-calendar-dark-r487.png | 0075d87d57dc86a5bc3963fa095ecbabaa06562542fbaea5922b2234fe9f6c4b |
| pc-calendar-light-r487.png | 8d8694de37850992579bc46f4d3f52827abeb1fb5a96a36f0397f70fbfba5fad |
| pc-dashboard-dark-r487.png | bc3fd2745cc6cc67b5a227ee5b22b590a4875f1abea66c1ff06acc88359dd9a3 |
| pc-dashboard-light-r487.png | 872d918a7e5827282c77b84493882e3b381cd60e911479c4d4820d2a43aaa190 |
| pc-file-dark-r487.png | 54bb7cfec34f1e0e3e7eb683babd934fc965b0b6cd486dfb48a6e3fd41c12ff1 |
| pc-file-light-r487.png | 6d2230edbcdb7b4b62d40935be08903f9ae858640ce956c982d366636e21942e |
| pc-form-dark-r487.png | 7c675548961afd8d9226d6b166b641d1a21e13dc20b5afc30a2bd9d47275d7d2 |
| pc-form-light-r487.png | 577b73c3de9a05e9ab28d20d1551e3de8af93c0f607d2b23c904997db8c24f80 |
| pc-login-dark-r487.png | 31ba900e21b08b47208aa270d8c7c617983264516d0782a4fa301ae984c241e0 |
| pc-login-light-r487.png | f439ddfb2c2aca99164f966d6f21e2556929e7d332c520c7c829e2fc80c18ce9 |

目视 4 图（pc-login-light / pc-dashboard-light / pc-bbs-dark / pc-file-dark，均 1440×900 非物理真机）：登录输入框蓝框、仪表盘卡片边界、深色论坛与文件页控件边界均清晰可辨，无可见退化，与实测数据一致。

## 9. 真值纪律

- 全文仅含占位符 <YOUR_TEST_DOMAIN>/<TEST_HOST>，无真实域名/IP/口令/个人路径；扫描 0 命中。
- 截图来源如实标注为 Playwright Chromium headless 非物理真机，未冒充真实设备。