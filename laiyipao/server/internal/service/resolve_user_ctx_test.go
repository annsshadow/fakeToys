package service

import (
	"context"
	"errors"
	"os"
	"sort"
	"strings"
	"testing"
	"time"
)

// `ResolveUser` 必须把调用方的 **ctx** 传给数据库查询（第 101 轮）。
//
// # 缺陷：ctx 参数被完全丢弃
//
// ```go
//
//	func (s *Service) ResolveUser(ctx context.Context, token string) (int64, error) {
//	    ...
//	    if err := s.pool.QueryRow(context.Background(),   // ← 丢弃了 ctx
//	        `SELECT status FROM users WHERE id = $1`, claims.Sub).Scan(&status); ...
//
// ```
//
// # 后果
//
//  1. **客户端断开不会取消这条查询。** fiber 在客户端断开后取消请求 ctx，
//     而这里换成了 `Background`，pgx 会**把查询跑完**。
//     慢查询 + 反复断开 = 连接池被占满，而每个占用者都不会超时退出。
//  2. **任何超时都不生效。** `ctxBackground()` 那个辅助函数的存在
//     说明这里本来是想传 ctx 的。
//  3. `ctx` 参数因此**完全未被使用** —— 而 **Go 不报「未使用的参数」**
//     （只报未使用的局部变量），所以它一直静默地错着。
//
// # 为什么单测抓不到
//
// `account_service_test.go` 等测试**直接传 `context.Background()`**，
// 于是「传进去的 ctx 被丢掉」与「正常传递」**行为完全一致**。
//
// 这与第 76/89/93/97 轮是同一族：**观测手段本身有前提**。
// 这次的前提是「测试传的 ctx 恰好没有 deadline，也没有取消」。
func TestResolveUserPropagatesCallerContext(t *testing.T) {
	ts := openTestService(t)
	uid := ts.newUser(t, ctxBG())

	// 1) 正常路径：必须成功
	if _, err := ts.ResolveUser(context.Background(), ts.tokenFor(t, uid)); err != nil {
		t.Fatalf("正常 ctx 下 ResolveUser 失败：%v", err)
	}

	// 2) **已过期**的 ctx：必须失败，且错误是 deadline
	expired, cancel := context.WithDeadline(context.Background(), time.Now().Add(-time.Second))
	defer cancel()
	_, err := ts.ResolveUser(expired, ts.tokenFor(t, uid))
	if err == nil {
		t.Fatal("ctx 已过期时 ResolveUser 竟然成功了 —— ctx 被丢弃了\n\n" +
			"这正是缺陷形态：`context.Background()` 换掉了调用方的 ctx，" +
			"于是任何取消/超时都不生效。\n" +
			"而既有测试全都直接传 `context.Background()`，" +
			"于是「丢掉」与「正常」观察不到差别。")
	}
	if !errors.Is(err, context.DeadlineExceeded) {
		t.Errorf("ctx 过期时应返回 DeadlineExceeded，实际 %v", err)
	}
}

// TestResolveUserHonoursCancellation 确认**主动取消**也被尊重。
//
// 与上一条分开：DeadlineExceeded 与 Canceled 是两个不同的错误，
// 只断言「有错误」分不出「因为 ctx 而失败」与「因为别的而失败」。
func TestResolveUserHonoursCancellation(t *testing.T) {
	ts := openTestService(t)
	uid := ts.newUser(t, ctxBG())
	tok := ts.tokenFor(t, uid)

	cancelled, cancel := context.WithCancel(context.Background())
	cancel() // 立即取消

	_, err := ts.ResolveUser(cancelled, tok)
	if err == nil {
		t.Fatal("ctx 已取消时 ResolveUser 竟然成功了 —— ctx 被丢弃了")
	}
	if !errors.Is(err, context.Canceled) {
		t.Errorf("ctx 取消时应返回 Canceled，实际 %v", err)
	}
}

