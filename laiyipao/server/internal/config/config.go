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

	// CORSAllowOrigins 是允许跨域访问的来源白名单。
	//
	// ⚠️ 语义与「空 = 全放行」**相反**：
	//
	//   - dev 且未设置   → `*`（前后端不同端口，必须放行）
	//   - prod 且未设置  → **一条 CORS 头都不发**（等价只允许同源）
	//
	// 取反向默认值的理由：本项目不用 cookie 存凭证（纯 Bearer 头），
	// 所以 `*` 眼下**不构成**带凭证的跨域读 —— 把它报成严重漏洞是错的。
	// 真正的风险是「将来有人加了 cookie 登录」，那时 `*` 立刻变成全站沦陷，
	// 且没有任何东西会提醒。所以默认取安全的一侧，
	// 需要跨域（后台与 H5 不同域名）就显式配置 —— 那是一个需要人做的决定。
	CORSAllowOrigins []string

	// SettleRecomputeMode 是服务端重算的处置档位：off / shadow / enforce。
	//
	// ⚠️ 默认 **enforce**，这是刻意的，也是本轮改过一次的地方。
	//
	// 第一版默认值定的是 `off`，理由是「引擎会漂移，enforce 会误伤」。
	// 现有测试立刻把它打红了 —— `replay_skills_e2e_test.go` 断言伪造的
	// `replay_skills` **必须**被拒。而那条校验早就在生产上跑着。
	//
	// 那个理由把两类重算混为一谈了：
	//
	//   - 整局引擎（尚未移植）：不符**有歧义**，可能是漂移 → 所以要先 shadow。
	//   - S 段 replay_skills（已在跑）：不符**无歧义**就是作弊 —— 服务端与
	//     客户端读同一份 DB 数据，中间没有第二个实现，不存在「算错」这一说。
	//
	// 对第二类默认 off 等于**新装部署出厂即关闭反作弊校验**。
	// 「尚未启用的功能」和「已在运行的防护」不能共用一个默认值。
	//
	// 所以：默认 enforce（保持现状），引擎接进来时再**主动**切到 shadow 观察。
	// 「先观察再收紧」是接入引擎时的动作，不是新装部署的默认状态。
	//
	// 切档**不需要发版**：改环境变量重启即可。
	SettleRecomputeMode string

	// 战斗结算上限校验
	BattleTokenTTL    time.Duration
	MinBattleDuration time.Duration

	// 离线收益
	OfflineCapHours int

	// 默认管理员（仅在 admin_users 为空时引导创建）
	BootstrapAdminUser string
	BootstrapAdminPass string

	// PlayerLoginRequired 开启「玩家必须用账号密码登录」。
	//
	// 缺省关闭：H5/小程序走游客通道（每设备一个游客号，凭证在 localStorage）。
	// 打开后：
	//
	//   - `POST /api/v1/auth/guest` 直接 403（code=login_required）——
	//     连**浏览器里缓存的旧客户端**也建不出新的游客号；
	//   - 启动时引导创建一个共享测试账号（用户名/口令与运营超管一致）。
	//
	// # 为什么服务端也要拦，而不是只改客户端
	//
	// 只改客户端的话，「每个浏览器一个号、测试数据散在各处」这个老问题
	// 会在**看起来已经修好**之后继续存在 ——
	// 因为浏览器里还缓存着旧 bundle，它照样调 /auth/guest 建新号。
	// 服务端开关是唯一能让「开关一拧，所有客户端立刻一致」的地方。
	//
	// # 为什么共享账号的口令与超管一致是**运维选择**而不是模型要求
	//
	// 数据模型上两者完全独立（`users` vs `admin_users`，令牌 kind 不同）。
	// 让它们共用一套口令只是省掉「再记一套测试凭据」，
	// 代价是这个登录端点同时成为**超管口令的爆破面** ——
	// 所以它的限流比游客建号严得多（见 middleware.go 的 playerLoginLimit）。
	PlayerLoginRequired bool
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

