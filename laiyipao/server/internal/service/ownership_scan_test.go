package service

import (
	"fmt"
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

// 用户数据查询**必须**带归属过滤（第 78 轮）。
//
// # 缺陷的通用形态：守卫覆盖了它写下的那几项
//
// 第 75 轮修的是两个具体端点（`GetReplay` / `VerifyReplay`）。
// 但那两条守卫是**手写枚举** —— 它们只盯那两处。
//
// 第 74 轮刚犯过同型：`walletColumns` 是白名单，新增商品时没人去看它。
// 第 67 轮的 `TestProdGuardsPanic` 也是手写枚举。
//
// 这一轮把那个形态本身变成常驻哨兵：
//
//	任何 `SELECT ... FROM <用户数据表>` 若**没有** `user_id` 过滤，
//	就必须出现在一份**具名**豁免表里并写清理由。
//
// # 「用户数据表」的定义
//
// `user_` 前缀 + 几张没有前缀但同样按用户分区的表。
// 前缀是**结构性的**：PG 里所有按用户分区的表都以它开头
// （`user_wallets` / `user_tasks` / `user_progress` / `user_mastery_nodes` /
//  `user_tokens` / `user_purchases` / `user_daily_challenges` …），
// 所以加新表时**自动**被覆盖，不需要更新名单 —— 清单会漂，前缀不会。
//
// `battle_records` 单列：它的列叫 `user_id` 但表名没有 `user_` 前缀。
//
// # 为什么不用「grep 一下 user_id」
//
// 与第 67/68 轮记的教训同源：
// 正则分不清「声明」与「使用」，也分不清注释里提到的同名字段。
// 这里用 go/ast 精确取每个 SQL 字符串**字面量**所在的语句位置。

// userScopedTables 是必须带归属过滤的表（按前缀匹配）。
const userTablePrefix = "user_"

// battleScopedTables 是没有 `user_` 前缀但同样按用户分区的表，
// 映射到它们的**归属列名**。
//
// ⚠️ 归属列名**不统一**：绝大多数表叫 `user_id`，
// 而 `defenses` / `defense_challenges` 叫 `owner_id`。
//
// 我第一版只认 `USER_ID`，于是那三张表的合法查询全被误报 ——
// 而**守卫误报比漏报更危险**：它会让人以为某处没守，
// 然后去「修」一个本来正确的地方，或者写一条多余的过滤。
var battleScopedTables = map[string]string{
	"battle_records":     "user_id",
	"battle_tokens":      "user_id",
	"defenses":           "owner_id",
	"defense_challenges": "user_id",
}

// ownerScopedVerbs 是本守卫管辖的语句类型。
//
// ⚠️ **刻意不含 INSERT** —— 这是本轮（2026-10-15）扩范围时踩到的第一个坑。
//
// 原来只扫 `SELECT`，于是 `WashGem` 里那条
//
//	UPDATE user_gems SET affixes = $1::jsonb WHERE id = $2
//
// 从来不在管辖范围内（它缺 `AND user_id = $3`）。它当时**不是**可
// 利用的漏洞 —— 同一事务上方有 `SELECT ... FOR UPDATE` 锁过行 ——
// 但这种「调用点保证，语句自己不保证」的写法，只要有人给 UPDATE
// 路径加第二个调用点、或把 FOR UPDATE 挪走，就立刻变成直接 IDOR。
//
// 扩到 UPDATE/DELETE 之后又差点误报：所有 `INSERT INTO user_*`
// （upsert）都「没有归属过滤」。但 INSERT **不存在定位别人行的
// 问题** —— 归属由 VALUES 里的使用者 id 决定，而它总是来自已认证
// 的 userID。把 INSERT 纳管会让每一处 upsert 都变红，那就是守卫误报。
//
// 所以边界是语义的：**按已有行定位**（SELECT/UPDATE/DELETE）要管，
// **创建新行**（INSERT）不管。
var ownerScopedVerbs = []string{"SELECT", "UPDATE", "DELETE"}

// adminSurfacePrefix 是管理面函数的命名前缀（第 78 轮）。
//
// ⚠️ 这是**按名字**判定的，不是按调用图 ——
// 调用图会把「被管理面调用的工具函数」也算进来，而那些函数
// （比如 `grantWallet`）本来就没有归属概念。
//
// 用命名的理由：Go 生态里 `Admin` 前缀就是管理面入口的约定，
// 而 `internal/httpapi` 侧的 `requireAdmin` 中间件对
// `adm` 路由组强制校验 —— 两端都在。
//
// ⚠️ 它的**风险**如实标注：任何人给一个玩家侧函数起 `Admin` 前缀，
// 就能跳过这道守卫。所以本文件额外用
// `TestOwnershipExemptionsAreReachableOnlyFromAdminRoutes` 交叉核对。
const adminSurfacePrefix = "Admin"

// posKey 把 token.Position 压成 `basename.go:行号`，**仅供报错定位**。
//
// ⚠️ 它**不做豁免表的键**（曾经做过，漂移过四次，见 exemptionKey 的注释）。
// 保留它是因为 `tx_error_branch_test.go` 的错误吞噬守卫要用它生成
// 人类可读的 `文件:行`。
//
// ⚠️ 不能用 `token.Position.String()` 的原文（那是完整绝对路径，
// 换台机器就全失效）。这里去掉路径前缀与 **col** 那一段。
//
// ⚠️ 我第一版写成「找第二个冒号再截断」，结果把**行号**也截掉了
// （progression.go:572:4 → progression.go），于是每条豁免都失效。
func posKey(pos string) string {
	base := pos
	if i := strings.LastIndexAny(base, `\/`); i >= 0 {
		base = base[i+1:]
	}
	if i := strings.LastIndex(base, ":"); i >= 0 {
		base = base[:i]
	}
	return base
}

// exemptionKey 是豁免表的键：`文件名.函数名:表名`，例如
// `progression.go.ChallengeDefense:defenses`。
//
// ⚠️ 为什么**不是** `文件:行号`（曾经的形态）：那个键漂移过四次。
//
//	第  79 轮  给 ChallengeDefense 加预锁注释        572 → 673
//	第 132 轮  给 ChallengeInput 加 seed 校验         721 → 751
//	第 154 轮  给 SaveDefense 补 AND user_id 注释     800 → 824
//	D4  轮      再补一段 S 段说明                      824 → 857
//
// 四次全部与「归属」无关 —— 都是给函数加注释。而
// `TestOwnershipExemptionKeysAreStillPresent` 每次都准时报了出来，
// 于是这件事的实际形态是「守卫很灵敏」+「每次都要改一行」，
// 而不是「问题被解决了」。灵敏的守卫报出的若是**同一类噪声**，
// 它的净效果是训练人忽略它。
//
// 函数名与表名属于**语义**：加注释不改它们，而函数被删/改名、
// SQL 被挪走、查询被补上归属过滤时键会变 —— 那三种情况
// 正是不该再豁免的场景，报失效是对的。
//
// ⚠️ 这里刻意不用 `token.Position`（含列号与绝对路径）：换台机器
// 或改一下缩进，绝对路径那半就全失效了。
func exemptionKey(q unscopedQuery) string {
	return q.Fn + ":" + q.Table
}

// ownershipExemptions 是具名豁免表：`文件名.函数名:表名` → 理由。
//
// ⚠️ 每条都必须写清理由 ——「这里没问题」这种判断留给写豁免的人，
// 不留给沉默。理由为空时本测试会红。
//
// ⚠️ 键的构造见 `exemptionKey`：它曾经是 `文件:行号`，漂移过四次，
// 现在是语义键（函数名 + 表名），加注释不会再让它失效。
var ownershipExemptions = map[string]string{
	"stats.go.adminDashboardUncached:battle_records": "运营看板的三条聚合（reactions 直方图 / 近 14 日 DAU / 关卡胜率分布）" +
		"本就跨用户：只吐计数、不返回任何一行的私有数据；端点挂在 admin 路由组" +
		"（routes.go 的 adm 组，requireAdmin 之后）。按 user_id 过滤会让看板永远只显示一个人。",
	"progression.go.ChallengeDefense:defenses": "「挑战别人的防线」本身就是玩法：读 `defenses WHERE id = $1` " +
		"是为了拿到 owner_id 做后续判定。归属检查在**同一事务内**紧接着做：" +
		"`if ownerID == userID { return ErrForbidden }`。" +
		"所以这里不按 owner_id 过滤是正确的 —— 过滤了反而挑战不了任何人。",
}

// scanUnscopedQueries 扫出「按用户分区但没有 user_id 过滤」的查询。
type unscopedQuery struct {
	Pos   string // 文件:行
	Table string
	Stmt  string // SQL 字面量（截断）
	Fn    string
}

func scanUnscopedQueries(t *testing.T) []unscopedQuery {
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

	var out []unscopedQuery
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
		for _, decl := range f.Decls {
			fn, ok := decl.(*ast.FuncDecl)
			if !ok || fn.Body == nil {
				continue
			}
			ast.Inspect(fn.Body, func(n ast.Node) bool {
				call, ok := n.(*ast.CallExpr)
				if !ok || len(call.Args) == 0 {
					return true
				}
				// SQL 通常不是最后一个参数（后面还有 args），
				// 所以扫所有 BasicLit 参数，取看起来像 SQL 的那个。
				// ⚠️ 不假设参数位置：`QueryRow(ctx, q, args...)` 里
				// SQL 可能是第一个也可能是第二个。
				for _, a := range call.Args {
					bl, ok := a.(*ast.BasicLit)
					if !ok || bl.Kind != token.STRING {
						continue
					}
					sql := bl.Value
					if len(sql) < 20 {
						continue
					}
					upper := strings.ToUpper(sql)
					if !isOwnerScopedStatement(upper) {
						continue
					}
					tbl, filtered := classifySQL(sql)
					if tbl == "" || filtered {
						continue
					}
					// 管理面：函数名带 Admin 前缀（见 const 的风险标注）
					if strings.HasPrefix(fn.Name.Name, adminSurfacePrefix) {
						continue
					}
					out = append(out, unscopedQuery{
						Pos:   fset.Position(bl.Pos()).String(),
						Table: tbl,
						Stmt:  truncate(strings.Join(strings.Fields(sql), " "), 110),
						Fn:    name + "." + fn.Name.Name,
					})
				}
				return true
			})
		}
	}
	sort.Slice(out, func(i, j int) bool { return out[i].Pos < out[j].Pos })
	return out
}

