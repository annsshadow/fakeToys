package domain

// 文档漂移守卫：把设计文档里的数字与**代码**逐项比对。
//
// # 为什么需要它
//
// `docs/GAME_DESIGN.md` 曾经写着「本文件是实现的单一事实源」，
// 而 29 轮实现之后它已经**大幅漂移**：步进频率写成 60Hz（实际 20Hz）、
// 文件清单列了 5 个不存在的 TS 文件、API 列了 4 个不存在的端点、
// 敌人数写成 20（实际 21）、合成配方写成 24（实际 18）、
// 装备契合度被写成「已实现」而实际上**已决定不做**。
//
// 漂移本身不可怕，**漂移而无人知晓**才可怕 —— 运营、策划、新人
// 都会拿这份文档当依据。
//
// # 这条守卫的定位：守「人类写下的数字」与「代码里的数字」的关系
//
// 本项目已 11 次栽在「度量前提不成立」上。所以这里刻意说明本文件
// **不做什么**：
//
//   - 它不跑引擎、不验公式 —— 那是行为守卫的职责
//   - 它不做纯文本搜索 —— 「grep 文档里有没有 60Hz」会把
//     「文档提到 60Hz 但同时说明实际是 20Hz」也判成命中
//
// 它做的是**三方比对**：文档文本 ↔ TS 引擎常量 ↔ Go 常量。
// 有些常量只在 TS 侧（如步进频率），有些只在 Go 侧（如反应链数量），
// 所以两边都得读。
//
// # Skip 会被显式打印
//
// 找不到文件时 Skip 而不是 Fail（Go 单元测试不依赖工作目录），
// 但 Skip 消息里写明「这条守卫**没有执行**」——
// 「Skip 藏在全绿里」是本项目吃过 3 次的坑。

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
	"testing"
)

// readFileFrom 沿若干候选路径找一个文件。
func readFileFrom(t *testing.T, what string, rels ...string) string {
	t.Helper()
	for _, rel := range rels {
		for _, base := range []string{"", "..", filepath.Join("..", ".."), filepath.Join("..", "..", "..")} {
			p := filepath.Join(base, rel)
			if b, err := os.ReadFile(p); err == nil {
				return string(b)
			}
		}
	}
	t.Skipf("未找到 %s（试过 %s）—— 这条文档漂移守卫**没有执行**，不是「通过了」",
		what, strings.Join(rels, ", "))
	return ""
}

const designDocPath = "docs/GAME_DESIGN.md"

// tsConst 从 TS 源码抽 `export const NAME = <number>`。
//
// 为什么读源码而不复制数字：
// 复制一份数字到 Go 里，就多了一个「两处可能不同步」的面 ——
// 那正是本守卫要防的那类漂移，只不过搬到了守卫自己身上。
func tsConst(t *testing.T, src, name string) int {
	t.Helper()
	re := regexp.MustCompile(`export const ` + regexp.QuoteMeta(name) + `\s*=\s*(\d+)`)
	m := re.FindStringSubmatch(src)
	if m == nil {
		t.Fatalf("TS 源码里找不到 `export const %s = <数字>` —— "+
			"它可能被改名或删除，本守卫的锚点失效了（守卫本身漂移）", name)
	}
	n, err := strconv.Atoi(m[1])
	if err != nil {
		t.Fatalf("%s = %q 解析失败：%v", name, m[1], err)
	}
	return n
}

