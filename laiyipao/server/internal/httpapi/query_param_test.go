package httpapi

import (
	"encoding/json"
	"fmt"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"
)

// 查询参数解析失败**必须变成 400**，而不是静默退化成默认值（第 104 轮）。
//
// # 缺陷
//
// `adminBattles` 原来写的是：
//
//	userID, _ := strconv.ParseInt(c.Query("user_id", "0"), 10, 64)
//
// 而 service 层的过滤条件是：
//
//	($1 = 0 OR br.user_id = $1) AND ($2 = 0 OR br.level_id = $2)
//
// —— **0 是「不过滤」的哨兵值**。
//
// 于是 `?user_id=abc` → 解析失败 → 0 → 过滤条件恒真
// → **返回所有用户的战报**，而且返回 **200**。实测确认。
//
// # 后果不是「参数没生效」那么轻
//
// 运营在查一个疑似作弊的玩家时，若 user_id 打错，
// 看到的是**别人的**战报列表，
// 于是得出「这个人没有异常战报」的结论 ——
// 一个**会误导调查方向的静默错误答案**。
//
// 400 至少把「你的查询有问题」这件事说出来；
// 200 + 别人的数据则**制造了一个看起来可信的错误结论**。
//
// `level_id=xyz` 完全同型。
//
// # 为什么不在 service 层兜底
//
// 因为 service 层**无法区分**「调用方要的是不过滤」
// 与「调用方传了个坏值」—— 两者在 int64 里都是 0。
// 这个歧义**只有在 HTTP 边界才看得见**。

func TestMalformedQueryFilterIsRejectedNotSilentlyDropped(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "qfilter104")

	uidA, _, idsA := playerWithBattles(t, e, "qf-a", 2)
	uidB, _, idsB := playerWithBattles(t, e, "qf-b", 2)

	// 基线 1：不带过滤参数 → 200，且能看到**两个**玩家的战报
	statusAll, bodyAll := e.get(t, "/api/v1/admin/battles?limit=200", adminTok)
	if statusAll != 200 {
		t.Fatalf("不带过滤参数应 200，实际 %d：%v", statusAll, bodyAll)
	}
	seenAll := userIDsIn(bodyAll)
	if !seenAll[uidA] || !seenAll[uidB] {
		t.Fatalf("基线就错了：无过滤时应当同时看到 A(%d) 与 B(%d)，实际 %v",
			uidA, uidB, seenAll)
	}

	// 基线 2：合法 user_id → 200，且**只有**该玩家
	statusOK, bodyOK := e.get(t,
		fmt.Sprintf("/api/v1/admin/battles?limit=200&user_id=%d", uidA), adminTok)
	if statusOK != 200 {
		t.Fatalf("合法 user_id 应 200，实际 %d：%v", statusOK, bodyOK)
	}
	seenOK := userIDsIn(bodyOK)
	if !seenOK[uidA] {
		t.Errorf("合法 user_id 应能看到 A(%d)，实际 %v", uidA, seenOK)
	}
	if seenOK[uidB] {
		t.Errorf("合法 user_id=%d 的结果里**混进了** B(%d) 的战报 —— 过滤没生效", uidA, uidB)
	}
	_ = idsA
	_ = idsB

	// 核心断言：非法值必须 400，且**绝不能**返回 200 + 数据
	for _, q := range []string{
		"user_id=abc", "user_id=", "user_id=%20", "level_id=xyz", "level_id=1.5",
		"level_id=0x10",
	} {
		status, body := e.get(t, "/api/v1/admin/battles?limit=200&"+q, adminTok)
		if status == fiber200 {
			t.Errorf("?%s 返回了 **200** —— 非法值被静默当成了默认/哨兵值\n"+
				"返回体：%v\n\n"+
				"这是第 104 轮修掉的缺陷：解析失败落到 0（=不过滤），\n"+
				"于是运营看到的是**所有人的**战报，会得出「该玩家没有异常战报」的错误结论。",
				q, body)
			continue
		}
		if status != fiber400 {
			t.Errorf("?%s 应 400，实际 %d", q, status)
		}
	}

	// limit / offset 同类（它们原本也会被丢错，只是后果轻一些）
	for _, q := range []string{"limit=abc", "offset=xyz"} {
		if status, _ := e.get(t, "/api/v1/admin/users?"+q, adminTok); status != fiber400 {
			t.Errorf("admin/users ?%s 应 400，实际 %d", q, status)
		}
	}

	// 排行榜与诊断端点同样
	if status, _ := e.get(t, "/api/v1/leaderboard?limit=abc", adminTok); status != fiber400 {
		t.Errorf("leaderboard ?limit=abc 应 400，实际 %d", status)
	}
}

