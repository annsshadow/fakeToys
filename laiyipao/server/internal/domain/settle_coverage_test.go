package domain

// `SettleInput` 每个字段的校验覆盖守卫。
//
// # 为什么要反射枚举
//
// `SettleInput` 是**全客户端可控、且直接落库**的结构体。
// 一个新增字段若忘了校验，它就会：
//
//	1. 静默落进 battle_records（存储放大）
//	2. 可能被分析查询读走（决策污染）
//	3. **没有任何测试会红** —— 因为没有测试知道它应该被校验
//
// 「没有测试知道」是关键：这类缺陷不是被测出来的，是**根本没被想过**。
// 反射枚举把「想过」变成可检查的事实：
// 新增字段 → 表里没有它 → 测试红 → 必须补校验或显式豁免并写明理由。
//
// # 判据是行为，不是文本搜索
//
// 不用「grep 字段名看有没有 if」那种判据 ——
// 本项目已经吃过 7 次「度量前提不成立」的亏，其中就有
// 「硬编码字段清单当反射用」。这里直接**构造越界输入调 ValidateSettle**，
// 看它是否真的拒绝。
//
// 代价是：每个字段需要一个能触发它边界检查的构造。
// 构造不出来 = 这个字段确实没有边界 = 要么补校验，要么写进豁免表并说明理由。

import (
	"fmt"
	"reflect"
	"strings"
	"testing"
	"time"
)

// settleExempt 是**显式豁免**的字段：没有边界校验，但必须有理由。
//
// 写进这张表是一个有意识的决定，不是「忘了」。
type settleExempt struct {
	// Reason 说明为什么不需要边界。空字符串会让测试失败 ——
	// 「豁免」而不说理由，等于没做判断。
	Reason string
}

var settleExemptions = map[string]settleExempt{
	"Result": {"取值只有 win/其他，而「其他」被当作 lose 处理（fail-safe），" +
		"不需要枚举校验。枚举反而会在新增一种结果时把老客户端打挂。"},
	"Stars": {"完全被忽略：结算时由 computeStars(res.Score, gl.StarTargets) 重算" +
		"（见 battle.go 的 StarsRecomputed），上报值不参与任何决策。"},
	"Score": {"有上界：scoreCapFor(gl, sec) 裁剪；下界有夹紧（负分归零）。" +
		"不需要独立的越界构造，所以放在豁免表而不是 fieldBounds。"},
}

// numericBounds 给出每个整型字段的「越界构造」与「合法基线」。
//
// 每个字段都必须有一个能把 ValidateSettle 推入拒绝态的构造。
type boundCase struct {
	// Mutate 把字段推到越界值
	Mutate func(in *SettleInput, gl GeneratedLevel)
	// Why 说明这个构造为什么能触发拒绝 —— 用来在失败时定位，
	// 而不是只看到一个 "want error, got nil"
	Why string
	// Mutate2 是**第二类**越界构造（可选）。
	//
	// 为什么需要它：一个集合字段至少有两类互不覆盖的边界 ——
	// 键的合法性、数值的上界。只测前者，后者删掉也不会红。
	//
	// 这个缺口是变异验证发现的：把「元素计数合计 > 命中数」整条删掉，
	// 10 个测试全绿。
	Mutate2 func(in *SettleInput, gl GeneratedLevel)
	Why2    string
}

// settleBase 是一个**能通过全部校验**的合法输入。
//
// 判据必须建立在「先能通过」之上：
// 若基线本身过不了校验，那么后面「期望它被拒绝」就毫无意义
// —— 任何改动都会让它被拒绝，测试恒绿。
func settleBase(t *testing.T, gl GeneratedLevel) SettleInput {
	t.Helper()
	kills := MaxKillsFor(gl)
	in := SettleInput{
		Result:       "win",
		Score:        gl.StarTargets[0],
		Kills:        kills,
		Leaked:       0,
		HPLeft:       int(gl.BaseHP),
		WaveReached:  gl.WaveCount,
		DurationMs:   int(limitsFor(gl).MinDurationMs) + 1000,
		Shots:        100,
		Hits:         80,
		Reactions:    10,
		HeatMax:      1000,
		ElementsUsed: map[string]int{"fire": 80},
		ReactionsUsed: map[string]int{
			"steam_burst": 10,
		},
		TerrainUsed: terrainList(gl),
		CardPicks:   make([]int, gl.WaveCount),
		ReplayHash:  "0123456789abcdef",
	}
	for i := range in.CardPicks {
		in.CardPicks[i] = 0
	}
	return in
}

