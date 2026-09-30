package domain

// 游戏内容定义（全部原创命名，未参考或复制任何既有作品）。
//
// 这些表是服务端与客户端的共同数据源：服务端落库后经 /api/v1/config 下发，
// 客户端不硬编码任何数值 —— 改平衡只需改这里 + 跑一次种子。
//
// ⚠️ 所有 struct 都带 snake_case json tag。这不是可选的风格问题：
// Go 在没有 tag 时会按字段名原文（PascalCase）序列化，而客户端类型定义
// 用的是 snake_case —— 两边对不上，客户端读到的每个字段都是 undefined，
// 表现为「战斗里一个敌人都没有」。这个坑真实发生过，加 tag 时不要漏。

// SeedEnemy 是敌人定义。
type SeedEnemy struct {
	ID          int    `json:"id"`
	Code        string `json:"code"`
	Name        string `json:"name"`
	Category    string `json:"category"` // normal/ranged/flying/special/elite/boss
	HP          int64  `json:"hp"`
	Speed       int64  `json:"speed"` // 定点 x1000，像素/秒
	Armor       int64  `json:"armor"`
	ShieldHP    int64  `json:"shield_hp"`
	Attack      int64  `json:"attack"`
	AttackRange int64  `json:"attack_range"`
	AttackEvery int64  `json:"attack_interval"` // 攻击间隔毫秒
	FlyHeight   int64  `json:"fly_height"`
	Burrow      bool   `json:"burrow"`
	IsBoss      bool   `json:"is_boss"`
	// Resist 五维抗性（千分比，-500..500）。BOSS 必须同时有高抗与负抗。
	Resist map[Element]int64 `json:"resist"`
	Descr  string            `json:"descr"`
}

// SeedSkills 是技能定义。24 个技能分 8 系，每系 3 个。
type SeedSkill struct {
	ID              int     `json:"id"`
	Code            string  `json:"code"`
	Name            string  `json:"name"`
	Family          string  `json:"family"`
	Element         Element `json:"element"`
	Kind            string  `json:"kind"` // active/passive
	Descr           string  `json:"descr"`
	BaseDamage      int64   `json:"base_damage"`
	HeatCost        int64   `json:"heat_cost"`
	CooldownMs      int64   `json:"cooldown_ms"`
	Pierce          int64   `json:"pierce"`
	AoeRadius       int64   `json:"aoe_radius"`
	ApplyElement    Element `json:"apply_element"`
	ApplyStacks     int64   `json:"apply_stacks"`
	ProjectileSpeed int64   `json:"projectile_speed"` // 定点 x1000
	Chain           int64   `json:"chain"`
	UnlockLevel     int     `json:"unlock_level"`
}

// SeedRecipe 是合成配方：A + B → output。
type SeedRecipe struct {
	Output  int `json:"output"`
	A       int `json:"a"`
	B       int `json:"b"`
	OutTier int `json:"out_tier"`
}

// SeedEquipment 是装备定义。6 部位 × 3 阶 = 18 件。
type SeedEquipment struct {
	ID           int     `json:"id"`
	Code         string  `json:"code"`
	Name         string  `json:"name"`
	Slot         string  `json:"slot"` // weapon/helmet/armor/gloves/boots/charm
	Tier         int     `json:"tier"`
	Element      Element `json:"element"` // 契合标签：与技能同系给大加成，异系给小额
	Descr        string  `json:"descr"`
	BaseArmor    int64   `json:"base_armor"`
	BaseBonusPct int64   `json:"base_bonus_pct"`
	UnlockLevel  int     `json:"unlock_level"`
}

// SeedGem 是宝石定义。8 属性。
type SeedGem struct {
	ID    int    `json:"id"`
	Code  string `json:"code"`
	Name  string `json:"name"`
	Attr  string `json:"attr"`
	Descr string `json:"descr"`
}

// GemQuality 五档品质。
var GemQualities = []struct {
	Name  string `json:"name"`
	Count int    `json:"affix_count"` // 该品质随机词条条数
	Mult  int64  `json:"mult"`
}{
	{"gray", 1, 1000},
	{"blue", 1, 1200},
	{"purple", 2, 1450},
	{"red", 2, 1700},
	{"gold", 3, 2000},
}

// SeedSkin 是皮肤定义。被动为机制型词条，改变机制而非只加数值。
type SeedSkin struct {
	ID      int    `json:"id"`
	Code    string `json:"code"`
	Name    string `json:"name"`
	Rarity  string `json:"rarity"`
	Passive string `json:"passive"`
	Descr   string `json:"descr"`
}

// --- 敌人 ---

// SeedEnemies 全部敌人定义。ID 1..22，与 levelgen 的引用一致。
// --- 内容表平衡缩放 ---
//
// 为什么不直接改 SeedEnemies 里的数值：
// 数据表里 22 个敌人 / 42 个技能的数值是**可读的基准值**，
// 而"该缩放多少倍"是一个会随实测反复调整的参数。
// 两者混在一起时，每次调平衡都要改几十个数字，
// 而且看不出某个数字是"设计值"还是"被缩放过的值"。
// 所以：表里存基准，缩放集中在这里，导出时统一应用。
//
// ⚠️ 缩放必须在**导出到客户端之前**应用，且只此一处。
// Go 端不跑战斗引擎（引擎在 miniapp），所以只要保证
// /config 与 testdata/* 导出的是缩放后的值即可。
// 若将来 Go 端也开始模拟战斗（I-5 防线值守），
// 这里的调用点必须同步补上，否则两端会用不同数值。

