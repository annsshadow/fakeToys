// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 第 20 轮：宝石佩戴。`user_gems.equipped_slot` 在此之前全仓零写入 ——
// 商城卖出的 gem_instance 永远躺在背包里，`loadGemsAffixRows` 的
// `WHERE equipped_slot <> ''` 恒空，**宝石词条对所有玩家零生效**。
//
// 判据三层：
//  1. 佩戴写入 equipped_slot，且构筑消费侧真的吃到词条
//     （applyLoadoutContribution 的 AffixCount 增长 —— 这是「生效」的证明，
//     而不只是「字段变了」）；
//  2. 上限 EquipSlotLimit 与行锁并发安全；
//  3. 归属隔离与幂等。

import (
	"context"
	"errors"
	"sync"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// equipTestGems 买 n 颗宝石（走真实商城路径，mintGem roll 真词条）。
func equipTestGems(t *testing.T, ts *testService, uid int64, n int) []GemView {
	t.Helper()
	ctx := context.Background()
	ts.grant(t, ctx, uid, map[string]int64{"gem": domain.GemShopPrice*int64(n) + 100})
	item := shopItemIDByCode(t, ts, ctx, "shop_gem")
	for i := 0; i < n; i++ {
		if _, err := ts.Buy(ctx, uid, item); err != nil {
			t.Fatalf("第 %d 次购买宝石失败：%v", i+1, err)
		}
	}
	gems, err := ts.ListGems(ctx, uid)
	if err != nil {
		t.Fatalf("列宝石失败：%v", err)
	}
	return gems
}

// insertTestGems 直接造 n 颗宝石实例（商城每日限购 3，上限测试需要 8 颗 ——
// 走商城会被限购挡住；词条/品质本测试不关心，固定值即可）。
func insertTestGems(t *testing.T, ts *testService, uid int64, n int) []GemView {
	t.Helper()
	for i := 0; i < n; i++ {
		ts.exec(t, `
			INSERT INTO user_gems (user_id, gem_id, quality, affixes)
			VALUES ($1, 1, 'rare', '[{"affix":"attack","value":10}]'::jsonb)`, uid)
	}
	gems, err := ts.ListGems(context.Background(), uid)
	if err != nil {
		t.Fatalf("列宝石失败：%v", err)
	}
	return gems
}

func TestEquipGemMakesAffixesCount(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	gems := equipTestGems(t, ts, uid, 1)
	gem := gems[0]

	// 佩戴前：词条不进构筑（equipped_slot 恒空）。
	before, err := ts.loadGemsAffixRows(ctx, uid)
	if err != nil {
		t.Fatalf("读佩戴宝石失败：%v", err)
	}
	if len(before) != 0 {
		t.Fatalf("佩戴前不应有词条进构筑，实际 %d 份", len(before))
	}

	got, err := ts.EquipGem(ctx, uid, gem.ID)
	if err != nil {
		t.Fatalf("佩戴失败：%v", err)
	}
	if got.Equipped == "" {
		t.Fatal("佩戴后 equipped_slot 必须非空")
	}
	after, err := ts.loadGemsAffixRows(ctx, uid)
	if err != nil {
		t.Fatalf("读佩戴宝石失败：%v", err)
	}
	if len(after) != 1 {
		t.Fatalf("佩戴后应有 1 份词条进构筑，实际 %d 份", len(after))
	}
	// 构筑装配真的计数（已佩戴宝石数 + 词条被消费）。
	c := applyLoadoutContribution(nil, after)
	if c.EquippedGemCount != 1 {
		t.Fatalf("佩戴宝石必须计入构筑（EquippedGemCount=%d 说明消费链路没接上）", c.EquippedGemCount)
	}
}

func TestUnequipGemRemovesAffixesAndIsIdempotent(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	gems := equipTestGems(t, ts, uid, 1)

	if _, err := ts.EquipGem(ctx, uid, gems[0].ID); err != nil {
		t.Fatalf("佩戴失败：%v", err)
	}
	got, err := ts.UnequipGem(ctx, uid, gems[0].ID)
	if err != nil {
		t.Fatalf("卸下失败：%v", err)
	}
	if got.Equipped != "" {
		t.Fatalf("卸下后 equipped_slot 必须为空，实际 %q", got.Equipped)
	}
	after, err := ts.loadGemsAffixRows(ctx, uid)
	if err != nil || len(after) != 0 {
		t.Fatalf("卸下后不应有词条进构筑（n=%d err=%v）", len(after), err)
	}
	// 幂等：再卸一次不报错。
	if _, err := ts.UnequipGem(ctx, uid, gems[0].ID); err != nil {
		t.Fatalf("重复卸下应幂等：%v", err)
	}
	// 重复佩戴同样幂等。
	if _, err := ts.EquipGem(ctx, uid, gems[0].ID); err != nil {
		t.Fatalf("佩戴失败：%v", err)
	}
	if _, err := ts.EquipGem(ctx, uid, gems[0].ID); err != nil {
		t.Fatalf("重复佩戴应幂等：%v", err)
	}
}

func TestEquipGemRejectsBeyondLimit(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	gems := insertTestGems(t, ts, uid, EquipSlotLimit+2)

	for i := 0; i < EquipSlotLimit; i++ {
		if _, err := ts.EquipGem(ctx, uid, gems[i].ID); err != nil {
			t.Fatalf("第 %d 颗佩戴失败：%v", i+1, err)
		}
	}
	// 第 7 颗：顺序拒绝。
	if _, err := ts.EquipGem(ctx, uid, gems[EquipSlotLimit].ID); !errors.Is(err, ErrBadInput) {
		t.Fatalf("超过上限应 ErrBadInput，实际 %v", err)
	}
	// 并发第 7 颗（两路同时）：行锁串行化后至多成功一颗，总数不越界。
	var wg sync.WaitGroup
	errs := make([]error, 2)
	for i := 0; i < 2; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			_, errs[i] = ts.EquipGem(ctx, uid, gems[EquipSlotLimit+1].ID)
		}(i)
	}
	wg.Wait()
	// gems[EquipSlotLimit] 顺序拒绝后未佩戴，并发两路争的是同一个空位：
	// 至多一个成功。
	success := 0
	for _, err := range errs {
		if err == nil {
			success++
		}
	}
	if success > 1 {
		t.Fatalf("并发佩戴同一空位至多成功一颗，实际 %d", success)
	}
	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM user_gems WHERE user_id = $1 AND equipped_slot <> ''`, uid).Scan(&n); err != nil {
		t.Fatal(err)
	}
	if n > EquipSlotLimit {
		t.Fatalf("佩戴数越过上限：实际 %d", n)
	}
}

func TestEquipGemOwnershipAndSlots(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	other := ts.newUser(t, ctx)
	gems := equipTestGems(t, ts, uid, 3)

	// 别人的宝石 404。
	if _, err := ts.EquipGem(ctx, other, gems[0].ID); !errors.Is(err, ErrNotFound) {
		t.Fatalf("佩戴他人宝石应 ErrNotFound，实际 %v", err)
	}
	// 槽位自动分配：1、2、3 互不重复。
	seen := map[string]bool{}
	for _, g := range gems {
		if _, err := ts.EquipGem(ctx, uid, g.ID); err != nil {
			t.Fatalf("佩戴 %d 失败：%v", g.ID, err)
		}
	}
	reloaded, err := ts.ListGems(ctx, uid)
	if err != nil {
		t.Fatal(err)
	}
	for _, g := range reloaded {
		if g.Equipped == "" {
			t.Errorf("宝石 %d 应处于佩戴态", g.ID)
			continue
		}
		if seen[g.Equipped] {
			t.Errorf("槽位 %s 被两颗宝石占用（唯一性破坏）", g.Equipped)
		}
		seen[g.Equipped] = true
	}
	if len(seen) != 3 {
		t.Errorf("应有 3 个不同槽位，实际 %v", seen)
	}
}
