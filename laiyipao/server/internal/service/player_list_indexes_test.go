package service

// 玩家列表端点索引的存在性守卫（2026-10-31 新一轮第 2 轮）。
//
// 迁移 00021 加的三个索引，服务的是 /me 下的三个玩家侧列表端点：
//
//   orders            MyOrders        WHERE user_id = $1 ORDER BY created_at DESC
//   redeem_usages     MyRedemptions   同上
//   user_purchases    MyPurchases     同上
//
// 失效方式与看板索引完全一样是**静默**的：索引被删/改名之后，所有查询
// 照常返回正确结果，只是变成顺序扫描 —— 数据量小时一点都看不出来，
// 流水表到十万行时「我的」页每次打开都白扫全表。
// 没有任何业务断言会发现它，所以必须直接问 PostgreSQL。
//
// ⚠️ 无 Postgres 时 openScratchService 会 Skip。那一刻这条断言是
// **未执行**，不是「通过」。

import (
	"context"
	"strings"
	"testing"
)

func TestPlayerListIndexesExist(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	want := []struct {
		name string
		// 只断言「首列是 user_id、第二列是 created_at」，不断言方向：
		// btree 索引可以反向扫描，`(user_id, created_at ASC)` 同样能服务
		// `ORDER BY created_at DESC`。列序才是关键 ——
		// `(created_at, user_id)` 与 `(user_id, created_at)` 是两个完全
		// 不同的索引，后者的首列是 user_id，按 user_id 过滤才用得上。
		// 这正是本轮之前的三个索引各自的问题。
		columns string
	}{
		{"idx_orders_user", "(user_id, created_at"},
		{"idx_redeem_usages_user", "(user_id, created_at"},
		{"idx_user_purchases_user", "(user_id, created_at"},
	}

	for _, w := range want {
		var def string
		err := ts.db.Pool.QueryRow(ctx,
			`SELECT indexdef FROM pg_indexes WHERE schemaname = 'public' AND indexname = $1`,
			w.name).Scan(&def)
		if err != nil {
			t.Errorf("索引 %s 不存在（迁移 00021 没跑？）：%v", w.name, err)
			continue
		}
		// indexdef 形如：CREATE INDEX xxx ON public.t USING btree (user_id, created_at DESC)
		if !strings.Contains(def, w.columns) {
			t.Errorf("索引 %s 的列序不对：期望 (%s...)，实际定义 %s\n"+
				"列序反了等于没有这个索引 —— 按 user_id 过滤时用不到首列之后的列",
				w.name, w.columns, def)
		}
	}
}