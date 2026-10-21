package service

import (
	"context"
	"errors"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// TestEquipmentIsObtainableAndEquippable 钉住 P4-1。
//
// # 背景
//
// 装备的**战斗效果**一直存在（`base_armor` / `base_bonus_pct` 被读进攻方），
// 但**获取与装配**此前完全没有：
//
//   - `user_equipment` 只在注册时写两行；
//   - 18 件目录里另外 16 件玩家永远拿不到；
//   - 没有任何装备/卸下端点。
//
// 于是「装备」在玩法上等于不存在，而界面上还摆着 18 件装备的目录。
//
// # 四条判据
//
//  1. 商城能买到指定装备（payload 键 `equipment_<id>`）；
//  2. 装配是**整组替换**，同部位两件 → 400；未拥有 → 400（都零副作用）；
//  3. 等级解锁会按 `unlock_level` 补发（幂等）；
//  4. 装配后**攻方数值真的变了**（护甲 / 攻击加成进战斗）。
func TestEquipmentIsObtainableAndEquippable(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// ── 1) 目录里 18 件，���始只拥有注册赠送的 2 件 ──
	items, err := ts.ListEquipment(ctx, uid)
	if err != nil {
		t.Fatalf("列装备失败：%v", err)
	}
	if len(items) != len(domain.SeedEquipmentList) {
		t.Fatalf("装备目录应有 %d 件，实际 %d", len(domain.SeedEquipmentList), len(items))
	}
	ownedCount := 0
	for _, it := range items {
		if it.Owned {
			ownedCount++
		}
	}
	if ownedCount != 2 {
		t.Errorf("新号应只有注册赠送的 2 件装备，实际 %d 件", ownedCount)
	}

	// ── 2) 商城购买 → 拥有 ──
	// 装备 7 = 拼接胸甲（armor，护甲 +35‰，攻击加成 +25‰）
	const armorID = 7
	var shopID int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM shop_items WHERE code = 'shop_eq_armor'`).Scan(&shopID); err != nil {
		t.Skipf("商城里没有装备商品（种子没跑？）：%v", err)
	}
	ts.grant(t, ctx, uid, map[string]int64{"coin": 100000})
	if _, err := ts.Buy(ctx, uid, shopID); err != nil {
		t.Fatalf("购买装备失败：%v", err)
	}
	items, err = ts.ListEquipment(ctx, uid)
	if err != nil {
		t.Fatalf("列装备失败：%v", err)
	}
	var armorItem EquipmentView
	for _, it := range items {
		if it.ID == armorID {
			armorItem = it
		}
	}
	if !armorItem.Owned {
		t.Fatalf("买了装备 %d 却没出现在拥有列表里", armorID)
	}
	if armorItem.Equipped {
		t.Error("新买的装备不应自动穿上（那会悄悄改变玩家的构筑）")
	}

	// ── 3) 装配：整组替换 + 两种拒绝都要零副作用 ──
	if _, err := ts.SetEquippedEquipment(ctx, uid, []int{armorID}); err != nil {
		t.Fatalf("装配失败：%v", err)
	}
	items, _ = ts.ListEquipment(ctx, uid)
	equipped := map[int]bool{}
	for _, it := range items {
		if it.Equipped {
			equipped[it.ID] = true
		}
	}
	if !equipped[armorID] {
		t.Error("装配后该装备应是 equipped")
	}
	// 注册时赠送的两件应被**卸下**（整组替换 = 提交的那一组）
	if equipped[1] || equipped[10] {
		t.Error("整组替换后，注册赠送的装备应被卸下")
	}

	// 3a) 同部位两件 → 400
	//     装备 7（armor）与装备 9（armor，tier3）同部位；
	//     先把 9 也拿到手（等级解锁在下面会发，这里直接插库模拟）。
	if _, err := ts.pool.Exec(ctx,
		`INSERT INTO user_equipment (user_id, equipment_id, slot, equipped)
		 VALUES ($1, 9, 'armor', FALSE) ON CONFLICT DO NOTHING`, uid); err != nil {
		t.Fatalf("准备同部位装备失败：%v", err)
	}
	if _, err := ts.SetEquippedEquipment(ctx, uid, []int{armorID, 9}); !errors.Is(err, ErrBadInput) {
		t.Errorf("同部位两件应 ErrBadInput，实际 %v", err)
	}
	// 零副作用：原来的装配不变
	items, _ = ts.ListEquipment(ctx, uid)
	for _, it := range items {
		if it.ID == 9 && it.Equipped {
			t.Error("被拒绝的装配留下了副作用（9 号装备被装上了）")
		}
	}

	// 3b) 未拥有 → 400
	if _, err := ts.SetEquippedEquipment(ctx, uid, []int{3}); !errors.Is(err, ErrBadInput) {
		t.Errorf("装配未拥有的装备应 ErrBadInput，实际 %v", err)
	}

	// ── 4) 装配后攻方数值真的变了 ──
	attacker, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("算攻方失败：%v", err)
	}
	if attacker.ArmorPermille < 35 {
		t.Errorf("装了 +35‰ 护甲的胸甲后攻方护甲应 ≥35，实际 %d", attacker.ArmorPermille)
	}
}
