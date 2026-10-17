package httpapi

// 第 127 轮：readonly 角色门禁。
//
// admin_users.role 自 00007 迁移就标了 admin/ops/readonly 三个角色，
// 但历史上**没有任何端点消费它** —— readonly 账号可以和 admin 一样
// 封禁玩家、发币、改关卡/商城（「纸面角色」缺陷）。
//
// 修法：requireAdmin 把角色写进 Locals，写端点挂 requireWritable，
// readonly 一律 403；读端点保持放行。
// 本文件把「哪些端点挡、哪些放行、不挡错 ops」钉死。

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"strconv"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"
)

// TestEveryAdminWriteRouteRequiresWritable 是上面那条 e2e 的**结构守卫**。
//
// # 为什么 e2e 不够
//
// `TestE2EReadonlyGating` 的 roWrites 是**手写枚举** —— 它只知道自己
// 列出的那 10 条。于是「新增一条后台写路由、忘了挂 requireWritable」
// 这个动作在测试世界里是**不可见**的：新路由不在枚举里，
// 所有既有断言照样全绿，而线上 readonly 账号立刻能调它。
//
// 这正是本仓库反复记录的根本形态（第 74 轮 walletColumns、
// 第 67 轮 TestProdGuardsPanic、第 78 轮回溯守卫都是它）：
// **守卫覆盖了它写下时的那几项，而不是那个类别。**
//
// # 为什么用 AST 而不是正则
//
// 与 ownership_scan_test.go 同一条理由：正则分不清「声明」与「使用」，
// 也认不出跨行写法。`adm.Put(\n "/x",\n requireWritable(), ...)` 这种
// 换行一旦出现，按行扫描的正则就会静默漏掉它 —— 而漏掉的那条恰好
// 是最需要被看到的一条。
//
// ⚠️ 自检：`TestAdminRouteScannerStillFindsRoutes` 保证扫到的条数
// 没有莫名缩水（结构变了 / 组名换了都会先在那里红）。
func TestEveryAdminWriteRouteRequiresWritable(t *testing.T) {
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, "routes.go", nil, 0)
	if err != nil {
		t.Fatalf("解析 routes.go 失败：%v", err)
	}
	groups := adminGroupNames(f)
	if len(groups) == 0 {
		t.Fatal("在 routes.go 里找不到挂在 \"/admin\" 上的路由组 —— 守卫本身失效了（组前缀改了？）")
	}

	var writes, guarded int
	var missing []string
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
		if !ok || !groups[recv.Name] {
			return true
		}
		if !adminWriteMethods[sel.Sel.Name] {
			return true
		}
		writes++
		for _, a := range call.Args {
			if ac, ok := a.(*ast.CallExpr); ok {
				if id, ok := ac.Fun.(*ast.Ident); ok && id.Name == "requireWritable" {
					guarded++
					return true
				}
			}
		}
		pat := routePattern(call)
		if reason, ok := writableRouteExemptions[pat]; ok {
			t.Logf("豁免 %s：%s", pat, reason)
			return true
		}
		missing = append(missing, fmt.Sprintf("%s %s（%s）",
			sel.Sel.Name, fset.Position(call.Pos()), pat))
		return true
	})

	if writes < 10 {
		t.Fatalf("只扫到 %d 条后台写路由，预期 10+ —— "+
			"要么守卫失效了，要么真的删了一批路由；先确认再改这个下界", writes)
	}
	if len(missing) > 0 {
		t.Errorf("这 %d 条后台写路由**没有**挂 requireWritable，也不在豁免表里：\n  %s\n\n"+
			"admin_users.role 里的 readonly 是靠这个中间件生效的。漏挂的后果不是\n"+
			"「少了个校验」，而是 readonly 账号在那条路由上就是 admin —— "+
			"而 e2e 的 roWrites 是手写枚举，它不会替你发现新路由。\n"+
			"写法：`%s` 的参数里加 `requireWritable()`（必须在 requireAdmin 之后）；\n"+
			"确属豁免的，先往 writableRouteExemptions 里写理由。",
			len(missing), strings.Join(missing, "\n  "), "adm.Put(\"/x\", requireWritable(), s.handler)")
	}
	// 反向保护：不能因为「有一条路由挂了两次」或对不上数而误判
	if guarded+len(writableRouteExemptions) != writes {
		t.Errorf("写路由 %d 条、挂了 requireWritable 的 %d 条、豁免 %d 条 —— 计数对不上，"+
			"说明扫描逻辑本身有问题（不是路由的问题）", writes, guarded, len(writableRouteExemptions))
	}
}

// writableRouteExemptions 是「后台写路由不挂 requireWritable」的**唯一豁免表**。
//
// ⚠️ 这张表必须是**显式的、带理由的**：任何要新增豁免的人，先把理由
// 写在这里（值 = 理由，守卫运行时逐条打印），再改 routes.go。
// 写不出来说服人的理由，就不该加豁免。
//
// 键 = 路径模板（routePattern 的取值）：
//
//   - `/logout`：登出是**会话生命周期**动作（吊销自己全部后台会话），
//     不是数据写。readonly 账号同样（也许更应该）能立刻切断自己手上的
//     凭证 —— 拒绝它等于让误开的后台标签页带着 2h 有效凭证滞留。
//     与玩家侧 /me/logout 不限角色同源。
var writableRouteExemptions = map[string]string{
	"/logout": "登出是会话生命周期，不是数据写（理由同步写在 handlers_admin.go 的 adminLogout 注释里）",
}

