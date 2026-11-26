# R8 改造后真浏览器独立复查（t84）

- 日期：2026-11-22
- 任务：Round8（t79 地形去重+威胁标签 / t81 规范 / t82 钳分HUD / t83 星级口径+机制拐点）改造后独立真浏览器复查
- 环境：本地回环 QA —— H5 http://127.0.0.1:5174/ 、admin http://127.0.0.1:5175/laiyipao-admin/ 、API http://127.0.0.1:8080 （PG 容器 lyp-qa-pg 回环 15440）
- **声明：所有截图来自真浏览器自动化的 H5/admin 页面，均非物理真机**；H5 关卡页/首页 390×844 移动视口，战斗页/admin 1258×588 桌面视口。
- 写域：仅 docs/laiyipao/audits/round8-after/**；未改任何业务源码、未执行 git、未部署。
- 真值纪律：报告与截图二进制均无真实域名/IP/口令/token/私钥（见 §6）。

## 1. 结论速览

| 契约项 | 结论 | 关键量化证据 |
| --- | --- | --- |
| ① H5 关卡页星级口径+机制拐点+地形威胁 | PASS | L98 星级卡满分 33646 / 1★20187（约60%）来自配置；17 个机制拐点；图例◆▲■；L100 22 拐点含[首领]威胁标签 |
| ② 战斗页 t82 钳分倒计时 HUD | PASS | HUD「钳分中·距满分 131s→122s」随战斗时钟递减，与星级差额胶囊并排不遮挡，官方 errors=[] |
| ③ R1-R7 不回退 | PASS | 新手引导/诚实文案/AoE25%/成长页两步确认结构/admin 顶栏脱敏 5 项全在 |
| ④ 探针 pe/ce/af/fr | PASS | 关卡页/首页/战斗页 consoleError=0/API失败=0/请求失败=0；官方 errors=[] |
| ⑤ 真值零泄露 | PASS | 文本扫描 0 命中；6 张截图二进制 strings 扫描 0 命中 |
| ⑥ 证据落盘+非物理真机标注 | PASS | 6 张截图 + 采集脚本 + 探针 + 本报告落 round8-after/ |

本轮**未发现新真实业务缺陷**。三处观测噪声均判定为假阳性（见 §7），不阻塞下游。

## 2. ① 关卡页复查（t83 星级口径 + 机制拐点、t79 地形/威胁不回退）

方法：全新会话 + document-start 注入探针硬加载 `?level=<N>`，读取 `.star-rule-card` / `.diff-milestone` / `.diff-legend-item` / `.threat-tag` DOM（stage-probe.js）。

| 关卡 | 本关满分(/1★门槛) | 难度/波次/元素上限 | 机制拐点数 | 首条拐点 | 威胁标签 | 探针 |
| --- | --- | --- | --- | --- | --- | --- |
| L98 | 33646 / 20187（约60%） | 4135‰ / 11波 / 8 | 17 | ◆ 第96关·首领关 | [远程][飞行]等 | pe0 ce0 af0 fr0 |
| L1（硬加载） | 19902 / 11941（约60%） | 1000‰ / 5波 / 2 | 9 | ▲ 第1关·本关敌种 游荡者 | 无（首关简单） | pe0 ce0 af0 fr0 |
| L100（硬加载） | 52311 / 31386（约60%） | 4200‰ / 12波 / 8 | 22 | ◆ 第96关·首领关 | [远程][飞行][首领] | pe0 ce0 af0 fr0 |

- **星级口径来自配置、非硬编码**：三关满分/门槛/难度/波次各不相同且与服务端口径一致；文案写明「按本关满分百分比给星；关卡难度仍逐级上升，星级门槛按本关满分缩放，因此不同关卡的绝对分数线会升降，不代表高关更容易」——诚实解释非单调，无写死具体关卡数字。
- **机制拐点三类标注齐全**：图例 `["◆ 首领","▲ 本关敌种","■ 地形"]`，形状去重后 `["◆","▲","■"]`。L1 首条是「▲ 第1关·本关敌种 游荡者」（t83 修正后只标当前关真实敌种，未再伪造「整章首现」）；L100 首条是「◆ 第96关·首领关 首领血量与技能机制改变打法」。
- **t79 不回退**：敌人组成威胁标签实测渲染 `[远程]`/`[飞行]`/`[首领]`，地形去重后按唯一关卡计。
- boss 关 56/60/91/95/96/100 在同会话 URL 切换下因 uni-app 框架 keep-alive 行为 DOM 未即时重渲染（见 §7 假阳性①）；以全新会话硬加载 L1/L100 为准，boss 关 96 拐点标注确实存在。

## 3. ② 战斗页 t82 钳分倒计时 HUD

方法：从已验证的 L1 关卡页原生 `click '.start-btn'` 真实进入第 1 关战斗，两个时点读 `.time-budget` / `.star-gap-text` / canvas / `.hud-pill`。

| 时点 | 钳分 HUD 文本 | class | 星级差额 | canvas | HUD胶囊数 | 探针 |
| --- | --- | --- | --- | --- | --- | --- |
| T0（进入即刻） | 钳分中 · 距满分 131s | hud-pill time-budget | 距 1 星 11938 | 1 | 4 | ce0 af0 fr0 |
| T8（8秒后） | 钳分中 · 距满分 122s | hud-pill time-budget | 距 1 星 10929 | 1 | 4 | ce0 af0 fr0 |

- **数值来自配置/战斗时钟**：倒计时随 elapsedMs 从 131s 递减到 122s（非写死）；锚点前诚实显示「钳分中·距满分 Xs」（t82 真实发现：服务端 scoreCapFor 是负向闸，旧「保分窗口」文案会误导，已改正）。
- **不遮挡/不冲突**：截图 r8-battle-clamp-hud.png 肉眼确认钳分胶囊与星级差额胶囊、波次/击杀胶囊并排于顶部 HUD，战场 Canvas 完整可见；`.time-budget` 与 `.star-gap-text` 各自独立（battle.vue:29/35）。
- 官方 `errors --json` → `errors=[]`。探针 pageErrors=7 全部是 `The play() request was interrupted by a call to pause()`（已知 F-2/t6 修复的观测口径噪声，见 §7 假阳性②）。

## 4. ③ R1-R7 不回退专项

| 项目 | 验证方式 | 结果 |
| --- | --- | --- |
| 首页新手引导（t62） | `.first-run-guide` DOM | 「新玩家先打第 1 关 / 守住防线，清完全部波次 / 去第 1 关 →」存在 |
| index 攻击力诚实文案（t31/R5） | 首页 body 文本 | hasFalseClaim=false，hasHonest=true：「元素侧是主力，面板攻击力也有贡献但占比受限（权重上限 30%）」 |
| AoE 25% 范围伤害（t41/t42） | 战斗页原生点击战场图例 | pct25=2 / radius120=2 / hasSteam=true / hasOverheat=true，图例文案与 25% 修复一致 |
| 成长页推荐两步确认结构（t66/t74/t77） | skills 页 DOM | `.r7-impact`=1，标题「真实收益预估 · 当前 vs 推荐」，honestEmpty=true（游客态无战报时诚实空态）。弹层真写入闭环已由 t78 专项覆盖 |
| admin 顶栏脱敏（t33） | admin 登录后 `.api-tag` | 「API 127.0.0.1:8080」（本地回环可见分支，非回环域名才脱敏为纯 API）；domainInBody=false，无真实域名 |
| 战斗结算归因（t64/R7） | battle.vue:279 源码 + 结构 | `<text v-if="settleAttribution && !settleFailed" class="settle-attribution">` 存在 |

## 5. ④ 探针与错误汇总

- 关卡页（L98/L1/L100）、首页、战斗页、admin 看板：consoleError=0 / apiFailures=0 / failedRequests=0 / 官方 errors=[]。
- 战斗页 pageErrors=7：`play() request was interrupted by a call to pause()` —— sfx.ts 的 installMediaRejectionFilter 在 initSfx 时注册，探针在 document-start 更早注册故先捕获；官方 errors=[] 证明过滤器已 preventDefault。与 t9/t32/t56 判定一致，非业务缺陷。
- 游客态 skills 页 af=2：`/me/recommendations?context=skills` 与 `/me/battles?limit=1` 返回 401 —— 未登录访问需鉴权接口的预期行为，非缺陷（t57/t61 同判定）。登录后会话因测试脚本 token 注入失败（TOKLEN=0）未复验，但 DOM 结构与诚实空态已确认不回退。

## 6. ⑤ 真值纪律

- 报告文本 grep 模式（JWT `eyJ...` / 私钥 PEM / 真实域名 / 带口令 DSN）：0 命中。
- 6 张 PNG 截图二进制 `strings | grep` 真域名/token 模式：0 命中。
- 仅在本地回环 QA 环境操作；口令从源码公开默认值动态读取（config.go:599 `env("BOOTSTRAP_ADMIN_PASS","admin12345")` 非生产真值），报告以占位口径描述，未回显任何真值。

## 7. ⑦ 假阳性排除（3 项，均非产品缺陷）

1. **同会话 hash 切换 DOM 未更新**：`capture-stage.sh` 在单一会话内连续 open `?level=1/56/60/...` 时，URL 已变但 DOM 仍显示 L98 数据（33646），导致 r8-L1-stage.png 与 r8-L98 sha256 相同（`f4989511...`）。根因是 uni-app H5 hash 路由在 keep-alive 页面不因 query 变化重挂载；真实用户点关卡格走响应式 `selectedId.value`（stage.vue:753）会正确更新。决定性实验：全新会话硬加载 L1→19902、L100→52311，数据正确。已用 fresh 截图替换。**非缺陷**。
2. **战斗 pageErrors=7**：全部 `play() request was interrupted by a call to pause()`，官方 errors=[]，sfx 过滤器生效，探针时序差异。t6 已修复 F-2。**非缺陷**。
3. **游客态 skills af=2（401）**：未登录访问 `/me/*` 的预期鉴权响应。**非缺陷**。

## 8. ⑧ 产物清单（round8-after/）

- 报告：REPORT.md（本文件）
- 截图（6 张，非物理真机）：r8-L98-star-rule-milestones.png / r8-L1-stage-fresh.png / r8-L100-boss-fresh.png / r8-battle-clamp-hud.png（正式证据）；r8-L1-stage.png / r8-L100-boss.png（首轮假阳性，sha 见 §7，保留供对照）
- 探针：probe-init.js（pe/ce/af/fr 捕获）、stage-probe.js、battle-probe.js、home-probe.js、honest-probe.js、growth-probe.js、admin-probe.js、aoe-legend.js、af-dump.js、pe-dump.js、btn-probe.js、stage-btn-probe.js
- 采集脚本：capture-stage.sh / capture-battle.sh / recapture.sh / growth-recheck2.sh / admin-login.sh / click-text.tpl.js

## 9. 截图 sha256

| 文件 | 尺寸 | sha256 |
| --- | --- | --- |
| r8-L98-star-rule-milestones.png | 390×844 | f49895117564bccd087bd55beb3e4ab91584893cdb3fdf616019de6fb8bd6871 |
| r8-L1-stage-fresh.png | 390×844 | d5cd93e676425bb5de9c747762120c5a6bd4205e1f499d2861dbc1467113de18 |
| r8-L100-boss-fresh.png | 390×844 | 228566a3086b51cded4589e5cbc3026eacae26f76217d6642f150fdf7a84d8cf |
| r8-battle-clamp-hud.png | 1258×588 | c255daba5d43f624583ae41b7ff59cdf4a097741e4aaeaae696fea5625a5f995 |

结论：R8 改造四项在真浏览器全部生效，R1-R7 不回退，可提交 design-reviewer 终审。