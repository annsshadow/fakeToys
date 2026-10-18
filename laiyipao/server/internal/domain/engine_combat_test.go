package domain

// Block D 的**针对性**守卫：专打跨端对齐抓不到的分支。
//
// # 为什么这些不能靠跨端对齐
//
// 每一条都是「当前数据下不会发生，但代码必须正确」的分支。
// 跨端对齐只能验「在真实数据上两端一致」——
// 数据落不到的分支，两端都走「不执行」，于是观察不到差别。
//
// 这一点在 TS 源码注释里已经被明确写下（「只要输入落不到分界线上，
// 关于分界线的断言都是空的」），这里把它变成可执行的守卫。

import "testing"

// TestCurrentAttackerClampsCritPermille 守住暴击率的封顶。
//
// # ⚠️ 这条断言在跨端对齐里是空的
//
// 五个对齐场景的 `critPermille` 都是 150‰ + 零加成 = 150，
// 远不到 1000 的边界 —— 夹取那段代码**一次都没执行过**。
// 实测：把 `min(..., permille)` 去掉，五个场景全绿。
//
// 而越界是**可达**的：局内属性卡里 crit 卡无夹取，
// 最难关卡 10 波、每波恰好 1 张属性卡，
// 6 张 crit 卡就是 550 + 480 = 1030 > 1000。
//
// # 为什么越界后果严重
//
// 消费侧的判据是 `roll >= permille - critPermille`，roll 是 0..999。
// critPermille = 1030 时右边是 **-30**，判据恒真 ——
// **每一发都暴击**。而暴击率进 `replay_hash`，于是：
//
//	客户端按「偶尔暴击」算哈希
//	服务端按「每发都暴击」重算
//	=> 哈希永久失配 => 所有玩家被误判成作弊
func TestCurrentAttackerClampsCritPermille(t *testing.T) {
	e := newProbeEngine()
	// 6 张 crit 卡（每张 80‰）+ 基础 550‰ = 1030 > 1000
	e.Buffs.CritPermille = 6 * 80
	e.Cfg.Attacker.CritPermille = 550

	got := e.currentAttacker().CritPermille
	if got != permille {
		t.Fatalf("critPermille = %d，期望夹到 %d —— "+
			"越界时消费判据 `roll >= 1000 - crit` 恒真（右边是负数），"+
			"于是每一发都暴击，而暴击率进 replay_hash", got, permille)
	}
	// 消费侧的直接后果：roll 的任何取值都判暴击。
	//
	// ⚠️ 这一段是「为什么会永久失配」的具象化：
	//   判据式写在测试里而不是调用生产代码，
	//   因为生产代码里它就是上面那个 min()。
	threshold := permille - got
	for _, roll := range []int64{0, 1, 500, 999} {
		if roll < threshold {
			t.Errorf("roll=%d 应判暴击（阈值 %d），却判了不暴击", roll, threshold)
		}
	}
}

// TestCurrentAttackerCritClampBoundary 守住封顶的**边界**。
func TestCurrentAttackerCritClampBoundary(t *testing.T) {
	cases := []struct {
		base, buff, want int64
	}{
		{0, 0, 0},
		{150, 0, 150},
		{550, 450, 1000}, // 恰好 1000：不夹
		{550, 451, 1000}, // 1001：夹
		{550, 480, 1000}, // 1030：夹（实测可达到的组合）
		{1000, 0, 1000},
	}
	for _, c := range cases {
		e := newProbeEngine()
		e.Cfg.Attacker.CritPermille = c.base
		e.Buffs.CritPermille = c.buff
		if got := e.currentAttacker().CritPermille; got != c.want {
			t.Errorf("base=%d + buff=%d -> %d，期望 %d",
				c.base, c.buff, got, c.want)
		}
	}
}

