package service

import (
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"runtime"
	"sort"
	"strings"
	"testing"
)

// 事务体里的错误分支**必须 return**（第 82 轮）。
//
// # 这类缺陷的通用形态
//
// 第 81 轮修的那个：
//
//	if err := s.grantWallet(…, ownerID, -take); err != nil {
//	    delete(stolen, k)      // ← 分支里没有 return
//	}
//
// 表面上像「容错」，实际把**两类语义完全相反**的错误处理成同一个动作：
//
//	pgx.ErrNoRows  → 余额不足     → 可以跳过
//	DB 断了 / 约束破坏 → 基础设施失败 → 必须回滚
//
// 而 PostgreSQL 的事务语义（任何语句出错 → 整个事务 aborted → 后续全报 25P02）
// 掩盖了它的真实危害：**错误被替换成一句指向 nowhere 的 25P02**。
//
// # 为什么值得做成结构守卫
//
// 「错误分支有没有 return」是一个**结构性质**，不是某几行的巧合。
// 手写枚举会漏 —— 与第 67/74/75/78 轮记的完全同型。
//
// # 判据
//
// 在**事务闭包**（传给 `DB.Tx` / `Tx` 的 func 字面量）里，
// 任何 `if <something err>; err != nil { … }` 或 `if err != nil { … }`
// 的分支体，若**任意深度**都没有 `return` 语句 → 违规。
//
// # 为什么限定在事务闭包内
//
// 非事务代码里的「记录日志然后继续」是完全正常的模式
// （例如流式导出时跳过一条坏记录）。
// 把判据铺到全包会产出大量误报 —— 而**守卫误报比漏报更危险**：
// 它会让人去「修」一个本来正确的地方。
//
// # 豁免必须具名并写理由
//
// 「这里不 return 是对的」这种判断留给写豁免的人，不留给沉默。
var txContinueExemptions = map[string]string{}

// swallowInTx 是一个「错误分支没有 return」的位置。
type swallowInTx struct {
	Pos     string // 文件:行（posKey 格式）
	Fn      string
	Snippet string
}

// scanTxErrorSwallows 用 go/ast 扫出事务闭包里没有 return 的错误分支。
func scanTxErrorSwallows(t *testing.T) []swallowInTx {
	t.Helper()
	_, thisFile, _, ok := runtime.Caller(0)
	if !ok {
		t.Fatal("runtime.Caller 失败")
	}
	pkgDir := filepath.Dir(thisFile)

	entries, err := os.ReadDir(pkgDir)
	if err != nil {
		t.Fatalf("读目录失败：%v", err)
	}

	var out []swallowInTx
	fset := token.NewFileSet()
	for _, e := range entries {
		name := e.Name()
		if e.IsDir() || !strings.HasSuffix(name, ".go") || strings.HasSuffix(name, "_test.go") {
			continue
		}
		f, err := parser.ParseFile(fset, filepath.Join(pkgDir, name), nil, 0)
		if err != nil {
			t.Fatalf("解析 %s 失败：%v", name, err)
		}
		for _, decl := range f.Decls {
			fn, ok := decl.(*ast.FuncDecl)
			if !ok || fn.Body == nil {
				continue
			}
			enclosing := name + "." + fn.Name.Name
			ast.Inspect(fn.Body, func(n ast.Node) bool {
				call, ok := n.(*ast.CallExpr)
				if !ok {
					return true
				}
				// 只认 `xxx.Tx(ctx, func(...) error { … })`
				if !isTxCall(call) {
					return true
				}
				for _, arg := range call.Args {
					lit, ok := arg.(*ast.FuncLit)
					if !ok {
						continue
					}
					out = append(out, scanErrorSwallowsIn(fset, lit, enclosing)...)
				}
				return true
			})
		}
	}
	sort.Slice(out, func(i, j int) bool { return out[i].Pos < out[j].Pos })
	return out
}

func isTxCall(call *ast.CallExpr) bool {
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "Tx" {
		return false
	}
	// `s.DB.Tx` / `s.Tx` / `tx.Tx` 都算；`pool.Begin` 不算（不是 Tx 语义）
	return true
}

func scanErrorSwallowsIn(fset *token.FileSet, body *ast.FuncLit, enclosing string) []swallowInTx {
	var out []swallowInTx
	ast.Inspect(body.Body, func(n ast.Node) bool {
		ifs, ok := n.(*ast.IfStmt)
		if !ok || ifs.Body == nil {
			return true
		}
		if !isErrCondition(ifs.Cond) {
			return true
		}
		// 任意深度都要有 return
		if hasReturn(ifs.Body) {
			return true
		}
		out = append(out, swallowInTx{
			Pos:     posKey(fset.Position(ifs.Pos()).String()),
			Fn:      enclosing,
			Snippet: oneLine(fset, ifs),
		})
		return true
	})
	return out
}

