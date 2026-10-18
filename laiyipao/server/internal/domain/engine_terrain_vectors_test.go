package domain

// 契约向量文件里的「地形接线」段落。
//
// # ⚠️ 刻意**独立解码**，不去共享的 `pureVectors` 里加字段
//
// 我第一版是把 `TerrainBattle` 加进 `pureVectors` 的，结果那一次会话里
// 因为拼接方式不当，连续三次把同文件里**其它块**的守卫连带截断
// （`TestValidateSnapshotMatchesCrossEnd` / `TestDeckScriptsMatchCrossEnd` /
// `TestSnapshotDigestMatchesCrossEnd`），每次都要从 git 恢复。
//
// 共享一个大结构体的代价是：任何一处改动都要重新审视整个文件。
// 而地形这一段**只有 E 块用**，独立解码让它的改动半径缩到一个文件里。
//
// # 键名必须带 `Battle` 后缀
//
// `terrain` 这个键已被更早的「地形纯函数契约」占用
// （ROTOR_DIR_1000 / onHit / onKillCharging 的记录型调用序列）。
// 两者性质完全不同：那份是**记录调用**的假引擎，这份是**真引擎**的逐 tick 对齐。
// 用同一个键名会让其中一份被静默覆盖 —— 而两份都绿。

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

// terrainBattleVectors 是向量文件里 `terrain_battle` 那一段。
type terrainBattleVectors struct {
	Rows []struct {
		Label       string         `json:"label"`
		LevelID     int            `json:"level_id"`
		Seed        string         `json:"seed"`
		Ticks       int            `json:"ticks"`
		SampleEvery int            `json:"sample_every"`
		Equipped    []int          `json:"equipped"`
		LevelWaves  []waveLevelDef `json:"level_waves"`
		// ⚠️ 只回显**种类**，不回显位置/参数：
		//   位置与参数由两端从同一份 `smoke_levels.json` 读，
		//   回显了就变成「用 Go 的地形表生成期望值」。
		//   buildTerrainEngine 会重新读夹具并逐项核对这份列表。
		TerrainKinds []string `json:"terrain_kinds"`
		Attacker     struct {
			Attack                 string `json:"attack"`
			CritPermille           string `json:"crit_permille"`
			CritMultiplierPermille string `json:"crit_multiplier_permille"`
			ReactionMultPermille   string `json:"reaction_mult_permille"`
			ElementCap             string `json:"element_cap"`
			ReactionTier           string `json:"reaction_tier"`
			ElementCoefPermille    string `json:"element_coef_permille"`
			HeatCapPermille        string `json:"heat_cap_permille"`
			ArmorPermille          string `json:"armor_permille"`
			MechanicPermille       string `json:"mechanic_permille"`
		} `json:"attacker"`
		Trace []terrainStep `json:"trace"`
	} `json:"rows"`
}

// loadTerrainBattleVectors 只解码 `terrain_battle` 段。
func loadTerrainBattleVectors(t *testing.T) terrainBattleVectors {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join("..", "..", "testdata", "engine_pure_vectors.json"))
	if err != nil {
		t.Fatalf("读向量文件失败：%v", err)
	}
	var doc struct {
		TerrainBattle terrainBattleVectors `json:"terrain_battle"`
	}
	if err := json.Unmarshal(raw, &doc); err != nil {
		t.Fatalf("向量文件解析失败：%v", err)
	}
	return doc.TerrainBattle
}
