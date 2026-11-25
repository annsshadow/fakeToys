# R33 规范：Wiki 敌人战斗机制字段与状态文案同步

## 0. 缺口依据（亲读带行号）
- laiyipao/admin/src/views/WikiView.vue:877-906 敌人表仅 6 列（ID/名称/类型/血量/五维抗性/说明），未展示 speed/armor/shield_hp/attack/attack_range/attack_interval/fly_height/burrow/is_boss；:33 本地 ConfigResp.enemies 类型只有 id/code/name/category/hp/resist/descr，机制字段全缺。
- 权威结构 laiyipao/server/internal/domain/content.go:18-35 SeedEnemy：Speed（定点 x1000，像素/秒）、Armor、ShieldHP、Attack、AttackRange、AttackEvery（json tag attack_interval，毫秒）、FlyHeight、Burrow、IsBoss，json tag 全 snake_case。种子 :256-322 真实含非零值：ID1 游荡者 Speed=40000（=40像素/秒）、ID3 壮硕体 Armor=50、ID10 铁罐炮 ShieldHP=60/Attack=28/AttackRange=500/AttackEvery=3200（=3.2秒）、ID13 浮空囊 FlyHeight=180、ID17 潜地者 Burrow=true、ID19 母巢守卫 IsBoss=true/Armor=250/ShieldHP=3000。以上为 content.go 源码字面值；实际下发到公开 /api/v1/config 的数值可能不同，§5 关键种子行以公开 config 为准。
- 客户端类型 laiyipao/miniapp/src/game/types.ts:19-39 EnemyDef 同步声明上述字段（speed 注释定点 x1000、attack_interval 注释毫秒）。
- 双端引擎真实消费：TS laiyipao/miniapp/src/game/engine.ts:1022 attackRangeFixed=toFixed(def.attack_range)、:1023 hitRadiusFixed=toFixed(28 + def.fly_height * 0.1)、:1034 shield=BigInt(def.shield_hp)、:1044 armorPermille=BigInt(def.armor || 0)、:1045-1047 flyHeight/burrow/isBoss、:1055-1057 speed/attack/attackRange、:1066 attackInterval=Math.max(1, Math.round((def.attack_interval * permille) / 1000))；Go laiyipao/server/internal/domain/engine.go:107-136 Enemy 结构、engine_wave.go:129-150 生成（Shield=def.ShieldHP、ArmorPermille=def.Armor、AttackInterval=scaledMs(def.AttackEvery,...)）、engine_combat.go:522-526 护甲折减、:240 飞行/潜地判定、:660 IsBoss 击败计分。字段已下发且影响战斗，Wiki 必须如实呈现。
- 权威配置端点沿用 t239 裁决：WikiView.vue:621 api.get<ConfigResp>('/config', { auth: false }) 读公开 /api/v1/config；/admin/reactions 仅同源旁证，不作对账基准。

## 1. 范围与零改动面
- 仅改 admin Wiki 敌人表（展示层）：WikiView.vue 本地 ConfigResp.enemies 类型补全九字段；敌人表新增九列；允许表格容器表内横向滚动；新增敌人机制 AI 说明行。
- 不改 engine.ts / 后端 / replay / hash / score / settle / loot / tokens；不改反应表、技能表、其它 Wiki 章节；不动令牌锚点 --lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6，禁止新增 --r33-* 令牌。
- 不新增裸 hash 路由：CTA 必须走 urlFor/goTo 复用现有 /wiki 导航，禁止 href="#enemies" 这类裸锚点（createWebHashHistory 下会被 catch-all 重定向到数据看板）。
- 本规范只写契约与测试，不创建业务实现；实施模块 laiyipao/admin/src/enemyMechanicsText.ts 由后续任务落地。

