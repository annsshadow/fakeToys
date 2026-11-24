# R473 h1.ai-gradient-text 渐变遮蔽复核甄别（真浏览器实证）

- 轮次：R473，任务 t337，attempt f8946e9e-5727-4777-a012-f2b7a744c49e
- 日期：2026-11-24
- 性质：只读甄别，不改动任何入库源码/契约（本文件为唯一交付）
- 基线：分支 opt1000/oa4rust-pc-ai-refactor，HEAD 15fbf753fe273561a1d21e09d07f06954d9fecde，AB=0 0

## 1. 概况与方法

R472 修复 NoteApp P2（笔记管理标题渐变被实心色遮蔽）后，队长静态差集锁定强候选 RecycleApp.vue:7 h1 带 ai-gradient-text、:272 渐变 transparent、:278 .view-header h1 实心 color 覆盖，与 NoteApp 修复前同构。本轮对 9 个含 h1.ai-gradient-text 的页面做双主题真浏览器实证，严禁仅凭静态同构预判。

覆盖页：RecycleApp / DeploymentApp / FileManager / MeetingApp / ProcessWork / BBSForum / BamApp / Dashboard / NoteApp，各 light+dark = 18 组。

测试环境：Playwright Chromium headless，非物理真机；PC 1440x900 DPR1，双主题（R473_DONE groups=18 shots=18 errors=0）。

采集项：h1.ai-gradient-text 的 color / backgroundImage / background-clip / -webkit-text-fill-color / 字号 / 主题；CDP CSS.getMatchedStylesForNode 取全部命中规则与属性，核特异性。

证据目录（gitignored，.gitignore:28 拦截）：oa4rust/test-results/r473-h1-gradient/（18 PNG + report.json + r473-e2e.cjs + r473-analyze.cjs + runner）。

## 2. 逐页 h1 computedStyle 实测（18 组）

| 页 | 主题 | 标题 | color | backgroundImage | clip | 渐变可见 |
|---|---|---|---|---|---|---|
| recycle | light | 回收站 | rgb(37,99,235) 实心 | linear-gradient(120deg,...) | text | 否 |
| recycle | dark | 回收站 | rgb(0,212,255) 实心 | linear-gradient(120deg,...) | text | 否 |
| deployment | 双 | 部署管理 | rgba(0,0,0,0) | linear-gradient(120deg,...) | text | 是 |
| file | 双 | 文件管理 | rgba(0,0,0,0) | linear-gradient(120deg,...) | text | 是 |
| meeting | 双 | 会议管理 | rgba(0,0,0,0) | linear-gradient(120deg,...) | text | 是 |
| process | 双 | 工作流待办 | rgba(0,0,0,0) | linear-gradient(120deg,...) | text | 是 |
| bbs | 双 | 团队论坛 | rgba(0,0,0,0) | linear-gradient(120deg,...) | text | 是 |
| bam | 双 | 业务活动监控 | rgba(0,0,0,0) | linear-gradient(120deg,...) | text | 是 |
| dashboard | 双 | 早安，admin | rgba(0,0,0,0) | linear-gradient(120deg,...) | text | 是 |
| note | 双 | 笔记管理 | rgba(0,0,0,0) | linear-gradient(120deg,...) | text | 是（R472 修复后） |

统计：渐变可见 16/18；失败 2/18 全部为 RecycleApp（light 与 dark 双主题一致复现）。-webkit-text-fill-color 在失败组与 color 同为实心色，进一步坐实渐变被遮蔽。

## 3. CDP 根因与特异性分析

### 3.1 RecycleApp（缺陷，P2）

CDP CSS.getMatchedStylesForNode（h1.ai-gradient-text）命中三条规则：
1. 全局 .ai-gradient-text：background:var(--ai-gradient-text)；color:#0000；clip:text。
2. scoped .ai-gradient-text[data-v-99d053bf]：同样 color:#0000 + clip:text。
3. ★后置 .view-header h1[data-v-99d053bf]：color:var(--color-primary)（出现两次）。

源码 oa4rust/frontend/apps/desktop/src/views/RecycleApp.vue:7 模板 h1.ai-gradient-text「回收站」；:272 scoped .ai-gradient-text{...color:transparent}；:278 .view-header h1{font-family:Orbitron;font-size:20px;color:var(--color-primary);...}。

特异性对比：:272 的 .ai-gradient-text 为 (0,1,0)，:278 的 .view-header h1 为 (0,1,1)——后者标签数多 1，后置实心 color 胜出；background-clip:text 仍在，渐变被实心遮蔽，标题显示纯色（light 蓝 rgb(37,99,235)、dark 青 rgb(0,212,255)）。与 NoteApp 修复前（:182 单类 vs :186 同类同构）完全同构。

### 3.2 正常页的救回机制（据实区分）

