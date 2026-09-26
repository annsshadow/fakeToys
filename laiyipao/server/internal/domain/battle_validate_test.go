package domain

import (
	"math"
	"testing"
	"time"
)

// ValidateSettle 的**上界与边界**测试。
//
// 这一组用例的定位：防作弊校验一旦缺失，攻击者可以凭空造出资源。
// 而这类漏洞的特征是「不报任何错，只是默默给钱」——
// 没有专门的负向测试，下一次重构就会把它删掉。
//
// 每条用例都对应一个具体的作弊手法，注释里写明「怎么利用它」。

func testLevel() GeneratedLevel {
	gl := GenerateLevel(1)
	return gl
}

func usableToken(levelID int, now time.Time) BattleToken {
	return BattleToken{
		ID:        1,
		UserID:    1,
		LevelID:   levelID,
		Seed:      12345,
		IssuedAt:  now.Add(-time.Minute),
		ExpiresAt: now.Add(time.Hour),
		UsedAt:    nil,
	}
}

func antiCheatLimits() SettleLimits {
	return SettleLimits{MinDurationMs: 5000, MaxScorePerSec: 50_000}
}

// legalShots 返回给定时长下的合法发射数（贴着上界）。
//
// ⚠️ 不要在用例里硬编码 Shots。发射数上界随时长线性变化
// （maxShotsFor），硬编码的 500 在 30 秒时合法、在 1 秒时非法，
// 于是「测 A 的用例」会因为「B 的上界」而红，排查方向完全被带偏。
func legalShots(gl GeneratedLevel, durationMs int) int {
	n := int(maxShotsFor(durationMs)) / 2 // 取一半，避免贴边
	if maxK := MaxKillsFor(gl); n > maxK*MaxShotsPerTickSlack {
		n = maxK * MaxShotsPerTickSlack
	}
	if n < 1 {
		n = 1
	}
	return n
}

// legalReactions 返回给定时长与击杀数下都合法的反应数。
func legalReactions(gl GeneratedLevel, shots int) int {
	r := shots * MaxReactionsPerHit
	if abs := MaxKillsFor(gl) * MaxReactionsPerEnemy; abs < r {
		r = abs
	}
	return r
}

// baseInput 是一份合法的上报，所有用例基于它改单个字段。
// 数值取自第 1 关的真实上限，确保只测「那一个字段」不越界。
func baseInput(gl GeneratedLevel) SettleInput {
	shots := legalShots(gl, 30_000)
	return SettleInput{
		Result:      "lose",
		Stars:       0,
		Score:       1000,
		Kills:       0,
		Leaked:      0,
		HPLeft:      1000,
		WaveReached: 1,
		DurationMs:  30_000,
		Shots:       shots,
		Hits:        shots / 2,
		Reactions:   shots / 2,
		HeatMax:     100,
	}
}

func settleOK(t *testing.T, gl GeneratedLevel, in SettleInput) SettleResult {
	t.Helper()
	res, err := validateOne(gl, in)
	if err != nil {
		t.Fatalf("期望通过但被拒：%v", err)
	}
	return res
}

// validateOne 用同一份 limits 跑一次校验，供只需要 error 的用例复用。
func validateOne(gl GeneratedLevel, in SettleInput) (SettleResult, error) {
	now := time.Now()
	return ValidateSettle(usableToken(gl.ID, now), gl, in, antiCheatLimits(), now)
}

func settleErr(t *testing.T, gl GeneratedLevel, in SettleInput, wantErr error) {
	t.Helper()
	now := time.Now()
	res, err := ValidateSettle(usableToken(gl.ID, now), gl, in, antiCheatLimits(), now)
	if err == nil {
		t.Fatalf("期望被拒（%v）但通过了，得分=%d 星级=%d", wantErr, res.Score, res.StarsRecomputed)
	}
	if wantErr != nil {
		// 只要求错误链包含目标错误，不要求完全相等
		if !errorChainContains(err, wantErr) {
			t.Fatalf("错误不匹配：got=%v want=%v", err, wantErr)
		}
	}
}

