package domain

// 跨端「整局重放」契约向量（R1）。
//
// # 这份向量在守什么
//
// 结算侧要堵上「伪造满分通关」，前提是**服务端能独立重算一局**。
// 而重算的正确性判据只有一个：Go 侧跑同一组输入，
// 必须与客户端跑出的结果**逐位相同** —— 事件流、回放哈希、以及
// 直接换算成奖励的 kills / leaked / score。
//
// 这份向量就是那个判据的载体。
//
// # 期望值为什么来自 TS 侧（这是最容易被质疑的一步）
//
// 项目既有规矩是「只导数据，不导期望值，用实现生成期望值就是自证」。
// 那条对**单个公式**向量成立，但对**整局重放**不成立：
//
//  1. 一局战斗的哈希无法人工推导。
//  2. **TS 引擎就是玩家设备上真实运行的那份代码。**
//     「合法的一局」在定义上就是「TS 引擎算出来的那一局」，
//     所以 TS 的输出是**定义**，而不是「用实现证明实现」。
//
// 方向没有搞反：本文件断言的是「Go 与 TS 一致」，
// 而不是「Go 与 Go 一致」—— 后者才是自证，那正是 `cmd/vectors` 的处境
// （它跑在 Go 侧、读 Go 常量，产出 Go 期望值）。
//
// 生成器：`miniapp/scripts/gen-replay-vectors.ts`（跑真实引擎）。
//
// # ⚠️ 本文件现在只守「契约本身是好的」
//
// Go 侧的引擎尚未移植，所以这里**没有**「Go 重算结果 == 期望值」这条断言 ——
// 写一条必然红的测试并提交，等于给所有人留一个假红灯，
// 而长期红灯的唯一效果就是被忽略。
//
// 所以本文件现在守的是**生成器输出的可信度**：确定性、形状、分支覆盖。
// 这些恰恰是「生成器坏了」的唯一可观测形态 —— 一个坏掉的生成器会
// 产出形状正确但数值离谱的向量，而那种向量在 Go 侧移植完成后
// 会被误当成「Go 引擎写错了」。
import (
	"encoding/json"
	"os"
	"path/filepath"
	"regexp"
	"runtime"
	"strconv"
	"testing"
)

// thisFile 是本测试文件的绝对路径，供夹具定位 testdata 用。
//
// 用 `runtime.Caller` 而不是 `os.Getwd()`：CWD 随调用方式变化
// （go test 在包目录、IDE 在仓库根、手工跑又在别处），
// 而契约文件的位置是**固定的**（相对本文件）。
var thisFile = func() string {
	_, file, _, ok := runtime.Caller(0)
	if !ok {
		panic("runtime.Caller 失败，无法定位 testdata")
	}
	return file
}()

type replayVectorDoc struct {
	Comment  string       `json:"_comment"`
	Version  int          `json:"version"`
	HashNote string       `json:"hash_note"`
	Cases    []replayCase `json:"cases"`
}

type replayCase struct {
	LevelID       int         `json:"level_id"`
	Seed          string      `json:"seed"`
	CardPicks     []int       `json:"card_picks"`
	Build         replayBuild `json:"build"`
	ExpectedHash  string      `json:"expected_hash"`
	ExpectedStats replayStats `json:"expected_stats"`
}

type replayBuild struct {
	Skills      map[string]replaySkill `json:"skills"`
	ActiveSlots int                    `json:"active_slots"`
	Attacker    replayAttacker         `json:"attacker"`
}

type replaySkill struct {
	ID      int    `json:"id"`
	Element string `json:"element"`
	Kind    string `json:"kind"`
	Level   int    `json:"level"`
	Slot    int    `json:"slot"`
}

