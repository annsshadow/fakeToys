// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// Command smoke_restore 在「恢复出来的数据库」上走一遍真实玩家/管理端路径，
// 并巡检 dump/restore 最容易漏掉的隐性依赖（序列、触发器、索引、goose、时区）。
//
// ## 为什么不能只验 /readyz 200
//
// `readyz` 只证明「能连上库」。R58 的恢复演练证明了行数/外键/索引/goose 版本
// 一致，但**没有证明恢复库能真的玩**：
//
//   - BIGSERIAL 的当前值若没随 dump 回来，新插入会撞已有 id（唯一键冲突）；
//   - 触发器是 schema 的一部分，恢复后若没生效，服务一切正常，
//     只是 dashboard 计数表**静默**不再更新 —— 线上极难定位；
//   - 时区不同会让「今天」的统计口径漂移（每日任务/看板 Today）。
//
// 所以这里按「玩家路径 → 管理端路径 → 写侧触发器」的顺序真跑一遍，
// 每一步都断言**内容**（金币确实变了、战报确实查得到、看板确实 +1）。
//
// ## 用法
//
//	DATABASE_URL=postgres://postgres@127.0.0.1:5432/laiyipao_restore_drill \
//	BASE_URL=http://127.0.0.1:18080 \
//	EXPECTED_TZ=Etc/UTC \
//	MIGRATIONS_DIR=migrations \
//	go run ./cmd/smoke_restore
//
// 退出码：0 = 全通过；1 = 某一步失败，输出会指出步骤名。
//
// ## 安全阀
//
// 本命令会**写数据**（建游客、结算、建临时管理员/公告）。因此库名必须含
// `drill` 或 `smoke`；确需对别的库跑时必须显式设 `ALLOW_NON_DRILL=1`。
package main

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/service"
	"github.com/laiyipao/server/internal/store"
)

type smoke struct {
	ctx  context.Context
	base string
	pool *pgxpool.Pool
	svc  *service.Service
	hc   *http.Client

	nickname  string
	userID    int64
	userToken string

	preCoin    int64
	battleID   int64
	settleCoin int64

	preAttempts int64
	preWins     int64
	preWaveSum  int64
	preOverheat int64

	battleTokenID   int64
	settleLootCoin  int
	settleKills     int
	settleReactions int
	claimCoin       int64

	adminID    int64
	adminUser  string
	adminPass  string
	adminToken string

	preDashboardBattles int64

	announcementID int

	cleanups []func()
}

func main() {
	databaseURL := strings.TrimSpace(os.Getenv("DATABASE_URL"))
	base := strings.TrimRight(strings.TrimSpace(os.Getenv("BASE_URL")), "/")
	if databaseURL == "" || base == "" {
		fail("preflight", errors.New("DATABASE_URL 与 BASE_URL 都必须设置"))
	}
	if !isDrillDatabase(databaseURL) && os.Getenv("ALLOW_NON_DRILL") != "1" {
		fail("preflight", fmt.Errorf("拒绝在非演练库上跑写操作：%s（库名需含 drill/smoke；确需覆盖请设 ALLOW_NON_DRILL=1）", databaseName(databaseURL)))
	}

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Minute)
	defer cancel()

	pool, err := pgxpool.New(ctx, databaseURL)
	if err != nil {
		fail("db.connect", fmt.Errorf("建立连接池：%w", err))
	}
	if err := pool.Ping(ctx); err != nil {
		fail("db.connect", fmt.Errorf("ping：%w", err))
	}
	defer pool.Close()

	cfg := config.Load()
	cfg.DatabaseURL = databaseURL

	s := &smoke{
		ctx:      ctx,
		base:     base,
		pool:     pool,
		svc:      service.New(&store.DB{Pool: store.WrapPool(pool, cfg.DBAcquireTimeout)}, cfg),
		hc:       &http.Client{Timeout: 30 * time.Second},
		nickname: fmt.Sprintf("drill_%d", time.Now().UnixNano()),
	}

	fmt.Printf("[smoke] 数据库=%s API=%s\n", databaseName(databaseURL), base)

	steps := []struct {
		name string
		run  func(*smoke) error
	}{
		{"db.timezone", (*smoke).checkTimezone},
		{"db.sequences", (*smoke).checkSequences},
		{"db.triggers", (*smoke).checkTriggers},
		{"db.indexes", (*smoke).checkIndexes},
		{"db.goose", (*smoke).checkGoose},
		{"http.readyz", (*smoke).checkReadyz},

		// 先建临时管理员：后续管理端步骤依赖它，且它不碰 users 表，
		// 不会污染「玩家数」基线。
		{"admin.login", (*smoke).adminLogin},
		{"player.guest_login", (*smoke).guestLogin},
		{"admin.dashboard.baseline", (*smoke).dashboardBaseline},

		{"player.config", (*smoke).playerConfig},
		{"player.level", (*smoke).playerLevel},
		{"player.wallet_before", (*smoke).walletBefore},
		{"db.stats_before", (*smoke).statsBefore},
		{"player.battle_token", (*smoke).battleToken},
		{"player.settle", (*smoke).settle},

		// 写入侧触发器：结算刚刚 INSERT 了一条 battle_records。
		// 这里断言两张计数表真的动了 —— 这是恢复后最容易静默失效的一环。
		{"db.trigger_effect", (*smoke).triggerEffect},

		{"player.battle_detail", (*smoke).battleDetail},
		{"player.progress", (*smoke).progress},
		{"player.task_claim", (*smoke).taskClaim},
		{"player.wallet_ledger", (*smoke).walletLedger},

		{"admin.dashboard.after_settle", (*smoke).dashboardAfterSettle},
		{"admin.users", (*smoke).adminUsers},
		{"admin.announcement_visible", (*smoke).announcementVisible},
		{"admin.audit", (*smoke).adminAudit},

		// R79：对账收尾——「能玩」升级为「能玩且数据自洽」。
		// 必须放在全部写入步骤之后，覆盖 smoke 自己产生的战报/流水/公告。
		{"admin.reconcile", (*smoke).adminReconcile},
	}

	for _, st := range steps {
		if err := st.run(s); err != nil {
			s.cleanup()
			fail(st.name, err)
		}
		fmt.Printf("[ OK ] %s\n", st.name)
	}
	s.cleanup()
	fmt.Printf("[PASS] 恢复库全路径通过（%d 步）\n", len(steps))
}