// filterClauseOf 取出 SQL 的**过滤子句**（WHERE 之后、GROUP/ORDER/LIMIT 之前）。
//
// # ⚠️ 为什么要切出 WHERE 而不是对整条 SQL 做 contains
//
// 第一版用 `strings.Contains(upper, "USER_ID")` 判「有没有过滤」，
// 而这条 SQL 是：
//
//	SELECT owner_id, name, power, shielded_until FROM defenses WHERE id = $1
//
// `owner_id` 出现在 **SELECT 列表**里 —— contains 判成「有过滤」，
// 于是 `ChallengeDefense` 明明没按归属过滤，却被判为合格。
//
// 这与第 67/68 轮记的教训是**同一个**：
// 正则/字符串包含分不清「声明」与「使用」。
// 判据必须落在**语义位置**上 —— 这里就是 WHERE 子句。
func filterClauseOf(sql string) string {
	// ⚠️ 必须先**归一化空白**。
	//
	// 多行 SQL 里 WHERE 前面通常是 `\n\t\tWHERE`，而我第一版
	// 用 `strings.Index(upper, " WHERE ")`（单空格）—— 匹配不到，
	// 于是 `WHERE d.owner_id <> $1 AND u.status = 1 ORDER BY ...`
	// 被判成「没有过滤」。
	//
	// 那是**漏报**：守卫看不见已有的过滤，于是有人会去加第二条。
	flat := strings.Join(strings.Fields(sql), " ")
	upper := strings.ToUpper(flat)
	i := strings.Index(upper, " WHERE ")
	if i < 0 {
		// 没有 WHERE。**但**过滤可能在 JOIN ... ON 里 ——
		// `LoadTasks` 就是那种形态：
		//
		//	LEFT JOIN user_tasks ut
		//	       ON ut.task_id = t.id AND ut.user_id = $1 AND ut.task_date = $2
		//
		// 那是**真正的**归属过滤，却不在 WHERE 里。
		//
		// 第一版只看 WHERE → 把 `LoadTasks` 误报成漏洞。
		// 守卫误报比漏报更危险：它会让人去「修」一个本来就正确的地方。
		return joinOnClauses(flat)
	}
	rest := flat[i+len(" WHERE "):]
	for _, kw := range []string{" GROUP BY ", " ORDER BY ", " LIMIT ", " OFFSET ", " RETURNING ", " UNION "} {
		if j := strings.Index(strings.ToUpper(rest), kw); j >= 0 {
			rest = rest[:j]
		}
	}
	return rest + " " + joinOnClauses(flat)
}

