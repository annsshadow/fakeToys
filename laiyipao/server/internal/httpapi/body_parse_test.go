package httpapi

import (
	"context"
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
	"testing"
)

// 请求体解析失败**不得让破坏性动作照常执行**（第 105 轮）。
//
// ## 缺陷
//
// ```go
// func (s *Server) adminBanUser(c *fiber.Ctx) error {
//     var body struct{ Reason string `json:"reason"` }
//     _ = c.BodyParser(&body)          // ← 错误被丢弃
//     if body.Reason == "" { body.Reason = "违反用户协议" }
//     u, err := s.Svc.AdminSetUserStatus(c.Context(), id, true, body.Reason)
// ```
//
// ## 实测
//
// ```
// 合法        status=200 ban_reason="测试理由"
// 截断的 JSON  status=200 ban_reason="违反用户协议"   ← 修前：已封禁
// ```
//
// `POST /admin/users/5/ban` 带 `{bad` → **200**，用户**真的被封禁**，
// 而且 `ban_reason` 落库成「违反用户协议」—— 运营从未提交过的理由。
//
// ## 为什么比第 104 轮那两条查询参数更重
//
//  1. **它是破坏性动作。** 前者返回错数据，这里改用户状态。
//     （有 unban，但那需要有人先意识到出错了。）
//  2. **审计轨迹被污染。** 事后看 `admin_audit_logs`，
//     「违反用户协议」像是运营深思熟虑后的判断，
//     而它只是一个解析失败的默认值。
//  3. **还吊销了全部 refresh token**（`AdminSetUserStatus` 里），
//     所以玩家的长期会话也一并失效。
//  4. **触发条件极易达到** —— 运营脚本 / 代理截断 / 复制粘贴漏字符。
//
// ## 为什么不能简单地把错误一律拒掉
//
// 「就封他，理由按默认的」是**合法**调用，
// 而 `BodyParser` 对**空 body** 返回 EOF 类错误。
//
// 所以判据必须是「body 非空且解析失败」→ 400，
// 而不是「BodyParser 返回错误」→ 400。
//
// 这与第 104 轮 `queryInt` 的「键不存在」vs「值为空」同源：
// **两种输入两种含义，不能压成同一个信号。**

const (
	statusActive = 1
	statusBanned = 2
)

func TestMalformedBanBodyMustNotBan(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "ban105")

	// 核心断言：**用户状态没被改**
	uid, _ := e.newPlayer(t, "ban-bad")
	//
	// ⚠️ 基线必须在动作**之前**取。
	// 我第一版直接写死「期望 0 个有效 token」，实测发现有 1 个 ——
	// `newPlayer` 会建一个 refresh token。而判据踩在
	// 「造测试数据时恰好有几个」这种**分界线输入**上，
	// 是第 88 轮之后反复出现的一类错误。
	//
	// 正确形态：**先量基线，再断言「没变」/「变成 0」**。
	tokensBefore := e.liveRefreshTokens(t, uid)
	status, body := e.post(t, fmt.Sprintf("/api/v1/admin/users/%d/ban", uid), adminTok, "{bad")
	if status != 400 {
		t.Fatalf("截断的 JSON 应 400，实际 %d：%v", status, body)
	}
	if got := e.userStatus(t, uid); got != statusActive {
		t.Errorf("用户状态被改成了 %d —— **已经封禁了**。\n"+
			"一个解析失败的请求体执行了破坏性动作；\n"+
			"而且 AdminSetUserStatus 里还吊销了全部 refresh token。", got)
	}
	if reason := e.banReason(t, uid); reason != "" {
		t.Errorf("ban_reason 落库成 %q —— 那是解析失败后的默认理由，"+
			"运营从未提交过它，而审计轨迹会把它当成真实判断", reason)
	}
	if n := e.liveRefreshTokens(t, uid); n != tokensBefore {
		t.Errorf("有效 refresh token 从 %d 变成 %d —— 动作没发生就不该有任何变化。"+
			"（封禁成功时它会是 0，因为 AdminSetUserStatus 会吊销全部令牌）",
			tokensBefore, n)
	}
	if n := e.auditCount(t, "ban_user", uid); n != 0 {
		t.Errorf("写入了 %d 条 ban_user 审计 —— 动作没发生就不该有审计记录", n)
	}

	// 「body 存在但不是 JSON」也要拒（比截断更隐蔽）
	for _, bad := range []string{`{bad`, `{"reason": }`, `not json at all`, `[]`} {
		uid2, _ := e.newPlayer(t, "ban-bad2")
		if st, _ := e.post(t, fmt.Sprintf("/api/v1/admin/users/%d/ban", uid2), adminTok, bad); st != 400 {
			t.Errorf("body=%q 应 400，实际 %d", bad, st)
		}
		if got := e.userStatus(t, uid2); got != statusActive {
			t.Errorf("body=%q 时用户状态变成 %d —— 不该封禁", bad, got)
		}
	}
}