// TestCurrentAttackerPassesThroughNonDamageFields 守住「三项原样透传」。
//
// ⚠️ `armorPermille` / `heatCapPermille` / `mechanicPermille`
//
//	**不与局内 Buffs 相加**，因为它们的消费者不在伤害公式里：
//	  armorPermille    -> defenseArmorPermille()（防线护甲）
//	  heatCapPermille  -> 构造器注入 heat.capBonus
//	  mechanicPermille -> applyMechanic 放大卡面数值
//
//	在这里相加会让它们**既被计入攻方又被单独消费**一次，
//	而 currentAttacker 的返回值参与重放哈希 —— 多加一次就哈希不符。
//
// 这三条在跨端对齐里也是空的：五个场景的 Buffs 这三项都恒为 0。
func TestCurrentAttackerPassesThroughNonDamageFields(t *testing.T) {
	e := newProbeEngine()
	e.Cfg.Attacker.ArmorPermille = 111
	e.Cfg.Attacker.HeatCapPermille = 222
	e.Cfg.Attacker.MechanicPermille = 333
	// 给 Buffs 全部三项加成 —— 若实现错误地把它们加进来，这里就会红
	e.Buffs.ArmorPermille = 400
	e.Buffs.FreeDiscard = 9

	got := e.currentAttacker()
	if got.ArmorPermille != 111 {
		t.Errorf("armorPermille = %d，期望 111（原样透传，不与 Buffs.ArmorPermille=%d 相加）",
			got.ArmorPermille, e.Buffs.ArmorPermille)
	}
	if got.HeatCapPermille != 222 {
		t.Errorf("heatCapPermille = %d，期望 222（原样透传）", got.HeatCapPermille)
	}
	if got.MechanicPermille != 333 {
		t.Errorf("mechanicPermille = %d，期望 333（原样透传）", got.MechanicPermille)
	}
}

// TestCurrentAttackerLevelOverridesAttacker 守住「关卡优先于攻方」。
//
// ⚠️ `element_cap` 的取值口径是 `lv.element_cap ? lv : base`，
//
//	而 0 在这里表示「关卡没给」而不是「上限为 0」。
//	写成 `lv.element_cap || base.elementCap` 在 JS 里等价，
//	但在 Go 里 `||` 不是运算符 —— 移植时最容易写成
//	「无条件用 lv.ElementCap」，那样关卡缺这个字段时上限变成 0，
//	于是**任何元素都叠不上层**，反应层整体静默失效。
func TestCurrentAttackerLevelOverridesAttacker(t *testing.T) {
	// 关卡给了 -> 用关卡的
	e := newProbeEngine()
	e.Cfg.Level.ElementCap = 5
	e.Cfg.Attacker.ElementCap = 3
	if got := e.currentAttacker().ElementCap; got != 5 {
		t.Errorf("关卡给了 5、攻方 3 时应取 5，实际 %d", got)
	}
	// 关卡没给（0）-> 用攻方的
	e2 := newProbeEngine()
	e2.Cfg.Level.ElementCap = 0
	e2.Cfg.Attacker.ElementCap = 3
	if got := e2.currentAttacker().ElementCap; got != 3 {
		t.Errorf("关卡未给（0）时应回落到攻方的 3，实际 %d —— "+
			"0 在这里是「没给」而不是「上限 0」", got)
	}
	// 再加上局内加成
	e3 := newProbeEngine()
	e3.Cfg.Level.ElementCap = 5
	e3.Buffs.ElementCapBonus = 2
	if got := e3.currentAttacker().ElementCap; got != 7 {
		t.Errorf("关卡 5 + 局内 2 应为 7，实际 %d", got)
	}
}

// TestReactionTierDefaultsToOne 守住反应等级的缺省。
//
// ⚠️ 同样是 `lv.max_reaction_tier || 1` 的移植：
//
//	关卡缺这个字段时必须是 1，不能是 0。
//	`reactionTier` 是反应伤害公式的乘数之一，
//	0 会让**所有反应伤害归零** ——
//	症状是「元素叠上了但反应不放伤害」，看起来像公式写错了。
func TestReactionTierDefaultsToOne(t *testing.T) {
	e := newProbeEngine()
	e.Cfg.Level.MaxReactionTier = 0
	if got := e.currentAttacker().ReactionTier; got != 1 {
		t.Errorf("关卡未给 reaction tier 时应为 1，实际 %d —— "+
			"0 是反应伤害公式的乘数之一，归零会让所有反应不放伤害", got)
	}
	e2 := newProbeEngine()
	e2.Cfg.Level.MaxReactionTier = 3
	if got := e2.currentAttacker().ReactionTier; got != 3 {
		t.Errorf("关卡给了 3 时应为 3，实际 %d", got)
	}
}

