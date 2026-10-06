package service

import (
	"context"
	"encoding/json"
	"fmt"
	"sort"
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
// AdminSetUserStatus 封禁/解封玩家。
//
// ⚠️ 第 114 轮：状态写入与 refresh token 吊销必须在**同一事务**里。
// 修前是两条独立 autocommit：封禁成功后第二条失败时，
// users.status 已落库回不去，而 refresh token 还有效——
// 被禁账号在 RefreshTTL 内仍可换访问令牌「续命」，
// 运营止损动作实际没止住。下面的注释「封禁即吊销」
// 与旧实现（两条可分别失败）自相矛盾，这正是缺陷能活下来的原因。
func (s *Service) AdminSetUserStatus(ctx context.Context, userID int64, banned bool, reason string) (AdminUserView, error) {
	status := 1
	if banned {
		status = 2
	}
	err := s.DB.Tx(ctx, func(tx txType) error {
		if _, err := tx.Exec(ctx,
			`UPDATE users SET status = $2, ban_reason = $3 WHERE id = $1`, userID, status, reason); err != nil {
			return fmt.Errorf("set user status: %w", err)
		}
		// 封禁即吊销所有 refresh token，否则封禁后仍可用长期令牌续命；
		// 失败则整单回滚（status 恢复原值），不允许「半封」状态。
		if banned {
			if _, err := tx.Exec(ctx,
				`UPDATE refresh_tokens SET revoked_at = now() WHERE user_id = $1 AND revoked_at IS NULL`,
				userID); err != nil {
				return fmt.Errorf("revoke tokens: %w", err)
			}
		}
		return nil
	})
	if err != nil {
		return AdminUserView{}, err
	}
	var v AdminUserView
	err = s.pool.QueryRow(ctx, `
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

// AdminGrantCurrency 后台发币（负数为回收）。
//
// ⚠️ 第 109 轮：**符号必须进审计**。
// 修前无论正负，钱包流水一律记 `admin_grant`、
// 后台审计一律记 `grant_currency` ——
// 运营在「发放资源」框里输入负数就是**静默回收**玩家货币，
// 事后翻审计只看到「发放」，分不清哪笔是扣款。
// 回收是真实需求（老测试写明的「回收场景」），但需求不该靠
// 「正数事件里混负数」实现 —— 符号本身必须成为审计字段。
func (s *Service) AdminGrantCurrency(ctx context.Context, userID int64, currency string, amount int64) (map[string]int64, error) {
	if amount == 0 {
		return nil, fmt.Errorf("%w: 数量为 0 的发放没有意义", ErrBadInput)
	}
	reason := "admin_grant"
	if amount < 0 {
		reason = "admin_revoke"
	}
	out := map[string]int64{}
	err := s.DB.Tx(ctx, func(tx txType) error {
		if err := s.grantWallet(ctx, tx, userID,
			map[string]int64{currency: amount}, reason, 0); err != nil {
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

	// 本场战报的验真状态。
	//
	// 上一轮把「每用户」的验真统计接进了 `/admin/users`，运营于是知道**谁**可疑；
	// 但「**哪一场**对局不匹配」仍然答不出来 —— 还得手工按 user_id 去查战报、
	// 再和 `replay_verifications` 交叉比对。这里补上最后一段。
	//
	// 同样是**两个字段**：没被验真过（0/0）不等于「验过且一致」。
	VerifyChecked    int64 `json:"verify_checked"`
	VerifyMismatched int64 `json:"verify_mismatched"`
}

// AdminListBattles 分页查询战报。
//
// onlyMismatched 为真时**只**返回「被验真过且有不匹配记录」的战报，
// 用来从「这个用户有 3 次不匹配」直接跳到「是哪 3 场」。
func (s *Service) AdminListBattles(ctx context.Context, userID int64, levelID int, limit int, onlyMismatched bool) ([]AdminBattleView, int64, error) {
	if limit <= 0 || limit > 200 {
		limit = 50
	}

	// 验真统计同样走**子查询预聚合**：一场战斗可以被验真多次，
	// 直接 JOIN 会放大行数，让分页切在错误粒度上。
	const vsub = `LEFT JOIN (
		SELECT battle_id,
		       COUNT(*) AS checked,
		       COUNT(*) FILTER (WHERE NOT matched) AS mismatched
		FROM replay_verifications GROUP BY battle_id
	) v ON v.battle_id = br.id`

	// 过滤条件写成 `COALESCE(v.mismatched, 0) > 0` 而不是 `v.mismatched > 0`。
	//
	// ⚠️ 这里我一开始写了「后者会让未验真的行被静默丢弃」—— **那句话是错的**，
	// 是我的第 16 次「前提不成立」。实测：把 COALESCE 去掉，三个测试**照样全绿**。
	//
	// 原因是 `> 0` 这种比较下两种写法**行为完全相同**：
	// LEFT JOIN 未匹配时 v.mismatched 是 NULL，`NULL > 0` 求值为 NULL（不是 true），
	// 于是那行被排除；而 COALESCE 版本得 `0 > 0` = false，也被排除。
	// 「未验真的战报不该出现在「只看不匹配」的列表里」是**预期行为**，
	// 不是 COALESCE 修掉的 bug。
	//
	// 那为什么还保留 COALESCE？**为了可读性**：它把「没有验真记录 ⇒ 计数为 0」
	// 这件事写在脸上，而不是依赖「NULL 会顺着比较运算传播」这条不直观的规则。
	// 真正会区分两者的写法是 `IS NOT TRUE` 那种（NULL 与 false 分开对待），
	// 本查询不需要。
	//
	// 保留这条注释而不是删掉：下一个人很可能也会认为这里有坑，
	// 然后为了「修」它而改动别的地方。
	where := `($1 = 0 OR br.user_id = $1) AND ($2 = 0 OR br.level_id = $2)`
	if onlyMismatched {
		where += ` AND COALESCE(v.mismatched, 0) > 0`
	}

	var total int64
	if err := s.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM battle_records br `+vsub+` WHERE `+where,
		userID, levelID).Scan(&total); err != nil {
		return nil, 0, fmt.Errorf("count battles: %w", err)
	}
	rows, err := s.pool.Query(ctx, `
		SELECT br.id, br.user_id, COALESCE(u.nickname,''), br.level_id, br.result, br.stars, br.score,
		       br.kills, br.leaked, br.wave_reached, br.duration_ms, br.reactions, br.heat_max,
		       br.replay_hash, br.created_at,
		       COALESCE(v.checked, 0), COALESCE(v.mismatched, 0)
		FROM battle_records br
		LEFT JOIN users u ON u.id = br.user_id
		`+vsub+`
		WHERE `+where+`
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
			&v.HeatMax, &v.ReplayHash, &v.CreatedAt,
			&v.VerifyChecked, &v.VerifyMismatched); err != nil {
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
func (s *Service) AdminUpdateLevel(ctx context.Context, levelID int, patch map[string]any) (map[string]any, error) {
	fields := []string{}
	args := []any{levelID}
	set := func(col string, v any) {
		args = append(args, v)
		fields = append(fields, fmt.Sprintf("%s = $%d", col, len(args)))
	}
	//
	// ⚠️ 第 107 轮：**非法键整单拒绝**，不再「静默丢弃那一项」。
	//
	// 修前每个 case 都是 `if 条件 { set(...) }`，条件不满足就什么都不做，
	// 而循环继续。于是：
	//
	//	PUT {"name":"P107改名", "base_hp":-5}  →  200
	//	  库里 name 改了，base_hp **没改**
	//
	// 运营看到 200 就以为两个字段都调过了。
	// 那是「部分生效 + 成功回执」，比直接报错坏得多 ——
	// 直接报错至少会说「你的请求有问题」。
	//
	// 只有一个键时 `len(fields)==0` 已经能兜住（返回 400「没有可更新的字段」），
	// 但**混合**情况兜不住 —— 而混合恰恰是运营调参时最常见的形状。
	var rejected []string
	reject := func(key, why string) {
		rejected = append(rejected, fmt.Sprintf("%s（%s）", key, why))
	}

	for key, val := range patch {
		switch key {
		case "name":
			v, ok := val.(string)
			if !ok || v == "" {
				reject(key, "必须是非空字符串")
				continue
			}
			set("name", v)
		case "base_hp":
			v, ok := toInt64(val)
			if !ok || v <= 0 {
				reject(key, "必须是正整数")
				continue
			}
			set("base_hp", v)
		case "wave_count":
			v, ok := toInt64(val)
			if !ok || v <= 0 || v > 50 {
				reject(key, "必须在 1..50")
				continue
			}
			set("wave_count", v)
		case "difficulty":
			v, ok := toInt64(val)
			if !ok || v <= 0 {
				reject(key, "必须是正整数")
				continue
			}
			set("difficulty", v)
		case "energy_cost":
			v, ok := toInt64(val)
			if !ok || v < 0 || v > 100 {
				reject(key, "必须在 0..100")
				continue
			}
			set("energy_cost", v)
		case "star_targets":
			//
			// ⚠️ 第 107 轮：原来只有 `json.Marshal` 成不成功这一个判据。
			//
			// 我**原以为** `{"star_targets": null}` 会撞 NOT NULL 变成 500 ——
			// **实测否证**：JSONB 的 `null` 是一个**合法的 JSONB 值**，
			// 不是 SQL NULL，所以 NOT NULL 约束照样通过。
			// （第 4 次「结论下得太早」：第 85/87/92/103 轮之后。）
			//
			// 但实测确实暴露了另一件事：`{"star_targets": "不是数组"}` 与
			// `{"terrain_config": 42}` 都会被**接受**，
			// 于是 `AdminListLevels` 的 `json.Unmarshal` 失败、
			// `stars` / `terrain` 保持 nil，后台看到的是「星级目标为空」。
			//
			// 那是「写进去一个永远读不出来的东西」——
			// 合法 JSONB、合法约束，但语义上已坏，且没有任何报错。
			// 所以判据必须落在**形状**上，不是「能不能序列化」。
			//
			// 显式的 JSON `null` 是**清空语义**，必须放行。
			// ⚠️ 我第一版的形状校验一刀切 `val.([]any)`，
			// 于是 `{"star_targets": null}` 变成 400 ——
			// 而「清空星级目标」是运营会真的想做的事。
			//
			// 判据踩在「合法」的边界上时，要么误杀、要么漏判；
			// 所以「**该接受的**」也要逐条列出（见
			// `TestLevelPatchAcceptsEmptyJSONArrays`）。
			//
			// 而且 `json.Unmarshal([]byte("null"), &[]int64{})` 是**成功**的
			// （结果为 nil 切片），所以存 `null` 读回来不会报错。
			if val == nil {
				set(key, json.RawMessage("null"))
				continue
			}
			arr, ok := val.([]any)
			if !ok {
				reject(key, "必须是数组或 null")
				continue
			}
			badElem := false
			for _, e := range arr {
				n, ok := toInt64(e)
				if !ok || n < 0 {
					reject(key, "数组元素必须是非负整数")
					badElem = true
					break
				}
			}
			if badElem {
				continue
			}
			raw, err := json.Marshal(arr)
			if err != nil {
				reject(key, "无法序列化")
				continue
			}
			set("star_targets", raw)
		case "terrain_config":
			if val == nil {
				set(key, json.RawMessage("null"))
				continue
			}
			arr, ok := val.([]any)
			if !ok {
				reject(key, "必须是数组或 null")
				continue
			}
			badElem := false
			for _, e := range arr {
				if _, ok := e.(map[string]any); !ok {
					reject(key, "数组元素必须是对象")
					badElem = true
					break
				}
			}
			if badElem {
				continue
			}
			raw, err := json.Marshal(arr)
			if err != nil {
				reject(key, "无法序列化")
				continue
			}
			set("terrain_config", raw)
		case "enabled":
			v, ok := val.(bool)
			if !ok {
				reject(key, "必须是布尔值")
				continue
			}
			set("enabled", v)
		case "is_boss":
			v, ok := val.(bool)
			if !ok {
				reject(key, "必须是布尔值")
				continue
			}
			set("is_boss", v)
		default:
			// 未知键必须报错 —— 「白名单外的字段被忽略」是个**陷阱**：
			// 运营拼错字段名（`baseHP` / `hp`）时会被静默吞掉，
			// 而请求整体因为有别的合法键而返回 200。
			reject(key, "不在可更新字段的白名单里")
		}
	}

	if len(rejected) > 0 {
		sort.Strings(rejected)
		return nil, fmt.Errorf("%w: 这些字段没被接受，整单未执行：%s",
			ErrBadInput, strings.Join(rejected, "、"))
	}
	if len(fields) == 0 {
		return nil, fmt.Errorf("%w: 没有可更新的字段", ErrBadInput)
	}
	sql := fmt.Sprintf(`UPDATE levels SET %s, updated_at = now() WHERE id = $1`,
		joinComma(fields))
	if _, err := s.pool.Exec(ctx, sql, args...); err != nil {
		return nil, fmt.Errorf("update level: %w", err)
	}
	//
	// ⚠️ 第 107 轮：这里原来返回 `domain.GenerateLevel(levelID)`。
	//
	// 那是**纯函数**（只从 ChapterOf + LCG 算，从不查库），
	// 所以运营刚把 base_hp 改成 987654，响应里却是改**前**的 1000。
	//
	// 一个与自己的写入相矛盾的响应，不管背后是哪条设计路线，都是缺陷。
	// 现在回读**真实落库的那一行**。
	//
	// ⚠️ 这**不代表**改动对玩家生效 —— 玩家侧走的是纯生成器。
	// 见 `admin_level_authority_test.go` 的现状刻画测试，
	// 以及 README 已知边界第 23 条（需要产品决策）。
	row, err := s.AdminLevelRow(ctx, levelID)
	if err != nil {
		return nil, fmt.Errorf("read back level: %w", err)
	}
	return row, nil
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
// AdminRegenerateLevels 重新生成全部关卡（覆盖后台的手工改动）。
//
// # 第 116 轮两处修正
//
//  1. **保留运营的 `base_hp`**（与后台确认文案一致）：
//     文案承诺「自定义的防线血量会保留」，但旧实现把 `base_hp` 也按生成器值
//     覆盖掉 —— 运营手工调过的防线血量被无声重置。
//     重新生成的语义是「同步生成器内容（波次/难度/地形/星级）」，
//     不该连运营自己调的 base_hp 一起冲掉。
//
//  2. **整批原子 + 不再吞错**：旧实现 100 条 UPDATE 各走独立 autocommit、
//     且 `starRaw,_ := json.Marshal` 静默吞错。中途失败会留下
//     「前 N 关已重生成 + 后 M 关保留旧值」的**混合态**且不可回滚。
//     现在整批进同一事务，任一失败整体回滚；Marshal 失败显式报错。
func (s *Service) AdminRegenerateLevels(ctx context.Context) (int, error) {
	levels := domain.GenerateAllLevels()
	err := s.DB.Tx(ctx, func(tx txType) error {
		for _, gl := range levels {
			starRaw, err := json.Marshal(gl.StarTargets)
			if err != nil {
				return fmt.Errorf("regenerate level %d: marshal star_targets: %w", gl.ID, err)
			}
			terrainRaw, err := json.Marshal(gl.Terrain)
			if err != nil {
				return fmt.Errorf("regenerate level %d: marshal terrain: %w", gl.ID, err)
			}
			// 注意：**没有 base_hp** —— 保留库里运营自定义的值。
			if _, err := tx.Exec(ctx, `
				UPDATE levels SET chapter=$2, name=$3, seed=$4, wave_count=$5,
				                 difficulty=$6, energy_cost=$7, star_targets=$8, terrain_config=$9,
				                 is_boss=$10, updated_at=now()
				 WHERE id=$1`,
				gl.ID, gl.Chapter, gl.Name, gl.Seed, gl.WaveCount, gl.Difficulty,
				gl.EnergyCost, starRaw, terrainRaw, gl.IsBoss); err != nil {
				return fmt.Errorf("regenerate level %d: %w", gl.ID, err)
			}
		}
		return nil
	})
	if err != nil {
		return 0, err
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
