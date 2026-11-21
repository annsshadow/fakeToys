# R6 真浏览器复查与前后端接口联通验证报告（round6-after）

> 性质：qa-e2e 只读审计，未改业务源码、未 git、未部署。截图均为**桌面 Chrome 非物理真机**（H5 为 390 逻辑宽度在 1258×588 窗口中居中，两侧留黑边）。本报告不含真实域名 / IP / 口令 / 连接串，统一用 `<YOUR_TEST_DOMAIN>` / `<ADMIN_PASS>` 占位。

## 0. 结论速览

- 契约 7 项：6 项 PASS，第②项「AI 卡不再标注本地规则引擎兜底」**部分未达成 → 记录 1 个真实缺陷 R6-BUG-1**（admin 推荐适配层空态未消费后端 empty_reason）。
- H5 三页（equipment/reactions/skills）接口联通正确：200 + rec-v1 + NO_HISTORY 诚实空态，无兜底。
- admin 五视图接口均 200 + rec-v1，但 content/equipment/wiki 空态回退本地兜底卡（标注「本地规则引擎」），与 H5 不一致。
- 全页探针 pe/ce/af/fr 全 0；R1-R5 不回退项（顶栏脱敏 / 25% AoE / 0 星差额 / index 诚实文案）均保持。

## 1. 环境与口径

- 本地回环 QA：后端 :8080（SETTLE_RECOMPUTE_MODE=shadow）、admin :5175（base /laiyipao-admin/）、H5 :5174（VITE_API_BASE 指向本地 API），PG 容器回环 15440→5432。
- 浏览器 agent-browser 0.41.1，会话 H5=lyp-r6-h5 / admin=lyp-r6-admin；探针 probe-init.js 注入页面最早期，捕获 pageErrors / consoleErrors / apiFailures / failedRequests / recResponses（推荐接口响应体）。
- admin 口令取源码公开默认 bootstrap 值并动态注入，报告脱敏为 `<ADMIN_PASS>`；登录成功跳 /admin/dashboard，token 有效。
- 快照 DB 无玩家战斗历史，后端推荐接口对空历史返回 `empty_reason.code=NO_HISTORY` + `insights=[]`——这是合法空态，H5 已正确处理。

## 2. 逐页实测数据（URL/title/结构三重确认非登录页）

| 页面 | URL/title | 接口 | AI 卡 | 探针 pe/ce/af/fr |
|---|---|---|---|---|
| H5 equipment | #/pages/equipment/equipment / 装备图鉴 | /me/recommendations?context=equipment 200 rec-v1 NO_HISTORY | 数据不足·NO_HISTORY 诚实空态 + 去打一场战斗 CTA | 0/0/0/0 |
| H5 reactions | #/pages/reactions/reactions | context=reactions 200 rec-v1 NO_HISTORY | 同上诚实空态 | 0/0/0/0 |
| H5 skills | #/pages/skills/skills | context=skills 200 rec-v1 NO_HISTORY | 同上诚实空态 | 0/0/0/0 |
| admin content | #/admin/content / 公告与兑换码 | /admin/recommendations/content 200 rec-v1 NO_HISTORY | **本地规则引擎兜底**（见 R6-BUG-1） | 0/0/0/0 |
| admin defenses | #/admin/defenses | .../defenses 200 rec-v1 NO_HISTORY | 数据不足空态（本地 rows 也空，未暴露兜底） | 0/0/0/0 |
| admin equipment | #/admin/equipment | .../equipment 200 rec-v1 NO_HISTORY | **本地规则引擎兜底** | 0/0/0/0 |
| admin wiki | #/wiki / 玩家玩法文档站 | .../wiki 200 rec-v1 NO_HISTORY | **本地规则引擎兜底** | 0/0/0/0 |
| H5 index（回归） | #/pages/index/index | — | AI 战术洞察卡 + 构筑评分雷达，诚实文案 | 0/0/0/0 |

admin login 视图：已登录态访问 /admin/login 被路由守卫重定向 dashboard，属预期行为（非缺陷、非本项验收对象）。

## 3. 前后端接口联通验证（契约②核心）

- 抓包证据（探针 recResponses）：H5 三页各 1 条 `/me/recommendations?context=<page>`、admin 五视图各 1 条 `/admin/recommendations/<scope>`，全部 HTTP 200、`schema_version=rec-v1`、`empty_reason.code=NO_HISTORY`、`insights=[]`。证明前端确已发起并接收真实推荐接口响应，非 404/兜底绕过。
- **H5 侧 PASS**：`normalizeRecommendation`（laiyipao/miniapp/src/pages/_shared/r6Recommendation.ts）消费 empty_reason，AI 卡渲染「AI 暂时没有足够数据给出建议 / 数据不足·NO_HISTORY」，无「本地规则引擎」字样。
- **admin 侧缺陷**：见第 4 节。

