package service

import (
	"context"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
	"testing"
	"time"

	"github.com/jackc/pgx/v5"
)

// 种子里出现的**每个** task metric 都必须在某个 `bumpTasks` 调用点出现（第 91 轮）。
//
// # 缺陷：任务「每日签到」永远无法推进、永远领不到
//
// 种子里有：
//
//	{4, 1, "daily_signin", "完成每日签到", "daily", "signin", …}
//
// 而 `bumpTasks` 全仓库**只有一个调用点**（`game.go` 的结算路径），
// 它传的 metric 只有 4 个：`kills` / `clears` / `reactions` / `max_stage`。
// **没有 `signin`**。
//
// 于是这条任务的 `progress` 永远是 0：玩家每天签到、每天都看到它卡在 0/1、
// 永远领不到那 500 金币。
//
// # 为什么既有测试没抓到
//
// 既有测试都是「先**造**一行 progress，再 ClaimTask」——
// **完全绕过了 bump 这一步**。于是「bump 从不发生」这件事在测试里不可见。
//
// # 为什么这条守卫是结构性的而不是枚举式的
//
// 手写「signin 必须被 bump」这种断言，加第 13 条任务时就会有人忘了加。
// 而**「种子里出现的每个 metric 都必须有 bump 站点」**是一个闭合的性质：
// 它在新增任务时自动生效，不需要有人记得更新清单。
//
// 与第 67/74/75/78/82 轮记的完全同型：**守卫要覆盖那类情况，而不是那几行**。
//
// # 判据只数「出现过」，不看 delta 的值
//
// 一个 metric 出现在某个调用点的 map 字面量里就算「有站点」。
// 值算错（比如把 +1 写成 -1）那是另一类问题，
// 本轮**测不到** —— 写在这里而不是留给后来的人猜。
func TestEveryTaskMetricHasABumpSite(t *testing.T) {
	seeded := seededMetrics(t)
	bumped := bumpedMetrics(t)

	missing := []string{}
	checked := 0
	for _, m := range seeded {
		checked++
		if !bumped[m] {
			missing = append(missing, m)
		}
	}
	sort.Strings(missing)

	// ⚠️ 自证：**检查过的数量必须等于 metric 总数**（第 91 轮变异逼出来的）。
	//
	// 变异「把循环改成 `seeded[0:4]`」→ **全绿**。
	// 因为 `seeded` 是排序后的 [clears kills max_stage reactions signin]，
	// 前 4 个恰好都有 bump 站点，被漏掉的第 5 个就是 `signin`。
	//
	// 也就是说：**这条守卫可以被「少检查几个」而悄悄失效**，
	// 而且失效方式看起来和「全部合规」一模一样。
	//
	// 这与第 78/82 轮「扫不到就 Fatal」是同一个问题的另一面 ——
	// 那次是「扫到 0 个」，这次是「扫少了几个」。
	// 两种都靠**计数**而不是「结果为空」来区分。
	if checked != len(seeded) {
		t.Fatalf("守卫只检查了 %d 个 metric，而种子里有 %d 个（%v）\n"+
			"「检查得少」与「全部合规」在输出里长得一模一样 —— "+
			"第 91 轮的实测：把循环改成 seeded[0:4] 就全绿了，而漏掉的正是 %s。\n"+
			"这是守卫**自身**被削弱，不是代码合规。",
			checked, len(seeded), seeded, lastOne(seeded))
	}

	if len(missing) == 0 {
		t.Logf("种子里 %d 个 metric 全部有 bump 站点：%v", len(seeded), seeded)
		return
	}

	t.Errorf("以下 metric 在种子里存在，但**没有任何 bumpTasks 调用点会推进它**：\n  %s\n\n"+
		"后果：对应的任务 progress 永远是 0，玩家永远领不到奖励，"+
		"而且界面上看不出任何异常（它只是停在 0）。\n\n"+
		"修法二选一：(a) 在对应的事件路径上调用 bumpTasks；"+
		"(b) 如果这条任务本来就不该存在，把它从种子里去掉。\n\n"+
		"⚠️ 既有测试抓不到这个 —— 它们都是「先造 progress 再 ClaimTask」，"+
		"绕过了 bump 那一步。",
		strings.Join(missing, "\n  "))
}