// TestServiceHasNoStrayContextBackground 是**源码**判据：
// service 包里不得出现「在有 ctx 可传的地方硬造 Background」。
//
// ⚠️ `progression.go` 的 `ctxBackground()` 是**有意的** HTTP 层包装
// （`ComputeRatingFor` / `ComputePowerFor` 给运营接口用，没有请求 ctx）。
// 所以豁免它，并要求豁免写明理由。
func TestServiceHasNoStrayContextBackground(t *testing.T) {
	allowed := map[string]string{
		// HTTP 层包装：给运营接口用，那些入口确实没有请求 ctx
		// （它们是「一个不需要取消语义」的纯计算调用）。
		// ctxBackground 这个名字本身就在说「我知道自己没有 ctx」。
		// 第 102 轮后：ComputePowerFor **不再需要** ctxBackground ——
		// computePower 变成纯函数后，它连 ctx 都不用造了。
		// 只剩 ctxBackground 本体，供 ComputeRatingFor（确实查库）使用。
		"progression.go": "只剩 ctxBackground() 本体，供 ComputeRatingFor 使用" +
			"（它确实查 user_progress，需要一个非取消的 ctx）。" +
			"ComputePowerFor 在第 102 轮已不再需要它。",
	}

	files := listServiceSources(t)
	if len(files) == 0 {
		t.Fatal("没列出任何源文件 —— 「零违规」毫无意义")
	}
	//
	// ⚠️ `len(files) == 0` 不够：变异「把范围缩到只剩 service.go」→
	// **全绿通过**（那个文件本来就干净）。
	//
	// 「扫得少」与「全部合规」在输出里**一模一样** —— 与第 91/99 轮同源。
	// 所以要有**下限**：service 包的源文件远多于这个数。
	const minExpectedFiles = 5
	if len(files) < minExpectedFiles {
		t.Fatalf("只扫了 %d 个文件，期望至少 %d 个\n"+
			"「扫得少」与「全部合规」在输出里一模一样 —— "+
			"变异「把范围缩到一个已知文件」是全绿的。", len(files), minExpectedFiles)
	}

	var offenders []string
	for _, name := range files {
		if _, ok := allowed[name]; ok {
			continue
		}
		code := stripGoComments(readFileInPackage(t, name))
		for i, ln := range strings.Split(code, "\n") {
			if !strings.Contains(ln, "context.Background()") &&
				!strings.Contains(ln, "context.TODO()") {
				continue
			}
			offenders = append(offenders,
				name+":"+itoa(int64(i+1))+": "+strings.TrimSpace(ln))
		}
	}
	for f, why := range allowed {
		if strings.TrimSpace(why) == "" {
			offenders = append(offenders,
				f+" 在豁免表里但理由为空 —— 「这里可以造 Background」不留给沉默")
		}
		_ = f
	}
	if len(offenders) > 0 {
		t.Errorf("以下位置在有 ctx 可传的地方硬造了 context.Background/TODO"+
			"（已扫 %d 个文件，豁免 %d 个）：\n  %s\n\n"+
			"丢弃调用方 ctx 的后果：客户端断开不会取消查询、任何超时都不生效。\n"+
			"而 Go **不报「未使用的参数」**，所以它会一直静默地错着。\n"+
			"若确实没有 ctx 可传，请写进豁免并说明理由。",
			len(files), len(allowed), strings.Join(offenders, "\n  "))
	}
}

// ctxBG 是测试里常用的 context（**不带 deadline**）——
// 刻意如此：带 deadline 的话，「ctx 被丢弃」就观察不出来了。
func ctxBG() context.Context { return context.Background() }

// tokenFor 为用户签一个 access token。
func (ts *testService) tokenFor(t *testing.T, uid int64) string {
	t.Helper()
	tok, _, err := ts.JWT.Issue(uid, "user")
	if err != nil {
		t.Fatalf("签发令牌失败：%v", err)
	}
	return tok
}

// listServiceSources 列出本包里参与编译的源文件（**排除 _test.go**）。
//
// ⚠️ 返回**全部**而不是挑几个：挑文件的那种清单会随新文件而失效，
// 而「新文件里的 context.Background」正是这条守卫要抓的。
func listServiceSources(t *testing.T) []string {
	t.Helper()
	entries, err := os.ReadDir(".")
	if err != nil {
		t.Fatalf("读包目录失败：%v", err)
	}
	var out []string
	for _, e := range entries {
		n := e.Name()
		if e.IsDir() || !strings.HasSuffix(n, ".go") || strings.HasSuffix(n, "_test.go") {
			continue
		}
		out = append(out, n)
	}
	sort.Strings(out)
	return out
}