// 合法调用一个都不能被误伤：这是上面那个修复的另一半。
func TestBanAcceptsMissingAndEmptyBody(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "ban105ok")

	cases := []struct {
		name       string
		body       any
		wantReason string
	}{
		{"完全不传 body", nil, "违反用户协议"},
		{"空对象", map[string]any{}, "违反用户协议"},
		{"显式空字符串理由", map[string]any{"reason": ""}, "违反用户协议"},
		{"正常理由", map[string]any{"reason": "开挂"}, "开挂"},
		{"理由带空格", map[string]any{"reason": " 刷数据  "}, " 刷数据  "},
	}
	for _, c := range cases {
		uid, _ := e.newPlayer(t, "banok")
		tokensBefore := e.liveRefreshTokens(t, uid)
		st, body := e.post(t, fmt.Sprintf("/api/v1/admin/users/%d/ban", uid), adminTok, c.body)
		if st != 200 {
			t.Errorf("%s：应 200，实际 %d：%v\n"+
				"「没 body」是**合法**调用 —— 判据若写成「BodyParser 报错就 400」，"+
				"就会把这条正常的封禁流程一起打死", c.name, st, body)
			continue
		}
		if got := e.userStatus(t, uid); got != statusBanned {
			t.Errorf("%s：状态应为 %d，实际 %d", c.name, statusBanned, got)
		}
		if got := e.banReason(t, uid); got != c.wantReason {
			t.Errorf("%s：ban_reason 期望 %q，实际 %q", c.name, c.wantReason, got)
		}
		// 封禁成功后必须**一个都不剩**（不是「比原来少」——
		// 那样写的话测试数据里本来就没有 token 时会假绿）。
		if n := e.liveRefreshTokens(t, uid); n != 0 {
			t.Errorf("%s：封禁应吊销**全部** refresh token（基线 %d），实际还有 %d 个有效",
				c.name, tokensBefore, n)
		}
	}
}

// adminVerifyBattle 那处原本也是丢弃错误，只是靠下游的空值检查**侥幸**安全。
// 这里把「侥幸」钉成显式契约。
func TestMalformedVerifyBodyMustNotWriteVerification(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "verify105")
	uid, tok := e.newPlayer(t, "vfy")
	bid := newSettledBattle(t, e, tok)

	st, body := e.post(t, fmt.Sprintf("/api/v1/admin/battles/%d/verify", bid), adminTok, "{bad")
	if st != 400 {
		t.Fatalf("截断的 JSON 应 400，实际 %d：%v", st, body)
	}
	var n int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT COUNT(*) FROM replay_verifications WHERE battle_id = $1`, bid).Scan(&n); err != nil {
		t.Fatalf("统计验真记录失败：%v", err)
	}
	if n != 0 {
		t.Errorf("写入了 %d 条验真记录 —— 动作没发生", n)
	}
	_ = uid
}

// --- 读库小工具 ---

func (e *e2e) userStatus(t *testing.T, uid int64) int {
	t.Helper()
	var n int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT status FROM users WHERE id = $1`, uid).Scan(&n); err != nil {
		t.Fatalf("读用户状态失败：%v", err)
	}
	return n
}

