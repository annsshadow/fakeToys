// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// reconcile.go — R78：一致性对账（只读）。
//
// # 为什么需要它
//
// R63/R64 引入了写入侧增量表（dashboard_*）、R34+ 引入了流水账本
// （wallet_flows）、R23 引入了兑换计数（used_count）—— 它们全是
// 「同一份事实写两遍」的机制。写两遍的东西迟早漂移，而漂移的症状是
// 「数字慢慢不对了」，没有任何报错。对账就是定期把「两遍」对在一起：
// 全绿证明机制没漂移，有差异则精确到键。
//
// # 合法漂移源（探针确认的排除项，改动它们先读这里）
//
//   - **energy（体力）**：LoadWallet 的懒恢复直接 UPDATE 钱包、不写流水
//     （按公式投影，不是事件）—— 能量**不参与**流水对账。
//   - **初始包不写流水**：initNewUserTx 直接 INSERT 钱包
//     （coin 2000 / gem 50 / keys 1）—— 期望值 = 初始值 + Σdelta。
//   - **放弃战斗消耗 token 但不产战报**（AbandonBattle 标 used_at）——
//     「已消耗 token 数 > 战报数」是合法的，差额 = 放弃数（进 notes，
//     不作为 diff）。真正的 token 不变量是：无重复结算（同一 token 两张战报）、
//     无未标记结算（战报存在而 token 仍 unused = 泄漏）。
//
// # 输出纪律
//
// 差异报告到「哪张表、哪个键、期望 vs 实际」；样本每类至多 50 条，
// 超出的部分只报总数（Truncated=true）。全绿时 Diffs 为空。

import (
	"context"
	"fmt"
	"time"
)

// ReconcileDiff 是一条不一致：哪个检查、哪个键、期望 vs 实际。
type ReconcileDiff struct {
	Check    string `json:"check"`
	Key      string `json:"key"`
	Expected string `json:"expected"`
	Actual   string `json:"actual"`
}

// ReconcileReport 是一次全量对账的结果。
type ReconcileReport struct {
	OK              bool             `json:"ok"`
	Checks          int              `json:"checks"`
	Diffs           []ReconcileDiff  `json:"diffs"`
	ViolationCounts map[string]int64 `json:"violation_counts"`
	Truncated       bool             `json:"truncated"`
	Notes           []string         `json:"notes"`
	DurationMs      int64            `json:"duration_ms"`
	CheckedAt       time.Time        `json:"checked_at"`
}

// reconcileSampleLimit 是每类检查最多回报的差异样本条数。
const reconcileSampleLimit = 50

// AdminReconcile 全量对账（只读，不写任何业务表）。
//
// 全量模式当前实测（1M users + 1M battle_records + 1M wallet_flows）耗时
// 为秒级（见台账 R78）；若未来涨到分钟级，先按用户/时间窗加 scope 参数，
// 不要在调用方轮询硬扛。
//
// # 巡检节奏（R79 的裁定）
//
// 适合每天跑一次（外部 cron/巡检脚本 GET 本端点，`ok=false` 即告警）。
// 它必须保持**只读**：结果只进调用方日志/告警通道，**不写审计、
// 不落快照表**——对账若变成写库动作，它自己就成了「同一份事实写两遍」
// 的第十个机制，也要被对账，套娃没有底。恢复演练里它已由
// `smoke_after_restore` 的第 25 步（admin.reconcile）自动执行。
func (s *Service) AdminReconcile(ctx context.Context) (ReconcileReport, error) {
	start := time.Now()
	rep := ReconcileReport{
		OK:              true,
		Diffs:           []ReconcileDiff{},
		ViolationCounts: map[string]int64{},
		Notes: []string{
			"energy（体力）不参与流水对账：懒恢复按公式投影、不写流水（合法漂移源）",
		},
		CheckedAt: start,
	}

	type checkFn func(context.Context, *ReconcileReport) error
	checks := []struct {
		name string
		fn   checkFn
	}{
		{"wallet_coin", s.reconcileWalletCoin},
		{"wallet_gem", s.reconcileWalletGem},
		{"wallet_keys", s.reconcileWalletKeys},
		{"wallet_tokens", s.reconcileWalletTokens},
		{"dashboard_battle_stats", s.reconcileBattleStats},
		{"dashboard_reaction_counts", s.reconcileReactionCounts},
		{"battle_double_settle", s.reconcileDoubleSettle},
		{"battle_unmarked_settle", s.reconcileUnmarkedSettle},
		{"redeem_used_count", s.reconcileRedeemUsedCount},
	}
	for _, c := range checks {
		if err := c.fn(ctx, &rep); err != nil {
			return ReconcileReport{}, fmt.Errorf("reconcile %s: %w", c.name, err)
		}
		rep.Checks++
	}

	// token 消耗 vs 战报数：信息项（放弃战斗会合法拉开差额）。
	var used, records int64
	if err := s.pool.QueryRow(ctx,
		`SELECT (SELECT COUNT(*) FROM battle_tokens WHERE used_at IS NOT NULL),
		        (SELECT COUNT(*) FROM battle_records)`).Scan(&used, &records); err != nil {
		return ReconcileReport{}, fmt.Errorf("reconcile token stats: %w", err)
	}
	rep.Notes = append(rep.Notes, fmt.Sprintf(
		"battle_tokens 已消耗=%d，battle_records=%d，差额=%d（正数=放弃战斗合法消耗；负数=无 token 的战报，批量灌数/历史数据会出现）",
		used, records, used-records))

	for _, n := range rep.ViolationCounts {
		if n > 0 {
			rep.OK = false
			break
		}
	}
	rep.DurationMs = time.Since(start).Milliseconds()
	return rep, nil
}

