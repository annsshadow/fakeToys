// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"go/ast"
	"go/parser"
	"go/token"
	"net/http/httptest"
	"os"
	"path/filepath"
	"regexp"
	"runtime"
	"sort"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"
)

// 对外错误码的契约测试（R68）。
//
// # 为什么需要它
//
// R67 修的是「瞬时故障被报成 500 而不是 503」。那类问题的根因不是
// 某一条 if 判错，而是**没有任何东西在看「错误码 → 状态码 → 重试语义」
// 这张表是否一致**：
//
//   - 同一个语义两个码（`bad_id` / `bad_body` / `bad_input`）；
//   - 503 是该重试的，但只有 `service_unavailable` 写了 Retry-After，
//     另外两个 503 和 429 都没有 —— 客户端只能猜等多久；
//   - 新增错误码时没有任何测试要求它登记状态与重试语义。
//
// 客户端按 `code` 做分支，所以这类漂移的终点是**客户端静默走兜底分支**：
// 用户看到「未知错误」，而服务端日志一切正常，排查会绕很久。
//
// # 本测试做三件事
//
//  1. AST 扫描**所有** `fail(...)` 调用点：码必须是 `errorcodes.go`
//     里的常量（字符串字面量直接红），且状态与契约表一致；
//  2. 契约表自身：码格式、值唯一、每个码都被真实调用过（无死码）；
//  3. 运行时驱动每个码走一遍 `fail`，断言响应状态与 Retry-After
//     头**与本机行为**一致，而不只是表里写着。
//
// ⚠️ 与「集合规模下限」同理：扫描必须断言「至少看到 N 个调用点」，
// 否则把 handler 全部删掉/改名后，本测试会以「0 违规」通过。

// minFailCallSites 是 `fail(...)` 调用点的下限。
const minFailCallSites = 80

func TestErrorCodeContract(t *testing.T) {
	consts, callSites := scanFailCallSites(t)

	// ---- 1. 契约表自身 ----
	//
	// ⚠️ errorContracts 的键是**码值**（"bad_input"），不是常量名。
	// 常量 → 值的映射由 AST 从 errorcodes.go 里读出来（consts）。
	nameRe := regexp.MustCompile(`^[a-z][a-z0-9_]*$`)
	valueToIdents := map[string][]string{}
	for ident, value := range consts {
		if !strings.HasPrefix(ident, "Code") {
			continue
		}
		valueToIdents[value] = append(valueToIdents[value], ident)
		if _, ok := errorContracts[value]; !ok {
			t.Errorf("常量 %s = %q 没有登记进 errorContracts —— 新码必须同时声明状态与重试语义",
				ident, value)
		}
	}
	for value, idents := range valueToIdents {
		if len(idents) > 1 {
			t.Errorf("错误码 %q 被多个常量复用（%s）—— 客户端无法按码分支",
				value, strings.Join(idents, ", "))
		}
	}

	for code, c := range errorContracts {
		if !nameRe.MatchString(code) {
			t.Errorf("错误码 %q 不符合对外格式（小写 snake_case）", code)
		}
		if c.Status < 400 {
			t.Errorf("%q 的状态 %d 不是错误状态", code, c.Status)
		}
		if c.RetryAfter != "" {
			if !regexp.MustCompile(`^[0-9]+$`).MatchString(c.RetryAfter) {
				t.Errorf("%s 的 RetryAfter = %q，必须是秒数字符串", code, c.RetryAfter)
			}
			// Retry-After 只对「稍后重试有意义」的状态有意义。
			if c.Status != fiber.StatusTooManyRequests && c.Status != fiber.StatusServiceUnavailable {
				t.Errorf("%s（%d）配了 Retry-After，但该状态不表示「稍后重试」", code, c.Status)
			}
		}
		// 503 / 429 必须带 Retry-After：这两个状态的语义就是让客户端退避。
		if (c.Status == fiber.StatusTooManyRequests || c.Status == fiber.StatusServiceUnavailable) && c.RetryAfter == "" {
			t.Errorf("%s 是 %d，但契约没给 Retry-After —— 客户端只能猜等多久", code, c.Status)
		}
	}

	// ---- 2. AST：调用点必须用常量，状态必须与契约一致 ----
	observed := map[string]int{}
	for _, site := range callSites {
		value, ok := consts[site.CodeIdent]
		if !ok {
			t.Errorf("%s 的码 %q 不是 errorcodes.go 里定义的常量", site.Pos, site.CodeIdent)
			continue
		}
		observed[value]++
		contract, ok := errorContracts[value]
		if !ok {
			t.Errorf("%s 使用了未登记的码 %q —— 请先在 errorContracts 登记状态与重试语义", site.Pos, value)
			continue
		}
		if site.Status != contract.Status {
			t.Errorf("%s：码 %s 契约状态是 %d，但调用点写的是 %d —— 同码两状态",
				site.Pos, value, contract.Status, site.Status)
		}
	}

	// 集合下限：调用点数量掉下来说明扫描器失效或 handler 被删空。
	if len(callSites) < minFailCallSites {
		t.Fatalf("只扫到 %d 个 fail(...) 调用点（下限 %d）—— 扫描器失效，本测试等于没跑",
			len(callSites), minFailCallSites)
	}

	// 每个登记的错误码都必须真的被发出过：死码会让表越来越像文档而非契约。
	var dead []string
	for name := range errorContracts {
		if observed[name] == 0 {
			dead = append(dead, name)
		}
	}
	sort.Strings(dead)
	if len(dead) > 0 {
		t.Errorf("以下错误码在契约表里登记但**没有任何调用点**（死码）：%s",
			strings.Join(dead, ", "))
	}

	t.Logf("契约码 %d 个，fail 调用点 %d 个：%v", len(errorContracts), len(callSites), observed)
}

