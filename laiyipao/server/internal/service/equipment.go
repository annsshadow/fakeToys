package service

import (
	"context"
	"fmt"
	"sort"
	"strconv"
	"strings"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/domain"
)

// ─────────────────────────────────────────────────────────────────────────────
// 装备：获取与装配（2026-10-21 新增，P4-1）
//
// # 为什么要有这个文件
//
// 装备的**战斗效果**一直存在（`base_armor` / `base_bonus_pct` 被
// `attackerFromParts` 读进攻方），但**获取与装配**此前完全没有：
//
//   - `user_equipment` 表只在新号注册时写两行（equipment 1 与 10）；
//   - 18 件目录里**另外 16 件玩家永远拿不到**；
//   - **没有任何装备/卸下端点** —— 客户端背包页的装备区是纯只读目录；
//   - 于是玩家看得到 18 件装备，买不到、装不了、也换不了。
//
// 这正是 KNOWN_LIMITS 第 25 条的教训：「服务端有扣减逻辑 ≠ 玩家能用」。
// 装备的数值有效果，但玩家**碰不到**它 —— 那等于没有。
//
// # 三条获取途径
//
//  1. **等级解锁**（与技能同款）：结算时按 `floor(level_exp / 100)`
//     补发 `unlock_level <= 等级` 的装备。`ON CONFLICT DO NOTHING` 幂等，
//     老玩家任意一场结算即回填。
//  2. **商城购买**：payload 键 `equipment_<id>`（见 equipmentPayloadPrefix）。
//  3. **新号礼包**（已有：equipment 1 与 10）。
//
// # 装配语义
//
// 一个部位（slot）同时只能装一件。装配是**整组替换**（`SetEquippedEquipment`）
// 而不是「装一件/卸一件」两个端点 —— 后者会留下「同部位两件都 equipped」
// 的中间态，而那正是「部分生效」那一族缺陷的温床。
//
// ─────────────────────────────────────────────────────────────────────────────

// equipmentPayloadPrefix 是商城 payload 里「发一件指定装备」的前缀。
//
// 形如 `{"equipment_5": 1}` 表示「发 1 件 id=5 的装备」。
//
// 为什么用前缀而不是一个 `equipment_instance`：
//   - 宝石之所以用 `gem_instance`，是因为宝石**词条随机**（品质 + 词条池），
//     玩家买的是「一颗随机的宝石」；
//   - 装备**没有随机性** —— 18 件是固定目录，玩家买的就是那一件。
//     用固定 id 才能让商城写出「商品名 ↔ 具体装备」的对应关系。
const equipmentPayloadPrefix = "equipment_"

// equipmentPayloadID 从 payload 键解析出装备 id；不是装备键则返回 ok=false。
func equipmentPayloadID(key string) (int, bool) {
	if !strings.HasPrefix(key, equipmentPayloadPrefix) {
		return 0, false
	}
	id, err := strconv.Atoi(strings.TrimPrefix(key, equipmentPayloadPrefix))
	if err != nil || id <= 0 {
		return 0, false
	}
	return id, true
}

// isEquipmentPayloadKey 报告这个 payload 键是不是「发一件装备」。
//
// ⚠️ 守卫必须**引用生产代码的判定**，不要把前缀抄一遍 ——
// 抄两遍时拼错的表现是「买了什么都没发生」，
// 而那恰好是最难排查的一类症状（KNOWN_LIMITS 第 26 条）。
func isEquipmentPayloadKey(key string) bool {
	_, ok := equipmentPayloadID(key)
	return ok
}

