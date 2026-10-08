package httpapi

import (
	"errors"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"runtime"
	"sort"
	"strings"
	"testing"

	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/service"
)

// TestEverySentinelErrorIsClassified 保证每个哨兵错误都被 HTTP 层显式归类，
// 且**归类真的生效**。
//
// ## 为什么这道测试必须存在（第 58 轮）
//
// `isSettleRejection` 与 `failErr` 的映射表都是硬编码名单。
// 原来的 `TestSettleRejectionsAllMapTo422` 只遍历它自己那份名单 ——
// 它的注释里甚至写着「如果漏了某个 domain 错误，这个测试同样会绿」。
//
// 后果是实测出来的三个 500：
//
//	ErrHPLeftExceedsBase      上报剩余血量 > 关卡初始血量（结算路径）
//	ErrSlotBudgetExceeded     装备件数 > 槽位预算（loadout 路径）
//	ErrReplaySkillsMismatch   回放 S 段与构筑不符（第 56 轮新加的校验）
//
// 三个都是「用户输入被理解后拒绝」，却都落到兜底的 500：
// 客户端只能显示「服务内部错误」，排查会自然滑向「服务端坏了」，
// 而且它们混进服务故障告警，把真正的故障淹掉。
//
// ## 这道测试怎么做到「完备」
//
// 用 `go/parser` 把 `domain` 与 `service` 两个包里所有
// `ErrXxx = errors.New(...)` 的**顶层变量声明**扫出来，
// 要求每一个都落在下面三类之一：
//
//	settle   → 归入结算拒绝，并且 isSettleRejection(err) 必须为 true
//	general  → 归入通用分支（failErr 前四条）
//	exempt   → 在 exemptSentinels 里具名豁免，且必须写清理由
//
// ⚠️ 「在名单里」不等于「生效」。所以对 settle 类额外断言
// `isSettleRejection(err)`：只比名单的话，名单写了而 isSettleRejection 漏了，
// 这道守卫会绿而实际仍然返回 500 —— 装饰品就是这么伪装成守卫的。
func TestEverySentinelErrorIsClassified(t *testing.T) {
	settle := []error{
		domain.ErrTokenNotFound,
		domain.ErrTokenUsed,
		domain.ErrTokenExpired,
		domain.ErrLevelMismatch,
		domain.ErrTooManyKills,
		domain.ErrTooShort,
		domain.ErrScoreRateExceeded,
		domain.ErrInvalidHitRate,
		domain.ErrWaveExceeded,
		domain.ErrTooManyReactions,
		domain.ErrTooManyShots,
		domain.ErrTooManyLeaked,
		domain.ErrInvalidField,
		domain.ErrHPLeftExceedsBase,
		service.ErrSlotBudgetExceeded,
		service.ErrReplaySkillsMismatch,
	}
	general := []error{
		service.ErrUnauthorized,
		service.ErrForbidden,
		service.ErrNotFound,
		service.ErrBadInput,
	}

	// 具名豁免。理由为空也会红 —— 那等于没表态。
	// 目前为空：扫到的 20 个哨兵都能归类，不需要的豁免就不写。
	exempt := map[string]string{}

	declared := scanSentinelErrors(t)

	var missing, notRecognized, unregistered, badExempt []string

	for _, name := range declared {
		if reason, ok := exempt[name]; ok {
			if strings.TrimSpace(reason) == "" {
				badExempt = append(badExempt, name+"（豁免但没写理由）")
			}
			continue
		}
		err, ok := byName[name]
		if !ok {
			// 登记表缺它 —— TestSentinelRegistryMatchesSource 会报，这里也记一笔，
			// 因为「无法分类」和「已分类」是两种完全不同的失败。
			unregistered = append(unregistered, name)
			continue
		}
		switch {
		case inList(err, settle):
			if !isSettleRejection(err) {
				notRecognized = append(notRecognized,
					name+"（在结算名单里，但 isSettleRejection 不认 -> 会返回 500）")
			}
		case inList(err, general):
			// 通用类无需额外断言：failErr 的前四条分支用的是 errors.Is，
			// 只要在列表里就必然命中（列表就是那四个值本身）。
		default:
			missing = append(missing, name)
		}
	}

	for _, s := range [][]string{badExempt, notRecognized, unregistered, missing} {
		sort.Strings(s)
	}
	if len(badExempt) > 0 {
		t.Errorf("豁免必须具名且有据：\n  %s", strings.Join(badExempt, "\n  "))
	}
	if len(notRecognized) > 0 {
		t.Fatalf("这些哨兵在结算名单里，但 isSettleRejection 不认，实际仍会返回 500：\n  %s",
			strings.Join(notRecognized, "\n  "))
	}
	if len(unregistered) > 0 {
		t.Fatalf("这些哨兵没登记进 sentinelNames，无法分类：\n  %s\n"+
			"新增哨兵时要同时登记，否则这道守卫会误报而不是漏报。",
			strings.Join(unregistered, "\n  "))
	}
	if len(missing) > 0 {
		t.Fatalf("这些哨兵没有被 HTTP 层归类，会落到兜底的 500：\n  %s\n"+
			"要么加进 isSettleRejection（422），要么加进 failErr 的通用分支，"+
			"要么在 exempt 里具名豁免并写清理由。",
			strings.Join(missing, "\n  "))
	}
	t.Logf("扫到 %d 个哨兵错误，全部已归类且映射生效", len(declared))
}

