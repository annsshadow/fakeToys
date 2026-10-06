// Package seeder 把 domain 层的游戏内容与生成的 100 关写入数据库。
//
// 幂等：可重复执行。配置类数据（敌人/技能/装备/宝石/皮肤/专精树/关卡）
// 采用 upsert 覆盖；玩家数据一律不碰。
package seeder

import (
	"context"
	"encoding/json"
	"fmt"

	pgx "github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"

	"github.com/laiyipao/server/internal/domain"
)

// Seed 写入全部游戏内容。返回各类实体的写入条数。
type SeedResult struct {
	Enemies    int
	Skills     int
	Recipes    int
	Equipment  int
	Gems       int
	Skins      int
	Mastery    int
	Levels     int
	Waves      int
	SignInDays int
	Tasks      int
	ShopItems  int
}

// Run 执行全部种子写入。
func Run(ctx context.Context, pool *pgxpool.Pool) (SeedResult, error) {
	var res SeedResult
	tx, err := pool.Begin(ctx)
	if err != nil {
		return res, fmt.Errorf("begin: %w", err)
	}
	defer tx.Rollback(ctx) //nolint:errcheck // 正常路径已 Commit，Rollback 返回 ErrTxClosed

	if err := seedEnemies(ctx, tx, &res); err != nil {
		return res, err
	}
	if err := seedSkills(ctx, tx, &res); err != nil {
		return res, err
	}
	if err := seedEquipmentAndGems(ctx, tx, &res); err != nil {
		return res, err
	}
	if err := seedSkins(ctx, tx, &res); err != nil {
		return res, err
	}
	if err := seedMastery(ctx, tx, &res); err != nil {
		return res, err
	}
	if err := seedLevels(ctx, tx, &res); err != nil {
		return res, err
	}
	if err := seedSignIn(ctx, tx, &res); err != nil {
		return res, err
	}
	if err := seedTasks(ctx, tx, &res); err != nil {
		return res, err
	}
	if err := seedShop(ctx, tx, &res); err != nil {
		return res, err
	}
	if err := seedRedeemCodes(ctx, tx); err != nil {
		return res, err
	}

	if err := tx.Commit(ctx); err != nil {
		return res, fmt.Errorf("commit: %w", err)
	}
	return res, nil
}

func seedEnemies(ctx context.Context, tx pgx.Tx, res *SeedResult) error {
	for _, e := range domain.SeedEnemies {
		_, err := tx.Exec(ctx, `
			INSERT INTO enemies (id, code, name, category, hp, speed, armor, shield_hp,
			                    attack, attack_range, attack_interval, fly_height, burrow, is_boss)
			VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)
			ON CONFLICT (id) DO UPDATE SET
				code=EXCLUDED.code, name=EXCLUDED.name, category=EXCLUDED.category,
				hp=EXCLUDED.hp, speed=EXCLUDED.speed, armor=EXCLUDED.armor,
				shield_hp=EXCLUDED.shield_hp, attack=EXCLUDED.attack,
				attack_range=EXCLUDED.attack_range, attack_interval=EXCLUDED.attack_interval,
				fly_height=EXCLUDED.fly_height, burrow=EXCLUDED.burrow, is_boss=EXCLUDED.is_boss`,
			e.ID, e.Code, e.Name, e.Category, e.HP, e.Speed, e.Armor, e.ShieldHP,
			e.Attack, e.AttackRange, e.AttackEvery, e.FlyHeight, e.Burrow, e.IsBoss)
		if err != nil {
			return fmt.Errorf("insert enemy %d: %w", e.ID, err)
		}
		for _, el := range domain.AllElements() {
			if _, err := tx.Exec(ctx, `
				INSERT INTO enemy_element_resist (enemy_id, element, resist_pct)
				VALUES ($1,$2,$3)
				ON CONFLICT (enemy_id, element) DO UPDATE SET resist_pct=EXCLUDED.resist_pct`,
				e.ID, string(el), e.Resist[el]); err != nil {
				return fmt.Errorf("insert resist enemy %d element %s: %w", e.ID, el, err)
			}
		}
		res.Enemies++
	}
	return nil
}

