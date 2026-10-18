// Package scratchdb 统一管理测试用的「一次性库」。
//
// # 它解决的是三个真实故障（第 150 轮）
//
// ① **库名每秒重复**
//
// 旧实现是 `fmt.Sprintf("laiyipao_svc_test_%d", time.Now().UnixNano()%1_000_000_000)`。
// `UnixNano()` 是自 epoch 起的纳秒数，`% 1e9` 取的是**最后 9 位** ——
// 也就是秒以下的部分，于是这个名字**每秒钟就绕回一次**。
// 两个相隔整一秒的测试会拿到完全相同的库名：
// 一个的 `t.Cleanup` 把库 DROP 掉，另一个还在往里写。
// 症状不是超时，而是**下一个测试的种子 INSERT 撞主键**（SQLSTATE 23505）。
//
// ② **DROP 超时留下孤儿库，累积到把 PG 压垮**
//
// `t.Cleanup` 里的 DROP 只有 10 秒预算。库一多，PG 变慢，DROP 超时，
// 孤儿就留下了。几百个孤儿库之后，`CREATE DATABASE` + 迁移 + 种子
// 会在资源耗尽的位置失败。
// `docs/laiyipao/audits/coverage-2026-09-28.md` §6 记过这件事（「曾残留 1049 个」），
// 但当时只做了「一次性清理」，**没有留下机制**，于是必然复发。
//
// ③ **建库失败会静默跳过**
//
// 旧实现是 `t.Skipf("无权创建测试库，跳过：%v", err)` ——
// 任何 CREATE 失败（包括 ② 的资源耗尽）都被当成「没权限」而跳过。
// 于是「测试全绿」与「一半用例根本没跑」无法区分。
// 这正是 AGENTS.md 第 11 条说的那种假绿。
//
// 现在：`Sweep` 在每个测试二进制启动前清干净；名字用进程内计数器保证唯一；
// 建库失败**分类**处理 —— 确实没有权限才跳过，其余一律 `t.Fatalf`。
package scratchdb

import (
	"context"
	"errors"
	"fmt"
	"os"
	"strings"
	"sync/atomic"
	"testing"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"
)

// Prefixes 是本项目所有一次性库的命名前缀。
//
// ⚠️ **必须与各测试文件实际用的前缀一致**，否则 Sweep 清不到。
// `TestSweepCoversEveryPrefix` 用反射从各包的 scratch 前缀常量做对拍，
// 少一个就在那里红 —— 因为「前缀写错」的症状是「孤儿库照样累积」，
// 而那要跑到几百个库之后才显形。
var Prefixes = []string{
	"laiyipao_svc_test_",
	"laiyipao_seeder_test_",
	"laiyipao_seed_test_",
	"laiyipao_seedcmd_test_",
	"laiyipao_mig_test_",
	"laiyipao_store_test_",
	"laiyipao_httpapi_e2e_",
}

var seq atomic.Int64

// Name 生成一个进程内唯一的库名。
//
// 组成：`前缀 + 进程 PID + 计数器`。三者合起来保证：
//   - 同一次 `go test` 内不重复（计数器）
//   - 不同进程之间不重复（PID）
//
// ⚠️ **不要**改回 `time.Now()` 派生的名字：那正是①的成因。
func Name(prefix string) string {
	return fmt.Sprintf("%s%d_%d", prefix, os.Getpid(), seq.Add(1))
}

// AdminDSN 从 DATABASE_URL 推导出指向管理库 `postgres` 的 DSN。
func AdminDSN() string {
	dsn := os.Getenv("TEST_DATABASE_URL")
	if dsn == "" {
		dsn = os.Getenv("DATABASE_URL")
	}
	if dsn == "" {
		return "postgres://postgres@127.0.0.1:5432/postgres?sslmode=disable"
	}
	if i := strings.LastIndex(dsn, "/"); i >= 0 {
		if j := strings.Index(dsn[i+1:], "?"); j >= 0 {
			return dsn[:i+1] + "postgres" + dsn[i+1+j:]
		}
		return dsn[:i+1] + "postgres"
	}
	return dsn
}

// swapDB 把管理库 DSN 里的库名换成别的库。
func swapDB(admin, name string) string {
	if i := strings.LastIndex(admin, "/"); i >= 0 {
		if j := strings.Index(admin[i+1:], "?"); j >= 0 {
			return admin[:i+1] + name + admin[i+1+j:]
		}
		return admin[:i+1] + name
	}
	return admin
}

