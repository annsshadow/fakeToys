package domain

// 客户端可控集合字段的边界。
//
// # 之前这些字段完全无界
//
// `SettleInput` 里有 5 个字段既没有长度/取值上界，也不参与任何奖励计算，
// 却**直接落进 battle_records** 并被分析查询读走：
//
//	ElementsUsed  map[string]int   → elements_used 列
//	ReactionsUsed map[string]int   → reactions_used 列（stats.go 的 GROUP BY 读它）
//	TerrainUsed   []string         → terrain_used 列
//	CardPicks     []int            → card_picks 列
//	ReplayHash    string           → replay_hash 列
//
// 后果不是「刷分」（它们不参与计分），而是**存储与分析污染**：
// 一个 10MB 的 `elements_used` map 就能把战报行撑到不可用，
// 而 `reactions_used` 是运营看板的分组维度 ——
// 塞进不存在的反应键，看板上的「反应分布」就成了客户端说了算的东西。
//
// 这与本项目记过的多次教训同型：
// **「不参与奖励计算」不等于「不需要校验」** ——
// 不校验它，它就进了数据库、进了报表、进了别人基于它做的决策。
//
// # 边界怎么定的
//
// 不是随手取整数，而是**从「客户端引擎实际会产出什么」反推**：
//
//   - `elements_used`  在 `applyHit` 里每命中一次 +1 ⇒ 总量 ≤ hits
//   - `reactions_used` 在同处每触发一次 +1     ⇒ 总量 ≤ reactions
//   - `terrain_used`   客户端发的是 `new Set(...)` ⇒ 不重复、且 ≤ 该关地形数
//   - `card_picks`     每波一个索引                ⇒ 长度 ≤ 波数
//
// 键名同样要校验：`reactions_used` 进了 GROUP BY，
// 塞进 `"totally_fake_reaction": 999` 就会在看板里多出一行。

import "fmt"

// maxReplayHashLen 是 replay_hash 的长度上限。
//
// 引擎产出的是 `hex16(fnv1a64(...))` = 16 个十六进制字符。
// 取 64 是为了给「将来换哈希算法」留余量，
// 同时仍然拒绝「传一段文章当哈希」这种明显不是哈希的输入。
const maxReplayHashLen = 64

// allTerrainKinds 返回全部章节声明过的地形种类。
//
// 数据源是**章节表**而不是硬编码列表 ——
// 地形种类是内容数据，加一章就多一种，
// 硬编码会让「新增地形」与「校验通过」这两件事脱钩
// （校验不认识新地形 → 玩家的正常战报被判非法）。
func allTerrainKinds() map[string]bool {
	out := make(map[string]bool)
	for _, c := range AllChapters() {
		if c.TerrainKind != "" {
			out[c.TerrainKind] = true
		}
	}
	return out
}

var (
	// 元素合法集合（包级，避免每次校验重建）
	knownElements = func() map[string]bool {
		m := make(map[string]bool)
		for _, e := range AllElements() {
			m[string(e)] = true
		}
		return m
	}()

	// 反应合法集合
	knownReactions = func() map[string]bool {
		m := make(map[string]bool)
		for _, r := range AllReactionSpecs() {
			m[string(r.Key)] = true
		}
		return m
	}()
)

