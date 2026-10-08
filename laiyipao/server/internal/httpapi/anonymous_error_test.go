package httpapi

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

// 函数体内的**匿名错误**必须被分类（第 68 轮）。
//
// # 缺陷：完备性守卫有个看不见的盲区
//
// `scanSentinelErrors` 用 go/ast 只扫**顶层 `var` GenDecl** ——
// 那正是哨兵声明的形态（`var ErrX = errors.New(...)`）。
// 于是它宣称「每一个哨兵都已分类」，而这个说法**只对哨兵成立**。
//
// 函数体里的 `errors.New(...)` 不在任何 GenDecl 里，
// 它位于 `ReturnStmt` 的 `CallExpr` —— 扫描器**根本看不见**。
//
// 实测：`domain.ValidateSettle` 里有一句
//
//	return SettleResult{}, errors.New("反应次数不能为负")
//
// 它是语义上的「上报被拒」，但因为不在哨兵清单里，
// `isSettleRejection` 返回 false → `failErr` 兜底成 **500**。
//
// 500 的代价与 README 记的三个哨兵事故完全相同：
// 客户端只显示「服务内部错误」，排查会滑向「服务端坏了」；
// 而且它混进服务故障告警，把真正的故障淹掉。
//
// # 与既有守卫的分工
//
// 它与 `TestEverySentinelErrorIsClassified` 是**互补**的两半：
//   - 那条：每个哨兵都归了类（覆盖「声明」）
//   - 本条：每处错误构造都能被识别（覆盖「使用」）
//
// 缺任何一条，另一半都可能漏。第 68 轮这个缺陷正是「声明侧全绿、
// 使用侧有漏」—— 而上一轮没有任何测试发现它。

// wrapKind 描述一个错误构造「包着什么」。
type wrapKind int

const (
	wrapSentinel wrapKind = iota // 包哨兵（%w + ErrXxx）→ errors.Is 认得 → 422
	wrapPlainErr                 // 包 err（%w + err）→ DB/IO 失败，真 500 是对的
	wrapNone                     // 裸 errors.New → 认不出 → 落 500 ← 要抓的
	wrapAbsent                   // 这里没有错误构造
)

func (k wrapKind) String() string {
	switch k {
	case wrapSentinel:
		return "包哨兵"
	case wrapPlainErr:
		return "包 err"
	case wrapNone:
		return "裸构造"
	default:
		return "无构造"
	}
}

// anonymousError 是一个在函数体内构造的错误。
type anonymousError struct {
	Pos       string
	Snippet   string
	Kind      wrapKind
	InSettle  bool
	Enclosing string
}

// scanAnonymousErrors 扫出 domain 与 service 包里所有函数体内的错误构造。
func scanAnonymousErrors(t *testing.T) []anonymousError {
	t.Helper()
	_, thisFile, _, ok := runtime.Caller(0)
	if !ok {
		t.Fatal("runtime.Caller 失败")
	}
	serverRoot := filepath.Join(filepath.Dir(thisFile), "..", "..")

	var out []anonymousError
	for _, pkg := range []struct{ dir, name string }{
		{filepath.Join(serverRoot, "internal", "domain"), "domain"},
		{filepath.Join(serverRoot, "internal", "service"), "service"},
	} {
		entries, err := os.ReadDir(pkg.dir)
		if err != nil {
			t.Fatalf("读目录 %s 失败：%v", pkg.dir, err)
		}
		for _, e := range entries {
			name := e.Name()
			if e.IsDir() || !strings.HasSuffix(name, ".go") || strings.HasSuffix(name, "_test.go") {
				continue
			}
			path := filepath.Join(pkg.dir, name)
			fset := token.NewFileSet()
			f, err := parser.ParseFile(fset, path, nil, 0)
			if err != nil {
				t.Fatalf("解析 %s 失败：%v", name, err)
			}

			for _, decl := range f.Decls {
				fn, ok := decl.(*ast.FuncDecl)
				if !ok || fn.Body == nil {
					continue
				}
				enclosing := pkg.name + "." + fn.Name.Name
				inSettle := isSettleRelatedFn(pkg.name, fn.Name.Name)

				ast.Inspect(fn.Body, func(n ast.Node) bool {
					ret, ok := n.(*ast.ReturnStmt)
					if !ok {
						return true
					}
					for _, res := range ret.Results {
						kind := classifyErrorExpr(res)
						if kind == wrapAbsent {
							continue
						}
						out = append(out, anonymousError{
							Pos:       fset.Position(res.Pos()).String(),
							Snippet:   callSource(fset, res),
							Kind:      kind,
							InSettle:  inSettle,
							Enclosing: enclosing,
						})
					}
					return true
				})
			}
		}
	}
	sort.Slice(out, func(i, j int) bool { return out[i].Pos < out[j].Pos })
	return out
}