// EquipmentView 是玩家拥有的一件装备（目录信息 + 拥有状态）。
type EquipmentView struct {
	ID      int    `json:"id"`
	Code    string `json:"code"`
	Name    string `json:"name"`
	Slot    string `json:"slot"`
	Tier    int    `json:"tier"`
	Element string `json:"element"`
	Descr   string `json:"descr"`
	// BaseArmor / BaseBonusPct 是**目录值**，与玩家是否装备无关 ——
	// 玩家要能比较「这件加多少」再决定穿不穿。
	BaseArmor    int64 `json:"base_armor"`
	BaseBonusPct int64 `json:"base_bonus_pct"`
	UnlockLevel  int   `json:"unlock_level"`
	// Owned 报告玩家是否**拥有**这一件。
	Owned bool `json:"owned"`
	// Equipped 报告是否已装配。
	Equipped bool `json:"equipped"`
	// Level / Star 是玩家持有实例的养成值。
	//
	// ⚠️ 2026-10-21：**这两个字段目前无任何效果**（KNOWN_LIMITS 第 10 条：
	// 实测「攻击侧越过封顶、护甲侧吃满只多买 0.68% 血量」，
	// 所以刻意**不做**缩放）。它们仍然随实例存库，是为了让将来要做时
	// 不必回填历史数据。
	Level int `json:"level"`
	Star  int `json:"star"`
}

// equipmentCatalog 是装备目录（id → 定义），带缓存。
var equipmentCatalog = func() map[int]domain.SeedEquipment {
	m := make(map[int]domain.SeedEquipment, len(domain.SeedEquipmentList))
	for _, e := range domain.SeedEquipmentList {
		m[e.ID] = e
	}
	return m
}()

// ListEquipment 返回**全部目录** + 玩家的拥有/装配状态。
//
// 为什么返回全部目录而不是只返回已拥有的：
// 玩家要能看到「18 件里我只有 2 件」，那 16 件的解锁等级本身就是目标。
// 只列已拥有的会让目录失去意义（玩家不知道有什么可争取）。
func (s *Service) ListEquipment(ctx context.Context, userID int64) ([]EquipmentView, error) {
	rows, err := s.pool.Query(ctx,
		`SELECT equipment_id, slot, equipped, level, star
		 FROM user_equipment WHERE user_id = $1`, userID)
	if err != nil {
		return nil, fmt.Errorf("load user equipment: %w", err)
	}
	defer rows.Close()
	owned := map[int]EquipmentView{}
	for rows.Next() {
		var id int
		var slot string
		var equipped bool
		var level, star int
		if err := rows.Scan(&id, &slot, &equipped, &level, &star); err != nil {
			return nil, err
		}
		owned[id] = EquipmentView{
			ID: id, Slot: slot, Equipped: equipped, Level: level, Star: star,
		}
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}

	out := make([]EquipmentView, 0, len(equipmentCatalog))
	for id, def := range equipmentCatalog {
		v := EquipmentView{
			ID: id, Code: def.Code, Name: def.Name, Slot: def.Slot, Tier: def.Tier,
			Element: string(def.Element), Descr: def.Descr,
			BaseArmor: def.BaseArmor, BaseBonusPct: def.BaseBonusPct,
			UnlockLevel: def.UnlockLevel,
		}
		if o, ok := owned[id]; ok {
			v.Owned = true
			v.Equipped = o.Equipped
			v.Level = o.Level
			v.Star = o.Star
		}
		out = append(out, v)
	}
	// 排序：按部位 → 阶级 → id（稳定，运营与玩家看到的顺序一致）。
	sort.Slice(out, func(i, j int) bool {
		if out[i].Slot != out[j].Slot {
			return out[i].Slot < out[j].Slot
		}
		if out[i].Tier != out[j].Tier {
			return out[i].Tier < out[j].Tier
		}
		return out[i].ID < out[j].ID
	})
	return out, nil
}

