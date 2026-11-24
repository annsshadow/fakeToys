# R32 规范：Wiki 技能作战机制字段与冷却单位同步

## 0. 缺口依据（亲读带行号）
- `laiyipao/admin/src/views/WikiView.vue:913-940` 技能表仅 9 列（id/name/element/family/base_damage/heat_cost/cooldown_ms/apply_stacks/descr），未展示 kind/pierce/aoe_radius/chain；`:33` 本地 `ConfigResp.skills` 类型也缺 chain 字段（有 kind/pierce/aoe_radius，但表格未消费），`:934` 冷却仍以原始毫秒数字直接展示。
- 权威结构 `laiyipao/server/internal/domain/content.go:39-57` `SeedSkill` 定义 `Kind`(active/passive)、`Pierce`、`AoeRadius`、`Chain`、`CooldownMs` 等，json tag 均为 snake_case；种子 `:326-410` 24 技能真实含多组非零值：ID3 温压弹 Pierce=1/AoeRadius=180/Chain=1、ID6 极寒冰弹 Pierce=3、ID7 电弧弹 Chain=2、ID9 雷暴链 Chain=4、ID13 聚焦激光 Pierce=6、ID14 高能射线 Pierce=8、ID19/ID22 Kind=passive/CooldownMs=0（被动）。
- H5/服务端真实消费 `laiyipao/miniapp/src/game/engine.ts:1468` `const evasiveCapable = s.aoeRadius > 0 || s.pierce > 0`、`:1505-1509` 弹丸 `pierceLeft: s.pierce + this.buffs.pierceBonus` / `chainLeft: s.chain + this.buffs.chainBonus` / `aoeRadius: s.aoeRadius + this.buffs.aoeBonus`、`:1618-1637` 范围与连锁结算、`:1656-1657` 穿透扣减；`laiyipao/miniapp/src/game/types.ts:41-59` `SkillDef` 同步声明上述字段。这些机制字段既已下发又影响战斗，Wiki 必须如实呈现。
- **aoe_radius 是像素半径整数，不是「原始定点值」**：Go `laiyipao/server/internal/domain/engine_combat.go:675` `rr := ToFixed(float64(radius))`、TS `laiyipao/miniapp/src/game/engine.ts:2014-2016` `private applyAoe(p: Projectile, origin: Enemy, radius: number): void` + `const rr = toFixed(radius)` 都直接把该字段当像素半径转定点后用于距离判定。§2 展示口径据此订正为「像素半径整数」。
- 权威配置端点沿用 t239 裁决：`WikiView.vue:620` `api.get<ConfigResp>('/config', { auth: false })` 读公开 `/api/v1/config`，`server/internal/service/service.go:1638` `Reactions: domain.AllReactionSpecs()` 同源；`/admin/reactions` 仅同源旁证不作对账基准。

## 1. 范围与零改动面
- 仅改 admin Wiki 技能表（展示层）：WikiView.vue 本地 `ConfigResp.skills` 类型补 `chain: number`；技能表新增「类型」「穿透」「范围」「连锁」列，冷却列改为格式化文案；允许表格容器表内横向滚动。
- 不改 `engine.ts` / 后端 / replay / hash / score / settle / loot / tokens；不改反应表、合成技能表或其它 Wiki 章节；不动令牌锚点 `--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`，禁止新增 `--r32-*` 令牌。
- 本规范只写契约与测试，不创建业务实现；实施模块 `laiyipao/admin/src/skillMechanicsText.ts` 由后续任务落地。

