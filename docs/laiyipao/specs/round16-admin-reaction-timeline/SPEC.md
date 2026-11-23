# R16 · admin 战报详情逐次反应时间线（reaction-timeline-v1 消费）契约与设计规范

> 状态：设计拍板 / test-first 契约已钉（本轮只写规范与契约测试，不改业务代码）。
> 依据（2026-11-24 真代码审计）：R15 已打通「后端字段 → 类型 → H5 渲染」，但 admin 运营详情抽屉只展示聚合 chip，缺最后一环。

## 0. 真实缺口与依据（带行号）

1. 后端已返回逐次归因，且明确标注为展示性：`laiyipao/server/internal/service/adminbattledetail.go:62-64` 注释「ReactionTimeline 是 R15 逐次反应归因（仅展示，非权威结算数据）」「旧行/空态为 schema_version=reaction-timeline-v1 + events=[]」，字段 `ReactionTimeline domain.ReactionTimeline `json:"reaction_timeline"``；查询 `:100` 已 SELECT `br.reaction_timeline`。
2. admin 类型已生成：`laiyipao/admin/src/api/types.gen.ts:76` `reaction_timeline: ReactionTimeline`；`:523-528` `ReactionTimeline{schema_version, events, truncated, total}`；`:530-536` `ReactionTimelineEvent{wave, tick, reaction, enemy_uid, damage}`。
3. H5 已消费：`laiyipao/miniapp/src/pages/battles/battles.vue:192-206` 用 `.r15-reaction-timeline` 渲染逐次归因，`:617-637` 做事件校验 + wave/tick 升序 + 反应名映射。
4. **缺口**：admin 运营详情抽屉 `laiyipao/admin/src/views/BattlesView.vue:507-617`（`el-drawer`）只渲染基础描述（`509-548`）、验真区（`558-589`）、元素使用分布（`591-599`）、反应使用分布（`601-609`）、触发地形（`611-615`）共三类聚合 chip；全文件无 `reaction_timeline` 消费，也无 `.r15-` / `.r16-` 时间线选择器。运营在验真/反作弊场景看不到「哪一波/哪个敌人/什么反应/多少伤害」的逐次归因。

本规范补齐这一环：admin 详情抽屉新增反应时间线模块，选择器与 H5 r15 区分但语义对齐。

## 1. admin 展示契约（固定选择器，供 E2E 与契约测试锚定）

在 `BattlesView.vue` 详情抽屉「反应使用分布」之后、「触发地形」之前（建议 `:610` 附近）新增模块：

| 用途 | 选择器 | 语义对齐 H5 | 约束 |
| --- | --- | --- | --- |
| 容器 | `.r16-reaction-timeline` | `.r15-reaction-timeline` | `role="status" aria-live="polite"`；标题含可见汉字「AI」 |
| 逐行 | `.r16-timeline-row` | `.r15-timeline-row` | 每行带 `[data-r16-wave="<wave>"]`，按 wave/tick 升序 |
| 空态 | `.r16-timeline-empty` | `.r15-timeline-empty` | 逐字文案，缺字段/空数组/结构异常都走它 |
| 截断态 | `.r16-truncated-note` | `.r15-truncated-note` | `truncated=true` 时渲染，否则不渲染 |

- **区分**：admin 一律 `.r16-*` / `[data-r16-wave]`，绝不复用或改写 H5 的 `.r15-*` / `[data-r15-wave]`。
- **行格式**：`第 {wave} 波 · {反应中文名} · #{enemy_uid} · {damage}伤害`（与 H5 语义一致；H5 用「 · {damage} 伤害」，admin 按任务书用「 · {damage}伤害」，同一端内必须统一）。
- **反应中文名**：只能用服务端下发的反应配置表，经现有 `@/reactions` 的 `labelOfReaction(reactionLabel, ev.reaction)` 取名（`laiyipao/admin/src/reactions.ts:62-64`，兜底返回原始 key）；**不得硬编码**反应中文名 map。`reactionLabel` 复用组件已有的 `ref<ReactionLabels>`（`BattlesView.vue:31-32`）与 `fetchReactionLabels` 流程。
- **数值格式化**：`damage` 必须经确定性格式化后再渲染，沿用 admin 既有 `toFixed` 惯例（如 `BattlesView.vue:125/284` 的 `toFixed(1)`），建议局部 `fmtTimelineDamage(n)`（整数用 `toFixed(0)`，非有限数返回破折号）。**禁止裸插值长浮点**，不新增全局数字工具、不引入随机/当前时间。`wave` / `enemy_uid` 为整数可直接插值。
- **事件校验**（对齐 H5 `:620-626`）：仅渲染同时满足 `wave` 为正整数、`tick` 为非负整数、`reaction` 为非空字符串、`enemy_uid` 为整数、`damage` 为有限数的事件；非法事件丢弃。排序 `a.wave-b.wave || a.tick-b.tick`。`truncated` 仅认 `=== true`。`schema_version !== 'reaction-timeline-v1'` 时按空态处理（不编造）。