// userIDsIn 从战报列表响应里取出出现过的 user_id 集合。
func userIDsIn(body map[string]any) map[int64]bool {
	out := map[int64]bool{}
	raw, _ := body["items"].([]any)
	for _, it := range raw {
		if m, ok := it.(map[string]any); ok {
			out[int64(num(m, "user_id"))] = true
		}
	}
	return out
}

// 显式常量，避免在断言里混进裸数字（第 106 轮那次教训的形态：判据踩在分界线上）。
const (
	fiber200 = 200
	fiber400 = 400
)

// TestQueryIntDistinguishesMissingFromMalformed 覆盖 queryInt 助手自身的语义。
//
// 关键区分是四条，缺一条就退回第 104 轮那个缺陷：
//
//	（键不存在）  → def
//	?limit=       → ErrBadInput
//	?limit=abc    → ErrBadInput
//	?limit=%20%20 → ErrBadInput
func TestQueryIntDistinguishesMissingFromMalformed(t *testing.T) {
	cases := []struct {
		query   string
		want    int
		wantErr bool
		why     string
	}{
		{"", 7, false, "参数缺失 → 用默认值"},
		{"?k=", 0, true, "显式空串 → 必须报错（当缺失处理会打开与 abc 同样的洞）"},
		{"?k=%20", 0, true, "只有空白 → 同上"},
		{"?k=42", 42, false, "合法值"},
		{"?k=%2042%20", 42, false, "带空白的合法值应当被 trim 后接受"},
		{"?k=-3", -3, false, "负数是合法整数（是否可用由 service 层判断）"},
		{"?k=abc", 0, true, "非数字 → 必须报错，不能退化成默认值"},
		{"?k=1.5", 0, true, "小数不是合法整数 → 必须报错"},
		{"?k=0x10", 0, true, "十六进制不被 Atoi 接受 → 必须报错"},
		{"?k=999999999999999999999", 0, true, "溢出 → 必须报错"},
	}

	for _, tc := range cases {
		t.Run(tc.query, func(t *testing.T) {
			got, errMsg := queryIntVia(t, tc.query)
			if tc.wantErr {
				if errMsg == "" {
					t.Fatalf("%s：期望 ErrBadInput，实际得到 %d（无错误）\n"+
						"静默退化成默认值是第 104 轮那个缺陷 —— "+
						"解析失败落到 0，而 0 在过滤条件里是「不过滤」。", tc.why, got)
				}
				if !strings.Contains(errMsg, "查询参数 k=") {
					t.Errorf("错误信息应当点名是哪个参数坏了，实际：%s", errMsg)
				}
				return
			}
			if errMsg != "" {
				t.Fatalf("%s：不应报错，实际 %s", tc.why, errMsg)
			}
			if got != tc.want {
				t.Errorf("%s：期望 %d，实际 %d", tc.why, tc.want, got)
			}
		})
	}
}

// discardedQueryParse 匹配「把 c.Query 的解析错误丢掉」的写法。
//
// 两种形态都覆盖：
//
//	limit, _ := strconv.Atoi(c.Query("limit", "50"))   ← 二元赋值里丢第二个
//	_ = strconv.Atoi(c.Query("level_id"))              ← 单值直接丢
var discardedQueryParse = regexp.MustCompile(
	`(?m)(,\s*_\s*:?=|\b_\s*:?=)\s*strconv\.\w+\(\s*c\.(?:Query|Params)\(`)

