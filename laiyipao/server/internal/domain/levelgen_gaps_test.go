package domain

// 关卡生成器（levelgen.go）与元素表（elements.go）的边界补漏。
//
// 这里全部是「上一轮没走到」的分支：兜底返回、防御性截断、
// 未知输入的 fail-closed。单独成文件的定位：它们不是行为主路径，
// 但每一个都对应"调用方的 bug 或坏数据进来时系统如何表现"。

import "testing"

// TestChapterOfFallsBackToLastChapter 兜底分支：越界 levelID
// （0 或超过 TotalLevels）回退到最后一章，而不是 panic 或返回零值章。
// GenerateLevel 全量调用它——这里钉的是"约定之外的输入"。
func TestChapterOfFallsBackToLastChapter(t *testing.T) {
	last := chapters[len(chapters)-1]
	for _, id := range []int{0, -1, TotalLevels + 1, 1 << 30} {
		if got := ChapterOf(id); got.ID != last.ID {
			t.Errorf("ChapterOf(%d) 应兜底到最后一章 %d，实际 %d", id, last.ID, got.ID)
		}
	}
	// 合法区间抽查：首关在第一章、末关在最后一章
	if got := ChapterOf(1); got.ID != chapters[0].ID {
		t.Errorf("第 1 关应在第一章，实际 %d", got.ID)
	}
}

// TestDefaultScoreRulesMatchesConstants 钉住「唯一定义处」：
// DefaultScoreRules 必须逐字段等于包内常量。
// 若有人直接改函数返回值而不改常量（或反过来），
// levelgen 估算、结算裁剪、cmd/vectors 导出三处会各自为政。
func TestDefaultScoreRulesMatchesConstants(t *testing.T) {
	r := DefaultScoreRules()
	if r.PerDamageUnit != scorePerDamageUnit {
		t.Errorf("PerDamageUnit = %d，常量 %d", r.PerDamageUnit, scorePerDamageUnit)
	}
	if r.OnKillNormal != scoreOnKillNormal {
		t.Errorf("OnKillNormal = %d，常量 %d", r.OnKillNormal, scoreOnKillNormal)
	}
	if r.OnKillBoss != scoreOnKillBoss {
		t.Errorf("OnKillBoss = %d，常量 %d", r.OnKillBoss, scoreOnKillBoss)
	}
	if r.StarTargetRatio != StarTargetRatio {
		t.Errorf("StarTargetRatio = %v，常量 %v", r.StarTargetRatio, StarTargetRatio)
	}
	if r.ScoreFullAtSec != ScoreFullAtSec {
		t.Errorf("ScoreFullAtSec = %d，常量 %d", r.ScoreFullAtSec, ScoreFullAtSec)
	}
}

// TestChapterTerrainForRejectsMismatchedLevel 越界守卫：
// levelID 与章不匹配（调用方 bug）时返回无地形，
// 而不是让越界 offset 走进后面的散列判定。
func TestChapterTerrainForRejectsMismatchedLevel(t *testing.T) {
	ch := chapters[0] // 1..20
	// 越界：levelID 远超章尾
	if kind, ok := chapterTerrainFor(ch, 500, 0); ok {
		t.Errorf("levelID=500 不属于第 1 章，应返回无地形，实际 (%q, true)", kind)
	}
	// 章前 3 关是保底地形关（TerrainFloorPerChapter），必须给地形
	for id := ch.StartLevel; id < ch.StartLevel+TerrainFloorPerChapter; id++ {
		if kind, ok := chapterTerrainFor(ch, id, 0); !ok || kind != ch.TerrainKind {
			t.Errorf("保底地形关 %d 应返回 (%q, true)，实际 (%q, %v)", id, ch.TerrainKind, kind, ok)
		}
	}
}

// TestChapterOffsetSpanGuard span <= 0 时返回 0 而不是 panic/负数。
// span 来自章定义，若配错成 0，生成器必须仍然产出合法的关卡。
func TestChapterOffsetSpanGuard(t *testing.T) {
	if got := chapterOffset(12345, 0); got != 0 {
		t.Errorf("span=0 应返回 0，实际 %d", got)
	}
	if got := chapterOffset(12345, -5); got != 0 {
		t.Errorf("span<0 应返回 0，实际 %d", got)
	}
	// 正常路径：结果必须落在 [0, span)
	for seed := int64(0); seed < 1000; seed++ {
		if got := chapterOffset(seed, 20); got < 0 || got >= 20 {
			t.Fatalf("chapterOffset(%d, 20) = %d 越界", seed, got)
		}
	}
}

