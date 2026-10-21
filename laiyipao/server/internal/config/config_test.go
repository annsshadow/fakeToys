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
	"os"
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
		"BATTLE_TOKEN_TTL", "MIN_BATTLE_DURATION",
		"SETTLE_RECOMPUTE_MODE",
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
	if cfg.OfflineCapHours != 12 {
		t.Errorf("OfflineCapHours 默认值 = %d", cfg.OfflineCapHours)
	}
	if cfg.BootstrapAdminUser != "admin" || cfg.BootstrapAdminPass != "admin12345" {
		t.Errorf("bootstrap 管理员默认值 = %q/%q", cfg.BootstrapAdminUser, cfg.BootstrapAdminPass)
	}
	if cfg.WechatEnabled() {
		t.Error("未配置 AppID/Secret 时微信通道不应可用")
	}
	// 缺省必须是**关闭**：游客通道是现有部署的默认行为，
	// 新开关不能顺手把默认改了。
	if cfg.PlayerLoginRequired {
		t.Error("PLAYER_LOGIN_REQUIRED 缺省应为 false（游客通道保持默认）")
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
	t.Setenv("OFFLINE_CAP_HOURS", "6")
	t.Setenv("BOOTSTRAP_ADMIN_USER", "boss")
	t.Setenv("BOOTSTRAP_ADMIN_PASS", "longpassword")
	t.Setenv("PLAYER_LOGIN_REQUIRED", "true")

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
	if cfg.OfflineCapHours != 6 {
		t.Errorf("数值覆盖未生效：%d", cfg.OfflineCapHours)
	}
	if cfg.BootstrapAdminUser != "boss" || cfg.BootstrapAdminPass != "longpassword" {
		t.Errorf("bootstrap 管理员覆盖未生效：%q/%q", cfg.BootstrapAdminUser, cfg.BootstrapAdminPass)
	}
	if !cfg.PlayerLoginRequired {
		t.Error("PLAYER_LOGIN_REQUIRED=true 未生效")
	}
}

// 非法值必须回落到默认值而不是报错 —— 配置解析失败让进程起不来
// 是更糟的失败模式（fail loud 的边界在"安全守卫"，不在"打错一个数"）。
func TestLoadFallsBackOnInvalidValues(t *testing.T) {
	clearEnv(t)
	t.Setenv("DB_MAX_CONNS", "not-a-number")
	t.Setenv("MIGRATIONS_ON_BOOT", "yes-please")
	t.Setenv("ACCESS_TTL", "5 minutes")

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

	// 对照组：prod + 显式密钥 + sslmode + 显式口令 → 必须正常通过
	//
	// ⚠️ 第 67 轮补上 BOOTSTRAP_ADMIN_PASS：它也是「有公开默认值」的敏感项，
	// prod 守卫新增了对应检查。不设它时这里会 panic —— 那正是守卫在起作用，
	// 不是这条对照组坏了。
	t.Run("prod合法配置放行", func(t *testing.T) {
		clearEnv(t)
		t.Setenv("APP_ENV", "prod")
		t.Setenv("JWT_SECRET", "real-secret-from-vault")
		t.Setenv("DATABASE_URL", "postgres://u:p@h:5432/db?sslmode=require")
		t.Setenv("BOOTSTRAP_ADMIN_PASS", "a-real-long-production-password")
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
	// 超出 int32 表示范围的值不得截断回绕，应回退默认（CodeQL go/incorrect-integer-conversion）
	t.Setenv("CFG_TEST_INT", "99999999999")
	if got := envInt32("CFG_TEST_INT", 1); got != 1 {
		t.Errorf("envInt32 越界应回退默认，实际 %d", got)
	}
	t.Setenv("CFG_TEST_INT", "-99999999999")
	if got := envInt32("CFG_TEST_INT", 1); got != 1 {
		t.Errorf("envInt32 负越界应回退默认，实际 %d", got)
	}
}

// TestEnvInt32BoundaryValues 钉住 int32 的**四个边界**。
//
// ⚠️ 第 84 轮补的。原因是变异测试暴露了一个真空缺：
//
// 把判据从「越界」改成「负数」（`if n < 0 { return def }`）→ **全部测试仍绿**。
//
// 那两种写法在**越界**输入上表现相同，只在
// 「负数但仍在 int32 范围内」（如 -5）上不同：
//
//	正确   n = -5 → 不越界 → int32(-5) = -5
//	变异   n = -5 → n < 0   → 回退默认 1
//
// 而 `DB_MAX_CONNS=-5` 在真实环境里是可能发生的（配错环境变量）——
// 它会一路传到 `pgxpool.New` 才报错，报错信息是
// 「MaxConns must be > 0」，与「配置写错了负数」之间隔了两层。
//
// 顺便把 int32 的两端边界都钉住：`MaxInt32` 本身**合法**（不该被夹），
// `MaxInt32+1` 才该回退。
func TestEnvInt32BoundaryValues(t *testing.T) {
	const k = "CFG_TEST_INT32"

	cases := []struct {
		name string
		set  string
		def  int32
		want int32
	}{
		{"int32 上界本身合法", "2147483647", 1, 2147483647},
		{"上界+1 回退默认", "2147483648", 7, 7},
		{"int32 下界本身合法", "-2147483648", 1, -2147483648},
		{"下界-1 回退默认", "-2147483649", 7, 7},
		// ⚠️ 这一条就是抓上面那个变异的那一条：
		// 「负但在范围内」必须**原样通过**，不能被当成「非法」。
		{"负但在范围内原样通过", "-5", 1, -5},
		{"0 原样通过", "0", 9, 0},
		{"非数字回退默认", "abc", 3, 3},
		{"空串回退默认", "", 4, 4},
	}

	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			t.Setenv(k, c.set)
			if got := envInt32(k, c.def); got != c.want {
				t.Errorf("envInt32(%q, %d) = %d，期望 %d", c.set, c.def, got, c.want)
			}
		})
	}
}