// classifyErrorExpr 判断一个 return 结果里构造的错误属于哪一类。
//
// # ⚠️ 判据用「参数位置」，不用「表达式里有没有某个标识符」
//
// 第一版用 `ast.Inspect` 遍历**整个表达式**，只要看到任何以 `Err` 开头的
// 标识符就算「包了哨兵」。那个判据有两个错：
//
//  1. **穿透到了不相干的参数**。`CardPickSkip` 不以 Err 开头，但
//     `CardPickHandSlots-1` 这种二元表达式会让 Inspect 深入到它的子节点，
//     于是判据可能命中一个**不是哨兵**的表达式。
//  2. **漏掉拼接形态**。`fmt.Errorf("..." + "...")` 的 CallExpr 在
//     BinaryExpr 里，只看 return 的直接结果会漏掉 —— 而那正是
//     card_picks 那处校验的实际写法。
//
// 正确判据：只看 `fmt.Errorf` 的**位置参数**（跳过第 0 个格式串），
// 并穿透 `+` 拼接去看那个 CallExpr。
func classifyErrorExpr(res ast.Expr) wrapKind {
	ce := findErrorCtor(res)
	if ce == nil {
		return wrapAbsent
	}
	sel := ce.Fun.(*ast.SelectorExpr)
	pkg := sel.X.(*ast.Ident)

	if pkg.Name == "errors" && sel.Sel.Name == "New" {
		return wrapNone // 裸 errors.New
	}
	if pkg.Name != "fmt" || sel.Sel.Name != "Errorf" {
		return wrapAbsent
	}

	// 收集格式串（可能因 `+` 拼接而有多段）
	verb := false
	for _, a := range ce.Args {
		if lit, ok := a.(*ast.BasicLit); ok && lit.Kind == token.STRING {
			if strings.Contains(lit.Value, "%w") {
				verb = true
			}
			continue
		}
		if isSentinelArg(a) {
			return wrapSentinel
		}
		if isPlainErrArg(a) {
			return wrapPlainErr
		}
	}
	if !verb {
		return wrapAbsent // fmt.Errorf 但没包任何东西 —— 不是包装
	}
	// 有 %w 但参数里既没哨兵也没 err —— 少见，仍按裸构造处理
	return wrapNone
}

// findErrorCtor 在表达式里找出 `errors.New(...)` 或 `fmt.Errorf(...)`。
//
// 必须穿透 BinaryExpr —— `fmt.Errorf("a" + "b")` 的 CallExpr 藏在 `+` 之下。
func findErrorCtor(res ast.Expr) *ast.CallExpr {
	switch v := res.(type) {
	case *ast.CallExpr:
		if sel, ok := v.Fun.(*ast.SelectorExpr); ok {
			if x, ok := sel.X.(*ast.Ident); ok {
				if (x.Name == "errors" && sel.Sel.Name == "New") ||
					(x.Name == "fmt" && sel.Sel.Name == "Errorf") {
					return v
				}
			}
		}
	case *ast.CompositeLit:
		for _, el := range v.Elts {
			if ce := findErrorCtor(el); ce != nil {
				return ce
			}
		}
	case *ast.BinaryExpr:
		if ce := findErrorCtor(v.X); ce != nil {
			return ce
		}
		if ce := findErrorCtor(v.Y); ce != nil {
			return ce
		}
	}
	return nil
}

// isSentinelArg 判断一个 Errorf 参数是不是哨兵。
//
// 只认**直接**的 `ErrXxx` 标识符或 `pkg.ErrXxx` selector ——
// 不递归：递归会穿透到 `CardPickHandSlots-1` 之类的子表达式里，
// 命中一个与哨兵无关的名字。
func isSentinelArg(a ast.Expr) bool {
	switch v := a.(type) {
	case *ast.Ident:
		return strings.HasPrefix(v.Name, "Err")
	case *ast.SelectorExpr:
		return strings.HasPrefix(v.Sel.Name, "Err")
	}
	return false
}

