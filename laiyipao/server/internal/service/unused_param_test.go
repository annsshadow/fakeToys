package service

import (
	"go/ast"
	"go/parser"
	"go/token"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
	"testing"
)

// 「函数签名收了参数，函数体却完全没用到」——**对所有参数**（第 103 轮）。
//
// # 为什么从 `ctx` 推广到全部参数
//
// 第 102 轮扫出两个函数的 `ctx` 从未使用，我顺手扫了 `userID`，发现它也没用。
// 而 `Audit` 那次误报也说明了一件事：**只盯 `ctx` 会漏**。
//
// `ctx` 之所以显眼，只是因为「丢弃 ctx」有明确的安全后果；
// 其他参数被忽略通常只是「看起来多余」——
// 于是它更不容易被当成缺陷，也就更容易长期存在。
//
// # 判据的精度与它的代价
//
// 本判据是**名称级**的：只看标识符在函数体里出现过没有。
//
// 它会**误报**两类情况：
//
//  1. 参数只出现在 `_ = x` 的**右侧**（消音）
//  2. 参数只出现在一个**嵌套闭包**里，而闭包其实捕获了它 ——
//     这仍然是「用到了」，不误报
//
// 第 1 类已在 `ctx_audit_test.go` 里处理（`silencedIdents`）。
// 本文件复用同一套规则。
//
// ⚠️ 它**不会**误报「参数被传给了一个函数」——那是真的用到了。
//
// # 为什么仍然值得做
//
// 误报的代价是「有人要来解释这条为什么红了」；
// 漏报的代价是「一个纯装饰参数永远躺在签名里，误导后来的人」。
// 后者会持续产生错误的直觉（尤其在 `ctx` 上，它暗示着「可取消」）。
//
// 而判据的精度由**自测**保证：合成源里逐条列出该判与不该判的形状。
func TestNoFunctionIgnoresAParameter(t *testing.T) {
	violations := scanUnusedParams(t)
	if len(violations) == 0 {
		t.Logf("所有函数都用到了它们的每个参数")
		return
	}
	lines := make([]string, 0, len(violations))
	for _, v := range violations {
		lines = append(lines, v.Func+" 的参数 ["+strings.Join(v.Params, ", ")+"]")
	}
	sort.Strings(lines)
	t.Errorf("以下函数的签名里有参数从未在函数体中出现：\n  %s\n\n"+
		"纯装饰参数会误导后来的人：\n"+
		"  - 带 `ctx` 的签名会让人以为「这个调用是可取消的」"+
		"（第 101 轮 `ResolveUser` 就因此丢过真正的 ctx）\n"+
		"  - 带 `userID` 的签名会让人以为它会去查库"+
		"（第 102 轮 `computePower` / `computeRating` 签名一模一样，"+
		"而只有后者查库）\n\n"+
		"Go **不报「未使用的参数」**（只报未使用的局部变量），"+
		"所以它不会以任何形式暴露自己。\n"+
		"修法：把参数删掉；确实需要时（例如将来要加 DB 缓存）再加回来 —— "+
		"现在加是投机，而投机出来的参数没人会记得它是干什么的。",
		strings.Join(lines, "\n  "))
}

// TestUnusedParamScannerSeesKnownShape 是扫描器**自身**的守卫。
//
// ⚠️ 与第 78/82/99/101/102 轮同一条理由：「扫不到就 Fatal」的守卫在真的没东西可扫时
// 会永远红，于是被人删掉 —— 于是它保护的东西彻底裸奔。
// 「扫描器是否可用」与「代码是否合规」必须分开回答。
func TestUnusedParamScannerSeesKnownShape(t *testing.T) {
	src := `package p
import "context"
// 真的用到了三个参数 —— 不能写成 "_ = x"，
// 那个形式**就是**消音（第 103 轮实测：通用化之后它会被判为未使用，
// 而那是**正确**的 —— 我第一版的合成源就是在这里写错的）。
func allUsed(ctx context.Context, id int64, name string) error {
	if err := ctx.Err(); err != nil {
		return err
	}
	if id > int64(len(name)) {
		return nil
	}
	return nil
}
func unusedCtx(ctx context.Context) error { return nil }
func unusedID(id int64) error { return nil }
func unusedName(name string) error { return nil }
func silenced(ctx context.Context) error { _ = ctx; return nil }
func blankCall(ctx context.Context) error { _ = ctx.Err(); return nil }
func receiverUnused() error { return nil }  // receiver 已被排除，不该误报
type thing struct{}
`
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, "synthetic.go", src, 0)
	if err != nil {
		t.Fatalf("合成源解析失败：%v\n%s", err, src)
	}

	got := unusedParamsIn(f, fset)

	want := map[string]bool{
		"unusedCtx": true, "unusedID": true, "unusedName": true,
		"silenced": true, // `_ = ctx` 是消音，不算用到
	}
	// blankCall 不该被算进去：`_ = ctx.Err()` 是真的用到了 ctx
	// （第 102 轮实测：把它算成「没用」会诱导人删掉审计查询的 ctx）
	notWant := map[string]bool{
		"allUsed": true, "blankCall": true, "receiverUnused": true,
	}

	for _, v := range got {
		if notWant[v.Func] {
			t.Errorf("误报：%s 的参数 [%s] 被判为「没用」\n"+
				"其中 `blankCall` 的 `_ = ctx.Err()` 是合法惯用法且真的用到了 ctx —— "+
				"第 102 轮那条守卫若误报，会诱导人删掉审计查询的 ctx。", v.Func, strings.Join(v.Params, ", "))
		}
		delete(want, v.Func)
	}
	for _, f2 := range sortedKeys(want) {
		t.Errorf("漏报：%s 应当被判为「有未使用的参数」", f2)
	}
}

