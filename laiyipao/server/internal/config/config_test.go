package config

// config 包的纯逻辑测试。
//
// 为什么值得测：Load() 里有两条 prod 环境的**启动即 panic** 守卫
// （默认 JWT 密钥、DSN 缺 sslmode）。它们存在的意义是"宁可起不来
// 也不能用可预测密钥上线"，属于安全兜底 —— 没有测试的话，
// 一次重构（比如把 panic 改成 log.Println）就会让守卫静默失效，
// 而全项目再没有别的东西会红。
//
// 全部用例零依赖：env 解析是纯函数，panic 分支用 recover 断言。

import (
	"strings"
	"testing"
	"time"
)

// clearEnv 把 Load 读到的全部变量显式置空。
//
// ⚠️ 必须显式置空而不是假设环境干净：开发者本机可能设置了
// DATABASE_URL / APP_ENV，默认值断言会随之漂移。
// t.Setenv(k, "") 在语义上等价于"未设置"（env() 对空串返回默认值），
// 且测试结束后自动恢复原值。
func clearEnv(t *testing.T) {
	t.Helper()
	for _, k := range []string{
		"ADDR", "APP_ENV", "LOG_LEVEL",
		"DATABASE_URL", "DB_MAX_CONNS", "DB_MIN_CONNS", "DB_CONN_LIFETIME",
		"MIGRATIONS_ON_BOOT",
		"JWT_SECRET", "ACCESS_TTL", "REFRESH_TTL",
		"WECHAT_APP_ID", "WECHAT_APP_SECRET", "WECHAT_ENDPOINT",
		"BATTLE_TOKEN_TTL", "MIN_BATTLE_DURATION", "MAX_SCORE_PER_SECOND",
		"OFFLINE_CAP_HOURS",
		"BOOTSTRAP_ADMIN_USER", "BOOTSTRAP_ADMIN_PASS",
	} {
		t.Setenv(k, "")
	}
}

func TestLoadDefaults(t *testing.T) {
	clearEnv(t)
	cfg := Load()

	if cfg.Addr != ":8080" {
		t.Errorf("Addr 默认值 = %q", cfg.Addr)
	}
	if cfg.Env != "dev" {
		t.Errorf("Env 默认值 = %q", cfg.Env)
	}
	if cfg.LogLevel != "info" {
		t.Errorf("LogLevel 默认值 = %q", cfg.LogLevel)
	}
	if !strings.HasPrefix(cfg.DatabaseURL, "postgres://") {
		t.Errorf("DatabaseURL 默认值 = %q", cfg.DatabaseURL)
	}
	if cfg.DBMaxConns != 10 || cfg.DBMinConns != 2 {
		t.Errorf("连接池默认值 = %d/%d", cfg.DBMaxConns, cfg.DBMinConns)
	}
	if cfg.DBConnLifetime != time.Hour {
		t.Errorf("DBConnLifetime 默认值 = %s", cfg.DBConnLifetime)
	}
	if !cfg.MigrationsOnBoot {
		t.Error("MigrationsOnBoot 默认应为 true")
	}
	if cfg.AccessTTL != 2*time.Hour || cfg.RefreshTTL != 30*24*time.Hour {
		t.Errorf("令牌 TTL 默认值 = %s/%s", cfg.AccessTTL, cfg.RefreshTTL)
	}
	if cfg.BattleTokenTTL != 30*time.Minute {
		t.Errorf("BattleTokenTTL 默认值 = %s", cfg.BattleTokenTTL)
	}
	if cfg.MinBattleDuration != 15*time.Second {
		t.Errorf("MinBattleDuration 默认值 = %s", cfg.MinBattleDuration)
	}
	if cfg.MaxScorePerSecond != 40_000 {
		t.Errorf("MaxScorePerSecond 默认值 = %d", cfg.MaxScorePerSecond)
	}
	if cfg.OfflineCapHours != 12 {
		t.Errorf("OfflineCapHours 默认值 = %d", cfg.OfflineCapHours)
	}
	if cfg.BootstrapAdminUser != "admin" || cfg.BootstrapAdminPass != "admin12345" {
		t.Errorf("bootstrap 管理员默认值 = %q/%q", cfg.BootstrapAdminUser, cfg.BootstrapAdminPass)
	}
	if cfg.WechatEnabled() {
		t.Error("未配置 AppID/Secret 时微信通道不应可用")
	}
}

