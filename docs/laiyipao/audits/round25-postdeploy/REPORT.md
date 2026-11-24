# R25 线上回归报告——单局战报反应伤害主力聚合卡

## 1. 结论

PASS —— R25 已完成四批精确提交、H5 生产包重建、三端集成验证与真浏览器决定性回归；本轮为纯 H5 只读展示派生（battles.vue 新增 .r25-reaction-impact），无后端/admin 业务源码变更，后端与 admin 沿用既有产物。聚合卡三行伤害/占比/权威次数与 SQL 逐字段一致，旧战报诚实空态正确，AI 四判据与诚实红线齐备，探针 pe/ce/af/fr 全 0，真值守卫 exit 0。全程未发现真值入库风险。

## 2. 部署产物（测试环境）

| 端 | 产物 | sha256 | 说明 |
| --- | --- | --- | --- |
| 后端 | laiyipao/deploy/bin/lyp-server-amd64 | 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6 | 沿用 R21，R25 无后端接口改动 |
| H5 | laiyipao/miniapp/dist/build/h5/assets/index-Cze6pKWS.js | 70239f0ccbba76c1668ace34d656f2d34efea44defb0d541d94448e989d9a532 | 本轮 VITE_API_BASE 指向回环 API 重建，index.html 引用一致 |
| H5 业务块 | pages-battles-battles.CqCNeXi-.js | grep 命中 r25-reaction-impact | 确认改动进入生产 bundle |
| admin | laiyipao/admin/dist/assets/index-CGo_zhD3.js | d5e9c483992f2a370b5def55d7418da958b695deda4db9c3446e26bd04d58c70 | 沿用 R20，R25 无 admin 业务改动 |

> 测试环境域名统一用 <YOUR_TEST_DOMAIN> 占位；报告不含真实域名、IP、token、口令或带口令连接串。

## 3. 集成验证

- 后端 /healthz = 200；/readyz = 200。
- H5 根路径（5174）= 200；admin /laiyipao-admin/（5175）= 200（hash 路由，非 hash 直链 404 属预期）。
- QA 库 Docker 容器 lyp-qa-pg Up，端口映射回环 15440→5432；后端、H5 dev、admin dev 三服务启动并复验通过。
- H5 生产构建 exit 0；index.html 引入 index-Cze6pKWS.js，bundle 内 r25-reaction-impact 标记命中。

## 4. R25 功能核心决定性证据

- 聚合卡 .r25-reaction-impact 为纯前端只读派生：消费已过滤的 reaction_timeline.events 按 reaction 累加 damage，count 取 detail.reactions_used（严禁 events 冒充），占比 toFixed(1) 降序，SUM≤0 归零防 NaN；不改 step()/replay_hash/score/loot/结算。
- 战报 378 有数据态 DOM（probe-378.json，role=status，timelineRows=69）三行：蒸汽爆发 5239.0 伤害 39.8% 权威23次 事件23条；超导 4472.0 伤害 34.0% 权威27次 事件27条；过热 3438.0 伤害 26.1% 权威19次 事件19条。SQL 独立核验按 reaction 聚合 steam_burst|5239|23、superconduct|4472|27、overheat|3438|19 与权威 reactions_used={overheat:19,steam_burst:23,superconduct:27} 逐字段一致；手算 5239/13149=39.8%、4472/13149=34.0%、3438/13149=26.1% 吻合。本局 events 条数与权威次数数值恰相等属数据巧合，口径由契约反例（count≠events）钉死。
- 战报 339 旧战报诚实空态（probe-339.json，timelineRows=0）：rows=[]，empty 逐字『本局无逐次反应伤害记录』，conclusion 逐字『本局无逐次反应伤害记录，不编造主力』；该局 reactions_used 有值但未反推主力，回放哈希 44faed4a07a3593b（SQL 一致）。
- AI 四判据两态齐备：aiTitle『AI · 反应伤害主力』、真实数据结论『AI：本局蒸汽爆发造成 39.8% 反应伤害，是伤害主力』、依据行逐字『依据：本局 reaction_timeline.events 的 damage 聚合；触发次数以 reactions_used 为准』、CTA『下一局优先凑齐该元素组合 →』、诚实行逐字『辅助展示非伤害权威,权威以 replay_hash 为准』。
- 有数据态回放哈希 7468252a57f6befe（SQL 一致），派生卡未改权威结算；hasUndefined=false、hasNaN=false；探针 pe/ce/af/fr 全 0（已过滤 Autoplay 噪声）。
- 契约与测试：2 文件 19 例全绿；三端令牌锚点 v1/v3-r3/r6 保留且无 --r25-（依赖 t194-t197 复查/终审证据）。

## 5. 真浏览器验证

- agent-browser 无头 Chromium 390×844（非物理真机），会话 lyp-r25-e2e，深链 <YOUR_TEST_DOMAIN>/#/pages/battles/battles?battle_id=378 与 battle_id=339；document-start 注入占位模板生成的 gitignored auth-init.js（真 token 不入库、不回显），本轮实测登录态有效。
- 决定性截图亲读：r25-02（390×844）显示聚合卡三行真实数据+AI四判据+诚实行+哈希 7468252a57f6befe；r25-04（1258×588）显示 339 逐字空态『本局无逐次反应伤害记录，不编造主力』+哈希 44faed4a07a3593b。均为「我的战报」页非登录页。
- 本轮线上 E2E 重跑 probe-378/probe-339 与入库证据逐字段一致；e2e-supp.sh 在 agent-browser 补充探针步骤挂起，已 job_kill，r25-04 保持入库版本未覆盖，不影响决定性证据。

## 6. 截图 sha256（入库版，两两不同）

| 截图 | sha256 |
| --- | --- |
| r25-01-impact-378.png | fa24f998555ddad04850b0367777942597d9e8fde85f6c8289fcad79de0e0c31 |
| r25-02-impact-detail.png | ed07f5749f3ee9ec57feb01ecd8bd0b77ff96441313cc959f9b55b622367bd08 |
| r25-03-old-empty.png | 6fc54cc3b1af2b56c88c91ef73280e6729483c52b3b6dfbb4cd65bc717fa38b5 |
| r25-04-old-empty-scroll.png | 375274166220ffa64ea4eaecffda8b224256c3156b3caf7351824ea8aa5a3033 |

## 7. 真值与收尾自检

- 四批精确 git add 显式路径（禁 -A/.）：批1 feat 1 文件 84 行 LEAK=0；批2 docs 3 文件 131 行 LEAK=0；批3 test 12 文件 498 行 LEAK=0 DATA=0 AUTHJS=0；批4 本报告单文件 LEAK=0。
- 严格正则覆盖：JWT 形态、guest_token 字面值、Windows 用户目录形态、root 加 IP 登录形态、私钥头形态、带口令连接串、引导口令赋值形态、真实域名/IP。
- 含真 token 的 auth-init.js 与 data/ 快照/样本经 .gitignore（round25 data/ 与 auth-init.js）排除且从未入库；仅 __LYP_TOKEN__ 占位模板入库。go test 真值守卫（TestNoRealSecretsInRepo 等三项）exit 0。报告本身零真值。
- 前三批本地 commit 88383d8f8419b3534ac9fa77b8e5f9eac9ad8b81 / f7ce912fffb0b423fcc702d0c15343cf102a92fc / a982354c95c8d2398f07059964f0c78dc58fd2c4，工作树 tracked 干净；push 后 AB=0/0。
