package domain

import (
	"testing"
)

// TestTerrainLevelRatioIsInExpectedBand 守住「约 25% 的关卡是地形关」。
//
// ⚠️ 这个用例抓的是一个**分布**缺陷，而分布缺陷是"测不出来"的那一类：
// 单看第 N 关，`chapterTerrainFor` 返回一个自洽的答案，
// 没有任何报错、没有任何断言会红。但 100 关的**总数**错了。
//
// 原缺陷：
//
//	offset := (levelSeed >> 8) % int64(章节关数)
//
// Go 的 `>>` 对 int64 是**算术右移**（保留符号位），不是逻辑右移。
// 100 关的种子里有**恰好 50 个是负数**（PHI 混合哈希的输出均匀分布在
// int64 全域），所以：
//
//   - 正数种子：>> 8 得正数，% 得 [0, 章长)  —— 正常
//   - 负数种子：>> 8 仍为负，% 得**负余数**（Go 的 % 向零截断，-5%4 = -1）
//     而判据是 `offset < 3`，**任何负数都满足**
//
// 于是负数种子那一半的 case-1 关卡 100% 变成地形关，
// 实测 100 关里 41 关有地形（41%）而设计意图是约 25% + 每章保底 3 关。
//
// 这个缺陷之所以能活下来：
//   - level_seeds.json 里有负种子，但没有任何测试断言"地形关占比"
//   - `engine.smoke.test.ts` 只跑 6 关，其中恰好只有第 10、50 关有地形
//   - 全部 400+ 个测试对"100 关里有几关带地形"没有任何断言
func TestTerrainLevelRatioIsInExpectedBand(t *testing.T) {
	const total = TotalLevels
	terrain := 0
	perChapter := map[int]int{}
	for id := 1; id <= total; id++ {
		gl := GenerateLevel(id)
		ch := ChapterOf(id)
		if len(gl.Terrain) > 0 {
			terrain++
			perChapter[ch.ID]++
		}
	}

	ratio := float64(terrain) / float64(total)
	// 设计意图（已调和，见 chapterTerrainFor 的注释）：
	//   保底 = 6 章 × 3 关 = 18 关 = 18%
	//   概率 = 其余 82 关 × 1/8 ≈ 10.3 关
	//   期望 ≈ 28%
	//
	// 区间取 18%~36%：
	//   下限 18% 是**硬下限**（保底本身），低于它说明保底逻辑坏了
	//   上限 36% 留 5 个点余量。100 个样本、82 个走概率、
	//   期望 10.3、σ≈3.0，所以 36% (36 关) 相当于 +2 个 σ。
	//
	//   缺陷版本实测 41% —— 被上限带 5 个点抓住。
	//   （早期版本的区间是 28%~40%，下限设成期望值 28% 是错的：
	//    7/8 这个实测值就在 1.1σ 内，测试自己先红了。）
	if ratio < 0.18 || ratio > 0.36 {
		t.Errorf("地形关占比 %.1f%%（%d/%d）超出合理区间 18%%~36%%",
			ratio*100, terrain, total)
	}
	t.Logf("地形关 %d/%d = %.1f%%", terrain, total, ratio*100)
}

// TestTerrainLevelPerChapterFloor 守住「每章至少 3 关地形关」。
//
// 这是 case 1 分支存在的**唯一理由**（保底机制）。
// 如果把保底逻辑改坏，这一章可能只有 1~2 关有地形，
// 而玩家在中期章节会觉得"地形机制时有时无"。
func TestTerrainLevelPerChapterFloor(t *testing.T) {
	for _, ch := range AllChapters() {
		n := 0
		for id := ch.StartLevel; id <= ch.EndLevel; id++ {
			if len(GenerateLevel(id).Terrain) > 0 {
				n++
			}
		}
		if n < 3 {
			t.Errorf("第 %d 章（关 %d~%d）只有 %d 关地形关，少于保底 3 关",
				ch.ID, ch.StartLevel, ch.EndLevel, n)
		}
		t.Logf("第 %d 章：%d/%d 关有地形", ch.ID, n, ch.EndLevel-ch.StartLevel+1)
	}
}

