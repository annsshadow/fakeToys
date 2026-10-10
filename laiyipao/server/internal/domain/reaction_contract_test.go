package domain

import (
	"encoding/json"
	"testing"
)

// 反应链数值表的不变量测试。
//
// ⚠️ 这里**刻意没有**「Go 表 == 契约文件」的断言。
// 那样的测试是自证：契约文件由 cmd/vectors 从同一张表导出，
// 读回来对比必然通过，测不出任何东西。
//
// 真正的跨端锁在客户端：miniapp/src/game/reaction_contract.test.ts 读同一个
// testdata/reaction_specs.json，与 miniapp/src/game/elements.ts 的 REACTIONS
// 常量对比。两份手工同步的副本里，改了任意一侧都会让**另一侧**红。
//
// 本文件负责的是另一类东西 —— 与「跨端一致」无关的**设计约束**：
// 反通胀红线、展示文案非空、key 可 JSON 往返。
// 这些性质哪怕两端一起改坏也必须被挡住。

// TestStarTargetsReachable 固化「星级门槛必须真的够得着」。
//
// ⚠️ 这个断言的定位：它检查的是**引擎与生成器的口径是否对齐**，
// 而不是「三个门槛递增」这种谁都能满足的弱性质。
//
// 曾经 levelgen 用 `每只怪 2000 分` 估算满分，而引擎实际只给
// 击杀 500/5000 + 伤害 totalDamage/100。第 1 关：
//
//	引擎理论上限 19542，1 星门槛 78000 —— 差 58458，恒 0 星。
//
// 同时 StarTargetRatio 是 [1000,1200,1400]，2/3 星要求**超过满分**，
// 于是 KeysOnThreeStar 这把钥匙没有任何合法获取路径。
//
// 两处偏差都"各自自洽"（Go 只生成、TS 只加分），
// 13 个公式一致性向量全过、levelgen_test 只断言"有 3 档且递增"。
// 本测试是唯一能发现这类跨端口径漂移的地方。
func TestStarTargetsReachable(t *testing.T) {
	for id := 1; id <= 100; id++ {
		gl := GenerateLevel(id)

		// 独立重算一遍引擎口径的理论满分，不复用生成器的中间变量 ——
		// 复用就变成自证了。
		var want int64
		for _, w := range gl.Waves {
			for _, sp := range w.Spawns {
				want += int64(sp.Count) * enemyScoreUpperBound(sp.EnemyID)
			}
		}
		if want <= 0 {
			t.Fatalf("第 %d 关理论满分为 %d，无法计算门槛", id, want)
		}

		one, two, three := gl.StarTargets[0], gl.StarTargets[1], gl.StarTargets[2]
		// 3 星必须够得着（否则三星钥匙是死配置）
		if three > want {
			t.Errorf("第 %d 关: 3 星门槛 %d 超过理论满分 %d —— 不可达", id, three, want)
		}
		// 3 星应当是个"努力可得"的目标：至少 40% 满分
		if three < want*2/5 {
			t.Errorf("第 %d 关: 3 星门槛 %d 低于理论满分的 40%%（%d）—— 白送", id, three, want*2/5)
		}
		// 1 星必须明显低于 3 星，否则星级没有区分度
		if one >= three {
			t.Errorf("第 %d 关: 1 星 %d 未低于 3 星 %d", id, one, three)
		}
		// 单调递增
		if !(one < two && two < three) {
			t.Errorf("第 %d 关: 门槛非递增 [%d %d %d]", id, one, two, three)
		}
		// 1 星不能要求打满（那是 3 星才该有的苛刻度）
		if one >= want {
			t.Errorf("第 %d 关: 1 星门槛 %d 已达理论满分 %d —— 恒 0 星", id, one, want)
		}
	}
}

// TestEnemyScoreUpperBoundMatchesEngineEngine 确认口径常量与引擎一致。
//
// 数值是人工从 engine.ts 抄来的（那里的两处 this.score +=）：
//
//	命中：Number(res.totalDamage / 100n)
//	击杀：e.isBoss ? 5000 : 500
//
// 这条测试的作用是「一旦有人改了引擎给分却没改这里，门槛会静默失真」——
// 它至少能让常量与文档保持同步，真正的对齐由 TS 侧的冒烟测试兜底。
func TestScoreConstantsMatchEngine(t *testing.T) {
	if scorePerDamageUnit != 100 {
		t.Errorf("每 %d 点伤害记 1 分，与 engine.ts 的 /100n 不符", scorePerDamageUnit)
	}
	if scoreOnKillNormal != 500 {
		t.Errorf("普通怪击杀 %d 分，与 engine.ts 的 500 不符", scoreOnKillNormal)
	}
	if scoreOnKillBoss != 5000 {
		t.Errorf("BOSS 击杀 %d 分，与 engine.ts 的 5000 不符", scoreOnKillBoss)
	}
}

