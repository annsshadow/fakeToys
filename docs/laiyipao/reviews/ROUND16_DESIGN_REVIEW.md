# R16 战报详情逐次反应时间线 设计终审报告（design-reviewer-wb）

- 任务：t144（R16 终审：设计门禁六项 + 真实缺陷裁决），attempt_id f4ed19d9-dd1f-473e-8d82-324de9f85261
- 审阅对象：t141 规范（SPEC.md + 契约测试）、t142 实施（BattlesView.vue + 渲染测试）、t143 复查（真浏览器 E2E + 探针 + 截图）及其全部产物
- 判定方式：本人独立亲读 SPEC / 契约测试 / 渲染测试 / 实施源码 / 后端与 H5 消费代码、亲读决定性截图并用 sha256sum 亲手复算、亲自复跑两组测试；**不采信任何成员自述结论**
- 环境：本地回环 + 浏览器自动化采集（Chromium + agent-browser），截图均为真浏览器运行产物，**非物理真机**
- 保密：本报告不回显任何令牌 / 口令 / 私钥 / 真实域名 / 真实 IP 与个人路径；证据仅以仓库相对路径 + 内容摘要 + sha256 引用

## 结论速览

**verdict = pass（status=completed）。** R16 六个门禁逐项独立实测通过，无 needs_revision / reject 级别阻断项。未发现新的真实缺陷。

| 门禁 | 主题 | 结论 |
|---|---|---|
| ① | 规范符合度（AI 四判据 / 选择器 / 诚实红线） | PASS |
| ② | 三端一致性（H5 r15 与 admin r16 语义对齐、后端字段不变） | PASS |
| ③ | 真 E2E 决定性证据（sha 各异 + 亲读非登录页 + DOM 探针） | PASS |
| ④ | 零真值 | PASS |
| ⑤ | R1–R15 不回退（令牌锚点 / 25% AoE / 顶栏脱敏 / 两步确认） | PASS |
| ⑥ | 新真实缺陷裁决 | PASS（未发现阻断缺陷） |

## 门禁① 规范符合度（PASS）

- 亲读 `docs/laiyipao/specs/round16-admin-reaction-timeline/SPEC.md`（72 行）：§1 固定选择器 `.r16-reaction-timeline`（含 `role=status aria-live=polite`）、`.r16-timeline-row`[`data-r16-wave`]、`.r16-timeline-empty`、`.r16-truncated-note`；行格式逐字；反应名只能走 `labelOfReaction` 服务端取名；事件校验规则；`sort(a.wave-b.wave || a.tick-b.tick)`；schema `reaction-timeline-v1`；§2 AI 四判据；空态逐字『该战报无逐次反应记录（旧版本结算）』、截断逐字『已截断，仅显示前 200 条』；§3 诚实红线（展示性归因、非权威结算、不进 replay_hash、不改 score/loot/星级、不反推编造、零真值）。
- 亲读实施 `laiyipao/admin/src/views/BattlesView.vue`：:314 `TIMELINE_SCHEMA='reaction-timeline-v1'`、:315 逐字空态常量、:326-329 `fmtTimelineDamage`（整数 toFixed(0)/非有限破折号）、:331-366 `reactionTimeline` computed（schema 校验 + 五字段严格校验 + 非法丢弃 + wave/tick 升序 + peakWave/total）、:671-689 模板四判据与 .r16-* DOM、:736-744 scoped CSS 复用令牌无新增视觉变量。
  - AI 四判据逐项落地：:673 可见汉字『AI · 反应时间线』；:674-675 真实数据结论『本局记录 {{total}} 次逐次反应，集中在第 {{peakWave}} 波』；:678 依据行『依据：reaction_timeline.events（wave/tick/reaction/enemy_uid/damage），展示性归因、非权威结算』；:679 CTA『发起验真 →』。
  - :687 行用 `labelOfReaction(reactionLabel, row.reaction)` 服务端取名，无硬编码映射；:689 截断提示逐字。