func fail(step string, err error) {
	fmt.Fprintf(os.Stderr, "[FAIL] %s: %v\n", step, err)
	os.Exit(1)
}

// cleanups 逆序执行；失败只告警，不掩盖真正的失败步骤。
func (s *smoke) cleanup() {
	for i := len(s.cleanups) - 1; i >= 0; i-- {
		s.cleanups[i]()
	}
	s.cleanups = nil
}

func (s *smoke) onCleanup(fn func()) { s.cleanups = append(s.cleanups, fn) }

// --- HTTP ---

func (s *smoke) call(method, path, token string, body, out any) error {
	var rdr io.Reader
	if body != nil {
		b, err := json.Marshal(body)
		if err != nil {
			return err
		}
		rdr = bytes.NewReader(b)
	}
	req, err := http.NewRequestWithContext(s.ctx, method, s.base+path, rdr)
	if err != nil {
		return err
	}
	if body != nil {
		req.Header.Set("Content-Type", "application/json")
	}
	if token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	resp, err := s.hc.Do(req)
	if err != nil {
		return fmt.Errorf("%s %s：%w", method, path, err)
	}
	defer resp.Body.Close()
	raw, err := io.ReadAll(io.LimitReader(resp.Body, 8<<20))
	if err != nil {
		return fmt.Errorf("%s %s 读响应：%w", method, path, err)
	}
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return fmt.Errorf("%s %s → %d：%s", method, path, resp.StatusCode, truncate(raw))
	}
	if out != nil {
		if err := json.Unmarshal(raw, out); err != nil {
			return fmt.Errorf("%s %s 响应不是预期 JSON：%w（%s）", method, path, err, truncate(raw))
		}
	}
	return nil
}

func truncate(b []byte) string {
	const max = 400
	s := strings.TrimSpace(string(b))
	if len(s) > max {
		return s[:max] + "…"
	}
	return s
}

// --- 数据库隐性依赖 ---

// checkTimezone 断言恢复库的服务器时区与源库一致。
//
// 期望值由演练脚本从源库读出后通过 EXPECTED_TZ 传入；单独运行时可以不给，
// 此时只要求能读到非空值。
func (s *smoke) checkTimezone() error {
	var tz string
	if err := s.pool.QueryRow(s.ctx, "SHOW timezone").Scan(&tz); err != nil {
		return fmt.Errorf("SHOW timezone：%w", err)
	}
	if strings.TrimSpace(tz) == "" {
		return errors.New("SHOW timezone 返回空值")
	}
	if want := strings.TrimSpace(os.Getenv("EXPECTED_TZ")); want != "" {
		norm := func(v string) string {
			return strings.ToLower(strings.ReplaceAll(strings.TrimSpace(v), " ", ""))
		}
		if norm(tz) != norm(want) {
			return fmt.Errorf("恢复库时区=%q，源库=%q（不一致）", tz, want)
		}
	}
	fmt.Printf("       时区=%s\n", tz)
	return nil
}

// checkSequences 遍历所有 serial 序列，断言「下一个值」不会撞表内已有 id。
//
// dump 用 `--format=plain` 且不带 `--data-only` 时，序列的 setval 一般会回来；
// 但一旦漏了，症状是线上偶发 `duplicate key value violates unique constraint`，
// 而且只在「表里已有数据 + 新写入」时出现。
func (s *smoke) checkSequences() error {
	rows, err := s.pool.Query(s.ctx, `
		SELECT c.relname, a.attname, quote_ident(n.nspname)||'.'||quote_ident(sq.relname)
		FROM pg_class sq
		JOIN pg_namespace n ON n.oid = sq.relnamespace
		JOIN pg_depend d ON d.objid = sq.oid AND d.deptype = 'a'
		JOIN pg_class c ON c.oid = d.refobjid
		JOIN pg_attribute a ON a.attrelid = c.oid AND a.attnum = d.refobjsubid
		WHERE sq.relkind = 'S' AND n.nspname = 'public' AND c.relkind = 'r'
		ORDER BY 1, 2`)
	if err != nil {
		return fmt.Errorf("枚举序列：%w", err)
	}
	defer rows.Close()

	var (
		checked int
		bad     []string
	)
	for rows.Next() {
		var tbl, col, seq string
		if err := rows.Scan(&tbl, &col, &seq); err != nil {
			return err
		}
		checked++

		var maxID int64
		if err := s.pool.QueryRow(s.ctx,
			fmt.Sprintf(`SELECT COALESCE(MAX(%s), 0) FROM %s`, quoteIdent(col), quoteIdent(tbl))).
			Scan(&maxID); err != nil {
			return fmt.Errorf("%s.%s 读最大 id：%w", tbl, col, err)
		}

		var last int64
		var called bool
		if err := s.pool.QueryRow(s.ctx,
			"SELECT last_value, is_called FROM "+seq).Scan(&last, &called); err != nil {
			return fmt.Errorf("%s 读序列：%w", seq, err)
		}
		next := last
		if called {
			next = last + 1
		}
		if next <= maxID {
			bad = append(bad, fmt.Sprintf("%s.%s 下一个值=%d 会撞已有 id=%d", tbl, col, next, maxID))
		}
	}
	if err := rows.Err(); err != nil {
		return err
	}
	if checked < 15 {
		return fmt.Errorf("只找到 %d 个 serial 序列（基线 ≥15）—— 表结构可能没完整恢复", checked)
	}
	if len(bad) > 0 {
		return errors.New(strings.Join(bad, "；"))
	}
	fmt.Printf("       序列=%d 个，nextval 均大于表内最大 id\n", checked)
	return nil
}