func seedSkills(ctx context.Context, tx pgx.Tx, res *SeedResult) error {
	// 基础技能 + 配方产物技能，都是 skills 表中的真实行
	all := make([]domain.SeedSkill, 0, len(domain.SeedSkills)+len(domain.SeedCompositeSkills))
	all = append(all, domain.SeedSkills...)
	all = append(all, domain.SeedCompositeSkills...)

	for _, s := range all {
		if _, err := tx.Exec(ctx, `
			INSERT INTO skills (id, code, name, family, element, kind, descr, base_damage,
			                   heat_cost, cooldown_ms, pierce, aoe_radius, apply_element,
			                   apply_stacks, projectile_speed, chain, unlock_level)
			VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17)
			ON CONFLICT (id) DO UPDATE SET
				code=EXCLUDED.code, name=EXCLUDED.name, family=EXCLUDED.family,
				element=EXCLUDED.element, kind=EXCLUDED.kind, descr=EXCLUDED.descr,
				base_damage=EXCLUDED.base_damage, heat_cost=EXCLUDED.heat_cost,
				cooldown_ms=EXCLUDED.cooldown_ms, pierce=EXCLUDED.pierce,
				aoe_radius=EXCLUDED.aoe_radius, apply_element=EXCLUDED.apply_element,
				apply_stacks=EXCLUDED.apply_stacks, projectile_speed=EXCLUDED.projectile_speed,
				chain=EXCLUDED.chain, unlock_level=EXCLUDED.unlock_level`,
			s.ID, s.Code, s.Name, s.Family, string(s.Element), s.Kind, s.Descr, s.BaseDamage,
			s.HeatCost, s.CooldownMs, s.Pierce, s.AoeRadius, string(s.ApplyElement),
			s.ApplyStacks, s.ProjectileSpeed, s.Chain, s.UnlockLevel); err != nil {
			return fmt.Errorf("insert skill %d: %w", s.ID, err)
		}
		res.Skills++
	}
	for _, r := range domain.SeedRecipes {
		if _, err := tx.Exec(ctx, `
			INSERT INTO skill_recipes (id, output_skill_id, a_skill_id, b_skill_id, output_tier)
			VALUES ($1,$2,$3,$4,$5)
			ON CONFLICT (a_skill_id, b_skill_id) DO UPDATE SET
				output_skill_id=EXCLUDED.output_skill_id, output_tier=EXCLUDED.output_tier`,
			r.Output*10, r.Output, r.A, r.B, r.OutTier); err != nil {
			return fmt.Errorf("insert recipe %d: %w", r.Output, err)
		}
		res.Recipes++
	}
	return nil
}

func seedEquipmentAndGems(ctx context.Context, tx pgx.Tx, res *SeedResult) error {
	for _, e := range domain.SeedEquipmentList {
		if _, err := tx.Exec(ctx, `
			INSERT INTO equipment (id, code, name, slot, tier, element, descr, base_armor,
			                       base_bonus_pct, unlock_level)
			VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
			ON CONFLICT (id) DO UPDATE SET
				code=EXCLUDED.code, name=EXCLUDED.name, slot=EXCLUDED.slot, tier=EXCLUDED.tier,
				element=EXCLUDED.element, descr=EXCLUDED.descr, base_armor=EXCLUDED.base_armor,
				base_bonus_pct=EXCLUDED.base_bonus_pct, unlock_level=EXCLUDED.unlock_level`,
			e.ID, e.Code, e.Name, e.Slot, e.Tier, string(e.Element), e.Descr,
			e.BaseArmor, e.BaseBonusPct, e.UnlockLevel); err != nil {
			return fmt.Errorf("insert equipment %d: %w", e.ID, err)
		}
		res.Equipment++
	}
	for _, g := range domain.SeedGems {
		if _, err := tx.Exec(ctx, `
			INSERT INTO gems (id, code, name, attr) VALUES ($1,$2,$3,$4)
			ON CONFLICT (id) DO UPDATE SET code=EXCLUDED.code, name=EXCLUDED.name, attr=EXCLUDED.attr`,
			g.ID, g.Code, g.Name, g.Attr); err != nil {
			return fmt.Errorf("insert gem %d: %w", g.ID, err)
		}
		res.Gems++
	}
	return nil
}

