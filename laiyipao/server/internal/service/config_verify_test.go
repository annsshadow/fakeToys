package service

// 覆盖率收尾第二批：LoadGameConfig（纯内存组装）与若干可构造的错误分支。

import (
	"context"
	"encoding/json"
	"errors"
	"testing"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

// TestLoadGameConfig 配置下发是纯内存组装，不碰库 —— 全量字段一次断言。
// 客户端把这一包当作整个游戏的内容真相，缺一块就是端上白屏/功能缺失。
func TestLoadGameConfig(t *testing.T) {
	cfg := openTestService(t).LoadGameConfig()
	if len(cfg.Levels) != domain.TotalLevels {
		t.Errorf("levels 应为 %d 关，实际 %d", domain.TotalLevels, len(cfg.Levels))
	}
	if len(cfg.Enemies) == 0 || len(cfg.Skills) == 0 || len(cfg.Composite) == 0 {
		t.Error("enemies/skills/composite 不应为空")
	}
	if len(cfg.Equipment) == 0 || len(cfg.Gems) == 0 || len(cfg.Skins) == 0 {
		t.Error("equipment/gems/skins 不应为空")
	}
	if len(cfg.Qualities) == 0 || len(cfg.Chapters) == 0 {
		t.Error("qualities/chapters 不应为空")
	}
	if cfg.ScoreRules.OnKillNormal == 0 {
		t.Error("score_rules 必须下发（跨端契约源，客户端曾硬编码 5 处）")
	}
	if cfg.SkillRules.MaxLevel == 0 || cfg.SkillRules.BaseCost == 0 {
		t.Errorf("skill_rules 必须下发：%+v", cfg.SkillRules)
	}
	// 必须可序列化：下发路径就是 JSON
	if _, err := json.Marshal(cfg); err != nil {
		t.Errorf("配置包不可序列化：%v", err)
	}
}

// TestWalletCarriesTokenBalances 第 150 轮：`/wallet` 必须带 `tokens`。
//
// 背景：`TokenBalance` 这个 service 方法早就有，但 HTTP 层**没有任何端点**
// 读它 —— 于是 `mastery_reset` 买得到却看不见，重铸按钮无从判断该不该亮，
// 而「买了不知道干嘛」的商品是最容易被当成 bug 报上来的那种。
//
// 判据三条：① 字段在；② 白名单里每个道具都出现（哪怕余额 0）；
// ③ 买了之后余额真的会变 —— ③ 是唯一能证明「这不是一个恒为 0 的空壳」的。
func TestWalletCarriesTokenBalances(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	w, err := ts.LoadWallet(ctx, uid)
	if err != nil {
		t.Fatalf("LoadWallet 失败：%v", err)
	}
	if w.Tokens == nil {
		t.Fatal("wallet.tokens 为 nil —— 客户端无法判断道具余额")
	}
	// 白名单里每个道具都必须在，没买的也要出现（余额 0）
	for _, kind := range WalletTokenKinds {
		if _, ok := w.Tokens[kind]; !ok {
			t.Errorf("wallet.tokens 缺 %q（余额 0 也必须出现，"+
				"否则客户端无法区分「0 个」与「这种道具不存在」）", kind)
		}
	}
	// 数量对齐：多出来的键说明白名单之外的东西漏进了下发
	if len(w.Tokens) != len(WalletTokenKinds) {
		t.Errorf("wallet.tokens 有 %d 个键，白名单是 %d 个 —— 多余的键来自哪里？%v",
			len(w.Tokens), len(WalletTokenKinds), w.Tokens)
	}

	// ③ 发 2 件再读，余额必须真的变（否则这就是个恒 0 的空壳）
	if err := ts.pool.QueryRow(ctx, `
		INSERT INTO user_tokens (user_id, token, balance) VALUES ($1, 'mastery_reset', 2)
		ON CONFLICT (user_id, token) DO UPDATE SET balance = user_tokens.balance + 2
		RETURNING balance`, uid).Scan(new(int64)); err != nil {
		t.Fatalf("造道具余额失败：%v", err)
	}
	w2, err := ts.LoadWallet(ctx, uid)
	if err != nil {
		t.Fatalf("LoadWallet 失败：%v", err)
	}
	if w2.Tokens["mastery_reset"] != 2 {
		t.Errorf("mastery_reset 余额 = %d，期望 2 —— tokens 字段可能只是个恒 0 的空壳",
			w2.Tokens["mastery_reset"])
	}
}

// TestConfigVersionIsStableAndContentDerived 钉住 `version` 的内容哈希语义。
//
// 第 150 轮把 `version` 从 `time.Now().Unix() % 100000` 改成了内容哈希，
// 于是客户端**第一次**能回答「内容变没变」。这个测试守两半，缺一半都白做：
//
//	① 稳定 —— 两次 LoadGameConfig() 的 version 必须相同。
//	   不成立 ⇒ 客户端永远认为内容变了，缓存永不复用（性能回归）。
//	② 内容派生 —— 改任何一个内容字段，哈希必须变。
//	   不成立 ⇒ 客户端可能永远用旧内容（内容不刷新，比①更糟）。
//
// ② 用 mutation 的方式验：逐类内容各改一处再算一次哈希。
// 「内容反正不变哈希照样对」这种假绿是这类测试最常见的失效方式。
func TestConfigVersionIsStableAndContentDerived(t *testing.T) {
	svc := openTestService(t)

	a := svc.LoadGameConfig()
	b := svc.LoadGameConfig()
	if a.Version == "" {
		t.Fatal("version 为空 —— 客户端没有可比的版本标识")
	}
	if a.Version != b.Version {
		t.Errorf("version 不稳定：%q vs %q —— 内容没变而版本变，"+
			"客户端会永远重拉 163KB 配置", a.Version, b.Version)
	}
	if len(a.Version) != 16 {
		t.Errorf("version 应为 16 位十六进制，实际 %q —— 短版本号要能进日志与缓存键", a.Version)
	}

	// 关键不变式：下发的 Version 就是「两个易变字段（version 自身 /
	// server_time）置零后」的内容哈希。
	//
	// 这一条才是真正防回归的地方 —— 它钉的是**装载路径的哈希输入**，
	// 而不是哈希函数本身。谁要是哪天把 ServerTime 算进了装载时的哈希，
	// 这里立刻红，而症状（每次请求都是新版本）不会在任何别处出现。
	//
	// ⚠️ 不能反过来用「给两个不同 ServerTime 算哈希」来证明它不参与 ——
	// 那证的是哈希函数排除了 server_time 字段，而哈希函数**没有**排除，
	// 它只是收到零值而已。方向错了就会写出永远绿的假测试。
	// （第 150 轮就是这么写错的：第一次跑出来就是红的。）
	zeroed := a
	zeroed.Version = ""
	zeroed.ServerTime = time.Time{}
	if got := configVersionHash(zeroed); got != a.Version {
		t.Errorf("下发的 Version 与「易变字段置零后的内容哈希」不一致：%q vs %q ——"+
			"说明装载时的哈希输入与这里的定义对不上", got, a.Version)
	}

	// ② mutation：逐类内容各改一处，哈希都必须变。
	//
	// 这一半是**构造性**的（不是只跑一次看看绿）：任何一类内容被漏算，
	// 对应的那条就会红。只断言「可变」不断言「每类都变」的测试，
	// 会在加字段时悄悄失效。
	// 全局内容表的快照：mutation 跑完之后必须逐字不变。
	//
	// 这条断言是刻意加的 —— 上面那条「深拷贝」的失败症状远在几百行之外，
	// 而这里当场红。测试污染全局状态是这个仓库反复吃的一类亏
	//（工程约束 8：测试要验证意图，不是留下地雷）。
	globalRecipeOutput := domain.SeedRecipes[0].Output
	globalEnemyHP := domain.ScaleAllEnemies()[0].HP

	base := func() GameConfig {
		// ⚠️ 必须**深拷贝**。
		//
		// GameConfig 的切片字段（Levels/Enemies/Recipes/…）在 LoadGameConfig 里是
		// 直接别名包级内容表的（`Recipes: domain.SeedRecipes`），
		// 浅拷贝会让下面每一条 mutation 改写**进程全局状态**：
		//
		//	recipes[0].Output 31 → 32，于是 SeedRecipes 里有两个 Output=32，
		//	播种时两条都算出 id = 32*10 = 320 → 23505 撞 skill_recipes 主键。
		//
		// 症状离病因极远：不是这条测试红，而是**上百个跟内容毫无关系的用例**
		// 在 seeder.Run 那一步集体红，且只在该测试跑过之后才出现。
		// 第 150 轮就是这么排查了很久才定位到这里。
		var g GameConfig
		raw, err := json.Marshal(zeroed)
		if err != nil {
			t.Fatalf("深拷贝 GameConfig 失败（marshal）：%v", err)
		}
		if err := json.Unmarshal(raw, &g); err != nil {
			t.Fatalf("深拷贝 GameConfig 失败（unmarshal）：%v", err)
		}
		return g
	}
	mutations := []struct {
		name string
		fn   func(g *GameConfig)
	}{
		{"levels", func(g *GameConfig) { g.Levels[0].BaseHP += 1 }},
		{"enemies", func(g *GameConfig) { g.Enemies[0].HP += 1 }},
		{"skills", func(g *GameConfig) { g.Skills[0].BaseDamage += 1 }},
		{"composite", func(g *GameConfig) { g.Composite[0].BaseDamage += 1 }},
		{"recipes", func(g *GameConfig) { g.Recipes[0].Output += 1 }},
		{"equipment", func(g *GameConfig) { g.Equipment[0].BaseArmor += 1 }},
		{"gems", func(g *GameConfig) { g.Gems[0].Descr = "x" }},
		{"qualities", func(g *GameConfig) { g.Qualities[0].Mult += 1 }},
		{"skins", func(g *GameConfig) { g.Skins[0].Descr = "x" }},
		{"mastery", func(g *GameConfig) { g.Mastery[0].Family = "x" }},
		{"reactions", func(g *GameConfig) { g.Reactions[0].BaseCoef += 1 }},
		{"chapters", func(g *GameConfig) { g.Chapters[0].Name = "x" }},
		{"rating_weights", func(g *GameConfig) { g.RatingW.ElementCoverage += 1 }},
		{"score_rules", func(g *GameConfig) { g.ScoreRules.OnKillNormal += 1 }},
		{"skill_rules", func(g *GameConfig) { g.SkillRules.MaxLevel += 1 }},
	}
	for _, m := range mutations {
		g := base()
		m.fn(&g)
		if got := configVersionHash(g); got == a.Version {
			t.Errorf("改 %s 后 version 没变 —— 哈希漏算了这一类内容，"+
				"客户端会把改动当成没改", m.name)
		}
	}

	// 收尾：mutation 不许留下任何全局痕迹。
	//
	// 断言放在这里而不是逐条中间，是为了让失败信息一次说清「改了什么」，
	// 而不是第一条就炸、后面 14 条再也跑不到。
	if got := domain.SeedRecipes[0].Output; got != globalRecipeOutput {
		t.Fatalf("mutation 改写了 domain.SeedRecipes[0].Output（%d → %d）——"+
			"base() 的深拷贝没生效。这些改动会污染同进程后续所有测试："+
			"播种时两条配方算出同一个 id，直接撞 skill_recipes 主键。",
			globalRecipeOutput, got)
	}
	if got := domain.ScaleAllEnemies()[0].HP; got != globalEnemyHP {
		t.Fatalf("mutation 改写了敌人血量（%d → %d）——同上，深拷贝没生效",
			globalEnemyHP, got)
	}
}

// TestToInt64JSONNumber json.Number 分支：decoder.UseNumber 场景。
func TestToInt64JSONNumber(t *testing.T) {
	v, ok := toInt64(json.Number("42"))
	if !ok || v != 42 {
		t.Errorf("toInt64(json.Number(42)) = %d, %t", v, ok)
	}
	if _, ok := toInt64(json.Number("x")); ok {
		t.Error("非法 json.Number 应返回 false")
	}
}

// TestBrokenAccountPaths 故障态下的账号域错误出口。
func TestBrokenAccountPaths(t *testing.T) {
	broken := openBrokenService(t)
	ctx := context.Background()
	if _, err := broken.GuestLogin(ctx, "tok", "n"); err == nil {
		t.Error("GuestLogin 故障态应报错")
	}
	if _, err := broken.ResolveUser(ctx, "any"); err == nil {
		t.Error("ResolveUser 故障态应报错")
	}
	if _, _, err := broken.ListDefenses(ctx, 1); err == nil {
		t.Error("ListDefenses 故障态应报错")
	}
	if err := broken.AllocateMastery(ctx, 1, 1); err == nil {
		t.Error("AllocateMastery 故障态应报错")
	}
}

// TestGuestLoginMidwayFailures 登录链路的中段失败（upsert 成功之后）。
func TestGuestLoginMidwayFailures(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// user_wallets 拿走：upsert 用户成功、initNewUser 第一步失败
	ts.renameTable(t, "user_wallets", "user_wallets_bak")
	_, err := ts.GuestLogin(ctx, "midway_guest", "n")
	ts.renameTable(t, "user_wallets_bak", "user_wallets")
	mustErr(t, err, "init wallet")
}

// TestValidateDefenseOwnershipQueryFailures 归属校验的三段查询各自失败。
func TestValidateDefenseOwnershipQueryFailures(t *testing.T) {
	ctx := context.Background()

	t.Run("装备归属查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_equipment", "user_equipment_bak")
		err := ts.validateDefenseOwnership(ctx, uid, SaveDefenseInput{Equipment: []int{1}})
		ts.renameTable(t, "user_equipment_bak", "user_equipment")
		if err == nil || errors.Is(err, ErrBadInput) {
			t.Fatalf("查询失败不应伪装成业务拒绝：%v", err)
		}
	})

	t.Run("专精归属查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_mastery_nodes", "user_mastery_nodes_bak")
		err := ts.validateDefenseOwnership(ctx, uid, SaveDefenseInput{MasteryNodes: []int{1}})
		ts.renameTable(t, "user_mastery_nodes_bak", "user_mastery_nodes")
		if err == nil || errors.Is(err, ErrBadInput) {
			t.Fatalf("查询失败不应伪装成业务拒绝：%v", err)
		}
	})
}
