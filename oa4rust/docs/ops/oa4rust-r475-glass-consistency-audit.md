# R475 AI glass 双体系毛玻璃一致性与视觉节奏甄别（真浏览器实证）

- 轮次：R475，任务 t341，attempt aa5e318e-0fad-497e-891b-d257e6f18c39
- 日期：2026-11-24
- 性质：只读甄别，不改动任何入库源码/契约（本文件为唯一交付）
- 基线：分支 opt1000/oa4rust-pc-ai-refactor，HEAD 96cd3b5d70e993d62d024880a39f0366107a75d1，AB=0 0，porcelain=0

## 1. 概况与方法

本轮据静态真实信号启动甄别：oa4rust/frontend/packages/ui/src/theme/dark.css:48/:107 原生玻璃体系定义 --glass-blur:blur(16px)，:226-233 .glass-card 消费 var(--glass-blur)；oa4rust/frontend/packages/ui/src/theme/ai-tokens.css:37/:79 定义 --ai-glass-blur:blur(12px)，:100-107 .ai-panel 消费 var(--ai-glass-blur)。两套 blur 半径数值不同（16px vs 12px），且 PC 管理页常在同一元素叠加 glass-card + ai-panel + modern-hero（如 RecycleApp.vue:6 view-header glass-card recycle-header modern-hero ai-panel）。

★队长判据（严格执行）：严禁仅凭两套令牌数值不同就预判缺陷，须有可见退化实证；无真实缺陷则据实判 0，不臆造。

测试环境：Playwright Chromium headless，非物理真机；PC 1440x900 DPR1，light/dark 双主题，8 个真实可达且叠加 glass 类的管理页（recycle/deployment/meeting/process/bbs/bam/dashboard/file）= 16 组（R475_DONE groups=16 shots=16 errors=0）。

采集：①每页逐元素 computedStyle backdrop-filter / background / border / radius / shadow；②CDP CSS.getMatchedStylesForNode 取全部 backdrop 命中规则与来源；③元素层令牌解析探针（强制设令牌观察属性是否联动）；④构建产物 CSS 核实；⑤截图目视；⑥双主题签名比对。

证据目录（gitignored，.gitignore:28 拦截）：oa4rust/test-results/r475-glass/（16 PNG + report.json + cdp-full.json + token-probe.json + r475-analysis.txt + 4 个脚本/runner）。

## 2. 逐页 backdrop-filter 生效值（16 组）

令牌实测（两主题一致）：--glass-blur=blur(16px)、--ai-glass-blur=blur(12px)；light 下 --glass-bg 空 / --ai-glass-bg=#ffffffb8，dark 下 --glass-bg=#0d1525a6 / --ai-glass-bg=#0d15258c。

最终渲染生效值统计：**全 16 组只出现 none 与 blur(12px) 两种，blur(16px) 从未出现在任何元素的最终 computedStyle（0 次）**。

| 页 | 主题 | 可见 glass/ai 元素数 | 生效 backdrop-filter 集合 | 同页一致性 |
|---|---|---|---|---|
| recycle | light/dark | 10 | 全 none | 全页一致（none） |
| deployment | light/dark | 8 | 全 none | 全页一致（none） |
| bam | light/dark | 11 | 全 none | 全页一致（none） |
| dashboard | light/dark | 8 | 全 none | 全页一致（none） |
| file | light/dark | 5 | 全 none | 全页一致（none） |
| meeting | light/dark | 7 | blur(12px) + none | hero/booking/mine/content-panel=blur(12px)，calendar=none |
| process | light/dark | 4 | blur(12px) + none | work-hero=blur(12px)，tabs/根 ai-panel=none |
| bbs | light/dark | 7 | blur(12px) + none | bbs-header=blur(12px)，sidebar/main/根 ai-panel=none |

双主题签名比对（CROSS_THEME）：8/8 页 light 签名 == dark 签名（SAME=true），无双主题失衡。

## 3. CDP 三层根因坐实（非预判）

### 3.1 第一层：matched rules 来源

