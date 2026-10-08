package service

import (
	"go/ast"
	"go/parser"
	"go/token"
	"path/filepath"
	"sort"
	"strings"
	"testing"
)

// 「函数签名收了 `ctx`，函数体却把它丢掉」**必须被看见**（第 102 轮）。
//
// # 为什么需要它
//
// 第 101 轮修掉的 `ResolveUser` 是**全仓库唯一一处** `context.Background()`。
// 那说明「扫字符串 `context.Background()`」这个判据**够用**——
// 但它只能抓「已经写成 `context.Background()`」的形态。
//
// 抓不到的是这几种**等价**写法：
//
//	func (s *Service) f(ctx context.Context) error {
//	    ctx2 := context.Background()      // ← 别名，字符串扫得到但语义一样
//	    ctx3 := context.TODO()
//	    inner := func() { … }              // 闭包里用了另一个 ctx
//	}
//
// 更根本的问题是：**没有任何机制让「签名里的 ctx 被用上了」这件事可判定**。
// Go 不报未使用的参数（第 101 轮实测：它就是这么静默地错着的）。
//
// # 判据：签名含 ctx 的函数，其函数体里必须出现 `ctx`（按名字）
//
// ⚠️ **这个判据是名称级的，不是语义级的**：
// `ctx` 可能只出现在 `_ = ctx` 里，实质上仍然被丢弃。
// 所以它只能当**粗筛** —— 但粗筛的价值在于：
//
//   - 它抓得到「参数完全没用」
//   - 它抓不到「参数只是被 `_ =` 消音」
//
// 后者由第 101 轮的行为判据（过期 ctx / 取消 ctx）覆盖。
// **两条判据互补**，不能互相替代。
//
// # 为什么要扫 AST 而不是文本
//
// 文本扫描分不清「签名里」与「注释里」，也分不清
// 「`func f(ctx context.Context)`」与「调用处的 `ctx`」。
// 本项目已因「文本扫描撞注释」踩坑 5 次（第 67/83/88/94/95 轮）。
func TestNoFunctionIgnoresItsCtxParameter(t *testing.T) {
	suspects := functionsWithUnusedCtx(t)
	if len(suspects) == 0 {
		t.Logf("所有带 ctx 参数的函数都在函数体里用到了它")
		return
	}
	sort.Strings(suspects)
	t.Errorf("以下函数的签名含 `ctx`，但函数体里**完全没出现** `ctx`：\n  %s\n\n"+
		"要么是参数名被改了（那 `ctx` 就是纯装饰），\n"+
		"要么是真的丢了调用方的 ctx —— 第 101 轮 `ResolveUser` 就是后者，\n"+
		"后果是客户端断开不会取消查询、超时全部失效，且 Go **不报未使用的参数**。\n\n"+
		"⚠️ 本判据是**名称级**的：它抓得到「完全没用」，抓不到「只是 `_ = ctx`」。\n"+
		"后者由 `TestResolveUserPropagatesCallerContext` 那类"+
		"「已过期 / 已取消的 ctx 必须失败」的行为判据覆盖。",
		strings.Join(suspects, "\n  "))
}

// TestCtxAuditScannerSeesKnownShape 是扫描器**自身**的守卫。
//
// ⚠️ 与第 78/82/99/101 轮同一条理由：「扫不到就 Fatal」的守卫在真的没东西可扫时
// 会永远红，于是被人删掉 —— 于是它保护的东西彻底裸奔。
// 「扫描器是否可用」与「代码是否合规」必须分开回答。
func TestCtxAuditScannerSeesKnownShape(t *testing.T) {
	// 合成源：一个用了 ctx 的函数，一个没用的
	src := `package p
import "context"
func used(ctx context.Context) error { return ctx.Err() }
func unused(ctx context.Context) error { return nil }
func noCtx() error { return nil }
`
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, "synthetic.go", src, 0)
	if err != nil {
		t.Fatalf("合成源解析失败：%v", err)
	}

	got := ctxUnusedFuncs(f, fset)
	if len(got) != 1 {
		t.Errorf("合成源里应当只命中 1 个（unused），实得 %d 个：%v", len(got), got)
	}
	if len(got) == 1 && got[0] != "unused" {
		t.Errorf("命中的是 %q，应为 \"unused\"", got[0])
	}

	// 没有 ctx 参数的函数不得被算进去
	for _, n := range got {
		if n == "noCtx" {
			t.Error("noCtx 没有 ctx 参数，不该被算进去 —— 判据把签名条件搞错了")
		}
	}

	// ⚠️ 「`_ = ctx` 消音」必须**也算**没用 ——
	// 变异实测：LoadGameConfig 的 ctx 参数加回来并写 `_ = ctx` → 第一版全绿。
	//
	// `_ = ctx` 的唯一作用就是让编译器闭嘴，对运行期毫无影响；
	// 它出现就说明「这个 ctx 被丢弃了，而作者希望没人发现」。
	src2 := `package p
import "context"
func silenced(ctx context.Context) error { _ = ctx; return nil }
`
	fset2 := token.NewFileSet()
	f2, err := parser.ParseFile(fset2, "synthetic2.go", src2, 0)
	if err != nil {
		t.Fatalf("合成源 2 解析失败：%v", err)
	}
	got2 := ctxUnusedFuncs(f2, fset2)
	if len(got2) != 1 || got2[0] != "silenced" {
		t.Errorf("「_ = ctx」消音应被算作「没用」，实得 %v", got2)
	}
}

