package service

// 公告列表缓存的集成判据（2026-10-31 实质优化第 22 轮）。
//
// 与签到日历缓存同一招：第一次调用成功后把 `announcements` 表改名，
// 第二次若仍成功返回相同数据 → 证明没有查库（缓存命中）。
//
// ⚠️ 无 Postgres 时 openScratchService 会 Skip —— 那一刻这些断言是
// **未执行**，不是「通过」。

import (
	"context"
	"testing"
)

// seedAnnouncements 插入 n 条已发布公告，返回首条标题。
func seedAnnouncements(t *testing.T, ts *testService, n int) string {
	t.Helper()
	ctx := context.Background()
	firstTitle := ""
	for i := 0; i < n; i++ {
		title := "公告"
		if i == 0 {
			title = "首条公告"
			firstTitle = title
		}
		if _, err := ts.db.Pool.Exec(ctx,
			`INSERT INTO announcements (title, body, published, published_at)
			 VALUES ($1, '正文', TRUE, now())`, title); err != nil {
			t.Fatalf("插入公告失败：%v", err)
		}
	}
	return firstTitle
}

func TestAnnouncementsServedFromCache(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	seedAnnouncements(t, ts, 2)

	first, err := ts.ListAnnouncements(ctx, 20)
	if err != nil {
		t.Fatalf("首次 ListAnnouncements 失败：%v", err)
	}
	// 排序是 published_at DESC：两条独立 INSERT 的 now() 不同，后插入的排前。
	// 夹具自检只看「有 2 条」且两条标题都在。
	if len(first) != 2 {
		t.Fatalf("夹具自检失败：应有 2 条公告，实际 %+v", first)
	}
	titles := map[string]bool{}
	for _, a := range first {
		titles[a.Title] = true
	}
	if !titles["首条公告"] || !titles["公告"] {
		t.Fatalf("夹具自检失败：两条公告标题缺失，实际 %+v", first)
	}

	// 把公告表改名 —— 之后任何一次真实查询都必然报错
	ts.renameTable(t, "announcements", "announcements_bak")
	defer ts.renameTable(t, "announcements_bak", "announcements")

	second, err := ts.ListAnnouncements(ctx, 20)
	if err != nil {
		t.Fatalf("TTL 内第二次调用应命中缓存（表已改名，查库必失败），实际报错：%v", err)
	}
	if len(second) != len(first) {
		t.Errorf("缓存快照应等于首次结果：first %d 条，second %d 条", len(first), len(second))
	}
	for i := range first {
		if second[i].ID != first[i].ID || second[i].Title != first[i].Title {
			t.Errorf("第 %d 条公告缓存快照不一致：first=%+v second=%+v", i, first[i], second[i])
		}
	}

	// 统计：第一次 miss，第二次 hit
	hits, misses := ts.announcementsCache.Stats()
	if hits < 1 {
		t.Errorf("第二次调用应计命中，实际 hits=%d", hits)
	}
	if misses < 1 {
		t.Errorf("第一次调用应计未命中，实际 misses=%d", misses)
	}

	// 手动失效后必须重新查库（此时表还是改名的，所以必须报错）
	ts.announcementsCache.flush()
	if _, err := ts.ListAnnouncements(ctx, 20); err == nil {
		t.Fatal("flush 后应重新查库（表已改名，必须报错）")
	}
}

func TestAnnouncementsCacheTTLZeroDisables(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 0 = 关闭缓存（与其它缓存同一条运维命令）
	ts.Cfg().AnnouncementsCacheTTL = 0
	seedAnnouncements(t, ts, 1)

	if _, err := ts.ListAnnouncements(ctx, 20); err != nil {
		t.Fatalf("首次 ListAnnouncements 失败：%v", err)
	}
	ts.renameTable(t, "announcements", "announcements_bak")
	defer ts.renameTable(t, "announcements_bak", "announcements")

	if _, err := ts.ListAnnouncements(ctx, 20); err == nil {
		t.Fatal("TTL=0 时缓存关闭，表已改名，第二次调用必须查库报错")
	}
}

func TestAnnouncementsLimitSlicing(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	seedAnnouncements(t, ts, 8)

	// limit=5 → 5 条；limit=20 → 全部 8 条（不同 limit 共享同一份缓存）
	five, err := ts.ListAnnouncements(ctx, 5)
	if err != nil {
		t.Fatalf("ListAnnouncements(5) 失败：%v", err)
	}
	if len(five) != 5 {
		t.Errorf("limit=5 应返回 5 条，实际 %d", len(five))
	}
	twenty, err := ts.ListAnnouncements(ctx, 20)
	if err != nil {
		t.Fatalf("ListAnnouncements(20) 失败：%v", err)
	}
	if len(twenty) != 8 {
		t.Errorf("limit=20 应返回全部 8 条，实际 %d", len(twenty))
	}
	// 同一份缓存快照：limit=5 应是 limit=20 结果的前 5 条（排序稳定）。
	twentyIDs := make([]int, 0, len(twenty))
	for _, a := range twenty {
		twentyIDs = append(twentyIDs, a.ID)
	}
	for i, a := range five {
		if i >= len(twenty) || a.ID != twentyIDs[i] {
			t.Errorf("limit=5 的第 %d 条（id=%d）不在 limit=20 的对应位置（缓存快照不一致）",
				i, a.ID)
			break
		}
	}
}
