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

// MaxAdminKeywordLen 是后台搜索词（`?keyword=`）的长度上限，**按字符数**。
//
// # 为什么需要它
//
// keyword 进的是 `%…%` 的 LIKE 模式，此前完全不限长。请求行本身受
// fasthttp 的 header 上限约束（默认 4KB），所以这不是远程 DoS ——
// 但实际后果有两条，都不需要恶意：
//
//  1. **运营从工单/聊天记录里粘贴一整段话**进来（这是最常见的触发），
//     服务端老老实实拿整段去全表 LIKE，结果必然是空集。
//     运营据此得出的结论是「查无此人」，而真正发生的是「整段被当成
//     一个整体去匹配」。这正是本项目反复修的「静默降级而不是失败」。
//  2. 4KB 的 LIKE 模式足以让全表扫描慢一个量级，而它耗的是后台自己
//     的连接池（逐行回退匹配，无法走索引）。
//
// 取 64 与 `MaxChallengeReplayHashLen` 同量级：玩家昵称是几个到十几个
// 字符，64 已经比任何真实昵称/关卡名都长 —— 它拒绝的只是「传一段文章」。
const MaxAdminKeywordLen = 64

// trimAdminKeyword 归一化后台搜索词：去首尾空白 + 长度上限。
//
// ⚠️ 两个调用点（`AdminListLevels` / `AdminListUsers`）必须走同一个函数。
//
// LIKE 转义目前就是**两份拷贝**：`adminapi.go` 与 `stats.go` 各写了一遍
// 同一个 Replacer（见第 118 轮的注释）。同一个语义写两遍就会分叉 ——
// 上一轮补转义时，两处是靠人记得才一起改的，不是靠结构保证的。
// 上界比转义更容易漏：漏了转义至少还在功能上看得见，漏了上界
// 什么都不会发生 —— 直到有人粘贴一整段话进来。
func trimAdminKeyword(keyword string) (string, error) {
	kw := strings.TrimSpace(keyword)
	if n := len([]rune(kw)); n > MaxAdminKeywordLen {
		return "", fmt.Errorf("%w: 查询参数 keyword 长 %d 字，上限 %d（昵称与关卡名都远短于此）",
			ErrBadInput, n, MaxAdminKeywordLen)
	}
	return kw, nil
}