type replayAttacker struct {
	Attack               int `json:"attack"`
	CritPermille         int `json:"crit_permille"`
	CritMultPermille     int `json:"crit_multiplier_permille"`
	ReactionMultPermille int `json:"reaction_mult_permille"`
	ElementCap           int `json:"element_cap"`
	ReactionTier         int `json:"reaction_tier"`
	ElementCoefPermille  int `json:"element_coef_permille"`
	HeatCapPermille      int `json:"heat_cap_permille"`
	ArmorPermille        int `json:"armor_permille"`
	MechanicPermille     int `json:"mechanic_permille"`
}

type replayStats struct {
	Kills  int `json:"kills"`
	Leaked int `json:"leaked"`
	Score  int `json:"score"`
	Waves  int `json:"waves"`
	// ⚠️ tag 是 `durationMs`（camelCase）而**不是** `duration_ms`。
	//
	// 向量文件里就是这个键名，而本文件此前写成 snake_case ——
	// 解码永远是 0，而**没有任何断言读它**，所以一直没被发现。
	// 教训：解码失败**静默**（零值），只有断言才会暴露。
	// 接线 G 块后 `TestWholeBattleReplayHashMatchesCrossEnd` 断言了它，当场变红。
	DurationMs int `json:"durationMs"`
}

// loadReplayVectors 读契约向量。缺失即失败 —— 绝不能回落到空集合。
//
// ⚠️ 路径必须由**本文件的绝对位置**推导，不能用 `../..` 相对 CWD。
// `go test` 的工作目录是**包目录**（`internal/domain`），
// 而 `go build` 之类命令的 CWD 可能在别处 —— 我第一版写成
// `filepath.Join("..", "testdata", ...)`，在 `go test` 下找不到，
// 报的是一句看不出原因的「系统找不到指定的文件」。
//
// 与 consistency_test.go 的做法保持一致（它用 `filepath.Dir(thisFile)`）。
func loadReplayVectors(t *testing.T) replayVectorDoc {
	t.Helper()
	path := filepath.Join(filepath.Dir(thisFile), "..", "..", "testdata", "replay_vectors.json")
	raw, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("读 replay_vectors.json 失败：%v\n"+
			"（路径 %s）\n"+
			"请运行 miniapp/scripts/gen-replay-vectors.ts 重新生成", err, path)
	}
	var doc replayVectorDoc
	if err := json.Unmarshal(raw, &doc); err != nil {
		t.Fatalf("replay_vectors.json 不是合法 JSON：%v", err)
	}
	return doc
}

// TestReplayVectorsExistAndAreNonEmpty 是夹具自检。
//
// 为什么需要：下面每一条断言都建立在「cases 非空」这个前提上，
// 而一个空的 cases 会让它们**全部无声通过**（for 循环零次迭代）。
// 这正是 README 记的那类「看起来测了东西、实际什么都没测」。
func TestReplayVectorsExistAndAreNonEmpty(t *testing.T) {
	doc := loadReplayVectors(t)
	if doc.Version != 1 {
		t.Errorf("version = %d，期望 1（版本变了说明字段语义变了，要重新审）", doc.Version)
	}
	if len(doc.Cases) == 0 {
		t.Fatal("cases 为空 —— 下面所有断言都会无声通过")
	}
	if doc.Comment == "" {
		t.Error("_comment 为空：这份向量的来源与「期望值为何可信」必须写在文件里，\n" +
			"否则下一个人会以为它和别的向量一样是自证的")
	}
}

// TestReplayVectorHashesAreWellFormed 断言哈希的**形状**。
//
// 判据用形状而不是具体值：具体值是数据（会随引擎改动而变），
// 形状是契约（引擎不该改）。一个生成器坏了的形态是
// 「长度不对 / 出了非十六进制字符」—— 那在移植之前就能看出来。
func TestReplayVectorHashesAreWellFormed(t *testing.T) {
	doc := loadReplayVectors(t)
	hex16 := regexp.MustCompile(`^[0-9a-f]{16}$`)
	seen := map[string]int{}
	for _, c := range doc.Cases {
		if !hex16.MatchString(c.ExpectedHash) {
			t.Errorf("第 %d 关的哈希 %q 不是 16 位小写十六进制 —— "+
				"生成器或引擎坏了（引擎产出的是 hex16(fnv1a64(...))）",
				c.LevelID, c.ExpectedHash)
		}
		if prev, dup := seen[c.ExpectedHash]; dup {
			t.Errorf("第 %d 关与第 %d 关哈希相同（%s）—— "+
				"两个不同关卡/种子/构筑必须产出不同哈希，否则哈希没有区分力",
				prev, c.LevelID, c.ExpectedHash)
		}
		seen[c.ExpectedHash] = c.LevelID
	}
}

