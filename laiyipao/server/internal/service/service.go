// Package service 承载业务编排：事务边界、跨表一致性、权限校验。
// 纯计算规则一律下沉到 domain，这里只处理 IO 与编排。
package service

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"strings"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"

	"github.com/laiyipao/server/internal/auth"
	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/store"
)

// Errors 是服务层返回给上层的语义化错误。
var (
	ErrNotFound     = errors.New("资源不存在")
	ErrUnauthorized = errors.New("未认证或令牌已失效")
	ErrForbidden    = errors.New("无权访问")
	ErrBadInput     = errors.New("输入非法")
)

// Service 聚合全部依赖。
type Service struct {
	DB   *store.DB
	Cfg  config.Config
	JWT  *auth.Signer
	pool *pgxpool.Pool
}

// New 构造服务。
func New(db *store.DB, cfg config.Config) *Service {
	return &Service{
		DB:   db,
		Cfg:  cfg,
		JWT:  auth.NewSigner(cfg.JWTSecret, cfg.AccessTTL),
		pool: db.Pool,
	}
}

// --- 账号 ---

// User 是玩家公开信息。
type User struct {
	ID        int64     `json:"id"`
	Nickname  string    `json:"nickname"`
	AvatarURL string    `json:"avatar_url"`
	IsGuest   bool      `json:"is_guest"`
	Status    int       `json:"status"`
	CreatedAt time.Time `json:"created_at"`
}

// TokenPair 是一次签发结果。
type TokenPair struct {
	AccessToken  string    `json:"access_token"`
	RefreshToken string    `json:"refresh_token"`
	ExpiresAt    time.Time `json:"expires_at"`
	User         User      `json:"user"`
}

// GuestLogin 以设备凭证建立/恢复游客账号。
//
// 游客凭证是客户端持久化的随机串，服务端不绑定任何个人信息 ——
// 这样在没有 AppID 的情况下也能完整体验，且天然满足最小化收集原则。
func (s *Service) GuestLogin(ctx context.Context, guestToken, nickname string) (TokenPair, error) {
	if strings.TrimSpace(guestToken) == "" {
		var err error
		guestToken, err = auth.NewGuestToken()
		if err != nil {
			return TokenPair{}, err
		}
	}
	if nickname == "" {
		nickname = "玩家" + fmt.Sprintf("%04d", time.Now().UnixNano()%10000)
	}

	var userID int64
	err := s.pool.QueryRow(ctx,
		`INSERT INTO users (guest_token, nickname, is_guest, last_login_at)
		 VALUES ($1, $2, TRUE, now())
		 ON CONFLICT (guest_token) DO UPDATE SET last_login_at = now()
		 RETURNING id`, guestToken, nickname).Scan(&userID)
	if err != nil {
		return TokenPair{}, fmt.Errorf("upsert user: %w", err)
	}

	if err := s.initNewUser(ctx, userID); err != nil {
		return TokenPair{}, err
	}

	u, err := s.loadUser(ctx, userID)
	if err != nil {
		return TokenPair{}, err
	}
	if u.Status != 1 {
		return TokenPair{}, fmt.Errorf("%w: 账号已被封禁", ErrForbidden)
	}
	return s.issueTokens(ctx, userID, u)
}

// WechatLogin 微信 code 登录。
//
// AppID/Secret 未配置时返回明确错误（501），而不是静默降级成游客登录 ——
// 静默降级会让"登录失败"变成"登录成功但数据不对应"，极难排查。
func (s *Service) WechatLogin(ctx context.Context, code string) (TokenPair, error) {
	if !s.Cfg.WechatEnabled() {
		return TokenPair{}, fmt.Errorf("%w: 微信登录未启用（缺少 WECHAT_APP_ID / WECHAT_APP_SECRET）", ErrForbidden)
	}
	if strings.TrimSpace(code) == "" {
		return TokenPair{}, fmt.Errorf("%w: code 不能为空", ErrBadInput)
	}
	openID, err := s.code2Session(ctx, code)
	if err != nil {
		return TokenPair{}, err
	}

	var userID int64
	err = s.pool.QueryRow(ctx,
		`INSERT INTO users (guest_token, openid, nickname, is_guest, last_login_at)
		 VALUES ($1, $2, '', FALSE, now())
		 ON CONFLICT (openid) DO UPDATE SET last_login_at = now()
		 RETURNING id`,
		"wx_"+openID, openID).Scan(&userID)
	if err != nil {
		return TokenPair{}, fmt.Errorf("upsert wechat user: %w", err)
	}
	if err := s.initNewUser(ctx, userID); err != nil {
		return TokenPair{}, err
	}
	u, err := s.loadUser(ctx, userID)
	if err != nil {
		return TokenPair{}, err
	}
	return s.issueTokens(ctx, userID, u)
}

