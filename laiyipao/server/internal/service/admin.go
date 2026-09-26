package service

import (
	"context"
	"errors"
	"fmt"

	"github.com/jackc/pgx/v5"
)

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
type AdminTokenPair struct {
	AccessToken string    `json:"access_token"`
	Session     string    `json:"session"`
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

	access, exp, err := s.JWT.Issue(a.ID, "admin")
	if err != nil {
		return AdminTokenPair{}, err
	}
	sessionPlain, sessionHash, err := NewAdminSession()
	if err != nil {
		return AdminTokenPair{}, err
	}
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO admin_tokens (admin_id, token_hash, expires_at) VALUES ($1,$2,$3)`,
		a.ID, sessionHash, timeNow().Add(s.Cfg.AccessTTL)); err != nil {
		return AdminTokenPair{}, fmt.Errorf("store admin session: %w", err)
	}
	if _, err := s.pool.Exec(ctx,
		`UPDATE admin_users SET last_login_at = now() WHERE id = $1`, a.ID); err != nil {
		return AdminTokenPair{}, err
	}
	return AdminTokenPair{
		AccessToken: access, Session: sessionPlain,
		ExpiresAt: exp.UTC().Format(timeFormat), Admin: a,
	}, nil
}

// ResolveAdmin 从 access token 解析管理员 ID。
// ResolveAdmin 解析管理员 access token。
//
// ⚠️ 必须查 admin_users.status。
// 只验签名的话，管理员离职/降权后，他手上的 token 在整个 access TTL
// （默认 2h）内仍能发币、封禁玩家、改商城价格 —— 运营的止损手段失效。
// 全项目只有 AdminMe 带 `AND status = 1`，于是会出现"只有 /admin/me 401，
// 其余管理端点照常可用"的诡异现象。
func (s *Service) ResolveAdmin(ctx context.Context, token string) (int64, error) {
	claims, err := s.JWT.Verify(token)
	if err != nil {
		return 0, fmt.Errorf("%w: %s", ErrUnauthorized, err)
	}
	if claims.Kind != "admin" {
		return 0, fmt.Errorf("%w: 令牌类型不匹配", ErrUnauthorized)
	}
	var status int
	if err := s.pool.QueryRow(ctx,
		`SELECT status FROM admin_users WHERE id = $1`, claims.Sub).Scan(&status); err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return 0, fmt.Errorf("%w: 管理员不存在", ErrUnauthorized)
		}
		return 0, err
	}
	if status != 1 {
		return 0, fmt.Errorf("%w: 管理员已停用", ErrForbidden)
	}
	return claims.Sub, nil
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

// Audit 记录后台操作。
func (s *Service) Audit(ctx context.Context, adminID int64, action, target string, detail any) {
	raw, err := marshalJSON(detail)
	if err != nil {
		raw = []byte(`{}`)
	}
	var username string
	_ = s.pool.QueryRow(ctx, `SELECT username FROM admin_users WHERE id = $1`, adminID).Scan(&username)
	// 审计写失败不能影响主流程，但必须留下痕迹到服务端日志
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO admin_audit_logs (admin_id, username, action, target, detail, ip)
		 VALUES ($1,$2,$3,$4,$5,'')`, adminID, username, action, target, raw); err != nil {
		fmt.Printf("[audit-failed] admin=%d action=%s target=%s err=%v\n", adminID, action, target, err)
	}
}
