package domain

import (
	"math"
	"testing"
	"time"
)

// 集合类上报字段的**合计**溢出击穿上界校验（第 62 轮）。
//
// # 缺陷形态
//
// `validateReportCollections` 原本对每个 map 值单独判 `v < 0`，
// 然后用 `total += v` 累加，最后拿 `total > cap` 当上界判据。
//
// 每一项都合法，但**合计会 int64 回绕**：两个键各报 2^62，
// 相加正好 2^63，在 int64 里变成 -9223372036854775808。
// 于是 `total > cap` 拿一个负数去比，恒为假 → 整条上界形同虚设。
//
// 这与本项目已修的两处同类但**不是同一个**：
//   - `reactions <= shots*8` 而 `shots` 无上界 → 约束了被保护量、漏了控制量
//   - 速率裁剪的分母 `duration_ms=0` → 分母为零让整段裁剪被跳过
//
// 这一处是：**约束了被保护量，却没约束产生它的算术**。
// 第三种形态，所以需要第三种守卫。

// overflowProbe 是能让 int64 累加回绕的两项。
//
// 各 2^62 = 4611686018427387904，相加 = 2^63，在 int64 里就是最小值。
// 键名取自合法白名单（knownElements / knownReactions），
// 所以「键白名单」那条防线挡不住它 —— 只有合计判据能挡。
func overflowProbe() (elemA, elemB, reactA, reactB string) {
	const half = int64(math.MaxInt64)/2 + 1 // 2^62，相加恰好回绕
	return "fire", "ice", "steam_burst", "overheat"
}

func halfOverflow() int { return int(math.MaxInt64)/2 + 1 } // 2^62

// settleInputForCollections 返回一个**其余字段全部合法**的基线输入。
//
// ⚠️ 这里的 Kills/WaveReached 必须按真实关卡生成，不能拍脑袋写 40：
// 第 1 关只有 39 只怪，写 40 会被「击杀数超过该关卡敌人总数」这条**更早**的
// 判据先拒掉 —— 测试会"通过"，但它证明的不是溢出被挡住了。
//
// 这是本项目记过多次的形态：**守卫被无关的早退路径满足**。
// 所以下面每个用例都额外断言 `isInvalidField`，把错误来源钉死。
func settleInputForCollections() SettleInput {
	gl := GenerateLevel(1)
	return SettleInput{
		Result:      "win",
		Kills:       gl.TotalEnemies(), // 用满，正好不触发 ErrTooManyKills
		WaveReached: gl.WaveCount,
		DurationMs:  120_000,
		Shots:       100,
		Hits:        80,
		Reactions:   10,
		HeatMax:     100,
		HPLeft:      int(gl.BaseHP),
	}
}

func tokenForCollections() BattleToken {
	now := time.Now()
	return BattleToken{
		ID:        1,
		UserID:    1,
		LevelID:   1,
		IssuedAt:  now,
		ExpiresAt: now.Add(time.Hour),
	}
}

func levelForCollections() GeneratedLevel {
	return GenerateLevel(1)
}

func limitsForCollections() SettleLimits {
	return SettleLimits{MinDurationMs: 1_000}
}

// TestCollectionsOverflowBypassesTotalBounds 是本轮的核心守卫：
// 两个 2^62 的合法键相加回绕，必须被拒。
func TestCollectionsOverflowBypassesTotalBounds(t *testing.T) {
	gl := levelForCollections()
	ea, eb, ra, rb := overflowProbe()

	cases := []struct {
		name string
		in   SettleInput
	}{
		{
			// 反应侧 cap 常常只有个位数，所以这条最容易被打穿。
			name: "reactions_used 两键各 2^62 → 回绕成负 → 放行",
			in: func() SettleInput {
				in := settleInputForCollections()
				in.ReactionsUsed = map[string]int{ra: halfOverflow(), rb: halfOverflow()}
				return in
			}(),
		},
		{
			name: "elements_used 两键各 2^62 → 回绕成负 → 放行",
			in: func() SettleInput {
				in := settleInputForCollections()
				in.ElementsUsed = map[string]int{ea: halfOverflow(), eb: halfOverflow()}
				return in
			}(),
		},
		{
			// 三键：回绕后未必是负数，可能是很小的正数。
			// 2^62 * 3 = 1.38e19，回绕两次 → 一个看似正常的小正数。
			// 这条比两键更阴险：上界判据"看起来在工作"，实际已经失真。
			name: "reactions_used 三键各 2^62 → 双重回绕成小正数",
			in: func() SettleInput {
				in := settleInputForCollections()
				in.ReactionsUsed = map[string]int{
					ra: halfOverflow(), rb: halfOverflow(), "tidal_surge": halfOverflow(),
				}
				return in
			}(),
		},
	}

	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			_, err := ValidateSettle(tokenForCollections(), gl, c.in, limitsForCollections(), time.Now())
			if err == nil {
				t.Fatalf("合计回绕绕过了上界校验，必须拒绝 —— 这是一处可把运营看板打成永久 500 的污染源")
			}
			if !isInvalidField(err) {
				t.Errorf("应归类为 ErrInvalidField，实得 %v", err)
			}
		})
	}
}

