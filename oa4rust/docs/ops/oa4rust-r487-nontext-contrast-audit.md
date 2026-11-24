# R487 WCAG 2.1 AA 1.4.11 非文本对比度（≥3:1）甄别报告（t367 补正版）

## 1. 概况

- 任务：t365 甄别 + t367 证据补正（report.json 与文档判 0 结论名实不符须重跑统一）；只读甄别，不改业务源码；唯一交付本文档。
- 判据：WCAG 2.1 AA 1.4.11 Non-text Contrast，功能性组件边界与图形对象对相邻背景对比度 ≥ 3:1；严格区别于 1.4.3 文字对比度（4.5:1）与 R479 焦点环（不重复甄别）。
- 来源：Playwright Chromium headless，PC 1440×900 DPR1，**非物理真机**；light/dark 双主题，登录页 + dashboard/calendar/form/bbs/attendance/file 六页，共 14 组、216 个采样元素。
- 真值纪律：<YOUR_TEST_DOMAIN>/<TEST_HOST> 占位；测试服真域名仅存在于 gitignored runner，运行时从受控凭据源注入进程环境，不回显、不入库。

## 2. 判据与口径（v3，已 programmatic 内嵌 report.json）

- 功能性边界（须 ≥3:1）：输入框可见边框、按钮边界或实心填充、可交互控件边界；半透明边框须与其下所有背景层自底向上合成后再算对比度。
- 比较对象为**元素边界视觉色 vs 外部相邻背景**（跳过自身填充层 layers.slice(1)），不是边框色 vs 自身填充。实心填充按钮用填充色对外部背景；完全透明且无边界、靠文字内容传达的按钮/输入框记为 NA（不计数、不判失败）；卡片/容器装饰描边（--border-card 系）属 1.4.11 明确豁免，仅记录不判缺陷。
- 合成算法：composite = channel×alpha + bg×(1-alpha)；相对亮度按 WCAG sRGB 线性化；ratio = (L亮+0.05)/(L暗+0.05)。
- 判据已写入 E2E 脚本 r487-e2e.cjs 的 analysed 段并产出 report.json.judge 与 report.json.summary；分类计数/逐组 funcFail/groupMin/整体 funcFail/overallMin 均由脚本 programmatic 计算，非手写理想值。

## 3. 令牌层与契约（静态依据）

- 契约测试 oa4rust/frontend/apps/desktop/src/utils/nonTextContrast.contract.test.ts（R189，130 行）：AA_NON_TEXT=3；themeBlock 解析 [data-theme] 块；BACKGROUNDS=[--bg-base,--bg-surface,--bg-elevated]；4 it 钉住 dark/light × subtle/active 各在三背景合成 ≥3:1，2 it 钉住 active 强于 subtle。
- 旧值（dark subtle rgba(0,212,255,.12)/active .35、light subtle rgba(0,0,0,.08)/active .4）合成仅 1.19–2.30:1，R189 已抬升为现行值。
- dark.css：--border-subtle rgba(0,212,255,0.5)（:40）、--border-active rgba(0,212,255,0.7)（:41）、--shadow-card 含 0 0 0 1px var(--border-subtle)（:58），背景 --bg-base #060913/:33、--bg-surface #0d1525/:34、--bg-elevated #131e33/:35。
- 浅色区：--border-subtle rgba(0,0,0,0.44)（:110）、--border-active rgba(37,99,235,0.85)（:111）、--bg-surface #ffffff/:100、--bg-elevated #f8fafc/:101。
- modern.css .glass-card border:1px solid var(--border-subtle)（:63）；LoginScreen.vue .form-input border:1px solid var(--border-subtle)（:420）、:focus 用 var(--color-primary)（:429）。

## 4. 方法与三处踩坑（含 t367 补正根因）

1. page.evaluate 仅接受单参数：初版传 (kind,theme,key) 报 FATAL Too many arguments，改为 evaluate(({kind,theme,key})=>{...},{kind,theme,key})。
2. **transition 中间态假象（决定性）**：主题切换后仅等 250/300ms 采样，落在 CSS transition(0.15s) 中途，dark 被误读为浅色令牌 rgba(0,0,0,0.44)。改等 1800ms 稳定后重跑全部归零。
3. **t367 补正根因（名实不符）**：t365 原 report.json 的 analysed 段仍是 v1 判据——resolveBg(layers) 含自身背景层、borderFinal=comp(bc,bg) 对比自身填充，产生 14 项 functional=true 且 passAA:false（FUNC_MIN=1.0，全是 borderFinal==bgFinal 的实心按钮与透明按钮）；而 v3 判据只在独立 analyze3.cjs 重算、未回写 report，导致文档（取 v3 的 0 失败/3.19）与唯一证据 report.json（v1 的 14 失败/1.0）系统性矛盾。t367 已将 v3 判据内嵌 E2E analysed 段，重跑后 report.json.summary 与文档逐项一致。
- CDP getComputedStyleForNode / getMatchedStylesForNode 交叉验证：主题稳定后 dark 密码框 border=rgba(0,212,255,0.5)、light=rgba(0,0,0,0.44)，根令牌 login light subtle=#00000070、dark=#00d4ff80，命中 .form-input[data-v-d722fe53] border:1px solid var(--border-subtle)。

