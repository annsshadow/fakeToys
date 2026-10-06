package service

import (
	"context"
	"encoding/json"
	"fmt"
	"sort"
	"strconv"
	"strings"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

// AdminListLevels 返回关卡列表（含真实通关率，供运营判断难度）。
//
// chapter > 0 时按章节过滤，keyword 非空时按关卡名模糊过滤。
// ⚠️ 第 111 轮：这两个参数此前在 handler 层就被丢弃了——
// UI 发 `?chapter=3&keyword=哨站`，服务端一个都不看，永远回全量 100 关。
// 运营选「第 3 章」看到 100 关，会以为「该章无数据」或干脆不看。
// keyword 进 LIKE 前必须转义 `%` / `_` / `\`——
// 搜索 "100%" 退化成全表匹配是「输入了东西却得到无过滤结果」。
func (s *Service) AdminListLevels(ctx context.Context, chapter int, keyword string) ([]map[string]any, int64, error) {
	query := `
		SELECT l.id, l.chapter, l.name, l.seed, l.base_hp, l.wave_count, l.difficulty,
		       l.energy_cost, l.star_targets, l.terrain_config, l.is_boss, l.enabled,
		       COUNT(br.id) AS attempts,
		       COUNT(br.id) FILTER (WHERE br.result = 'win') AS clears,
		       COALESCE(AVG(br.wave_reached), 0) AS avg_wave
		FROM levels l
		LEFT JOIN battle_records br ON br.level_id = l.id`
	var args []any
	conds := []string{}
	if chapter > 0 {
		args = append(args, chapter)
		conds = append(conds, fmt.Sprintf("l.chapter = $%d", len(args)))
	}
	if kw := strings.TrimSpace(keyword); kw != "" {
		escaped := strings.NewReplacer(`\`, `\\`, `%`, `\%`, `_`, `\_`).Replace(kw)
		args = append(args, "%"+escaped+"%")
		conds = append(conds, fmt.Sprintf("l.name LIKE $%d ESCAPE '\\'::char", len(args)))
	}
	if len(conds) > 0 {
		query += "\n		WHERE " + strings.Join(conds, " AND ")
	}
	query += `
		GROUP BY l.id
		ORDER BY l.id`
	rows, err := s.pool.Query(ctx, query, args...)
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
			"seed":    strconv.FormatInt(seed, 10),
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
		isBoss, enabled                                bool
		attempts, clears                               int64
		avgWave                                        float64
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
		"seed":    strconv.FormatInt(seed, 10),
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
	//
	// ⚠️ 第 119 轮：基础/复合用**生成器种子成员**判定，不再用写死的 `id <= 24`。
	// 24 是魔法数字——当前 SeedSkills 恰好 1..24、SeedCompositeSkills 从 31 起，
	// 但它只反映「今天种子里有多少基础技能」：
	// 将来往 SeedSkills 加一个 id>24 的基础技能（seeder 会把它灌进 skills 表），
	// `id<=24` 会把它**静默归进复合桶**，后台分组从此漂移且无任何报错。
	// 基础技能的事实源是 `domain.SeedSkills`（skills 表就是它的种子），
	// 分类跟着种子走，加新技能自动跟上。
	baseIDs := make(map[int]bool, len(domain.SeedSkills))
	for _, sk := range domain.SeedSkills {
		baseIDs[sk.ID] = true
	}

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
		if baseIDs[id] {
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
//
// # 第 113 轮：**整单拒绝 + price/payload 形状校验**（第 107 轮的同族缺陷）
//
// 修前的两个缺陷：
//
//  1. 部分生效 + 成功回执：每个 case 都是 `if ok { set }`，
//     混合 patch（一个合法键 + 一个非法键）返回 200，
//     合法的写进去、非法的被静默丢弃。运营以为都改过了。
//
//  2. price/payload 只判「json.Marshal 成不成功」：
//     一个标量 `100`、字符串 `"abc"`、数组 `[1,2]` 都能塞进 jsonb。
//     而玩家侧 `Buy` 读它们是 `json.Unmarshal(..., &map[string]int)`——
//     一旦写进标量，该商品**购买链路永久 500**，且没有任何报错在写入时出现。
//
// 所以判据必须落在**形状**上（和「能不能序列化」无关）：
// price/payload 必须是「已知货币 → 非负整数」的对象。
func (s *Service) AdminUpdateShopItem(ctx context.Context, id int64, patch map[string]any) (map[string]any, error) {
	fields := []string{}
	args := []any{id}
	set := func(col string, v any) {
		args = append(args, v)
		fields = append(fields, fmt.Sprintf("%s = $%d", col, len(args)))
	}
	var rejected []string
	reject := func(key, why string) {
		rejected = append(rejected, fmt.Sprintf("%s（%s）", key, why))
	}
	for key, val := range patch {
		switch key {
		case "name":
			v, ok := val.(string)
			if !ok || v == "" {
				reject(key, "必须是非空字符串")
				continue
			}
			set("name", v)
		case "price", "payload":
			m, ok := toIntMap(val)
			if !ok {
				reject(key, "必须是「货币:非负整数」的对象，不能是标量/数组")
				continue
			}
			for k := range m {
				if !validWalletCurrency(k) {
					reject(key, "未知货币 "+k+"（拼错会静默坏掉玩家购买）")
					m = nil
					break
				}
			}
			if m == nil {
				continue
			}
			raw, err := json.Marshal(m)
			if err != nil {
				reject(key, "无法序列化")
				continue
			}
			set(key, raw)
		case "limit_per_day":
			v, ok := toInt64(val)
			if !ok || v < 0 {
				reject(key, "必须是非负整数")
				continue
			}
			set("limit_per_day", v)
		case "sort_order":
			v, ok := toInt64(val)
			if !ok {
				reject(key, "必须是整数")
				continue
			}
			set("sort_order", v)
		case "enabled":
			v, ok := val.(bool)
			if !ok {
				reject(key, "必须是布尔值")
				continue
			}
			set("enabled", v)
		default:
			// 未知键必须报错 —— 后台旧 UI 发 `limit`（正确键是 limit_per_day）
			// 时会被静默吞掉、整单「没有可更新的字段」400，运营不知道键名错了。
			reject(key, "不在可更新字段白名单里")
		}
	}
	if len(rejected) > 0 {
		sort.Strings(rejected)
		return nil, fmt.Errorf("%w: 这些字段没被接受，整单未执行：%s",
			ErrBadInput, strings.Join(rejected, "、"))
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

// toIntMap 把 JSON 值转成 map[string]int。
// 合法输入只有两种形状：map[string]int（Go 直调）或
// map[string]any（JSON 反序列化，值是 float64），且每个值都是非负整数。
func toIntMap(val any) (map[string]int, bool) {
	switch m := val.(type) {
	case map[string]int:
		for _, v := range m {
			if v < 0 {
				return nil, false
			}
		}
		return m, true
	case map[string]any:
		out := make(map[string]int, len(m))
		for k, v := range m {
			n, ok := toInt64(v)
			if !ok || n < 0 {
				return nil, false
			}
			out[k] = int(n)
		}
		return out, true
	}
	return nil, false
}

// validWalletCurrency 报告某货币名是否在「货币全集」内
// （walletColumns 的 4 列 ∪ walletTokens 的 3 个道具）。
func validWalletCurrency(k string) bool {
	if _, ok := walletColumns[k]; ok {
		return true
	}
	return isWalletToken(k)
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
//
// expiresAt 为 nil = 永久有效。
// ⚠️ 第 110 轮：这个参数此前在 handler 层就被丢掉了——
// UI 有「过期时间」输入框、handler 也解析了 `expires_at`，
// 却从不传进来，INSERT 也不写这一列。
// 运营设的过期时间静默丢失，兑换码永久有效（可被无限期转卖滥用）。
// 兑换路径本就认 `expires_at`（economy.go 的 Redeem SQL：
// `expires_at IS NULL OR expires_at > now()`），只差创建端不写库。
func (s *Service) AdminCreateRedeemCode(ctx context.Context, code string, reward map[string]int, maxUses int, expiresAt *time.Time) (map[string]any, error) {
	raw, err := json.Marshal(reward)
	if err != nil {
		return nil, err
	}
	var id int
	if err := s.pool.QueryRow(ctx,
		`INSERT INTO redeem_codes (code, reward, max_uses, expires_at)
		 VALUES ($1,$2,$3,$4) RETURNING id`,
		code, raw, maxUses, expiresAt).Scan(&id); err != nil {
		return nil, fmt.Errorf("create redeem code: %w", err)
	}
	out := map[string]any{"id": id, "code": code, "reward": reward, "max_uses": maxUses}
	if expiresAt != nil {
		out["expires_at"] = *expiresAt
	}
	return out, nil
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
