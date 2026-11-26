# R31 规范：Wiki 反应机制字段与状态文案同步（round31-wiki-reaction-mechanics）

> 目标：让 admin 文档站反应链一览表逐字段对齐服务端 ReactionSpec 已下发但本地类型丢弃的三个机制字段（spread_stacks / armor_shred_permille / knockback），并改正把 corrosion_spread 已实现能力说成没有特殊效果的陈旧文案。只改 admin Wiki 的本地类型、表列与展示辅助，不改引擎/后端/replay_hash/score/结算/tokens，本轮不实现 burn_cloud 火区（C1 产品阻塞）。

## 0 缺口依据（亲读带行号；不可编造）

- laiyipao/admin/src/views/WikiView.vue:57 本地 reactions: Array<{ key; name; base_coef; attack_weight_pct; status_duration_ms; aoe_radius; dispel_shield; amplify_pct; descr }> —— 缺 spread_stacks / armor_shred_permille / knockback，与服务端 json tag 不同步（同文件 :54-56 曾因 PascalCase 踩过字段静默空白的坑）。
- laiyipao/admin/src/views/WikiView.vue:767-827 反应表 8 列（反应 / 系数 / 攻击力权重 / 控制效果 / 受击增伤 / 驱散护盾 / 溅射半径 / 说明），三字段无任何列消费者；:762-765 注释钉死列宽总和要留在桌面 ~914px 正文可用宽度内、横向滚动藏在表底。
- laiyipao/admin/src/views/WikiView.vue:831-833 仍写"仍**没有特殊效果**的只剩两条（燃烧云的火区 / 腐蚀扩散的层数传播）—— 具体缺什么见 README 已知边界第 19 条"，与 docs/laiyipao 已知边界表 D2（2026-10-20 已实施 corrosion_spread 层数传播 1 层 / 120px，不触发新反应）冲突。
- laiyipao/server/internal/domain/elements.go:106-149 ReactionSpec 真源：:125 SpreadStacks 的 json tag 为 spread_stacks（层数传播量，corrosion_spread 专用，0=无；触发时把层数写入半径 AoeRadius 内其他敌人的腐蚀层数、clamp elementCap、不触发新反应）；:135 ArmorShredPermille 的 json tag 为 armor_shred_permille（削甲量，千分比，armor_break 专用，0=无；StatusDurationMs 内目标护甲被削本值）；:138 Knockback 的 json tag 为 knockback（击退位移，定点 ×1000，armor_break 专用，0=无；一次性向右位移）。
- laiyipao/server/internal/domain/elements.go:235 corrosion_spread 为 AoeRadius: 120, SpreadStacks: 1；:241-245 armor_break 为 StatusDurationMs: 4000, ArmorShredPermille: 150, Knockback: 4000。
- laiyipao/server/internal/service/service.go:1601-1609 LoadGameConfig 从 staticConfig() 取全量配置，:1638 `Reactions: domain.AllReactionSpecs()` —— 完整三字段随公开配置端点 /api/v1/config 下发；这是 Wiki 读数的**权威真源**。
- laiyipao/admin/src/api/types.gen.ts:595-608 生成类型 ReactionSpec 已含 spread_stacks:number / armor_shred_permille:number / knockback:number —— 只有 WikiView 本地手写类型没跟上。
- 引擎消费证据：laiyipao/miniapp/src/game/engine.ts:1105-1107 knockback 一次性 e.x += e.knockback 后清零；:1684-1685 有效护甲 = armorPermille − armorShredPermille；:1775-1781 armor_break 写 armorShredPermille/knockback；:1873-1874 与 :2000-2012 corrosion_spread 走 triggerCorrosionSpread（只写层数、不造成伤害、不触发新反应）。
- 结论：服务端与引擎都已实现并消费三字段，唯一缺口在 admin Wiki 的展示层；修法是把本地类型补齐、加列、加显示辅助与诚实文案，不发明玩法、不改权威数值。

## 1 逐字段显示口径（真实单位，禁止夸大 10 倍）

