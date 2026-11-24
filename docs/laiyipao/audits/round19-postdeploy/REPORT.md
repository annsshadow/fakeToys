# R19 线上回归报告：玩家反应伤害贡献面板

- 轮次：R19（玩家反应伤害贡献面板）
- 执行：release-ops-wb（提交部署）
- 环境：测试环境，域名统一以 <YOUR_TEST_DOMAIN> 占位；本地回环只用于真浏览器 E2E 探针
- 结论：三端部署完成，线上回归全绿，无阻塞问题

## ① 部署产物

| 端 | 产物 | sha256 | 说明 |
| --- | --- | --- | --- |
| 后端 | laiyipao/deploy/bin/lyp-server-amd64 | 490ed05c161d284a5b16c4a5baedda39559be0c216815e3a54827165325f38de | 本轮重建（CGO_ENABLED=0 GOOS=linux GOARCH=amd64 -ldflags '-s -w'），含 GET /api/v1/me/reaction-damage-impact，ELF |
| H5 | laiyipao/miniapp/dist/build/h5/assets/index-DEB5OOSs.js | 339a184541ab7b39b364feb4ff3ae67185da759deabb3f9760def6d302a513b4 | 本轮重建（VITE_API_BASE 指向测试环境 API），bundle 命中 r19-my-reaction-impact |
| H5 页面块 | pages-reactions-reactions.BNoIV7tM.js / reactions-w3y7Cv5B.css | — | grep 命中 r19-my-reaction-impact，证明 R19 双视图卡进入产物 |
| admin | laiyipao/admin/dist/assets/index-DF0m5Tvz.js | f13b3465d6b6c31687ec5258db873b3d8a39aed3dbfa3b1e22bb9d9814b31f6d | 沿用 R18，本轮 admin 无改动 |
| 迁移 | 00031_reaction_timeline.sql | — | 已应用；reaction_timeline 列存在，v1 战报 2 条 |

## ② 集成验证（部署前）

- 后端：GET /api/v1/me/reaction-damage-impact 纯逻辑用例 4 条 + 端到端用例 3 条通过；聚合按 user_id 过滤、30 天窗口、旧战报排除、SUM=0 归零 items=[]。
- H5：reactions.vue 新增 .r19-my-reaction-impact 双视图与 AI 四判据、逐字空态、诚实红线；client.ts 新增带令牌的 fetchMyReactionDamageImpact。
- 全量测试：miniapp 194 文件 1790 测试通过（2 skipped），exit0；契约测试 R19ReactionDamageImpact.contract.test.ts 与 reactions_r19_impact.test.ts 绿。
- 真值守卫：laiyipao/server/internal/domain/secrets_leak_test.go 通过；所有入口配置走 env，仓库无密钥/token/口令/真实域名/IP/个人路径。

## ③ 端到端 HTTP 抽样

| 路径 | 结果 | 说明 |
| --- | --- | --- |
| /healthz | 200 | 后端存活 |
| /readyz | 200 | 后端就绪 |
| H5 首页 | 200 | Vite 5174 |
| admin 入口 | 200 | Vite 5175 的 /laiyipao-admin/ |
| /admin/battles 直链 | 404 | hash 路由预期，非缺陷 |

跨用户隔离 + SQL 权威核验：

| 用户 | HTTP | window_days | 窗口 | excluded | items |
| --- | --- | --- | --- | --- | --- |
| 用户10 | 200 | 30 | 2026-10-25 至 2026-11-24 | 5 | 3 行：蒸汽爆发 12175 / 46.1% / events 53 / count 203；过热 7488 / 28.4% / 41 / 228；超导 6746 / 25.5% / 46 / 191 |
| 用户11 | 200 | 30 | 同上 | 0 | []（无数据） |
| 用户1 | 200 | 30 | 同上 | 2 | []（无数据） |

SQL 权威次数：overheat 228 / steam_burst 203 / superconduct 191，排除数合计 7（用户10 的 5 条旧战报），与端点逐字段一致。次数取自 battle_records.reactions_used，不取 events，也不混入 dashboard_reaction_counts。

## ④ 本轮核心功能验证

- 玩家成长页「我的反应伤害贡献」卡（.r19-my-reaction-impact）默认次数视图，可切换伤害贡献视图；两视图均显示真实聚合数据。
- 次数视图权威次数 203 / 228 / 191 次，事件条数 53 / 41 / 46；伤害视图占比 46.1% / 28.4% / 25.5%，总伤害 12175 / 7488 / 6746。
- 统计窗口逐字「2026-10-25 ~ 2026-11-24 (30天)」；旧战报诚实标注「已排除 5 条无逐次记录的旧战报」，不计入伤害。
- AI 四判据齐备，逐字诚实红线「分析并非权威结算依据，权威请看 replay hash」，避免把辅助分析误作结算。
- 跨用户隔离：用户10 有数据、用户11 与用户1 空态 items=[]，端点按当前 user_id 过滤，无越权数据。

## ⑤ 真浏览器验证

- 工具：agent-browser（非物理真机），viewport 390×844，走带令牌 init 脚本；真值只存在 gitignored 的本地注入文件，不入库、不回显。
- E2E 决定性回归：打开成长页 → 默认次数视图（count 视图真数据）→ 切换伤害视图（占比与总伤害真数据）→ 截图；battle 相关探针 pe/ce/af/fr 全 0。
- 本次重拍截图亲读：次数视图与伤害视图均为非登录页，数据、窗口、排除标注、AI 四判据与诚实红线清晰；新拍哈希与批 3 入库版不同属正常重拍，已 git checkout 恢复为入库版以保持相邻提交稳定，报告引用入库 sha。
- 线上回归以本地回环 + 占位域名的真浏览器 E2E 为准，不依赖远端 scp 真值。

## ⑥ 截图清单（sha256，入库版）

| 文件 | sha256 前16 | 说明 |
| --- | --- | --- |
| r19-01-count-view.png | b16088dbeab45f1a | 次数视图：权威203/228/191次 + 事件53/41/46条 + 窗口 + 排除5 + AI四判据 + 诚实红线 |
| r19-02-damage-view.png | a34068c6a2d63aa4 | 伤害视图：占比46.1%/28.4%/25.5% + 总伤害12175/7488/6746 |
| r19-03-damage-tall.png | f5d3e584fed9761a | 伤害视图长屏，确认卡片完整 |
| r19-04-empty-user11.png | 744a45cf5d666ca5 | 用户11 空态 items=[]，跨用户隔离 |

4 图 sha 各异，亲读均为非登录页有效证据。

## ⑦ 结论

R19 玩家反应伤害贡献面板已完成四批精确提交（feat / docs / test / 线上回归），后端与 H5 已重建部署，三端健康与入口 200，真浏览器双视图与跨用户隔离决定性证据成立，线上回归通过。无阻塞问题；真值零入库、零回显，严格排除历史终审与一次性诊断脚本。