- 独立复跑（不采信自述）：`vitest run tests/R16ReactionTimeline.contract.test.ts` → **5/5 passed**（exit 0）；`vitest run tests/R16ReactionTimeline.render.test.ts` → **9/9 passed**（exit 0，仅 memoryRouter 未注册路由的良性 warn，不影响断言）。渲染测试真实 mount BattlesView + 打开详情抽屉，覆盖升序/取名/格式化/非法事件丢弃/未知 key 兜底/截断 true-false/四种空态/诚实红线，断言非空摆设。

## 门禁② 三端一致性（PASS）

- 后端 `laiyipao/server/internal/domain/reaction_timeline.go` 亲读：:17 `ReactionTimelineSchemaVersion = "reaction-timeline-v1"`、:19 `ReactionTimelineMaxEvents = 200`、:29 `EnemyUID int ` + `json:"enemy_uid"`（snake_case）、:36 `schema_version`、:43 兼容包装对象、:79-92 超 200 保留最早 200 + truncated/total。**R16 未改后端字段与 schema。**
- H5 `laiyipao/miniapp/src/pages/battles/battles.vue` 亲读：:192 `.r15-reaction-timeline`、:201 行格式含 `#{{ ev.enemy_uid }}`、:205 逐字空态、:624 `Number.isInteger(ev.enemy_uid)`、:631 `enemy_uid: ev.enemy_uid`。H5 r15 与 admin r16 消费同一 `reaction-timeline-v1` 与五字段语义，enemy_uid 两端均 snake_case。
- admin r16 消费同一 schema（:314/:337）且五字段校验与 H5 对齐（wave 正整数/tick 非负整数/reaction 非空串/enemy_uid 整数/damage 有限数）；契约测试⑤断言三端不回退文本守卫仍在。三端一致、无字段漂移。

## 门禁③ 真 E2E 决定性证据（PASS）

> 浏览器为 agent-browser 驱动 Chromium（非物理真机），如实标注；DOM 探针与截图来自真实运行页面。

- **sha 亲手复算**（本人 `sha256sum`，5 图两两不同，推翻队长 t143 输出中『r16-02 与 r16-03 sha 相同(de8b36c8)』的观察——实测 r16-03 = `aeb9a08c`，与 r16-02 不同，该观察不成立；以本报告亲算为准）：
  - r16-01-battles-list.png `349ee46bc47dcfcf5939cb770a94a4bf9147a7ac5147de58c1222b975888b2a1`
  - r16-02-timeline-top.png `de8b36c8147e5f169ffd6607841d802d4b300f85da6222e844737f73e55b1198`
  - r16-03-timeline-scroll.png `aeb9a08c1cd1611b74c5ebb268a40cf5dc2b9cd668b474420fcd6c98d51b5f79`
  - r16-05-old-empty-339.png `9d973a4a4d9b80a2cb33ba63e15c42c951bf3c7c6cc644434fd2648e67af39ce`
  - r16-06-timeline-bottom.png `aa29cdcb58324e0a3a52d1e97025bf995cfe5f7485d14fe616192c6f38869ebb`
- **亲读 r16-02**（1440×900，sha de8b36c8）：非登录页，战报详情抽屉 battle_id=378，显示『AI · 反应时间线　本局记录 69 次逐次反应，集中在第 5 波』+ 逐字依据行 + CTA『发起验真 →』+ 真实逐行数据『第 1 波 · 超导 · #1 · 70伤害 / 第 1 波 · 蒸汽爆发 · #1 · 167伤害 / 第 1 波 · 过热 · #2 · 79伤害』，AI 四判据齐全、五字段真实、wave/tick 升序。
- **亲读 r16-05**（1440×900，sha 9d973a4a）：非登录页，battle_id=339 显示『AI · 反应时间线』+ 依据行 + 逐字空态『该战报无逐次反应记录（旧版本结算）』，诚实空态成立、不编造、无 NaN/undefined。
- DOM 探针（probe.js/init.js 只读、不篡改字段）：有数据态 url=/admin/battles、loginPage=false、drawerOpen=true、boxRole=status、rowsCount=69、waves 首 1 末 5 升序、firstRow/lastRow 与截图一致、hasUndefined/hasNaN/hasR15=false、pageErrors/consoleErrors/apiFailures/failedRequests 全 0、bodyHasDomain=false；空态 battle_id=339 逐字空态 + 0 行 + 探针全 0。
- 服务环境为 QA 容器 + migration 00031 reaction_timeline 列存在，E2E 后已 pg_restore 还原快照（报告 §五）。综上构成决定性证据链。