// Refresh 刷新访问令牌。
func (s *Service) Refresh(ctx context.Context, refreshToken string) (TokenPair, error) {
	if refreshToken == "" {
		return TokenPair{}, fmt.Errorf("%w: refresh_token 不能为空", ErrBadInput)
	}
	hash := auth.HashRefreshToken(refreshToken)
	var userID int64
	err := s.pool.QueryRow(ctx,
		`UPDATE refresh_tokens SET revoked_at = now()
		 WHERE token_hash = $1 AND revoked_at IS NULL AND expires_at > now()
		 RETURNING user_id`, hash).Scan(&userID)
	if err != nil {
		return TokenPair{}, fmt.Errorf("%w: refresh_token 无效或已过期", ErrUnauthorized)
	}
	u, err := s.loadUser(ctx, userID)
	if err != nil {
		return TokenPair{}, err
	}
	return s.issueTokens(ctx, userID, u)
}

// issueTokens 签发玩家令牌对。
//
// ⚠️ 封禁检查必须放在**这里**，而不是散在各个登录 handler 里。
// 之前只有 GuestLogin 查 u.Status，WechatLogin 与 Refresh 都没查，
// 于是"封禁一个微信用户 → 他重新登录拿新令牌"即可绕过封禁。
// 把检查下沉到唯一的签发出口，所有路径（游客/微信/刷新）自动覆盖。
func (s *Service) issueTokens(ctx context.Context, userID int64, u User) (TokenPair, error) {
	if u.Status != 1 {
		return TokenPair{}, fmt.Errorf("%w: 账号已被封禁", ErrForbidden)
	}
	access, exp, err := s.JWT.Issue(userID, "user")
	if err != nil {
		return TokenPair{}, err
	}
	refreshPlain, refreshHash, err := auth.NewRefreshToken()
	if err != nil {
		return TokenPair{}, err
	}
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1,$2,$3)`,
		userID, refreshHash, time.Now().Add(s.Cfg.RefreshTTL)); err != nil {
		return TokenPair{}, fmt.Errorf("store refresh token: %w", err)
	}
	return TokenPair{
		AccessToken: access, RefreshToken: refreshPlain,
		ExpiresAt: exp, User: u,
	}, nil
}

// initNewUser 为新账号铺设初始数据（钱包、进度、签到记录、离线记录）。
// 用 ON CONFLICT DO NOTHING 保证幂等 —— 重复调用不会重置玩家进度。
func (s *Service) initNewUser(ctx context.Context, userID int64) error {
	_, err := s.pool.Exec(ctx, `
		INSERT INTO user_wallets (user_id, coin, gem, energy, keys) VALUES ($1, 2000, 50, 30, 1)
		ON CONFLICT (user_id) DO NOTHING`, userID)
	if err != nil {
		return fmt.Errorf("init wallet: %w", err)
	}
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO user_progress (user_id) VALUES ($1) ON CONFLICT (user_id) DO NOTHING`,
		userID); err != nil {
		return fmt.Errorf("init progress: %w", err)
	}
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO offline_rewards (user_id) VALUES ($1) ON CONFLICT (user_id) DO NOTHING`,
		userID); err != nil {
		return fmt.Errorf("init offline: %w", err)
	}
	// 新号赠送第一件装备，避免背包空白
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO user_equipment (user_id, equipment_id, slot, equipped)
		 VALUES ($1, 1, 'weapon', TRUE), ($1, 10, 'gloves', TRUE)
		 ON CONFLICT (user_id, equipment_id) DO NOTHING`, userID); err != nil {
		return fmt.Errorf("init equipment: %w", err)
	}
	// 新号解锁前 4 个技能
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO user_skills (user_id, skill_id) VALUES ($1,1),($1,2),($1,3),($1,4)
		 ON CONFLICT (user_id, skill_id) DO NOTHING`, userID); err != nil {
		return fmt.Errorf("init skills: %w", err)
	}

	// ⚠️ 必须同时给出战槽位，否则新玩家进游戏就是死局：
	// 开局拿到的 build.skills 全部 slot=-1，引擎没有任何技能可用，
	// 而唯一的配置入口（/me/loadout）在界面上又被"先打完一场"的引导挡在后面。
	//
	// 默认装 3 个：1 焰 / 2 冰 / 3 电 —— 三种不同元素，
	// 保证新号一进关就能触发反应链（I-1 的核心体验不能被"新号不会玩"埋掉）。
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO user_skill_slots (user_id, slot, skill_id) VALUES ($1,0,1),($1,1,2),($1,2,3)
		 ON CONFLICT (user_id, slot) DO NOTHING`, userID); err != nil {
		return fmt.Errorf("init skill slots: %w", err)
	}
	return nil
}