// pool 打开一个到管理库的短命连接池；失败时返回 nil（调用方决定怎么处理）。
func pool(ctx context.Context) (*pgxpool.Pool, error) {
	admin := AdminDSN()
	p, err := pgxpool.New(ctx, admin)
	if err != nil {
		return nil, err
	}
	if err := p.Ping(ctx); err != nil {
		p.Close()
		return nil, err
	}
	return p, nil
}

// canCreateDatabases 探测当前账号有没有 CREATEDB 权限。
//
// 只在 CREATE 真的失败**之后**才调（而不是每次先探一次）——
// 每个测试用例建一次库，多一次 CREATE+DROP 是白花的负载，
// 而在孤儿库把 PG 压垮的时候恰恰是**最不能多给**的时候。
func canCreateDatabases(ctx context.Context, p *pgxpool.Pool) bool {
	probe := fmt.Sprintf("laiyipao_probe_%d_%d", os.Getpid(), seq.Add(1))
	if _, err := p.Exec(ctx, `CREATE DATABASE `+quoteIdent(probe)); err != nil {
		return false
	}
	_, _ = p.Exec(ctx, `DROP DATABASE IF EXISTS `+quoteIdent(probe))
	return true
}

// Sweep 删掉**本包前缀**下的所有残留一次性库；失败即 `t.Fatalf`。
func Sweep(t *testing.T, prefix string) {
	t.Helper()
	n, err := SweepPrefix(prefix)
	if err != nil {
		t.Fatalf("清理残留测试库失败：%v", err)
	}
	if n > 0 {
		t.Logf("已清理 %d 个残留测试库（前缀 %s）", n, prefix)
	}
}

// SweepPrefix 是 Sweep 的无 testing 依赖版本，供 `TestMain` 调用。
//
// 为什么单独开一个：`TestMain` 里拿不到 `*testing.T`，而
// 「造一个伪 T」会把 Fatal 的语义搅浑 —— 所以底层函数只返回 error，
// 由调用点决定是 Skip 还是 Fatal。
func SweepPrefix(prefix string) (int, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Minute)
	defer cancel()

	p, err := pool(ctx)
	if err != nil {
		// 管理库不可达不是本包的失败：后续 DB 用例会各自判定要不要跳过。
		return 0, nil
	}
	defer p.Close()

	rows, err := p.Query(ctx,
		`SELECT datname FROM pg_database WHERE datname LIKE $1`, prefix+"%")
	if err != nil {
		return 0, fmt.Errorf("列残留测试库失败: %w", err)
	}
	var names []string
	for rows.Next() {
		var n string
		if err := rows.Scan(&n); err != nil {
			rows.Close()
			return 0, fmt.Errorf("读残留测试库名失败: %w", err)
		}
		names = append(names, n)
	}
	rows.Close()
	if err := rows.Err(); err != nil {
		return 0, fmt.Errorf("遍历残留测试库失败: %w", err)
	}
	if len(names) == 0 {
		return 0, nil
	}
	// 先掐连接：DROP DATABASE 在有连接时会失败，而残留库里常常挂着
	// 上一次被 kill 掉的测试进程留下的连接。
	if _, err := p.Exec(ctx,
		`SELECT pg_terminate_backend(pid) FROM pg_stat_activity
		  WHERE datname = ANY($1)`, names); err != nil {
		return 0, fmt.Errorf("断开残留测试库连接失败: %w", err)
	}
	for _, n := range names {
		if _, err := p.Exec(ctx, `DROP DATABASE IF EXISTS `+quoteIdent(n)); err != nil {
			return 0, fmt.Errorf("删除残留测试库 %s 失败: %w", n, err)
		}
	}
	return len(names), nil
}

// quoteIdent 给库名加双引号。
//
// 库名由本包生成（只含字母数字与下划线），理论上不需要转义；
// 但 `DROP DATABASE` 是**不能参数化**的语句，所以只能拼 ——
// 于是必须转义，否则一个带引号的库名就能拼成注入。
func quoteIdent(s string) string {
	return `"` + strings.ReplaceAll(s, `"`, `""`) + `"`
}

// ErrNoPermission 表示当前账号没有 CREATEDB 权限 —— 这是「本机没条件跑 DB 用例」，
// 与「真故障」必须分开，否则资源耗尽会伪装成「测试全绿、用例全跳过」。
var ErrNoPermission = errors.New("没有 CREATEDB 权限")