// TestAmplifyIsNotGatedOnStatusDuration 钉住「增伤与状态时长是两件独立的事」。
//
// # ⚠️ 实测：把 `if spec.AmplifyPct > 0` 改回
//
//	`if spec.AmplifyPct > 0 && res.StatusDurationMs > 0`，
//	五个跨端场景**全绿**。
//
// # 为什么跨端验不到
//
// 因为当前 7 条反应里**没有任何一条**落在这个组合：
//
//	amplify_pct > 0 的两条（superconduct 600、flash_freeze 300）
//	它们的 status_duration_ms 都是 4000 / 2000（> 0）
//	status_duration_ms == 0 的其余五条 amplify_pct 全是 0
//
// 所以判据「amplify 生效」在今天与「不生效」观察不到差别 ——
// 它是一个**只有在改动之后才会显形**的洞。
//
// # 这条守卫能做什么、不能做什么
//
// 能：钉住「当前数据下两者恰好同时成立」这个事实，
//
//	使得将来新增一条 amplify_pct > 0 且 status_duration_ms == 0 的反应时
//	**本测试会红**，提醒去验解耦后的那条路径。
//
// 不能：证明解耦后的代码是对的（没有这样的反应可验）。
// 所以它必须写成**表征测试**（characterization test）而不是断言行为：
// 断的是「数据」，不是「代码」。
func TestAmplifyIsNotGatedOnStatusDuration(t *testing.T) {
	for _, spec := range AllReactionSpecs() {
		if spec.AmplifyPct <= 0 {
			continue
		}
		if spec.StatusDurationMs <= 0 {
			// 这条一旦出现，说明「增伤被状态时长把门」这个洞
			// 在**数据**上第一次变得可达。
			// 那时本测试应当红，提醒补一条真正验解耦路径的测试。
			t.Errorf("反应 %q 出现了 amplify_pct=%d 但 status_duration_ms=%d 的组合。\n"+
				"这是 `hitEnemy` 里「两件事被捆在一起」的历史 bug 第一次在数据上变得可达。\n"+
				"请补一条场景，让跨端对齐真的走到「有增伤、无状态」这条路径 —— "+
				"当前所有 align 场景都验不到它（那条 `&& statusDurationMs > 0` 加上也不红）。",
				spec.Key, spec.AmplifyPct, spec.StatusDurationMs)
		}
	}
}

// TestSqrtApproxMatchesIntegerRoots 守住整数平方根。
//
// ⚠️ 判据给出**具体数值**，不是「单调递增」这类性质。
//
//	性质守卫看不见符号错误（sqrt(-1) 也能「单调」）。
//	更要紧的是：牛顿法的迭代次数取决于初值，
//	换成另一种开方算法在 `len` 只差 1 的边界上会给出不同弹道方向。
func TestSqrtApproxMatchesIntegerRoots(t *testing.T) {
	cases := []struct{ n, want int64 }{
		{0, 0}, {1, 1}, {2, 1}, {3, 1}, {4, 2}, {8, 2}, {9, 3},
		{15, 3}, {16, 4}, {99, 9}, {100, 10},
		// 真实弹道里出现的量级：距离平方 ~ 10^6
		{1000000, 1000}, {2000000, 1414}, {999999, 999},
	}
	for _, c := range cases {
		if got := sqrtApprox(c.n); got != c.want {
			t.Errorf("sqrtApprox(%d) = %d，期望 %d", c.n, got, c.want)
		}
	}
	// 不变量：对 n >= 2，结果必须满足 x*x == n 或 (x+1)*(x+1) > n。
	//
	// ⚠️ 这个判据**只能**作为补充：它对「向下取整一格」是绿的
	// （(x-1)^2 <= n 也成立），
	// 所以真正的把关靠上面那张具体数值表。
	for n := int64(2); n < 5000; n++ {
		x := sqrtApprox(n)
		if x*x > n {
			t.Errorf("sqrtApprox(%d) = %d，平方已超过 n", n, x)
		}
		if (x+1)*(x+1) <= n {
			t.Errorf("sqrtApprox(%d) = %d，(x+1)^2 仍 <= n —— 结果偏小", n, x)
		}
	}
}

// TestDamageScoreAndKillScore 守住评分换算。
func TestDamageScoreAndKillScore(t *testing.T) {
	rules := DefaultScoreRules()
	if rules.PerDamageUnit <= 0 {
		t.Fatalf("PerDamageUnit = %d，必须为正", rules.PerDamageUnit)
	}
	if got := DamageScore(rules, rules.PerDamageUnit); got != 1 {
		t.Errorf("刚好一个单位 = 1 分，实际 %d", got)
	}
	if got := DamageScore(rules, rules.PerDamageUnit-1); got != 0 {
		t.Errorf("不足一个单位应 0 分，实际 %d", got)
	}
	// 负伤害不得给负分
	if got := DamageScore(rules, -1000); got != 0 {
		t.Errorf("负伤害应 0 分，实际 %d —— 让 `totalDamage / unit` 直接算会得到负分", got)
	}
	if got := DamageScore(rules, 0); got != 0 {
		t.Errorf("零伤害应 0 分，实际 %d", got)
	}
	// 除数为 0 时不得 panic（分数规则来自 /config，是网络数据）
	zero := ScoreRules{PerDamageUnit: 0}
	if got := DamageScore(zero, 1000); got != 0 {
		t.Errorf("PerDamageUnit=0 时应 0 分，实际 %d", got)
	}

	if KillScore(rules, false) != rules.OnKillNormal {
		t.Error("普通怪击杀分应取 OnKillNormal")
	}
	if KillScore(rules, true) != rules.OnKillBoss {
		t.Error("BOSS 击杀分应取 OnKillBoss")
	}
}

