// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R62：看板查询合并的**等价性**护栏。
//
// 本轮把 7 段毫秒级小表统计合并成 1 条语句、并改写了 DAU 的 SQL。
// 合并的代价是 SQL 复杂度上升，所以必须有一条「数值与改前逐一相同」的
// 断言，而不是只看接口 200。这里把**改前**的 13 段查询原样保留成
// `legacyDashboard`（串行执行、顺序与旧实现一致），在同一份夹具上
// 与合并后的实现做 reflect.DeepEqual 比对。
//
// 顺带把「防线总数」的口径钉死：旧实现里 `dashboard challenges` 段
// 会把 `defense_challenges` 的行数写进 `Defenses.Total`，而
// `dashboard defenses` 段又写 `defenses` 的行数 —— 两段并行，最终值
// 由调度决定。夹具刻意让两边数量不同（2 条防线、5 次今日挑战），
// 断言必须是 2 / 5，防止以后有人把字段再写串。

import (
	"context"
	"reflect"
	"testing"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

func TestDashboardMergedMatchesLegacy(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	mustExec := func(q string, args ...any) {
		t.Helper()
		if _, err := ts.pool.Exec(ctx, q, args...); err != nil {
			t.Fatalf("夹具写入失败：%v\nSQL: %s", err, q)
		}
	}
	var uid1, uid2, uid3 int64
	insertUser := func(dst *int64, token, nick string, guest bool, status int16, created string) {
		t.Helper()
		if err := ts.pool.QueryRow(ctx, `
			INSERT INTO users (guest_token, nickname, is_guest, status, created_at, last_login_at)
			VALUES ($1, $2, $3, $4, now() + ($5)::interval, now())
			RETURNING id`, token, nick, guest, status, created).Scan(dst); err != nil {
			t.Fatalf("建用户失败：%v", err)
		}
	}
	insertUser(&uid1, "r62_u1", "R62客1", true, 1, "0 seconds")
	insertUser(&uid2, "r62_u2", "R62号2", false, 1, "0 seconds")
	insertUser(&uid3, "r62_u3", "R62禁3", true, 2, "-5 days")

	var b1, b2, b3, b4 int64
	insertBattle := func(dst *int64, uid int64, level int, result string, dur int, age string, reactions string) {
		t.Helper()
		if err := ts.pool.QueryRow(ctx, `
			INSERT INTO battle_records (user_id, level_id, result, duration_ms, created_at, reactions_used)
			VALUES ($1, $2, $3, $4, now() + ($5)::interval, $6::jsonb)
			RETURNING id`, uid, level, result, dur, age, reactions).Scan(dst); err != nil {
			t.Fatalf("建战报失败：%v", err)
		}
	}
	// 四份战报覆盖：今日/昨日/10 天前/13 天前；三条在 DAU 窗口内。
	insertBattle(&b1, uid1, 1, "win", 1000, "0 seconds", `{"steam_burst":2,"overheat":1}`)
	insertBattle(&b2, uid1, 2, "lose", 2000, "-1 days", `{}`)
	insertBattle(&b3, uid2, 100, "win", 3000, "-10 days", `null`)
	insertBattle(&b4, uid3, 1, "win", 4000, "-13 days", `{"steam_burst":3}`)

	mustExec(`INSERT INTO user_progress (user_id, max_stage, level_exp) VALUES ($1, 10, 100), ($2, 30, 700)`, uid1, uid2)
	mustExec(`INSERT INTO level_stars (user_id, level_id, stars, clears) VALUES ($1, 100, 3, 1), ($2, 100, 0, 0)`, uid1, uid2)
	mustExec(`INSERT INTO wallet_flows (user_id, currency, delta, balance, reason) VALUES
		($1, 'coin', 5, 5, 'r62'), ($1, 'coin', -2, 3, 'r62'), ($2, 'gem', 7, 7, 'r62')`, uid1, uid2)
	mustExec(`INSERT INTO orders (user_id, order_no, sku, amount, status) VALUES ($1, 'r62-paid', 'sku', 6, 'paid'), ($2, 'r62-pending', 'sku', 6, 'pending')`, uid1, uid2)
	mustExec(`INSERT INTO user_sign_ins (user_id, sign_date, day_index, reward) VALUES ($1, CURRENT_DATE, 1, '{}')`, uid1)

	var codeID int
	if err := ts.pool.QueryRow(ctx, `SELECT COALESCE(MAX(id), 0) + 1 FROM redeem_codes`).Scan(&codeID); err != nil {
		t.Fatalf("取兑换码 id 失败：%v", err)
	}
	mustExec(`INSERT INTO redeem_codes (id, code, reward, max_uses) VALUES ($1, 'R62-CODE', '{}', 1)`, int32(codeID))
	mustExec(`INSERT INTO redeem_usages (code_id, user_id) VALUES ($1, $2)`, int32(codeID), uid1)

	mustExec(`INSERT INTO replay_verifications (battle_id, verifier_id, expected_hash, actual_hash, matched) VALUES
		($1, $3, 'h', 'h', TRUE), ($2, $3, 'h', 'x', FALSE)`, b1, b2, uid1)

	var def1, def2 int64
	if err := ts.pool.QueryRow(ctx, `INSERT INTO defenses (owner_id, snapshot, snapshot_hash) VALUES ($1, '{}', '') RETURNING id`, uid1).Scan(&def1); err != nil {
		t.Fatalf("建防线失败：%v", err)
	}
	if err := ts.pool.QueryRow(ctx, `INSERT INTO defenses (owner_id, snapshot, snapshot_hash) VALUES ($1, '{}', '') RETURNING id`, uid2).Scan(&def2); err != nil {
		t.Fatalf("建防线失败：%v", err)
	}
	// 5 条今日挑战挂在第一条防线上（与 defenses=2 刻意不同）。
	mustExec(`INSERT INTO defense_challenges (defense_id, challenger_id, seed, won, created_at) VALUES
		($1, $2, 1, TRUE, CURRENT_DATE + INTERVAL '1 hour'),
		($1, $2, 2, FALSE, CURRENT_DATE + INTERVAL '2 hours'),
		($1, $2, 3, TRUE, CURRENT_DATE + INTERVAL '3 hours'),
		($1, $2, 4, TRUE, CURRENT_DATE + INTERVAL '4 hours'),
		($1, $2, 5, FALSE, CURRENT_DATE + INTERVAL '5 hours')`,
		def1, uid3)

	legacy, err := legacyDashboard(ctx, ts)
	if err != nil {
		t.Fatalf("旧实现参考值失败：%v", err)
	}
	got, err := ts.adminDashboardUncached(ctx)
	if err != nil {
		t.Fatalf("合并后实现失败：%v", err)
	}

	// 口径优先级：先钉死被旧实现写串的那两个字段，再做整体等价。
	if got.Defenses.Total != 2 {
		t.Errorf("防线总数应为 defenses 行数 2，实际 %d（写成挑战次数就是回归）", got.Defenses.Total)
	}
	if got.Defenses.ChallengesToday != 5 {
		t.Errorf("今日挑战应为 5，实际 %d", got.Defenses.ChallengesToday)
	}
	if !reflect.DeepEqual(legacy, got) {
		t.Fatalf("合并后看板与改前不一致\n旧=%+v\n新=%+v", legacy, got)
	}
}

// legacyDashboard 是 R62 之前 `adminDashboardUncached` 的 13 段查询，
// 串行执行、顺序与旧实现一致。它只作为测试参考值存在，不参与生产路径。
func legacyDashboard(ctx context.Context, ts *testService) (Dashboard, error) {
	var d Dashboard
	scan := func(q string, dest ...any) error {
		return ts.pool.QueryRow(ctx, q).Scan(dest...)
	}

	if err := scan(`
		SELECT COUNT(*),
		       COUNT(*) FILTER (WHERE is_guest),
		       COUNT(*) FILTER (WHERE NOT is_guest),
		       COUNT(*) FILTER (WHERE status = 2),
		       COUNT(*) FILTER (WHERE created_at >= CURRENT_DATE)
		FROM users`,
		&d.Users.Total, &d.Users.Guests, &d.Users.Wechat, &d.Users.Banned, &d.Users.NewToday); err != nil {
		return d, err
	}
	if err := scan(`
		SELECT COUNT(*),
		       COUNT(*) FILTER (WHERE created_at >= CURRENT_DATE),
		       COUNT(*) FILTER (WHERE result = 'win'),
		       COALESCE(AVG(duration_ms), 0)
		FROM battle_records`,
		&d.Battles.Total, &d.Battles.Today, &d.Battles.Wins, &d.Battles.AvgDurationMs); err != nil {
		return d, err
	}
	if err := scan(`
		SELECT COUNT(*) FILTER (WHERE created_at >= CURRENT_DATE),
		       COUNT(*)
		FROM defense_challenges`,
		&d.Defenses.ChallengesToday, &d.Defenses.Total); err != nil {
		return d, err
	}
	if err := scan(`SELECT COUNT(*) FROM defenses`, &d.Defenses.Total); err != nil {
		return d, err
	}
	if err := scan(`
		SELECT COALESCE(AVG(max_stage), 0), COALESCE(AVG(level_exp), 0),
		       (SELECT COUNT(*) FROM level_stars WHERE level_id = 100 AND clears > 0)
		FROM user_progress`,
		&d.Progression.AvgMaxStage, &d.Progression.AvgPower, &d.Progression.Stage100Clear); err != nil {
		return d, err
	}
	if err := scan(`
		SELECT
		  COALESCE(SUM(delta) FILTER (WHERE currency='coin' AND delta > 0), 0),
		  COALESCE(-SUM(delta) FILTER (WHERE currency='coin' AND delta < 0), 0),
		  COALESCE(SUM(delta) FILTER (WHERE currency='gem' AND delta > 0), 0),
		  COALESCE(-SUM(delta) FILTER (WHERE currency='gem' AND delta < 0), 0)
		FROM wallet_flows`,
		&d.Economy.CoinIn, &d.Economy.CoinOut, &d.Economy.GemIn, &d.Economy.GemOut); err != nil {
		return d, err
	}
	if err := scan(`
		SELECT COUNT(*) FILTER (WHERE status IN ('paid','delivered')),
		       COUNT(*) FILTER (WHERE status = 'pending')
		FROM orders`,
		&d.Economy.OrdersPaid, &d.Economy.OrdersPending); err != nil {
		return d, err
	}
	if err := scan(`SELECT COUNT(*) FROM user_sign_ins WHERE sign_date = CURRENT_DATE`,
		&d.Economy.SignInsToday); err != nil {
		return d, err
	}
	if err := scan(`SELECT COUNT(*) FROM redeem_usages WHERE created_at >= CURRENT_DATE`,
		&d.Economy.RedeemsToday); err != nil {
		return d, err
	}
	if err := scan(`
		SELECT COUNT(*), COUNT(*) FILTER (WHERE matched), COUNT(*) FILTER (WHERE NOT matched)
		FROM replay_verifications`,
		&d.Verification.Checked, &d.Verification.Matched, &d.Verification.Mismatched); err != nil {
		return d, err
	}

	rows, err := ts.pool.Query(ctx, `
		SELECT e.key, SUM(e.value::bigint) AS total
		FROM battle_records b,
		     LATERAL jsonb_each_text(b.reactions_used) AS e(key, value)
		WHERE jsonb_typeof(b.reactions_used) = 'object'
		GROUP BY e.key
		ORDER BY total DESC
		LIMIT 100`)
	if err != nil {
		return d, err
	}
	defer rows.Close()
	agg := map[string]int64{}
	for rows.Next() {
		var key string
		var total int64
		if err := rows.Scan(&key, &total); err != nil {
			return d, err
		}
		agg[key] = total
	}
	if err := rows.Err(); err != nil {
		return d, err
	}
	for _, spec := range domain.AllReactionSpecs() {
		d.ReactionUsage = append(d.ReactionUsage, ReactionCount{
			Reaction: string(spec.Key), Count: agg[string(spec.Key)],
		})
	}

	drows, err := ts.pool.Query(ctx, `
		SELECT d::date AS day,
		       COUNT(DISTINCT user_id) AS dau
		FROM generate_series(CURRENT_DATE - INTERVAL '13 days', CURRENT_DATE, '1 day') d
		LEFT JOIN battle_records br ON br.created_at >= d AND br.created_at < d + INTERVAL '1 day'
		GROUP BY d::date ORDER BY 1`)
	if err != nil {
		return d, err
	}
	defer drows.Close()
	for drows.Next() {
		var day time.Time
		var dau int64
		if err := drows.Scan(&day, &dau); err != nil {
			return d, err
		}
		d.DailyActive = append(d.DailyActive, DauPoint{Date: day.Format("2006-01-02"), DAU: dau})
	}
	if err := drows.Err(); err != nil {
		return d, err
	}

	frows, err := ts.pool.Query(ctx, `
		SELECT level_id, COUNT(*), COUNT(*) FILTER (WHERE result='win')
		FROM battle_records GROUP BY level_id ORDER BY level_id`)
	if err != nil {
		return d, err
	}
	defer frows.Close()
	for frows.Next() {
		var f StageFunnel
		if err := frows.Scan(&f.LevelID, &f.Attempts, &f.Number); err != nil {
			return d, err
		}
		d.StageFunnel = append(d.StageFunnel, f)
	}
	return d, frows.Err()
}
