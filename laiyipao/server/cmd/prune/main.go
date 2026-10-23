// Package prune 按保留期清理历史数据（2026-10-23）。
//
// # 为什么需要它
//
// `battle_records` / `replay_verifications` / `admin_audit_logs` 都是
// **只增不减**的表，仓库里此前没有任何清理、归档或分区机制。
// 它们的增长速度是「每局一条」量级，半年后备份体积与索引膨胀会先于
// 功能问题暴露出来 —— 而那时再补，第一批要删的数据已经在生产里了。
//
// # 为什么默认 dry-run
//
// 清理是**不可逆**的，而这个命令最可能被这样使用：
//
//	ssh prod
//	go run ./cmd/prune --days=30        ← 忘了加 --apply 才好
//
// 默认只打印「会删多少行」，`--apply` 才真的删。宁可多一次确认，
// 不要一次「顺手跑一下」。
//
// # 为什么 --days 没有默认值且必须 >= 7
//
// 给默认值等于允许「什么都不想就删」。而下限 7 天是因为：
// 战报要支撑「验真 / 回放 / 客服申诉」三条业务，7 天是能覆盖
// 一次完整申诉周期的最短窗口。
//
// # 为什么不删 wallet_flows
//
// 它是经济系统的**原始凭证**：每一笔货币增减都在这里，删掉之后
// 「这个玩家的金币怎么来的」永远无法追。要压缩它应当归档到冷表
// （见 docs/laiyipao/OPERATIONS.md 的「数据保留」），不是删除。
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

// pruneTarget 是一张可清理的表。
type pruneTarget struct {
	table string
	// why 说明这张表为什么可以删 —— 运维在 dry-run 输出里会看到它。
	// 不写清楚的话，下一个人只会看到一串表名，然后凭感觉加表或删表。
	why string
}

// pruneTargets 是**白名单**。不在这个列表里的表一律不碰 ——
// 包括 wallet_flows（理由见包注释）。
var pruneTargets = []pruneTarget{
	{
		table: "battle_records",
		why:   "战报正文。删除会连带 replay_verifications（ON DELETE CASCADE），且旧战报的回放/验真永久失效",
	},
	{
		table: "replay_verifications",
		why:   "验真记录。仅用于运营排查「谁在什么时候验过哪一场」，不参与结算",
	},
	{
		table: "admin_audit_logs",
		why:   "后台操作审计。保留期应不短于内部合规要求（默认 30 天对多数场景偏短，请按需调大）",
	},
}

// opts 是命令行解析结果。
type opts struct {
	Days   int
	Apply  bool
	Batch  int
	Target string // 空 = 全部；否则只清理这一张
}

// minRetentionDays 是 --days 的硬下限（见包注释）。
const minRetentionDays = 7

// maxBatch 防止一次 DELETE 锁表太久。
const maxBatch = 50000

// parseOpts 解析并校验参数。
//
// 抽成函数是为了能在**没有数据库**的情况下测（本机与 CI 都可能没有 PG，
// 而参数校验恰恰是最该被测的部分 —— 它决定了「误删」能不能发生）。
func parseOpts(args []string) (opts, error) {
	fs := flag.NewFlagSet("prune", flag.ContinueOnError)
	fs.SetOutput(os.Stderr)
	days := fs.Int("days", 0, "保留天数（必填，>= 7）")
	apply := fs.Bool("apply", false, "真的执行删除（默认只统计）")
	batch := fs.Int("batch", 5000, "每批删除行数（避免长事务锁表）")
	target := fs.String("table", "", "只清理这一张表（留空 = 全部白名单表）")
	if err := fs.Parse(args); err != nil {
		return opts{}, err
	}
	if *days == 0 {
		return opts{}, fmt.Errorf("必须显式指定 --days（不给默认值是为了避免「顺手跑一下」删掉数据）")
	}
	if *days < minRetentionDays {
		return opts{}, fmt.Errorf("--days 不得小于 %d（战报要支撑验真/回放/申诉，短于此值会删掉仍在用的数据）",
			minRetentionDays)
	}
	if *batch <= 0 || *batch > maxBatch {
		return opts{}, fmt.Errorf("--batch 必须在 1..%d 之间，实际 %d", maxBatch, *batch)
	}
	if *target != "" && !isKnownTable(*target) {
		return opts{}, fmt.Errorf("未知表 %q（白名单：%v）", *target, knownTables())
	}
	return opts{Days: *days, Apply: *apply, Batch: *batch, Target: *target}, nil
}

