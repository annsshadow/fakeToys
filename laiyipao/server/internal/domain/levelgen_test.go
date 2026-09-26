package domain

import (
	"encoding/json"
	"testing"
)

// 生成器必须是确定性的：同样的输入必须产出完全一样的关卡。
// 这是 I-6（回放哈希可复现）的前置条件，也是关卡数据能在
// 客户端 / 服务端 / 后台三处保持一致的基础。
func TestGenerateLevelIsDeterministic(t *testing.T) {
	for _, id := range []int{1, 7, 20, 21, 45, 60, 81, 95, 100} {
		a := GenerateLevel(id)
		b := GenerateLevel(id)
		if a.Seed != b.Seed {
			t.Errorf("关卡 %d seed 不一致：%d vs %d", id, a.Seed, b.Seed)
		}
		if a.BaseHP != b.BaseHP {
			t.Errorf("关卡 %d 防线血量不一致：%d vs %d", id, a.BaseHP, b.BaseHP)
		}
		if len(a.Waves) != len(b.Waves) {
			t.Errorf("关卡 %d 波次数不一致：%d vs %d", id, len(a.Waves), len(b.Waves))
		}
		for w := range a.Waves {
			if len(a.Waves[w].Spawns) != len(b.Waves[w].Spawns) {
				t.Errorf("关卡 %d 第 %d 波刷怪组数不一致", id, w)
				continue
			}
			for g := range a.Waves[w].Spawns {
				if a.Waves[w].Spawns[g] != b.Waves[w].Spawns[g] {
					t.Errorf("关卡 %d 第 %d 波第 %d 组刷怪不一致：%+v vs %+v",
						id, w, g, a.Waves[w].Spawns[g], b.Waves[w].Spawns[g])
				}
			}
		}
		if len(a.Terrain) != len(b.Terrain) {
			t.Errorf("关卡 %d 地形数量不一致：%d vs %d", id, len(a.Terrain), len(b.Terrain))
		}
	}
}

// 章节边界必须正确：第 20 关属第 1 章，第 21 关属第 2 章。
func TestChapterBoundaries(t *testing.T) {
	cases := map[int]int{
		1: 1, 20: 1, 21: 2, 40: 2, 41: 3, 60: 3, 61: 4, 80: 4, 81: 5, 95: 5, 96: 6, 100: 6,
	}
	for levelID, wantChapter := range cases {
		if got := ChapterOf(levelID).ID; got != wantChapter {
			t.Errorf("关卡 %d 应属第 %d 章，实际第 %d 章", levelID, wantChapter, got)
		}
	}
}

// 难度必须随关卡单调递增（允许章节切换处的台阶，但不能倒退）。
func TestDifficultyIsMonotonic(t *testing.T) {
	prev := 0
	for id := 1; id <= 100; id++ {
		gl := GenerateLevel(id)
		if gl.Difficulty < prev {
			t.Errorf("关卡 %d 难度倒退：%d < 上一关 %d", id, gl.Difficulty, prev)
		}
		prev = gl.Difficulty
	}
}

// 全部 100 关的基本合法性：波次非空、有刷怪、星门槛递增。
func TestAllGeneratedLevelsAreValid(t *testing.T) {
	all := GenerateAllLevels()
	if len(all) != 100 {
		t.Fatalf("应生成 100 关，实际 %d", len(all))
	}
	terrainLevels := 0
	for _, gl := range all {
		if len(gl.Waves) == 0 {
			t.Errorf("关卡 %d 没有任何波次", gl.ID)
			continue
		}
		if gl.BaseHP <= 0 {
			t.Errorf("关卡 %d 防线血量 %d 非法", gl.ID, gl.BaseHP)
		}
		if gl.EnergyCost <= 0 {
			t.Errorf("关卡 %d 体力消耗 %d 非法", gl.ID, gl.EnergyCost)
		}
		if len(gl.StarTargets) != 3 {
			t.Errorf("关卡 %d 应有 3 档星级门槛，实际 %d", gl.ID, len(gl.StarTargets))
			continue
		}
		for i := 1; i < 3; i++ {
			if gl.StarTargets[i] <= gl.StarTargets[i-1] {
				t.Errorf("关卡 %d 星级门槛非递增：%v", gl.ID, gl.StarTargets)
				break
			}
		}
		if gl.TotalEnemies() <= 0 {
			t.Errorf("关卡 %d 敌人总数为 0", gl.ID)
		}
		// 刷怪必须引用本章敌人池
		ch := ChapterOf(gl.ID)
		allowed := map[int]bool{}
		for _, id := range ch.EnemyIDs {
			allowed[id] = true
		}
		if ch.BossEnemyID != 0 {
			allowed[ch.BossEnemyID] = true
		}
		for _, w := range gl.Waves {
			for _, s := range w.Spawns {
				if !allowed[s.EnemyID] {
					t.Errorf("关卡 %d 第 %d 波引用了不属于第 %d 章的敌人 %d",
						gl.ID, w.Index, ch.ID, s.EnemyID)
				}
				if s.Count <= 0 {
					t.Errorf("关卡 %d 第 %d 波刷怪数量 %d 非法", gl.ID, w.Index, s.Count)
				}
			}
		}
		if len(gl.Terrain) > 0 {
			terrainLevels++
		}
	}
	// I-4 要求约 25% 的关卡是地形关
	if terrainLevels < 20 {
		t.Errorf("地形关只有 %d 关，低于设计目标 25 关（100 关的 25%%）", terrainLevels)
	}
	t.Logf("地形关数量：%d / 100", terrainLevels)
}

