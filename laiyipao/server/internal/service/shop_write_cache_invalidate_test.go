// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 第 97 轮（task-185）：商城商品写后两个读缓存必须即时失效。
// 缺陷：AdminEconomy（后台经济看板，含 Shop 配置）与 LoadShop（玩家商城）
// 都把 shop_items 静态部分缓存 30 秒，但 AdminUpdateShopItem 写库后从不
// flush 任一缓存。运营调价或上下架后，后台重拉与玩家展示仍拿到旧商品，
// 而 Buy 实时读库按新价扣钱，显示价与实付价不一致。
// 对照：公告写路径写后均显式 flush announcementsCache，商品写路径应同口径。
// 判据：先让缓存装载旧值，再 UPDATE 真库，然后立刻读；仍返回旧值即失败。

import (
	"context"
	"testing"
)

func TestAdminUpdateShopItemInvalidatesReadCaches(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 装载玩家商城缓存，记录初始商品名。
	first, err := ts.LoadShop(ctx, uid)
	if err != nil {
		t.Fatalf("首次 LoadShop 失败：%v", err)
	}
	if len(first) == 0 {
		t.Fatal("夹具自检：种子后商城不应为空")
	}
	id := first[0].ID
	oldName := first[0].Name

	// 同时装载经济看板缓存，其 Shop 段同样含商品名。
	beforeEcon, err := ts.AdminEconomy(ctx)
	if err != nil {
		t.Fatalf("首次 AdminEconomy 失败：%v", err)
	}
	oldEconName := ""
	found := false
	for _, it := range beforeEcon.Shop {
		if it.ID == id {
			oldEconName = it.Name
			found = true
			break
		}
	}
	if !found {
		t.Fatal("夹具自检：经济看板应含该商品")
	}
	if oldEconName != oldName {
		t.Fatalf("夹具自检：两处商品名应一致，LoadShop=%q AdminEconomy=%q", oldName, oldEconName)
	}

	newName := oldName + "_r97sync"
	if _, err := ts.AdminUpdateShopItem(ctx, int64(id), map[string]any{"name": newName}); err != nil {
		t.Fatalf("AdminUpdateShopItem 失败：%v", err)
	}

	// 真库必须是新值，否则后续断言无意义。
	var dbName string
	if err := ts.pool.QueryRow(ctx,
		`SELECT name FROM shop_items WHERE id = $1`, id).Scan(&dbName); err != nil {
		t.Fatalf("读回真库失败：%v", err)
	}
	if dbName != newName {
		t.Fatalf("写库失败：期望 %q，实际 %q，后续断言无意义", newName, dbName)
	}

	// 1) 玩家商城：写后立刻重拉应见新名称。
	after, err := ts.LoadShop(ctx, uid)
	if err != nil {
		t.Fatalf("写后 LoadShop 失败：%v", err)
	}
	playerName := ""
	for _, it := range after {
		if it.ID == id {
			playerName = it.Name
			break
		}
	}
	if playerName != newName {
		t.Errorf("写后玩家商城仍返回旧名称（期望 %q，实际 %q）：shopCache 未在 AdminUpdateShopItem 后失效", newName, playerName)
	}

	// 2) 后台经济看板：写后立刻重拉应见新名称。
	afterEcon, err := ts.AdminEconomy(ctx)
	if err != nil {
		t.Fatalf("写后 AdminEconomy 失败：%v", err)
	}
	adminName := ""
	for _, it := range afterEcon.Shop {
		if it.ID == id {
			adminName = it.Name
			break
		}
	}
	if adminName != newName {
		t.Errorf("写后后台经济看板仍返回旧名称（期望 %q，实际 %q）：economyCache 未在 AdminUpdateShopItem 后失效", newName, adminName)
	}
}
