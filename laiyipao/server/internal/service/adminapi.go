package service

import (
	"context"
	"encoding/json"
	"fmt"
	"strconv"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

// AdminListLevels 返回关卡列表（含真实通关率，供运营判断难度）。
func (s *Service) AdminListLevels(ctx context.Context) ([]map[string]any, int64, error) {
	rows, err := s.pool.Query(ctx, `
		SELECT l.id, l.chapter, l.name, l.seed, l.base_hp, l.wave_count, l.difficulty,
		       l.energy_cost, l.star_targets, l.terrain_config, l.is_boss, l.enabled,
		       COUNT(br.id) AS attempts,
		       COUNT(br.id) FILTER (WHERE br.result = 'win') AS clears,
		       COALESCE(AVG(br.wave_reached), 0) AS avg_wave
		FROM levels l
		LEFT JOIN battle_records br ON br.level_id = l.id
		GROUP BY l.id
		ORDER BY l.id`)
	if err != nil {
		return nil, 0, fmt.Errorf("admin levels: %w", err)
	}
	defer rows.Close()

	var out []map[string]any
	for rows.Next() {
		var (
			id, chapter, waveCount, difficulty, energyCost int
			name                                           string
			seed, baseHP                                   int64
			starRaw, terrainRaw                            []byte
			isBoss, enabled                                bool
			attempts, clears                               int64
			avgWave                                        float64
		)
		if err := rows.Scan(&id, &chapter, &name, &seed, &baseHP, &waveCount, &difficulty,
			&energyCost, &starRaw, &terrainRaw, &isBoss, &enabled,
			&attempts, &clears, &avgWave); err != nil {
			return nil, 0, err
		}
		var stars []int64
		var terrain []domain.TerrainPlacement
		_ = json.Unmarshal(starRaw, &stars)
		_ = json.Unmarshal(terrainRaw, &terrain)

		clearRate := 0.0
		if attempts > 0 {
			clearRate = float64(clears) * 100 / float64(attempts)
		}
		out = append(out, map[string]any{
			"id": id, "chapter": chapter, "name": name,
			// 第 108 轮：seed 以**字符串**下发。
			// seed 是 64 位 LCG 值（如 -7046029255919282421），
			// 超过 2^53 后 JS Number 直接失精，
			// 且后台 TS 契约（AdminLevel.seed）本来就是 string。
			// 玩家侧的同族约定见 routes_e2e_test.go 的 seed_str。
			"seed": strconv.FormatInt(seed, 10),
			"base_hp": baseHP, "wave_count": waveCount, "difficulty": difficulty,
			"energy_cost": energyCost, "star_targets": stars, "terrain_config": terrain,
			"is_boss": isBoss, "enabled": enabled,
			"attempts": attempts, "clears": clears,
			"clear_rate": clearRate, "avg_wave": avgWave,
		})
	}
	return out, int64(len(out)), rows.Err()
}

// AdminLevelRow 读回**单行**关卡配置（来自 `levels` 表）。
//
// # 它与 `domain.GenerateLevel` 的区别是本轮的核心（第 107 轮）
//
//	AdminLevelRow        → 读库，返回运营改过的值
//	domain.GenerateLevel → **纯函数**，只从 ChapterOf + LCG 算，从不查库
//
// 所以：运营在后台改关卡 → 库里变了、后台列表显示变了、
// 但 `GET /levels/:id` 与 `LoadGameConfig` 走的仍是生成器，玩家拿不到改动。
//
// 这是**设计决策**，不是我能单方面定的（见 README 已知边界）。
// 但 `AdminUpdateLevel` 的**响应**必须与自己的写入一致 —— 那没有决策空间。
//
// # 第 108 轮：形状与列表行**完全一致**
//
// 写后读回的行若缺 `attempts/clears/clear_rate/avg_wave`，
// 后台 `Object.assign(row, res.level)` 后列表统计列就停在上次刷新的值，
// 「改完刷新前后不一致」又回到第 107 轮修掉的同一种缺陷。
// 所以这里直接复用列表的 JOIN 查询（按 id 过滤），
// 保证「回读响应 = 刷新列表会看到的行」。
func (s *Service) AdminLevelRow(ctx context.Context, levelID int) (map[string]any, error) {
	var (
		id, chapter, waveCount, difficulty, energyCost int
		name                                           string
		seed, baseHP                                   int64
		starRaw, terrainRaw                            []byte
		isBoss, enabled                                 bool
		attempts, clears                                int64
		avgWave                                         float64
	)
	err := s.pool.QueryRow(ctx, `
		SELECT l.id, l.chapter, l.name, l.seed, l.base_hp, l.wave_count, l.difficulty,
		       l.energy_cost, l.star_targets, l.terrain_config, l.is_boss, l.enabled,
		       COUNT(br.id) AS attempts,
		       COUNT(br.id) FILTER (WHERE br.result = 'win') AS clears,
		       COALESCE(AVG(br.wave_reached), 0) AS avg_wave
		FROM levels l
		LEFT JOIN battle_records br ON br.level_id = l.id
		WHERE l.id = $1
		GROUP BY l.id`, levelID).
		Scan(&id, &chapter, &name, &seed, &baseHP, &waveCount, &difficulty,
			&energyCost, &starRaw, &terrainRaw, &isBoss, &enabled,
			&attempts, &clears, &avgWave)
	if err != nil {
		return nil, fmt.Errorf("level row %d: %w", levelID, err)
	}
	var stars []int64
	var terrain []domain.TerrainPlacement
	_ = json.Unmarshal(starRaw, &stars)
	_ = json.Unmarshal(terrainRaw, &terrain)

	clearRate := 0.0
	if attempts > 0 {
		clearRate = float64(clears) * 100 / float64(attempts)
	}
	return map[string]any{
		"id": id, "chapter": chapter, "name": name,
		"seed": strconv.FormatInt(seed, 10),
		"base_hp": baseHP, "wave_count": waveCount, "difficulty": difficulty,
		"energy_cost": energyCost, "star_targets": stars, "terrain_config": terrain,
		"is_boss": isBoss, "enabled": enabled,
		"attempts": attempts, "clears": clears,
		"clear_rate": clearRate, "avg_wave": avgWave,
	}, nil
}

// AdminLevelWaves 返回某关的波次配置。
func (s *Service) AdminLevelWaves(ctx context.Context, levelID int) ([]map[string]any, error) {
	rows, err := s.pool.Query(ctx,
		`SELECT wave_index, spawns FROM level_waves WHERE level_id = $1 ORDER BY wave_index`, levelID)
	if err != nil {
		return nil, fmt.Errorf("admin waves: %w", err)
	}
	defer rows.Close()
	var out []map[string]any
	for rows.Next() {
		var idx int
		var raw []byte
		if err := rows.Scan(&idx, &raw); err != nil {
			return nil, err
		}
		var spawns []domain.Spawn
		_ = json.Unmarshal(raw, &spawns)
		out = append(out, map[string]any{"wave_index": idx, "spawns": spawns})
	}
	return out, rows.Err()
}

// AdminSkills 返回技能与配方。
func (s *Service) AdminSkills(ctx context.Context) (map[string]any, error) {
	base, composite, err := s.fetchSkills(ctx)
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"items":   append(base, composite...),
		"recipes": domain.SeedRecipes,
	}, nil
}

