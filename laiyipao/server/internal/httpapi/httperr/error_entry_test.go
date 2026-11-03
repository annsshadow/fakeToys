// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httperr_test

import (
	"context"
	"encoding/json"
	"go/ast"
	"go/parser"
	"go/token"
	"io"
	"net"
	"net/http/httptest"
	"os"
	"path/filepath"
	"regexp"
	"runtime"
	"sort"
	"strings"
	"syscall"
	"testing"

	"github.com/gofiber/fiber/v2"
	"github.com/jackc/pgx/v5/pgconn"

	"github.com/laiyipao/server/internal/httpapi/httperr"
	"github.com/laiyipao/server/internal/service"
)

// ===== R69：单一错误出口的静态守卫 =====

// minNewCallSites / minScannedFiles 是「集合规模下限」。
//
// 守卫只在异常时报警，而「扫到 0 个」与「全部合规」在输出里一模一样。
// 目录变了、包名改了、或写法换了，守卫会静默失效 —— 下限让它自己先红。
const (
	minNewCallSites = 80
	minScannedFiles = 20
)

func TestErrorEntryIsSingle(t *testing.T) {
	serverDir, err := findServerDir()
	if err != nil {
		t.Fatalf("定位 server 目录失败：%v", err)
	}
	fset := token.NewFileSet()

	newSites, scannedFiles := 0, 0
	var offenders []string

	err = filepath.Walk(serverDir, func(path string, info os.FileInfo, werr error) error {
		if werr != nil {
			return werr
		}
		if info.IsDir() {
			// 跳过 vendor / 测试缓存一类目录。
			if info.Name() == "vendor" || info.Name() == "testdata" || info.Name() == ".git" {
				return filepath.SkipDir
			}
			return nil
		}
		if !strings.HasSuffix(path, ".go") || strings.HasSuffix(path, "_test.go") {
			return nil
		}
		// 入口包自身允许写响应；其他任何地方都不允许。
		entryDir := filepath.Join(serverDir, "internal", "httpapi", "httperr")
		rel, _ := filepath.Rel(entryDir, path)
		inEntry := !strings.HasPrefix(rel, "..")

		f, perr := parser.ParseFile(fset, path, nil, 0)
		if perr != nil {
			t.Fatalf("解析 %s 失败：%v", path, perr)
		}
		scannedFiles++

		ast.Inspect(f, func(n ast.Node) bool {
			call, ok := n.(*ast.CallExpr)
			if !ok {
				return true
			}
			id, ok := call.Fun.(*ast.Ident)
			if !ok {
				return true
			}
			// R69 的核心判据：`fail(` / `failErr(` 只能出现在入口包内。
			// 包外一旦出现，就说明有人绕过了单一出口 —— 哪怕它当时
			// 「看起来也能用」，下次就会和 Render 的口径分叉。
			if id.Name == "fail" || id.Name == "failErr" {
				if !inEntry {
					offenders = append(offenders, fset.Position(call.Pos()).String()+" -> "+id.Name)
				}
			}
			return true
		})

		// 统计 httperr.New 调用点（供契约核对与下限）。
		ast.Inspect(f, func(n ast.Node) bool {
			call, ok := n.(*ast.CallExpr)
			if !ok {
				return true
			}
			sel, ok := call.Fun.(*ast.SelectorExpr)
			if !ok || sel.Sel.Name != "New" {
				return true
			}
			if x, ok := sel.X.(*ast.Ident); !ok || x.Name != "httperr" {
				return true
			}
			newSites++
			return true
		})
		return nil
	})
	if err != nil {
		t.Fatalf("遍历失败：%v", err)
	}

	sort.Strings(offenders)
	if len(offenders) > 0 {
		t.Fatalf("以下位置在入口包外写错误响应（`fail`/`failErr`）：\n  %s\n\n"+
			"handler 应只 `return err`，由 httperr.Render 统一写响应。",
			strings.Join(offenders, "\n  "))
	}
	if scannedFiles < minScannedFiles {
		t.Fatalf("只扫描了 %d 个 Go 文件（下限 %d）—— 路径或遍历失效，本守卫等于没跑",
			scannedFiles, minScannedFiles)
	}
	if newSites < minNewCallSites {
		t.Fatalf("只找到 %d 个 httperr.New 调用点（下限 %d）—— handler 写法变了或扫描失效",
			newSites, minNewCallSites)
	}
	t.Logf("扫描 %d 个文件：0 处包外错误出口，%d 个 httperr.New 调用点", scannedFiles, newSites)
}

