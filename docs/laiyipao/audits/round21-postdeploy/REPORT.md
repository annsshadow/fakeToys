# R21 部署后线上回归报告 · 玩家卡点教练

- 任务：t176「R21提交部署：四批精确提交+三端部署+线上回归」。
- 环境：测试环境域名统一占位 `<YOUR_TEST_DOMAIN>`，本机回环 `http://127.0.0.1` 仅用于内部连通性核验；真值（令牌/口令/私钥/真实域名/IP/个人路径）零入库零回显。
- 浏览器口径：agent-browser 自动化浏览器（**非物理真机**），H5 视口 390×844；每步均采 DOM 探针与截图并亲读。

## 一、部署产物表

| 端 | 产物 | sha256 | 说明 |
|---|---|---|---|
| 后端 | `laiyipao/deploy/bin/lyp-server-amd64` | `3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6` | 本次重建，ELF；含 `GET /api/v1/me/level-coach` |
| H5 | `laiyipao/miniapp/dist/build/h5/assets/index-DiC6E98L.js` | `9352267e51e8e90e42467c28d1118d16f0c0f67f27d86682388adbcf76d48e5f` | 本次重建；`pages-stage-stage.Wx_xBZdU.js` 与 `stage-CbVmmazn.css` 含 `r21-level-coach` |
| admin | `laiyipao/admin/dist/assets/index-CGo_zhD3.js` | `d5e9c483992f2a370b5def55d7418da958b695deda4db9c3446e26bd04d58c70` | 本次重建后与 R20 产物字节一致；本轮 admin 仅类型声明更新，无界面渲染改动 |

三端构建均 exit 0。后端 `go run ./cmd/api` 运行实例为同源码；H5/admin 运行 dev server 直读新源码，产物已同步重建确保静态部署一致。

## 二、集成验证（部署前）

- 契约/渲染：miniapp `stage_r21_level_coach.contract.test.ts`(6) + `stage_r21_coach.test.ts`(5) = 11/11 PASS；admin `R21LevelCoach.contract.test.ts` 8/8 PASS；后端纯逻辑 `TestAggregateLevelCoachFieldsAndStuck` / `TestAggregateLevelCoachEmptyAndZeroDivision` 2 条 PASS。
- 全量（t174 复查记录）：miniapp 198 文件 1810 tests passed（2 skipped）；admin 763 tests PASS。
- 真值守卫：`go test ./internal/domain -run 'TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo|TestNoJWTLiteralInRepo' -count=1` exit 0；终审 t175 verdict=pass（六门禁全过，队长关注五点逐项核实）。

## 三、端到端 HTTP 抽样（三用户隔离 + SQL 独立核验）

端点 `GET /api/v1/me/level-coach?window_days=30`（回环，Bearer 由会话换取，不入库）：

| 用户 | HTTP | items | 关卡明细 | 结论 |
|---|---|---|---|---|
| 10 | 200 | 3 行 | level1 3/3/5/5.0、level5 3/3/5/5.0、level40 1/1/8/8.0，stuck 全 false | 多关按 attempts 降序，与 SQL 逐字段一致 |
| 100 | 200 | 1 行 | level40 尝试3/胜0/最好7波/平均6.3波/lose/stuck=true | 连续失利阈值=2 命中，SQL 一致 |
| 11 | 200 | `[]` | 无记录 | 诚实空态，非 null，不编造建议 |

- 窗口统一 `2026-10-25` 至 `2026-11-24`；t174/t175 已用 ROW_NUMBER 窗口函数独立核验 stuck（连续 lose 头数≥2 遇 win 即停）与八字段逐值一致。
- 参数守卫（本次部署后实测）：`window_days=7` → 400；重复 `window_days=30&window_days=30` → 400；无鉴权 → 401。
- 端点严格按会话 user_id 过滤，前端无 user_id 入参。

## 四、本轮核心功能验证

