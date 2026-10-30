package service

// 后台重置玩家密码的服务层测试。
//
// 关键行为锚点：
//   - 重置后旧密码失效、新密码可登录，且**同一事务**内吊销全部 refresh token；
//   - 新密码门槛与其他账号一致（至少 10 位）；
//   - 游客号（无用户名密码）拒绝重置；
//   - 不存在的用户 → ErrNotFound；
//   - 库故障必须报错而不是静默成功。

import (
	"context"
	"errors"
	"fmt"
	"testing"
	"time"
)

func TestAdminResetPlayerPasswordSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	username := fmt.Sprintf("reset_pw_%d", time.Now().UnixNano())
	const oldPassword = "test-password-12345"
	created, err := ts.EnsureBootstrapPlayer(ctx, username, oldPassword)
	if err != nil {
		t.Fatalf("引导账号失败：%v", err)
	}
	if !created {
		t.Fatal("首次引导应报告「新建」")
	}
	tp, err := ts.Login(ctx, username, oldPassword)
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}
	id := tp.User.ID

	const newPassword = "admin-reset-password-99"
	if err := ts.AdminResetPlayerPassword(ctx, id, newPassword); err != nil {
		t.Fatalf("重置失败：%v", err)
	}

	// 旧密码失效
	if _, err := ts.Login(ctx, username, oldPassword); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("旧密码登录应失败（ErrUnauthorized），实际 %v", err)
	}
	// 新密码可登录回同一账号
	tp2, err := ts.Login(ctx, username, newPassword)
	if err != nil {
		t.Fatalf("新密码登录失败：%v", err)
	}
	if tp2.User.ID != id {
		t.Errorf("登录身份应为 %d，实际 %d", id, tp2.User.ID)
	}
	// 旧 refresh token 必须已吊销
	if _, err := ts.Refresh(ctx, tp.RefreshToken); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("重置后旧 refresh token 应失效（ErrUnauthorized），实际 %v", err)
	}
}

func TestAdminResetPlayerPasswordValidation(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	username := fmt.Sprintf("reset_val_%d", time.Now().UnixNano())
	if _, err := ts.EnsureBootstrapPlayer(ctx, username, "test-password-12345"); err != nil {
		t.Fatalf("引导账号失败：%v", err)
	}
	var id int64
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT id FROM users WHERE username = $1`, username).Scan(&id); err != nil {
		t.Fatalf("查询账号失败：%v", err)
	}

	for _, pw := range []string{"", "short", "   "} {
		if err := ts.AdminResetPlayerPassword(ctx, id, pw); !errors.Is(err, ErrBadInput) {
			t.Errorf("密码 %q 应返回 ErrBadInput，实际 %v", pw, err)
		}
	}
}

func TestAdminResetPlayerPasswordGuestRejected(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "resetguest")
	err := ts.AdminResetPlayerPassword(ctx, id, "admin-reset-password-99")
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("游客号重置应返回 ErrBadInput，实际 %v", err)
	}
}

func TestAdminResetPlayerPasswordUnknownUser(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	err := ts.AdminResetPlayerPassword(ctx, 999_999_999, "admin-reset-password-99")
	if !errors.Is(err, ErrNotFound) {
		t.Fatalf("不存在的用户应返回 ErrNotFound，实际 %v", err)
	}
}

func TestAdminResetPlayerPasswordBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	err := ts.AdminResetPlayerPassword(ctx, 1, "admin-reset-password-99")
	if err == nil {
		t.Fatal("连接池已关闭时重置必须报错，不能静默成功")
	}
}