func errorChainContains(err, target error) bool {
	for e := err; e != nil; {
		if e == target {
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

// TestSettleRejectsZeroDuration 覆盖 C1（Critical）。
//
// 作弊手法：DurationMs=0 → 旧代码整段跳过得分裁剪 →
// 紧接着的 `if res.Score == 0 { res.Score = in.Score }` 兜底把
// 未裁剪的原始分数写回 → 分数任意大 → 星级恒 3 → 单局掉落触顶。
func TestSettleRejectsZeroDuration(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.DurationMs = 0
	in.Result = "lose" // 失败局可绕过「通关最短时长」检查
	in.Kills = MaxKillsFor(gl)
	in.Score = 999_999_999
	in.Stars = 3
	settleErr(t, gl, in, ErrTooShort)
}

func TestSettleRejectsNegativeDuration(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.DurationMs = -1
	settleErr(t, gl, in, ErrTooShort)
}

// TestScoreClampIsAlwaysApplied 直接验证裁剪生效，不依赖 duration=0 的入口拦截。
// 即便入口被绕过，裁剪本身也必须把分数压回上限。
func TestScoreClampIsAlwaysApplied(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.DurationMs = 1000 // 1 秒
	in.Score = 999_999_999
	in.Shots = legalShots(gl, 1000)
	in.Hits = in.Shots / 2
	in.Reactions = legalReactions(gl, in.Shots)
	res := settleOK(t, gl, in)

	full := gl.StarTargets[len(gl.StarTargets)-1]
	if res.Score > full {
		t.Fatalf("分数未被裁剪到星门槛上限：%d > %d", res.Score, full)
	}
	if !res.Clamped {
		t.Fatal("应标记为已裁剪")
	}
	// 1 秒的额度 = MaxScorePerSec（50000），它落在第 1 档星门槛之下，
	// 所以星级应为 0 —— 这正是"速率上限真的在生效"的证据：
	// 若裁剪失效，分数会是 1e9，星级 3。
	if res.StarsRecomputed != 0 {
		t.Fatalf("1 秒内只够到第一档门槛之下，星级应为 0，实际 %d（分数 %d）", res.StarsRecomputed, res.Score)
	}
	// 额度取 min(1秒 * MaxScorePerSec, 星门槛上限)
	if want := antiCheatLimits().MaxScorePerSec; int64(want) < int64(full) && res.Score != int64(want) {
		t.Fatalf("应被裁剪到每秒上限 %d，实际 %d", want, res.Score)
	}
}

// TestSettleRejectsAbsurdReactions 覆盖 C2（Critical）。
//
// 作弊手法：上报 reactions=1e9。reactions 原样进 bumpTasks，
// 任务与成就的 progress 列无上界 → 一次顶满全部反应类奖励。
func TestSettleRejectsAbsurdReactions(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	// Shots 与 Hits 必须同步改，否则会先撞上命中率校验，
	// 测到的就不是 reactions 那条分支了。
	in.Shots = 1
	in.Hits = 1
	in.Reactions = 1_000_000_000
	in.DurationMs = 1_000_000_000 // 给足时长，排除被时长上界先拒
	settleErr(t, gl, in, ErrTooManyReactions)
}

func TestSettleRejectsNegativeReactions(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.Reactions = -1
	settleErr(t, gl, in, nil)
}

// TestZeroShotsMeansZeroReactions 固化「没开火不可能有反应」这条不变量。
func TestZeroShotsMeansZeroReactions(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.Shots = 0
	in.Hits = 0
	in.Reactions = 1
	settleErr(t, gl, in, ErrTooManyReactions)

	// 合法组合：0 开火 0 反应
	in.Reactions = 0
	in.DurationMs = 30_000
	settleOK(t, gl, in)
}

func TestSettleRejectsReactionsAboveShotsBound(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.Shots = 10
	in.Hits = 10
	in.Reactions = 10*MaxReactionsPerHit + 1
	in.DurationMs = 30_000
	settleErr(t, gl, in, ErrTooManyReactions)
}

// TestReactionsBoundSurvivesShotsInflation 覆盖审计发现的绕过。
//
// 上一轮给 reactions 加了上界 `reactions <= shots * MaxReactionsPerHit`，
// 但 shots 当时无上界。于是这条校验形同虚设：
// 上报 {"shots":125000000,"reactions":1000000000} 时
// 上界正好 = 125000000*8 = 1e9，恰好放行它本该挡掉的 1e9。
//
// 教训：**约束了被保护量却没约束控制量，等于没约束。**
// 这条测试固化两个上界都要在：shots 有物理上界（随时长），
// reactions 另有不依赖 shots 的绝对上界（按总怪数）。
func TestReactionsBoundSurvivesShotsInflation(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.DurationMs = 1_000_000_000 // 极长时长，排除被时长上界先拒
	in.Shots = 125_000_000
	in.Hits = 125_000_000
	in.Reactions = 1_000_000_000
	// 先撞发射数上界（第一道防线）
	_, err := validateOne(gl, in)
	if err == nil {
		t.Fatal("1.25 亿发应当被拒")
	}
	if !errorChainContains(err, ErrTooManyShots) {
		t.Fatalf("应报发射数超限，实际：%v", err)
	}

	// 把 shots 压到合法区间，reactions 仍必须被绝对上界挡住
	in.Shots = legalShots(gl, in.DurationMs)
	in.Hits = in.Shots
	in.Reactions = 1_000_000_000
	_, err = validateOne(gl, in)
	if err == nil {
		t.Fatal("反应次数远超该关总怪数应当被拒")
	}
	if !errorChainContains(err, ErrTooManyReactions) {
		t.Fatalf("应报反应次数超限，实际：%v", err)
	}
}

// TestShotsBoundScalesWithDuration 固化「发射数随时长线性增长」。
//
// 如果有人把 maxShotsFor 改成常数，短时长的高射速对局会被误拒；
// 如果改成无界，超长时长的对局就成了刷量后门。
func TestShotsBoundScalesWithDuration(t *testing.T) {
	gl := testLevel()
	prev := int64(0)
	for _, ms := range []int{1_000, 5_000, 30_000, 120_000, 600_000} {
		bound := maxShotsFor(ms)
		if bound <= prev {
			t.Fatalf("ms=%d 的上界 %d 未大于 ms 更小时的 %d", ms, bound, prev)
		}
		prev = bound
		// 恰好等于上界必须通过（上界校验最常见的错误是写成 > 而非 >=）
		in := baseInput(gl)
		in.DurationMs = ms
		in.Shots = int(bound)
		in.Hits = int(bound)
		in.Reactions = legalReactions(gl, in.Shots)
		settleOK(t, gl, in)
		// 超出一发必须被拒
		in.Shots = int(bound) + 1
		in.Hits = in.Shots
		_, err := validateOne(gl, in)
		if !errorChainContains(err, ErrTooManyShots) {
			t.Fatalf("ms=%d 超界一发应被拒，实际：%v", ms, err)
		}
	}
}

// TestMaxShotsForOverflowFailsClosed 确认极端时长不会让上界变成负数。
//
// 时长来自客户端上报，DurationMs 只有 `> 0` 校验、没有上界。
// 若不夹住，durationMs=2^62 时 (d/50+1)*8 会溢出成负数，
// 而「负数小于任何上报值」会让**所有**上报都通过 ——
// 那比没有校验更糟。
func TestMaxShotsForOverflowFailsClosed(t *testing.T) {
	for _, ms := range []int{1 << 20, 1 << 40, 1 << 60, math.MaxInt32} {
		bound := maxShotsFor(ms)
		if bound <= 0 {
			t.Fatalf("ms=%d 的上界为 %d，必须为正（溢出未 fail-closed）", ms, bound)
		}
	}
	if maxShotsFor(0) != 0 {
		t.Error("ms=0 的上界应为 0")
	}
	if maxShotsFor(-1) != 0 {
		t.Error("负时长的上界应为 0")
	}
}

// TestComputeLootNeverNegative 覆盖溢出绕过封顶的路径。
//
// 曾经的写法是 `coin += (reactions/10)*Bonus` 后 `if coin > MaxCoin`：
// reactions 传 2^62 时乘法回绕成负数，于是 `coin > 60000` 为假、
// 裁剪被跳过、负数金币直接进 grantWallet。
// 上游现在给 reactions 上了界，但 computeLoot 是导出 API，不该依赖调用方。
func TestComputeLootNeverNegative(t *testing.T) {
	gl := testLevel()
	maxK := int64(MaxKillsFor(gl))
	caps := DefaultLootCaps()
	for _, reactions := range []int64{0, 1, 1000, 1 << 40, math.MaxInt64} {
		for _, kills := range []int64{0, maxK, 1 << 40, math.MaxInt64} {
			for _, stars := range []int64{0, 3, 1 << 40} {
				loot := computeLoot(gl, stars, kills, maxK, reactions)
				coin, ok := loot["coin"]
				if !ok {
					t.Fatal("掉落缺少 coin")
				}
				if coin < 0 {
					t.Fatalf("stars=%d kills=%d reactions=%d → 负数金币 %d",
						stars, kills, reactions, coin)
				}
				if coin > caps.MaxCoinPerBattle {
					t.Fatalf("stars=%d kills=%d reactions=%d → 金币 %d 超过封顶 %d",
						stars, kills, reactions, coin, caps.MaxCoinPerBattle)
				}
			}
		}
	}
}

// TestComputeLootReachesCap 确认封顶不是"永远触发不到"的死代码。
//
// 上一条只验「不越界」。若 satAdd 写错导致永远返回 0，它也会通过。
// 这条确认在足够大的输入下确实能顶到封顶值。
func TestComputeLootReachesCap(t *testing.T) {
	gl := testLevel()
	cap := DefaultLootCaps().MaxCoinPerBattle
	loot := computeLoot(gl, 3, math.MaxInt64, math.MaxInt64, math.MaxInt64)
	if got := loot["coin"]; got != cap {
		t.Fatalf("极端输入应恰好顶到封顶 %d，实际 %d", cap, got)
	}
}

func TestSettleRejectsHeatAboveCap(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.HeatMax = MaxReportedHeat + 1
	settleErr(t, gl, in, ErrInvalidField)
}

func TestSettleRejectsNegativeHeat(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.HeatMax = -1
	settleErr(t, gl, in, ErrInvalidField)
}

func TestSettleRejectsLeakedAboveTotal(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.Leaked = MaxKillsFor(gl) + 1
	settleErr(t, gl, in, ErrTooManyLeaked)
}

func TestSettleRejectsNegativeHPLeft(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.HPLeft = -1
	settleErr(t, gl, in, ErrInvalidField)
}

// TestKillsPlusLeakedConserved 固化结算守恒。
// 这是服务端 ValidateSettle 与客户端 engine.settleInput 共同依赖的性质。
func TestKillsPlusLeakedConserved(t *testing.T) {
	gl := testLevel()
	total := MaxKillsFor(gl)
	for _, tc := range []struct{ kills, leaked int }{
		{0, total},
		{total, 0},
		{total / 2, total / 2},
	} {
		in := baseInput(gl)
		in.Result = "lose"
		in.Kills = tc.kills
		in.Leaked = tc.leaked
		in.WaveReached = gl.WaveCount
		in.Hits = 0
		in.Shots = 0
		in.Reactions = 0
		settleOK(t, gl, in)
	}
}

func TestSettleRejectsKillsAboveTotal(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.Kills = MaxKillsFor(gl) + 1
	settleErr(t, gl, in, ErrTooManyKills)
}

func TestSettleRejectsWaveBeyondLevel(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.WaveReached = gl.WaveCount + 1
	settleErr(t, gl, in, ErrWaveExceeded)
}

func TestSettleRejectsHitsAboveShots(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.Shots = 10
	in.Hits = 11
	settleErr(t, gl, in, ErrInvalidHitRate)
}

func TestSettleRejectsNegativeScore(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.Score = -100
	res := settleOK(t, gl, in)
	if res.Score != 0 {
		t.Fatalf("负分应归零，实际 %d", res.Score)
	}
}

// TestWinRequiresAllKills 固化「胜利 = 清完全部敌人」。
func TestWinRequiresAllKills(t *testing.T) {
	gl := testLevel()
	total := MaxKillsFor(gl)

	in := baseInput(gl)
	in.Result = "win"
	in.Kills = total - 1
	in.WaveReached = gl.WaveCount
	res := settleOK(t, gl, in)
	if res.Win {
		t.Fatal("未清完敌人却判为胜利")
	}

	in.Kills = total
	res = settleOK(t, gl, in)
	if !res.Win {
		t.Fatal("清完敌人应判为胜利")
	}
}

// TestWinTooFastRejected 覆盖「秒通关」。
func TestWinTooFastRejected(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.Result = "win"
	in.Kills = MaxKillsFor(gl)
	in.DurationMs = 10
	in.Stars = 3
	settleErr(t, gl, in, ErrTooShort)
}

// TestLoseFastAllowed 失败局允许秒退，这是设计意图。
func TestLoseFastAllowed(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.Result = "lose"
	in.DurationMs = 10
	in.Shots = 1
	in.Hits = 0
	in.Reactions = 0
	in.Kills = 0
	in.Leaked = 1
	res := settleOK(t, gl, in)
	if res.Win {
		t.Fatal("失败局不应判为胜利")
	}
}

// TestStarsRecomputedNotTrusted 星级永远由服务端重算。
func TestStarsRecomputedNotTrusted(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.Stars = 3 // 谎报三星
	in.Score = 1 // 实际分数极低
	res := settleOK(t, gl, in)
	if res.StarsRecomputed != 0 {
		t.Fatalf("星级应被重算为 0，实际 %d", res.StarsRecomputed)
	}
	if !res.Clamped {
		t.Fatal("谎报星级应标记为已修正")
	}
}

// TestTokenReuseRejected token 一次性。
func TestTokenReuseRejected(t *testing.T) {
	gl := testLevel()
	now := time.Now()
	used := now.Add(-time.Second)
	tok := BattleToken{
		ID: 1, UserID: 1, LevelID: gl.ID, Seed: 1,
		IssuedAt: now.Add(-time.Minute), ExpiresAt: now.Add(time.Hour), UsedAt: &used,
	}
	in := baseInput(gl)
	if _, err := ValidateSettle(tok, gl, in, antiCheatLimits(), now); err == nil {
		t.Fatal("已使用的 token 应被拒绝")
	}
}

func TestExpiredTokenRejected(t *testing.T) {
	gl := testLevel()
	now := time.Now()
	tok := BattleToken{
		ID: 1, UserID: 1, LevelID: gl.ID, Seed: 1,
		IssuedAt: now.Add(-2 * time.Hour), ExpiresAt: now.Add(-time.Hour), UsedAt: nil,
	}
	in := baseInput(gl)
	if _, err := ValidateSettle(tok, gl, in, antiCheatLimits(), now); err == nil {
		t.Fatal("过期 token 应被拒绝")
	}
}

func TestTokenLevelMismatchRejected(t *testing.T) {
	gl := testLevel()
	now := time.Now()
	tok := usableToken(gl.ID+1, now)
	in := baseInput(gl)
	if _, err := ValidateSettle(tok, gl, in, antiCheatLimits(), now); err == nil {
		t.Fatal("关卡不匹配的 token 应被拒绝")
	}
}

// TestSettleLimitsAreConservative 逐档确认裁剪上限随时长线性增长，
// 防止有人把 MaxScorePerSec 改成硬编码常量。
func TestSettleLimitsAreConservative(t *testing.T) {
	gl := testLevel()
	lim := antiCheatLimits()
	for _, sec := range []int{1, 5, 30, 120} {
		in := baseInput(gl)
		in.DurationMs = sec * 1000
		in.Shots = legalShots(gl, in.DurationMs)
		in.Hits = in.Shots / 2
		in.Reactions = legalReactions(gl, in.Shots)
		in.Score = int64(sec) * lim.MaxScorePerSec * 2 // 故意超一倍
		res := settleOK(t, gl, in)
		want := int64(sec) * lim.MaxScorePerSec
		full := int64(gl.StarTargets[len(gl.StarTargets)-1])
		if want > full {
			want = full
		}
		if res.Score != want {
			t.Fatalf("sec=%d 期望裁剪到 %d，实际 %d", sec, want, res.Score)
		}
	}
}

// TestBoundaryExactValuesAccepted 精确的上界值必须**通过**。
// 上界校验最常见的错误是写成 > 而不是 >=，把合法上报也拒了。
func TestBoundaryExactValuesAccepted(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.DurationMs = 30_000
	in.Shots = legalShots(gl, in.DurationMs)
	in.Hits = in.Shots
	in.Reactions = legalReactions(gl, in.Shots) // 恰好等于两个上界的较小者
	in.HeatMax = MaxReportedHeat                // 恰好等于上界
	in.Leaked = MaxKillsFor(gl)                 // 恰好等于上界
	in.Kills = MaxKillsFor(gl)
	res := settleOK(t, gl, in)
	_ = res
}

// TestRandomFuzzNeverPanics 用固定序列的伪随机输入跑一遍，
// 目的是保证任何组合都不会 panic（panic 会被 recover 成 500，且掩盖真实原因）。
func TestRandomFuzzNeverPanics(t *testing.T) {
	gl := testLevel()
	now := time.Now()
	maxKills := MaxKillsFor(gl)
	seed := uint64(20260926)
	next := func() int {
		seed = seed*6364136223846793005 + 1442695040888963407
		return int((seed >> 33) % 4000)
	}
	for i := 0; i < 2000; i++ {
		in := SettleInput{
			Result:      []string{"win", "lose", "bogus"}[next()%3],
			Stars:       next(),
			Score:       int64(next()),
			Kills:       next() % (maxKills + 5),
			Leaked:      next() % (maxKills + 5),
			HPLeft:      next(),
			WaveReached: next() % (gl.WaveCount + 3),
			DurationMs:  next(),
			Shots:       next(),
			Hits:        next(),
			Reactions:   next(),
			HeatMax:     next(),
		}
		// 只要求不 panic；接受或拒绝都合法
		_, _ = ValidateSettle(usableToken(gl.ID, now), gl, in, antiCheatLimits(), now)
	}
}