// TestCollectionsSingleOversizedKeyStillRejected 守住「没修坏原本的判据」：
// 单键超界（不涉及溢出）仍然必须被拒。
func TestCollectionsSingleOversizedKeyStillRejected(t *testing.T) {
	gl := levelForCollections()
	_, _, ra, _ := overflowProbe()

	in := settleInputForCollections()
	in.ReactionsUsed = map[string]int{ra: in.Reactions + 1000}
	if _, err := ValidateSettle(tokenForCollections(), gl, in, limitsForCollections(), time.Now()); err == nil {
		t.Fatal("单键超界必须被拒")
	}
}

// TestCollectionsAccumulatorCannotExceedCap 把「无溢出判超界」这一性质
// 立成守卫，而不是只测「某组具体值被拒」。
//
// 判据是**关系**而非某个魔数：
//   - 任何会回绕的项，sumOverCap 必须返回 true
//   - 任何合计 ≤ cap 的项，必须返回 false（含**恰好等于** cap）
//
// 最后一条是本轮最关键的一条：第一版用 satAdd 判 `> cap`，
// 因为 satAdd 饱和到 cap 而恒为假 —— 而如果改成 `>= cap` 就会在这里红。
// 守卫必须能区分「恰好等于 cap」与「超过 cap」。
func TestCollectionsAccumulatorCannotExceedCap(t *testing.T) {
	h := int64(halfOverflow())

	t.Run("会回绕的项必须判超界", func(t *testing.T) {
		for _, cap := range []int64{0, 1, 7, 100, 9_700_000_000} {
			for _, terms := range [][]int64{
				{h, h},       // 恰好回绕到负
				{h, h, h},    // 双重回绕成小正数
				{h, h, h, h}, // 回绕回正
				{h, 1},
				{1, h},
			} {
				if !sumOverCap(cap, terms...) {
					t.Errorf("sumOverCap(%d, %v) = false，但这些项会回绕 —— 判据失真", cap, terms)
				}
			}
		}
	})

	t.Run("合计不超过 cap 必须放行（含恰好等于）", func(t *testing.T) {
		for _, cap := range []int64{0, 1, 7, 100, 9_700_000_000} {
			for _, terms := range [][]int64{
				{},
				{0},
				{cap},
				{cap / 2, cap / 2},   // 恰好等于 cap
				{cap / 2, cap/2 - 1}, // 小于 cap
				{1, 1, 1},
				{cap / 3, cap / 3, cap / 3},
			} {
				var sum int64
				for _, x := range terms {
					sum += x
				}
				if sum > cap {
					continue // 这组本来就该拒，不是本子测试的关心范围
				}
				if sumOverCap(cap, terms...) {
					t.Errorf("sumOverCap(%d, %v) = true，但合计 %d ≤ cap —— 合法对局被误拒", cap, terms, sum)
				}
			}
		}
	})

	t.Run("恰好等于 cap 与超过 cap 必须可区分", func(t *testing.T) {
		const cap = 100
		if sumOverCap(cap, 40, 60) {
			t.Error("恰好等于 cap（40+60）被判超界")
		}
		if !sumOverCap(cap, 40, 61) {
			t.Error("超过 cap（40+61）未被判超界")
		}
	})

	t.Run("负项按 0 处理（负值由调用方单独拒）", func(t *testing.T) {
		// sumOverCap 不负责负值 —— 负值在上游逐项拒掉了。
		// 这里只钉住「负项不会让判据误判为超界」。
		if sumOverCap(10, -1) {
			t.Error("负项应按 0 处理，不该判超界")
		}
	})
}

// isInvalidField 判断错误链里是否含 ErrInvalidField。
func isInvalidField(err error) bool {
	for e := err; e != nil; {
		if e == ErrInvalidField {
			return true
		}
		u, ok := e.(interface{ Unwrap() error })
		if !ok {
			return false
		}
		e = u.Unwrap()
	}
	return false
}
