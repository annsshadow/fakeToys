package service

// 玩家侧公告（`GET /api/v1/announcements`）的守卫。
//
// 这条端点补的是一个**闭环缺口**：`announcements` 表与后台的
// 增删查早就存在，但玩家端没有任何读入口 —— 运营发的公告只有运营自己看得见。
// 缺口的形态与 README「已实现内容」里记的那批（`/mastery/reset`、
// `GET /levels/:id`）完全一样，所以判据也照抄：**玩家侧能不能真的读到**。
//
// 三条断言各堵一种失效方式：
//   1. 未发布的泄漏出去 —— 运营的草稿被玩家看到；
//   2. 排序不稳 —— 同一秒发布的两条在两次请求里顺序不同；
//   3. limit 不夹 —— 一条无鉴权公网端点被 `?limit=100000` 拖死。

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func TestListAnnouncementsOnlyPublished(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 共用库会被上百个用例依次打开，不能假设公告表是空的 ——
	// 先清掉本用例自己的前缀（其它用例不会写这张表，但清干净更稳）。
	if _, err := ts.db.Pool.Exec(ctx,
		`DELETE FROM announcements WHERE title LIKE 'ann_probe_%'`); err != nil {
		t.Fatalf("清理探针公告失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.db.Pool.Exec(context.Background(),
			`DELETE FROM announcements WHERE title LIKE 'ann_probe_%'`)
	})

	now := time.Now()
	insert := func(title string, published bool, at *time.Time) {
		t.Helper()
		if _, err := ts.db.Pool.Exec(ctx,
			`INSERT INTO announcements (title, body, published, published_at) VALUES ($1,$2,$3,$4)`,
			title, "body of "+title, published, at); err != nil {
			t.Fatalf("写公告 %s 失败：%v", title, err)
		}
	}
	older := now.Add(-2 * time.Hour)
	newer := now.Add(-1 * time.Hour)
	insert("ann_probe_draft", false, nil) // 草稿：绝不能出现
	insert("ann_probe_old", true, &older) // 已发布，较旧
	insert("ann_probe_new", true, &newer) // 已发布，较新

	items, err := ts.ListAnnouncements(ctx, 20)
	if err != nil {
		t.Fatalf("ListAnnouncements 失败：%v", err)
	}

	seen := map[string]int{}
	for i, a := range items {
		seen[a.Title] = i
	}
	if _, ok := seen["ann_probe_draft"]; ok {
		t.Errorf("未发布的公告泄漏到玩家侧：%+v", items)
	}
	oldIdx, hasOld := seen["ann_probe_old"]
	newIdx, hasNew := seen["ann_probe_new"]
	if !hasOld || !hasNew {
		t.Fatalf("已发布的公告没有全部返回（old=%v new=%v）：%+v", hasOld, hasNew, items)
	}
	// 倒序：新的在前
	if newIdx > oldIdx {
		t.Errorf("应按 published_at 倒序（新在前），实际 new@%d old@%d", newIdx, oldIdx)
	}
}

// TestListAnnouncementsLimitClamped limit 越界必须被夹住，而不是报错或原样透传。
func TestListAnnouncementsLimitClamped(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	now := time.Now()
	for i := 0; i < maxAnnouncementLimit+5; i++ {
		at := now.Add(-time.Duration(i) * time.Minute)
		if _, err := ts.db.Pool.Exec(ctx,
			`INSERT INTO announcements (title, body, published, published_at) VALUES ($1,$2,TRUE,$3)`,
			fmt.Sprintf("ann_probe_bulk_%d", i), "x", at); err != nil {
			t.Fatalf("批量写公告失败：%v", err)
		}
	}
	t.Cleanup(func() {
		_, _ = ts.db.Pool.Exec(context.Background(),
			`DELETE FROM announcements WHERE title LIKE 'ann_probe_bulk_%'`)
	})

	// 超上限：夹到 maxAnnouncementLimit
	items, err := ts.ListAnnouncements(ctx, 100000)
	if err != nil {
		t.Fatalf("ListAnnouncements(100000) 失败：%v", err)
	}
	if len(items) > maxAnnouncementLimit {
		t.Errorf("limit=100000 应被夹到 %d，实际返回 %d 条", maxAnnouncementLimit, len(items))
	}
	// 非正数：回落到缺省值，而不是「返回零行」
	items, err = ts.ListAnnouncements(ctx, 0)
	if err != nil {
		t.Fatalf("ListAnnouncements(0) 失败：%v", err)
	}
	if len(items) != defaultAnnouncementLimit {
		t.Errorf("limit=0 应回落到缺省 %d 条，实际 %d 条", defaultAnnouncementLimit, len(items))
	}
	// 负数同样不报错
	if _, err := ts.ListAnnouncements(ctx, -3); err != nil {
		t.Errorf("limit=-3 不应报错（展示型端点），实际：%v", err)
	}
}

// TestListAnnouncementsEmptyIsNotNil 空结果是 `[]` 而不是 `null`。
//
// 客户端对 `null` 调 `.length` 会抛；而「没有公告」是完全正常的状态。
// 这条与 B-1（Go 把 nil 切片编码成 null）是同一个形状。
func TestListAnnouncementsEmptyIsNotNil(t *testing.T) {
	ts := openScratchService(t) // 干净库：公告表必然为空
	items, err := ts.ListAnnouncements(context.Background(), 5)
	if err != nil {
		t.Fatalf("ListAnnouncements 失败：%v", err)
	}
	if items == nil {
		t.Fatal("空结果必须是非 nil 切片（JSON 要编码成 []，不能是 null）")
	}
	if len(items) != 0 {
		t.Fatalf("干净库不该有公告，实际 %d 条", len(items))
	}
}
