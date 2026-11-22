# t67 R7 真浏览器复查与 Top3 闭环验证报告

> 截图均由桌面 Chrome 真浏览器访问本地回环开发服务截取，**非物理真机**；无公网域名、账号口令或私钥真值（真值文件 laiyipao/deploy/.local.keys 未读取、未入库）。审计只读，未改业务源码、未执行任何 git 操作。

## 1. 结论速览

- R7 Top3 缺口中，**首屏引导卡（t62）、战报归因串联/超时长钳分/星级门槛/漏怪（t64）、战斗 HUD 实时星级差额（t64）、成长页 AI 因果解释（t66）均真浏览器验证生效**。
- **发现 1 个真实跨层缺陷 R7-BUG-1（高）**：服务端 `/me/recommendations` 的所有 insight CTA params 只下发 `{context}`/`{slot}`/`{reaction}` 等，**不含** impact 接口必需的 `id`（候选 id）与 `replace`（当前已装备 id）；而前端 skills/equipment 页 `loadR6Recommendation()` 恰恰依赖 `rec.cta.params?.id` 与 `?.replace` 才调用 `/me/recommendations/impact`。结果：impact 接口本身能返回真实数据（手动核验 insights=1/hasImpact=true），但真实用户流程中前端永远不调用它，R7「当前 vs 推荐 · 确认替换」两步确认弹层无法打开。
- 已在本账号用**真实浏览器玩完 3 场第 1 关战斗**产生真实 battle_records（0→3），排除「账号无历史」这一疑似根因，坐实根因为 CTA 契约失配。
- **证据覆盖缺口**：t72 门禁④要求的「有数据态对比弹层截图（含确认按钮与写配置前后差异）」未能产出——因为产品链路本身走不通（R7-BUG-1），不是脚本卡选卡态或数据不足。按队长收敛指令如实记录，不以 eval 伪造弹层。

## 2. 环境与口径

- 本地回环 QA：后端 :8080（SETTLE_RECOMPUTE_MODE=shadow）、admin :5175（base /laiyipao-admin/）、H5 :5174；PG 容器回环 15440→5432。三服务 healthz/admin/h5 全 200。
- 浏览器 agent-browser 0.41.1，会话 lyp-r7-h5（H5，viewport 390x844）/ lyp-r7-admin（admin，viewport 1258x588）；探针 probe-init.js 注入捕获 pageErrors/consoleErrors/apiFailures/failedRequests/recResponses。
- H5 玩家登录用原生 click .btn-primary（eval click 是假登录）；admin 登录用 localStorage.clear + fill 两输入框 + eval 精确点登录，跳 /admin/dashboard。
- 口令为源码公开默认 bootstrap 值，报告中一律记 `<ADMIN_PASS>`。

## 3. 逐项契约验证

| 契约 | 结果 | 证据 |
|---|---|---|
| ① Top3 闭环 | 部分（含缺陷） | 首屏引导卡 firstRunGuide=true、文案「新玩家先打第1关…去第1关」；结算归因「服务已修正本次结算：得分时长裁剪」+真实得分 9582+门槛 11941+漏怪 0；HUD 实时「距1星 10424」；0 星动态差额「还差2359分就能拿1星（门槛11941分）」；成长页 AI 因果链真实数据。**当前vs推荐预览弹层 FAIL → R7-BUG-1** |
| ② impact 接口真实调用 | PASS（接口侧） | 手动 GET `/me/recommendations/impact?kind=skill&id=2&replace=1` 返回 status=200、sv=rec-v1、insights=1、hasImpact=true、len=2339；合法空态诚实显示「收益数据不可用…不展示虚构数值」，无本地兜底冒充。但前端真实流程未触发（见 R7-BUG-1） |
| ③ R1-R6 不回退 | PASS | 顶栏回环 apiTag 显示本地地址=t33 设计分支、非回环纯 API；AoE 25% 文案与 t41 修复一致；0 星差额、index「元素侧主力/面板攻击力占比受限·权重上限30%」诚实文案、推荐空态、命令面板 AI 分区全在 |
| ④ 探针/三重确认 | PASS | H5 三页与 admin 三视图探针 pe/ce/af/fr 全 0（af=2 为登录前 401 噪声）；URL/title/结构三重确认均非登录页；命令面板 panel=1/aiGroup=1/aiOpts=3/ariaModal=true/focus=cmd-input |
| ⑤ 真值零泄露 | PASS | 报告/sh/js 文本扫描无公网域名/IP/口令/私钥；截图二进制 strings 扫描无真域名 |
| ⑥ 证据落目录 | PASS | 归档 docs/laiyipao/audits/round7-after/，全部标注非物理真机 |
| ⑦ 只读不改源码 | PASS | 未改业务源码、未 git、未部署 |
| 队长补充：有数据态弹层截图 | FAIL（证据覆盖缺口） | 根因 R7-BUG-1，详见 §5 |

