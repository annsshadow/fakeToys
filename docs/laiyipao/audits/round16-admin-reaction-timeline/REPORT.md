# R16 复查报告：admin 反应时间线真浏览器实测 + 契约 + 诚实红线

- 任务：t143（attempt_id=c49a906c-b690-43b5-a042-15af0e8a9873）
- 复查对象：t141 规范 + t142 实施（laiyipao/admin/src/views/BattlesView.vue + R16 测试）
- 复查人：qa-e2e-wb（独立复查，不采信实施者自述；不做 git）
- 判定：**PASS（建议）** —— 四项验收全部满足，无阻断缺陷。

## 一、验收①：契约测试 RED→GREEN 与断言齐全（PASS）

### 亲读文件
- `docs/laiyipao/specs/round16-admin-reaction-timeline/SPEC.md`（72 行，t141 产出），钉死 reaction-timeline-v1 消费契约：events 五字段（wave/tick/reaction/enemy_uid/damage）、200 上限截断、schema_version、诚实红线、旧战报逐字空态。
- `laiyipao/admin/tests/R16ReactionTimeline.contract.test.ts`（82 行，5 用例）：主干 schema/五字段/200/truncated；选择器 .r16-* / [data-r16-wave] / labelOfReaction；排序/空态/截断/诚实红线逐字；AI 四判据；三端不回退。
- `laiyipao/admin/tests/R16ReactionTimeline.render.test.ts`（209 行，9 用例）：wave/tick 升序 + 服务端取名 + fmt 格式化 + AI 四要素；非法事件丢弃；未知 key 兜底；截断 true/false；4 种空态逐字；诚实红线不复用 r15。

### 实跑验证
| 命令 | 结果 |
| --- | --- |
| `npx vitest run --config vitest.config.ts tests/R16ReactionTimeline.contract.test.ts` | 5/5 passed |
| `npx vitest run --config vitest.config.ts tests/R16ReactionTimeline.render.test.ts` | 9/9 passed |
| `npx vue-tsc --noEmit` | exit 0 |

### RED 依据（非自述）
- `git show HEAD:laiyipao/admin/src/views/BattlesView.vue | grep -c 'r16-'` → **0**（HEAD 无实现）
- `git show HEAD:laiyipao/admin/tests/R16ReactionTimeline.render.test.ts` → **fatal: path ... not in HEAD**（HEAD 无测试）
- 两测试文件 `git status` 为 `??` 未入库，符合“不做 git”纪律。

## 二、验收②：源码取名/数值链路（PASS）

- 源码 `laiyipao/admin/src/views/BattlesView.vue`（+91 行未提交）：
  - `:326` `fmtTimelineDamage`：整数 toFixed(0) / 非整数 toFixed(1) / 非有限破折号；
  - `:331` `reactionTimeline` computed：schema 校验 + 严格事件校验 + wave/tick 升序 + peakWave + total=合法事件数；
  - `:663` / `:671-689` 模板 AI 四要素 + .r16-* DOM，行格式『第 {wave} 波 · {labelOfReaction} · #{enemyUid} · {damageText}伤害』。
- 反应名走 `@/reactions` 的 `labelOfReaction`（`laiyipao/admin/src/reactions.ts:62-64`，服务端下发），**非硬编码映射、不拿聚合 map 反推**；数值走 `fmtTimelineDamage`。

## 三、验收③：真浏览器实测（PASS）

> 浏览器为 agent-browser 驱动的 Chromium（**非物理真机**），如实标注；每一步 DOM 探针与截图均来自真实运行页面，非静态构造。

### 服务环境
- QA 容器 `lyp-qa-pg`（回环 15440）+ migration 00031 `reaction_timeline` 列存在。
- 后端 API：127.0.0.1:8080（start-api.sh，APP_ENV=local，BOOTSTRAP_ADMIN_PASS 为 config.go 公开默认值）；admin：127.0.0.1:5175（start-admin.sh，vite.config.ts:45 port5175）。
- admin 登录 POST /api/v1/admin/login 成功；token 存 localStorage `lyp_admin_token`（client.ts:21）。
- QA 快照：`data/qa-before.dump`，sha256=`57bd97d0ccdeaa1225146c947e4e8d5c9385868a988d1bc898c10f82fe550eae`。

