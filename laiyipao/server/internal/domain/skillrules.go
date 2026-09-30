package domain

// 技能升级规则。
//
// # 之前 `user_skills.level` 是一列**永远为 1** 的数据
//
// 被读进 build 快照、从无写入、也没有任何升级端点。
// 而 `HitInput.SkillDamage` 的注释曾声称伤害公式含「等级系数」——
// 一个不存在的因子。
//
// 接线要三件事同时成立，缺一不可：
//
//  1. **有升级路径**（service.UpgradeSkill）：消耗金币 → level+1
//  2. **等级进公式**（本文件 + 客户端 skillBaseDamageAtLevel）
//  3. **两端一致**（level 已在 build 快照里，两端按同一系数缩放）
//
// 只做 2 而不做 1，等级恒为 1、系数恒等于 1 ——
// 功能「看起来接好了」但完全无效。这是本项目反复栽过的形状
// （见 README「已知边界」里「只写不读」「纯装饰」那几条）。
//
// # 为什么规则放 domain 而不是 service
//
// 费用与上限是**经济规则**，系数是**伤害公式**。两者都必须在客户端可见：
// 升级按钮要显示费用与「已满级」，战斗要算伤害。
// 放 domain 有两个好处：
//
//   - `cmd/vectors` 能把它导出进契约夹具，客户端有守卫可比对
//     （照 `score_rules` 的做法，见 TestSkillRulesMatchServerContract）
//   - service 的扣费与客户端的伤害**读同一个定义**，而不是两份常量
//
// 分散在 service 和 TS 各写一份，是「跨端硬编码」的教科书案例 ——
// 项目里已经因此吃过一次亏（分数规则 5 处硬编码，Round 3 才收归服务端下发）。

const (
	// MaxSkillLevel 技能等级上限。
	MaxSkillLevel = 10

	// SkillLevelCoefPermille 每升一级提供的伤害加成（千分比）。
	//
	// ⚠️ 刻意取 50‰（满级 +450‰），**不取更大的值**，理由有实测支撑：
	// 技能伤害与面板攻击走的是同一个乘区 —— 都乘在 `SkillDamage` 上、
	// 再进 D，而 D 同时喂给直接伤害与反应的**攻击力侧**。
	// 所以提高它与提高攻击力的后果一样：**压制反应**。
	//
	// 实测（100 关）：攻击力堆到 3000‰ 时反应次数掉 25%，
	// 攻击封顶已因此从 3000 砍到 1000。
	// 满级技能的 +450‰ 只有攻击封顶的一半，
	// 保留「值得练」的手感而不喧宾夺主。
	SkillLevelCoefPermille = 50

	// SkillUpgradeBaseCost 从 level 升到 level+1 的费用基数（金币）。
	//
	// 实际费用 = `BaseCost × 当前等级`，线性递增。
	// 满级单个技能共 100×(1+2+…+9) = 4500 金币。
	//
	// 刻意用**线性**而不是指数：指数曲线会让「差一级就完全不可用」，
	// 而这个游戏里技能的定位是构筑多样性（4 个槽各自有取舍），
	// 不是数值竞赛。线性保证任何一级都拿得到、任何一级的边际收益相同。
	SkillUpgradeBaseCost = 100
)

// SkillRules 是下发给客户端的技能升级规则。
//
// 字段名与 JSON key 一律 snake_case，与 `ScoreRules` 保持同构 ——
// 客户端 `skillRulesFromServer` 可以照 `scoreRulesFromServer` 写。
type SkillRules struct {
	// MaxLevel 等级上限。
	MaxLevel int `json:"max_level"`
	// CoefPermille 每级的伤害加成（千分比），见 SkillLevelCoefPermille。
	CoefPermille int64 `json:"coef_permille"`
	// BaseCost 费用基数，实际费用 = BaseCost × 当前等级。
	BaseCost int64 `json:"base_cost"`
}

// DefaultSkillRules 返回默认规则。
//
// 单一真源：service 的扣费与客户端的伤害缩放都从这里派生。
func DefaultSkillRules() SkillRules {
	return SkillRules{
		MaxLevel:     MaxSkillLevel,
		CoefPermille: SkillLevelCoefPermille,
		BaseCost:     SkillUpgradeBaseCost,
	}
}

// CoefPermilleAt 返回该等级的伤害系数（千分比，1000 = 无加成）。
//
// **level 1 → 1000‰ 是精确的恒等变换**（不是近似）：
//
//	coef(1) = 1000 + (1-1)*50 = 1000
//	damage = base * 1000 / 1000 = base   （整除，无余数）
//
// 这条性质是**兼容性要求**，不是巧合：
// 升级功能上线前所有 `user_skills.level` 恒为 1，
// 所以所有**存量战报的重放哈希必须逐位不变**。
// 若把基线取成 1000‰ 以外的值（例如 900‰ 或 1050‰），
// 每一个历史战报都会静默失配 —— 而失配表现为「验真失败」，
// 看起来像作弊，实际是上线了一个数学常数。
//
// 超出 [1, MaxLevel] 的输入在此夹紧：脏数据不该变成
// 一个客户端与服务端解释不同的伤害。
func (r SkillRules) CoefPermilleAt(level int) int64 {
	if level < 1 {
		level = 1
	}
	if r.MaxLevel > 0 && level > r.MaxLevel {
		level = r.MaxLevel
	}
	return 1000 + int64(level-1)*r.CoefPermille
}

// CostFrom 返回把技能从 level 升到 level+1 的金币花费。
//
// 满级返回 0 —— 调用方**必须**先查 `CoefPermilleAt` 之外的上限，
// 否则「满级还扣 0 费」会让玩家以为升级成功。
// 这里返回 0 是为了让「费用为 0」这个信号在两端都表现为「不需要花」，
// 而不是让某个魔法分支去发散。
func (r SkillRules) CostFrom(level int) int64 {
	if level < 1 {
		level = 1
	}
	if r.MaxLevel > 0 && level >= r.MaxLevel {
		return 0
	}
	return r.BaseCost * int64(level)
}

// ClampLevel 把任意输入夹到合法等级区间。
//
// 存在的理由：`loadSkillsAndSlots` 读的是 `COALESCE(us.level, 1)`，
// 直通数据库。如果库里有一行 `level = 99`（迁移事故、手工改库、
// 未来某个端点忘了带上限），客户端会算出 4900‰ 的伤害 ——
// 而服务端的攻击封顶逻辑完全不知道这件事。
//
// 与其相信「不会有脏数据」，不如让读路径**必然**产出合法值。
func (r SkillRules) ClampLevel(level int) int {
	if level < 1 {
		return 1
	}
	if r.MaxLevel > 0 && level > r.MaxLevel {
		return r.MaxLevel
	}
	return level
}
