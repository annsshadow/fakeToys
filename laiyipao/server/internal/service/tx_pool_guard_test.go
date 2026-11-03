// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// tx_pool_guard_test.go — R83：「事务作用域内再取池连接 / 触发懒惰加载」静态守卫。
//
// # 为什么必须有它
//
// R82 的 tasksCache 死锁证明了一件事：这类缺陷 **PG 侧完全不可见**
// （连接被占满时只表现为超时/503，数据库一切正常），只能靠静态扫描防复发：
//
//   - 直接形态：事务闭包（`DB.Tx(ctx, func(tx pgx.Tx) error {...})`）内
//     调用 `s.pool.Query/Exec/Acquire/...` —— 绕过事务另取一条连接；
//   - 间接形态（R82 原型）：闭包内调用带 tx 的函数（bumpTasks），
//     该函数内部触发 singleflight/缓存 loader，loader 再从池取连接；
//   - 传递形态：闭包 → L1 函数 → L2 函数（applyProgress → grantEquipmentByLevel）。
//
// # 判据（不是按函数名猜）
//
// 「事务作用域」= 闭包参数 tx 有效的区间：
//   1. 闭包体内直接出现的 `*.pool.*` / `*.DB.Pool.*` 调用 → 违规；
//   2. 闭包体内直接出现的 `*.getOrLoad(` → 违规（loader 会取池）；
//   3. 闭包内「把 tx 作为实参传出」的调用 → 被调函数纳入作用域（追至 L2），
//      其体内按 1/2 同规则扫描；
//   4. 作用域内 `x := tx` 的别名赋值会把别名纳入 txNames（浅跟踪一层）。
//
// # 下限断言与破坏实验
//
//   - 事务闭包数 ≥ 40（R83 实测 43，防扫描器静默失效扫不到也报绿）；
//   - 事务作用域扩展函数数 ≥ 8；
//   - tx 传递调用总次数 ≥ 50；
//   - 破坏实验（R83 手工验证）：给 bumpTasks 加一行 `s.tasksCache.getOrLoad`
//     或 `s.pool.Ping` → 本守卫红；还原后绿。

import (
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"testing"
)

// txGuardPoolMethods 是「取池」的方法名集合（出现 .*pool*.<这些方法> 即违规）。
var txGuardPoolMethods = map[string]bool{
	"Query": true, "QueryRow": true, "Exec": true, "Acquire": true,
	"Begin": true, "BeginTx": true, "SendBatch": true, "Ping": true,
}

// txFnInfo 是一个函数/方法的扫描单元。
type txFnInfo struct {
	name       string
	file       string
	txParams   map[string]bool // 类型为 pgx.Tx/txType 的形参名
	paramOrder []string        // 形参顺序（实参↔形参按位映射用）
	body       *ast.BlockStmt
}

// txGuardViolation 是一条违规。
type txGuardViolation struct {
	chain  []string // 调用链：闭包位置 → L1 → L2
	kind   string   // "pool" / "getOrLoad"
	detail string
}