// EnemyHpScale 是敌人血量倍率。
//
// 取 8 的依据（`miniapp/src/game/balance.probe.test.ts` 的定档扫描，
// 漏怪伤害修复之后，弹丸速度 ×4 的前提下）：
//
//	hp×4  194s  全清零漏  hp 100%  atk +0.0%  crit +0.2%   ← 太软，成长维度无梯度
//	hp×8  194s  全清零漏  hp 100%  atk +0.0%  crit +0.2%
//	hp×20 283s  杀34漏5   hp  50%  atk +3.0%  crit +11.9%  ← 有压力
//	hp×24 304s  杀31漏8   hp  20%  atk +13.0% crit +24.1%
//	hp×28 306s  失败
//
// 没取 ×20 是因为它是**第 1 关**的合适值，而第 1 关（5 波 39 怪）
// 是全 100 关里最简单的 —— 难度梯度还要靠波次数（5→10）与
// base_hp（1000→10000）继续往上走。×8 保守，留出后期调整空间；
// 跨关卡的难度递增由 TestLevelDifficultyRisesAcrossChapters 守住。
//
// ⚠️ 原始倍率下玩家**打不动**：单次命中 254 伤害 vs 敌人 hp 70~260，
// 一击就超过敌人血量，于是"多打一发"没有意义 ——
// 攻击力 ×200 之后完全饱和、暴击率 0→1000‰ 只差 0.02%、
// 元素反应随攻击力单调归零。整套养成维度被压平，I-1/I-2 感受不到。
const EnemyHpScale = 8

// SkillProjectileScale 是弹丸速度倍率。
//
// 取 4 的依据：内容表 projectile_speed 是 45~80 逻辑单位/秒，
// 跨越 900 单位宽的场地要 10~20 秒，而弹丸瞄的是**发射瞬间**的敌人位置。
// 敌人 15 秒走 400~1100 单位，弹丸几乎必然落在目标身后。
// 实测命中率只有 15% —— 85% 的输出被浪费。
//
//	×1  命中 15%  338s  失败
//	×2  命中 30%  193s  通关
//	×4  命中 76%  194s  通关   ← 选定
//	×8  命中 99%   77s  通关   ← 过于轻松，战斗失去张力
//
// 命中率 ×4 后是 76%，仍有约 1/4 的弹丸落空：
// 移动目标上"打不中"本身是玩法的一部分，全自动命中反而无趣。
const SkillProjectileScale = 4

// ScaleEnemy 返回应用了血量缩放的敌人定义。
func ScaleEnemy(e SeedEnemy) SeedEnemy {
	e.HP *= EnemyHpScale
	e.ShieldHP *= EnemyHpScale
	return e
}

// ScaleSkill 返回应用了弹丸速度缩放的技能定义。
func ScaleSkill(s SeedSkill) SeedSkill {
	s.ProjectileSpeed *= SkillProjectileScale
	return s
}

// ScaleAllEnemies 返回缩放后的全部敌人。
func ScaleAllEnemies() []SeedEnemy {
	out := make([]SeedEnemy, 0, len(SeedEnemies))
	for _, e := range SeedEnemies {
		out = append(out, ScaleEnemy(e))
	}
	return out
}

// ScaleAllSkills 返回缩放后的全部基础技能。
func ScaleAllSkills() []SeedSkill {
	out := make([]SeedSkill, 0, len(SeedSkills))
	for _, s := range SeedSkills {
		out = append(out, ScaleSkill(s))
	}
	return out
}

// ScaleAllCompositeSkills 返回缩放后的全部合成技能。
func ScaleAllCompositeSkills() []SeedSkill {
	out := make([]SeedSkill, 0, len(SeedCompositeSkills))
	for _, s := range SeedCompositeSkills {
		out = append(out, ScaleSkill(s))
	}
	return out
}