/**
 * 「`_ = f(ctx, …)`」**必须**算「用到了」。
 *
 * ⚠️ 这条是**误报守卫**。
 *
 * 我第一版的消音规则写成「`_ = …` 右侧的整个表达式区间」，
 * 于是 `Audit` 里这行被判成「ctx 没用」：
 *
 *	_ = s.pool.QueryRow(ctx, `SELECT …`).Scan(&username)
 *
 * 那是**完全合法且必要**的 Go 惯用法（显式丢弃 error），
 * 而它**真的用到了** ctx —— 查询还带着取消语义。
 *
 * 若照那条守卫去「修」，后果是把审计查询的 ctx 删掉，
 * 于是审计写入不再可取消 —— **守卫制造了一个它声称在防的缺陷。**
 *
 * 这就是 README 里那句「误报比漏报更危险」的实例。
 */
func TestBlankAssignedCallStillCountsAsUsingCtx(t *testing.T) {
	src := `package p
import "context"
func uses(ctx context.Context) error {
	_ = ctx.Err()          // 合法的「显式丢弃」惯用法
	_ = ctx                 // 这才是消音
	return nil
}
`
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, "synthetic3.go", src, 0)
	if err != nil {
		t.Fatalf("合成源解析失败：%v", err)
	}
	got := ctxUnusedFuncs(f, fset)
	if len(got) != 0 {
		t.Errorf("函数里既有 `_ = ctx.Err()` 也有 `_ = ctx`，应判为「用到了」"+
			"（因为 `_ = ctx.Err()` 真的在用），实得 %v", got)
	}

	// 只有 `_ = ctx` 时才判为没用
	src2 := `package p
import "context"
func onlySilenced(ctx context.Context) error {
	_ = ctx
	return nil
}
`
	fset2 := token.NewFileSet()
	f2, err := parser.ParseFile(fset2, "synthetic4.go", src2, 0)
	if err != nil {
		t.Fatalf("合成源 2 解析失败：%v", err)
	}
	if got2 := ctxUnusedFuncs(f2, fset2); len(got2) != 1 {
		t.Errorf("只有 `_ = ctx` 时应判为「没用」，实得 %v", got2)
	}
}

// functionsWithUnusedCtx 用 AST 扫出「签名含 ctx 参数但函数体没用它」的函数。
func functionsWithUnusedCtx(t *testing.T) []string {
	t.Helper()
	fset := token.NewFileSet()
	var out []string
	files := listServiceSources(t)

	// ⚠️ 「扫得少」与「全部合规」在输出里**一模一样** ——
	// 变异「在循环里跳过非 service.go 的文件」→ **全绿通过**（那个文件本来干净）。
	// 这是同一个陷阱的第 5 次（第 91/99/101 轮与上一条同型）。
	//
	// ⚠️ 下限必须数**真正解析过**的文件，而不是 `len(files)` ——
	// 变异改的是「循环里跳过」，列表长度不变。
	// 我第一版就写成 `len(files)`，于是那条变异仍然全绿。
	parsed := 0
	inspected := 0
	const minExpectedFiles = 5

	for _, name := range files {
		f, err := parser.ParseFile(fset, filepath.Join(".", name), nil, 0)
		if err != nil {
			t.Fatalf("解析 %s 失败：%v", name, err)
		}
		parsed++
		// ⚠️ `inspected++` 必须在**真正调用检查之前**，
		// 不能在解析之后就加 ——
		// 变异「解析后 continue」会骗过一个「先加后跳」的计数。
		// 我第一版就写在了 parsed++ 旁边，于是那条变异照样全绿。
		inspected++
		for _, n := range ctxUnusedFuncs(f, fset) {
			out = append(out, name+": "+n)
		}
	}

	if inspected < minExpectedFiles {
		t.Fatalf("只**检查**了 %d 个文件（列表 %d 个、解析 %d 个），期望至少 %d 个\n"+
			"范围被人为收窄了，而「扫得少」与「全部合规」在输出里一模一样。\n"+
			"⚠️ 下限必须数 `ctxUnusedFuncs` 的调用次数，"+
			"数「解析了几个文件」会被「解析后 continue」骗过。",
			inspected, len(files), parsed, minExpectedFiles)
	}
	// ⚠️ parsed 也不够：变异可以在**解析之后、调用检查之前** `continue` ——
	// 于是 parsed 仍然是全量，而实际只检查了一个文件。
	// 我第一版就只查 parsed，那条变异照样全绿。
	return out
}