## 2. 文案规范（AI 四判据，延续 admin 既有洞察卡口径）

模块标题/摘要须同时具备四要素（对照 `laiyipao/admin/src/utils/adminAiInsight.ts:11-14` 的 `base(headline, evidence, ctaLabel, ctaPath)`）：

1. **可见汉字「AI」**：标题形如「AI · 反应时间线」——让运营一眼识别这是归因视图。
2. **真实数据结论**：摘要只陈述服务端 timeline 里真实存在的事件，例如「本局记录 N 次逐次反应，集中在第 M 波」；`N/M` 必须从渲染前的合法事件数确定性得出；无数据则不做结论、走空态。
3. **依据行**：明确写出依据字段，例如「依据：reaction_timeline.events（wave/tick/reaction/enemy_uid/damage），展示性归因、非结算凭证」。
4. **可执行语义**：给出 CTA 指向验真——例如「发起验真 →」定位到同抽屉已有验真区（`:558-589`），让运营看完逐次归因即可比对 replay_hash；不新造路由。

- 空态逐字：`该战报无逐次反应记录（旧版本结算）`（与 H5 r15 完全一致，不改写）。
- 截断态逐字：`已截断，仅显示前 200 条`（与 H5 r15 完全一致）。

## 3. 诚实红线（不可协商）

- **展示性归因、非权威结算**：timeline 是「发生了什么」的叙事，不是结算凭证；不得用于重算 score / 星级 / loot，不参与 replay_hash，不改变验真结论。验收真区仍是唯一比对入口。
- **不得反推编造**：严禁用聚合 `elements_used` / `reactions_used` map 反推波次、敌人或伤害来填充时间线；缺数据就是空态。
- **旧战报空态不报错**：字段缺失 / `events=[]` / 结构异常 / schema 不符一律显示逐字空态，不抛错、不显示 `undefined` / `NaN`。
- **反应名服务端来源**：只用 `labelOfReaction`，不硬编码中文名；未知 key 兜底原始 key。
- **数值确定性**：damage 经格式化，不裸插值长浮点；波次/敌人 uid 原样取服务端值，不补零、不伪造。
- **零真值**：本规范与实现不得写入任何域名/IP/口令/JWT/本机路径真值；API 地址只用回环/占位符。

## 4. test-first 契约测试设计

新增 `laiyipao/admin/tests/R16ReactionTimeline.contract.test.ts`（admin 的 vitest `include: ['tests/**/*.test.ts']`，`vitest.config.ts:55`，故测试落 `tests/` 而非 `src/`；jsdom + @vue/test-utils + `tests/setup.ts` 垫片齐备）。测试为**规范/基线守卫**：读 SPEC.md 断言条款齐全，读三端源码断言既有基线不回退；实施者删改规范或破坏锚点即变红。分组：

1. 契约主干：SPEC 含 `reaction-timeline-v1`、`reaction_timeline`、五字段、`truncated`、`200`、`schema_version`。
2. 选择器：SPEC 与源码含 `.r16-reaction-timeline` / `.r16-timeline-row` / `[data-r16-wave]` / `.r16-timeline-empty` / `.r16-truncated-note`；含 `labelOfReaction` 约束。
3. 排序/空态/截断/诚实红线：SPEC 含 `wave/tick 升序`、逐字空态与截断文案、`展示性归因`、`非权威结算`、`反推编造`、`旧战报空态`。
4. AI 四判据：SPEC 含 `AI`、`真实数据结论`、`依据行`、`可执行`。
5. 三端不回退：`types.gen.ts` 仍含 `reaction_timeline` 与 `ReactionTimelineEvent` 五字段；`adminbattledetail.go` 仍含 `json:"reaction_timeline"` 与「非权威结算」注释；H5 `battles.vue` 仍含 `.r15-reaction-timeline`（且 admin 源码不含 `.r15-`）；`tokens.css` 三锚点（`--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6`）不变且无 `--r16-` 视觉变量。

运行：`npx vitest run --config vitest.config.ts tests/R16ReactionTimeline.contract.test.ts --reporter=verbose`（在 `laiyipao/admin` 下）。

## 5. 三端分工与不回退

- **backend**：字段、查询、注释均不动（`adminbattledetail.go:62-64/100` 已就位）；本轮零后端改动。
- **h5**：`.r15-*` 选择器、文案、computed 校验全部不变；本轮零 H5 改动。
- **admin**：仅 `BattlesView.vue` 详情抽屉新增时间线模块（后续实施任务）；本轮只交付规范与契约测试。
- **不回退**：R1–R15 诚实文案、25% AoE、顶栏脱敏、令牌 v1/v3-r3/r6 锚点（`tokens.css:12-14`）不变；零新增视觉令牌；既有验真区、元素/反应/地形 chip 不被替换或删除。

> 公开仓库约束：本规范与配套测试不含任何域名/IP/JWT/口令/本机路径真值；所有路径均为仓库相对路径。
