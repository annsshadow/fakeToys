// Package service 承载业务编排：事务边界、跨表一致性、权限校验。
// 纯计算规则一律下沉到 domain，这里只处理 IO 与编排。
package service

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"sort"
	"strings"
	"sync"
	"sync/atomic"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgconn"
	"github.com/jackc/pgx/v5/pgxpool"

	"github.com/laiyipao/server/internal/auth"
	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/store"
)

// mustMarshalJSON 序列化一个「结构上不可能失败」的值。
//
// GameConfig 全是 string/number/bool/数组/map，json.Marshal 不可失败。
// 这里仍然 panic 而不是忽略错误：哈希的输入若是悄悄的空字节串，
// 所有配置都会算出同一个 version，缓存判断从此失效 —— 而那种失效
// **不报任何错**。宁可崩在第一次请求，也不要静默给出错误答案。
func mustMarshalJSON(v any) []byte {
	b, err := json.Marshal(v)
	if err != nil {
		panic(fmt.Sprintf("service: 静态配置不可序列化（哈希输入无效）: %v", err))
	}
	return b
}

// Errors 是服务层返回给上层的语义化错误。
var (
	ErrNotFound     = errors.New("资源不存在")
	ErrUnauthorized = errors.New("未认证或令牌已失效")
	ErrForbidden    = errors.New("无权访问")
	ErrBadInput     = errors.New("输入非法")
)

// Service 聚合全部依赖。
type Service struct {
	DB  *store.DB
	JWT *auth.Signer

	// cfg 是**配置快照**（原子指针，2026-10-29）。
	//
	// 此前是值字段 `Cfg config.Config`，`New` 时拷贝一份就再也不变 ——
	// 于是 `DASHBOARD_CACHE_TTL` / `LEADERBOARD_CACHE_TTL` / 结算重算模式
	// 这些「运维想改但不想重启」的旋钮，只能靠重启整进程生效。
	//
	// 改成原子指针后，`ReloadConfig` 可以整体换掉快照，而读方永远拿到一份
	// **完整**的配置（不会读到换到一半的字段）。与 `gate` 同一个理由与形状。
	//
	// ⚠️ 读配置一律走 `Cfg()` 取快照后**在同一次调用内复用**：不要写
	// `if s.Cfg().A > 0 { ... s.Cfg().A ... }` —— 两次 Load 之间可能发生重载。
	cfg atomic.Pointer[config.Config]

	pool *pgxpool.Pool

	// gate 是结算重算闸门（R1）。
	//
	// 由 initRecomputeGate 在 New 里回填，而不是直接内联构造 ——
	// 因为它的不符率回调要查库，而 s 在构造过程中还不存在。
	// gate 是**原子指针**（2026-10-21）。
	//
	// ⚠️ 此前它是普通字段，而 `PUT /admin/recomputes/mode` 会在**运行时**换它
	// —— 结算热路径同时在读它。裸指针的读写是数据竞争
	// （`go test -race` 能抓到）。
	//
	// 用 `atomic.Pointer` 的原因：换档是「整体换掉一个不可变对象」，
	// 读方**永远拿到一个完整的 gate**（不会出现「读到一半换掉的」）。
	gate atomic.Pointer[RecomputeGate]

	// 整局重算的分支计数（见 recompute_engine.go）。
	//
	// ⚠️ 为什么需要它们：台账 settle_recomputes **只记不符**，
	// 于是「校验正常工作，100% 通过」与「校验根本没跑」在台账上
	// 都是 0 行，两者无法区分 —— 而后者是灾难性的静默失败。
	// 这几个计数器提供「跑到了多少局」这个独立读数。
	//
	// 用 atomic 而不是普通 int64：SettleBattle 在并发请求里跑，
	// 读数在管理端接口里，两者并发。
	recomputeVerified  int64
	recomputeMismatch  int64
	recomputeCannotRun int64
	recomputeSkipped   int64

	// 进程内缓存（2026-10-23）。语义与取舍见 `ttlcache.go` 与 config 的同名字段。
	//
	// ⚠️ 它们**按 Service 实例隔离**：每个 `New` 出来的服务各自一份。
	// 这不是疏忽 —— 测试里每个用例新建一个 Service，于是「改完数据再读」
	// 天然读到新值；而线上只有一个实例，缓存才真正生效。
	// 若哪天改成全局单例，`dashboard_funnel_test` 那类「造数据→读看板」
	// 的用例会开始读到上一个用例的快照。
	dashboardCache ttlCache[Dashboard]
	// leaderboardCache 按 kind 分键。三个键在 New 里一次建好，
	// 之后**只读这个 map**（不再增删），所以不需要外层锁。
	leaderboardCache map[string]*ttlCache[[]LeaderboardEntry]
}

// leaderboardCacheKinds 是与 `leaderboardKinds` 同步的三个榜。
//
// 这里单独列一份是因为 `leaderboardKinds` 在 httpapi 包（校验用），
// 而缓存初始化发生在 service 包。两处都由
// `TestLeaderboardCacheCoversEveryKind` 对齐。
var leaderboardCacheKinds = []string{"power", "stage", "efficiency"}

// LeaderboardCacheKinds 返回缓存覆盖的榜名（副本）。
//
// 导出它是为了让 `httpapi` 的校验名单（`leaderboardKinds`）能与之对拍：
// 两处各自硬编码三个字符串，**加第四个榜时只改一处**，另一处不会报错 ——
// 症状是新榜没有缓存（性能问题），或者能查但路由拒绝（功能问题）。
// 判据见 `TestEveryLeaderboardKindHasCache`。
func LeaderboardCacheKinds() []string {
	out := make([]string, len(leaderboardCacheKinds))
	copy(out, leaderboardCacheKinds)
	return out
}

