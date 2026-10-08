package domain

import (
	"go/ast"
	"go/parser"
	"go/token"
	"path/filepath"
	"runtime"
	"sort"
	"strconv"
	"strings"
	"testing"
)

// 攻方系数**不参与任何收益计算** —— 这条不变式的守卫（第 59 轮）。
//
// ## 为什么值得立守卫
//
// 回放前缀里的 `A` 段（攻方系数）一直挂着一条「已知边界：抓不到，
// 因为需要模拟」。这句话把一个**能力缺口**写成了问题，
// 好像补上 tick 循环移植就能解决。
//
// 量过之后的真相是：**补上也换不来任何东西。**
//
//	scoreCapFor(gl, sec)             只用关卡 MaxScore 与时长
//	MaxKillsFor(gl)                  只用关卡敌人总数
//	computeLoot(gl, stars, kills…)    只用关卡、星级、击杀
//	validateHPLeft(gl, hpLeft)       只用关卡初始血量
//
// 攻方一个都没进去。而结算能兑换的东西只有分数、星级、掉落、进度 ——
// 全都不看攻方。所以伪造 `A` 段换不来任何收益，
// 校验它只能证明「哈希与构筑一致」，抓不到任何能兑现的作弊。
//
// 留着这条边界描述而不立守卫，代价是：将来有人为了「修」它去移植
// 2,470 行 tick 循环，而那件事解决不了任何问题。
//
// ## 这道守卫做什么
//
// 用 AST 扫 `battle.go`，检查下面这些函数体**没有**引用任何攻方字段：
//
//	scoreCapFor / computeLoot / ComputeLoot / MaxKillsFor /
//	validateHPLeft / computeStars / fullStarTarget / ValidateSettle
//
// 有人把攻方接进收益路径时这里会红，并指出是哪个函数的哪一行。
//
// ## 为什么用 AST 而不是文本搜索
//
// 文本搜索会被注释与字符串命中 —— 而这个文件里关于攻方的**注释**比代码多
// （`A` 段的讨论、字段来源说明全在注释里）。第一版用 `strings.Contains`，
// 结果文件里随便提一句攻方就误报。
func TestAttackerNeverEntersRewardComputation(t *testing.T) {
	// 攻方字段名。来自 `domain.Attacker` 的实际字段。
	attackerFields := []string{
		"Attack", "CritPermille", "CritMultiplierPermille",
		"ReactionMultPermille", "ElementCap", "ReactionTier",
		"ElementCoefPermille", "ArmorPermille", "MechanicPermille",
		"HeatCapPermille",
	}

	// 必须保持「无攻方依赖」的函数。
	//
	// `DiagnoseFailure` 故意不在列表里：它的职责就是解释「为什么打不过」，
	// 读攻方是它的本职。它输出的是诊断文本，不参与任何发奖计算。
	rewardFns := []string{
		"scoreCapFor", "computeLoot", "ComputeLoot", "MaxKillsFor",
		"validateHPLeft", "computeStars", "fullStarTarget", "ValidateSettle",
	}

	_, thisFile, _, ok := runtime.Caller(0)
	if !ok {
		t.Fatal("runtime.Caller 失败")
	}
	path := filepath.Join(filepath.Dir(thisFile), "battle.go")

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, path, nil, 0)
	if err != nil {
		t.Fatalf("解析 battle.go 失败：%v", err)
	}

	// 收集所有顶层函数
	funcs := map[string]*ast.FuncDecl{}
	for _, d := range file.Decls {
		if fd, ok := d.(*ast.FuncDecl); ok && fd.Recv == nil {
			funcs[fd.Name.Name] = fd
		}
	}

	// ⚠️ 守卫自身的活性检查：下面这些必须真的被找到。
	// 否则「全部通过」可能只是因为函数改名了 —— 那是静默失效。
	var notFound []string
	for _, name := range rewardFns {
		if funcs[name] == nil {
			notFound = append(notFound, name)
		}
	}
	if len(notFound) > 0 {
		sort.Strings(notFound)
		t.Fatalf("这些函数在 battle.go 里找不到了：%v\n"+
			"要么改了名（请同步本守卫的名单），要么被删了（那本守卫就该跟着失效并重新论证）。",
			notFound)
	}

	var violations []string
	for _, name := range rewardFns {
		fd := funcs[name]
		// 检查：标识符使用、选择器字段、以及函数签名里的参数名/类型
		ast.Inspect(fd, func(n ast.Node) bool {
			switch v := n.(type) {
			case *ast.Ident:
				for _, f := range attackerFields {
					if v.Name == f {
						pos := fset.Position(v.Pos())
						violations = append(violations,
							name+"() 引用了攻方字段 "+f+"（"+path+":"+strconv.Itoa(pos.Line)+"）")
					}
				}
			case *ast.SelectorExpr:
				sel := v.Sel.Name
				for _, f := range attackerFields {
					if sel == f {
						pos := fset.Position(v.Sel.Pos())
						violations = append(violations,
							name+"() 读取了 ."+sel+"（"+path+":"+strconv.Itoa(pos.Line)+"）")
					}
				}
			}
			return true
		})
	}

	// 函数名里带 attacker 也不放过 —— 那是同一种依赖的另一种写法
	for name := range funcs {
		if strings.Contains(strings.ToLower(name), "attacker") {
			violations = append(violations,
				"battle.go 里出现了 "+name+"()：本文件属于收益计算路径，不该有攻方相关的函数")
		}
	}

	sort.Strings(violations)
	if len(violations) > 0 {
		t.Fatalf("攻方系数被接进了收益计算路径：\n  %s\n\n"+
			"这会让伪造 A 段变得有收益，从而必须配套完整的战斗模拟才防得住。\n"+
			"若这是有意为之，请连同「A 段可被校验」的结论一起改，并在此处留下理由。",
			strings.Join(violations, "\n  "))
	}

	t.Logf("已扫描 %d 个收益计算函数，均无攻方依赖", len(rewardFns))
}