var (
	requiredTriggers = []string{"trg_dashboard_battle_stats", "trg_dashboard_reaction_counts"}
	requiredFuncs    = []string{
		"dashboard_battle_stats_sync",
		"dashboard_battle_stats_apply",
		"dashboard_reaction_counts_sync",
		"dashboard_reaction_counts_apply",
	}
)

// checkTriggers 断言触发器「存在」且「启用」。
//
// 存在性还不够：`tgenabled='D'`（disabled）的触发器看起来一切正常，
// 只是永远不触发。所以判据是 `tgenabled <> 'D'`。
func (s *smoke) checkTriggers() error {
	rows, err := s.pool.Query(s.ctx, `
		SELECT t.tgname, t.tgenabled
		FROM pg_trigger t
		JOIN pg_class c ON c.oid = t.tgrelid
		JOIN pg_namespace n ON n.oid = c.relnamespace
		WHERE n.nspname = 'public' AND c.relname = 'battle_records' AND NOT t.tgisinternal`)
	if err != nil {
		return fmt.Errorf("读 battle_records 触发器：%w", err)
	}
	defer rows.Close()

	got := map[string]string{}
	for rows.Next() {
		var name, enabled string
		if err := rows.Scan(&name, &enabled); err != nil {
			return err
		}
		got[name] = enabled
	}
	if err := rows.Err(); err != nil {
		return err
	}
	for _, want := range requiredTriggers {
		enabled, ok := got[want]
		if !ok {
			return fmt.Errorf("缺少触发器 %s（恢复库的 battle_records 触发器集合=%v）", want, keys(got))
		}
		if enabled == "D" {
			return fmt.Errorf("触发器 %s 处于 disabled（tgenabled='D'），计数表会静默不更新", want)
		}
	}

	var funcs int
	if err := s.pool.QueryRow(s.ctx, `
		SELECT COUNT(*) FROM pg_proc p
		JOIN pg_namespace n ON n.oid = p.pronamespace
		WHERE n.nspname = 'public' AND p.proname = ANY($1)`,
		requiredFuncs).Scan(&funcs); err != nil {
		return fmt.Errorf("读触发器函数：%w", err)
	}
	if funcs != len(requiredFuncs) {
		return fmt.Errorf("触发器函数只回来了 %d/%d 个：%v", funcs, len(requiredFuncs), requiredFuncs)
	}
	fmt.Printf("       触发器=%v 已启用，函数 4/4 存在\n", requiredTriggers)
	return nil
}

// checkIndexes 用「数量下限」兜住索引整体丢失。
//
// 精确比对由 db_restore_drill.sh 做（源库 vs 恢复库）；这里只防灾难性缺失，
// 因此下限取得比当前基线（公共索引 116、外键 52）低一截。
func (s *smoke) checkIndexes() error {
	var indexes, fks, battleIndexes int
	if err := s.pool.QueryRow(s.ctx,
		`SELECT COUNT(*) FROM pg_indexes WHERE schemaname = 'public'`).Scan(&indexes); err != nil {
		return fmt.Errorf("统计公共索引：%w", err)
	}
	if err := s.pool.QueryRow(s.ctx,
		`SELECT COUNT(*) FROM pg_constraint WHERE contype = 'f'`).Scan(&fks); err != nil {
		return fmt.Errorf("统计外键：%w", err)
	}
	if err := s.pool.QueryRow(s.ctx,
		`SELECT COUNT(*) FROM pg_indexes WHERE schemaname = 'public' AND tablename = 'battle_records'`).
		Scan(&battleIndexes); err != nil {
		return fmt.Errorf("统计 battle_records 索引：%w", err)
	}
	if indexes < 100 {
		return fmt.Errorf("公共索引只剩 %d 个（基线 ≥100）—— 索引没有重建", indexes)
	}
	if fks < 45 {
		return fmt.Errorf("外键只剩 %d 个（基线 ≥45）—— 约束没有恢复", fks)
	}
	if battleIndexes < 2 {
		return fmt.Errorf("battle_records 只有 %d 个索引（基线 ≥2）", battleIndexes)
	}
	fmt.Printf("       索引=%d 外键=%d battle_records 索引=%d\n", indexes, fks, battleIndexes)
	return nil
}

