package httpapi

// 端点消费面守卫：服务端挂的每条路由都必须有客户端调用方。
//
// ## 为什么需要它
//
// 「端点存在但没人调」在本仓库已经发生过多轮：
//
//   - `POST /mastery/reset`（第 74 轮就有，第 150 轮才被客户端接上）
//   - `GET /levels/:id`（一直零消费）
//   - `POST /auth/wechat`（需要外部凭证，属产品门禁）
//
// 症状不是报错，而是「端点逻辑写得挺完整、测试覆盖率还很高，
// 但线上永远没人走那条路」。覆盖率在这里是**反向指标**。
//
// ## 口径
//
// 1. 从 `Register` 的源码里正则抽出全部路由与方法；
// 2. 在 `miniapp/src` 与 `admin/src` 的**产品源码**里找调用方。
//
// 已知无法闭合的端点写进 `KNOWN_UNSERVED`，且必须写明具体理由；
// 理由不达标（太短 / 不具体）会在 `TestEveryExemptionIsJustified` 里红。

import (
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"
)

// KNOWN_UNSERVED 是「服务端有、客户端不可能调」的端点。
//
// 新增条目必须回答：这个端点为什么在本仓库里接不上？
// 「暂时没空接」不算理由 —— 那会让缺口永久合法化。
var KNOWN_UNSERVED = map[string]string{
	"POST /api/v1/auth/wechat": "微信登录通道受 WECHAT_ENABLED 开关与 AppID/Secret 外部凭证门禁。" +
		"未配置时服务端返回明确的 501 而不是静默降级成游客登录；客户端接上它需要真机 + 真凭证，" +
		"在本仓库的自动化验证里无法成立（见 README「已知边界」第 2 条）。",
	"GET /metrics": "Prometheus 指标端点：消费方是仓库**外**的抓取器（Prometheus/Grafana），" +
		"本仓库没有任何客户端会调它；且受 METRICS_ENABLED 开关门禁、默认不注册" +
		"（见 docs/laiyipao/OPERATIONS.md §2 环境变量表）——接上它需要外部抓取基础设施，" +
		"不在本仓验证面内。",
}

var routeRe = regexp.MustCompile(`([A-Za-z_][A-Za-z0-9_]*)\.(Get|Post|Put|Patch|Delete)\("([^"]*)"`)
var groupRe = regexp.MustCompile(`([A-Za-z_][A-Za-z0-9_]*)\s*:?=\s*([A-Za-z_][A-Za-z0-9_]*)\.Group\("([^"]*)"`)

// serverRoutes 从 routes.go 重建**完整**路径的路由表。
//
// 为什么不能直接抓 `X.Get("/guest")` 里的字面量：那是**组内**片段，
// 单看它既不完整也不唯一（`/admin/...` 与玩家端的 `/...` 会混在一起）。
// 而「端点没被消费」这个问题只有在**完整路径**上才说得清 ——
// 玩家端的 `/shop` 和后台的 `/admin/shop` 是两个端点。
//
// 做法：顺序扫 `Register`，维护「变量名 → 组前缀」映射，
// `v1 := app.Group("/api/v1")` → v1=/api/v1；`me := v1.Group("/me", ...)` → me=/api/v1/me。
// httpMethods 把 fiber 的方法名映射成 HTTP 动词。
//
// ⚠️ 别用 `ToUpper(s[:1]) + ToLower(s[1:])`：那会得到 "Get"（G 大写、et 小写）。
// 那是个**看起来对**的转换 —— 它第一次跑就以「Get /api/v1/xxx」的形式
// 出现在失败信息里，而没人会去追究一个方法名的大小写，
// 于是后面真正的缺口被这条噪声盖住了。
var httpMethods = map[string]string{
	"Get": "GET", "Post": "POST", "Put": "PUT", "Patch": "PATCH", "Delete": "DELETE",
}

