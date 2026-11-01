package service

// 后台兑换码启用/禁用的服务层测试（2026-10-31 实质优化第 24 轮）。
//
// 关键行为锚点：
//   - 切换 enabled 落库；
//   - 禁用后玩家 `Redeem` 立即失败（ErrBadInput）；
//   - 重新启用后玩家可兑换；
//   - 不存在的兑换码 → ErrNotFound；
//   - 库故障必须报错而不是静默成功。

import (
	"context"
	"errors"
	"fmt"
	"testing"
	"time"
)

func TestAdminSetRedeemCodeEnabled(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	code := fmt.Sprintf("ENABLE_%d", time.Now().UnixNano())
	created, err := ts.AdminCreateRedeemCode(ctx, code, map[string]int{"coin": 100}, 10, nil)
	if err != nil {
		t.Fatalf("创建兑换码失败：%v", err)
	}
	if !created.Enabled {
		t.Fatal("新兑换码默认应启用")
	}

	// 禁用
	disabled, err := ts.AdminSetRedeemCodeEnabled(ctx, created.ID, false)
	if err != nil {
		t.Fatalf("禁用兑换码失败：%v", err)
	}
	if disabled.Enabled {
		t.Error("禁用后 Enabled 应为 false")
	}

	// 禁用后玩家兑换失败
	uid, _ := newGuestUser(t, ts, "redeem_disabled")
	// 新游客号没有钱包行，兑换发币会报「coin 不足」——先走新手包初始化。
	if err := ts.initNewUser(ctx, uid); err != nil {
		t.Fatalf("初始化新用户失败：%v", err)
	}
	if _, err := ts.Redeem(ctx, uid, code); !errors.Is(err, ErrBadInput) {
		t.Errorf("禁用后兑换应失败（ErrBadInput），实际 %v", err)
	}

	// 重新启用
	enabled, err := ts.AdminSetRedeemCodeEnabled(ctx, created.ID, true)
	if err != nil {
		t.Fatalf("重新启用失败：%v", err)
	}
	if !enabled.Enabled {
		t.Error("重新启用后 Enabled 应为 true")
	}
	if _, err := ts.Redeem(ctx, uid, code); err != nil {
		t.Errorf("重新启用后兑换应成功，实际 %v", err)
	}
}

func TestAdminSetRedeemCodeEnabledNotFound(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	_, err := ts.AdminSetRedeemCodeEnabled(ctx, 999_999, false)
	if !errors.Is(err, ErrNotFound) {
		t.Fatalf("不存在的兑换码应返回 ErrNotFound，实际 %v", err)
	}
}

func TestAdminSetRedeemCodeEnabledBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	_, err := ts.AdminSetRedeemCodeEnabled(ctx, 1, false)
	if err == nil {
		t.Fatal("连接池已关闭时切换必须报错，不能静默成功")
	}
}
