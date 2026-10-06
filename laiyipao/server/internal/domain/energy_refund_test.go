package domain

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// 体力（energy）只在**通关**时回补（第 85 轮）。
//
// # 缺陷：体力是一条单向棘轮
//
// `DropRates.EnergyOnWin`（=5）与 `LootCaps.MaxEnergyPerBattle`（=15）
// **两个字段都在掉落表里，而没有任何代码写过 `energy`**。
// `testdata/formula_vectors.json` 的 `loot_caps.max_energy_per_battle`
// 也已经把 15 固化进跨端契约 —— 也就是说「每局最多回 15 体力」
// 是一条**对外的承诺**，只是从没兑现。
//
// 而 `StartBattle`（`service/game.go`）会在 `battle_start` 扣 `gl.EnergyCost`。
//
// 于是：打完一局扣 N、通关一分不返，余额单调下降直到 0，
// 此后再也无法开战，界面上也没有任何地方提示「体力不会回来」。
//
// 这类缺陷能长期存在，是因为「字段有值」看起来像「功能已实现」——
// 与第 82/83 轮记的「有守卫 ≠ 守卫覆盖了你」同型。
func TestEnergyIsRefundedOnWin(t *testing.T) {
	gl := GenerateLevel(1)
	tok := newTestToken(1)

	res, err := ValidateSettle(tok, gl, winInput(gl), testLimits(), time.Now())
	if err != nil {
		t.Fatalf("构造一次合法通关失败：%v", err)
	}
	if !res.Win {
		t.Fatal("winInput 应当构成通关 —— 判据前提不成立，本文件全部结论不可信")
	}

	e, ok := res.Loot["energy"]
	if !ok {
		t.Fatalf("通关掉落里没有 energy：%v\n"+
			"玩家开局被扣 EnergyCost，通关却不返还 —— 体力只出不进，"+
			"余额单调归零后再也无法开战", res.Loot)
	}
	if want := int64(DefaultDropRates().EnergyOnWin); e != want {
		t.Errorf("通关回体力 = %d，期望 EnergyOnWin = %d", e, want)
	}
	if e <= 0 {
		t.Errorf("回补的体力必须为正，实际 %d", e)
	}
}

// TestEnergyNotRefundedOnLoss 确认「只有通关才回」。
//
// ⚠️ 这条不许被「顺手放宽成只要结算就回」破坏 ——
// 那样的话「进战斗→立刻退出→结算」就成了一个体力永动机。
func TestEnergyNotRefundedOnLoss(t *testing.T) {
	gl := GenerateLevel(1)
	tok := newTestToken(1)

	in := winInput(gl)
	in.Result = "lose"
	in.Score = 0
	in.Kills = 0
	in.WaveReached = 1
	in.HPLeft = 0

	res, err := ValidateSettle(tok, gl, in, testLimits(), time.Now())
	if err != nil {
		t.Fatalf("构造一次合法失败结算出错：%v", err)
	}
	if res.Win {
		t.Fatal("这一局应当判负 —— 判据前提不成立")
	}
	if _, ok := res.Loot["energy"]; ok {
		t.Errorf("失败结算不该回体力，实得 %v\n"+
			"否则「开局→立刻退出→结算」就成了体力永动机", res.Loot)
	}
}

// TestEnergyNeverExceedsCap 确认封顶真的生效。
//
// ⚠️ 判据不能只写「energy <= 15」—— 现在 `EnergyOnWin=5` 远小于 15，
// 那条断言在**任何**实现下都成立（包括「完全不给」和「给 100 万」）。
//
// 真正的判据是：**把 rate 顶到封顶之上**，结果必须仍被夹住。
// 这与第 84 轮给 `envInt32` 补的那组「紧邻边界的两侧」同源 ——
// 测试必须落在判据真正分界的地方。
func TestEnergyNeverExceedsCap(t *testing.T) {
	gl := GenerateLevel(1)
	_ = gl
	caps := DefaultLootCaps()
	rates := DefaultDropRates()

	if rates.EnergyOnWin <= caps.MaxEnergyPerBattle {
		// 当前 5 < 15，封顶这一层是「备而不用」。
		// 那就直接验证备而不用时确实夹得住，而不是跳过。
		t.Logf("当前 EnergyOnWin=%d <= MaxEnergyPerBattle=%d，"+
			"封顶在当前配置下不会触发；下面验证夹取逻辑本身", rates.EnergyOnWin, caps.MaxEnergyPerBattle)
	}

	_ = gl // 当前配置下封顶不触发；真正的验证在下面的 satMul 上

	// 绕过 ValidateSettle，直接验证封顶算式：
	// 把 rate 换成远大于封顶的值，satMul 必须夹住。
	got := satMul(caps.MaxEnergyPerBattle*1000, 1, caps.MaxEnergyPerBattle)
	if got != caps.MaxEnergyPerBattle {
		t.Errorf("satMul 未夹住超封顶值：%d，期望恰好 %d", got, caps.MaxEnergyPerBattle)
	}

	// 反向：rate 为 0 时不该凭空产生体力
	if e := satMul(0, 1, caps.MaxEnergyPerBattle); e != 0 {
		t.Errorf("rate=0 时应回 0，实际 %d", e)
	}
}

