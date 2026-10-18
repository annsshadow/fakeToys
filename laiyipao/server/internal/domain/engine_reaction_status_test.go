package domain

// 「有增伤、无状态」这条路径的直接守卫。
//
// ⚠️ 为什么这条必须单独存在
//
// 历史 bug：`amplify_pct`（受击伤害放大）的生效被一个
// `if statusDurationMs > 0` 把门。实测把那个 `&&` 加回去，
// **五个跨端场景全绿** —— 因为当前 7 条反应里没有任何一条
// 落在「amplify > 0 且 status == 0」这个组合上：
//
//	amplify_pct > 0 的两条（superconduct 600 / flash_freeze 300）
//	    status_duration_ms = 4000 / 2000（都 > 0）
//	status_duration_ms == 0 的其余五条 amplify_pct 全是 0
//
// 这就是「只要输入落不到分界线上，关于分界线的断言都是空的」。
//
// 现在的解法：`applyReactionStatus` 被抽成独立函数后，
// 可以直接喂**构造出来的** HitResult 进去验这条路径 ——
// 不必等到「产品真的加了那条反应」那天才发现代码是错的。

import "testing"

// reactionTableForTest 是给「测试专用」的反应表覆盖。
//
// ⚠️ 为什么不真的往 `reactionSpecs` 里加一条：
//
//	那张表是**服务端 canonical**，`reaction_specs.json` 与 TS 的
//	`REACTIONS` 由 `reaction_contract_test.go` 双向锁定。
//	为了测试往里塞一条假数据，会：
//	  - 让 `/config` 下发给所有客户端一个不存在的反应
//	  - 让 `TestDocsHasNoDrift` 之类的一致性检查失败
//	  - 更糟：让「数据表」与「代码」的责任边界糊掉
//
// 所以这里用一个**包级可替换的钩子**，只在测试期间替换：
// withReactionOverride 临时替换反应表。
func withReactionOverride(t *testing.T, specs map[ReactionKey]ReactionSpec) {
	t.Helper()
	prev := reactionSpecLookup
	reactionSpecLookup = func(k ReactionKey) (ReactionSpec, bool) {
		s, ok := specs[k]
		return s, ok
	}
	t.Cleanup(func() { reactionSpecLookup = prev })
}

// reactionSpecForTest 取测试期间可能被覆盖的规格。
// TestAmplifyAppliesWithoutStatusDuration 直接验解耦后的那条路径。
//
// ⚠️ 判据给出**具体数值**（300），并同时断言「没有状态被施加」——
//
//	只断言前者的话，「顺便也设了 FrozenMs」这种错误也过得去。
func TestAmplifyAppliesWithoutStatusDuration(t *testing.T) {
	withReactionOverride(t, map[ReactionKey]ReactionSpec{
		"fake_amplify_only": {
			Key:              "fake_amplify_only",
			Name:             "测试专用：只有增伤",
			BaseCoef:         50,
			AttackWeightPct:  300,
			AmplifyPct:       300,
			StatusDurationMs: 0, // ← 关键：没有状态
			AoeRadius:        0,
		},
	})

	en := &Enemy{UID: 1, HP: 100, MaxHP: 100}
	applyReactionStatus(en, HitResult{Reaction: "fake_amplify_only"})

	if en.AmplifyPermille != 300 {
		t.Errorf("AmplifyPermille = %d，期望 300 —— "+
			"增伤被 `statusDurationMs > 0` 把门了。"+
			"新增一条「只有增伤、没有状态」的反应时，受击放大永远不会生效，"+
			"而代码看起来完全正常", en.AmplifyPermille)
	}
	// 且**不得**顺带施加任何状态
	if en.FrozenMs != 0 || en.StunnedMs != 0 ||
		en.ArmorShredMs != 0 || en.Knockback != 0 {
		t.Errorf("没有状态的反应不得施加状态：frozen=%d stunned=%d shred=%d knockback=%d",
			en.FrozenMs, en.StunnedMs, en.ArmorShredMs, en.Knockback)
	}
}

