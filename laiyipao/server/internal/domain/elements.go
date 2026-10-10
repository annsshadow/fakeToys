// Package domain 存放纯业务规则：不触碰数据库、不依赖 Fiber，可直接单元测试。
//
// ⚠️ 双端公式一致性契约
//
// elements.go / power.go / drop.go 中的公式在 Go（服务端校验）与 TS（客户端渲染与结算）
// 各实现一次，两边必须同输入同输出。改动任一公式必须同时改两处并更新
// server/testdata/formula_vectors.json，否则集成测试 formula_consistency_test.go 会失败。
package domain

// Element 五种元素。I-1 的全部机制建立在此之上。
type Element string

const (
	ElementFire      Element = "fire"      // 焰
	ElementIce       Element = "ice"       // 冰
	ElementLightning Element = "lightning" // 电
	ElementCorrosion Element = "corrosion" // 毒
	ElementKinetic   Element = "kinetic"   // 动能
)

// AllElements 返回固定顺序的五元素，供遍历与展示使用。
// 顺序必须与 TS 侧 elements.ts 的 ELEMENT_ORDER 保持一致（哈希与覆盖率计算依赖它）。
func AllElements() []Element {
	return []Element{ElementFire, ElementIce, ElementLightning, ElementCorrosion, ElementKinetic}
}

// IsElement 判断字符串是否为合法元素标识。
func IsElement(s string) bool {
	switch Element(s) {
	case ElementFire, ElementIce, ElementLightning, ElementCorrosion, ElementKinetic:
		return true
	}
	return false
}

// ReactionKey 标识一条反应链。
type ReactionKey string

const (
	ReactionSteamBurst     ReactionKey = "steam_burst"      // 焰+冰：范围伤害 + 驱散护盾
	ReactionOverheat       ReactionKey = "overheat"         // 焰+电：爆炸 + 眩晕
	ReactionBurnCloud      ReactionKey = "burn_cloud"       // 焰+毒：生成持续火区
	ReactionSuperconduct   ReactionKey = "superconduct"     // 冰+电：受击伤害 +60%
	ReactionFlashFreeze    ReactionKey = "flash_freeze"     // 冰+毒：冻结
	ReactionCorrosionSpray ReactionKey = "corrosion_spread" // 电+毒：元素层数向周围传播
	ReactionArmorBreak     ReactionKey = "armor_break"      // 动能+任意：击退 + 削护甲
)

// AllReactions 返回固定顺序的七条反应链，顺序与 TS 侧一致。
func AllReactions() []ReactionKey {
	return []ReactionKey{
		ReactionSteamBurst,
		ReactionOverheat,
		ReactionBurnCloud,
		ReactionSuperconduct,
		ReactionFlashFreeze,
		ReactionCorrosionSpray,
		ReactionArmorBreak,
	}
}

// reactionTable 定义"已附着元素 A + 新施加元素 B"触发的反应。
// 反应不可交换：B 触发时 A 已经在目标身上，顺序不同结果不同（例如焰后冰是蒸汽爆发，
// 冰后焰也是蒸汽爆发但焰后电是过热、电后焰不触发任何反应——
// 这正是"时序"成为构筑变量来源的原因）。
var reactionTable = map[Element]map[Element]ReactionKey{
	ElementFire: {
		ElementIce:       ReactionSteamBurst,
		ElementLightning: ReactionOverheat,
		ElementCorrosion: ReactionBurnCloud,
		ElementKinetic:   ReactionArmorBreak,
	},
	ElementIce: {
		ElementFire:      ReactionSteamBurst,
		ElementLightning: ReactionSuperconduct,
		ElementCorrosion: ReactionFlashFreeze,
		ElementKinetic:   ReactionArmorBreak,
	},
	ElementLightning: {
		ElementFire:      ReactionOverheat,
		ElementIce:       ReactionSuperconduct,
		ElementCorrosion: ReactionCorrosionSpray,
		ElementKinetic:   ReactionArmorBreak,
	},
	ElementCorrosion: {
		ElementFire:      ReactionBurnCloud,
		ElementIce:       ReactionFlashFreeze,
		ElementLightning: ReactionCorrosionSpray,
		ElementKinetic:   ReactionArmorBreak,
	},
	// 动能只能作为"后手"元素触发破甲：先有任意元素，再来动能
	ElementKinetic: {
		ElementFire:      ReactionArmorBreak,
		ElementIce:       ReactionArmorBreak,
		ElementLightning: ReactionArmorBreak,
		ElementCorrosion: ReactionArmorBreak,
	},
}