// DSN 建一次性库并返回指向它的 DSN。
//
// 失败语义（③）：
//   - 管理库不可达 / 没有 CREATEDB 权限 → `t.Skip`（这是「本机没条件跑 DB 用例」，
//     README 明确要求 CI 无 PG 时自行跳过）
//   - 其它任何失败 → `t.Fatal`（这是真故障，静默跳过等于假装测过了）
func DSN(t *testing.T, prefix string) string {
	t.Helper()
	dsn, cleanup, err := Open(prefix)
	if err != nil {
		if errors.Is(err, ErrNoPermission) {
			t.Skip(err.Error())
		}
		t.Fatalf("建一次性库失败：%v", err)
	}
	t.Cleanup(cleanup)
	return dsn
}

// Open 是 DSN 的**无 testing 依赖**版本，供 `TestMain` 构造「整包共用」的库。
//
// 为什么需要它：包级 `TestMain` 里拿不到 `*testing.T`，而造一个伪 T
// 会把 Skip/Fatal 的语义搅浑 —— 所以底层只返回 error 与 cleanup，
// 由调用点自己决定怎么表达失败。
//
// 典型用法（`internal/service` 用它替掉「打开发库当测试库」）：
//
//	var pkgDSN string
//	func TestMain(m *testing.M) {
//	    dsn, cleanup, err := scratchdb.Open(scratchPrefix + "shared")
//	    if err == nil {
//	        pkgDSN = dsn
//	        defer cleanup()
//	    }
//	    os.Exit(m.Run())
//	}
//
// 每轮跑测于是都从一张**干净**的库开始，而不是在开发库上层层堆积。
func Open(prefix string) (string, func(), error) {
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()

	p, err := pool(ctx)
	if err != nil {
		return "", nil, err
	}
	defer p.Close()

	name := Name(prefix)
	if _, err := p.Exec(ctx, `CREATE DATABASE `+quoteIdent(name)); err != nil {
		// 分清「本机没条件」与「真故障」。旧实现一律 Skip，
		// 于是「孤儿库把 PG 压垮」这种真故障会伪装成「测试全绿、用例全跳过」。
		if !canCreateDatabases(ctx, p) {
			return "", nil, ErrNoPermission
		}
		return "", nil, fmt.Errorf("创建一次性库 %s 失败：%w —— %s",
			name, err, cleanupHint(prefix))
	}

	cleanup := func() {
		dropCtx, dropCancel := context.WithTimeout(context.Background(), 60*time.Second)
		defer dropCancel()
		// ⚠️ 不能复用上面那个池：`defer p.Close()` 在函数返回时就执行了，
		// 而清理是**之后**才跑的 —— 到这里 p 早已关闭，
		// 每次 DROP 都会返回 `closed pool`，于是**每个库都留成孤儿**。
		//
		// 这个 bug 极隐蔽：清理「看起来写了」、Sweep 也在跑，
		// 只是残留量悄悄回到几百个 —— 正好是这个包要根治的那个症状。
		// 是新加的失败日志把它喊出来的（`[scratchdb] 删除 ... 失败：closed pool`）。
		dp, err := pool(dropCtx)
		if err != nil {
			fmt.Fprintf(os.Stderr,
				"[scratchdb] 连不上管理库，无法删除 %s（下轮 TestMain 会清）：%v\n", name, err)
			return
		}
		defer dp.Close()
		_, _ = dp.Exec(dropCtx,
			`SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1`, name)
		if _, err := dp.Exec(dropCtx, `DROP DATABASE IF EXISTS `+quoteIdent(name)); err != nil {
			// 不吞：残留是**累积性**故障（跑得越多越红）。Sweep 是兜底，
			// 不是这条路径的替代品，所以这里必须让它可见 ——
			// 否则「测试全绿但库越堆越多」又变成一个几百个库之后
			// 才显形、且没人查得到原因的问题。
			fmt.Fprintf(os.Stderr,
				"[scratchdb] 删除 %s 失败（下轮 TestMain 会清）：%v\n", name, err)
		}
	}
	return swapDB(AdminDSN(), name), cleanup, nil
}

func cleanupHint(prefix string) string {
	return fmt.Sprintf(
		"先清掉 %s* 的残留库（CREATE DATABASE 会随库数增长而超时），"+
			"再重跑；TestMain 已内置这一步，若仍失败说明有别的进程正占着这些库", prefix)
}