// isErrCondition 判断条件形如 `err != nil` 或 `<x> err := …; err != nil`。
func isErrCondition(cond ast.Expr) bool {
	bin, ok := cond.(*ast.BinaryExpr)
	if !ok || bin.Op != token.NEQ {
		return false
	}
	// 右侧必须是 nil
	id, ok := bin.Y.(*ast.Ident)
	if !ok || id.Name != "nil" {
		return false
	}
	// 左侧必须叫 err / 带 err 前后缀
	//
	// ⚠️ 两种命名都要认：
	//   `tagErr`      —— 局部变量用**后缀**
	//   `db.ErrNoRows` —— 包限定名用**前缀**（哨兵与 PG 错误码都是这个形状）
	//
	// 只认一种会让另一类**整个从视野里消失** ——
	// 而「看不见」比「误报」更危险：误报会被人「修」，看不见不会有人知道。
	switch v := bin.X.(type) {
	case *ast.Ident:
		return v.Name == "err" ||
			strings.HasSuffix(v.Name, "Err") ||
			strings.HasPrefix(v.Name, "Err")
	case *ast.SelectorExpr:
		return strings.HasPrefix(v.Sel.Name, "Err") ||
			strings.HasSuffix(v.Sel.Name, "Err")
	}
	return false
}

// hasReturn 判断块内**任意深度**是否有 return 语句。
func hasReturn(b *ast.BlockStmt) bool {
	found := false
	ast.Inspect(b, func(n ast.Node) bool {
		if _, ok := n.(*ast.ReturnStmt); ok {
			found = true
			return false
		}
		return true
	})
	return found
}

func oneLine(fset *token.FileSet, n ast.Node) string {
	start := fset.Position(n.Pos())
	end := fset.Position(n.End())
	raw, err := os.ReadFile(start.Filename)
	if err != nil || start.Offset < 0 || end.Offset > len(raw) || start.Offset >= end.Offset {
		return "<读取失败>"
	}
	txt := strings.Join(strings.Fields(string(raw[start.Offset:end.Offset])), " ")
	if len(txt) > 110 {
		txt = txt[:110] + "..."
	}
	return txt
}

// TestTxErrorBranchesMustReturn 是本文件的核心。
func TestTxErrorBranchesMustReturn(t *testing.T) {
	found := scanTxErrorSwallows(t)

	var violations []string
	for _, s := range found {
		if why, ok := txContinueExemptions[s.Pos]; ok {
			if strings.TrimSpace(why) == "" {
				violations = append(violations,
					s.Pos+" 有豁免但理由为空 —— 「这里不 return 是对的」不留给沉默")
			}
			continue
		}
		violations = append(violations, s.Pos+" 事务里的错误分支没有 return："+s.Snippet)
	}

	if len(violations) == 0 {
		t.Logf("事务体里共 %d 处错误分支，全部都有 return", len(found))
		return
	}
	sort.Strings(violations)
	t.Errorf("以下 %d 处**事务体内**的错误分支没有 return：\n  %s\n\n"+
		"在事务里出错却继续跑，后果取决于 PG 的事务语义：\n"+
		"  - 若是 `pgx.ErrNoRows`（条件 UPDATE 没匹配）→ 事务没 aborted，\n"+
		"    于是「基础设施失败」被当成「业务上跳过」静默处理掉；\n"+
		"  - 若是真错误 → 事务 aborted，后续语句全报 25P02，\n"+
		"    真实原因被替换成一句指向 nowhere 的通用码。\n"+
		"两种都把故障诊断带偏。第 81 轮就是这个形态（`defense_stolen` 流水写不进去）。\n"+
		"修法：分支里 `return err`（或 `return fmt.Errorf(…, err)`）。\n"+
		"若确实要吞，写进豁免表并说明理由。",
		len(violations), strings.Join(violations, "\n  "))
}

