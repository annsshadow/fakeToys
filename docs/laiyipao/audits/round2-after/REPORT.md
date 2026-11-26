# R2 改造后真浏览器复查与动态证据（t15）

> 截图均在桌面 Chrome 无头浏览器中采集，**非物理真机**；H5 采用 390×844 移动视口模拟，admin 采用 1258×588 桌面视口。所有 URL 均为本地回环 QA 环境占位（127.0.0.1:<LOCAL_PORT>），不含真实域名 / IP / 口令。

## 1. 结论速览

| 锚点 | 验收项 | 结论 |
|---|---|---|
| ① | 首屏改造前后并排截图，肉眼可辨差异（硬门禁） | **PASS** — 三页差异显著 |
| ② | 双端可见 AI 元素截图佐证 | **PASS** — H5 三页 + admin 看板/命令面板均可见 |
| ③ | 受限网速抓骨架屏运行时截图 + 入场动画中间帧（F-10-5 闭环） | **PASS** — 两项动态证据均抓到并视觉确认 |
| ④ | 工程达标项不回退（0 pageError / 0 consoleError / 0 API 失败） | **PASS** — 三页与 admin 探针全 0 |

本轮未发现新真实缺陷。t18 已完成，看板与 BattlesView 的 t18 后真浏览器复查见 §8，两页结构证据非 0、截图为真实页面、探针全 0。四锚点全部闭环。

## 2. 环境与方法

- 工作目录：本 worktree；后端 127.0.0.1:8080（healthz/readyz 200，SETTLE_RECOMPUTE_MODE=shadow）、admin 127.0.0.1:5175/laiyipao-admin/（base 已配）、H5 127.0.0.1:5174（VITE_API_BASE 指向本地 API）。PG 容器 lyp-qa-pg 回环映射。
- 浏览器：agent-browser 0.41.1 + Chrome；会话 H5=lyp-qa-r2、admin=lyp-admin-r2。全局 flag 置于命令前；--init-script 注入探针；set viewport 390 844 设 H5 移动视口。
- 探针 probe-init.js：捕获 pageErrors / unhandledrejection / console.error / fetch≥400 / XHR≥400，计数口径 {pe,ce,af,fr}。
- 口令：dev bootstrap 口令从源码 config.go 的 env 默认值动态提取注入 ADMIN_PASS 环境变量，脚本与报告均不落真值（文档记 <ADMIN_PASS>）。

## 3. 锚点① — 首屏改造前后并排截图（硬门禁 PASS）

逐张比对 t13 前后 6 张截图（均 390×844，**非物理真机**）：

| 页面 | 改造前 | 改造后 | 可辨差异 |
|---|---|---|---|
| 首页 | before-01-home.png | after-01-home.png | 顶栏下新增「AI 战术洞察」卡：AI 徽章 + 焰/冰/电/毒/动能五元素圆点 + 动态短板结论「装备契合仅 0%（权重 21%）：优先补它，收益 378」+ 依据 + 去补强 CTA |
| 选关 | before-03-stage.png | after-03-stage.png | 顶部新增「作战简报 · ACTIVE FRONT」hero：大关卡号 1 + 五元素渐变星级 + 继续挑战 CTA |
| 战斗 | before-02-battle.png | after-02-battle.png | 顶部新增五元素连锁 HUD（火/冰/雷/腐/动徽章 +「元素连锁：命中触发反应」），血条/波次胶囊玻璃化 |

三页差异肉眼显著可辨，硬门禁通过。

## 4. 锚点② — 双端 AI 元素可见（PASS）

H5（data/h5-capture.log）：

- 首页 r2-h5-home-ai.png — aiCard=1，文本含「AI 战术洞察 / 规则引擎 / 装备契合仅 0%（权重 21%）：优先补它，收益 378 / 去补强」，探针 {pe:0,ce:0,af:0,fr:0}。
- 选关 r2-h5-stage-hero.png — hero=1、continueCta=true、sigil=true，探针全 0。
- 战斗 r2-h5-battle-sigil.png — sigilRail=1、sigilLabel=五元素连锁、canvas=1（塔防画布渲染），探针全 0。

admin（data/admin-capture.log）：

- 看板 r2-admin-dashboard.png — aiCard=1，文本含「AI 运营洞察 / 规则引擎 · 数据截至 2026-11-21 / 第 1 关流失最大（尝试 12 → 通关 0，净流失 12），建议优先复核关卡难度 / 关卡漏斗柱状图，共 12 关 / 复核关卡难度 →」；令牌 bg=#0b0f14、accent=#ff6b35；探针 {pe:0,ce:0,af:0}。
- 命令面板 r2-admin-command-ai.png — panel=1、aiGroup=1(.cmd-ai-group-label)、aiOptions=3(.cmd-ai-option)、navOptions=10、ariaModal=true、aiLabel=✦ AI 洞察、首项「✦ 查看今日运营洞察」。

六张截图已逐张读图视觉确认 AI 元素清晰可见。看板为 t18 前形态；t18 后形态的看板与战报另见 §8（r2-admin-dashboard-t18.png / r2-admin-battles-t18.png）。

## 5. 锚点③ — 骨架屏运行时 + 入场动画中间帧（F-10-5 闭环 PASS）

### 5.1 骨架屏运行时态（抓到）

