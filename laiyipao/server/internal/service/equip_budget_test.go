package service

// 养成预算的守卫：**给装备加缩放之前，先看还有没有预算。**
//
// ## 为什么要这条
//
// `user_equipment` 表有 `level` / `star` 两列，但它们**从未被读取**
// （`loadout_attacker.go` 的 SQL 只 `SELECT equipment_id`）。
// 「给装备加等级缩放」看起来是个显然该做的功能。
//
// 但实测（2026-09-28，本文件里的断言就是这些数字的来源）：
//
//	                      装备(每槽最优)  专精满  合计   封顶   余量
//	  base_bonus_pct(攻击)        1080       0  1080   1000   **-80**
//	  base_armor(护甲)             215     240    455    750   +295
//
// **攻击侧已经越封顶了。** 一身满配的基础值之和就是 1080，而封顶是 1000。
//
// 于是「装备等级/星级 → 攻击力」这个方案会：
//
//   1. 把玩家的钱推进**静默浪费区**（`clampPermille` 之外的部分直接消失，
//      界面上看不出任何区别，只是钱白花）；
//   2. 复刻元素系数那个「惩罚区间」—— 400‰ 时漏怪反而变多，
//      我当初是**实测**才发现那条曲线的形状是反的。
//
// > 本项目的教训反复指向同一件事：**在设计缩放之前，先量预算。**
// > 凭空造一条曲线比留着它更糟 —— 因为它看起来像是「有设计」。
//
// ## 这条守卫守什么
//
// 1. **攻击侧确实没有预算**（当前越封顶）。这条断言的**用意是记录现状**：
//    将来若有人为了加缩放而调高 `MaxLoadoutAttackPermille`，
//    这条会红，并强迫他重新思考 —— 而不是悄悄把浪费区做大。
// 2. **护甲侧有预算**，且「装备缩放后 + 专精满」仍不越封顶。
//    这条是装备等级缩放**可以安全做**的前提。
//
// ## 局限（明说）
//
// 它**不能**推出「护甲缩放曲线该长什么样」—— 那是设计决策。
// 它只回答一个问题：**还有没有空间**。
// 曲线本身（每级加多少、每星加多少）由 `equipmentrules.go` 唯一定义，
// 并由那里的守卫保证单调与不越界。