CDP CSS.getMatchedStylesForNode（light 态）：
- recycle .view-header（glass-card + ai-panel 叠加）命中 .glass-card → -webkit-backdrop-filter:var(--glass-blur) 与 .ai-panel → -webkit-backdrop-filter:var(--ai-glass-blur)，但 computed backdrop=none。
- recycle .content-panel / .stat-card 命中 .glass-card → -webkit-backdrop-filter:var(--glass-blur)，computed=none。
- meeting .meeting-hero 命中 .ai-panel → -webkit-backdrop-filter:var(--ai-glass-blur) **加上** scoped .ai-page .meeting-hero[data-v-ef93c602] → backdrop-filter:var(--ai-glass-blur)（标准属性），computed=blur(12px)。
- meeting .content-panel 命中 scoped .ai-page .booking/.mine/.content-panel[data-v-ef93c602] → backdrop-filter:var(--ai-glass-blur)，computed=blur(12px)；而 .calendar 命中 .glass-card → 仅前缀属性，computed=none。
- bbs .bbs-header 命中 scoped .ai-page .bbs-header[data-v-eea3df64] → backdrop-filter:var(--ai-glass-blur)，computed=blur(12px)；.bbs-sidebar/.bbs-main 命中 .glass-card → 仅前缀属性，computed=none。

### 3.2 第二层：元素层令牌解析探针

对 .view-header / .content-panel / .meeting-hero / .calendar / .bbs-header / .bbs-sidebar 逐一读取：元素层与根层令牌均正常解析为 blur(16px)/blur(12px)（elTokens 与 rootTokens 一致）；强制在元素层重设 --glass-blur:blur(16px) 与 --ai-glass-blur:blur(12px) 后，after.backdrop 完全不变（none 仍 none、blur(12px) 仍 blur(12px)）。

结论：**不是令牌缺失或解析失败**，而是消费令牌的标准 backdrop-filter 声明本身在对应元素上不存在。getComputedStyle().webkitBackdropFilter 在本 Chromium 版本为 undefined（前缀别名不映射到标准 computed 属性）。

### 3.3 第三层：构建产物 CSS 核实

ssh 读取测试服构建产物 index-*.css，精确规则：
- .glass-card{background:var(--bg-glass);-webkit-backdrop-filter:var(--glass-blur);border:...} —— **只有 -webkit- 前缀，无标准 backdrop-filter**。
- .ai-panel{background:var(--ai-glass-bg);-webkit-backdrop-filter:var(--ai-glass-blur);border:...} —— 同样只有前缀。

决定性归因：源码 dark.css:228-229 与 ai-tokens.css:102-103 原本同时写了 backdrop-filter 与 -webkit-backdrop-filter 两条声明，但构建/后处理产物中标准 backdrop-filter 被剥离，只保留 -webkit- 前缀；因此仅依赖 .glass-card/.ai-panel 公共类的元素最终 computed backdrop-filter=none，只有页面自身 scoped 规则里显式写标准 backdrop-filter:var(--ai-glass-blur) 的 hero 类元素才得到 blur(12px)。这解释了为何 16px 体系从未生效、12px 只在局部 hero 出现。

## 4. 同页一致性与视觉节奏

- 同页 blur/none 混合（meeting/process/bbs）是**不同元素的既定设计分工**：主视觉 hero 头图带模糊，侧栏/表格/纯色半透明区不带模糊；并非"同类卡片强度不一"。
- 全页 none 组（recycle/deployment/bam/dashboard/file）内所有卡片一致，无内部强度差异。
- 视觉节奏：元素 radius 统一 16px（大卡）/10px（modern-card 内卡），layout 完整、间距均匀；本批次页未见 --ai-grid-*/--ai-shadow-* 相关的可见断裂信号，据实不记录为缺陷。

## 5. 目视证据（4 图，名实相符）

- pc-recycle-light（全 none）：回收站头图/统计卡/内容卡呈现一致的浅色半透明块，边界清晰、无强度突兀、无可见退化。
- pc-meeting-light（blur(12px)+none）：会议管理头图带轻微模糊，预订/我的会议卡协调，日历区无模糊但背景色相近，无可见割裂。
- pc-bbs-light（blur(12px)+none）：论坛头图带模糊、侧栏/主区纯色，观感自然。
- pc-dashboard-dark（全 none）：深色下 4 张统计卡渐变背景 + 半透明块层次分明，观感协调，无模糊缺失导致的可见问题。