// ctxUnusedFuncs 返回「签名含 ctx 参数但函数体完全没出现 ctx」的函数名。
func ctxUnusedFuncs(f *ast.File, fset *token.FileSet) []string {
	var out []string
	for _, decl := range f.Decls {
		fn, ok := decl.(*ast.FuncDecl)
		if !ok || fn.Body == nil {
			continue
		}
		hasCtx := false
		for _, p := range fn.Type.Params.List {
			for _, nm := range p.Names {
				if nm.Name == "ctx" {
					hasCtx = true
				}
			}
		}
		if !hasCtx {
			continue
		}
		// 「用到」= 出现过 ctx 标识符，且**不只**出现在 `_ = ctx` 的右侧。
		//
		// ⚠️ 变异实测：把 `LoadGameConfig` 的 ctx 参数加回来并写 `_ = ctx` →
		// 第一版守卫**全绿通过**。`_ = ctx` 让 AST 看见了 `ctx` 这个标识符，
		// 于是「用到了」的判断成立 —— 而它其实什么都没做。
		//
		// `_ = ctx` 的**唯一**作用就是让编译器和 linter 闭嘴，
		// 对运行期毫无影响。它出现就说明
		//「这个 ctx 被丢弃了，而作者希望没人发现」。
		//
		// 判据：**只有** `_ = ctx` 这个裸标识符赋值算消音。
		//
		// ⚠️ 我第一版写成「`_ = …` 右侧的整个表达式区间」——
		// 于是 `Audit` 里这行被误判：
		//
		//	_ = s.pool.QueryRow(ctx, `SELECT …`).Scan(&username)
		//
		// 那是**完全合法且必要**的 Go 惯用法（显式丢弃 error），
		// 而它**真的用到了** ctx —— 查询还带着取消语义。
		//
		// **误报比漏报更危险**：它会让人去「修」一个本来正确的地方，
		// 而这里的后果是把审计查询的 ctx 删掉。
		//
		// 所以判据必须精确到「Rhs **就是**那个标识符本身」。
		silenced := silencedIdents(fn.Body)
		used := false
		ast.Inspect(fn.Body, func(n ast.Node) bool {
			id, ok := n.(*ast.Ident)
			if !ok || id.Name != "ctx" {
				return true
			}
			if silenced[id] {
				return true // 是 `_ = ctx` 的那个标识符本身
			}
			used = true
			return false
		})
		if !used {
			out = append(out, fn.Name.Name)
		}
	}
	return out
}

/**
 * silencedIdents 收集「`_ = ctx`」里那个**裸** `ctx` 标识符。
 *
 * ⚠️ 只认 Rhs **恰好是** `*ast.Ident` 且名字为 `ctx` 的情形。
 * 任何更复杂的表达式（哪怕里面含 `ctx`）都**不算**消音 ——
 * `_ = s.pool.QueryRow(ctx, …)` 是在真的用它。
 *
 * 判据的精度就是它的价值：太宽会误报合法惯用法，
 * 而误报会让人去改本来正确的地方。
 */
func silencedIdents(body *ast.BlockStmt) map[*ast.Ident]bool {
	out := map[*ast.Ident]bool{}
	ast.Inspect(body, func(n ast.Node) bool {
		as, ok := n.(*ast.AssignStmt)
		if !ok {
			return true
		}
		blank := false
		for _, lhs := range as.Lhs {
			if id, ok := lhs.(*ast.Ident); ok && id.Name == "_" {
				blank = true
			}
		}
		if !blank {
			return true
		}
		for _, rhs := range as.Rhs {
			// ⚠️ 只认 Rhs **就是**一个标识符的情形，且**不限定名字**。
			//
			// 我第一版写的是 `id.Name == "ctx"` —— 于是变异
			// 「`ComputePowerFor` 的 userID 参数加回来并写 `_ = userID`」
			// **全绿通过**：消音规则只认 ctx，认不出 userID。
			//
			// 通用化之后两个守卫（ctx 专用与全参数）共用同一条规则，
			// 不会再出现「一个认得、另一个认不出」的裂缝。
			if id, ok := rhs.(*ast.Ident); ok && id.Name != "" {
				out[id] = true
			}
		}
		return true
	})
	return out
}