func limitsFor(gl GeneratedLevel) SettleLimits {
	// ⚠️ `ScoreFullAtSec` 是**包级常量**、不是 `SettleLimits` 的字段 ——
	// 第一版把它写进结构体字面量，编译不过。
	// 顺带说明它为什么在包里而不是配置里：它是分数裁剪速率的锚点，
	// 必须与 `scoreCapFor` 用同一个值；放进可配置结构体
	// 就多了一个「两处可能不同步」的面。
	return SettleLimits{
		MinDurationMs:  minDurationMsFor(gl),
		MaxScorePerSec: 0, // 已废弃，不参与判定
	}
}

// terrainList 复现客户端的 `new Set(this.terrainUsed)` 行为。
//
// ⚠️ **必须去重**。第一版直接遍历 `gl.Terrain` 全量，
// 而关卡数据里可以有**多个同类地形**（实测第 63 关有多个 `tidal_gate`）——
// 于是基线里出现了重复项，被新加的「terrain_used 不许重复」校验当场拒绝。
//
// 这条是被守卫自己抓出来的：判据建立在「基线必须先能通过」之上，
// 基线错了，后面每一条断言都不可信。
func terrainList(gl GeneratedLevel) []string {
	out := make([]string, 0, len(gl.Terrain))
	seen := make(map[string]bool, len(gl.Terrain))
	for _, t := range gl.Terrain {
		if seen[t.Kind] {
			continue
		}
		seen[t.Kind] = true
		out = append(out, t.Kind)
	}
	return out
}

// minDurationMsFor 取该关的合理最短时长。
//
// 用「一个宽松的常数」而不是精确推导：
// 判据只关心「越界是否被拒」，基线时长只要合法即可。
func minDurationMsFor(gl GeneratedLevel) int64 {
	base := int64(30) * 1000
	if int64(gl.WaveCount) > 0 {
		base = int64(gl.WaveCount) * 2000
	}
	return base
}

// fieldBounds 覆盖所有需要边界的字段。
//
// ⚠️ 键必须与结构体字段名逐字一致 —— TestEverySettleInputFieldIsAccountedFor
// 会检查「表里没有的字段」与「结构体里没有的表项」两个方向。
var fieldBounds = map[string]boundCase{
	"Kills": {
		Mutate: func(in *SettleInput, _ GeneratedLevel) { in.Kills = 1 << 30 },
		Why:    "超过该关总怪数",
	},
	"Leaked": {
		Mutate: func(in *SettleInput, _ GeneratedLevel) { in.Leaked = 1 << 30 },
		Why:    "超过该关总怪数",
	},
	"WaveReached": {
		Mutate: func(in *SettleInput, _ GeneratedLevel) { in.WaveReached = 1 << 20 },
		Why:    "超过该关总波数",
	},
	"DurationMs": {
		Mutate: func(in *SettleInput, _ GeneratedLevel) { in.DurationMs = 0 },
		Why:    "时长必须为正",
	},
	"Shots": {
		Mutate: func(in *SettleInput, _ GeneratedLevel) { in.Shots = 1 << 28 },
		Why:    "超过该时长的物理上限",
	},
	"Hits": {
		Mutate: func(in *SettleInput, _ GeneratedLevel) { in.Hits = 1 << 28 },
		Why:    "超过发射数（或超过时长上限）",
	},
	"Reactions": {
		Mutate: func(in *SettleInput, _ GeneratedLevel) { in.Reactions = 1 << 28 },
		Why:    "超过 min(shots×每击上限, 总怪数×每怪上限)",
	},
	"HeatMax": {
		Mutate: func(in *SettleInput, _ GeneratedLevel) { in.HeatMax = 1 << 30 },
		Why:    "超过 MaxReportedHeat",
	},
	"HPLeft": {
		Mutate: func(in *SettleInput, gl GeneratedLevel) { in.HPLeft = int(gl.BaseHP) + 1 },
		Why:    "超过该关初始血量",
	},
	"ElementsUsed": {
		// ⚠️ 两个越界构造，**都要有**。
		//
		// 第一版只有「未知元素键」，于是「合计 > 命中数」那条检查
		// 从来没被触发过 —— 把它删掉，测试照样全绿。
		// 一个 map 字段至少有两类边界（键合法性、数值上界），
		// 只测一个就等于宣称「它被守住了」。
		Mutate: func(in *SettleInput, _ GeneratedLevel) { in.ElementsUsed = map[string]int{"not_an_element": 1} },
		Why:    "未知元素键",
		Mutate2: func(in *SettleInput, gl GeneratedLevel) {
			// 键完全合法，但计数超过上界（shots × 该关怪数）
			//
			// ⚠️ 上界不是 `hits` —— 实测真实引擎的元素合计 / hits 最大 1.214，
			// 取 hits 会拒掉正常对局。详见 battle_collections.go 的注释。
			in.ElementsUsed = map[string]int{"fire": in.Shots*MaxKillsFor(gl) + 1000}
		},
		Why2: "元素计数合计超过 shots×怪数（键合法，只有数值越界）",
	},
	"ReactionsUsed": {
		Mutate: func(in *SettleInput, _ GeneratedLevel) {
			in.ReactionsUsed = map[string]int{"fake_reaction": 1}
		},
		Why: "未知反应键（它会进运营看板的 GROUP BY）",
		Mutate2: func(in *SettleInput, _ GeneratedLevel) {
			// 键完全合法，但分项合计超过反应总数
			in.ReactionsUsed = map[string]int{"steam_burst": in.Reactions + 1000}
		},
		Why2: "反应分项合计超过反应总数（键合法，只有数值越界）",
	},
	"TerrainUsed": {
		Mutate: func(in *SettleInput, _ GeneratedLevel) { in.TerrainUsed = []string{"no_such_terrain"} },
		Why:    "未知地形",
	},
	"CardPicks": {
		Mutate: func(in *SettleInput, _ GeneratedLevel) { in.CardPicks = make([]int, 1<<20) },
		Why:    "长度超过该关波数",
	},
	"ReplayHash": {
		Mutate: func(in *SettleInput, _ GeneratedLevel) { in.ReplayHash = strings.Repeat("a", 4096) },
		Why:    "长度超过上限",
	},
}

