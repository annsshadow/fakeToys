// cmd/api 是「来一炮」后端服务入口。
//
// 启动顺序：读配置 → 连库（失败即退出）→ 迁移 → 引导管理员 → 挂路由 → 监听。
// 任一步失败都直接退出并打印原因，不做"先起来再重试"，
// 因为容器编排与本地开发都需要立刻知道失败在哪。
package main

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"strings"
	"syscall"
	"time"

	"github.com/gofiber/fiber/v2"

	// 注册 /debug/pprof/* 到 DefaultServeMux，供 startPprof 的独立 server 使用。
	_ "net/http/pprof"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/httpapi"
	"github.com/laiyipao/server/internal/service"
	"github.com/laiyipao/server/internal/store"
)

func main() {
	if err := run(); err != nil {
		fmt.Fprintln(os.Stderr, "FATAL:", err)
		os.Exit(1)
	}
}

func run() error {
	// CONFIG_FILE：可热加载的配置文件覆盖层（键名与环境变量同名）。
	//
	// 环境变量在进程启动后无法再改，所以「SIGHUP 重读配置」必须有一个
	// 进程能重新读的文件来源，否则热加载只是表演。见 config.overlay 的说明。
	if path := strings.TrimSpace(os.Getenv("CONFIG_FILE")); path != "" {
		m, err := config.LoadFile(path)
		if err != nil {
			return err
		}
		config.ApplyOverlay(m)
		fmt.Printf("[来一炮 server] 已加载配置文件 %s（%d 个键覆盖环境变量）\n", path, len(m))
	}
	cfg := config.Load()
	fmt.Println("[来一炮 server] 启动配置:", cfg.Redacted())

	// 第 154 轮：默认值本身是公开的（写在源码与 README 里），
	// 而「忘设 APP_ENV=prod」的部署会安静地拿到它们 —— 启动日志里
	// 与正常启动**一模一样**。所以把「正在用开发默认凭证」显式喊出来。
	//
	// ⚠️ 这不是 fail-fast：新克隆的仓库不设任何环境变量也能起来是
	// 被测试锁住的决定（见 config.DevDefaultWarnings 的注释）。
	// 这里只负责让那种状态不再隐形。
	if warns := cfg.DevDefaultWarnings(); len(warns) > 0 {
		fmt.Println("[来一炮 server] ⚠️⚠️ 正在使用开发默认凭证（APP_ENV=" + cfg.Env + "，不是 prod）：")
		for _, w := range warns {
			fmt.Println("[来一炮 server] ⚠️   " + w)
		}
		fmt.Println("[来一炮 server] ⚠️ 这只适合本机开发。要部署就设 APP_ENV=prod —— " +
			"那时缺失的配置会直接 panic，而不是像现在这样静默放行。")
	}

	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()

	db, err := store.Open(ctx, cfg)
	if err != nil {
		return fmt.Errorf("连接数据库: %w", err)
	}
	defer db.Close()

	if cfg.MigrationsOnBoot {
		if err := db.Migrate(ctx); err != nil {
			return fmt.Errorf("执行迁移: %w", err)
		}
	}

	svc := service.New(db, cfg)
	bootCtx, bootCancel := context.WithTimeout(context.Background(), 15*time.Second)
	defer bootCancel()
	created, err := svc.EnsureBootstrapAdmin(bootCtx, cfg.BootstrapAdminUser, cfg.BootstrapAdminPass)
	if err != nil {
		return fmt.Errorf("引导管理员: %w", err)
	}
	if created {
		fmt.Printf("[来一炮 server] 已创建初始管理员：%s（密码来自 BOOTSTRAP_ADMIN_PASS，请尽快修改）\n",
			cfg.BootstrapAdminUser)
	}

	// 玩家账号登录（PLAYER_LOGIN_REQUIRED=true 时）。
	//
	// 引导一个共享测试账号：用户名与口令**与运营超管一致**，
	// 于是测试环境只需要记一套凭据，全体测试者的进度/金币聚在同一个号上。
	//
	// ⚠️ 账号已存在时只同步口令、**不碰任何玩家数据** ——
	// 钱包与进度是这个功能要保住的东西，重建账号等于把它清空。
	if cfg.PlayerLoginRequired {
		created, err := svc.EnsureBootstrapPlayer(bootCtx, cfg.BootstrapAdminUser, cfg.BootstrapAdminPass)
		if err != nil {
			return fmt.Errorf("引导玩家测试账号: %w", err)
		}
		if created {
			fmt.Printf("[来一炮 server] 已创建玩家测试账号：%s（口令与运营超管一致）\n",
				cfg.BootstrapAdminUser)
		} else {
			fmt.Printf("[来一炮 server] 玩家测试账号已存在：%s（口令已同步为 BOOTSTRAP_ADMIN_PASS）\n",
				cfg.BootstrapAdminUser)
		}
	}

	// 第 129 轮：内容表体检。全新库自动补种子；库比当前版本「少内容」
	// 时打警示（不自动覆盖，避免冲掉运营自定义行）。
	if msg, err := svc.EnsureContentSeeded(bootCtx); err != nil {
		return fmt.Errorf("内容体检: %w", err)
	} else if msg != "" {
		fmt.Printf("[来一炮 server] %s\n", msg)
	}

	app := buildApp(svc)

	// pprof：仅在 PPROF_ADDR 非空时启动独立 server（默认不启动）。
	// 与主 API 端口分离，且只建议用回环地址 —— 见 config.PprofAddr。
	if cfg.PprofAddr != "" {
		startPprof(cfg.PprofAddr)
	}

	// 优雅退出：收到 SIGINT/SIGTERM 后给在途请求 10 秒收尾
	stop := make(chan os.Signal, 1)
	signal.Notify(stop, os.Interrupt, syscall.SIGTERM)
	go func() {
		<-stop
		fmt.Println("\n[来一炮 server] 收到退出信号，开始优雅关闭...")
		if err := app.ShutdownWithTimeout(10 * time.Second); err != nil {
			fmt.Fprintln(os.Stderr, "关闭超时:", err)
		}
	}()

	// 配置热加载：SIGHUP 重读 CONFIG_FILE，原子替换服务内的配置快照。
	//
	// 只影响「每次请求现读」的旋钮（看板/榜单 TTL、结算重算模式、微信端点）。
	// 连接池、JWT 签名器、监听地址、路由级限流桶在构造期固化，改这些仍需重启
	// —— 见 service.ReloadConfig 的说明。
	reload := make(chan os.Signal, 1)
	signal.Notify(reload, syscall.SIGHUP)
	go func() {
		for range reload {
			if err := reloadConfig(svc); err != nil {
				fmt.Fprintf(os.Stderr, "[来一炮 server] 配置重载失败，**保留原配置**：%v\n", err)
				continue
			}
			c := svc.Cfg()
			fmt.Printf("[来一炮 server] 配置已重载（看板 TTL=%s，榜单 TTL=%s，重算档=%s）\n",
				c.DashboardCacheTTL, c.LeaderboardCacheTTL, c.SettleRecomputeMode)
		}
	}()

	fmt.Printf("[来一炮 server] 监听 %s（微信登录通道：%s）\n",
		cfg.Addr, map[bool]string{true: "已启用", false: "未启用（游客登录）"}[cfg.WechatEnabled()])
	if cfg.PlayerLoginRequired {
		fmt.Printf("[来一炮 server] 玩家登录：已开启账号登录（/auth/guest 已关闭）\n")
	}

	if err := app.Listen(cfg.Addr); err != nil {
		return fmt.Errorf("监听 %s: %w", cfg.Addr, err)
	}
	return nil
}