func (s *Service) loadUser(ctx context.Context, id int64) (User, error) {
	var u User
	err := s.pool.QueryRow(ctx,
		`SELECT id, nickname, avatar_url, is_guest, status, created_at
		 FROM users WHERE id = $1`, id).
		Scan(&u.ID, &u.Nickname, &u.AvatarURL, &u.IsGuest, &u.Status, &u.CreatedAt)
	if errors.Is(err, pgx.ErrNoRows) {
		return User{}, fmt.Errorf("%w: user %d", ErrNotFound, id)
	}
	if err != nil {
		return User{}, fmt.Errorf("load user: %w", err)
	}
	return u, nil
}

// ResolveUser 从 access token 解析用户 ID。
//
// ⚠️ 这里必须查一次 users.status。
// 只验 JWT 签名意味着被封禁的账号在 access token 整个 TTL（默认 2h）内
// 仍然畅通无阻 —— 运营封禁一个外挂号，它还能继续刷资源。
// 每请求多一次主键索引查询，换来「封禁即时生效」，值得。
func (s *Service) ResolveUser(ctx context.Context, token string) (int64, error) {
	claims, err := s.JWT.Verify(token)
	if err != nil {
		return 0, fmt.Errorf("%w: %s", ErrUnauthorized, err)
	}
	if claims.Kind != "user" {
		return 0, fmt.Errorf("%w: 令牌类型不匹配", ErrUnauthorized)
	}
	var status int
	if err := s.pool.QueryRow(context.Background(),
		`SELECT status FROM users WHERE id = $1`, claims.Sub).Scan(&status); err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return 0, fmt.Errorf("%w: 用户不存在", ErrUnauthorized)
		}
		return 0, err
	}
	if status != 1 {
		return 0, fmt.Errorf("%w: 账号已被封禁", ErrForbidden)
	}
	return claims.Sub, nil
}

// --- 钱包 ---

// Wallet 是玩家钱包快照。
type Wallet struct {
	Coin            int64     `json:"coin"`
	Gem             int64     `json:"gem"`
	Energy          int64     `json:"energy"`
	Keys            int64     `json:"keys"`
	EnergyUpdatedAt time.Time `json:"energy_updated_at"`
}