- spread_stacks（层数传播量）：列标题"层数传播"；值为正整数时显示"传播 N 层"（N 为原始整数，不做百分比/秒换算）；0 或缺失显示"—"。仅 corrosion_spread 非零。依据 :119-125 / :235，真实值 1 层（半径以既有溅射半径列 120px 呈现）。
- armor_shred_permille（削甲比例）：列标题"削甲"；单位**千分比**，直接显示 N‰（150 → 150‰，即 15%）；**不得**当百分比直接拼 %（那会把效果夸大 10 倍，与 WikiView.vue:542-548 amplifyPctText(v)=v/10 的踩坑同源）；0 或缺失显示"—"。仅 armor_break 非零。
- knockback（击退位移）：列标题"击退"；服务端是**定点 ×1000**，显示像素 = 值 / 1000（4000 → 4px，2500 → 2.5px）；0 或缺失显示"—"。仅 armor_break 非零。依据 :136-138 / :245 与"4px（定点 4000）"口径。
- 纯显示辅助统一约定：三个 helper 在 0 / null / undefined / 非有限数时返回展示空值 MECHANISM_EMPTY（逐字"—"），绝不返回"0"、"NaN"、"undefined" 或空串；col 模板仅消费 helper 结果，不自行拼接单位。
- 诚实红线：三字段是**展示既有机制参数**，不参与结算/replay_hash/score；admin 从公开配置端点 /config（/api/v1/config）读取真值（WikiView.vue:619 `api.get<ConfigResp>('/config', { auth: false })`，页面 :673 自述与游戏内数值同源）。零值显示"—"表示"该反应无此机制"，**不得**被解读成"该机制未实现"，尤其不得暗示 burn_cloud 火区已实现。/api/v1/admin/reactions 只是共享同一 `domain.AllReactionSpecs()` 的轻量运营端点，可作为**同源旁证**（reactions_endpoint_test.go:116-128 证明三字段也在其白名单内），但**不是 Wiki 的权威读数来源**，本轮对账以 /config 为准。

## 2 纯显示辅助模块（新增 src 模块，真实导入）

- 新增 laiyipao/admin/src/reactionMechanicsText.ts（与 laiyipao/admin/src/reactions.ts 同级），导出：
  - export const MECHANISM_EMPTY = "—"；
  - export function spreadStacksText(v: number | null | undefined): string → 正整数"传播 N 层"，否则 MECHANISM_EMPTY；
  - export function armorShredText(v: number | null | undefined): string → 正数 N‰（千分比原值），否则 MECHANISM_EMPTY；
  - export function knockbackText(v: number | null | undefined): string → 正数 Npx（值 / 1000），否则 MECHANISM_EMPTY。
- 纯函数、无副作用、无 I/O、不引第三方；不读全局配置、不写 localStorage。WikiView 通过 @/reactionMechanicsText 真实 import 并在三个 el-table-column 的 #default 槽调用，禁止在测试里内嵌参考实现（本轮契约测试必须先因模块不存在而 RED）。

## 3 WikiView.vue 本地类型与反应表列契约

- 本地 reactions 数组元素类型补齐三字段（snake_case，与 Go json tag 逐字一致）：spread_stacks: number; armor_shred_permille: number; knockback: number，同时保留既有 key/name/base_coef/attack_weight_pct/status_duration_ms/aoe_radius/dispel_shield/amplify_pct/descr；顺序与服务端一致便于逐字段对账。
- 反应表在既有 8 列基础上新增 3 列，插在溅射半径与说明之间：
  - <el-table-column label="层数传播" width="90"><template #default=`{ row }`>{{ spreadStacksText(row.spread_stacks) }}</template></el-table-column>
  - <el-table-column label="削甲" width="80"> … {{ armorShredText(row.armor_shred_permille) }} … </el-table-column>
  - <el-table-column label="击退" width="80"> … {{ knockbackText(row.knockback) }} … </el-table-column>
- 既有 8 列（含 prop=descr 说明列）、statusText(row.key, row.status_duration_ms)、amplifyPctText(row.amplify_pct)、行 :data-focus 聚焦钩子、#empty emptyText 空态全部原样保留；不得删除说明列或改既有文案函数语义。
- 列宽纪律（:762-765）：新增列不得把桌面正文挤出横向页面溢出；沿用 el-table border size=small 与既有 min-width/--table-min 内部表滚动策略，横向滚动仅发生在表内，页面级 documentElement.scrollWidth 在 375/390 视口不得超视口。

