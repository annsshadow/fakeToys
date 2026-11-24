# R471 AI 风格重构主线视觉/交互打磨甄别（真浏览器实证）

- 轮次：R471，任务 t333，attempt c2411d99-d34b-4e79-a31e-79a12ea5228e
- 日期：2026-11-24
- 性质：只读甄别，不改动任何入库源码/契约（本文件为唯一交付）
- 基线：分支 opt1000/oa4rust-pc-ai-refactor，HEAD d8f4431bdc0e9028819c6dbc7f919e3da42bb60c

## 1. 概况与方法

队长据静态侦察将本轮维度收敛为「AI 组件交互反馈/动效节奏 + 半成品标记页视觉一致性」，从候选页中确认择 3 页：Personal.vue / NoteApp.vue / ProcessTaskCenterApp.vue，并以 Dashboard.vue / FileManager.vue 作同体系对照页。硬排除 6 个受 R306C 精确 class 契约保护的页面（PortalScriptDesignerApp / PortalWidgetDesignerApp / ServiceInvokeDesignerApp / FindDesignerApp / PdfViewerApp / CommonApp）。

测试环境：Playwright Chromium headless，非物理真机；PC 1440x900 DPR1 双主题（light/dark），3 候选 + 2 对照 = 5 页 × 2 主题 = 10 组（R471_DONE groups=10 shots=10 errors=0）。

采集项：根容器 class 与 computedStyle；header computedStyle；首个 h1/h2 的 color/bgImage/background-clip/fontSize；玻璃卡数量；按钮 hover/focus 前后 computedStyle（交互反馈探针）；CDP CSS.getMatchedStylesForNode 坐实根因。

证据目录（gitignored，.gitignore:28 拦截）：oa4rust/test-results/r471-ai-visual-refine/（10 PNG + report.json + analysis.txt + r471-e2e.cjs + r471-analyze.cjs + r471-cdp-extract.cjs + runner）。

## 2. 择页依据（静态代码信号）

- Personal.vue:5 根 div class 为「personal-view modern-page」，缺 ai-page/ai-page-b5/ai-panel；:6 header「profile-header modern-hero ai-panel」；:8 h1「个人中心」无 ai-gradient-text；main.ts:69-73 路由 personal 可达。
- NoteApp.vue:5 根 div class 为「crud-view modern-page ai-panel」，缺 ai-page/ai-page-b5；:6 header「view-header glass-card modern-hero ai-panel」；:8 h1「笔记管理」带 ai-gradient-text；main.ts:114 路由 note 可达。
- ProcessTaskCenterApp.vue:5 根 div class 为「mod-view modern-page ai-panel」，缺 ai-page/ai-page-b5；:11 header 内 h2「流程任务中心」带 ai-gradient-text；main.ts:307-312 路由 process-task-center 可达。
- 对照 Dashboard.vue:5 根「dashboard ai-page ai-page-dashboard」，:8 h1「早安，admin」带 ai-gradient-text（welcome-heading）；FileManager.vue:5 根「file-view modern-page ai-page ai-panel」，:8 h1「文件管理」带 ai-gradient-text。

## 3. 逐页实测数据

### 3.1 根容器标记与 header

| 页/主题 | 根 class | 根 display/bgColor | header bgImage |
|---|---|---|---|
| personal-light | personal-view modern-page | flex / transparent | linear-gradient(160deg, rgba(79,70,229,.06), rgba(8,145,178,.02)) |
| note-light | crud-view modern-page ai-panel | flex / rgba(255,255,255,.72) | none（view-header 用 --ai-gradient-surface 但被 scoped 覆盖为纯色） |
| ptc-light | mod-view modern-page ai-panel | flex / rgba(255,255,255,.72) | linear-gradient(160deg, rgba(79,70,229,.06), rgba(8,145,178,.02)) |
| dashboard-light（对照） | dashboard ai-page ai-page-dashboard | flex / transparent | linear-gradient(160deg, rgba(79,70,229,.06), rgba(8,145,178,.02)) |
| file-light（对照） | file-view modern-page ai-page ai-panel | flex / rgba(255,255,255,.72) | none |