## 2. 显示格式契约（纯函数 ../src/enemyMechanicsText，真实 import，禁止内嵌参考实现）
- 统一空值常量 MECHANISM_EMPTY = '—'。所有函数入参用 unknown（真实覆盖非数字），纯显示、无副作用、不改入参。
- speedText(v: unknown): string：v 为 number 且 Number.isFinite(v) 且 v>0 时按「定点 x1000」除以 1000 得像素/秒，整数显示 N像素/秒（40000 → 40像素/秒、75000 → 75像素/秒），非整数最多一位小数去尾随 0（如 42500 → 42.5像素/秒）；v===0 → '—'（不动=无移动机制，不伪装为有速度）；负数/NaN/Infinity/字符串/null/undefined/对象 → '—'。不做四舍五入到整数、不追加「倍」等推断。
- armorText(v: unknown): string：v 为 number 且 Number.isInteger(v) 且 v>0 → N‰（千分比，50 → 50‰、250 → 250‰、350 → 350‰）。必须显示千分号 ‰，绝不按百分比显示成 50%（那会把护甲夸大 10 倍，与 R31 amplifyPctText(v)=v/10 踩坑同源）。v===0 → '—'（无护甲）；1.5/负数/NaN/Infinity/非数字 → '—'。
- shieldHpText(v: unknown): string：v 为 number 且 Number.isInteger(v) 且 v>0 → 原值 N（60 → 60、3000 → 3000，不换算单位）；v===0 → '—'（无护盾）；非法 → '—'。
- attackText(v: unknown): string：v 为 number 且 Number.isInteger(v) 且 v>0 → 原值 N（8 → 8、35 → 35）；v===0 → '—'（不主动攻击，如 ID12 索敌眼 Attack=0）；非法 → '—'。
- attackRangeText(v: unknown): string：v 为 number 且 Number.isInteger(v) 且 v>0 → 原值 N（引擎逻辑单位，150 → 150、500 → 500）；不乘 toFixed、不换算像素、不追加单位词；v===0 → '—'；非法 → '—'。
- attackIntervalText(v: unknown): string：v 为 number 且 Number.isFinite(v) 且 v>0 时按毫秒转秒（v/1000），整数秒显示 N秒（2000 → 2秒、4000 → 4秒），非整数最多一位小数去尾随 0（2500 → 2.5秒、3200 → 3.2秒、2600 → 2.6秒）；v===0 → '—'（无主动攻击间隔）；负数/NaN/Infinity/非数字 → '—'，绝不输出 NaN秒。此值为种子秒数，引擎会按 tuning 倍率缩放（engine.ts:1066），Wiki 展示的是配置基值，需在依据行说明。
- flyHeightText(v: unknown): string：v 为 number 且 Number.isInteger(v) 且 v>0 → 原值 N（引擎逻辑单位，180 → 180、300 → 300）；v===0 → '—'（地面单位）；非法 → '—'。
- boolText(v: unknown): string：严格 v===true → '是'；v===false → '—'（无此特性，不显示「否」以免被误读为有机制）；其它（含 1/'true'/null/undefined/对象）→ '—'。用于 burrow/is_boss 两列。
- 零值规则总纲：0 是真实配置值，语义为「无此机制」，统一显示 '—'；非法值同样 '—'，二者都不声称机制存在。禁止把 0 改写成「未知」「无数据」以外的推断文案，禁止 NaN/undefined/[object Object] 出现在表格。
- 诚实口径：全部格式化仅展示，不参与任何战斗计算；展示的是配置基值（如 attack_interval 为缩放前毫秒），实际效果以引擎结算为准。

## 3. Wiki 敌人表 UI 契约与选择器
- 敌人表外层 section（WikiView.vue:865 起，section v-show activeTab === 'enemies'）追加类 .r33-enemy-mechanics；表格包一层 .r33-enemy-table-scroll（overflow-x: auto、max-width: 100%），表内横向滚动，不造成页面级横向溢出。
- 新增列逐字命名与调用：速度（speedText(row.speed)）、护甲（armorText(row.armor)）、护盾（shieldHpText(row.shield_hp)）、攻击（attackText(row.attack)）、射程（attackRangeText(row.attack_range)）、攻击间隔（attackIntervalText(row.attack_interval)）、飞行高度（flyHeightText(row.fly_height)）、潜地（boolText(row.burrow)）、首领（boolText(row.is_boss)）。原 ID/名称/类型/血量/五维抗性/说明列与 rowFocusClass('enemy')、空态 el-empty 全部保留。
- 本地 ConfigResp.enemies 类型(item)必须补 speed/armor/shield_hp/attack/attack_range/attack_interval/fly_height/burrow/is_boss 十个字段（含 code），与 snake_case 逐字一致。
- 单元测试守卫 WikiView.vue 必须真实 import 八个格式化函数与 MECHANISM_EMPTY、buildEnemyMechanicsAi、ENEMY_AI_CTA，并对每个函数有调用（仅 import 不算）。