var SeedEnemies = []SeedEnemy{
	// 普通 8
	{ID: 1, Code: "wanderer", Name: "游荡者", Category: "normal", HP: 100, Speed: 40000, Descr: "最常见的感染者，移动缓慢但数量多。",
		Resist: map[Element]int64{ElementFire: 0, ElementIce: 0, ElementLightning: 0, ElementCorrosion: 100, ElementKinetic: -100}},
	{ID: 2, Code: "sprinter", Name: "疾行者", Category: "normal", HP: 70, Speed: 75000, Descr: "速度极快，优先拦截。",
		Resist: map[Element]int64{ElementFire: 100, ElementIce: 0, ElementLightning: 200, ElementCorrosion: 0, ElementKinetic: -200}},
	{ID: 3, Code: "brute", Name: "壮硕体", Category: "normal", HP: 260, Speed: 30000, Armor: 50, Descr: "皮糙肉厚，需要破甲。",
		Resist: map[Element]int64{ElementFire: 0, ElementIce: 0, ElementLightning: 0, ElementCorrosion: 200, ElementKinetic: 0}},
	{ID: 4, Code: "rottweiler", Name: "腐化犬", Category: "normal", HP: 90, Speed: 60000, Descr: "成群扑向防线，咬合力惊人。",
		Resist: map[Element]int64{ElementFire: 200, ElementIce: 0, ElementLightning: 0, ElementCorrosion: -200, ElementKinetic: 0}},
	{ID: 5, Code: "stoneskin", Name: "石肤兽", Category: "normal", HP: 400, Speed: 22000, Armor: 150, Descr: "外骨骼覆盖，常规伤害难以奏效。",
		Resist: map[Element]int64{ElementFire: 300, ElementIce: 0, ElementLightning: 300, ElementCorrosion: 0, ElementKinetic: -300}},
	{ID: 6, Code: "voltfin", Name: "电鳍兽", Category: "normal", HP: 160, Speed: 45000, Attack: 8, AttackRange: 150, AttackEvery: 2000, Descr: "会放电反击近身单位。",
		Resist: map[Element]int64{ElementFire: 0, ElementIce: 0, ElementLightning: 500, ElementCorrosion: 0, ElementKinetic: 0}},
	{ID: 7, Code: "frostbit", Name: "霜蚀者", Category: "normal", HP: 200, Speed: 35000, Descr: "自身体表覆霜，冰系打它收益极低。",
		Resist: map[Element]int64{ElementFire: -300, ElementIce: 500, ElementLightning: 0, ElementCorrosion: 0, ElementKinetic: 0}},
	{ID: 8, Code: "boneeater", Name: "蚀骨者", Category: "normal", HP: 320, Speed: 28000, Armor: 80, Descr: "专吃有元素层数的目标。",
		Resist: map[Element]int64{ElementFire: 100, ElementIce: 100, ElementLightning: 100, ElementCorrosion: -400, ElementKinetic: 0}},

	// 远程 4
	{ID: 9, Code: "pitcher", Name: "投手", Category: "ranged", HP: 120, Speed: 25000, Attack: 10, AttackRange: 400, AttackEvery: 2500,
		Descr:  "远程投掷减速弹，会拖慢防线节奏。",
		Resist: map[Element]int64{ElementFire: 0, ElementIce: 200, ElementLightning: 0, ElementCorrosion: 0, ElementKinetic: 0}},
	{ID: 10, Code: "cannon", Name: "铁罐炮", Category: "ranged", HP: 260, Speed: 18000, Attack: 28, AttackRange: 500, AttackEvery: 3200, ShieldHP: 60,
		Descr:  "直线重炮，射程远且带小护盾。",
		Resist: map[Element]int64{ElementFire: 0, ElementIce: 0, ElementLightning: 200, ElementCorrosion: 0, ElementKinetic: -200}},
	{ID: 11, Code: "hivemother", Name: "蜂巢母", Category: "ranged", HP: 500, Speed: 12000, Attack: 6, AttackRange: 350, AttackEvery: 1800,
		Descr:  "持续召唤小型单位。",
		Resist: map[Element]int64{ElementFire: 200, ElementIce: 0, ElementLightning: 0, ElementCorrosion: -200, ElementKinetic: 0}},
	{ID: 12, Code: "seeker", Name: "索敌眼", Category: "ranged", HP: 180, Speed: 20000, Attack: 0, AttackRange: 600, AttackEvery: 4000,
		Descr:  "标记目标，使其受到的伤害提高。",
		Resist: map[Element]int64{ElementFire: 0, ElementIce: 0, ElementLightning: 0, ElementCorrosion: 0, ElementKinetic: 300}},

	// 飞行 3（直线弹道打不到，必须用溅射或高能射线）
	{ID: 13, Code: "floater", Name: "浮空囊", Category: "flying", HP: 110, Speed: 50000, FlyHeight: 180, Descr: "悬浮前进，躲过直线弹道。",
		Resist: map[Element]int64{ElementFire: 0, ElementIce: 0, ElementLightning: 0, ElementCorrosion: 100, ElementKinetic: -300}},
	{ID: 14, Code: "winged", Name: "翼化体", Category: "flying", HP: 200, Speed: 85000, FlyHeight: 220, Descr: "高速俯冲，最难处理的一类。",
		Resist: map[Element]int64{ElementFire: 0, ElementIce: 200, ElementLightning: 0, ElementCorrosion: 0, ElementKinetic: -200}},
	{ID: 15, Code: "scoutdrone", Name: "侦察机", Category: "flying", HP: 90, Speed: 60000, FlyHeight: 300, Attack: 12, AttackRange: 450, AttackEvery: 2000,
		Descr:  "高空侦察并回传情报，会强化周围敌人。",
		Resist: map[Element]int64{ElementFire: 200, ElementIce: 0, ElementLightning: -300, ElementCorrosion: 0, ElementKinetic: 0}},

	// 精英 / BOSS
	{ID: 16, Code: "ironjaw", Name: "铁颚精英", Category: "elite", HP: 3000, Speed: 20000, Armor: 200, ShieldHP: 800, IsBoss: true,
		Descr:  "冻原领主。高抗冰与毒，但极怕电火。",
		Resist: map[Element]int64{ElementFire: -400, ElementIce: 500, ElementLightning: -400, ElementCorrosion: 500, ElementKinetic: 0}},

	{ID: 17, Code: "burrower", Name: "潜地者", Category: "special", HP: 240, Speed: 55000, Burrow: true,
		Descr:  "钻入地下，无视直线弹道，只能靠溅射伤害。",
		Resist: map[Element]int64{ElementFire: 0, ElementIce: 0, ElementLightning: 0, ElementCorrosion: 300, ElementKinetic: -400}},
	{ID: 18, Code: "shieldbearer", Name: "护盾卫", Category: "special", HP: 400, Speed: 26000, Armor: 100, ShieldHP: 600,
		Descr:  "护盾极厚，只有蒸汽爆发能驱散。",
		Resist: map[Element]int64{ElementFire: 0, ElementIce: 200, ElementLightning: 200, ElementCorrosion: 200, ElementKinetic: 0}},

	{ID: 19, Code: "hiveguard", Name: "母巢守卫", Category: "boss", HP: 12000, Speed: 16000, Armor: 250, ShieldHP: 3000, Attack: 35, AttackRange: 400, AttackEvery: 2600, IsBoss: true,
		Descr:  "母巢外围守门者。高抗毒，必须用焰+冰。",
		Resist: map[Element]int64{ElementFire: -500, ElementIce: -500, ElementLightning: 300, ElementCorrosion: 500, ElementKinetic: 0}},
	{ID: 20, Code: "blightenvoy", Name: "腐化使", Category: "elite", HP: 4000, Speed: 30000, Armor: 150,
		Descr:  "持续给周围敌人叠加腐蚀层数。",
		Resist: map[Element]int64{ElementFire: 0, ElementIce: 300, ElementLightning: 0, ElementCorrosion: -500, ElementKinetic: 0}},
	{ID: 21, Code: "corenode", Name: "蚀核", Category: "boss", HP: 20000, Speed: 12000, Armor: 300, ShieldHP: 5000, Attack: 45, AttackRange: 450, AttackEvery: 2200, IsBoss: true,
		Descr:  "母巢的能量核心。高抗动能，需要焰系破防。",
		Resist: map[Element]int64{ElementFire: -500, ElementIce: 200, ElementLightning: 200, ElementCorrosion: 400, ElementKinetic: 500}},
	{ID: 22, Code: "chalkqueen", Name: "白垩母皇", Category: "boss", HP: 40000, Speed: 10000, Armor: 350, ShieldHP: 8000, Attack: 60, AttackRange: 500, AttackEvery: 2000, IsBoss: true,
		Descr:  "最终首领。对冰与电几乎免疫，但惧怕腐蚀与动能。",
		Resist: map[Element]int64{ElementFire: 200, ElementIce: 500, ElementLightning: 500, ElementCorrosion: -500, ElementKinetic: -500}},
}

// --- 技能（8 系 × 3 = 24） ---