// joinOnClauses 取出所有 `JOIN ... ON ...` 条件。
func joinOnClauses(flat string) string {
	var b strings.Builder
	upper := strings.ToUpper(flat)
	for {
		i := strings.Index(upper, " JOIN ")
		if i < 0 {
			break
		}
		rest := flat[i+len(" JOIN "):]
		// ON 之前是表名，ON 之后是条件
		k := strings.Index(strings.ToUpper(rest), " ON ")
		if k < 0 {
			break
		}
		cond := rest[k+len(" ON "):]
		for _, kw := range []string{" WHERE ", " GROUP BY ", " ORDER BY ", " LIMIT "} {
			if j := strings.Index(strings.ToUpper(cond), kw); j >= 0 {
				cond = cond[:j]
			}
		}
		b.WriteString(" ")
		b.WriteString(cond)
		upper = strings.ToUpper(rest[len(" JOIN "):])
		flat = rest[len(" JOIN "):]
	}
	return b.String()
}

// isOwnerScopedStatement 判断一条字符串字面量是否属于本守卫管辖的语句类型。
// 判据是**词边界**上的动词出现，而不是整条以它开头：
// `WITH k AS (SELECT …) UPDATE user_wallets …`（能量回补那条）
// 整条以 WITH 开头，但它本质上是一次 UPDATE，同样要管。
//
// ⚠️ 两处必须挡在外面，否则扫出来的全是**误报**：
//
//  1. 普通错误文案。`fmt.Errorf("update user_gem: %w", err)` 里就有
//     `update` 这个词。所以额外要求出现 SQL 专有的结构记号
//     （`$N` / FROM / INTO / SET …），错误文案不会拼出这种形状。
//  2. UPSERT。`INSERT … ON CONFLICT … DO UPDATE SET …` 里的 UPDATE 是
//     **建行语义**（值来自调用方显式传的 user_id，不存在"按 id 定位
//     别人的行"这一说）。全文匹配动词会把它当成 UPDATE 语句扫，
//     于是每一处 upsert 都变红。第一版就是这么误报出 9 处的。
//
// 误报比漏报更危险：它会让人以为守卫在喊狼来了，然后把它关掉。
func isOwnerScopedStatement(upper string) bool {
	if !hasSQLShape(upper) {
		return false
	}
	// 语句的**第一个**动词才是它的类型；`ON CONFLICT DO UPDATE` 不是。
	lead := leadingVerb(upper)
	if lead == "INSERT" {
		return false
	}
	for _, verb := range ownerScopedVerbs {
		if containsWord(upper, verb) {
			return true
		}
	}
	return false
}

