// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
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
	"strings"
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

// TestRedeemHonorsConcurrentDisable 停用必须对「已在途」的兑换生效。
//
// 兑换流程是 SELECT（看 enabled/过期/余量）→ INSERT 使用记录 → 条件 UPDATE
// 占位。改前条件 UPDATE 只重判 used_count：运营在 SELECT 之后停用，在途
// 那笔仍会成功（停用等于漏一次）。这里用 AFTER INSERT 触发器在两条语句
// 之间把码停用，模拟该竞态；改后条件写重判 enabled → 整笔回滚。
func TestRedeemHonorsConcurrentDisable(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	code := "RACE_DISABLE"
	if _, err := ts.AdminCreateRedeemCode(ctx, code, map[string]int{"coin": 100}, 5, nil); err != nil {
		t.Fatalf("创建兑换码失败：%v", err)
	}
	uid, _ := newGuestUser(t, ts, "redeem_race")
	if err := ts.initNewUser(ctx, uid); err != nil {
		t.Fatalf("初始化新用户失败：%v", err)
	}
	var coinBefore int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT coin FROM user_wallets WHERE user_id = $1`, uid).Scan(&coinBefore); err != nil {
		t.Fatalf("读钱包失败：%v", err)
	}

	ts.exec(t, `CREATE OR REPLACE FUNCTION svc_disable_on_redeem_usage() RETURNS trigger
		LANGUAGE plpgsql AS $$
		BEGIN
			UPDATE redeem_codes SET enabled = FALSE WHERE id = NEW.code_id;
			RETURN NEW;
		END $$`)
	ts.exec(t, `CREATE TRIGGER svc_disable_on_redeem_usage AFTER INSERT ON redeem_usages
		FOR EACH ROW EXECUTE FUNCTION svc_disable_on_redeem_usage()`)

	_, err := ts.Redeem(ctx, uid, code)
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("在途停用后兑换应失败（ErrBadInput），实际 %v", err)
	}
	if !strings.Contains(err.Error(), "已停用") {
		t.Errorf("错误信息应说明已停用，实际 %q", err.Error())
	}

	// 整笔回滚：使用记录、used_count、钱包都不应变化（停用本身也在同一事务里回滚）。
	var usages, used int
	var coinAfter int64
	if err := ts.pool.QueryRow(ctx, `SELECT COUNT(*) FROM redeem_usages`).Scan(&usages); err != nil {
		t.Fatalf("统计使用记录失败：%v", err)
	}
	if err := ts.pool.QueryRow(ctx,
		`SELECT used_count FROM redeem_codes WHERE code = $1`, code).Scan(&used); err != nil {
		t.Fatalf("读 used_count 失败：%v", err)
	}
	if err := ts.pool.QueryRow(ctx,
		`SELECT coin FROM user_wallets WHERE user_id = $1`, uid).Scan(&coinAfter); err != nil {
		t.Fatalf("读钱包失败：%v", err)
	}
	if usages != 0 || used != 0 || coinAfter != coinBefore {
		t.Fatalf("失败必须整笔回滚，实际 usages=%d used=%d coin=%d(前 %d)",
			usages, used, coinAfter, coinBefore)
	}

	ts.exec(t, `DROP TRIGGER svc_disable_on_redeem_usage ON redeem_usages`)
	ts.exec(t, `DROP FUNCTION svc_disable_on_redeem_usage()`)
	if _, err := ts.Redeem(ctx, uid, code); err != nil {
		t.Fatalf("移除触发器后兑换应成功：%v", err)
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
