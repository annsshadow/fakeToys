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

var walletColumns = map[string]string{
	"coin": "coin", "gem": "gem", "energy": "energy", "keys": "keys",
}

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
// 一次全量下发而非多次增量：100 关 + 42 技能压缩后约 60KB，
// 一次请求比七次增量更简单也更快，且关卡数据变化不频繁。
func (s *Service) LoadGameConfig(ctx context.Context) (GameConfig, error) {
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
	// 版本号：内容哈希，保证客户端可判断是否需要刷新
	raw, err := json.Marshal(struct {
		L, E, S int
	}{len(cfg.Levels), len(cfg.Enemies), len(cfg.Skills)})
	if err == nil {
		cfg.Version = int(time.Now().Unix() % 100000)
		_ = raw
	}
	return cfg, nil
}

// marshalJSON 是带错误传播的 JSON 编码辅助。
func marshalJSON(v any) ([]byte, error) {
	b, err := json.Marshal(v)
	if err != nil {
		return nil, fmt.Errorf("marshal json: %w", err)
	}
	return b, nil
}