// checkGoose 断言恢复库的迁移版本与仓库里的迁移文件完全对齐。
//
// 版本落后 = 恢复的是旧 dump（schema 缺列会让服务在第一条相关查询上 500）；
// 版本超前 = 恢复库跑过仓库里没有的迁移（回滚代码后行为不可预测）。
func (s *smoke) checkGoose() error {
	dir := strings.TrimSpace(os.Getenv("MIGRATIONS_DIR"))
	if dir == "" {
		dir = "migrations"
	}
	entries, err := os.ReadDir(dir)
	if err != nil {
		return fmt.Errorf("读迁移目录 %q：%w（可用 MIGRATIONS_DIR 指定）", dir, err)
	}
	var versions []int
	for _, e := range entries {
		name := e.Name()
		if e.IsDir() || !strings.HasSuffix(name, ".sql") || strings.Contains(name, ".down.") {
			continue
		}
		prefix := strings.SplitN(name, "_", 2)[0]
		v, err := strconv.Atoi(prefix)
		if err != nil {
			continue
		}
		versions = append(versions, v)
	}
	if len(versions) == 0 {
		return fmt.Errorf("迁移目录 %q 里没有找到 NNNNN_*.sql", dir)
	}
	sort.Ints(versions)
	wantMax := versions[len(versions)-1]

	var gotMax, applied int
	if err := s.pool.QueryRow(s.ctx,
		`SELECT COALESCE(MAX(version_id), 0) FROM goose_db_version WHERE is_applied`).Scan(&gotMax); err != nil {
		return fmt.Errorf("读 goose 最大版本：%w", err)
	}
	if err := s.pool.QueryRow(s.ctx,
		`SELECT COUNT(*) FROM goose_db_version WHERE version_id > 0 AND is_applied`).Scan(&applied); err != nil {
		return fmt.Errorf("读 goose 应用条数：%w", err)
	}
	if gotMax != wantMax {
		return fmt.Errorf("goose 最大版本=%d，仓库最新迁移=%d", gotMax, wantMax)
	}
	if applied != len(versions) {
		return fmt.Errorf("goose 已应用 %d 条，仓库迁移文件 %d 个", applied, len(versions))
	}
	fmt.Printf("       goose 版本=%d（已应用 %d 条，与仓库一致）\n", gotMax, applied)
	return nil
}

func (s *smoke) checkReadyz() error {
	var body struct {
		OK bool `json:"ok"`
	}
	if err := s.call(http.MethodGet, "/readyz", "", nil, &body); err != nil {
		return err
	}
	if !body.OK {
		return errors.New("/readyz 返回 ok=false")
	}
	return nil
}

// --- 管理端 ---

// adminLogin 在恢复库上建一个临时管理员并用它登录。
//
// 为什么不复用 dump 里的管理员：口令哈希不可逆，演练脚本拿不到原口令。
// 建临时行的意义正是要验证「恢复库的 admin_users / 角色 / 令牌 / 审计」
// 这条链路能真实工作；dump 里原有的管理员行数由演练脚本的行数比对覆盖。
func (s *smoke) adminLogin() error {
	pass := fmt.Sprintf("drill-%d", time.Now().UnixNano())
	hash, err := service.HashPassword(pass)
	if err != nil {
		return fmt.Errorf("生成口令哈希：%w", err)
	}
	if err := s.pool.QueryRow(s.ctx, `
		INSERT INTO admin_users (username, password_hash, role, status)
		VALUES ($1, $2, 'ops', 1) RETURNING id`, s.nickname, hash).Scan(&s.adminID); err != nil {
		return fmt.Errorf("建临时管理员：%w", err)
	}
	s.adminUser = s.nickname
	s.adminPass = pass
	s.onCleanup(func() {
		// 逆序清理；失败只告警（演练库随后会被 DROP）。
		for _, q := range []string{
			`DELETE FROM admin_tokens WHERE admin_id = $1`,
			`DELETE FROM admin_audit_logs WHERE admin_id = $1`,
			`DELETE FROM admin_users WHERE id = $1`,
		} {
			if _, err := s.pool.Exec(context.Background(), q, s.adminID); err != nil {
				fmt.Printf("[warn] 清理临时管理员 %d：%v\n", s.adminID, err)
			}
		}
	})

	var resp struct {
		AccessToken string `json:"access_token"`
		Admin       struct {
			ID       int64  `json:"id"`
			Username string `json:"username"`
			Role     string `json:"role"`
		} `json:"admin"`
	}
	if err := s.call(http.MethodPost, "/api/v1/admin/login", "", map[string]string{
		"username": s.adminUser,
		"password": s.adminPass,
	}, &resp); err != nil {
		return err
	}
	if resp.AccessToken == "" || resp.Admin.ID != s.adminID {
		return fmt.Errorf("登录响应不完整：token 空=%v admin_id=%d(期望 %d)", resp.AccessToken == "", resp.Admin.ID, s.adminID)
	}
	if resp.Admin.Role != "ops" {
		return fmt.Errorf("管理员角色=%q（期望 ops）", resp.Admin.Role)
	}
	s.adminToken = resp.AccessToken
	return nil
}

func (s *smoke) dashboardBaseline() error {
	var d struct {
		Users struct {
			Total int64 `json:"total"`
		} `json:"users"`
		Battles struct {
			Total int64 `json:"total"`
		} `json:"battles"`
		Economy struct {
			CoinIn int64 `json:"coin_in"`
		} `json:"economy"`
	}
	if err := s.call(http.MethodGet, "/api/v1/admin/dashboard", s.adminToken, nil, &d); err != nil {
		return err
	}
	// 此时刚建完游客，至少 1 个玩家。
	if d.Users.Total < 1 {
		return fmt.Errorf("看板 users.total=%d（刚建过游客，应 ≥1）", d.Users.Total)
	}
	s.preDashboardBattles = d.Battles.Total
	fmt.Printf("       看板基线：users=%d battles=%d coin_in=%d\n",
		d.Users.Total, d.Battles.Total, d.Economy.CoinIn)
	return nil
}

// --- 玩家路径 ---

func (s *smoke) guestLogin() error {
	var resp struct {
		AccessToken string `json:"access_token"`
		GuestToken  string `json:"guest_token"`
		User        struct {
			ID int64 `json:"id"`
		} `json:"user"`
	}
	if err := s.call(http.MethodPost, "/api/v1/auth/guest", "", map[string]string{
		"guest_token": s.nickname,
		"nickname":    s.nickname,
	}, &resp); err != nil {
		return err
	}
	if resp.AccessToken == "" {
		return errors.New("游客登录没有返回 access_token")
	}
	if resp.User.ID <= 0 {
		return fmt.Errorf("游客登录返回 user.id=%d", resp.User.ID)
	}
	s.userID = resp.User.ID
	s.userToken = resp.AccessToken
	s.onCleanup(func() {
		if _, err := s.pool.Exec(context.Background(), `DELETE FROM users WHERE id = $1`, s.userID); err != nil {
			fmt.Printf("[warn] 清理临时玩家 %d：%v\n", s.userID, err)
		}
	})
	fmt.Printf("       游客 user_id=%d\n", s.userID)
	return nil
}