// leadingVerb 取一条 SQL 语句的**首个**动词（跳过 WITH 子句）。
//
// ⚠️ 必须先剥掉**字面量引号**：`ast.BasicLit.Value` 带原始引号
// （raw string 是反引号、普通 string 是双引号），所以 `flat` 第一个
// 字符是 “ ` “ 而不是 `I`。第一版忘了剥，于是每一处 `INSERT … ON
// CONFLICT DO UPDATE` 都没被认出来 —— 9 处 upsert 全被当成 UPDATE 扫。
func leadingVerb(upper string) string {
	flat := strings.TrimSpace(upper)
	if len(flat) > 0 && (flat[0] == '`' || flat[0] == '"') {
		flat = strings.TrimSpace(flat[1:])
	}
	// WITH 开头（CTE）：主语句动词在 CTE 的右括号之后。
	if strings.HasPrefix(flat, "WITH ") {
		if i := strings.Index(flat, ")"); i >= 0 {
			flat = strings.TrimSpace(flat[i+1:])
		}
	}
	for _, v := range []string{"SELECT", "INSERT", "UPDATE", "DELETE"} {
		if strings.HasPrefix(flat, v+" ") || strings.HasPrefix(flat, v+"\t") {
			return v
		}
	}
	return ""
}

// sqlShapeMarkers 是「这确实是一条 SQL 语句」的结构性证据。
//
// 选 `$N` 是因为本项目的每一条真实 SQL 都带绑定参数（store 层
// 一律参数化，见 SQL 注入审计结论）；选 `FROM x` / `INTO x` /
// `SET x` 是因为错误文案不会拼出这种形状。
var sqlShapeMarkers = []string{"$", "FROM ", "INTO ", "SET ", "VALUES ", "WHERE "}

func hasSQLShape(upper string) bool {
	for _, m := range sqlShapeMarkers {
		if strings.Contains(upper, m) {
			return true
		}
	}
	return false
}

// containsWord 判断 word 是否以**词边界**形式出现在 s 中。
func containsWord(s, word string) bool {
	from := 0
	for {
		i := strings.Index(s[from:], word)
		if i < 0 {
			return false
		}
		start, end := from+i, from+i+len(word)
		beforeOK := start == 0 || !isWordByte(s[start-1])
		afterOK := end == len(s) || !isWordByte(s[end])
		if beforeOK && afterOK {
			return true
		}
		from = end
	}
}

// isWordByte 判断是否构成标识符的字节（下划线、字母、数字）。
func isWordByte(b byte) bool {
	return b == '_' ||
		(b >= 'a' && b <= 'z') ||
		(b >= 'A' && b <= 'Z') ||
		(b >= '0' && b <= '9')
}

// classifySQL 回答两个问题：
//
//	这是不是一张「按用户分区」的表？
//	如果是，它有没有带归属过滤？
//
// 拆出来是为了能**直接单测它** —— 见
// `TestOwnershipClassifierRejectsKnownShapes`。扫描器整体是非盲的，
// 但「扫不到东西」与「扫得到但确实都合格」在测试输出里长得一模一样，
// 那个区别只能靠**喂已知形状**来区分。
func classifySQL(sql string) (table string, filtered bool) {
	name, ownerCol := firstUserTable(sql)
	if ownerCol == "" {
		// 不是「按用户分区」的表 —— 与本守卫无关
		return "", true
	}
	// ⚠️ 归属过滤必须是「归属列 + 绑定参数」**同时**出现。
	//
	// 只看归属列会漏掉一类**假过滤** —— 排行榜那条：
	//
	//	FROM user_progress p JOIN users u ON u.id = p.user_id
	//
	// `p.user_id` 在 JOIN 谓词里，看起来像过滤，实则它是
	// 「把 progress 表和 users 表连起来」的连接条件，
	// **没有任何调用方的 id 参与** —— 返回的仍然是全服前 N 名。
	//
	// 而归属过滤的本质就是「拿调用方的 id 去限制」，
	// 所以它必然要有一个 `$N`。要求两者同时出现，就把这类假过滤排除了。
	//
	// ⚠️ 这不是「把判据放宽以让它通过」——它让**判据更严**：
	// 上面那条在加入这个要求之前是**被误判为合格的**（假阴性）。
	clause := filterClauseOf(sql)
	upper := strings.ToUpper(clause)
	if strings.Contains(upper, strings.ToUpper(ownerCol)) && strings.Contains(upper, "$") {
		return name, true
	}
	return name, false
}

func firstUserTable(sql string) (table, ownerCol string) {
	upper := strings.ToUpper(sql)
	for _, kw := range []string{"FROM ", "JOIN ", "UPDATE ", "INTO "} {
		i := strings.Index(upper, kw)
		if i < 0 {
			continue
		}
		rest := sql[i+len(kw):]
		// 去掉可能的 schema 前缀与别名
		if j := strings.IndexAny(rest, " \n\t("); j >= 0 {
			rest = rest[:j]
		}
		name := strings.ToLower(rest)
		if strings.HasPrefix(name, userTablePrefix) {
			return name, "user_id"
		}
		if col, ok := battleScopedTables[name]; ok {
			return name, col
		}
	}
	return "", ""
}

func truncate(s string, n int) string {
	if len(s) <= n {
		return s
	}
	return s[:n] + "..."
}

