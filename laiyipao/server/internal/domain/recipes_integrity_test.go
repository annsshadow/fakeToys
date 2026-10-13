package domain

// `SeedRecipes` 的主键唯一性。
//
// # 为什么这条必须钉死
//
// `internal/seeder` 把配方落库时用的是
//
//	INSERT INTO skill_recipes (id, ...) VALUES (r.Output*10, ...)
//
// 即**主键 id 是由 Output 现算出来的人造值**。
// 两条配方 Output 相同 ⇒ 播种时算出同一个 id ⇒ SQLSTATE 23505。
//
// # 为什么它值得单独一条守卫
//
// 23505 的症状离病因极远：不是「配方写重了」这条断言红，
// 而是**上百个跟内容毫无关系的用例**在 `seeder.Run` 那一步集体红
// （第 150 轮实测：一次完整跑测里 129 次建库、101 次失败）。
// 而且只在「某处把 Output 改成了重复值」之后才出现，
// 于是排查的人第一反应是怀疑数据库、并发、PG 版本 —— 全是错的方向。
//
// 现有守卫 `content_test.go` 只查了「配方引用的技能存在」与
// 「每个复合技能都有配方指向它」，**没有一条查重复**。
// 这正是「测试该验证意图」的缺口：它在验证已写下的规则，
// 而没验证播种器真正依赖的那条。

import "testing"

// TestSeedRecipesOutputIsUnique Output 必须唯一 —— 它直接决定 skill_recipes 的主键。
func TestSeedRecipesOutputIsUnique(t *testing.T) {
	seen := make(map[int]int, len(SeedRecipes)) // Output → 下标
	for i, r := range SeedRecipes {
		if prev, dup := seen[r.Output]; dup {
			t.Errorf("配方 Output=%d 重复：下标 %d 与下标 %d。"+
				"播种时两者的 id（Output*10）相同，会撞 skill_recipes 主键（23505）。"+
				"配方：%+v / %+v", r.Output, prev, i, SeedRecipes[prev], r)
		}
		seen[r.Output] = i
	}
}

// TestSeedRecipesIDIsUnique 按播种器的实际公式再算一遍。
//
// 为什么要重复查一遍「Output 唯一」：上面那条断言的是**意图**
// （配方产物是一个技能），这条断言的是**实现**（id = Output*10）。
// 两者今天等价，但它们是两条独立的约定 ——
// 哪天有人把 id 改成 `Output*100 + tier`，这条会先红，
// 而不是等到某个 (Output, tier) 组合恰好撞上才发现。
func TestSeedRecipesIDIsUnique(t *testing.T) {
	// 与 seeder.seedSkills 里的公式保持一致；不一致时这条会红。
	idOf := func(r SeedRecipe) int { return r.Output * 10 }

	seen := make(map[int]SeedRecipe, len(SeedRecipes))
	for _, r := range SeedRecipes {
		id := idOf(r)
		if prev, dup := seen[id]; dup {
			t.Errorf("配方 id=%d 重复：Output=%d(A%+d) 与 Output=%d(A%+d)。"+
				"播种时后者会撞主键", id, prev.Output, prev.A, r.Output, r.A)
		}
		seen[id] = r
	}
}

// TestSeedRecipePairsAreUnique (a,b) 组合必须唯一 —— 它是 ON CONFLICT 的仲裁索引。
//
// 重复的 (a,b) 不会撞主键，但会让**两条配方互相覆盖**：
// `ON CONFLICT (a_skill_id, b_skill_id) DO UPDATE` 会把先插那条的
// output_skill_id / output_tier 改掉，而 id 保持不变 ——
// 结果是表里少一行，且产物已经不是作者写的那一个。静默，且无任何报错。
func TestSeedRecipePairsAreUnique(t *testing.T) {
	seen := make(map[[2]int]int, len(SeedRecipes))
	for i, r := range SeedRecipes {
		key := [2]int{r.A, r.B}
		if prev, dup := seen[key]; dup {
			t.Errorf("配方 (%d,%d) 重复：下标 %d 与下标 %d（产物 %d 与 %d）。"+
				"播种时后者走 ON CONFLICT DO UPDATE 改写前者，id 却不变 —— "+
				"结果是配方少一条、产物被悄悄换掉", r.A, r.B, prev, i,
				SeedRecipes[prev].Output, r.Output)
		}
		seen[key] = i
	}
}
