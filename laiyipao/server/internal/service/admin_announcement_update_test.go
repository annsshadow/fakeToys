package service

// 后台公告编辑/删除的服务层测试（2026-10-31 实质优化第 23 轮）。
//
// 关键行为锚点：
//   - 更新标题/正文/发布状态落库；
//   - 草稿首次发布设置 published_at；撤回（true→false）保留 published_at；
//   - 空标题/正文 → ErrBadInput；
//   - 更新/删除不存在的公告 → ErrNotFound；
//   - 库故障必须报错而不是静默成功。

import (
	"context"
	"errors"
	"testing"
)

func TestAdminUpdateAnnouncementSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	created, err := ts.AdminCreateAnnouncement(ctx, "旧标题", "旧正文", false)
	if err != nil {
		t.Fatalf("创建公告失败：%v", err)
	}

	updated, err := ts.AdminUpdateAnnouncement(ctx, created.ID, "新标题", "新正文", true)
	if err != nil {
		t.Fatalf("更新公告失败：%v", err)
	}
	if updated.Title != "新标题" || updated.Body != "新正文" {
		t.Errorf("更新结果不正确：%+v", updated)
	}
	if !updated.Published {
		t.Error("发布状态应为 true")
	}
	if updated.PublishedAt == nil {
		t.Error("草稿首次发布应设置 published_at")
	}
}

func TestAdminUpdateAnnouncementUnpublishKeepsTimestamp(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	created, err := ts.AdminCreateAnnouncement(ctx, "标题", "正文", true)
	if err != nil {
		t.Fatalf("创建公告失败：%v", err)
	}

	updated, err := ts.AdminUpdateAnnouncement(ctx, created.ID, "标题", "正文", false)
	if err != nil {
		t.Fatalf("撤回公告失败：%v", err)
	}
	if updated.Published {
		t.Error("撤回后发布状态应为 false")
	}
	// 撤回保留发布时间（历史事实不改写）
	if updated.PublishedAt == nil {
		t.Error("撤回不应清空 published_at")
	}
}

func TestAdminUpdateAnnouncementValidation(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	created, err := ts.AdminCreateAnnouncement(ctx, "标题", "正文", false)
	if err != nil {
		t.Fatalf("创建公告失败：%v", err)
	}

	for _, c := range []struct {
		name      string
		title, ok string
	}{
		{"空标题", "", "正文"},
		{"空正文", "标题", ""},
	} {
		t.Run(c.name, func(t *testing.T) {
			if _, err := ts.AdminUpdateAnnouncement(ctx, created.ID, c.title, c.ok, false); !errors.Is(err, ErrBadInput) {
				t.Errorf("应返回 ErrBadInput，实际 %v", err)
			}
		})
	}
}

func TestAdminUpdateAnnouncementNotFound(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	_, err := ts.AdminUpdateAnnouncement(ctx, 999_999, "标题", "正文", false)
	if !errors.Is(err, ErrNotFound) {
		t.Fatalf("更新不存在的公告应返回 ErrNotFound，实际 %v", err)
	}
}

func TestAdminDeleteAnnouncementSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	created, err := ts.AdminCreateAnnouncement(ctx, "待删", "正文", false)
	if err != nil {
		t.Fatalf("创建公告失败：%v", err)
	}
	if err := ts.AdminDeleteAnnouncement(ctx, created.ID); err != nil {
		t.Fatalf("删除公告失败：%v", err)
	}

	// 库中已无记录
	items, _, err := ts.AdminListAnnouncements(ctx, 50, 0, nil, nil, nil)
	if err != nil {
		t.Fatalf("查询公告失败：%v", err)
	}
	for _, a := range items {
		if a.ID == created.ID {
			t.Fatal("删除后公告仍存在")
		}
	}
}

func TestAdminDeleteAnnouncementNotFound(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	err := ts.AdminDeleteAnnouncement(ctx, 999_999)
	if !errors.Is(err, ErrNotFound) {
		t.Fatalf("删除不存在的公告应返回 ErrNotFound，实际 %v", err)
	}
}

func TestAdminUpdateDeleteBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, err := ts.AdminUpdateAnnouncement(ctx, 1, "标题", "正文", false); err == nil {
		t.Error("连接池已关闭时更新必须报错")
	}
	if err := ts.AdminDeleteAnnouncement(ctx, 1); err == nil {
		t.Error("连接池已关闭时删除必须报错")
	}
}