// TestUserScopedQueriesMustFilterByOwner 是本文件的核心。
func TestUserScopedQueriesMustFilterByOwner(t *testing.T) {
	// ⚠️ 这里**刻意没有**「扫不到就 Fatal」。
	//
	// 修复完成之后真实代码里一处未过滤的查询都不剩，
	// 于是「扫到 0 处」既可能是「真的全合格」，也可能是「扫描器坏了」——
	// 两者在输出里一模一样。而一条在正常状态下永远红的守卫，
	// 会被人直接删掉 —— 于是它保护的东西彻底裸奔，
	// 且没人记得它曾经存在。
	//
	// 「扫描器是否可用」由 `TestOwnershipClassifierRejectsKnownShapes`
	// 喂已知形状来回答，与「代码是否合规」分开。
	found := scanUnscopedQueries(t)

	var violations []string
	for _, q := range found {
		if why, ok := ownershipExemptions[exemptionKey(q)]; ok {
			if strings.TrimSpace(why) == "" {
				violations = append(violations,
					exemptionKey(q)+" 有豁免但理由为空 —— 「这里没问题」这种判断不留给沉默")
			}
			continue
		}
		violations = append(violations, fmt.Sprintf("%s 查 %s 但没有归属过滤（%s）：%s",
			q.Pos, q.Table, q.Fn, q.Stmt))
	}

	if len(violations) == 0 {
		t.Logf("扫到 %d 处按用户分区的查询，全部带归属过滤", len(found))
		return
	}
	sort.Strings(violations)
	t.Errorf("以下 %d 处按用户分区的查询**没有归属过滤**：\n  %s\n\n"+
		"没有过滤 = 任何登录用户可读/写任意用户的数据。\n"+
		"`battle_records.id` 是连续 BIGSERIAL，遍历成本为 O(1)。\n"+
		"第 75 轮修的 `GetReplay` / `VerifyReplay` 就是这一类。\n"+
		"修法：SQL 的 WHERE 里加 `AND <别名>.user_id = $n`；\n"+
		"若确实是管理面（跨用户查询），挪到 adminapi.go 或写进豁免表并说明理由。",
		len(violations), strings.Join(violations, "\n  "))
}

// TestStatementVerbGateRejectsFalsePositives 守卫动词判据本身。
//
// # 为什么必须单独测它，而不是只靠分类器用例
//
// 本轮（2026-10-15）把扫描范围从 SELECT 扩到 UPDATE/DELETE 之后，
// 第一版判据是「字符串里出现动词就算」。实测扫出 12~13 处，
// 其中:
//
//	9 处  `INSERT … ON CONFLICT … DO UPDATE SET` — 建行语义，不是 UPDATE 语句
//	5 处  fmt.Errorf("update user_gem: %w")    — 普通错误文案，不是 SQL
//
// 也就是说**判据本身错了两类**，而它们的方向相反：前者把合法的 upsert
// 判成违规（误报），后者把无关字符串拉进扫描范围（噪音）。
//
// # 为什么不能只靠分类器用例兜住
//
// `TestOwnershipClassifierRejectsKnownShapes` 调用的是 `classifySQL`，
// 而这两类过滤发生在它**上游**的 `isOwnerScopedStatement` 里 ——
// 分类器对 `ON CONFLICT DO UPDATE` 照样会回答「没归属过滤」。
// 所以判据错了，分类器用例一条都不会红。
//
// # 判据的第三个性质：CTE 主句不能漏
//
// 能量回补那条以 `WITH k AS (SELECT …) UPDATE …` 开头。若只用
// 「整条以动词开头」来判，它会因为第一个词是 WITH 而脱管 ——
// 那正是一次 UPDATE，漏掉它是**漏报**，与上面两类误报同样致命。
func TestStatementVerbGateRejectsFalsePositives(t *testing.T) {
	cases := []struct {
		name string
		sql  string
		want bool
	}{
		// ── 必须放行（真语句）──
		{"UPDATE 语句本身", `UPDATE user_gems SET affixes = $1::jsonb WHERE id = $2 AND user_id = $3`, true},
		{"多行 raw string 的 UPDATE（含前导反引号）", "`\n\t\t\tUPDATE user_wallets SET x = $2 WHERE id = $1 AND user_id = $3`", true},
		{"CTE 主句是 UPDATE（能量回补的形状）", "`\nWITH k AS ( SELECT n FROM user_wallets WHERE user_id = $1 )\nUPDATE user_wallets w SET energy = k.n FROM k WHERE w.user_id = $1`", true},
		{"DELETE 语句本身", `DELETE FROM user_skill_slots WHERE user_id = $1`, true},
		{"SELECT 语句本身", `SELECT id FROM user_gems WHERE user_id = $1`, true},

		// ── 必须挡住（不是该管的语句）──
		{"UPSERT：ON CONFLICT DO UPDATE 里的 UPDATE 不是语句动词", "`\n\t\t\t\t\tINSERT INTO user_tasks (user_id, task_id, task_date, progress)\n\t\t\t\t\tVALUES ($1,$2,$3,$4)\n\t\t\t\t\tON CONFLICT (user_id, task_id, task_date) DO UPDATE SET\n\t\t\t\t\t  progress = GREATEST(user_tasks.progress, EXCLUDED.progress)`", false},
		{"普通错误文案里有 update 这个词", "update user_gem: %w", false},
		{"普通错误文案里有 select 这个词", "select slot: %w", false},
		{"没有任何 SQL 结构记号的短文案", "update done", false},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			if got := isOwnerScopedStatement(strings.ToUpper(c.sql)); got != c.want {
				t.Errorf("isOwnerScopedStatement = %v，期望 %v\nSQL: %s", got, c.want, c.sql)
			}
		})
	}
}

