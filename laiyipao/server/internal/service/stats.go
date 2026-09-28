package service

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

// --- 看板 ---

// Dashboard 是后台数据看板。
type Dashboard struct {
	Users struct {
		Total    int64 `json:"total"`
		Guests   int64 `json:"guests"`
		Wechat   int64 `json:"wechat"`
		Banned   int64 `json:"banned"`
		NewToday int64 `json:"new_today"`
	} `json:"users"`
	Battles struct {
		Total         int64   `json:"total"`
		Today         int64   `json:"today"`
		Wins          int64   `json:"wins"`
		AvgDurationMs float64 `json:"avg_duration_ms"`
	} `json:"battles"`
	Progression struct {
		AvgMaxStage   float64 `json:"avg_max_stage"`
		AvgPower      float64 `json:"avg_power"`
		Stage100Clear int64   `json:"stage_100_clears"`
	} `json:"progression"`
	Economy struct {
		CoinIn        int64 `json:"coin_in"`
		CoinOut       int64 `json:"coin_out"`
		GemIn         int64 `json:"gem_in"`
		GemOut        int64 `json:"gem_out"`
		OrdersPaid    int64 `json:"orders_paid"`
		OrdersPending int64 `json:"orders_pending"`
	} `json:"economy"`
	// I-6：验真健康度
	Verification struct {
		Checked    int64 `json:"checked"`
		Matched    int64 `json:"matched"`
		Mismatched int64 `json:"mismatched"`
	} `json:"verification"`
	// I-1：反应使用分布。分布长期集中在一两种反应 = 元素设计没生效
	ReactionUsage []ReactionCount `json:"reaction_usage"`
	DailyActive   []DauPoint      `json:"daily_active"`
	StageFunnel   []StageFunnel   `json:"stage_funnel"`
}

// ReactionCount 是反应使用计数。
type ReactionCount struct {
	Reaction string `json:"reaction"`
	Count    int64  `json:"count"`
}

// DauPoint 是某日活跃数。
type DauPoint struct {
	Date string `json:"date"`
	DAU  int64  `json:"dau"`
}

// StageFunnel 是关卡漏斗。
type StageFunnel struct {
	LevelID  int   `json:"level_id"`
	Attempts int64 `json:"attempts"`
	Number   int64 `json:"clears"`
}