func serverRoutes(t *testing.T) [][2]string {
	t.Helper()
	src, err := os.ReadFile("routes.go")
	if err != nil {
		t.Fatalf("读 routes.go 失败：%v", err)
	}
	prefixes := map[string]string{"app": ""}
	var out [][2]string
	for _, line := range strings.Split(string(src), "\n") {
		if g := groupRe.FindStringSubmatch(line); g != nil {
			base := prefixes[g[2]]
			prefixes[g[1]] = base + g[3]
			continue
		}
		m := routeRe.FindStringSubmatch(line)
		if m == nil {
			continue
		}
		method, known := httpMethods[m[2]]
		if !known {
			t.Fatalf("routes.go 里有未知的方法名 %q（第 %s 行）—— "+
				"守卫的方法映射表要同步，否则这条路由会被静默漏掉", m[2], line)
		}
		base, known := prefixes[m[1]]
		if !known {
			continue // 不是组变量（极少数），不纳入本守卫
		}
		out = append(out, [2]string{method + " " + base + m[3], method})
	}
	if len(out) == 0 {
		t.Fatal("一条路由都没扫到 —— 守卫本身失效了（routes.go 结构变了？）")
	}
	return out
}

// clientSource 收集两个前端的**产品源码**（排除测试文件）。
//
// 排除测试文件是有意的：测试里的 URL 拼装不算「有人用这个端点」——
// 那恰好是「写了测试但线上没人调」最常见的伪装。
func clientSource(t *testing.T) string {
	t.Helper()
	repo := serverRoot(t)
	var b strings.Builder
	for _, sub := range []string{"laiyipao/miniapp/src", "laiyipao/admin/src"} {
		root := filepath.Join(repo, sub)
		err := filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return nil
			}
			if info.IsDir() {
				if name := info.Name(); name == "node_modules" || name == "dist" {
					return filepath.SkipDir
				}
				return nil
			}
			if !strings.HasSuffix(path, ".ts") && !strings.HasSuffix(path, ".vue") {
				return nil
			}
			if strings.HasSuffix(path, ".test.ts") || strings.Contains(path, string(filepath.Separator)+"test"+string(filepath.Separator)) {
				return nil
			}
			data, err := os.ReadFile(path)
			if err != nil {
				return nil
			}
			b.Write(data)
			b.WriteString("\n")
			return nil
		})
		if err != nil {
			t.Fatalf("遍历 %s 失败：%v", sub, err)
		}
	}
	if b.Len() == 0 {
		t.Fatal("没扫到任何客户端源码 —— 守卫本身失效了（路径变了？）")
	}
	return b.String()
}

// callerRE 把路由模板编译成「客户端源码里该长什么样」的匹配式。
//
// 形态：`/tasks/:task_id/claim` → `/tasks/[^/'"`?&#]+/claim`
// 且后面必须紧跟一个**终止符**（引号 / 反引号 / `?` / `&` / 行尾）。
//
// ## 为什么必须有终止符
//
// 第一版只判断「路径片段出现在源码里」，于是 `/redeem` 的调用方
// `'/redeem'` 会**顺带**满足 `/redeem/never-called` 的检查 ——
// 也就是说把一条路由改成任何更深的路径，守卫都照样绿。
//
// 那是这条守卫最坏的失效方式：**看起来在守着，实际什么也没守**。
// 变异实测：把 `redeem.Post("/")` 改成 `redeem.Post("/never-called")`，
// 守卫仍然通过。终止符就是为堵这个而加的。
//
// ## 为什么参数段用 `[^/'"`?&#]+`
//
// 客户端写成 `/tasks/${taskId}/claim`（字面串）或
// `/levels/${id}/replay`；而 `/mastery/reset` 是纯字面量。
// 参数段不能跨分隔符，否则 `/a/:x` 会去匹配 `/a/b/c`。
var paramPattern = regexp.MustCompile(`:[A-Za-z_][A-Za-z0-9_]*`)

// paramSep 是「路由参数段」在客户端源码里可能长成的样子。
// 不能跨分隔符，否则 `/a/:x` 会去匹配 `/a/b/c`。
var paramSep = `[^/'"` + "`" + `?&#]+`

func callerRE(routePath string) *regexp.Regexp {
	// 归一化尾部斜杠：`me.Get("/", ...)` 注册出来的是 `/api/v1/me/`，
	// 而客户端写的是 `'/me'`。不归一化的话每个「组根路径」都会被误报成零消费。
	p := strings.TrimSuffix(routePath, "/")
	parts := paramPattern.Split(p, -1)
	var b strings.Builder
	for i, seg := range parts {
		if i > 0 {
			b.WriteString(paramSep)
		}
		b.WriteString(regexp.QuoteMeta(seg))
	}
	// 终止符：引号 / 反引号 / `?` / `&` / 空白 / 右括号 / 逗号 / 分号 / `}`
	b.WriteString("([\"'`?&\\s),;}])")
	return regexp.MustCompile(b.String())
}

