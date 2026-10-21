package service

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// TestSettleStoresANonEmptyBuildSnapshot 守住 I-6 的**锚点**。
//
// # 为什么这条重要
//
// `battle_records.build_snapshot` 是回放哈希与验真的锚点：
// 客户端重放时要拿到「当时的技能与养成属性」才能算出同一个哈希。
// 若它被写成空的，重放必然对不上 —— 于是**所有正常对局都被判成伪造**。
//
// 修前结算路径上有 5 处 `marshalJSON` 的错误被 `_ =` 丢弃
// （`elements_used` / `reactions_used` / `terrain_used` / `build_snapshot` / `loot`）。
// 那一族「不参与奖励计算 ≠ 不需要校验」的缺陷：写坏了不报错，
// 运营看到的是「这一局没用任何元素 / 没触发任何反应 / 没有地形」，
// 而真相是「数据坏了」。
//
// # 判据
//
// 结算成功后，那五列都必须**有内容**（非 null、非空对象/数组）。
// 这条能失败：任何人把 `_ =` 放回去、或让某一列写空，它立刻红。
func TestSettleStoresANonEmptyBuildSnapshot(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})

	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	gl := domain.GenerateLevel(1)
	in := loseSettleInput()
	in.Result = "win"
	in.Kills = domain.MaxKillsFor(gl)
	in.Score = 2000
	in.ElementsUsed = map[string]int{"fire": 3}
	in.ReactionsUsed = map[string]int{"steam_burst": 1}
	in.TerrainUsed = []string{"oil_drum"}

	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, in); err != nil {
		t.Fatalf("结算失败：%v", err)
	}

	var (
		buildRaw, lootRaw, elementsRaw, reactionsRaw []byte
		terrainRaw                                   []byte
	)
	err = ts.pool.QueryRow(ctx, `
		SELECT build_snapshot, loot, elements_used, reactions_used, terrain_used
		FROM battle_records WHERE user_id = $1 ORDER BY id DESC LIMIT 1`, uid).
		Scan(&buildRaw, &lootRaw, &elementsRaw, &reactionsRaw, &terrainRaw)
	if err != nil {
		t.Fatalf("读战报失败：%v", err)
	}

	// 1) build_snapshot 必须是**非空对象**，且带得出重放所需的关键字段。
	var build map[string]any
	if err := json.Unmarshal(buildRaw, &build); err != nil {
		t.Fatalf("build_snapshot 不是合法 JSON：%v", err)
	}
	if len(build) == 0 {
		t.Fatal("build_snapshot 是空的 —— 重放拿不到技能与养成属性，" +
			"算出的哈希必然对不上，于是所有正常对局都会被判成伪造")
	}
	for _, key := range []string{"skills", "attacker"} {
		if _, ok := build[key]; !ok {
			t.Errorf("build_snapshot 缺关键字段 %q（重放靠它算哈希）", key)
		}
	}
	// settle_input 也必须在里面（审计与纠纷追溯用）
	if _, ok := build["settle_input"]; !ok {
		t.Error("build_snapshot 缺 settle_input（上报原文）")
	}

	// 2) 其余四列也不能是 null / 空。
	var loot map[string]any
	if err := json.Unmarshal(lootRaw, &loot); err != nil {
		t.Fatalf("loot 不是合法 JSON：%v", err)
	}
	var elements map[string]int
	if err := json.Unmarshal(elementsRaw, &elements); err != nil {
		t.Fatalf("elements_used 不是合法 JSON：%v", err)
	}
	if elements["fire"] != 3 {
		t.Errorf("elements_used 应记下 fire=3，实际 %v（写空 = 「这一局没用任何元素」）", elements)
	}
	var reactions map[string]int
	if err := json.Unmarshal(reactionsRaw, &reactions); err != nil {
		t.Fatalf("reactions_used 不是合法 JSON：%v", err)
	}
	if reactions["steam_burst"] != 1 {
		t.Errorf("reactions_used 应记下 steam_burst=1，实际 %v", reactions)
	}
	var terrain []string
	if err := json.Unmarshal(terrainRaw, &terrain); err != nil {
		t.Fatalf("terrain_used 不是合法 JSON：%v", err)
	}
	if len(terrain) != 1 || terrain[0] != "oil_drum" {
		t.Errorf("terrain_used 应记下 [oil_drum]，实际 %v", terrain)
	}
}
