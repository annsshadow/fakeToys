# laiyipao 改造后 Round1 真浏览器全页复查与缺陷修复验证报告

> 执行者：qa-e2e ｜ 日期：2026-11-21 ｜ 任务：t9（attempt 23c1d794）
> 性质：**非物理真机**——桌面 Chrome 157（agent-browser 0.41.1）+ H5 移动端视口，非物理手机。
> 保密：真域名/IP/口令不入库；本文仅含回环地址 127.0.0.1 与本地端口，仅用于本地测试证据。

## 0. 结论速览

| 验收项 | 结果 | 依据 |
|---|---|---|
| ① H5 全 19 页逐页复查 | **PASS** | 19/19 页 0 pageError / 0 consoleError / 0 API 失败 / 0 空页；截图 h5-after-01..19 |
| ① admin 全 11 视图（12 路由含登录）复查 | **PASS** | 11/11 视图 0 错误 / 0 API 失败 / 0 空页；截图 admin-after-01..11 + 登录页 |
| ② F-1 零战报看板红横幅 | **GREEN** | hasUnreachable=false，3 张图表正常渲染为空态数据表，无「后端不可达」 |
| ② F-2 战斗 12s play() interrupted | **GREEN** | 真实 CDP 点击进入战斗 12s，5 条 interrupted rejection 全部 defaultPrevented=true，浏览器 errors=[] |
| ③ 视觉符合度初评 | **符合 V1 规范** | 深色科幻底 + 五元素色 + 卡片表面 + 玻璃层 + 网格底纹，详见 §4 |
| ④ 命令面板 Ctrl/Cmd+K | **PASS** | 打开/过滤/ESC/路由自动关闭/焦点陷阱/触发按钮全通过 |
| ④ 骨架屏 | **PASS（静态核验）** | shimmer CSS 规则存在；数据态下骨架节点为 0（正常，加载已完成） |

**本轮未发现新真实缺陷。** 1 项探针观测口径差异（F-2 探针仍记录 interrupted）已用 defaultPrevented 决定性实验排除为假阳性，非业务缺陷。

## 1. 测试环境与可复现命令

三服务本地回环启动（纯临时 QA 环境，restart 幂等）：
- 后端：DATABASE_URL=<LOCAL_DSN> ADDR=:8080 SETTLE_RECOMPUTE_MODE=shadow go run ./cmd/api，healthz/readyz=200
- admin：pnpm dev（:5175，base /laiyipao-admin/，proxy /api→:8080）
- H5：VITE_API_BASE=<LOCAL_API> pnpm dev:h5 --port 5174

采集脚本（只读，位于本目录）：
- bash collect-h5-after.sh → data/h5-after-collection.log（会话 lyp-qa-after）
- bash collect-admin-after.sh → data/admin-after-collection.log（会话 lyp-admin-after，dev 登录：账号 admin，口令由环境变量 BOOTSTRAP_ADMIN_PASS 注入（占位 <ADMIN_PASS>）；仅本地 QA 环境）
- 探针 probe-init.js：监听 error/unhandledrejection/console.error/fetch/XHR 失败。

## 2. 逐页复查数据

### 2.1 H5 全 19 页（截图 h5-after-01..19-pages_*.png）

| 页 | DOM 元素 | 文本长度 | 页根 | 卡片 | 网格底纹 | 错误 |
|---|---|---|---|---|---|---|
| index | 466 | 629 | 1 | 5 | ✓ | 0 |
| announcements | 75 | 55 | 1 | 1 | ✓ | 0 |
| stage | 450 | 746 | 1 | 5 | ✓ | 0 |
| battle | 110 | 62 | 1 | 0（Canvas×1） | ✓ | 0 |
| bag | 1570 | 3315 | 1 | 8 | ✓ | 0 |
| mastery | 348 | 610 | 1 | 4 | ✓ | 0 |
| reactions | 593 | 1119 | 1 | 5 | ✓ | 0 |
| enemies | 882 | 2609 | 1 | 1 | ✓ | 0 |
| skills | 971 | 2331 | 1 | 3 | ✓ | 0 |
| equipment | 587 | 1529 | 1 | 1 | ✓ | 0 |
| gems | 200 | 292 | 1 | 1 | ✓ | 0 |
| skins | 308 | 632 | 1 | 1 | ✓ | 0 |
| battles | 96 | 99 | 1 | 1 | ✓ | 0 |
| shop | 483 | 947 | 1 | 8 | ✓ | 0 |
| signin | 216 | 353 | 1 | 4 | ✓ | 0 |
| rank | 166 | 139 | 1 | 4 | ✓ | 0 |
| verify | 115 | 202 | 1 | 2 | ✓ | 0 |
| me | 206 | 610 | 1 | 7 | ✓ | 0 |
| login | 115 | 203 | 1 | 1 | ✓ | 0 |