func (s *Service) fetchSkills(ctx context.Context) (base, composite []map[string]any, err error) {
	rows, qerr := s.pool.Query(ctx, `
		SELECT id, code, name, family, element, kind, descr, base_damage, heat_cost,
		       cooldown_ms, pierce, aoe_radius, apply_element, apply_stacks,
		       projectile_speed, chain, unlock_level
		FROM skills ORDER BY id`)
	if qerr != nil {
		return nil, nil, fmt.Errorf("fetch skills: %w", qerr)
	}
	defer rows.Close()
	for rows.Next() {
		var (
			id, heatCost, cooldown, pierce, aoe, stacks, speed, chain, unlock int
			code, name, family, element, kind, descr, applyElement            string
			baseDamage                                                        int64
		)
		if err := rows.Scan(&id, &code, &name, &family, &element, &kind, &descr,
			&baseDamage, &heatCost, &cooldown, &pierce, &aoe, &applyElement, &stacks,
			&speed, &chain, &unlock); err != nil {
			return nil, nil, err
		}
		item := map[string]any{
			"id": id, "code": code, "name": name, "family": family, "element": element,
			"kind": kind, "descr": descr, "base_damage": baseDamage,
			"heat_cost": heatCost, "cooldown_ms": cooldown, "pierce": pierce,
			"aoe_radius": aoe, "apply_element": applyElement, "apply_stacks": stacks,
			"projectile_speed": speed, "chain": chain, "unlock_level": unlock,
		}
		if id <= 24 {
			base = append(base, item)
		} else {
			composite = append(composite, item)
		}
	}
	return base, composite, rows.Err()
}