func (s *smoke) playerConfig() error {
	var resp struct {
		Levels []struct {
			ID   int    `json:"id"`
			Name string `json:"name"`
		} `json:"levels"`
	}
	if err := s.call(http.MethodGet, "/api/v1/config", s.userToken, nil, &resp); err != nil {
		return err
	}
	if len(resp.Levels) != 100 {
		return fmt.Errorf("配置下发 %d 个关卡（期望 100）", len(resp.Levels))
	}
	for _, l := range resp.Levels {
		if l.ID < 1 || l.Name == "" {
			return fmt.Errorf("配置关卡项不完整：id=%d name=%q", l.ID, l.Name)
		}
	}
	return nil
}

func (s *smoke) playerLevel() error {
	var lvl struct {
		ID   int    `json:"id"`
		Name string `json:"name"`
	}
	if err := s.call(http.MethodGet, "/api/v1/levels/1", s.userToken, nil, &lvl); err != nil {
		return err
	}
	if lvl.ID != 1 || lvl.Name == "" {
		return fmt.Errorf("关卡详情不完整：id=%d name=%q", lvl.ID, lvl.Name)
	}
	return nil
}

func (s *smoke) walletBefore() error {
	var w struct {
		Coin int64 `json:"coin"`
		Gem  int64 `json:"gem"`
	}
	if err := s.call(http.MethodGet, "/api/v1/wallet/", s.userToken, nil, &w); err != nil {
		return err
	}
	s.preCoin = w.Coin
	fmt.Printf("       结算前钱包：coin=%d gem=%d\n", w.Coin, w.Gem)
	return nil
}

func (s *smoke) battleToken() error {
	var resp struct {
		TokenID int64  `json:"token_id"`
		Seed    string `json:"seed"`
		Level   struct {
			ID int `json:"id"`
		} `json:"level"`
	}
	if err := s.call(http.MethodPost, "/api/v1/battle/token", s.userToken,
		map[string]int{"level_id": 1}, &resp); err != nil {
		return err
	}
	if resp.TokenID <= 0 || resp.Seed == "" || resp.Level.ID != 1 {
		return fmt.Errorf("开局响应不完整：token_id=%d seed=%q level=%d", resp.TokenID, resp.Seed, resp.Level.ID)
	}
	s.battleTokenID = resp.TokenID
	return nil
}

func (s *smoke) settle() error {
	level := domain.GenerateLevel(1)
	// 每日击杀任务是 30，但关卡总怪数可能更少 —— 取两者较小值，
	// 保证结算一定合法；每日反应任务（20 次）单独保证至少有一个可领任务。
	kills := 30
	if max := domain.MaxKillsFor(level); kills > max {
		kills = max
	}
	const reactions = 20
	picks := []int{0, 1, -1}

	hash, err := s.replayHash(s.battleTokenID, picks)
	if err != nil {
		return err
	}

	var resp struct {
		BattleID int64          `json:"battle_id"`
		Result   string         `json:"result"`
		Loot     map[string]int `json:"loot"`
		Wallet   struct {
			Coin int64 `json:"coin"`
		} `json:"wallet"`
	}
	if err := s.call(http.MethodPost, "/api/v1/battle/settle", s.userToken, map[string]any{
		"token_id":       s.battleTokenID,
		"result":         "lose",
		"score":          1000,
		"duration_ms":    30000,
		"shots":          200,
		"hits":           100,
		"reactions":      reactions,
		"heat_max":       50,
		"hp_left":        500,
		"wave_reached":   1,
		"kills":          kills,
		"leaked":         0,
		"replay_hash":    hash,
		"elements_used":  map[string]int{"fire": 3},
		"reactions_used": map[string]int{"overheat": reactions},
		"terrain_used":   []string{},
		"card_picks":     picks,
	}, &resp); err != nil {
		return err
	}
	if resp.BattleID <= 0 {
		return fmt.Errorf("结算没有返回 battle_id：%+v", resp)
	}
	if resp.Result != "lose" {
		return fmt.Errorf("结算结果=%q（期望 lose）", resp.Result)
	}
	if resp.Loot["coin"] <= 0 {
		return fmt.Errorf("掉落金币=%d（应 >0）", resp.Loot["coin"])
	}
	if resp.Wallet.Coin <= s.preCoin {
		return fmt.Errorf("结算后金币=%d，结算前=%d —— 金币没有变化", resp.Wallet.Coin, s.preCoin)
	}
	s.battleID = resp.BattleID
	s.settleCoin = resp.Wallet.Coin
	s.settleLootCoin = resp.Loot["coin"]
	s.settleKills = kills
	s.settleReactions = reactions
	fmt.Printf("       结算：battle_id=%d 掉落 coin=%d 金币 %d → %d（kills=%d reactions=%d）\n",
		resp.BattleID, resp.Loot["coin"], s.preCoin, resp.Wallet.Coin, kills, reactions)
	return nil
}