// inList 判断 err 是否在 errs 里。哨兵是包级变量，按值比即可 ——
// 它们都是各自 errors.New 产生的唯一值。
func inList(err error, errs []error) bool {
	for _, e := range errs {
		if errors.Is(err, e) {
			return true
		}
	}
	return false
}

// byName 是 sentinelNames 的反向表：名字 -> 哨兵值。
var byName = func() map[string]error {
	m := make(map[string]error, len(sentinelNames))
	for err, name := range sentinelNames {
		m[name] = err
	}
	return m
}()

// sentinelNames 把哨兵值映射回它的变量名。
//
// 手写而不是反射：反射拿到的是 errors 包的内部类型，拿不到声明处的名字。
// 集中维护一份的代价是新增哨兵时要登记 —— 而新增时这道测试本来就要跑一遍，
// 漏登记会直接红（见 TestSentinelRegistryMatchesSource）。
var sentinelNames = map[error]string{
	domain.ErrTokenNotFound:         "domain.ErrTokenNotFound",
	domain.ErrTokenUsed:             "domain.ErrTokenUsed",
	domain.ErrTokenExpired:          "domain.ErrTokenExpired",
	domain.ErrLevelMismatch:         "domain.ErrLevelMismatch",
	domain.ErrTooManyKills:          "domain.ErrTooManyKills",
	domain.ErrTooShort:              "domain.ErrTooShort",
	domain.ErrScoreRateExceeded:     "domain.ErrScoreRateExceeded",
	domain.ErrInvalidHitRate:        "domain.ErrInvalidHitRate",
	domain.ErrWaveExceeded:          "domain.ErrWaveExceeded",
	domain.ErrTooManyReactions:      "domain.ErrTooManyReactions",
	domain.ErrTooManyShots:          "domain.ErrTooManyShots",
	domain.ErrTooManyLeaked:         "domain.ErrTooManyLeaked",
	domain.ErrInvalidField:          "domain.ErrInvalidField",
	domain.ErrHPLeftExceedsBase:     "domain.ErrHPLeftExceedsBase",
	service.ErrSlotBudgetExceeded:   "service.ErrSlotBudgetExceeded",
	service.ErrReplaySkillsMismatch: "service.ErrReplaySkillsMismatch",
	service.ErrUnauthorized:         "service.ErrUnauthorized",
	service.ErrForbidden:            "service.ErrForbidden",
	service.ErrNotFound:             "service.ErrNotFound",
	service.ErrBadInput:             "service.ErrBadInput",
}

