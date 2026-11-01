package main

// wallet_flows 归档的数据库集成测试（2026-10-30 实质优化第 10 轮）。
//
// 归档与删除不同：老流水不是消失，而是搬进冷表 wallet_flows_archive。
// 判据：
//   - 不带 --archive-wallet-flows 时，wallet_flows 一行都不动；
//   - dry-run 只统计，原表数据不变；
//   - apply 把超过保留期的流水原子搬进冷表（原表删除、归档表出现）；
//   - 重复 apply 是幂等的（第二次 0 行）。

import (
	"bytes"
	"context"
	"fmt"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/seeder"
	"github.com/laiyipao/server/internal/store"
	"github.com/laiyipao/server/internal/testutil/scratchdb"
)

// pruneScratchPrefix 是本包一次性库的命名前缀。
// 改动必须同步登记到 scratchdb.Prefixes（见 scratchdb 的文档注释）。
const pruneScratchPrefix = "laiyipao_prune_test_"

func TestMain(m *testing.M) {
	if n, err := scratchdb.SweepPrefix(pruneScratchPrefix); err != nil {
		fmt.Fprintf(os.Stderr, "[prune] 清理残留测试库失败（不阻断）：%v\n", err)
	} else if n > 0 {
		fmt.Fprintf(os.Stderr, "[prune] 已清理 %d 个残留测试库（前缀 %s）\n", n, pruneScratchPrefix)
	}
	os.Exit(m.Run())
}

// openPruneScratchDB 建一次性库、迁移到顶、灌种子，返回 store.DB。
func openPruneScratchDB(t *testing.T) *store.DB {
	t.Helper()
	ctx, cancel := context.WithTimeout(context.Background(), 120*time.Second)
	defer cancel()

	dsn := scratchdb.DSN(t, pruneScratchPrefix)
	cfg := config.Load()
	cfg.DatabaseURL = dsn
	cfg.DBMaxConns = 4
	cfg.DBMinConns = 1

	db, err := store.Open(ctx, cfg)
	if err != nil {
		t.Skipf("scratch 库连不上，跳过：%v", err)
	}
	t.Cleanup(db.Close)
	if err := db.Migrate(ctx); err != nil {
		t.Fatalf("迁移失败：%v", err)
	}
	if _, err := seeder.Run(ctx, db.Pool); err != nil {
		t.Fatalf("种子失败：%v", err)
	}
	return db
}

// seedWalletFlows 插入一个用户和两条流水：一条 8 天前（应归档），一条昨天（应保留）。
// 返回用户 id。
func seedWalletFlows(t *testing.T, db *store.DB) int64 {
	t.Helper()
	ctx := context.Background()

	var uid int64
	if err := db.Pool.QueryRow(ctx,
		`INSERT INTO users (guest_token, nickname, is_guest, last_login_at)
		 VALUES ($1, $2, TRUE, now())
		 RETURNING id`,
		fmt.Sprintf("prune_guest_%d", time.Now().UnixNano()), "归档测试").Scan(&uid); err != nil {
		t.Fatalf("插入用户失败：%v", err)
	}

	old := time.Now().AddDate(0, 0, -8)
	recent := time.Now().AddDate(0, 0, -1)
	for _, f := range []struct {
		created time.Time
		delta   int64
	}{
		{old, 100},
		{old, 200},
		{recent, -50},
		{recent, 300},
	} {
		if _, err := db.Pool.Exec(ctx,
			`INSERT INTO wallet_flows (user_id, currency, delta, balance, reason, created_at)
			 VALUES ($1, 'coin', $2, $2, 'prune_test', $3)`, uid, f.delta, f.created); err != nil {
			t.Fatalf("插入流水失败：%v", err)
		}
	}
	return uid
}