## 4. AI 四判据（人类可见 AI，结论/依据/CTA/红线同屏）
- A1：Wiki 敌人区必须存在可见汉字 AI 标记（新增 .r33-ai-note 说明行）。
- A2：AI 结论由公开 cfg.enemies 确定性聚合，导出 buildEnemyMechanicsAi(enemies: readonly EnemyMechanicsRow[]): string，逐字模板 共 {total} 种敌人：首领 {boss} 种，飞行 {fly} 种，潜地 {burrow} 种，带护甲 {armor} 种；total=enemies.length；boss 严格 is_boss===true 计数；fly 严格 category==='flying' 计数；burrow 严格 burrow===true 计数；armor 按 armorText(row.armor) 不为 '—' 计数（合法正整数）。空数组返回 共 0 种敌人：首领 0 种，飞行 0 种，潜地 0 种，带护甲 0 种（诚实空态，不编造）。模板绑定逐字 {{ buildEnemyMechanicsAi(cfg?.enemies ?? []) }}。
- A3：依据行逐字 依据：公开 /api/v1/config 的 enemies（与 H5/Go 引擎同源 SeedEnemy）；展示为配置基值，实际效果以引擎结算为准。
- A4：CTA 导出常量 ENEMY_AI_CTA = '查看敌人列表 →'，指向当前敌人 tab。必须 hash-router 安全：:href="urlFor('enemies')" + @click.prevent="goTo({ tab: 'enemies' })"，复用现有目录导航；禁止裸 href="#enemies"（createWebHashHistory 下会被 catch-all 重定向到 /admin/dashboard）。链接保留可复制/新标签打开能力。
- 诚实红线逐字 辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准；禁止把展示值冒充伤害承诺或结算结果。
- 结论/依据/CTA/红线四要素同屏可见，禁止把 {total}/{boss}/{fly}/{burrow}/{armor} 说明占位写进 UI（须用真实数值）。

## 5. 三视口 QA 门禁（实施后必须实测）
- 390×844 / 768×1024 / 1280×800 三视口打开 Wiki 敌人 tab：表内横向滚动可用、document.documentElement.scrollWidth <= innerWidth + 1；列名与文案为中文，无 NaN/undefined/[object Object]。
- 关键种子行核对：ID1 游荡者 → 40像素/秒 / 护甲— / 护盾— / 攻击— / 射程— / 攻击间隔— / 飞行高度— / 潜地— / 首领—；ID10 铁罐炮 → 18像素/秒 / 480 / 28 / 500 / 3.2秒；ID13 浮空囊 → 飞行高度 180；ID17 潜地者 → 潜地 是；ID22 白垩母皇 → 首领 是 / 护甲 350‰ / 护盾 64000。
- 示例权威口径：以上数值取自公开 /api/v1/config 的 enemies 快照，仅作对账说明，不是独立真源；与 §0 记录的 content.go 种子字面值来源不同（§0 为源码字面值、§5 为下发配置值），出现差异时以公开 config 为准。QA 曾发现本行护盾示例漂移（ID10 写 60、ID22 写 8000），现已按 config 修正为 480/64000，轨迹见 docs/laiyipao/audits/round33-wiki-enemy-mechanics/REPORT.md。
- AI 聚合按 §4 口径由真实 cfg.enemies 计算，禁止手填；CTA 点击后停留在 /wiki 敌人 tab。
- token 三锚点 v1/v3-r3/r6 保留；R31/R32 既有契约测试全绿；grep 确认无 --r33- 令牌。

## 6. 契约测试与 RED 门禁
- laiyipao/admin/tests/WikiEnemyMechanics.test.ts 必须真实 import { speedText, armorText, shieldHpText, attackText, attackRangeText, attackIntervalText, flyHeightText, boolText, MECHANISM_EMPTY, buildEnemyMechanicsAi, ENEMY_AI_CTA } from '../src/enemyMechanicsText'，禁止内嵌参考实现；模块不存在时整 suite RED。
- 逐字断言 §2 全部格式与边界（千分号 ‰、40像素/秒、3.2秒、0 值 '—'、Number.isInteger/Number.isFinite 拒绝 1.5/NaN/Infinity/非数字）、§0 权威类型与双端引擎消费点、§3 选择器/列名/真实调用/类型字段、§4 AI 聚合/依据/CTA/红线/hash 安全链接、§1/§5 令牌锚点与禁用 --r33-。
- 先落本规范与测试并执行 cd laiyipao/admin && npx vitest run --config vitest.config.ts tests/WikiEnemyMechanics.test.ts --reporter=verbose，保留模块缺失的 RED 输出（exit=1、Failed to resolve import ../src/enemyMechanicsText）作为 test-first 证据；实施模块落地后同命令应全绿。

## 7. 分工
- t257：本 SPEC + 契约测试 + RED 证据，不写实施。
- 后续实施任务：新增 enemyMechanicsText.ts、改 WikiView.vue 敌人表、三视口实测；不得触碰 engine/后端/replay/hash/score/settle/loot/tokens。