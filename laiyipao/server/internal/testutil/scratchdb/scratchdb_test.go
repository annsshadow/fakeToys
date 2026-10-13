package scratchdb

// 这个包自身的行为守卫。
//
// 它管的是**测试基础设施**，而基础设施出错的症状极其难认：
// 「测试全绿」可能意味着一半用例根本没跑；「测试变红」可能只是孤儿库太多。
// 所以这里把三条约定各自钉住。

import (
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"testing"
)

// TestNameIsUniqueWithinProcess 库名必须在进程内唯一。
//
// 为什么这条最重要：旧实现用 `time.Now().UnixNano() % 1e9`，
// 取的是**最后 9 位**（秒以下部分），于是名字**每秒钟绕回一次** ——
// 两个相隔整一秒的测试会拿到同名库，一个的 Cleanup 把另一个正在用的库删掉。
// 症状不是超时，而是后续测试的种子 INSERT 撞主键（SQLSTATE 23505）。
func TestNameIsUniqueWithinProcess(t *testing.T) {
	seen := make(map[string]bool, 1000)
	for i := 0; i < 1000; i++ {
		n := Name("laiyipao_svc_test_")
		if seen[n] {
			t.Fatalf("第 %d 次生成的名字与之前重复：%s", i, n)
		}
		seen[n] = true
	}
}

// TestNameCarriesPrefix Prefix 必须原样出现在名字里。
//
// Sweep 是按前缀 `LIKE 'prefix%'` 过滤的，而前缀写错的后果是
// 「孤儿库照样累积」—— 要跑到几百个库、PG 开始超时才显形。
func TestNameCarriesPrefix(t *testing.T) {
	for _, p := range Prefixes {
		got := Name(p)
		if !strings.HasPrefix(got, p) {
			t.Errorf("Name(%q) = %q，没有保留前缀", p, got)
		}
	}
}

// dbNameTimeModulo 是被禁的写法：拿「时间取模」当**一次性库名**。
//
// 形态限定得很窄 —— 必须是「同一个 Sprintf 调用里，既有 laiyipao 开头的
// 库名前缀字面量，又有 UnixNano()%1e9」。
//
// 为什么不能只查 `UnixNano()%1_000_000_000`：这个仓库里**兑换码/工单码**
// 也用同样的取模（例如 `CONC%d`），而它们对唯一性的要求完全不同 ——
// 撞名的后果是「两张券同一串」，比库名互删轻得多，不该被同一条守卫拦住。
const dbNameTimeModulo = `laiyipao`

// TestNoScratchNameUsesTimeModulo 扫全部测试源码，禁止再出现按时间取模的**库名**。
//
// 这是一条**结构性**守卫：`UnixNano() % 1_000_000_000` 这种写法
// 一旦被人「照着旧代码」写回来，只有等库累积到几百个才会显形。
func TestNoScratchNameUsesTimeModulo(t *testing.T) {
	root := repoRoot(t)
	self, err := filepath.Abs(".")
	if err != nil {
		t.Fatal(err)
	}
	var offenders []string
	err = filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if info.IsDir() {
			if name := info.Name(); name == "node_modules" || name == ".git" || name == "dist" {
				return filepath.SkipDir
			}
			return nil
		}
		if !strings.HasSuffix(path, ".go") {
			return nil
		}
		abs, err := filepath.Abs(path)
		if err == nil && strings.HasPrefix(abs, self) {
			return nil // 守卫自身含有这个模式字面量，跳过自己
		}
		fset := token.NewFileSet()
		f, err := parser.ParseFile(fset, path, nil, 0)
		if err != nil {
			return nil
		}
		ast.Inspect(f, func(n ast.Node) bool {
			call, ok := n.(*ast.CallExpr)
			if !ok {
				return true
			}
			// 找形如 fmt.Sprintf("laiyipao_xxx_%d", ..., time.Now().UnixNano()%1_000_000_000)
			hasLibPrefix, hasTimeModulo := false, false
			ast.Inspect(call, func(m ast.Node) bool {
				switch v := m.(type) {
				case *ast.BasicLit:
					if v.Kind == token.STRING && strings.HasPrefix(strings.Trim(v.Value, `"`), dbNameTimeModulo) {
						hasLibPrefix = true
					}
				case *ast.SelectorExpr:
					if x, ok := v.X.(*ast.Ident); ok && x.Name == "time" && v.Sel.Name == "Now" {
						// 粗粒度判定：只要这棵子树里出现取模，就是嫌疑
						ast.Inspect(v, func(z ast.Node) bool {
							if b, ok := z.(*ast.BinaryExpr); ok && b.Op == token.REM {
								hasTimeModulo = true
							}
							return true
						})
					}
				}
				return true
			})
			if hasLibPrefix && hasTimeModulo {
				rel, _ := filepath.Rel(root, path)
				offenders = append(offenders, rel+" (Sprintf 里的库名用了时间取模)")
				return false
			}
			return true
		})
		return nil
	})
	if err != nil {
		t.Fatalf("遍历仓库失败：%v", err)
	}
	if len(offenders) > 0 {
		t.Errorf("这些地方还在用「时间取模」当一次性库名（每秒重复，会互删对方的库）：\n  %v\n"+
			"改成 scratchdb.DSN(t, prefix)", offenders)
	}
}

