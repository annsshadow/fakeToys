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

	db, err := store.Open(ctx, cfg)
	if err != nil {
		fmt.Fprintln(os.Stderr, "FATAL 连接数据库失败:", err)
		os.Exit(1)
	}
	defer db.Close()

	if *down {
		if err := db.MigrateDown(ctx); err != nil {
			fmt.Fprintln(os.Stderr, "FATAL 回滚失败:", err)
			os.Exit(1)
		}
		fmt.Println("已回滚一步")
		return
	}

	if err := db.Migrate(ctx); err != nil {
		fmt.Fprintln(os.Stderr, "FATAL 迁移失败:", err)
		os.Exit(1)
	}
	fmt.Println("迁移完成")
}