// TestReactionIndexStartsAtOne 守住反应编号从 1 起。
//
// ⚠️ 从 0 起的话，`reaction` 重放事件的 `b` 位会是 0，
//
//	而 0 与「无标记」不可区分 —— 于是「这条记录没写编号」与
//	「编号就是 0」在回放里长得一样。
func TestReactionIndexStartsAtOne(t *testing.T) {
	all := AllReactions()
	if len(all) == 0 {
		t.Fatal("反应表为空")
	}
	for i, k := range all {
		if got := ReactionIndex(k); got != int64(i+1) {
			t.Errorf("ReactionIndex(%q) = %d，期望 %d", k, got, i+1)
		}
	}
	// 未知 key -> 0（表示「没有这条反应」）
	if got := ReactionIndex(ReactionKey("not_a_reaction")); got != 0 {
		t.Errorf("未知反应应返回 0，实际 %d", got)
	}
	if got := ReactionIndex(""); got != 0 {
		t.Errorf("空 key 应返回 0，实际 %d", got)
	}
}

// TestLCGRollRange 守住暴击滚点的范围。
//
// ⚠️ 必须是 0..999 **而不是** 0..9999。
//
//	消费侧判据 `roll >= permille - critPermille` 的上界是 1000。
//	范围放大到 9999 时，critPermille=1000 的判据是 `roll >= 0` —— 恒真，
//	每一发都暴击。
//
// ⚠️ 判据用「20000 次取样必须覆盖 >900 个不同值」这个**分布**性质，
//
//	而不是「最大值 < 1000」——
//	后者对「前 20000 次恰好都小于 1000」是绿的。
func TestLCGRollRange(t *testing.T) {
	const n = 20000
	r := newLCG(4242)
	seen := make(map[int64]bool, n)
	min, max := int64(1<<62), int64(-1<<62)
	for i := 0; i < n; i++ {
		v := r.Roll()
		if v < 0 || v > 999 {
			t.Fatalf("第 %d 次 roll = %d，越出 0..999", i, v)
		}
		seen[v] = true
		if v < min {
			min = v
		}
		if v > max {
			max = v
		}
	}
	if len(seen) < 900 {
		t.Errorf("%d 次取样只出现 %d 个不同值 —— roll 分布退化，"+
			"暴击会变成「按固定周期触发」", n, len(seen))
	}
	if min != 0 {
		t.Errorf("roll 的下界应能取到 0，实际最小 %d", min)
	}
	if max != 999 {
		t.Errorf("roll 的上界应能取到 999，实际最大 %d", max)
	}
}

// TestProjectileSubStepsMustStayAboveOne 钉住「子步数不能改成 1」。
//
// ⚠️ 这是表征测试：断的是**意图**而不是当前行为。
//
//	高速弹丸一个 tick 能跨过整只敌人，「线段穿过敌人」被采成
//	「起点在左、终点在右」—— 两个采样点都不在敌人身上，于是漏判。
//	症状是「高弹速技能偶尔完全打不中」，且与弹速正相关，
//	排查时会指向「弹丸速度算错了」，而真正的原因是采样太稀。
//
//	改成 1 时本测试会红，但**不会**告诉你正确值是多少。
//	那需要一条「高速弹丸必须能命中」的实测守卫。
func TestProjectileSubStepsMustStayAboveOne(t *testing.T) {
	if ProjectileSubSteps < 2 {
		t.Fatalf("ProjectileSubSteps = %d —— 改成 1 会让高速弹丸跨过敌人时漏判。"+
			"提高速度时请改这里而不是降低它：子步是运动学层面的补偿，"+
			"与伤害公式无关", ProjectileSubSteps)
	}
}

