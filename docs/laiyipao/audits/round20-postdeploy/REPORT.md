# R20 部署后线上回归报告 · 战斗效率趋势分析

- 任务：t170「R20提交部署：分批精确提交+三端部署+线上回归」。
- 环境：测试环境域名统一占位 `<YOUR_TEST_DOMAIN>`，本机回环 `http://127.0.0.1` 仅用于内部连通性核验；真值（令牌/口令/私钥/真实域名/IP/个人路径）零入库零回显。
- 浏览器口径：agent-browser 自动化浏览器（**非物理真机**），H5 视口 390×844、admin 视口 1258×588；每步均采 DOM 探针与截图并亲读。

## 一、部署产物表

| 端 | 产物 | sha256 | 说明 |
|---|---|---|---|
| 后端 | `laiyipao/deploy/bin/lyp-server-amd64` | `f3113ad6fc60ef9531c4f38f9fa30500bcf5c674777bc40d5f412ef472dbb72a` | 本次重建，ELF；含 `/api/v1/me/battle-efficiency` 与 `/api/v1/admin/battle-efficiency` |
| H5 | `laiyipao/miniapp/dist/build/h5/assets/index-DQYmwaff.js` | `16e6d16712171dc2f419078a7d2c77994aef329f88f06bf761bef7236be17538` | 本次重建；`pages-battles-battles.MuvuiEZA.js` 与 `battles-Do0XAu4M.css` 含 `r20-efficiency-trend` |
| admin | `laiyipao/admin/dist/assets/index-CGo_zhD3.js` | `d5e9c483992f2a370b5def55d7418da958b695deda4db9c3446e26bd04d58c70` | 本次重建 |
| admin | `laiyipao/admin/dist/assets/DashboardView-dKUupfJh.js` | `5d2c9cf1114513e390b92ffbc3fd103caad36975e952d01da12cfe3194f7fc0a` | 含 `r20-efficiency-trend`（grep -rl 命中） |

三端构建均 exit 0。后端 `go run ./cmd/api` 运行实例为同源码，产物重建后哈希与上一轮不同；admin/H5 运行 dev server 直读新源码，产物已同步重建以确保静态部署一致。

## 二、集成验证（部署前）

- 契约/渲染：admin 2 文件 14/14 PASS；miniapp 2 文件 9/9 PASS；后端 `go test ./internal/service/... -run TestAggregateBattleEfficiency` 3 条纯逻辑 PASS；DB E2E 3 条因本机无管理库 DSN 按仓库约定 SKIP，由下文 HTTP+SQL 实测补证。
- 全量：admin 67 文件 755 tests PASS；miniapp 196 文件 1799 tests passed（2 skipped）。
- 真值守卫：`go test ./internal/domain -run 'TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo|TestNoJWTLiteralInRepo'` exit 0；终审 t169 verdict=pass（六门禁全过，H5(ms)/admin(s) 时长单位差异裁定为展示口径差异非缺陷）。

## 三、端到端 HTTP 抽样（三用户隔离 + SQL 独立核验）

端点 `GET /api/v1/me/battle-efficiency?window_days=30`（回环，Bearer 由会话换取，不入库）：

| 用户 | HTTP | 场次 | shots/hits | 命中率 | buckets | 结论 |
|---|---|---|---|---|---|---|
| 10 | 200 | 7 | 1032/633 | 61.3% | 2026-11-22(3/394/287→72.8%)、2026-11-23(4/638/346→54.2%) | 与 SQL 逐字段一致 |
| 11 | 200 | 0 | 0/0 | 0 | `[]`，`ai_insight={}` | 诚实空态，不编造 |
| 1 | 200 | 2 | 0/0 | 0 | 2026-11-23(2/0/0→0) | SUM=0 归零，无 NaN/Infinity |

- 窗口 `2026-10-25` 至 `2026-11-24`；SQL 独立核验用户10 分桶 2026-11-22=3/394/287、2026-11-23=4/638/346，用户 1/11 场次=2/0 且 shots/hits=0，与端点逐字段一致。
- 非法参数 `window_days=31` → HTTP 400 `window_days 只接受 30`（主动校验，不计入正常流量失败）。
- admin 端点 `GET /api/v1/admin/battle-efficiency` 全平台：103 场/10922 shots/6009 hits/55.0%/avg_heat 27.4/leaked 1.1/avg_duration_ms 32857.3，两桶 73.1%/54.1%，与真浏览器嗅探响应逐字段一致。

## 四、本轮核心功能验证