func seedSkins(ctx context.Context, tx pgx.Tx, res *SeedResult) error {
	for _, s := range domain.SeedSkins {
		if _, err := tx.Exec(ctx, `
			INSERT INTO skins (id, code, name, rarity, passive, unlock_type)
			VALUES ($1,$2,$3,$4,$5,'shop')
			ON CONFLICT (id) DO UPDATE SET
				code=EXCLUDED.code, name=EXCLUDED.name, rarity=EXCLUDED.rarity, passive=EXCLUDED.passive`,
			s.ID, s.Code, s.Name, s.Rarity, s.Passive); err != nil {
			return fmt.Errorf("insert skin %d: %w", s.ID, err)
		}
		res.Skins++
	}
	return nil
}

func seedMastery(ctx context.Context, tx pgx.Tx, res *SeedResult) error {
	for _, fam := range domain.AllMasteryFamilies() {
		if _, err := tx.Exec(ctx, `
			INSERT INTO mastery_trees (family, name) VALUES ($1,$2)
			ON CONFLICT (family) DO UPDATE SET name=EXCLUDED.name`,
			fam.Family, fam.Name); err != nil {
			return fmt.Errorf("insert mastery tree %s: %w", fam.Family, err)
		}
		for _, n := range fam.Nodes {
			if _, err := tx.Exec(ctx, `
				INSERT INTO mastery_nodes (id, family, layer, slot, name, kind, value, prereq_family, prereq_layer)
				VALUES ($1,$2,$3,$4,$5,$6,$7,'',$8)
				ON CONFLICT (family, layer, slot) DO UPDATE SET
					name=EXCLUDED.name, kind=EXCLUDED.kind, value=EXCLUDED.value,
					prereq_layer=EXCLUDED.prereq_layer`,
				n.ID, n.Family, n.Layer, n.Slot, n.Name, n.Kind, n.Value, n.PrereqLay); err != nil {
				return fmt.Errorf("insert mastery node %d: %w", n.ID, err)
			}
			res.Mastery++
		}
	}
	return nil
}

func seedLevels(ctx context.Context, tx pgx.Tx, res *SeedResult) error {
	for _, gl := range domain.GenerateAllLevels() {
		starTargets, err := json.Marshal(gl.StarTargets)
		if err != nil {
			return fmt.Errorf("marshal star targets level %d: %w", gl.ID, err)
		}
		terrain, err := json.Marshal(gl.Terrain)
		if err != nil {
			return fmt.Errorf("marshal terrain level %d: %w", gl.ID, err)
		}
		if _, err := tx.Exec(ctx, `
			INSERT INTO levels (id, chapter, name, seed, base_hp, wave_count, difficulty,
			                    energy_cost, star_targets, terrain_config, is_boss, enabled)
			VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,TRUE)
			ON CONFLICT (id) DO UPDATE SET
				chapter=EXCLUDED.chapter, name=EXCLUDED.name, seed=EXCLUDED.seed,
				base_hp=EXCLUDED.base_hp, wave_count=EXCLUDED.wave_count,
				difficulty=EXCLUDED.difficulty, energy_cost=EXCLUDED.energy_cost,
				star_targets=EXCLUDED.star_targets, terrain_config=EXCLUDED.terrain_config,
				is_boss=EXCLUDED.is_boss`,
			gl.ID, gl.Chapter, gl.Name, gl.Seed, gl.BaseHP, gl.WaveCount, gl.Difficulty,
			gl.EnergyCost, starTargets, terrain, gl.IsBoss); err != nil {
			return fmt.Errorf("insert level %d: %w", gl.ID, err)
		}
		res.Levels++

		for _, w := range gl.Waves {
			spawns, err := json.Marshal(w.Spawns)
			if err != nil {
				return fmt.Errorf("marshal spawns level %d wave %d: %w", gl.ID, w.Index, err)
			}
			if _, err := tx.Exec(ctx, `
				INSERT INTO level_waves (level_id, wave_index, spawns) VALUES ($1,$2,$3)
				ON CONFLICT (level_id, wave_index) DO UPDATE SET spawns=EXCLUDED.spawns`,
				gl.ID, w.Index, spawns); err != nil {
				return fmt.Errorf("insert waves level %d: %w", gl.ID, err)
			}
			res.Waves++
		}
	}
	return nil
}

