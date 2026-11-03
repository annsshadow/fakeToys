// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R74-Q4：审计完整性守卫 —— 每条后台写路由都必须留下审计行。
//
// # 为什么需要它
//
// R34–R48 逐轮把「值钱/有破坏性的后台动作」改成了「业务与审计同一事务」，
// 但那是**逐条修**的：下一版新增一条写路由、忘了接审计，没有任何东西会红。
// readonly_guard_test.go 守的是「谁能调」（requireWritable），
// 本文件守的是「调完有没有留痕」—— 前者管权限，后者管追责，是两个维度。
//
// # 判据
//
// 对 routes.go 里每条 `adm.{Post|Put|Patch|Delete}` 路由，取出 handler 名
// （`s.adminXxx`），再在本包全部非测试 .go 文件里解析该函数体，要求出现
// 以下三种审计痕迹之一：
//
//	① `s.Svc.Audit(`      —— handler 直接写（读类/会话类动作用这个）；
//	② `AuditTx(`          —— handler 体内直接调事务版（少见但合法）；
//	③ `AdminAuditCtx{`    —— 把上下文传给 service，由 service 在业务事务内写。
//
// ③ 是 R34 起的主模式：审计与业务同一事务，「无审计即无操作」。
//
// # 豁免
//
// 与 requireWritable 的豁免表同理：显式、带理由、运行时打印、自检不死。
// 当前豁免为空 —— 19 条写路由在 R34–R48 后全部有审计。
// 新增豁免必须先写得出理由（例如「纯内存切换、无数据库写」）。

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

// auditRouteExemptions 是「后台写路由不写审计」的唯一豁免表（键=路径模板）。
var auditRouteExemptions = map[string]string{}

func TestEveryAdminWriteRouteWritesAudit(t *testing.T) {
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, "routes.go", nil, 0)
	if err != nil {
		t.Fatalf("解析 routes.go 失败：%v", err)
	}
	groups := adminGroupNames(f)
	if len(groups) == 0 {
		t.Fatal("找不到挂在 requireAdmin 上的路由组 —— 守卫本身失效了")
	}

	// 收集写路由 → handler 名（最后一个 s.xxx 选择器参数）。
	type routeRef struct {
		pat     string
		handler string
	}
	var routes []routeRef
	ast.Inspect(f, func(n ast.Node) bool {
		call, ok := n.(*ast.CallExpr)
		if !ok {
			return true
		}
		sel, ok := call.Fun.(*ast.SelectorExpr)
		if !ok {
			return true
		}
		recv, ok := sel.X.(*ast.Ident)
		if !ok || !groups[recv.Name] || !adminWriteMethods[sel.Sel.Name] {
			return true
		}
		for i := len(call.Args) - 1; i >= 0; i-- {
			if hs, ok := call.Args[i].(*ast.SelectorExpr); ok {
				if x, ok := hs.X.(*ast.Ident); ok && x.Name == "s" {
					routes = append(routes, routeRef{pat: routePattern(call), handler: hs.Sel.Name})
					break
				}
			}
		}
		return true
	})
	if len(routes) < 15 {
		t.Fatalf("只扫到 %d 条后台写路由（预期 ≥15）—— 守卫本身可能失效", len(routes))
	}

	// 解析本包全部非测试 .go，标出每个函数体是否含审计痕迹。
	auditMark := map[string]bool{}
	files := listHTTPSources(t)
	for _, file := range files {
		if strings.HasSuffix(file, "_test.go") {
			continue
		}
		ff, err := parser.ParseFile(fset, file, nil, 0)
		if err != nil {
			t.Fatalf("解析 %s 失败：%v", file, err)
		}
		ast.Inspect(ff, func(n ast.Node) bool {
			fn, ok := n.(*ast.FuncDecl)
			if !ok || fn.Body == nil {
				return true
			}
			found := false
			ast.Inspect(fn.Body, func(m ast.Node) bool {
				if found {
					return false
				}
				switch node := m.(type) {
				case *ast.CallExpr:
					if s2, ok := node.Fun.(*ast.SelectorExpr); ok &&
						(s2.Sel.Name == "Audit" || s2.Sel.Name == "AuditTx") {
						found = true
					}
				case *ast.CompositeLit:
					if id, ok := node.Type.(*ast.SelectorExpr); ok && id.Sel.Name == "AdminAuditCtx" {
						found = true
					}
				}
				return true
			})
			if found {
				auditMark[fn.Name.Name] = true
			}
			return true
		})
	}

	var missing []string
	for _, r := range routes {
		if auditMark[r.handler] {
			continue
		}
		if reason, ok := auditRouteExemptions[r.pat]; ok {
			t.Logf("豁免 %s（%s）：%s", r.pat, r.handler, reason)
			continue
		}
		missing = append(missing, fmt.Sprintf("%s → %s", r.pat, r.handler))
	}
	if len(missing) > 0 {
		t.Errorf("这 %d 条后台写路由的 handler 体内**没有任何审计痕迹**：\n  %s\n\n"+
			"后果：这类操作生效后可以没有任何「谁、何时、对什么」的记录，且没人会发现。\n"+
			"写法：值钱/破坏性动作把 `service.AdminAuditCtx{AdminID: adminIDFrom(c), IP: c.IP()}` 传给\n"+
			"service 并在业务事务内 AuditTx（R34 模式）；会话/读类动作用 s.Svc.Audit(...)；\n"+
			"确属例外的，往 auditRouteExemptions 写理由。",
			len(missing), strings.Join(missing, "\n  "))
	}
	// 反向保护：豁免表不能变成死注释（路由改名/补挂审计后条目必须清掉）。
	for pat := range auditRouteExemptions {
		found := false
		for _, r := range routes {
			if r.pat == pat && !auditMark[r.handler] {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("豁免表里的 %q 已不对应「无审计的写路由」（补上了？改名了？）—— 清掉这条豁免", pat)
		}
	}
	t.Logf("审计覆盖：%d 条后台写路由全部有审计痕迹（豁免 %d 条）", len(routes), len(auditRouteExemptions))
}
