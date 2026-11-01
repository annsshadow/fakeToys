package service

// 玩家账号信息（/me account）的服务层测试（2026-10-31 实质优化第 34 轮）。
//
// 关键行为锚点：
//   - MyAccount 返回注册时间与最后登录时间；
//   - 不存在的用户 → ErrNotFound；
//   - 库故障必须报错。

import (
	"context"
	"errors"
	"testing"
)

func TestMyAccountReturnsTimestamps(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	username := "acct_info"
	created, err := ts.EnsureBootstrapPlayer(ctx, username, "test-password-12345")
	if err != nil {
		t.Fatalf("引导账号失败：%v", err)
	}
	if !created {
		t.Fatal("首次引导应报告「新建」")
	}
	tp, err := ts.Login(ctx, username, "test-password-12345")
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}

	u, err := ts.MyAccount(ctx, tp.User.ID)
	if err != nil {
		t.Fatalf("MyAccount 失败：%v", err)
	}
	if u.CreatedAt.IsZero() {
		t.Error("注册时间不应为零值")
	}
	if u.LastLoginAt.IsZero() {
		t.Error("最后登录时间不应为零值")
	}
	// 最后登录应 >= 注册时间
	if u.LastLoginAt.Before(u.CreatedAt) {
		t.Error("最后登录不应早于注册时间")
	}
}

func TestMyAccountNotFound(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	_, err := ts.MyAccount(ctx, 999_999_999)
	if !errors.Is(err, ErrNotFound) {
		t.Fatalf("不存在的用户应返回 ErrNotFound，实际 %v", err)
	}
}

func TestMyAccountBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	_, err := ts.MyAccount(ctx, 1)
	if err == nil {
		t.Fatal("连接池已关闭时查询必须报错，不能静默返回零值")
	}
}