// TestFilterAliveProjectilesClearsTail 守住「剔除后尾部置 nil」。
//
// ⚠️ `in[:0]` 与 `in` 共享底层数组，所以过滤是**原地压缩**：
//
//	存活元素被移到前部，尾部槽位保留着旧的指针。
//	不清零的话，已死弹丸仍能从底层数组读出来 ——
//	而那正是「死弹丸还在命中」这类 bug 的入口。
//
// ⚠️ 断言必须查**最后一个槽位**，不是索引 0：
//
//	`backing[0]` 在压缩后已经是存活的那只（被移过来了），
//	查它会永远是「非 nil」，于是这条守卫恒绿。
func TestFilterAliveProjectilesClearsTail(t *testing.T) {
	dead := &Projectile{UID: 1, Dead: true}
	alive := &Projectile{UID: 2}
	in := []*Projectile{dead, alive}
	backing := in

	out := filterAliveProjectiles(in)
	if len(out) != 1 || out[0].UID != 2 {
		t.Fatalf("应只剩 uid=2，实际 %d 只", len(out))
	}
	if backing[len(backing)-1] != nil {
		t.Errorf("底层数组尾部未置 nil，backing[%d] = %v —— "+
			"已死弹丸仍可被读到", len(backing)-1, backing[len(backing)-1])
	}
	if backing[0] != alive {
		t.Errorf("backing[0] = %v，期望是存活那只（原地压缩后应移到这里）", backing[0])
	}
}

// TestFilterAliveEnemiesClearsTail 守 enemy's 的同一条。
func TestFilterAliveEnemiesClearsTail(t *testing.T) {
	dead := &Enemy{UID: 1, Dead: true}
	alive := &Enemy{UID: 2}
	in := []*Enemy{dead, alive}
	backing := in

	out := filterAliveEnemies(in)
	if len(out) != 1 || out[0].UID != 2 {
		t.Fatalf("应只剩 uid=2，实际 %d 只", len(out))
	}
	if backing[len(backing)-1] != nil {
		t.Errorf("底层数组尾部未置 nil，backing[%d] = %v",
			len(backing)-1, backing[len(backing)-1])
	}
}

// TestFilterAliveKeepsOrder 守住「过滤不改变存活元素的相对顺序」。
//
// ⚠️ 顺序会影响「同一 tick 内谁先抵达、谁先触发 finish」，
//
//	而 `finish` 会直接返回 —— 于是顺序变了，终局就变了。
//	原地压缩恰好保序（`append` 按遍历顺序追加），
//	但换成「重建一个新切片」或「反向遍历」就会变。
func TestFilterAliveKeepsOrder(t *testing.T) {
	mk := func(uid int64, dead bool) *Enemy { return &Enemy{UID: uid, Dead: dead} }
	in := []*Enemy{mk(1, false), mk(2, true), mk(3, false), mk(4, true), mk(5, false)}

	out := filterAliveEnemies(in)
	got := make([]int64, 0, len(out))
	for _, en := range out {
		got = append(got, en.UID)
	}
	want := []int64{1, 3, 5}
	if len(got) != len(want) {
		t.Fatalf("存活 uid = %v，期望 %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("存活 uid = %v，期望 %v —— 过滤不得改变相对顺序", got, want)
		}
	}
}

// TestSteamBurstSplashIsQuarter 守住 steam_burst 的溅射口径。
//
// ⚠️ `reactionDmg >> 2` 是**右移**（定点除法）：
//
//	大数上 `n/4` 与 `n>>2` 一致，但语义不同 ——
//	`>>2` 在 TS 里是 bigint 的算术右移，移植时写成 `n / 4`
//	在大数上与负数上会分叉（Go 的 `/` 是**向零截断**，不是下取整）。
//	反应伤害恒为正，所以今天观察不到差别 —— 记下来以免将来踩。
//
// 真正要守的是「**不掷骰子**」：溅射若消耗 RNG，
// 后续所有 `Roll()` 的取值都会偏移一格。
func TestSteamBurstSplashIsQuarter(t *testing.T) {
	cases := []struct{ reactionDmg, want int64 }{
		{0, 0}, {1, 0}, {3, 0}, {4, 1}, {5, 1}, {7, 1}, {8, 2},
		{1000, 250}, {1001, 250},
	}
	for _, c := range cases {
		got := c.reactionDmg >> 2
		if got != c.want {
			t.Errorf("%d >> 2 = %d，期望 %d", c.reactionDmg, got, c.want)
		}
	}
	// ⚠️ 溅射半径必须来自反应表，不是硬编码。
	// 写成常量 120 的话，改反应表时不会红 ——
	// 而 `reaction_specs.json` 是服务端 canonical，与 TS 表由契约测试锁定。
	if reactionAoeRadius(ReactionSteamBurst) <= 0 {
		t.Errorf("steam_burst 的 aoe_radius = %d，必须为正 —— "+
			"半径为 0 时 `triggerReactionAoe` 什么都不做，"+
			"而卡面文案仍写着「半径 120 像素内的敌人受到 25%% 范围伤害」",
			reactionAoeRadius(ReactionSteamBurst))
	}
}

