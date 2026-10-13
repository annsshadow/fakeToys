package service

// 宝石词条数值的标定守卫（第 152 轮）。
//
// # 为什么标定要写成测试而不是写在注释里
//
// 注释里写「一颗金宝石 ≈ 上限的 20%–50%」没有任何约束力 ——
// 半年后有人把 crit 区间调成 40–160，注释还在，测试也还是绿的，
// 而游戏已经变了。数值标定只有变成断言才防得住漂移。
//
// 这与 `internal/domain/balance_scale_pins_test.go` 是同一个做法：
// 那里钉的是 `EnemyHpScale == 8`，这里钉的是「宝石能占各属性上限的多少」。
//
// # 本文件钉的三件事
//
//	① 装备/专精已经占了多少、还剩多少空间（词条池的取值依据）
//	② 每种词条一颗金宝石能吃到该属性上限的多少（区间本身）
//	③ `element_cap` 永远不随品质放大（天花板 ×2 是浪费）
//
// 第 ① 条与 `equip_budget_test.go` 的两条测试**互为对照**：
// 那两条说「装备已占多少」，这里说「因此宝石最多能再占多少」。

import (
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// gemAffixCap 是各属性由宝石贡献时的**硬上限**，取自 `applyLoadout` 用的常量。
//
// ⚠️ 这份表必须与 `loadout_attacker.go` 的 `MaxLoadout*` 同步。
// 不同步的后果是**宝石溢出后被静默 clamp**：玩家看到词条却没生效，
// 而服务端不报错（`clampPermille` 只是取 min）。
var gemAffixCap = map[string]int64{
	"attack":        MaxLoadoutAttackPermille,       // 1000
	"armor":         domain.MaxArmorPermille,        // 750
	"crit":          MaxLoadoutCritPermille,         // 500
	"element_coef":  MaxLoadoutElementCoefPermille,  // 1000
	"reaction_mult": MaxLoadoutReactionMultPermille, // 800
	"heat_cap":      MaxLoadoutHeatCapPermille,      // 600
	// skill_damage 在 `loadout_attacker` 里也加到 Attack，走同一个 clamp。
	// 不登记它，它就会走「宝石独占」分支，被按一个它根本没有的上限来审 ——
	// 而实际上它和 attack 共享同一份预算。
	"skill_damage": MaxLoadoutAttackPermille,
}

// gemEquippedSlots 是玩家最多能同时装备几颗宝石。
//
// 宝石与装备共用同一个 clamp 预算，所以「几颗」决定了几颗之后预算见底。
// 取 3：4 件装备已把 attack 占满，宝石位给到 4 就等于白送一个 attack 位。
const gemEquippedSlots = 3

// TestGemAffixBudgetsMatchEquipmentHeadroom 钉住「装备占多少、宝石还剩多少」。
//
// 这是第 ② 条的依据：如果哪天装备的 `base_armor` 涨了，
// 护甲词条的 295 空间就没了，区间要跟着重算。
func TestGemAffixBudgetsMatchEquipmentHeadroom(t *testing.T) {
	armorFromGear, attackFromGear := bestPerSlot()

	// attack：装备已经**超过**上限 —— 这条决定了 attack 词条必须给得很小。
	if attackFromGear < MaxLoadoutAttackPermille {
		t.Errorf("最佳装备的攻击贡献 = %d，已低于上限 %d —— "+
			"「attack 词条在满装备后价值为 0」这条前提已不成立，"+
			"SeedGemAffixes 里的 attack 区间要重新标定",
			attackFromGear, MaxLoadoutAttackPermille)
	}

	// armor：装备 + 专精仍留有缺口 —— 这条决定了 armor 词条值得给。
	// 专精 armor 上限取 8 节点满配的 240（与 equip_budget_test 同值）。
	const masteryArmorMax = 240
	armorHeadroom := domain.MaxArmorPermille - (armorFromGear + masteryArmorMax)
	if armorHeadroom <= 0 {
		t.Errorf("最佳装备 + 专精的护甲合计 = %d，已达上限 %d —— "+
			"「armor 词条填永久缺口」这条前提已不成立，"+
			"SeedGemAffixes 里的 armor 区间要重新标定",
			armorFromGear+masteryArmorMax, domain.MaxArmorPermille)
	}
	t.Logf("[标定] attack 装备贡献 %d / 上限 %d（已超 %d，满配后宝石攻击无效）；"+
		"armor 装备 %d + 专精 %d = %d / 上限 %d（余 %d）",
		attackFromGear, MaxLoadoutAttackPermille, attackFromGear-MaxLoadoutAttackPermille,
		armorFromGear, masteryArmorMax, armorFromGear+masteryArmorMax,
		domain.MaxArmorPermille, armorHeadroom)
}

// TestGemAffixRangesFitTheBudgets 每种词条的「满配金宝石」贡献落在预算内。
//
// 判据是**三颗金宝石**（= gemEquippedSlots）的最坏情况总贡献：
//
//	attack / skill_damage  → 必须显著低于装备已占的部分，
//	                          否则玩家看到词条却因为 clamp 而毫无变化
//	其余（宝石独占）        → 必须占上限的 5%–50%
//
// 上下限都断言：只有上限时，一个被调到 0 的区间也能过。
func TestGemAffixRangesFitTheBudgets(t *testing.T) {
	goldMult := domain.GemQualityMult("gold") // 2000
	if goldMult != 2000 {
		t.Fatalf("标定假设「金 = 灰的 2 倍」（mult=2000），实际 %d —— "+
			"若 mult 改了，下面所有区间都要重算", goldMult)
	}

	// 装备/专精已经先占掉的部分，用来判断 attack 还剩多少空间。
	armorFromGear, attackFromGear := bestPerSlot()
	const masteryArmorMax = 240
	occupied := map[string]int64{
		"attack": attackFromGear,
		"armor":  armorFromGear + masteryArmorMax,
	}

	for affix, cap := range gemAffixCap {
		// 取**最大值**而不是最后一次赋值。
		// 同一条词条会被多种宝石提供（`crit` 出现在 5 种宝石上），
		// 写成赋值的话这里量到的是词条表里**最后一条**的区间，
		// 而不是最坏的一条 —— 那种守卫看起来在管预算，其实只管了顺序。
		// （这个洞是被变异测试发现的：把 gem_atk 的 attack 调大一倍，
		//   它恰好是词条表里第一条，守卫完全没反应。）
		worst := int64(0)
		for _, a := range domain.SeedGemAffixes {
			if a.Affix != affix {
				continue
			}
			if v := int64(a.MaxValue) * goldMult / 1000; v > worst {
				worst = v
			}
		}
		if worst == 0 {
			t.Errorf("词条 %q 在 SeedGemAffixes 里没有任何宝石提供 —— "+
				"它已被登记进 gemAffixCap（说明有预算），却没有获取途径", affix)
			continue
		}
		full := worst * gemEquippedSlots

		prev, sharedWithGear := occupied[affix]
		if sharedWithGear {
			room := cap - prev
			if room > 0 {
				// 装备没占满：宝石满配总量不得超过剩余预算的一半。
				// 超过就意味着「一套宝石挤掉了装备」—— 而装备更便宜也更好刷。
				if full > room/2 {
					t.Errorf("词条 %q 满配 %d 超过剩余预算 %d 的一半 —— "+
						"满配后它会挤掉装备的价值，而装备更便宜也更好刷",
						affix, full, room)
				}
				continue
			}

			// room ≤ 0：满配玩家身上这条词条**完全无效**（被 clampPermille 吃掉）。
			//
			// 这里刻意**不报错**，而是改判「早期预算」：
			// `user_equipment` 初期是空的，那段时间装备贡献 0，
			// 整份上限都还在。所以这条词条对**新玩家**有意义，
			// 前提是它满配也不该大到让后期玩家为它付钱。
			//
			// 判据取上限的 25%：够新玩家感觉到差别，
			// 又小到后期玩家即使装了也只是「锦上添花」而不是「买了就变强」。
			if full*4 > cap {
				t.Errorf("词条 %q 的装备贡献 %d 已达/超上限 %d（满配后完全无效），"+
					"但它满配仍有 %d（上限的 %d%%）—— "+
					"后期玩家花钱买到的是零效果，早期玩家也被抬得太高",
					affix, prev, cap, full, full*100/cap)
			}
			continue
		}

		// 宝石独占：满配应占上限的 5%–50%。
		// 下限防止有人把区间调到接近 0（那样宝石等于没给东西），
		// 上限防止一条宝石顶掉半个装备条。
		if full*20 < cap {
			t.Errorf("词条 %q 满配 %d 不足上限 %d 的 5%% —— "+
				"满配三颗金宝石都几乎看不出效果，宝石就失去了存在理由",
				affix, full, cap)
		}
		if full*2 > cap {
			t.Errorf("词条 %q 满配 %d 超过上限 %d 的一半 —— 一套宝石能顶掉半个装备条",
				affix, full, cap)
		}
	}
}

// TestEveryGemHasAtLeastOneWorthwhileAffix 每种宝石至少有一条「值得」的词条。
//
// 为什么单列一条：`TestGemAffixRangesFitTheBudgets` 量的是**全表最坏值**
// （同一条词条被 5 种宝石提供时，取其中最大的那个）。它的盲区是
// 「某一种宝石整体被调残」—— 把 gem_crit 的 crit 从 15–40 改成 1–2
// 不会改变全表最坏值，于是上一条测试完全无反应，而 gem_crit 这个名字
// 已经在对玩家承诺「暴击石」。
//
// 判据：每种宝石的三条词条里，至少有一条的金宝石最大值达到该属性上限的 10%。
func TestEveryGemHasAtLeastOneWorthwhileAffix(t *testing.T) {
	goldMult := domain.GemQualityMult("gold")
	armorFromGear, attackFromGear := bestPerSlot()
	const masteryArmorMax = 240
	// 已被装备/专精吃满的属性：满配玩家身上它们是**零效果**，
	// 不能拿来证明「这颗宝石值得」—— 那等于给一件只有新玩家用得上的
	// 东西发合格证。
	deadToGear := map[string]bool{
		"attack":       attackFromGear >= MaxLoadoutAttackPermille,
		"skill_damage": attackFromGear >= MaxLoadoutAttackPermille,
		"armor":        armorFromGear+masteryArmorMax >= domain.MaxArmorPermille,
	}

	// element_cap 没有 permille 上限可比（它是绝对档位数），
	// 它的判据在 TestGemElementCapIsNeverQualityScaled 里。
	if _, bad := gemAffixCap["element_cap"]; bad {
		t.Error("element_cap 被登记进了 gemAffixCap —— " +
			"它是绝对档位数不是千分比，不该按上限比例审")
	}

	for _, g := range domain.SeedGems {
		worthwhile := ""
		for _, a := range domain.SeedGemAffixes {
			if a.GemID != g.ID || domain.UnscaledGemAffixes[a.Affix] || deadToGear[a.Affix] {
				continue
			}
			cap, known := gemAffixCap[a.Affix]
			if !known {
				// 词条名认不认识由 TestGemAffixPoolIsSane 负责，这里不重复
				continue
			}
			full := int64(a.MaxValue) * goldMult / 1000 * gemEquippedSlots
			if full*10 >= cap {
				worthwhile = a.Affix
				break
			}
		}
		if worthwhile == "" {
			t.Errorf("宝石 %s(%d) 没有一条值得的词条：满配三颗金宝石对任何"+
				"「满配后仍有效」的属性的贡献都不到该上限的 10%% —— "+
				"它卖 %d 钻石却给不出效果",
				g.Code, g.ID, domain.GemShopPrice)
		}
	}
}
func TestGemElementCapIsNeverQualityScaled(t *testing.T) {
	if !domain.UnscaledGemAffixes["element_cap"] {
		t.Fatal("element_cap 不在 UnscaledGemAffixes 里 —— " +
			"品质放大后总元素上限会超过 MaxElementCap，超出的部分被静默丢弃")
	}

	gold := domain.GemQualityMult("gold")
	gray := domain.GemQualityMult("gray")
	// 满配三颗金宝石 + base 3 应**不超过** MaxElementCap，
	// 否则内容表里必然有一档 roll 是纯浪费。
	maxPerGem := 0
	for _, a := range domain.SeedGemAffixes {
		if a.Affix == "element_cap" && a.MaxValue > maxPerGem {
			maxPerGem = a.MaxValue
		}
	}
	ceiling := domain.DefaultAttacker().ElementCap
	total := ceiling + int64(maxPerGem)*gemEquippedSlots
	if total > domain.MaxElementCap {
		t.Errorf("元素上限满配 = %d + %d×%d = %d，超过 MaxElementCap %d —— "+
			"最后一颗宝石的一部分 roll 是纯浪费",
			ceiling, int64(maxPerGem), gemEquippedSlots, total, domain.MaxElementCap)
	}
	if gray != 1000 || gold != 2000 {
		t.Errorf("品质 mult 变了（gray=%d gold=%d），本文件的区间推算失效", gray, gold)
	}
}