## 门禁④ 零真值（PASS）

- 审计目录扫描（JWT / Bearer / 私钥头 / 真实域名 / 个人路径）0 命中；脚本仅用回环地址与 config.go 公开默认管理员口令（非真值）。截图顶栏为脱敏回环 API 标签，无真实域名/IP 泄露。
- 含真实 guest_token 的 `qa-before.dump` 已删除：`ops_glob` 复查 round16 data 目录与 qa-before.dump 均不存在；`.gitignore:279-280` 新增该审计目录 data/ 忽略规则生效。真值隐患已清。

## 门禁⑤ R1–R15 不回退（PASS）

- **令牌锚点**：`laiyipao/admin/src/styles/tokens.css`:12-14 亲 grep 确认 `--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6`，全 src 无 `--r16-` 视觉变量；零新增视觉令牌，R16 样式复用既有令牌。
- **25% AoE**：历史 t44/t41 线上已验 pct25=2/radius120=2（`docs/laiyipao/.../js-aoe.js`、REPORT.md）；R16 SPEC §70 明确钉死 AoE 25% 不回退，本轮只在战报详情加展示组件、不触引擎与图例文案；契约⑤含三端不回退守卫。
- **顶栏脱敏**：`laiyipao/admin/src/utils/apiLabel.ts`:23 `maskApiBase` + `AdminLayout.vue`:178 实际调用、:22 import 均在，截图顶栏呈脱敏回环标签，未回退。
- **两步确认**：`ElMessageBox.confirm` 仍在 ContentView / DefensesView / EconomyView / LevelsView / RecomputesView / UsersView 六个破坏性操作视图中使用（亲 grep files_with_matches），R16 未删除或替换既有验真区与 chip。
- SPEC §61/§70 逐字固化上述锚点，契约⑤做文本守卫，实测均成立。

## 门禁⑥ 新真实缺陷裁决（PASS，未发现阻断缺陷）

- 独立亲读源码 + 两组测试 + 决定性截图 + sha 复算 + 不回退锚点，未发现新的真实缺陷；R16 时间线展示性归因、不参与 replay_hash、不影响 score/loot/星级、非权威结算，符合 SPEC 诚实红线。
- 低优先级观察（如实记录，**不判 FAIL**）：
  1. 队长 t143 输出记『r16-02 与 r16-03 sha 相同』，本人 sha256 亲算证伪（r16-02=de8b36c8、r16-03=aeb9a08c 不同），属复查输出笔误，不影响功能判定。
  2. e2e.sh 仍保留点 battle 41 与 r16-04 截图的旧脚本片段，但最终决定性证据已改用 r16-02（有数据 378）+ r16-05（空态 339），旧误导截图已删除；脚本与新证据口径不一致但不影响产物与功能。
  3. 渲染测试有 memoryRouter 未注册 `/admin/battles` 的良性 warn，非断言失败。

## 环境备注（诚实记录，非 R16 缺陷）

- 本次终审未重跑真浏览器 E2E（队长指令基于 t143 已验证的决定性证据收敛），但本人对截图做了 sha 复算与逐图亲读、对 DOM 探针结论做了源码级核验，证据链完整可信。
- 浏览器为本地回环自动化（非物理真机），已在开头与门禁③如实标注。

## 最终结论

六门禁全部 PASS，verdict = pass。R16 战报详情逐次反应时间线在规范符合度、三端一致性、真浏览器证据、零真值、R1–R15 不回退五个维度均达标，未发现需修复的真实缺陷。准许以 status=completed 收敛本轮终审。
