// 装配（loadout）→ 攻方属性。
//
// ⚠️ 这个文件存在的理由是一笔**本轮才发现的欠账**：
// 在它出现之前，`computeAttacker` 只用了专精树的 3 种 kind，
// 而**装备与宝石对战斗数值零影响**：
//
//   - 18 件装备（6 槽 × 3 阶）的 `BaseArmor` / `BaseBonusPct`
//     只被 `seed.go` 写进 DB，`computeAttacker` **从不读取**。
//     玩家把资源投到三阶武器上，战斗里看不到任何变化。
//   - 8 种宝石的加成存在每颗实例的 `affixes` JSONB 里
//     （`SeedGem.Attr` 只是展示标签），而**全项目没有任何代码读过
//     `user_gems`** —— 装备/卸下/佩戴都只是数据，不产生效果。
//   - 专精树 96 个节点里，`heat_cap`（16 个）、`armor`（8 个）、
//     `skill_damage`（8 个）、`mechanic`（8 个）四类**全部惰性**。
//
// 也就是说：I-3 专精树与装备系统**基本是空壳**。
// 此前之所以没人发现，是因为 `computeRating`（I-7 构筑评分）
// 会把这些项算进去并展示 —— 看起来"有数值"，实际战斗里不生效。
//
// **教训**：展示层读同一个数据源，不等于战斗层也读。
// 凡是"玩家能看到某个属性"的字段，都必须有一条
// "它改变了战斗里某个可观测量"的测试。

package service

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/laiyipao/server/internal/domain"
)

// gemAffix 是 `user_gems.affixes` JSONB 里的一项。
//
// 形如 `[{"affix":"crit","value":80}, ...]`。
//
// ⚠️ 词表与专精树**部分重叠但不完全相同**：专精用 element_cap/
// reaction_mult/heat_cap/skill_damage/crit/armor/extra_slot/mechanic，
// 宝石用 attack/element_coef/crit/reaction_mult/element_cap/
// heat_cap/armor/skill_damage。
// 两个不同的名字（专精 element_cap 与宝石 element_cap）指的是同一件事，
// 而专精的 element_coef 并不存在 —— 这套命名本身就是欠债，
// 见「未解决」注释。
type gemAffix struct {
	Affix string `json:"affix"`
	Value int64  `json:"value"`
}