// TestSteamBurstSplashDoesNotConsumeRng 守住「溅射不掷骰」。
//
// ⚠️ 这是「二次伤害不用 `hitEnemy`」的**核心**约束。
//
// `hitEnemy` 会调 `e.Rng.Roll()`。若溅射也走它，
// 每次溅射都会多消耗一次 RNG —— 于是后续每一次暴击判定的取值
// 全部偏移一格，整局事件流改变。
//
// 症状极隐蔽：伤害数值完全正确（roll 只影响 crit），
// 只有**暴击分布**变了，而那要跑完整局 + 对比哈希才看得出来。
func TestSteamBurstSplashDoesNotConsumeRng(t *testing.T) {
	// 直接验证 Roll 的消耗次数：同一种子跑两次，
	// 一次带一次不带「溅射」，后续序列必须相同。
	e := newProbeEngine()
	e.Cfg.Enemies = map[int]SeedEnemy{1: {ID: 1, HP: 100, FlyHeight: 0}}
	e.Phase = PhaseWave
	origin := &Enemy{UID: 1, X: 500000, Y: 500000, HP: 1000, MaxHP: 1000,
		SpawnProgress: SpawnProgressFull}
	neighbour := &Enemy{UID: 2, X: 505000, Y: 500000, HP: 1000, MaxHP: 1000,
		SpawnProgress: SpawnProgressFull}
	e.Enemies = []*Enemy{origin, neighbour}

	before := e.Rng.Roll()
	e.Rng = newLCG(20240101)
	_ = before

	att := Attacker{Attack: 100, ReactionMultPermille: 1000, ElementCap: 3,
		ReactionTier: 1, ElementCoefPermille: 1000}
	e.triggerReactionAoe(origin, ReactionSteamBurst, 400, att)

	// 触发溅射之后再取三次：若溅射内部掷过骰子，
	// 这三次会与「同种子下不掷骰」的序列错开。
	r1 := e.Rng.Roll()
	r2 := e.Rng.Roll()

	fresh := newProbeEngine()
	fresh.Rng = newLCG(20240101)
	want1 := fresh.Rng.Roll()
	want2 := fresh.Rng.Roll()
	if r1 != want1 || r2 != want2 {
		t.Errorf("溅射后 RNG 序列 = [%d,%d]，不掷骰时应为 [%d,%d] —— "+
			"溅射消耗了 RNG，后续每次暴击判定的取值都会偏移",
			r1, r2, want1, want2)
	}
}

