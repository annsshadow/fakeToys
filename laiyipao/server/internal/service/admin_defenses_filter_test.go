package service

// 后台防线列表按防线名/玩家昵称搜索 + owner_id 过滤的测试
// （2026-10-31 新一轮第 19 轮）。

import (
	"context"
	"testing"
)

func TestAdminListDefensesKeywordFilter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uidA, _ := newGuestUser(t, ts, "def_a")
	uidB, _ := newGuestUser(t, ts, "def_b")
	seedDefense(t, ts, uidA, "铁壁防线")
	seedDefense(t, ts, uidB, "尖刺陷阱")

	// 按防线名搜索 → 1 条
	items, total, err := ts.AdminListDefenses(ctx, 50, 0, "铁壁", 0)
	if err != nil {
		t.Fatalf("防线名搜索失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].Name != "铁壁防线" {
		t.Errorf("搜「铁壁」应 1 条，实际 total=%d items=%+v", total, items)
	}

	// 按 owner_id 过滤 → 1 条（A 的防线）
	items, total, err = ts.AdminListDefenses(ctx, 50, 0, "", uidA)
	if err != nil {
		t.Fatalf("owner_id 过滤失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].OwnerID != uidA {
		t.Errorf("owner_id=%d 应 1 条，实际 total=%d", uidA, total)
	}

	// 无筛选 → 2 条
	items, total, err = ts.AdminListDefenses(ctx, 50, 0, "", 0)
	if err != nil {
		t.Fatalf("无筛选失败：%v", err)
	}
	if total != 2 {
		t.Errorf("无筛选应 2 条，实际 %d", total)
	}
}

func TestAdminListDefensesKeywordEscapesWildcards(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uidA, _ := newGuestUser(t, ts, "def_wild_a")
	uidB, _ := newGuestUser(t, ts, "def_wild_b")
	seedDefense(t, ts, uidA, "100%防线")
	seedDefense(t, ts, uidB, "普通防线")

	// 搜「%」应只命中名字里带 % 的防线（通配符被转义为字面量）
	items, total, err := ts.AdminListDefenses(ctx, 50, 0, "%", 0)
	if err != nil {
		t.Fatalf("通配符搜索失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].Name != "100%防线" {
		t.Errorf("搜「%%」应只命中 100%%防线，实际 total=%d items=%+v", total, items)
	}
}