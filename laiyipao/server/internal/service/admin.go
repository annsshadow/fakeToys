package service

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/seeder"
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

// AdminRole 返回管理账号的角色（'admin'/'ops'/'readonly'，见 00007 迁移注释）。
//
// ⚠️ 第 127 轮：这个列**历史上从未被任何端点消费过** —— 一个
// readonly 账号和 admin 账号能做的事完全一样（封禁、发币、改关卡/商城）。
// 它是「纸面角色」。requireAdmin 现在把角色写进 Locals，
// requireWritable 据此拒绝 readonly 的写操作。
func (s *Service) AdminRole(ctx context.Context, adminID int64) (string, error) {
	var role string
	err := s.pool.QueryRow(ctx, `SELECT role FROM admin_users WHERE id = $1`, adminID).Scan(&role)
	if err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return "", fmt.Errorf("%w: 管理员不存在", ErrUnauthorized)
		}
		return "", fmt.Errorf("load admin role: %w", err)
	}
	return role, nil
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
// ⚠️ 第 125 轮：审计记录不得「以省略撒谎」。
// detail 序列化失败时，旧代码静默把 detail 换成 `{}` ——
// 那行审计日志看起来与「这个操作本来就没有明细」**完全相同**，
// 运营/调查者后读审计轨迹时被误导：字段没了，但没有任何痕迹。
// 现在失败时 detail 诚实记 `{"detail_error": "..."}`：
// 行本身即证据——明细丢了、为什么丢，都写在里面。
func (s *Service) Audit(ctx context.Context, adminID int64, action, target string, detail any) {
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
	// 审计写失败不能影响主流程，但必须留下痕迹到服务端日志
	if _, err := s.pool.Exec(ctx,
		`INSERT INTO admin_audit_logs (admin_id, username, action, target, detail, ip)
		 VALUES ($1,$2,$3,$4,$5,'')`, adminID, username, action, target, raw); err != nil {
		fmt.Printf("[audit-failed] admin=%d action=%s target=%s err=%v\n", adminID, action, target, err)
	}
}