// SetEquippedEquipment 整组替换玩家的装配集合。
//
// # 语义
//
// body 是「玩家想要装备的 equipment_id 列表」（**完整集合**，不是增量）。
// 服务端在一个事务里：
//  1. 校验每一件都已拥有（没拥有 → 400，零副作用）；
//  2. 校验同部位不超过一件（超了 → 400）；
//  3. 把未列出的已装配件置为未装配，把列出的置为已装配。
//
// # 为什么是「整组替换」而不是「装一件 / 卸一件」
//
// 两个增量端点会留下中间态：「装上第二把武器」要么被服务端默默拒绝
// （前端以为成功了）、要么两把都 equipped（攻方算成两倍）。
// 整组替换让「不变量」只存在于**一个地方**：提交的那一组。
func (s *Service) SetEquippedEquipment(ctx context.Context, userID int64, ids []int) ([]EquipmentView, error) {
	// 去重（同一件重复出现不应算成两件）
	seen := map[int]bool{}
	uniq := make([]int, 0, len(ids))
	for _, id := range ids {
		if id <= 0 {
			return nil, fmt.Errorf("%w: 装备 id 非法", ErrBadInput)
		}
		if seen[id] {
			continue
		}
		seen[id] = true
		uniq = append(uniq, id)
	}

	err := s.DB.Tx(ctx, func(tx txType) error {
		// 1) 校验拥有
		for _, id := range uniq {
			def, ok := equipmentCatalog[id]
			if !ok {
				return fmt.Errorf("%w: 装备 %d 不在目录里", ErrBadInput, id)
			}
			var owned bool
			if err := tx.QueryRow(ctx,
				`SELECT EXISTS(SELECT 1 FROM user_equipment WHERE user_id = $1 AND equipment_id = $2)`,
				userID, id).Scan(&owned); err != nil {
				return fmt.Errorf("check equipment ownership: %w", err)
			}
			if !owned {
				return fmt.Errorf("%w: 尚未拥有装备 %d（%s）", ErrBadInput, id, def.Name)
			}
		}
		// 2) 校验同部位唯一
		slotOf := map[int]string{}
		for _, id := range uniq {
			slotOf[id] = equipmentCatalog[id].Slot
		}
		slotSeen := map[string]int{}
		for _, id := range uniq {
			slot := slotOf[id]
			if prev, dup := slotSeen[slot]; dup {
				return fmt.Errorf("%w: 部位 %s 只能装一件（%d 与 %d 冲突）",
					ErrBadInput, slot, prev, id)
			}
			slotSeen[slot] = id
		}
		// 3) 整组替换
		if _, err := tx.Exec(ctx,
			`UPDATE user_equipment SET equipped = FALSE WHERE user_id = $1`, userID); err != nil {
			return fmt.Errorf("unequip all: %w", err)
		}
		for _, id := range uniq {
			if _, err := tx.Exec(ctx,
				`UPDATE user_equipment SET equipped = TRUE WHERE user_id = $1 AND equipment_id = $2`,
				userID, id); err != nil {
				return fmt.Errorf("equip %d: %w", id, err)
			}
		}
		return nil
	})
	if err != nil {
		return nil, err
	}
	return s.ListEquipment(ctx, userID)
}

// grantEquipmentByLevel 按玩家等级补发装备（与技能解锁同款口径）。
//
// ⚠️ **幂等**：`ON CONFLICT DO NOTHING`。
// 老玩家不需要回填脚本 —— 任意一场结算即按当前等级补齐。
//
// ⚠️ 等级口径只有一处定义（`domain.PlayerLevelDivisor`）——
// 同一个量在两处各写一份常量，漂了不报错（工程约束 4）。
func grantEquipmentByLevel(ctx context.Context, tx pgx.Tx, userID int64) error {
	_, err := tx.Exec(ctx, `
		INSERT INTO user_equipment (user_id, equipment_id, slot, equipped)
		SELECT $1, e.id, e.slot, FALSE
		  FROM equipment e
		 WHERE e.unlock_level <= (SELECT FLOOR(p.level_exp / $2) FROM user_progress p WHERE p.user_id = $1)
		ON CONFLICT (user_id, equipment_id) DO NOTHING`,
		userID, domain.PlayerLevelDivisor)
	if err != nil {
		return fmt.Errorf("unlock equipment by level: %w", err)
	}
	return nil
}

// mintEquipment 发出**一件指定**装备（商城购买走这条）。
func mintEquipment(ctx context.Context, tx pgx.Tx, userID int64, equipmentID int) error {
	def, ok := equipmentCatalog[equipmentID]
	if !ok {
		return fmt.Errorf("%w: 装备 %d 不在目录里", ErrBadInput, equipmentID)
	}
	if _, err := tx.Exec(ctx,
		`INSERT INTO user_equipment (user_id, equipment_id, slot, equipped)
		 VALUES ($1, $2, $3, FALSE)
		 ON CONFLICT (user_id, equipment_id) DO NOTHING`,
		userID, def.ID, def.Slot); err != nil {
		return fmt.Errorf("grant equipment %d: %w", equipmentID, err)
	}
	return nil
}
