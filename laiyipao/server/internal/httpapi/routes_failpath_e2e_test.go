package httpapi

// 「同请求内先成功后失败」路径的端到端测试。
//
// upgradeSkill 的响应要把升级后的余额一并回给客户端（避免前端再发一次
// /wallet 的竞态），于是 handler 里有第二次查询：UpgradeSkill 成功后
// LoadWallet 失败 → 500。
//
// 这个分支在共享开发库上不可构造：两笔查询都读同一个 user_wallets。
// 这里用一次性 scratch 库 + 列重命名构造真实失败：
//   - UpgradeSkill 只写 user_skills 与 user_wallets.coin（不引用 keys 列）
//   - LoadWallet SELECT coin, gem, energy, keys, ... （引用 keys）
// 把 keys 改名后，升级照常成功、回读余额必然报"列不存在" → 走 500 分支。
//
// 为什么是 scratch 库而不是开发库上临时改名：并行会话可能同时在跑测试，
// 改共享表的列名会让别人的用例在改名窗口内随机爆炸。

import (
	"context"
	"fmt"
	"net/url"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/seeder"
	"github.com/laiyipao/server/internal/store"
)

// e2eAdminDSN 返回连到 postgres 系统库的 DSN（CREATE/DROP DATABASE 用）。
func e2eAdminDSN() string {
	if dsn := os.Getenv("TEST_DATABASE_URL"); dsn != "" {
		return strings.Replace(dsn, "/laiyipao", "/postgres", 1)
	}
	return "postgres://postgres@127.0.0.1:5432/postgres?sslmode=disable"
}

// e2eWithDBName 把 DSN 的库名替换为 name。
func e2eWithDBName(dsn, name string) string {
	u, err := url.Parse(dsn)
	if err != nil {
		return dsn
	}
	u.Path = "/" + name
	return u.String()
}

// e2eScratchDB 建一次性库、迁移到顶，并让 newE2E 连它。
func e2eScratchDB(t *testing.T) {
	t.Helper()
	admin := e2eAdminDSN()
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()

	cfg := config.Load()
	cfg.DatabaseURL = admin
	db, err := store.Open(ctx, cfg)
	if err != nil {
		t.Skipf("连不上管理库，跳过：%v", err)
	}
	name := fmt.Sprintf("laiyipao_httpapi_e2e_%d", time.Now().UnixNano()%1_000_000_000)
	if _, err := db.Pool.Exec(ctx, `CREATE DATABASE `+name); err != nil {
		db.Close()
		t.Skipf("无权创建测试库，跳过：%v", err)
	}
	dsn := e2eWithDBName(admin, name)
	t.Cleanup(func() {
		dropCtx, dropCancel := context.WithTimeout(context.Background(), 15*time.Second)
		defer dropCancel()
		_, _ = db.Pool.Exec(dropCtx,
			`SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1`, name)
		_, _ = db.Pool.Exec(dropCtx, `DROP DATABASE IF EXISTS `+name)
		db.Close()
	})

	// 迁移 scratch 库到顶（e2e 假定 schema 已存在，不做迁移）。
	// 必须用指向 scratch 库的**独立连接**迁移，绝不能拿 admin 连接迁移
	// —— 那会把系统库 postgres 迁到最新版本。
	scratchCfg := config.Load()
	scratchCfg.DatabaseURL = dsn
	scratch, err := store.Open(ctx, scratchCfg)
	if err != nil {
		t.Fatalf("scratch 库连不上：%v", err)
	}
	if err := scratch.Migrate(ctx); err != nil {
		scratch.Close()
		t.Fatalf("迁移 scratch 库失败：%v", err)
	}
	// e2e 会给新号铺装备/技能，缺种子数据会外键失败 —— 必须先种子
	if _, err := seeder.Run(ctx, scratch.Pool); err != nil {
		scratch.Close()
		t.Fatalf("scratch 库种子失败：%v", err)
	}
	scratch.Close()
	t.Setenv("TEST_DATABASE_URL", dsn)
}

func TestE2EUpgradeSkillWalletLoadFails(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "walletpath")

	// 升级技能 1（新号默认拥有，2000 金币远超一级成本）
	e.exec(t, `UPDATE user_wallets SET coin = 2000 WHERE user_id = $1`, uid)

	// 构造失败：LoadWallet 引用的 keys 列改名。
	// 注册恢复逻辑要赶在 e2eScratchDB 的 DROP 清理之前执行（t.Cleanup LIFO），
	// 这里与共享库改名同等谨慎：任何退出路径都先把列名还原。
	e.exec(t, `ALTER TABLE user_wallets RENAME COLUMN keys TO keys_renamed_for_test`)
	renameBack := func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `ALTER TABLE user_wallets RENAME COLUMN keys_renamed_for_test TO keys`)
	}
	t.Cleanup(renameBack)

	status, body := e.post(t, "/api/v1/me/skills/1/upgrade", tok, map[string]any{})
	if status != fiber.StatusInternalServerError {
		t.Fatalf("升级成功但回读余额失败应 500，实际 %d %v", status, body)
	}
	if body["error"].(map[string]any)["code"] != "internal_error" {
		t.Errorf("应返回 internal_error，实际 %v", body)
	}

	// 还原后同一玩家再升级：必须 200（证明 500 只来自余额回读，
	// 而不是升级本身坏了）
	renameBack()
	status, body = e.post(t, "/api/v1/me/skills/1/upgrade", tok, map[string]any{})
	if status != fiber.StatusOK {
		t.Fatalf("列名还原后升级应 200，实际 %d %v", status, body)
	}
	if got := body["level"].(float64); got != 3 {
		// 第一次升级已成功提交（新等级 2），这次是第二次 → 3
		t.Errorf("两次升级后等级应为 3，实际 %v", got)
	}
}