// replayHash 按「关卡 + 种子 + 构筑 + 选牌」跑一遍引擎，拿真哈希。
// 直接复用 service 的构筑快照与 domain 的引擎 —— 与 e2e 用的是同一条路径。
func (s *smoke) replayHash(tokenID int64, picks []int) (string, error) {
	var (
		userID  int64
		seed    string
		levelID int
	)
	if err := s.pool.QueryRow(s.ctx,
		`SELECT user_id, seed, level_id FROM battle_tokens WHERE id = $1`, tokenID).
		Scan(&userID, &seed, &levelID); err != nil {
		return "", fmt.Errorf("读 battle_tokens：%w", err)
	}
	build, err := s.svc.LoadBuildSnapshot(s.ctx, userID)
	if err != nil {
		return "", fmt.Errorf("读构筑快照：%w", err)
	}
	raw, err := json.Marshal(build)
	if err != nil {
		return "", err
	}
	var bs domain.BuildSnapshot
	if err := json.Unmarshal(raw, &bs); err != nil {
		return "", fmt.Errorf("构筑快照解码：%w", err)
	}
	out := domain.Replay(
		domain.GenerateLevel(levelID), bs, seed, picks, "",
		domain.EnemiesByID(), domain.SkillsByID(), domain.DefaultScoreRules(),
	)
	if out.Err != nil {
		return "", fmt.Errorf("重放引擎：%w", out.Err)
	}
	if out.ComputedHash == "" {
		return "", errors.New("重放引擎返回空哈希")
	}
	return out.ComputedHash, nil
}

// statsBefore 在结算前拍下计数表基线（含「无行」的零值）。
func (s *smoke) statsBefore() error {
	err := s.pool.QueryRow(s.ctx, `
		SELECT attempts, wins, wave_sum FROM dashboard_battle_stats WHERE level_id = 1`).
		Scan(&s.preAttempts, &s.preWins, &s.preWaveSum)
	if errors.Is(err, pgx.ErrNoRows) {
		s.preAttempts, s.preWins, s.preWaveSum = 0, 0, 0
	} else if err != nil {
		return fmt.Errorf("读 dashboard_battle_stats：%w", err)
	}
	err = s.pool.QueryRow(s.ctx, `
		SELECT count FROM dashboard_reaction_counts WHERE level_id = 1 AND reaction = 'overheat'`).
		Scan(&s.preOverheat)
	if errors.Is(err, pgx.ErrNoRows) {
		s.preOverheat = 0
	} else if err != nil {
		return fmt.Errorf("读 dashboard_reaction_counts：%w", err)
	}
	fmt.Printf("       计数表基线：attempts=%d wins=%d wave_sum=%d overheat=%d\n",
		s.preAttempts, s.preWins, s.preWaveSum, s.preOverheat)
	return nil
}

// triggerEffect 断言结算这一条 INSERT 真的推动了两张计数表。
//
// 这是 R71 的核心：恢复后触发器若没生效，/readyz、结算、看板**全部正常**，
// 只有计数表停在旧值。这个断言让它在演练当天就变红。
func (s *smoke) triggerEffect() error {
	var attempts, wins, wave int64
	if err := s.pool.QueryRow(s.ctx, `
		SELECT attempts, wins, wave_sum FROM dashboard_battle_stats WHERE level_id = 1`).
		Scan(&attempts, &wins, &wave); err != nil {
		return fmt.Errorf("读 dashboard_battle_stats：%w", err)
	}
	wantAttempts := s.preAttempts + 1
	wantWave := s.preWaveSum + 1
	if attempts != wantAttempts {
		return fmt.Errorf("dashboard_battle_stats.attempts=%d，期望 %d（结算前 %d）—— 触发器没生效",
			attempts, wantAttempts, s.preAttempts)
	}
	if wins != s.preWins {
		return fmt.Errorf("失败局不应增加 wins：%d → %d", s.preWins, wins)
	}
	if wave != wantWave {
		return fmt.Errorf("dashboard_battle_stats.wave_sum=%d，期望 %d —— 触发器没生效", wave, wantWave)
	}

	var got int64
	if err := s.pool.QueryRow(s.ctx,
		`SELECT count FROM dashboard_reaction_counts WHERE level_id = 1 AND reaction = 'overheat'`).
		Scan(&got); err != nil {
		return fmt.Errorf("读 overheat 计数：%w", err)
	}
	wantReactions := s.preOverheat + int64(s.settleReactions)
	if got != wantReactions {
		return fmt.Errorf("reaction overheat=%d，期望 %d（结算前 %d）—— 反应触发器没生效",
			got, wantReactions, s.preOverheat)
	}
	fmt.Printf("       触发器生效：attempts %d→%d wave_sum %d→%d overheat→%d\n",
		s.preAttempts, attempts, s.preWaveSum, wave, got)
	return nil
}

func (s *smoke) battleDetail() error {
	var d struct {
		ID     int64          `json:"id"`
		Level  int            `json:"level_id"`
		Result string         `json:"result"`
		Kills  int            `json:"kills"`
		Loot   map[string]int `json:"loot"`
		Hash   string         `json:"replay_hash"`
	}
	if err := s.call(http.MethodGet, fmt.Sprintf("/api/v1/me/battles/%d", s.battleID),
		s.userToken, nil, &d); err != nil {
		return err
	}
	if d.ID != s.battleID || d.Level != 1 || d.Result != "lose" {
		return fmt.Errorf("战报详情不匹配：id=%d level=%d result=%q（期望 %d/1/lose）",
			d.ID, d.Level, d.Result, s.battleID)
	}
	if d.Kills != s.settleKills {
		return fmt.Errorf("战报 kills=%d，结算上报=%d", d.Kills, s.settleKills)
	}
	if d.Loot["coin"] != s.settleLootCoin {
		return fmt.Errorf("战报掉落 coin=%d，结算返回=%d", d.Loot["coin"], s.settleLootCoin)
	}
	if d.Hash == "" {
		return errors.New("战报 replay_hash 为空")
	}
	return nil
}

