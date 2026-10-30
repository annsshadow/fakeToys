package service

// 后台兑换码编辑的服务层测试（2026-10-31 实质优化第 25 轮）。
//
// 关键行为锚点：
//   - 更新奖励/次数上限/过期时间落库；
//   - 空奖励 → ErrBadInput；
//   - 不存在的兑换码 → ErrNotFound；
//   - 库故障必须报错而不是静默成功。

import (
	"context"
	"errors"
	"fmt"
	"testing"
	"time"
)

func TestAdminUpdateRedeemCodeSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	created, err := ts.AdminCreateRedeemCode(ctx, fmt.Sprintf("EDIT_%d", time.Now().UnixNano()),
		map[string]int{"coin": 100}, 10, nil)
	if err != nil {
		t.Fatalf("创建兑换码失败：%v", err)
	}

	expiresAt := time.Now().Add(24 * time.Hour)
	updated, err := ts.AdminUpdateRedeemCode(ctx, created.ID,
		map[string]int{"gem": 50}, 5, &expiresAt)
	if err != nil {
		t.Fatalf("更新兑换码失败：%v", err)
	}
	if updated.Reward["gem"] != 50 || len(updated.Reward) != 1 {
		t.Errorf("奖励未更新：%+v", updated.Reward)
	}
	if updated.MaxUses != 5 {
		t.Errorf("次数上限 = %d，期望 5", updated.MaxUses)
	}
	if updated.ExpiresAt == nil {
		t.Error("过期时间应已更新")
	}
}

func TestAdminUpdateRedeemCodeValidation(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	created, err := ts.AdminCreateRedeemCode(ctx, fmt.Sprintf("EDITVAL_%d", time.Now().UnixNano()),
		map[string]int{"coin": 100}, 10, nil)
	if err != nil {
		t.Fatalf("创建兑换码失败：%v", err)
	}

	_, err = ts.AdminUpdateRedeemCode(ctx, created.ID, map[string]int{}, 5, nil)
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("空奖励应返回 ErrBadInput，实际 %v", err)
	}
}

func TestAdminUpdateRedeemCodeNotFound(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	_, err := ts.AdminUpdateRedeemCode(ctx, 999_999, map[string]int{"coin": 1}, 5, nil)
	if !errors.Is(err, ErrNotFound) {
		t.Fatalf("不存在的兑换码应返回 ErrNotFound，实际 %v", err)
	}
}

func TestAdminUpdateRedeemCodeBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	_, err := ts.AdminUpdateRedeemCode(ctx, 1, map[string]int{"coin": 1}, 5, nil)
	if err == nil {
		t.Fatal("连接池已关闭时更新必须报错，不能静默成功")
	}
}
