package service

// 看板索引的存在性守卫（2026-10-23）。
//
// 迁移 00019 加的两个索引，服务的是看板里**按时间范围扫描**的那几段：
//
//	users            WHERE created_at >= CURRENT_DATE
//	battle_records   WHERE created_at >= CURRENT_DATE
//	battle_records   LEFT JOIN ... ON br.created_at >= d AND br.created_at < d+1day
//	wallet_flows     WHERE created_at >= ...
//
// 它们的失效方式是完全静默的：索引被删/改名之后，**所有查询照常返回正确结果**，
// 只是变成顺序扫描 —— 数据量小时一点都看不出来，百万行时看板开始转圈。
// 没有任何业务断言会发现它，所以必须直接问 PostgreSQL。
//
// ⚠️ 无 Postgres 时 openScratchService 会 Skip。那一刻这条断言是
// **未执行**，不是「通过」。

import (
	"context"
	"strings"
	"testing"
)

func TestDashboardIndexesExist(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	want := []struct {
		name string
		// 列序必须对：`(created_at, user_id)` 与 `(user_id, created_at)` 是
		// 两个完全不同的索引 —— 后者的首列是 user_id，范围条件用不上它。
		// 这正是 00019 之前的状态。
		columns string
	}{
		{"idx_battle_records_created", "created_at, user_id"},
		{"idx_wallet_flows_created", "created_at"},
	}

	for _, w := range want {
		var def string
		err := ts.db.Pool.QueryRow(ctx,
			`SELECT indexdef FROM pg_indexes WHERE schemaname = 'public' AND indexname = $1`,
			w.name).Scan(&def)
		if err != nil {
			t.Errorf("索引 %s 不存在（迁移 00019 没跑？）：%v", w.name, err)
			continue
		}
		// indexdef 形如：CREATE INDEX xxx ON public.t USING btree (created_at, user_id)
		if !strings.Contains(def, "("+w.columns+")") {
			t.Errorf("索引 %s 的列序不对：期望 (%s)，实际定义 %s\n"+
				"列序反了等于没有这个索引 —— 范围条件用不到非首列",
				w.name, w.columns, def)
		}
	}
}
