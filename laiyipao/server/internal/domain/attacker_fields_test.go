package domain

// 守卫：攻方的**每一个字段**都必须真的改变战斗结果。
//
// ## 为什么需要这条
//
// 本轮之前，`domain.Attacker` 里有三项字段根本不存在
// （热量上限 / 玩家护甲 / 机制卡强度），而**已经存在的七项里，
// 没有任何一条测试断言"扰动它，结算结果会变"**。
//
// 结果是：`computeAttacker` 只装了专精树的 3 种 kind，
// 18 件装备、8 种宝石、以及 4 类共 40 个专精节点全部惰性 ——
// 它们出现在 `/config`、出现在 I-7 构筑评分里，战斗里却什么都不变。
//
// 三个已有测试都"覆盖"了这些数据，却没有一个断言"它有效果"：
//   - 内容表测试只查节点数量与 ID 规则
//   - 评分测试只查总分落在区间内（贡献 0 也在区间内）
//   - 战斗测试用固定攻方，不经过成长链路
//
// ## 为什么是行为式而不是文本搜索
//
// 曾经想写"按字段名 grep 读取点"的版本，**放弃了** ——
// 文本搜索会因为大小写、别名、或读取点换位置而误判，
// 而一个会误判的守卫比没有守卫更糟：它让人以为这一项被守着。
//
// 改成：扰动字段 → 跑一次真实结算 → 比对结果。
// 不影响伤害的字段必须列进 `notDamageRelated` 并写明理由；
// 新增字段时若既没加进扰动表、也没说明，这条会红。

import (
	"reflect"
	"testing"
)

func TestEveryAttackerFieldChangesResolveHit(t *testing.T) {
	def := Defender{
		HP:            1_000_000,
		Shield:        0,
		ElementStacks: map[Element]int64{ElementFire: 4, ElementIce: 2},
	}
	// Roll = 999‰ 让暴击判定**必然成立**（判定是 `roll >= 1000 - critPermille`，
	// 默认 critPermille=50‰ ⇒ 阈值 950，999 在其上）。
	// 不固定它的话，"扰动 CritPermille / CritMultiplierPermille"
	// 会因随机数落在阈值同侧而不改变结果 —— 变成一条**偶发**的守卫。
	//
	// ForceReaction 显式指定 steam_burst（冰+火），
	// 否则 ReactionMultPermille / ReactionTier 的扰动毫无作用 ——
	// 那两个字段只在反应伤害那一段被读。
	in := HitInput{
		SkillDamage:     1000,
		SkillElement:    ElementFire,
		ApplyStacks:     2,
		ForceReaction:   "steam_burst",
		ReactionElement: ElementIce,
		Roll:            999,
	}

	base := DefaultAttacker()
	baseline := ResolveHit(base, &def, in)

	// 不参与 ResolveHit 的字段，附理由。
	//
	// ⚠️ **新增字段必须在此说明理由**，否则本测试会在
	// "字段数与扰动表数不一致"那条上失败 —— 那是刻意的：
	// 强迫做决定，而不是让新字段静默地不受任何检查。
	notDamageRelated := map[string]string{
		"HeatCapPermille":  "热量上限由客户端 HeatMeter 消费（get cap()），不进伤害公式",
		"ArmorPermille":    "玩家护甲进 defenseArmorPermille()，作用于漏怪与被敌人攻击，不在 ResolveHit",
		"MechanicPermille": "机制卡强度作用在卡面数值上，由客户端 applyMechanic 消费",
		"ElementCap": "元素层数**上限**由 ResolveHit 末尾的 ApplyElement 消费，" +
			"影响的是后续几击的层数累积；单看一击的 TotalDamage 不会变。",
	}

	perturbers := map[string]func(a *Attacker){
		"Attack":                 func(a *Attacker) { a.Attack += 500 },
		"ReactionMultPermille":   func(a *Attacker) { a.ReactionMultPermille += 800 },
		"ElementCoefPermille":    func(a *Attacker) { a.ElementCoefPermille += 500 },
		"CritMultiplierPermille": func(a *Attacker) { a.CritMultiplierPermille = 3000 },
		"ReactionTier":           func(a *Attacker) { a.ReactionTier = 4 },
		// 往**下**扰动而不是往上：Roll=999 已让基线必然暴击，
		// 再往上抬暴击率不会改变任何结果（1000‰ 已是上限）。
		// 调到 0 让这一击不暴击，总伤害立刻变。
		"CritPermille": func(a *Attacker) { a.CritPermille = 0 },
	}

	fields := make([]string, 0, 10)
	tt := reflect.TypeOf(Attacker{})
	for i := 0; i < tt.NumField(); i++ {
		fields = append(fields, tt.Field(i).Name)
	}

	for _, name := range fields {
		perturb, hasPerturb := perturbers[name]
		if !hasPerturb {
			if reason, explained := notDamageRelated[name]; !explained {
				t.Errorf(
					"攻方字段 %q 既不在扰动表里、也没在 notDamageRelated 里说明"+
						"为什么不影响伤害 —— 请补上其中一个", name)
			} else {
				_ = reason
			}
			continue
		}

		a := DefaultAttacker()
		perturb(&a)
		got := ResolveHit(a, &def, in)
		if got.TotalDamage == baseline.TotalDamage {
			t.Errorf(
				"扰动 Attacker.%s 后 ResolveHit 的总伤害没变（%d）—— "+
					"该字段可能又是一个只写不读的字段",
				name, got.TotalDamage)
		}
	}

	// 数量对不上时点名：新增字段最常见的失败模式就是"忘了加进任何一张表"。
	// 判据是**每个字段恰好出现在一张表里**，而不是两个表的大小相加 ——
	// 后者会在有人把字段同时加进两张表时仍然通过。
	for _, name := range fields {
		_, inPerturb := perturbers[name]
		_, inExplained := notDamageRelated[name]
		if inPerturb == inExplained {
			t.Errorf(
				"攻方字段 %q 在扰动表与 notDamageRelated 中%s —— 必须**恰好**出现在一张表里",
				name, map[bool]string{true: "都出现了", false: "都没出现"}[inPerturb])
		}
	}
	// 反向：表里有字段名不存在于 Attacker（改名后忘了同步表）
	for name := range perturbers {
		if !fieldIn(fields, name) {
			t.Errorf("扰动表里的 %q 不在 domain.Attacker 的字段里（改名后忘了同步）", name)
		}
	}
	for name := range notDamageRelated {
		if !fieldIn(fields, name) {
			t.Errorf("notDamageRelated 里的 %q 不在 domain.Attacker 的字段里（改名后忘了同步）", name)
		}
	}
}

func fieldIn(list []string, s string) bool {
	for _, x := range list {
		if x == s {
			return true
		}
	}
	return false
}