func (e *e2e) banReason(t *testing.T, uid int64) string {
	t.Helper()
	var s string
	if err := e.pool.QueryRow(context.Background(),
		`SELECT ban_reason FROM users WHERE id = $1`, uid).Scan(&s); err != nil {
		t.Fatalf("读 ban_reason 失败：%v", err)
	}
	return s
}

func (e *e2e) liveRefreshTokens(t *testing.T, uid int64) int {
	t.Helper()
	var n int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL`, uid).Scan(&n); err != nil {
		t.Fatalf("读 refresh token 失败：%v", err)
	}
	return n
}

func (e *e2e) auditCount(t *testing.T, action string, uid int64) int {
	t.Helper()
	var n int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT COUNT(*) FROM admin_audit_logs WHERE action = $1 AND target = $2`,
		action, strconv.FormatInt(uid, 10)).Scan(&n); err != nil {
		t.Fatalf("读审计失败：%v", err)
	}
	return n
}

// ---------------------------------------------------------------------------
// 这一类的源码守卫
// ---------------------------------------------------------------------------

// TestNoHandlerDiscardsBodyParseError 是**这一类**的守卫（第 105 轮）。
//
// # 为什么用 AST 而不是正则
//
// 判据是「`c.BodyParser(...)` 的返回值有没有被检查」，
// 这是个结构问题：`_ = f(x)` 与 `x, err := f(y)` 的差别在 AST 里一目了然，
// 用正则要靠 `_ =` 这个前缀去猜 —— 而 `_ = f(x)` 也是**合法**写法
// （「我明确不要这个错误」），正则分不出「丢弃」与「明示忽略」。
//
// # 豁免表必须写理由
//
// 与第 101 轮同一条：豁免不留白。
// 现在豁免表是空的 —— 那正是应有的状态：
// 「这一类问题在本包里已经清零」是可陈述的事实。
var bodyParseExemptions = map[string]string{}

func TestNoHandlerDiscardsBodyParseError(t *testing.T) {
	offenders, parsed, files := scanDiscardedBodyParse(t)
	//
	// ⚠️ 本包只有 3 个非测试源文件（middleware / routes / handlers_admin）。
	// 下限写错成 15 时它会永远红 —— 那是「第 78/82/99/101/102/103 轮」
	// 记的那条：「扫不到就 Fatal」的守卫会被删掉，于是保护的东西裸奔。
	// 所以下限必须等于**真实**文件数，且与集合比对分开回答两个问题：
	//
	//	「文件够不够」 → 这个下限
	//	「有没有全看过」 → 下面那句集合比对
	if parsed < 3 {
		t.Fatalf("只解析了 %d 个文件，期望至少 3 个（读过的：%v）", parsed, files)
	}
	// 与第 104 轮同一条：「扫得少」与「全部合规」在输出里一模一样。
	// 比对**读过的文件集合**，而不是列表长度。
	all := listHTTPSources(t)
	if len(files) != len(all) {
		t.Fatalf("只检查了 %d 个文件（目录里有 %d 个）：%v", len(files), len(all), files)
	}
	if len(offenders) > 0 {
		sort.Strings(offenders)
		t.Errorf("以下位置丢弃了 c.BodyParser 的错误：\n  %s\n\n"+
			"第 105 轮实测：封禁接口收到截断的 JSON 仍然返回 200 并**真的封禁了用户**，\n"+
			"还把解析失败的默认理由写进了 ban_reason 与审计日志，并吊销了全部 refresh token。\n\n"+
			"正确写法（注意先判长度，「没 body」是合法调用）：\n"+
			"    if len(bytes.TrimSpace(c.Body())) > 0 {\n"+
			"        if err := c.BodyParser(&body); err != nil {\n"+
			"            return fail(c, fiber.StatusBadRequest, \"bad_json\", \"请求体不是合法 JSON\")\n"+
			"        }\n"+
			"    }\n\n"+
			"若确实要显式忽略，请写进豁免表并说明理由（当前豁免 %d 条）。",
			strings.Join(offenders, "\n  "), len(bodyParseExemptions))
	}
	_ = bodyParseExemptions
}