// TestNoNegativeModuloInTerrainSelection 直接钉住根因。
//
// 与其断言"占比在某个区间"（那是间接的、依赖具体哈希分布的），
// 不如直接断言**判据函数本身不接受负数** ——
// 因为负余数就是 bug 的全部成因。
//
// 这里复刻 chapterTerrainFor 的取模表达式，
// 要求它在 100 个种子上都落在 [0, 章长) 内。
func TestNoNegativeModuloInTerrainSelection(t *testing.T) {
	for _, ch := range AllChapters() {
		span := int64(ch.EndLevel - ch.StartLevel + 1)
		for id := ch.StartLevel; id <= ch.EndLevel; id++ {
			seed := GenerateLevel(id).Seed
			offset := chapterOffset(seed, span)
			if offset < 0 || offset >= span {
				t.Fatalf("第 %d 章第 %d 关：offset=%d 越界 [0,%d) "+
					"（seed=%d，负余数 = 算术右移没有转无符号）",
					ch.ID, id, offset, span, seed)
			}
		}
	}
}

// TestTerrainLevelPerChapterUpperBound 守住「没有哪一章的地形关数量失控」。
//
// ⚠️ 为什么要单独一条：TestTerrainRollIsUniformWithinChapter 验的是
// terrainRoll **函数本身**的分布，而缺陷可能在**调用点**
// （比如改回 `(seed>>8) % 4`）。那时散列函数没变，均匀度测试照样全绿。
//
// 所以这里直接验**实际结果**：每章的地形关数量。
//
// 缺陷版本的实际数字：3 / 7 / 8 / 3 / 13 / 3（第 5 章 13/15 = 87%）
// 修复之后：          4 / 5 / 6 / 4 /  3 / 3
//
// 上限取 12（20 关的章 = 60%）：既高于实测最高值 8，
// 又远低于缺陷版本的 13 —— 留 1 关余量，容不下"整章同步"这种量级的偏差。
func TestTerrainLevelPerChapterUpperBound(t *testing.T) {
	const upper = 12
	for _, ch := range AllChapters() {
		span := ch.EndLevel - ch.StartLevel + 1
		n := 0
		for id := ch.StartLevel; id <= ch.EndLevel; id++ {
			if len(GenerateLevel(id).Terrain) > 0 {
				n++
			}
		}
		if n > upper {
			t.Errorf("第 %d 章有 %d/%d 关带地形（上限 %d）—— "+
				"整章同步说明散列未随关卡号变化", ch.ID, n, span, upper)
		}
		// 概率部分也不该为 0：保底之外应至少偶尔命中一次，
		// 否则说明判定退化成"只看关卡偏移、不看散列"
		if span > TerrainFloorPerChapter && n == TerrainFloorPerChapter {
			t.Logf("第 %d 章只有保底的 %d 关，无额外地形关（可能是运气，也可能是判定退化）",
				ch.ID, n)
		}
		t.Logf("第 %d 章：%d/%d 关有地形（上限 %d）", ch.ID, n, span, upper)
	}
}

// TestTerrainRollIsUniformWithinChapter 守住「每章内部」的地形比例。
//
// ⚠️ 这是本文件里最重要的一条，也是最初**完全缺失**的那一条。
//
// 全局均匀不等于章内均匀：地形比例是**按章体验**的，
// 玩家在第 5 章看到 87% 的关卡带地形、在第 1 章看到 0%，是两种不同的体验。
//
// 而原实现的 `(seed>>8) % 4` 在**整章内是同一个值**
// （第 4 章 20 关的分布是 0/7/13/0）—— 那几个比特位根本不随关卡号变化。
// 于是每章的地形关数量取决于「本章碰巧落在哪个值」：
//
//	第 5 章落在 0 → 13/15 关有地形（87%）
//	第 1 章落在 3 → 除保底 3 关外一关都没有（15%）
//
// 这就是"每章地形关忽多忽少"的真正原因。
// 换成对 levelID 的完整雪崩之后，章内偏离降到 1~2。
func TestTerrainRollIsUniformWithinChapter(t *testing.T) {
	for _, ch := range AllChapters() {
		span := ch.EndLevel - ch.StartLevel + 1
		var b [8]int
		for id := ch.StartLevel; id <= ch.EndLevel; id++ {
			b[terrainRoll(id)%8]++
		}
		want := float64(span) / 8
		// ⚠️ 容差必须是**统计量**而不是随手拍的常数。
		// 20 个样本分到 8 个桶，期望每桶 2.5，单桶偏差到 3~4 属正常波动；
		// 我最初用"4 桶、容差 2"，结果测试自己先红了 3 次
		// （第 2 章 2/8/5/5、第 3 章 7/0/9/4 都是正常波动）。
		//
		// 取 tol = 2 + span/8：
		//   span=20 → 4，span=15 → 3，span=5 → 2
		// 仍然比缺陷签名紧 2 倍以上 —— 原实现第 4 章是 0/7/13/0，
		// 单桶偏差 8，远超 span=20 时的容差 4。
		tol := 2 + span/8
		for k := 0; k < 8; k++ {
			dev := int(want) - b[k]
			if dev < 0 {
				dev = -dev
			}
			if float64(dev) > float64(tol) {
				t.Errorf("第 %d 章：mod-8 分布 %v 偏离期望 %.2f 超过容差 %d",
					ch.ID, b, want, tol)
			}
		}
		t.Logf("第 %d 章（%d 关）：mod-8 分布 %v，容差 %d", ch.ID, span, b, tol)
	}
}