// scanSentinelErrors 扫出 domain 与 service 包里所有哨兵错误的 `包名.变量名`。
//
// 用 AST 而不是正则：正则分不清
// `var ErrX = errors.New(...)`（真声明）与 `if err == ErrX`（使用），
// 也分不清注释里提到的同名。第一版用正则，结果把注释里的名字也算成了声明。
func scanSentinelErrors(t *testing.T) []string {
	t.Helper()
	_, thisFile, _, ok := runtime.Caller(0)
	if !ok {
		t.Fatal("runtime.Caller 失败，无法定位源码目录")
	}
	serverRoot := filepath.Join(filepath.Dir(thisFile), "..", "..")

	fset := token.NewFileSet()
	var out []string

	for _, pkg := range []struct{ dir, name string }{
		{filepath.Join(serverRoot, "internal", "domain"), "domain"},
		{filepath.Join(serverRoot, "internal", "service"), "service"},
	} {
		entries, err := os.ReadDir(pkg.dir)
		if err != nil {
			t.Fatalf("读目录 %s 失败：%v", pkg.dir, err)
		}
		for _, e := range entries {
			name := e.Name()
			if e.IsDir() || !strings.HasSuffix(name, ".go") || strings.HasSuffix(name, "_test.go") {
				continue
			}
			f, err := parser.ParseFile(fset, filepath.Join(pkg.dir, name), nil, 0)
			if err != nil {
				t.Fatalf("解析 %s 失败：%v", name, err)
			}
			for _, d := range f.Decls {
				gd, ok := d.(*ast.GenDecl)
				if !ok || gd.Tok != token.VAR {
					continue
				}
				for _, spec := range gd.Specs {
					vs, ok := spec.(*ast.ValueSpec)
					if !ok {
						continue
					}
					for _, id := range vs.Names {
						if !strings.HasPrefix(id.Name, "Err") {
							continue
						}
						if isSentinelInit(vs.Values) {
							out = append(out, pkg.name+"."+id.Name)
						}
					}
				}
			}
		}
	}
	sort.Strings(out)
	return out
}

// isSentinelInit 判断这一组变量是不是用 errors.New / fmt.Errorf 建的哨兵。
func isSentinelInit(values []ast.Expr) bool {
	for _, v := range values {
		ce, ok := v.(*ast.CallExpr)
		if !ok {
			continue
		}
		sel, ok := ce.Fun.(*ast.SelectorExpr)
		if !ok {
			continue
		}
		pkgID, ok := sel.X.(*ast.Ident)
		if !ok {
			continue
		}
		if (pkgID.Name == "errors" && sel.Sel.Name == "New") ||
			(pkgID.Name == "fmt" && sel.Sel.Name == "Errorf") {
			return true
		}
	}
	return false
}

// TestSentinelRegistryMatchesSource 保证 sentinelNames 登记的名字与源码里的声明一一对应。
//
// 分类检查靠 sentinelNames 反查，所以：
//   - 登记表漏了一个新哨兵 -> 误报「无法分类」
//   - 登记表多了一个已改名的 -> 分类悄悄失效
//
// 这条把两边钉死。
func TestSentinelRegistryMatchesSource(t *testing.T) {
	declared := scanSentinelErrors(t)
	declaredSet := map[string]bool{}
	for _, d := range declared {
		declaredSet[d] = true
	}

	for _, name := range sentinelNames {
		if !declaredSet[name] {
			t.Errorf("sentinelNames 登记了 %s，但源码里没有这个声明 —— "+
				"改名或删除后这里会红，避免分类悄悄失效", name)
		}
	}
	if len(sentinelNames) != len(declared) {
		t.Errorf("源码声明了 %d 个哨兵（%s），sentinelNames 只登记了 %d 个 —— "+
			"新增哨兵时要在这里登记",
			len(declared), strings.Join(declared, ", "), len(sentinelNames))
	}
}