// TestWritableRouteExemptionsStillApply 豁免表自检：
// 表里每个键必须**真的**对应 routes.go 里一条未挂 requireWritable 的写路由。
// 否则（该路由补挂了中间件 / 路径改名 / 路由被删）这张表就成了死注释，
// 而「计数对不上」那条断言会红得没头没脑。
func TestWritableRouteExemptionsStillApply(t *testing.T) {
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, "routes.go", nil, 0)
	if err != nil {
		t.Fatalf("解析 routes.go 失败：%v", err)
	}
	groups := adminGroupNames(f)
	var unexempt map[string]bool
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
		for _, a := range call.Args {
			if ac, ok := a.(*ast.CallExpr); ok {
				if id, ok := ac.Fun.(*ast.Ident); ok && id.Name == "requireWritable" {
					return true // 已挂中间件，不属于豁免对象
				}
			}
		}
		if unexempt == nil {
			unexempt = map[string]bool{}
		}
		unexempt[routePattern(call)] = true
		return true
	})
	for pat := range writableRouteExemptions {
		if !unexempt[pat] {
			t.Errorf("豁免表里的 %q 现在**不是**一条未挂 requireWritable 的写路由了"+
				"（它补挂了中间件？路径改名了？路由删了？）—— 把这条豁免连同理由一起清掉", pat)
		}
	}
}

// adminWriteMethods 是「会改数据」的 HTTP 方法。
//
// ⚠️ GET **刻意不在内**：后台读端点对 readonly 保持放行是本轮的设计
// （见 requireWritable 的注释）。把 GET 也拉进来会让 13 条读路由全部变红。
var adminWriteMethods = map[string]bool{
	"Put": true, "Post": true, "Patch": true, "Delete": true,
}

// adminGroupNames 找出 routes.go 里**挂了 requireAdmin** 的路由组变量名。
//
// ⚠️ 只认路径字面量是不够的：`v1.Group("/admin")` 也存在，
// 那是**公开**的后台登录端点（`/admin/login`）—— 它没有管理员身份，
// 自然也谈不上 readonly。第一版按路径判定，于是它立刻报了
// 「/login 没有挂 requireWritable」这条**误报**。
//
// 语义上的分界是「这个组里的请求已经有一个管理员身份」，
// 而那就是 `requireAdmin` 中间件，不是路径前缀。
func adminGroupNames(f *ast.File) map[string]bool {
	names := map[string]bool{}
	ast.Inspect(f, func(n ast.Node) bool {
		as, ok := n.(*ast.AssignStmt)
		if !ok || len(as.Lhs) != 1 || len(as.Rhs) != 1 {
			return true
		}
		lhs, ok := as.Lhs[0].(*ast.Ident)
		if !ok {
			return true
		}
		call, ok := as.Rhs[0].(*ast.CallExpr)
		if !ok || len(call.Args) == 0 {
			return true
		}
		sel, ok := call.Fun.(*ast.SelectorExpr)
		if !ok || sel.Sel.Name != "Group" {
			return true
		}
		lit, ok := call.Args[0].(*ast.BasicLit)
		if !ok || lit.Kind != token.STRING {
			return true
		}
		if p, err := strconv.Unquote(lit.Value); err != nil || p != "/admin" {
			return true
		}
		if !hasMiddlewareArg(call, "requireAdmin") {
			return true
		}
		names[lhs.Name] = true
		return true
	})
	return names
}

// hasMiddlewareArg 判断调用参数里是否出现了指定名字的中间件。
func hasMiddlewareArg(call *ast.CallExpr, name string) bool {
	for _, a := range call.Args {
		if id, ok := a.(*ast.Ident); ok && id.Name == name {
			return true
		}
		if ac, ok := a.(*ast.CallExpr); ok {
			if id, ok := ac.Fun.(*ast.Ident); ok && id.Name == name {
				return true
			}
		}
	}
	return false
}

// routePattern 取路由调用的第一个字符串字面量参数（路径模板），失败则空串。
// 只用于让失败信息**能直接定位到哪条路由**，不影响判定。
func routePattern(call *ast.CallExpr) string {
	for _, a := range call.Args {
		if lit, ok := a.(*ast.BasicLit); ok && lit.Kind == token.STRING {
			if p, err := strconv.Unquote(lit.Value); err == nil {
				return p
			}
		}
	}
	return "?"
}

