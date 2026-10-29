// Package config 从环境变量读取运行时配置，提供开发环境默认值。
package config

import (
	"encoding/json"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
	"sync/atomic"
	"time"
)

// overlay 是**配置文件覆盖层**（nil = 只读环境变量）。
//
// # 为什么需要它
//
// 本服务的配置全部来自环境变量，而运行中的进程**改不了自己的环境**
// —— 于是「SIGHUP 重读配置」在纯环境变量模型下是空操作，热加载会变成
// 一场表演（运维以为生效了，其实什么都没变）。要让热加载真的成立，
// 必须有一个进程能重新读的**文件**来源。
//
// 用法：`CONFIG_FILE` 指向一个扁平 JSON（键名与环境变量同名），
// 启动时读一次，SIGHUP 时重读。文件里出现的键**覆盖**同名环境变量；
// 没出现的键仍走环境变量。
//
// 空字符串按「未设置」处理（与 env* 辅助函数既有的 `TrimSpace != ""` 口径
// 一致），所以把某个键写成 "" 等于回退到环境变量。
var overlay atomic.Pointer[map[string]string]

// rawEnv 读一个配置键：覆盖层优先，其次环境变量。
//
// 覆盖层里的**空值视为未设置**（与 env* 辅助函数既有的 `TrimSpace != ""`
// 口径一致）：否则把某个键写成 "" 会得到「配置为空」而不是「回退」，
// 而后者才是运维改配置时想要的语义。
func rawEnv(key string) string {
	if m := overlay.Load(); m != nil {
		if v, ok := (*m)[key]; ok && strings.TrimSpace(v) != "" {
			return v
		}
	}
	return os.Getenv(key)
}

// ApplyOverlay 安装配置文件覆盖层。传 nil 或空 map 即清除。
//
// 并发安全：整体替换指针，读方永远看到一份完整覆盖层。
func ApplyOverlay(m map[string]string) {
	if len(m) == 0 {
		overlay.Store(nil)
		return
	}
	cp := make(map[string]string, len(m))
	for k, v := range m {
		cp[k] = v
	}
	overlay.Store(&cp)
}

// OverlaySnapshot 返回当前覆盖层的副本（nil = 没有覆盖）。
//
// 用途只有一个：重载失败时把覆盖层回滚到上一次成功的那一份 ——
// 否则「读到了坏文件」会留下一个半生效的覆盖层，比直接失败更难看懂。
func OverlaySnapshot() map[string]string {
	m := overlay.Load()
	if m == nil {
		return nil
	}
	cp := make(map[string]string, len(*m))
	for k, v := range *m {
		cp[k] = v
	}
	return cp
}

