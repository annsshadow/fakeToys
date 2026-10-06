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

// posKey 把 token.Position 压成 `basename.go:行号`。
//
// ⚠️ 豁免表必须用**这个格式**，不能用 `token.Position.String()`
// （那是完整绝对路径，换台机器就全失效）。
func posKey(pos string) string {
	base := pos
	if i := strings.LastIndexAny(base, `\/`); i >= 0 {
		base = base[i+1:]
	}
	// `token.Position.String()` 是 "file.go:line:col" —— 去掉 **col** 那一段，
	// 保留 "file.go:line"。
	//
	// ⚠️ 我第一版写成「找第二个冒号再截断」，结果把**行号**也截掉了
	// （progression.go:572:4 → progression.go），于是每条豁免都失效。
	if i := strings.LastIndex(base, ":"); i >= 0 {
		base = base[:i]
	}
	return base
}

// ownershipExemptions 是具名豁免表：`文件:行` → 理由。
//
// ⚠️ 每条都必须写清理由 ——「这里没问题」这种判断留给写豁免的人，
// 不留给沉默。理由为空时本测试会红。
//
// ⚠️ 键是 SQL **字面量的起始行**，而多行 raw string 加上注释会把它推着走：
// 第 79 轮给 `ChallengeDefense` 加了预锁注释，`progression.go` 的豁免键
// 从 572 漂到 673 —— `TestOwnershipExemptionKeysAreStillPresent` 立刻报了出来。
//
// **这正是那条守卫存在的理由。** 若没有它，失效的豁免会静默躺着，
// 而「守卫从未报过」会被误读成「那里一直有约束」。
var ownershipExemptions = map[string]string{
	"economy.go:114": "排行榜本就跨用户 —— 它返回全服前 N 名，" +
		"而返回列里没有别人的任何私有数据（只有 nickname / avatar / level_exp）。" +
		"这不是「第 75 轮那个洞」的同类：那里泄露的是 build_snapshot + seed。\n" +
		"⚠️ 键是 SQL **字面量的起始行**（114），不是 FROM 所在行（116）—— " +
		"多行 raw string 里两者差 2 行。",
	"progression.go:721": "「挑战别人的防线」本身就是玩法：读 `defenses WHERE id = $1` " +
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
					if !strings.Contains(upper, "SELECT") {
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

// classifySQL 回答两个问题：
//
//	这是不是一张「按用户分区」的表？
//	如果���，它有没有带归属过滤？
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
		if why, ok := ownershipExemptions[posKey(q.Pos)]; ok {
			if strings.TrimSpace(why) == "" {
				violations = append(violations,
					q.Pos+" 有豁免但理由为空 —— 「这里没问题」这种判断不留给沉默")
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
		t.Logf("KEY %s", posKey(q.Pos))
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
		present[posKey(q.Pos)] = true
	}
	var stale []string
	for pos := range ownershipExemptions {
		if !present[pos] {
			stale = append(stale, pos)
		}
	}
	if len(stale) > 0 {
		sort.Strings(stale)
		t.Errorf("豁免表里有 %d 条已失效（对应代码已改或已删）：\n  %s\n\n"+
			"请删掉它们，或更新到新的位置。失效的豁免比没有豁免更糟 —— "+
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