var SeedSkills = []SeedSkill{
	// 焰系
	{ID: 1, Code: "ember", Name: "燃烧弹", Family: "flame", Element: ElementFire, Kind: "active",
		Descr: "基础火球，命中施加 1 层焰。", BaseDamage: 100, HeatCost: 20, CooldownMs: 800,
		Pierce: 0, AoeRadius: 60, ApplyElement: ElementFire, ApplyStacks: 1, ProjectileSpeed: 60000, Chain: 0, UnlockLevel: 1},
	{ID: 2, Code: "napalm", Name: "燃油弹", Family: "flame", Element: ElementFire, Kind: "active",
		Descr: "大范围燃烧，施加 2 层焰。", BaseDamage: 140, HeatCost: 30, CooldownMs: 1400,
		Pierce: 0, AoeRadius: 140, ApplyElement: ElementFire, ApplyStacks: 2, ProjectileSpeed: 50000, Chain: 0, UnlockLevel: 8},
	{ID: 3, Code: "blastshell", Name: "温压弹", Family: "flame", Element: ElementFire, Kind: "active",
		Descr: "高压弹丸，撞击后大范围爆炸。", BaseDamage: 180, HeatCost: 40, CooldownMs: 2000,
		Pierce: 1, AoeRadius: 180, ApplyElement: ElementFire, ApplyStacks: 2, ProjectileSpeed: 55000, Chain: 1, UnlockLevel: 20},

	// 冰系
	{ID: 4, Code: "dryice", Name: "干冰弹", Family: "frost", Element: ElementIce, Kind: "active",
		Descr: "命中减速并施加 1 层冰。", BaseDamage: 90, HeatCost: 18, CooldownMs: 700,
		Pierce: 1, AoeRadius: 0, ApplyElement: ElementIce, ApplyStacks: 1, ProjectileSpeed: 65000, Chain: 0, UnlockLevel: 3},
	{ID: 5, Code: "hail", Name: "冰雹加农", Family: "frost", Element: ElementIce, Kind: "active",
		Descr: "大范围冰雹，施加 2 层冰。", BaseDamage: 120, HeatCost: 32, CooldownMs: 1600,
		Pierce: 0, AoeRadius: 160, ApplyElement: ElementIce, ApplyStacks: 2, ProjectileSpeed: 45000, Chain: 0, UnlockLevel: 12},
	{ID: 6, Code: "glacier", Name: "极寒冰弹", Family: "frost", Element: ElementIce, Kind: "active",
		Descr: "贯穿一切的超低温弹，穿透 3 个目标。", BaseDamage: 200, HeatCost: 42, CooldownMs: 2200,
		Pierce: 3, AoeRadius: 0, ApplyElement: ElementIce, ApplyStacks: 3, ProjectileSpeed: 80000, Chain: 0, UnlockLevel: 26},

	// 电系
	{ID: 7, Code: "arc", Name: "电弧弹", Family: "volt", Element: ElementLightning, Kind: "active",
		Descr: "命中后弹射到附近目标。", BaseDamage: 110, HeatCost: 24, CooldownMs: 900,
		Pierce: 0, AoeRadius: 0, ApplyElement: ElementLightning, ApplyStacks: 1, ProjectileSpeed: 70000, Chain: 2, UnlockLevel: 6},
	{ID: 8, Code: "thunderlance", Name: "电磁穿刺", Family: "volt", Element: ElementLightning, Kind: "active",
		Descr: "高速穿刺，穿透并施加 2 层电。", BaseDamage: 150, HeatCost: 34, CooldownMs: 1500,
		Pierce: 2, AoeRadius: 0, ApplyElement: ElementLightning, ApplyStacks: 2, ProjectileSpeed: 90000, Chain: 1, UnlockLevel: 16},
	{ID: 9, Code: "stormgrid", Name: "雷暴链", Family: "volt", Element: ElementLightning, Kind: "active",
		Descr: "连环闪电，可弹射 4 次。", BaseDamage: 175, HeatCost: 44, CooldownMs: 2400,
		Pierce: 0, AoeRadius: 0, ApplyElement: ElementLightning, ApplyStacks: 2, ProjectileSpeed: 0, Chain: 4, UnlockLevel: 30},

	// 物理系
	{ID: 10, Code: "slug", Name: "重炮弹", Family: "kinetic", Element: ElementKinetic, Kind: "active",
		Descr: "高抛物重弹，动能伤害。", BaseDamage: 130, HeatCost: 22, CooldownMs: 1100,
		Pierce: 1, AoeRadius: 80, ApplyElement: ElementKinetic, ApplyStacks: 1, ProjectileSpeed: 40000, Chain: 0, UnlockLevel: 2},
	{ID: 11, Code: "shrapnel", Name: "破片弹", Family: "kinetic", Element: ElementKinetic, Kind: "active",
		Descr: "空中炸裂成破片，覆盖一片。", BaseDamage: 160, HeatCost: 36, CooldownMs: 1700,
		Pierce: 0, AoeRadius: 200, ApplyElement: ElementKinetic, ApplyStacks: 2, ProjectileSpeed: 50000, Chain: 0, UnlockLevel: 18},
	{ID: 12, Code: "siege", Name: "攻城锤", Family: "kinetic", Element: ElementKinetic, Kind: "active",
		Descr: "贯穿一切的重型动能弹。", BaseDamage: 260, HeatCost: 50, CooldownMs: 2800,
		Pierce: 4, AoeRadius: 0, ApplyElement: ElementKinetic, ApplyStacks: 3, ProjectileSpeed: 35000, Chain: 0, UnlockLevel: 34},

	// 光系
	{ID: 13, Code: "laser", Name: "聚焦激光", Family: "light", Element: ElementKinetic, Kind: "active",
		Descr: "瞬发光束，穿透直线上的所有敌人。", BaseDamage: 95, HeatCost: 26, CooldownMs: 1200,
		Pierce: 6, AoeRadius: 0, ApplyElement: ElementKinetic, ApplyStacks: 1, ProjectileSpeed: 0, Chain: 0, UnlockLevel: 10},
	{ID: 14, Code: "ray", Name: "高能射线", Family: "light", Element: ElementKinetic, Kind: "active",
		Descr: "高穿透光束，专治成群敌人。", BaseDamage: 145, HeatCost: 38, CooldownMs: 2000,
		Pierce: 8, AoeRadius: 0, ApplyElement: ElementKinetic, ApplyStacks: 1, ProjectileSpeed: 0, Chain: 0, UnlockLevel: 24},
	{ID: 15, Code: "prism", Name: "棱镜散射", Family: "light", Element: ElementFire, Kind: "active",
		Descr: "一束光分裂成多道，每道携带不同元素。", BaseDamage: 135, HeatCost: 40, CooldownMs: 2200,
		Pierce: 1, AoeRadius: 120, ApplyElement: ElementFire, ApplyStacks: 2, ProjectileSpeed: 75000, Chain: 3, UnlockLevel: 38},

	// 毒系
	{ID: 16, Code: "venom", Name: "腐蚀弹", Family: "blight", Element: ElementCorrosion, Kind: "active",
		Descr: "施加 2 层毒，可持续掉血。", BaseDamage: 85, HeatCost: 20, CooldownMs: 800,
		Pierce: 1, AoeRadius: 0, ApplyElement: ElementCorrosion, ApplyStacks: 2, ProjectileSpeed: 60000, Chain: 0, UnlockLevel: 4},
	{ID: 17, Code: "sprayer", Name: "喷射毒雾", Family: "blight", Element: ElementCorrosion, Kind: "active",
		Descr: "大范围毒雾覆盖，施加 3 层毒。", BaseDamage: 110, HeatCost: 30, CooldownMs: 1500,
		Pierce: 0, AoeRadius: 220, ApplyElement: ElementCorrosion, ApplyStacks: 3, ProjectileSpeed: 0, Chain: 0, UnlockLevel: 20},
	{ID: 18, Code: "plague", Name: "液化毒雾", Family: "blight", Element: ElementCorrosion, Kind: "active",
		Descr: "把目标的毒层数按一半传播给周围敌人。", BaseDamage: 130, HeatCost: 46, CooldownMs: 2600,
		Pierce: 0, AoeRadius: 260, ApplyElement: ElementCorrosion, ApplyStacks: 3, ProjectileSpeed: 0, Chain: 0, UnlockLevel: 44},

	// 无人机系
	{ID: 19, Code: "scoutdrone_skill", Name: "侦察无人机", Family: "drone", Element: ElementLightning, Kind: "passive",
		Descr: "被动：持续标记最远的目标，提高全队对其伤害。", BaseDamage: 0, HeatCost: 0, CooldownMs: 0, UnlockLevel: 14},
	{ID: 20, Code: "strikedron", Name: "打击无人机", Family: "drone", Element: ElementKinetic, Kind: "active",
		Descr: "无人机自动攻击，优先打飞行单位。", BaseDamage: 120, HeatCost: 28, CooldownMs: 1300,
		Pierce: 0, AoeRadius: 0, ApplyElement: ElementKinetic, ApplyStacks: 1, ProjectileSpeed: 0, Chain: 0, UnlockLevel: 28},
	{ID: 21, Code: "swarm", Name: "蜂群战术", Family: "drone", Element: ElementFire, Kind: "active",
		Descr: "无人机群齐射，覆盖整个屏幕。", BaseDamage: 190, HeatCost: 48, CooldownMs: 2600,
		Pierce: 0, AoeRadius: 400, ApplyElement: ElementFire, ApplyStacks: 2, ProjectileSpeed: 0, Chain: 0, UnlockLevel: 52},

	// 工程系
	{ID: 22, Code: "barricade", Name: "阻击工事", Family: "engineering", Element: ElementKinetic, Kind: "passive",
		Descr: "被动：防线额外获得护甲。", BaseDamage: 0, HeatCost: 0, CooldownMs: 0, UnlockLevel: 16},
	{ID: 23, Code: "repair", Name: "工程修复", Family: "engineering", Element: ElementIce, Kind: "active",
		Descr: "每 3 秒回复防线血量。", BaseDamage: 0, HeatCost: 35, CooldownMs: 3000, UnlockLevel: 32},
	{ID: 24, Code: "tesla", Name: "电磁栅栏", Family: "engineering", Element: ElementLightning, Kind: "active",
		Descr: "沿防线铺开电栅，接触者持续受电并被减速。", BaseDamage: 150, HeatCost: 44, CooldownMs: 2400,
		Pierce: 10, AoeRadius: 40, ApplyElement: ElementLightning, ApplyStacks: 2, ProjectileSpeed: 0, Chain: 0, UnlockLevel: 48},
}