// TestEnemyScoreUpperBoundUsesTable 确认查表生效而不是永远返回兜底值。
//
// 兜底值（找不到就返回 500）会让「查表失败」这件事静默变成
// 「门槛偏低但看起来正常」。
func TestEnemyScoreUpperBoundUsesTable(t *testing.T) {
	seen := map[int]bool{}
	for _, e := range SeedEnemies {
		got := enemyScoreUpperBound(e.ID)
		seen[e.ID] = true
		want := int64(scoreOnKillNormal)
		if e.IsBoss {
			want = scoreOnKillBoss
		}
		// ⚠️ 期望值必须用**缩放后**的血量。
		//
		// 引擎（engine.ts）里 `this.score += Number(res.totalDamage / 100n)`
		// 累加的是**实际造成的伤害**，而实际伤害对应缩放后的血量（×8）。
		// 估算用基准血量就与实际差 8 倍的伤害分 ——
		// 这条测试曾经也用基准血量，于是「实现与测试同时用同一个错的口径」，
		// 13 个公式一致性向量也照样全绿。
		//
		// 这与 README 里「跨端一致 ≠ 两端都对」是同一类：
		// 一致性锁住了「没漂移」，但没锁住「对不对」。
		scaled := ScaleEnemy(e)
		want += (scaled.HP + scaled.ShieldHP) / scorePerDamageUnit
		if got != want {
			t.Errorf("敌人 %d: 得分上界 %d，期望 %d（缩放后血量 %d）",
				e.ID, got, want, scaled.HP)
		}
	}
	if len(seen) != len(SeedEnemies) {
		t.Errorf("只覆盖了 %d/%d 个敌人", len(seen), len(SeedEnemies))
	}
	// 未登记的 id 返回兜底下界，且必须为正（返回 0 会让门槛塌到 0 分）
	if got := enemyScoreUpperBound(999999); got != scoreOnKillNormal {
		t.Errorf("未登记 id 的兜底值 %d，应为 %d", got, scoreOnKillNormal)
	}
}

// TestStarTargetRatioIsMonotonicAndSane 确认比例常量本身的合理性。
func TestStarTargetRatioIsMonotonicAndSane(t *testing.T) {
	for i, r := range StarTargetRatio {
		if r <= 0 {
			t.Errorf("第 %d 档比例 %d 非正", i, r)
		}
		if r > permille {
			t.Errorf("第 %d 档比例 %d 超过 100%%（%d）—— 要求超过理论满分，永远不可达",
				i, r, permille)
		}
		if i > 0 && r <= StarTargetRatio[i-1] {
			t.Errorf("第 %d 档比例 %d 未高于上一档 %d", i, r, StarTargetRatio[i-1])
		}
	}
}

// TestReactionAttackWeightRedLine 固化 I-1 的反通胀红线。
//
// 这条断言的价值在于它**不依赖**两端一致：即使有人同时改掉契约文件、
// Go 表和 TS 常量，让三处"重新一致"，暴击权重超过 300‰ 时本测试仍会红。
//
// 反应伤害中由面板攻击力贡献的比例是整个设计的核心约束。
// 一旦它超过 30%，玩家堆攻击力就能线性放大全部反应伤害，
// 元素反应矩阵（I-1）会退化成"攻击力乘区"，五元素构筑失去意义。
func TestReactionAttackWeightRedLine(t *testing.T) {
	specs := AllReactionSpecs()
	if len(specs) == 0 {
		t.Fatal("反应表为空")
	}
	for _, s := range specs {
		if s.AttackWeightPct > 300 {
			t.Errorf("%s: 攻击力权重 %d‰ 超过 300‰ 红线，I-1 的反数值通胀保证被破坏",
				s.Key, s.AttackWeightPct)
		}
		if s.AttackWeightPct < 0 {
			t.Errorf("%s: 攻击力权重为负 %d‰", s.Key, s.AttackWeightPct)
		}
	}
}

// TestReactionStructuralInvariants 挡掉「文档站整列为空」这类静默缺陷。
//
// Name / Descr 是运营后台玩法文档站的直接数据源。
// 漏填不会让任何代码报错，只会在文档站上留下一列空白 ——
// 而文档站的定位恰恰是「读同一份 /config 保证说明与线上版本一致」，
// 一列空白正好摧毁它存在的理由。
func TestReactionStructuralInvariants(t *testing.T) {
	for _, s := range AllReactionSpecs() {
		if s.Name == "" {
			t.Errorf("%s: Name 为空，文档站「反应」列会是空白", s.Key)
		}
		if s.Descr == "" {
			t.Errorf("%s: Descr 为空，文档站「效果」列会是空白", s.Key)
		}
		if s.BaseCoef <= 0 {
			t.Errorf("%s: BaseCoef 必须为正（千分比），实际 %d", s.Key, s.BaseCoef)
		}
		if s.AoeRadius < 0 {
			t.Errorf("%s: AoeRadius 为负 %d", s.Key, s.AoeRadius)
		}
		if s.AmplifyPct < 0 {
			t.Errorf("%s: AmplifyPct 为负 %d‰", s.Key, s.AmplifyPct)
		}
		if s.ArmorShredPermille < 0 {
			t.Errorf("%s: ArmorShredPermille 为负 %d‰", s.Key, s.ArmorShredPermille)
		}
		if s.Knockback < 0 {
			t.Errorf("%s: Knockback 为负 %d", s.Key, s.Knockback)
		}
	}
}

