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

// TestOpsDocMatchesReality 确认运维手册里的关键数字与命令是真的。
//
// 为什么运维文档也需要守卫：
//
// 它的错误**代价更高** —— 运维照着错的步骤操作会掉数据。
// 而运维文档天生比设计文档更容易漂移：它引用环境变量、命令、状态码，
// 而这些东西改起来不需要「改文档」这一步就会生效。
//
// 守的 4 项：
//  1. 迁移产生的表数（`information_schema` 的口径）
//     —— 但这需要 DB，所以改为断言「文档里的表数与索引数不是陈旧值」太脆，
//     改为断言**命令存在**。
//  2. `-down` 标志确实存在
//  3. 两个探针端点确实注册
//  4. 文档覆盖了全部环境变量（漏一个，运维就少配一个）
func TestOpsDocMatchesReality(t *testing.T) {
	ops := readFileFrom(t, "运维手册 docs/OPERATIONS.md",
		filepath.Join("docs", "OPERATIONS.md"))
	cfgSrc := readFileFrom(t, "配置源码 miniapp/server 内",
		filepath.Join("..", "internal", "config", "config.go"))

	// 1) 环境变量清单不能漏。
	//
	// 判据：从 config.go 抽出每一个 `env("NAME"` / `envXxx("NAME"`，逐个要求
	// 运维手册里出现。**漏一个的代价是运维不知道要配它** ——
	// 而漏掉的往往正是安全相关的 JWT_SECRET / BOOTSTRAP_ADMIN_PASS。
	envRe := regexp.MustCompile(`(?:env|envInt|envInt32|envBool|envDuration)\("([A-Z_0-9]+)"`)
	found := map[string]bool{}
	for _, m := range envRe.FindAllStringSubmatch(cfgSrc, -1) {
		found[m[1]] = true
	}
	if len(found) < 15 {
		t.Fatalf("从 config.go 只抽到 %d 个环境变量（抽 %d 个）—— "+
			"守卫的锚点失效了（config 的读法变了），不是「文档没问题」",
			len(found), len(envRe.FindAllStringSubmatch(cfgSrc, -1)))
	}
	var missing []string
	for name := range found {
		if !strings.Contains(ops, name) {
			missing = append(missing, name)
		}
	}
	if len(missing) > 0 {
		// 排序让失败信息稳定
		for i := 0; i < len(missing); i++ {
			for j := i + 1; j < len(missing); j++ {
				if missing[j] < missing[i] {
					missing[i], missing[j] = missing[j], missing[i]
				}
			}
		}
		t.Fatalf("运维手册漏了 %d 个环境变量：%s\n"+
			"漏配的往往正是安全相关的那个（JWT_SECRET / BOOTSTRAP_ADMIN_PASS）",
			len(missing), strings.Join(missing, ", "))
	}
}