func TestGameDesignNumbersMatchCode(t *testing.T) {
	doc := readFileFrom(t, "设计文档 "+designDocPath, designDocPath)
	engine := readFileFrom(t, "引擎源码 miniapp/src/game/engine.ts",
		filepath.Join("miniapp", "src", "game", "engine.ts"))
	heatmap := readFileFrom(t, "卡牌/热量源码 miniapp/src/game/heatmap.ts",
		filepath.Join("miniapp", "src", "game", "heatmap.ts"))

	tickHz := tsConst(t, engine, "TICK_HZ")
	activeSlots := tsConst(t, heatmap, "ACTIVE_SLOTS")
	heatMax := tsConst(t, heatmap, "HEAT_MAX")
	overheatMs := tsConst(t, heatmap, "OVERHEAT_DURATION_MS")
	discardPerWave := tsConst(t, heatmap, "DISCARD_PER_WAVE")

	type check struct {
		label   string
		actual  string
		pattern *regexp.Regexp
		why     string
	}
	cases := []check{
		{
			label:   "引擎步进频率",
			actual:  strconv.Itoa(tickHz),
			pattern: regexp.MustCompile(`固定\s*\*\*(\d+)\s*Hz\*\*\s*步进`),
			why: "步进频率决定定点数每 tick 的位移。文档写 60Hz 而代码是 " +
				strconv.Itoa(tickHz) + "Hz，会让人按 3 倍频次推导伤害",
		},
		{
			label:   "主动插槽常量",
			actual:  strconv.Itoa(activeSlots),
			pattern: regexp.MustCompile(`ACTIVE_SLOTS\s*=\s*(\d+)`),
			why:     "这是重放哈希的锚点，改它会让所有历史战报失去可重放性",
		},
		{
			label:   "热量基础上限",
			actual:  strconv.Itoa(heatMax),
			pattern: regexp.MustCompile(`基础上限\s*\*\*(\d+)\*\*`),
			why:     "热量上限是 I-2「有代价的构筑」的核心数值",
		},
		{
			label:  "过热时长",
			actual: strconv.Itoa(overheatMs),
			// ⚠️ 文档写「2000ms（2s）」而不是「2s」。
			//
			// 守卫的正则抓的是**毫秒数**，与代码常量同单位。
			// 第一版文档只写「2s」，于是守卫报「文档写 2，代码是 2000」——
			// 那是**守卫的单位缺陷**而不是文档错误。
			//
			// 但改文档比改守卫好：设计文档只写「2s」时，
			// 读者无法把它和 `OVERHEAT_DURATION_MS = 2000` 对上；
			// 写出「2000ms（2s）」既保留可读性，又锚定了单位。
			pattern: regexp.MustCompile(`\*\*过热 (\d+)ms`),
			why:     "过热时长直接决定「高热量爆发流」是否可行",
		},
		{
			label:   "每波弃牌数",
			actual:  strconv.Itoa(discardPerWave),
			pattern: regexp.MustCompile(`每波可免费弃\s*\*\*(\d+)\*\*\s*张`),
			why:     "弃牌是 I-2 里唯一「回热量」的选择，它的次数是平衡参数",
		},
		{
			label:   "反应链数量",
			actual:  strconv.Itoa(len(AllReactionSpecs())),
			pattern: regexp.MustCompile(`恰好\s*(\d+)\s*条`),
			why:     "反应链数量是 I-1 的核心，「无唯一最优解」红线依赖它的连通性",
		},
		{
			label:   "元素数量",
			actual:  strconv.Itoa(len(AllElements())),
			pattern: regexp.MustCompile(`\*\*(\d+) 种元素\*\*`),
			why:     "元素数决定反应矩阵维度与构筑评分的「元素覆盖」满分",
		},
		{
			label:   "专精层内可选上限",
			actual:  strconv.Itoa(PerLayerPickLimit),
			pattern: regexp.MustCompile(`4 选 (\d+)`),
			why:     "「不可能点满专精树」这条设计意图全靠它成立",
		},
	}

	var bad []string
	for _, c := range cases {
		m := c.pattern.FindStringSubmatch(doc)
		if m == nil {
			bad = append(bad, fmt.Sprintf("%s：文档里找不到匹配 %q 的表述",
				c.label, c.pattern))
			continue
		}
		if m[1] != c.actual {
			bad = append(bad, fmt.Sprintf("%s：文档写 %s，代码是 %s —— %s",
				c.label, m[1], c.actual, c.why))
		}
	}
	if len(bad) > 0 {
		t.Fatalf("设计文档与代码有 %d 处数字漂移：\n  %s",
			len(bad), strings.Join(bad, "\n  "))
	}
}

// TestGameDesignDeclaresKnownGaps 确认文档**主动声明**了未实现的项。
//
// 为什么这条与「数字准确」同等重要：
//
// 一份把「装备契合度」写成已实现的设计文档，比一份没有这段的文档
// 危害更大 —— 没人会去核对它。本项目已经因为
// 「18 件装备 10 条描述在骗玩家」吃过一次亏，性质完全相同。
func TestGameDesignDeclaresKnownGaps(t *testing.T) {
	doc := readFileFrom(t, "设计文档 "+designDocPath, designDocPath)

	idx := strings.Index(doc, "已知边界（设计意图 vs 实现现状）")
	if idx < 0 {
		t.Fatal("设计文档缺少「已知边界（设计意图 vs 实现现状）」章节 —— " +
			"它必须列出设计意图与实现现状的差异，否则读者会把未实现的机制当成已实现")
	}
	tail := doc[idx:]

	mustDeclare := []struct{ keyword, why string }{
		{"装备契合度", "它曾被文档写成已实现，实际全项目从未读取装备的 element 字段"},
		{"DoT tick", "反应表只有控制时长，若不声明，读者会以为有持续伤害 tick"},
		{"唯一最优解", "红线 1 声明了，但没有对应的穷举测试"},
		{"阶", "原设计的「1/2/3 阶升格」不存在，不声明会让人去找一个没有的界面"},
	}
	var missing []string
	for _, m := range mustDeclare {
		if !strings.Contains(tail, m.keyword) {
			missing = append(missing, fmt.Sprintf("%q —— %s", m.keyword, m.why))
		}
	}
	if len(missing) > 0 {
		t.Fatalf("「已知边界」章未声明以下未实现项：\n  %s",
			strings.Join(missing, "\n  "))
	}
}

// TestDesignDocIsNotClaimedAsSourceOfTruth 确认文档不再自称「单一事实源」。
//
// 这条是**字面**检查，但它守的是一个真实发生过的失败模式：
// 文档开头那句「本文件是实现的单一事实源」让所有读者都跳过了核对，
// 于是 20+ 处漂移在 29 轮里无人发现。
//
// 当漂移可以被工具发现时，称自己为「事实源」就是有害的。
func TestDesignDocIsNotClaimedAsSourceOfTruth(t *testing.T) {
	doc := readFileFrom(t, "设计文档 "+designDocPath, designDocPath)
	if strings.Contains(doc, "本文件是实现的单一事实源") {
		t.Fatal("设计文档仍自称「实现的单一事实源」—— " +
			"它已经有 20+ 处与代码不符，自称事实源会让读者跳过核对。" +
			"实现的单一事实源是代码与 server/testdata/ 的契约夹具")
	}
	// 反向：必须**明确指向**真正的单一事实源
	if !strings.Contains(doc, "契约夹具") {
		t.Error("设计文档未指向真正的单一事实源（server/testdata/ 的契约夹具）")
	}
}