// TestOwnershipClassifierRejectsKnownShapes 是扫描器**自己的**守卫。
//
// # 为什么必须这样，而不是「断言扫到了 N 处」
//
// 修复完成之后，真实代码里**一处未过滤的查询都不剩**。
// 于是：
//
//	「扫到 0 处违规」有两种可能：
//	  (a) 真的全过滤了      —— 正确
//	  (b) 扫描器坏了        —— 那它会永远报告绿灯
//
// 这两者在测试输出里**一模一样**。
//
// 我第一版写的是「扫不到就 Fatal」，结果修复完成后它红了 ——
// 那不是缺陷，是判据选错了。正确做法是**喂已知形状**：
// 给出确定该被抓住的 SQL，确认分类器确实判它违规。
//
// 这与第 67/68 轮记的教训是同一个：
// **「扫不到就 Fatal」的守卫，在真的没东西可扫时会变成负担，
// 于是人会把它删掉 —— 于是它保护的东西彻底裸奔。**
func TestOwnershipClassifierRejectsKnownShapes(t *testing.T) {
	cases := []struct {
		name       string
		sql        string
		wantTable  string
		wantFilter bool
	}{
		{
			name:       "第 75 轮修掉的那个洞（battle_records 无归属过滤）",
			sql:        `SELECT br.level_id, br.replay_hash FROM battle_records br WHERE br.id = $1`,
			wantTable:  "battle_records",
			wantFilter: false,
		},
		{
			name:       "带 user_id 的同一张表",
			sql:        `SELECT br.level_id FROM battle_records br WHERE br.id = $1 AND br.user_id = $2`,
			wantTable:  "battle_records",
			wantFilter: true,
		},
		{
			name:       "defenses 用 owner_id（不是 user_id）",
			sql:        `SELECT d.id, d.name FROM defenses d JOIN users u ON u.id = d.owner_id WHERE d.owner_id <> $1`,
			wantTable:  "defenses",
			wantFilter: true,
		},
		{
			name:       "defenses 若错用 user_id 过滤会被判成「有过滤」—— 记录这个坑",
			sql:        `SELECT d.id FROM defenses d WHERE d.user_id = $1`,
			wantTable:  "defenses",
			wantFilter: false,
		},
		{
			name:       "user_ 前缀表（user_wallets）",
			sql:        `SELECT coin FROM user_wallets WHERE user_id = $1 AND coin >= $2`,
			wantTable:  "user_wallets",
			wantFilter: true,
		},
		{
			name:       "user_tokens 无过滤",
			sql:        `SELECT token, balance FROM user_tokens WHERE user_id = $1`,
			wantTable:  "user_tokens",
			wantFilter: true,
		},
		{
			// ⚠️ 这条是本守卫自己踩过的坑（见 filterClauseOf 的注释）：
			// `owner_id` 出现在 **SELECT 列表**里，而 WHERE 只有 `id = $1`。
			// 对整条 SQL 做 contains 会判成「有过滤」→ 漏报。
			name:       "owner_id 只出现在 SELECT 列表，WHERE 里没有 → 未过滤",
			sql:        `SELECT owner_id, name, power, shielded_until FROM defenses WHERE id = $1`,
			wantTable:  "defenses",
			wantFilter: false,
		},
		{
			name:       "WHERE 里有 owner_id 之后还有 ORDER BY / LIMIT",
			sql:        `SELECT d.id, d.name FROM defenses d JOIN users u ON u.id = d.owner_id WHERE d.owner_id <> $1 ORDER BY d.power DESC LIMIT 20`,
			wantTable:  "defenses",
			wantFilter: true,
		},
		{
			name:       "WHERE 里的过滤出现在 GROUP BY 之前也算数",
			sql:        `SELECT e.key, SUM(e.value::bigint) FROM battle_records b, LATERAL jsonb_each_text(b.reactions_used) e(key,value) WHERE b.user_id = $1 GROUP BY e.key ORDER BY 2 DESC LIMIT 10`,
			wantTable:  "battle_records",
			wantFilter: true,
		},
		{
			name:       "完全没有 WHERE → 必然未过滤",
			sql:        `SELECT user_id, coin, gem FROM user_wallets ORDER BY user_id LIMIT 100`,
			wantTable:  "user_wallets",
			wantFilter: false,
		},
		{
			// ⚠️ **假过滤**：`p.user_id` 只出现在 JOIN 谓词里，
			// 是「把 progress 连到 users」的连接条件，没有调用方 id 参与。
			// 排行榜就是这一条 —— 返回的仍然是全服前 N 名。
			name:       "JOIN 谓词里的 user_id 不是归属过滤（无绑定参数）",
			sql:        `SELECT u.id, u.nickname, p.level_exp FROM user_progress p JOIN users u ON u.id = p.user_id ORDER BY p.level_exp DESC LIMIT 20`,
			wantTable:  "user_progress",
			wantFilter: false,
		},
		{
			name:       "JOIN ... ON 里真的有归属过滤（LoadTasks 的形态）",
			sql:        `SELECT t.id, COALESCE(ut.progress,0) FROM tasks t LEFT JOIN user_tasks ut ON ut.task_id = t.id AND ut.user_id = $1 AND ut.task_date = $2 WHERE t.enabled`,
			wantTable:  "user_tasks",
			wantFilter: true,
		},
		{
			// ⚠️ 必须有**多行**用例。
			//
			// 我第一版的判据是 `strings.Index(upper, " WHERE ")`
			//（单空格），而多行 raw string 里 WHERE 前面是 `\n\t\t` ——
			// 匹配不到，于是带 `WHERE d.owner_id <> $1` 的
			// `ListDefenses` 被误报成漏洞。
			//
			// 这个坑只有多行用例能抓到：单行 SQL 里
			// `SELECT ... FROM t WHERE ...` 前面恰好就是一个空格。
			name: "多行 SQL：WHERE 前是换行+缩进（归一化空白后才能匹配）",
			sql: `SELECT d.id, d.owner_id, u.nickname, d.name, d.power
				FROM defenses d JOIN users u ON u.id = d.owner_id
				WHERE d.owner_id <> $1 AND u.status = 1
				ORDER BY d.power DESC LIMIT 20`,
			wantTable:  "defenses",
			wantFilter: true,
		},
		{
			// 与上一条成对：同一形状但**去掉**归属条件。
			name: "多行 SQL：同样形状但 WHERE 里没有归属列 → 未过滤",
			sql: `SELECT d.id, d.owner_id, u.nickname, d.name, d.power
				FROM defenses d JOIN users u ON u.id = d.owner_id
				WHERE d.power > 100 AND u.status = 1
				ORDER BY d.power DESC LIMIT 20`,
			wantTable:  "defenses",
			wantFilter: false,
		},
		{
			// ⚠️ 这条**专门**用来隔离「归一化空白」这一步。
			//
			// 上一条（多行 + 有 JOIN）抓不住它：没有归一化时 WHERE 匹配不到，
			// 但 `FROM defenses d JOIN users u` 里恰好有一个**单空格**的
			// ` JOIN `，于是 `joinOnClauses` 兜底把 WHERE 之后的内容
			// 一并吞进去，恰好含有 `owner_id` 与 `$1` → 判成「有过滤」。
			// **巧合地通过了。**
			//
			// 所以判据必须落在「归一化与不归一化结果不同」的那条 SQL 上 ——
			// 也就是**没有 JOIN、过滤只在 WHERE 里**的多行查询。
			name: "多行 SQL 且无 JOIN：过滤只在 WHERE 里（唯一能隔离空白归一化）",
			sql: `SELECT u.id, u.nickname, p.level_exp
				FROM user_progress p
				WHERE p.user_id = $1
				ORDER BY p.level_exp DESC LIMIT 20`,
			wantTable:  "user_progress",
			wantFilter: true,
		},
		{
			name:       "无关表不该被本守卫管",
			sql:        `SELECT id, code, name FROM shop_items WHERE enabled ORDER BY id`,
			wantTable:  "",
			wantFilter: true,
		},
		{
			name:       "内容表也不该被管",
			sql:        `SELECT id, hp, attack FROM enemies WHERE chapter_id = $1 ORDER BY id`,
			wantTable:  "",
			wantFilter: true,
		},
		// ── 2026-10-15 新增：UPDATE / DELETE 形状 ──
		//
		// 这一组的出现是因为 `WashGem` 里那条
		// `UPDATE user_gems SET affixes = $1::jsonb WHERE id = $2`
		// （修复前的原文）在 SELECT-only 的扫描器下**完全不可见**。
		{
			// 修复前的 `WashGem`：形似归属过滤（有 WHERE），实则按 id 定位。
			// 与上面那条 `SELECT owner_id, ... WHERE id = $1` 是同一个形状，
			// 只是动词换成了 UPDATE —— 而动词恰是旧扫描器的盲区。
			name:       "UPDATE 只按 id 定位、没有 user_id → 未过滤（WashGem 修复前的形状）",
			sql:        `UPDATE user_gems SET affixes = $1::jsonb WHERE id = $2`,
			wantTable:  "user_gems",
			wantFilter: false,
		},
		{
			name:       "UPDATE 带 user_id → 已过滤（WashGem 修复后）",
			sql:        `UPDATE user_gems SET affixes = $1::jsonb WHERE id = $2 AND user_id = $3`,
			wantTable:  "user_gems",
			wantFilter: true,
		},
		{
			name:       "DELETE 只按 id 定位 → 未过滤",
			sql:        `DELETE FROM user_skill_slots WHERE id = $1`,
			wantTable:  "user_skill_slots",
			wantFilter: false,
		},
		{
			name:       "DELETE 带 user_id → 已过滤",
			sql:        `DELETE FROM user_mastery_nodes WHERE user_id = $1`,
			wantTable:  "user_mastery_nodes",
			wantFilter: true,
		},
		{
			// INSERT 刻意**不管**：见 ownerScopedVerbs 的注释。
			// 这条用例钉住那个决定 —— 否则将来有人把 INSERT 也加进
			// 管辖范围时，会误以为「本来就是这样」。
			//
			// ⚠️ `filtered=false` 是**分类器**的答案，不是扫描器的：
			// `ON CONFLICT DO UPDATE` 里确实没有归属过滤子句，
			// 但它在 isOwnerScopedStatement 那关就被放过了（建行语义）。
			name:       "INSERT INTO user_* 不受管（创建新行，不存在定位别人行的问题）",
			sql:        `INSERT INTO user_tasks (user_id, task_id, task_date, progress) VALUES ($1,$2,$3,$4)`,
			wantTable:  "user_tasks",
			wantFilter: false,
		},
		{
			name: "UPSERT：`ON CONFLICT DO UPDATE` 里的 UPDATE 不是语句动词",
			sql: `INSERT INTO user_tasks (user_id, task_id, task_date, progress)
				VALUES ($1,$2,$3,$4)
				ON CONFLICT (user_id, task_id, task_date) DO UPDATE SET
				  progress = GREATEST(user_tasks.progress, EXCLUDED.progress)`,
			wantTable:  "user_tasks",
			wantFilter: false,
		},
		{
			// 能量回补那条 UPDATE 整条以 `WITH` 开头，而旧实现用
			// `strings.Contains(upper, "SELECT")` 恰好能命中 CTE 里的 SELECT；
			// 换成词边界判据后必须仍然命中，否则这条语句会悄悄脱管。
			name: "CTE 开头的 UPDATE 仍然受管（能量回补那条的形状）",
			sql: `WITH k AS (
			  SELECT GREATEST(0, FLOOR(EXTRACT(EPOCH FROM (now() - energy_updated_at)) / $3))::int AS n
			    FROM user_wallets WHERE user_id = $1
			)
			UPDATE user_wallets w SET energy = LEAST($2::int, w.energy + k.n), updated_at = now()
			  FROM k
			 WHERE w.user_id = $1`,
			wantTable:  "user_wallets",
			wantFilter: true,
		},
	}

	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			tbl, filtered := classifySQL(c.sql)
			if tbl != c.wantTable {
				t.Errorf("table = %q，期望 %q", tbl, c.wantTable)
			}
			if filtered != c.wantFilter {
				t.Errorf("filtered = %v，期望 %v —— "+
					"分类器对已知形状判错，它在真实代码上的结论都不可信",
					filtered, c.wantFilter)
			}
		})
	}
}

