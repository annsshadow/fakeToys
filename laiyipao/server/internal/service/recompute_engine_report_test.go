package service

// 引擎侧读数的可观测性（2026-10-15）。
//
// # 这组测试守的是什么
//
// 整局重算的**全部**拒发证据都写在 `settle_recomputes` 里，而那张表
// **只记不符**。于是最危险的一种状态 ——「校验根本没跑」——
// 在台账上的表现是 **0 行**，与「校验正常工作、从不不符」完全一样。
//
// 这组测试把那个不可区分的状态变成**看得见的一行字**。

import (
	"context"
	"encoding/json"
	"testing"
	"time"
)

// TestEngineTotalsStatusCoversEveryState 逐个状态断言 `Status` 的判据。
//
// 判据本身是运维知识，所以在这里钉死而不是让前端各自实现一遍。
func TestEngineTotalsStatusCoversEveryState(t *testing.T) {
	cases := []struct {
		name     string
		totals   EngineTotals
		wantHas  string
		wantMiss string
	}{
		{
			name:     "一局都没跑过",
			totals:   EngineTotals{},
			wantHas:  "尚无结算",
			wantMiss: "验真覆盖为 0",
		},
		{
			// ⚠️ 最危险的一格：跑到重算的局不少，但一局都没验成。
			// 台账此时也是空的 —— 两个读数同时为空。
			name: "验真覆盖为 0（静默失效）",
			totals: EngineTotals{
				RecomputeEngineCounters: RecomputeEngineCounters{
					CannotRun: 7, Mismatch: 3, Verified: 0, Skipped: 0,
				},
				SettledTotal: 10,
			},
			wantHas:  "验真覆盖为 0",
			wantMiss: "尚无结算",
		},
		{
			// ⚠️ 这一格也危险：所有局都被「放行」了，
			// 但放行的理由是**算不出来**，不是算出来没问题。
			name: "过半算不出来",
			totals: EngineTotals{
				RecomputeEngineCounters: RecomputeEngineCounters{
					Verified: 2, CannotRun: 8, Mismatch: 0, Skipped: 0,
				},
				SettledTotal: 10, VerifiedPermille: 200,
			},
			wantHas:  "cannot_run 偏高",
			wantMiss: "正常",
		},
		{
			name: "健康",
			totals: EngineTotals{
				RecomputeEngineCounters: RecomputeEngineCounters{
					Verified: 190, Mismatch: 4, CannotRun: 2, Skipped: 4,
				},
				SettledTotal: 200, VerifiedPermille: 950,
			},
			wantHas:  "正常",
			wantMiss: "验真覆盖为 0",
		},
		{
			// 全部是老客户端（没上报哈希）—— 这不算「失效」，
			// 因为那条路径本来就是放行。所以不能报成「覆盖为 0」。
			name: "全是老客户端",
			totals: EngineTotals{
				RecomputeEngineCounters: RecomputeEngineCounters{Skipped: 12},
				SettledTotal:            12, VerifiedPermille: -1,
			},
			wantHas:  "验真覆盖",
			wantMiss: "验真覆盖为 0",
		},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := engineTotalsStatus(tc.totals)
			if !contains(got, tc.wantHas) {
				t.Errorf("状态串应含 %q，实际 %q", tc.wantHas, got)
			}
			if contains(got, tc.wantMiss) {
				t.Errorf("状态串不该含 %q，实际 %q", tc.wantMiss, got)
			}
		})
	}
}

func indexOf(s, sub string) int {
	for i := 0; i+len(sub) <= len(s); i++ {
		if s[i:i+len(sub)] == sub {
			return i
		}
	}
	return -1
}