汇总：19/19 非空，pageErrors=0, consoleErrors=0, failedRequests=0, apiFailures=0。令牌实测 bg=#070a10、fire=#ff6b35 生效。

### 2.2 admin 11 视图（截图 admin-after-01..11 + admin-00-login）

| 视图 | DOM | 表格 | 行 | 图表 | 命令触发 | 错误 |
|---|---|---|---|---|---|---|
| dashboard | 309 | 3 | 24 | 3 | 1 | 0 |
| levels | 4106 | 2 | 101 | 0 | 1 | 0 |
| skills | 1775 | 2 | 43 | 0 | 1 | 0 |
| equipment | 875 | 6 | 35 | 0 | 1 | 0 |
| users | 1119 | 6 | 32 | 0 | 1 | 0 |
| battles | 1359 | 8 | 43 | 0 | 1 | 0 |
| recomputes | 1374 | 8 | 43 | 0 | 1 | 0 |
| defenses | 1351 | 8 | 43 | 0 | 1 | 0 |
| economy | 1712 | 10 | 47 | 0 | 1 | 0 |
| content | 1751 | 12 | 48 | 0 | 1 | 0 |
| wiki | 3544 | 20 | 136 | 0 | 0（wiki 独立布局） | 0 |

汇总：11/11 非空，pageErrors=0, consoleErrors=0, failedRequests=0, apiFailures=0。所有 /api/v1/admin/* 200。令牌 lyp-bg=#0b0f14、lyp-accent=#ff6b35 生效。

## 3. 两缺陷修复专项复查（测试锚点②）

### 3.1 F-1 零战报看板误报「后端不可达」 —— GREEN

数据集成本地 DB 处于零战报态（战斗总数 0、验真一致率 —）。真浏览器打开 dashboard：
- hasUnreachable=false、hasBanner=false（正文无「后端不可达」「加载失败」）
- 3 张 ECharts 图表正常渲染：近14日活跃/元素反应使用分布(I-1健康度)/关卡漏斗(前30关) 均显示全 0 空态数据表（表格化渲染，不再因 stage_funnel=null 触发 .slice() 崩溃）
- 证据：special-f1-dashboard-zero-battles.png
- 修复链核验：后端归一空 stage_funnel 为 []、前端 renderCharts 双重问号兜底，两者在真浏览器生效。

### 3.2 F-2 战斗 12s play() interrupted unhandledrejection —— GREEN

用新会话 lyp-qa-f2 + 干净探针，真实 CDP 点击链路：首页「开始」→ stage「开始战斗（消耗6体力）」→ battle（canvas=1），非直接 URL 跳转、非 eval 触发。

12s 观测：
- agent-browser errors --json → errors:[]（浏览器层面无未处理错误）
- 内置探针记录 5 条 unhandledrejection: The play() request was interrupted by a call to pause() —— 初看疑似未修复

决定性实验（排除假阳性）：在 sfx 过滤器注册后注入诊断监听器，读取每条 rejection 的 defaultPrevented：
- 结果：total=5, interrupted=5, preventedTrue=5, preventedFalse=0
- 结论：5/5 均 defaultPrevented=true。F-2 修复（initSfx 内 installMediaRejectionFilter 对 message 含 play() request was interrupted 的 rejection 调 preventDefault）确实生效，浏览器默认错误行为被摘除。探针因在 init-script 最早期注册、先于 sfx 过滤器捕获事件，故仍记录到事件本身——这是探针观测口径差异，不是业务缺陷，真实用户与监控不受污染。
- 证据：special-f2-battle-12s.png
- 附注：early 用 eval click 进战斗时出现的是另一类 play() failed because the user didn't interact with the document first（浏览器自动播放策略拦截 untrusted 事件），属工具假阳性；真实 CDP 点击链路不复现。

## 4. 视觉符合度初评（供 t10 design-reviewer 终审）

对照 docs/laiyipao/design/AI_VISUAL_SYSTEM.md §1-§3 五支柱，真浏览器 DOM + 截图逐页初评：

| 维度 | 规范 | H5 实测 | admin 实测 | 初评 |
|---|---|---|---|---|
| P1 深色科幻底/网格底纹 | 近黑蓝底 + 极弱径向光 | body backgroundImage ✓；bg=#070a10 | lyp-bg=#0b0f14；侧栏分层明度 ✓ | 符合 |
| P2 五元素主题色 | 火冰雷腐蚀动能五色 | fire=#ff6b35，battle 三技能键火/冰/雷三色 | lyp-accent=#ff6b35，品牌橙一致 | 符合 |
| P3 克制玻璃拟态 | 浮层/面板半透明模糊 | .card 表面分层 | 命令面板 glass-surface、role=dialog 浮层 ✓ | 符合 |
| P4 微交互 | 按压/聚焦/反馈 | 按钮态可见（battle 速度档位态） | 焦点环 + 面板高亮 active 项 ✓ | 符合（静态；动效需 t10 动态核） |
| 卡片表面与对比度 | 三级明度递增 | panel 卡面可辨，文字对比清晰 | 表格行/卡面层次可辨 | 符合 |
| 响应式 | PC 宽屏多列 / H5 限宽单列 | H5 单列居中（非物理真机桌面视口） | admin 216px 侧栏 + 多列卡片 | 符合 |

代表性截图：h5-after-01（构筑评分雷达 + 元素进度条）、h5-after-04（battle Canvas 塔防 + 三元素技能键）、h5-after-14（商城卡片群）、admin-after-01（看板多列卡片）、special-command-palette（玻璃浮层命令面板）。

## 5. 命令面板 / 骨架屏交互（测试锚点④）

命令面板（AdminLayout.vue，来源 onGlobalKeydown/onPanelKeydown）：
- Ctrl+K 打开：overlay=true, dialog=true, aria-label=命令面板, aria-modal=true，焦点自动落在搜索框 ✓
- 过滤「用户」：命中 3 项（清空/ESC/用户管理）✓
- ESC 关闭：overlay=false，焦点归还 ✓
- 路由变化自动关闭：打开后 open #/admin/levels → overlayAfterRoute=false ✓
- 触发按钮 .command-trigger 点击可打开 ✓
- 焦点陷阱（精确 .cmd-panel 选择器）：打开时 focusInPanel=true；Tab×5 / Shift+Tab×2 后焦点始终在面板内（cmd-input ↔ cmd-esc 循环），不逃逸到背景 ✓
- 证据：special-command-palette.png

骨架屏：
- shimmer CSS 规则存在（keyframes/skeleton 类，静态核验通过）
- 数据加载完成后 DOM 中 [class*=skeleton] 节点为 0（正常，骨架已让位给真实数据）；未在受限网速下做动态抓拍——非缺陷，记为覆盖边界。

## 6. 假阳性与排除记录

1. resize 命令 unknown —— agent-browser 无此命令，非应用缺陷（基线已知）。
2. 早期 eval click 进战斗触发 play() failed because the user didn't interact —— untrusted 事件被自动播放策略拦截，工具假阳性；真实 CDP 点击不复现。
3. F-2 探针 5 条 interrupted 但 defaultPrevented=true —— 探针先于过滤器注册的观测口径差异，非缺陷（§3.2 决定性实验）。
4. 命令面板 focusInDialog=false —— 用 role=dialog 选择器误选，精确 .cmd-panel 实测焦点陷阱正常（§5）。

## 7. 产物清单

- 报告：本文件 REPORT.md
- 采集脚本：collect-h5-after.sh / collect-admin-after.sh / probe-init.js
- 日志：data/h5-after-collection.log（19 页）/ data/admin-after-collection.log（11 视图）
- 截图：screenshots/h5-after-01..19（19）+ admin-after-01..11（11）+ special-f1/f2/command-palette（3）
- 说明：全部截图为非物理真机（桌面 Chrome + H5 视口），非手机实拍。