func TestEverySettleInputFieldIsAccountedFor(t *testing.T) {
	// 先确认基线合法 —— 否则下面所有「期望拒绝」都是空断言
	gl := GenerateLevel(1)
	now := time.Now()
	tok := BattleToken{LevelID: gl.ID, ExpiresAt: now.Add(time.Hour), UsedAt: nil}
	if _, err := ValidateSettle(tok, gl, settleBase(t, gl), limitsFor(gl), now); err != nil {
		t.Fatalf("合法基线竟被拒，后续「期望拒绝」的断言全部无意义：%v", err)
	}

	tt := reflect.TypeOf(SettleInput{})
	var missing []string
	for i := 0; i < tt.NumField(); i++ {
		name := tt.Field(i).Name
		_, hasBound := fieldBounds[name]
		ex, hasExempt := settleExemptions[name]
		if !hasBound && !hasExempt {
			missing = append(missing, name)
		}
		if hasExempt && strings.TrimSpace(ex.Reason) == "" {
			t.Errorf("字段 %s 被豁免但没写理由 —— 「豁免」而不说理由等于没做判断", name)
		}
	}
	if len(missing) > 0 {
		t.Fatalf("SettleInput 有 %d 个字段既无边界校验、也未显式豁免：%s\n"+
			"新增字段若不表态，它会静默落库（reactions_used 还会进看板的 GROUP BY）",
			len(missing), strings.Join(missing, ", "))
	}

	// 反向：表里有、结构体里没有的项 —— 那是重构后的残留，
	// 会让人误以为那个字段被守着
	known := map[string]bool{}
	for i := 0; i < tt.NumField(); i++ {
		known[tt.Field(i).Name] = true
	}
	var stale []string
	for name := range fieldBounds {
		if !known[name] {
			stale = append(stale, name+"（fieldBounds）")
		}
	}
	for name := range settleExemptions {
		if !known[name] {
			stale = append(stale, name+"（settleExemptions）")
		}
	}
	if len(stale) > 0 {
		t.Errorf("覆盖表里有结构体上已不存在的字段：%s", strings.Join(stale, ", "))
	}
}

func TestEveryBoundedFieldActuallyRejectsOutOfRange(t *testing.T) {
	// 行为级判据：真的构造越界输入，真的调 ValidateSettle，真的要求它拒绝。
	//
	// ⚠️ 覆盖多关而不是只测第 1 关：
	// 边界依赖关卡的怪数/波数/地形，只测一关的话，
	// 「边界恰好在这一关上不生效」会被漏掉。
	ids := []int{1, 2, 7, 13, 25, 50, 63, 77, 88, 100}
	for _, id := range ids {
		gl := GenerateLevel(id)
		now := time.Now()
		tok := BattleToken{LevelID: gl.ID, ExpiresAt: now.Add(time.Hour)}

		// 前提：基线必须合法
		base := settleBase(t, gl)
		if _, err := ValidateSettle(tok, gl, base, limitsFor(gl), now); err != nil {
			t.Fatalf("第 %d 关合法基线被拒，判据不成立：%v", id, err)
		}

		for _, name := range sortedKeys(fieldBounds) {
			bc := fieldBounds[name]
			bad := settleBase(t, gl)
			bc.Mutate(&bad, gl)
			_, err := ValidateSettle(tok, gl, bad, limitsFor(gl), now)
			if err == nil {
				t.Errorf("第 %d 关：%s 越界（%s）竟被接受", id, name, bc.Why)
			}
			if bc.Mutate2 == nil {
				continue
			}
			bad2 := settleBase(t, gl)
			bc.Mutate2(&bad2, gl)
			if _, err := ValidateSettle(tok, gl, bad2, limitsFor(gl), now); err == nil {
				t.Errorf("第 %d 关：%s 越界（%s）竟被接受", id, name, bc.Why2)
			}
		}
	}
}