func TestLoadParsesEnvOverrides(t *testing.T) {
	clearEnv(t)
	t.Setenv("ADDR", ":9999")
	t.Setenv("APP_ENV", "dev")
	t.Setenv("LOG_LEVEL", "debug")
	t.Setenv("DATABASE_URL", "postgres://u:p@h:1/x?sslmode=require")
	t.Setenv("DB_MAX_CONNS", "33")
	t.Setenv("DB_MIN_CONNS", "3")
	t.Setenv("DB_CONN_LIFETIME", "90s")
	t.Setenv("MIGRATIONS_ON_BOOT", "false")
	t.Setenv("JWT_SECRET", "s3cret")
	t.Setenv("ACCESS_TTL", "5m")
	t.Setenv("REFRESH_TTL", "1h")
	t.Setenv("WECHAT_APP_ID", "wx123")
	t.Setenv("WECHAT_APP_SECRET", "sec")
	t.Setenv("WECHAT_ENDPOINT", "https://example.com/wx")
	t.Setenv("BATTLE_TOKEN_TTL", "10m")
	t.Setenv("MIN_BATTLE_DURATION", "1s")
	t.Setenv("MAX_SCORE_PER_SECOND", "123")
	t.Setenv("OFFLINE_CAP_HOURS", "6")
	t.Setenv("BOOTSTRAP_ADMIN_USER", "boss")
	t.Setenv("BOOTSTRAP_ADMIN_PASS", "longpassword")

	cfg := Load()

	if cfg.Addr != ":9999" || cfg.LogLevel != "debug" {
		t.Errorf("基础覆盖未生效：%q %q", cfg.Addr, cfg.LogLevel)
	}
	if cfg.DatabaseURL != "postgres://u:p@h:1/x?sslmode=require" {
		t.Errorf("DatabaseURL 覆盖未生效：%q", cfg.DatabaseURL)
	}
	if cfg.DBMaxConns != 33 || cfg.DBMinConns != 3 {
		t.Errorf("连接池覆盖未生效：%d/%d", cfg.DBMaxConns, cfg.DBMinConns)
	}
	if cfg.DBConnLifetime != 90*time.Second {
		t.Errorf("DBConnLifetime 覆盖未生效：%s", cfg.DBConnLifetime)
	}
	if cfg.MigrationsOnBoot {
		t.Error("MIGRATIONS_ON_BOOT=false 未生效")
	}
	if cfg.AccessTTL != 5*time.Minute || cfg.RefreshTTL != time.Hour {
		t.Errorf("TTL 覆盖未生效：%s/%s", cfg.AccessTTL, cfg.RefreshTTL)
	}
	if !cfg.WechatEnabled() {
		t.Error("配置了 AppID+Secret 后微信通道应可用")
	}
	if cfg.WechatEndpoint != "https://example.com/wx" {
		t.Errorf("WechatEndpoint 覆盖未生效：%q", cfg.WechatEndpoint)
	}
	if cfg.BattleTokenTTL != 10*time.Minute || cfg.MinBattleDuration != time.Second {
		t.Errorf("战斗配置覆盖未生效：%s/%s", cfg.BattleTokenTTL, cfg.MinBattleDuration)
	}
	if cfg.MaxScorePerSecond != 123 || cfg.OfflineCapHours != 6 {
		t.Errorf("数值覆盖未生效：%d/%d", cfg.MaxScorePerSecond, cfg.OfflineCapHours)
	}
	if cfg.BootstrapAdminUser != "boss" || cfg.BootstrapAdminPass != "longpassword" {
		t.Errorf("bootstrap 管理员覆盖未生效：%q/%q", cfg.BootstrapAdminUser, cfg.BootstrapAdminPass)
	}
}

// 非法值必须回落到默认值而不是报错 —— 配置解析失败让进程起不来
// 是更糟的失败模式（fail loud 的边界在"安全守卫"，不在"打错一个数"）。
func TestLoadFallsBackOnInvalidValues(t *testing.T) {
	clearEnv(t)
	t.Setenv("DB_MAX_CONNS", "not-a-number")
	t.Setenv("MIGRATIONS_ON_BOOT", "yes-please")
	t.Setenv("ACCESS_TTL", "5 minutes")
	t.Setenv("MAX_SCORE_PER_SECOND", "")

	cfg := Load()
	if cfg.DBMaxConns != 10 {
		t.Errorf("非法 int 应回落默认 10，实际 %d", cfg.DBMaxConns)
	}
	if !cfg.MigrationsOnBoot {
		t.Error("非法 bool 应回落默认 true")
	}
	if cfg.AccessTTL != 2*time.Hour {
		t.Errorf("非法 duration 应回落默认 2h，实际 %s", cfg.AccessTTL)
	}
	if cfg.MaxScorePerSecond != 40_000 {
		t.Errorf("空值应回落默认 40000，实际 %d", cfg.MaxScorePerSecond)
	}
}

// 数值与布尔的合法变体各覆盖一条：负数、显式 "0"、不同大小写的 bool。
func TestLoadValueVariants(t *testing.T) {
	clearEnv(t)
	t.Setenv("DB_MIN_CONNS", "0")
	t.Setenv("MIGRATIONS_ON_BOOT", "False")
	t.Setenv("OFFLINE_CAP_HOURS", "-3")

	cfg := Load()
	if cfg.DBMinConns != 0 {
		t.Errorf("DB_MIN_CONNS=0 应被接受（0 是合法值），实际 %d", cfg.DBMinConns)
	}
	if cfg.MigrationsOnBoot {
		t.Error("False（大小写混合）应解析为 false")
	}
	if cfg.OfflineCapHours != -3 {
		t.Errorf("负数应原样解析，实际 %d", cfg.OfflineCapHours)
	}
}