// ===== R69：错误码契约（沿用 R68，改成全仓扫描 httperr.New） =====

func TestErrorCodeContract(t *testing.T) {
	serverDir, err := findServerDir()
	if err != nil {
		t.Fatalf("定位 server 目录失败：%v", err)
	}
	// 从 codes.go 读出常量名 → 值。
	consts, sites := scanCodeConstantsAndNewSites(t, serverDir)
	// Render 内部产出的码（不经 handler 的 httperr.New）。
	renderIdents := scanRenderEmittedCodes(t,
		filepath.Join(serverDir, "internal", "httpapi", "httperr"))
	renderEmitted := map[string]bool{}
	for ident := range renderIdents {
		if v, ok := consts[ident]; ok {
			renderEmitted[v] = true
		}
	}

	// 契约表自身。
	nameRe := regexp.MustCompile(`^[a-z][a-z0-9_]*$`)
	contracts := httperr.Contracts()
	nonContract := httperr.NonContractCodes()
	valueToIdents := map[string][]string{}
	for ident, value := range consts {
		if !strings.HasPrefix(ident, "Code") {
			continue
		}
		valueToIdents[value] = append(valueToIdents[value], ident)
		if _, ok := contracts[value]; !ok {
			if _, exempt := nonContract[value]; exempt {
				continue
			}
			t.Errorf("常量 %s = %q 没有登记进契约表（也不是 nonContractCodes）", ident, value)
		}
	}
	for value, idents := range valueToIdents {
		if len(idents) > 1 {
			t.Errorf("错误码 %q 被多个常量复用（%s）", value, strings.Join(idents, ", "))
		}
	}
	for code, c := range contracts {
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
		}
		if (c.Status == fiber.StatusTooManyRequests || c.Status == fiber.StatusServiceUnavailable) &&
			c.RetryAfter == "" {
			t.Errorf("%s 是 %d，但契约没给 Retry-After", code, c.Status)
		}
	}
	for code, why := range nonContract {
		if strings.TrimSpace(why) == "" {
			t.Errorf("nonContractCodes 的 %q 没有写理由", code)
		}
		if _, dup := contracts[code]; dup {
			t.Errorf("%q 同时在契约表与 nonContractCodes 里", code)
		}
	}

	// 调用点：码必须是常量、状态必须与契约一致。
	observed := map[string]int{}
	for _, s := range sites {
		value, ok := consts[s.Code]
		if !ok {
			t.Errorf("%s 的码 %q 不是 codes.go 的常量", s.Pos, s.Code)
			continue
		}
		observed[value]++
		c, ok := contracts[value]
		if !ok {
			t.Errorf("%s 使用了未登记的码 %q", s.Pos, value)
			continue
		}
		if s.Status != c.Status {
			t.Errorf("%s：码 %s 契约状态 %d，调用点写的是 %d", s.Pos, value, c.Status, s.Status)
		}
	}

	var dead []string
	for code := range contracts {
		// 「有没有调用点」不能只看 httperr.New：一部分码是 Render **内部**
		// 由哨兵/瞬时故障映射出来的（forbidden/not_found/service_unavailable…），
		// 它们按设计不会出现在任何 handler 里。若把它们当成死码，唯一的
		// 「修复」就是去 handler 里硬写一个永远走不到的 New —— 那才是真正的死码。
		//
		// 所以判据是「被 New 构造过 **或** 被 Render 的实现引用过」。
		if observed[code] == 0 && !renderEmitted[code] {
			dead = append(dead, code)
		}
	}
	sort.Strings(dead)
	if len(dead) > 0 {
		t.Errorf("契约表里登记但既无调用点、Render 也不产出的死码：%s", strings.Join(dead, ", "))
	}
	t.Logf("契约码 %d 个，httperr.New 调用点 %d 个，Render 产出 %d 个：%v",
		len(contracts), len(sites), len(renderEmitted), observed)
}