// LoadFile 读取扁平 JSON 配置文件（键名与环境变量同名）。
//
// 只接受**标量值**：嵌套对象/数组一律报错而不是猜一个序列化形式 ——
// 猜错的形状会在某个 envList 上静默变成「一个元素」，而配置错误本该
// 在启动时就吵起来（AGENTS #11 fail loud）。
func LoadFile(path string) (map[string]string, error) {
	raw, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("读配置文件 %s: %w", path, err)
	}
	var parsed map[string]any
	if err := json.Unmarshal(raw, &parsed); err != nil {
		return nil, fmt.Errorf("解析配置文件 %s（需要扁平 JSON 对象）: %w", path, err)
	}
	out := make(map[string]string, len(parsed))
	for k, v := range parsed {
		switch val := v.(type) {
		case string:
			out[k] = val
		case bool:
			out[k] = strconv.FormatBool(val)
		case float64:
			// JSON 只有一种数字：整数值不带小数点，避免 "1.000000" 这种
			// 进 Atoi 失败后静默回退默认值。
			if val == math.Trunc(val) && math.Abs(val) < 1e15 {
				out[k] = strconv.FormatInt(int64(val), 10)
			} else {
				out[k] = strconv.FormatFloat(val, 'f', -1, 64)
			}
		case nil:
			out[k] = ""
		default:
			return nil, fmt.Errorf("配置文件 %s 的键 %q 不是标量（%T）：配置只支持扁平键值", path, k, v)
		}
	}
	return out, nil
}

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

	// DashboardCacheTTL / LeaderboardCacheTTL 是看板与榜单的进程内缓存有效期
	// （2026-10-23）。
	//
	// # 为什么这两个端点是「可以旧 30 秒」的
	//
	//   - 看板：运营看的是趋势与异常，30 秒前的数字不改变任何决策；
	//     而它每次要跑 9 段全表聚合（5 个 COUNT(*)、14 桶 DAU、100 关漏斗）。
	//   - 榜单：玩家看的是名次，15 秒的陈旧不会有人察觉；
	//     而它是 `level_stars JOIN users` 的全表 GROUP BY。
	//
	// # 为什么用环境变量而不是写死
	//
	// 设成 `0` 即**关闭缓存**（`ttlCache.get` 对 `ttl <= 0` 恒不命中）。
	// 排查「数字为什么不动」时第一件事就是关掉它 —— 那必须是运维能做、
	// 且不需要发版的动作。
	DashboardCacheTTL   time.Duration
	LeaderboardCacheTTL time.Duration

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

	// MetricsEnabled 打开 `GET /metrics`（Prometheus 文本格式，当前暴露
	// 看板/榜单缓存命中/未命中计数）。默认**关**。
	//
	// 为什么默认关而不是默认开：指标端点虽只含非敏感计数，但「多一个
	// 公开端点就多一个面」，而缓存命中率对**运维**有意义、对公网无意义。
	// 所以由运维显式打开（并配反向代理只放行内网/抓取端）—— 这是一个
	// 需要人做的决定，与 CORS 白名单同一条纪律。
	MetricsEnabled bool

	// PprofAddr 非空时在该地址（建议 `127.0.0.1:6060`）启动一个**独立**的
	// pprof server（net/http/pprof），与主 API 端口分离；默认空 = 不启动。
	//
	// 用独立回环地址而不是挂在主 app 上：profiling 端点绝不该走到公网面，
	// 而主端口是对外的。默认不启动，需要抓 CPU/堆火焰图时由运维显式开。
	PprofAddr string
}

func env(key, def string) string {
	if v := strings.TrimSpace(rawEnv(key)); v != "" {
		return v
	}
	return def
}

func envInt(key string, def int) int {
	if v := strings.TrimSpace(rawEnv(key)); v != "" {
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
	raw := strings.TrimSpace(rawEnv(key))
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
	if v := strings.TrimSpace(rawEnv(key)); v != "" {
		if b, err := strconv.ParseBool(v); err == nil {
			return b
		}
	}
	return def
}

func envDuration(key string, def time.Duration) time.Duration {
	if v := strings.TrimSpace(rawEnv(key)); v != "" {
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

		// 30s / 15s：见字段注释里的取舍。0 = 关闭缓存。
		DashboardCacheTTL:   envDuration("DASHBOARD_CACHE_TTL", 30*time.Second),
		LeaderboardCacheTTL: envDuration("LEADERBOARD_CACHE_TTL", 15*time.Second),

		BootstrapAdminUser: env("BOOTSTRAP_ADMIN_USER", "admin"),
		BootstrapAdminPass: env("BOOTSTRAP_ADMIN_PASS", "admin12345"),

		PlayerLoginRequired: envBool("PLAYER_LOGIN_REQUIRED", false),

		MetricsEnabled: envBool("METRICS_ENABLED", false),
		PprofAddr:      env("PPROF_ADDR", ""),
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
		if rawEnv("BOOTSTRAP_ADMIN_PASS") == "" {
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
// 与 `rawEnv("BOOTSTRAP_ADMIN_PASS") == ""`），而不是另写一套
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
	if rawEnv("BOOTSTRAP_ADMIN_PASS") == "" {
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