// grantWallet 在事务内增减货币并写流水。
//
// 扣减时用 `WHERE coin >= $amount` 保证不会出现负余额 ——
// 靠 CHECK 约束兜底会返回整批失败，业务层无法给出可读错误。
func (s *Service) grantWallet(ctx context.Context, tx pgx.Tx, userID int64, deltas map[string]int64, reason string, refID int64) error {
	for currency, delta := range deltas {
		if delta == 0 {
			continue
		}
		col, ok := walletColumns[currency]
		if !ok {
			// 第 74 轮：道具走 user_tokens，与货币列分开。
			//
			// 原来这里直接 `ErrBadInput: 未知货币`，而种子数据里
			// 有三个商品的 payload 是道具名 —— 于是**商城 8 件商品里
			// 有 3 件永远买不了**，客户端只看到一句
			// 「未知货币 "revive_token"」的 400。
			if isWalletToken(currency) {
				if err := grantToken(ctx, tx, userID, currency, delta, reason, refID); err != nil {
					return err
				}
				continue
			}
			return fmt.Errorf("%w: 未知货币 %q", ErrBadInput, currency)
		}

		// 增减两条语句的占位符不同，因此参数切片必须分别构造，
		// 否则会出现"传了 3 个参数但 SQL 只有 2 个占位符"的运行期错误。
		var q string
		args := []any{userID, delta}
		if delta > 0 {
			q = fmt.Sprintf(
				`UPDATE user_wallets SET %s = %s + $2, updated_at = now()
				 WHERE user_id = $1 RETURNING %s`, col, col, col)
		} else {
			// 扣减必须显式校验余额，靠 CHECK 约束兜底会整批失败且无法给出可读错误
			args = append(args, -delta)
			q = fmt.Sprintf(
				`UPDATE user_wallets SET %s = %s + $2, updated_at = now()
				 WHERE user_id = $1 AND %s >= $3 RETURNING %s`, col, col, col, col)
		}

		var balance int64
		if err := tx.QueryRow(ctx, q, args...).Scan(&balance); err != nil {
			if errors.Is(err, pgx.ErrNoRows) {
				return fmt.Errorf("%w: %s 不足（需要 %d）", ErrBadInput, currency, -delta)
			}
			return fmt.Errorf("grant %s: %w", currency, err)
		}
		if _, err := tx.Exec(ctx,
			`INSERT INTO wallet_flows (user_id, currency, delta, balance, reason, ref_id)
			 VALUES ($1,$2,$3,$4,$5,$6)`,
			userID, currency, delta, balance, reason, refID); err != nil {
			return fmt.Errorf("insert flow: %w", err)
		}
	}
	return nil
}

// grantToken 增减一件道具（第 74 轮）。
//
// # 与货币路径的三处**刻意**差异
//
//  1. **UPSERT 而非 UPDATE** —— 货币四列都有 DEFAULT 0、注册时就有行；
//     道具是稀疏的，第一次拿到时行还不存在。
//     用 `INSERT ... ON CONFLICT DO UPDATE` 一条语句解决，
//     不必先 SELECT 再分支。
//
//  2. **扣减不靠 WHERE 余额判定** —— 货币那条 `WHERE coin >= $3`
//     返回 `pgx.ErrNoRows`，上层据此报「不足」。
//     道具侧没有 `ErrNoRows` 这条线索（UPSERT 永远成功），
//     所以余额判定放到 CHECK 约束 + 显式回读：
//     违反时整批失败，报的是 PG 的 check violation。
//
//     ⚠️ 这是本函数**唯一**不如货币路径干净的地方，如实标注。
//     `grantToken` 目前只被 `grantWallet` 以 `delta > 0` 的方式调用
//     （pay 侧只接受货币，因为道具的花费语义还没定），
//     所以这条分支当前**不可达**。一旦有消费端点接上，必须先补余额判定。
//
//  3. **余额也写 wallet_flows** —— 运营看板与「货币流水」查询
//     按 currency 聚合，道具的进出必须出现在同一条流里，
//     否则看板会显示「道具收入 0」而实际有进账。
func grantToken(ctx context.Context, tx pgx.Tx, userID int64, token string, delta int64, reason string, refID int64) error {
	var balance int64
	q := `INSERT INTO user_tokens (user_id, token, balance)
	      VALUES ($1, $2, $3)
	      ON CONFLICT (user_id, token)
	      DO UPDATE SET balance = user_tokens.balance + EXCLUDED.balance, updated_at = now()
	      RETURNING balance`
	if err := tx.QueryRow(ctx, q, userID, token, delta).Scan(&balance); err != nil {
		return fmt.Errorf("grant token %s: %w", token, err)
	}
	if balance < 0 {
		// UPSERT 之后余额为负只可能是扣减超额。
		// CHECK 约束其实会先拦下来（整批失败，报 PG 的 check violation），
		// 这里的显式判断是**第二道**——它给出的是可读的中文错误。
		return fmt.Errorf("%w: %s 不足（需要 %d）", ErrBadInput, token, -delta)
	}
	if _, err := tx.Exec(ctx,
		`INSERT INTO wallet_flows (user_id, currency, delta, balance, reason, ref_id)
		 VALUES ($1,$2,$3,$4,$5,$6)`,
		userID, token, delta, balance, reason, refID); err != nil {
		return fmt.Errorf("insert token flow: %w", err)
	}
	return nil
}