// TestStatusAppliesWithoutAmplify 验反向：只有状态、没有增伤。
func TestStatusAppliesWithoutAmplify(t *testing.T) {
	withReactionOverride(t, map[ReactionKey]ReactionSpec{
		"flash_freeze": {
			Key:              "flash_freeze",
			Name:             "急速冻结",
			BaseCoef:         70,
			AttackWeightPct:  300,
			AmplifyPct:       0, // 实际表里是 300，这里刻意设 0 验独立性
			StatusDurationMs: 2000,
		},
	})

	en := &Enemy{UID: 1, HP: 100, MaxHP: 100}
	applyReactionStatus(en, HitResult{Reaction: "flash_freeze", StatusDurationMs: 2000})

	if en.FrozenMs != 2000 {
		t.Errorf("FrozenMs = %d，期望 2000 —— 状态时长应独立生效", en.FrozenMs)
	}
	if en.AmplifyPermille != 0 {
		t.Errorf("AmplifyPermille = %d，期望 0 —— 反应没配增伤就不该有", en.AmplifyPermille)
	}
}

// TestNoReactionAppliesNothing 守住「无反应不查表」。
//
// ⚠️ 这条在 TS 侧是**崩溃防护**：`REACTIONS[”]` 是 undefined，
//
//	紧接着读 `.amplifyPct` 抛 TypeError。
//
// Go 侧 `Spec()` 返回零值，读它不会崩 —— 于是这条守卫的作用是
// 「让『无反应』与『反应字段全 0』在代码上保持可区分」，
// 而不是「防止 panic」。
//
// 判据：零值 ReactionSpec 下，任何分支都不该被进入
// （这正是 TS 靠 `undefined` 得到的效果）。
func TestNoReactionAppliesNothing(t *testing.T) {
	en := &Enemy{UID: 1, HP: 100, MaxHP: 100,
		AmplifyPermille: 42, FrozenMs: 99, StunnedMs: 88}
	applyReactionStatus(en, HitResult{Reaction: ""})
	if en.AmplifyPermille != 42 || en.FrozenMs != 99 || en.StunnedMs != 88 {
		t.Errorf("无反应时不得改动任何状态，实际 amplify=%d frozen=%d stunned=%d",
			en.AmplifyPermille, en.FrozenMs, en.StunnedMs)
	}
}

// TestArmorBreakShredAndKnockback 守住 armor_break 的两项副作用。
func TestArmorBreakShredAndKnockback(t *testing.T) {
	withReactionOverride(t, map[ReactionKey]ReactionSpec{
		"armor_break": {
			Key:                "armor_break",
			Name:               "破甲",
			BaseCoef:           60,
			AttackWeightPct:    300,
			StatusDurationMs:   4000,
			ArmorShredPermille: 250,
			Knockback:          -60,
		},
	})

	en := &Enemy{UID: 1, HP: 100, MaxHP: 100}
	applyReactionStatus(en, HitResult{Reaction: "armor_break", StatusDurationMs: 4000})

	if en.ArmorShredMs != 4000 {
		t.Errorf("ArmorShredMs = %d，期望 4000", en.ArmorShredMs)
	}
	if en.ArmorShredPermille != 250 {
		t.Errorf("ArmorShredPermille = %d，期望 250", en.ArmorShredPermille)
	}
	if en.Knockback != -60 {
		t.Errorf("Knockback = %d，期望 -60 —— 击退是一次性右推（远离防线），"+
			"由 stepEnemyMotion 的 knockback 分支消费后清零", en.Knockback)
	}
	// 击退是**累加**：多次触发应叠加（否则后一次会覆盖前一次）
	en2 := &Enemy{UID: 1, HP: 100, MaxHP: 100}
	applyReactionStatus(en2, HitResult{Reaction: "armor_break", StatusDurationMs: 4000})
	applyReactionStatus(en2, HitResult{Reaction: "armor_break", StatusDurationMs: 4000})
	if en2.Knockback != -120 {
		t.Errorf("两次触发后 Knockback = %d，期望 -120（累加而非覆盖）", en2.Knockback)
	}
}

