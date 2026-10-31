package service

// 后台用户详情防线字段的测试（2026-10-31 新一轮第 31 轮）。

import (
	"context"
	"testing"
)

func TestAdminUserDetailIncludesDefense(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx) // GuestLogin 会初始化钱包/进度
	seedDefense(t, ts, uid, "我的铁壁")

	detail, err := ts.AdminUserDetail(ctx, uid)
	if err != nil {
		t.Fatalf("查询用户详情失败：%v", err)
	}
	if detail.Defense == nil {
		t.Fatal("有防线的用户详情应包含 defense")
	}
	if detail.Defense.Name != "我的铁壁" {
		t.Errorf("防线名 = %q，期望 我的铁壁", detail.Defense.Name)
	}
	if detail.Defense.OwnerID != uid {
		t.Errorf("owner_id = %d，期望 %d", detail.Defense.OwnerID, uid)
	}
	if detail.Defense.Power != 1000 {
		t.Errorf("战力 = %d，期望 1000（seedDefense 默认）", detail.Defense.Power)
	}
}

func TestAdminUserDetailDefenseNullWhenNone(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx) // GuestLogin 会初始化钱包/进度
	detail, err := ts.AdminUserDetail(ctx, uid)
	if err != nil {
		t.Fatalf("查询用户详情失败：%v", err)
	}
	if detail.Defense != nil {
		t.Errorf("未设置防线的用户 defense 应为 nil，实际 %+v", detail.Defense)
	}
}