func seedSignIn(ctx context.Context, tx pgx.Tx, res *SeedResult) error {
	// 新手七日签到：金币递增，第 7 天给钻石
	for day := 1; day <= 7; day++ {
		reward := map[string]int{"coin": 1000 * day}
		if day == 3 || day == 7 {
			reward["gem"] = 20 * day
		}
		if day == 7 {
			reward["energy"] = 50
		}
		raw, err := json.Marshal(reward)
		if err != nil {
			return err
		}
		if _, err := tx.Exec(ctx, `
			INSERT INTO sign_in_calendar (day_index, reward) VALUES ($1,$2)
			ON CONFLICT (day_index) DO UPDATE SET reward=EXCLUDED.reward`,
			day, raw); err != nil {
			return fmt.Errorf("insert sign-in day %d: %w", day, err)
		}
		res.SignInDays++
	}
	return nil
}

func seedTasks(ctx context.Context, tx pgx.Tx, res *SeedResult) error {
	tasks := []struct {
		id, target                int
		code, name, scope, metric string
		reward                    map[string]int
	}{
		{1, 30, "daily_kills_30", "今日击杀 30 个敌人", "daily", "kills", map[string]int{"coin": 800}},
		{2, 3, "daily_clears_3", "今日通关 3 关", "daily", "clears", map[string]int{"coin": 1000, "energy": 10}},
		{3, 20, "daily_reactions_20", "今日触发 20 次元素反应", "daily", "reactions", map[string]int{"coin": 1200, "gem": 5}},
		{4, 1, "daily_signin", "今日完成签到", "daily", "signin", map[string]int{"coin": 500}},
		{5, 200, "weekly_kills_200", "本周累计击杀 200 个敌人", "weekly", "kills", map[string]int{"coin": 5000, "gem": 20}},
		{6, 20, "weekly_clears_20", "本周累计通关 20 关", "weekly", "clears", map[string]int{"coin": 6000, "keys": 2}},
		{7, 150, "weekly_reactions_150", "本周累计触发 150 次元素反应", "weekly", "reactions", map[string]int{"gem": 40}},
		{8, 1, "ach_first_clear", "首次通关任意关卡", "achievement", "clears", map[string]int{"gem": 30}},
		// ⚠️ 第 92 轮：target 从 1 改成关号。
		//
		// `max_stage` 的 bump 值是**刚结算那一关的关号**，
		// 而 `bumpTasks` 对 achievement 走 `GREATEST(progress, value)`
		// —— 所以 progress 的语义是「**历史上到达过的最高关号**」。
		//
		// 「达到 N 关」的达成条件应当是 `progress >= N`，
		// 而 target 写 1 时条件变成 `progress >= 1` ——
		// **通关第 1 关就解锁**。
		//
		// 实测（只结算第 1 关）：
		//
		//	ach_reach_20   progress=1 target=1  可领取（应为不可）
		//	ach_reach_60   progress=1 target=1  可领取（应为不可）
		//	ach_reach_100  progress=1 target=1  可领取 —— 500 钻石！
		//
		// 三项合计 710 钻石只要清第 1 关。
		{9, 20, "ach_reach_20", "抵达第 20 关", "achievement", "max_stage", map[string]int{"gem": 60}},
		{10, 60, "ach_reach_60", "抵达第 60 关", "achievement", "max_stage", map[string]int{"gem": 150}},
		{11, 100, "ach_reach_100", "通关第 100 关", "achievement", "max_stage", map[string]int{"gem": 500}},
		{12, 100, "ach_reactions_100", "累计触发 100 次元素反应", "achievement", "reactions", map[string]int{"gem": 80}},
	}
	for _, t := range tasks {
		raw, err := json.Marshal(t.reward)
		if err != nil {
			return err
		}
		if _, err := tx.Exec(ctx, `
			INSERT INTO tasks (id, code, name, scope, target, metric, reward, enabled)
			VALUES ($1,$2,$3,$4,$5,$6,$7,TRUE)
			ON CONFLICT (id) DO UPDATE SET
				name=EXCLUDED.name, scope=EXCLUDED.scope, target=EXCLUDED.target,
				metric=EXCLUDED.metric, reward=EXCLUDED.reward`,
			t.id, t.code, t.name, t.scope, t.target, t.metric, raw); err != nil {
			return fmt.Errorf("insert task %d: %w", t.id, err)
		}
		res.Tasks++
	}
	return nil
}