// TestErrorCodeRuntimeBehavior 用真实 Fiber 响应验证契约表不是「纸面文档」。
func TestErrorCodeRuntimeBehavior(t *testing.T) {
	app := fiber.New()
	app.Get("/emit/:code", func(c *fiber.Ctx) error {
		code := c.Params("code")
		contract, ok := contractOf(code)
		if !ok {
			return fiber.NewError(fiber.StatusNotFound)
		}
		return fail(c, contract.Status, code, "契约测试")
	})

	for name, contract := range errorContracts {
		t.Run(name, func(t *testing.T) {
			resp, err := app.Test(httptest.NewRequest("GET", "/emit/"+name, nil))
			if err != nil {
				t.Fatalf("请求失败：%v", err)
			}
			defer resp.Body.Close()
			if resp.StatusCode != contract.Status {
				t.Errorf("HTTP 状态 = %d，契约要求 %d", resp.StatusCode, contract.Status)
			}
			got := resp.Header.Get(fiber.HeaderRetryAfter)
			if got != contract.RetryAfter {
				t.Errorf("Retry-After = %q，契约要求 %q", got, contract.RetryAfter)
			}
		})
	}
}

// failCallSite 是一个 `fail(...)` 调用点。
type failCallSite struct {
	Pos       string
	CodeIdent string
	Status    int
}

// scanFailCallSites 解析包内所有非测试文件，收集 fail 调用点与常量表。
func scanFailCallSites(t *testing.T) (map[string]string, []failCallSite) {
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

	// fiber 状态常量名 → 数值。只登记本包实际用到的。
	statusByName := map[string]int{
		"StatusBadRequest":            fiber.StatusBadRequest,
		"StatusUnauthorized":          fiber.StatusUnauthorized,
		"StatusForbidden":             fiber.StatusForbidden,
		"StatusNotFound":              fiber.StatusNotFound,
		"StatusRequestEntityTooLarge": fiber.StatusRequestEntityTooLarge,
		"StatusUnprocessableEntity":   fiber.StatusUnprocessableEntity,
		"StatusTooManyRequests":       fiber.StatusTooManyRequests,
		"StatusInternalServerError":   fiber.StatusInternalServerError,
		"StatusServiceUnavailable":    fiber.StatusServiceUnavailable,
	}

	consts := map[string]string{}
	var sites []failCallSite
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

		// 收集 `CodeX = "bad_input"` 形式的常量。
		for _, decl := range f.Decls {
			gd, ok := decl.(*ast.GenDecl)
			if !ok || gd.Tok != token.CONST {
				continue
			}
			for _, spec := range gd.Specs {
				vs, ok := spec.(*ast.ValueSpec)
				if !ok {
					continue
				}
				for i, ident := range vs.Names {
					if i >= len(vs.Values) {
						continue
					}
					bl, ok := vs.Values[i].(*ast.BasicLit)
					if !ok || bl.Kind != token.STRING {
						continue
					}
					consts[ident.Name] = strings.Trim(bl.Value, `"`)
				}
			}
		}

		ast.Inspect(f, func(n ast.Node) bool {
			call, ok := n.(*ast.CallExpr)
			if !ok {
				return true
			}
			fn, ok := call.Fun.(*ast.Ident)
			if !ok || fn.Name != "fail" || len(call.Args) != 4 {
				return true
			}
			// 第 2 个参数：fiber.Status*。
			sel, ok := call.Args[1].(*ast.SelectorExpr)
			if !ok {
				return true
			}
			status, ok := statusByName[sel.Sel.Name]
			if !ok {
				return true // 非常见状态名：交给人工看，但下方数量下限会兜住
			}
			// 第 3 个参数：必须是标识符（常量），字符串字面量直接红。
			codeIdent, ok := call.Args[2].(*ast.Ident)
			if !ok {
				t.Errorf("%s 的 fail 调用把错误码写成字面量/表达式，必须用 errorcodes.go 的常量",
					fset.Position(call.Pos()).String())
				return true
			}
			sites = append(sites, failCallSite{
				Pos:       fset.Position(call.Pos()).String(),
				CodeIdent: codeIdent.Name,
				Status:    status,
			})
			return true
		})
	}
	return consts, sites
}
