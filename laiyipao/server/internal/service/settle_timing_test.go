// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R80：结算分段计时的行为测试。
//
//  1. 成功结算后聚合里必须有真实数字（不是「计时全零」的摆设）；
//  2. 超阈值必须打慢日志，且日志内容纪律：只有 id 与毫秒数 ——
//     不含 token 明文 / seed / replay_hash / 昵称 / IP（泄漏断言单独跑）；
//  3. 分段之和必须能对上端到端（对不上说明有未计时的空档 —— R80 要求 1）。

import (
	"context"
	"encoding/json"
	"io"
	"os"
	"strings"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// r80Settle 走一次真实结算（真 token + 真重算哈希），返回战报 id 与分段。
func r80Settle(t *testing.T, ts *testService, ctx context.Context, uid int64) int64 {
	t.Helper()
	var got SettleTimings
	restore := settleProbeSet(func(tm SettleTimings) { got = tm })
	defer restore()

	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Fatalf("开局失败：%v", err)
	}
	tokenID := int64(tp.TokenID)

	// 真哈希：与 e2e 的 realReplayHash 同一组装（否则 enforce 档拒发）。
	var seedText string
	if err := ts.pool.QueryRow(ctx, `SELECT seed FROM battle_tokens WHERE id=$1`, tokenID).Scan(&seedText); err != nil {
		t.Fatalf("读 seed 失败：%v", err)
	}
	build, err := ts.LoadBuildSnapshot(ctx, uid)
	if err != nil {
		t.Fatalf("读构筑失败：%v", err)
	}
	out := domain.Replay(domain.GenerateLevel(1), buildSnapshotForReplay(t, build), seedText,
		[]int{0, 1, -1}, "", domain.EnemiesByID(), domain.SkillsByID(), domain.DefaultScoreRules())
	if out.Err != nil {
		t.Fatalf("重算跑不起来：%v", out.Err)
	}

	resp, err := ts.SettleBattle(ctx, uid, tokenID, domain.SettleInput{
		Result: "lose", Score: 1000, DurationMs: 30_000,
		Shots: 200, Hits: 100, Reactions: 20, HeatMax: 50,
		HPLeft: 500, WaveReached: 1, Kills: 30, Leaked: 0,
		ReplayHash:    out.ComputedHash,
		ElementsUsed:  map[string]int{"fire": 3},
		ReactionsUsed: map[string]int{"overheat": 20},
		TerrainUsed:   []string{},
		CardPicks:     []int{0, 1, -1},
	})
	if err != nil {
		t.Fatalf("结算失败：%v", err)
	}
	if got.Total == 0 {
		t.Fatal("探针钩子没有收到分段 —— settleTimingProbe 接线断了")
	}
	return resp.BattleID
}

// buildSnapshotForReplay 把服务层快照 map 转成 domain.BuildSnapshot
// （与 e2e realReplayHash 的 JSON roundtrip 同路径）。
func buildSnapshotForReplay(t *testing.T, build map[string]any) domain.BuildSnapshot {
	t.Helper()
	raw, err := marshalJSON(build)
	if err != nil {
		t.Fatalf("快照编码失败：%v", err)
	}
	var bs domain.BuildSnapshot
	if err := json.Unmarshal(raw, &bs); err != nil {
		t.Fatalf("快照解码失败：%v", err)
	}
	return bs
}

// TestSettleTimingAggregatesHaveRealNumbers：结算后聚合里必须有真数。
func TestSettleTimingAggregatesHaveRealNumbers(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})

	before := ts.SettleStats()
	r80Settle(t, ts, ctx, uid)
	after := ts.SettleStats()

	if after.Total != before.Total+1 {
		t.Fatalf("结算总数 %d → %d，应恰好 +1", before.Total, after.Total)
	}
	for _, seg := range SettleSegmentNames {
		d := after.Segments[seg].SumMs - before.Segments[seg].SumMs
		if d < 0 {
			t.Errorf("分段 %s 的 sum 倒退（%d）", seg, d)
		}
	}
	// 至少战报插入段必须有非零耗时（它是写库，不可能 0ms；若恒 0 说明计时没接上）。
	if after.Segments["tx_insert"].SumMs == before.Segments["tx_insert"].SumMs {
		t.Error("tx_insert 的累计毫秒没动 —— 分段计时是空转的")
	}
}