// AdminDashboard 汇总看板数据。
func (s *Service) AdminDashboard(ctx context.Context) (Dashboard, error) {
	var d Dashboard

	scan := func(q string, dest ...any) error {
		return s.pool.QueryRow(ctx, q).Scan(dest...)
	}

	if err := scan(`
		SELECT COUNT(*),
		       COUNT(*) FILTER (WHERE is_guest),
		       COUNT(*) FILTER (WHERE NOT is_guest),
		       COUNT(*) FILTER (WHERE status = 2),
		       COUNT(*) FILTER (WHERE created_at >= CURRENT_DATE)
		FROM users`,
		&d.Users.Total, &d.Users.Guests, &d.Users.Wechat, &d.Users.Banned, &d.Users.NewToday); err != nil {
		return d, fmt.Errorf("dashboard users: %w", err)
	}

	if err := scan(`
		SELECT COUNT(*),
		       COUNT(*) FILTER (WHERE created_at >= CURRENT_DATE),
		       COUNT(*) FILTER (WHERE result = 'win'),
		       COALESCE(AVG(duration_ms), 0)
		FROM battle_records`,
		&d.Battles.Total, &d.Battles.Today, &d.Battles.Wins, &d.Battles.AvgDurationMs); err != nil {
		return d, fmt.Errorf("dashboard battles: %w", err)
	}

	if err := scan(`
		SELECT COALESCE(AVG(max_stage), 0), COALESCE(AVG(level_exp), 0),
		       (SELECT COUNT(*) FROM level_stars WHERE level_id = 100 AND clears > 0)
		FROM user_progress`,
		&d.Progression.AvgMaxStage, &d.Progression.AvgPower, &d.Progression.Stage100Clear); err != nil {
		return d, fmt.Errorf("dashboard progression: %w", err)
	}

	// 货币流水按符号分流入/流出
	if err := scan(`
		SELECT
		  COALESCE(SUM(delta) FILTER (WHERE currency='coin' AND delta > 0), 0),
		  COALESCE(-SUM(delta) FILTER (WHERE currency='coin' AND delta < 0), 0),
		  COALESCE(SUM(delta) FILTER (WHERE currency='gem' AND delta > 0), 0),
		  COALESCE(-SUM(delta) FILTER (WHERE currency='gem' AND delta < 0), 0)
		FROM wallet_flows`,
		&d.Economy.CoinIn, &d.Economy.CoinOut, &d.Economy.GemIn, &d.Economy.GemOut); err != nil {
		return d, fmt.Errorf("dashboard economy: %w", err)
	}
	if err := scan(`
		SELECT COUNT(*) FILTER (WHERE status IN ('paid','delivered')),
		       COUNT(*) FILTER (WHERE status = 'pending')
		FROM orders`,
		&d.Economy.OrdersPaid, &d.Economy.OrdersPending); err != nil {
		return d, fmt.Errorf("dashboard orders: %w", err)
	}

	if err := scan(`
		SELECT COUNT(*), COUNT(*) FILTER (WHERE matched), COUNT(*) FILTER (WHERE NOT matched)
		FROM replay_verifications`,
		&d.Verification.Checked, &d.Verification.Matched, &d.Verification.Mismatched); err != nil {
		return d, fmt.Errorf("dashboard verification: %w", err)
	}

	// 反应使用分布
	// 反应使用分布
	//
	// ⚠️ 两处改动，都因为「读侧不该依赖写侧」。
	//
	// 1) 过滤条件用 `jsonb_typeof(...) = 'object'`，不用 `<> '{}'`。
	//    `<> '{}'` 对 jsonb `null` 求值为 **NULL 而不是 true**，
	//    于是那些行被**静默排除**。实测 58 行里有 8 行（13.8%）带 jsonb null。
	//
	// 2) 改成在 SQL 里按 key 聚合，不用 `GROUP BY reactions_used`。
	//    原来的写法按**整个 map** 分组再在 Go 里逐键累加，
	//    代价是：对 JSONB 列做哈希分组（无索引可用）+ 最多 500 组被截断。
	//    实测当前只有 2 种组合所以看不出问题，但组合数会随战报量爆炸 ——
	//    6 种反应各 0~5 次就是 6^6 种组合，`LIMIT 500` 会开始丢数据。
	//
	//    顺带修掉一个更硬的问题：`jsonb_each_text(jsonb 'null')` 会**报错**
	//    （「不能在非对象上调用」），所以旧写法连改个过滤条件都改不动。
	rows, err := s.pool.Query(ctx, `
		SELECT e.key, SUM(e.value::bigint) AS total
		FROM battle_records b,
		     LATERAL jsonb_each_text(b.reactions_used) AS e(key, value)
		WHERE jsonb_typeof(b.reactions_used) = 'object'
		GROUP BY e.key
		ORDER BY total DESC
		LIMIT 100`)
	if err != nil {
		return d, fmt.Errorf("dashboard reactions: %w", err)
	}
	defer rows.Close()
	// SQL 已经按 key 聚合好了，这里直接读两列。
	//
	// ⚠️ 旧实现在 Go 里 `json.Unmarshal` 整个 map 再逐键累加，
	// 而错误被 `_ =` **静默吞掉** —— 解析失败时那一行就凭空消失，
	// 没有任何日志。一个统计数字悄悄少掉一部分，比直接报错更难发现。
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

	// 近 14 日 DAU
	drows, err := s.pool.Query(ctx, `
		SELECT d::date AS day,
		       COUNT(DISTINCT user_id) AS dau
		FROM generate_series(CURRENT_DATE - INTERVAL '13 days', CURRENT_DATE, '1 day') d
		LEFT JOIN battle_records br ON br.created_at >= d AND br.created_at < d + INTERVAL '1 day'
		GROUP BY d::date ORDER BY 1`)
	if err != nil {
		return d, fmt.Errorf("dashboard dau: %w", err)
	}
	defer drows.Close()
	for drows.Next() {
		var day time.Time
		var dau int64
		if err := drows.Scan(&day, &dau); err != nil {
			return d, err
		}
		d.DailyActive = append(d.DailyActive, DauPoint{
			Date: day.Format("2006-01-02"), DAU: dau,
		})
	}
	if err := drows.Err(); err != nil {
		return d, err
	}

	// 关卡漏斗（全部 100 关）
	frows, err := s.pool.Query(ctx, `
		SELECT level_id, COUNT(*), COUNT(*) FILTER (WHERE result='win')
		FROM battle_records GROUP BY level_id ORDER BY level_id`)
	if err != nil {
		return d, fmt.Errorf("dashboard funnel: %w", err)
	}
	defer frows.Close()
	for frows.Next() {
		var f StageFunnel
		if err := frows.Scan(&f.LevelID, &f.Attempts, &f.Number); err != nil {
			return d, err
		}
		d.StageFunnel = append(d.StageFunnel, f)
	}
	return d, rows.Err()
}