// AdminEquipment 返回装备 / 宝石 / 皮肤 / 敌人抗性。
func (s *Service) AdminEquipment(ctx context.Context) (map[string]any, error) {
	rows, err := s.pool.Query(ctx, `
		SELECT id, code, name, slot, tier, element, descr, base_armor, base_bonus_pct, unlock_level
		FROM equipment ORDER BY id`)
	if err != nil {
		return nil, fmt.Errorf("admin equipment: %w", err)
	}
	defer rows.Close()
	var equipment []map[string]any
	for rows.Next() {
		var (
			id, tier, armor, bonus, unlock   int
			code, name, slot, element, descr string
		)
		if err := rows.Scan(&id, &code, &name, &slot, &tier, &element, &descr,
			&armor, &bonus, &unlock); err != nil {
			return nil, err
		}
		equipment = append(equipment, map[string]any{
			"id": id, "code": code, "name": name, "slot": slot, "tier": tier,
			"element": element, "descr": descr, "base_armor": armor,
			"base_bonus_pct": bonus, "unlock_level": unlock,
		})
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}

	return map[string]any{
		"equipment":      equipment,
		"gems":           domain.SeedGems,
		"qualities":      domain.GemQualities,
		"skins":          domain.SeedSkins,
		"enemies":        domain.ScaleAllEnemies(),
		"reactions":      domain.AllReactionSpecs(),
		"mastery":        domain.AllMasteryFamilies(),
		"rating_weights": domain.DefaultRatingWeights(),
	}, nil
}

// AdminListDefenses 返回全部防线。
func (s *Service) AdminListDefenses(ctx context.Context, limit int) ([]map[string]any, int64, error) {
	if limit <= 0 || limit > 200 {
		limit = 50
	}
	var total int64
	if err := s.pool.QueryRow(ctx, `SELECT COUNT(*) FROM defenses`).Scan(&total); err != nil {
		return nil, 0, err
	}
	rows, err := s.pool.Query(ctx, `
		SELECT d.id, d.owner_id, u.nickname, d.name, d.power, d.element_coverage,
		       d.mastery_done, d.wins, d.losses, d.shielded_until, d.updated_at
		FROM defenses d JOIN users u ON u.id = d.owner_id
		ORDER BY d.power DESC LIMIT $1`, limit)
	if err != nil {
		return nil, 0, fmt.Errorf("admin defenses: %w", err)
	}
	defer rows.Close()
	var out []map[string]any
	for rows.Next() {
		var (
			id, ownerID, power, coverage, masteryDone, wins, losses int64
			ownerName, name                                         string
			shielded                                                *time.Time
			updated                                                 time.Time
		)
		if err := rows.Scan(&id, &ownerID, &ownerName, &name, &power, &coverage,
			&masteryDone, &wins, &losses, &shielded, &updated); err != nil {
			return nil, 0, err
		}
		out = append(out, map[string]any{
			"id": id, "owner_id": ownerID, "owner_name": ownerName, "name": name,
			"power": power, "element_coverage": coverage, "mastery_done": masteryDone,
			"wins": wins, "losses": losses, "shielded_until": shielded,
			"updated_at": updated,
		})
	}
	return out, total, rows.Err()
}