// TestEveryScratchPrefixIsRegistered 源码里用到的每个前缀都必须在 Prefixes 里。
//
// 反向漏登记的后果同 TestNameCarriesPrefix：Sweep 清不到它。
//
// 收集方式：扫所有 `const <名字含 Prefix> = "字面量"` 声明。
// 为什么不直接扫 `scratchdb.DSN(t, "...")` 的实参 —— 各包传的都是**常量标识符**
// （`scratchPrefix`），不是字面量，那样扫会一个都找不到，
// 于是「扫到 0 个就报错」的自检会先红，而那是个假警报。
func TestEveryScratchPrefixIsRegistered(t *testing.T) {
	root := repoRoot(t)
	registered := make(map[string]bool, len(Prefixes))
	for _, p := range Prefixes {
		registered[p] = true
	}
	fset := token.NewFileSet()
	found := 0
	err := filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if info.IsDir() {
			if name := info.Name(); name == "node_modules" || name == ".git" || name == "dist" {
				return filepath.SkipDir
			}
			return nil
		}
		if !strings.HasSuffix(path, ".go") {
			return nil
		}
		f, err := parser.ParseFile(fset, path, nil, 0)
		if err != nil {
			return nil
		}
		for _, decl := range f.Decls {
			gd, ok := decl.(*ast.GenDecl)
			if !ok || gd.Tok != token.CONST {
				continue
			}
			for _, spec := range gd.Specs {
				vs, ok := spec.(*ast.ValueSpec)
				if !ok || len(vs.Names) != 1 || len(vs.Values) != 1 {
					continue
				}
				if !strings.Contains(vs.Names[0].Name, "Prefix") {
					continue
				}
				lit, ok := vs.Values[0].(*ast.BasicLit)
				if !ok || lit.Kind != token.STRING {
					continue
				}
				prefix, err := strconv.Unquote(lit.Value)
				if err != nil {
					continue
				}
				if !strings.HasPrefix(prefix, "laiyipao") {
					continue
				}
				found++
				if !registered[prefix] {
					rel, _ := filepath.Rel(root, path)
					t.Errorf("%s 的常量 %s 用了前缀 %q，但它没登记在 scratchdb.Prefixes —— "+
						"Sweep 清不到它，孤儿库会重新累积", rel, vs.Names[0].Name, prefix)
				}
			}
		}
		return nil
	})
	if err != nil {
		t.Fatalf("遍历仓库失败：%v", err)
	}
	if found == 0 {
		t.Fatal("一个前缀常量都没扫到 —— 守卫本身失效了（仓库结构变了？）")
	}
	if found != len(Prefixes) {
		t.Logf("注意：Prefixes 有 %d 项，源码里只找到 %d 个前缀常量 —— "+
			"可能有多余登记（无害），也可能漏了调用点（看 TestNoScratchNameUsesTimeModulo）",
			len(Prefixes), found)
	}
}

func repoRoot(t *testing.T) string {
	t.Helper()
	dir, err := os.Getwd()
	if err != nil {
		t.Fatal(err)
	}
	// internal/testutil/scratchdb → 仓库根是 server/
	for i := 0; i < 3; i++ {
		dir = filepath.Dir(dir)
	}
	if _, err := os.Stat(filepath.Join(dir, "go.mod")); err != nil {
		t.Fatalf("找不到 go.mod（起点 %s）：%v", dir, err)
	}
	return dir
}
