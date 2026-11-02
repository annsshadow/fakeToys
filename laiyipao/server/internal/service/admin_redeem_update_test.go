// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
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
	"strings"
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

// TestAdminUpdateRedeemCodeRejectsMaxUsesBelowUsed 新上限不得低于已使用次数。
//
// 表上有 `chk_redeem_uses CHECK (used_count <= max_uses)`。改前把上限调低
// 会直接撞这个约束（23514），经 %w 冒到 HTTP 层是 500，错误里只有约束名；
// 运营看到的是 Internal，而不是「已经兑出去 2 次，不能把上限改成 1」。
func TestAdminUpdateRedeemCodeRejectsMaxUsesBelowUsed(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	created, err := ts.AdminCreateRedeemCode(ctx, "CAP_BELOW_USED",
		map[string]int{"coin": 100}, 3, nil)
	if err != nil {
		t.Fatalf("创建兑换码失败：%v", err)
	}
	if _, err := ts.pool.Exec(ctx,
		`UPDATE redeem_codes SET used_count = 2 WHERE id = $1`, created.ID); err != nil {
		t.Fatalf("预置已用次数失败：%v", err)
	}

	_, err = ts.AdminUpdateRedeemCode(ctx, created.ID, map[string]int{"coin": 100}, 1, nil)
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("上限 1 < 已用 2 应返回 ErrBadInput（走 400），实际 %v", err)
	}
	if !strings.Contains(err.Error(), "已使用次数 2") {
		t.Errorf("错误信息应说明已用次数，实际 %q", err.Error())
	}

	// 失败不得改动任何一列（旧实现是撞约束回滚，新实现是谓词零行）。
	var maxUses, used int
	if err := ts.pool.QueryRow(ctx,
		`SELECT max_uses, used_count FROM redeem_codes WHERE id = $1`, created.ID).Scan(&maxUses, &used); err != nil {
		t.Fatalf("复查兑换码失败：%v", err)
	}
	if maxUses != 3 || used != 2 {
		t.Errorf("失败后不应改动数据，实际 max_uses=%d used_count=%d", maxUses, used)
	}

	// 合法上调（3 → 5）仍要成功。
	if _, err := ts.AdminUpdateRedeemCode(ctx, created.ID, map[string]int{"coin": 100}, 5, nil); err != nil {
		t.Fatalf("上限 5 >= 已用 2 应成功：%v", err)
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