## 4 状态说明文案修订（诚实口径）

- 删除 :831-833 陈旧表述"仍**没有特殊效果**的只剩两条（燃烧云的火区 / 腐蚀扩散的层数传播）"及过时的"README 已知边界第 19 条"引用。
- 改为逐字语义：腐蚀扩散的**层数传播已实现**（传播 1 层、半径 120px、不造成伤害、不触发新反应），新增层数传播 / 削甲 / 击退三列显示的是**已实现机制的真实参数**；**燃烧云 burn_cloud 的火区仍未实现**（C1 产品阻塞，本轮不实现），wiki 不得暗示其已生效。
- 保留溅射半径说明：蒸汽爆发 120 / 过热 90 已接真实范围伤害；"单体"表示无溅射；"攻击力权重 ≤300‰（30%）是结构性限制"的说明原样保留（:835-836）。

## 5 三端不回退与写域

- 目录 / 搜索 / 深链保留：SECTIONS（:108-134）与 anchors id=reactions（:113）、targetFromQuery（:166-190）、urlFor（:220）、scrollToAnchor（:253-257）、scrollToFocus（:261-265）、searchIndex（:408）、searchMatches（:494）、pick（:517-520）、URL ?tab=&anchor=&focus= 全部不动；本轮只动 basics 页签内反应表。
- 布局：桌面 / 375 / 390 三档页面级无横向溢出、表内可横向滚动、.wiki-body 的 :data-focus 聚焦钩子不丢；tokens 三锚点 --lyp-token-version: lyp-tokens-v1、--lyp-r3-version: lyp-tokens-v3-r3、--lyp-r6-version: lyp-tokens-r6 原样保留，禁止新增 --r31-。
- 不改 laiyipao/server/**、不改 laiyipao/miniapp/src/game/engine.ts、不改 replay_hash/score/结算/loot、不改 laiyipao/admin/src/api/types.gen.ts（生成文件真源在服务端）、不改其它 admin 视图。
- 实施任务写域仅 laiyipao/admin/src/views/WikiView.vue 与新增 laiyipao/admin/src/reactionMechanicsText.ts；本规范任务只写本 SPEC 与 laiyipao/admin/tests/WikiReactionMechanics.test.ts，不做 git。

## 6 契约测试与 RED→GREEN、验收门禁

- 契约测试 laiyipao/admin/tests/WikiReactionMechanics.test.ts **真实 import** ../src/reactionMechanicsText 并对三 helper 做单元断言（传播层数 / 千分比削甲 / 定点击退换算 / 零值与缺失空态），另做源码守卫：本地类型含三字段、反应表三列与 helper 调用、陈旧文案消失、SECTIONS/深链/搜索/令牌/空态/说明列不回退、服务端白名单与生成类型含三字段、零真值。
- RED（本轮，模块不存在）：workdir=laiyipao/admin 下 npx vitest run --config vitest.config.ts tests/WikiReactionMechanics.test.ts --reporter=verbose → exit 1，Cannot find module "../src/reactionMechanicsText" / ERR_MODULE_NOT_FOUND，Test Files 1 failed、Tests no tests。
- GREEN（实施后）：同命令全绿 exit 0；再跑 admin 全量 npx vitest run、type-check、build 全绿；真浏览器打开 wiki，以页面实际读取的公开配置端点 /config（/api/v1/config，WikiView.vue:619 auth:false）为**权威逐字段对账**：三列值与其中 Reactions 条目的 spread_stacks / armor_shred_permille / knockback 逐条一致；/api/v1/admin/reactions 仅作同源旁证核对，不作为对账基准。零值显示"—"，陈旧文案消失，桌面 / 375 / 390 页面级无横向溢出（表内可滚动），截图与对账记录落审计目录。
- 零真值：SPEC 与测试不得含 JWT / 真实域名 / IP / 口令 / 本机绝对路径；真值只从公开配置端点 /config（/api/v1/config，与 WikiView.vue:619 实际调用一致）与本仓库服务端真源（service.go:1638 domain.AllReactionSpecs()）读取。