func (s *smoke) progress() error {
	var me struct {
		UserID   int64 `json:"user_id"`
		MaxStage int   `json:"max_stage"`
		Power    int64 `json:"power"`
	}
	if err := s.call(http.MethodGet, "/api/v1/me/", s.userToken, nil, &me); err != nil {
		return err
	}
	if me.UserID != s.userID {
		return fmt.Errorf("/me user_id=%d，期望 %d", me.UserID, s.userID)
	}
	if me.MaxStage < 0 {
		return fmt.Errorf("/me max_stage=%d（不能为负）", me.MaxStage)
	}
	return nil
}

func (s *smoke) taskClaim() error {
	var list struct {
		Items []struct {
			ID      int            `json:"id"`
			Code    string         `json:"code"`
			Done    bool           `json:"done"`
			Claimed bool           `json:"claimed"`
			Reward  map[string]int `json:"reward"`
		} `json:"items"`
	}
	if err := s.call(http.MethodGet, "/api/v1/tasks/?scope=daily", s.userToken, nil, &list); err != nil {
		return err
	}
	var target int
	for _, it := range list.Items {
		if it.Done && !it.Claimed {
			target = it.ID
			break
		}
	}
	if target == 0 {
		return fmt.Errorf("结算后没有可领取的每日任务（kills=%d reactions=%d）：%+v",
			s.settleKills, s.settleReactions, list.Items)
	}

	var resp struct {
		Reward map[string]int `json:"reward"`
	}
	if err := s.call(http.MethodPost, fmt.Sprintf("/api/v1/tasks/%d/claim", target),
		s.userToken, nil, &resp); err != nil {
		return err
	}
	if len(resp.Reward) == 0 {
		return fmt.Errorf("领取任务 %d 的 reward 为空", target)
	}
	s.claimCoin = int64(resp.Reward["coin"])
	fmt.Printf("       领取任务 id=%d：reward=%v\n", target, resp.Reward)
	return nil
}

// walletLedger 断言两件事：
//  1. 钱包余额与「结算 + 任务奖励」对得上（读侧与写侧一致）；
//  2. 钱包流水表里真有 battle_reward 这一笔（恢复后流水表/索引可用）。
func (s *smoke) walletLedger() error {
	var w struct {
		Coin int64 `json:"coin"`
		Gem  int64 `json:"gem"`
	}
	if err := s.call(http.MethodGet, "/api/v1/wallet/", s.userToken, nil, &w); err != nil {
		return err
	}
	want := s.settleCoin + s.claimCoin
	if w.Coin != want {
		return fmt.Errorf("钱包 coin=%d，结算后=%d + 任务奖励=%d（期望 %d）",
			w.Coin, s.settleCoin, s.claimCoin, want)
	}

	rows, err := s.pool.Query(s.ctx, `
		SELECT reason, delta FROM wallet_flows
		WHERE user_id = $1 AND currency = 'coin' AND reason = 'battle_reward'
		ORDER BY id DESC LIMIT 1`, s.userID)
	if err != nil {
		return fmt.Errorf("读 wallet_flows：%w", err)
	}
	defer rows.Close()
	if !rows.Next() {
		return errors.New("wallet_flows 里没有 battle_reward 流水")
	}
	var reason string
	var delta int64
	if err := rows.Scan(&reason, &delta); err != nil {
		return err
	}
	if delta != int64(s.settleLootCoin) {
		return fmt.Errorf("battle_reward 流水 delta=%d，结算掉落=%d", delta, s.settleLootCoin)
	}
	fmt.Printf("       钱包 coin=%d（对账一致），流水 battle_reward delta=%d\n", w.Coin, delta)
	return nil
}

// --- 管理端（结算后） ---

func (s *smoke) dashboardAfterSettle() error {
	var d struct {
		Battles struct {
			Total int64 `json:"total"`
		} `json:"battles"`
		StageFunnel []struct {
			LevelID  int   `json:"level_id"`
			Attempts int64 `json:"attempts"`
		} `json:"stage_funnel"`
	}
	if err := s.call(http.MethodGet, "/api/v1/admin/dashboard", s.adminToken, nil, &d); err != nil {
		return err
	}
	if got, want := d.Battles.Total, s.preDashboardBattles+1; got != want {
		return fmt.Errorf("看板 battles.total=%d，结算前=%d（期望 +1=%d）；"+
			"若差别只是缓存，请给 API 设 DASHBOARD_CACHE_TTL=0 后重跑",
			got, s.preDashboardBattles, want)
	}
	return nil
}

func (s *smoke) adminUsers() error {
	var resp struct {
		Items []struct {
			ID       int64  `json:"id"`
			Nickname string `json:"nickname"`
		} `json:"items"`
		Total int64 `json:"total"`
	}
	path := "/api/v1/admin/users?limit=50&keyword=" + urlQueryEscape(s.nickname)
	if err := s.call(http.MethodGet, path, s.adminToken, nil, &resp); err != nil {
		return err
	}
	for _, it := range resp.Items {
		if it.ID == s.userID && it.Nickname == s.nickname {
			return nil
		}
	}
	return fmt.Errorf("玩家列表里找不到刚建的游客 user_id=%d nickname=%q（total=%d items=%v）",
		s.userID, s.nickname, resp.Total, resp.Items)
}