// --- 玩家管理 ---

// AdminUserView 是后台玩家列表条目。
type AdminUserView struct {
	ID          int64     `json:"id"`
	Nickname    string    `json:"nickname"`
	IsGuest     bool      `json:"is_guest"`
	Status      int       `json:"status"`
	MaxStage    int       `json:"max_stage"`
	Power       int64     `json:"power"`
	Coin        int64     `json:"coin"`
	Gem         int64     `json:"gem"`
	CreatedAt   time.Time `json:"created_at"`
	LastLoginAt time.Time `json:"last_login_at"`

	// 验真统计：把已有但**不可执行**的信号接到执法链上。
	//
	// 背景（实测查证）：`replay_verifications` 表全项目**只被一个查询读过** ——
	// `stats.go` 里的那个 COUNT。也就是说一次不匹配会被**记录、被统计**，
	// 但没有任何后果：不撤销奖励、不标记用户，运营也查不出是哪几场。
	// 看板上的「验真一致率」因此是个**不可执行的数字**：
	// 运营看到「10/1000 不匹配」，却不知道该封谁。
	//
	// ⚠️ **两个字段而不是一个**，因为这两件事必须可区分：
	//
	//   VerifyChecked = 0, VerifyMismatched = 0  →  从没被验真过 = **未知**
	//   VerifyChecked = 8, VerifyMismatched = 0  →  验过且都一致 = 干净
	//   VerifyChecked = 8, VerifyMismatched = 3  →  **可疑**
	//
	// 合成一个 `mismatches` 字段的话，「没验过」和「验过且一致」都会显示 0，
	// 于是**最不可信的那批用户看起来最干净** ——
	// 那正是本项目反复修的「静默降级而不是失败」。
	VerifyChecked    int64 `json:"verify_checked"`
	VerifyMismatched int64 `json:"verify_mismatched"`
}

