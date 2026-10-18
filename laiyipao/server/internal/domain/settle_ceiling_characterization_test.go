package domain

import (
	"errors"
	"testing"
	"time"
)

// 结算防作弊能力的**边界刻画**（本轮安全审计新增）。
//
// # 为什么这是一条「刻画」而不是「守卫」
//
// 本项目的防作弊是**上限校验**而不是**重算**（服务端刻意不跑战斗引擎，
// 那是 I-5「防线值守」与 I-6「回放验真」得以成立的前提）。
// 上限校验的性质是：**它挡住越界，挡不住"恰好顶满"**。
//
// 后果具体且严重：任何持有合法 token 的玩家，只要把每个上报字段
// 填成该字段的**理论上界**，就能拿到与真实通关**逐位相同**的结算结果 ——
// 满星、满分、满掉落，外加 `max_stage` 推进。
//
// 这一点此前只以散文形式记在 README 的「已知边界」第 3/11 条里。
// 散文的问题不是不准确，而是**读不到** —— 而这里的危害恰恰是
// 「有人以为它被覆盖了」。
//
// 所以把它写成测试：任何人收紧/放宽上限校验，这条会先红；
// 将来若产品决定让服务端跑引擎，这条会**主动失败**并提醒删掉自己。
//
// # 判据为什么是「全部字段顶满且被接受」
//
// 只断言某一个字段顶满（例如 `reactions == maxReactions`）能被
// 「只拒绝单个极端字段」的实现骗过 —— 而那仍然留下完整的刷分路径。
// 真正要钉住的是**整体**：一个全顶满的报文被接受。

// TestMaximalLegalSettlementIsAccepted —— 刻画「上限校验挡不住顶满」。
//
// ⚠️ 这条测试**必须通过**。它红不代表有 bug，而代表上限校验变严了
// （那是好事），届时应当连同本文件一起更新为「已收紧」的断言。
func TestMaximalLegalSettlementIsAccepted(t *testing.T) {
	now := time.Now()
	for _, id := range []int{1, 50, 100} {
		gl := GenerateLevel(id)
		maxKills := MaxKillsFor(gl)

		// 把每个字段都填成该关的理论上界。
		//
		// ⚠️ 每一步都要用**与被测代码同一个来源**推导，否则这条会
		// 因为「我自己算错了上界」而红 —— 那是假绿的反面（假红），
		// 同样有害。这里刻意直接调用 maxShotsFor / MaxReactionsPerEnemy /
		// scoreCapFor，与 ValidateSettle 内部用的是同一批常量。
		durMs := ScoreFullAtSec * 1000 // 恰好够到满分额度的时长
		maxShots := maxShotsFor(durMs)
		maxReactions := int64(maxShots) * MaxReactionsPerHit
		if absMax := int64(maxKills) * MaxReactionsPerEnemy; absMax < maxReactions {
			maxReactions = absMax
		}
		sec := int64(durMs) / 1000
		maxScore := scoreCapFor(gl, sec)

		in := SettleInput{
			Result:      "win",
			Score:       maxScore,
			Kills:       maxKills,
			Leaked:      0,
			HPLeft:      int(gl.BaseHP),
			WaveReached: gl.WaveCount,
			DurationMs:  durMs,
			Shots:       int(maxShots),
			Hits:        int(maxShots),
			Reactions:   int(maxReactions),
			HeatMax:     MaxReportedHeat,
		}

		tok := BattleToken{ID: 1, UserID: 1, LevelID: id, Seed: 42,
			IssuedAt: now, ExpiresAt: now.Add(time.Hour)}
		limits := SettleLimits{MinDurationMs: 15_000, MaxScorePerSec: 40_000}

		res, err := ValidateSettle(tok, gl, in, limits, now)
		if err != nil {
			t.Errorf("第 %d 关：全顶满的上报被拒（err=%v）—— "+
				"上限校验已收紧，请更新本刻画测试并重算攻击面", id, err)
			continue
		}
		if !res.Win {
			t.Errorf("第 %d 关：全顶满的上报竟判负，win=%v", id, res.Win)
		}
		if res.StarsRecomputed != len(gl.StarTargets) {
			t.Errorf("第 %d 关：全顶满竟只得 %d 星（满分 %d）",
				id, res.StarsRecomputed, len(gl.StarTargets))
		}
		if res.Score != maxScore {
			t.Errorf("第 %d 关：得分 %d 应等于额度 %d", id, res.Score, maxScore)
		}
		// 掉落必须**非空且可观** —— 否则「顶满也没好处」，
		// 这条刻画就说错了强度。
		coin := res.Loot["coin"]
		if coin <= 0 {
			t.Errorf("第 %d 关：全顶满的掉落金币 = %d，应为正", id, coin)
		}
		if res.Stars >= 3 && res.Loot["keys"] <= 0 {
			t.Errorf("第 %d 关：3 星顶满竟没掉钥匙 —— 「顶满无收益」的说法不成立", id)
		}
		if res.Loot["energy"] <= 0 {
			t.Errorf("第 %d 关：通关顶满竟不回补体力", id)
		}
	}
}