func isKnownTable(t string) bool {
	for _, p := range pruneTargets {
		if p.table == t {
			return true
		}
	}
	return false
}

func knownTables() []string {
	out := make([]string, 0, len(pruneTargets))
	for _, p := range pruneTargets {
		out = append(out, p.table)
	}
	return out
}

func main() {
	o, err := parseOpts(os.Args[1:])
	if err != nil {
		fmt.Fprintln(os.Stderr, "参数错误:", err)
		os.Exit(2)
	}

	cfg := config.Load()
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Minute)
	defer cancel()

	db, err := store.Open(ctx, cfg)
	if err != nil {
		fmt.Fprintln(os.Stderr, "连接数据库失败:", err)
		os.Exit(1)
	}
	defer db.Close()

	if err := run(ctx, db, o); err != nil {
		fmt.Fprintln(os.Stderr, "FATAL:", err)
		os.Exit(1)
	}
}

// run 执行清理。返回错误时退出码 1。
func run(ctx context.Context, db *store.DB, o opts) error {
	cutoff := time.Now().AddDate(0, 0, -o.Days)
	mode := "DRY-RUN（不会删除任何数据）"
	if o.Apply {
		mode = "APPLY（会真的删除）"
	}
	fmt.Printf("保留期：%d 天，截止 %s\n模式：%s\n\n", o.Days, cutoff.Format(time.RFC3339), mode)

	for _, t := range pruneTargets {
		if o.Target != "" && t.table != o.Target {
			continue
		}
		// 表名来自白名单常量，不来自用户输入 —— 这里的拼接是安全的。
		var n int64
		if err := db.Pool.QueryRow(ctx,
			`SELECT COUNT(*) FROM `+t.table+` WHERE created_at < $1`, cutoff).Scan(&n); err != nil {
			return fmt.Errorf("统计 %s 失败: %w", t.table, err)
		}
		fmt.Printf("%-22s 待删 %8d 行\n    %s\n", t.table, n, t.why)
		if !o.Apply || n == 0 {
			continue
		}
		deleted, err := deleteInBatches(ctx, db, t.table, cutoff, o.Batch)
		if err != nil {
			return fmt.Errorf("清理 %s 失败（已删 %d 行）: %w", t.table, deleted, err)
		}
		fmt.Printf("%-22s 已删 %8d 行\n", t.table, deleted)
	}

	if !o.Apply {
		fmt.Println("\n以上为统计结果。确认无误后加 --apply 执行。")
		fmt.Println("⚠️ 删除不可逆：battle_records 一旦删除，对应战报的回放与验真将永久失效。")
	}
	return nil
}

// deleteInBatches 分批删除，避免一个长事务锁住整张表。
//
// 用 `id IN (SELECT ... LIMIT n)` 而不是 `DELETE ... LIMIT`：
// 后者是 MySQL 方言，PostgreSQL 不支持。
func deleteInBatches(ctx context.Context, db *store.DB, table string, cutoff time.Time, batch int) (int64, error) {
	var total int64
	for {
		// 同样：table 来自白名单常量。
		tag, err := db.Pool.Exec(ctx,
			`DELETE FROM `+table+` WHERE id IN (
			   SELECT id FROM `+table+` WHERE created_at < $1 ORDER BY id LIMIT $2
			 )`, cutoff, batch)
		if err != nil {
			return total, err
		}
		n := tag.RowsAffected()
		total += n
		if n == 0 {
			return total, nil
		}
		// 每批之间让出一次，避免长时间占住连接池里的同一条连接
		select {
		case <-ctx.Done():
			return total, ctx.Err()
		case <-time.After(50 * time.Millisecond):
		}
	}
}
