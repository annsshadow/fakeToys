# R36 规范：Wiki 合成技能表字段对齐权威 SeedSkill + 修复本地类型漂移

## 0. 缺口依据（亲读带行号）
- 权威下发：laiyipao/server/internal/service/service.go:1517 定义 Composite []domain.SeedSkill 的 json tag composite_skills，:1631-1632 以 Skills: domain.ScaleAllSkills() / Composite: domain.ScaleAllCompositeSkills() 同源下发。公开端点就是 WikiView 现有 api.get<ConfigResp>('/config', { auth: false })（laiyipao/admin/src/views/WikiView.vue:657），即公开 /api/v1/config 的 composite_skills；/admin/* 仅旁证，不作对账基准。
- 权威结构：laiyipao/server/internal/domain/content.go:43-56 的 SeedSkill 含 Kind(active/passive)、BaseDamage、HeatCost、CooldownMs、Pierce、AoeRadius、ApplyStacks、Chain、UnlockLevel，json tag 全为 snake_case；laiyipao/server/internal/domain/content.go:413-473 的 SeedCompositeSkills 是配方产物 18 条（ID 31~48），例如 ID31 灼热弹幕 BaseDamage=260/HeatCost=46/CooldownMs=1900/AoeRadius=190/ApplyStacks=3/Chain=1/UnlockLevel=30，ID38 雷暴链 Chain=6/UnlockLevel=62，ID48 光棱攻城 BaseDamage=520/Pierce=3/AoeRadius=300/Chain=5/UnlockLevel=70。laiyipao/admin/src/api/types.gen.ts:752-770 的 SeedSkill 同步声明上述字段。
- 双端真实消费：laiyipao/miniapp/src/game/engine.ts:1468 的 const evasiveCapable = s.aoeRadius > 0 || s.pierce > 0，:1505-1509 的 pierceLeft: s.pierce + this.buffs.pierceBonus / chainLeft: s.chain + this.buffs.chainBonus / aoeRadius: s.aoeRadius + this.buffs.aoeBonus，:1656-1657 穿透扣减。合成技能是 SeedSkill 同源，字段既下发又进结算，文档站必须如实呈现。
- 当前漂移：laiyipao/admin/src/views/WikiView.vue:42 的 composite_skills 本地类型只有 { id, name, element, descr } 四字段；模板合成技能表（约 :1038-1057）也只展示 ID/名称/元素/说明四列。玩家在文档站对「合成技能到底强在哪、几级解锁」无答案，与 R31 反应 / R32 技能 / R33 敌人 / R34 关卡 / R35 宝石同源漂移。

## 1. 范围与零改动面
- 本轮只写本规范与契约测试 laiyipao/admin/tests/WikiCompositeMechanics.test.ts，先制造真实 RED；不改 WikiView.vue，不创建实现模块。实施任务再新增 laiyipao/admin/src/compositeMechanicsText.ts 并改 Wiki 合成技能表。
- 不改 engine.ts / 后端 / replay / hash / score / settle / loot / tokens；不改种子数值、配方、服务端字段；不动令牌三锚点 lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6，禁止新增 `--r36-*` 令牌（复用既有 lyp 令牌）。
- 合成技能与基础技能同为 SeedSkill：字段格式化逻辑必须复用 @/skillMechanicsText，不重复实现。即复用 kindText / cooldownText / pierceText / aoeRadiusText / chainText 与同值 MECHANISM_EMPTY，禁止在 compositeMechanicsText.ts 或模板里写第二套格式化。

## 2. 显示格式契约（纯函数 ../src/compositeMechanicsText，真实 import，禁止内嵌参考实现）
- MECHANISM_EMPTY = 破折号（—）：与 R32 skillMechanicsText 同源同值；0 / 缺失 / 非法 / 非有限一律显示它，禁止输出 NaN / undefined / [object Object]。
- 复用 R32 五个函数，入参一律 unknown，逐字口径不变：kindText（active→主动、passive→被动、其它→未知类型）、cooldownText（毫秒转秒，整数秒 N秒，非整数最多一位小数，1500→1.5秒、800→0.8秒，0/非法→—）、pierceText（合法正整数→穿透 N，其余→—）、aoeRadiusText（像素半径整数，正整数→范围 N，其余→—）、chainText（弹射次数正整数→连锁 N，其余→—）。aoe_radius 是像素半径整数，不做像素换算、不追加数字。
- compositeUnlockText(n: unknown): string：只接受有限正整数（typeof n === 'number' && Number.isInteger(n) && n > 0）→ N 级解锁（例 30→30 级解锁、70→70 级解锁）；0 / 负数 / 小数 / NaN / ±Infinity / 字符串 / null / undefined / 对象 → MECHANISM_EMPTY。新福利字段 unlock_level（展示 unlock 文案）只走此函数，禁止把 0 伪装成「1 级解锁」。
- apply_stacks（层数）与 chain（连锁）均为正整数：连锁列直接调用 chainText(row.chain)；层数列只展示合法的 row.apply_stacks，非法/缺失归 MECHANISM_EMPTY，禁止自造第二套正整数格式化函数。
- AI 行形状 CompositeMechanicsRow：{ kind: unknown; pierce: unknown; aoe_radius: unknown; chain: unknown }，只读、不改入参。
- 诚实边界：以上全部是公开配置的配置基值展示，不参与任何战斗计算；实际效果以引擎结算为准，权威回放以 replay_hash 为准。

## 3. Wiki 合成技能表 UI 契约与选择器
- ConfigResp.composite_skills 本地类型必须补齐并与 Go json tag 逐字一致：id / name / element / descr / kind / base_damage / heat_cost / cooldown_ms / pierce / aoe_radius / apply_stacks / chain / unlock_level（可含 family），修复四字段漂移。
- id="composite" 段整体包 .r36-composite-mechanics，表格外包 .r36-composite-table-scroll，overflow-x: auto；max-width: 100%，只允许表内横向滚动，不得造成页面级横向溢出；CSS 只作用于这两个选择器，不改全局 .el-table。
- 表格在保留 ID / 名称 / 元素 / 说明的基础上，新增列逐字命名：类型（kindText(row.kind)）、冷却（cooldownText(row.cooldown_ms)）、伤害（row.base_damage）、热量（row.heat_cost）、穿透（pierceText(row.pierce)）、范围（aoeRadiusText(row.aoe_radius)）、连锁（chainText(row.chain)）、层数（row.apply_stacks）、解锁等级（compositeUnlockText(row.unlock_level)）。
- WikiView 必须真实从 @/compositeMechanicsText 导入 MECHANISM_EMPTY / compositeUnlockText / buildCompositeMechanicsAi / COMPOSITE_AI_BASIS / COMPOSITE_AI_HONESTY / COMPOSITE_AI_CTA，并对格式化函数与 AI 聚合有真实调用（仅 import 不算）；R32 五函数继续从 @/skillMechanicsText 导入。

## 4. AI 四判据（结论 / 依据 / CTA / 红线同屏）
- A1：合成技能段有可见汉字 AI 标记（AI 机制概览），不得只藏在注释或测试数据里。
- A2：AI 结论由公开 cfg.composite_skills 确定性聚合，不手填、不依赖选中行。buildCompositeMechanicsAi(rows: readonly CompositeMechanicsRow[]): string，逐字模板 共 {total} 个合成技能：主动 {active} 个，多目标 {multi} 个；total = rows.length，active 按 kindText(row.kind) === 主动 计数，multi 按 pierce > 0 || aoe_radius > 0 || chain > 0（仅合法整数参与，非法不计）计数。空数组返回 共 0 个合成技能：主动 0 个，多目标 0 个（诚实空态）。18 个真实种子按此口径为 共 18 个合成技能：主动 18 个，多目标 18 个（禁止手填，测试用样本验证确定性）。
- A3：依据行逐字 依据：公开 /api/v1/config 的 composite_skills（与服务端 SeedSkill / SeedCompositeSkills 同源）；格式化仅展示，不改结算（COMPOSITE_AI_BASIS）。
- A4：CTA 逐字 查看合成技能 →（COMPOSITE_AI_CTA）。必须 hash-router 安全：href 取 urlFor('skills', 'composite')（内部 router.resolve({ path: '/wiki', query: { tab: 'skills', anchor: 'composite' } }).href），点击 @click.prevent="goTo({ tab: 'skills', anchor: 'composite' })"；禁止裸 href="#composite" —— createWebHashHistory 下会替换整段 hash 并被 catch-all 重定向到 /admin/dashboard（R32 真实教训）。链接保留可复制 / 新标签打开能力。
- 红线逐字 辅助展示非权威战斗数据，展示为配置基值，实际效果以引擎结算为准；权威回放以 replay_hash 为准（COMPOSITE_AI_HONESTY），与结论 / 依据 / CTA 同屏；禁止把配置基值冒充伤害承诺或实战结算结果。

## 5. 三视口 QA 门禁（实施后必须实测）
- 390×844 H5 竖屏、768×1024 平板、1280×800 桌面打开 Wiki 技能 tab 的合成技能段：表内横向滚动可用，document.documentElement.scrollWidth <= innerWidth + 1；无 NaN / undefined / [object Object]。
- 种子行核对：ID31 灼热弹幕 → 主动 / 1.9秒 / 260 / 46 / — / 范围 190 / 连锁 1 / 层数 3 / 30 级解锁；ID38 雷暴链 → 主动 / 3.2秒 / 400 / 68 / — / — / 连锁 6 / 层数 4 / 62 级解锁；ID48 光棱攻城 → 主动 / 3.4秒 / 520 / 72 / 穿透 3 / 范围 300 / 连锁 5 / 层数 4 / 70 级解锁。
- AI 聚合核对：18 条真实种子按 §4 口径得 共 18 个合成技能：主动 18 个，多目标 18 个；CTA 点击后停留在 /wiki 合成技能段。
- 令牌三锚点不回退；grep 确认无 --r36-；R31–R35 既有契约测试全绿。

## 6. 契约测试与 RED 门禁
- laiyipao/admin/tests/WikiCompositeMechanics.test.ts 必须真实 import { MECHANISM_EMPTY, compositeUnlockText, buildCompositeMechanicsAi, COMPOSITE_AI_BASIS, COMPOSITE_AI_HONESTY, COMPOSITE_AI_CTA } from "../src/compositeMechanicsText"，禁止内嵌参考实现；模块不存在时整 suite RED。
- 逐字断言 §2 格式与 unknown 边界、§0 权威字段/种子/引擎消费、§3 类型声明与列/真实调用、§4 四判据与 hash 安全 CTA、§1/§5 令牌锚点与禁用 --r36-。
- 先执行 cd laiyipao/admin && pnpm exec vitest run tests/WikiCompositeMechanics.test.ts --reporter=dot，保留模块缺失 RED（exit=1、逐字 Failed to resolve import "../src/compositeMechanicsText"）作为 test-first 证据；实施模块落地后同命令应全绿。

## 7. 分工
- t1（本轮）：本 SPEC + 契约测试 + RED 证据，不写实施、不改 WikiView.vue。
- 后续实施任务：落地 compositeMechanicsText.ts（复用 @/skillMechanicsText）、改 Wiki 合成技能表与本地类型、三视口实测；不得触碰 engine / 后端 / replay / hash / score / settle / loot / tokens。