// TestSummaryExposesEngineReadings 断言 `AdminRecomputeSummary` 真的带上了引擎读数。
//
// 为什么必须测这一条：计数器在 `recompute_engine.go` 里、汇总在
// `recompute_gate.go` 里，两处不同文件。**加计数器却忘了接到汇总上**，
// 编译照过、测试照绿、运营照样看不见 —— 那就是白加。
// 这条断言的是 JSON 里**真的出现**了 `engine` 键。
func TestSummaryExposesEngineReadings(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	ts.resetEngineCounters()

	// 空读数时也必须带 `engine` 键 —— 字段缺失与「读数为 0」要分得开，
	// 否则前端会把「没接」当成「全 0」，而全 0 正是要告警的状态。
	sum, err := ts.AdminRecomputeSummary(ctx, time.Now().Add(-time.Hour), 5)
	if err != nil {
		t.Fatalf("汇总失败：%v", err)
	}
	raw, err := json.Marshal(sum)
	if err != nil {
		t.Fatalf("汇总序列化失败：%v", err)
	}
	var doc map[string]any
	if err := json.Unmarshal(raw, &doc); err != nil {
		t.Fatalf("汇总反序列化失败：%v", err)
	}
	eng, ok := doc["engine"]
	if !ok {
		t.Fatalf("汇总 JSON 里没有 engine 键：%s", raw)
	}
	engMap, ok := eng.(map[string]any)
	if !ok {
		t.Fatalf("engine 不是对象：%T", eng)
	}
	for _, k := range []string{
		"verified", "mismatch", "cannot_run", "skipped",
		"settled_total", "verified_permille", "status",
	} {
		if _, has := engMap[k]; !has {
			t.Errorf("engine 里缺 %q 键：%v", k, engMap)
		}
	}
	// 台账那两个老键也必须在（本次改动不能把它们弄丢）
	for _, k := range []string{"totals", "levels", "mode", "demoted"} {
		if _, has := doc[k]; !has {
			t.Errorf("汇总 JSON 里原有的 %q 键被弄丢了", k)
		}
	}

	// 跑一局真哈希，verified 必须涨且出现在读数里
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})
	in, tp, err := ts.realHashSettle(t, ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, in); err != nil {
		t.Fatalf("结算应成功：%v", err)
	}
	sum2, err := ts.AdminRecomputeSummary(ctx, time.Now().Add(-time.Hour), 5)
	if err != nil {
		t.Fatalf("汇总失败：%v", err)
	}
	if sum2.Engine.Verified != 1 {
		t.Errorf("跑完一局后 verified 应为 1，实际 %d", sum2.Engine.Verified)
	}
	if sum2.Engine.SettledTotal != 1 {
		t.Errorf("settled_total 应为 1，实际 %d", sum2.Engine.SettledTotal)
	}
	if sum2.Engine.VerifiedPermille != 1000 {
		t.Errorf("一局验成时覆盖率应为 1000‰，实际 %d", sum2.Engine.VerifiedPermille)
	}
}

// TestSetRecomputeModeRejectsGarbage 切档的非法值必须**报错**而不是静默降级。
//
// 静默降级在这里特别危险：运营把档位拼错 → 服务退回 off →
// 防护静默关闭，而界面上显示的是一个看起来合法的档位名。
func TestSetRecomputeModeRejectsGarbage(t *testing.T) {
	ts := openTestService(t)

	if err := ts.SetRecomputeMode("enforse"); err == nil {
		t.Error("拼错的档位应报错")
	}
	if err := ts.SetRecomputeMode(""); err == nil {
		t.Error("空档位应报错")
	}
	// 非法调用**不得**改变当前档位
	if got := ts.RecomputeMode(); got != RecomputeEnforce {
		t.Errorf("非法调用后档位不该变，实际 %q（期望保持 %q）", got, RecomputeEnforce)
	}
	// 合法值要真的生效
	if err := ts.SetRecomputeMode(RecomputeShadow); err != nil {
		t.Fatalf("切 shadow 应成功：%v", err)
	}
	if got := ts.RecomputeMode(); got != RecomputeShadow {
		t.Errorf("切档后读到 %q，期望 %q", got, RecomputeShadow)
	}
	_ = ts.SetRecomputeMode(RecomputeEnforce)
}
