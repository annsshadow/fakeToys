// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"go/ast"
	"go/parser"
	"go/token"
	"io/fs"
	"path/filepath"
	"runtime"
	"strings"
	"testing"
)

// TestEveryAdminExportChecksRowsErr 保证每个 AdminExport*（CSV 导出）的
// 行迭代都以 rows.Err() 收尾。
//
// ## 为什么这道测试必须存在（第 2 轮）
//
// 迭代中途断连会让 rows.Next() 静默返回 false —— 循环正常退出，
// 不查 rows.Err() 就会把**截断的 CSV 当成功返回**：运营拿到的
// 补偿/对账快照少了尾部若干行，而接口是 200。
//
// 实测恰有两个导出漏了这道检查（AdminExportDefenseChallenges /
// AdminExportRedeemUsages），其余 8 个都有。列表函数全部以
// `return ..., rows.Err()` 收尾，只有导出函数是"写完 CSV 直接返回 buf"的
// 形状，最容易在新增时漏掉。
//
// ## 这道测试怎么做到「完备」
//
// 用 go/parser 扫 service 包全部 AdminExport* 函数体，断言至少出现一次
// 对 `*rows` 形态变量的 `.Err()` 调用。新增导出函数漏查即红。
func TestEveryAdminExportChecksRowsErr(t *testing.T) {
	_, thisFile, _, ok := runtime.Caller(0)
	if !ok {
		t.Fatal("runtime.Caller(0) 失败，无法定位 service 包源码目录")
	}
	dir := filepath.Dir(thisFile)

	fset := token.NewFileSet()
	pkgs, err := parser.ParseDir(fset, dir, func(fi fs.FileInfo) bool {
		return strings.HasSuffix(fi.Name(), ".go") && !strings.HasSuffix(fi.Name(), "_test.go")
	}, 0)
	if err != nil {
		t.Fatalf("解析 service 包失败: %v", err)
	}

	exportCount := 0
	for _, pkg := range pkgs {
		for _, file := range pkg.Files {
			for _, decl := range file.Decls {
				fn, ok := decl.(*ast.FuncDecl)
				if !ok || fn.Recv == nil || !strings.HasPrefix(fn.Name.Name, "AdminExport") {
					continue
				}
				exportCount++
				if !callsRowsErr(fn.Body) {
					t.Errorf("%s 的行迭代没有以 rows.Err() 收尾 —— 迭代中途断连会把截断的 CSV 当成功返回（fail-loud 缺口）", fn.Name.Name)
				}
			}
		}
	}
	if exportCount < 10 {
		t.Fatalf("只扫到 %d 个 AdminExport* 函数，扫描器可能失灵（应至少 10 个）", exportCount)
	}
}

// callsRowsErr 递归查找函数体里是否有 `X.Err()` 调用且 X 的名字以 rows 结尾
// （覆盖 rows / srows / vrows 等既有命名）。
func callsRowsErr(node ast.Node) bool {
	found := false
	ast.Inspect(node, func(n ast.Node) bool {
		if found {
			return false
		}
		call, ok := n.(*ast.CallExpr)
		if !ok || len(call.Args) != 0 {
			return true
		}
		sel, ok := call.Fun.(*ast.SelectorExpr)
		if !ok || sel.Sel.Name != "Err" {
			return true
		}
		id, ok := sel.X.(*ast.Ident)
		if ok && strings.HasSuffix(id.Name, "rows") {
			found = true
			return false
		}
		return true
	})
	return found
}