// TestArmorBreakIgnoresZeroShred 守住「削甲量为 0 时不写状态」。
func TestArmorBreakIgnoresZeroShred(t *testing.T) {
	withReactionOverride(t, map[ReactionKey]ReactionSpec{
		"armor_break": {
			Key:                "armor_break",
			Name:               "破甲",
			BaseCoef:           60,
			AttackWeightPct:    300,
			StatusDurationMs:   4000,
			ArmorShredPermille: 0, // ← 关键：没有削甲
			Knockback:          -60,
		},
	})
	en := &Enemy{UID: 1, HP: 100, MaxHP: 100}
	applyReactionStatus(en, HitResult{Reaction: "armor_break", StatusDurationMs: 4000})
	if en.ArmorShredMs != 0 || en.Knockback != 0 {
		t.Errorf("削甲量为 0 时不得施加削甲与击退，实际 shred=%d knockback=%d",
			en.ArmorShredMs, en.Knockback)
	}
}

// TestStatusDurationZeroBlocksStatus 守住「状态时长为 0 时不施加状态」。
//
// ⚠️ 注意这与「增伤不依赖状态时长」**不矛盾**：
//
//	增伤看 `spec.AmplifyPct`（表里的值）
//	状态看 `res.StatusDurationMs`（结算结果里的值）
//
// 两者是**不同的来源**，这正是它们必须解耦的原因。
func TestStatusDurationZeroBlocksStatus(t *testing.T) {
	withReactionOverride(t, map[ReactionKey]ReactionSpec{
		"overheat": {
			Key: "overheat", Name: "过热",
			BaseCoef: 55, AttackWeightPct: 300,
			StatusDurationMs: 1500,
		},
	})
	// 结算结果里的 StatusDurationMs 为 0（被别的因素压掉了）
	en := &Enemy{UID: 1, HP: 100, MaxHP: 100}
	applyReactionStatus(en, HitResult{Reaction: "overheat", StatusDurationMs: 0})
	if en.StunnedMs != 0 {
		t.Errorf("结算结果里没有状态时长时不得施加眩晕，实际 %d", en.StunnedMs)
	}
}

// TestReactionSpecForTestMirrorsRealTable 守住测试钩子本身不污染真表。
//
// ⚠️ 这条是「测试替身不能变成第二份定义」的守卫：
//
//	`reactionSpecsOverride` 若忘了还原，后续测试会读到假数据，
//	而症状是「某个测试的结论莫名其妙」—— 极难定位。
//	Go 的 `t.Cleanup` 保证了这一点，本测试只是把「它确实生效」钉住。
func TestReactionSpecForTestMirrorsRealTable(t *testing.T) {
	real, ok := reactionSpecLookup("steam_burst")
	if !ok {
		t.Fatal("真实表里应有 steam_burst")
	}
	if real.AoeRadius <= 0 {
		t.Errorf("steam_burst 的 aoe_radius = %d，应为正", real.AoeRadius)
	}

	withReactionOverride(t, map[ReactionKey]ReactionSpec{
		"steam_burst": {Key: "steam_burst", AoeRadius: 1},
	})
	got, _ := reactionSpecLookup("steam_burst")
	if got.AoeRadius != 1 {
		t.Errorf("覆盖期间应读到 aoe_radius=1，实际 %d", got.AoeRadius)
	}
	// withReactionOverride 的 t.Cleanup 在本测试结束时才跑，
	// 所以这里无法验「已还原」。由 t.Cleanup 的语义保证 ——
	// 这里只钉住「覆盖确实生效」，否则覆盖本身是死代码。
}