// hasCaller 判断客户端源码里是否有该路由的调用。
func hasCaller(src, path string) bool {
	path = strings.SplitN(path, "?", 2)[0]
	return callerRE(path).MatchString(src)
}

// TestEveryRouteHasClientCaller 每条路由都要有客户端调用方或写明豁免。
func TestEveryRouteHasClientCaller(t *testing.T) {
	src := clientSource(t)
	var orphans []string
	for _, r := range serverRoutes(t) {
		key := r[0] // "GET /api/v1/config"
		method, path, _ := strings.Cut(key, " ")
		if strings.HasPrefix(path, "/healthz") || strings.HasPrefix(path, "/readyz") {
			continue // 探活端点，本来就不该被客户端调
		}
		if _, ok := KNOWN_UNSERVED[key]; ok {
			continue
		}
		// 客户端的 BASE_URL 已经含 /api/v1，所以只匹配后半段
		if !hasCaller(src, strings.TrimPrefix(path, "/api/v1")) {
			orphans = append(orphans, method+" "+path)
		}
	}
	if len(orphans) > 0 {
		t.Errorf("这些端点没有任何客户端调用方：\n  %s\n\n"+
			"端点存在、逻辑完整、还可能测到了，但线上没人走那条路 —— "+
			"覆盖率在这里是反向指标。要么接上，要么写进 KNOWN_UNSERVED 并说明原因。",
			strings.Join(orphans, "\n  "))
	}
}

// TestEveryExemptionIsJustified 豁免必须写具体理由，且理由里要有可核对的锚点。
//
// 判据给得比较硬，是因为豁免是这条守卫唯一的漏出口：
// 写「暂时没空」等于把缺口永久合法化，而没有任何测试会再提醒。
func TestEveryExemptionIsJustified(t *testing.T) {
	routes := make(map[string]bool)
	for _, r := range serverRoutes(t) {
		routes[r[0]] = true
	}
	for key, why := range KNOWN_UNSERVED {
		if !routes[key] {
			t.Errorf("KNOWN_UNSERVED 里有 %q，但它不是一条真实路由 —— 端点被删了，豁免没删", key)
		}
		if len([]rune(why)) < 40 {
			t.Errorf("端点 %q 的豁免理由只有 %d 字，太短了。"+
				"它必须回答「为什么在本仓库接不上」并给出可核对的锚点"+
				"（开关名 / 环境变量 / 文档章节），否则缺口会被永久合法化。",
				key, len([]rune(why)))
		}
	}
}

// TestRouteScannerStillFindsRoutes 守卫自身的有效性自检。
//
// 上一条会检查「路由是否被删」，但没有检查「正则是否还能扫到东西」——
// 两者都失效时，`TestEveryRouteHasClientCaller` 会因为 orphans 为空而**全绿**。
func TestRouteScannerStillFindsRoutes(t *testing.T) {
	routes := serverRoutes(t)
	if len(routes) < 40 {
		t.Errorf("只扫到 %d 条路由，预期 40+ —— 正则可能已经匹配不到新写法了", len(routes))
	}
}

func serverRoot(t *testing.T) string {
	t.Helper()
	dir, err := os.Getwd() // <repo>/laiyipao/server/internal/httpapi
	if err != nil {
		t.Fatal(err)
	}
	// 往上找到同时含 laiyipao/server 与 laiyipao/miniapp 的那一层，
	// 这样它既能定位前端源码，也能确认自己在预期位置。
	for i := 0; i < 6; i++ {
		if isDir(filepath.Join(dir, "laiyipao", "server")) && isDir(filepath.Join(dir, "laiyipao", "miniapp")) {
			return dir
		}
		dir = filepath.Dir(dir)
	}
	t.Fatalf("从当前目录往上找不到仓库根（含 laiyipao/server 与 laiyipao/miniapp）")
	return ""
}

func isDir(p string) bool {
	st, err := os.Stat(p)
	return err == nil && st.IsDir()
}