- 传输诊断（diag-init.js）：init-script 确实注入（flag=true）；公告页数据走 XHR，URL 命中 …/api/v1/announcements?limit=20（transport=xhr），另有统计 SDK 走 fetch（与本测无关）。
- 决定性实验（diag-delay-init.js）：hook XMLHttpRequest.prototype.open 记录 __u，对 URL 含 /announcements 的 send 延迟 5000ms 后放行。
  - @1s：sendCalled=1, delayed=1, urls=[…/announcements], skeletonRows=3, loading=true, count=null — 骨架屏 3 行 +「加载中…」处于运行态。
  - @7s：count=共 20 条公告 — 数据到达，骨架屏消失。
- 截图 r2-skeleton-running.png（1258×588，**非物理真机**，sha256 b857a425…）已读图确认：公告卡内「加载中…」+ 3 行灰条骨架屏可见。r2-skeleton-loaded.png 为加载完成态。
- **诚实说明**：data/skeleton-anim-capture2.log 记录的自动采集（skeleton-delay-init.js 版本）当时 skeletonRows=0 未抓到，属脚本延迟注入未命中的采集失败；最终 r2-skeleton-running.png 由上述决定性实验的截图**覆盖产出**并视觉确认，非脚本日志那次。上述 @1s/@7s 为决定性实验实测值。

### 5.2 入场动画中间帧（抓到）

- 通过重新导航进入首页捕捉 ai-insight-in-83a5a03c 动画：ANIM_BEFORE 为 finished/currentTime=240；ANIM_MID 为 states=[running], times=[0], opacity=0, transform=matrix(1,0,0,1,0,6) — 动画起点中间帧（透明 + 下移 6px）。
- 截图 r2-ai-animation-midframe.png（1258×588，**非物理真机**，sha256 8b308a8b…）已读图确认 AI 卡处于入场初帧。

## 6. 锚点④ — 工程达标项不回退（PASS）

- H5 三页 + admin 看板/命令面板：pageErrors、consoleErrors（console.error）、失败请求（fetch/XHR≥400）**全部为 0**，无白屏、无空页。
- 所有 /api/v1/admin/* 调用返回 200。
- 命令面板/骨架屏等既有交互未见回归。

## 7. 假阳性排除

| 现象 | 判定 | 依据 |
|---|---|---|
| skeleton-anim-capture2.log 的 skeletonRows=0 | 采集脚本失败，非产品缺陷 | 决定性实验证明骨架屏真实运行（见 §5.1）；截图证据覆盖产出 |
| 统计 SDK 的 tongji-collector fetch | 外部统计上报，非业务 API | 诊断中 transport=fetch 但 URL 为 WebTrack.gif，与游戏无关 |
| 动画 ANIM_BEFORE=finished | 重新导航时机问题，非缺陷 | ANIM_MID 已抓到 running 帧 |

## 8. t18 后 admin 看板与战报复查（PASS）

t18 完成后，在真浏览器重新登录 admin（口令经源码默认值动态注入，不落真值），复查看板与 BattlesView 两页，确认结构重构生效、非采集假阳性。

- **看板** r2-admin-dashboard-t18.png（1258×588，**非物理真机**）：\`sectionKicker=3\`（核心指标 CORE METRICS / 经济与防线 ECONOMY & DEFENSE / 趋势与分布 ANALYTICS 三段分区），\`kpiPrimaryCards=4\`、\`kpiSecondaryCards=4\`（两级 KPI 指标带），\`aiCard=1\`（AI 运营洞察卡保留），\`charts=14\`，探针 \`{pe:0,ce:0,af:0,fr:0}\`。已读图确认 3 段分区 + 4 主 KPI（左侧橙强调条 + 大数值）+ AI 洞察卡肉眼可辨。
- **战报** r2-admin-battles-t18.png（1258×588，**非物理真机**）：URL=/admin/battles，\`本页记录/本页胜率/验真异常/守恒可疑\` 4 项速览指标带全在，\`filterRows=2\`（两层筛选），\`tableRows=14\`（表头与数据行结构），探针 \`{pe:0,ce:0,af:0,fr:0}\`。已读图确认为真实战报页（顶部 4 项速览卡 + 两层筛选 + 表格列）。本轮 QA DB 处于空数据态，战报显示「没有匹配的战报」空列表，不影响结构重构验证。
- **采集假阳性排查**：首次自动脚本 capture-t18-after.sh 因口令注入长度异常（误取 21 位）登录失败，两张截图仅是登录页（DASH_T18 url 停 /admin/login、全 0）——已判定为采集失败、作废重抓。修正口令后逐步登录（确认跳 /admin/dashboard）重抓，上述结构证据与截图均为真实页面。

- t18（admin 标杆重构）已完成并经队长复核，看板/战报 t18 后真浏览器复查见 §8。
- t16（R2 设计复审）依赖 t15 与 t18，两者均完成后可由 design-reviewer 终审。

## 9. 产物清单

- 报告：docs/laiyipao/audits/round2-after/REPORT.md（本文件）
- 脚本：capture-h5.sh、capture-admin.sh、capture-skeleton-anim.sh、probe-init.js、diag-init.js、diag-delay-init.js、skeleton-delay-init.js
- 数据：data/h5-capture.log、data/admin-capture.log、data/skeleton-anim-capture.log、data/skeleton-anim-capture2.log
- 截图（10 张，均标注非物理真机）：r2-h5-home-ai.png、r2-h5-stage-hero.png、r2-h5-battle-sigil.png、r2-admin-dashboard.png、r2-admin-command-ai.png、r2-skeleton-running.png、r2-skeleton-loaded.png、r2-ai-animation-midframe.png、r2-admin-dashboard-t18.png、r2-admin-battles-t18.png