// TestAdminRouteScannerStillFindsRoutes 守卫自身的有效性自检。
//
// 与 route_consumption_test.go 的 TestRouteScannerStillFindsRoutes 同源：
// 一条「所有 X 都必须挂 Y」的守卫，在**扫不到任何 X** 时会全绿 ——
// 那是最坏的失效方式（看起来在守着，实际什么也没守）。
func TestAdminRouteScannerStillFindsRoutes(t *testing.T) {
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, "routes.go", nil, 0)
	if err != nil {
		t.Fatalf("解析 routes.go 失败：%v", err)
	}
	groups := adminGroupNames(f)
	if len(groups) == 0 {
		t.Fatal("找不到 /admin 路由组：守卫会在「一条都没扫到」的情况下全绿")
	}
	var gets, writes int
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
		if !ok || !groups[recv.Name] {
			return true
		}
		switch {
		case sel.Sel.Name == "Get":
			gets++
		case adminWriteMethods[sel.Sel.Name]:
			writes++
		}
		return true
	})
	if gets < 13 {
		t.Errorf("后台 GET 路由只扫到 %d 条，预期 13+ —— 读/写分界可能已经失准", gets)
	}
	if writes < 10 {
		t.Errorf("后台写路由只扫到 %d 条，预期 10+", writes)
	}
}

func TestE2EReadonlyGating(t *testing.T) {
	e := newE2E(t)
	_, roTok := e.newAdminRole(t, "ro127", "readonly")
	_, opsTok := e.newAdminRole(t, "ops127", "ops")

	// --- readonly：所有写端点必须 403（门禁在 handler 前，参数可非法） ---
	uid, _ := e.newPlayer(t, "rovictim")
	roWrites := []struct {
		method, url string
		body        any
	}{
		{"POST", fmt.Sprintf("/api/v1/admin/users/%d/ban", uid), map[string]any{}},
		{"POST", fmt.Sprintf("/api/v1/admin/users/%d/unban", uid), nil},
		{"POST", fmt.Sprintf("/api/v1/admin/users/%d/grant", uid), map[string]any{"currency": "coin", "amount": 1}},
		{"PUT", "/api/v1/admin/levels/1", map[string]any{"name": "x"}},
		{"POST", "/api/v1/admin/levels/regenerate", nil},
		{"PUT", "/api/v1/admin/skills/1", map[string]any{"name": "x"}},
		{"PUT", "/api/v1/admin/shop/1", map[string]any{"name": "x"}},
		{"POST", "/api/v1/admin/announcements", map[string]any{"title": "x", "body": "y"}},
		{"POST", "/api/v1/admin/redeem-codes", map[string]any{"code": "RO", "reward": map[string]int{"coin": 1}}},
		{"POST", "/api/v1/admin/battles/1/verify", map[string]any{"replay_hash": "0000000000000000"}},
	}
	for _, w := range roWrites {
		var status int
		var body map[string]any
		switch w.method {
		case "PUT":
			status, body = e.put(t, w.url, roTok, w.body)
		default:
			status, body = e.post(t, w.url, roTok, w.body)
		}
		if status != fiber.StatusForbidden {
			t.Errorf("readonly %s %s 应 403，实际 %d %v", w.method, w.url, status, body)
		}
	}

	// --- readonly：读端点保持放行（各 200） ---
	//
	// ⚠️ battles/{id} 必须用**本测试自己结算出来的** id，不能写死 1。
	//
	// 写死 1 是「夹具落在保护之外」的活例：`go test ./...` 默认并行跑包，
	// service 包在同一时刻写同一个 laiyipao 库，battle_records 的
	// sequence 被并发的结算推进（实测全新库首条战报就是 id 29）——
	// 于是这条断言只在「开发机库里恰好躺着重历史的 battle 1」时通过，
	// 干净库 / CI（若哪天给 DB 测试接上 PG）上必红。
	// 门禁要验的是「这个读端点不拦 readonly」，与库里有没有 id=1 无关，
	// 所以自己造一条。
	_, roReaderTok := e.newPlayer(t, "roreader")
	roBattleID := newSettledBattle(t, e, roReaderTok)
	roReads := []string{
		"/api/v1/admin/dashboard",
		"/api/v1/admin/levels",
		"/api/v1/admin/levels/1/waves",
		"/api/v1/admin/skills",
		"/api/v1/admin/equipment",
		"/api/v1/admin/users",
		"/api/v1/admin/battles",
		fmt.Sprintf("/api/v1/admin/battles/%d", roBattleID),
		"/api/v1/admin/defenses",
		"/api/v1/admin/economy",
		"/api/v1/admin/announcements",
		"/api/v1/admin/redeem-codes",
		"/api/v1/admin/audit-logs",
	}
	for _, url := range roReads {
		status, body := e.get(t, url, roTok)
		if status != 200 {
			t.Errorf("readonly 读端点 %s 应 200，实际 %d %v", url, status, body)
		}
	}

	// --- ops：同一个写端点必须 200（证明没有过度拒绝，门禁只挡 readonly） ---
	status, body := e.post(t, fmt.Sprintf("/api/v1/admin/users/%d/unban", uid), opsTok, nil)
	if status != 200 {
		t.Errorf("ops 账号解封应 200（不被 requireWritable 误伤），实际 %d %v", status, body)
	}
	status, body = e.post(t, fmt.Sprintf("/api/v1/admin/users/%d/ban", uid), opsTok, map[string]any{})
	if status != 200 {
		t.Errorf("ops 账号封禁应 200，实际 %d %v", status, body)
	}
}