// reconcileRows 跑一条「差异查询」并装填样本与总数。
//
// q 必须返回 (key, expected, actual) 三列文本，按差异行产出；
// totalQ 返回该检查的差异总行数（样本被截断时报告仍完整）。
func (s *Service) reconcileRows(ctx context.Context, rep *ReconcileReport, check, q, totalQ string) error {
	var total int64
	if err := s.pool.QueryRow(ctx, totalQ).Scan(&total); err != nil {
		return err
	}
	rep.ViolationCounts[check] = total
	if total == 0 {
		return nil
	}
	if total > reconcileSampleLimit {
		rep.Truncated = true
	}
	rows, err := s.pool.Query(ctx, q+fmt.Sprintf(" LIMIT %d", reconcileSampleLimit))
	if err != nil {
		return err
	}
	defer rows.Close()
	for rows.Next() {
		var key, exp, act string
		if err := rows.Scan(&key, &exp, &act); err != nil {
			return err
		}
		rep.Diffs = append(rep.Diffs, ReconcileDiff{Check: check, Key: key, Expected: exp, Actual: act})
	}
	return rows.Err()
}

// reconcileWalletCurrency 核对一个货币列：balance == 初始值 + Σdelta。
//
// ⚠️ col 与 currency 是调用点的编译期常量（'coin'/'gem'/'keys'），直接拼接；
// 不为这个 helper 开参数化通道（调用面越小，拼 SQL 的面越小）。
func (s *Service) reconcileWalletCurrency(ctx context.Context, rep *ReconcileReport, check, col, currency string, initial int64) error {
	// 差异行：期望 = initial + Σflows（无流水按 0），实际 = 钱包余额。
	diff := fmt.Sprintf(`
		SELECT 'user#' || w.user_id,
		       (%[1]d + COALESCE(f.s, 0))::text,
		       w.%[2]s::text
		  FROM user_wallets w
		  LEFT JOIN (SELECT user_id, SUM(delta) s FROM wallet_flows WHERE currency = '%[3]s' GROUP BY user_id) f
		    ON f.user_id = w.user_id
		 WHERE w.%[2]s IS DISTINCT FROM (%[1]d + COALESCE(f.s, 0))
		 ORDER BY w.user_id`, initial, col, currency)
	total := fmt.Sprintf(`SELECT COUNT(*) FROM (%s) d`, diff)
	return s.reconcileRows(ctx, rep, check, diff, total)
}

func (s *Service) reconcileWalletCoin(ctx context.Context, rep *ReconcileReport) error {
	return s.reconcileWalletCurrency(ctx, rep, "wallet_coin", "coin", "coin", 2000)
}
func (s *Service) reconcileWalletGem(ctx context.Context, rep *ReconcileReport) error {
	return s.reconcileWalletCurrency(ctx, rep, "wallet_gem", "gem", "gem", 50)
}
func (s *Service) reconcileWalletKeys(ctx context.Context, rep *ReconcileReport) error {
	return s.reconcileWalletCurrency(ctx, rep, "wallet_keys", "keys", "keys", 1)
}

// reconcileWalletTokens 核对道具余额：balance == Σdelta（道具无初始值）。
func (s *Service) reconcileWalletTokens(ctx context.Context, rep *ReconcileReport) error {
	diff := `
		SELECT 'user#' || t.user_id || ':' || t.token,
		       COALESCE(f.s, 0)::text,
		       t.balance::text
		  FROM user_tokens t
		  LEFT JOIN (SELECT user_id, currency, SUM(delta) s FROM wallet_flows
		              WHERE currency IN (SELECT DISTINCT token FROM user_tokens)
		              GROUP BY user_id, currency) f
		    ON f.user_id = t.user_id AND f.currency = t.token
		 WHERE t.balance IS DISTINCT FROM COALESCE(f.s, 0)
		 ORDER BY t.user_id`
	total := `SELECT COUNT(*) FROM (` + diff + `) d`
	return s.reconcileRows(ctx, rep, "wallet_tokens", diff, total)
}

