package domain

import (
	"testing"
	"time"
)

// card_picks 取值域的**跨端契约**（第 66 轮引入，第 159 轮重写）。
//
// # 为什么必须锁
//
// `card_picks` 的负值编码在两端各有一份边界：
//
//	Go：CardPickMin（internal/domain/battle_collections.go）—— 按它拒
//	TS：PICK_MIN（src/game/engine.ts）—— 按它生成
//
// 漂了的后果是**静默**的，而且是最坏的那种：
// 客户端正常对局被服务端 422 拒掉，而客户端测试、Go 测试、
// 服务端 e2e **全部照样绿** —— 因为它们都不跑一次真实结算。
//
// 与 `replay_skills` 用同一套机制：向量里放字面期望值，
// 两端各读同一份。若确认是协议改动，两端一起改并回填向量。

func TestCardPickMinMatchesCrossEndVectors(t *testing.T) {
	v := loadVectors(t)
	cp := v.CardPicks
	if len(cp.Cases) == 0 {
		t.Fatal("formula_vectors.json 里 card_picks.cases 是空的 —— " +
			"向量被误删时必须红，否则这个测试会静默变成空转")
	}

	if int64(CardPickMin) != cp.Min {
		t.Fatalf("Go 侧 CardPickMin = %d，契约向量写的是 %d\n"+
			"两端漂了的后果：客户端正常对局被服务端 422 拒掉，而所有测试都绿。\n"+
			"若确认是协议改动，两端一起改并回填 formula_vectors.json。",
			CardPickMin, cp.Min)
	}
}

// TestCardPickNegativeRangeIsContiguous 钉住新的负值域形状。
//
// 第 159 轮起：负值域 = 连续区间 [min..-code_base] + 独立值 skip(-1)，
// -2 是刻意留的空隙。`validCardPick` 的判据就建立在这个形状上 ——
// 形状漂了（比如 code_max 改了但 validCardPick 没跟着改）
// 会让「客户端产物」与「服务端校验」错位。
func TestCardPickNegativeRangeIsContiguous(t *testing.T) {
	v := loadVectors(t)
	cp := v.CardPicks

	if cp.CodeBase != CardPickCodeBase || cp.CodeMax != CardPickCodeMax {
		t.Fatalf("契约向量 code_base/code_max = %d/%d，Go 侧 = %d/%d —— "+
			"「弃牌掩码 + 取/跳」的枚举区间两端漂了",
			cp.CodeBase, cp.CodeMax, CardPickCodeBase, CardPickCodeMax)
	}
	if cp.HandSlots != CardPickHandSlots {
		t.Fatalf("契约向量 hand_slots = %d，Go 侧 = %d", cp.HandSlots, CardPickHandSlots)
	}
	if cp.Min != -(cp.CodeBase + cp.CodeMax) {
		t.Fatalf("契约 min=%d 与 code 区间 [0..%d] 不自洽（应为 -(base+max) = %d）",
			cp.Min, cp.CodeMax, -(cp.CodeBase + cp.CodeMax))
	}

	// 每一个枚举 code 都必须被 validCardPick 放行 ——
	// 否则「客户端按枚举表产出」会被服务端自己人拒掉。
	for code := int64(0); code <= cp.CodeMax; code++ {
		pick := -(cp.CodeBase + code)
		if !validCardPick(int(pick)) {
			t.Errorf("枚举 code=%d（pick=%d）被 validCardPick 拒绝 —— "+
				"服务端会拒掉客户端自己产生的编码", code, pick)
		}
	}
	if !validCardPick(int(cp.Skip)) {
		t.Errorf("skip=%d 被 validCardPick 拒绝", cp.Skip)
	}
	if validCardPick(-2) {
		t.Error("-2 应落在空隙里（协议刻意留的，不是笔误）")
	}
	if validCardPick(int(cp.Min) - 1) {
		t.Errorf("pick=%d 低于下界却被放行", cp.Min-1)
	}
}

// pickWithMask 按服务端编码约定重算「弃 mask 位手牌 + 取/跳」的 card_picks 值。
//
// take = -1 表示不取（跳过）。
func pickWithMask(mask, take int) int {
	return -(PickCodeBase + encodeDiscardCode(mask, take))
}