// AdminListUsers 分页查询玩家。
func (s *Service) AdminListUsers(ctx context.Context, keyword string, limit, offset int) ([]AdminUserView, int64, error) {
	if limit <= 0 || limit > 200 {
		limit = 50
	}
	if offset < 0 {
		offset = 0
	}
	pattern := "%" + strings.TrimSpace(keyword) + "%"

	var total int64
	if err := s.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM users WHERE $1 = '%%' OR nickname LIKE $2 OR guest_token LIKE $2`,
		pattern, pattern).Scan(&total); err != nil {
		return nil, 0, fmt.Errorf("count users: %w", err)
	}

	// 验真统计走**子查询预聚合**再 LEFT JOIN，而不是直接 JOIN 明细表。
	//
	// 原因：一场战斗可以被验真多次（不同验真人、或同一人重验），
	// 直接 JOIN 会让 users 一行变成 N 行 —— 而这个接口是**分页**的，
	// 行数被放大后 `LIMIT/OFFSET` 就切在错误的粒度上（分页结果会重复/漏掉用户）。
	//
	// 子查询里 `GROUP BY br.user_id` 保证每个用户最多贡献一行。
	//
	// 关联路径是 `replay_verifications → battle_records → users`：
	// 注意 `replay_verifications.verifier_id` 是**验真人**，
	// 不是这场战斗的**主人**。按 verifier 统计会得到完全错误的数字
	// （谁验得多谁就「可疑」）。
	rows, err := s.pool.Query(ctx, `
		SELECT u.id, u.nickname, u.is_guest, u.status,
		       COALESCE(p.max_stage, 0), COALESCE(p.level_exp, 0),
		       COALESCE(w.coin, 0), COALESCE(w.gem, 0), u.created_at, u.last_login_at,
		       COALESCE(v.checked, 0), COALESCE(v.mismatched, 0)
		FROM users u
		LEFT JOIN user_progress p ON p.user_id = u.id
		LEFT JOIN user_wallets w ON w.user_id = u.id
		LEFT JOIN (
			SELECT br.user_id,
			       COUNT(*) AS checked,
			       COUNT(*) FILTER (WHERE NOT rv.matched) AS mismatched
			FROM replay_verifications rv
			JOIN battle_records br ON br.id = rv.battle_id
			GROUP BY br.user_id
		) v ON v.user_id = u.id
		WHERE $1 = '%%' OR u.nickname LIKE $1 OR u.guest_token LIKE $1
		ORDER BY u.id DESC LIMIT $2 OFFSET $3`, pattern, limit, offset)
	if err != nil {
		return nil, 0, fmt.Errorf("list users: %w", err)
	}
	defer rows.Close()

	var out []AdminUserView
	for rows.Next() {
		var v AdminUserView
		if err := rows.Scan(&v.ID, &v.Nickname, &v.IsGuest, &v.Status, &v.MaxStage,
			&v.Power, &v.Coin, &v.Gem, &v.CreatedAt, &v.LastLoginAt,
			&v.VerifyChecked, &v.VerifyMismatched); err != nil {
			return nil, 0, err
		}
		out = append(out, v)
	}
	return out, total, rows.Err()
}

// AdminSetUserStatus 封禁 / 解封玩家。
func (s *Service) AdminSetUserStatus(ctx context.Context, userID int64, banned bool, reason string) (AdminUserView, error) {
	status := 1
	if banned {
		status = 2
	}
	if _, err := s.pool.Exec(ctx,
		`UPDATE users SET status = $2, ban_reason = $3 WHERE id = $1`, userID, status, reason); err != nil {
		return AdminUserView{}, fmt.Errorf("set user status: %w", err)
	}
	// 封禁即吊销所有 refresh token，否则封禁后仍可用长期令牌续命
	if banned {
		if _, err := s.pool.Exec(ctx,
			`UPDATE refresh_tokens SET revoked_at = now() WHERE user_id = $1 AND revoked_at IS NULL`,
			userID); err != nil {
			return AdminUserView{}, fmt.Errorf("revoke tokens: %w", err)
		}
	}
	var v AdminUserView
	err := s.pool.QueryRow(ctx, `
		SELECT u.id, u.nickname, u.is_guest, u.status,
		       COALESCE(p.max_stage, 0), COALESCE(p.level_exp, 0),
		       COALESCE(w.coin, 0), COALESCE(w.gem, 0), u.created_at, u.last_login_at
		FROM users u
		LEFT JOIN user_progress p ON p.user_id = u.id
		LEFT JOIN user_wallets w ON w.user_id = u.id
		WHERE u.id = $1`, userID).
		Scan(&v.ID, &v.Nickname, &v.IsGuest, &v.Status, &v.MaxStage,
			&v.Power, &v.Coin, &v.Gem, &v.CreatedAt, &v.LastLoginAt)
	return v, err
}

// AdminGrantCurrency 后台发币。
func (s *Service) AdminGrantCurrency(ctx context.Context, userID int64, currency string, amount int64) (map[string]int64, error) {
	out := map[string]int64{}
	err := s.DB.Tx(ctx, func(tx txType) error {
		if err := s.grantWallet(ctx, tx, userID,
			map[string]int64{currency: amount}, "admin_grant", 0); err != nil {
			return err
		}
		w, err := s.loadWalletTx(ctx, tx, userID)
		if err != nil {
			return err
		}
		out["coin"], out["gem"], out["energy"], out["keys"] = w.Coin, w.Gem, w.Energy, w.Keys
		return nil
	})
	return out, err
}

func (s *Service) loadWalletTx(ctx context.Context, tx txType, userID int64) (Wallet, error) {
	var w Wallet
	err := tx.QueryRow(ctx,
		`SELECT coin, gem, energy, keys, energy_updated_at FROM user_wallets WHERE user_id = $1`, userID).
		Scan(&w.Coin, &w.Gem, &w.Energy, &w.Keys, &w.EnergyUpdatedAt)
	return w, err
}

// --- 战报 ---

// AdminBattleView 是后台战报条目。
type AdminBattleView struct {
	ID          int64     `json:"id"`
	UserID      int64     `json:"user_id"`
	Nickname    string    `json:"nickname"`
	LevelID     int       `json:"level_id"`
	Result      string    `json:"result"`
	Stars       int       `json:"stars"`
	Score       int64     `json:"score"`
	Kills       int       `json:"kills"`
	Leaked      int       `json:"leaked"`
	WaveReached int       `json:"wave_reached"`
	DurationMs  int       `json:"duration_ms"`
	Reactions   int       `json:"reactions"`
	HeatMax     int       `json:"heat_max"`
	ReplayHash  string    `json:"replay_hash"`
	CreatedAt   time.Time `json:"created_at"`
}

// AdminListBattles 分页查询战报。
func (s *Service) AdminListBattles(ctx context.Context, userID int64, levelID int, limit int) ([]AdminBattleView, int64, error) {
	if limit <= 0 || limit > 200 {
		limit = 50
	}
	var total int64
	if err := s.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM battle_records
		 WHERE ($1 = 0 OR user_id = $1) AND ($2 = 0 OR level_id = $2)`,
		userID, levelID).Scan(&total); err != nil {
		return nil, 0, fmt.Errorf("count battles: %w", err)
	}
	rows, err := s.pool.Query(ctx, `
		SELECT br.id, br.user_id, COALESCE(u.nickname,''), br.level_id, br.result, br.stars, br.score,
		       br.kills, br.leaked, br.wave_reached, br.duration_ms, br.reactions, br.heat_max,
		       br.replay_hash, br.created_at
		FROM battle_records br LEFT JOIN users u ON u.id = br.user_id
		WHERE ($1 = 0 OR br.user_id = $1) AND ($2 = 0 OR br.level_id = $2)
		ORDER BY br.id DESC LIMIT $3`, userID, levelID, limit)
	if err != nil {
		return nil, 0, fmt.Errorf("list battles: %w", err)
	}
	defer rows.Close()
	var out []AdminBattleView
	for rows.Next() {
		var v AdminBattleView
		if err := rows.Scan(&v.ID, &v.UserID, &v.Nickname, &v.LevelID, &v.Result, &v.Stars,
			&v.Score, &v.Kills, &v.Leaked, &v.WaveReached, &v.DurationMs, &v.Reactions,
			&v.HeatMax, &v.ReplayHash, &v.CreatedAt); err != nil {
			return nil, 0, err
		}
		out = append(out, v)
	}
	return out, total, rows.Err()
}

