# R16 部署后线上回归报告（round16-postdeploy）

> 环境：测试环境部署后线上回归；浏览器为 agent-browser 驱动的 Chromium **真浏览器，非物理真机**（admin 视口 1440×900）。
> 域名一律用 <YOUR_TEST_DOMAIN> 占位；真值只从 laiyipao/deploy/.local.keys 读取，本报告与截图不含真域名、真 IP、口令、令牌、私钥。
> 本轮部署为 admin 单端重建（新增 R16 战报详情逐次反应时间线）；H5 与后端无源码改动，沿用 R15 已部署产物。

## 1. 部署产物

| 端 | 产物 | sha256 | 说明 |
| --- | --- | --- | --- |
| admin | index-Bc7F3tn5.js | 60642e479c0395fe84342f02f8f49c0c9a83e5b1d54ad94f963f1bd7b10a39ba | 本轮重建入口 |
| admin | BattlesView-CBILzIvZ.js | 11351f2cd66de40524e97f9fb5d52ddef9269b62558c2eadee963092a6ea7ec4 | 含 r16-reaction-timeline（count=1），R16 核心产物 |
| H5 | index-_bFQ9SKL.js | 8c280c24223247ba56c20a299ce30fa7cf551d149075ebf8e3361e72cd3c8159 | 无改动，沿用 R15 |
| 后端 | lyp-server-amd64 | a3317ac472fefb16bff03c250d7ff6d2a859dfaccbd1c315e6d58e84e86be1d1 | 无改动，沿用 R15 |

服务复验：/healthz=200、/readyz=200、admin(5175)=200、H5(5174)=200。
Migration 00031 reaction_timeline jsonb 列已应用（lyp-qa-pg 回环 15440）：battle 378 含 object（schema_version=reaction-timeline-v1），battle 339 为 []。

## 2. 集成验证（部署前）

- admin R16 契约测试 5/5、渲染测试 9/9、`npx vue-tsc --noEmit` exit 0；admin 全量 722/722 绿。
- 真值守卫 `go test ./internal/domain -run 'TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo|TestNoJWTLiteralInRepo' -count=1` → ok，exit 0。
- 批3 审计 LEAK_TOTAL=3 经逐项复核全为误报（config.go 公开默认口令、bodyHasDomain 检测正则、个人路径占位描述），真实泄露=0；data/ 含真实 dump 已被 .gitignore:280 拦截，未入库。

## 3. 端到端 HTTP 抽样

| 项 | 结果 |
| --- | --- |
| /healthz / /readyz | 200 / 200 |
| admin 登录 → dashboard | 200（token 存 localStorage lyp_admin_token） |
| admin 战报详情（battle_id=378） | 200，reaction_timeline schema=reaction-timeline-v1，69 事件 |
| H5 首页 | 200 |
| HEAD_TRUTH_LEAK | 0 |

## 4. R16 核心：admin 战报详情逐次反应时间线

battle_id=378 点开详情抽屉后 DOM 探针（agent-browser 真浏览器）：

| 判据 | 实测 |
| --- | --- |
| 时间线容器 .r16-reaction-timeline | box=true、role=status |
| 标题 / summary | AI · 反应时间线 / 本局记录 69 次逐次反应，集中在第 5 波 |
| 依据行 / CTA | 依据：reaction_timeline.events（wave/tick/reaction/enemy_uid/damage），展示性归因、非权威结算 / 发起验真 → |
| 行数 / 波次 | rowsCount=69，waves=[1..5] 升序 |
| 首行 / 末行 | 第 1 波 · 超导 · #1 · 70伤害 / 第 5 波 · 过热 · #131 · 415伤害 |
| 脏值 | hasUndefined=false、hasNaN=false、hasR15=false |
| 探针 | pageErrors=0、consoleErrors=0、apiFailures=0、failedRequests=0 |
| 域名泄露 | bodyHasDomain=false |

诚实空态：battle_id=339（DB reaction_timeline='[]'）逐字显示「该战报无逐次反应记录（旧版本结算）」，rowsCount=0、summary/CTA 为空，探针全 0（证据截图 r16-05-old-empty-339.png）。
展示性归因红线：时间线不参与 replay_hash、不改 score/loot/星级、非权威结算，与 SPEC 一致。

## 5. 真浏览器验证（agent-browser，非物理真机）

- admin 1440×900 真浏览器：登录 → 战报列表 → 点开 378 详情抽屉 → 时间线顶部（r16-02）与滚动后（r16-03/06）均真实渲染逐行数据，亲读确认非登录页。
- 首轮 E2E 出现假阴性：点「详情」后抽屉时间线未渲染（rowsCount=0，r16-02 拍成列表页）。定位为审计脚本 bug 而非产品缺陷——find-row.js 给整行加 data-battle-id，e2e.sh 选择器 `[data-battle-id=378] .el-button` 命中行内第一个按钮（复制）而非详情按钮。修复为对含「详情」文本的按钮打 data-detail-id，选择器改 `[data-detail-id=378]`；复测拿到上述决定性 DOM，与 t143/t144 终审证据完全一致。
- 探针全 0、bodyHasDomain=false，页面无真实域名/IP/口令/令牌。

## 6. 截图清单（5 张，均 1440×900 非物理真机，sha256 两两不同）

| sha256 前16 | 文件 | 内容（亲读） |
| --- | --- | --- |
| 349ee46bc47dcfcf | r16-01-battles-list.png | 战报列表，378/341/339 行可见 |
| de8b36c8147e5f16 | r16-02-timeline-top.png | 378 抽屉顶部：AI·反应时间线 69 次/第 5 波 + 前若干行真实数据 |
| aeb9a08c1cd1611b | r16-03-timeline-scroll.png | 抽屉滚动后第 1-2 波逐行渲染 |
| 9d973a4a4d9b80a2 | r16-05-old-empty-339.png | 339 旧战报逐字空态 |
| aa29cdcb58324e0a | r16-06-timeline-bottom.png | 时间线尾部第 5 波末行 |

截图位于 docs/laiyipao/audits/round16-admin-reaction-timeline/screenshots/（批3已提交）。误导性样本 r16-04-old-empty.png（实为 battle41 有 71 行数据、非空态）已排除不提交。

## 7. 结论

R16 三端部署与线上回归全绿：admin 时间线 R16 产物已进包，battle378 有数据态 69 行 DOM 决定性证据、battle339 诚实空态、探针全 0、5 图各异且亲读非登录页、零真值泄露。E2E 首轮假阴性已定位为审计脚本选择器 bug 并修复复测通过，非产品缺陷。R16 闭环完成。
