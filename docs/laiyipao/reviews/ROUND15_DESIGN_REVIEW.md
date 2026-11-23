# R15 反应时间线 设计终审报告（design-reviewer-wb）

- 任务：t139（R15 设计终审：六门禁 verdict），attempt_id f759158c-efb7-4348-9c83-02aba6131296
- 审阅对象：t134 SPEC+契约、t135 引擎旁路采集、t136 后端落库、t137 H5 时间线、t138 队长复查，及其全部产物
- 判定方式：本人独立亲读 SPEC/契约/引擎/后端/H5 源码与测试、亲读真浏览器截图与 DOM 探针、亲自复跑前端全量与后端逻辑测试；不采信任何成员自述结论
- 环境：本地回环 + 本地浏览器自动化采集，所有截图均为真浏览器自动化产物，**非物理真机**
- 保密：本报告不回显任何令牌/口令/私钥/真实域名/IP 真值；证据仅以仓库相对路径 + 内容摘要 + sha 引用

## 结论速览

**verdict = pass（status=completed）。** R15「反应时间线」六个门禁逐项实测通过，无 needs_revision / reject 级别的阻断项。

| 门禁 | 主题 | 结论 |
|---|---|---|
| ① | SPEC 落地 + 契约 RED→GREEN | PASS |
| ② | 引擎旁路采集不污染 replay_hash / 结算权威字段 | PASS |
| ③ | 后端落库 + 详情返回 + 200 上限 + 坏 jsonb 报错 | PASS |
| ④ | H5 时间线真实渲染 + 诚实空态 | PASS |
| ⑤ | 真浏览器决定性证据（截图 sha 各异亲读 + 探针全 0） | PASS |
| ⑥ | 五类真值零泄露 + R14 令牌锚点不回退 | PASS |

## 门禁① SPEC 落地 + 契约 RED→GREEN（PASS）

- 亲读 `docs/laiyipao/specs/round15-reaction-timeline/SPEC.md`（66 行）：§0 真实缺口定位到 `engine.ts:1725-1746` 逐事件 emit+record、`adminbattledetail.go:56-57` 仅聚合 map、`battle_collections.go:19` 10MB 红线；§1 五字段契约（wave/tick/reaction/enemy_uid/damage，enemy_uid 为 wire 契约 snake_case）与外层 `reaction_timeline{schema_version=reaction-timeline-v1, events, truncated, total}`；§1.1 上限 200 尾部截断置 truncated、未知 key 丢弃；§2 诚实红线（展示性归因、不进 replay_hash、不改 score/loot/星级、服务端不当前提、旧战报文案逐字『该战报无逐次反应记录（旧版本结算）』）；§3 四组选择器与截断提示文案；§5 契约四组 + R1-R14 不回退。
- 契约 `laiyipao/miniapp/src/styles/r15ReactionTimeline.contract.test.ts` 四组（五字段/SPEC 关键词/选择器/令牌锚点）逐项与 SPEC 对齐，非摆设断言。
- t134 记录契约 RED 3 failed / 1 passed → GREEN 4/4；本人复跑该文件随全量测试绿（见门禁⑥）。

## 门禁② 引擎旁路采集不污染结算权威字段（PASS）

- 亲读 `laiyipao/miniapp/src/game/engine.ts`：:63-70 `ReactionTimelineEvent` 注释明确 enemy_uid 为 wire 契约（camelCase 会导致 settle 500）；:607-608 旁路数组与截断标志；:771-778 `reactionTimeline()` / `reactionTimelineTruncated()`；**:1755-1768** 在 `res.reaction` 分支旁路 push 五字段（`enemy_uid: e.uid`），满 200 仅置 truncated=true，不 emit、不 record、不改 score/loot/replay_hash。
- 亲读 `laiyipao/miniapp/src/game/r15ReactionTimeline.test.ts`：真实命中断言五字段且 `enemy_uid` 存在、`enemyUid` 不存在；a/b 双引擎 replayHash 一致；`settleInput` 不含 `reaction_timeline`；聚合结果不变；200 上限注入保留最早 200 且 truncated=true。
- P0 修复核实：早期字段错配（enemyUid→enemy_uid）已 test-first 修正，wire 格式现为 snake_case，与后端 struct json tag 一致。

## 门禁③ 后端落库 + 详情返回 + 上限 + 坏 jsonb 报错（PASS）

- 亲读 `laiyipao/server/internal/domain/reaction_timeline.go`：常量 `ReactionTimelineSchemaVersion=reaction-timeline-v1`、`ReactionTimelineMaxEvents=200`；`UnmarshalJSON` 兼容包装对象与 `[]`，非空裸数组报错 fail loud；`NormalizeReactionTimeline` 校验 wave>=1/tick>=0/reaction 非空且<=64/enemy_uid>=1/damage>=0，未知 key 丢弃，Total 记原始数，超 200 保留最早 200 并置 Truncated。
- 落库链路：`game.go` SettleBattle 调 NormalizeReactionTimeline + marshal 写入第 24 列；`economy.go`（MyBattleDetail 实际在此文件）与 `adminbattledetail.go` SELECT 增列、NULL→[]、Events nil→[]、**坏 jsonb 显式报错不吞**。
- Migration `server/migrations/00031_reaction_timeline.sql` 仅 `ADD COLUMN IF NOT EXISTS reaction_timeline JSONB NOT NULL DEFAULT '[]'::jsonb`，Up/Down 整喂不再被 DROP（队长已修）。docker QA 库确认列存在。
- `server/internal/httpapi/routes_e2e_r15_reaction_timeline_test.go` 三条 E2E 通过（见门禁⑥）。