func TestTxPoolGuard(t *testing.T) {
	fset := token.NewFileSet()
	files := map[string]*ast.File{}
	funcs := map[string]*txFnInfo{} // key: 函数名（方法与包级同名时方法优先——本仓无此冲突）

	dir := "."
	entries, err := os.ReadDir(dir)
	if err != nil {
		t.Fatalf("读包目录失败：%v", err)
	}
	for _, e := range entries {
		name := e.Name()
		if e.IsDir() || !strings.HasSuffix(name, ".go") || strings.HasSuffix(name, "_test.go") {
			continue
		}
		path := filepath.Join(dir, name)
		f, err := parser.ParseFile(fset, path, nil, 0)
		if err != nil {
			t.Fatalf("解析 %s 失败：%v", path, err)
		}
		files[path] = f
		for _, decl := range f.Decls {
			fd, ok := decl.(*ast.FuncDecl)
			if !ok || fd.Body == nil {
				continue
			}
			info := &txFnInfo{name: fd.Name.Name, file: name, txParams: map[string]bool{}, body: fd.Body}
			if fd.Type.Params != nil {
				for _, field := range fd.Type.Params.List {
					for _, n := range field.Names {
						info.paramOrder = append(info.paramOrder, n.Name)
						if isPgxTxType(field.Type) {
							info.txParams[n.Name] = true
						}
					}
				}
			}
			funcs[fd.Name.Name] = info
		}
	}

	// ---- 扫描 ----
	var closures int
	var extendedFns = map[string][]string{} // 扩展函数名 → 首次发现它的调用链
	var txPassCalls int
	var violations []txGuardViolation

	// analyze 扫描一个函数体：txNames 是作用域内有效的事务变量名。
	var analyze func(info *txFnInfo, txNames map[string]bool, depth int, chain []string)
	analyze = func(info *txFnInfo, txNames map[string]bool, depth int, chain []string) {
		// 浅别名跟踪：x := tx / x := txNames 中任意一个。
		ast.Inspect(info.body, func(n ast.Node) bool {
			as, ok := n.(*ast.AssignStmt)
			if !ok {
				return true
			}
			for i, rhs := range as.Rhs {
				id, ok := rhs.(*ast.Ident)
				if !ok || !txNames[id.Name] || i >= len(as.Lhs) {
					continue
				}
				if lhs, ok := as.Lhs[i].(*ast.Ident); ok {
					txNames[lhs.Name] = true
				}
			}
			return true
		})

		ast.Inspect(info.body, func(n ast.Node) bool {
			call, ok := n.(*ast.CallExpr)
			if !ok {
				return true
			}
			sel, ok := call.Fun.(*ast.SelectorExpr)
			// 违规 B：事务作用域内触发懒惰加载（任何对象的 getOrLoad）。
			if ok && sel.Sel.Name == "getOrLoad" {
				violations = append(violations, txGuardViolation{
					chain: append(append([]string{}, chain...), info.name),
					kind:  "getOrLoad", detail: "事务作用域内调用 getOrLoad（loader 会从池再取连接）",
				})
				return true
			}
			// 违规 A：*.pool.* / *.DB.Pool.* 的取池方法调用。
			if ok && txGuardPoolMethods[sel.Sel.Name] && isPoolAccess(sel.X) {
				violations = append(violations, txGuardViolation{
					chain: append(append([]string{}, chain...), info.name),
					kind:  "pool", detail: "事务作用域内调用池方法 " + sel.Sel.Name,
				})
				return true
			}
			// 追踪：闭包体内的**所有同包函数调用**都在事务作用域内
			// （不只传 tx 的）——R82 原型正是「不传 tx 也死锁」：
			// 被调函数内部触发 loader 与它有没有 tx 无关。
			// 只按「传 tx」追会漏掉 LoadTasks 这类无 tx 形参的读方法
			// （它们在闭包内被调时，体内的 getOrLoad 同样致命）。
			if depth >= 2 {
				return true
			}
			var calleeName string
			switch fun := call.Fun.(type) {
			case *ast.SelectorExpr: // s.method(...) / pkg.Func(...)
				calleeName = fun.Sel.Name
			case *ast.Ident: // 包级函数
				calleeName = fun.Name
			default:
				return true
			}
			callee, ok := funcs[calleeName]
			if !ok || callee.body == nil {
				return true // 包外函数（pgx/stdlib）追不到，也不管
			}
			txPassCalls++
			// 被调函数的 tx 形参若收到实参 tx，则其形参名纳入作用域；
			// 否则 txNames 为空（它内部不能也不该用 tx）。
			nextNames := map[string]bool{}
			for i, param := range callee.paramOrder {
				if i >= len(call.Args) {
					break
				}
				if !callee.txParams[param] {
					continue
				}
				if id, ok := call.Args[i].(*ast.Ident); ok && txNames[id.Name] {
					nextNames[param] = true
				}
			}
			next := append(append([]string{}, chain...), info.name)
			if _, seen := extendedFns[calleeName]; !seen {
				extendedFns[calleeName] = append(next, calleeName)
			}
			analyze(callee, nextNames, depth+1, next)
			return true
		})
	}

	// 找事务闭包：X.DB.Tx(ctx, func(tx ...) error {...})。
	for path, f := range files {
		ast.Inspect(f, func(n ast.Node) bool {
			call, ok := n.(*ast.CallExpr)
			if !ok || len(call.Args) < 2 {
				return true
			}
			sel, ok := call.Fun.(*ast.SelectorExpr)
			if !ok || sel.Sel.Name != "Tx" {
				return true
			}
			lit, ok := call.Args[1].(*ast.FuncLit)
			if !ok || lit.Type.Params == nil || len(lit.Type.Params.List) == 0 {
				return true
			}
			closures++
			pos := fset.Position(call.Pos())
			txNames := map[string]bool{}
			for _, field := range lit.Type.Params.List {
				for _, nm := range field.Names {
					txNames[nm.Name] = true
				}
			}
			chain := []string{filepath.Base(path) + pos.String()[strings.LastIndex(pos.String(), ":"):] + " DB.Tx"}
			analyze(&txFnInfo{name: "<closure>", file: filepath.Base(path), body: lit.Body},
				txNames, 0, chain)
			return true
		})
	}

	// ---- 下限断言（防扫描器静默失效） ----
	if closures < 40 {
		t.Errorf("事务闭包数 %d < 40 —— 扫描器可能没扫到文件（R83 实测 43）", closures)
	}
	if len(extendedFns) < 8 {
		t.Errorf("事务作用域扩展函数数 %d < 8 —— tx 传递追踪可能断了", len(extendedFns))
	}
	if txPassCalls < 50 {
		t.Errorf("tx 传递调用 %d < 50 —— 传递追踪可能没生效", txPassCalls)
	}

	// ---- 清单输出（实测枚举，回报用） ----
	var extNames []string
	for name := range extendedFns {
		extNames = append(extNames, name)
	}
	sort.Strings(extNames)
	t.Logf("事务闭包 %d 处；作用域扩展函数 %d 个：%v", closures, len(extNames), extNames)

	// ---- 违规判定 ----
	for _, v := range violations {
		t.Errorf("违规[%s] %s：%s", v.kind, strings.Join(v.chain, " → "), v.detail)
	}
	if len(violations) == 0 {
		t.Logf("当前无新增违规（R82 的 bumpTasks 已修）；扩展函数全部用 tx 取数")
	}
}

// isPgxTxType 判定形参类型是否 pgx.Tx / txType（本仓别名，admin.go:24）。
func isPgxTxType(expr ast.Expr) bool {
	switch t := expr.(type) {
	case *ast.SelectorExpr: // pgx.Tx
		if id, ok := t.X.(*ast.Ident); ok {
			return id.Name == "pgx" && t.Sel.Name == "Tx"
		}
	case *ast.Ident: // txType
		return t.Name == "txType"
	}
	return false
}

// isPoolAccess 判定表达式是否「连接池字段访问」：X.pool / X.DB.Pool / X.db.Pool 等。
func isPoolAccess(expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	if strings.EqualFold(sel.Sel.Name, "pool") {
		return true
	}
	// *.DB.Pool：内层是 .DB 字段。
	if inner, ok := sel.X.(*ast.SelectorExpr); ok {
		return inner.Sel.Name == "DB" && strings.EqualFold(sel.Sel.Name, "Pool")
	}
	return false
}

func cloneNameSet(in map[string]bool) map[string]bool {
	out := make(map[string]bool, len(in))
	for k := range in {
		out[k] = true
	}
	return out
}
