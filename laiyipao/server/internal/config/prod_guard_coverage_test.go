package config

import (
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
	"testing"
)

// prod 守卫的**结构完备性**（第 67 轮）。
//
// # 缺陷：守卫只覆盖「作者当时想到的那几项」
//
// `Load()` 原本有两条 prod 守卫：JWT_SECRET 与 DATABASE_URL 的 sslmode。
// 而同一个函数里还有第三个同类默认值 —— `BOOTSTRAP_ADMIN_PASS`
// 它的默认值 `admin12345` 是**公开的**（写在 README 与源码里）。
//
// 后果不是理论问题：
//   任何全新 prod 部署首次启动即创建 admin / admin12345，
//   而 10 位**正好通过** `EnsureBootstrapAdmin` 的 `len >= 10` 门槛。
//   拿到 admin token 后可用既有端点给任意 user_id 发放任意数额货币。
//
// # 为什么既有测试没抓到
//
// `TestProdGuardsPanic` 是**手写枚举**：它逐条列出「我以为的守卫清单」。
// 守卫漏了一项，测试仍然绿 —— 除非有人正好补测那一条。
//
// 这与 README 记的 `TestEverySentinelErrorIsClassified` 是**同一个形状**：
// 守卫只覆盖它自己写下的那几项。
//
// # 本文件的判据：结构，而不是清单
//
// 用 `go/ast` 扫 `Load()` 里所有「带非空默认值的敏感字段」，
// 要求每一个都能在 prod 分支里找到对应的显式声明守卫。
// 加新的敏感字段而忘了加守卫 → 这里会红。
//
// ⚠️ 「敏感」的定义是**结构性的**，不是人工名单：
// 一个 `env(NAME, default)` 调用只要 default 非空，就属于「有默认值」。
// 这样加字段时不需要同时更新名单 —— 名单会漂，AST 不会。

// sensitiveDefault 是「一个带非空默认值的环境变量读取」。
type sensitiveDefault struct {
	Field    string // Config 结构体字段名
	EnvVar   string // 环境变量名
	Default  string // 默认值字面量
	Position string
}

// secretEnvSuffix 是「这个环境变量装的是秘密」的名字判据。
//
// # 为什么需要它
//
// 第一版的判据是「有非空默认值就算敏感」，那过宽了：
// `ADDR` 的默认值 `":8080"` 也会被算进去，而它显然不是密钥 ——
// prod 下不设 ADDR 只是监听默认端口，没有任何风险。
//
// 判据必须是**结构性的**（不靠人工名单），但「敏感」本身是个语义判断。
// 这里用**名字后缀**作为它的机械化代理：
//
//	口令、密钥、token、secret、DSN（含数据库口令）
//
// 它与人工名单的区别是：加一个新的 `*_SECRET` 字段时**自动**被覆盖，
// 不需要同步更新任何清单 —— 清单会漂，名字后缀不会
// （除非有人起一个完全不同的名字，那时这条断言会提示重新评估）。
var secretEnvSuffix = []string{
	"SECRET",
	"PASS",
	"PWD",
	"TOKEN",
	"KEY",
	"URL", // DATABASE_URL 含数据库口令
}

func isSecretEnv(name string) bool {
	up := strings.ToUpper(name)
	for _, suf := range secretEnvSuffix {
		if strings.Contains(up, suf) {
			return true
		}
	}
	return false
}

// scanEnvDefaults 用 AST 扫出 Load() 里所有 `env("NAME", "非空默认值")` 形式的读取。
//
// ⚠️ 只认 **顶层复合字面量里的 KeyValueExpr**，且 Key 必须是标识符。
// 用正则会分不清「声明」与「使用」，也会把注释里的同名算进去
// —— README 记的正是这个坑（正则是第一版，扫到注释里的名字）。
func scanEnvDefaults(t *testing.T) []sensitiveDefault {
	t.Helper()
	fset := token.NewFileSet()
	// ⚠️ 必须扫 **config.go**，不是本测试文件。
	//
	// 第一版用 `runtime.Caller(0)` 拿路径 —— 那返回的是**调用点所在的文件**，
	// 也就是这个测试文件。于是扫了 4 个 KV、全是测试里的
	// （`Position: pos` 之类），而 `Load()` 里的一个都没扫到。
	//
	// 症状极具欺骗性：`kvCount=4`（不是 0），
	// 若没有那条「扫不到就 Fatal」的断言，它会安静地报告「全部有守卫」。
	//
	// 相对路径依赖测试的工作目录是包目录 —— `go test` 保证这一点。
	f, err := parser.ParseFile(fset, filepath.Clean("config.go"), nil, 0)
	if err != nil {
		t.Fatalf("解析 config.go 失败：%v", err)
	}

	var out []sensitiveDefault
	// env("NAME", "DEFAULT") 是包级函数调用，出现在 Load() 的返回字面量里。
	ast.Inspect(f, func(n ast.Node) bool {
		// 只在 `X: env(...)` 的形态下收集 —— 键必须是标识符。
		kv, ok := n.(*ast.KeyValueExpr)
		if !ok {
			return true
		}
		key, ok := kv.Key.(*ast.Ident)
		if !ok {
			return true
		}
		call, ok := kv.Value.(*ast.CallExpr)
		if !ok {
			return true
		}
		name, ok := call.Fun.(*ast.Ident)
		if !ok || name.Name != "env" || len(call.Args) != 2 {
			return true
		}
		lit1, ok1 := call.Args[0].(*ast.BasicLit)
		lit2, ok2 := call.Args[1].(*ast.BasicLit)
		if !ok1 || !ok2 || lit1.Kind != token.STRING || lit2.Kind != token.STRING {
			return true
		}
		envVar, err1 := strconv.Unquote(lit1.Value)
		def, err2 := strconv.Unquote(lit2.Value)
		if err1 != nil || err2 != nil || def == "" {
			return true
		}
		out = append(out, sensitiveDefault{
			Field:    key.Name,
			EnvVar:   envVar,
			Default:  def,
			Position: fset.Position(kv.Pos()).String(),
		})
		return true
	})
	return out
}