// TestGenerateWaveClampsOversizedProgress 池截断分支：
// index 超过 total-1（调用方 bug）会让 progress > 1，
// 池大小必须被夹回 len(EnemyIDs)，而不是越界索引。
func TestGenerateWaveClampsOversizedProgress(t *testing.T) {
	ch := chapters[0] // EnemyIDs 4 个
	rng := newLCG(42)
	w := generateWave(rng, ch, 5, 1, 1, false) // index=5 >> total=1
	if len(w.Spawns) == 0 {
		t.Fatal("应产出刷怪指令")
	}
	for _, s := range w.Spawns {
		found := false
		for _, id := range ch.EnemyIDs {
			if s.EnemyID == id {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("刷怪指令引用了池外敌人 %d（池 = %v）", s.EnemyID, ch.EnemyIDs)
		}
	}
}

// TestGenerateTerrainDedupCoordinates 去重循环：同一关内的地形 X 坐标
// 不得重叠。去重重画分支几乎无法通过固定关卡触达（650 个位置取 2~3 个），
// 这里用已探明的种子（rng 预推进 1 次、seed=1097、count=3 时第二次
// x 抽取撞上第一次）直接驱动真实函数走进"碰撞 → 重画"路径。
//
// 重画发生在函数内部、输出不可见，所以用 rng 状态推进步数来证实：
// generateTerrain 每件地形消耗 3 次抽取（x、param、y）+ 1 次 count，
// 实测消耗 11 步而"无重画"的期望是 10 步 —— 多出的 1 步就是重画。
func TestGenerateTerrainDedupCoordinates(t *testing.T) {
	const seed = int64(1097)

	rng := newLCG(seed)
	rng.Next() // 模拟真实调用链：地形生成前 rng 已被波次生成消费

	before := rng.state
	terrain := generateTerrain(rng, "oil_drum", 1)
	steps := lcgStepsTo(before, rng.state)

	count := len(terrain)
	if count < 1 || count > 3 {
		t.Fatalf("地形件数应在 1..3，实际 %d", count)
	}
	// 无重画期望：1（count）+ count*3（每件 x/param/y）
	if want := 1 + count*3; steps != want+1 {
		t.Fatalf("种子 %d 应恰好触发 1 次重画（消耗 %d 步，期望 %d）；若不再触发，说明抽取顺序被改动，需重探种子",
			seed, steps, want+1)
	}

	// 契约断言：无论碰撞与否，输出的 X 坐标必须两两不同（地形叠加是渲染与交互 bug）
	xs := map[int]bool{}
	for _, tp := range terrain {
		if xs[tp.X] {
			t.Errorf("地形 X = %d 重复——去重失效，地形会叠在一起", tp.X)
		}
		xs[tp.X] = true
		if tp.X < 250 || tp.X > 900 {
			t.Errorf("地形 X = %d 超出取值范围 [250, 900)", tp.X)
		}
	}
}

// lcgStepsTo 从 from 推进 LCG 状态直到 to，返回步数；不可达返回 -1。
func lcgStepsTo(from, to uint64) int {
	s := from
	for i := 1; i <= 64; i++ {
		s = s*6364136223846793005 + 1442695040888963407
		if s == to {
			return i
		}
	}
	return -1
}

// TestItoaEdges itoa 的三个分支：零、负数、多字节正数。
// 它拼进错误消息与文件名，负号位置错了会产出 "5-" 这类乱码。
func TestItoaEdges(t *testing.T) {
	for _, c := range []struct {
		in   int
		want string
	}{
		{0, "0"},
		{-5, "-5"},
		{-1234567, "-1234567"},
		{7, "7"},
		{123456789, "123456789"},
	} {
		if got := itoa(c.in); got != c.want {
			t.Errorf("itoa(%d) = %q，期望 %q", c.in, got, c.want)
		}
	}
}

// TestIsElementRejectsUnknown 非法元素必须被拒——元素名会进数据库与 GROUP BY。
func TestIsElementRejectsUnknown(t *testing.T) {
	for _, s := range []string{"", "Fire", "fire ", "poison", "fogo"} {
		if IsElement(s) {
			t.Errorf("IsElement(%q) 应为 false", s)
		}
	}
	for _, e := range AllElements() {
		if !IsElement(string(e)) {
			t.Errorf("IsElement(%q) 应为 true", e)
		}
	}
}

// TestSpecUnknownReactionFailsClosed 未知反应键返回零值 Spec 而不是 panic。
// 反应键最终来自客户端报文（ForceReaction），这里是不信任边界的最后一环。
func TestSpecUnknownReactionFailsClosed(t *testing.T) {
	got := ReactionKey("not_a_reaction").Spec()
	if got.Key != "" || got.BaseCoef != 0 || got.StatusDurationMs != 0 {
		t.Errorf("未知反应应返回零值 Spec，实际 %+v", got)
	}
}

// TestLookupReactionUnknownElement 未知 incoming 元素必须 fail-closed
// 返回无反应，而不是在表中查到零值键。
func TestLookupReactionUnknownElement(t *testing.T) {
	if k, ok := LookupReaction(ElementFire, Element("bogus")); ok {
		t.Errorf("未知 incoming 应无反应，实际 %q", k)
	}
	if k, ok := LookupReaction(Element("bogus"), ElementFire); ok {
		t.Errorf("未知 existing 应无反应，实际 %q", k)
	}
	// 相同元素不反应（设计契约：时序反应的前提是两种不同元素）
	if k, ok := LookupReaction(ElementFire, ElementFire); ok {
		t.Errorf("同元素不应反应，实际 %q", k)
	}
	// 正常路径抽查：焰后冰 = 蒸汽爆发
	if k, ok := LookupReaction(ElementFire, ElementIce); !ok || k != ReactionSteamBurst {
		t.Errorf("焰后冰应为蒸汽爆发，实际 (%q, %v)", k, ok)
	}
}