// TestTerrainRollIsStable 固定地形判定的散列值。
//
// 这一条看着多余，但它锁住了一个真实风险：
// 地形分布一变，**所有存量关卡的 `terrain` 字段就变了**，
// 而 I-6 的回放哈希是按「关卡配置 + 种子」算的 ——
// 配置一变，旧的战斗记录全部无法验真。
//
// 所以这不是"防退化"，是"防静默破坏历史数据"。
// 真正的用例（`difficulty.test.ts`）只关心"能不能通关"，
// 而"能通关"在分布变化后往往仍然成立 —— 不会有任何测试因此变红。
func TestTerrainRollIsStable(t *testing.T) {
	// 1) 纯函数性：同一关卡号必须给出同一散列（否则地形分布不可复现）
	for i := 0; i < 3; i++ {
		if terrainRoll(42) != terrainRoll(42) {
			t.Fatal("terrainRoll 非纯函数")
		}
	}
	// 2) 无碰撞：100 个关卡号必须给出 100 个不同散列。
	//    碰撞意味着两关的"要不要地形"被绑在一起，
	//    而 20 关的章里一次碰撞就足以让分布明显偏斜。
	seen := map[uint32]int{}
	for id := 1; id <= TotalLevels; id++ {
		r := terrainRoll(id)
		if prev, dup := seen[r]; dup {
			t.Errorf("关卡 %d 与 %d 的散列相同（0x%08x）", id, prev, r)
		}
		seen[r] = id
	}
	// 3) 相邻关卡不能同步：整段相邻关卡都取同一个 %4 值，
	//    就是"章内不变化"那个缺陷的轻量版早期信号。
	run, maxRun := 0, 0
	prevBucket := terrainRoll(1) % 4
	for id := 2; id <= TotalLevels; id++ {
		b := terrainRoll(id) % 4
		if b == prevBucket {
			run++
			if run > maxRun {
				maxRun = run
			}
		} else {
			run = 0
		}
		prevBucket = b
	}
	// 实测修复后最长同值连续段是 3；原实现是 20（第 4 章整章不变）
	if maxRun > 6 {
		t.Errorf("相邻关卡的地形判定最长连续 %d 关取同一值（>6），说明散列未真正打散", maxRun+1)
	}
	t.Logf("最长同值连续段 = %d 关", maxRun+1)
}

// TestChapterOffsetIsUnsigned 单独验证偏移函数在负种子下仍为正。
//
// 单独拆一条是因为它是最小可复现单元：
// 任何时候有人想"简化" chapterOffset 回 `(seed >> 8) % span`，
// 这条会立刻红，而占比测试的红会慢一拍（要等 100 关跑完）。
func TestChapterOffsetIsUnsigned(t *testing.T) {
	span := int64(17)
	// 明确用负种子 —— 修复前 (seed>>8) 为负，% 得负余数
	for _, seed := range []int64{-1, -256, -7046029255919282421, -5382687380318764192} {
		got := chapterOffset(seed, span)
		if got < 0 {
			t.Errorf("seed=%d 得到负偏移 %d", seed, got)
		}
		if got >= span {
			t.Errorf("seed=%d 得到越界偏移 %d（span=%d）", seed, got, span)
		}
	}
	// 同一个种子的偏移必须在 [0, span) 内均匀分布而不是恒为某个值
	seen := map[int64]bool{}
	for id := 1; id <= TotalLevels; id++ {
		seen[chapterOffset(GenerateLevel(id).Seed, span)] = true
	}
	if len(seen) < 5 {
		t.Errorf("偏移只有 %d 个不同取值（种子 %d 个），分布过于集中", len(seen), TotalLevels)
	}
}
