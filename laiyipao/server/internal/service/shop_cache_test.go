package service

// LoadShop 静态商品列表缓存的集成判据（2026-10-31 新一轮第 6 轮）。
//
// 修前 `LoadShop` 每次请求都重新读整张 `shop_items` 配置表，并对每个商品
// 各做一次关联子查询计数；修后商品静态部分走 TTL 缓存（TTL 见
// config.ShopCacheTTL，0 = 关闭），玩家今日已购次数改为一条分组查询。
//
// 判据沿用本仓的「改底表」手法：**首次调用成功之后，把 shop_items 改名**。
// 第二次调用若仍成功，说明商品列表来自缓存 —— 那正是「缓存命中」的定义。
//
// ⚠️ 无 Postgres 时 openScratchService 会 Skip —— 那一刻这条断言是
// **未执行**，不是「通过」。

import (
	"context"
	"testing"
)

func TestShopServedFromCache(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx)

	first, err := ts.LoadShop(ctx, uid)
	if err != nil {
		t.Fatalf("首次 LoadShop 失败：%v", err)
	}
	if len(first) == 0 {
		t.Fatal("夹具自检失败：种子后商城应非空")
	}

	// 把商品表改名 —— 之后任何真实查询都必然报错
	ts.renameTable(t, "shop_items", "shop_items_bak")
	defer ts.renameTable(t, "shop_items_bak", "shop_items")

	// TTL 内第二次调用应命中缓存（user_purchases 分组查询仍正常）
	second, err := ts.LoadShop(ctx, uid)
	if err != nil {
		t.Fatalf("TTL 内第二次调用应命中缓存（商品表已改名，查库必失败），实际报错：%v", err)
	}
	if len(second) != len(first) {
		t.Fatalf("缓存快照应等于首次结果：first=%d 条 second=%d 条", len(first), len(second))
	}
	for i := range first {
		if first[i].ID != second[i].ID || first[i].Name != second[i].Name {
			t.Errorf("缓存快照第 %d 项不一致：first=%+v second=%+v", i, first[i], second[i])
		}
	}
}