// SeedCompositeSkills 是配方产物（ID 31~48）。
//
// 这些不是基础技能，而是由两个基础技能熔炼出的高阶技能，
// 在 skills 表中同样是真实行 —— 配方只是"怎么得到它"的另一条路径。
// 单独区分是为了校验"24 个基础技能 = 8 系 × 3"这条设计约束。
var SeedCompositeSkills = []SeedSkill{
	{ID: 31, Code: "cmp_ember", Name: "灼热弹幕", Family: "flame", Element: ElementFire, Kind: "active",
		Descr: "燃烧弹 + 燃油弹。范围燃烧且层数叠加更快。", BaseDamage: 260, HeatCost: 46, CooldownMs: 1900,
		Pierce: 0, AoeRadius: 190, ApplyElement: ElementFire, ApplyStacks: 3, ProjectileSpeed: 52000, Chain: 1, UnlockLevel: 30},
	{ID: 32, Code: "cmp_napalm", Name: "燃油弹幕", Family: "flame", Element: ElementFire, Kind: "active",
		Descr: "燃油弹 + 破片弹。大范围持续燃烧。", BaseDamage: 320, HeatCost: 52, CooldownMs: 2200,
		Pierce: 0, AoeRadius: 240, ApplyElement: ElementFire, ApplyStacks: 3, ProjectileSpeed: 48000, Chain: 0, UnlockLevel: 42},
	{ID: 33, Code: "cmp_scorch", Name: "焦土重锤", Family: "flame", Element: ElementFire, Kind: "active",
		Descr: "温压弹 + 破片弹。三阶配方，爆炸后留下灼烧地带。", BaseDamage: 420, HeatCost: 64, CooldownMs: 3000,
		Pierce: 1, AoeRadius: 280, ApplyElement: ElementFire, ApplyStacks: 4, ProjectileSpeed: 46000, Chain: 2, UnlockLevel: 60},
	{ID: 34, Code: "cmp_blizzard", Name: "霜暴弹幕", Family: "frost", Element: ElementIce, Kind: "active",
		Descr: "干冰弹 + 冰雹加农。急速冻结的常见起手。", BaseDamage: 230, HeatCost: 44, CooldownMs: 1800,
		Pierce: 0, AoeRadius: 190, ApplyElement: ElementIce, ApplyStacks: 3, ProjectileSpeed: 48000, Chain: 0, UnlockLevel: 30},
	{ID: 35, Code: "cmp_zerodeg", Name: "绝对零度", Family: "frost", Element: ElementIce, Kind: "active",
		Descr: "冰雹加农 + 极寒冰弹。三阶配方，大范围冻结。", BaseDamage: 380, HeatCost: 60, CooldownMs: 2800,
		Pierce: 1, AoeRadius: 260, ApplyElement: ElementIce, ApplyStacks: 4, ProjectileSpeed: 62000, Chain: 0, UnlockLevel: 56},
	{ID: 36, Code: "cmp_super", Name: "超导穿刺", Family: "frost", Element: ElementIce, Kind: "active",
		Descr: "极寒冰弹 + 电弧弹。冰电双元素，天然走超导链。", BaseDamage: 300, HeatCost: 52, CooldownMs: 2300,
		Pierce: 2, AoeRadius: 0, ApplyElement: ElementIce, ApplyStacks: 3, ProjectileSpeed: 78000, Chain: 2, UnlockLevel: 44},
	{ID: 37, Code: "cmp_arcpierce", Name: "电磁穿刺", Family: "volt", Element: ElementLightning, Kind: "active",
		Descr: "电弧弹 + 电磁穿刺。高速弹射电击。", BaseDamage: 290, HeatCost: 50, CooldownMs: 2100,
		Pierce: 2, AoeRadius: 0, ApplyElement: ElementLightning, ApplyStacks: 3, ProjectileSpeed: 86000, Chain: 3, UnlockLevel: 34},
	{ID: 38, Code: "cmp_storm", Name: "雷暴链", Family: "volt", Element: ElementLightning, Kind: "active",
		Descr: "电磁穿刺 + 雷暴链。三阶配方，六连弹射。", BaseDamage: 400, HeatCost: 68, CooldownMs: 3200,
		Pierce: 0, AoeRadius: 0, ApplyElement: ElementLightning, ApplyStacks: 4, ProjectileSpeed: 0, Chain: 6, UnlockLevel: 62},
	{ID: 39, Code: "cmp_shrap", Name: "破片重弹", Family: "kinetic", Element: ElementKinetic, Kind: "active",
		Descr: "重炮弹 + 破片弹。空中解体造成大范围动能伤害。", BaseDamage: 310, HeatCost: 48, CooldownMs: 2000,
		Pierce: 0, AoeRadius: 240, ApplyElement: ElementKinetic, ApplyStacks: 3, ProjectileSpeed: 42000, Chain: 0, UnlockLevel: 32},
	{ID: 40, Code: "cmp_crush", Name: "动能碾碎", Family: "kinetic", Element: ElementKinetic, Kind: "active",
		Descr: "破片弹 + 攻城锤。三阶配方，专破高护甲。", BaseDamage: 480, HeatCost: 66, CooldownMs: 3000,
		Pierce: 4, AoeRadius: 0, ApplyElement: ElementKinetic, ApplyStacks: 4, ProjectileSpeed: 38000, Chain: 0, UnlockLevel: 58},
	{ID: 41, Code: "cmp_lance", Name: "贯穿光束", Family: "light", Element: ElementKinetic, Kind: "active",
		Descr: "重炮弹 + 聚焦激光。弹道型高穿透光束。", BaseDamage: 270, HeatCost: 46, CooldownMs: 1900,
		Pierce: 5, AoeRadius: 0, ApplyElement: ElementKinetic, ApplyStacks: 2, ProjectileSpeed: 88000, Chain: 0, UnlockLevel: 36},
	{ID: 42, Code: "cmp_flood", Name: "光能洪流", Family: "light", Element: ElementKinetic, Kind: "active",
		Descr: "高能射线 + 聚焦激光。三阶配方，贯穿一切。", BaseDamage: 360, HeatCost: 64, CooldownMs: 2800,
		Pierce: 12, AoeRadius: 0, ApplyElement: ElementKinetic, ApplyStacks: 2, ProjectileSpeed: 0, Chain: 0, UnlockLevel: 60},
	{ID: 43, Code: "cmp_miasma", Name: "毒雾弥漫", Family: "blight", Element: ElementCorrosion, Kind: "active",
		Descr: "腐蚀弹 + 喷射毒雾。铺设持续毒区。", BaseDamage: 210, HeatCost: 44, CooldownMs: 1800,
		Pierce: 0, AoeRadius: 260, ApplyElement: ElementCorrosion, ApplyStacks: 3, ProjectileSpeed: 0, Chain: 0, UnlockLevel: 34},
	{ID: 44, Code: "cmp_plague", Name: "瘟疫降世", Family: "blight", Element: ElementCorrosion, Kind: "active",
		Descr: "喷射毒雾 + 液化毒雾。三阶配方，把毒层数传遍全场。", BaseDamage: 300, HeatCost: 62, CooldownMs: 2900,
		Pierce: 0, AoeRadius: 400, ApplyElement: ElementCorrosion, ApplyStacks: 4, ProjectileSpeed: 0, Chain: 0, UnlockLevel: 64},
	{ID: 45, Code: "cmp_voltvenom", Name: "腐蚀电弧", Family: "blight", Element: ElementLightning, Kind: "active",
		Descr: "电弧弹 + 腐蚀弹。弹射并沿途播撒毒层。", BaseDamage: 240, HeatCost: 46, CooldownMs: 1900,
		Pierce: 0, AoeRadius: 0, ApplyElement: ElementCorrosion, ApplyStacks: 3, ProjectileSpeed: 72000, Chain: 4, UnlockLevel: 40},
	{ID: 46, Code: "cmp_guidance", Name: "制导光束", Family: "light", Element: ElementLightning, Kind: "active",
		Descr: "高能射线 + 打击无人机。自动锁定补刀。", BaseDamage: 280, HeatCost: 50, CooldownMs: 2100,
		Pierce: 4, AoeRadius: 0, ApplyElement: ElementLightning, ApplyStacks: 2, ProjectileSpeed: 0, Chain: 2, UnlockLevel: 46},
	{ID: 47, Code: "cmp_cryogrid", Name: "低温栅栏", Family: "engineering", Element: ElementIce, Kind: "active",
		Descr: "电磁栅栏 + 干冰弹。沿防线铺开的低温电栅。", BaseDamage: 300, HeatCost: 58, CooldownMs: 2600,
		Pierce: 12, AoeRadius: 50, ApplyElement: ElementIce, ApplyStacks: 3, ProjectileSpeed: 0, Chain: 0, UnlockLevel: 54},
	{ID: 48, Code: "cmp_prismsiege", Name: "光棱攻城", Family: "light", Element: ElementFire, Kind: "active",
		Descr: "攻城锤 + 棱镜散射。三阶配方，巨型光棱贯穿。", BaseDamage: 520, HeatCost: 72, CooldownMs: 3400,
		Pierce: 3, AoeRadius: 300, ApplyElement: ElementFire, ApplyStacks: 4, ProjectileSpeed: 60000, Chain: 5, UnlockLevel: 70},
}

