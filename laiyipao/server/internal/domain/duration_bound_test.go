package domain

import (
	"strings"
	"testing"
	"time"
)

// duration_ms 缺上界导致结算必然 500（第 63 轮）。
//
// # 缺陷形态：「校验过的值」与「入库的值」不是同一个
//
// SettleInput.DurationMs 是 Go 的 int（64 位），
// 而 battle_records.duration_ms 是 PG 的 INTEGER（int32，上限 2147483647）。
// ValidateSettle 原本只判 `DurationMs <= 0`，**没有上界**：
//
//   - maxShotsFor(durationMs) 内部把时长夹到 24h → 发射数校验通过
//     （那一层夹了，所以看不出问题）
//   - 而 service/game.go 的 INSERT 用的是**原始未夹紧**的值
//     → PG `22003 integer out of range` → failErr 兜底成 500
//
// 任何持有合法 token 的玩家都能稳定把结算打成 500。
// 500 的代价是具体的：客户端只能显示「服务内部错误」，
// 排查会滑向「服务端坏了」而不是「这个上报不可信」，
// 而且它们混进服务故障告警，把真正的故障淹掉。
//
// # 与已修缺口的区别
//
// 同一族的三个缺口，逐层向外：
//   - reactions 的上界由 shots 界定，而 shots 当时无上界 → 漏了控制量
//   - shots 的上界由 durationMs 算，而 durationMs 当时无上界
//     → **判据自己是从一个无界的量算出来的**
//   - 本轮：直接给 durationMs 定上界

func durationSettleInput(gl GeneratedLevel) SettleInput {
	return SettleInput{
		Result:      "win",
		Kills:       gl.TotalEnemies(),
		WaveReached: gl.WaveCount,
		DurationMs:  120_000,
		Shots:       100,
		Hits:        80,
		Reactions:   10,
		HeatMax:     100,
		HPLeft:      int(gl.BaseHP),
	}
}

func durationToken(levelID int) BattleToken {
	now := time.Now()
	return BattleToken{
		ID: 1, UserID: 1, LevelID: levelID,
		IssuedAt: now, ExpiresAt: now.Add(time.Hour),
	}
}

// TestDurationMsUpperBoundRejectsOutOfInt32Range 是本轮的核心守卫。
//
// 判据不是「某个具体大数被拒」，而是**入库安全性**：
// 任何通过校验的 duration_ms 都必须能装进 PG INTEGER。
func TestDurationMsUpperBoundRejectsOutOfInt32Range(t *testing.T) {
	gl := GenerateLevel(1)
	now := time.Now()
	limits := SettleLimits{MinDurationMs: 1_000}

	cases := []struct {
		name string
		ms   int
	}{
		{"24h 上界本身（合法）", MaxPlausibleDurationMs},
		{"24h +1ms", MaxPlausibleDurationMs + 1},
		{"int32 上界（24.8 天）", 2147483647},
		{"int32 上界 +1", 2147483648},
		{"2^31", 1 << 31},
		{"2^62", 1 << 62},
		{"int64 最大", 9223372036854775807},
	}

	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			in := durationSettleInput(gl)
			in.DurationMs = c.ms

			_, err := ValidateSettle(durationToken(1), gl, in, limits, now)
			if c.ms == MaxPlausibleDurationMs {
				// 上界本身必须放行（哪怕被别的判据拒也不能是时长上界这条），
				// 否则上界定得太紧，真实玩家的长局会被误拒。
				if isDurationUpperBoundErr(err) {
					t.Fatalf("24h 上界本身被时长判据拒了，上界定得太紧：%v", err)
				}
				return
			}
			if err == nil {
				t.Fatalf("duration_ms=%d 超过可信上界却被放行 —— 入库会报 22003 并退化成 500", c.ms)
			}
			if !isInvalidField(err) {
				t.Errorf("应归类为 ErrInvalidField，实得 %v", err)
			}
		})
	}
}

// TestAcceptedDurationAlwaysFitsInt32 把「入库安全」立成**关系**守卫：
// 任何被放行的 duration_ms 都必须装得进 PG INTEGER。
//
// 这比逐个值断言强 —— 它对未来的新字段/新判据同样成立，
// 而且直接对应真实故障（PG 22003）。
func TestAcceptedDurationAlwaysFitsInt32(t *testing.T) {
	gl := GenerateLevel(1)
	now := time.Now()
	limits := SettleLimits{MinDurationMs: 1_000}

	const pgIntegerMax = int64(2147483647)

	// 扫一遍上界附近与远超上界的取值。
	for _, ms := range []int{
		1, 1000, 120_000, 600_000,
		MaxPlausibleDurationMs - 1,
		MaxPlausibleDurationMs,
		MaxPlausibleDurationMs + 1,
		2147483646, 2147483647, 2147483648,
		1 << 40, 1 << 62, 9223372036854775807,
	} {
		in := durationSettleInput(gl)
		in.DurationMs = ms
		_, err := ValidateSettle(durationToken(1), gl, in, limits, now)
		if err != nil {
			continue // 被拒就没事发生
		}
		if int64(ms) > pgIntegerMax {
			t.Errorf("duration_ms=%d 被放行但超过 PG INTEGER 上界 %d —— 入库必然报 22003", ms, pgIntegerMax)
		}
	}
}

// TestDurationBoundCoversMaxShotsDerivation 钉住「判据自身不再从无界量推导」：
// maxShotsFor 的夹取是 fail-closed 的第二道防线，
// 而 ValidateSettle 的上界判据是第一道。
//
// 这条测试存在的意义：若有人删掉 ValidateSettle 里的上界判据、
// 只靠 maxShotsFor 夹取，上面的守卫会红（因为 int32 上界+1 会被放行）。
// 换句话说它记录的是「两道防线缺一不可」。
func TestDurationBoundCoversMaxShotsDerivation(t *testing.T) {
	// maxShotsFor 必须对任意 int 输入都不返回负数（fail-closed 的含义）。
	for _, ms := range []int{-1, 0, 1, 2147483647, 2147483648, 1 << 62, 9223372036854775807} {
		if got := maxShotsFor(ms); got < 0 {
			t.Errorf("maxShotsFor(%d) = %d 为负 —— 上界成负会让所有上报通过", ms, got)
		}
	}

	// 夹取必须单调：更大的时长给出不更小的上界。
	prev := int64(0)
	for _, ms := range []int{1, 1000, 100_000, 2147483647, 2147483648, 1 << 62} {
		got := maxShotsFor(ms)
		if got < prev {
			t.Errorf("maxShotsFor(%d) = %d 小于更短时长的 %d —— 夹取破坏单调", ms, got, prev)
		}
		prev = got
	}
}

// isDurationUpperBoundErr 判断错误**是否来自 MaxPlausibleDurationMs 那条判据**。
//
// ⚠️ 不能用 isInvalidField 代替 —— 时长上界、元素合计、卡片下界
// 全都是 ErrInvalidField，isInvalidField 对它们一律返回 true。
// 第一版就踩了这个：断言「不是时长上界拒的」却写成了 isInvalidField，
// 于是「被别的 ErrInvalidField 判据拒掉」也算通过 —— 断言等于没写。
//
// 所以这里比对**错误消息**：只有含时长上界文案与上界值的才是那条判据。
func isDurationUpperBoundErr(err error) bool {
	if err == nil {
		return false
	}
	msg := err.Error()
	return strings.Contains(msg, "超过可信上界")
}