// 装配贡献的封顶。
//
// 全部是**必要**的，不是保守：装备 6 槽 + 宝石任意多颗 + 专精 48 节点
// 三者相加没有天然上界，而 Attack / ElementCoefPermille 这些字段
// 直接进伤害公式，无界就等于"数值失控"。
//
// Attack 侧的上界尤其要小心：Attack 是 I-1 反通胀公式里攻击力侧的那一项，
// 给它开一个无界入口等于绕过 30‰ 红线（见 damage.go 的 Attacker 注释）。
const (
	// MaxLoadoutAttackPermille 装备+宝石+专精能提供的额外攻击力上限。
	//
	// ⚠️ 从 3000 降到 **1000**，依据是 100 关的实测扫描
	// （miniapp/src/game/tier_impact.test.ts 的档位对比）：
	//
	//	attack‰   击杀数   分数      反应次数
	//	   0      5262    790188   15064
	//	 300      5268    778983   14750  (-2.1%)
	//	 600      5271    792239   14351  (-4.7%)
	//	1000      5272    791402   13787  (-8.5%)
	//	2000      5274    806682   12326  (-18.2%)
	//	3000      5275    807665   11279  (-25.1%)
	//
	// 两个结论：
	//  1. **堆攻击力买不到分数**（+0.6%），因为分数由击杀分主导，
	//     而击杀数几乎不随攻击力变（战斗的瓶颈是热量/冷却周期，不是伤害）。
	//  2. **堆攻击力直接压制反应**（单调 -25%）：敌人死得比攒层数快，
	//     于是反应触发得更少 —— 而反应正是 I-1 的核心创新。
	//
	// 也就是说这条成长线**同时没有收益且有代价**。
	// 封顶砍到 1000‰，把预算留给元素系数与反应倍率（那两个方向是正的）。
	MaxLoadoutAttackPermille = 1000

	// MaxLoadoutElementCoefPermille 元素系数加成上限。
	//
	// 专精 reaction_mult 之外的独立来源（宝石 element_coef）。
	// 实测这个方向是**正**的：元素系数提高 → 反应伤害提高。
	//
	// 历史：1800 → 1200（第 54 轮），依据是逐关回归 ——
	// 1800‰ 时总分 +1.4%，但**有 2 关分数反而下降、1 关从 3 星掉到 2 星**。
	//
	// ⚠️ 第 65 轮：1200 → **1000**。修好弹射/溅射后战斗整体上移，
	// 逐关回归在 1200‰ 重新出现（第 10 关 22322→21373，-4.25%，掉 1 星）。
	// 全 100 关扫描（满配 vs 裸装，可比口径）：
	//
	//	elementCoef‰   满配漏怪增加的关数   掉分关数   掉星关数
	//	     600              2                0         0
	//	     800              1                0         1
	//	    1000              1                0         0
	//	    1200              3                1         1
	//
	// 1000‰ 是「无掉分、无掉星」且漏怪最少的那一档。
	// ⚠️ 曲线**非单调**这一点仍未解决：800‰ 掉星而 1000‰ 不掉。
	// 已记入 README「已知边界」。
	MaxLoadoutElementCoefPermille = 1000

	// MaxLoadoutCritPermille 暴击率加成上限（总上限另由 computeAttacker 封）。
	MaxLoadoutCritPermille = 500

	// MaxLoadoutReactionMultPermille 反应倍率加成上限。
	// 与专精同一上限量级，避免"宝石+专精"叠出比专精独占更高的倍率。
	MaxLoadoutReactionMultPermille = 800

	// MaxLoadoutHeatCapPermille 热量上限加成上限。
	//
	// ⚠️ 单位是**千分比**（不是绝对值）。`HeatMeter.cap` 按
	// `HEAT_MAX × (1000 + capBonus) / 1000` 计算。
	//
	// 曾经错写成 `HEAT_MAX + capBonus`，把千分比当绝对值加 ——
	// 1000‰ 让上限变成 1100 而不是 200，而**热量衰减率是固定的**，
	// 于是玩家长期卡在"热量高但放不出技能"的死区：
	//
	//	capBonus‰   cap    过热次数   漏怪    分数变化
	//	     0      100     9028      486     基准
	//	   200      300     3511      476     -
	//	  1000     1100     1174     1168     **-13.8%**
	//
	// 「提高热量上限」是**有利属性**，出现反向曲线只能是单位错了。
	// 修正后单调有益：0‰→1000‰ 漏怪 486→465、时长 28076s→21513s。
	//
	// ⚠️ 第 65 轮：1000 → **600**。这条属性现在**不再单调有益**：
	// 提高热量上限让开火更密更快，而战斗整体上移后，
	// 「更快」意味着敌人推进更快 —— 于是满配反而更容易漏怪。
	// 全 100 关扫描（满配 vs 裸装）：
	//
	//	heatCapPermille‰   满配漏怪增加的关数   掉分关数   掉星关数
	//	     0                    1                0         0
	//	   200                    0                0         0
	//	   400                    2                0         0
	//	   600                    1                0         0
	//	   800                    1                0         0
	//	  1000                    3                1         1
	//
	// 600‰ 与 elementCoef 1000‰ 组合时（2026-10-03 实测）：
	// 掉分关数 0、掉星关数 0、满配失败关数 0，仅第 100 关漏怪 0→9。
	MaxLoadoutHeatCapPermille = 600

	// MaxLoadoutMechanicPermille 机制卡强度加成上限。
	MaxLoadoutMechanicPermille = 1000
)

// loadoutContribution 是"装配对攻方的贡献"，尚未叠加。
//
// 刻意**不直接返回 domain.Attacker** —— 那样每个调用点都要记得
// 自己套封顶。分成"读原始值"与"封顶"两步，封顶只写一次。
type loadoutContribution struct {
	Attack            int64
	Armor             int64
	Crit              int64
	ElementCoef       int64
	ElementCap        int64
	ReactionMult      int64
	HeatCap           int64
	Mechanic          int64
	EquipmentCount    int
	EquippedGemCount  int
	AffixCountIgnored int
}

