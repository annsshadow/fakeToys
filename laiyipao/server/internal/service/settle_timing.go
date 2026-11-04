// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// settle_timing.go — R80：结算写路径的分段计时与慢结算观测。
//
// # 为什么要有它
//
// 结算是钱的路：「结算变慢」过去只有一个端到端体感，没有分段归因 —
// 慢了都不知道慢在 token 抢占、战报插入（含 R63/R64 触发器）、
// 钱包/流水、还是事务后的榜单/评分收尾。这里给每段计时，
// 超过预算阈值时打一行**可定位**的日志（含 battle_id 与分段拆解）。
//
// # 日志内容纪律（单独检查过，别破）
//
// 慢日志只含：battle_id、user_id（内部代理 id）、各分段毫秒数。
// **绝不含**：token 明文、battle seed、replay_hash、昵称、IP。
// settle_timing_test.go 有一条专门的泄漏断言在守这条。
//
// # 触发器时间在哪一段
//
// R63/R64 的增量触发器在 `INSERT INTO battle_records` 语句内联触发，
// 应用侧无法把它从 insert 里拆出来 —— tx_insert 段 = 插入 + 触发器。
// 触发器自身的份额由 R63 的 pgbench A/B 实测给出（c=8 +0.077ms/行、
// c=32 +0.58ms/行），不在这里假装能拆。

import (
	"context"
	"fmt"
	"sync"
	"time"
)

// TraceIDFrom 从请求上下文取 R84 的 trace id（没有时返回 "-"）。
//
// 链路：httpapi.traceMiddleware 把 id 写进 fiber Locals（fasthttp
// userValues）→ handler 调 service 时传的 `c.Context()`
// （*fasthttp.RequestCtx 作为 context.Context）按同一 string key 返回。
// 服务内方法约定：要打「属于某次请求」的日志时用它取 id，
// 不要自己发明第二套关联字段。
func TraceIDFrom(ctx context.Context) string {
	if v, ok := ctx.Value("trace_id").(string); ok && v != "" {
		return v
	}
	return "-"
}

// settleNow 是结算分段计时的**统一可替换时间源**（R93）。
//
// # 为什么需要它
//
// 分段覆盖率（已计时之和 / 端到端）曾经直接读真实墙钟。真实墙钟里
// 夹杂着未计时的窗口（ValidateSettle、序列化、榜单 flush、评分），
// 这些窗口在慢 CI 上会被放大 —— 于是同一条「覆盖 ≥50%」断言在快机器上
// 绿、在慢机器上只有 42.9%，门禁变成随机事件。
//
// 计时点全部读同一个变量后，测试可以替换成假时钟：
// 每段耗时由测试决定，覆盖率断言变成确定性的。
// 生产默认仍是 time.Now，行为不变。
var settleNow = time.Now

// settleSince 用**同一个**时间源计算一段耗时。
//
// 不要写回 `time.Since(x)` —— 那会让某一段偷偷绕开假时钟，
// 覆盖率就又不是确定性的了。
func settleSince(t time.Time) time.Duration { return settleNow().Sub(t) }

// SettleTimings 是一次结算的分段耗时。
type SettleTimings struct {
	TokenLoad    time.Duration // 事务前读 token
	SkillsCheck  time.Duration // replay_skills S 段比对（含 DB 读）
	BuildLoadPre time.Duration // 事务前构筑快照（写库用 buildRaw）
	Recompute    time.Duration // 整局引擎重算（CPU）
	TxClaim      time.Duration // token 抢占 UPDATE
	TxPrefetch   time.Duration // R81：nextval 预取战报 id（让 INSERT 挪到事务尾）
	TxWallet     time.Duration // 掉落 grantWallet（钱包 + 流水）
	TxProgress   time.Duration // applyProgress
	TxStars      time.Duration // level_stars upsert
	TxTasks      time.Duration // bumpTasks
	TxInsert     time.Duration // 战报 INSERT（触发器内联在此段；R81 起在事务尾）
	TailLoad     time.Duration // 事务后钱包/构筑并行收尾
	Total        time.Duration
}

// SettleSegmentNames 给聚合与日志一个稳定的分段顺序（/metrics 渲染也用）。
var SettleSegmentNames = []string{
	"token_load", "skills_check", "build_load_pre", "recompute",
	"tx_claim", "tx_prefetch", "tx_wallet", "tx_progress", "tx_stars", "tx_tasks",
	"tx_insert", "tail_load",
}