// TestEnvInt32ClampDoesNotSwallowDefaults 确认「回退的是 def 而不是 0」。
//
// ⚠️ 一个很容易写错的形态：`n` 越界时若 `return 0` 而不是 `return def`，
// 那么 `DB_MAX_CONNS=99999999999` 会让连接池上限变成 0 ——
// 而 0 在 `pgxpool` 里意味着「用默认值」，于是**配置写错反而拿到合理值**，
// 彻底静默。回退 `def` 才是「我不知道你要什么，用我准备好的」。
func TestEnvInt32ClampDoesNotSwallowDefaults(t *testing.T) {
	t.Setenv("CFG_TEST_INT32", "99999999999")
	if got := envInt32("CFG_TEST_INT32", 0); got != 0 {
		t.Errorf("def=0 时应回退 0，实际 %d", got)
	}
	t.Setenv("CFG_TEST_INT32", "-99999999999")
	if got := envInt32("CFG_TEST_INT32", 42); got != 42 {
		t.Errorf("越界应回退 def=42，实际 %d", got)
	}
}

// TestNoScoreRateConfig 防止「速率上限」以新配置项的形式回来。
//
// 2026-10-20 删掉的 `MAX_SCORE_PER_SECOND` 是一个**误导性配置面**：
// 运维手册里写着它，运营会以为把它调小能收紧反作弊；而它的值
// 从来不影响任何判定（min(sec*50000, full) 永远取满分那侧）。
// 一个「看起来能配、配了不生效」的旋钮，比没有这个旋钮更糟。
//
// 现在速率上限由 domain 的 `scoreCapFor` 按关卡推导，锚点与
// `ScoreFullAtSec` 共用同一份常量。做成可配置项就等于制造
// 「配置里一个值、公式里另一个值」的两处可能不同步的面。
//
// 判据必须是「配置源码里不出现这个名字」—— 只断言字段不存在的话，
// 下一个人加个 `MaxRate int` 也能通过。
//
// ⚠️ 必须先剥注释再看：这个文件（以及 domain/battle.go 的 SettleLimits
// 注释）里**刻意**写着 `MAX_SCORE_PER_SECOND` 作为历史说明。
// 不剥注释的话，「写注释解释为什么删」这件事本身就会让守卫红 ——
// 那会激励下一个人把说明删掉，比留下死字段更糟。
// 同理 bundle_size.test.ts 的 stripComments 是同类坑。
func TestNoScoreRateConfig(t *testing.T) {
	src := stripComments(readSelf(t))
	for _, needle := range []string{"MAX_SCORE_PER_SECOND", "MaxScorePerSec"} {
		if strings.Contains(src, needle) {
			t.Errorf("config 里又出现了 %q —— 速率上限不该是可配置项，"+
				"应由 domain.scoreCapFor 按关卡推导（见该测试上方的说明）", needle)
		}
	}
}

// readSelf 读本包的配置源码（供源码级守卫使用）。
func readSelf(t *testing.T) string {
	t.Helper()
	b, err := os.ReadFile("config.go")
	if err != nil {
		t.Fatalf("读 config.go 失败：%v", err)
	}
	return string(b)
}

// stripComments 剥掉 Go 源码里的行注释与块注释，保留字符串字面量。
//
// 与 admin/tests/bundle_size.test.ts 的同名函数同一个理由：
// 源码级守卫匹配到**自己写的解释性注释**时，「把代码改对」反而会红，
// 那会激励下一个人删掉说明而不是修代码。
func stripComments(src string) string {
	var out strings.Builder
	for i := 0; i < len(src); i++ {
		c := src[i]
		if c == '/' && i+1 < len(src) {
			if src[i+1] == '/' {
				for i < len(src) && src[i] != '\n' {
					i++
				}
				continue
			}
			if src[i+1] == '*' {
				i += 2
				for i+1 < len(src) && !(src[i] == '*' && src[i+1] == '/') {
					i++
				}
				i++ // 跳过 '*'，循环的 i++ 再跳过 '/'
				continue
			}
		}
		if c == '"' || c == '`' || c == '\'' {
			quote := c
			out.WriteByte(c)
			for i++; i < len(src) && src[i] != quote; i++ {
				if quote == '"' && src[i] == '\\' && i+1 < len(src) {
					out.WriteByte(src[i])
					i++
				}
				out.WriteByte(src[i])
			}
			if i < len(src) {
				out.WriteByte(src[i])
			}
			continue
		}
		out.WriteByte(c)
	}
	return out.String()
}