// AdminEconomy 返回经济流水与商城配置。
func (s *Service) AdminEconomy(ctx context.Context) (map[string]any, error) {
	rows, err := s.pool.Query(ctx, `
		SELECT reason, currency, COUNT(*), COALESCE(SUM(delta), 0)
		FROM wallet_flows GROUP BY reason, currency ORDER BY COUNT(*) DESC LIMIT 50`)
	if err != nil {
		return nil, fmt.Errorf("admin economy flows: %w", err)
	}
	defer rows.Close()
	flows := []map[string]any{}
	for rows.Next() {
		var reason, currency string
		var count, delta int64
		if err := rows.Scan(&reason, &currency, &count, &delta); err != nil {
			return nil, err
		}
		flows = append(flows, map[string]any{
			"reason": reason, "currency": currency, "count": count, "delta": delta,
		})
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}

	srows, err := s.pool.Query(ctx,
		`SELECT id, code, name, category, price, payload, limit_per_day, sort_order, enabled
		 FROM shop_items ORDER BY sort_order`)
	if err != nil {
		return nil, fmt.Errorf("admin shop: %w", err)
	}
	defer srows.Close()
	shop := []map[string]any{}
	for srows.Next() {
		var id, limit, sort int
		var code, name, category string
		var priceRaw, payloadRaw []byte
		var enabled bool
		if err := srows.Scan(&id, &code, &name, &category, &priceRaw, &payloadRaw,
			&limit, &sort, &enabled); err != nil {
			return nil, err
		}
		var price, payload map[string]int
		_ = json.Unmarshal(priceRaw, &price)
		_ = json.Unmarshal(payloadRaw, &payload)
		shop = append(shop, map[string]any{
			"id": id, "code": code, "name": name, "category": category,
			"price": price, "payload": payload, "limit_per_day": limit,
			"sort_order": sort, "enabled": enabled,
		})
	}
	return map[string]any{"flows": flows, "shop": shop}, srows.Err()
}

// AdminUpdateShopItem 更新商城配置。
func (s *Service) AdminUpdateShopItem(ctx context.Context, id int64, patch map[string]any) (map[string]any, error) {
	fields := []string{}
	args := []any{id}
	set := func(col string, v any) {
		args = append(args, v)
		fields = append(fields, fmt.Sprintf("%s = $%d", col, len(args)))
	}
	for key, val := range patch {
		switch key {
		case "name":
			if v, ok := val.(string); ok && v != "" {
				set("name", v)
			}
		case "price":
			if raw, err := json.Marshal(val); err == nil {
				set("price", raw)
			}
		case "payload":
			if raw, err := json.Marshal(val); err == nil {
				set("payload", raw)
			}
		case "limit_per_day":
			if v, ok := toInt64(val); ok && v >= 0 {
				set("limit_per_day", v)
			}
		case "sort_order":
			if v, ok := toInt64(val); ok {
				set("sort_order", v)
			}
		case "enabled":
			if v, ok := val.(bool); ok {
				set("enabled", v)
			}
		}
	}
	if len(fields) == 0 {
		return nil, fmt.Errorf("%w: 没有可更新的字段", ErrBadInput)
	}
	sql := fmt.Sprintf(`UPDATE shop_items SET %s, updated_at = now() WHERE id = $1`, joinComma(fields))
	if _, err := s.pool.Exec(ctx, sql, args...); err != nil {
		return nil, fmt.Errorf("update shop item: %w", err)
	}
	return map[string]any{"id": id, "updated": fields}, nil
}

