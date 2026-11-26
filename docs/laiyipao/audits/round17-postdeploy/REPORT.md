# R17 部署后线上回归报告（round17-postdeploy）

> 环境：测试环境部署后线上回归；浏览器为 agent-browser 驱动的 Chromium **真浏览器，非物理真机**（验真页视口 390×844）。
> 域名一律用 <YOUR_TEST_DOMAIN> 占位；真值只从 laiyipao/deploy/.local.keys 读取，本报告与截图不含真域名、真 IP、口令、令牌、私钥。
> 本轮部署为 H5 单端重建（新增 R17 验真页逐次反应序列对照卡 + 引擎旁路采集）；admin 与后端产物无改动沿用既有版本，后端源码（economy.go replay 下发 reaction_timeline）由 go run 实例承载。

## 1. 部署产物

| 端 | 产物 | sha256 | 说明 |
| --- | --- | --- | --- |
| H5 | index-CIZj2kWc.js | eadd221a0a6afab59ab174d70c68b6d44cb727a87cbf5bbfdc15f60e64143c85 | 本轮重建入口，含 r17-reaction-diff（count=1） |
| H5 | pages-verify-verify.C4yX4tB-.js | — | 验真页 chunk，含 R17 对照卡逻辑 |
| admin | BattlesView-CBILzIvZ.js | 11351f2cd66de40524e97f9fb5d52ddef9269b62558c2eadee963092a6ea7ec4 | 无改动，沿用 R16 |
| admin | index-Bc7F3tn5.js | 60642e479c0395fe84342f02f8f49c0c9a83e5b1d54ad94f963f1bd7b10a39ba | 无改动，沿用 R16 |
| 后端 | lyp-server-amd64 | a3317ac472fefb16bff03c250d7ff6d2a859dfaccbd1c315e6d58e84e86be1d1 | 二进制沿用；R17 为 economy.go 源码改动，go run 实例已承载 |

服务复验：/healthz=200、/readyz=200、H5(5174)=200。
Migration 00031 reaction_timeline jsonb 列已应用（lyp-qa-pg 回环 15440）：battle 378 含 object（schema_version=reaction-timeline-v1），battle 339 为 []。

## 2. 集成验证（部署前）

- H5 全量测试 193 文件 / 1786 passed | 2 skipped（1788）exit 0；R17 契约测试 5/5、渲染测试 9/9、引擎测试全绿（t146-t150 复查记录）。
- 真值守卫 `go test ./internal/domain -run 'TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo|TestNoJWTLiteralInRepo' -count=1` → ok，exit 0。
- 前三批提交逐批 STAGED_LEAK_TOTAL=0；R17 data/（含真实 token 的 qa-before.dump）与 auth-init.js 已被 .gitignore:282-283 拦截，未入库；auth-init.tpl.js 用 __LYP_TOKEN__ 占位。

## 3. 端到端 HTTP 抽样

| 项 | 结果 |
| --- | --- |
| /healthz / /readyz | 200 / 200 |
| H5 验真页首页 | 200 |
| GET /api/v1/battle/378/replay | 200，含 replay_hash + reaction_timeline（schema=reaction-timeline-v1，69 事件） |
| POST /api/v1/battle/verify | 200，验真通过，提交哈希 7468 252A 57F6 BEFE 一致 |
| 鉴权接口（未带令牌） | 401（预期，未带令牌） |
| HEAD_TRUTH_LEAK | 0 |

## 4. R17 核心：验真页逐次反应序列对照

battle_id=378（新版本有逐次时间线）真浏览器 DOM 探针：

| 判据 | 实测 |
| --- | --- |
| 对照卡容器 .r17-reaction-diff | box=true、role=region |
| summary | 「一致 69 条 · 差异 0 条」 |
| 依据行 | 本地重放 reaction_timeline.events 与服务端逐条比对，辅助对照线索，非权威判据，权威裁定仍以 replay_hash 为准 |
| CTA | 「查看权威裁定 ↑」 |
| 行数 / 差异 | rows=69，差异 0 条（五字段 wave/tick/reaction/enemy_uid/damage 逐条全等） |
| 权威裁定 | 本地重放哈希 7468 252A 57F6 BEFE = 服务端记录，验真通过 |
| 探针 | pageErrors=0、consoleErrors=0、apiFailures=0、failedRequests=0 |
| 域名泄露 | bodyHasDomain=false |

诚实红线：时间线仅为辅助对照线索，不改 replay_hash、不改 score/loot/星级、不冒充验真通过/失败；权威裁定仍以 replay_hash 为准。
空态：battle_id=339（DB reaction_timeline='[]'）逐字空态「该战报无逐次反应记录，暂无法对照（旧版本结算）」，rows=0，探针全 0。t150 终审如实记录：真浏览器下滚到空态卡的 DOM 探针因 agent-browser daemon EOF 失败，空态文案由 verify.vue:172 源码 + verify_r17_diff.test.ts 契约双重覆盖，不判 FAIL。

## 5. 真浏览器验证（agent-browser，非物理真机）

- H5 390×844 真浏览器：验真页 `#/pages/verify/verify?battle_id=378` → 自动拉复现信息 → 点本地重放 → 对照卡真实渲染 69 行逐条全等、诚实红线与 CTA 全在；r17-06、r17-03 由队长与 t150 终审亲读确认为非登录页真实运行页面。
- 首轮 E2E 出现假阴性：提交版 e2e.sh 走手动 fill input.inp '378' + 点「取复现信息」路径，无 API 请求也无失败（hasHash=false、replay=[]、NOBTN）——该脚本正是 t149 报告标记的『battle_id 填错的早期失败样本』。决定性路径为带 `?battle_id=378` 打开页面由 onLoad 自动拉复现（verify.vue onLoad(query)），与 t149/t150 决定性证据完全一致。该假阴性为审计脚本路径问题，非产品缺陷。
- 探针全 0、bodyHasDomain=false，页面无真实域名/IP/口令/令牌。

## 6. 截图清单（5 张，均 390×844 非物理真机，sha256 两两不同）

| sha256 前16 | 文件 | 内容（亲读） |
| --- | --- | --- |
| fb14829e97f1da6b | r17-01-verify-replay.png | battle 378 取到复现信息 |
| 55801ec8b25ca71d | r17-02-diff-top.png | 取到复现信息 + AI 复核提示 |
| ddc7c6fa1e731aca | r17-03-diff-scroll.png | 本地重放哈希 7468 252A 57F6 BEFE 一致 / 分数可复现 / 验真通过 |
| 3617bca2228a8a9875 | r17-05-old-empty-clean.png | battle 339 旧战报（空态由源码+契约覆盖） |
| 4efaf2a7a00f55605 | r17-06-r17-card.png | 一致 69 条 · 差异 0 条 + 诚实红线 + CTA + 验真通过 |

截图位于 docs/laiyipao/audits/round17-verify-reaction-diff/screenshots/（批3已提交）。误导性/失败样本 r17-04-old-empty.png（与 r17-02 同 sha）、r17-01-verify-top.png / r17-02-diff-card.png（battle_id 填错，sha e1d089ad）均已排除不提交。

## 7. 结论

R17 三端部署与线上回归全绿：H5 R17 产物已进包，battle378 一致 69 条差异 0 条 DOM 决定性证据 + 权威哈希一致验真通过、诚实红线达成、探针全 0、5 图各异且亲读非登录页、零真值泄露。E2E 首轮假阴性已定位为审计脚本 battle_id 填错路径，决定性证据由 t149 复查与 t150 终审（verdict=pass 六门禁全过）确认，非产品缺陷。R17 闭环完成。