// TestSettleTimingSegmentsSumToTotal：分段之和与端到端的覆盖关系。
//
// 允许未计时窗口存在（ValidateSettle / 序列化 / flushLeaderboards / computePower），
// 但它必须**小**：已计时份额低于 50% 就说明有大的空档没人看见 —— 那正是
// 「加起来对不上 P95」要抓的事。
func TestSettleTimingSegmentsSumToTotal(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})

	var got SettleTimings
	restore := settleProbeSet(func(tm SettleTimings) { got = tm })
	// 探针要在结算前就挂上（r80Settle 内部也挂一次，这里先挂住它拿同一帧）。
	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Fatalf("开局失败：%v", err)
	}
	tokenID := int64(tp.TokenID)
	var seedText string
	if err := ts.pool.QueryRow(ctx, `SELECT seed FROM battle_tokens WHERE id=$1`, tokenID).Scan(&seedText); err != nil {
		t.Fatalf("读 seed 失败：%v", err)
	}
	build, err := ts.LoadBuildSnapshot(ctx, uid)
	if err != nil {
		t.Fatalf("读构筑失败：%v", err)
	}
	out := domain.Replay(domain.GenerateLevel(1), buildSnapshotForReplay(t, build), seedText,
		[]int{0, 1, -1}, "", domain.EnemiesByID(), domain.SkillsByID(), domain.DefaultScoreRules())
	if _, err := ts.SettleBattle(ctx, uid, tokenID, domain.SettleInput{
		Result: "lose", Score: 1000, DurationMs: 30_000, Shots: 200, Hits: 100,
		Reactions: 20, HeatMax: 50, HPLeft: 500, WaveReached: 1, Kills: 30, Leaked: 0,
		ReplayHash: out.ComputedHash, ElementsUsed: map[string]int{"fire": 3},
		ReactionsUsed: map[string]int{"overheat": 20}, TerrainUsed: []string{}, CardPicks: []int{0, 1, -1},
	}); err != nil {
		t.Fatalf("结算失败：%v", err)
	}
	restore()

	var sum int64
	for _, d := range got.segments() {
		sum += d.Milliseconds()
	}
	total := got.Total.Milliseconds()
	if sum > total {
		t.Fatalf("分段之和 %dms > 端到端 %dms —— 计时器有重叠段", sum, total)
	}
	coverage := float64(sum) / float64(total)
	t.Logf("分段覆盖：%d/%dms = %.1f%%（未计时窗口：校验/序列化/榜单 flush/评分）", sum, total, coverage*100)
	if coverage < 0.5 {
		t.Errorf("已计时份额 %.1f%% < 50%% —— 存在大的未计时空档，预算归因不可信", coverage*100)
	}
}

// TestSettleSlowLogContentDiscipline：超阈值必打慢日志，且**只含 id 与毫秒**。
//
// 这是 R80 要求 3 的专门检查：慢日志带 battle_id 与分段拆解，
// 但绝不能带 token 明文 / seed / replay_hash / 昵称 / IP。
func TestSettleSlowLogContentDiscipline(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})

	// 阈值压到 1ms：任何真实结算都超 → 必打日志。
	old := ts.Cfg().SettleSlowMs
	ts.Cfg().SettleSlowMs = 1
	t.Cleanup(func() { ts.Cfg().SettleSlowMs = old })

	logged := captureStdout(t, func() {
		r80Settle(t, ts, ctx, uid)
	})

	if !strings.Contains(logged, "[settle-slow]") {
		t.Fatalf("阈值 1ms 下没有打慢日志")
	}
	// R84：慢日志必须带 req=（无 HTTP 上下文时为 "-"，字段必须存在）。
	for _, want := range []string{"req=", "battle=", "user=", "total=", "insert=", "recompute=", "tail=", "tasks="} {
		if !strings.Contains(logged, want) {
			t.Errorf("慢日志缺 %q：%q", want, logged)
		}
	}
	// 泄漏面：seed / replay_hash / token 字符串 / IP 一律不得出现。
	// 它们都是十六进制/点分格式，用模式断言而不是值断言（值断言会被顺序变动搞脆）。
	for _, banned := range []string{"replay", "seed", "hash=", "token=", "10.0.0", "192.168", "昵称", "nickname"} {
		if strings.Contains(strings.ToLower(logged), banned) {
			t.Errorf("慢日志含禁词 %q：%q", banned, logged)
		}
	}
}

// captureStdout 捕获 fn 期间的 stdout 写入。
func captureStdout(t *testing.T, fn func()) string {
	t.Helper()
	old := os.Stdout
	r, w, err := os.Pipe()
	if err != nil {
		t.Fatalf("建管道失败：%v", err)
	}
	os.Stdout = w
	defer func() { os.Stdout = old }()

	done := make(chan string)
	go func() {
		raw, _ := io.ReadAll(r)
		done <- string(raw)
	}()
	fn()
	_ = w.Close()
	out := <-done
	os.Stdout = old
	return out
}