// TestBodyParseScannerSeesKnownShape 是扫描器**自身**的守卫。
func TestBodyParseScannerSeesKnownShape(t *testing.T) {
	//
	// ⚠️ 合成源必须是**能编译的** Go。
	// 我第一版写了 `type ctx struct{ Body() []byte }` —— struct 不能有方法，
	// 解析直接报错，于是自测测的是「解析器会不会报错」而不是判据本身。
	// **守卫的守卫坏了却看不出来**，比没有自测更危险。
	src := `package p

type fakeCtx struct{}

func (c *fakeCtx) Body() []byte             { return nil }
func (c *fakeCtx) BodyParser(out any) error { return nil }

type payload struct{ in any }

// 检查了错误：if-init 形态
func checked(c *fakeCtx, b *payload) error {
	if err := c.BodyParser(&b.in); err != nil {
		return err
	}
	return nil
}

// 显式丢弃：_ = 形态
func discarded(c *fakeCtx, b *payload) error {
	_ = c.BodyParser(&b.in)
	return nil
}

// 先判长度再解析：两处都有，只有一处检查 —— 不得被误判为违规
func guarded(c *fakeCtx, b *payload) error {
	if len(c.Body()) > 0 {
		if err := c.BodyParser(&b.in); err != nil {
			return err
		}
	}
	return nil
}

// 一个函数里既有检查又有丢弃：丢弃的那次调用**仍然是缺陷**
func mixed(c *fakeCtx, b *payload) error {
	if err := c.BodyParser(&b.in); err != nil {
		return err
	}
	_ = c.BodyParser(&b.in)
	return nil
}

// 完全没有 BodyParser：不相关
func unrelated(c *fakeCtx) error { return nil }
`
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, "synthetic.go", src, 0)
	if err != nil {
		t.Fatalf("合成源解析失败：%v\n%s", err, src)
	}
	got := discardedBodyParseFuncs(f, fset)
	sort.Strings(got)
	want := []string{"discarded", "mixed"}
	if len(got) != len(want) {
		t.Fatalf("应当命中 %v，实得 %v", want, got)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("应当命中 %v，实得 %v", want, got)
		}
	}
	for _, n := range got {
		if n == "checked" || n == "guarded" || n == "unrelated" {
			t.Errorf("误报 %s —— 那三段都是合规写法", n)
		}
	}
	//
	// ⚠️ `mixed` 这个用例是**变异实测逼出来的**：
	// 我第一版的规则是 `len(discarded) > len(checked)`（混合即安全），
	// 而合成源里没有任何「既有检查又有丢弃」的函数，
	// 于是那条错误规则在自测里照样通过 —— **自测覆盖不到自己以为覆盖的地方**。
	//
	// 判据要能发现「一个函数里混着两种形态」，
	// 合成源就必须**恰好包含**那个形态。
	// 这与第 91 轮「守卫要覆盖这一类而不是我已知的几处」同源。
}