// BOSS 关必须在最后一波出现 BOSS。
func TestBossLevelHasBossInFinalWave(t *testing.T) {
	for _, id := range []int{16, 56, 91, 96, 100} {
		gl := GenerateLevel(id)
		if !gl.IsBoss {
			continue
		}
		last := gl.Waves[len(gl.Waves)-1]
		ch := ChapterOf(id)
		found := false
		for _, s := range last.Spawns {
			if s.EnemyID == ch.BossEnemyID {
				found = true
			}
		}
		if !found {
			t.Errorf("关卡 %d 标记为 BOSS 关，但最后一波没有 BOSS(敌人 %d)", id, ch.BossEnemyID)
		}
	}
}

// 地形数组必须始终是可遍历的 JSON 数组，不能是 null。
// json.Marshal(nil slice) 产出 null，客户端遍历时会直接炸 —— 这条测试守住它。
func TestTerrainAlwaysMarshalsToArrayNotNull(t *testing.T) {
	for _, gl := range GenerateAllLevels() {
		raw, err := json.Marshal(gl.Terrain)
		if err != nil {
			t.Fatalf("关卡 %d 地形序列化失败：%v", gl.ID, err)
		}
		if string(raw) == "null" {
			t.Fatalf("关卡 %d 的地形被序列化为 null，客户端遍历会崩溃", gl.ID)
		}
		if len(gl.Terrain) == 0 && string(raw) != "[]" {
			t.Fatalf("关卡 %d 无地形时应序列化为 []，实际 %s", gl.ID, raw)
		}
		if len(gl.Terrain) > 0 && string(raw) == "[]" {
			t.Fatalf("关卡 %d 有 %d 件地形却序列化为空数组", gl.ID, len(gl.Terrain))
		}
		// 往返校验：反序列化后必须与原切片一致
		var back []TerrainPlacement
		if err := json.Unmarshal(raw, &back); err != nil {
			t.Fatalf("关卡 %d 地形反序列化失败：%v", gl.ID, err)
		}
		if len(back) != len(gl.Terrain) {
			t.Fatalf("关卡 %d 地形往返后长度不一致：%d vs %d", gl.ID, len(back), len(gl.Terrain))
		}
	}
}
func TestTerrainPlacementsAreValid(t *testing.T) {
	valid := map[string]bool{
		"oil_drum": true, "tidal_gate": true, "rotor_vane": true,
		"collapse_wall": true, "charge_tower": true,
	}
	for _, gl := range GenerateAllLevels() {
		for i, tp := range gl.Terrain {
			if !valid[tp.Kind] {
				t.Errorf("关卡 %d 地形 %d 种类非法：%q", gl.ID, i, tp.Kind)
			}
			if tp.X < 0 || tp.X > 1000 {
				t.Errorf("关卡 %d 地形 %d x 坐标越界：%d", gl.ID, i, tp.X)
			}
			if tp.Y < 0 || tp.Y > 1000 {
				t.Errorf("关卡 %d 地形 %d y 坐标越界：%d", gl.ID, i, tp.Y)
			}
		}
	}
}

// LCG 必须与 TS 侧产生相同序列。这里锁定 seed=12345 的前三个输出作为
// 跨端契约向量 —— TS 侧实现见 miniapp/src/game/lcg.ts（用 BigInt 做 64 位运算）。
//
// 这三个值已用 Node/BigInt 独立验证过，与 Go 的 uint64 结果逐位一致：
//
//	node lcgcheck(12345) => 6917529027818027904, 1269031133, 9223372037807892195
//
// 若此测试失败，说明 LCG 实现被改动，TS 侧必须同步，否则关卡数据与
// 回放哈希会在两端产生分歧（I-6 直接失效）。
func TestLCGKnownVectors(t *testing.T) {
	r := newLCG(12345)
	want := []uint64{
		6917529027818027904,
		1269031133,
		9223372037807892195,
	}
	for i, w := range want {
		if got := r.Next(); got != w {
			t.Errorf("LCG 第 %d 个值不匹配：got=%d want=%d —— TS 侧 lcg.ts 必须同步",
				i+1, got, w)
		}
	}
}

// Intn 必须在 [0,n) 内，且 n<=0 时安全返回 0。
func TestLCGIntnRange(t *testing.T) {
	for _, seed := range []int64{0, 1, 42, 12345, -1} {
		r := newLCG(seed)
		for i := 0; i < 500; i++ {
			v := r.Intn(10)
			if v < 0 || v >= 10 {
				t.Fatalf("seed=%d 第 %d 次 Intn(10) 越界：%d", seed, i, v)
			}
		}
	}
	r := newLCG(7)
	for _, n := range []int{0, -1, -100} {
		if got := r.Intn(n); got != 0 {
			t.Errorf("Intn(%d) 应返回 0，实际 %d", n, got)
		}
	}
}
