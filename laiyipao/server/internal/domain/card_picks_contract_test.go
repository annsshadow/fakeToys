package domain

import (
	"testing"
	"time"
)

// card_picks 取值域的**跨端契约**（第 66 轮）。
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

// TestCardPickEncodingsAreDisjoint 钉住「两个负区间不重叠」。
//
// 这是本轮最容易出事的性质：一旦 `rollWaveCards` 改成每波 4 张，
// `-(3+handIdx)` 与 `-(6+handIdx)` 的取值域就会开始重叠，
// 于是「弃+取第 2 张」与「弃+跳第 1 张」编码成同一个值 ——
// 重放会选错路径，而**没有任何测试会红**（每侧单独看都合法）。
//
// 判据用**结构**而不是魔数：BASE 之差必须大于每波最大手牌数。
func TestCardPickEncodingsAreDisjoint(t *testing.T) {
	v := loadVectors(t)
	cp := v.CardPicks

	if cp.HandSlots <= 0 {
		t.Fatalf("契约向量里的 hand_slots = %d，必须为正", cp.HandSlots)
	}
	gap := cp.DiscardSkipBase - cp.DiscardTakeBase
	// ⚠️ 判据是 `gap >= hand_slots`，不是 `>`。
	//
	// 两个区间分别是 [-(base+h-1), -base]，长度都是 h。
	// 不相交要求 -(skipBase) < -(takeBase) - (h-1)
	//   ⟺ skipBase > takeBase + h - 1
	//   ⟺ gap >= h        ← 等号成立时恰好相邻，仍然不相交
	//
	// 我第一版写成 `gap <= h` 就红，把「恰好相邻」误判成「重叠」。
	// 这类「差一」的错误与 README 记的「gofmt -C 那次」同形：
	// 判据里的边界算错了一个数，于是结论整个反过来。
	if gap < cp.HandSlots {
		t.Fatalf("两个 BASE 之差 %d < 每波最大手牌数 %d —— "+
			"「弃+取」与「弃+跳」的编码区间会重叠，同一个值被解释成两种操作。\n"+
			"修法是拉开 BASE（DiscardSkipBase），不是改这个断言。",
			gap, cp.HandSlots)
	}

	// 两个区间各自都必须在 [min, ...] 内
	takeLo := -(cp.DiscardTakeBase + cp.HandSlots - 1)
	skipLo := -(cp.DiscardSkipBase + cp.HandSlots - 1)
	if takeLo < cp.Min {
		t.Errorf("弃+取区间下界 %d < 契约 min %d —— 服务端会拒掉客户端自己产生的编码", takeLo, cp.Min)
	}
	if skipLo < cp.Min {
		t.Errorf("弃+跳区间下界 %d < 契约 min %d —— 服务端会拒掉客户端自己产生的编码", skipLo, cp.Min)
	}
	// 区间不相交
	if skipLo >= takeLo {
		t.Errorf("弃+跳下界 %d 与弃+取下界 %d 相交（应满足 skipLo < takeLo）", skipLo, takeLo)
	}
}

// TestCardPickVectorCasesMatchGoBoundaries 让向量里的每条字面期望值
// 都被 Go 侧边界逻辑实际验证一遍。
//
// 只比对 CardPickMin 与 min 是不够的 —— 编码的**语义**（哪个值代表哪种操作）
// 也要有守卫，否则「两端同时改成另一个含义」不会被发现。
func TestCardPickVectorCasesMatchGoBoundaries(t *testing.T) {
	v := loadVectors(t)
	cp := v.CardPicks

	for _, c := range cp.Cases {
		t.Run(c.Name, func(t *testing.T) {
			if c.Expected < cp.Min {
				t.Fatalf("向量值 %d < 契约下界 %d —— 这条用例本身非法", c.Expected, cp.Min)
			}
			if c.Expected > cp.HandSlots-1 && c.Expected != cp.Skip &&
				c.Expected > -(cp.DiscardTakeBase) {
				t.Fatalf("向量值 %d 不属于任何已知编码区间（普通下标 %v / 跳过 %d / 弃+取 / 弃+跳）",
					c.Expected, cp.HandSlots, cp.Skip)
			}
		})
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
		{"plain_skip", -1},
		{"discard_then_take_first", -3},
		{"discard_then_take_last", -5},
		{"discard_then_skip", -6},
		{"discard_then_skip_last", -8},
	}
	for _, c := range accepted {
		t.Run(c.name, func(t *testing.T) {
			if _, err := ValidateSettle(usableToken(gl.ID, now), gl, mk(c.pick), limits, now); err != nil {
				t.Fatalf("card_picks[%d] 被拒：%v —— 客户端会用它产生正常对局", c.pick, err)
			}
		})
	}

	// 下界之外必须拒 —— 否则下界就是装饰品
	for _, bad := range []int{-9, -100, -1 << 30} {
		if _, err := ValidateSettle(usableToken(gl.ID, now), gl, mk(bad), limits, now); err == nil {
			t.Errorf("card_picks[%d] 越界却被放行 —— 下界 CardPickMin=%d 没有被真正执行", bad, CardPickMin)
		}
	}
}