// --- 内容配置 ---

// AdminUpdateLevel 更新关卡配置。
//
// 只有白名单字段可写 —— 客户端上报的任意字段绝不能直接进库。
func (s *Service) AdminUpdateLevel(ctx context.Context, levelID int, patch map[string]any) (domain.GeneratedLevel, error) {
	fields := []string{}
	args := []any{levelID}
	set := func(col string, v any) {
		args = append(args, v)
		fields = append(fields, fmt.Sprintf("%s = $%d", col, len(args)))
	}
	for key, val := range patch {
		switch key {
		case "name":
			if v, ok := val.(string); ok && v != "" {
				set("name", v)
			}
		case "base_hp":
			if v, ok := toInt64(val); ok && v > 0 {
				set("base_hp", v)
			}
		case "wave_count":
			if v, ok := toInt64(val); ok && v > 0 && v <= 50 {
				set("wave_count", v)
			}
		case "difficulty":
			if v, ok := toInt64(val); ok && v > 0 {
				set("difficulty", v)
			}
		case "energy_cost":
			if v, ok := toInt64(val); ok && v >= 0 && v <= 100 {
				set("energy_cost", v)
			}
		case "star_targets":
			raw, err := json.Marshal(val)
			if err == nil {
				set("star_targets", raw)
			}
		case "terrain_config":
			raw, err := json.Marshal(val)
			if err == nil {
				set("terrain_config", raw)
			}
		case "enabled":
			if v, ok := val.(bool); ok {
				set("enabled", v)
			}
		case "is_boss":
			if v, ok := val.(bool); ok {
				set("is_boss", v)
			}
		}
	}
	if len(fields) == 0 {
		return domain.GeneratedLevel{}, fmt.Errorf("%w: 没有可更新的字段", ErrBadInput)
	}
	sql := fmt.Sprintf(`UPDATE levels SET %s, updated_at = now() WHERE id = $1`,
		joinComma(fields))
	if _, err := s.pool.Exec(ctx, sql, args...); err != nil {
		return domain.GeneratedLevel{}, fmt.Errorf("update level: %w", err)
	}
	return domain.GenerateLevel(levelID), nil
}

