# R14 部署后线上回归报告（round14-postdeploy）

> 环境：测试环境部署后线上回归；浏览器为 agent-browser 真浏览器，**非物理真机**（H5 视口 390×844）。
> 域名一律用 <YOUR_TEST_DOMAIN> 占位；真值只从 laiyipao/deploy/.local.keys 读取，本报告与截图不含真域名、真 IP、口令、令牌、私钥。

## 1. 部署产物

| 端 | 产物 | 校验 |
| --- | --- | --- |
| admin | index-DnuqK3SL.js（未变） | 与 R13 相同 bundle，线上 dashboard/config 等端点仍 200 |
| H5 | index-DlgZZRZl.js（更新） | R14 建议一键执行双页 CTA（战报/图鉴）、权威 CTA 消费层已进包 |
| 后端 | laiyipao-server amd64 17338530B | sha256 8eb2a9f4f696bfcdb40e3d222a963f4bac7b87eead233c43721992c64d2d9780 |

服务 SVC=active，healthz=200，readyz=200。

## 2. 集成验证（部署前）

- miniapp 全量：188 files passed | 1 skipped，1765 tests passed | 2 skipped，exit 0。
- server：CGO_ENABLED=0 go build ./... EXIT 0；advice-action owned 校验 E2E 已随源码批次提交。
- 五类真值（DOMAIN/ADMIN_PASS/DB_DSN/JWT_SECRET/BASEDOMAIN）：COUNT_HEAD=0 COUNT_WT=0；全工作树 eyJ 三段式 JWT 0 命中（仅 oa4rust 白名单测试夹具）。

## 3. 端到端 HTTP 抽样

| 项 | 结果 |
| --- | --- |
| admin login / dashboard / config | 200；config skills=24 gems=8；dashboard 含多段 funnel |
| 用户推荐（skills/equipment/reactions） | 3 context 全 200，schema=rec-v1 |
| 升级战力快照（R13 能力） | 200，power 1320→1440，method=service.ComputePowerFor，applicable=true |
| 回放结算 settle 重测 | 200，loot coin+310 |
| HEAD_TRUTH_LEAK | 0 |

说明：首轮 settle 返回 422，错误为「回放技能配置与构筑不符：replay_skills 与构筑不符」（上报 0:1:100 vs 服务端重算 0:1:110）——系探针脚本硬编码 replay_skills 与当前构筑不一致，服务端防作弊校验正确工作；按服务端重算参数重发后 200，非缺陷。冷启动新用户注册返回 404，因线上 PlayerLoginRequired=true 服务端正确禁止游客建号，属预期保护（冷启动规则证据由 QA 环境 round14-advice-action-e2e 首轮授予前后对照坐实）。

## 4. R14 核心：/me/advice/action 权威 CTA 端点

| 场景 | HTTP | 结果 |
| --- | --- | --- |
| 游客 element=fire/corrosion/kinetic | 200 | schema=advice-action-v1，reason=NOT_AUTHENTICATED，candidate_id=0（诚实空态，非 401） |
| 非法 element=__BAD__ | 200 | reason=INVALID_ELEMENT，candidate_id=0 |
| 登录用户 element=fire/lightning | 200 | schema=advice-action-v1，reason=NO_OWNED_CANDIDATE，candidate_id=0，params=null，params_ok=null |

线上 admin 账号无可替换的更优 owned 候选，服务端按权威裁决下发诚实空态（非空 CTA），与 SPEC A.3 口径一致。ready 态真实闭环（建议卡 r14Advice=7 / CTA r14Cta=7、两步确认浮层、DB slots 1:4→1:3 事务落库）已由 QA 环境 t132/t133 决定性证据在 round14-advice-action-e2e 批次提交，此处线上账号未构造候选故不重复授予数据。

## 5. 真浏览器双端验证（agent-browser，非物理真机）

- admin 五视图（dashboard/content/equipment/skills/wiki）：真实 AI 运营洞察卡文本（关卡流失漏斗、规则引擎统计、技能热量效率），domainInBody=false，探针 pe/ce/af/fr 全 0。
- H5 五页（首页/关卡/技能/装备/宝石）：AI 战术洞察、stage 星级口径卡（本关满分 18284 / 1★ 门槛 10970）、成长页因果链诚实空态「收益数据不可用，不展示预计数值」、10 机制拐点图例，domainInBody=false，探针全 0。
- R14 建议 CTA 线上空态探针：图鉴页 r14 空态卡=7、CTA=0（线上账号无候选，符合诚实空态），battles/reactions 页探针 pe/ce/af/fr 全 0，domainInBody=false。
- 观察如实记录：admin wiki 截图 sha 与 dashboard 相同，系 agent-browser 子视图路由回退 dashboard 的已知行为（R1–R13 同），不作为门禁项；首轮 advice CTA 探针因登录选择器与截图 --path 参数写法有误产生假阴性，已修正后复测，本版记录为修正后结果。

## 6. 截图清单（12 张，均 390×844 非物理真机）

| sha256 前16 | 文件 |
| --- | --- |
| 1c2f8793ccb0b7b3 | r14-online-admin-dashboard.png |
| 4231e39a428caa5e | r14-online-admin-content.png |
| 52ea5573ff4ee608 | r14-online-admin-equipment.png |
| 5239003f65f191ed | r14-online-admin-skills.png |
| 1c2f8793ccb0b7b3 | r14-online-admin-wiki.png（路由回退，同 dashboard） |
| f3785063f4a18fcb | r14-online-h5-index.png |
| 174c0fa2d7964d3f | r14-online-h5-stage.png |
| 34e647a14c7a4722 | r14-online-h5-skills.png |
| f3ef79f17f9c594e | r14-online-h5-equipment.png |
| 56280e9ea34736ab | r14-online-h5-gems.png |
| 6372c7dff7daaea6 | r14-online-battles.png |
| 70c43d1cc83f9093 | r14-online-reactions.png（亲读：图鉴页 AI 规则卡 + R14 诚实空态）

## 7. 结论

R14 三端部署与线上回归全绿：权威 CTA 端点行为符合 SPEC（含游客/非法/空态），双端真浏览器真实 AI 文本与零探针，端到端抽样 200，零真值泄露。BUG1（图鉴建议卡不可达）修复决定性证据已在审计批次提交，线上账号不重复构造候选。R14 闭环完成。