// envList 读一个逗号分隔的环境变量列表，逐项去空白、丢空项。
//
// # 为什么不能直接 strings.Split 后全塞进去
//
// `CORS_ALLOW_ORIGINS="https://a.com, https://b.com"` 是最自然的写法，
// 第二项会带上前导空格。而 CORS 的 `Origin` 请求头**绝不含空格**，
// 于是那一项永远匹配不上 —— 表现为「配了白名单但某个域名还是被拒」，
// 排查时很难想到是空格。
//
// 丢空项的另一个理由：`"a,,b"` 与 `"a, ,b"` 不该产生空字符串来源，
// 否则中间件里 `allowSet[""] = true` 会让「无 Origin 头」的请求
// 命中白名单分支。
func envList(key string) []string {
	raw := strings.TrimSpace(os.Getenv(key))
	if raw == "" {
		return nil
	}
	var out []string
	for _, p := range strings.Split(raw, ",") {
		if p = strings.TrimSpace(p); p != "" {
			out = append(out, p)
		}
	}
	return out
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

		CORSAllowOrigins: envList("CORS_ALLOW_ORIGINS"),

		BattleTokenTTL:    envDuration("BATTLE_TOKEN_TTL", 30*time.Minute),
		MinBattleDuration: envDuration("MIN_BATTLE_DURATION", 15*time.Second),

		SettleRecomputeMode: env("SETTLE_RECOMPUTE_MODE", "enforce"),

		OfflineCapHours: envInt("OFFLINE_CAP_HOURS", 12),

		BootstrapAdminUser: env("BOOTSTRAP_ADMIN_USER", "admin"),
		BootstrapAdminPass: env("BOOTSTRAP_ADMIN_PASS", "admin12345"),

		PlayerLoginRequired: envBool("PLAYER_LOGIN_REQUIRED", false),
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

// DevDefaultWarnings 返回「正在使用开发默认凭证」的清单（第 154 轮）。
//
// # 它解决的问题：默认值本身是**公开的**
//
// prod 分支会在缺失配置时 panic（见上面那段），但那要求 `APP_ENV=prod`
// **被设对了**。`APP_ENV` 的默认值是 `dev` —— 于是「忘设 APP_ENV」的部署
// 会安静地拿到：
//
//	JWT_SECRET = dev-only-insecure-secret-change-me  （写在源码与 README 里）
//	BOOTSTRAP_ADMIN_PASS = admin12345                  （同上）
//
// 后果不是「弱口令」而是**无需爆破**：知道签名的密钥就能直接签一枚
// `{uid:1, kind:...}` 的访问令牌，而默认管理员口令是公开字面量。
// 而这一切在启动日志里**没有任何痕迹** —— 它长得和正常启动一模一样。
//
// # 为什么是告警而不是禁止
//
// 因为「新克隆仓库不设任何环境变量也能起来」是被测试锁住的产品决定
// （见 `TestNonProdStillGetsDevelopmentDefaults` 与 README 快速开始）。
// 禁止默认值会让那条路断掉；而静默又让误配置不可见。
// 告警是两者之间唯一诚实的选项：**默认照常可用，但它不再是隐形的**。
//
// # 判据与 prod 守卫**同源**
//
// 刻意复用 prod 分支完全相同的两个判断（`HasPrefix(JWTSecret,"dev-only")`
// 与 `os.Getenv("BOOTSTRAP_ADMIN_PASS") == ""`），而不是另写一套
// 「值是否等于默认字面量」。两套判据迟早会分叉，而分叉之后一定是
// 「prod 会 panic，但这里不告警」那种最坏的组合。
//
// ⚠️ 不要为了让这条更「精确」而把默认值提成常量的同时改掉
// `env("BOOTSTRAP_ADMIN_PASS", "admin12345")` 的字面量形态 ——
// `prod_guard_coverage_test.go` 的 AST 扫描只认 BasicLit，
// 换成标识符会让那个环境变量**从敏感清单里静默消失**。
func (c Config) DevDefaultWarnings() []string {
	var out []string
	if strings.HasPrefix(c.JWTSecret, "dev-only") {
		out = append(out, "JWT_SECRET 未显式声明，正在使用本仓库源码里公开的默认密钥 —— "+
			"任何读到它的人都能伪造任意账号（含管理员）的访问令牌")
	}
	if os.Getenv("BOOTSTRAP_ADMIN_PASS") == "" {
		out = append(out, "BOOTSTRAP_ADMIN_PASS 未显式声明，正在使用公开默认口令 admin12345 —— "+
			"首次启动会创建 admin/admin12345")
	}
	return out
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
