package service

// 公告写操作后玩家端缓存立即失效的测试（2026-11-01 新一轮第 59 轮）。
//
// 判据：写操作（创建/更新/删除）后，下一次 ListAnnouncements 必须
// **重新查库**（hits 不增加、misses 增加）—— 运营发布/撤回公告后
// 玩家端不再等 TTL 才看到新内容。

import (
	"context"
	"testing"
)

func TestAnnouncementWritesFlushCache(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	seedAnnouncements(t, ts, 2)

	// 第一次调用：填充缓存（miss）
	if _, err := ts.ListAnnouncements(ctx, 20); err != nil {
		t.Fatalf("首次 ListAnnouncements 失败：%v", err)
	}
	// 第二次调用：命中缓存
	if _, err := ts.ListAnnouncements(ctx, 20); err != nil {
		t.Fatalf("第二次 ListAnnouncements 失败：%v", err)
	}
	hitsAfterHits, _ := ts.announcementsCache.Stats()
	if hitsAfterHits < 1 {
		t.Fatalf("第二次调用应命中缓存，实际 hits=%d", hitsAfterHits)
	}

	// 创建公告 → 缓存失效 → 下一次调用重新查库（hits 不增加）
	if _, err := ts.AdminCreateAnnouncement(ctx, "新公告", "正文", true); err != nil {
		t.Fatalf("创建公告失败：%v", err)
	}
	hitsBefore, missesBefore := ts.announcementsCache.Stats()
	if _, err := ts.ListAnnouncements(ctx, 20); err != nil {
		t.Fatalf("创建后 ListAnnouncements 失败：%v", err)
	}
	hitsAfter, missesAfter := ts.announcementsCache.Stats()
	if hitsAfter != hitsBefore {
		t.Errorf("创建公告后应重新查库（hits 不增加）：before=%d after=%d", hitsBefore, hitsAfter)
	}
	if missesAfter <= missesBefore {
		t.Errorf("创建公告后应计未命中：before=%d after=%d", missesBefore, missesAfter)
	}

	// 更新公告 → 缓存失效
	created := listAllAnnouncements(t, ts)
	if len(created) == 0 {
		t.Fatal("应有公告可更新")
	}
	if _, err := ts.AdminUpdateAnnouncement(ctx, created[0].ID, "改标题", "改正文", true); err != nil {
		t.Fatalf("更新公告失败：%v", err)
	}
	hitsBefore, missesBefore = ts.announcementsCache.Stats()
	if _, err := ts.ListAnnouncements(ctx, 20); err != nil {
		t.Fatalf("更新后 ListAnnouncements 失败：%v", err)
	}
	hitsAfter, missesAfter = ts.announcementsCache.Stats()
	if hitsAfter != hitsBefore {
		t.Errorf("更新公告后应重新查库（hits 不增加）：before=%d after=%d", hitsBefore, hitsAfter)
	}
	if missesAfter <= missesBefore {
		t.Errorf("更新公告后应计未命中：before=%d after=%d", missesBefore, missesAfter)
	}

	// 删除公告 → 缓存失效
	if err := ts.AdminDeleteAnnouncement(ctx, created[0].ID); err != nil {
		t.Fatalf("删除公告失败：%v", err)
	}
	hitsBefore, missesBefore = ts.announcementsCache.Stats()
	if _, err := ts.ListAnnouncements(ctx, 20); err != nil {
		t.Fatalf("删除后 ListAnnouncements 失败：%v", err)
	}
	hitsAfter, missesAfter = ts.announcementsCache.Stats()
	if hitsAfter != hitsBefore {
		t.Errorf("删除公告后应重新查库（hits 不增加）：before=%d after=%d", hitsBefore, hitsAfter)
	}
	if missesAfter <= missesBefore {
		t.Errorf("删除公告后应计未命中：before=%d after=%d", missesBefore, missesAfter)
	}
}

// listAllAnnouncements 取全部已发布公告（按发布时间倒序）。
func listAllAnnouncements(t *testing.T, ts *testService) []AnnouncementView {
	t.Helper()
	items, err := ts.ListAnnouncements(context.Background(), 20)
	if err != nil {
		t.Fatalf("ListAnnouncements 失败：%v", err)
	}
	return items
}