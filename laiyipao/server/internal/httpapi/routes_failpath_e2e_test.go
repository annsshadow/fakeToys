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
	"github.com/laiyipao/server/internal/testutil/scratchdb"
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

// e2eScratchPrefix 是本包一次性库的命名前缀。
// 改动必须同步登记到 scratchdb.Prefixes（见 scratchdb 的文档注释）。
const e2eScratchPrefix = "laiyipao_httpapi_e2e_"

// TestMain 跑测试前清掉本包前缀下的残留库。
//
// 没有它，孤儿库会**累积**：单次跑测留下的孤儿让下一次跑测开始失败，
// 于是「跑得越多越红」。docs/audits/coverage-2026-09-28.md §6 记过这件事
// （曾残留 1049 个），但当时只做了一次性清理，没留下机制，必然复发。
func TestMain(m *testing.M) {
	if n, err := scratchdb.SweepPrefix(e2eScratchPrefix); err != nil {
		fmt.Fprintf(os.Stderr, "[scratchdb] 清理残留库失败（不阻断）：%v\n", err)
	} else if n > 0 {
		fmt.Fprintf(os.Stderr, "[scratchdb] 已清理 %d 个残留测试库（前缀 %s）\n", n, e2eScratchPrefix)
	}
	os.Exit(m.Run())
}

// e2eScratchDB 建一次性库、迁移到顶、灌种子，并让 newE2E 连它。
func e2eScratchDB(t *testing.T) {
	t.Helper()
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()

	// 第 150 轮：建库/清理收敛到 internal/testutil/scratchdb。
	// 旧实现用 `UnixNano()%1e9` 当库名 —— 每秒重复，两个 e2e 用例会拿到
	// 同名库并互删（症状是随机的「查询失败」，与真实缺陷无关）。
	dsn := scratchdb.DSN(t, e2eScratchPrefix)

	// 迁移 scratch 库到顶（e2e 假定 schema 已存在，不做迁移）。
	// 必须用指向 scratch 库的连接迁移，绝不能拿管理库连接迁移
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