// New 构造服务。
func New(db *store.DB, cfg config.Config) *Service {
	s := &Service{
		DB:   db,
		JWT:  auth.NewSigner(cfg.JWTSecret, cfg.AccessTTL),
		pool: db.Pool,
	}
	s.cfg.Store(&cfg)
	s.leaderboardCache = make(map[string]*ttlCache[[]LeaderboardEntry], len(leaderboardCacheKinds))
	for _, kind := range leaderboardCacheKinds {
		s.leaderboardCache[kind] = &ttlCache[[]LeaderboardEntry]{}
	}
	s.initRecomputeGate()
	return s
}

// Cfg 返回当前配置快照。
//
// 热加载会整体替换这个指针，所以调用方应**取一次、用完**：在一次判断/计算里
// 反复 `s.Cfg().X` 会读到两个不同代际的配置。
//
// 返回指针（而非值拷贝）是为了让测试能就地改单个字段
// （`ts.Cfg().SettleRecomputeMode = ...`）——这是既有测试的写法。
func (s *Service) Cfg() *config.Config {
	if c := s.cfg.Load(); c != nil {
		return c
	}
	// 零值 `&Service{}`（测试里只调不碰库的方法，如 `LoadGameConfig`）没有快照。
	// 这里装一份**该实例专属**的零值配置，而不是返回共享包变量：
	//   - 返回共享变量会让某个用例的 `Cfg().X = ...` 泄漏到别的用例；
	//   - CAS 保证并发下只装一份，且所有读方拿到同一个指针。
	// 生产路径上 `New` 必装快照，走不到这里。
	var zero config.Config
	s.cfg.CompareAndSwap(nil, &zero)
	return s.cfg.Load()
}

// ReloadConfig 原子替换配置快照（SIGHUP 热加载入口）。
//
// # 生效范围
//
// 只影响**每次请求现读**的旋钮：`DASHBOARD_CACHE_TTL`、
// `LEADERBOARD_CACHE_TTL`、结算重算模式、微信端点/凭据等。
//
// # 不生效（改完仍需重启）
//
// 构造期固化的东西：数据库连接池大小与生命周期、`JWT` 签名器（密钥与
// 令牌有效期）、监听地址、路由级限流桶（在 `NewRouter` 时装配）。
// 这些若也做热加载，等于在运行中替换连接池/中间件链，收益远小于风险。
func (s *Service) ReloadConfig(cfg config.Config) {
	prev := s.cfg.Load()
	s.cfg.Store(&cfg)
	// 配置文件里**显式改了这一档**才重建 gate。
	//
	// 运行期切档的唯一真相是 gate（`PUT /admin/recomputes/mode` 只换 gate、
	// 不回写文件）。若每次重载都无条件按文件重建，一次「只改了 TTL」的
	// SIGHUP 就会把运维刚切过的档悄悄抹回去 —— 那是比不支持热加载更糟的
	// 形状：它看起来生效了，实际是被覆盖了。
	if prev == nil || prev.SettleRecomputeMode != cfg.SettleRecomputeMode {
		s.initRecomputeGateWith(cfg.SettleRecomputeMode)
	}
}

// FlushStatsCache 立刻失效看板与榜单缓存。
//
// ⚠️ **生产代码里没有调用方**，这一点必须说清（不要以为它接了运维按钮）：
//   - 榜单缓存由 `SettleBattle` 在结算成功后自动 flush（见那里的注释）；
//   - 看板缓存**故意不随写操作失效** —— 30 秒正是它存在的理由，
//     而结算在活跃期几秒一次，flush 它等于把缓存取消掉；
//   - 运维想让看板立刻刷新，手段是把 `DASHBOARD_CACHE_TTL` 设成 `0` 重启
//     （不需要发版），或等 30 秒。
//
// 保留它的唯一用途是**测试**（「造数据 → 读看板」需要新值）。
// 不做成 HTTP 端点是有意的：一个没人点的「刷新」按钮比没有更糟 ——
// 它会让运营以为看板是实时的。
func (s *Service) FlushStatsCache() {
	s.dashboardCache.flush()
	s.flushLeaderboards()
}

// flushLeaderboards 只失效榜单缓存。
//
// 与 `FlushStatsCache` 拆开是因为两者的**触发频率差两个数量级**：
// 结算（几秒一次）只该动榜单；看板（30 秒窗口）不该被结算打断。
func (s *Service) flushLeaderboards() {
	for _, c := range s.leaderboardCache {
		c.flush()
	}
}

// CacheStats 是进程内缓存的命中/未命中快照（getOrLoad 口径）。
type CacheStats struct {
	DashboardHits, DashboardMisses     int64
	LeaderboardHits, LeaderboardMisses int64
}