## 5. 逐组实测（v3 判据，边界视觉色对外部背景合成后最小对比度）

下表数值全部取自 t367 重跑后 report.json（node 脚本逐组 programmatic 导出），非手写。

| 主题 | 页面 | 输入框数 | 输入框 min | 输入框代表 | 按钮数 | 按钮 min | 按钮代表 | 装饰卡数 | NA 数 |
|---|---|---|---|---|---|---|---|---|---|
| light | login | 2 | 3.23 | form-input | 4 | 3.23 | oauth-btn | 1 | 0 |
| light | dashboard | 0 | — | 无 | 6 | 4.94 | handle-btn/去处理 | 4 | 0 |
| light | calendar | 0 | — | 无 | 13 | 3.23 | nav-btn/‹ | 3 | 3 |
| light | form | 0 | — | 无 | 18 | 3.22 | btn-sm/编辑 | 4 | 0 |
| light | bbs | 0 | — | 无 | 3 | 3.25 | add-section-btn/+ | 4 | 1 |
| light | attendance | 1 | 3.19 | mi 搜索框 | 18 | 3.23 | cfg-mini/详情 | 4 | 0 |
| light | file | 1 | 4.34 | fi 搜索框 | 14 | 3.22 | modern-btn ghost | 4 | 0 |
| dark | login | 2 | 3.46 | form-input | 4 | 3.46 | oauth-btn | 1 | 0 |
| dark | dashboard | 0 | — | 无 | 6 | 9.40 | handle-btn/去处理 | 4 | 0 |
| dark | calendar | 0 | — | 无 | 13 | 3.19 | cal-chip | 3 | 3 |
| dark | form | 0 | — | 无 | 18 | 3.34 | btn-sm/编辑 | 4 | 0 |
| dark | bbs | 0 | — | 无 | 3 | 3.46 | add-section-btn/+ | 4 | 1 |
| dark | attendance | 1 | 3.45 | mi 搜索框 | 18 | 3.45 | cfg-mini/详情 | 4 | 0 |
| dark | file | 1 | 4.51 | fi 搜索框 | 14 | 3.34 | modern-btn ghost | 4 | 0 |

- report.json.summary（programmatic）：itemsTotal=216；categories={input:8, button:152, cardDecor:48, na:8}；funcTotal=160；funcFail=0；overallMin=3.19；fails=[]。
- 逐组 groupMin（light→dark）：login 3.23/3.46、dashboard 4.94/9.40、calendar 3.23/3.19、form 3.22/3.34、bbs 3.25/3.46、attendance 3.19/3.45、file 3.22/3.34；逐组 funcFail 全 0。
- 输入框对比度分布：3.19 / 3.23 / 3.45 / 3.46 / 4.34 / 4.51 / 5.08 / 10.65（8 项全 ≥3:1）。
- NA 8 项构成：日历 today-btn「回到今天」、cll-tab on「我的日历（10）」、cal-create-entry「+ 新建日历」各 light/dark 共 6 个；bbs search-input light/dark 共 2 个，均完全透明无边界、靠文字或占位传达，非边界组件。

## 6. cal-chip 低对比疑点的 CDP 坐实（队长指定，决定性）

- t365 报告与队长复核提及 dark calendar 四个 cal-chip ratio=1.12、bgFinal=rgb(107,107,107) 灰——此为 **v1 判据（边框色 vs 自身灰填充）假象**，非真实边界对比不足。
- t367 CDP getComputedStyleForNode + getMatchedStylesForNode 坐实（证据 calendar-chip-cdp.json，0 error）：四个 chip（data-r487-chip=0..3，对应日历顶部 R311 复查日历等标签）computed 一致 border-top-color=rgb(20,98,190) 蓝、border-style=outset、border-width=2px、background-color=rgb(107,107,107) 灰；命中的 matched CSS 规则仅 UA 默认 `input,textarea,select,button` 与 `input[type=button i],... button`，无自定义低对比规则。
- 按 v3 判据（边界色 vs 外部背景，外部 bg=rgb(10,16,29)）：四 chip borderRatio=3.19、fillRatio=3.57，均 ≥3:1；light 对照外部 bg=rgb(251,252,253)、borderRatio=5.81。故判**正常规范预期**，非缺陷；1.12 仅出现在把蓝边框与自身灰填充相除的废弃算法里。