// scanRenderEmittedCodes 收集**入口包实现文件**里引用到的错误码常量名。
//
// 排除 codes.go 自身：那里声明了全部常量，若把它算进来，每个码都会
// 「被引用」，死码检查立刻变成永远为真 —— 守卫看着在跑，其实什么都没查。
func scanRenderEmittedCodes(t *testing.T, entryDir string) map[string]bool {
	t.Helper()
	out := map[string]bool{}
	fset := token.NewFileSet()
	files := 0
	err := filepath.Walk(entryDir, func(path string, info os.FileInfo, werr error) error {
		if werr != nil {
			return werr
		}
		if info.IsDir() || !strings.HasSuffix(path, ".go") || strings.HasSuffix(path, "_test.go") {
			return nil
		}
		if info.Name() == "codes.go" {
			return nil
		}
		f, perr := parser.ParseFile(fset, path, nil, 0)
		if perr != nil {
			t.Fatalf("解析 %s 失败：%v", path, perr)
		}
		files++
		ast.Inspect(f, func(n ast.Node) bool {
			switch x := n.(type) {
			case *ast.Ident:
				if strings.HasPrefix(x.Name, "Code") && len(x.Name) > len("Code") {
					out[x.Name] = true
				}
			case *ast.SelectorExpr:
				if strings.HasPrefix(x.Sel.Name, "Code") && len(x.Sel.Name) > len("Code") {
					out[x.Sel.Name] = true
				}
			}
			return true
		})
		return nil
	})
	if err != nil {
		t.Fatalf("遍历入口包失败：%v", err)
	}
	// 集合规模下限：Render 至少要引用 6 个码（forbidden/not_found/
	// export_too_large/service_unavailable/settle_rejected/export_busy）。
	// 少于下界说明扫描没读到实现文件，下面的死码检查会失去意义。
	if files < 2 || len(out) < 6 {
		t.Fatalf("只解析 %d 个实现文件、收集到 %d 个码常量（下限 2/6）—— 扫描失效",
			files, len(out))
	}
	return out
}

// TestRenderIsTheOnlyPathToFiberError 确认 main.go 的唯一接线点。
func TestRenderIsTheOnlyPathToFiberError(t *testing.T) {
	serverDir, err := findServerDir()
	if err != nil {
		t.Fatal(err)
	}
	raw, err := os.ReadFile(filepath.Join(serverDir, "cmd", "api", "main.go"))
	if err != nil {
		t.Fatalf("读 main.go 失败：%v", err)
	}
	if !strings.Contains(string(raw), "httperr.Render(c, err)") {
		t.Fatal("main.go 的 ErrorHandler 没有接 httperr.Render —— 单一出口未接线")
	}
}

// ===== R69：运行时行为（可对照改前/改后） =====

