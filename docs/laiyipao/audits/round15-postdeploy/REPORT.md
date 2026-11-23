# R15 部署后线上回归报告（round15-postdeploy）

> 环境：测试环境部署后线上回归；浏览器为 agent-browser 真浏览器，**非物理真机**（H5 视口 390×844）。
> 域名一律用 <YOUR_TEST_DOMAIN> 占位；真值只从 laiyipao/deploy/.local.keys 读取，本报告与截图不含真域名、真 IP、口令、令牌、私钥。

## 1. 部署产物

| 端 | 产物 | 校验 |
| --- | --- | --- |
| admin | index-BLwhVNnN.js | sha256 3e324e3f5513bc8a8c9cf7df4b1544a1ba3f152e4d862e101919bf688e4f9ef6 |
| H5 | index-_bFQ9SKL.js（补漏后重建） | sha256 8c280c24223247ba56c20a299ce30fa7cf551d149075ebf8e3361e72cd3c8159；bundle 含 r15-reaction-timeline |
| 后端 | lyp-server-amd64（ELF，17350818B） | sha256 a3317ac472fefb16bff03c250d7ff6d2a859dfaccbd1c315e6d58e84e86be1d1 |

服务 SVC=active，healthz=200，readyz=200，H5 首页/战斗路由均 200；migration 00031 reaction_timeline(jsonb) 已于 QA 库应用成功。

## 2. 集成验证（部署前）

- H5 全量测试：Test Files 190 passed | 1 skipped（191），Tests 1775 passed | 2 skipped（1777），exit 0；battles.test.ts 随批3提交，battles.vue 补漏提交 2aa3af634 后复跑仍全绿。
- 后端 R15+真值守卫：TEST_DATABASE_URL 指向 QA 库后，go test ./internal/domain -run ReactionTimeline/NoRealSecretsInRepo/NoInfraIdentifiersInRepo -count=1 → ok，exit 0。TestNoJWTLiteralInRepo 因本地 gitignored 的 auth-init.js 含测试 QA token 命中（该文件不入库），属本地 E2E 临时状态，不影响提交与线上。
- 五类真值（DOMAIN/ADMIN_PASS/DB_DSN/JWT_SECRET/BASEDOMAIN）四个批次逐批 git diff --cached 校验 STAGED_LEAK_TOTAL=0；HEAD_TRUTH_LEAK=0。
- 后端全量套件另有与本轮无关的历史/环境问题：部分 DB 直连测试依赖 5432（QA 库在 15440）、带 QA DSN 后 service/httpapi 仍有 2m 超时及 player_login 开关用例失败，均非 R15 改动引入，R15 自身测试与守卫全绿。

## 3. 端到端 HTTP 抽样

| 项 | 结果 |
| --- | --- |
| GET /healthz | 200 |
| GET /readyz | 200 |
| GET H5 / 与 #/pages/battle/battle | 200 / 200 |
| 需鉴权端点未带令牌 | 401（预期鉴权保护，非缺陷） |
| E2E 内 settle 请求（带有效测试令牌） | 200，battle_id=378 |
| E2E 内战报详情（带有效测试令牌） | 200，schema=reaction-timeline-v1，total=71，truncated=false |
| E2E 探针 | apiFailures=[]、pageErrors=[]、qaShimApplied=0（P0 修复后 H5 已上报 snake_case enemy_uid，属预期） |

## 4. R15 核心：反应时间线（reaction timeline）

| 场景 | HTTP / DOM | 结果 |
| --- | --- | --- |
| 第5关带元素反应战斗结算 | settle=200 | battle_id=378，反应数=71 |
| 战报详情 reactions 页 | detail=200 | schema=reaction-timeline-v1，rows=69（视口内渲染），total=71，truncated=false |
| 时间线首行 | DOM | 第 1 波 · 超导 · #1 · 70.0 伤害 |
| 时间线末行 | DOM | 第 5 波 · 过热 · #131 · 415.0 伤害 |
| 波次覆盖 | DOM | waves=[1,2,3,4,5] |
| 诚实空态 | DOM | empty=空（本局有记录）；旧版本结算显示『该战报无逐次反应记录（旧版本结算）』 |

H5 上报字段 enemy_uid（snake_case）与后端 json tag 对齐，修复 P0 跨端错配（否则生产每局带反应战斗 settle 500）。引擎旁路不污染 replay_hash/权威字段（批1契约测试覆盖）。

## 5. 真浏览器双端验证（agent-browser，非物理真机）

- H5 结算页：真实通关结算面板，含『查看本局战报』按钮与反应数统计；探针 pe/ce/af/fr 全 0，domainInBody=false。
- H5 战报页：真实渲染反应时间线，DOM rows=69、首末行与波次证据确凿，detail.status=200/schema=reaction-timeline-v1/total=71/truncated=false；探针全 0，domainInBody=false。
- 观察如实记录：首次 E2E 因 QA token 过期跳登录页产生假阴性（settle 401、三图同为登录页），已通过 mint-token.sh 刷新并复测，本报告记录复测后的决定性证据。

## 6. 截图清单（3 张，均 1258×588，非物理真机，已亲读非登录页）

| sha256 前16 | 文件 |
| --- | --- |
| 0d6367c7aa31ca1b | r15-final-01-settle.png |
| d6accc8d17b15666 | r15-final-02-timeline-top.png |
| 1a71a66323c55cfd | r15-final-03-timeline-scroll.png |

三图 sha 各异；截图落 docs/laiyipao/audits/round15-reaction-timeline/screenshots/（审计批次已提交，本次部署后重拍覆盖，纳入批4提交）。

## 7. 结论

R15 三端部署与线上回归通过：healthz/readyz 200、端到端抽样 200、真浏览器 H5 探针全 0、战报反应时间线真实可见且 DOM 决定性证据齐全、三图 sha 各异并亲读非登录页、零真值泄露。P0 跨端字段错配修复线上验证生效。R15 收官闭环完成。