### 有数据态：battle_id=378（69 条真实 timeline）
点『详情』后 DOM 探针：
- url=/admin/battles（非登录页）、loginPage=false、drawerOpen=true、box=true、boxRole=status
- title=`AI · 反应时间线`、summary=`本局记录 69 次逐次反应，集中在第 5 波`
- basis=`依据：reaction_timeline.events（wave/tick/reaction/enemy_uid/damage），展示性归因、非权威结算`、cta=`发起验真 →`、empty=''
- rowsCount=69、waves 首 1 末 5 升序、firstRow=`第 1 波 · 超导 · #1 · 70伤害`、lastRow=`第 5 波 · 过热 · #131 · 415伤害`
- hasUndefined=false、hasNaN=false、hasR15=false、pe/ce/af/fr 全 0、bodyHasDomain=false

### 旧战报空态：battle_id=339（DB reaction_timeline='[]'，schema 缺失）
点『详情』后：empty 逐字=`该战报无逐次反应记录（旧版本结算）`、rowsCount=0、summary=''、cta=''、hasUndefined/NaN=false、pe/ce/af/fr 全 0。
（id41 不在首页 50 条，用同样 schema 缺失+空 events 的 id339 验证，属合法旧战报。）

### 截图（全部亲读，确认非登录页与真实渲染）
目录 `docs/laiyipao/audits/round16-admin-reaction-timeline/screenshots/`，5 张 1440x900 PNG，sha256 两两不同：
| 文件 | sha256 | 内容（亲读） |
| --- | --- | --- |
| r16-01-battles-list.png | 349ee46bc47dcfcf5939cb770a94a4bf9147a7ac5147de58c1222b975888b2a1 | 战报列表，378/341/339 行可见 |
| r16-02-timeline-top.png | de8b36c8147e5f169ffd6607841d802d4b300f85da6222e844737f73e55b1198 | 378 抽屉顶部，AI·反应时间线 69 次/第 5 波 + 前 10 行真实数据 |
| r16-03-timeline-scroll.png | aeb9a08c1cd1611b74c5ebb268a40cf5dc2b9cd668b474420fcd6c98d51b5f79 | 抽屉滚动后，第 1-2 波逐行渲染 |
| r16-05-old-empty-339.png | 9d973a4a4d9b80a2cb33ba63e15c42c951bf3c7c6cc644434fd2648e67af39ce | 339 旧战报逐字空态 |
| r16-06-timeline-bottom.png | aa29cdcb58324e0a3a52d1e97025bf995cfe5f7485d14fe616192c6f38869ebb | 时间线尾部，第 5 波末行『过热 · #131 · 415伤害』 |

说明：早期误导性截图 r16-04-old-empty.png（实为 378 重复）与哈希撞车的旧 03 已删除/重截。

## 四、验收④：零真值子串扫描（PASS）

- 扫描范围：整个审计目录（排除 dump 二进制与截图），模式：JWT（eyJ…）、Bearer token、私钥头、真实域名（.com/.cn/.net/.org）、个人路径（C:\Users\...）。
- 结果：**0 命中**（仅输出 SCAN_DONE）。
- 审计目录内脚本使用占位符与回环地址（127.0.0.1:8080/5175/15440），管理员口令为 config.go 公开默认值，非真值。

## 五、QA 数据还原（PASS）

- E2E 开始前已 pg_dump 快照；完成后执行 `pg_restore --clean --if-exists < qa-before.dump`。
- 还原后复核：battle_records 103 条、max(id)=378，与快照一致；id339 timeline='[]'（array）、id378 timeline 为 object（69 事件包装体），数据结构完整。

## 六、遗留说明

- 本轮未发现阻断缺陷；r16 时间线展示性归因、不参与 replay_hash、不影响 score/loot/星级、非权威结算，符合 SPEC 诚实红线。
- 截图与探针、契约源码均已亲读核验，非采信实施者自述。
- verdict 建议：**pass**。