// TestOverheatSplashUsesOwnRadius 守住 overheat 爆炸的三个口径（第 160 轮）。
//
// # 为什么这条必须存在
//
// `triggerReactionAoe` 的半径读自**触发反应自己的表项**。若实现里写死
// steam_burst 的半径（或共用一个常量），overheat 就会按 120 结算 ——
// 而文案写的是 90。夹具把邻居摆在**两条半径之间**（100px）：
// 120 打到、90 打不到，于是「半径读的是哪条反应」在观测上可分辨。
//
// 另外两个口径一并钉住：
//   - 眩晕（StatusDurationMs）只作用主目标：溅射不写 StunnedMs；
//   - 源体不计：origin 自己不再吃一次伤害。
//
// ⚠️ 对照组（同场景换 steam_burst，100px 邻居**应当**被打到）不可省：
// 没有它，「far 没掉血」可能只是溅射整体坏了 —— 那是另一种红。
func TestOverheatSplashUsesOwnRadius(t *testing.T) {
	att := Attacker{Attack: 100, ReactionMultPermille: 1000, ElementCap: 3,
		ReactionTier: 1, ElementCoefPermille: 1000}

	// 圆心 + 两邻居：5px（两条半径都覆盖）与 100px（steam 120 覆盖、
	// overheat 90 不覆盖）。坐标是定点 ×1000。
	mk := func() (origin, near, far *Enemy) {
		origin = &Enemy{UID: 1, X: 500000, Y: 500000, HP: 10000, MaxHP: 10000,
			SpawnProgress: SpawnProgressFull}
		near = &Enemy{UID: 2, X: 505000, Y: 500000, HP: 10000, MaxHP: 10000,
			SpawnProgress: SpawnProgressFull}
		far = &Enemy{UID: 3, X: 600000, Y: 500000, HP: 10000, MaxHP: 10000,
			SpawnProgress: SpawnProgressFull}
		return
	}

	e := newProbeEngine()
	origin, near, far := mk()
	e.Enemies = []*Enemy{origin, near, far}
	e.triggerReactionAoe(origin, ReactionOverheat, 400, att)

	if origin.HP != 10000 {
		t.Errorf("溅射不得伤到源体：origin.HP = %d", origin.HP)
	}
	if near.HP >= 10000 {
		t.Errorf("5px 邻居应被 overheat 溅射命中（半径 90）：HP = %d", near.HP)
	}
	if far.HP != 10000 {
		t.Errorf("100px 邻居在 overheat 半径（90）之外却吃到了伤害（HP = %d）—— "+
			"半径很可能读成了 steam_burst 的 120", far.HP)
	}
	if near.StunnedMs != 0 {
		t.Errorf("溅射不得扩散眩晕：near.StunnedMs = %d", near.StunnedMs)
	}
	for _, el := range AllElements() {
		if near.Stacks[el] != 0 {
			t.Errorf("溅射不得施加元素层数：near.Stacks[%s] = %d", el, near.Stacks[el])
		}
	}

	// 对照组：同一几何、换成 steam_burst（半径 120），100px 邻居应被打到。
	e2 := newProbeEngine()
	o2, n2, f2 := mk()
	e2.Enemies = []*Enemy{o2, n2, f2}
	e2.triggerReactionAoe(o2, ReactionSteamBurst, 400, att)
	if n2.HP >= 10000 {
		t.Errorf("对照组：5px 邻居应被 steam_burst 溅射命中：HP = %d", n2.HP)
	}
	if f2.HP >= 10000 {
		t.Errorf("对照组：100px 邻居应被 steam_burst（半径 120）命中：HP = %d", f2.HP)
	}
}

// TestPickTargetPrefersEvASiveForMultiTargetSkills 守住索敌优先级。
//
// ⚠️ 「飞行/钻地只能被溅射或穿透命中」这条约束决定了索敌顺序：
//
//	给单体技能安排飞行目标，那一发必然打空。
//	表现为「玩家看到弹丸朝敌人飞过去却什么都没发生」。
func TestPickTargetPrefersEvASiveForMultiTargetSkills(t *testing.T) {
	mkEnemy := func(uid int64, x int64, fly int64, burrow bool) *Enemy {
		return &Enemy{UID: uid, X: x, SpawnProgress: SpawnProgressFull,
			FlyHeight: fly, Burrow: burrow}
	}
	// 地面怪更靠近防线（x 小），飞行/钻地怪更远
	ground := mkEnemy(1, 100, 0, false)
	fly := mkEnemy(2, 900, 100, false)
	burrow := mkEnemy(3, 800, 0, true)

	e := newProbeEngine()
	e.Enemies = []*Enemy{ground, fly, burrow}

	// 单体技能：取最靠近防线的 -> ground
	if got := e.pickTarget(&EquippedSkill{Pierce: 0, AoeRadius: 0}); got.UID != 1 {
		t.Errorf("单体技能应选最近的地面怪（uid=1），实际 uid=%d", got.UID)
	}
	// 有溅射的技能：优先给飞行/钻地里最靠近的 -> burrow（x=800 < fly.x=900）
	if got := e.pickTarget(&EquippedSkill{Pierce: 0, AoeRadius: 100}); got.UID != 3 {
		t.Errorf("溅射技能应优先选钻地怪（uid=3, x=800），实际 uid=%d", got.UID)
	}
	// 有穿透的技能：同样优先多目标能力
	if got := e.pickTarget(&EquippedSkill{Pierce: 2, AoeRadius: 0}); got.UID != 3 {
		t.Errorf("穿透技能应优先选钻地怪（uid=3），实际 uid=%d", got.UID)
	}

	// 全场都是飞行怪时，单体技能也应选最近的那个（而不是 nil）
	allFly := newProbeEngine()
	allFly.Enemies = []*Enemy{mkEnemy(1, 900, 50, false), mkEnemy(2, 300, 50, false)}
	if got := allFly.pickTarget(&EquippedSkill{}); got == nil || got.UID != 2 {
		t.Errorf("全场飞行时单体技能应选最近的（uid=2），实际 %v", got)
	}

	// 空场 -> nil
	empty := newProbeEngine()
	if got := empty.pickTarget(&EquippedSkill{}); got != nil {
		t.Errorf("空场应返回 nil，实际 uid=%d", got.UID)
	}
}

