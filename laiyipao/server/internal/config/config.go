// Package config 从环境变量读取运行时配置，提供开发环境默认值。
package config

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
	"time"
)

// Config 是后端全部运行时配置。所有敏感项（数据库口令、JWT 密钥、微信 AppSecret）
// 只从环境变量读取，仓库内不保存任何真实值。
type Config struct {
	Addr     string
	Env      string // dev / prod
	LogLevel string

	DatabaseURL    string
	DBMaxConns     int32
	DBMinConns     int32
	DBConnLifetime time.Duration
	// DBTimeZone 是连接会话时区（第 90 轮新增）。
	// 详见 Config.DBTimeZone 的注释。
	DBTimeZone       string
	MigrationsOnBoot bool

	JWTSecret  string
	AccessTTL  time.Duration
	RefreshTTL time.Duration

	// 微信登录：AppID / AppSecret 未配置时登录走游客通道，
	// /api/v1/auth/wechat 返回 501 而不是静默失败（fail loud）。
	WechatAppID     string
	WechatAppSecret string
	WechatEndpoint  string

	// 战斗结算上限校验
	BattleTokenTTL    time.Duration
	MinBattleDuration time.Duration
	MaxScorePerSecond int64

	// 离线收益
	OfflineCapHours int

	// 默认管理员（仅在 admin_users 为空时引导创建）
	BootstrapAdminUser string
	BootstrapAdminPass string
}

func env(key, def string) string {
	if v := strings.TrimSpace(os.Getenv(key)); v != "" {
		return v
	}
	return def
}

func envInt(key string, def int) int {
	if v := strings.TrimSpace(os.Getenv(key)); v != "" {
		if n, err := strconv.Atoi(v); err == nil {
			return n
		}
	}
	return def
}

// envInt32 带范围裁剪：envInt 返回平台 int（64 位），直接 int32 截断会让
// 越界值（如 DB_MAX_CONNS=99999999999）静默回绕成负数/错值，超界一律回退默认。
func envInt32(key string, def int32) int32 {
	n := envInt(key, int(def))
	if n < math.MinInt32 || n > math.MaxInt32 {
		return def
	}
	return int32(n)
}

func envBool(key string, def bool) bool {
	if v := strings.TrimSpace(os.Getenv(key)); v != "" {
		if b, err := strconv.ParseBool(v); err == nil {
			return b
		}
	}
	return def
}

func envDuration(key string, def time.Duration) time.Duration {
	if v := strings.TrimSpace(os.Getenv(key)); v != "" {
		if d, err := time.ParseDuration(v); err == nil {
			return d
		}
	}
	return def
}

// Load 组装配置。生产环境（Env=prod）下若 JWT 密钥或数据库 DSN 仍为默认值，
// 直接 panic —— 宁可启动失败，也不要用可预测的密钥上线。
func Load() Config {
	cfg := Config{
		Addr:     env("ADDR", ":8080"),
		Env:      env("APP_ENV", "dev"),
		LogLevel: env("LOG_LEVEL", "info"),

		DatabaseURL:      env("DATABASE_URL", "postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable"),
		DBMaxConns:       envInt32("DB_MAX_CONNS", 10),
		DBMinConns:       envInt32("DB_MIN_CONNS", 2),
		DBConnLifetime:   envDuration("DB_CONN_LIFETIME", time.Hour),
		DBTimeZone:       env("DB_TIMEZONE", "Asia/Shanghai"),
		MigrationsOnBoot: envBool("MIGRATIONS_ON_BOOT", true),

		JWTSecret:  env("JWT_SECRET", "dev-only-insecure-secret-change-me"),
		AccessTTL:  envDuration("ACCESS_TTL", 2*time.Hour),
		RefreshTTL: envDuration("REFRESH_TTL", 30*24*time.Hour),

		WechatAppID:     env("WECHAT_APP_ID", ""),
		WechatAppSecret: env("WECHAT_APP_SECRET", ""),
		WechatEndpoint:  env("WECHAT_ENDPOINT", "https://api.weixin.qq.com/sns/jscode2session"),

		BattleTokenTTL:    envDuration("BATTLE_TOKEN_TTL", 30*time.Minute),
		MinBattleDuration: envDuration("MIN_BATTLE_DURATION", 15*time.Second),
		MaxScorePerSecond: int64(envInt("MAX_SCORE_PER_SECOND", 40_000)),

		OfflineCapHours: envInt("OFFLINE_CAP_HOURS", 12),

		BootstrapAdminUser: env("BOOTSTRAP_ADMIN_USER", "admin"),
		BootstrapAdminPass: env("BOOTSTRAP_ADMIN_PASS", "admin12345"),
	}

	if cfg.Env == "prod" {
		if strings.HasPrefix(cfg.JWTSecret, "dev-only") {
			panic("config: APP_ENV=prod 时必须显式设置 JWT_SECRET")
		}
		if !strings.Contains(cfg.DatabaseURL, "sslmode=") {
			panic("config: APP_ENV=prod 时 DATABASE_URL 必须显式声明 sslmode")
		}
		// ⚠️ 第 67 轮补上这条 —— 它是 prod 守卫**唯一漏掉的密钥**。
		//
		// 上面两条守卫 JWT_SECRET 与 sslmode，而 `BootstrapAdminPass` 与 JWT_SECRET
		// 是同一个函数里的同类默认值（都是"不设就给一个能用的值"），
		// 却被漏掉了。后果不是理论问题：
		//
		//   任何全新 prod 部署（未显式设 BOOTSTRAP_ADMIN_PASS）
		//   首次启动即创建 admin / admin12345
		//
		// 而 `admin12345` 恰好 10 位，**正好通过** `EnsureBootstrapAdmin` 的
		// `len(password) >= 10` 门槛 —— 那个门槛拦不住它。
		//
		// 拿到 admin token 后可做什么（全部是既有端点，无需任何额外漏洞）：
		//   POST /admin/users/:id/grant  给**任意** user_id 发放任意数额货币
		//   封禁任意玩家 / 改商城价格 / 读全部战报与钱包
		// 即完整经济系统与封禁体系的接管。
		//
		// 为什么用「显式声明」而不是「够长就行」：
		// 长不等于安全，而默认值本身就是公开的（在 README 与本文件里），
		// 任何人都能查到。用环境变量是否被设置来判定，语义明确且无法误解。
		if os.Getenv("BOOTSTRAP_ADMIN_PASS") == "" {
			panic("config: APP_ENV=prod 时必须显式设置 BOOTSTRAP_ADMIN_PASS（默认值 admin12345 是公开的，不能用于生产）")
		}
	}
	return cfg
}

// WechatEnabled 报告微信登录通道是否可用。
func (c Config) WechatEnabled() bool {
	return c.WechatAppID != "" && c.WechatAppSecret != ""
}

// Redacted 返回可安全打印的配置副本（隐藏密钥）。
func (c Config) Redacted() string {
	return fmt.Sprintf(
		"addr=%s env=%s db=%s wechat_enabled=%t access_ttl=%s refresh_ttl=%s",
		c.Addr, c.Env, redactDSN(c.DatabaseURL), c.WechatEnabled(), c.AccessTTL, c.RefreshTTL,
	)
}

func redactDSN(dsn string) string {
	at := strings.LastIndex(dsn, "@")
	scheme := strings.Index(dsn, "://")
	if at < 0 || scheme < 0 || at < scheme {
		return dsn
	}
	creds := dsn[scheme+3 : at]
	if colon := strings.Index(creds, ":"); colon >= 0 {
		creds = creds[:colon] + ":***"
	}
	return dsn[:scheme+3] + creds + dsn[at:]
}