// TestCardPickVectorCasesMatchGoBoundaries 让向量里的每条字面期望值
// 都被 Go 侧编码逻辑实际验证一遍。
//
// 只比对 CardPickMin 与 min 是不够的 —— 编码的**语义**（哪个值代表哪种操作）
// 也要有守卫，否则「两端同时改成另一个含义」不会被发现。
func TestCardPickVectorCasesMatchGoBoundaries(t *testing.T) {
	v := loadVectors(t)
	cp := v.CardPicks

	// name → 本地重算值。判据不是「常量相等」而是「算出来的值等于向量」——
	// 后者能抓住「改了枚举但忘了同步向量」这种半途而废。
	byName := map[string]int64{
		"plain_pick_first":           0,
		"plain_pick_last":            cp.HandSlots - 1,
		"plain_skip":                 cp.Skip,
		"discard_first_take_first":   int64(pickWithMask(0b001, 0)),
		"discard_last_take_first":    int64(pickWithMask(0b100, 0)),
		"discard_first_skip":         int64(pickWithMask(0b001, -1)),
		"discard_two_take_remaining": int64(pickWithMask(0b011, 0)),
		"discard_two_skip":           int64(pickWithMask(0b011, -1)),
		"discard_all_skip":           int64(pickWithMask(0b111, -1)),
	}
	for _, c := range cp.Cases {
		t.Run(c.Name, func(t *testing.T) {
			want, ok := byName[c.Name]
			if !ok {
				t.Fatalf("向量里出现未知用例 %s —— Go 侧还没有对应编码", c.Name)
			}
			if !validCardPick(int(c.Expected)) {
				t.Fatalf("向量值 %d 不是合法编码（validCardPick 拒绝）—— 这条用例本身非法", c.Expected)
			}
			if want != c.Expected {
				t.Errorf("用例 %s：本地重算 %d ≠ 向量 %d", c.Name, want, c.Expected)
			}
		})
	}

	// 字面值人工推导核对：枚举表里 mask=1/4/3/7 的区间起点是 4/12/10/19。
	// 这条把「枚举表」与「向量字面值」绑在一起，两边不一致时必红。
	literal := map[string]int{
		"discard_first_take_first":   -7,
		"discard_last_take_first":    -15,
		"discard_first_skip":         -9,
		"discard_two_take_remaining": -13,
		"discard_two_skip":           -14,
		"discard_all_skip":           -22,
	}
	for name, want := range literal {
		if got := int(byName[name]); got != want {
			t.Errorf("%s 本地重算 = %d，人工推导 = %d（枚举表与实现漂了）", name, got, want)
		}
	}
}

// TestValidateSettleAcceptsNewCardPickRange 端到端确认：新的负编码真的被接受。
//
// 契约测试只比对常量与向量；这条确认**校验逻辑**真的放行。
// 它防的是「常量改了但校验分支没跟上」—— 那种情况下常量与向量一致，
// 而真实对局仍被 422 拒掉。
func TestValidateSettleAcceptsNewCardPickRange(t *testing.T) {
	gl := testLevel()
	limits := antiCheatLimits()
	now := time.Now()

	// 基线：把合法字段都填成真实值，只让 CardPicks 变。
	// ⚠️ 用 baseInput(gl) 而不是手写 —— 手写容易漏掉某个字段，
	// 于是更早的判据先拒，用例变成「因为别的原因红/绿」。
	mk := func(pick int) SettleInput {
		in := baseInput(gl)
		in.Kills = gl.TotalEnemies()
		in.WaveReached = gl.WaveCount
		in.HPLeft = int(gl.BaseHP)
		in.Shots = legalShots(gl, in.DurationMs)
		in.Hits = in.Shots
		in.CardPicks = []int{pick}
		return in
	}

	accepted := []struct {
		name string
		pick int
	}{
		{"plain_pick_first", 0},
		{"plain_pick_last", int(CardPickHandSlots) - 1},
		{"plain_skip", CardPickSkip},
		{"discard_first_take_first", pickWithMask(0b001, 0)}, // -7
		{"discard_last_take_first", pickWithMask(0b100, 0)},  // -15
		{"discard_first_skip", pickWithMask(0b001, -1)},      // -9
		{"discard_two_skip", pickWithMask(0b011, -1)},        // -14
		{"discard_all_skip", pickWithMask(0b111, -1)},        // -22（下界）
	}
	for _, c := range accepted {
		t.Run(c.name, func(t *testing.T) {
			if _, err := ValidateSettle(usableToken(gl.ID, now), gl, mk(c.pick), limits, now); err != nil {
				t.Fatalf("card_picks[%d] 被拒：%v —— 客户端会用它产生正常对局", c.pick, err)
			}
		})
	}

	// 空隙与下界之外必须拒 —— 否则下界就是装饰品
	for _, bad := range []int{-2, CardPickMin - 1, -100, -1 << 30} {
		if _, err := ValidateSettle(usableToken(gl.ID, now), gl, mk(bad), limits, now); err == nil {
			t.Errorf("card_picks[%d] 越界却被放行 —— 下界 CardPickMin=%d 没有被真正执行", bad, CardPickMin)
		}
	}
}