## 2. 显示格式契约（纯函数 `../src/skillMechanicsText`，真实 import，禁止内嵌参考实现）
- 导出 `kindText(kind: unknown): string`：逐字 `active` → `主动`，`passive` → `被动`；其它/空/非字符串 → `未知类型`（绝不把空值伪装成主动）。
- 导出 `cooldownText(ms: unknown): string`：仅当 `typeof ms === 'number' && Number.isFinite(ms) && ms > 0` 时转秒（毫秒 / 1000），整数秒显示 `N秒`，非整数最多保留一位小数并去掉尾随 0（例 1500 → `1.5秒`、800 → `0.8秒`、3000 → `3秒`）；`ms === 0` 逐字 `—`（被动技能无冷却，0 是真实值但语义为无冷却，不是未知）；`ms < 0` / NaN / Infinity / 非数字 → `—`，绝不输出 `NaN秒` / `-1秒`。
- 导出 `pierceText(n: unknown): string`：仅当 `typeof n === 'number' && Number.isInteger(n) && n > 0` → `穿透 N`（例 1 → `穿透 1`、8 → `穿透 8`）；`n === 0` → `—`（无穿透，不写「未知」）；`1.5` / 负数 / NaN / Infinity / 字符串 / null / undefined / 对象 → `—`。
- 导出 `aoeRadiusText(n: unknown): string`：仅当 `typeof n === 'number' && Number.isInteger(n) && n > 0` → `范围 N`（N 为**像素半径整数**，例 180 → `范围 180`；不做像素换算、不夸大、不追加除 N 外的数字）；`n === 0` → `—`；`1.5` / 负数 / NaN / Infinity / 字符串 / null / undefined / 对象 → `—`。依据见 §0 的 Go `ToFixed(float64(radius))` 与 TS `toFixed(radius)`。
- 导出 `chainText(n: unknown): string`：仅当 `typeof n === 'number' && Number.isInteger(n) && n > 0` → `连锁 N`（弹射次数整数，例 2 → `连锁 2`、4 → `连锁 4`）；`n === 0` → `—`；`1.5` / 负数 / NaN / Infinity / 字符串 / null / undefined / 对象 → `—`。
- 导出 `MECHANISM_EMPTY = '—'` 供列空值统一引用；所有格式化函数纯显示、无副作用、不修改入参。入参签名必须用 `unknown`，以便真实覆盖非数字（不是用 `number` 伪装的编译期约束）。
- 诚实口径：0 值统一显示 `—` 表示「该技能无此机制」，与 `未知类型` 严格区分；未知 key 只来自真实服务端非法值，禁止为了好看把 0 改写成「无」之外的任何推断；展示值不参与任何战斗计算。

## 3. Wiki 技能表 UI 契约与选择器
- 技能表外层 `#skill-table`（保留现有 id）追加 `.r32-skill-mechanics` 容器类；表格横向滚动容器 `.r32-skill-table-scroll`，`overflow-x: auto`、`max-width: 100%`，不得造成页面级横向溢出。
- 新增列必须逐字命名：`类型`（调用 `kindText(row.kind)`）、`穿透`（`pierceText(row.pierce)`）、`范围`（`aoeRadiusText(row.aoe_radius)`）、`连锁`（`chainText(row.chain)`）；原「冷却」列保留列名但单元格改调 `cooldownText(row.cooldown_ms)`；原列顺序与 ID/名称/元素/系别/伤害/热量/层数/说明均保留。
- 单元测试守卫 `WikiView.vue` 必须真实 import 以上五个函数并调用（仅 import 不算），且 `.r32-skill-mechanics` / `.r32-skill-table-scroll` 出现在技能表段（第 913-940 行区间），技能表列名齐全。
- CSS 只作用于 `.r32-skill-table-scroll` 与新增列，桌面/移动端均表内滚动；不改全局 `.el-table` 规则、不改其它 tab。