// discardedBodyParseFuncs 返回「调用了 c.BodyParser 但丢弃了返回错误」的函数名。
//
// # 判据
//
// 一次 `c.BodyParser(...)` 调用若落在下列任一区间里，算「检查了错误」：
//
//	if err := c.BodyParser(&x); err != nil { … }   ← IfStmt.Init
//	_, err := c.BodyParser(&x)                      ← 普通赋值（错误名随意）
//
// 若落在「左值全为 `_`」的赋值区间里，算**显式丢弃**。
//
// # 「一个函数里既有丢弃又有检查」必须**算违规**
//
// 变异实测：我在 `adminBanUser` 里加一处 `_ = c.BodyParser(&body)`
// （同时保留原本检查过的那一处），第一版守卫**全绿通过** ——
// 因为我写的是 `len(discarded) > len(checked)`。
//
// 那个规则是我想当然编的，理由是「检查过的那条路径是安全的」。
// **判据必须逐个调用点判断**：那次被丢弃的调用照样会执行、照样会失败、
// 照样让破坏性动作照常发生。函数里有别的检查救不了它。
//
// ⚠️ 用**位置区间**而不是「父节点是谁」：
// 条件里的 `err != nil`、`else if err != nil`、`return err` 等形态太多，
// 逐个枚举父节点会漏。区间法只要错误值出现在语句里就算「处理过」。
func discardedBodyParseFuncs(f *ast.File, fset *token.FileSet) []string {
	var out []string
	for _, decl := range f.Decls {
		fn, ok := decl.(*ast.FuncDecl)
		if !ok || fn.Body == nil {
			continue
		}
		var checked, discarded [][2]token.Pos
		ast.Inspect(fn.Body, func(n ast.Node) bool {
			switch st := n.(type) {
			case *ast.AssignStmt:
				if !anyBodyParserIn(exprsToNodes(st.Rhs)) {
					return true
				}
				rng := [2]token.Pos{st.Pos(), st.End()}
				if hasBlankLhs(st) {
					discarded = append(discarded, rng)
				} else {
					checked = append(checked, rng)
				}
			case *ast.IfStmt:
				if st.Init != nil && anyBodyParserIn([]ast.Node{st.Init}) {
					checked = append(checked, [2]token.Pos{st.Init.Pos(), st.Init.End()})
				}
			}
			return true
		})
		_ = fset
		_ = checked
		if len(discarded) > 0 {
			out = append(out, fn.Name.Name)
		}
	}
	return out
}

// anyBodyParserIn 判断这些节点里（递归）有没有 c.BodyParser 调用。
//
// ⚠️ 参数是 `[]ast.Node` 而不是 `[]ast.Expr`：
// `IfStmt.Init` 是 **Stmt**（`if err := …; err != nil` 里它是 AssignStmt），
// 强转成 Expr 会得到 nil，而 `ast.Inspect(nil, …)` **直接 panic**。
// 我第一版就是这么写的，变异跑到那条分支就崩了 —— 崩掉比红更难查。
func anyBodyParserIn(nodes []ast.Node) bool {
	found := false
	for _, e := range nodes {
		if e == nil {
			continue
		}
		ast.Inspect(e, func(n ast.Node) bool {
			call, ok := n.(*ast.CallExpr)
			if !ok {
				return true
			}
			sel, ok := call.Fun.(*ast.SelectorExpr)
			if ok && sel.Sel.Name == "BodyParser" {
				found = true
			}
			return true
		})
	}
	return found
}

func hasBlankLhs(st *ast.AssignStmt) bool {
	for _, lhs := range st.Lhs {
		id, ok := lhs.(*ast.Ident)
		if !ok || id.Name != "_" {
			return false
		}
	}
	return len(st.Lhs) > 0
}

func scanDiscardedBodyParse(t *testing.T) (offenders []string, parsed int, files []string) {
	t.Helper()
	fset := token.NewFileSet()
	for _, name := range listHTTPSources(t) {
		f, err := parser.ParseFile(fset, filepath.Join(".", name), nil, 0)
		if err != nil {
			t.Fatalf("解析 %s 失败：%v", name, err)
		}
		parsed++
		files = append(files, name)
		for _, fn := range discardedBodyParseFuncs(f, fset) {
			offenders = append(offenders, name+": "+fn)
		}
	}
	return offenders, parsed, files
}

// exprsToNodes 把 []ast.Expr 转成 []ast.Node。
//
// Go 里 []ast.Expr 不能直接当 []ast.Node 用（切片类型不同），
// 但 `anyBodyParserIn` 只需要遍历，所以统一收 Node 更省事。
func exprsToNodes(exprs []ast.Expr) []ast.Node {
	out := make([]ast.Node, 0, len(exprs))
	for _, e := range exprs {
		out = append(out, e)
	}
	return out
}