// AdminListLevels 返回关卡列表（含真实通关率，供运营判断难度）。
//
// chapter > 0 时按章节过滤，keyword 非空时按关卡名模糊过滤。
// ⚠️ 第 111 轮：这两个参数此前在 handler 层就被丢弃了——
// UI 发 `?chapter=3&keyword=哨站`，服务端一个都不看，永远回全量 100 关。
// 运营选「第 3 章」看到 100 关，会以为「该章无数据」或干脆不看。
// keyword 进 LIKE 前必须转义 `%` / `_` / `\`——
// 搜索 "100%" 退化成全表匹配是「输入了东西却得到无过滤结果」。
// AdminListLevels 列出关卡（运营后台列表）。
//
// ⚠️ 2026-10-21（P3-4）：现在带 `limit`/`offset` 分页，且 `total` 是
// **真实总数**（COUNT(*)），不再是 `len(out)`。
//
// 此前 `total` 就是 `len(out)` —— 在没有分页时它恰好等于真实总数，
// 看起来是对的；但一旦加了 limit，它就变成了「这次返回了几行」，
// 于是前端分页器会显示「共 50 条」而实际有 100 关。
func (s *Service) AdminListLevels(ctx context.Context, chapter int, keyword string, limit, offset int) ([]AdminLevelView, int64, error) {
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
	// keyword 进 LIKE 前必须转义 `%` / `_` / `\`——
	// 搜索 "100%" 退化成全表匹配是「输入了东西却得到无过滤结果」。
	//
	// 第 154 轮补上长度上限：上界与转义是**两件事**，转义保证
	// 「输入被当成字面量」，上界保证「输入不会长到没有意义」。
	// 详见 trimAdminKeyword 的说明。
	kw, err := trimAdminKeyword(keyword)
	if err != nil {
		return nil, 0, err
	}
	if kw != "" {
		escaped := strings.NewReplacer(`\`, `\\`, `%`, `\%`, `_`, `\_`).Replace(kw)
		args = append(args, "%"+escaped+"%")
		conds = append(conds, fmt.Sprintf("l.name LIKE $%d ESCAPE '\\'::char", len(args)))
	}
	whereClause := ""
	if len(conds) > 0 {
		whereClause = " WHERE " + strings.Join(conds, " AND ")
		query += "\n		" + whereClause
	}
	query += `
		GROUP BY l.id
		ORDER BY l.id`
	// 真实总数：与列表用**同一组过滤条件**（否则分页器的「共 N 条」和
	// 实际能翻到的条数对不上 —— 那是最容易被当成「数据丢了」的形状）。
	var total int64
	if err := s.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM levels l`+whereClause, args...).Scan(&total); err != nil {
		return nil, 0, fmt.Errorf("admin levels count: %w", err)
	}
	if limit > 0 {
		args = append(args, limit)
		query += fmt.Sprintf("\n LIMIT $%d", len(args))
		if offset > 0 {
			args = append(args, offset)
			query += fmt.Sprintf(" OFFSET $%d", len(args))
		}
	}
	rows, err := s.pool.Query(ctx, query, args...)
	if err != nil {
		return nil, 0, fmt.Errorf("admin levels: %w", err)
	}
	defer rows.Close()

	var out []AdminLevelView
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
		// 第 146 轮：fail-loud（AGENTS #11）。这两列是生成器写入的 jsonb，
		// 正常必为合法数组。`_ =` 会把「列被写坏 / jsonb 变 null」静默吞掉，
		// 后台关卡列表就把该关的星级门槛与地形显示成空 —— 运营误以为「这关没门槛/没地形」。
		// 与第 125 轮（审计 `{}` 谎言）/ 第 144 轮（漏斗查错句柄）同族：出错必须响。
		if err := json.Unmarshal(starRaw, &stars); err != nil {
			return nil, 0, fmt.Errorf("admin levels: 关卡 %d 的 star_targets 非法 jsonb: %w", id, err)
		}
		if err := json.Unmarshal(terrainRaw, &terrain); err != nil {
			return nil, 0, fmt.Errorf("admin levels: 关卡 %d 的 terrain_config 非法 jsonb: %w", id, err)
		}

		clearRate := 0.0
		if attempts > 0 {
			clearRate = float64(clears) * 100 / float64(attempts)
		}
		out = append(out, AdminLevelView{
			ID: id, Chapter: chapter, Name: name,
			// seed is a STRING on purpose: it is a 64-bit LCG value
			// (e.g. -7046029255919282421) and past 2^53 a JS Number loses
			// precision. The admin TS contract has always typed it string.
			// See routes_e2e_test.go seed_str for the player-side twin.
			Seed:          strconv.FormatInt(seed, 10),
			BaseHP:        baseHP,
			WaveCount:     waveCount,
			Difficulty:    difficulty,
			EnergyCost:    energyCost,
			StarTargets:   stars,
			TerrainConfig: terrain,
			IsBoss:        isBoss,
			Enabled:       enabled,
			Attempts:      attempts,
			Clears:        clears,
			ClearRate:     clearRate,
			AvgWave:       avgWave,
			// 23rd product decision (route b: the generator stays
			// authoritative). List rows carry these too, otherwise ops sees
			// the edited HP in the table with no way to know it does not
			// reach players.
			PlayerAuthority:  playerAuthorityGenerator,
			AppliedToPlayers: false,
		})
	}
	return out, total, rows.Err()
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
func (s *Service) AdminLevelRow(ctx context.Context, levelID int) (AdminLevelView, error) {
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
		return AdminLevelView{}, fmt.Errorf("level row %d: %w", levelID, err)
	}
	var stars []int64
	var terrain []domain.TerrainPlacement
	// 第 146 轮：fail-loud，见 AdminListLevels 同族说明。单关详情更不该静默吞坏数据。
	if err := json.Unmarshal(starRaw, &stars); err != nil {
		return AdminLevelView{}, fmt.Errorf("level row %d: star_targets 非法 jsonb: %w", levelID, err)
	}
	if err := json.Unmarshal(terrainRaw, &terrain); err != nil {
		return AdminLevelView{}, fmt.Errorf("level row %d: terrain_config 非法 jsonb: %w", levelID, err)
	}

	clearRate := 0.0
	if attempts > 0 {
		clearRate = float64(clears) * 100 / float64(attempts)
	}
	return AdminLevelView{
		ID: id, Chapter: chapter, Name: name,
		Seed:          strconv.FormatInt(seed, 10),
		BaseHP:        baseHP,
		WaveCount:     waveCount,
		Difficulty:    difficulty,
		EnergyCost:    energyCost,
		StarTargets:   stars,
		TerrainConfig: terrain,
		IsBoss:        isBoss,
		Enabled:       enabled,
		Attempts:      attempts,
		Clears:        clears,
		ClearRate:     clearRate,
		AvgWave:       avgWave,
		// 23rd product decision (route b: the generator stays authoritative).
		// These two fields make that machine-readable so the admin UI can say
		// "recorded, does not affect live players" instead of rendering the
		// write receipt as if it had taken effect.
		PlayerAuthority:  playerAuthorityGenerator,
		AppliedToPlayers: false,
	}, nil
}