// seededMetrics 从 seeder 源码里取出 `seedTasks` 用的 metric 集合。
//
// ⚠️ 用 AST 而不是正则：注释里也会出现 metric 名
// （第 76 轮那条 `task_date = periodStart(now, "daily")` 的注释就是例子），
// 正则会把注释里的字面量当成数据 ——
// 这与第 83/88 轮撞过的「文本扫描分不清注释与代码」是同一面墙。
func seededMetrics(t *testing.T) []string {
	t.Helper()
	fset := token.NewFileSet()
	// internal/service -> internal/seeder 是**一级**不是两级。
	// ⚠️ 我第一版写成 `../..`，于是解析失败 —— 而那个失败被后续
	// 「一个站点都没扫到」的断言接手，报成了「扫描器坏了」，
	// 把我真正的 bug（参数取错下标）伪装成另一个问题。
	path := filepath.Join("..", "seeder", "seed.go")
	f, err := parser.ParseFile(fset, path, nil, 0)
	if err != nil {
		t.Fatalf("解析 seeder/seed.go 失败：%v", err)
	}

	set := map[string]bool{}
	ast.Inspect(f, func(n ast.Node) bool {
		kv, ok := n.(*ast.KeyValueExpr)
		if !ok {
			return true
		}
		lit, ok := kv.Value.(*ast.BasicLit)
		if !ok || lit.Kind != token.STRING {
			return true
		}
		v, err := strconv.Unquote(lit.Value)
		if err != nil {
			return true
		}
		// seedTasks 的结构体字段顺序是 id,target,code,name,scope,metric,reward
		// 第 6 个字符串字段就是 metric。用位置判断：scope 的取值只有
		// daily/weekly/achievement，可据此定位。
		if v == "daily" || v == "weekly" || v == "achievement" {
			if ident, ok := kv.Key.(*ast.Ident); ok && ident.Name == "scope" {
				// 这是 scope，紧接着的下一个字段才是 metric；
				// 这里改为在整行里找 —— 见下方 collect。
				_ = v
			}
		}
		return true
	})

	// 收集具名结构体字面量：seedTasks 里 `tasks := []struct{ … }{ {1, 30, …}, … }`
	// 每个元素是位置字面量（无字段名），所以按**位置**取第 6 个字符串。
	ast.Inspect(f, func(n ast.Node) bool {
		cl, ok := n.(*ast.CompositeLit)
		if !ok || len(cl.Elts) < 6 {
			return true
		}
		elts := make([]string, 0, 6)
		for _, e := range cl.Elts {
			switch v := e.(type) {
			case *ast.BasicLit:
				if v.Kind == token.STRING {
					s, err := strconv.Unquote(v.Value)
					if err != nil {
						return true
					}
					elts = append(elts, s)
				} else {
					elts = append(elts, "")
				}
			default:
				elts = append(elts, "")
			}
		}
		// elts[4] 若是 scope，则 elts[5] 是 metric
		if len(elts) >= 6 && (elts[4] == "daily" || elts[4] == "weekly" || elts[4] == "achievement") {
			set[elts[5]] = true
		}
		return true
	})

	out := make([]string, 0, len(set))
	for k := range set {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}

// bumpedMetrics 用 AST 扫出所有 `bumpTasks(…, map[string]int64{…}, …)` 的 key。
func bumpedMetrics(t *testing.T) map[string]bool {
	t.Helper()
	fset := token.NewFileSet()
	entries, err := os.ReadDir(".")
	if err != nil {
		t.Fatalf("读包目录失败：%v", err)
	}

	set := map[string]bool{}
	for _, e := range entries {
		name := e.Name()
		if e.IsDir() || !strings.HasSuffix(name, ".go") || strings.HasSuffix(name, "_test.go") {
			continue
		}
		f, err := parser.ParseFile(fset, filepath.Join(".", name), nil, 0)
		if err != nil {
			t.Fatalf("解析 %s 失败：%v", name, err)
		}
		ast.Inspect(f, func(n ast.Node) bool {
			call, ok := n.(*ast.CallExpr)
			if !ok {
				return true
			}
			sel, ok := call.Fun.(*ast.SelectorExpr)
			if !ok || sel.Sel.Name != "bumpTasks" {
				return true
			}
			// ⚠️ **遍历全部参数**找 map 字面量，不要按下标取。
			//
			// 签名是 `bumpTasks(ctx, tx, userID, deltas, now)`，
			// map 是**第 4 个**（下标 3）。
			// 我第一版取 `Args[1]`（那是 `tx`）→ 一个站点都扫不到 →
			// 报「扫描器坏了」。
			//
			// 而「按位置取参数」本身就是脆的：改一次签名就错，
			// 且失败方式是**静默地扫到 0 个**，看起来像合规。
			for _, arg := range call.Args {
				cl, ok := arg.(*ast.CompositeLit)
				if !ok {
					continue
				}
				for _, kv := range cl.Elts {
					kve, ok := kv.(*ast.KeyValueExpr)
					if !ok {
						continue
					}
					lit, ok := kve.Key.(*ast.BasicLit)
					if !ok || lit.Kind != token.STRING {
						continue
					}
					if v, err := strconv.Unquote(lit.Value); err == nil {
						set[v] = true
					}
				}
			}
			return true
		})
	}
	return set
}

// TestBumpMetricScannerSeesKnownShape 是扫描器**自身**的守卫。
//
// ⚠️ 与第 78/82 轮同一条理由：「扫不到就 Fatal」的守卫在真的没东西可扫时
// 会永远红，于是被人删掉 —— 于是它保护的东西彻底裸奔。
// 「扫描器是否可用」与「代码是否合规」必须分开回答。
func TestBumpMetricScannerSeesKnownShape(t *testing.T) {
	if len(bumpedMetrics(t)) == 0 {
		t.Fatal("扫描器一个 bump 站点都没扫到 —— 它坏了，" +
			"于是 TestEveryTaskMetricHasABumpSite 的「全绿」毫无意义")
	}
	// 已知形状：结算路径那 4 个
	got := bumpedMetrics(t)
	for _, m := range []string{"kills", "clears", "reactions", "max_stage", "signin"} {
		if !got[m] {
			t.Errorf("结算/签到路径的 %q 没被扫到 —— 扫描器漏了调用点", m)
		}
	}
	// 注释里的字面量不得被扫进来
	if got["task_date"] {
		t.Error("扫到了 `task_date` —— 那只出现在注释里（第 76 轮那条注释），" +
			"说明扫描没有用 AST 而退化成了文本匹配")
	}
}

/**
 * 「每日签到」任务必须**真的**能推进并领取（第 91 轮的行为判据）。
 *
 * 守卫 `TestEveryTaskMetricHasABumpSite` 是**结构**的
 * （「这个 metric 有 bump 站点」）。结构判据挡不住
 * 「站点存在但 delta 算错」「站点在错误的时刻被调用」
 * 「站点在事务被回滚之后才写」这类形态。
 *
 * 那条签到任务在修复前就属于后者：`SignIn` 里根本没有那一行。
 *
 * 所以这条直接走完整链路：签到 → 查任务进度 → 领取 → 钱包到账。
 *
 * ⚠️ 断言必须量**增量**而不是绝对值：
 * `newUser` 会送一笔初始金币（第 76 轮记过这个坑 ——
 * 把夹具的巧合性质当成了前提）。
 */
func TestSignInDailyTaskProgressesAndClaims(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	uid := scratch.newUser(t, ctx)

	// 前置：该任务存在且启用，否则本用例什么也没测到
	var target int
	var scope string
	if err := scratch.pool.QueryRow(ctx,
		`SELECT target, scope FROM tasks WHERE code = 'daily_signin'`).Scan(&target, &scope); err != nil {
		t.Skipf("种子里没有 daily_signin 任务（可能是被有意删掉了）：%v", err)
	}
	if target != 1 || scope != "daily" {
		t.Fatalf("daily_signin 的 target/scope = %d/%s，期望 1/daily", target, scope)
	}

	before := scratch.wallet(t, ctx, uid).Coin

	// 签到 —— 修复前这一步不会写任何 user_tasks 行
	if _, err := scratch.SignIn(ctx, uid); err != nil {
		t.Fatalf("签到失败：%v", err)
	}

	var progress int
	if err := scratch.pool.QueryRow(ctx,
		`SELECT progress FROM user_tasks
		  WHERE user_id = $1 AND task_id =
		        (SELECT id FROM tasks WHERE code = 'daily_signin')
		    AND task_date = $2`,
		uid, periodStart(time.Now(), "daily")).Scan(&progress); err != nil {
		if err == pgx.ErrNoRows {
			t.Fatalf("签到后**没有** user_tasks 行 —— 「每日签到」任务的进度从未被写过\n" +
				"它会永远停在 0/1，玩家每天都看到它、永远领不到奖励")
		}
		t.Fatal(err)
	}
	if progress < target {
		t.Errorf("签到后 progress = %d，期望 >= %d", progress, target)
	}

	// 领取 —— 端到端
	// ClaimTask 收的是**数字 id**，不是 code —— 我第一版传了 code，
	// 编译器当场抓住（这比运行时才发现好）。
	var taskID int
	if err := scratch.pool.QueryRow(ctx,
		`SELECT id FROM tasks WHERE code = 'daily_signin'`).Scan(&taskID); err != nil {
		t.Fatal(err)
	}
	claimed, err := scratch.ClaimTask(ctx, uid, taskID)
	if err != nil {
		t.Fatalf("领取「每日签到」任务失败：%v\n"+
			"这与第 76 轮周任务同族 —— 一个玩家永远领不到的任务", err)
	}
	if len(claimed) == 0 {
		t.Errorf("领取成功但奖励为空")
	}

	after := scratch.wallet(t, ctx, uid).Coin
	if after <= before {
		t.Errorf("领取后金币 %d -> %d，必须增加（领奖是这条断言真正要守的东西）", before, after)
	}
}

// 让 context / time / pgx 保持被使用（上面用到了）
var (
	_ = context.Background
	_ = time.Now
	_ = pgx.ErrNoRows
)

/** lastOne 返回最后一个元素（用于上面那条自证的错误信息）。 */
func lastOne(xs []string) string {
	if len(xs) == 0 {
		return "<空>"
	}
	return xs[len(xs)-1]
}