// validateReportCollections 校验上报的集合类字段。
//
// 分三层，每层一个独立的错误：
//  1. **键合法性** —— 键会进数据库、进 GROUP BY，必须是已知取值
//  2. **总量上界** —— 从引擎实际产出反推，不是拍脑袋
//  3. **结构性约束** —— terrain_used 不重复、card_picks 每波一个
//
// 负值单独拒：负的计数会让 `sum()` 变小，
// 而 `diagnose` 用 `totalElements` 与 `in.Kills` 比较来决定诊断分支
// （`reactionsLow := in.FailedTimes >= 2 && totalElements < in.Kills`）——
// 一个负数就能把「元素搭配不足」的诊断结论翻转。
func validateReportCollections(gl GeneratedLevel, in SettleInput) error {
	// ── elements_used ──
	total := 0
	if len(in.ElementsUsed) > len(knownElements) {
		return fmt.Errorf("%w：elements_used 含 %d 个键，合法元素只有 %d 个",
			ErrInvalidField, len(in.ElementsUsed), len(knownElements))
	}
	for k, v := range in.ElementsUsed {
		if !knownElements[k] {
			return fmt.Errorf("%w：未知元素 %q", ErrInvalidField, k)
		}
		if v < 0 {
			return fmt.Errorf("%w：元素 %q 的计数为负 %d", ErrInvalidField, k, v)
		}
		total += v
	}
	// 引擎里 elements_used 在 applyHit 内每命中一次 +1
	if total > in.Hits {
		return fmt.Errorf("%w：元素使用合计 %d 超过命中数 %d", ErrInvalidField, total, in.Hits)
	}

	// ── reactions_used ──
	totalR := 0
	if len(in.ReactionsUsed) > len(knownReactions) {
		return fmt.Errorf("%w：reactions_used 含 %d 个键，合法反应只有 %d 种",
			ErrInvalidField, len(in.ReactionsUsed), len(knownReactions))
	}
	for k, v := range in.ReactionsUsed {
		if !knownReactions[k] {
			return fmt.Errorf("%w：未知反应 %q", ErrInvalidField, k)
		}
		if v < 0 {
			return fmt.Errorf("%w：反应 %q 的计数为负 %d", ErrInvalidField, k, v)
		}
		totalR += v
	}
	// 引擎里 reactionsUsed 与 reactions 计数器同步 +1
	if totalR > in.Reactions {
		return fmt.Errorf("%w：反应分项合计 %d 超过反应总数 %d",
			ErrInvalidField, totalR, in.Reactions)
	}

	// ── terrain_used ──
	// 客户端发的是 `new Set(this.terrainUsed)`，所以天然不重复且长度有界。
	// 重复项说明上报不是引擎产出的。
	//
	// ⚠️ 上界取「**去重后的种类数**」而不是 `len(gl.Terrain)`：
	// 关卡数据里可以有多个同类地形（实测第 63 关有多个 `tidal_gate`），
	// 而客户端发的是去重集合 —— 用地形总数当上界会**放行**
	// 「把 3 个 tidal_gate 各写一遍」这种上报。
	kinds := allTerrainKinds()
	distinct := make(map[string]bool, len(gl.Terrain))
	for _, t := range gl.Terrain {
		distinct[t.Kind] = true
	}
	if len(in.TerrainUsed) > len(distinct) {
		return fmt.Errorf("%w：terrain_used 有 %d 项，该关只有 %d 种地形",
			ErrInvalidField, len(in.TerrainUsed), len(distinct))
	}
	seen := make(map[string]bool, len(in.TerrainUsed))
	for _, t := range in.TerrainUsed {
		if !kinds[t] {
			return fmt.Errorf("%w：未知地形 %q", ErrInvalidField, t)
		}
		// ⚠️ 必须**逐项**落在该关自己的地形种类里。
		//
		// 只查「是不是全局已知种类」是不够的：那样玩家可以把
		// 「别的关才有的地形」写进本关的 terrain_used，
		// 按关分析立刻失真。而 `len(distinct)` 那条长度检查抓不到它 ——
		// 一个不重复、长度合法、但内容不属于本关的列表，
		// 能同时绕开长度检查与重复检查。
		//
		// （这条是变异验证逼出来的：把长度上界从 `len(distinct)`
		//  放宽成 `len(gl.Terrain)` 时，没有任何测试变红，
		//  说明这两条检查其实是重叠的、而真正的漏洞在别处。）
		if !distinct[t] {
			return fmt.Errorf("%w：地形 %q 不在本关的地形里", ErrInvalidField, t)
		}
		if seen[t] {
			return fmt.Errorf("%w：terrain_used 含重复项 %q（引擎发的是去重集合）",
				ErrInvalidField, t)
		}
		seen[t] = true
	}

	// ── card_picks ──
	if len(in.CardPicks) > gl.WaveCount {
		return fmt.Errorf("%w：card_picks 有 %d 项，该关只有 %d 波",
			ErrInvalidField, len(in.CardPicks), gl.WaveCount)
	}
	for i, p := range in.CardPicks {
		// -1 = 整波跳过，是合法取值。上界不设：引擎对越界索引按「跳过」处理，
		// 而卡面数量本身由 `rollWaveCards` 决定（当前每波 3 张），
		// 把它写死成一个常量会在加第 4 张卡那天变成一个静默的错误上限。
		if p < -1 {
			return fmt.Errorf("%w：card_picks[%d] = %d，合法值是 >= -1（-1 表示跳过）",
				ErrInvalidField, i, p)
		}
	}

	// ── replay_hash ──
	if len(in.ReplayHash) > maxReplayHashLen {
		return fmt.Errorf("%w：replay_hash 长 %d 字符，上限 %d",
			ErrInvalidField, len(in.ReplayHash), maxReplayHashLen)
	}
	return nil
}