// ReactionSpec 描述一条反应链的数值效果。
//
// ⚠️ 带 snake_case json tag —— /config 会把它下发给客户端与文档站。
// 漏掉 tag 会让客户端读到 PascalCase 字段，文档站的反应表整列为空。
type ReactionSpec struct {
	Key ReactionKey `json:"key"`
	// Name 中文名，用于后台统计与诊断文案。
	Name string `json:"name"`
	// BaseCoef 反应基础系数（千分比，定点数）。乘以元素层数与反应等级得到反应伤害。
	BaseCoef int `json:"base_coef"`
	// AttackWeightPct 反应伤害中由玩家攻击力贡献的比例（千分比）。
	// 硬性红线：该值不得超过 300（即 30%），这是反数值通胀的机制保证。
	AttackWeightPct int `json:"attack_weight_pct"`
	// StatusDurationMs 附加状态时长（0 表示无状态）
	StatusDurationMs int `json:"status_duration_ms"`
	// AoeRadius 溅射半径（像素，0 表示单体）
	AoeRadius int `json:"aoe_radius"`
	// DispelShield 是否驱散护盾
	DispelShield bool `json:"dispel_shield"`
	// AmplifyPct 对目标造成的"受击伤害放大"比例（千分比，0 表示无）
	AmplifyPct int `json:"amplify_pct"`
	// ArmorShredPermille 削甲量（千分比，0 表示无）。armor_break 专用：
	// 触发后目标护甲在 StatusDurationMs 内被削减本值。
	// 消费者：客户端引擎 `engine.ts` 的 hitEnemy（构建 Defender 时折进有效护甲）；
	// 服务端不跑引擎（I-5 取舍），所以只随 /config 下发 + 契约文件锁定，
	// 数值本身不进任何服务端公式。
	ArmorShredPermille int `json:"armor_shred_permille"`
	// Knockback 击退位移（定点 ×1000，0 表示无）。armor_break 专用：
	// 触发时目标一次性向右（远离防线）位移本值，客户端引擎消费。
	Knockback int `json:"knockback"`
	// Descr 面向玩家的效果说明。
	//
	// ⚠️ 第 87 轮更正：原文写「运营后台的玩法文档站直接展示它」——
	// **那是错的**。admin 的 `WikiView.vue` 确实有一张「反应链一览」表，
	// 但它的本地类型里**没有 descr**，表里也没有「说明」列，
	// 所以这个字段在那一页被**丢弃**了。
	//
	// 连带后果：第 83 轮修正的 5 条错误文案（范围伤害 / 爆炸 / 火区 /
	// 传播 / 击退削甲）**一个都看不到** —— 我当时把它们当成「对外文案」，
	// 实际它们只躺在 JSON 里。
	//
	// 本轮把「说明」列补上（与敌人 / 技能 / 复合技能三张表一致），
	// 这条注释才重新成立。
	//
	// 为什么必须由服务端下发而不是后台自己抄一份：
	// 抄的那份迟早与数值表脱节，而**脱节时没有任何测试会发现** ——
	// 两端各自都「自洽」。
	Descr string `json:"descr"`
}

// reactionSpecs 反应链数值表。AttackWeightPct 全部 ≤ 300，符合 GAME_DESIGN I-1 红线。
//
// ⚠️ 这张表与 miniapp/src/game/elements.ts 的 REACTIONS 常量是**两份手工同步的副本**，
// 而它们直接决定伤害。任何一边单方面改动都会让两端算出不同的伤害，
// 而 I-6 的回放哈希会随之失配（表现为"所有人都验不出真伪"）。
// 两份副本由 testdata/reaction_specs.json 双向锁住，
// 改动任何一侧都必须同步另一侧并重新生成契约文件。
//
// ⚠️ 本轮接通 `steam_burst` 的范围伤害（aoeRadius 120，客户端引擎以命中
// 敌人为圆心、半径内邻居受 25% 反应伤害）。余下 **3 条反应仍無特殊效果**：
// overheat 的爆炸、burn_cloud 的持续火区、corrosion_spread 的层数传播。
//
// # I-6 安全性（为什么「仅实现 steam_burst AoE 就不摇旗»
//
// 服务端 replay_hash 仅封 S 段（构筑快照），**不重算事件流**
// （battle_collections.go: replay_hash 只封长 ≤ 64，store as-opaque）；
// I-6 的 server-side 校验是 `replay_skills` 段的相等。
// 因此 steam_burst 的溅射事件只需「客户端局内哈希稳定」，
// 不需要服务端独立验证 —— 它不进入 replay_skills，因而不影响 I-6。
//
// # 口采一致性
//
// 双端唯一要一致的是「溅射伤害」本身：`reactionDmg >> 2`。
// `reactionDmg` 出自 `resolveHit`，由 formula_vectors.json 锁定逐位一致
// （攻击侧上限 cap /= reactionMultPermille，专精 reaction_mult 未满 800‰
// 恒 `< MAX_REACTION_ATTACK_WEIGHT`，所以 cap 与旧式等价）。
// 溅射后半径 `aoeRadius` 与邻居集合（刷怪洗牌 + 坐标）皆由种子决定、
// 全定点 —— 不进入 server 校验范围，也不漂移。
//
// 逐字段核过消费面（`resolveHit` + 服务端重算）：
//
//	baseCoef / attackWeightPct / statusDurationMs / dispelShield /
//	amplifyPct / armorShredPermille / knockback               ✓
//	aoeRadius                                                      ✓（steam_burst：客户端溅射）
//