// SeedRecipes 合成配方：把两个技能熔炼成更高阶。
var SeedRecipes = []SeedRecipe{
	{Output: 31, A: 1, B: 2, OutTier: 2},   // 灼热弹幕
	{Output: 32, A: 2, B: 11, OutTier: 2},  // 燃油弹幕
	{Output: 33, A: 3, B: 11, OutTier: 3},  // 焦土重锤
	{Output: 34, A: 4, B: 5, OutTier: 2},   // 霜暴弹幕
	{Output: 35, A: 5, B: 6, OutTier: 3},   // 绝对零度
	{Output: 36, A: 6, B: 7, OutTier: 2},   // 超导穿刺
	{Output: 37, A: 7, B: 8, OutTier: 2},   // 电磁穿刺
	{Output: 38, A: 8, B: 9, OutTier: 3},   // 雷暴链
	{Output: 39, A: 10, B: 11, OutTier: 2}, // 破片重弹
	{Output: 40, A: 11, B: 12, OutTier: 3}, // 动能碾碎
	{Output: 41, A: 10, B: 13, OutTier: 2}, // 贯穿光束
	{Output: 42, A: 14, B: 13, OutTier: 3}, // 光能洪流
	{Output: 43, A: 16, B: 17, OutTier: 2}, // 毒雾弥漫
	{Output: 44, A: 17, B: 18, OutTier: 3}, // 瘟疫降世
	{Output: 45, A: 7, B: 16, OutTier: 2},  // 腐蚀电弧
	{Output: 46, A: 14, B: 20, OutTier: 2}, // 制导光束
	{Output: 47, A: 24, B: 4, OutTier: 2},  // 低温栅栏
	{Output: 48, A: 12, B: 15, OutTier: 3}, // 光棱攻城
}