// AdminListAnnouncements 返回公告。
func (s *Service) AdminListAnnouncements(ctx context.Context) ([]map[string]any, error) {
	rows, err := s.pool.Query(ctx,
		`SELECT id, title, body, published, published_at, created_at FROM announcements ORDER BY id DESC`)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var out []map[string]any
	for rows.Next() {
		var id int
		var title, body string
		var published bool
		var publishedAt, createdAt *time.Time
		if err := rows.Scan(&id, &title, &body, &published, &publishedAt, &createdAt); err != nil {
			return nil, err
		}
		out = append(out, map[string]any{
			"id": id, "title": title, "body": body, "published": published,
			"published_at": publishedAt, "created_at": createdAt,
		})
	}
	return out, rows.Err()
}

// AdminCreateAnnouncement 创建公告。
func (s *Service) AdminCreateAnnouncement(ctx context.Context, title, body string, published bool) (map[string]any, error) {
	var id int
	var at *time.Time
	if published {
		now := time.Now()
		at = &now
	}
	if err := s.pool.QueryRow(ctx,
		`INSERT INTO announcements (title, body, published, published_at) VALUES ($1,$2,$3,$4) RETURNING id`,
		title, body, published, at).Scan(&id); err != nil {
		return nil, fmt.Errorf("create announcement: %w", err)
	}
	return map[string]any{"id": id, "title": title, "body": body, "published": published}, nil
}

// AdminListRedeemCodes 返回兑换码。
func (s *Service) AdminListRedeemCodes(ctx context.Context) ([]map[string]any, error) {
	rows, err := s.pool.Query(ctx,
		`SELECT id, code, reward, max_uses, used_count, expires_at, enabled FROM redeem_codes ORDER BY id`)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var out []map[string]any
	for rows.Next() {
		var id, maxUses, used int
		var code string
		var rewardRaw []byte
		var expires *time.Time
		var enabled bool
		if err := rows.Scan(&id, &code, &rewardRaw, &maxUses, &used, &expires, &enabled); err != nil {
			return nil, err
		}
		var reward map[string]int
		_ = json.Unmarshal(rewardRaw, &reward)
		out = append(out, map[string]any{
			"id": id, "code": code, "reward": reward, "max_uses": maxUses,
			"used_count": used, "expires_at": expires, "enabled": enabled,
		})
	}
	return out, rows.Err()
}

// AdminCreateRedeemCode 创建兑换码。
func (s *Service) AdminCreateRedeemCode(ctx context.Context, code string, reward map[string]int, maxUses int) (map[string]any, error) {
	raw, err := json.Marshal(reward)
	if err != nil {
		return nil, err
	}
	var id int
	if err := s.pool.QueryRow(ctx,
		`INSERT INTO redeem_codes (code, reward, max_uses) VALUES ($1,$2,$3) RETURNING id`,
		code, raw, maxUses).Scan(&id); err != nil {
		return nil, fmt.Errorf("create redeem code: %w", err)
	}
	return map[string]any{"id": id, "code": code, "reward": reward, "max_uses": maxUses}, nil
}

// AdminListAuditLogs 返回操作审计。
func (s *Service) AdminListAuditLogs(ctx context.Context, limit int) ([]map[string]any, error) {
	if limit <= 0 || limit > 500 {
		limit = 100
	}
	rows, err := s.pool.Query(ctx,
		`SELECT id, admin_id, username, action, target, detail, created_at
		 FROM admin_audit_logs ORDER BY id DESC LIMIT $1`, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var out []map[string]any
	for rows.Next() {
		var id, adminID *int64
		var username, action, target string
		var detail []byte
		var createdAt time.Time
		if err := rows.Scan(&id, &adminID, &username, &action, &target, &detail, &createdAt); err != nil {
			return nil, err
		}
		var d any
		_ = json.Unmarshal(detail, &d)
		out = append(out, map[string]any{
			"id": id, "admin_id": adminID, "username": username, "action": action,
			"target": target, "detail": d, "created_at": createdAt,
		})
	}
	return out, rows.Err()
}