三候选页根缺 ai-page/ai-page-b5 标记，但根 display:flex、header 渐变/玻璃卡 computedStyle 与对照页一致，无真实视觉退化；判规范预期（详见第 5 节）。

### 3.2 标题 h1 computedStyle（决定性）

| 页/主题 | h1 文本 | class | color | bgImage | background-clip | 判定 |
|---|---|---|---|---|---|---|
| personal-light | 个人中心 | （空） | rgb(15,23,42) | none | border-box | 纯色，profile 头像式 header 内协调可读 |
| personal-dark | 个人中心 | （空） | rgb(240,246,252) | none | border-box | 同上，深色主题清晰 |
| note-light | 笔记管理 | ai-gradient-text | rgb(37,99,235) 实心 | linear-gradient(120deg, rgb(67,56,202), rgb(14,116,144)) | text | 渐变被实心色覆盖，失效 |
| note-dark | 笔记管理 | ai-gradient-text | rgb(0,212,255) 实心 | linear-gradient(120deg, rgb(165,180,252), rgb(103,232,249)) | text | 渐变被实心色覆盖，失效 |
| ptc-light | 流程任务中心 | admin-title ai-gradient-text | rgba(0,0,0,0) | linear-gradient(120deg, rgb(67,56,202), rgb(14,116,144)) | text | 渐变正常 |
| ptc-dark | 流程任务中心 | admin-title ai-gradient-text | rgba(0,0,0,0) | linear-gradient(120deg, rgb(165,180,252), rgb(103,232,249)) | text | 渐变正常 |
| dashboard-light（对照） | 早安，admin | welcome-heading ai-gradient-text | rgba(0,0,0,0) | linear-gradient(120deg, rgb(67,56,202), rgb(14,116,144)) | text | 渐变正常 |
| file-light（对照） | 文件管理 | ai-gradient-text | rgba(0,0,0,0) | linear-gradient(120deg, rgb(67,56,202), rgb(14,116,144)) | text | 渐变正常 |

## 4. Note 标题渐变遮蔽根因（CDP + 源码双重坐实）

CDP CSS.getMatchedStylesForNode（note h1）命中规则顺序：
1. 全局类 .ai-gradient-text：background=var(--ai-gradient-text)，color=#0000（即 transparent），-webkit-background-clip=text，background-clip=text。
2. 提权类 .ai-gradient-text[data-v-c1f3425b]：同样 color=#0000 + clip:text。
3. ★后置 scoped 规则 .view-header h1[data-v-c1f3425b]：color=var(--color-primary)（出现两次）。

源码 oa4rust/frontend/apps/desktop/src/views/NoteApp.vue:182 定义 scoped .ai-gradient-text{background:var(--ai-gradient-text);-webkit-background-clip:text;background-clip:text;color:transparent}；但 :186 又定义 .view-header h1{font-family:Orbitron;font-size:20px;color:var(--color-primary);margin:0 0 4px;text-shadow:0 0 15px var(--color-primary-glow)}。

根因：.view-header h1[data-v] 特异性 (0,2,1) 高于 .ai-gradient-text[data-v] (0,2,0)，后置 color 声明胜出，把 transparent 覆盖为实心 --color-primary；由于 background-clip:text 仍在，渐变背景被实心 color 遮蔽，标题显示为纯色（light 蓝 rgb(37,99,235)、dark 青 rgb(0,212,255)），与同体系 Dashboard/FileManager/PTC 的蓝紫渐变标题不一致。

目视证实：pc-note-light.png 与 pc-note-dark.png 标题「笔记管理」为实心色；对照 pc-file-light.png「文件管理」为蓝紫渐变。双主题一致复现。