func seedShop(ctx context.Context, tx pgx.Tx, res *SeedResult) error {
	items := []struct {
		id                   int
		code, name, category string
		price, grant         map[string]int
		limit                int
	}{
		{1, "shop_coin_small", "金币袋", "currency", map[string]int{"gem": 10}, map[string]int{"coin": 20000}, 3},
		{2, "shop_coin_big", "金币箱", "currency", map[string]int{"gem": 50}, map[string]int{"coin": 120000}, 1},
		{3, "shop_energy", "体力补给", "currency", map[string]int{"gem": 20}, map[string]int{"energy": 30}, 2},
		{4, "shop_keys", "钥匙串", "currency", map[string]int{"gem": 30}, map[string]int{"keys": 3}, 1},
		{5, "shop_retry", "重试符", "boost", map[string]int{"coin": 5000}, map[string]int{"revive_token": 1}, 5},
		{6, "shop_mastery_reset", "专精点重置", "boost", map[string]int{"gem": 80}, map[string]int{"mastery_reset": 1}, 0},
		{7, "shop_firstpay", "首充礼包", "starter", map[string]int{"coin": 0}, map[string]int{"gem": 300, "coin": 50000, "energy": 50}, 1},
		{8, "shop_gem_wash", "洗练石", "boost", map[string]int{"coin": 20000}, map[string]int{"gem_wash_token": 1}, 3},
	}
	for _, it := range items {
		priceRaw, err := json.Marshal(it.price)
		if err != nil {
			return err
		}
		grantRaw, err := json.Marshal(it.grant)
		if err != nil {
			return err
		}
		if _, err := tx.Exec(ctx, `
			INSERT INTO shop_items (id, code, name, category, price, payload, limit_per_day, sort_order, enabled)
			VALUES ($1,$2,$3,$4,$5,$6,$7,$8,TRUE)
			ON CONFLICT (id) DO UPDATE SET
				name=EXCLUDED.name, category=EXCLUDED.category, price=EXCLUDED.price,
				payload=EXCLUDED.payload, limit_per_day=EXCLUDED.limit_per_day,
				sort_order=EXCLUDED.sort_order`,
			it.id, it.code, it.name, it.category, priceRaw, grantRaw, it.limit, it.id); err != nil {
			return fmt.Errorf("insert shop item %d: %w", it.id, err)
		}
		res.ShopItems++
	}
	return nil
}

func seedRedeemCodes(ctx context.Context, tx pgx.Tx) error {
	codes := []struct {
		id      int
		code    string
		reward  map[string]int
		maxUses int
	}{
		{1, "LAIYIPAO", map[string]int{"coin": 30000, "gem": 50}, 1},
		{2, "ELEMENT5", map[string]int{"gem": 100, "energy": 30}, 1},
		{3, "REACTION", map[string]int{"gem": 200, "keys": 3}, 1},
	}
	for _, c := range codes {
		raw, err := json.Marshal(c.reward)
		if err != nil {
			return err
		}
		if _, err := tx.Exec(ctx, `
			INSERT INTO redeem_codes (id, code, reward, max_uses, used_count, enabled)
			VALUES ($1,$2,$3,$4,0,TRUE)
			ON CONFLICT (id) DO UPDATE SET
				code=EXCLUDED.code, reward=EXCLUDED.reward, max_uses=EXCLUDED.max_uses`,
			c.id, c.code, raw, c.maxUses); err != nil {
			return fmt.Errorf("insert redeem code %d: %w", c.id, err)
		}
	}
	// 00009 之后 redeem_codes.id 是 IDENTITY：显式插入固定 id 不会推进序列，
	// 必须手动对齐，否则管理端第一条"自动 id"兑换码会撞上种子数据
	//（unique violation → 创建接口 500）。
	if _, err := tx.Exec(ctx, `
		SELECT setval(
			pg_get_serial_sequence('redeem_codes', 'id'),
			GREATEST((SELECT COALESCE(MAX(id), 0) FROM redeem_codes), 3) + 1,
			false
		)`); err != nil {
		return fmt.Errorf("sync redeem_codes id sequence: %w", err)
	}
	return nil
}