func TestProdGuardsPanic(t *testing.T) {
	// 守卫一：prod + 默认密钥 → 必须拒绝启动
	t.Run("prod使用默认密钥", func(t *testing.T) {
		clearEnv(t)
		t.Setenv("APP_ENV", "prod")
		// JWT_SECRET 留空 → 用默认的 dev-only 前缀密钥
		mustPanic(t, "config: APP_ENV=prod 时必须显式设置 JWT_SECRET", func() { _ = Load() })
	})

	// 守卫二：prod + DSN 缺 sslmode → 必须拒绝启动
	t.Run("prod的DSN缺sslmode", func(t *testing.T) {
		clearEnv(t)
		t.Setenv("APP_ENV", "prod")
		t.Setenv("JWT_SECRET", "real-secret-from-vault")
		t.Setenv("DATABASE_URL", "postgres://u:p@h:5432/db")
		mustPanic(t, "sslmode", func() { _ = Load() })
	})

	// 对照组：prod + 显式密钥 + sslmode → 必须正常通过
	t.Run("prod合法配置放行", func(t *testing.T) {
		clearEnv(t)
		t.Setenv("APP_ENV", "prod")
		t.Setenv("JWT_SECRET", "real-secret-from-vault")
		t.Setenv("DATABASE_URL", "postgres://u:p@h:5432/db?sslmode=require")
		cfg := Load()
		if cfg.Env != "prod" {
			t.Errorf("Env = %q", cfg.Env)
		}
	})
}

// mustPanic 断言 fn panic 且消息包含 want。
// 用"消息必须匹配"而不是只断言"panic 了"：
// 否则两条守卫互相写错（A 的消息进 B 的分支）测试照样绿。
func mustPanic(t *testing.T, want string, fn func()) {
	t.Helper()
	defer func() {
		r := recover()
		if r == nil {
			t.Fatalf("应 panic（期望消息含 %q）却没有", want)
		}
		msg, ok := r.(string)
		if !ok {
			t.Fatalf("panic 值应为 string，实际 %T: %v", r, r)
		}
		if !strings.Contains(msg, want) {
			t.Fatalf("panic 消息 %q 不含期望片段 %q", msg, want)
		}
	}()
	fn()
}

// --- redactDSN ---

func TestRedactedHidesPassword(t *testing.T) {
	cfg := Config{
		Addr: ":8080", Env: "dev",
		DatabaseURL: "postgres://alice:hunter2@db.internal:5432/laiyipao?sslmode=require",
	}
	out := cfg.Redacted()
	if strings.Contains(out, "hunter2") {
		t.Fatalf("Redacted 泄露了口令：%s", out)
	}
	if !strings.Contains(out, "alice:***@") {
		t.Errorf("口令应被替换为 ***：%s", out)
	}
	if !strings.Contains(out, "wechat_enabled=false") {
		t.Errorf("应包含 wechat_enabled：%s", out)
	}
}

func TestRedactDSNEdgeCases(t *testing.T) {
	cases := []struct {
		name string
		in   string
		want string
	}{
		// 无 @ 或无 scheme 的串不是 DSN，原样返回 —— 改动这段
		// 会把普通字符串也"脱敏"坏
		{"无at", "postgres://host/db", "postgres://host/db"},
		{"无scheme", "alice:hunter2@host/db", "alice:hunter2@host/db"},
		{"at在scheme前", "a@b://c", "a@b://c"},
		{"无口令", "postgres://alice@host/db", "postgres://alice@host/db"},
		{"有口令", "postgres://alice:pw@host/db", "postgres://alice:***@host/db"},
		{"多个at取最后一个", "postgres://a:b@c@d/db", "postgres://a:***@d/db"},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			if got := redactDSN(c.in); got != c.want {
				t.Errorf("redactDSN(%q) = %q，期望 %q", c.in, got, c.want)
			}
		})
	}
}

// --- env 家族直接单测（Load 已隐式覆盖，这里锁住边界语义） ---

func TestEnvHelpersWhitespace(t *testing.T) {
	t.Setenv("CFG_TEST_STR", "  spaced  ")
	t.Setenv("CFG_TEST_INT", " 7 ")
	if got := env("CFG_TEST_STR", "def"); got != "spaced" {
		t.Errorf("env 应裁剪空白，实际 %q", got)
	}
	if got := envInt("CFG_TEST_INT", 1); got != 7 {
		t.Errorf("envInt 应裁剪空白后解析，实际 %d", got)
	}
	// " 7 " 裁剪后是合法数字；纯空白视为未设置
	t.Setenv("CFG_TEST_BLANK", "   ")
	if got := env("CFG_TEST_BLANK", "def"); got != "def" {
		t.Errorf("纯空白应视为未设置，实际 %q", got)
	}
	if got := envInt32("CFG_TEST_INT", 1); got != 7 {
		t.Errorf("envInt32 应转发 envInt，实际 %d", got)
	}
}
