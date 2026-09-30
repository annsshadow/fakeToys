package service

import (
	"context"
	"fmt"
	"time"

	"golang.org/x/crypto/bcrypt"

	"github.com/laiyipao/server/internal/auth"
)

// 管理员相关的薄封装，放在单独文件以保持 admin.go 聚焦。

// timeFormat 是 API 统一时间格式。
const timeFormat = time.RFC3339

func timeNow() time.Time { return time.Now() }

// NewAdminSession 生成后台会话令牌（明文 + 落库摘要）。
func NewAdminSession() (plain, hash string, err error) {
	p, _, err := auth.NewRefreshToken()
	if err != nil {
		return "", "", err
	}
	return "a_" + p, auth.HashRefreshToken("a_" + p), nil
}

// VerifyPassword 校验 bcrypt 密码。
func VerifyPassword(hash, plain string) error {
	return bcrypt.CompareHashAndPassword([]byte(hash), []byte(plain))
}

// HashPassword 生成 bcrypt 哈希。
func HashPassword(plain string) (string, error) {
	b, err := bcrypt.GenerateFromPassword([]byte(plain), bcrypt.DefaultCost)
	if err != nil {
		return "", fmt.Errorf("hash password: %w", err)
	}
	return string(b), nil
}

// EnsureBootstrapAdmin 在 admin_users 为空时创建初始管理员。
// 密码来自配置，不设硬编码默认值以外的任何隐式账号。
func (s *Service) EnsureBootstrapAdmin(ctx context.Context, username, password string) (bool, error) {
	var count int
	if err := s.pool.QueryRow(ctx, `SELECT COUNT(*) FROM admin_users`).Scan(&count); err != nil {
		return false, err
	}
	if count > 0 {
		return false, nil
	}
	if len(password) < 10 {
		return false, fmt.Errorf("bootstrap 管理员密码至少 10 位（当前 %d 位）", len(password))
	}
	hash, err := HashPassword(password)
	if err != nil {
		return false, err
	}
	_, err = s.pool.Exec(ctx,
		`INSERT INTO admin_users (username, password_hash, role) VALUES ($1,$2,'admin')`,
		username, hash)
	if err != nil {
		return false, fmt.Errorf("create bootstrap admin: %w", err)
	}
	return true, nil
}