// TestReplayVectorSeedsFitInt64 断言种子是**字符串形态且能解析**。
//
// ⚠️ 这条守的是一个已经真实发生过的坑：int64 超过 2^53 后
//
//	JSON number 在 JS 侧解析即失精，`6917529027818027904`
//	会被解析成 `...8032`，而断言会**悄悄通过**。
//
// 所以跨端契约里的 64 位值一律用字符串承载，且必须能被 Go 的
// ParseInt(s,10,64) 解析 —— 含负数与边界值（本向量里有 -1 与 -4.6e18）。
func TestReplayVectorSeedsFitInt64(t *testing.T) {
	doc := loadReplayVectors(t)
	for _, c := range doc.Cases {
		if _, err := strconv.ParseInt(c.Seed, 10, 64); err != nil {
			t.Errorf("第 %d 关的种子 %q 不能解析为 int64（必须用字符串承载，"+
				"JSON number 超过 2^53 后 JS 解析即失精）：%v", c.LevelID, c.Seed, err)
		}
	}
}

// TestReplayVectorPicksAreValid 断言选牌项都在合法区间。
//
// 判据复用 `validCardPick` —— 即**生产代码里那个函数**，而不是抄一份区间。
// 抄一份的话，区间一旦调整，这里会变成一个没人更新的过期常量。
func TestReplayVectorPicksAreValid(t *testing.T) {
	doc := loadReplayVectors(t)
	for _, c := range doc.Cases {
		for i, p := range c.CardPicks {
			if !validCardPick(p) {
				t.Errorf("第 %d 关 card_picks[%d] = %d 不是合法编码（合法区间见 validCardPick）",
					c.LevelID, i, p)
			}
		}
	}
}