// isPlainErrArg 判断一个 Errorf 参数是不是普通 err（DB/IO 错误）。
func isPlainErrArg(a ast.Expr) bool {
	switch v := a.(type) {
	case *ast.Ident:
		return v.Name == "err"
	case *ast.SelectorExpr:
		return v.Sel.Name == "Err" || v.Sel.Name == "Error"
	}
	return false
}

// isSettleRelatedFn 判断这个函数是否在结算拒绝路径上。
//
// 用**名字**而不是调用图：调用图会把「被结算调用的工具函数」也算进来，
// 而那里构造错误往往是合理的。名单式判断够用，且可被复核。
func isSettleRelatedFn(pkg, fn string) bool {
	if pkg == "domain" {
		switch fn {
		case "ValidateSettle", "validateReportCollections", "validateHPLeft":
			return true
		}
		return false
	}
	return strings.Contains(fn, "Settle") || strings.Contains(fn, "settle")
}

func callSource(fset *token.FileSet, res ast.Expr) string {
	start := fset.Position(res.Pos())
	end := fset.Position(res.End())
	if start.Filename != end.Filename {
		return "<跨文件>"
	}
	raw, err := os.ReadFile(start.Filename)
	if err != nil || start.Offset < 0 || end.Offset > len(raw) || start.Offset >= end.Offset {
		return "<读取失败>"
	}
	txt := string(raw[start.Offset:end.Offset])
	if len(txt) > 90 {
		txt = txt[:90] + "..."
	}
	return strings.ReplaceAll(strings.TrimSpace(txt), "\n", " ")
}

// settleAnonymousExemptions 是具名豁免表。
//
// ⚠️ 每条都必须写清理由 ——「这不是拒绝」这种判断留给写豁免的人，
// 不留给沉默。理由为空时本测试会红。
var settleAnonymousExemptions = map[string]string{}

// TestNoAnonymousErrorsOnSettleRejectionPaths 是本文件的核心：
// 结算拒绝路径上不允许出现**裸构造**的错误。
func TestNoAnonymousErrorsOnSettleRejectionPaths(t *testing.T) {
	found := scanAnonymousErrors(t)

	var violations []string
	settleTotal, settlePlain := 0, 0
	for _, a := range found {
		if !a.InSettle {
			continue
		}
		settleTotal++
		if a.Kind == wrapPlainErr {
			settlePlain++
			continue
		}
		if a.Kind == wrapSentinel {
			continue
		}
		if why, ok := settleAnonymousExemptions[a.Pos]; ok {
			if strings.TrimSpace(why) == "" {
				violations = append(violations,
					a.Pos+" 有豁免但理由为空 —— 「这不是拒绝」这种判断不留给沉默")
			}
			continue
		}
		violations = append(violations,
			a.Pos+" 在 "+a.Enclosing+" 里裸构造了错误："+a.Snippet)
	}

	if len(violations) == 0 {
		t.Logf("结算路径上共 %d 处错误构造（其中 %d 处包 err —— 真 500 是对的），零裸构造",
			settleTotal, settlePlain)
		return
	}
	sort.Strings(violations)
	t.Errorf("结算拒绝路径上有 %d 处裸构造的错误（会落 500 而不是 422）：\n  %s\n\n"+
		"500 的代价：客户端只能显示「服务内部错误」，排查会滑向「服务端坏了」；"+
		"而且它们混进服务故障告警，把真正的故障淹掉。\n"+
		"修法：用 `fmt.Errorf(\"%%w：说明\", ErrXxx)` 包一个已有哨兵，"+
		"并在 sentinel_classification_test.go 的登记表里归类。\n"+
		"若某处确实不该是 422，写进 settleAnonymousExemptions 并说明理由。",
		len(violations), strings.Join(violations, "\n  "))
}

// TestAnonymousErrorScannerIsNotBlind 确认扫描器**真的能看到东西**。
//
// ⚠️ 这是上一条的前提。扫描器坏掉时上一条会安静地报告「零违规」。
func TestAnonymousErrorScannerIsNotBlind(t *testing.T) {
	found := scanAnonymousErrors(t)
	if len(found) == 0 {
		t.Fatal("扫描器一个错误构造都没扫到 —— 它已经坏了，" +
			"而 TestNoAnonymousErrorsOnSettleRejectionPaths 会安静地报告「零违规」")
	}
	t.Logf("扫描器工作正常：共扫到 %d 处错误构造", len(found))
}