// AdminLevelWaves 返回某关的波次配置。
func (s *Service) AdminLevelWaves(ctx context.Context, levelID int) ([]AdminLevelWaveView, error) {
	rows, err := s.pool.Query(ctx,
		`SELECT wave_index, spawns FROM level_waves WHERE level_id = $1 ORDER BY wave_index`, levelID)
	if err != nil {
		return nil, fmt.Errorf("admin waves: %w", err)
	}
	defer rows.Close()
	var out []AdminLevelWaveView
	for rows.Next() {
		var idx int
		var raw []byte
		if err := rows.Scan(&idx, &raw); err != nil {
			return nil, err
		}
		var spawns []domain.Spawn
		// 第 149 轮：fail-loud。level_waves 的波次坏掉时静默给空 spawns，
		// 运营会以为「这关没有刷怪」。
		if err := json.Unmarshal(raw, &spawns); err != nil {
			return nil, fmt.Errorf("admin level waves: 第 %d 波 spawns 非法 jsonb: %w", idx, err)
		}
		out = append(out, AdminLevelWaveView{WaveIndex: idx, Spawns: spawns})
	}
	return out, rows.Err()
}

// AdminSkills 返回技能与配方。
func (s *Service) AdminSkills(ctx context.Context) (AdminSkillsResp, error) {
	base, composite, err := s.fetchSkills(ctx)
	if err != nil {
		return AdminSkillsResp{}, err
	}
	return AdminSkillsResp{
		Items:   append(base, composite...),
		Recipes: domain.SeedRecipes,
	}, nil
}