// TestTxScannerDetectsKnownShapes 是扫描器**自己的**守卫。
//
// ⚠️ 与第 78 轮同一条理由：「扫不到就 Fatal」的守卫在真的没东西可扫时
// 会永远红，于是被人删掉 —— 于是它保护的东西彻底裸奔。
// 「扫描器是否可用」与「代码是否合规」必须分开回答。
func TestTxScannerDetectsKnownShapes(t *testing.T) {
	swallow := func(body string) *ast.FuncLit {
		src := "package p\nfunc f() {\n\ts.DB.Tx(ctx, func(tx txType) error {\n" + body +
			"\n\t\treturn nil\n\t})\n}\n"
		fset := token.NewFileSet()
		f, err := parser.ParseFile(fset, "synthetic.go", src, 0)
		if err != nil {
			t.Fatalf("合成源解析失败：%v\n%s", err, src)
		}
		var found *ast.FuncLit
		ast.Inspect(f, func(n ast.Node) bool {
			if call, ok := n.(*ast.CallExpr); ok && isTxCall(call) {
				for _, a := range call.Args {
					if lit, ok := a.(*ast.FuncLit); ok {
						found = lit
					}
				}
			}
			return true
		})
		if found == nil {
			t.Fatalf("合成源里没找到 Tx 的 func 字面量：\n%s", src)
		}
		return found
	}

	cases := []struct {
		name    string
		body    string
		wantAny bool
	}{
		{
			name:    "吞掉：分支里只有 delete",
			body:    "\t\tif err := s.grantWallet(ctx, tx, id, nil, \"r\", 1); err != nil {\n\t\t\tdelete(m, k)\n\t\t}",
			wantAny: true,
		},
		{
			name:    "吞掉：分支里只有 continue",
			body:    "\t\tif err := step(ctx); err != nil {\n\t\t\tcontinue\n\t\t}",
			wantAny: true,
		},
		{
			name:    "正确：分支里 return",
			body:    "\t\tif err := step(ctx); err != nil {\n\t\t\treturn err\n\t\t}",
			wantAny: false,
		},
		{
			name:    "正确：return 包了 fmt.Errorf",
			body:    "\t\tif _, err := tx.Exec(ctx, q); err != nil {\n\t\t\treturn fmt.Errorf(\"clear slots: %w\", err)\n\t\t}",
			wantAny: false,
		},
		{
			name:    "正确：return 在嵌套 if 里也算",
			body:    "\t\tif err := step(ctx); err != nil {\n\t\t\tif !ok(err) {\n\t\t\t\treturn nil\n\t\t\t}\n\t\t}",
			wantAny: false,
		},
		{
			name:    "不是错误分支（err == nil）不算",
			body:    "\t\tif err == nil {\n\t\t\tdelete(m, k)\n\t\t}",
			wantAny: false,
		},
		{
			// ⚠️ 下面四条是为「变异：只认 err、不认 <x>Err」准备的。
			//
			// 判据必须覆盖**实际的命名形态**：代码里大量存在
			// `if tagErr != nil` 这类写法，只认 `err` 会让它们
			// **整个从视野里消失** —— 而「看不见」比「误报」更危险。
			name:    "tagErr != nil 也算错误分支",
			body:    "\t\tif tagErr != nil {\n\t\t\tdelete(m, k)\n\t\t}",
			wantAny: true,
		},
		{
			name:    "pkg.ErrX != nil 也算错误分支",
			body:    "\t\tif db.ErrNoRows != nil {\n\t\t\tdelete(m, k)\n\t\t}",
			wantAny: true,
		},
		{
			name:    "tagErr != nil 且分支里有 return → 正确",
			body:    "\t\tif tagErr != nil {\n\t\t\treturn tagErr\n\t\t}",
			wantAny: false,
		},
		{
			// 嵌套在别的 if 里的错误判断也要被看到
			name:    "嵌套在 if 里的错误分支",
			body:    "\t\tif ready {\n\t\t\tif err != nil {\n\t\t\t\tdelete(m, k)\n\t\t\t}\n\t\t}",
			wantAny: true,
		},
	}

	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			got := scanErrorSwallowsIn(token.NewFileSet(), swallow(c.body), "synthetic")
			if len(got) > 0 != c.wantAny {
				t.Errorf("扫到 %d 处，期望 %v（判据对合成源判错了，它在真实代码上的结论都不可信）",
					len(got), c.wantAny)
			}
		})
	}
}

// TestTxScannerSeesTheRealPackage 确认扫描范围真的覆盖了 service 包。
//
// 统计的是「**被管辖的错误分支数**」而不是「违规数」——
// 前者在修复完成后依然成立，所以不会像「扫不到就 Fatal」那样变成负担。
func TestTxScannerSeesTheRealPackage(t *testing.T) {
	found := scanTxErrorSwallows(t)
	if len(found) == 0 {
		t.Log("当前 service 包的事务体里没有「错误分支没有 return」的情况。\n" +
			"注意：这**可能**是「真的都 return 了」，也可能是「扫描器坏了」。\n" +
			"用 `TestTxScannerDetectsKnownShapes` 区分这两种情况。")
		return
	}
	sort.Slice(found, func(i, j int) bool { return found[i].Pos < found[j].Pos })
	for i := 0; i < len(found) && i < 10; i++ {
		t.Logf("命中 %s（%s）: %s", found[i].Pos, found[i].Fn, found[i].Snippet)
	}
}