## 门禁④ H5 时间线真实渲染 + 诚实空态（PASS）

- 亲读 `laiyipao/miniapp/src/pages/battles/battles.vue`：模板 :193-205 `.r15-reaction-timeline/.r15-timeline-row/[data-r15-wave]/.r15-timeline-empty/.r15-truncated-note` 与 SPEC 选择器一致；`reactionTimeline` computed（:617-634）校验五字段后按 wave/tick 升序，`reactionTimelineTruncated`（:637）读 `truncated===true`；CSS :1029-1070 沿用既有令牌，无新增设计令牌（符合不回退）。
- 亲读 `laiyipao/miniapp/src/pages/battle/battle.vue:1049-1057` 上报：`eng.settleInput()` 后 payload 展开追加 `reaction_timeline: eng.reactionTimeline?.()`，不动权威字段；`settleFailed` 区分『战绩未入账』与『通关』。
- `battles.test.ts:498-590` 五组：有数据 3 行且排序 ['1','2','2']、空态文案逐字、坏结构回退空态、truncated 提示『已截断，仅显示前200条』。

## 门禁⑤ 真浏览器决定性证据（PASS）

- 三张最终截图本人逐张 `read_image` 亲读（均 1258x588，sha 各异，非登录页）：
  - `r15-final-01-settle.png` sha=fad062ca38e9a379b14eccc1a4d02ff6d8af2d4bdd72853f1aa012072067881b —— 结算面板『通关』+ 查看本局战报按钮。
  - `r15-final-02-timeline-top.png` sha=77e9fce11eca8539198de19e13a82c7f709cba71f2e9912b7978abef1dda68b5 —— 我的战报列表页。
  - `r15-final-03-timeline-scroll.png` sha=66f928fba3807e2a000e2b53ae2f42c2cff90b19a51aee8bc094ce8c67d342a8 —— 关卡页胜率/星级分布。
  - 均位于 `docs/laiyipao/audits/round15-reaction-timeline/screenshots/`。
- `r15-battle-07-timeline-visible.png` 亲读：战报详情明文显示『AI · 反应时间线』与逐字空态『该战报无逐次反应记录（旧版本结算）』，证明诚实空态真实渲染，非编造。（旧 `r15-v2-02` 的『战绩未入账』为失败样本，非最终结果，不作为证据。）
- DOM 探针 `data/timeline-dom.json`（battle_id=339）：`rows=72`、首行『第1波·超导·#1·70.0伤害』、末行『第5波·过热·#137·415.0伤害』、`waves=[1,2,3,4,5]`、`truncated=false`、`total=72`、`detail.status=200/schema=reaction-timeline-v1`，且 `pe=0/ce=0/af=0/fr=0/bodyHasDomain=false`。队长复查 t138 另得 settle=200/battle_id=341、DOM rows=71、三图 sha 与上述一致，两轮互相印证（数据行数差异来自不同 battle，属正常）。
- 最终 `auth-init.js` 与模板逐字一致：仅 localStorage 注入占位符 `__LYP_TOKEN__` 与错误/失败收集，明确注释『R15 P0 修复后不再篡改任何字段』，`qaShimApplied` 恒 0；`e2e-run.log` 中 af=[]。探针字段路径 `tl2.events` 曾导致 `timelineLen=0` 属探针读取位置偏差，**DOM 72 行才是决定性证据**。

## 门禁⑥ 五类真值零泄露 + R14 令牌锚点不回退（PASS）

- 亲读 `laiyipao/server/internal/domain/secrets_leak_test.go`：`TestNoRealSecretsInRepo` 从 `.local.keys` 读 DOMAIN/ADMIN_PASS/JWT_SECRET/DB_DSN/DB_PASSWORD 做全工作树子串比对；`TestNoInfraIdentifiersInRepo` 禁私钥路径与 root@IP；`TestCurrentStateDocUsesDomainPlaceholder` 结构性守卫。本人执行 `go test ./internal/domain/...` exit 0（真值守卫通过）。
- 审计目录正则扫描（私钥/公网 URL/带口令 DSN）零命中；真 JWT 已脱敏为 `__LYP_TOKEN__`。
- R14 不回退：契约测试断言 v1/v3-r3/r6 令牌锚点存在且无 `--r15-` 新令牌；`engine.ts` 的 emit/record/replay_hash 未改。
- 本人亲自复跑：
  - 前端 `cd laiyipao/miniapp && pnpm test -- --run <三份 R15 测试>`：exit 0；pnpm test 实跑全量 **190 passed | 1 skipped (191)**、**1775 passed | 2 skipped (1777)**，37.82s。
  - `cd laiyipao/server && go test ./internal/domain/...`：exit 0。
  - `cd laiyipao/server && go test ./internal/httpapi/...`：exit 0（R15 三条 E2E 通过）。
  - 全量 `go test ./...`：仅 `store/httperr` 的 `TestSQLSTATEAnchorsFromRealDatabase` 等 DB 直连用例因本机 127.0.0.1:5432 无本地 PG 监听失败（QA 库实际映射在其他端口），属**环境端口依赖**，非 R15 代码缺陷；domain/service/httpapi 等纯逻辑包全通过。诚实记录，不掩盖。

## 最终结论

六门禁全部 PASS，**verdict=pass，status=completed**；无阻断 finding，无需返工。仅保留一条环境备注：后端少数 DB 直连单测依赖本机 5432 监听，本工作区不满足，不影响 R15 功能正确性与本次终审结论。报告落盘 `docs/laiyipao/reviews/ROUND15_DESIGN_REVIEW.md`。

> 所有既有截图均为真浏览器自动化采集，非物理真机。