## 4. 真实缺陷 R6-BUG-1（中）admin 推荐适配层空态未消费 empty_reason，回退本地兜底卡

- 现象：admin content / equipment / wiki 三视图，接口 200 + rec-v1 + NO_HISTORY + insights=[]（合法空态），AI 卡却显示本地推导结论并标注「本地规则引擎」，例如 content「已发布 0 条公告、3 张兑换码… / 本地规则引擎 · 公告 0 条 / 兑换码 3 张 / 审计 0 条」。契约②要求「AI 卡不再标注本地规则引擎兜底」未达成。
- 复现：登录 admin → 访问 #/admin/content（或 #/admin/equipment、#/wiki）→ 读 #ai-insight-card 文本，含「本地规则引擎」。
- 根因（非契约举例的枚举不一致，scope 合法、接口 200）：`insightFromRecommendation`（laiyipao/admin/src/utils/recommendation.ts）以 `if (!resp || !Array.isArray(resp.insights) || resp.insights.length === 0) return fallback` 单一条件判定，未检查 `resp.empty_reason`。接口合法空态被误判为「接口无数据」，回退 `buildLocalRecommendation` 本地规则引擎卡。H5 的同名适配层有 empty_reason 分支，两端不一致。
- 影响：空态（新服 / 新管理员）下 admin AI 卡显示「本地规则引擎」推导卡而非后端诚实空态，误导运营以为是数据驱动的结论；不影响核心玩法与结算。
- 建议修复（qa 不改源码，交队长安排）：在 `insightFromRecommendation` 顶部先判断 `resp?.empty_reason?.code`，命中时返回 `emptyInsight(resp.empty_reason.message, resp.generated_at?.slice(0,10))`，与 H5 对齐；补单测：接口空态时 AI 卡 evidence 不得含「本地规则引擎」。
- 证据截图：screenshots/r6-admin-content.png（sha256 7594893f…）、r6-admin-wiki.png（6d6995e9…）。

## 5. R1-R5 不回退核对

- 顶栏脱敏：content 截图顶栏 `API 127.0.0.1:8080` 为本地回环，符合 apiLabel.ts 的回环豁免分支（非回环才显示纯「API」），非真值泄露。
- 25% AoE：wiki 截图表格可见「蒸汽爆发…真正的敌人受到 25% 范围伤害」「半径 120 像素」，t41 修复不回退。
- 0 星差额：battle.vue:407 仍为 `还差 N 分就能拿 X 星（门槛…）` 动态文案（源码核对；本轮未进战斗页实测，引用 t49 真浏览器结论）。
- index 攻击力诚实文案：H5 index honest=true、falseClaim=false，构筑面板展示权重上限等诚实表述，R5 文案不回退。

## 6. 假阳性排除

1. admin /admin/login 被守卫重定向 dashboard：已登录态预期行为，非缺陷。
2. wiki 首轮 #/admin/wiki 重定向 dashboard：采集脚本 URL 错误，改 #/wiki 后 title/结构正确，非产品缺陷。
3. timeout 包装器 RC=126：本环境不支持该命令包装，改用直接后台 job，非产品缺陷。
4. defenses 页 AI 卡显示「数据不足」：因本地 rows 也为空，emptyInsight 恰好诚实，非兜底缺陷；仅 content/equipment/wiki 暴露 R6-BUG-1。

## 7. 产物清单

- 报告：REPORT.md（本文）；探针：probe-init.js。
- 脚本：capture-h5.sh / login-admin.sh / capture-admin.sh / card-detail.sh / verify-regression.sh（均 bash -n 通过）。
- 日志：data/{h5.log,admin.log,h5-open.log,admin-open.log,nav-equip.log}。
- 截图 9 张：screenshots/r6-h5-{equipment,reactions,skills,index-honest}.png、r6-admin-{content,defenses,equipment,wiki,login}.png（均 1258×588，非物理真机）。

## 8. 真值纪律

- 本地回环 QA，无真实域名 / IP / 口令 / 连接串入库；报告与脚本用 `<YOUR_TEST_DOMAIN>` / `<ADMIN_PASS>` 占位；未执行任何 git 操作。