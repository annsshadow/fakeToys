package domain

// 契约向量文件里的「卡牌接线」段落（Block F）。
//
// ⚠️ 与 `terrain_battle` 同样**独立解码**，不去共享的 `pureVectors` 里加字段。
// 理由见 `engine_terrain_vectors_test.go` 的注释（共享大结构体的改动半径太大）。

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

// cardStep 是一步战斗快照的期望值（F 块关心的字段）。
type cardStep struct {
	Tick      int64  `json:"tick"`
	Phase     string `json:"phase"`
	ElapsedMs int64  `json:"elapsed_ms"`
	Enemies   []struct {
		UID int64  `json:"uid"`
		X   string `json:"x"`
		HP  string `json:"hp"`
	} `json:"enemies"`
	Replay      []string `json:"replay"`
	Events      []string `json:"events"`
	BaseHP      string   `json:"base_hp"`
	Score       string   `json:"score"`
	Shots       int64    `json:"shots"`
	Hits        int64    `json:"hits"`
	Kills       int64    `json:"kills"`
	Leaked      int64    `json:"leaked"`
	TerrainUsed string   `json:"terrain_used"`
	// CardPicks 是「每波选了什么」的**完整序列**。
	//
	// ⚠️ 必须逐步记：它是服务端重算局内 buff 的唯一输入，
	// 而「第 2 波与第 3 波选反了」只看终值观察不到。
	CardPicks    string   `json:"card_picks"`
	Hand         []string `json:"hand"`
	DiscardsLeft int      `json:"discards_left"`
	Skills       []string `json:"skills"`
	Buffs        string   `json:"buffs"`
	Heat         string   `json:"heat"`
	HeatCap      string   `json:"heat_cap"`
	Overheated   bool     `json:"overheated"`
	Terrains     []struct {
		Kind      string `json:"kind"`
		State     string `json:"state"`
		Timer     string `json:"timer"`
		Charge    int    `json:"charge"`
		Angle     int    `json:"angle"`
		Triggered bool   `json:"triggered"`
	} `json:"terrains"`
}

// cardBattleVectors 是向量文件里 `card_battle` 那一段。
type cardBattleVectors struct {
	Rows []struct {
		Label        string         `json:"label"`
		LevelID      int            `json:"level_id"`
		Seed         string         `json:"seed"`
		Ticks        int            `json:"ticks"`
		SampleEvery  int            `json:"sample_every"`
		Equipped     []int          `json:"equipped"`
		LevelWaves   []waveLevelDef `json:"level_waves"`
		TerrainKinds []string       `json:"terrain_kinds"`
		// Picks 是**输入**：选牌脚本。
		//
		// ⚠️ 必须带进文件 —— 它就是这一局的「用户输入」，
		//   漏带的话 Go 侧只能猜，而不同脚本给出不同战斗。
		Picks    []int `json:"picks"`
		Attacker struct {
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
		Trace []cardStep `json:"trace"`
	} `json:"rows"`
}

// loadCardBattleVectors 只解码 `card_battle` 段。
func loadCardBattleVectors(t *testing.T) cardBattleVectors {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join("..", "..", "testdata", "engine_pure_vectors.json"))
	if err != nil {
		t.Fatalf("读向量文件失败：%v", err)
	}
	var doc struct {
		CardBattle cardBattleVectors `json:"card_battle"`
	}
	if err := json.Unmarshal(raw, &doc); err != nil {
		t.Fatalf("向量文件解析失败：%v", err)
	}
	return doc.CardBattle
}
