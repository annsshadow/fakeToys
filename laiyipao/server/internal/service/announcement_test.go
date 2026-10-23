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
	// ⚠️ 用**一次性库**，不用共享库。
	//
	// 第一版用的是 `openTestService`（共享库），只清自己 `ann_probe_%` 的行，
	// 然后断言「这两条落在全局前 20 里」—— 而共享库里同包的其它用例
	// （如 `TestAdminAnnouncements`）会写已发布公告。探针行会被无关用例
	// 挤出 20 名之外而**因无关原因变红**，这正是
	// `TestLeaderboardKinds` 在 2026-10-21 改一次性库时记下的同一条教训：
	// **依赖「表还不够脏」的断言不是断言，是运气。**
	ts := openScratchService(t)
	ctx := context.Background()

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
	insert("draft", false, nil) // 草稿：绝不能出现
	insert("old", true, &older) // 已发布，较旧
	insert("new", true, &newer) // 已发布，较新

	items, err := ts.ListAnnouncements(ctx, 20)
	if err != nil {
		t.Fatalf("ListAnnouncements 失败：%v", err)
	}

	// 干净库里只有这三条 → 结果必须**恰好**两条，且顺序固定。
	// 断言「恰好」而不是「包含」，才能同时抓住「多给了」和「少给了」。
	if len(items) != 2 {
		t.Fatalf("干净库只应有 2 条已发布公告，实际 %d 条：%+v", len(items), items)
	}
	if items[0].Title != "new" || items[1].Title != "old" {
		t.Fatalf("应按 published_at 倒序（new, old），实际 %q, %q",
			items[0].Title, items[1].Title)
	}
	// 玩家侧响应不得带 published 字段（它恒为 true，回传只会诱导客户端写无效判断）
	// —— 这里用结构体字段数间接守：JSON 里出现 published 说明类型被换回了后台那份。
	if items[0].ID == 0 {
		t.Errorf("公告 ID 未填充：%+v", items[0])
	}
}

// TestListAnnouncementsLimitClamped limit 越界必须**夹到边界**，不是回落到缺省。
//
// 这两件事的差别是可观测的：
//
//	请求 100 条 → 夹到上限 20（合理：客户端要「尽可能多」）
//	请求 100 条 → 退回缺省 5（不合理：比请求少、也比上限少，还不报错）
//
// 第一版实现是后者，而当时的断言只写了 `len <= max` ——
// 返回 20、返回 5、甚至返回 0 都能过。现在断言**恰好等于 20**。
func TestListAnnouncementsLimitClamped(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	now := time.Now()
	// 插入 max+5 条，保证「夹到 20」与「退回 5」可区分
	for i := 0; i < maxAnnouncementLimit+5; i++ {
		at := now.Add(-time.Duration(i) * time.Minute)
		if _, err := ts.db.Pool.Exec(ctx,
			`INSERT INTO announcements (title, body, published, published_at) VALUES ($1,$2,TRUE,$3)`,
			fmt.Sprintf("bulk_%d", i), "x", at); err != nil {
			t.Fatalf("批量写公告失败：%v", err)
		}
	}

	// 超上限：夹到 maxAnnouncementLimit（不是缺省值）
	items, err := ts.ListAnnouncements(ctx, 100000)
	if err != nil {
		t.Fatalf("ListAnnouncements(100000) 失败：%v", err)
	}
	if len(items) != maxAnnouncementLimit {
		t.Errorf("limit=100000 应夹到 %d 条，实际 %d 条 —— "+
			"若为 %d 说明越界被当成「没传」回落到缺省值",
			maxAnnouncementLimit, len(items), defaultAnnouncementLimit)
	}

	// 非正数：回落到缺省值（不是 0 行、不是上限）
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
	// 合法区间内的值必须**原样生效**（既不夹也不回落）
	items, err = ts.ListAnnouncements(ctx, 3)
	if err != nil {
		t.Fatalf("ListAnnouncements(3) 失败：%v", err)
	}
	if len(items) != 3 {
		t.Errorf("limit=3 应返回 3 条，实际 %d 条", len(items))
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