- **玩家端**：H5 `.r21-level-coach[role=region]` 展示 `.r21-coach-row[data-r21-level]` 逐关尝试/胜/最好波/平均波，stuck 行标注「卡点」；星级/单局诊断/G132 本地卡全部保留，不替换既有能力。
- **AI 四判据齐备**：标题「AI · 卡点教练」+ 真实结论（近30天该关尝试3次、胜0次、最好第7波，建议先补强）+ 逐字依据行「依据：近 30 天 battle_records 的 level_id/result/wave_reached 聚合；连续失利沿用 StuckThreshold=2」+ CTA「去补强 →」。
- **诚实红线逐字**：「本卡为辅助诊断，不参与结算、score、星级、loot；权威结果以 replay_hash 为准。」分母为 0 时平均波次归零、无 NaN/Infinity；空态逐字「近 30 天无关卡尝试记录」，AI 空态「近30天暂无足够记录生成教练建议。」不编造。
- **不回退**：R15–R20 端点（时间线/replay 对照/伤害贡献/效率趋势）与三端令牌锚点均保留；无新增 `--r21-` 令牌。

## 五、真浏览器验证（agent-browser 非物理真机）

- **本次部署后实跑**：会话 lyp-r21-data（用户100）/`lyp-r21-empty`（用户11），视口 390×844，打开选关页 + reload + DOM 探针。
- **有数据态**：`.r21-level-coach` 存在、`role=region`、窗口「2026-10-25 ~ 2026-11-24」、`rowsCount=1`、行文本「第 40 关卡点尝试 3 次胜 0 次最好第 7 波平均 6.3 波」；`coachResponses[0].status=200` 且 items 与 HTTP 抽样逐字段一致；`hasUndefined=false, hasNaN=false`。
- **空态**：`rowsCount=0`、`empty="近 30 天无关卡尝试记录"`、`coachResponses[0].status=200 items=[]`，无编造结论。
- **探针全 0**：两态 `pageErrors/consoleErrors/apiFailures/failedRequests` 均为 `[]`，`loginPage=false`。
- 本次 E2E 重拍先覆盖了 r21-data.png 与 r21-empty-card.png 两张（内容同为选关页/空态特写），为保持相邻提交稳定已 `git checkout HEAD` 恢复为批3入库的决定性版本，四图哈希与清单一致；跨用户隔离脚本在写完三份 HTTP JSON 后进程挂起，已终止并直接用同一回环端点复验三项参数守卫，结论一致。不以失败样本冒充证据。

## 六、截图清单

目录 `docs/laiyipao/audits/round21-player-level-coach/screenshots/`，四图 sha256 各异、亲读非登录页（t174/t175 与本次重拍交叉确认）：

| 文件 | sha256 | 口径 |
|---|---|---|
| r21-data.png | `d36c4f162dfc7ef8ab44d0d279ada661996b44c5bb3ac8c3150cf416159389ed` | 用户100 选关页顶部 |
| r21-data-card.png | `65e67d6c0c5e80973e255168153a7ad599d27bb89b5a3a2fe9ea687910eb9301` | 卡点教练卡特写（第40关3/0/7波/6.3波 + AI + 依据 + CTA + 诚实行） |
| r21-empty.png | `b142cc03ea68c6b2492fdcc083f93b426bac612344e82b77eb056b4e3da21c9a` | 用户11 选关页空态入口 |
| r21-empty-card.png | `063c9b4d358051d1f21e10180e50c1c0ab37532f4361bfd93e289ae57d2f25ab` | 空态卡特写（逐字空态 + AI 空态 + 诚实行） |

## 七、结论

R21 四批提交与三端部署完成：后端/H5 产物均已重建（admin 类型更新后产物字节一致），`healthz/readyz/H5/admin` 全部 HTTP 200；线上回归确认 me 端点跨用户隔离、30 天窗口、连续失败阈值遇胜即停、分母归零、诚实空态与 AI 四判据在真浏览器与 HTTP/SQL 多重核验下成立，探针全 0；四图哈希各异且亲读非登录页；真值零泄露、R1–R20 不回退。本轮无阻塞缺陷，t176 收官。