- BamApp:7 h1 在 div.hero-copy 内，:660 同样有 .view-header h1{color:var(--color-primary)}；但 :655 用双类 .ai-gradient-text.ai-gradient-text 特异性 (0,2,0) 高于 :660 的 (0,1,1)（class 数 2>1），渐变 transparent 胜出，故「业务活动监控」渐变正常。目视 pc-bam-light.png 证实。
- DeploymentApp:182 显式 .view-header h1.ai-gradient-text{...color:transparent} 特异性 (0,2,1)，渐变正常。
- MeetingApp:1126 .ai-page .meeting-hero .admin-title{color:transparent}、ProcessWork:3251 .ai-page .work-hero .admin-title{color:transparent} 为高特异性救回规则，渐变正常。
- NoteApp:186 已由 R472 修复为 color:transparent，本轮实测 color=rgba(0,0,0,0)、grad=true，修复生效确认（GREEN）。

## 4. 目视证据（名实相符）

- pc-recycle-light.png（sha dbb484b1…）：「回收站」标题实心蓝，渐变缺失；页面其余布局完整。
- pc-recycle-dark.png（sha eb53de15…）：同页深色，标题实心青，双主题一致复现。
- pc-bam-light.png（sha db9f6178…）：「业务活动监控」蓝紫渐变正常，作为正常对照。
- pc-note-light.png（sha 3073d4e6…）：「笔记管理」蓝紫渐变，R472 修复后正常。

## 5. 三分判定

- 可达真缺陷：1 项 P2——RecycleApp h1「回收站」渐变遮蔽（RecycleApp.vue:278 .view-header h1 实心 color 覆盖 :272 渐变 transparent；CDP + 源码行号 + 双主题截图三证合一）。P2 依据：仅视觉降级、可读仍成立、有 H5 与其他页替代路径，非阻断。
- 不可达 L1：0。
- 正常-规范预期：16 组（含 NoteApp 修复后、BamApp 双类提权救回、Deployment/Meeting/ProcessWork 高特异性救回、File/BBS/Dashboard 标准类）。

修复建议（供后续 RED→GREEN 轮，本轮不改码）：将 RecycleApp.vue:278 .view-header h1 的 color:var(--color-primary) 改为 color:transparent（与 NoteApp R472 修复同构），或把 :272 提升为 .ai-gradient-text.ai-gradient-text (0,2,0) 以高于 :278；保留 Orbitron 字体/字号/text-shadow。

## 6. 截图 sha256（命令行原始值，18 张）

    066053df18c5f1c676bd606d31b2a046745444692e2e06ce106958f71fe7547f  pc-file-light.png
    13f0f1a4130b893f5424d0cd0899c8f53c33469c172fce53e34719b4417d3af2  pc-deployment-dark.png
    1cdfa352dc73baedcc374b29e31413d3ca4c4c876b068c3af0dd2ebb53762a72  pc-process-light.png
    3073d4e69a41e8a9713c65923d08c8af13fb0a2b0911df0f6abc568be55bd8c0  pc-note-light.png
    45b6da6a7740feb557ba0dca9528c5be909835e68826002c7f8a40ab7b0c1709  pc-dashboard-dark.png
    4e03ecfe70674aa5f7d922002b850bb20ee4fc8875bf509e34c977dfd99ef577  pc-process-dark.png
    4f8279150d32c028e5165c2b0636c59d76dfcda93e561bf3dae0af6a347c6b46  pc-meeting-dark.png
    56e1f230e589cd1d2e1ac772666e0160af2ef8282209205d25c7f88c29eba064  pc-dashboard-light.png
    5b7c9fb6099896699a23f5222eaac51ce596ca7a26638f3de9a2283ee45ebf6b  pc-note-dark.png
    612f84e58105254d62bde5be7e0db5d94ef7dcd4e856fd2239650ce934baba34  pc-deployment-light.png
    6971592b8637568d54e01261e7a0b99ac5127ef79091e3d63d55ea5724d8082c  pc-bbs-dark.png
    803f39ce719360f5e95c925e22c1895bb4fb8669066a947f8885f56c0caed8e4  pc-bam-dark.png
    888c846de2215abce2a1c80e20f9a34065c33da793433710f4f243e29fac2105  pc-bbs-light.png
    94c4840fbd266eeaae6c3287abe09f1d0e3b7e874f6e35ef8a358d73fd891b26  pc-meeting-light.png
    d7ebe750e0bf26efbca0896ab619ae7dae03e2af697ff92616fa43b1c35b9875  pc-file-dark.png
    db9f61789cd8952e790619961dd01111f755f2523f3fcaa3c46cdbd49f3074b7  pc-bam-light.png
    dbb484b159e7e066d1f9169def089b0d89809f693c207230a2d3337b0667ec26  pc-recycle-light.png
    eb53de15f228fc54a284bf0353ae9393918dae218ea953d6a4ac0e3db2b246e7  pc-recycle-dark.png

## 7. 真值纪律

本文所有服务器地址/账号/端口均以占位符表示（<TEST_HOST>/<YOUR_TEST_DOMAIN>），无真域名/IP/口令/个人路径入库。凭据与 OA_BASE_URL 真值仅存在于 gitignored 的 test-results runner 中，运行时从受控凭据源提取后注入进程环境，不回显、不入库。截图来源如实标注为 Playwright Chromium headless 非物理真机。本文不含任何真实环境定位信息。