// segments 按稳定顺序返回各分段值（与 settleSegmentNames 对齐）。
func (t SettleTimings) segments() []time.Duration {
	return []time.Duration{
		t.TokenLoad, t.SkillsCheck, t.BuildLoadPre, t.Recompute,
		t.TxClaim, t.TxPrefetch, t.TxWallet, t.TxProgress, t.TxStars, t.TxTasks,
		t.TxInsert, t.TailLoad,
	}
}

// SettleSegmentStat 是一个分段的聚合快照（count/sum/max，毫秒）。
type SettleSegmentStat struct {
	Count int64
	SumMs int64
	MaxMs int64
}

// SettleStats 是 /metrics 与探针共用的结算聚合快照。
type SettleStats struct {
	Total    int64 // 结算总次数
	Slow     int64 // 超阈值次数
	TotalMs  int64 // 端到端总耗时之和（均值可由此推出）
	Segments map[string]SettleSegmentStat
}

// settleAgg 是进程内聚合器。互斥锁 + 纯整数累加，每次结算一次 Lock。
type settleAgg struct {
	mu   sync.Mutex
	seg  map[string]*SettleSegmentStat
	slow int64
	sum  int64
}

func newSettleAgg() *settleAgg {
	m := map[string]*SettleSegmentStat{}
	for _, n := range SettleSegmentNames {
		m[n] = &SettleSegmentStat{}
	}
	return &settleAgg{seg: m}
}

func (a *settleAgg) record(t SettleTimings, slow bool) {
	a.mu.Lock()
	defer a.mu.Unlock()
	for i, n := range SettleSegmentNames {
		st := a.seg[n]
		ms := t.segments()[i].Milliseconds()
		st.Count++
		st.SumMs += ms
		if ms > st.MaxMs {
			st.MaxMs = ms
		}
	}
	a.sum += t.Total.Milliseconds()
	if slow {
		a.slow++
	}
}

func (a *settleAgg) snapshot() SettleStats {
	a.mu.Lock()
	defer a.mu.Unlock()
	out := SettleStats{Segments: map[string]SettleSegmentStat{}, TotalMs: a.sum, Slow: a.slow}
	var total int64
	for n, st := range a.seg {
		out.Segments[n] = *st
		total = st.Count
	}
	out.Total = total
	return out
}

// SettleStats 返回结算聚合快照（/metrics 与测试用）。
func (s *Service) SettleStats() SettleStats {
	return s.settleAgg.snapshot()
}

// recordSettleTiming 每次结算末尾调用：聚合 + 超预算打可定位慢日志。
//
// R84：慢日志带 req=<trace id>（与访问日志/错误详情同一标识）——
// 慢结算不再是一座孤岛，能拼回它所属的那次 HTTP 请求。
// id 来自中间件的白名单值或服务端生成值，本身即合法日志内容（无注入面）。
func (s *Service) recordSettleTiming(ctx context.Context, t SettleTimings, battleID, userID int64) {
	threshold := s.Cfg().SettleSlowMs
	slow := threshold > 0 && t.Total.Milliseconds() >= int64(threshold)
	s.settleAgg.record(t, slow)
	if slow {
		// ⚠️ 内容纪律（R80 要求 3）：只允许 id 与毫秒数。
		// 加字段前先想「它会不会是凭证/昵称/IP」——是就别放。
		fmt.Printf("[settle-slow] req=%s battle=%d user=%d total=%dms "+
			"token_load=%d skills=%d build_pre=%d recompute=%d "+
			"claim=%d prefetch=%d wallet=%d progress=%d stars=%d tasks=%d insert=%d tail=%d\n",
			TraceIDFrom(ctx), battleID, userID, t.Total.Milliseconds(),
			t.TokenLoad.Milliseconds(), t.SkillsCheck.Milliseconds(),
			t.BuildLoadPre.Milliseconds(), t.Recompute.Milliseconds(),
			t.TxClaim.Milliseconds(), t.TxPrefetch.Milliseconds(), t.TxWallet.Milliseconds(),
			t.TxProgress.Milliseconds(), t.TxStars.Milliseconds(), t.TxTasks.Milliseconds(),
			t.TxInsert.Milliseconds(), t.TailLoad.Milliseconds())
	}
}

// settleTimingProbe 是探针/测试专用的逐请求采样钩子（生产默认 nil，零开销）。
//
// 只读快照，探针不得改写。挂它的成本是一次 nil 判断。
var settleTimingProbe func(SettleTimings)

// settleProbeSet 仅测试/探针包内设置；返回还原函数。
func settleProbeSet(hook func(SettleTimings)) func() {
	prev := settleTimingProbe
	settleTimingProbe = hook
	return func() { settleTimingProbe = prev }
}