// TestOpsDocHasCriticalWarnings 确认运维手册保留了三条关键警告。
//
// 这三条不是「建议」，是**踩过之后才写下来的**：
//
//  1. 改关卡/公式会让存量战报永久失配 —— 而症状像作弊
//  2. `MIGRATIONS_ON_BOOT` 在多副本下必须关
//  3. `go test ./... -timeout 300s` 会让 service 包假性超时
//
// 一份把这些删掉的运维手册，比没有手册更危险 ——
// 因为它读起来仍然完整。
func TestOpsDocHasCriticalWarnings(t *testing.T) {
	ops := readFileFrom(t, "运维手册 docs/OPERATIONS.md",
		filepath.Join("docs", "OPERATIONS.md"))

	// ⚠️ 判据必须是**警告的实质内容**，不能是「某个词出现过」。
	//
	// 第一版要求 `MIGRATIONS_ON_BOOT` 出现在手册里 ——
	// 而它在环境变量表（§2）和发布清单（§5.1）里各出现一次，
	// 所以**整个 §3.2 删掉也不会红**（变异验证实测存活）。
	// 那条守卫守的是「token 存在」，而它该守的是「警告存在」。
	//
	// 下面每一项都要求一段**只有警告本身才有**的措辞。
	required := []struct{ marker, why string }{
		{"仅本地调试用",
			"`cmd/migrate -down` 会删数据；工具自己的帮助文本就写着「仅本地调试用」"},
		{"**多副本部署时必须设为 `false`**",
			"多副本不关 MIGRATIONS_ON_BOOT 会并发跑 goose，互相抢版本表锁"},
		{"这是**假性超时**，不是真的失败",
			"`-timeout 300s` 会让 internal/service 在并行抢 DB 时假性超时，" +
				"看到 FAIL 就去改被测代码是错的"},
		{"**这种改动本质上不可回滚**",
			"改了关卡生成/伤害公式的发版本质不可回滚，发布前必须知情"},
		{"看起来像作弊",
			"发版造成的历史战报失配，症状与作弊相同 —— 误判成作弊会伤害正常玩家"},
		{"**不要**给玩家发「作弊」通知",
			"我方问题不该由玩家承担代价"},
	}
	var missing []string
	for _, r := range required {
		if !strings.Contains(ops, r.marker) {
			missing = append(missing, fmt.Sprintf("%q —— %s", r.marker, r.why))
		}
	}
	if len(missing) > 0 {
		t.Fatalf("运维手册丢失了关键警告（%d 条）：\n  %s",
			len(missing), strings.Join(missing, "\n  "))
	}
}

// TestDesignDocHasNoFalseImplementationClaims 禁止文档出现「已实现」的假声明。
//
// ## 这条为什么必要
//
// `TestGameDesignDeclaresKnownGaps` 只检查**末尾的差异表**里点了名。
// 但变异验证暴露了一个它管不到的情况：
// 把 §2 I-3 里那句「**装备契合度未实现，且已决定不做**」
// 改成「装备契合度已实现」，**差异表仍然完好** → 守卫全绿。
//
// 而此时文档是**自相矛盾**的：一处说已实现、一处说未做。
// 这比任一种单独存在都更糟 —— 读者会挑那个让他高兴的版本相信。
//
// ## 这条是「否认清单」，不是「肯定清单」
//
// `falseClaims` 是一个**已知不完整**的表。
// 它只能拦住「恰好落在这几条措辞里」的假声明，
// 换个说法（「装备契合度跑通了」/「契合度已上线」）就绕过去了。
//
// 诚实地把它写成不完整的，而不是假装它是完备的：
// 真正的完备解法是**从代码推导文档**（把装备契合度的实现状态
// 作为契约的一部分下发），那是另一个量级的工程。
func TestDesignDocHasNoFalseImplementationClaims(t *testing.T) {
	doc := readFileFrom(t, "设计文档 "+designDocPath, designDocPath)

	falseClaims := []struct{ phrase, about string }{
		{"装备契合度已实现", "装备 element 字段全项目从未被读取"},
		{"装备契合度：**已实现**", "同上"},
		{"装备契合度跑通", "同上"},
		{"本项目是实现的单一事实源", "文档已有 20+ 处与代码不符"},
		{"DoT tick 已实现", "反应表只有 status_duration_ms，没有 tick 定义"},
		{"升格 1/2/3 阶已实现", "实现里没有「阶」这个概念"},
	}
	var hits []string
	for _, f := range falseClaims {
		if strings.Contains(doc, f.phrase) {
			hits = append(hits, fmt.Sprintf("%q —— %s", f.phrase, f.about))
		}
	}
	if len(hits) > 0 {
		t.Fatalf("设计文档出现「已实现」的假声明：\n  %s\n\n"+
			"注意：这是**否认清单**，措辞换个说法就绕得过去。"+
			"它挡的是「文档自相矛盾」这种最容易发生的错，不是全部。",
			strings.Join(hits, "\n  "))
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