// CacheStats 汇总看板与三个榜单缓存的命中/未命中计数，供 /metrics 观测。
//
// 为什么值得暴露：缓存命中率是这两个缓存**唯一**的健康信号 ——
// 命中率骤降意味着 TTL 被设太小或击穿没被单飞兜住，而这两者都不会报错，
// 只是变慢。没有它，线上只能看到「看板接口偶发变慢」这种说不清的现象。
func (s *Service) CacheStats() CacheStats {
	var st CacheStats
	st.DashboardHits, st.DashboardMisses = s.dashboardCache.Stats()
	for _, c := range s.leaderboardCache {
		h, m := c.Stats()
		st.LeaderboardHits += h
		st.LeaderboardMisses += m
	}
	return st
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
// MaxNicknameLen 玩家昵称长度上限。
//
// # 为什么必须有一条上界
//
// `users.nickname` 是 TEXT（见 `00001_core.sql`），而请求体上限是
// `BodyLimit = 1MB` —— 也就是说**单个注册请求就能写入接近 1MB 的昵称**，
// 且没有任何东西在看。
//
// 它的后果不需要恶意触发：
//
//  1. 昵称会进**所有列表响应**：排行榜（`LeaderboardEntry.Nickname`）、
//     后台用户列表、战报、防线候选。一个 1MB 的昵称会让每一次
//     这些查询都多传 1MB —— 而坏数据一旦落库就永远在那儿。
//  2. 它是**玩家可控**字段，客户端改一行 JSON 就能写。
//
// 取 64 与 `MaxDefenseNameLen`（32）/ `MaxChallengeReplayHashLen`（64）
// 同一量级：微信昵称上限是 32 个字符，64 已是两倍余量。
// 只拒绝「传一段文章」，不拒绝任何真实昵称。
const MaxNicknameLen = 64

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
	// ⚠️ 只在校验通过后写入。此处**不截断** —— 静默截断会让客户端
	// 以为昵称设成功了（而它显示的是截断后的结果，玩家却不知道自己
	// 那串被改了）。与防线名（`MaxDefenseNameLen`）同一个口径。
	//
	// 注意这条校验只在**首次**创建时生效：`ON CONFLICT DO UPDATE`
	// 只更新 `last_login_at`，不覆盖昵称 —— 所以它拦的是唯一一条
	// 能把外部字符串写进 `nickname` 的路径。
	if n := len([]rune(nickname)); n > MaxNicknameLen {
		return TokenPair{}, fmt.Errorf("%w: 昵称超过 %d 字（收到 %d 字）",
			ErrBadInput, MaxNicknameLen, n)
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
	if !s.Cfg().WechatEnabled() {
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
	access, exp, err := s.JWT.Issue(userID, "user", "")
	if err != nil {
		return TokenPair{}, err
	}
	refreshPlain, refreshHash, err := auth.NewRefreshToken()
	if err != nil {
		return TokenPair{}, err
	}
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1,$2,$3)`,
		userID, refreshHash, time.Now().Add(s.Cfg().RefreshTTL)); err != nil {
		return TokenPair{}, fmt.Errorf("store refresh token: %w", err)
	}
	return TokenPair{
		AccessToken: access, RefreshToken: refreshPlain,
		ExpiresAt: exp, User: u,
	}, nil
}

// LogoutResult 是登出的结果。
type LogoutResult struct {
	// RevokedTokens 是被吊销的 refresh token 条数。
	RevokedTokens int64 `json:"revoked"`
	// GuestToken 是**轮换后**的游客凭证。
	//
	// 为什么登出要连它一起换：guest_token 与口令同级 ——
	// 实测给出同一个 guest_token 调 GuestLogin 就能拿到该账号的
	// access + refresh token。而它此前**永远不变、无法吊销**，
	// 于是「登出」对持有旧凭证的人毫无意义（30 天有效期 × 永久有效）。
	//
	// 客户端必须把这一枚**存下来**：不存的话，下一次冷启动会带着
	// 已作废的旧 token 去 GuestLogin → 匹配不到 → **建出一个新号**，
	// 玩家的进度看起来凭空消失了。
	// 那是比「凭证没轮换」更糟的后果，所以这一条必须与客户端改动同时上线。
	GuestToken string `json:"guest_token"`
}

// Logout 吊销该用户的全部未过期 refresh token，并**轮换**其游客凭证。
//
// # 为什么必须有它（本轮审计新增）
//
// 此前**没有任何登出端点**，后果不是「体验差一点」而是：
//
//   - 玩家换手机 / 令牌被读走 → 攻击者持有 refresh token 可续命
//     **30 天**（默认 RefreshTTL），服务端**没有**任何切断手段。
//   - 唯一会吊销 refresh token 的既有路径是「封禁玩家」
//     （`AdminSetUserStatus`），而那会把正常玩家一起踢下线 ——
//     用「封号」当「登出」，代价与意图完全不成比例。
//
// access token 在 TTL（默认 2h）内本就无法单条吊销，这是 JWT 的固有性质；
// 真正必须切断的是那条**长期**的 refresh token与那个**永久**的 guest_token。
//
// # 为什么吊销「全部」而不是「本次这一条」
//
// 客户端把令牌存在本地，登出的语义是「这个人不信任了」。
// 只吊销一条 = 被盗的那条依然有效 30 天。
// 全部吊销的最坏后果是「重新登一次」，而那正是登出的定义。
//
// # 为什么是幂等的
//
// 没有未过期令牌时返回 0 条而不是报错：重复点两次登出不应该报错，
// 且**不能**让「令牌已失效」与「吊销失败」在响应上可区分。
//
// # 为什么轮换在**同一事务**里
//
// 「吊销 refresh token」与「换掉 guest_token」必须同时成功或同时失败 ——
// 只做了前者的话，旧凭证仍能换出新会话，这次登出等于没做；
// 只做了后者的话，被盗的 refresh token 还活着。
// 两者同生共死才是「登出」这个语义。
func (s *Service) Logout(ctx context.Context, userID int64) (LogoutResult, error) {
	// 先备好新凭证：它在事务内才写，但在事务外生成 ——
	// crypto/rand 失败的话不应该先把事务开起来。
	fresh, err := auth.NewGuestToken()
	if err != nil {
		return LogoutResult{}, err
	}

	var out LogoutResult
	err = s.DB.Tx(ctx, func(tx pgx.Tx) error {
		tag, err := tx.Exec(ctx,
			`UPDATE refresh_tokens SET revoked_at = now()
			  WHERE user_id = $1 AND revoked_at IS NULL`, userID)
		if err != nil {
			return fmt.Errorf("logout revoke tokens: %w", err)
		}
		out.RevokedTokens = tag.RowsAffected()

		// ⚠️ UNIQUE(guest_token) 可能撞车：极低概率（24 字节随机），
		//   但撞上了就是 500。撞了就重试，而不是把罕见错误当不可能。
		//   上限 5 次足以把概率压到可以忽略；仍失败则如实报错。
		for attempt := 0; attempt < 5; attempt++ {
			_, err := tx.Exec(ctx,
				`UPDATE users SET guest_token = $2 WHERE id = $1`, userID, fresh)
			if err == nil {
				out.GuestToken = fresh
				return nil
			}
			if !isUniqueViolation(err) {
				return fmt.Errorf("logout rotate guest token: %w", err)
			}
			// 撞车了：换一枚再来
			next, gerr := auth.NewGuestToken()
			if gerr != nil {
				return gerr
			}
			fresh = next
		}
		return fmt.Errorf("logout: 生成游客凭证连续 5 次撞唯一键（概率上不可能，视为故障）")
	})
	if err != nil {
		return LogoutResult{}, err
	}
	return out, nil
}

// isUniqueViolation 报告 err 是否是 PostgreSQL 的 23505。
//
// ⚠️ 为什么不直接 `errors.As(err, &pgErr)` 再比 Code：
//
//	那样写更啰嗦，且**每次都要断言类型**。这里给个具名函数，
//	顺带把「只认 23505、不认其它约束」这件事写在一处 ——
//	签到路径（economy.go）也判过同一个码，两处应当看同一个函数。
func isUniqueViolation(err error) bool {
	var pgErr *pgconn.PgError
	return errors.As(err, &pgErr) && pgErr.Code == "23505"
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
	// 新号赠送三元素入门包：1 焰 ember（Lv1）/ 4 冰 dryice（Lv3）/ 7 电 arc（Lv6）。
	//
	// ⚠️ 此前这里发的是技能 1–4，而 2/3 按种子数据是燃油弹/温压弹 —— 全是焰系，
	// 与下面槽位注释宣称的"三元素保反应链"矛盾（ID 重排后没同步），新手实际
	// 三焰开局，I-1 的跨元素反应链根本触发不了。第 29 条做发放侧时一并纠正。
	//
	// 赠送超出新号等级（0 级）是礼包语义；第 4 个技能不再白送 ——
	// Lv2 解锁 slug（重炮弹，物理系）成为玩家第一次"升级解锁新技能"的体验。
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO user_skills (user_id, skill_id) VALUES ($1,1),($1,4),($1,7)
		 ON CONFLICT (user_id, skill_id) DO NOTHING`, userID); err != nil {
		return fmt.Errorf("init skills: %w", err)
	}

	// ⚠️ 必须同时给出战槽位，否则新玩家进游戏就是死局：
	// 开局拿到的 build.skills 全部 slot=-1，引擎没有任何技能可用，
	// 而唯一的配置入口（/me/loadout）在界面上又被"先打完一场"的引导挡在后面。
	//
	// 默认装 3 个：1 焰 / 4 冰 / 7 电 —— 三种不同元素，
	// 保证新号一进关就能触发反应链（I-1 的核心体验不能被"新号不会玩"埋掉）。
	// 装备校验（loadout.go「技能未拥有」）与 build 组装都以 user_skills 为准，
	// 所以槽位上的三个技能必须都在上面的赠送列表里 —— 缺一个就是幽灵槽位。
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO user_skill_slots (user_id, slot, skill_id) VALUES ($1,0,1),($1,1,4),($1,2,7)
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
	//
	// ⚠️ 第 101 轮：这里原本是 `context.Background()` —— **丢弃了调用方的 ctx**。
	//
	// # 后果
	//
	// 1. **客户端断开不会取消这条查询。** fiber 在客户端断开后取消请求 ctx，
	//    而这里换成了 Background，于是 pgx 会**把查询跑完**。
	//    一个慢查询 + 反复断开 = 连接池被占满，而每个占用者都不会超时退出。
	// 2. **任何超时都不生效。** `ctxBackground()` 那个辅助函数的存在
	//    说明这里**本来**是想传 ctx 的（HTTP 层包装成 HTTP 层用的）。
	// 3. `ctx` 参数因此**完全未被使用** —— Go 不报「未使用的参数」，
	//    所以它一直静默地错着。
	//
	// # 为什么单测抓不到
	//
	// `account_service_test.go` 等测试直接传 `context.Background()`，
	// 于是「传进去的 ctx 被丢掉」与「正常传递」**行为完全一致**。
	//
	// 判据必须构造一个**带 Deadline 且已过期**的 ctx：
	// 若实现忽略它，查询会成功；正确传递则必然返回「context deadline exceeded」。
	if err := s.pool.QueryRow(ctx,
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
// Wallet 是玩家的钱包。
//
// Tokens 是**道具**余额（第 74 轮起走独立的 user_tokens 表）。
//
// 为什么必须随钱包一起下发：道具要有消费闭环，
// 「客户端能看到自己有几个」是这个闭环的第一环。
// 第 150 轮之前只有 `TokenBalance` 这个 service 方法，而 HTTP 层
// 没有任何端点读它 —— 于是 `mastery_reset` 买了却不知道买了几个，
// 重铸按钮也就无从判断该不该亮。
//
// ⚠️ 只下发 `walletTokens` 白名单里的道具（显式名单，不是"全表"）：
// 新道具必须先在这里声明，顺便回答「它被谁消费」。
type Wallet struct {
	Coin            int64            `json:"coin"`
	Gem             int64            `json:"gem"`
	Energy          int64            `json:"energy"`
	Keys            int64            `json:"keys"`
	EnergyUpdatedAt time.Time        `json:"energy_updated_at"`
	Tokens          map[string]int64 `json:"tokens"`
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
// # 发放 / 扣减走**两条不同**的语句（审计轮修复）
//
// 原实现只有一条 UPSERT，对 `delta` 不区分正负：
//
//	INSERT INTO user_tokens (…) VALUES ($1,$2,$3)
//	ON CONFLICT (user_id, token) DO UPDATE SET balance = balance + $3
//
// 扣减时它**必然**撞上 `CHECK (balance >= 0)` —— CHECK 在语句级判定，
// 于是整笔事务带着 **SQLSTATE 23514** 回滚，`ErrBadInput` 那个可读分支
// （`if balance < 0`）**永远执行不到**。而 23514 不在
// `isSettleRejection` 名单里，`failErr` 兜底成 500「服务内部错误」。
//
// # 为什么原注释说「这条分支当前不可达」—— 它当时确实不可达，**现在不是了**
//
// 修复前那句话说对了一半：`grantToken` 过去确实只被 `delta > 0` 调用。
// 但第 148~151 轮接上了道具消费端点（`spendToken`）**并且**
// `AdminUpdateShopItem` 的 `validWalletCurrency` **明确允许**把道具名
// 写进商品 `price`。于是 `Buy` 的 `deduct` 就会带上负的道具 delta，
// 「不可达」变成了「运营改个价就能踩到」。
//
// 实测两条可达路径（都不是理论问题）：
//
//	POST /admin/users/:id/grant  {"currency":"revive_token","amount":-1}
//	  → ERROR: violates check constraint "chk_user_tokens_nonneg" (23514) → 500
//	运营把某商品 price 改成 {"mastery_reset":5}，玩家点购买
//	  → 同上 → 该商品**永久** 500，且钱已经扣了（事务回滚，钱没扣）
//
// # 修法：与货币路径、`spendToken` 完全对称
//
// 扣减改成**条件 UPDATE**（`balance >= $3 RETURNING balance`）：
// 余额不够时匹配不到行 → `pgx.ErrNoRows` → 可读的「道具不足」400。
// 这与 `grantWallet` 的货币负数分支、`spendToken` 是同一个模式。
//
// UPSERT 保留给 `delta > 0`：道具是稀疏的（没买过的连行都没有），
// UPSERT 一次搞定，不必先 SELECT 再分支。
func grantToken(ctx context.Context, tx pgx.Tx, userID int64, token string, delta int64, reason string, refID int64) error {
	if delta == 0 {
		return nil
	}
	var balance int64
	if delta > 0 {
		// 发放：UPSERT（道具行可能还不存在）。
		err := tx.QueryRow(ctx,
			`INSERT INTO user_tokens (user_id, token, balance)
			      VALUES ($1, $2, $3)
			      ON CONFLICT (user_id, token)
			      DO UPDATE SET balance = user_tokens.balance + EXCLUDED.balance, updated_at = now()
			      RETURNING balance`,
			userID, token, delta).Scan(&balance)
		if err != nil {
			return fmt.Errorf("grant token %s: %w", token, err)
		}
	} else {
		// 扣减：条件 UPDATE，余额不足即无行（与货币负数分支同构）。
		//
		// ⚠️ $3 传的是**要扣掉的绝对值 need**，SQL 写成 `balance - $3`。
		// 我第一版写的是 `balance + $3` 且直接传 delta（负数）——
		// 于是条件变成 `balance >= -4`，余额 3 也满足，扣成 -1，
		// 照样撞 CHECK。**判据必须落在「扣掉多少」而不是「变化了多少」上**：
		//   - `balance + (-4)` 与 `balance - 4` 数值上等价，
		//     但参与比较的一边符号相反，于是守卫条件完全失效。
		// 这个错误是 `TestGrantTokenPartialDebitWorks` 抓到的 ——
		// 只测「余额 0 时扣 1」的用例会让它绿。
		need := -delta
		err := tx.QueryRow(ctx,
			`UPDATE user_tokens SET balance = balance - $3, updated_at = now()
			  WHERE user_id = $1 AND token = $2 AND balance >= $3
			  RETURNING balance`,
			userID, token, need).Scan(&balance)
		if err != nil {
			if errors.Is(err, pgx.ErrNoRows) {
				return fmt.Errorf("%w: %s 不足（需要 %d）", ErrBadInput, token, need)
			}
			return fmt.Errorf("grant token %s: %w", token, err)
		}
	}
	if _, err := tx.Exec(ctx,
		`INSERT INTO wallet_flows (user_id, currency, delta, balance, reason, ref_id)
		 VALUES ($1,$2,$3,$4,$5,$6)`,
		userID, token, delta, balance, reason, refID); err != nil {
		return fmt.Errorf("insert token flow: %w", err)
	}
	return nil
}

// spendToken 在事务内**消耗**一件道具（第 2026-10 轮，道具消费端点接入）。
//
// # 为什么不用 `grantWallet` 的负数路径
//
// `user_tokens` 有 `CHECK (balance >= 0)` 约束。`grantToken` 走的是
// `INSERT ... ON CONFLICT DO UPDATE`，把 `0 - 1` 写进去会**先触发 CHECK 违例**
// （PG 报错 23514），而不是给人一句可读的「道具不足」。
// 货币侧 `grantWallet` 的负数分支用 `WHERE coin >= $3` 提前拦，道具侧要对称，
// 所以消费走**条件 UPDATE**（余额 < 1 时直接 `ErrNoRows`），不靠约束兜底。
//
// # 语义
//
// 消耗 1 件 `token`。余额不足（没有行、或 balance < 1）返回 `ErrBadInput`。
// 成功后写一条 `wallet_flows`（currency = 道具名），与 `grantToken` 的入账流水
// 同一条流，运营看板按 currency 聚合时「进了多少 / 花了多少」才对称。
func spendToken(ctx context.Context, tx pgx.Tx, userID int64, token string, reason string, refID int64) error {
	var balance int64
	err := tx.QueryRow(ctx,
		`UPDATE user_tokens SET balance = balance - 1, updated_at = now()
		  WHERE user_id = $1 AND token = $2 AND balance >= 1
		  RETURNING balance`,
		userID, token).Scan(&balance)
	if err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return fmt.Errorf("%w: 道具 %s 不足（需要 1）", ErrBadInput, token)
		}
		return fmt.Errorf("spend token %s: %w", token, err)
	}
	if _, err := tx.Exec(ctx,
		`INSERT INTO wallet_flows (user_id, currency, delta, balance, reason, ref_id)
		 VALUES ($1,$2,$3,$4,$5,$6)`,
		userID, token, -1, balance, reason, refID); err != nil {
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
// grantToken 归还一个道具（`spendToken` 的逆操作）。
//
// ⚠️ 2026-10-21（P4-5）：放弃战斗时退复活币用的就是它。
//
// 为什么需要一个「归还」而不是直接 UPDATE：
//   - 余额有 `CHECK (balance >= 0)` 约束；
//   - **流水必须留痕** —— 道具的每次增减都要能对账。
//     少写一条流水，运营看到的道具余额就与消耗对不上，
//     而「道具凭空多了」和「道具凭空少了」一样难查。
func (s *Service) grantToken(ctx context.Context, tx txType, userID int64, token string, amount int64, reason string, refID int64) error {
	if amount <= 0 {
		return nil
	}
	var balance int64
	if err := tx.QueryRow(ctx,
		`UPDATE user_tokens SET balance = balance + $3, updated_at = now()
		 WHERE user_id = $1 AND token = $2
		 RETURNING balance`, userID, token, amount).Scan(&balance); err != nil {
		return fmt.Errorf("grant token %s: %w", token, err)
	}
	if _, err := tx.Exec(ctx,
		`INSERT INTO wallet_flows (user_id, currency, delta, balance, reason, ref_id)
		 VALUES ($1,$2,$3,$4,$5,$6)`,
		userID, token, amount, balance, reason, refID); err != nil {
		return fmt.Errorf("insert token grant flow: %w", err)
	}
	return nil
}

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
// 消费面（2026-10 第 148–151 轮全部接通）：
//
//   - mastery_reset     → POST /mastery/reset（ResetMastery 清空专精，spendToken 扣 1）
//   - revive_token      → StartBattle 开战扣 1 枚，本关 base_hp ×1500‰
//     （**战前加成**，不是局内复活：复活要动战斗结算的重放锚点 I-6）
//   - gem_wash_token    → POST /me/gems/:id/wash（重 roll 词条，品质不变）
//
// 发放面：三者的**唯一**途径都是商城 `shop_items`（签到/任务/成就/兑换码都不发）。
// 客户端此前没有商城页，所以「服务端有扣减逻辑」不等于「玩家能用」——
// 第 151 轮补上 `pages/shop` 后这三条才是真正的机制闭环。
var walletTokens = map[string]bool{
	"revive_token":   true,
	"mastery_reset":  true,
	"gem_wash_token": true,
}

// isWalletToken 报告这个货币是不是道具。
func isWalletToken(currency string) bool { return walletTokens[currency] }

// 体力恢复参数。提到包级：LoadWallet（结算并写库）与 LoadWalletReadOnly
// （只投影不写库）必须用同一组数，两处各写一份就会漂。
const (
	walletEnergyPerInterval = 6 * time.Minute
	walletEnergyMax         = 120
)

// LoadWallet 读取钱包并顺带做体力恢复。
func (s *Service) LoadWallet(ctx context.Context, userID int64) (Wallet, error) {
	const energyPerInterval = walletEnergyPerInterval
	const energyMax = walletEnergyMax
	//
	// ⚠️ 第 86 轮：`energy_updated_at` 必须按**整间隔**推进，而不是推到 now()。
	//
	// # 缺陷：每次推进都丢掉不足一格的时间
	//
	// 原写法：
	//
	//	energy_updated_at = CASE WHEN elapsed >= interval THEN now() ELSE energy_updated_at END
	//
	// `now()` 落在下一次读取的时刻上，于是**两次读取之间不足一格的余数
	// 被永久丢弃**。
	//
	// 客户端每次进页面都 `fetchWallet`，所以读取间隔（3~5 分钟）
	// **短于**回复间隔（6 分钟）—— 正是最容易丢的形态。
	//
	// 实测（每 5 分钟读一次，一小时的账）：
	//
	//	t=0     at=0
	//	t=300   +0  at=0      （300 < 360，不推进）
	//	t=600   +1  at=600    ← 余数 240 被吞
	//	t=900   +0  at=600
	//	t=1200  +1  at=1200   ← 又吞 240
	//	…
	//	3600 分钟内共 12 次读取 → 只回 6 点
	//
	// 而 1 小时应当回 10 点 —— **少回 40%**。
	//
	// # 修法
	//
	// 推进量与加点量用**同一个**整格数，且推进量是 `interval` 的整数倍：
	//
	//	energy += k        （k = FLOOR(elapsed / interval)）
	//	energy_updated_at += k * interval
	//
	// 这样余数留在 `energy_updated_at` 里，下一次继续累积，
	// 长时间看总量精确等于 `FLOOR(总时长 / interval)`。
	//
	// 顺带：`LEAST($2::int, …)` 之外还要保证**扣能量时不回推时间戳** ——
	// 原写法本来就没回推，保持不变。
	const energyIntervalSecs = int64(energyPerInterval / time.Second)
	// 2026-10-25（R3）：体力恢复 UPDATE 与随后的读钱包 SELECT **合成一条**
	// （UPDATE ... RETURNING）。两者本就是同一行、同一时刻的状态，
	// 分两次往返纯属历史拆分；本函数挂在每次进页面都会拉的
	// fetchWallet 上，省下的是最热路径的一次往返。
	//
	// 2026-10-29：**没有恢复量时不写库**。
	//
	// 原写法无条件 `SET updated_at = now()`，于是即使 `n = 0`（距上次恢复
	// 不足一格）也每次产生一个新元组 + WAL + 行锁。客户端 3~5 分钟拉一次钱包，
	// 而恢复间隔是 6 分钟 —— 绝大多数读取都是「n = 0」，等于把最热的读端点
	// 变成持续的写放大源（`user_wallets` 的 dead tuple 与 autovacuum 压力）。
	//
	// 现在用 `cur` 先算 n，只有 `n > 0` 才走 `regen` 分支写库；`n = 0` 直接
	// 返回 `cur` 的现有值。仍是**一条语句、一次往返**，语义逐位保持：
	//   - 行不存在 → cur 为空 → 两个分支都无行 → ErrNoRows → ErrNotFound，
	//     与原「UPDATE 命中 0 行」落到同一个分支；
	//   - 有恢复 → 返回 RETURNING 的**更新后**值（energy 与 energy_updated_at
	//     都含本次恢复），与原 SELECT 读到的一致；
	//   - `updated_at` 的语义修正为「数据真的变了」——读一次不再算一次修改。
	//
	// 满体力时仍然推进时间戳（`n > 0` 就写）：否则玩家消耗体力后会按
	// 「上次满体力那一刻」起算，瞬间回满。
	var w Wallet
	err := s.pool.QueryRow(ctx, `
		WITH cur AS (
		  SELECT coin, gem, energy, keys, energy_updated_at,
		         GREATEST(0, FLOOR(EXTRACT(EPOCH FROM (now() - energy_updated_at)) / $3))::int AS n
		    FROM user_wallets WHERE user_id = $1
		), regen AS (
		  UPDATE user_wallets w
		     SET energy = LEAST($2::int, w.energy + cur.n),
		         energy_updated_at = w.energy_updated_at
		                         + make_interval(secs => (cur.n * $3)::double precision),
		         updated_at = now()
		    FROM cur
		   WHERE w.user_id = $1 AND cur.n > 0
		  RETURNING w.coin, w.gem, w.energy, w.keys, w.energy_updated_at
		)
		SELECT coin, gem, energy, keys, energy_updated_at FROM regen
		UNION ALL
		SELECT coin, gem, energy, keys, energy_updated_at FROM cur WHERE cur.n = 0`,
		userID, energyMax, energyIntervalSecs).
		Scan(&w.Coin, &w.Gem, &w.Energy, &w.Keys, &w.EnergyUpdatedAt)
	if errors.Is(err, pgx.ErrNoRows) {
		return Wallet{}, fmt.Errorf("%w: wallet for user %d", ErrNotFound, userID)
	}
	if err != nil {
		// 恢复与读取已合并，故错误可能来自任一侧——消息里点明，便于排查。
		return Wallet{}, fmt.Errorf("regen+load wallet: %w", err)
	}
	// 道具余额：未持有过的补 0（TokenBalance 内部按传入列表补齐）。
	//
	// 为什么 0 也要出现：客户端要靠「键在不在」区分「0 个」与
	// 「这个道具不存在」，而后者只可能是服务端漏了下发。
	w.Tokens, err = s.TokenBalance(ctx, userID, WalletTokenKinds)
	if err != nil {
		return Wallet{}, err
	}
	return w, nil
}

// LoadWalletReadOnly 读钱包**不写库**，体力按同一公式**投影**出「此刻应有的值」。
//
// 为什么需要它（2026-10-25，R3，LY-GAP-2）：
//
// 后台玩家详情是 GET，却经 LoadWallet 触发了体力 regen 的 UPDATE——
// 读端点带写副作用有三个具体坏处：
//  1. `readonly` 角色的运营账号本应完全无写权（requireWritable 的语义），
//     但这个写发生在 GET 里，绕过了那层门；
//  2. 只读副本/读路由下会失败（写语句打不出去）；
//  3. 审计面「谁改了什么」出现没人主动触发的写入。
//
// 语义选择：**投影**而不是「直接显示落库值」——否则运营看到的是
// 「玩家说还有 20 点，后台显示 5 点」这种对不上账的形态。投影用的是
// LoadWallet 同一组常量与同一公式（不结算、不推进 energy_updated_at），
// 所以显示值与玩家侧结算后会看到的一致。
//
// 不返回 Tokens：调用方（后台详情）只用 coin/gem/energy/keys，
// 省掉一次 TokenBalance 往返。
func (s *Service) LoadWalletReadOnly(ctx context.Context, userID int64) (Wallet, error) {
	intervalSecs := int64(walletEnergyPerInterval / time.Second)
	var w Wallet
	err := s.pool.QueryRow(ctx, `
		SELECT coin, gem,
		       LEAST($2::int, energy + GREATEST(0, FLOOR(EXTRACT(EPOCH FROM (now() - energy_updated_at)) / $3))::int) AS energy,
		       keys, energy_updated_at
		  FROM user_wallets WHERE user_id = $1`,
		userID, walletEnergyMax, intervalSecs).
		Scan(&w.Coin, &w.Gem, &w.Energy, &w.Keys, &w.EnergyUpdatedAt)
	if errors.Is(err, pgx.ErrNoRows) {
		return Wallet{}, fmt.Errorf("%w: wallet for user %d", ErrNotFound, userID)
	}
	if err != nil {
		return Wallet{}, fmt.Errorf("load wallet (read-only): %w", err)
	}
	return w, nil
}

// WalletTokenKinds 是随钱包下发的道具白名单。
//
// 从 `walletTokens` 派生而不是再抄一份名单：两处名单一旦漂移，
// 商店里能买到的东西在钱包里看不到，而**没有任何测试会红**。
//
// 排序只为让下发顺序稳定（客户端落缓存时字节可比对），不承载语义。
var WalletTokenKinds = func() []string {
	out := make([]string, 0, len(walletTokens))
	for k := range walletTokens {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}()

// --- 配置下发 ---

// GameConfig 是客户端一次拉取所需的全部内容。
//
// ⚠️ **Version 与 ServerTime 语义相反，别混**：
//   - `Version` 是**内容哈希**，只在配置内容真的变了才变。
//     客户端据此判断本地缓存还能不能用（见 LoadGameConfig 的注释）。
//   - `ServerTime` 每次请求都可能不同（服务端当时挂钟），
//     客户端拿它校准本地时钟偏移。
//
// 曾经 `Version` 是 `time.Now().Unix() % 100000` —— 每秒都在变，
// 「版本没变就不用刷新」这个判断从来没被实现过。现在是 SHA-256 前 16 位十六进制。
type GameConfig struct {
	Version string `json:"version"`
	// RequireLogin 告诉客户端「本环境要求账号登录」。
	//
	// ⚠️ 它**不在**内容哈希里（version 只覆盖游戏内容），
	// 所以客户端持久化的旧配置副本里这个值可能是过期的。
	// 因此客户端不能只靠它做判断 —— 服务端在 /auth/guest 上还会
	// 硬拦一道（403 login_required），那条才是权威判据。
	// 这个字段只是让客户端**少走一次冤枉路**。
	RequireLogin bool `json:"require_login"`
	// Levels 是**列表摘要**（不含 waves）。
	//
	// 为什么不是 GeneratedLevel：`waves` 占这个字段的 75%（实测 93,338 字节），
	// 而客户端只有「防线挑战的标准战场」一处要它 —— 那一处走
	// `GET /api/v1/levels/:id` 按需取。详见 domain.LevelSummary 的注释。
	Levels    []domain.LevelSummary  `json:"levels"`
	Enemies   []domain.SeedEnemy     `json:"enemies"`
	Skills    []domain.SeedSkill     `json:"skills"`
	Composite []domain.SeedSkill     `json:"composite_skills"`
	Recipes   []domain.SeedRecipe    `json:"recipes"`
	Equipment []domain.SeedEquipment `json:"equipment"`
	Gems      []domain.SeedGem       `json:"gems"`
	Qualities []QualityInfo          `json:"gem_qualities"`
	Skins     []domain.SeedSkin      `json:"skins"`
	Mastery   []domain.MasteryFamily `json:"mastery_families"`
	Reactions []domain.ReactionSpec  `json:"reactions"`
	Chapters  []ChapterInfo          `json:"chapters"`
	RatingW   domain.RatingWeights   `json:"rating_weights"`
	// ScoreRules 是分数规则。客户端引擎的加分逻辑必须用这一份，
	// 不能自己硬编码 500/5000/100 —— 漂移了星级就会与实际表现不符，
	// 而两端各自都「自洽」，没有任何测试会发现。
	ScoreRules domain.ScoreRules `json:"score_rules"`
	// DropRates 是掉落表（2026-10-21 P4-6）。
	//
	// 此前它只在服务端，客户端**不知道通关会返还 5 点体力** ——
	// 于是选关页显示「体力消耗 6」，实际净消耗是 1。
	// 玩家看到的数与实际付出的数不是一回事。
	DropRates domain.DropRates `json:"drop_rates"`
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
//
// ## 第 150 轮：`version` 从「每秒都变的时间戳」改成「内容哈希」
//
// 原实现是 `int(time.Now().Unix() % 100000)`，于是：
//
//  1. 它**从不稳定**，客户端无法据此判断「内容变没变」；
//  2. 客户端**根本不读它**（`store/game.ts` 的 `loadConfig()` 只有
//     `if (config.value) return` 这个内存内缓存，`config.version` 全仓库零引用）——
//     也就是说「版本没变就不用刷新」这个判断从来没被实现过；
//  3. 配置里还有 `server_time: time.Now()`，两次响应本来就不相同。
//
// 现在三件事一起做了（只做 a 是「只写不读」，还会给最热的端点
// 加上「marshal 167KB + 哈希」的每请求 CPU，所以这里连缓存一起做）：
//
//	a. `version` = 静态内容的 SHA-256 前 16 位十六进制（哈希时
//	   `version`/`server_time` 取零值，天然被排除）；
//	b. 静态部分进程内只组装一次（`staticConfig`）——
//	   原来每个请求都重跑 `GenerateAllLevels()`，那是白花的 CPU；
//	c. 客户端把配置**持久化**（`uni.setStorageSync`）并按 version 比对，
//	   条件请求见 `httpapi.getConfig` 的 `?if_version=`。
//
// 返回的 GameConfig 是**浅拷贝**：slice 头指向缓存里的同一批数据。
// 因此调用方必须把它当只读（`/config` 的 handler 只 marshal）。
// 若将来有调用方要改配置内容，必须先改成深拷贝 —— 否则会污染缓存，
// 而且症状是「配置在运行中慢慢变了」，极难排查。
func (s *Service) LoadGameConfig() GameConfig {
	cfg := staticConfig()
	// 唯一每请求不同的字段：服务端当时挂钟，客户端拿它校准本地时钟偏移。
	cfg.ServerTime = time.Now()
	// ⚠️ 这一条**依赖实例**（config 里来的开关），所以不能放进 staticConfig
	// —— 那个 OnceValue 是全局静态的，与 Service 实例无关。
	cfg.RequireLogin = s.Cfg().PlayerLoginRequired
	return cfg
}

// staticConfig 是 /config 的静态部分，进程内只组装一次。
//
// 它 99% 的内容与请求无关（100 关由 LCG 生成 + 22 敌人 + 42 技能
// + 18 配方 + 18 装备 + 8 宝石 + 6 皮肤 + 96 专精节点 + 5 反应），
// 而 /config 是最热的端点（每次冷启动一次，H5 刷新页面也是）。
// 原来每个请求都重新生成一遍 —— 那是纯浪费。
//
// 用 `sync.OnceValue` 而不是 `Service` 的字段：本函数一次库都不查、
// 一个配置项都不读，是**全局静态数据**，与 Service 实例无关。
// 挂在字段上还得处理「Service 被拷贝时 Once 被一起拷」的问题。
var staticConfig = sync.OnceValue(func() GameConfig {
	cfg := GameConfig{
		Levels: domain.SummarizeAll(domain.GenerateAllLevels()),
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
		// 2026-10-21（P4-6）：掉落表随 config 下发，
		// 客户端才能算出「通关净消耗多少体力」（成本 - 通关返还）。
		DropRates: domain.DefaultDropRates(),
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
	// 内容哈希：Version 与 ServerTime 都还是零值，所以「哈希时排除它们」
	// 这件事由零值天然完成，不需要额外构造一份剔除后的拷贝。
	cfg.Version = configVersionHash(cfg)
	return cfg
})

// configVersionHash 对配置内容取内容哈希。
//
// `GameConfig.Version` 与 `ServerTime` 在调用点都必须是零值 ——
// 它们是唯一随时间变的两个字段，零值即「被排除」。
//
// ⚠️ 这个函数是 `version` 语义的全部：一旦有人在里面加入
// 时间戳/随机数/指针地址，「版本没变就不用刷新」立刻失效，
// 而失效方式是**客户端永远拿到新版本**（性能回归）。
// `TestConfigVersionIsStableAndContentDerived` 分别钉住这两半边。
func configVersionHash(cfg GameConfig) string {
	// 取 16 位十六进制而不是全文：64 位空间下，配置内容从「发布一次」
	// 变成「不同」的概率可以忽略；而短版本号在日志/客户端缓存里可读。
	// 判断口径是**字符串全等**，所以位数短不影响正确性。
	sum := sha256.Sum256(mustMarshalJSON(cfg))
	return hex.EncodeToString(sum[:8])
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