// TokenBalance 读一件道具的余额（第 74 轮）。
//
// # 为什么需要它
//
// `Wallet` 结构体只有四个货币字段，而道具是**稀疏**的
// （没买过的道具连行都没有）。塞进 Wallet 会让每个玩家的
// 响应里都带着三个恒为 0 的字段，而它们又确实需要一个「读得到」的地方——
// 否则买了也看不见。
//
// 按需查询：只查调用方点名的那些 token。
func (s *Service) TokenBalance(ctx context.Context, userID int64, tokens []string) (map[string]int64, error) {
	out := map[string]int64{}
	if len(tokens) == 0 {
		return out, nil
	}
	rows, err := s.pool.Query(ctx,
		`SELECT token, balance FROM user_tokens WHERE user_id = $1 AND token = ANY($2)`,
		userID, tokens)
	if err != nil {
		return nil, fmt.Errorf("load tokens: %w", err)
	}
	defer rows.Close()
	for rows.Next() {
		var tok string
		var bal int64
		if err := rows.Scan(&tok, &bal); err != nil {
			return nil, err
		}
		out[tok] = bal
	}
	// 没买过的道具也要出现在结果里（余额 0），
	// 否则客户端无法区分「0 个」与「这个道具不存在」。
	for _, tok := range tokens {
		if _, ok := out[tok]; !ok {
			out[tok] = 0
		}
	}
	return out, rows.Err()
}

var walletColumns = map[string]string{
	"coin": "coin", "gem": "gem", "energy": "energy", "keys": "keys",
}

// walletTokens 是「道具类」货币的白名单（第 74 轮）。
//
// # 为什么它必须是一份**显式**名单，而不是「不在 walletColumns 里就算道具」
//
// 那样打错字会静默成功：`{"cino": 1}` 会被当成一种叫 `cino` 的道具，
// 扣钱照扣、发货照发，只是发的东西没人看得见。
// 而现在它会报「未知货币 "cino"」—— 那才是打错字时该看到的。
//
// 刻意的摩擦：新增一种道具要先在这里显式声明，
// 顺便回答「这个道具被谁消耗」。
//
// # 完整的货币全集（守卫：`TestEveryShopPayloadKeyIsSpendable`）
//
//	货币（user_wallets 列）：coin / gem / energy / keys
//	道具（user_tokens 行）  ：revive_token / mastery_reset / gem_wash_token
//
// ⚠️ 这三个道具**目前都没有消费端点**（没有「用掉复活币」的地方）。
// 本轮只修「买不到」这个更硬的问题 —— 买了至少能在背包里看到。
var walletTokens = map[string]bool{
	"revive_token":   true,
	"mastery_reset":  true,
	"gem_wash_token": true,
}

// isWalletToken 报告这个货币是不是道具。
func isWalletToken(currency string) bool { return walletTokens[currency] }

// LoadWallet 读取钱包并顺带做体力恢复。
func (s *Service) LoadWallet(ctx context.Context, userID int64) (Wallet, error) {
	// 体力每 6 分钟恢复 1 点，上限 120
	const energyPerInterval = 6 * time.Minute
	const energyMax = 120
	if _, err := s.pool.Exec(ctx, `
		UPDATE user_wallets
		   SET energy = LEAST($2::int,
		             energy + GREATEST(0, FLOOR(EXTRACT(EPOCH FROM (now() - energy_updated_at)) / $3))::int),
		       energy_updated_at = CASE
		           WHEN EXTRACT(EPOCH FROM (now() - energy_updated_at)) >= $3 THEN now()
		           ELSE energy_updated_at END,
		       updated_at = now()
		 WHERE user_id = $1`, userID, energyMax, int64(energyPerInterval/time.Second)); err != nil {
		return Wallet{}, fmt.Errorf("regen energy: %w", err)
	}

	var w Wallet
	err := s.pool.QueryRow(ctx,
		`SELECT coin, gem, energy, keys, energy_updated_at FROM user_wallets WHERE user_id = $1`, userID).
		Scan(&w.Coin, &w.Gem, &w.Energy, &w.Keys, &w.EnergyUpdatedAt)
	if errors.Is(err, pgx.ErrNoRows) {
		return Wallet{}, fmt.Errorf("%w: wallet for user %d", ErrNotFound, userID)
	}
	if err != nil {
		return Wallet{}, fmt.Errorf("load wallet: %w", err)
	}
	return w, nil
}