修复建议（供后续 RED→GREEN 轮，本轮不改码）：在 NoteApp.vue scoped style 中将 :186 .view-header h1 的 color:var(--color-primary) 移除或改为 color:transparent（保留 text-shadow 需重估），使 :182 .ai-gradient-text 的 transparent + clip:text 生效；或对需要渐变的标题加更高特异性规则 .view-header h1.ai-gradient-text{color:transparent}。注意 :186 的 Orbitron 字体与字号应保留。

## 5. 交互反馈与动效节奏

10/10 组首个可见按钮 hover 均产生 computedStyle 变化：color 由 rgb(71,85,105)（dark rgb(139,157,195)）变为 rgb(8,145,178)（dark rgb(34,211,238)），border 由半透明变为 rgb(79,70,229)（dark rgb(99,102,241)）；transition 为 0.14s cubic-bezier(0.16,1,0.3,1)。focus 态 outline/shadow 完整。个人中心头像卡 .avatar-large:hover{transform:scale(1.04)}、.profile-header:hover{box-shadow:var(--ai-shadow-2)}（Personal.vue:762-764）均生效。判定为正常-规范预期，交互反馈一致。

## 6. 三分判定汇总

| 项 | 判定 | 依据 |
|---|---|---|
| Note 标题渐变遮蔽 | 可达真缺陷 P2 | NoteApp.vue:186 scoped .view-header h1 color 覆盖 :182 渐变 transparent；CDP 特异性坐实 + 双主题截图证实；同体系对照页正常 |
| Personal h1 纯色 | 规范预期/正常 | profile 头像式 header 视觉定位不同，纯色 rgb(15,23,42)/rgb(240,246,252) 清晰协调，非遗漏降级 |
| 三候选页根缺 ai-page/b5 标记 | 规范预期/正常 | computedStyle（display/header 渐变/玻璃卡）与对照页一致，无真实视觉退化，仅标记不齐 |
| PTC 标题渐变 | 正常 | admin-title ai-gradient-text color=transparent + clip:text 渐变生效 |
| 交互反馈/过渡 | 正常 | 10/10 hover 反馈生效，transition 0.14s 一致 |

总计：可达真缺陷 1 项（P2，Note 标题渐变遮蔽）；不可达 L1 0；正常-规范预期若干。不臆造、不凑轮次。

## 7. 截图 sha256（命令行原始值，10 张）

    066053df18c5f1c676bd606d31b2a046745444692e2e06ce106958f71fe7547f  pc-file-light.png
    09f0480cc26900baf75002de0638309808b752074f29bdf63815a3dccd9837c5  pc-ptc-dark.png
    1459c1f14638d3f96959341cd21901859dfe643b1816ef0677649a541f4f7cc8  pc-personal-light.png
    16c9d0cd76fd57b64e0b13ccda72610ef94508a49b79c439e55d4a0801bca46e  pc-ptc-light.png
    20c6937e9d3b50e2e9b73be2a5563b191263acf18c8275b71cbb29bf56ad02bd  pc-dashboard-dark.png
    4f0af707dfb7558b4eb8f771d8787c098c617ab1f9c72c50cc0f75eb9d65e41c  pc-note-light.png
    50a90e3f6296a47c3ade4a6d8069bc926ba9e3cc3c6208b4b2a506751d5efde9  pc-note-dark.png
    708599ce35bd89d0d7bf5cce3d1fb83e88a18771503b4c81cadf495f0e71f711  pc-dashboard-light.png
    ab251a567519c4ec0fce414eafb392b8b986ba926ffec46fb05865f0de5d24dc  pc-personal-dark.png
    d7ebe750e0bf26efbca0896ab619ae7dae03e2af697ff92616fa43b1c35b9875  pc-file-dark.png

## 8. 真值纪律

本文所有服务器地址/账号/端口均以占位符表示（<TEST_HOST>/<YOUR_TEST_DOMAIN>），无真域名/IP/口令/个人路径入库。凭据与 OA_BASE_URL 真值仅存在于 gitignored 的 test-results runner 中，运行时从受控凭据源提取后注入进程环境，不回显、不入库。截图来源如实标注为 Playwright Chromium headless 非物理真机。本文不含任何真实环境定位信息。