// TestNoHandlerDiscardsQueryParseErrors 是**这一类**的源码守卫（第 104 轮）。
//
// # 为什么不能只按「已知的几处」列清单
//
// 枚举式断言在第 13 个端点加上同样的写法时就会失效。
// 本条扫的是**形状**：`c.Query(...)` 的结果有没有被 `strconv` 解析后丢错。
//
// # 为什么扫正则而不扫 AST
//
// 这个形状天然是「一条赋值语句里左边有个 `_`」，
// 而 AST 判据要同时处理 `:=` 与 `=`，还要判断右值是哪个函数 ——
// 对这一条来说正则更短，且**误报的形状已经被 testkit 剥注释排除**。
//
// ⚠️ 若将来这类写法以别的形状出现（比如先 `raw := c.Query(...)`
// 再 `strconv.Atoi(raw)` 丢错），本条会漏。
// 那时的修法是加 AST 判据，**不是**把清单改长。
func TestNoHandlerDiscardsQueryParseErrors(t *testing.T) {
	offenders, inspected, readFiles := scanDiscardedQueryParse(t)
	//
	// ⚠️ 变异实测：把扫描范围缩到只剩 `handlers_admin.go` →
	// 第一个版本（只数「检查过的 c.Query 出现次数 ≥ 8」）**全绿通过**，
	// 因为那一个文件自己就有 8 处以上。
	//
	// 所以计数下限是**弱**判据：它挡不住「恰好还有够多次数」的范围收缩。
	//
	// 换成比对**读过的文件集合** —— 变异跳过的那个文件压根没被读，
	// 集合大小立刻不等。这是与「列表长度」和「计数」都不同的第三条路：
	// 前两者都能被「跳过一个仍然够多的文件」骗过，集合比对不能。
	//
	// 同一个陷阱的第 6 次（第 91/99/101/102 轮与本轮前两次同型）。
	all := listHTTPSources(t)
	if len(readFiles) != len(all) {
		missing := map[string]bool{}
		for _, f := range all {
			missing[f] = true
		}
		for _, f := range readFiles {
			delete(missing, f)
		}
		var gone []string
		for f := range missing {
			gone = append(gone, f)
		}
		sort.Strings(gone)
		t.Fatalf("只读了 %d 个文件（%d 个在目录里），这些根本没被检查：%s\n"+
			"「扫得少」与「全部合规」在输出里一模一样。", len(readFiles), len(all),
			strings.Join(gone, ", "))
	}
	if inspected < 8 {
		t.Fatalf("只检查了 %d 处 c.Query / c.Params 调用，期望至少 8 处", inspected)
	}
	if len(offenders) > 0 {
		sort.Strings(offenders)
		t.Errorf("以下位置把 c.Query 的解析错误丢掉了：\n  %s\n\n"+
			"丢弃之后 strconv 返回的是**零值**，而零值在过滤条件里往往就是\n"+
			"「不过滤」的哨兵值 —— 于是 `?user_id=abc` 返回 200 + **所有人的**数据。\n\n"+
			"用 `queryInt(c, \"名字\", 默认值)`：\n"+
			"  - 参数缺失 / 空串 / 只有空白 → 用默认值（前端拼 URL 常见）\n"+
			"  - 存在但非法 → ErrBadInput → 400，且错误信息点名参数\n\n"+
			"（已检查 %d 处 c.Query 调用）",
			strings.Join(offenders, "\n  "), inspected)
	}
}

// TestDiscardedQueryParseScannerSeesKnownShape 是扫描器**自身**的守卫。
//
// ⚠️ 与第 78/82/99/101/102/103 轮同一条理由：
// 「扫不到就 Fatal」的守卫在真的没东西可扫时会永远红，于是被人删掉。
// 「扫描器是否可用」与「代码是否合规」必须分开回答。
func TestDiscardedQueryParseScannerSeesKnownShape(t *testing.T) {
	bad := []string{
		`limit, _ := strconv.Atoi(c.Query("limit", "50"))`,
		`userID, _ := strconv.ParseInt(c.Query("user_id", "0"), 10, 64)`,
		`_ = strconv.Atoi(c.Query("level_id"))`,
		`x, _ := strconv.ParseInt(c.Params("id"), 10, 64)`,
	}
	for _, src := range bad {
		if !discardedQueryParse.MatchString(src) {
			t.Errorf("扫描器漏掉了这种形状：%s", src)
		}
	}

	good := []string{
		`limit, err := queryInt(c, "limit", 50)`,
		`id, err := strconv.Atoi(c.Params("id"))`,
		`kind := c.Query("type", "power")`,
		`v := intOrZero(c.Query("x"))`,
	}
	for _, src := range good {
		if discardedQueryParse.MatchString(src) {
			t.Errorf("扫描器误报了合规写法：%s", src)
		}
	}

	// 注释里出现同样文本**不得**被判违规 —— 那是第 67/83/88/94/95 轮踩过的坑。
	commented := "// 原来是 limit, _ := strconv.Atoi(c.Query(\"limit\", \"50\"))"
	if discardedQueryParse.MatchString(stripHTTPComments(commented)) {
		t.Error("剥注释后仍被判违规 —— 扫描撞到了注释里的示例代码")
	}
}

