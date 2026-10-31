package service

// 后台经济流水页缓存的集成判据（2026-10-31 新一轮第 28 轮）。
//
// 修前 `AdminEconomy` 每次请求都对增长最快的 `wallet_flows` 做全表聚合。
// 修后走 TTL 缓存（config.EconomyCacheTTL，0 = 关闭）。
//
// 判据沿用本仓的「改底表」手法：**首次调用成功之后，把 wallet_flows 改名**。
// 第二次调用若仍成功，说明流水来自缓存 —— 那正是「缓存命中」的定义。

import (
	"context"
	"testing"
)

func TestAdminEconomyServedFromCache(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 造用户 + 一笔流水
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"coin": 100})

	first, err := ts.AdminEconomy(ctx)
	if err != nil {
		t.Fatalf("首次 AdminEconomy 失败：%v", err)
	}
	if len(first.Flows) == 0 {
		t.Fatal("夹具自检失败：流水不应为空")
	}

	// 把 wallet_flows 改名 —— 之后任何真实查询都必然报错
	ts.renameTable(t, "wallet_flows", "wallet_flows_bak")
	defer ts.renameTable(t, "wallet_flows_bak", "wallet_flows")

	// TTL 内第二次调用应命中缓存（shop_items 查询仍正常）
	second, err := ts.AdminEconomy(ctx)
	if err != nil {
		t.Fatalf("TTL 内第二次调用应命中缓存（流水表已改名，查库必失败），实际报错：%v", err)
	}
	if len(second.Flows) != len(first.Flows) {
		t.Errorf("缓存快照应等于首次结果：first=%d 条 second=%d 条",
			len(first.Flows), len(second.Flows))
	}
	for i := range first.Flows {
		if first.Flows[i] != second.Flows[i] {
			t.Errorf("缓存快照第 %d 项不一致：first=%+v second=%+v",
				i, first.Flows[i], second.Flows[i])
		}
	}
}