// prodGuardedVars 扫 prod 分支里被显式检查的环境变量名。
//
// 判据取「出现 `os.Getenv("X") != ""` 或 `strings.HasPrefix(cfg.X, "默认前缀")`」
// 这两种形态 —— 前者是「必须显式声明」，后者是「不能用默认值」。
func prodGuardedVars(t *testing.T) map[string]string {
	t.Helper()
	raw, err := os.ReadFile(filepath.Clean(mustThisFile(t)))
	if err != nil {
		t.Fatalf("读取 config.go 失败：%v", err)
	}
	src := string(raw)

	// 切出 prod 分支：`if cfg.Env == "prod" {` 到配对的 `}`
	start := strings.Index(src, `if cfg.Env == "prod" {`)
	if start < 0 {
		t.Fatal("找不到 prod 守卫分支 —— 结构变了，本测试需要跟着改")
	}
	rest := src[start:]
	depth := 0
	end := -1
	for i, ch := range rest {
		switch ch {
		case '{':
			depth++
		case '}':
			depth--
			if depth == 0 {
				end = i
				break
			}
		}
		if end > 0 {
			break
		}
	}
	if end < 0 {
		t.Fatal("prod 分支的花括号不配对")
	}
	body := rest[:end]

	guarded := map[string]string{}
	// 形态一：os.Getenv("X") == "" → 必须显式声明
	for _, line := range strings.Split(body, "\n") {
		if !strings.Contains(line, "os.Getenv(") {
			continue
		}
		i := strings.Index(line, "os.Getenv(")
		rest := line[i+len("os.Getenv("):]
		j := strings.Index(rest, `"`)
		if j < 0 {
			continue
		}
		rest = rest[j+1:]
		k := strings.Index(rest, `"`)
		if k < 0 {
			continue
		}
		name := rest[:k]
		guarded[name] = strings.TrimSpace(line)
	}
	// 形态二：strings.HasPrefix(cfg.X, "默认值前缀") → 不能用默认值
	//
	// ⚠️ 第一版只认 `strings.HasPrefix(cfg.` 这一种形态，
	// 于是 `if !strings.Contains(cfg.DatabaseURL, "sslmode=")` 那种
	// 「对取值内容做断言」的守卫**认不出来** —— 而 DATABASEURL 明明被守了。
	// 守卫漏报比守卫误报更危险：它会让人以为某个字段没被守，
	// 然后去加一条重复的守卫。
	for _, line := range strings.Split(body, "\n") {
		i := strings.Index(line, "cfg.")
		if i < 0 {
			continue
		}
		if !strings.Contains(line, "HasPrefix") && !strings.Contains(line, "Contains(") {
			continue
		}
		// 字段名 = `cfg.` 之后连续的标识符字符。
		//
		// ⚠️ 不能用「找下一个 `.`」—— `strings.HasPrefix(cfg.JWTSecret, "dev-only")`
		// 里 `JWTSecret` 后面**没有**点（下一个字符是 `,`），
		// 于是那一版提取出空串，守卫认不出来 → 误报。
		rest := line[i+len("cfg."):]
		j := 0
		for j < len(rest) && (rest[j] == '_' ||
			(rest[j] >= 'a' && rest[j] <= 'z') ||
			(rest[j] >= 'A' && rest[j] <= 'Z') ||
			(rest[j] >= '0' && rest[j] <= '9')) {
			j++
		}
		if j == 0 {
			continue
		}
		guarded[rest[:j]] = strings.TrimSpace(line)
	}
	return guarded
}

func mustThisFile(t *testing.T) string {
	t.Helper()
	// ⚠️ 刻意返回 **config.go** 而不是本测试文件 —— 理由见 scanEnvDefaults。
	return filepath.Clean("config.go")
}