// TestEnergyCostExceedsWinRefund 是上面那条的**经济后果**刻画。
//
// 为什么单独成条：上面的刻画证明「顶满能拿满掉落」，但那条本身
// 不能回答「能不能无限刷」—— 关键在体力。
//
// 实测数据：EnergyCost（第 1 关 6）> EnergyOnWin（5），
// 且 100 关里**没有任何一关**出现净回体力。所以体力是一条
// **真实但缓慢**的节流阀：新号 120 点体力可以顶满约 20 局，
// 之后每 6 分钟回 1 点。
//
// ⚠️ 若将来有人把 EnergyOnWin 调到 ≥ EnergyCost，这条会红 ——
// 那意味着**体力不再构成任何约束**，刷分变成无时限的。
func TestEnergyCostExceedsWinRefund(t *testing.T) {
	rates := DefaultDropRates()
	caps := DefaultLootCaps()
	refund := satMul(rates.EnergyOnWin, 1, caps.MaxEnergyPerBattle)

	profitable := 0
	for id := 1; id <= TotalLevels; id++ {
		gl := GenerateLevel(id)
		if refund >= int64(gl.EnergyCost) {
			profitable++
		}
	}
	if profitable > 0 {
		t.Errorf("有 %d/%d 关的通关回补体力 ≥ 消耗体力 —— "+
			"体力不再构成节流阀，顶满结算可无限循环", profitable, TotalLevels)
	}
}

// TestSettleRejectsOvershootBeyondEveryCap 守住另一半。
//
// 上一条说「顶满被接受」，这一条说「**超过顶满**被拒绝」——
// 两者必须**同时**成立，只有一条成立都是错的：
//
//   - 只有上一条 → 上限校验根本不存在
//   - 只有这一条 → 上限校验太紧，正常玩家被拒
//
// 判据用**恰好越界一个单位**（`+1`），而不是某个拍出来的「大值」：
// 前者精确落在判据的分界处，后者会因为「远大于上界」而
// 掩盖掉上界算错的事实。
// 越界一单位，而不是「拍一个很大的数」：前者精确落在判据的分界处，
// 后者会因为「远大于上界」而掩盖掉上界本身算错的事实。
func TestSettleRejectsOvershootBeyondEveryCap(t *testing.T) {
	now := time.Now()
	gl := GenerateLevel(1)
	maxKills := MaxKillsFor(gl)
	tok := BattleToken{ID: 1, UserID: 1, LevelID: 1, Seed: 42,
		IssuedAt: now, ExpiresAt: now.Add(time.Hour)}
	limits := SettleLimits{MinDurationMs: 15_000, MaxScorePerSec: 40_000}

	base := func() SettleInput {
		durMs := ScoreFullAtSec * 1000
		maxShots := maxShotsFor(durMs)
		return SettleInput{
			Result: "lose", DurationMs: durMs,
			Shots: int(maxShots), Hits: int(maxShots),
			Kills: maxKills, HPLeft: int(gl.BaseHP), WaveReached: gl.WaveCount,
		}
	}

	cases := []struct {
		name  string
		mut   func(*SettleInput)
		wants error
	}{
		{"击杀越界", func(i *SettleInput) { i.Kills = maxKills + 1 }, ErrTooManyKills},
		{"波次越界", func(i *SettleInput) { i.WaveReached = gl.WaveCount + 1 }, ErrWaveExceeded},
		{"发射越界", func(i *SettleInput) { i.Shots++ }, ErrTooManyShots},
		{"命中越界", func(i *SettleInput) { i.Hits = i.Shots + 1 }, ErrInvalidHitRate},
		{"反应越界", func(i *SettleInput) { i.Reactions = maxKills*MaxReactionsPerEnemy + 1 }, ErrTooManyReactions},
		{"漏怪越界", func(i *SettleInput) { i.Leaked = maxKills + 1 }, ErrTooManyLeaked},
		{"血量越界", func(i *SettleInput) { i.HPLeft = int(gl.BaseHP) + 1 }, ErrHPLeftExceedsBase},
		{"时长越界", func(i *SettleInput) { i.DurationMs = MaxPlausibleDurationMs + 1 }, ErrInvalidField},
		{"热量越界", func(i *SettleInput) { i.HeatMax = MaxReportedHeat + 1 }, ErrInvalidField},
		{"时长为零", func(i *SettleInput) { i.DurationMs = 0 }, ErrTooShort},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			in := base()
			tc.mut(&in)
			if _, err := ValidateSettle(tok, gl, in, limits, now); err == nil {
				t.Errorf("%s：越界一个单位竟被接受", tc.name)
			} else if !errors.Is(err, tc.wants) {
				t.Errorf("%s：错误类型 = %v，期望 %v", tc.name, err, tc.wants)
			}
		})
	}
}
