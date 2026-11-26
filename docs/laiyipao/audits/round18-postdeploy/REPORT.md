# R18 部署后线上回归报告（round18-postdeploy）

> 环境：测试环境部署后线上回归；浏览器为 agent-browser 驱动的 Chromium **真浏览器，非物理真机**（admin 看板视口 1440×900）。
> 域名一律用 <YOUR_TEST_DOMAIN> 占位；真值只从 laiyipao/deploy/.local.keys 读取，本报告与截图不含真域名、真 IP、口令、令牌、私钥。
> 本轮部署为 admin 看板反应伤害贡献分析（双视图 + AI 洞察 + 诚实标注）+ 后端聚合接口重建；H5 无改动沿用 R17 产物，QA 库 migration 00031 已应用。

## 1. 部署产物

| 端 | 产物 | sha256 | 说明 |
| --- | --- | --- | --- |
| admin | index-DF0m5Tvz.js | f13b3465d6b6c31687ec5258db873b3d8a39aed3dbfa3b1e22bb9d9814b31f6d | 本轮重建入口 |
| admin | DashboardView-rO-zrjXQ.js | 781a095cecb071646ad4b479b261e99c83300e3ccd0fc189571788700e1c79fb | 本轮重建，含 r18-damage-impact（count=1） |
| H5 | index-CIZj2kWc.js | eadd221a0a6afab59ab174d70c68b6d44cb727a87cbf5bbfdc15f60e64143c85 | 无改动，沿用 R17（含 r17-reaction-diff） |
| 后端 | lyp-server-amd64 | 6ecfd7c2902cd02918162c7f796944b9727c4c3fc2fa830e6c8b33ad4cc834b6 | 本轮重建，含 R18 /admin/reaction-damage-impact 聚合（ELF） |

服务复验：/healthz=200、/readyz=200、admin(5175)=200、H5(5174)=200。
QA 库（lyp-qa-pg 回环 15440）migration 00031 reaction_timeline jsonb 列已应用：battle 378 / 341 含 schema_version=reaction-timeline-v1（69 / 71 事件），battle 339 为旧战报（空）。

## 2. 集成验证（部署前）

- R18 契约守卫 9/9、admin 渲染用例 10/10 全绿；后端聚合纯逻辑测试 ok（t152-t156 复查记录）。
- 真值守卫 `go test ./internal/domain -run 'TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo|TestNoJWTLiteralInRepo' -count=1` → ok，exit 0。
- 前三批提交逐批 STAGED_LEAK_TOTAL=0（github.com 为 Go import 良性误报、127.0.0.1 为回环）；R18 审计目录 data/（含 qa-before.dump）、auth-init.js、mint-probe.sh、probe.js 已被 .gitignore 拦截，probe.tpl.js 用 __R18_TOKEN__ 占位。

## 3. 端到端 HTTP 抽样

| 项 | 结果 |
| --- | --- |
| /healthz / /readyz | 200 / 200 |
| admin 数据看板首页 / H5 首页 | 200 / 200 |
| GET /api/v1/admin/reaction-damage-impact（带令牌） | 200，3 行聚合 + window_from/to + excluded_old_battles=101 |
| 鉴权接口（未带令牌） | 401（预期） |
| SQL 独立核验（verify-agg.sql） | steam_burst 12175/53、overheat 7488/41、superconduct 6746/46、EXCLUDED_WINDOW=101，与 API 响应逐字段一致 |
| HEAD_TRUTH_LEAK | 0 |

## 4. R18 核心：admin 看板反应伤害贡献双视图

真浏览器 DOM 探针（session r18f2，dashboard 非登录页 loginPage=false）：

| 判据 | 实测 |
| --- | --- |
| 容器 .r18-damage-impact（role=region） | r18=true |
| 默认视图 | countPressed=true、damagePressed=false、rows=[]（原次数视图不回退） |
| 切到 .r18-view-damage | damagePressed=true、countPressed=false、rows=3 行真实数据 |
| 数据行 | 蒸汽爆发 46.1%/12175/306、过热 28.4%/7488/507、超导 25.5%/6746/306 |
| 统计窗口 | 逐字「统计窗口：近 30 天（2026-10-25 至 2026-11-24）」 |
| 旧战报排除 | 逐字「已排除 101 条无逐次记录的旧战报（不计入伤害）」 |
| 诚实红线 | 逐字「伤害贡献为辅助分析；触发次数以物化视图为准，不参与结算与 replay_hash」 |
| AI 洞察四判据 | 「AI · 反应伤害洞察 伤害贡献最高的是 蒸汽爆发（46.1%）… 依据：近 30 天 reaction_timeline.events 的 damage 聚合；触发次数以 dashboard_reaction_counts 为准 前往战报列表验真 →」，汉字 AI + 真实数据结论 + 依据行 + 可执行 CTA 齐备 |
| 探针 | impactResponses[0].status=200；pageErrors=0、consoleErrors=0、apiFailures=0、failedRequests=0 |

权威次数取自物化视图 dashboard_reaction_counts（count=306/507/306），与 events（53/41/46）分离，未冒充；伤害占比为辅助分析，不改结算、score、loot、星级与 replay_hash。

## 5. 真浏览器验证（agent-browser，非物理真机）

- 1440×900 真浏览器打开 `#/admin/dashboard`（非登录页），默认次数视图正常；点击「伤害贡献」后双视图切换成立，3 行真实数据 + AI 洞察 + 诚实红线 + 排除标注全部渲染。
- 接口响应体与 SQL 独立核验逐字段一致，证明伤害来自近 30 天 reaction_timeline.events、次数来自权威视图、旧战报诚实排除、空数据不产 NaN。
- 探针全 0，页面无真实域名/IP/口令/令牌；顶栏 API 标签经 maskApiBase 脱敏为回环地址。

## 6. 截图清单（决定性 2 张，1258×588 非物理真机）

| sha256 前16 | 文件 | 内容（亲读） |
| --- | --- | --- |
| c20354846ec70cdecda66f41d1e190cdef3d55ad53654b24d5812a4e36f16581 | r18-01-damage-top.png | 伤害贡献视图 3 行真实数据 + AI 洞察 + 排除 101 条旧战报 + 诚实红线，「伤害贡献」高亮 |
| 7e38ff48ca6228303d2c2e71427ef0636a82e720fde0f7dedc8af7b399a988b3 | r18-03-count-view.png | 同卡切回次数视图，「次数」高亮，证明原视图保留不回退 |

截图位于 docs/laiyipao/audits/round18-reaction-damage-impact/screenshots/（批3已提交）。本次线上回归重拍 r18-01 与提交版字节一致；滚动辅助图 r18-02-damage-scroll.png 与 r18-01 同 sha，按 t156 终审结论不列为决定性证据，排除不提交。一次性诊断脚本与含真 token 的 data/ 均排除不提交。

## 7. 结论

R18 admin 看板反应伤害贡献分析与后端聚合接口三端部署及线上回归全绿：产物已重建进包，双视图切换 + 3 行真实伤害贡献数据 + AI 四判据 + 诚实红线 + 旧战报排除 101 条 + 权威次数分离全部经真浏览器 DOM 决定性证据验证，接口响应与 SQL 独立核验逐字段一致，探针全 0、截图各异且亲读非登录页、零真值泄露。R18 闭环完成。