// AdminUpdateSkill 更新技能配置。
func (s *Service) AdminUpdateSkill(ctx context.Context, skillID int, patch map[string]any) error {
	fields := []string{}
	args := []any{skillID}
	set := func(col string, v any) {
		args = append(args, v)
		fields = append(fields, fmt.Sprintf("%s = $%d", col, len(args)))
	}
	for key, val := range patch {
		switch key {
		case "name", "descr":
			if v, ok := val.(string); ok {
				set(key, v)
			}
		case "base_damage", "heat_cost", "cooldown_ms", "pierce", "aoe_radius",
			"apply_stacks", "projectile_speed", "chain":
			if v, ok := toInt64(val); ok && v >= 0 {
				set(key, v)
			}
		case "element", "family", "kind":
			if v, ok := val.(string); ok && v != "" {
				set(key, v)
			}
		}
	}
	if len(fields) == 0 {
		return fmt.Errorf("%w: 没有可更新的字段", ErrBadInput)
	}
	sql := fmt.Sprintf(`UPDATE skills SET %s WHERE id = $1`, joinComma(fields))
	if _, err := s.pool.Exec(ctx, sql, args...); err != nil {
		return fmt.Errorf("update skill: %w", err)
	}
	return nil
}

// AdminRegenerateLevels 重新生成全部关卡（覆盖后台的手工改动）。
func (s *Service) AdminRegenerateLevels(ctx context.Context) (int, error) {
	levels := domain.GenerateAllLevels()
	for _, gl := range levels {
		starRaw, _ := json.Marshal(gl.StarTargets)
		terrainRaw, _ := json.Marshal(gl.Terrain)
		if _, err := s.pool.Exec(ctx, `
			UPDATE levels SET chapter=$2, name=$3, seed=$4, base_hp=$5, wave_count=$6,
			                 difficulty=$7, energy_cost=$8, star_targets=$9, terrain_config=$10,
			                 is_boss=$11, updated_at=now()
			 WHERE id=$1`,
			gl.ID, gl.Chapter, gl.Name, gl.Seed, gl.BaseHP, gl.WaveCount, gl.Difficulty,
			gl.EnergyCost, starRaw, terrainRaw, gl.IsBoss); err != nil {
			return 0, fmt.Errorf("regenerate level %d: %w", gl.ID, err)
		}
	}
	return len(levels), nil
}

func joinComma(s []string) string {
	out := ""
	for i, v := range s {
		if i > 0 {
			out += ", "
		}
		out += v
	}
	return out
}

func toInt64(v any) (int64, bool) {
	switch n := v.(type) {
	case float64:
		return int64(n), true
	case int:
		return int64(n), true
	case int64:
		return n, true
	case json.Number:
		i, err := n.Int64()
		return i, err == nil
	}
	return 0, false
}