- **玩家端**：H5 `.r20-efficiency-trend[role=region]` 汇总四项 KPI（命中率/峰值热量/漏怪/时长）+按日趋势条 `data-r20-bucket` 升序；`near 30 days` 窗口保留；无战报用户逐字空态「近 30 天无战斗记录」，AI 卡不渲染。
- **管理端**：admin `.r20-efficiency-trend` 汇总/趋势双视图切换，5 指标以 `toFixed(1)` 输出、非有限数→「—」；趋势条按 bucket 升序；空态逐字「近 30 天无战斗记录」；切视图不阻塞权威看板（try-catch 降级）。
- **AI 四判据齐备**：可见标题「AI · 战斗效率诊断」+真实结论（admin 55.0% 峰值热量下降建议关注 heat / H5 61.3%）+逐字依据行「依据：近 30 天 battle_records 的 shots/hits/heat_max 聚合；场次以 battle_records 行数为准」+可执行 CTA（「去战斗→」/「查看战报 →」）。
- **诚实红线逐字**：H5「辅助诊断非权威结算依据，权威请看 replay_hash」；admin「效率趋势为辅助诊断；不参与结算、score、星级、loot，也不覆盖 replay_hash」。SUM=0 归零与空态均如实呈现。
- **不回退**：R18 /admin/reaction-damage-impact、R19 /me/reaction-damage-impact、R15-R17 时间线生态、25% AoE、顶栏脱敏、三端令牌锚点均保留（契约测试分组断言）。

## 五、真浏览器验证（agent-browser 非物理真机）

- **admin 决定性 DOM（本次部署后实跑）**：登录成功（`loginPage=false`），汇总态 `hasCard=true, role=region`，窗口「统计窗口：近 30 天（2026-10-25 至 2026-11-24）」，汇总行「场次 103 / 命中率 55.0% / 平均峰值热量 27.4 / 场均漏怪 1.1 / 场均时长 32.9s」；点击「趋势」后汇总行隐藏、趋势条 `data-r20-bucket=2026-11-22`（73.1%）、`2026-11-23`（54.1%）升序呈现；AI 四判据与诚实红线如上；`efficiencyResponses[0].status=200` 且与 SQL 逐字段一致；`pe=ce=af=fr=0`。
- **H5 决定性证据（t168 复查，同源码同构建口径）**：用户10 `.r20-efficiency-trend` 汇总 61.3%/98.7/0.0/87278.6ms、两条趋势 72.8%·54.2%，用户11 逐字空态「近 30 天无战斗记录」，探针全 0。
- 截图亲读：`r20-05-admin-trend.png`（1258×588）确认为运营后台数据看板的「战斗效率趋势」区块，两条真实趋势条 73.1%/54.1% + AI 卡 + 诚实行，非登录页；其余五图见 t168/t169 亲读记录（非登录页）。
- 本次 H5 E2E 脚本因 agent-browser `eval` 会话挂起被终止，未产生新截图；已保留批3入库的决定性截图（哈希未变）并以上述 HTTP+SQL 与终审证据交叉补证，不以失败样本冒充证据。

## 六、截图清单

目录 `docs/laiyipao/audits/round20-combat-efficiency-trends/screenshots/`，六图 sha256 各异、亲读非登录页：

| 文件 | sha256 | 口径 |
|---|---|---|
| r20-01-h5-efficiency.png | `6bd832f9b5be2df6aa21e3a8674d219ea9f086ed3a711f930f1c1ce587b90c64` | H5 顶部 KPI |
| r20-02-h5-scroll.png | `829bac239e97b60e2063d6a160edcaa44d4be579a421f6bca19609f7ed338546` | H5 滚动态 |
| r20-03-h5-card.png | `efa5ac86e85bd849e2c4a3e7f092e79ba3c4a827efbb7643b8d92ea756e2496b` | H5 效率卡完整 |
| r20-04-admin-summary.png | `36e9e213a9dce6376af6a94628df07f7625262f0799ad24e15374fddd91418cf` | admin 汇总视图 |
| r20-05-admin-trend.png | `d177d7b6679d7cb249b2f7149cc932927dd89dfa0181f607f113778a0d9b1bf6` | admin 趋势视图（本次亲读） |
| r20-06-h5-empty.png | `4741b0c1570276e0bb76e6c864dc0d08fd864f694923f29df7e35b151b48e838` | H5 空态 |

## 七、结论

R20 分批提交与三端部署完成：后端/H5/admin 产物均已重建，`healthz/readyz/H5/admin` 全部 HTTP 200；线上回归确认 me/admin 端点跨用户隔离、30 天窗口、SUM=0 归零、诚实空态与 AI 四判据在真实浏览器与 HTTP/SQL 双重核验下成立，探针全 0；六图哈希各异且亲读非登录页；真值零泄露、R1-R19 不回退。本轮无阻塞缺陷，t170 收官。