// TestReplayContractCoversEveryBranch 断言这批向量**真的覆盖到了各条分支**。
//
// # 为什么这条最重要
//
// 一批「全部命中同一路径」的向量会全绿，却让「覆盖度」变成错觉 ——
// 那正是 README 反复记的「可完成性是逐关性质，必须全量跑」与
// 「分布缺陷不会让单点自检变红」的同类。
//
// 所以这里钉的不是「有几个用例」，而是**每个用例触到了什么**。
// 判据是向量里**可从数据直接读出**的性质，而不是「跑一遍引擎去数」——
// 后者在 Go 侧移植完成前根本不存在。
func TestReplayContractCoversEveryBranch(t *testing.T) {
	doc := loadReplayVectors(t)

	var (
		withPicks, withoutPicks int
		levelIDs                = map[int]bool{}
	)
	pickShapes := map[string]bool{}

	for _, c := range doc.Cases {
		levelIDs[c.LevelID] = true
		if len(c.CardPicks) == 0 {
			withoutPicks++
			continue
		}
		withPicks++
		// 形态分类：全取牌 / 全跳过 / 混合。
		// 必须至少覆盖「有弃牌编码」与「纯跳过」两类，
		// 否则 discardTake / discardSkip 这两条分支无人守。
		allSkip, allTake := true, true
		for _, p := range c.CardPicks {
			if p >= 0 {
				allSkip = false
			} else {
				allTake = false
			}
		}
		switch {
		case allSkip:
			pickShapes["skip-only"] = true
		case allTake:
			pickShapes["take-only"] = true
		default:
			pickShapes["mixed"] = true
		}
		if len(c.Build.Skills) == 0 {
			t.Errorf("第 %d 关的构筑没有任何技能 —— 那是一局打不出任何东西的战斗", c.LevelID)
		}
		if c.Build.ActiveSlots <= 0 {
			t.Errorf("第 %d 关的 active_slots = %d，期望 > 0", c.LevelID, c.Build.ActiveSlots)
		}
		// 攻方七项必须全部非零 —— A 段的每一项都进哈希前缀，
		// 全零的向量会让「攻方系数是否真的影响哈希」这条无人守。
		a := c.Build.Attacker
		if a.Attack == 0 || a.CritPermille == 0 || a.CritMultPermille == 0 ||
			a.ReactionMultPermille == 0 || a.ElementCap == 0 ||
			a.ReactionTier == 0 || a.ElementCoefPermille == 0 {
			t.Errorf("第 %d 关的攻方七项有零值（%+v）—— A 段每一项都要进哈希前缀，"+
				"零值会让「这项是否影响哈希」变成不可判定", c.LevelID, a)
		}
	}

	if withPicks == 0 {
		t.Error("没有任何带选牌的用例 —— 取牌是改变战斗的输入，必须有向量覆盖")
	}
	if withoutPicks == 0 {
		t.Error("没有任何「无选牌」的用例 —— 老战报缺该字段的退化路径必须被覆盖\n" +
			"（缺它的话，将来有人给选牌加上界时，那条路径无人守）")
	}
	if len(pickShapes) < 2 {
		t.Errorf("选牌形态只有 %d 种（%v）—— discardTake 与 discardSkip "+
			"是两条不同分支，必须都被覆盖", len(pickShapes), pickShapes)
	}
	if len(levelIDs) < 5 {
		t.Errorf("只覆盖了 %d 个关卡（%v）—— 难度谱两端与中段都要覆盖", len(levelIDs), levelIDs)
	}
}

// TestReplayContractCoversTerrainAndBoss 断言这批关卡里**确实有地形关与 BOSS 关**。
//
// 为什么要单独一条：地形与 BOSS 是两条独立的引擎分支
// （地形走 updateTerrain；BOSS 影响击杀分与部分反应系数），
// 而它们是**关卡内容**的属性、不在向量文件里 —— 只看向量的话，
// 「覆盖度」可以在一次都没触到它们的情况下显示全绿。
//
// 这与 README 记的「采样会掩盖分布问题」同源：
// 单看每个用例都自洽，合起来可能整类分支一次都没走到。
func TestReplayContractCoversTerrainAndBoss(t *testing.T) {
	doc := loadReplayVectors(t)

	raw, err := os.ReadFile(filepath.Join(filepath.Dir(thisFile),
		"..", "..", "testdata", "smoke_levels.json"))
	if err != nil {
		t.Fatalf("读 smoke_levels.json 失败：%v", err)
	}
	var fx struct {
		Levels []struct {
			ID      int  `json:"id"`
			IsBoss  bool `json:"is_boss"`
			Terrain []struct {
				Kind string `json:"kind"`
			} `json:"terrain"`
		} `json:"levels"`
	}
	if err := json.Unmarshal(raw, &fx); err != nil {
		t.Fatalf("smoke_levels.json 解析失败：%v", err)
	}

	covered := map[int]bool{}
	for _, c := range doc.Cases {
		covered[c.LevelID] = true
	}

	var terrain, boss int
	for _, l := range fx.Levels {
		if !covered[l.ID] {
			continue
		}
		if len(l.Terrain) > 0 {
			terrain++
		}
		if l.IsBoss {
			boss++
		}
	}
	if terrain == 0 {
		t.Errorf("向量覆盖的 %d 个关卡里**没有一个带地形** —— "+
			"updateTerrain / withinTerrain / withinEnemy 三条分支无人守", len(covered))
	}
	if boss == 0 {
		t.Errorf("向量覆盖的 %d 个关卡里**没有一个是 BOSS 关** —— "+
			"BOSS 的击杀分与反应系数分支无人守", len(covered))
	}
}