// announcementVisible 是「管理端改一条配置，玩家侧能看到」的最小闭环：
// 建公告（写恢复库 + 审计）→ 玩家无鉴权读公告列表 → 看到刚发的那条。
func (s *smoke) announcementVisible() error {
	title := "drill-restore-" + strconv.FormatInt(time.Now().UnixNano(), 10)
	var created struct {
		Item struct {
			ID    int    `json:"id"`
			Title string `json:"title"`
		} `json:"item"`
	}
	if err := s.call(http.MethodPost, "/api/v1/admin/announcements", s.adminToken, map[string]any{
		"title":     title,
		"body":      "恢复演练临时公告（smoke 结束后会删除）",
		"published": true,
	}, &created); err != nil {
		return err
	}
	if created.Item.ID <= 0 {
		return fmt.Errorf("建公告没返回 id：%+v", created)
	}
	s.announcementID = created.Item.ID
	s.onCleanup(func() {
		if _, err := s.pool.Exec(context.Background(),
			`DELETE FROM announcements WHERE id = $1`, s.announcementID); err != nil {
			fmt.Printf("[warn] 清理临时公告 %d：%v\n", s.announcementID, err)
		}
	})

	// 公告读侧有缓存（默认几秒），轮询到可见或超时。
	deadline := time.Now().Add(5 * time.Second)
	for {
		var list struct {
			Items []struct {
				ID    int    `json:"id"`
				Title string `json:"title"`
			} `json:"items"`
		}
		if err := s.call(http.MethodGet, "/api/v1/announcements?limit=20", "", nil, &list); err != nil {
			return err
		}
		for _, it := range list.Items {
			if it.ID == created.Item.ID && it.Title == title {
				fmt.Printf("       公告 id=%d 已被玩家侧读到\n", created.Item.ID)
				return nil
			}
		}
		if time.Now().After(deadline) {
			return fmt.Errorf("玩家侧 5s 内没读到刚发的公告 id=%d title=%q（items=%v）",
				created.Item.ID, title, list.Items)
		}
		time.Sleep(250 * time.Millisecond)
	}
}

// adminAudit 断言管理端写操作真的落了审计。
//
// 恢复库最容易坏的是「审计表还在、但写入链路的权限/触发器/默认值没回来」，
// 表现同样是静默：运营改了配置，后台一切正常，审计日志里却没有记录。
func (s *smoke) adminAudit() error {
	var n int64
	if err := s.pool.QueryRow(s.ctx, `
		SELECT COUNT(*) FROM admin_audit_logs
		WHERE admin_id = $1 AND action = 'create_announcement'`, s.adminID).Scan(&n); err != nil {
		return fmt.Errorf("读 admin_audit_logs：%w", err)
	}
	if n < 1 {
		return fmt.Errorf("临时管理员 %d 没有 create_announcement 审计记录", s.adminID)
	}
	return nil
}

// adminReconcile 收尾对账（R79）：把 R78 的九项不变量在恢复库上核一遍。
//
// 它能抓到 R58 行数比对永远看不到的漂移：比如触发器虽然存在但计数表是
// 旧快照、或恢复丢了一部分 wallet_flows —— 那些场景下行数完全一致、
// 服务完全正常，只有「明细与计数对不上」会暴露。
//
// 失败文案刻意区分两类故障：玩法步骤失败报步骤名；对账漂移报
// 「对账发现漂移：check/key/期望/实际」，一眼可辨。
func (s *smoke) adminReconcile() error {
	var rep struct {
		OK     bool `json:"ok"`
		Checks int  `json:"checks"`
		Diffs  []struct {
			Check    string `json:"check"`
			Key      string `json:"key"`
			Expected string `json:"expected"`
			Actual   string `json:"actual"`
		} `json:"diffs"`
		ViolationCounts map[string]int64 `json:"violation_counts"`
	}
	if err := s.call(http.MethodGet, "/api/v1/admin/reconcile", s.adminToken, nil, &rep); err != nil {
		return fmt.Errorf("调 /admin/reconcile：%w", err)
	}
	if rep.Checks != 9 {
		return fmt.Errorf("对账只跑了 %d 项（期望 9）—— 服务版本过旧或对账被裁减", rep.Checks)
	}
	if rep.OK {
		fmt.Printf("       对账 %d 项全绿\n", rep.Checks)
		return nil
	}
	var b strings.Builder
	kinds := 0
	for _, n := range rep.ViolationCounts {
		if n > 0 {
			kinds++
		}
	}
	fmt.Fprintf(&b, "对账发现漂移（%d 类）：", kinds)
	shown := 0
	for _, d := range rep.Diffs {
		if shown >= 5 {
			b.WriteString(" …")
			break
		}
		fmt.Fprintf(&b, " [%s %s 期望=%s 实际=%s]", d.Check, d.Key, d.Expected, d.Actual)
		shown++
	}
	return errors.New(b.String())
}

// --- 小工具 ---

func isDrillDatabase(dsn string) bool {
	return strings.Contains(strings.ToLower(databaseName(dsn)), "drill") ||
		strings.Contains(strings.ToLower(databaseName(dsn)), "smoke")
}

func databaseName(dsn string) string {
	// 只需要应付 postgres://.../db?x 与 postgres://.../db 两种形态。
	// 解析失败就退化为空串（安全阀会因此拒绝执行）。
	slash := strings.LastIndex(dsn, "/")
	if slash < 0 || slash == len(dsn)-1 {
		return ""
	}
	name := dsn[slash+1:]
	if q := strings.IndexByte(name, '?'); q >= 0 {
		name = name[:q]
	}
	return name
}

func quoteIdent(s string) string {
	return `"` + strings.ReplaceAll(s, `"`, `""`) + `"`
}

func keys(m map[string]string) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}

func urlQueryEscape(s string) string {
	// keyword 只含字母/数字/下划线（nickname 由本程序生成），
	// 但仍按标准编码，避免以后改生成规则时踩到。
	var b strings.Builder
	for _, r := range s {
		if r == ' ' {
			b.WriteByte('+')
			continue
		}
		if (r >= 'a' && r <= 'z') || (r >= 'A' && r <= 'Z') || (r >= '0' && r <= '9') ||
			r == '-' || r == '_' || r == '.' || r == '~' {
			b.WriteRune(r)
			continue
		}
		b.WriteString(fmt.Sprintf("%%%02X", r))
	}
	return b.String()
}