// --- 配置下发 ---

// GameConfig 是客户端一次拉取所需的全部内容。
type GameConfig struct {
	Version   int                     `json:"version"`
	Levels    []domain.GeneratedLevel `json:"levels"`
	Enemies   []domain.SeedEnemy      `json:"enemies"`
	Skills    []domain.SeedSkill      `json:"skills"`
	Composite []domain.SeedSkill      `json:"composite_skills"`
	Recipes   []domain.SeedRecipe     `json:"recipes"`
	Equipment []domain.SeedEquipment  `json:"equipment"`
	Gems      []domain.SeedGem        `json:"gems"`
	Qualities []QualityInfo           `json:"gem_qualities"`
	Skins     []domain.SeedSkin       `json:"skins"`
	Mastery   []domain.MasteryFamily  `json:"mastery_families"`
	Reactions []domain.ReactionSpec   `json:"reactions"`
	Chapters  []ChapterInfo           `json:"chapters"`
	RatingW   domain.RatingWeights    `json:"rating_weights"`
	// ScoreRules 是分数规则。客户端引擎的加分逻辑必须用这一份，
	// 不能自己硬编码 500/5000/100 —— 漂移了星级就会与实际表现不符，
	// 而两端各自都「自洽」，没有任何测试会发现。
	ScoreRules domain.ScoreRules `json:"score_rules"`
	// SkillRules 是技能升级规则（等级上限 / 每级伤害系数 / 费用基数）。
	//
	// 客户端的升级按钮要显示费用与「已满级」，战斗要按等级缩放伤害，
	// 所以这三个数**必须**下发而不是各端硬编码。
	// 照 ScoreRules 的先例：Go 侧是唯一定义点，客户端 `DEFAULT_SKILL_RULES`
	// 只是离线兜底，由 TestSkillRulesMatchServerContract 守卫两者不漂移。
	SkillRules domain.SkillRules `json:"skill_rules"`
	ServerTime time.Time         `json:"server_time"`
}

// QualityInfo 是宝石品质信息。
type QualityInfo struct {
	Name  string `json:"name"`
	Count int    `json:"affix_count"`
	Mult  int64  `json:"mult"`
}

// ChapterInfo 是章节元信息（客户端地图页用）。
type ChapterInfo struct {
	ID        int    `json:"id"`
	Name      string `json:"name"`
	Start     int    `json:"start_level"`
	End       int    `json:"end_level"`
	Terrain   string `json:"terrain_kind"`
	BossEnemy int    `json:"boss_enemy_id"`
}