## 4. AI 四判据（人类可见 AI，结论/依据/CTA/红线同屏）
- A1：Wiki 技能区必须存在可见汉字 `AI` 标记（新增 `.r32-ai-note` 说明行即可），不得只藏在注释或测试数据里。
- A2：AI 结论改为**由 `cfg.skills` 确定性聚合**，不得依赖「当前选中行」概念（Wiki 技能表没有选中行）。导出 `buildSkillMechanicsAi(skills: readonly SkillMechanicsRow[]): string`，逐字模板 `共 {total} 个技能：主动 {active} 个，被动 {passive} 个，多目标 {multi} 个`；`total = skills.length`；`active` / `passive` 按 `kindText(row.kind)` 结果为 `主动` / `被动` 计数；`multi` 按该行 `pierce > 0 || aoe_radius > 0 || chain > 0`（仅合法整数参与，非法值不计）计数。空数组返回 `共 0 个技能：主动 0 个，被动 0 个，多目标 0 个`（诚实空态，不编造）。
- A3：依据行逐字 `依据：公开 /api/v1/config 的 skills（与 H5 引擎同源 SeedSkill）；格式化仅展示，不改结算`。
- A4：CTA 导出常量 `SKILL_AI_CTA = '查看合成技能 →'`，指向 Wiki 已有 `#composite` 锚点。**必须用 hash-router 安全链接**：`href` 取 `urlFor('skills', 'composite')`（内部 `router.resolve({ path: '/wiki', query: { tab: 'skills', anchor: 'composite' } }).href`），点击处理 `@click.prevent="goTo({ tab: 'skills', anchor: 'composite' })"` 复用现有目录导航，保持当前 `/wiki` 路由并滚动到 `#composite`，不跳数据看板。禁止裸 `href="#composite"` —— 在 `createWebHashHistory` 下它会替换整段 hash，被 catch-all `/:pathMatch(.*)*` 重定向到 `/admin/dashboard`（真实缺陷）。链接保留可复制/新标签打开能力；结论/依据/CTA/红线四要素同屏可见。
- 诚实红线逐字 `辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准`；禁止把展示值冒充伤害承诺或结算结果。

## 5. 三视口 QA 门禁（实施后必须实测）
- 390×844 H5 竖屏、768×1024 平板、1280×800 桌面三视口打开 Wiki 技能 tab：表内横向滚动可用、页面 `document.documentElement.scrollWidth <= innerWidth + 1`；列名与函数文案为中文，无 `NaN` / `undefined` / `[object Object]`。
- 关键种子行核对：ID3 温压弹 → 主动 / 2秒 / 穿透 1 / 范围 180 / 连锁 1；ID7 电弧弹 → 主动 / 0.9秒 / — / — / 连锁 2；ID19 侦察无人机 → 被动 / — / — / — / —。
- AI 聚合核对：24 个种子技能按 §4 口径得 `共 24 个技能：主动 22 个，被动 2 个，多目标 N 个`（N 由真实种子计算，禁止手填）；CTA 点击后跳到 `#composite`。
- 令牌锚点 v1 / v3-r3 / r6 三端不回退；R28/R29/R30/R31 既有契约测试全绿；grep 确认无 `--r32-` 令牌。

## 6. 契约测试与 RED 门禁
- `laiyipao/admin/tests/WikiSkillMechanics.test.ts` 必须真实 `import { kindText, cooldownText, pierceText, aoeRadiusText, chainText, MECHANISM_EMPTY, buildSkillMechanicsAi, SKILL_AI_CTA } from '../src/skillMechanicsText'`，禁止把参考实现写进测试文件；模块不存在时整 suite RED。
- 逐字断言 §2 全部格式与边界（含被动 0 值、`Number.isInteger` 拒绝 1.5/NaN/Infinity/非数字、不产 NaN）、§0 像素半径证据（Go `rr := ToFixed(float64(radius))` 与 TS `const rr = toFixed(radius)`）、§3 选择器/列名/真实调用、§4 AI 聚合/依据/CTA/红线、§1/§5 令牌锚点与禁用 `--r32-`。
- 先提交本规范与测试并执行 `cd laiyipao/admin && npx vitest run --config vitest.config.ts tests/WikiSkillMechanics.test.ts --reporter=verbose`，保留模块缺失的 RED 输出（exit=1、Failed to resolve import ../src/skillMechanicsText）作为 test-first 证据；实施模块落地后同命令应全绿。

## 7. 分工
- t247（首版）+ t248（补强）：本 SPEC + 契约测试 + RED 证据，均不写实施。
- 后续实施任务：新增 `skillMechanicsText.ts`、改 `WikiView.vue` 技能表、三视口实测；不得触碰 engine/后端/replay/hash/score/settle/loot/tokens。