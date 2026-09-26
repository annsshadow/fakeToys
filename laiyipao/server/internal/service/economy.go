package service

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strconv"
	"strings"
	"time"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/domain"
)

// code2Session 调用微信 jscode2session 换取 openid。
//
// 真实接入需要 AppID/Secret；未配置时 WechatLogin 会先行拒绝，
// 所以这里只在配置完整时才会被调用。
func (s *Service) code2Session(ctx context.Context, code string) (string, error) {
	q := url.Values{}
	q.Set("appid", s.Cfg.WechatAppID)
	q.Set("secret", s.Cfg.WechatAppSecret)
	q.Set("js_code", code)
	q.Set("grant_type", "authorization_code")

	req, err := http.NewRequestWithContext(ctx, http.MethodGet,
		s.Cfg.WechatEndpoint+"?"+q.Encode(), nil)
	if err != nil {
		return "", fmt.Errorf("build wechat request: %w", err)
	}
	reqCtx, cancel := context.WithTimeout(ctx, 8*time.Second)
	defer cancel()

	resp, err := http.DefaultClient.Do(req.WithContext(reqCtx))
	if err != nil {
		return "", fmt.Errorf("call wechat: %w", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(io.LimitReader(resp.Body, 1<<20))
	if err != nil {
		return "", fmt.Errorf("read wechat response: %w", err)
	}

	var payload struct {
		OpenID     string `json:"openid"`
		UnionID    string `json:"unionid"`
		SessionKey string `json:"session_key"`
		ErrCode    int    `json:"errcode"`
		ErrMsg     string `json:"errmsg"`
	}
	if err := json.Unmarshal(body, &payload); err != nil {
		return "", fmt.Errorf("parse wechat response: %w", err)
	}
	if payload.ErrCode != 0 {
		return "", fmt.Errorf("%w: 微信返回 errcode=%d %s", ErrForbidden, payload.ErrCode, payload.ErrMsg)
	}
	if payload.OpenID == "" {
		return "", fmt.Errorf("%w: 微信未返回 openid", ErrForbidden)
	}
	return payload.OpenID, nil
}

// --- 排行榜 ---

// LeaderboardEntry 是一条榜单记录。
type LeaderboardEntry struct {
	Rank      int    `json:"rank"`
	UserID    int64  `json:"user_id"`
	Nickname  string `json:"nickname"`
	AvatarURL string `json:"avatar_url"`
	Score     int64  `json:"score"`
	Detail    string `json:"detail,omitempty"`
}

// Leaderboard 返回指定榜单。
//
// 三种榜单对应三种不同的"强"：战力榜=资源总量，效率榜=技巧，
// 关卡榜=进度。分开排才能让低养成高技巧的玩家也有位置。
func (s *Service) Leaderboard(ctx context.Context, kind string, limit int) ([]LeaderboardEntry, error) {
	if limit <= 0 || limit > 200 {
		limit = 50
	}
	var rows interface {
		Next() bool
		Scan(...any) error
		Err() error
		Close()
	}
	var err error

	switch kind {
	case "stage":
		rows, err = s.pool.Query(ctx, `
			SELECT u.id, u.nickname, u.avatar_url, MAX(ls.level_id)
			FROM level_stars ls JOIN users u ON u.id = ls.user_id
			WHERE ls.clears > 0
			GROUP BY u.id, u.nickname, u.avatar_url
			ORDER BY 4 DESC, u.id ASC LIMIT $1`, limit)
	case "efficiency":
		// 效率榜：通关时使用的最低战力越低越靠前（L-2）
		rows, err = s.pool.Query(ctx, `
			SELECT u.id, u.nickname, u.avatar_url, MIN(ls.min_power_clear)
			FROM level_stars ls JOIN users u ON u.id = ls.user_id
			WHERE ls.min_power_clear > 0
			GROUP BY u.id, u.nickname, u.avatar_url
			ORDER BY 4 ASC, u.id ASC LIMIT $1`, limit)
	default: // "power"
		rows, err = s.pool.Query(ctx, `
			SELECT u.id, u.nickname, u.avatar_url, p.level_exp
			FROM user_progress p JOIN users u ON u.id = p.user_id
			ORDER BY 4 DESC, u.id ASC LIMIT $1`, limit)
	}
	if err != nil {
		return nil, fmt.Errorf("leaderboard %s: %w", kind, err)
	}
	defer rows.Close()

	out := make([]LeaderboardEntry, 0, limit)
	rank := 1
	for rows.Next() {
		var e LeaderboardEntry
		if err := rows.Scan(&e.UserID, &e.Nickname, &e.AvatarURL, &e.Score); err != nil {
			return nil, err
		}
		e.Rank = rank
		rank++
		out = append(out, e)
	}
	return out, rows.Err()
}

// --- 签到 / 离线 / 兑换 / 商店 ---

// SignInResult 是签到结果。
type SignInResult struct {
	Already  bool           `json:"already"`
	DayIndex int            `json:"day_index"`
	Reward   map[string]int `json:"reward"`
	Wallet   Wallet         `json:"wallet"`
}

// SignIn 执行每日签到。
//
// 同一天只能签一次：靠 user_sign_ins 的 (user_id, sign_date) 主键保证，
// 不做"先查后写"的两步判断 —— 那是并发下的经典重复领取漏洞。
func (s *Service) SignIn(ctx context.Context, userID int64) (SignInResult, error) {
	res := SignInResult{}
	err := s.DB.Tx(ctx, func(tx pgx.Tx) error {
		var dayIndex int
		var rewardRaw []byte
		if err := tx.QueryRow(ctx,
			`SELECT day_index, reward FROM sign_in_calendar
			 WHERE day_index = (
			   SELECT COALESCE(MAX(day_index), 0) + 1 FROM user_sign_ins WHERE user_id = $1)`,
			userID).Scan(&dayIndex, &rewardRaw); err != nil {
			if err == pgx.ErrNoRows {
				res.Already = true // 七天已签完或今天已签
				return nil
			}
			return err
		}

		today := time.Now().Truncate(24 * time.Hour)
		if _, err := tx.Exec(ctx,
			`INSERT INTO user_sign_ins (user_id, sign_date, day_index, reward) VALUES ($1,$2,$3,$4)`,
			userID, time.Now(), dayIndex, rewardRaw); err != nil {
			// 唯一键冲突 = 今天已签
			return fmt.Errorf("%w: 今日已签到", ErrForbidden)
		}
		_ = today

		var reward map[string]int
		if err := json.Unmarshal(rewardRaw, &reward); err != nil {
			return err
		}
		deltas := map[string]int64{}
		for k, v := range reward {
			deltas[k] = int64(v)
		}
		if err := s.grantWallet(ctx, tx, userID, deltas, "signin", int64(dayIndex)); err != nil {
			return err
		}
		res.DayIndex = dayIndex
		res.Reward = reward
		return nil
	})
	if err != nil {
		return SignInResult{}, err
	}
	w, err := s.LoadWallet(ctx, userID)
	if err != nil {
		return SignInResult{}, err
	}
	res.Wallet = w
	return res, nil
}

// Redeem 兑换兑换码。
func (s *Service) Redeem(ctx context.Context, userID int64, code string) (map[string]int, error) {
	var out map[string]int
	err := s.DB.Tx(ctx, func(tx pgx.Tx) error {
		var codeID int
		var rewardRaw []byte
		if err := tx.QueryRow(ctx,
			`SELECT id, reward FROM redeem_codes
			 WHERE code = $1 AND enabled AND (expires_at IS NULL OR expires_at > now())
			   AND used_count < max_uses`, code).Scan(&codeID, &rewardRaw); err != nil {
			if err == pgx.ErrNoRows {
				return fmt.Errorf("%w: 兑换码无效或已用尽", ErrBadInput)
			}
			return err
		}
		// 先占位：唯一键保证一个用户一个码只能用一次
		tag, err := tx.Exec(ctx,
			`INSERT INTO redeem_usages (code_id, user_id) VALUES ($1,$2)
			 ON CONFLICT (code_id, user_id) DO NOTHING`, codeID, userID)
		if err != nil {
			return err
		}
		if tag.RowsAffected() == 0 {
			return fmt.Errorf("%w: 你已经使用过该兑换码", ErrForbidden)
		}
		// ⚠️ 条件 UPDATE + 检查 RowsAffected 才是互斥点。
		//
		// 上面的 `used_count < max_uses` 只是前置 SELECT，在并发下是**无效预检**：
		// N 个并发事务全部在对方提交前读到 used_count=0，于是全部通过。
		// 旧实现是无条件 `used_count = used_count + 1 WHERE id = $1`，
		// 于是 used_count 一路冲过 max_uses，最终靠 redeem_codes 上的
		// CHECK (used_count <= max_uses) 整笔回滚。
		//
		// 资源上没超发，但代价有三个：
		//  1) 客户端收到 SQLSTATE 23514（数据库约束名），而不是"兑换码已被抢完"
		//  2) 一旦有人迁移掉那条 CHECK（它只是"恰好"存在，不是并发设计），立刻变成无限超发
		//  3) 报错信息泄露内部表结构
		//
		// 与 Buy 的 FOR UPDATE、ClaimTask 的 claimed_at IS NULL 是同一个模式：
		// 让数据库的条件写充当互斥量，用 RowsAffected 判断是否真的抢到。
		tag, err = tx.Exec(ctx,
			`UPDATE redeem_codes SET used_count = used_count + 1
			 WHERE id = $1 AND used_count < max_uses`, codeID)
		if err != nil {
			return err
		}
		if tag.RowsAffected() == 0 {
			return fmt.Errorf("%w: 兑换码已被抢完", ErrBadInput)
		}
		var reward map[string]int
		if err := json.Unmarshal(rewardRaw, &reward); err != nil {
			return err
		}
		deltas := map[string]int64{}
		for k, v := range reward {
			deltas[k] = int64(v)
		}
		if err := s.grantWallet(ctx, tx, userID, deltas, "redeem", int64(codeID)); err != nil {
			return err
		}
		out = reward
		return nil
	})
	return out, err
}

// ShopItemView 是商城条目。
type ShopItemView struct {
	ID       int            `json:"id"`
	Code     string         `json:"code"`
	Name     string         `json:"name"`
	Category string         `json:"category"`
	Price    map[string]int `json:"price"`
	Payload  map[string]int `json:"payload"`
	Limit    int            `json:"limit_per_day"`
	Bought   int            `json:"bought_today"`
}

// LoadShop 返回商城，附带玩家今日已购次数。
func (s *Service) LoadShop(ctx context.Context, userID int64) ([]ShopItemView, error) {
	rows, err := s.pool.Query(ctx, `
		SELECT si.id, si.code, si.name, si.category, si.price, si.payload, si.limit_per_day,
		       (SELECT COUNT(*) FROM user_purchases up
		         WHERE up.user_id = $1 AND up.item_id = si.id
		           AND up.purchase_date = CURRENT_DATE)
		FROM shop_items si WHERE si.enabled ORDER BY si.sort_order`, userID)
	if err != nil {
		return nil, fmt.Errorf("load shop: %w", err)
	}
	defer rows.Close()

	var out []ShopItemView
	for rows.Next() {
		var v ShopItemView
		var priceRaw, payloadRaw []byte
		if err := rows.Scan(&v.ID, &v.Code, &v.Name, &v.Category, &priceRaw, &payloadRaw, &v.Limit, &v.Bought); err != nil {
			return nil, err
		}
		_ = json.Unmarshal(priceRaw, &v.Price)
		_ = json.Unmarshal(payloadRaw, &v.Payload)
		out = append(out, v)
	}
	return out, rows.Err()
}

// Buy 购买商城商品。
func (s *Service) Buy(ctx context.Context, userID, itemID int64) (map[string]int, error) {
	var out map[string]int
	err := s.DB.Tx(ctx, func(tx pgx.Tx) error {
		var priceRaw, payloadRaw []byte
		var limit int
		if err := tx.QueryRow(ctx,
			`SELECT price, payload, limit_per_day FROM shop_items WHERE id = $1 AND enabled`,
			itemID).Scan(&priceRaw, &payloadRaw, &limit); err != nil {
			if err == pgx.ErrNoRows {
				return fmt.Errorf("%w: 商品 %d", ErrNotFound, itemID)
			}
			return err
		}

		if limit > 0 {
			// 先锁住该用户的钱包行，再做 COUNT。
			//
			// 「SELECT COUNT(*) 再 INSERT」在并发下是**完全无效**的限购：
			// N 个并发事务全部在对方 INSERT 之前读到 bought=0，于是全部放行。
			// shop_firstpay 价格是 {coin: 0}（首充 0 元购），
			// 并发打 20 个请求就能白拿 20 份 gem/coin/energy。
			//
			// 锁 user_wallets 是因为它必然会被本事务后续的 grantWallet 更新，
			// 提前加锁把同一用户的购买串行化，且不引入新的锁顺序分支
			// （所有路径都是先钱包后 purchases）。
			var lockedUID int64
			if err := tx.QueryRow(ctx,
				`SELECT user_id FROM user_wallets WHERE user_id = $1 FOR UPDATE`,
				userID).Scan(&lockedUID); err != nil {
				if errors.Is(err, pgx.ErrNoRows) {
					return fmt.Errorf("%w: 钱包不存在", ErrNotFound)
				}
				return err
			}
			var bought int
			if err := tx.QueryRow(ctx,
				`SELECT COUNT(*) FROM user_purchases
				 WHERE user_id = $1 AND item_id = $2 AND purchase_date = CURRENT_DATE`,
				userID, itemID).Scan(&bought); err != nil {
				return err
			}
			if bought >= limit {
				return fmt.Errorf("%w: 今日购买次数已达上限 %d", ErrForbidden, limit)
			}
		}

		var price, payload map[string]int
		if err := json.Unmarshal(priceRaw, &price); err != nil {
			return err
		}
		if err := json.Unmarshal(payloadRaw, &payload); err != nil {
			return err
		}

		// 先扣钱再发货，扣钱失败则整体回滚
		deduct := map[string]int64{}
		for k, v := range price {
			if v > 0 {
				deduct[k] = int64(-v)
			}
		}
		if err := s.grantWallet(ctx, tx, userID, deduct, "shop_buy", itemID); err != nil {
			return err
		}

		grant := map[string]int64{}
		for k, v := range payload {
			if v > 0 {
				grant[k] = int64(v)
			}
		}
		if err := s.grantWallet(ctx, tx, userID, grant, "shop_buy", itemID); err != nil {
			return err
		}
		if _, err := tx.Exec(ctx,
			`INSERT INTO user_purchases (user_id, item_id, purchase_date, price) VALUES ($1,$2,CURRENT_DATE,$3)`,
			userID, itemID, priceRaw); err != nil {
			return err
		}
		out = payload
		return nil
	})
	return out, err
}

// --- 验真（I-6） ---

// VerifyInput 是验真请求。
type VerifyInput struct {
	BattleID   int64  `json:"battle_id"`
	ReplayHash string `json:"replay_hash"`
}

// VerifyResult 是验真结果。
type VerifyResult struct {
	BattleID int64  `json:"battle_id"`
	Expected string `json:"expected_hash"`
	Actual   string `json:"actual_hash"`
	Matched  bool   `json:"matched"`
	LevelID  int    `json:"level_id"`
	// Seed 必须是字符串：int64 超过 2^53 后 JSON number 在 JS 侧解析即失精，
	// 客户端据此重放会拿到不同的种子，算出的哈希必然对不上（I-6 失效）。
	Seed       string `json:"seed"`
	RecordedAt string `json:"recorded_at"`
}

// VerifyReplay 记录一次验真尝试（I-6）。
//
// 服务端不重放（那需要服务端跑引擎）—— 客户端本地重放后把哈希回传，
// 服务端只做比对与留痕。被质疑的分数因此可被任何人独立复现。
func (s *Service) VerifyReplay(ctx context.Context, userID, battleID int64, actualHash string) (VerifyResult, error) {
	var r VerifyResult
	// created_at 是 timestamptz，必须扫进 time.Time 再格式化 ——
	// 扫进 string 会失败，而失败若被当成 not_found 包装，就会把
	// "扫描类型错了" 误报成 "战报不存在"，极难排查。
	var createdAt time.Time
	var seed int64
	err := s.pool.QueryRow(ctx,
		`SELECT br.level_id, COALESCE(bt.seed, 0), br.replay_hash, br.created_at
		 FROM battle_records br
		 LEFT JOIN battle_tokens bt ON bt.id = br.battle_token_id
		 WHERE br.id = $1`, battleID).
		Scan(&r.LevelID, &seed, &r.Expected, &createdAt)
	if errors.Is(err, pgx.ErrNoRows) {
		return VerifyResult{}, fmt.Errorf("%w: battle %d", ErrNotFound, battleID)
	}
	if err != nil {
		return VerifyResult{}, fmt.Errorf("load battle %d for verify: %w", battleID, err)
	}
	r.RecordedAt = createdAt.UTC().Format(timeFormat)
	r.BattleID = battleID
	r.Seed = strconv.FormatInt(seed, 10)
	r.Actual = actualHash
	r.Matched = r.Expected == actualHash

	if _, err := s.pool.Exec(ctx,
		`INSERT INTO replay_verifications (battle_id, verifier_id, expected_hash, actual_hash, matched)
		 VALUES ($1,$2,$3,$4,$5)`,
		battleID, userID, r.Expected, r.Actual, r.Matched); err != nil {
		return VerifyResult{}, fmt.Errorf("record verification: %w", err)
	}
	return r, nil
}

// ReplayInfo 返回复现一局所需的全部信息（供客户端本地重放）。
type ReplayInfo struct {
	BattleID int64 `json:"battle_id"`
	LevelID  int   `json:"level_id"`
	// Seed 用字符串下发：int64 超过 2^53 后 JSON number 在 JS 侧解析即失精，
	// 客户端拿到的种子与库里那个不是同一个数 → 重放必然算不出相同哈希。
	Seed       string    `json:"seed"`
	Level      LevelResp `json:"level"`
	ReplayHash string    `json:"expected_hash"`
	CreatedAt  string    `json:"created_at"`
	// Build 是该局结算时冻结的构筑快照。
	//
	// ⚠️ 这个字段是 I-6 能否成立的关键：回放哈希由「关卡 + 种子 + 构筑」共同决定，
	// 只给关卡和种子而不给构筑，客户端重放出来的哈希必然对不上，
	// 于是所有正常对局都会被判为"不一致"—— 验真机制就成了纯噪音。
	Build map[string]any `json:"build"`
	// CardPicks 每波选中的手牌索引（-1 = 跳过）。
	//
	// ⚠️ 没有它就复现不出原局：选牌会改变后续战斗（技能升格/属性加成/机制词条）。
	// 服务端存的是逗号分隔的紧凑文本，这里解析成数组下发。
	CardPicks []int `json:"card_picks"`
}

// parseCardPicks 解析逗号分隔的选牌序列。空串表示旧记录没有该字段。
func parseCardPicks(raw string) []int {
	if strings.TrimSpace(raw) == "" {
		return nil
	}
	parts := strings.Split(raw, ",")
	out := make([]int, 0, len(parts))
	for _, p := range parts {
		n, err := strconv.Atoi(strings.TrimSpace(p))
		if err != nil {
			// 容忍脏数据：跳过单个坏值，而不是让整条记录不可用
			continue
		}
		out = append(out, n)
	}
	return out
}

// GetReplay 返回复现信息。
func (s *Service) GetReplay(ctx context.Context, battleID int64) (ReplayInfo, error) {
	var r ReplayInfo
	var createdAt time.Time
	var seed int64
	var cardPicks string
	err := s.pool.QueryRow(ctx,
		`SELECT br.level_id, COALESCE(bt.seed, 0), br.replay_hash, br.created_at,
		        br.build_snapshot, COALESCE(br.card_picks, '')
		 FROM battle_records br
		 LEFT JOIN battle_tokens bt ON bt.id = br.battle_token_id
		 WHERE br.id = $1`, battleID).
		Scan(&r.LevelID, &seed, &r.ReplayHash, &createdAt, &r.Build, &cardPicks)
	if err != nil {
		return ReplayInfo{}, fmt.Errorf("%w: battle %d", ErrNotFound, battleID)
	}
	r.BattleID = battleID
	r.Seed = strconv.FormatInt(seed, 10)
	r.CardPicks = parseCardPicks(cardPicks)
	r.CreatedAt = createdAt.UTC().Format(timeFormat)
	r.Level = NewLevelResp(domain.GenerateLevel(r.LevelID))

	// ⚠️ 从玩家侧响应里剥离 settle_input。
	//
	// 验真（I-6）只需要 level + seed + build + card_picks ——
	// 这些都是服务端权威数据，公开是设计使然（否则无人能验真）。
	// 但 settle_input 是**客户端上报的原始结算报文**，对验真毫无用处，
	// 却是伪造高分战报的最佳蓝本：照抄一个高星玩家的
	// kills / reactions / duration_ms 就能构造出"合法"的上报。
	//
	// battle_records.id 是连续 BIGSERIAL，任何注册用户都能遍历，
	// 所以这不是理论风险。
	delete(r.Build, "settle_input")
	return r, nil
}

// parseInt64 宽容地把字符串/数字解析为 int64。
func parseInt64(v string) (int64, error) {
	return strconv.ParseInt(v, 10, 64)
}