// TestPickTargetSkipsSpawningEnemies 守住「跳过正在出场的敌人」。
func TestPickTargetSkipsSpawningEnemies(t *testing.T) {
	e := newProbeEngine()
	e.Enemies = []*Enemy{
		{UID: 1, X: 100, SpawnProgress: 0},                 // 正在出场
		{UID: 2, X: 900, SpawnProgress: SpawnProgressFull}, // 正常
	}
	got := e.pickTarget(&EquippedSkill{})
	if got == nil || got.UID != 2 {
		t.Fatalf("应跳过正在出场的 uid=1，实际 %v", got)
	}
	// 全部还在出场 -> nil
	e2 := newProbeEngine()
	e2.Enemies = []*Enemy{{UID: 1, X: 100, SpawnProgress: SpawnProgressStep}}
	if got := e2.pickTarget(&EquippedSkill{}); got != nil {
		t.Errorf("全部在出场中时应返回 nil，实际 uid=%d", got.UID)
	}
}

// TestCurrentAttackerAddsBuffAttack 守住 attack 与局内加成的相加。
//
// ⚠️ 这条在跨端对齐里是空的：
//
// Buffs 是 F 块（卡牌）写的，而 F 块尚未移植 ——
// 于是五个场景的 `Buffs.AttackPermille` 恒为 0，
// 实测把 `base.Attack + e.Buffs.AttackPermille` 改成 `base.Attack` 全绿。
//
// 而它**会**被读到：卡牌「弹药强化：攻击力 +X%」是常驻属性卡之一，
// 而 currentAttacker 的返回值参与重放哈希 —— 漏加就是哈希不符。
func TestCurrentAttackerAddsBuffAttack(t *testing.T) {
	e := newProbeEngine()
	e.Cfg.Attacker.Attack = 100
	e.Buffs.AttackPermille = 25
	if got := e.currentAttacker().Attack; got != 125 {
		t.Errorf("attack = %d，期望 100+25=125 —— 漏加会让所有攻击力卡静默失效，"+
			"而 currentAttacker 的返回值进重放哈希", got)
	}
	// 与 crit/elementCoef 同构：也要确认负值/零不被特殊处理
	e2 := newProbeEngine()
	e2.Cfg.Attacker.Attack = 100
	if got := e2.currentAttacker().Attack; got != 100 {
		t.Errorf("无加成时 attack 应为 100，实际 %d", got)
	}
	e3 := newProbeEngine()
	e3.Cfg.Attacker.Attack = 100
	e3.Buffs.AttackPermille = -10
	if got := e3.currentAttacker().Attack; got != 90 {
		t.Errorf("负加成 -10 时 attack 应为 90，实际 %d", got)
	}
}

// TestCurrentAttackerAddsBuffElementCoef 守住元素系数的相加。
func TestCurrentAttackerAddsBuffElementCoef(t *testing.T) {
	e := newProbeEngine()
	e.Cfg.Attacker.ElementCoefPermille = 1000
	e.Buffs.ElementCoefPermille = 150
	if got := e.currentAttacker().ElementCoefPermille; got != 1150 {
		t.Errorf("elementCoefPermille = %d，期望 1150", got)
	}
}

// TestCurrentAttackerAddsBuffCrit 守住暴击率与局内加成的相加。
//
// ⚠️ 注意与 `TestCurrentAttackerClampsCritPermille` 配对：
//
//	那是「相加**之后**要夹」，这条是「相加本身不能漏」。
//	两者缺一不可：只测夹取会漏掉「漏加」，只测相加会漏掉「没夹」。
func TestCurrentAttackerAddsBuffCrit(t *testing.T) {
	e := newProbeEngine()
	e.Cfg.Attacker.CritPermille = 100
	e.Buffs.CritPermille = 30
	if got := e.currentAttacker().CritPermille; got != 130 {
		t.Errorf("critPermille = %d，期望 100+30=130", got)
	}
}

// TestCurrentAttackerAddsBuffReactionMult 守住反应倍率的相加。
func TestCurrentAttackerAddsBuffReactionMult(t *testing.T) {
	e := newProbeEngine()
	e.Cfg.Attacker.ReactionMultPermille = 1000
	e.Buffs.ElementCoefPermille = 0
	// 反应倍率没有 Buffs 项 —— 确认它原样透传
	if got := e.currentAttacker().ReactionMultPermille; got != 1000 {
		t.Errorf("reactionMultPermille = %d，期望 1000（原样透传）", got)
	}
}