// buildApp 组装 fiber 实例并挂载全部路由。
// 从 run 抽出：路由、超时与统一错误处理是纯配置，可在无端口环境下测试。
func buildApp(svc *service.Service) *fiber.App {
	app := fiber.New(fiber.Config{
		AppName:               "来一炮 server",
		DisableStartupMessage: true,
		ReadTimeout:           15 * time.Second,
		WriteTimeout:          20 * time.Second,
		// 结算上报体积较小，但仍给足余量
		BodyLimit: 1 << 20,
		ErrorHandler: func(c *fiber.Ctx, err error) error {
			if fe, ok := err.(*fiber.Error); ok {
				return c.Status(fe.Code).JSON(httpapi.APIError{
					Error: httpapi.ErrorBody{Code: "http_error", Message: fe.Message},
				})
			}
			return err
		},
	})
	httpapi.New(svc).Register(app)
	return app
}

// startPprof 在独立地址启动 net/http/pprof（CPU/堆/goroutine 火焰图）。
//
// 独立于主 fiber app 而不是挂在它上面：主服务跑在 fasthttp 上，
// 混在一个监听里要么引 adaptor、要么起第二个 server；后者更干净，
// 也天然把 profiling 端点和公网 API 端口分开（默认只绑回环）。
// 进程退出即随之结束，不接入优雅关闭 —— 它是排障工具，不是常驻面。
func startPprof(addr string) {
	go func() {
		fmt.Printf("[来一炮 server] pprof 监听 %s（/debug/pprof）\n", addr)
		if err := http.ListenAndServe(addr, nil); err != nil && err != http.ErrServerClosed {
			fmt.Fprintln(os.Stderr, "pprof server 退出:", err)
		}
	}()
}

// reloadConfig 重读 CONFIG_FILE 并原子替换服务内的配置快照。
//
// # 为什么必须重读文件而不是"再读一次环境变量"
//
// 进程改不了自己的环境变量。只调 `config.Load()` 会得到与启动时**逐字节相同**
// 的结果，于是 SIGHUP 看起来成功、实际什么都没变 —— 那比不支持热加载更糟：
// 运维会以为改完就生效了。
//
// # 失败必须保留原配置
//
// `config.Load()` 在 `APP_ENV=prod` 且 DSN 缺 sslmode 时会 panic（启动路径上
// 就该失败）。重载路径不能沿用这个行为：一次写错的配置文件不该打死正在服务的
// 进程。所以这里 recover 成错误，并把覆盖层回滚到上一次成功的那份。
func reloadConfig(svc *service.Service) (err error) {
	path := strings.TrimSpace(os.Getenv("CONFIG_FILE"))
	if path == "" {
		return fmt.Errorf("未设置 CONFIG_FILE：配置只来自环境变量，热加载不可用（改环境变量需重启）")
	}
	m, err := config.LoadFile(path)
	if err != nil {
		return err
	}

	prev := config.OverlaySnapshot()
	defer func() {
		if r := recover(); r != nil {
			config.ApplyOverlay(prev)
			err = fmt.Errorf("重载后配置校验失败（已回滚）：%v", r)
		}
	}()

	config.ApplyOverlay(m)
	svc.ReloadConfig(config.Load())
	return nil
}