## 7. 技术观察（非缺陷，据实记录）

- 卡片玻璃装饰描边：CARD_DECOR 48 项中 42 项有对比度值（ok 8 / 低对比 34，低对比分布 1.18–1.28:1）、6 项纯透明 ratio=null；低对比项属 WCAG 1.4.11 明确豁免的装饰性边界（不承担组件识别，内部文字与实心填充另按 1.4.3 达标），不判 P 级。
- 8 项 NA（6 透明文字按钮 + 2 bbs 搜索框）非边界组件，不计入功能性失败。

## 8. 三分判定

- 可达真缺陷：0。功能性边界（输入框 8/8、按钮 152/152，合计 funcTotal=160、funcFail=0）合成后全部 ≥3:1（overallMin=3.19）；cal-chip 疑点经 CDP 坐实为 v1 假象；transition 中间态假象已排除。
- 不可达 L1：0。
- 正常规范预期：全部功能性边界达标；装饰描边豁免；NA 组件靠文字传达；另 2 项技术观察见第 7 节。

## 9. 截图与证据 sha256（命令行原始值，与 report.json 逐一一致）

证据目录 oa4rust/test-results/r487-nontext-contrast/（14 PNG + report.json + cdp-detail.json + calendar-chip-cdp.json，全部 gitignored）；node sha-cross 校验 report 内 g.sha 与磁盘文件 14/14 全吻合。

| 证据 | sha256 |
|---|---|
| pc-login-light-r487.png | 20603e1e530af284e864f28b0599968f124f7e476441a2ec77cb196a95a4562c |
| pc-login-dark-r487.png | 5fe423e730b1e1046222d7ffe91470abae47ac5a5d37fd7cebe217117bf08acd |
| pc-dashboard-light-r487.png | c917d029fc49c00b6aeb3be41b0ab7678cbdec9ad529b7058c6b358c1b5908a8 |
| pc-dashboard-dark-r487.png | 03696ea0493293faa874e3e3ebe7b61c5d9ffe1ea051904e7582a7a6cf31e4f1 |
| pc-calendar-light-r487.png | 8d8694de37850992579bc46f4d3f52827abeb1fb5a96a36f0397f70fbfba5fad |
| pc-calendar-dark-r487.png | 0075d87d57dc86a5bc3963fa095ecbabaa06562542fbaea5922b2234fe9f6c4b |
| pc-form-light-r487.png | 577b73c3de9a05e9ab28d20d1551e3de8af93c0f607d2b23c904997db8c24f80 |
| pc-form-dark-r487.png | 7c675548961afd8d9226d6b166b641d1a21e13dc20b5afc30a2bd9d47275d7d2 |
| pc-bbs-light-r487.png | 888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105 |
| pc-bbs-dark-r487.png | 6971592b8637568d54e01261e7a0b99ac5127ef79091e3d63d55ea5724d8082c |
| pc-attendance-light-r487.png | 678bf7a97db9f909f44902582cb3cc4a9dcaa670b3e68032f41dfc16ac91144b |
| pc-attendance-dark-r487.png | 85e665fc7e5fad10eae3b95d83cf41381eb3dc24858cdf2ee5c4cb972f582df7 |
| pc-file-light-r487.png | 6d2230edbcdb7b4b62d40935be08903f9ae858640ce956c982d366636e21942e |
| pc-file-dark-r487.png | 54bb7cfec34f1e0e3e7eb683babd934fc965b0b6cd486dfb48a6e3fd41c12ff1 |
| report.json | 80139d6e1382e058a01f24128f02d190e3c2909bee6aa368599566a467ed8b8c |
| cdp-detail.json | 5c77bce5f6aaee2342e402561e1fe065a92470b2c9d40468729795e52c61b2d6 |
| calendar-chip-cdp.json | 6d4b0fb1e3b0bcb4251ba38c47d031a57be35c5539ef8fe957503aabd92716ec |

目视 4 图（pc-login-light / pc-dashboard-light / pc-calendar-dark / pc-file-dark，均 1440×900 非物理真机）：登录输入框蓝框、仪表盘卡片边界、深色日历顶部四个 cal-chip 灰底蓝字、深色文件页控件边界均清晰可辨，无可见退化，与实测数据一致。

## 10. 真值纪律

- 全文仅含占位符 <YOUR_TEST_DOMAIN>/<TEST_HOST>，无真实域名/IP/口令/个人路径；扫描 0 命中。
- 截图来源如实标注为 Playwright Chromium headless 非物理真机，未冒充真实设备。
- t367 补正：report.json 与本文档逐项 programmatic 一致（分类计数 input8/button152/cardDecor48/na8、funcTotal160/funcFail0/overallMin3.19、逐组 min、cal-chip CDP、全部证据 sha），消除 t365 的名实不符。