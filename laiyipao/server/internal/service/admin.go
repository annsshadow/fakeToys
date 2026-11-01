package service

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/auth"
	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/seeder"
)

// HashRefreshToken 重导出：会话行以 SHA-256 摘要落库（与 refresh token 同形态）。
func HashRefreshToken(plain string) string { return auth.HashRefreshToken(plain) }

// txType 是 pgx.Tx 的最小别名，让 service 各函数能同时接受事务与连接池上下文。
type txType = pgx.Tx

// --- 管理员鉴权 ---

// AdminUser 是后台账号。
type AdminUser struct {
	ID       int64  `json:"id"`
	Username string `json:"username"`
	Role     string `json:"role"`
}

// AdminTokenPair 是后台登录结果。
//
// ⚠️ 第 155 轮删掉了曾经的 `Session` 字段：那枚明文会话令牌**没有签发方
// 消费它**（客户端只用 access_token），留着它等于谎称后台有「另一把
// 会话钥匙」。会话与令牌的绑定改成 JWT 载荷里的 sid 明文（摘要落库），
// 见 AdminLogin / ResolveAdmin。
type AdminTokenPair struct {
	AccessToken string    `json:"access_token"`
	ExpiresAt   string    `json:"expires_at"`
	Admin       AdminUser `json:"admin"`
}