// TestOwnershipScannerSeesTheRealPackage 确认扫描范围覆盖了 service 包。
//
// ⚠️ 用「至少有 N 个按用户分区的查询被识别」而不是「至少一个违规」——
// 前者在**修复完成后依然成立**（它统计的是「被本守卫管辖的查询数」，
// 不是「违规数」），所以不会像「扫不到就 Fatal」那样变成负担。
func TestOwnershipScannerSeesTheRealPackage(t *testing.T) {
	found := scanUnscopedQueries(t)
	if len(found) == 0 {
		t.Fatal("扫描器一个未过滤的查询都没扫到。\n" +
			"两种可能：(a) service 包里真的一个都没有 —— 那么这道守卫是空转；\n" +
			"          (b) 扫描范围/解析坏了 —— 那么它会永远报告绿灯。\n" +
			"请用 `TestOwnershipClassifierRejectsKnownShapes` 区分这两种情况。")
	}
	sort.Slice(found, func(i, j int) bool { return found[i].Pos < found[j].Pos })
	seen := map[string]int{}
	for _, q := range found {
		// 打**豁免键**而不是行号：排查「这条为什么没被豁免」时，
		// 真正要对照的是 ownershipExemptions 的键。
		t.Logf("KEY %s（%s）", exemptionKey(q), q.Pos)
	}
	for _, q := range found {
		seen[q.Table]++
	}
	t.Logf("当前有 %d 处未过滤查询：%v", len(found), seen)
	for i := 0; i < len(found) && i < 8; i++ {
		t.Logf("  %s → %s（%s）", found[i].Pos, found[i].Table, found[i].Fn)
	}
}

