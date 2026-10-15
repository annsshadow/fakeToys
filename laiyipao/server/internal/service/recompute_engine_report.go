package service

import "fmt"

// 把整局重算的分支计数接到管理端读数上（2026-10-15）。
//
// # 为什么要补这一步
//
// `recompute_engine.go` 加了四个计数器（verified / mismatch /
// cannot_run / skipped），理由是台账 `settle_recomputes` **只记不符**，
// 于是「校验正常工作」与「校验根本没跑」在台账上都是 0 行。
//
// 但光有计数器不解决问题 —— 它们只在进程内，运营看不到。
// **洞没关上等于没关。** 这一步把它们接进 `AdminRecomputeSummary`，
// 让「跑到了多少局」成为一个能在后台页面上读到的数字。
//
// # 三个必须一眼看出来的状态
//
//	Verified == 0 且 Skipped == 0
//	    没有任何一局被验真过 —— 校验是死的。**最危险**，因为台账也是空的。
//	CannotRun 持续上涨
//	    重算长期跑不起来（输入缺失 / 数据不完整）。
//	    每一局都被「放行」了，但**放行的理由是算不出来，不是算出来没问题**。
//	Verified 远大于 Mismatch
//	    正常状态。

// EngineTotals 引擎侧累计计数（进程生命周期内，不随 since 窗口变化）。
//
// ⚠️ 它与 `RecomputeTotals` 的口径**不同**，别混用：
//
//	RecomputeTotals  来自台账 SQL，只含**不符**的局，且受 `since` 窗口限制
//	EngineTotals     来自进程内计数器，含**所有**局，且**从进程启动算起**
//
// 两者互补：台账告诉你「哪些不符」，计数器告诉你「到底有没有在跑」。
// 单看任何一个都会误判 —— 只有台账会以为「没问题」，
// 而那正是最坏的那种误判。
type EngineTotals struct {
	RecomputeEngineCounters
	// SettledTotal = Verified + Mismatch + CannotRun + Skipped，
	// 即「走到重算这一步的局数」。用它做分母才能算出验真覆盖率。
	SettledTotal int64 `json:"settled_total"`
	// VerifiedPermille = Verified / SettledTotal * 1000。
	//
	// 之所以给比率而不只给绝对数：光看 `Verified=3` 不知道该高兴还是该担心，
	// 取决于同时跑了多少局。比率把这个上下文带出来。
	//
	// SettledTotal == 0 时为 -1 —— 「没有样本」与「覆盖率为 0」是两件事，
	// 前者是还没跑过一局，后者是跑了但一局都没验真成。
	VerifiedPermille int `json:"verified_permille"`
	// Status 是一句可直接显示的结论，见 engineTotalsStatus。
	Status string `json:"status"`
}

// engineTotals 汇总引擎侧计数并算出覆盖率与状态。
func (s *Service) engineTotals() EngineTotals {
	c := s.EngineCounters()
	total := c.Verified + c.Mismatch + c.CannotRun + c.Skipped
	t := EngineTotals{
		RecomputeEngineCounters: c,
		SettledTotal:            total,
		VerifiedPermille:        -1,
	}
	if total > 0 {
		t.VerifiedPermille = int(c.Verified * 1000 / total)
	}
	t.Status = engineTotalsStatus(t)
	return t
}

// engineTotalsStatus 给出**一眼可判**的结论，供后台直接显示。
//
// 判据放在后端而不是让前端算比率：什么算「低」、什么算「偏高」
// 是运维知识，搬到前端就变成两处各写一遍、必然漂移。
func engineTotalsStatus(t EngineTotals) string {
	switch {
	case t.SettledTotal == 0:
		return "尚无结算经过重算"
	case t.Verified == 0 && t.Skipped == 0:
		// 跑到重算这一步的局不少，但一局都没验成 —— 校验实际是死的。
		// 这时台账也是空的（一致不入账、不符才会入），两个读数同时为空。
		return "验真覆盖为 0：重算从未成功过（台账同样为空，两者叠加即为静默失效）"
	case t.CannotRun > 0 && t.CannotRun*2 >= t.SettledTotal:
		// 一半以上算不出来。注意此时 Verified 可能仍是 0，
		// 所以这条排在「验真覆盖为 0」之后、但在「正常」之前。
		return "过半结算因输入缺失而放行（cannot_run 偏高，重算长期失效）"
	case t.VerifiedPermille >= 900:
		return "正常"
	default:
		return fmt.Sprintf("验真覆盖 %d‰（%d/%d 局）",
			t.VerifiedPermille, t.Verified, t.SettledTotal)
	}
}