// LoadGameConfig 组装配置下发包。
//
// 一次全量下发而非多次增量：实测未压缩 **167,124 字节**（163 KB），
// 其中 `levels` 一个字段就占 123,717 字节（74%）。
// 一次请求比七次增量更简单也更快，且关卡数据变化不频繁。
//
// ⚠️ 这里原来写的是「压缩后约 60KB」—— **那个数字是错的**（实测 163KB），
// 而且「压缩后」指的是压缩前就已存在的 JSON 体积，不是 gzip 之后的。
// 现已按实测数字改写，量级由 `internal/httpapi` 的
// `TestConfigVolumeIsKnown` 守着。
//
// 传输侧由 `middleware/compress` 兜住：gzip 后 23,964 字节（14.3%）。
func (s *Service) LoadGameConfig(ctx context.Context) GameConfig {
	cfg := GameConfig{
		Levels: domain.GenerateAllLevels(),
		// ⚠️ 分数规则必须下发，不能让客户端自己写死
		ScoreRules: domain.DefaultScoreRules(),
		// 平衡缩放在这里统一应用（见 content.go 的 EnemyHpScale /
		// SkillProjectileScale）。客户端引擎读到的就是缩放后的数值，
		// 而 content.go 的数据表保留可读的基准值。
		// ⚠️ 任何新的下发路径都必须用 Scale* 而不是直接引用 Seed*。
		Enemies:    domain.ScaleAllEnemies(),
		Skills:     domain.ScaleAllSkills(),
		Composite:  domain.ScaleAllCompositeSkills(),
		Recipes:    domain.SeedRecipes,
		Equipment:  domain.SeedEquipmentList,
		Gems:       domain.SeedGems,
		Skins:      domain.SeedSkins,
		Mastery:    domain.AllMasteryFamilies(),
		Reactions:  domain.AllReactionSpecs(),
		RatingW:    domain.DefaultRatingWeights(),
		SkillRules: domain.DefaultSkillRules(),
		ServerTime: time.Now(),
	}
	for _, q := range domain.GemQualities {
		cfg.Qualities = append(cfg.Qualities, QualityInfo{q.Name, q.Count, q.Mult})
	}
	for _, c := range domain.AllChapters() {
		cfg.Chapters = append(cfg.Chapters, ChapterInfo{
			ID: c.ID, Name: c.Name, Start: c.StartLevel, End: c.EndLevel,
			Terrain: c.TerrainKind, BossEnemy: c.BossEnemyID,
		})
	}
	// 版本号：客户端据此判断是否需要刷新配置。
	//
	// ⚠️ **当前这个机制是死的**，而且死因不止一处。查证结果：
	//
	//  1. 这里是**时间戳取模**，不是内容哈希 —— 每秒都在变。
	//  2. 客户端**根本没有读它**：`miniapp/src/store/game.ts` 的
	//     `loadConfig()` 只有 `if (config.value) return` 这个**内存内**缓存，
	//     `config.version` 在整个客户端代码里**零引用**。
	//     也就是说「版本没变就不用刷新」这个判断**从来没被实现过**。
	//  3. 配置里还有 `server_time: time.Now()`，所以**同一个配置在两次请求里
	//     本来就不相同** —— 客户端无法按内容做任何缓存。
	//     （`internal/httpapi` 的 `TestCompressedPayloadIsByteIdenticalToPlain`
	//     一开始就栽在这上面：它以为两次响应该逐字节相同，结果红了。）
	//
	// 因此每次冷启动都会实打实拉一次全量配置（163KB，gzip 后 24KB）。
	//
	// ## 要真正做缓存，需要三件事一起做
	//
	//   a. `version` 改成**内容哈希**（哈希时排除 `server_time`/`version` 自己）；
	//   b. 客户端把配置**持久化**（`uni.setStorageSync`），而不是只存内存；
	//   c. 客户端拿持久化的 `version` 与响应里的比对，相同就不更新。
	//
	// 只做 a 是「只写不读」—— 仍然没人读它，还给最热的端点加上
	// 「marshal 167KB + 哈希」的每请求 CPU。所以这里**故意保持现状**，
	// 只把事实记录下来。传输侧的成本已由 `middleware/compress` 压到 14.3%。
	cfg.Version = int(time.Now().Unix() % 100000)
	return cfg
}

// orEmptyMap 把 nil map 换成空 map。
//
// ## 为什么需要它
//
// `json.Marshal(map[string]int(nil))` 返回 `[]byte("null")`，
// 存进 jsonb 列就是 jsonb `null`（注意不是 SQL NULL）。
//
// 而 jsonb `null` 有两个实际危害（都实测过）：
//
//  1. `null <> '{}'::jsonb` 在 SQL 里求值为 **NULL 而不是 true**，
//     所以 `WHERE reactions_used <> '{}'` 会**静默排除**这些行。
//     实测 58 行里 8 行（13.8%）的反应分布没进后台看板。
//  2. `jsonb_each_text(jsonb 'null')` 直接**报错**
//     （「不能在非对象上调用」），任何 JSONB 聚合 SQL 都会崩。
//
// 空 map 序列化成 `{}`，两条都不会发生，且语义完全正确
// （「本局没用到任何反应」就是 `{}`）。
func orEmptyMap(m map[string]int) map[string]int {
	if m == nil {
		return map[string]int{}
	}
	return m
}

// orEmptySlice 把 nil slice 换成空 slice（`[]` 而不是 `null`）。
//
// 理由同 orEmptyMap。
func orEmptySlice[T any](s []T) []T {
	if s == nil {
		return []T{}
	}
	return s
}

// marshalJSON 是带错误传播的 JSON 编码辅助。
func marshalJSON(v any) ([]byte, error) {
	b, err := json.Marshal(v)
	if err != nil {
		return nil, fmt.Errorf("marshal json: %w", err)
	}
	return b, nil
}