func sortedKeys(m map[string]boundCase) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	// 排序让失败信息稳定（map 遍历顺序随机）
	for i := 0; i < len(out); i++ {
		for j := i + 1; j < len(out); j++ {
			if out[j] < out[i] {
				out[i], out[j] = out[j], out[i]
			}
		}
	}
	return out
}

func TestNegativeCountsRejected(t *testing.T) {
	// 负计数单独守：它们不违反长度/键的边界，
	// 但会让 `sum()` 变小，而 `diagnose` 用元素合计与 kills 比较来选诊断分支
	//（`reactionsLow := in.FailedTimes >= 2 && totalElements < in.Kills`）
	// —— 一个负数就能把「元素搭配不足」的结论翻转。
	gl := GenerateLevel(1)
	now := time.Now()
	tok := BattleToken{LevelID: gl.ID, ExpiresAt: now.Add(time.Hour)}

	for _, tc := range []struct {
		name string
		in   SettleInput
	}{
		{"元素计数为负", func() SettleInput {
			in := settleBase(t, gl)
			in.ElementsUsed = map[string]int{"fire": -1}
			return in
		}()},
		{"反应计数为负", func() SettleInput {
			in := settleBase(t, gl)
			in.ReactionsUsed = map[string]int{"steam_burst": -1}
			return in
		}()},
	} {
		if _, err := ValidateSettle(tok, gl, tc.in, limitsFor(gl), now); err == nil {
			t.Errorf("%s 竟被接受", tc.name)
		}
	}
}

func TestTerrainUsedRejectsDuplicates(t *testing.T) {
	// 客户端发的是 `new Set(...)`，天然不重复。
	// 重复项说明上报不是引擎产出的 —— 或者说，它是被手工构造的。
	//
	// ⚠️ 用**合成关卡**，不用真实关卡数据。
	//
	// 原因：`generateTerrain` 产出的 1~3 个地形**全是同一个 kind**
	//（章节的 `TerrainKind`），而 100 关里 75 关根本没有地形。
	// 于是**没有任何一关的 `terrain_used` 合法长度能超过 1** ——
	// 真实数据永远构造不出「不超长但重复」的样本。
	//
	// 第一版用真实关卡 + `t.Skip`，结果这条守卫**从未真正执行过**，
	// 而 vitest/go test 把 SKIP 计入 PASS ⇒ 「全绿」里藏着一个没测的东西。
	// 这正是「Tests pass is wrong if any were skipped」那条。
	//
	// 合成两���地形的关卡后：`[A, A]` 长度 2 ≤ distinct 2，
	// 长度检查放行，**只有重复检查**能拦 —— 守卫这才真的被测到。
	gl := GenerateLevel(1)
	kinds := terrainKindsOn(gl)
	if len(kinds) == 0 {
		t.Fatalf("第 1 关没有地形，无法作为合成关卡的底本")
	}
	// 追加一个与现有不同的合法地形种类
	other := ""
	for k := range allTerrainKinds() {
		if !hasTerrain(gl, k) {
			other = k
			break
		}
	}
	if other == "" {
		t.Fatalf("找不到第二种地形种类，无法构造合成关卡")
	}
	gl.Terrain = append(gl.Terrain, TerrainPlacement{Kind: other, X: 400, Y: 400, Param: 100})

	now := time.Now()
	tok := BattleToken{LevelID: gl.ID, ExpiresAt: now.Add(time.Hour)}
	in := settleBase(t, gl)
	in.TerrainUsed = []string{kinds[0], kinds[0]}

	// 前提：长度必须**不超**上界，否则会被长度检查顺带拦下，
	// 这条测试就退化��在测长度检查。
	distinct := len(terrainKindsOn(gl))
	if len(in.TerrainUsed) > distinct {
		t.Fatalf("构造有误：长度 %d 超过 distinct %d", len(in.TerrainUsed), distinct)
	}
	if _, err := ValidateSettle(tok, gl, in, limitsFor(gl), now); err == nil {
		t.Errorf("terrain_used 含重复项 %v 竟被接受（distinct=%d）", in.TerrainUsed, distinct)
	}
}