func (s *Service) fetchSkills(ctx context.Context) (base, composite []AdminSkillView, err error) {
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
		item := AdminSkillView{
			ID: id, Code: code, Name: name, Family: family, Element: element,
			Kind: kind, Descr: descr, BaseDamage: baseDamage,
			HeatCost: heatCost, CooldownMs: cooldown, Pierce: pierce,
			AoeRadius: aoe, ApplyElement: applyElement, ApplyStacks: stacks,
			ProjectileSpeed: speed, Chain: chain, UnlockLevel: unlock,
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
func (s *Service) AdminEquipment(ctx context.Context) (AdminEquipmentResp, error) {
	rows, err := s.pool.Query(ctx, `
		SELECT id, code, name, slot, tier, element, descr, base_armor, base_bonus_pct, unlock_level
		FROM equipment ORDER BY id`)
	if err != nil {
		return AdminEquipmentResp{}, fmt.Errorf("admin equipment: %w", err)
	}
	defer rows.Close()
	var equipment []AdminEquipmentView
	for rows.Next() {
		var (
			id, tier, armor, bonus, unlock   int
			code, name, slot, element, descr string
		)
		if err := rows.Scan(&id, &code, &name, &slot, &tier, &element, &descr,
			&armor, &bonus, &unlock); err != nil {
			return AdminEquipmentResp{}, err
		}
		equipment = append(equipment, AdminEquipmentView{
			ID: id, Code: code, Name: name, Slot: slot, Tier: tier,
			Element: element, Descr: descr, BaseArmor: armor,
			BaseBonusPct: bonus, UnlockLevel: unlock,
		})
	}
	if err := rows.Err(); err != nil {
		return AdminEquipmentResp{}, err
	}

	return AdminEquipmentResp{
		Equipment:      equipment,
		Gems:           domain.SeedGems,
		Qualifications: qualitiesFromDomain(),
		Skins:          domain.SeedSkins,
		Enemies:        domain.ScaleAllEnemies(),
		Reactions:      domain.AllReactionSpecs(),
		Mastery:        domain.AllMasteryFamilies(),
		RatingWeights:  domain.DefaultRatingWeights(),
	}, nil
}

// AdminListDefenses 返回全部防线。
func (s *Service) AdminListDefenses(ctx context.Context, limit int, offset int) ([]AdminDefenseView, int64, error) {
	if limit <= 0 || limit > 200 {
		limit = 50
	}
	var total int64
	if err := s.pool.QueryRow(ctx, `SELECT COUNT(*) FROM defenses`).Scan(&total); err != nil {
		return nil, 0, err
	}
	// ⚠️ 2026-10-21（二轮审计 P2-6）：补 offset ——
	// 此前只能看前 100 条，而 `total` 是全量 COUNT，于是
	// 「共 N 条防线」却只看得到 100 条（而且漏掉的是**弱**防线，
	// 因为按战力倒序）。
	if offset < 0 {
		offset = 0
	}
	rows, err := s.pool.Query(ctx, `
		SELECT d.id, d.owner_id, u.nickname, d.name, d.power, d.element_coverage,
		       d.mastery_done, d.wins, d.losses, d.shielded_until, d.updated_at,
		       d.snapshot_hash
		FROM defenses d JOIN users u ON u.id = d.owner_id
		ORDER BY d.power DESC LIMIT $1 OFFSET $2`, limit, offset)
	if err != nil {
		return nil, 0, fmt.Errorf("admin defenses: %w", err)
	}
	defer rows.Close()
	var out []AdminDefenseView
	for rows.Next() {
		var (
			id, ownerID, power, coverage, masteryDone, wins, losses int64
			ownerName, name                                         string
			snapshotHash                                            string
			shielded                                                *time.Time
			updated                                                 time.Time
		)
		if err := rows.Scan(&id, &ownerID, &ownerName, &name, &power, &coverage,
			&masteryDone, &wins, &losses, &shielded, &updated, &snapshotHash); err != nil {
			return nil, 0, err
		}
		out = append(out, AdminDefenseView{
			ID: id, OwnerID: ownerID, OwnerName: ownerName, Name: name,
			Power: power, ElementCoverage: int(coverage), MasteryDone: int(masteryDone),
			Wins: int(wins), Losses: int(losses), ShieldedUntil: shielded,
			UpdatedAt: updated,
			// 2026-10-21: the admin "defense watch" page ranks snapshot-hash
			// stability as its TOP concern, but the server never returned this
			// column -> the column was permanently blank and the concern was
			// invisible to the very page built to surface it.
			SnapshotHash: snapshotHash,
		})
	}
	return out, total, rows.Err()
}

// AdminEconomy 返回经济流水与商城配置。
func (s *Service) AdminEconomy(ctx context.Context) (AdminEconomyResp, error) {
	rows, err := s.pool.Query(ctx, `
		SELECT reason, currency, COUNT(*), COALESCE(SUM(delta), 0)
		FROM wallet_flows GROUP BY reason, currency ORDER BY COUNT(*) DESC LIMIT 50`)
	if err != nil {
		return AdminEconomyResp{}, fmt.Errorf("admin economy flows: %w", err)
	}
	defer rows.Close()
	flows := []AdminEconomyFlowView{}
	for rows.Next() {
		var reason, currency string
		var count, delta int64
		if err := rows.Scan(&reason, &currency, &count, &delta); err != nil {
			return AdminEconomyResp{}, err
		}
		flows = append(flows, AdminEconomyFlowView{
			Reason: reason, Currency: currency, Count: count, Delta: delta,
		})
	}
	if err := rows.Err(); err != nil {
		return AdminEconomyResp{}, err
	}

	srows, err := s.pool.Query(ctx,
		`SELECT id, code, name, category, price, payload, limit_per_day, sort_order, enabled
		 FROM shop_items ORDER BY sort_order`)
	if err != nil {
		return AdminEconomyResp{}, fmt.Errorf("admin shop: %w", err)
	}
	defer srows.Close()
	shop := []AdminShopItemView{}
	for srows.Next() {
		var id, limit, sort int
		var code, name, category string
		var priceRaw, payloadRaw []byte
		var enabled bool
		if err := srows.Scan(&id, &code, &name, &category, &priceRaw, &payloadRaw,
			&limit, &sort, &enabled); err != nil {
			return AdminEconomyResp{}, err
		}
		var price, payload map[string]int
		// fail-loud, same rule as LoadShop.
		if err := json.Unmarshal(priceRaw, &price); err != nil {
			return AdminEconomyResp{}, fmt.Errorf("admin economy: shop price is invalid jsonb: %w", err)
		}
		if err := json.Unmarshal(payloadRaw, &payload); err != nil {
			return AdminEconomyResp{}, fmt.Errorf("admin economy: shop payload is invalid jsonb: %w", err)
		}
		shop = append(shop, AdminShopItemView{
			ID: id, Code: code, Name: name, Category: category,
			Price: price, Payload: payload, LimitPerDay: limit,
			SortOrder: sort, Enabled: enabled,
		})
	}
	return AdminEconomyResp{Flows: flows, Shop: shop}, srows.Err()
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
func (s *Service) AdminUpdateShopItem(ctx context.Context, id int64, patch map[string]any) (AdminShopUpdateResp, error) {
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
		return AdminShopUpdateResp{}, fmt.Errorf("%w: these fields were rejected, nothing was written: %s",
			ErrBadInput, strings.Join(rejected, ", "))
	}
	if len(fields) == 0 {
		return AdminShopUpdateResp{}, fmt.Errorf("%w: no updatable fields", ErrBadInput)
	}
	sql := fmt.Sprintf(`UPDATE shop_items SET %s, updated_at = now() WHERE id = $1`, joinComma(fields))
	if _, err := s.pool.Exec(ctx, sql, args...); err != nil {
		return AdminShopUpdateResp{}, fmt.Errorf("update shop item: %w", err)
	}
	return AdminShopUpdateResp{ID: id, Updated: fields}, nil
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
	if isWalletToken(k) {
		return true
	}
	// ⚠️ 2026-10-21（二轮审计 P2）：`equipment_<id>` 也是合法的 payload 键。
	//
	// 此前这里不认它 → `PUT /admin/shop/:id {"payload":{"equipment_5":1}}` 被 400 拒，
	// 于是**后台改不了、删不掉**种子造出来的装备商品（读写不对称：
	// 种子能造，后台不能改）。
	//
	// 判定引用生产代码的谓词（`isEquipmentPayloadKey`），别把前缀抄一遍。
	return isEquipmentPayloadKey(k)
}

// AdminListAnnouncements 返回公告。
func (s *Service) AdminListAnnouncements(ctx context.Context, limit, offset int) ([]AdminAnnouncementView, int64, error) {
	// ⚠️ 2026-10-21（二轮审计 P2）：负 offset 必须夹到 0 ——
	// 否则 PostgreSQL 报 `OFFSET must not be negative` → 整页 500。
	if offset < 0 {
		offset = 0
	}
	// ⚠️ 2026-10-21（二轮审计 P2-6）：`total` 必须是**全量行数**，
	// 不是当前页的条数 —— 前端要拿它算页数。
	var total int64
	if err := s.pool.QueryRow(ctx, `SELECT COUNT(*) FROM announcements`).Scan(&total); err != nil {
		return nil, 0, fmt.Errorf("admin announcements count: %w", err)
	}
	// ⚠️ 2026-10-21：limit 必须夹住 ——
	// `limit=0` 在 SQL 里是「**返回零行**」（不是「不限」），
	// `limit=999999` 则是全表扫描。
	if limit <= 0 || limit > 500 {
		limit = 50
	}
	rows, err := s.pool.Query(ctx,
		`SELECT id, title, body, published, published_at, created_at FROM announcements ORDER BY id DESC LIMIT $1 OFFSET $2`, limit, offset)
	if err != nil {
		return nil, 0, err
	}
	defer rows.Close()
	var out []AdminAnnouncementView
	for rows.Next() {
		var id int
		var title, body string
		var published bool
		var publishedAt, createdAt *time.Time
		if err := rows.Scan(&id, &title, &body, &published, &publishedAt, &createdAt); err != nil {
			return nil, 0, err
		}
		out = append(out, AdminAnnouncementView{
			ID: id, Title: title, Body: body, Published: published,
			PublishedAt: publishedAt, CreatedAt: createdAt,
		})
	}
	return out, total, rows.Err()
}

// AdminCreateAnnouncement 创建公告。
func (s *Service) AdminCreateAnnouncement(ctx context.Context, title, body string, published bool) (AdminAnnouncementView, error) {
	var id int
	var at *time.Time
	if published {
		now := time.Now()
		at = &now
	}
	if err := s.pool.QueryRow(ctx,
		`INSERT INTO announcements (title, body, published, published_at) VALUES ($1,$2,$3,$4) RETURNING id`,
		title, body, published, at).Scan(&id); err != nil {
		return AdminAnnouncementView{}, fmt.Errorf("create announcement: %w", err)
	}
	return AdminAnnouncementView{
		ID: id, Title: title, Body: body, Published: published, PublishedAt: at,
	}, nil
}

// AdminListRedeemCodes 返回兑换码。
func (s *Service) AdminListRedeemCodes(ctx context.Context, limit, offset int) ([]AdminRedeemCodeView, int64, error) {
	// ⚠️ 2026-10-21（二轮审计 P2）：负 offset 必须夹到 0 ——
	// 否则 PostgreSQL 报 `OFFSET must not be negative` → 整页 500。
	if offset < 0 {
		offset = 0
	}
	// ⚠️ 2026-10-21（二轮审计 P2-6）：`total` 必须是**全量行数**，
	// 不是当前页的条数 —— 前端要拿它算页数。
	var total int64
	if err := s.pool.QueryRow(ctx, `SELECT COUNT(*) FROM redeem_codes`).Scan(&total); err != nil {
		return nil, 0, fmt.Errorf("admin redeem_codes count: %w", err)
	}
	// ⚠️ 2026-10-21：limit 必须夹住 ——
	// `limit=0` 在 SQL 里是「**返回零行**」（不是「不限」），
	// `limit=999999` 则是全表扫描。
	if limit <= 0 || limit > 500 {
		limit = 50
	}
	rows, err := s.pool.Query(ctx,
		`SELECT id, code, reward, max_uses, used_count, expires_at, enabled FROM redeem_codes ORDER BY id LIMIT $1 OFFSET $2`, limit, offset)
	if err != nil {
		return nil, 0, err
	}
	defer rows.Close()
	var out []AdminRedeemCodeView
	for rows.Next() {
		var id, maxUses, used int
		var code string
		var rewardRaw []byte
		var expires *time.Time
		var enabled bool
		if err := rows.Scan(&id, &code, &rewardRaw, &maxUses, &used, &expires, &enabled); err != nil {
			return nil, 0, err
		}
		var reward map[string]int
		// fail-loud: a broken reward map would otherwise read as "this code
		// grants nothing".
		if err := json.Unmarshal(rewardRaw, &reward); err != nil {
			return nil, 0, fmt.Errorf("admin redeem codes: code %s has invalid reward jsonb: %w", code, err)
		}
		out = append(out, AdminRedeemCodeView{
			ID: id, Code: code, Reward: reward, MaxUses: maxUses,
			UsedCount: used, ExpiresAt: expires, Enabled: enabled,
		})
	}
	return out, total, rows.Err()
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
func (s *Service) AdminCreateRedeemCode(ctx context.Context, code string, reward map[string]int, maxUses int, expiresAt *time.Time) (AdminRedeemCodeView, error) {
	raw, err := json.Marshal(reward)
	if err != nil {
		return AdminRedeemCodeView{}, err
	}
	var id int
	if err := s.pool.QueryRow(ctx,
		`INSERT INTO redeem_codes (code, reward, max_uses, expires_at)
		 VALUES ($1,$2,$3,$4) RETURNING id`,
		code, raw, maxUses, expiresAt).Scan(&id); err != nil {
		return AdminRedeemCodeView{}, fmt.Errorf("create redeem code: %w", err)
	}
	return AdminRedeemCodeView{
		ID: id, Code: code, Reward: reward, MaxUses: maxUses, ExpiresAt: expiresAt,
	}, nil
}

// AdminListAuditLogs 返回操作审计。
func (s *Service) AdminListAuditLogs(ctx context.Context, limit, offset int) ([]AdminAuditLogView, int64, error) {
	if limit <= 0 || limit > 500 {
		limit = 100
	}
	// ⚠️ 2026-10-21（二轮审计 P2）：负 offset / 负 limit 必须夹住 ——
	// 否则 PostgreSQL 直接报 `OFFSET must not be negative` → 整页 500。
	if offset < 0 {
		offset = 0
	}
	// ⚠️ 2026-10-21（二轮审计 P2-6）：`total` 必须是**全量行数**，
	// 不是当前页的条数 —— 前端要拿它算页数。
	var total int64
	if err := s.pool.QueryRow(ctx, `SELECT COUNT(*) FROM admin_audit_logs`).Scan(&total); err != nil {
		return nil, 0, fmt.Errorf("admin admin_audit_logs count: %w", err)
	}
	rows, err := s.pool.Query(ctx,
		`SELECT id, admin_id, username, action, target, detail, created_at
		 FROM admin_audit_logs ORDER BY id DESC LIMIT $1 OFFSET $2`, limit, offset)
	if err != nil {
		return nil, 0, err
	}
	defer rows.Close()
	var out []AdminAuditLogView
	for rows.Next() {
		var id, adminID *int64
		var username, action, target string
		var detail []byte
		var createdAt time.Time
		if err := rows.Scan(&id, &adminID, &username, &action, &target, &detail, &createdAt); err != nil {
			return nil, 0, err
		}
		var d any
		// 2026-10-21: this used to be `_ = json.Unmarshal(detail, &d)`, so a
		// broken payload became null -- indistinguishable from "no detail at
		// all". The write side already writes {"detail_error": ...}; the read
		// side must not pretend a broken payload is an empty one.
		if err := json.Unmarshal(detail, &d); err != nil {
			return nil, 0, fmt.Errorf("admin audit logs: entry %v has invalid detail jsonb: %w", id, err)
		}
		out = append(out, AdminAuditLogView{
			ID: id, AdminID: adminID, Username: username, Action: action,
			Target: target, Detail: d, CreatedAt: createdAt,
		})
	}
	return out, total, rows.Err()
}