func countTable(t *testing.T, db *store.DB, table string) int64 {
	t.Helper()
	// 归档表在 dry-run / 未开启归档时可能还没创建 —— 先查 pg_class，
	// 不存在按 0 处理（避免 42P01 把「表不存在」误报成测试失败）。
	var reg *string
	if err := db.Pool.QueryRow(context.Background(),
		`SELECT to_regclass($1)::text`, table).Scan(&reg); err != nil {
		t.Fatalf("检查表 %s 是否存在失败：%v", table, err)
	}
	if reg == nil {
		return 0
	}
	var n int64
	if err := db.Pool.QueryRow(context.Background(),
		`SELECT COUNT(*) FROM `+table).Scan(&n); err != nil {
		t.Fatalf("统计 %s 失败：%v", table, err)
	}
	return n
}

func TestArchiveWalletFlowsDryRunMovesNothing(t *testing.T) {
	db := openPruneScratchDB(t)
	ctx := context.Background()
	seedWalletFlows(t, db)

	var out bytes.Buffer
	o := opts{Days: 7, Apply: false, Batch: 100, ArchiveWallet: true}
	if err := run(ctx, db, o, &out); err != nil {
		t.Fatalf("dry-run 失败：%v", err)
	}
	if !strings.Contains(out.String(), "待归档") || !strings.Contains(out.String(), "wallet_flows") {
		t.Errorf("输出应包含 wallet_flows 归档统计，实际：\n%s", out.String())
	}
	if got := countTable(t, db, "wallet_flows"); got != 4 {
		t.Errorf("dry-run 后主表应有 4 行，实际 %d", got)
	}
	if got := countTable(t, db, "wallet_flows_archive"); got != 0 {
		t.Errorf("dry-run 后归档表应为 0 行，实际 %d", got)
	}
}

func TestArchiveWalletFlowsApplyMovesOldRows(t *testing.T) {
	db := openPruneScratchDB(t)
	ctx := context.Background()
	seedWalletFlows(t, db)

	var out bytes.Buffer
	o := opts{Days: 7, Apply: true, Batch: 100, ArchiveWallet: true}
	if err := run(ctx, db, o, &out); err != nil {
		t.Fatalf("apply 失败：%v", err)
	}
	if !strings.Contains(out.String(), "已归档") {
		t.Errorf("输出应包含已归档统计，实际：\n%s", out.String())
	}

	// 8 天前的 2 行搬走，昨天的 2 行保留
	if got := countTable(t, db, "wallet_flows"); got != 2 {
		t.Errorf("主表应剩 2 行，实际 %d", got)
	}
	if got := countTable(t, db, "wallet_flows_archive"); got != 2 {
		t.Errorf("归档表应有 2 行，实际 %d", got)
	}

	// 再次 apply：幂等，0 行新增
	var out2 bytes.Buffer
	if err := run(ctx, db, o, &out2); err != nil {
		t.Fatalf("第二次 apply 失败：%v", err)
	}
	if got := countTable(t, db, "wallet_flows_archive"); got != 2 {
		t.Errorf("重复归档后归档表应仍是 2 行，实际 %d", got)
	}
}

func TestRunWithoutArchiveLeavesWalletFlowsUntouched(t *testing.T) {
	db := openPruneScratchDB(t)
	ctx := context.Background()
	seedWalletFlows(t, db)

	var out bytes.Buffer
	// 不带 --archive-wallet-flows（默认关闭），即使 --apply 也不碰流水
	o := opts{Days: 7, Apply: true, Batch: 100, ArchiveWallet: false}
	if err := run(ctx, db, o, &out); err != nil {
		t.Fatalf("run 失败：%v", err)
	}
	if strings.Contains(out.String(), "wallet_flows") {
		t.Errorf("未开启归档时输出不应提及 wallet_flows：\n%s", out.String())
	}
	if got := countTable(t, db, "wallet_flows"); got != 4 {
		t.Errorf("未开启归档时主表应保持 4 行，实际 %d", got)
	}
	if got := countTable(t, db, "wallet_flows_archive"); got != 0 {
		t.Errorf("未开启归档时归档表应不存在/为 0 行，实际 %d", got)
	}
}