func terrainKindsOn(gl GeneratedLevel) []string {
	seen := map[string]bool{}
	var out []string
	for _, t := range gl.Terrain {
		if !seen[t.Kind] {
			seen[t.Kind] = true
			out = append(out, t.Kind)
		}
	}
	return out
}

// TestTerrainUsedMustBelongToThisLevel 守「按关分析」的正确性。
//
// 只校验「是不是全局已知地形种类」是不够的：
// 玩家可以把**别的关才有的地形**写进本关的 terrain_used，
// 于是「第 63 关的潮汐门触发率」这类按关统计立刻失真。
//
// 这条与长度检查、重复检查**互不覆盖**：
// 一个不重复、长度合法、内容却属于别的关的列表，
// 能同时绕开前两条 —— 这正是变异验证发现的漏洞。
func TestTerrainUsedMustBelongToThisLevel(t *testing.T) {
	// 找一关：它有地形，且**不是**全部 5 种地形都有
	var target *GeneratedLevel
	for id := 1; id <= 100 && target == nil; id++ {
		gl := GenerateLevel(id)
		if len(gl.Terrain) == 0 {
			continue
		}
		distinct := map[string]bool{}
		for _, tr := range gl.Terrain {
			distinct[tr.Kind] = true
		}
		if len(distinct) >= len(allTerrainKinds()) {
			continue // 这关地形种类齐全，构不出「不属于本关」的项
		}
		g := gl
		target = &g
	}
	if target == nil {
		t.Skip("没有任何一关的地形种类少于全集，构不出越界样本")
	}
	gl := *target

	// 找一个「全局合法但本关没有」的地形
	var foreign string
	for k := range allTerrainKinds() {
		if !hasTerrain(gl, k) {
			foreign = k
			break
		}
	}
	if foreign == "" {
		t.Skip("找不到可用于越界的地形种类")
	}

	now := time.Now()
	tok := BattleToken{LevelID: gl.ID, ExpiresAt: now.Add(time.Hour)}
	in := settleBase(t, gl)
	// ⚠️ 必须**替换**而不是追加。
	//
	// 第一版用 `append`，结果长度检查（`len > len(distinct)`）
	// 顺带把它拦下了 —— 于是「删掉 `distinct[t]` 那条」这个变异
	// **不会让任何测试变红**，看起来像有守卫、实际没有。
	//
	// 这就是「守卫本身不守卫」：断言红了不等于它测的是你想测的东西。
	//
	// 替换掉一个合法项 → 长度不变、无重复、内容含外来地形，
	// 此时只有 `distinct[t]` 那条能拦。
	if len(in.TerrainUsed) == 0 {
		t.Fatalf("第 %d 关基线的 terrain_used 为空，无法构造替换样本", gl.ID)
	}
	in.TerrainUsed[0] = foreign

	_, err := ValidateSettle(tok, gl, in, limitsFor(gl), now)
	if err == nil {
		t.Fatalf("第 %d 关上报了本关没有的地形 %q 竟被接受 —— 按关的「地形触发率」统计会失真",
			gl.ID, foreign)
	}
}

func hasTerrain(gl GeneratedLevel, kind string) bool {
	for _, t := range gl.Terrain {
		if t.Kind == kind {
			return true
		}
	}
	return false
}

func TestOversizedCollectionIsRejectedNotTruncated(t *testing.T) {
	// 「拒绝」而不是「截断」是有意的：
	// 截断会让上报值与实际战斗不符，而这类字段不进计分，
	// 截断唯一的收益是让请求成功 —— 那是拿数据完整性换一次 200，
	// 不划算。直接拒绝并报错，客户端就知道上报有问题。
	gl := GenerateLevel(1)
	now := time.Now()
	tok := BattleToken{LevelID: gl.ID, ExpiresAt: now.Add(time.Hour)}
	in := settleBase(t, gl)
	big := make(map[string]int, 4096)
	for i := 0; i < 4096; i++ {
		big[fmt.Sprintf("ghost_%d", i)] = 1
	}
	in.ReactionsUsed = big
	if _, err := ValidateSettle(tok, gl, in, limitsFor(gl), now); err == nil {
		t.Errorf("4096 个未知反应键竟被接受 —— 这会把它们塞进运营看板的 GROUP BY")
	}
}