func TestRenderBehaviorMatrix(t *testing.T) {
	cases := []struct {
		name       string
		err        error
		wantStatus int
		wantCode   string
		wantRetry  string
	}{
		// handler 主动构造
		{"typed_bad_json", httperr.New(fiber.StatusBadRequest, httperr.CodeBadJSON, "x"), 400, "bad_json", ""},
		{"typed_rate_limited", httperr.New(fiber.StatusTooManyRequests, httperr.CodeRateLimited, "x"), 429, "rate_limited", "60"},
		{"typed_export_busy", httperr.New(fiber.StatusServiceUnavailable, httperr.CodeExportBusy, "x"), 503, "export_busy", "3"},
		{"typed_db_unavailable", httperr.New(fiber.StatusServiceUnavailable, httperr.CodeDBUnavailable, "x"), 503, "db_unavailable", "5"},
		// 框架错误
		{"fiber_404", fiber.NewError(404, "not found"), 404, "http_error", ""},
		{"fiber_405", fiber.NewError(405, "method"), 405, "http_error", ""},
		// service 哨兵
		{"unauthorized", service.ErrUnauthorized, 401, "unauthorized", ""},
		{"forbidden", service.ErrForbidden, 403, "forbidden", ""},
		{"not_found", service.ErrNotFound, 404, "not_found", ""},
		{"bad_input", service.ErrBadInput, 400, "bad_input", ""},
		{"export_too_large", service.ErrExportTooLarge, 413, "export_too_large", ""},
		{"export_busy", httperr.ErrExportBusy, 503, "export_busy", "3"},
		// 依赖瞬时故障
		{"pg_08006", &pgconn.PgError{Code: "08006"}, 503, "service_unavailable", "5"},
		{"pg_57014", &pgconn.PgError{Code: "57014"}, 503, "service_unavailable", "5"},
		{"pg_53300", &pgconn.PgError{Code: "53300"}, 503, "service_unavailable", "5"},
		{"pg_53200", &pgconn.PgError{Code: "53200"}, 503, "service_unavailable", "5"},
		{"net_refused", &net.OpError{Op: "dial", Err: syscall.ECONNREFUSED}, 503, "service_unavailable", "5"},
		{"io_eof", io.EOF, 503, "service_unavailable", "5"},
		// 负锚点：重试无意义 → 500
		{"pg_23505", &pgconn.PgError{Code: "23505"}, 500, "internal_error", ""},
		{"pg_42P01", &pgconn.PgError{Code: "42P01"}, 500, "internal_error", ""},
		{"canceled", context.Canceled, 500, "internal_error", ""},
		{"unknown", io.ErrClosedPipe, 500, "internal_error", ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			app := fiber.New(fiber.Config{ErrorHandler: httperr.Render})
			app.Get("/x", func(c *fiber.Ctx) error { return tc.err })
			resp, err := app.Test(httptest.NewRequest("GET", "/x", nil))
			if err != nil {
				t.Fatalf("请求失败：%v", err)
			}
			defer resp.Body.Close()
			if resp.StatusCode != tc.wantStatus {
				t.Errorf("status = %d，期望 %d", resp.StatusCode, tc.wantStatus)
			}
			if got := resp.Header.Get(fiber.HeaderRetryAfter); got != tc.wantRetry {
				t.Errorf("Retry-After = %q，期望 %q", got, tc.wantRetry)
			}
			raw, _ := io.ReadAll(resp.Body)
			var body httperr.APIError
			if err := json.Unmarshal(raw, &body); err != nil {
				t.Fatalf("响应不是 JSON：%q", raw)
			}
			if body.Error.Code != tc.wantCode {
				t.Errorf("code = %q，期望 %q", body.Error.Code, tc.wantCode)
			}
		})
	}
}

// ===== 扫描工具 =====

type newSite struct {
	Pos    string
	Code   string
	Status int
}

func scanCodeConstantsAndNewSites(t *testing.T, serverDir string) (map[string]string, []newSite) {
	t.Helper()
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
	var sites []newSite
	fset := token.NewFileSet()

	// 常量从入口包的 codes.go 读。
	f, err := parser.ParseFile(fset, filepath.Join(serverDir, "internal", "httpapi", "httperr", "codes.go"), nil, 0)
	if err != nil {
		t.Fatalf("解析 codes.go 失败：%v", err)
	}
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

	_ = filepath.Walk(serverDir, func(path string, info os.FileInfo, werr error) error {
		if werr != nil || info.IsDir() || !strings.HasSuffix(path, ".go") ||
			strings.HasSuffix(path, "_test.go") {
			return nil
		}
		pf, perr := parser.ParseFile(fset, path, nil, 0)
		if perr != nil {
			t.Fatalf("解析 %s 失败：%v", path, perr)
		}
		ast.Inspect(pf, func(n ast.Node) bool {
			call, ok := n.(*ast.CallExpr)
			if !ok {
				return true
			}
			sel, ok := call.Fun.(*ast.SelectorExpr)
			if !ok || sel.Sel.Name != "New" {
				return true
			}
			x, ok := sel.X.(*ast.Ident)
			if !ok || x.Name != "httperr" || len(call.Args) < 2 {
				return true
			}
			codeSel, ok := call.Args[1].(*ast.SelectorExpr)
			if !ok {
				t.Errorf("%s 的错误码必须写成 httperr.CodeX 常量", fset.Position(call.Pos()).String())
				return true
			}
			statusSel, ok := call.Args[0].(*ast.SelectorExpr)
			if !ok {
				return true
			}
			status, ok := statusByName[statusSel.Sel.Name]
			if !ok {
				return true
			}
			sites = append(sites, newSite{
				Pos:    fset.Position(call.Pos()).String(),
				Code:   codeSel.Sel.Name,
				Status: status,
			})
			return true
		})
		return nil
	})
	return consts, sites
}

func findServerDir() (string, error) {
	_, thisFile, _, ok := runtime.Caller(0)
	if !ok {
		return "", os.ErrNotExist
	}
	dir := filepath.Dir(thisFile)                    // .../server/internal/httpapi/httperr
	return filepath.Join(dir, "..", "..", ".."), nil // .../server
}
