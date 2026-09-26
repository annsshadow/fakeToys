// cmd/api 是「来一炮」后端服务入口。
//
// 启动顺序：读配置 → 连库（失败即退出）→ 迁移 → 引导管理员 → 挂路由 → 监听。
// 任一步失败都直接退出并打印原因，不做"先起来再重试"，
// 因为容器编排与本地开发都需要立刻知道失败在哪。
package main

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/gofiber/fiber/v2"

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
	cfg := config.Load()
	fmt.Println("[来一炮 server] 启动配置:", cfg.Redacted())

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

	fmt.Printf("[来一炮 server] 监听 %s（微信登录通道：%s）\n",
		cfg.Addr, map[bool]string{true: "已启用", false: "未启用（游客登录）"}[cfg.WechatEnabled()])

	if err := app.Listen(cfg.Addr); err != nil {
		return fmt.Errorf("监听 %s: %w", cfg.Addr, err)
	}
	return nil
}