// --- 装备（6 部位 × 3 阶） ---

var equipmentSlots = []string{"weapon", "helmet", "armor", "gloves", "boots", "charm"}

var SeedEquipmentList = []SeedEquipment{
	// 武器
	{ID: 1, Code: "w_p1", Name: "制式炮管", Slot: "weapon", Tier: 1, Element: ElementKinetic, Descr: "军方量产的基础炮管。攻击力 +50‰。", BaseArmor: 0, BaseBonusPct: 50, UnlockLevel: 1},
	{ID: 2, Code: "w_p2", Name: "高压膛线", Slot: "weapon", Tier: 2, Element: ElementFire, Descr: "膛线内刻高温纹路。攻击力 +120‰。", BaseArmor: 0, BaseBonusPct: 120, UnlockLevel: 15},
	{ID: 3, Code: "w_p3", Name: "相位炮芯", Slot: "weapon", Tier: 3, Element: ElementLightning, Descr: "让弹丸短暂进入相位态。攻击力 +250‰。", BaseArmor: 0, BaseBonusPct: 250, UnlockLevel: 40},
	// 头盔
	{ID: 4, Code: "h_p1", Name: "防砸盔", Slot: "helmet", Tier: 1, Element: ElementKinetic, Descr: "提升防线护甲。护甲 +20‰，攻击力 +30‰。", BaseArmor: 20, BaseBonusPct: 30, UnlockLevel: 4},
	{ID: 5, Code: "h_p2", Name: "滤毒面罩", Slot: "helmet", Tier: 2, Element: ElementCorrosion, Descr: "抵御腐蚀侵袭的风蚀面罩。护甲 +45‰，攻击力 +80‰。", BaseArmor: 45, BaseBonusPct: 80, UnlockLevel: 18},
	{ID: 6, Code: "h_p3", Name: "观测灵镜", Slot: "helmet", Tier: 3, Element: ElementLightning, Descr: "看清敌人身上的元素层数。护甲 +60‰，攻击力 +140‰。", BaseArmor: 60, BaseBonusPct: 140, UnlockLevel: 44},
	// 护甲
	{ID: 7, Code: "a_p1", Name: "拼接胸甲", Slot: "armor", Tier: 1, Element: ElementKinetic, Descr: "用废旧钢板拼成，聊胜于无。护甲 +35‰，攻击力 +25‰。", BaseArmor: 35, BaseBonusPct: 25, UnlockLevel: 6},
	{ID: 8, Code: "a_p2", Name: "液氮夹层", Slot: "armor", Tier: 2, Element: ElementIce, Descr: "夹层液氮的复合装甲。护甲 +70‰，攻击力 +70‰。", BaseArmor: 70, BaseBonusPct: 70, UnlockLevel: 22},
	{ID: 9, Code: "a_p3", Name: "反应装甲", Slot: "armor", Tier: 3, Element: ElementFire, Descr: "受热变形的重型装甲。护甲 +110‰，攻击力 +120‰。", BaseArmor: 110, BaseBonusPct: 120, UnlockLevel: 50},
	// 手套
	{ID: 10, Code: "g_p1", Name: "劳保手套", Slot: "gloves", Tier: 1, Element: ElementKinetic, Descr: "装填机构打磨过。护甲 +5‰，攻击力 +40‰。", BaseArmor: 5, BaseBonusPct: 40, UnlockLevel: 2},
	{ID: 11, Code: "g_p2", Name: "磁力手套", Slot: "gloves", Tier: 2, Element: ElementLightning, Descr: "引导弹丸轨迹的稳定器。护甲 +10‰，攻击力 +100‰。", BaseArmor: 10, BaseBonusPct: 100, UnlockLevel: 14},
	{ID: 12, Code: "g_p3", Name: "元素编织手套", Slot: "gloves", Tier: 3, Element: ElementCorrosion, Descr: "徒手强化元素层数的指套。护甲 +15‰，攻击力 +200‰。", BaseArmor: 15, BaseBonusPct: 200, UnlockLevel: 42},
	// 靴子
	{ID: 13, Code: "b_p1", Name: "作战靴", Slot: "boots", Tier: 1, Element: ElementKinetic, Descr: "基础机动。护甲 +10‰，攻击力 +35‰。", BaseArmor: 10, BaseBonusPct: 35, UnlockLevel: 3},
	{ID: 14, Code: "b_p2", Name: "减震靴", Slot: "boots", Tier: 2, Element: ElementKinetic, Descr: "鞋底的缓冲结构。护甲 +20‰，攻击力 +75‰。", BaseArmor: 20, BaseBonusPct: 75, UnlockLevel: 20},
	{ID: 15, Code: "b_p3", Name: "相位靴", Slot: "boots", Tier: 3, Element: ElementIce, Descr: "相位护踝。护甲 +30‰，攻击力 +150‰。", BaseArmor: 30, BaseBonusPct: 150, UnlockLevel: 48},
	// 挂件
	{ID: 16, Code: "c_p1", Name: "旧军牌", Slot: "charm", Tier: 1, Element: ElementFire, Descr: "略微放大元素反应。攻击力 +60‰。", BaseArmor: 0, BaseBonusPct: 60, UnlockLevel: 5},
	{ID: 17, Code: "c_p2", Name: "元素棱镜", Slot: "charm", Tier: 2, Element: ElementLightning, Descr: "棱镜切分光路。攻击力 +110‰。", BaseArmor: 0, BaseBonusPct: 110, UnlockLevel: 26},
	{ID: 18, Code: "c_p3", Name: "母核碎片", Slot: "charm", Tier: 3, Element: ElementCorrosion, Descr: "从母巢抢出的碎片，仍在搏动。攻击力 +220‰。", BaseArmor: 0, BaseBonusPct: 220, UnlockLevel: 60},
}