// loadLoadout 读取玩家已装备的装备与宝石，返回**未封顶**的原始贡献。
//
// ⚠️ 不在这里套封顶：封顶在 applyLoadout 里，因为封顶必须与
// 专精树的贡献**相加之后**再做（否则专精 + 装备可以绕过上限）。
func (s *Service) loadLoadout(ctx context.Context, userID int64) (loadoutContribution, error) {
	var c loadoutContribution

	// --- 装备 ---
	//
	// 只读 equipped = TRUE 的行。
	//
	// ⚠️ `level` / `star` 两列**存在但仍未接线**（下面 SQL 只 SELECT equipment_id）。
	// 这不是「忘了做」，而是**实测过预算之后决定不做**。数据（2026-09-28）：
	//
	//	                      装备(每槽最优)  专精满  合计   封顶   余量
	//	  base_bonus_pct(攻击)        1080       0  1080   1000   **-80**
	//	  base_armor(护甲)             215     240    455    750   +295
	//
	// **攻击侧的基础值之和已经越过封顶**（1080 > 1000）。
	// 所以「装备等级/星级 → 攻击力」会把玩家的钱推进**静默浪费区**：
	// clampPermille 之外的部分直接消失，界面上看不出任何区别。
	// 这与元素系数那个「惩罚区间」是同一种形状（400‰ 时漏怪反而变多，
	// 那是实测才发现曲线是反的）。
	//
	// 护甲侧有 295‰ 真实预算，**可以**做缩放 —— 但曲线本身是设计决策，
	// 且要保证「满级满星 + 专精满 ≤ 750」。这两条事实由
	// `internal/service/equip_budget_test.go` 钉住。
	//
	// 完整推导见该测试的文件头。
	rows, err := s.pool.Query(ctx,
		`SELECT equipment_id FROM user_equipment WHERE user_id = $1 AND equipped ORDER BY slot`, userID)
	if err != nil {
		return c, fmt.Errorf("load equipment: %w", err)
	}
	defer rows.Close()
	for rows.Next() {
		var id int
		if err := rows.Scan(&id); err != nil {
			return c, err
		}
		eq, ok := domain.EquipmentByID(int64(id))
		if !ok {
			// 未知 id：跳过而不是回落到默认值。
			// 回落到"某个默认装备"会给玩家一份白拿的属性。
			continue
		}
		c.EquipmentCount++
		c.Attack += eq.BaseBonusPct
		c.Armor += eq.BaseArmor
	}
	if err := rows.Err(); err != nil {
		return c, err
	}

	// --- 宝石 ---
	//
	// `equipped_slot <> ''` 即视为已佩戴。表上有 `affixes` JSONB
	// 存每颗的词条，这里解析它。`gem_id` 指向 SeedGems，
	// 但 SeedGem.Attr 只是**展示标签**，真正的数值在 affixes 里 ——
	// 同一 Attr 的不同品质宝石数值不同，靠 gem_id 推不出来。
	gemRows, err := s.pool.Query(ctx,
		`SELECT affixes FROM user_gems WHERE user_id = $1 AND equipped_slot <> ''`, userID)
	if err != nil {
		return c, fmt.Errorf("load gems: %w", err)
	}
	defer gemRows.Close()
	for gemRows.Next() {
		var raw []byte
		if err := gemRows.Scan(&raw); err != nil {
			return c, err
		}
		var affixes []gemAffix
		if err := json.Unmarshal(raw, &affixes); err != nil {
			// 解析失败**不能静默跳过**：一颗词条读不出来的宝石，
			// 静默跳过等于玩家付了成本却没生效，且没有任何迹象。
			// 记在 Ignored 里向上暴露，由调用方决定是否拒绝这局。
			c.AffixCountIgnored++
			continue
		}
		c.EquippedGemCount++
		for _, af := range affixes {
			switch af.Affix {
			case "attack", "skill_damage":
				c.Attack += af.Value
			case "armor":
				c.Armor += af.Value
			case "crit":
				c.Crit += af.Value
			case "element_coef":
				c.ElementCoef += af.Value
			case "element_cap":
				c.ElementCap += af.Value
			case "reaction_mult":
				c.ReactionMult += af.Value
			case "heat_cap":
				c.HeatCap += af.Value
			default:
				// 未知词条：与解析失败同等对待 —— 不能默默吞掉。
				c.AffixCountIgnored++
			}
		}
	}
	if err := gemRows.Err(); err != nil {
		return c, err
	}
	return c, nil
}

// applyLoadout 把装配贡献**封顶后**加到攻方上。
//
// 必须在专精树的贡献已经加完之后调用：封顶针对的是"总贡献"，
// 否则"专精 800‰ + 装备 2000‰"这种组合会绕过单一来源的上限。
func applyLoadout(a *domain.Attacker, c loadoutContribution) {
	a.Attack += clampPermille(c.Attack, MaxLoadoutAttackPermille)
	a.ArmorPermille += clampPermille(c.Armor, domain.MaxArmorPermille)
	a.CritPermille += clampPermille(c.Crit, MaxLoadoutCritPermille)
	a.ElementCoefPermille += clampPermille(c.ElementCoef, MaxLoadoutElementCoefPermille)
	a.ElementCap += c.ElementCap
	a.ReactionMultPermille += clampPermille(c.ReactionMult, MaxLoadoutReactionMultPermille)
	a.HeatCapPermille += clampPermille(c.HeatCap, MaxLoadoutHeatCapPermille)
	a.MechanicPermille += clampPermille(c.Mechanic, MaxLoadoutMechanicPermille)
}

// clampPermille 把千分比数值夹到 [0, max]。
//
// 负值夹到 0 而不是原样保留：一个负的攻击力加成会让伤害变小，
// 而"负收益"从来不是任何一件装备或词条的效果 —— 它只可能来自脏数据，
// 而脏数据应该被夹住而不是被解释成设计。
func clampPermille(v, max int64) int64 {
	if v < 0 {
		return 0
	}
	if v > max {
		return max
	}
	return v
}