// AdminLogin 管理员登录。
func (s *Service) AdminLogin(ctx context.Context, username, password string) (AdminTokenPair, error) {
	var a AdminUser
	var hash string
	var status int
	err := s.pool.QueryRow(ctx,
		`SELECT id, username, role, status, password_hash FROM admin_users WHERE username = $1`, username).
		Scan(&a.ID, &a.Username, &a.Role, &status, &hash)
	if err != nil {
		if err == pgx.ErrNoRows {
			// 用户不存在与密码错误返回同一错误，避免用户名枚举
			return AdminTokenPair{}, fmt.Errorf("%w: 用户名或密码错误", ErrUnauthorized)
		}
		return AdminTokenPair{}, fmt.Errorf("load admin: %w", err)
	}
	if status != 1 {
		return AdminTokenPair{}, fmt.Errorf("%w: 账号已停用", ErrForbidden)
	}
	if err := VerifyPassword(hash, password); err != nil {
		return AdminTokenPair{}, fmt.Errorf("%w: 用户名或密码错误", ErrUnauthorized)
	}

	// 会话与令牌的绑定：明文 sid 进 JWT 载荷，SHA-256 摘要落库（与 refresh
	// token 同一形态：数据库被拖库也无法直接签出可用的管理员令牌）。
	// 第 155 轮起 ResolveAdmin 每次都查这一行 —— 登出/过期后立刻失效。
	sessionPlain, sessionHash, err := NewAdminSession()
	if err != nil {
		return AdminTokenPair{}, err
	}
	access, exp, err := s.JWT.Issue(a.ID, "admin", sessionPlain)
	if err != nil {
		return AdminTokenPair{}, err
	}
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO admin_tokens (admin_id, token_hash, expires_at) VALUES ($1,$2,$3)`,
		a.ID, sessionHash, timeNow().Add(s.Cfg().AccessTTL)); err != nil {
		return AdminTokenPair{}, fmt.Errorf("store admin session: %w", err)
	}
	if _, err := s.pool.Exec(ctx,
		`UPDATE admin_users SET last_login_at = now() WHERE id = $1`, a.ID); err != nil {
		return AdminTokenPair{}, err
	}
	return AdminTokenPair{
		AccessToken: access,
		ExpiresAt:   exp.UTC().Format(timeFormat), Admin: a,
	}, nil
}

// ResolveAdminSession 解析管理员 access token，并**一次查询**带回会话有效性
// 与角色。
//
// ⚠️ 必须查 admin_users.status。
// 只验签名的话，管理员离职/降权后，他手上的 token 在整个 access TTL
// （默认 2h）内仍能发币、封禁玩家、改商城价格 —— 运营的止损手段失效。
// 全项目只有 AdminMe 带 `AND status = 1`，于是会出现"只有 /admin/me 401，
// 其余管理端点照常可用"的诡异现象。
//
// ⚠️ 第 155 轮起还必须**每次**查 admin_tokens 的会话行（sid 摘要）：
// 会话被登出吊销（AdminLogout）或落库的 expires_at 过期后，
// 一枚**签名仍然合法**的 JWT 也必须立刻失效 —— 这是把「登出」
// 从纯前端清 localStorage 升级成服务端真切断（与玩家侧 refresh
// token 吊销同形态）。
//
// ⚠️ 2026-10-25（R3）：token 行与 admin_users 行**合并成一条 JOIN**。
// 原先是 requireAdmin 里三次往返（admin_tokens → admin_users.status →
// admin_users.role），每次后台请求都付；role 本来就是同一行的列，
// 分两次查纯属历史拆分。判定顺序逐位保持：会话存在 → 未吊销 → 未过期 →
// 账号未停用（FK ON DELETE CASCADE 保证 token 行的 admin 必存在，
// 故「查不到行」与「会话不存在」同义）。
func (s *Service) ResolveAdminSession(ctx context.Context, token string) (int64, string, error) {
	claims, err := s.JWT.Verify(token)
	if err != nil {
		return 0, "", fmt.Errorf("%w: %s", ErrUnauthorized, err)
	}
	if claims.Kind != "admin" {
		return 0, "", fmt.Errorf("%w: 令牌类型不匹配", ErrUnauthorized)
	}
	if claims.Sid == "" {
		// 第 155 轮之前签发的存量管理员令牌没有 sid —— 它们随 2h TTL
		// 自然消亡，不需要数据迁移；新令牌由 AdminLogin 签发、恒带 sid。
		return 0, "", fmt.Errorf("%w: 管理员会话不存在", ErrUnauthorized)
	}
	var revoked bool
	var expiresAt time.Time
	var status int
	var role string
	// 一条 JOIN 取齐：会话行 + 账号状态 + 角色。WHERE 带 admin_id 而非
	// claims.Sub —— token_hash 已唯一确定会话，用会话行的 admin_id 更诚实
	// （token 里的 Sub 只是签发时的快照）。
	if err := s.pool.QueryRow(ctx,
		`SELECT t.revoked_at IS NOT NULL, t.expires_at, u.status, u.role
		   FROM admin_tokens t
		   JOIN admin_users u ON u.id = t.admin_id
		  WHERE t.token_hash = $1`,
		HashRefreshToken(claims.Sid)).Scan(&revoked, &expiresAt, &status, &role); err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return 0, "", fmt.Errorf("%w: 管理员会话不存在", ErrUnauthorized)
		}
		return 0, "", fmt.Errorf("查管理员会话: %w", err)
	}
	if revoked {
		return 0, "", fmt.Errorf("%w: 会话已登出吊销", ErrUnauthorized)
	}
	if !expiresAt.After(timeNow()) {
		return 0, "", fmt.Errorf("%w: 会话已过期", ErrUnauthorized)
	}
	if status != 1 {
		return 0, "", fmt.Errorf("%w: 管理员已停用", ErrForbidden)
	}
	return claims.Sub, role, nil
}

// ResolveAdmin 只要身份、不要角色时的入口（委托 ResolveAdminSession，零额外查询）。
//
// 保留它是为了语义分层：大量测试断言的是「这枚 token 能不能用」
// （吊销/过期/停用/类型不符），与角色无关；它们不必为了多一个返回值改写。
// 需要角色的调用方（requireAdmin）走 ResolveAdminSession。
func (s *Service) ResolveAdmin(ctx context.Context, token string) (int64, error) {
	id, _, err := s.ResolveAdminSession(ctx, token)
	return id, err
}

// AdminLogout 吊销该管理员的**全部**后台会话（幂等，返回吊销条数）。
//
// 与玩家侧 Logout（吊销全部 refresh token）同口径：登出的语义是
// 「切断这个人此刻手上的所有凭证」，而不是「只切当前这一个」。
// 运营在 A 设备登出，B 设备上 2h 内还有效的会话一并失效 ——
// 重新登录即恢复，代价远低于「忘了那台设备还开着后台」。
func (s *Service) AdminLogout(ctx context.Context, adminID int64) (int, error) {
	res, err := s.pool.Exec(ctx,
		`UPDATE admin_tokens SET revoked_at = now()
		 WHERE admin_id = $1 AND revoked_at IS NULL`, adminID)
	if err != nil {
		return 0, fmt.Errorf("revoke admin sessions: %w", err)
	}
	return int(res.RowsAffected()), nil
}

// AdminMe 返回当前管理员信息。
func (s *Service) AdminMe(ctx context.Context, adminID int64) (AdminUser, error) {
	var a AdminUser
	err := s.pool.QueryRow(ctx,
		`SELECT id, username, role FROM admin_users WHERE id = $1 AND status = 1`, adminID).
		Scan(&a.ID, &a.Username, &a.Role)
	if err != nil {
		return AdminUser{}, fmt.Errorf("%w: admin", ErrUnauthorized)
	}
	return a, nil
}

// EnsureContentSeeded 是启动期的内容表体检（第 128/129 轮）：
//
//  1. 内容表（enemies/skills）**全空** → 全新库还没跑过 cmd/seed。
//     自动补一次 `seeder.Run`（幂等 upsert，内容表不存玩家数据，
//     安全）。此时玩家登录拿到的签到日历/任务/商城/兑换码全是空的，
//     静默启动等于「开服即内容缺失」。
//  2. 行数**少于** domain 期望（应用升级加了新敌人/技能，库没跟上）
//     → 不自动覆盖（库里可能有运营自定义的行），返回**警示文案**
//     让 main 打出来。投放走内存 domain，玩家可玩；
//     但后台与 loadout 的 skills JOIN 会缺行（新技能在库里无对应行）。
//  3. 行数齐平 → 无事发生。
//
// 返回 (message, error)：message 非空即供启动日志打印。
func (s *Service) EnsureContentSeeded(ctx context.Context) (string, error) {
	var enemiesN, skillsN int
	if err := s.pool.QueryRow(ctx, `SELECT COUNT(*) FROM enemies`).Scan(&enemiesN); err != nil {
		return "", fmt.Errorf("content health: count enemies: %w", err)
	}
	var wantSkills int
	if err := s.pool.QueryRow(ctx, `SELECT COUNT(*) FROM skills`).Scan(&skillsN); err != nil {
		return "", fmt.Errorf("content health: count skills: %w", err)
	}
	wantSkills = len(domain.SeedSkills) + len(domain.SeedCompositeSkills)

	if enemiesN == 0 && skillsN == 0 {
		if _, err := seeder.Run(ctx, s.pool); err != nil {
			return "", fmt.Errorf("auto-seed content: %w", err)
		}
		return "内容表为空，已自动写入种子内容（等同 go run ./cmd/seed）", nil
	}
	if enemiesN < len(domain.SeedEnemies) || skillsN < wantSkills {
		return fmt.Sprintf("内容表陈旧：enemies=%d（期望 %d）、skills=%d（期望 %d）——请运行 go run ./cmd/seed 刷新（投放走内存 domain，库只影响后台与 loadout JOIN）",
			enemiesN, len(domain.SeedEnemies), skillsN, wantSkills), nil
	}
	return "", nil
}

// Audit 记录后台操作。
//
// 参数顺序与 `admin_audit_logs` 的列顺序一致：
// `(adminID, action, target, ip, detail)`。
// ⚠️ ip 夹在 target 与 detail 之间是刻意的 —— 它与 action/target 同属
// 「这一次操作是什么」那一组，而 detail 是自由载荷。
//
// ⚠️ 第 125 轮：审计记录不得「以省略撒谎」。
// detail 序列化失败时，旧代码静默把 detail 换成 `{}` ——
// 那行审计日志看起来与「这个操作本来就没有明细」**完全相同**，
// 运营/调查者后读审计轨迹时被误导：字段没了，但没有任何痕迹。
// 现在失败时 detail 诚实记 `{"detail_error": "..."}`：
// 行本身即证据——明细丢了、为什么丢，都写在里面。
//
// # ip 必须真的记（本轮审计修复）
//
// 原实现写死 `”`。于是**每一行**后台审计的 IP 列都是空的 ——
// 它看起来像「这一列是可选的」，而它其实是「本项目从来没有过来源信息」。
//
// 后果与缺 `detail` 同型且更重：运营账号被盗用（令牌被读走、
// 或后台被点击劫持诱导操作 —— 见 httpapi.securityHeaders）时，
// 事后唯一的取证材料就是 IP。它必须记。
//
// ⚠️ `c.IP()` 在 Fiber 里取的是 TCP 对端地址。反代之后它是**反代**的 IP，
//
//	所以它只在「后端直接暴露」或「反代正确透传 X-Forwarded-For」时可用。
//	这一点如实标注：生产部署**必须**配好反代的透传头，
//	否则记下来的是反代地址 —— 那比空串好一点（能看出有集中来源），但不精确。
func (s *Service) Audit(ctx context.Context, adminID int64, action, target, ip string, detail any) {
	raw, err := marshalJSON(detail)
	if err != nil {
		// 兜底字面量保证 detail 恒为合法 JSONB
		raw = []byte(`{"detail_error":"json marshal failed"}`)
		if m, merr := json.Marshal(map[string]string{
			"detail_error": "json marshal failed: " + err.Error(),
		}); merr == nil {
			raw = m
		}
		// 服务端日志留痕（与全库 stdout 日志惯例一致）
		fmt.Printf("[audit-failed] admin=%d action=%s target=%s detail 丢失：%v\n",
			adminID, action, target, err)
	}
	var username string
	_ = s.pool.QueryRow(ctx, `SELECT username FROM admin_users WHERE id = $1`, adminID).Scan(&username)
	if len(ip) > 64 { // ip 列是 VARCHAR(64)，超长会让整行插入失败
		ip = ip[:64]
	}
	// 审计写失败不能影响主流程，但必须留下痕迹到服务端日志
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO admin_audit_logs (admin_id, username, action, target, detail, ip)
		 VALUES ($1,$2,$3,$4,$5,$6)`, adminID, username, action, target, raw, ip); err != nil {
		fmt.Printf("[audit-failed] admin=%d action=%s target=%s err=%v\n", adminID, action, target, err)
	}
}