## 4. 真实战斗数据产生过程（用于排除数据不足假设）

- 用真浏览器登录→选关→开战→8× 倍速→每波真鼠标点「全部不要了，直接下一波」推进→结算，循环 3 场。
- 三场均通关：score 9582 / 13268 / 11056（分别 0/1/0 星）；battle_records count 0→3；金币 2000→5840，体力按规则扣补。结算截图 r7-battle-settle-{1,2,3}.png。
- 刷新 skills 页后推荐卡显示真实结论「近 7 场场均击杀 39.0，平均推进到第 5.0 波」+ AI 因果链「本局电 58 次、焰 46 次…拿到 11056 分，0 星」（r7-real-rec-data-skills.png）。手动 fetch impact 亦返回真实 impact。
- **结论：账号有真实历史、impact 接口有真实数据、推荐接口有真实 insight——唯独前端拿不到触发 impact 所需参数。**

## 5. 真实缺陷 R7-BUG-1（跨层契约失配）

- 现象：战斗产生真实历史后，skills/equipment 页「真实收益预估 · 当前 vs 推荐」卡仍永远显示「当前 — → 推荐 — / 收益数据不可用：接口未提供真实预估」，无「应用推荐 · 先看收益对比 →」CTA 按钮，两步确认弹层无法打开。
- 根因链：
  1) 服务端 laiyipao/server/internal/service/recommendations.go:283 起，各 context 的 `recInsight(...)` 传入 params 仅为 `{context:"skills"}` / `{slot:0}` / `{reaction:...}` / `{max_stage:...}` / `{element:...}` 等，**无任何 insight 携带 `id` 与 `replace`**。
  2) 前端 laiyipao/miniapp/src/pages/skills/skills.vue:447-450 与 equipment.vue:447-450：`const candidateId = Number(rec.cta.params?.id ?? 0); const replacedId = Number(rec.cta.params?.replace ?? 0); if (candidateId && replacedId) { await fetchRecommendationImpact(...) }`——两者为 0 时整段 impact 调用被跳过。
  3) impact 端点（routes.go:126 / handlers_recommendation_impact.go）要求 `kind`/`id`/`replace` 缺一不可，参数本身正确；t65/t66 单测与手动 fetch 均 GREEN。断裂点在 recommendations → 前端的 CTA 参数契约。
- 影响：R7「当前 vs 推荐两步确认、确认后才写入配置」这一核心玩法闭环在真实用户流程中完全不可达；t66 的 13 项单测与 impact 接口测试无法覆盖这段跨层串联。
- 修复方向（供队长派单，本 QA 不改源码）：让 recommendations 的 skill/equipment insight CTA params 下发候选 `id` 与当前已装备 `replace`（与 impact 接口契约对齐），或前端从构筑快照自行推导这两个 id 并调用 impact；并补一条端到端测试覆盖「推荐 insight → impact → 对比弹层」全链。
- 附：equipment 页 CTA params 为 `{slot:0}`（实测 EQREC），同样缺 `id`/`replace`；gems 页未接 impact（不属本缺陷）。

## 6. 假阳性排除

- admin 首轮 capture-admin.sh 截图落在 login?redirect（session 未登录），已用 capture-admin2.sh 重登，admin2.log 确认 URL/title/结构三重生效，截图有效。
- H5 探针 af=2 为登录前未鉴权 401 噪声；登录后差分 pe/ce/af/fr 全 0。
- 顶栏 apiTag 显示 127.0.0.1:8080 属回环开发服务设计行为（t33 仅对非回环脱敏为纯 API），非泄露。
- strings 命中 `Sg.CN` 为 PNG 二进制压缩字节偶然串，非真实域名。

## 7. 产物清单

- 报告 REPORT.md；探针 probe-init.js；采集 capture-h5.sh / capture-admin.sh / capture-admin2.sh；
- 真实战斗闭环 battle-loop.sh + battle-{start,speed,poll,retry,wallet,records}.js + data/battle3.log（三场结算与 impact 结果）；
- 接口探针 impact.js / rec-probe.js / eqprobe.js / timing-probe.js / final-state.js / r7-cta-probe.js / cta-geometry.js；
- 截图 14 张：r7-after-{index-guide,skills,equipment,admin-dashboard,admin-levels,admin-users,admin-palette}.png、r7-battle-settle-{1,2,3}.png、r7-impact-blocked-{skills,equipment}.png、r7-real-rec-data-skills.png。

## 8. 上报

R7-BUG-1 已同步队长与 design-reviewer；证据覆盖缺口（有数据态弹层）根因为该真实缺陷，非数据不足或脚本问题，不以伪造截图补齐。
