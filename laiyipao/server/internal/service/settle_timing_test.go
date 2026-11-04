// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R80：结算分段计时的行为测试。
//
//  1. 成功结算后聚合里必须有真实数字（不是「计时全零」的摆设）；
//  2. 超阈值必须打慢日志，且日志内容纪律：只有 id 与毫秒数 ——
//     不含 token 明文 / seed / replay_hash / 昵称 / IP（泄漏断言单独跑）；
//  3. 分段之和必须能对上端到端（对不上说明有未计时的空档 —— R80 要求 1）。
//
// ⚠️ R93：覆盖率断言改成**假时钟下的确定性断言**；真时钟只做冒烟
// （总时长 > 0、各段非负、和不超总时长），不再按真实墙钟判覆盖比例 ——
// 否则慢 CI 上未计时窗口被放大，同一条断言会随机变红。

import (
	"context"
	"encoding/json"
	"io"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

// r80Settle 走一次真实结算（真 token + 真重算哈希），返回战报 id 与分段。
func r80Settle(t *testing.T, ts *testService, ctx context.Context, uid int64) int64 {
	t.Helper()
	var got SettleTimings
	restore := settleProbeSet(func(tm SettleTimings) { got = tm })
	defer restore()

	id := r80SettleBody(t, ts, ctx, uid)
	if got.Total == 0 {
		t.Fatal("探针钩子没有收到分段 —— settleTimingProbe 接线断了")
	}
	return id
}

// r80SettleBody 是 r80Settle 去掉探针断言的版本 —— 供需要**自定义探针**
// 或替换时间源（R93 假时钟）的用例复用同一条真实结算路径。
func r80SettleBody(t *testing.T, ts *testService, ctx context.Context, uid int64) int64 {
	t.Helper()
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

// settleFakeClock 是 R93 的**假时钟**：只被 `settleNow` 调用。
//
// 调用顺序与实现严格对应：第 0 次是 tmStart，之后每两个调用是一段
// （start → end）。因此：
//
//	奇数次调用 = 段起点：在上一个终点之后留 gap
//	偶数次调用 = 段终点（含最后那次 Total）：起点 + 固定段长
//
// 段耗时与空档都由测试决定，**与机器快慢无关**。
type settleFakeClock struct {
	n         int
	t         time.Time
	lastStart time.Time
}

const (
	fakeSegmentLen = 10 * time.Millisecond
	fakeGapLen     = 1 * time.Millisecond
)

func (c *settleFakeClock) now() time.Time {
	i := c.n
	c.n++
	switch {
	case i == 0:
		c.t = time.Date(2026, 3, 1, 0, 0, 0, 0, time.UTC)
	case i%2 == 1: // 段起点
		c.t = c.t.Add(fakeGapLen)
		c.lastStart = c.t
	default: // 段终点
		c.t = c.lastStart.Add(fakeSegmentLen)
	}
	return c.t
}

func settleWithFakeClock(t *testing.T, ts *testService, ctx context.Context, uid int64) SettleTimings {
	t.Helper()
	clock := &settleFakeClock{}
	orig := settleNow
	settleNow = clock.now
	t.Cleanup(func() { settleNow = orig })

	var got SettleTimings
	restore := settleProbeSet(func(tm SettleTimings) { got = tm })
	defer restore()
	r80SettleBody(t, ts, ctx, uid) // 真实结算路径，但计时读假时钟
	return got
}

// TestSettleTimingCoverageIsDeterministic（R93）：覆盖率必须是**确定性的**。
//
// 改前直接读真实墙钟：慢 CI 上未计时窗口（校验/序列化/榜单 flush/评分）被放大，
// 实测覆盖率只有 42.9%，而同一条断言在快机器上绿 —— 门禁成了随机事件。
//
// 门禁不是性能指标：一个「有时红有时绿」的断言等于没有断言。
//
// # 本用例判什么
//
//   - 每段耗时必须等于假时钟给的 10ms —— 证明**所有**计时点都读同一个时间源；
//     谁改回 `time.Since` 就会漏掉假时钟，这一段立刻不是 10ms。
//   - 覆盖份额 ≥50%（原断言的意义**不放宽**）：已计时部分必须占端到端的一半以上。
//
// 假时钟下 total = 12×(10ms + 1ms) ≈ 132ms、sum = 120ms → 覆盖率 ≈90%，
// 与机器负载无关。
func TestSettleTimingCoverageIsDeterministic(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})

	got := settleWithFakeClock(t, ts, ctx, uid)

	for i, name := range SettleSegmentNames {
		if d := got.segments()[i]; d != fakeSegmentLen {
			t.Errorf("分段 %s = %v，期望 %v\n"+
				"有计时点没有读 settleNow（可能改回了 time.Since），"+
				"覆盖率就不再是确定性的。", name, d, fakeSegmentLen)
		}
	}

	var sum time.Duration
	for _, d := range got.segments() {
		sum += d
	}
	total := got.Total
	if sum > total {
		t.Fatalf("分段之和 %v > 端到端 %v —— 计时器有重叠段", sum, total)
	}
	coverage := float64(sum) / float64(total)
	t.Logf("假时钟分段覆盖：%v/%v = %.1f%%（确定性）", sum, total, coverage*100)
	if coverage < 0.5 {
		t.Errorf("已计时份额 %.1f%% < 50%% —— 存在大的未计时空档，预算归因不可信", coverage*100)
	}
}

// TestSettleTimingSmokeRealClock 是**真实机器上的冒烟断言**（R93 要求 3）。
//
// 假时钟测逻辑；真时钟只测「不崩、数字合理」：
// 总时长 > 0、各段非负、各段之和不超过端到端（含 2ms 容差）。
//
// ⚠️ **不要**在这里断言覆盖率比例：真实墙钟里未计时窗口的占比
// 随机器负载变化，比例断言在慢 CI 上必然翻车（那正是 R93 要修的形态）。
func TestSettleTimingSmokeRealClock(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})

	var got SettleTimings
	restore := settleProbeSet(func(tm SettleTimings) { got = tm })
	defer restore()
	r80SettleBody(t, ts, ctx, uid)

	if got.Total <= 0 {
		t.Fatalf("真实时钟下 total = %v，必须 > 0", got.Total)
	}
	var sum time.Duration
	for i, d := range got.segments() {
		if d < 0 {
			t.Errorf("分段 %s = %v，不能为负", SettleSegmentNames[i], d)
		}
		sum += d
	}
	const tolerance = 2 * time.Millisecond
	if sum > got.Total+tolerance {
		t.Errorf("分段之和 %v > 端到端 %v + %v —— 计时器有重叠段", sum, got.Total, tolerance)
	}
	t.Logf("真实时钟冒烟：total=%v，分段和=%v", got.Total, sum)
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
