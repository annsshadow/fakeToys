// Package main 写入全部游戏内容到数据库。
// 用法：go run ./cmd/seed
package main

import (
	"context"
	"fmt"
	"os"
	"time"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/seeder"
	"github.com/laiyipao/server/internal/store"
)

func main() {
	cfg := config.Load()
	fmt.Println("config:", cfg.Redacted())

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Minute)
	defer cancel()

	db, err := store.Open(ctx, cfg)
	if err != nil {
		fmt.Fprintln(os.Stderr, "FATAL 连接数据库失败:", err)
		os.Exit(1)
	}
	defer db.Close()

	if err := db.Migrate(ctx); err != nil {
		fmt.Fprintln(os.Stderr, "FATAL 迁移失败:", err)
		os.Exit(1)
	}
	fmt.Println("迁移已是最新，开始写入游戏内容...")

	res, err := seeder.Run(ctx, db.Pool)
	if err != nil {
		fmt.Fprintln(os.Stderr, "FATAL 种子写入失败:", err)
		os.Exit(1)
	}

	fmt.Println("写入完成：")
	fmt.Printf("  敌人 %d（含五维抗性）\n", res.Enemies)
	fmt.Printf("  技能 %d / 合成配方 %d\n", res.Skills, res.Recipes)
	fmt.Printf("  装备 %d / 宝石 %d / 皮肤 %d\n", res.Equipment, res.Gems, res.Skins)
	fmt.Printf("  专精树节点 %d\n", res.Mastery)
	fmt.Printf("  关卡 %d / 波次 %d\n", res.Levels, res.Waves)
	fmt.Printf("  签到 %d 天 / 任务 %d / 商城 %d 项\n", res.SignInDays, res.Tasks, res.ShopItems)
}