// TestArmorBreakEffectIsExclusive 把「削甲与击退是 armor_break 专属效果」立成不变式。
//
// 引擎侧（engine.ts 的 hitEnemy）只对 react === 'armor_break' 写
// armorShredMs / armorShredPermille / knockback。若将来给**别的**反应
// 配上 ArmorShredPermille / Knockback 而引擎没有对应分支，那个反应就会
// 「字段有值但从未生效」—— 与第 12 轮记的「reactionMultPermille 空转」同形：
// 数值看着在 /config 里流动，战斗里却什么都没发生。
//
// 所以这张表必须满足：除 armor_break 外，削甲与击退**全为 0**；
// armor_break 三者（时长、削甲量、击退量）**全为正** —— 三件缺一即半接线。
// 变异验证思路：给任意一条非 armor_break 反应配上非 0 削甲/击退（引擎无分支），
// 或把 armor_break 任一项归零，本测试立刻红。
func TestArmorBreakEffectIsExclusive(t *testing.T) {
	for _, s := range AllReactionSpecs() {
		if s.Key == ReactionArmorBreak {
			if s.StatusDurationMs <= 0 {
				t.Errorf("armor_break: StatusDurationMs %d 必须为正（削甲持续时长）", s.StatusDurationMs)
			}
			if s.ArmorShredPermille <= 0 {
				t.Errorf("armor_break: ArmorShredPermille %d 必须为正（削甲量），否则时长字段是空的", s.ArmorShredPermille)
			}
			if s.Knockback <= 0 {
				t.Errorf("armor_break: Knockback %d 必须为正（击退位移），「破甲击退」只剩半个", s.Knockback)
			}
			continue
		}
		if s.ArmorShredPermille != 0 {
			t.Errorf("%s: ArmorShredPermille %d 非 0，但引擎只对 armor_break 消费削甲 —— 数值在空转",
				s.Key, s.ArmorShredPermille)
		}
		if s.Knockback != 0 {
			t.Errorf("%s: Knockback %d 非 0，但引擎只对 armor_break 消费击退 —— 数值在空转",
				s.Key, s.Knockback)
		}
	}
}

// TestReactionKeyIsPlainASCII 确认 key 是可 JSON 往返的普通字符串。
//
// ReactionKey 会作为 map key、URL 片段、el-table 的 :key 出现在多处。
// 带非 ASCII 或控制字符会在这些地方产生难查的问题。
func TestReactionKeyIsPlainASCII(t *testing.T) {
	for _, s := range AllReactionSpecs() {
		k := string(s.Key)
		if k == "" {
			t.Errorf("存在空 key 的反应规格")
			continue
		}
		for i := 0; i < len(k); i++ {
			c := k[i]
			if c < 0x20 || c > 0x7e {
				t.Errorf("key %q 含非可打印 ASCII 字节 0x%02x", k, c)
				break
			}
		}
	}
}

// TestReactionSpecJSONRoundTrip 确认 spec 经 JSON 往返后逐字段不变。
//
// 客户端拿到的就是 JSON 形态。若某个字段因为 omitempty 或类型不匹配
// 在往返中丢失/变形，跨端一致性测试会以很难定位的方式失败。
// 在这里先挡住，失败信息直接指向字段。
func TestReactionSpecJSONRoundTrip(t *testing.T) {
	for _, s := range AllReactionSpecs() {
		raw, err := json.Marshal(s)
		if err != nil {
			t.Fatalf("%s: 序列化失败 %v", s.Key, err)
		}
		var back ReactionSpec
		if err := json.Unmarshal(raw, &back); err != nil {
			t.Fatalf("%s: 反序列化失败 %v", s.Key, err)
		}
		if back != s {
			t.Errorf("%s: JSON 往返后不一致\n  前: %+v\n  后: %+v", s.Key, s, back)
		}
	}
}

// TestAllReactionSpecsCoversAllReactions 确认导出的规格覆盖全部反应键。
//
// cmd/vectors 按 AllReactionSpecs() 导出，客户端按这个顺序断言。
// 若某个键在 AllReactions() 里但不在表里（或反之），
// 客户端与 Go 的数组长度会对不上，契约测试会以"长度不符"这种
// 无信息量的方式失败。这里给出可读的诊断。
func TestAllReactionSpecsCoversAllReactions(t *testing.T) {
	specs := AllReactionSpecs()
	if len(specs) != len(AllReactions()) {
		t.Errorf("反应键 %d 个，但只有 %d 份规格", len(AllReactions()), len(specs))
	}
	seen := map[ReactionKey]bool{}
	for _, s := range specs {
		if seen[s.Key] {
			t.Errorf("反应 %s 在表里出现了两次", s.Key)
		}
		seen[s.Key] = true
	}
	for _, k := range AllReactions() {
		if !seen[k] {
			t.Errorf("反应 %s 缺少数值规格", k)
		}
	}
}