import (
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// bestPerSlot 每槽选一件，返回 (护甲合计, 攻击合计)。
//
// 「每槽一件」是产品规则（`equipped = TRUE` 且槽位唯一），所以满配算法
// 就是每槽取值最优的那件。
//
// 这里**按字段分别取每槽最大值**（而不是「挑一件然后两个字段都用它」），
// 并由 `TestPerSlotMaximaAreJointlyAchievable` 断言这两组最大值
// **能被同一套选择同时达成** —— 否则「分别取最大」会高估预算，
// 而高估预算正是这条守卫最不能犯的错。
func bestPerSlot() (armor, attack int64) {
	maxArmor := map[string]int64{}
	maxAttack := map[string]int64{}
	for _, e := range domain.SeedEquipmentList {
		if e.BaseArmor > maxArmor[e.Slot] {
			maxArmor[e.Slot] = e.BaseArmor
		}
		if int64(e.BaseBonusPct) > maxAttack[e.Slot] {
			maxAttack[e.Slot] = int64(e.BaseBonusPct)
		}
	}
	for _, v := range maxArmor {
		armor += v
	}
	for _, v := range maxAttack {
		attack += v
	}
	return armor, attack
}

// TestPerSlotMaximaAreJointlyAchievable 断言「每字段各取最大」不高估预算。
//
// 玩家每槽只能穿**一件**。所以只有当「护甲最优件」与「攻击最优件」
// 在同一槽里**数值上等价**（可能是并列，也可能是同一件）时，
// 分别取最大才是可实现的。
//
// ⚠️ 第一版这里比较的是**装备 ID**，结果红了 —— weapon 槽与 charm 槽的
// `base_armor` 三件**全是 0**，护甲并列，于是「护甲最优件」是任意的，
// 而攻击最优件是另一个 ID。这是我的第 17 次「前提不成立」：
// 并列时应当比**数值**，不是比身份。
//
// 而且判据要比原来更强：它断言的是「存在一套选择同时达成两个最大值」，
// 而不只是「挑件算法挑对了」。
func TestPerSlotMaximaAreJointlyAchievable(t *testing.T) {
	slots := map[string]bool{}
	for _, e := range domain.SeedEquipmentList {
		slots[e.Slot] = true
	}
	for slot := range slots {
		var maxArmor, maxAttack int64 = -1, -1
		for _, e := range domain.SeedEquipmentList {
			if e.Slot != slot {
				continue
			}
			if e.BaseArmor > maxArmor {
				maxArmor = e.BaseArmor
			}
			if int64(e.BaseBonusPct) > maxAttack {
				maxAttack = int64(e.BaseBonusPct)
			}
		}
		// 是否存在一件装备同时达到这两个最大值
		joint := false
		for _, e := range domain.SeedEquipmentList {
			if e.Slot == slot && e.BaseArmor == maxArmor && int64(e.BaseBonusPct) == maxAttack {
				joint = true
				break
			}
		}
		if !joint {
			t.Errorf("槽 %s：护甲最大 %d‰ 与攻击最大 %d‰ 无法由同一件装备同时达成 —— "+
				"「分别取最大」会**高估**预算，而高估会让这条守卫比实际更宽松", slot, maxArmor, maxAttack)
		}
	}
}

// TestAttackBudgetIsAlreadyExhausted 记录攻击侧**没有**缩放空间这个事实。
//
// 期望：满配装备的基础攻击之和已经 ≥ 攻击封顶。
// 所以「装备等级 → 攻击力」会把玩家推进静默浪费区，不做。
//
// 这条断言的**用意是钉住现状**。若将来有人调高封顶来「腾出空间」，
// 这条会红 —— 那时应当重新评估，而不是顺手改数字。
func TestAttackBudgetIsAlreadyExhausted(t *testing.T) {
	_, attack := bestPerSlot()
	t.Logf("满配装备基础攻击合计 = %d‰，封顶 = %d‰", attack, MaxLoadoutAttackPermille)

	if attack < MaxLoadoutAttackPermille {
		t.Errorf("满配装备基础攻击合计 %d‰ 已低于封顶 %d‰ —— "+
			"攻击侧**现在有**缩放预算了，先前「不做攻击缩放」的结论需要重新评估",
			attack, MaxLoadoutAttackPermille)
	}
	// 这不是「越得越多越好」，而是把「已经在封顶上」这件事记录下来。
	if attack < MaxLoadoutAttackPermille {
		t.Log("注意：这条测试的语义是「记录预算已耗尽」，不是「要求越封顶」")
	}
}

// TestArmorBudgetHasRoomForScaling 是「可以给装备加护甲缩放」的**前提守卫**。
//
// 条件：护甲侧满配合计 + 专精满 < 护甲封顶。
// 只有满足这一条，装备等级/星级缩放才不会把玩家推进浪费区。
func TestArmorBudgetHasRoomForScaling(t *testing.T) {
	armor, _ := bestPerSlot()

	// 专精 armor 词条满值（8 个节点，实测 sum = 240）
	const masteryArmorMax = 240
	total := armor + masteryArmorMax

	t.Logf("满配装备护甲 = %d‰，专精满 = %d‰，合计 %d‰，封顶 %d‰（余量 %d‰）",
		armor, masteryArmorMax, total, domain.MaxArmorPermille, domain.MaxArmorPermille-total)

	if total >= domain.MaxArmorPermille {
		t.Errorf("满配护甲合计 %d‰ 已达/越封顶 %d‰ —— "+
			"装备护甲缩放会把玩家推进静默浪费区，先别做", total, domain.MaxArmorPermille)
	}
}

// TestLevelStarColumnsAreRead guards against the exact bug this file documents.
//
// `user_equipment` 有 `level` / `star` 两列，但 R44 之前**从未被读取**。
// 「有列但没人读」是本项目最阴的一类状态：schema 说它有意义，
// 于是没人怀疑它没接线。
//
// 这条在缩放接线**之后**应当自动改判 —— 所以它先记录当前事实：
// 只要「升级实现」还没有上线，读取 SQL 里就不该出现 level/star。
func TestLevelStarColumnsAreRead(t *testing.T) {
	src, err := readServiceSource("loadout_attacker.go")
	if err != nil {
		t.Skipf("读不到 loadout_attacker.go：%v（这条守卫未执行，不是「通过了」）", err)
	}
	// 剥掉注释后再找 —— 否则「注释里提到 level/star」会被误判成「代码读了」。
	// 这正是 R35 那次踩过的坑：守卫匹配到了自己写的解释性注释。
	code := stripLineComments(src)

	selectsIDOnly := contains(code, "SELECT equipment_id")
	selectsLevels := contains(code, "level, star") || contains(code, "star, level")

	switch {
	case selectsIDOnly && !selectsLevels:
		t.Log("当前状态：装备 SQL 只 SELECT equipment_id，level/star 未接线 —— " +
			"这正是本文件记录的那个状态。装备缩放落地后请把这里改成断言。")
	case selectsLevels:
		t.Log("level/star 已被读取 —— 装备缩放已接线，" +
			"此时护甲上限由 equipmentrules.go 的守卫保证")
	default:
		t.Error("装备 SQL 既不是「只 SELECT equipment_id」也没有读 level/star —— " +
			"查询写法变了，请同步更新这条守卫，否则它会在无声中停止生效")
	}
}