// reconcileBattleStats 核对每关增量统计与直接聚合（FULL JOIN 双向）。
func (s *Service) reconcileBattleStats(ctx context.Context, rep *ReconcileReport) error {
	// 期望 = battle_records 直接聚合；实际 = dashboard_battle_stats。
	diff := `
		SELECT 'level#' || COALESCE(d.level_id, a.level_id),
		       CONCAT_WS(',', 'attempts=' || a.attempts, 'wins=' || a.wins,
		                 'duration_sum=' || a.duration_sum, 'wave_sum=' || a.wave_sum),
		       CONCAT_WS(',', 'attempts=' || d.attempts, 'wins=' || d.wins,
		                 'duration_sum=' || d.duration_sum, 'wave_sum=' || d.wave_sum)
		  FROM (
		      -- R149：计数器按 shard 分片，先合并回每关一行再对账。
		      SELECT level_id, SUM(attempts)::bigint attempts,
		             SUM(wins)::bigint wins,
		             SUM(duration_sum)::bigint duration_sum,
		             SUM(wave_sum)::bigint wave_sum
		        FROM dashboard_battle_stats GROUP BY level_id
		  ) d
		  FULL JOIN (
		      SELECT level_id, COUNT(*)::bigint attempts,
		             (COUNT(*) FILTER (WHERE result='win'))::bigint wins,
		             COALESCE(SUM(duration_ms),0)::bigint duration_sum,
		             COALESCE(SUM(wave_reached),0)::bigint wave_sum
		        FROM battle_records GROUP BY level_id
		  ) a ON a.level_id = d.level_id
		 WHERE d.attempts IS DISTINCT FROM a.attempts
		    OR d.wins IS DISTINCT FROM a.wins
		    OR d.duration_sum IS DISTINCT FROM a.duration_sum
		    OR d.wave_sum IS DISTINCT FROM a.wave_sum
		 ORDER BY 1`
	total := `SELECT COUNT(*) FROM (` + diff + `) d`
	return s.reconcileRows(ctx, rep, "dashboard_battle_stats", diff, total)
}

// reconcileReactionCounts 核对反应计数逐键与直接聚合。
func (s *Service) reconcileReactionCounts(ctx context.Context, rep *ReconcileReport) error {
	diff := `
		SELECT 'level#' || COALESCE(d.level_id, a.level_id) || ':' || COALESCE(d.reaction, a.reaction),
		       COALESCE(a.cnt, 0)::text,
		       COALESCE(d.count, 0)::text
		  FROM (
		      -- R149：同一 (level, reaction) 可能有多片，先 SUM 再对账。
		      SELECT level_id, reaction, SUM(count)::bigint AS count
		        FROM dashboard_reaction_counts GROUP BY level_id, reaction
		  ) d
		  FULL JOIN (
		      SELECT b.level_id, e.key AS reaction, SUM(e.value::bigint)::bigint AS cnt
		        FROM battle_records b,
		             LATERAL jsonb_each_text(b.reactions_used) AS e(key, value)
		       WHERE jsonb_typeof(b.reactions_used) = 'object'
		       GROUP BY b.level_id, e.key
		  ) a ON a.level_id = d.level_id AND a.reaction = d.reaction
		 WHERE d.count IS DISTINCT FROM a.cnt
		 ORDER BY 1`
	total := `SELECT COUNT(*) FROM (` + diff + `) d`
	return s.reconcileRows(ctx, rep, "dashboard_reaction_counts", diff, total)
}

// reconcileDoubleSettle 同一开局 token 不得对应两张战报（无重复结算）。
func (s *Service) reconcileDoubleSettle(ctx context.Context, rep *ReconcileReport) error {
	diff := `
		SELECT 'token#' || battle_token_id, '1', COUNT(*)::text
		  FROM battle_records
		 WHERE battle_token_id IS NOT NULL
		 GROUP BY battle_token_id HAVING COUNT(*) > 1
		 ORDER BY battle_token_id`
	total := `SELECT COUNT(*) FROM (` + diff + `) d`
	return s.reconcileRows(ctx, rep, "battle_double_settle", diff, total)
}

// reconcileUnmarkedSettle 战报存在而 token 仍未标记已用 = 结算泄漏。
func (s *Service) reconcileUnmarkedSettle(ctx context.Context, rep *ReconcileReport) error {
	diff := `
		SELECT 'battle#' || br.id, 'token used_at 非空', 'token#' || br.battle_token_id || ' used_at IS NULL'
		  FROM battle_records br
		  JOIN battle_tokens t ON t.id = br.battle_token_id
		 WHERE t.used_at IS NULL
		 ORDER BY br.id`
	total := `SELECT COUNT(*) FROM (` + diff + `) d`
	return s.reconcileRows(ctx, rep, "battle_unmarked_settle", diff, total)
}

// reconcileRedeemUsedCount 兑换码计数与兑换记录逐码一致。
func (s *Service) reconcileRedeemUsedCount(ctx context.Context, rep *ReconcileReport) error {
	diff := `
		SELECT 'code#' || c.id || ':' || c.code,
		       COALESCE(u.n, 0)::text,
		       c.used_count::text
		  FROM redeem_codes c
		  LEFT JOIN (SELECT code_id, COUNT(*) n FROM redeem_usages GROUP BY code_id) u
		    ON u.code_id = c.id
		 WHERE c.used_count IS DISTINCT FROM COALESCE(u.n, 0)
		 ORDER BY c.id`
	total := `SELECT COUNT(*) FROM (` + diff + `) d`
	return s.reconcileRows(ctx, rep, "redeem_used_count", diff, total)
}