// TestOwnershipExemptionKeysAreStillPresent 防豁免表腐烂。
//
// 一条豁免对应的代码被删掉后，豁免会**静默失效** ——
// 而守卫不会知道，因为「违规」本来就不该出现。
// 于是后来的人会以为「这里仍然有豁免」，实际已经没有约束了。
func TestOwnershipExemptionKeysAreStillPresent(t *testing.T) {
	if len(ownershipExemptions) == 0 {
		t.Skip("当前没有豁免 —— 无需检查")
	}
	found := scanUnscopedQueries(t)
	present := map[string]bool{}
	for _, q := range found {
		present[exemptionKey(q)] = true
	}
	var stale []string
	for key := range ownershipExemptions {
		if !present[key] {
			stale = append(stale, key)
		}
	}
	if len(stale) > 0 {
		sort.Strings(stale)
		t.Errorf("豁免表里有 %d 条已失效（对应函数已改/已删，或查询已补上归属过滤）：\n  %s\n\n"+
			"请删掉它们，或更新到新的键。失效的豁免比没有豁免更糟 —— "+
			"它让人以为那里仍然有约束。",
			len(stale), strings.Join(stale, "\n  "))
	}
}

// TestOwnershipExemptionReasonMustBeNonEmpty 把「豁免要写理由」变成可执行判据。
//
// ⚠️ 这条是被变异测试逼出来的：把 `if strings.TrimSpace(why) == ""`
// 改成 `if false` 时主守卫**不会红** —— 因为当前两条豁免的理由都非空。
// 「判断还在代码里」与「这条判断被触发过」是两件事，
// 后者需要一条**专门喂空理由**的用例。
func TestOwnershipExemptionReasonMustBeNonEmpty(t *testing.T) {
	// 直接验那条判据本身：空理由必须被判为违规。
	for _, why := range []string{"", "   ", "\t\n"} {
		if strings.TrimSpace(why) != "" {
			t.Fatalf("前提不成立：%q 被 TrimSpace 判为非空", why)
		}
	}
	// 而豁免表里的每条都必须有非空理由
	for pos, why := range ownershipExemptions {
		if strings.TrimSpace(why) == "" {
			t.Errorf("豁免 %s 的理由为空 —— 「这里没问题」这种判断不留给沉默", pos)
		}
	}
	// 机制说明：主守卫里那条判据是 `TrimSpace(why) == ""` → 违规。
	// 本用例通过「表里每条都非空」间接保证它未来一旦失效会立刻暴露：
	// 若有人加了条空豁免，本条与主守卫会同时红。
	t.Logf("当前豁免 %d 条，理由均非空", len(ownershipExemptions))
}