var reactionSpecs = map[ReactionKey]ReactionSpec{
	ReactionSteamBurst: {
		Key: ReactionSteamBurst, Name: "蒸汽爆发", BaseCoef: 60, AttackWeightPct: 300,
		// ⚠️ 2026-10-10：0 -> 120，实现范围伤害。
		// 客户端 `engine.ts triggerSteamBurstAoe` 以命中敌人为圆心、半径
		// aoeRadius 内的邻居造成 reactionDmg >> 2（25%）二次伤害，
		// 不再是「屏幕震动谎称爆炸」。I-6 安全：溅射伤害来自
		// resolveHit（formula_vectors.json 锁定），不进入 replay_skills 段。
		AoeRadius: 120, DispelShield: true, Descr: "驱散护盾并造成反应伤害（触发时以自身为圆心，半径 120 像素内的敌人受到 25% 范围伤害）",
	},
	ReactionOverheat: {
		Key: ReactionOverheat, Name: "过热", BaseCoef: 55, AttackWeightPct: 300,
		AoeRadius: 0, StatusDurationMs: 1500, Descr: "眩晕 1.5 秒并造成反应伤害",
	},
	ReactionBurnCloud: {
		// ⚠️ 第 83 轮：这条反应的**整条文案都是未实现的**。
		// statusDurationMs=0 -> 无燃烧状态；AoeRadius 未被消费 -> 无火区。
		// 唯一的实现是「一个系数较低的应伤害」（baseCoef 40，七条里第二低）。
		Key: ReactionBurnCloud, Name: "燃烧云", BaseCoef: 40, AttackWeightPct: 250,
		AoeRadius: 0, Descr: "造成反应伤害",
	},
	ReactionSuperconduct: {
		Key: ReactionSuperconduct, Name: "超导", BaseCoef: 50, AttackWeightPct: 250,
		AmplifyPct: 600, StatusDurationMs: 4000, Descr: "受击伤害 +60%",
	},
	ReactionFlashFreeze: {
		Key: ReactionFlashFreeze, Name: "急速冻结", BaseCoef: 45, AttackWeightPct: 250,
		StatusDurationMs: 2000, AmplifyPct: 300, Descr: "冻结并提高受击伤害",
	},
	ReactionCorrosionSpray: {
		Key: ReactionCorrosionSpray, Name: "腐蚀扩散", BaseCoef: 35, AttackWeightPct: 200,
		AoeRadius: 0, Descr: "造成反应伤害",
	},
	ReactionArmorBreak: {
		// 第 83 轮时「击退」与「削减护甲」都未实现，该条只有反应伤害，
		// 文案改成诚实描述。本轮把两个效果实现出来（客户端引擎消费）：
		// 削甲 ArmorShredPermille=150 持续 StatusDurationMs=4000ms
		// （引擎 hitEnemy 折进有效护甲），击退 Knockback=4000 定点
		// （一次性右推，stepEnemyMotion 的 knockback 分支消费后清零）。
		Key: ReactionArmorBreak, Name: "破甲击退", BaseCoef: 30, AttackWeightPct: 300,
		StatusDurationMs: 4000, ArmorShredPermille: 150, Knockback: 4000,
		Descr: "击退目标并削减其护甲 4 秒（150‰），同时造成反应伤害",
	},
}

// Spec 返回反应链的数值规格。
func (k ReactionKey) Spec() ReactionSpec {
	if s, ok := reactionSpecs[k]; ok {
		return s
	}
	return ReactionSpec{}
}

// AllReactionSpecs 按固定顺序返回全部反应规格。
func AllReactionSpecs() []ReactionSpec {
	out := make([]ReactionSpec, 0, len(reactionSpecs))
	for _, k := range AllReactions() {
		out = append(out, reactionSpecs[k])
	}
	return out
}

// LookupReaction 返回"已附着元素 A + 新施加元素 B"触发的反应。
// 第二个返回值为 false 表示该组合不触发任何反应。
func LookupReaction(existing, incoming Element) (ReactionKey, bool) {
	if existing == "" || incoming == "" || existing == incoming {
		return "", false
	}
	byIncoming, ok := reactionTable[incoming]
	if !ok {
		return "", false
	}
	key, ok := byIncoming[existing]
	return key, ok
}
