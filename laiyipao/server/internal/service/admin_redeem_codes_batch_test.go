// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 后台兑换码批量生成的服务层测试（2026-11-01 新一轮第 57 轮）。
//
// 关键行为锚点：
//   - 一次生成 count 个不重复的随机码；
//   - 奖励/次数上限/过期时间与单个创建一致；
//   - count 越界（<=0 或 >100）报 ErrBadInput；
//   - 库故障必须报错而不是静默返回空列表。

import (
	"context"
	"testing"
	"time"
)

func TestAdminCreateRedeemCodesBatch(t *testing.T) {
	ts := openScratchService(t) // 一次性库，避免共享库数据干扰
	ctx := context.Background()

	expiresAt := time.Now().Add(24 * time.Hour).UTC().Truncate(time.Second)
	items, err := ts.AdminCreateRedeemCodes(ctx, 10, map[string]int{"coin": 100, "gem": 5}, 3, &expiresAt)
	if err != nil {
		t.Fatalf("批量生成失败：%v", err)
	}
	if len(items) != 10 {
		t.Fatalf("应生成 10 个码，实际 %d", len(items))
	}
	// 码不重复且格式正确（12 位，字符集无 0/O/1/I）
	seen := map[string]bool{}
	for _, it := range items {
		if seen[it.Code] {
			t.Errorf("出现重复码：%s", it.Code)
		}
		seen[it.Code] = true
		if len(it.Code) != 12 {
			t.Errorf("码长度应为 12，实际 %q（%d）", it.Code, len(it.Code))
		}
		for _, ch := range it.Code {
			if ch == '0' || ch == 'O' || ch == '1' || ch == 'I' {
				t.Errorf("码包含易混淆字符 %q：%s", string(ch), it.Code)
			}
		}
		if it.Reward["coin"] != 100 || it.Reward["gem"] != 5 {
			t.Errorf("奖励不正确：%+v", it.Reward)
		}
		if it.MaxUses != 3 {
			t.Errorf("次数上限应为 3，实际 %d", it.MaxUses)
		}
		if it.ExpiresAt == nil || !it.ExpiresAt.Equal(expiresAt) {
			t.Errorf("过期时间不正确：%v", it.ExpiresAt)
		}
		if !it.Enabled {
			t.Error("新生成的码应默认启用（Enabled=true）")
		}
	}
}

func TestAdminCreateRedeemCodesBatchInvalidCount(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	if _, err := ts.AdminCreateRedeemCodes(ctx, 0, map[string]int{"coin": 1}, 1, nil); err == nil {
		t.Error("count=0 应报错")
	}
	if _, err := ts.AdminCreateRedeemCodes(ctx, 101, map[string]int{"coin": 1}, 1, nil); err == nil {
		t.Error("count=101 应报错")
	}
}

// TestAdminCreateRedeemCodesBatchIsAtomic 批量中途失败必须整批回滚。
//
// 改前逐条 autocommit：第 5 条 INSERT 失败时接口报错，但前 4 个码已提交 ——
// 运营拿不到清单（不知道哪些码生效），重试又生成新码，旧码仍可兑换。
func TestAdminCreateRedeemCodesBatchIsAtomic(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 用触发器在第 5 条 INSERT 注入失败，模拟批量中途库故障。
	ts.exec(t, `CREATE TABLE svc_redeem_insert_counter (n int NOT NULL)`)
	ts.exec(t, `INSERT INTO svc_redeem_insert_counter VALUES (0)`)
	ts.exec(t, `CREATE OR REPLACE FUNCTION svc_fail_redeem_insert() RETURNS trigger
		LANGUAGE plpgsql AS $$
		BEGIN
			UPDATE svc_redeem_insert_counter SET n = n + 1;
			IF (SELECT n FROM svc_redeem_insert_counter) >= 5 THEN
				RAISE EXCEPTION 'injected redeem insert failure';
			END IF;
			RETURN NEW;
		END $$`)
	ts.exec(t, `CREATE TRIGGER svc_fail_redeem_insert BEFORE INSERT ON redeem_codes
		FOR EACH ROW EXECUTE FUNCTION svc_fail_redeem_insert()`)

	if _, err := ts.AdminCreateRedeemCodes(ctx, 10, map[string]int{"coin": 7}, 1, nil); err == nil {
		t.Fatal("触发器注入失败后批量生成必须报错")
	}
	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM redeem_codes WHERE reward->>'coin' = '7'`).Scan(&n); err != nil {
		t.Fatalf("统计兑换码失败：%v", err)
	}
	if n != 0 {
		t.Fatalf("批量中途失败必须整批回滚，实际残留 %d 个码（改前会留 4 个）", n)
	}

	ts.exec(t, `DROP TRIGGER svc_fail_redeem_insert ON redeem_codes`)
	ts.exec(t, `DROP FUNCTION svc_fail_redeem_insert()`)
	items, err := ts.AdminCreateRedeemCodes(ctx, 10, map[string]int{"coin": 7}, 1, nil)
	if err != nil || len(items) != 10 {
		t.Fatalf("移除触发器后应成功生成 10 个，实际 %d（err=%v）", len(items), err)
	}
}

func TestAdminCreateRedeemCodesBatchBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, err := ts.AdminCreateRedeemCodes(ctx, 3, map[string]int{"coin": 1}, 1, nil); err == nil {
		t.Fatal("连接池已关闭时批量生成必须报错")
	}
}