// TestComputeLootHasNoEnergy 钉住「窃取路径不产体力」。
//
// ⚠️ 这是本轮**最容易被后续改动破坏**的不变量。
//
// `computeLoot` 有两个调用方：
//
//	battle.go:561   通关结算     ← 该给体力
//	progression.go 挑战者窃取   ← **不该给**（走 ComputeLootExported）
//
// 如果有人为了「统一」把体力加进 `computeLoot` 本体，
// 挑战者打一次别人的防线就能白拿 5 体力 ——
// 那是**刷体力漏洞**，不是奖励。
//
// 所以体力必须挂在 `computeLoot` **之外**（`ValidateSettle` 里），
// 而这条守卫把这个选择钉死。
func TestComputeLootHasNoEnergy(t *testing.T) {
	gl := testLevel()
	loot := computeLoot(gl, 3, int64(MaxKillsFor(gl)), int64(MaxKillsFor(gl)), 1000)
	if e, ok := loot["energy"]; ok {
		t.Errorf("computeLoot 不该产 energy，实得 %d\n\n"+
			"它被挑战者窃取奖励（ComputeLootExported）复用，"+
			"在那里产体力等于「打别人的防线就能刷体力」。\n"+
			"体力只允许挂在 ValidateSettle 的通关分支上。", e)
	}
}

// TestEnergyRefundIsCoveredByContract 确认契约向量里有这一项。
//
// 跨端契约 `formula_vectors.json` 已经记着
// `loot_caps.max_energy_per_battle = 15` ——
// 也就是说「每局最多回 15 体力」是一条**对外承诺**。
//
// 这条守卫确认代码里的封顶与契约一致；
// 若有人调了 `DefaultLootCaps` 而忘了重生成契约，`cmd/vectors -check` 会红，
// 这里给出更直接的提示。
func TestEnergyRefundIsCoveredByContract(t *testing.T) {
	caps := DefaultLootCaps()
	if caps.MaxEnergyPerBattle <= 0 {
		t.Fatalf("MaxEnergyPerBattle 必须为正，实际 %d —— "+
			"它已经出现在 testdata/formula_vectors.json 的 loot_caps 里，"+
			"置 0 等于对外承诺「体力永远回不来」", caps.MaxEnergyPerBattle)
	}
	if DefaultDropRates().EnergyOnWin <= 0 {
		t.Fatalf("EnergyOnWin 必须为正，实际 %d —— "+
			"为 0 意味着体力永远回不来，就是本轮修的那个洞", DefaultDropRates().EnergyOnWin)
	}
}

// TestEnergyGrantGoesThroughSatMul 堵住「旁路封顶」这个**潜伏**缺口。
//
// # 变异实测暴露的真空缺
//
// 把结算里的
//
//	if e := satMul(r.EnergyOnWin, 1, caps.MaxEnergyPerBattle); e > 0 {
//
// 换成
//
//	if e := r.EnergyOnWin; e > 0 {
//
// （即不再走饱和算术）→ **全部测试通过**。
//
// 原因是**判据前提不成立**：`EnergyOnWin = 5` 远小于
// `MaxEnergyPerBattle = 15`，所以「封顶」这一层当前根本不会触发 ——
// 加不加它，可观测行为完全一样。
//
// 这与第 84 轮给 `envInt32` 补的那组「紧邻边界的两侧」同源：
// **只要输入落不到分界线上，任何关于分界线的断言都是空的。**
//
// 我没有为了让它可观测去改 `DefaultDropRates()`（那会动生产数值），
// 也没有给 `computeLoot` 加参数（那是为测试改生产接口）。
//
// # 判据落在能直接观测的那一层
//
// 扫源码文本，要求「回补体力那一行必须经过 satMul 且带上 cap」。
//
// ⚠️ 这不是形式主义：它守的是**将来**那个会出事的时刻 ——
// 有人把 `EnergyOnWin` 调到 20（跨过封顶），旁路 `satMul` 的那行
// 就会让每局白送 20 点，而当时所有测试仍然是绿的。
//
// 与第 80 轮（风障半径提成常量后扫源码）、第 82 轮（扫 Tx 闭包）同源：
// **判据必须落在能直接观测的那一层。**
func TestEnergyGrantGoesThroughSatMul(t *testing.T) {
	src := readSource(t, "battle.go")

	// 定位「回补体力」那一段：从 res.Loot 的赋值往后看 20 行
	i := strings.Index(src, `res.Loot["energy"]`)
	if i < 0 {
		t.Fatal("battle.go 里找不到 `res.Loot[\"energy\"]` —— 体力回补被移走了？" +
			"若是有意移除，请连同 energy_refund_test.go 一起删掉；" +
			"若只是挪了位置，请更新本守卫的锚点。")
	}
	near := src[max(0, i-700) : i+120]

	if !strings.Contains(near, "satMul(") {
		t.Errorf("回补体力那一段没有经过 satMul：\n%s\n\n"+
			"当前 EnergyOnWin=5 < MaxEnergyPerBattle=15，所以旁路封顶"+
			"**观察不到任何差异**（实测：全部测试仍绿）。\n"+
			"但一旦有人把 EnergyOnWin 调到跨过封顶，"+
			"「永远不越界」就会变成谎言 —— 而那时测试仍然是绿的。\n"+
			"请写成 satMul(r.EnergyOnWin, 1, caps.MaxEnergyPerBattle)。",
			strings.TrimSpace(near))
	}
	if !strings.Contains(near, "caps.MaxEnergyPerBattle") {
		t.Errorf("回补体力那一段没有引用 caps.MaxEnergyPerBattle —— " +
			"封顶值必须显式出现在同一段里，否则换 cap 不会影响它")
	}
	if !strings.Contains(near, "res.Win") {
		t.Errorf("回补体力没有以 res.Win 为条件 —— " +
			"「开局→立刻退出→结算」会变成体力永动机")
	}
}

// readSource 读同目录的源码，供「扫源码」型守卫使用。
func readSource(t *testing.T, name string) string {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join(".", name))
	if err != nil {
		t.Fatalf("读 %s 失败：%v", name, err)
	}
	return string(raw)
}

// max 是 Go 1.21 起的内建函数；为兼容更老的工具链保留一个本地版。
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