## 6. 三分判定

- **可达真缺陷：0（据实）**。判据要求"同页毛玻璃显著不一致/双主题失衡**且有可见退化**"：①双主题 8/8 完全一致，无失衡；②同页 blur/none 混合属 hero 与侧栏的设计分工，非同类卡片强度突变；③4 图目视无可见退化——半透明底色本身已提供层次，模糊在纯色/低对比背景上几乎不可见，none 不损害可读性/布局/功能；④两套令牌数值不同（16 vs 12）这个静态信号**未导致任何可见不一致**（16px 路径根本未渲染），按队长判据不得预判为缺陷。
- **不可达 L1：1**。dark.css:48/:107 的 --glass-blur:blur(16px) 令牌在元素层可解析，但消费它的标准 backdrop-filter 声明在构建产物中被剥离（仅余 -webkit- 前缀），16px 模糊规则不可达实际渲染。属"规则声明不生效"，非可见退化，不判 P 级。
- **正常-规范预期：若干**（双主题一致、目视协调、hero/侧栏设计分工、视觉节奏无断裂）。

技术观察（非缺陷，建议后续独立评估）：产物仅保留 -webkit-backdrop-filter 前缀、标准属性缺失，是长期既有实现状态，非本轮引入回归；如后续要恢复真正毛玻璃观感，应统一补齐标准 backdrop-filter 声明并评估是否保持现有协调观感，本轮不改码、不臆断为缺陷。

## 7. 截图 sha256（命令行原始值，16 张）

    046b61d67e3c259c21b6331da23b47dec461f4a6bf2c6f9a64a8695b0f2ee916  pc-dashboard-light.png
    066053df18c5f1c676bd606d31b2a046745444692e2e06ce106958f71fe7547f  pc-file-light.png
    13f0f1a4130b893f5424d0cd0899c8f53c33469c172fce53e34719b4417d3af2  pc-deployment-dark.png
    1cdfa352dc73baedcc374b29e31413d3ca4c4c876b068c3af0dd2ebb53762a72  pc-process-light.png
    4e03ecfe70674aa5f7d922002b850bb20ee4fc8875bf509e34c977dfd99ef577  pc-process-dark.png
    612f84e58105254d62bde5be7e0db5d94ef7dcd4e856fd2239650ce934baba34  pc-deployment-light.png
    6971592b8637568d54e01261e7a0b99ac5127ef79091e3d63d55ea5724d8082c  pc-bbs-dark.png
    803f39ce719360f5e95c925e22c1895bb4fb8669066a947f8885f56c0caed8e4  pc-bam-dark.png
    8456a5267ac759247176081ae925fc0a4248fe6fc32637152a1f43d0e2475472  pc-recycle-light.png
    888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105  pc-bbs-light.png
    94c4840fbd266eeaae6c3287abe09f1d0e3b7e874f6e35ef8a358d73fd891b26  pc-meeting-light.png
    9ff0653f9e93a36a3273b637fd88bbc16839dad876290b1025a8a564849ea774  pc-meeting-dark.png
    b99ce5d9819c702b16c73186fe09005d814353677e4e9fccc23dfbae994999cc  pc-dashboard-dark.png
    d7ebe750e0bf26efbca0896ab619ae7dae03e2af697ff92616fa43b1c35b9875  pc-file-dark.png
    db9f61789cd8952e790619961dd01111f755f2523f3fcaa3c46cdbd49f3074b7  pc-bam-light.png
    f73f7f32bfc4f021e5924b70d28a0d80e19edae1f84e613f2aa888e664d40c87  pc-recycle-dark.png

## 8. 真值纪律

本文所有服务器地址/账号/端口均以占位符表示（<TEST_HOST>/<YOUR_TEST_DOMAIN>），无真域名/IP/口令/个人路径入库。凭据与 OA_BASE_URL 真值仅存在于 gitignored 的 test-results runner 中，运行时从受控凭据源提取后注入进程环境，不回显、不入库。截图来源如实标注为 Playwright Chromium headless 非物理真机。本文不含任何真实环境定位信息。
