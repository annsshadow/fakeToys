// Package migrate 提供一个独立的一次性迁移命令，便于本地与 CI 单独执行。
// 用法：go run ./cmd/migrate [-down]
package main

import (
	"context"
	"flag"
	"fmt"
	"os"
	"time"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/store"
)

func main() {
	down := flag.Bool("down", false, "回滚一步（仅本地调试用）")
	flag.Parse()

	cfg := config.Load()
	fmt.Println("config:", cfg.Redacted())

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Minute)
	defer cancel()

	if err := run(ctx, cfg, *down); err != nil {
		fmt.Fprintln(os.Stderr, "FATAL:", err)
		os.Exit(1)
	}
}

// run 执行迁移主体，从 main 抽出以便测试（main 只负责 flag 解析与退出码）。
// 迁移成功/失败完全由数据库侧状态裁决，行为与原先的 main 内联版本一致。
func run(ctx context.Context, cfg config.Config, down bool) error {
	db, err := store.Open(ctx, cfg)
	if err != nil {
		return fmt.Errorf("连接数据库失败: %w", err)
	}
	defer db.Close()

	if down {
		if err := db.MigrateDown(ctx); err != nil {
			return fmt.Errorf("回滚失败: %w", err)
		}
		fmt.Println("已回滚一步")
		return nil
	}

	if err := db.Migrate(ctx); err != nil {
		return fmt.Errorf("迁移失败: %w", err)
	}
	fmt.Println("迁移完成")
	return nil
}