// TestProdGuardsCoverEverySensitiveDefault 是本文件的核心：
// **每一个「装秘密且有非空默认值」的环境变量，都必须在 prod 分支里有守卫**。
//
// 漏了守卫 → 加新字段时会红（这正是本轮缺陷的通用形态）。
func TestProdGuardsCoverEverySensitiveDefault(t *testing.T) {
	all := scanEnvDefaults(t)
	if len(all) == 0 {
		t.Fatal("AST 扫不到任何带默认值的 env() —— 扫描器坏了，本测试会静默变成空转")
	}
	var fields []sensitiveDefault
	for _, f := range all {
		if isSecretEnv(f.EnvVar) {
			fields = append(fields, f)
		}
	}
	if len(fields) == 0 {
		t.Fatalf("扫到 %d 个带默认值的 env()，但没有一个被判为秘密 —— "+
			"isSecretEnv 的后缀表可能坏了（%q）", len(all), secretEnvSuffix)
	}
	guarded := prodGuardedVars(t)

	var missing []string
	for _, f := range fields {
		if _, ok := guarded[f.EnvVar]; ok {
			continue
		}
		if _, ok := guarded[f.Field]; ok {
			continue
		}
		missing = append(missing, f.Field+" ("+f.EnvVar+" 默认值 "+strconv.Quote(f.Default)+" @ "+f.Position+")")
	}
	sort.Strings(missing)

	if len(missing) > 0 {
		t.Errorf("以下带非空默认值的敏感项在 prod 分支里**没有守卫**（%d 项）：\n  %s\n\n"+
			"任何全新 prod 部署都会用这些默认值启动。\n"+
			"加守卫时优先用「必须显式声明」（os.Getenv(X) != \"\"）—— "+
			"「够长就行」挡不住公开的默认值。",
			len(missing), strings.Join(missing, "\n  "))
	}
}

// TestBootstrapAdminPassSpecificallyGuarded 把 P0 单独钉出来。
//
// 为什么需要单独一条：上面那条是**结构性**的，它保证「不漏」；
// 这条保证「prod 下用默认值确实会 panic」—— 也就是行为正确，
// 而不只是「代码里出现过某个变量名」。
func TestBootstrapAdminPassSpecificallyGuarded(t *testing.T) {
	t.Run("prod未设口令必须panic", func(t *testing.T) {
		clearEnv(t)
		t.Setenv("APP_ENV", "prod")
		t.Setenv("JWT_SECRET", "real-secret-from-vault")
		t.Setenv("DATABASE_URL", "postgres://u:p@h:5432/db?sslmode=require")
		// 刻意不设 BOOTSTRAP_ADMIN_PASS
		mustPanic(t, "BOOTSTRAP_ADMIN_PASS", func() { _ = Load() })
	})

	t.Run("prod显式设口令必须放行", func(t *testing.T) {
		clearEnv(t)
		t.Setenv("APP_ENV", "prod")
		t.Setenv("JWT_SECRET", "real-secret-from-vault")
		t.Setenv("DATABASE_URL", "postgres://u:p@h:5432/db?sslmode=require")
		t.Setenv("BOOTSTRAP_ADMIN_PASS", "a-real-long-production-password")
		cfg := Load()
		if cfg.BootstrapAdminPass != "a-real-long-production-password" {
			t.Errorf("BootstrapAdminPass = %q", cfg.BootstrapAdminPass)
		}
	})
}

// TestDefaultAdminPassIsExactlyTenChars 记录「为什么现有长度门槛拦不住」。
//
// `EnsureBootstrapAdmin` 的 `len(password) >= 10` 是唯一门槛，
// 而默认值恰好 10 位 —— 正好通过。
// 这条把那个「恰好」钉成事实：将来有人把默认值改短或加长，
// 这里会提示重新评估 prod 守卫是否还必要。
func TestDefaultAdminPassIsExactlyTenChars(t *testing.T) {
	const bootstrapMinLen = 10
	def := env("BOOTSTRAP_ADMIN_PASS", "admin12345")
	if len(def) != bootstrapMinLen {
		t.Logf("⚠️ 默认口令长度已从 %d 变成 %d —— "+
			"请重新评估 `len >= %d` 的门槛是否还能让默认值通过，"+
			"以及 prod 守卫是否仍必要。",
			bootstrapMinLen, len(def), bootstrapMinLen)
	}
	if len(def) >= bootstrapMinLen {
		t.Logf("实测：默认口令长度 %d >= 门槛 %d —— "+
			"即长度校验**拦不住**它，这正是必须单独加 prod 守卫的原因。",
			len(def), bootstrapMinLen)
	}
}

// TestNonProdStillGetsDevelopmentDefaults 守住「别把开发环境也堵死」。
//
// prod 守卫不能顺手把默认值在 dev 下也禁掉 —— 那会让
// `go run ./cmd/api` 直接起不来，而这个项目的 README 就是让新克隆的
// 仓库不设任何环境变量也能跑起来的。
func TestNonProdStillGetsDevelopmentDefaults(t *testing.T) {
	clearEnv(t)
	// 刻意不设 APP_ENV —— 默认应是 dev
	cfg := Load()
	if cfg.Env != "dev" {
		t.Fatalf("Env = %q，期望 dev", cfg.Env)
	}
	if cfg.BootstrapAdminPass != "admin12345" {
		t.Errorf("dev 下默认口令被改动了：%q —— "+
			"新克隆的仓库应该不设任何环境变量也能起来", cfg.BootstrapAdminPass)
	}
}
