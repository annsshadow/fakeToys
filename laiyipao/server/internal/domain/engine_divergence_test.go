package domain

// 跨端对齐的「已知差异表」。
//
// # 这张表存在的理由
//
// 跨端对齐是**逐字段**比对的，但端口是**分块**推进的：
// 在 B 完成、C 未完成的时刻，有两个字段 TS 会推进而 Go 侧静止
// （`x` 与 `spawn_progress`，因为它们由 `updateEnemies` 驱动）。
//
// 当时的处理是「跳过这两个字段，并把它们写进这张表，指定由哪一块消除」。
// 之所以不直接跳过：
//
//	静默跳过会变成**没人记得的债**。C 落地时若忘了把它们改成真正比对，
//	本文件会继续绿（因为它从来没比过那两个值）——
//	而那正是 C 该抓的 bug。
//
// # 生命周期
//
//	每一条差异都由 `TestNoKnownCrossEndDivergences` 盯着：
//	C 落地后这张表清空、字段改成真正比对，那条测试继续绿；
//	若将来又出现新的未比对字段（忘了加进表），它会红。
import "testing"

var knownCrossEndDivergences = map[string]string{}

// TestNoKnownCrossEndDivergences 盯着上面那张表。
//
// ⚠️ 这条测试的判据是「表必须为空」，而不是「表里的每条都还成立」。
//
// 后者（B 块的做法）是**反向**检查：一旦 Go 侧开始动就红，
// 提醒把比对改成真的。它能工作，但要求「表里必须非空」——
// 于是清空表的那一刻它自己会红，而那正是应该发生的时刻。
//
// 现在的做法是「表为空 = 全部比对 = 好」：
// 表里多一条就红，且消息直接指向「这个字段你在比对，但你登记了它不比对」。
func TestNoKnownCrossEndDivergences(t *testing.T) {
	if len(knownCrossEndDivergences) != 0 {
		t.Errorf("跨端对齐仍有 %d 个「已登记但不比对」的字段：%v\n"+
			"要么把它们改成真正比对（并从这里删掉），"+
			"要么它们其实已经一致了（那更说明该删掉登记）",
			len(knownCrossEndDivergences), knownCrossEndDivergences)
	}
}

// TestCrossEndCoversMotionFields 确认 `x` 与 `spawn_progress` **真的**在比对。
//
// ⚠️ 这条不是形式检查。
//
//	B 块时期这两个字段被登记为「已知差异」而跳过，
//
// 守卫当时的形态是「断言它们**仍然**有差异」。
//
//	C 落地后若只是把登记删掉、却忘了把比对加回去，
//
// 「表为空」那条会绿，而这两个字段实际上又变成了静默跳过。
//
// 所以这里单独确认比对代码**确实存在**且在跑：
// 跑一遍第 1 关，若 x 与 spawn_progress 全程不变，那比对必然是死的。
func TestCrossEndCoversMotionFields(t *testing.T) {
	v := loadPureVectors(t)
	if len(v.Wave.Rows) == 0 {
		t.Fatal("向量里没有波次场景")
	}

	movedX, movedProgress := false, false
	for _, r := range v.Wave.Rows {
		e := buildWaveEngine(t, waveRow{
			Label: r.Label, LevelID: r.LevelID, Seed: r.Seed, Ticks: r.Ticks,
			SampleEvery:           r.SampleEvery,
			AttackerArmorPermille: r.AttackerArmorPermille,
			LevelBaseHP:           r.LevelBaseHP,
			LevelArmorPermille:    r.LevelArmorPermille,
			LevelWaves:            r.LevelWaves,
			Trace:                 r.Trace,
		})
		e.Start()
		// 出生点 x：StepEnemyMotion 之前全体相同，用它判断「有没有动过」
		const birthX = 960000
		for i := 0; i < r.Ticks; i++ {
			if e.Phase != PhaseWave && e.Phase != PhasePrepare {
				break
			}
			e.Step()
			for _, en := range e.Enemies {
				if en.X != birthX {
					movedX = true
				}
				if en.SpawnProgress != 0 && en.SpawnProgress != SpawnProgressFull {
					movedProgress = true
				}
			}
			if movedX && movedProgress {
				break
			}
		}
	}
	if !movedX {
		t.Error("Go 侧 x 全程停在出生点 —— " +
			"说明 `updateEnemies` 没接上，或 `checkWaveTick` 里的 x 比对是死的。" +
			"若它被登记进 knownCrossEndDivergences，那是漏消除的差异")
	}
	if !movedProgress {
		t.Error("Go 侧 spawnProgress 全程为 0 —— " +
			"说明出场动画没推进，或比对是死的")
	}
}