// scanDiscardedQueryParse 扫本包全部非测试源码，返回违规位置与检查过的 c.Query 数。
func scanDiscardedQueryParse(t *testing.T) (offenders []string, inspected int, readFiles []string) {
	t.Helper()
	files := listHTTPSources(t)
	//
	// ⚠️ 本包只有 3 个非测试源文件（middleware / routes / handlers_admin），
	// 所以下限就是 3。再加文件不会红，而**减**文件会 ——
	// 那说明有人重构掉了某个文件，值得看一眼。
	if len(files) < 3 {
		t.Fatalf("只列出 %d 个源文件，期望至少 3 个", len(files))
	}
	for _, name := range files {
		code := stripHTTPComments(readHTTPFile(t, name))
		readFiles = append(readFiles, name)
		for i, ln := range strings.Split(code, "\n") {
			if strings.Contains(ln, "c.Query(") || strings.Contains(ln, "c.Params(") {
				inspected++
			}
			if discardedQueryParse.MatchString(ln) {
				offenders = append(offenders,
					name+":"+strconv.Itoa(i+1)+": "+strings.TrimSpace(ln))
			}
		}
	}
	//
	// ⚠️ 「inspected = 0」与「全部合规」在输出里一模一样。
	// 变异「把扫描范围缩到一个已知文件」是**全绿**的 ——
	// 这是同一个陷阱的第 6 次（第 91/99/101/102 轮与本轮内两次同型）。
	// 所以下限必须数**真正逐行看过**的 c.Query / c.Params 调用，
	// 而且 caller 那条 Fatal 也留着同样的下限，两处都数。
	return offenders, inspected, readFiles
}

// listHTTPSources 列出本包参与编译的源文件（**排除 _test.go**）。
func listHTTPSources(t *testing.T) []string {
	t.Helper()
	entries, err := os.ReadDir(".")
	if err != nil {
		t.Fatalf("读包目录失败：%v", err)
	}
	var out []string
	for _, e := range entries {
		n := e.Name()
		if e.IsDir() || !strings.HasSuffix(n, ".go") || strings.HasSuffix(n, "_test.go") {
			continue
		}
		out = append(out, n)
	}
	sort.Strings(out)
	return out
}

func readHTTPFile(t *testing.T, name string) string {
	t.Helper()
	b, err := os.ReadFile(name)
	if err != nil {
		t.Fatalf("读 %s 失败：%v", name, err)
	}
	return string(b)
}

// stripHTTPComments 去掉 Go 注释，保留行数（每行输出一个空串或代码）。
//
// ⚠️ 与 testkit 里的 TS 版同源问题：文本扫描撞注释。
// 第 67/83/88/94/95 轮已经因此踩坑 5 次。
func stripHTTPComments(src string) string {
	var out []string
	for _, line := range strings.Split(src, "\n") {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "//") {
			out = append(out, "")
			continue
		}
		if i := strings.Index(line, "//"); i >= 0 {
			line = line[:i]
		}
		out = append(out, line)
	}
	return strings.Join(out, "\n")
}

// queryIntVia 跑一次真实请求，返回 queryInt 的 (值, 错误信息)。
//
// ⚠️ 为什么用真的 fiber app 而不是手搓一个 `*fiber.Ctx`：
// `c.Query()` 读的是 fasthttp 的 URI 状态，手搓 ctx 需要
// `acquireCtx()` 之类的内部 API，那是 fasthttp 的实现细节。
// 而**判据踩在自己的测试装置上**正是本项目最贵的一类错误
// （第 88 轮把 statusDurationMs 写进判据、第 95 轮复制判据副本）。
//
// 走真实 app 还有个额外好处：`?k=%20` 这种空白、
// `%20` 编码、缺参数时的默认值，全都按真实解码路径走一遍。
func queryIntVia(t *testing.T, query string) (int, string) {
	t.Helper()
	app := fiber.New()
	app.Get("/q", func(c *fiber.Ctx) error {
		v, err := queryInt(c, "k", 7)
		if err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"err": err.Error()})
		}
		return c.JSON(fiber.Map{"v": v})
	})
	resp, err := app.Test(get("/q"+query), 5000)
	if err != nil {
		t.Fatalf("请求 %s 执行失败：%v", "/q"+query, err)
	}
	defer resp.Body.Close()
	var body map[string]any
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Fatalf("解析响应失败（status=%d）：%v", resp.StatusCode, err)
	}
	if msg, ok := body["err"].(string); ok {
		return 0, msg
	}
	f, ok := body["v"].(float64)
	if !ok {
		t.Fatalf("响应里既没有 err 也没有可解析的 v：%v", body)
	}
	return int(f), ""
}