// EquipmentByID 按 id 查装备。
//
// ⚠️ 找不到时返回 ok=false，**不返回零值装备**。
//
// 调用方（service.loadLoadout）遇到未知 id 会跳过。回落到"某件默认装备"
// 等于白送一份属性 —— 而 DB 里的 equipment_id 有外键约束，
// 出现未知 id 说明数据被改过或 seed 版本不匹配，
// 那种情况下宁可少给也不能多给。
func EquipmentByID(id int64) (SeedEquipment, bool) {
	for _, e := range SeedEquipmentList {
		if e.ID == int(id) {
			return e, true
		}
	}
	return SeedEquipment{}, false
}

// --- 宝石（8 属性） ---

var SeedGems = []SeedGem{
	{ID: 1, Code: "gem_atk", Name: "锋锐石", Attr: "attack", Descr: "提高面板攻击力。"},
	{ID: 2, Code: "gem_elem", Name: "元素石", Attr: "element_coef", Descr: "提高元素系数 —— 反应伤害的核心。"},
	{ID: 3, Code: "gem_crit", Name: "锐锋石", Attr: "crit", Descr: "提高暴击率。"},
	{ID: 4, Code: "gem_react", Name: "共振石", Attr: "reaction_mult", Descr: "提高反应倍率。"},
	{ID: 5, Code: "gem_cap", Name: "容量石", Attr: "element_cap", Descr: "提高单元素层数上限。"},
	{ID: 6, Code: "gem_heat", Name: "散热石", Attr: "heat_cap", Descr: "提高热量上限。"},
	{ID: 7, Code: "gem_armor", Name: "壁垒石", Attr: "armor", Descr: "提高防线护甲。"},
	{ID: 8, Code: "gem_skill", Name: "技艺石", Attr: "skill_damage", Descr: "提高所有技能伤害。"},
}

// --- 皮肤（6 款，机制型词条） ---

var SeedSkins = []SeedSkin{
	{ID: 1, Code: "skin_ranger", Name: "游猎者", Rarity: "rare",
		Passive: "kinetic_bonus", Descr: "动能系技能额外附带一次破甲击退。"},
	{ID: 2, Code: "skin_pyro", Name: "炽炎使徒", Rarity: "epic",
		Passive: "fire_burn", Descr: "焰系技能命中时，额外附加一层燃烧。"},
	{ID: 3, Code: "skin_cryo", Name: "霜行者", Rarity: "epic",
		Passive: "ice_freeze", Descr: "冰系技能有 20% 概率直接冻结目标。"},
	{ID: 4, Code: "skin_volt", Name: "雷鸣者", Rarity: "epic",
		Passive: "volt_chain", Descr: "电系技能弹射次数 +1。"},
	{ID: 5, Code: "skin_blight", Name: "腐沼行者", Rarity: "legendary",
		Passive: "blight_spread", Descr: "毒系技能命中时，向周围传播 1 层毒。"},
	{ID: 6, Code: "skin_chalk", Name: "白垩女王", Rarity: "legendary",
		Passive: "boss_breaker", Descr: "对 BOSS 的反应伤害提高 30%。"},
}