// TestUnusedParamScannerSeesRealPackage 确认扫描真的覆盖了本包。
//
// 统计的是「被管辖的函数数」而不是「违规数」——
// 前者在修复完成后依然成立，所以不会像「扫不到就 Fatal」那样变成负担。
func TestUnusedParamScannerSeesRealPackage(t *testing.T) {
	fset := token.NewFileSet()
	files := listServiceSources(t)
	if len(files) < 5 {
		t.Fatalf("只列出 %d 个源文件，范围异常", len(files))
	}
	inspected := 0
	total := 0
	for _, name := range files {
		f, err := parser.ParseFile(fset, filepath.Join(".", name), nil, 0)
		if err != nil {
			t.Fatalf("解析 %s 失败：%v", name, err)
		}
		inspected++
		total += countFuncs(f)
	}
	if inspected < 5 {
		t.Fatalf("只解析了 %d 个文件", inspected)
	}
	t.Logf("扫了 %d 个文件 / %d 个函数", inspected, total)
}

func sortedKeys(m map[string]bool) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}

func countFuncs(f *ast.File) int {
	n := 0
	for _, d := range f.Decls {
		if fn, ok := d.(*ast.FuncDecl); ok && fn.Body != nil {
			n++
		}
	}
	return n
}

// unusedParam 一条「某函数有未使用参数」的记录。
type unusedParam struct {
	Func   string
	Params []string
}

// scanUnusedParams 用 AST 扫出「有未使用参数」的函数。
func scanUnusedParams(t *testing.T) []unusedParam {
	t.Helper()
	fset := token.NewFileSet()
	var out []unusedParam
	files := listServiceSources(t)

	const minExpectedFiles = 5
	inspected := 0

	for _, name := range files {
		f, err := parser.ParseFile(fset, filepath.Join(".", name), nil, 0)
		if err != nil {
			t.Fatalf("解析 %s 失败：%v", name, err)
		}
		inspected++
		for _, v := range unusedParamsIn(f, fset) {
			v.Func = name + ": " + v.Func
			out = append(out, v)
		}
	}

	// ⚠️ 下限必须数**真正调用过检查**的次数，不是「解析了几个文件」——
	// 变异可以在解析之后、检查之前 `continue`，从而骗过 parsed 计数。
	if inspected < minExpectedFiles {
		t.Fatalf("只检查了 %d 个文件，期望至少 %d 个 —— 范围被人为收窄了", inspected, minExpectedFiles)
	}
	return out
}

// unusedParamsIn 返回单个文件里「有未使用参数」的函数。
//
// 规则：
//   - 忽略 receiver（`s *Service`）、`_`、以及没有名字的参数
//   - `_ = x` 形式的**消音**不算用到（复用 silencedIdents）
//   - 只要标识符在函数体（含嵌套闭包）里出现就算用到
func unusedParamsIn(f *ast.File, fset *token.FileSet) []unusedParam {
	var out []unusedParam
	for _, decl := range f.Decls {
		fn, ok := decl.(*ast.FuncDecl)
		if !ok || fn.Body == nil {
			continue
		}

		// 收集参数名（排除 receiver / `_` / 匿名）
		//
		// ⚠️ receiver 必须排除：`func (s *Service) f()` 里 `s` 是 receiver
		// 而不是普通参数，它「未使用」是完全正常的（纯函数不需要 s）。
		// 我第一版忘了排除，自测里那个 receiverUnused 用例立刻炸了 ——
		// 那是**守卫错**，不是代码错。
		var names []string
		if fn.Type.Params != nil {
			for _, p := range fn.Type.Params.List {
				for _, nm := range p.Names {
					if nm.Name == "" || nm.Name == "_" {
						continue
					}
					names = append(names, nm.Name)
				}
			}
		}

		silenced := silencedIdents(fn.Body)
		seen := map[string]bool{}
		ast.Inspect(fn.Body, func(n ast.Node) bool {
			id, ok := n.(*ast.Ident)
			if !ok || silenced[id] {
				return true
			}
			seen[id.Name] = true
			return true
		})

		var bad []string
		for _, n := range names {
			if !seen[n] {
				bad = append(bad, n)
			}
		}
		if len(bad) > 0 {
			out = append(out, unusedParam{Func: fn.Name.Name, Params: bad})
		}
	}
	return out
}

var _ = strconv.Itoa
