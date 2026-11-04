// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 后台写端点的「拒绝」「找不到」「坏数据」三类分支。
//
// 为什么这些值得单独测：它们的共同失败模式是**把拒绝说成成功**。
// 兑换码改上限低于已用次数、商店改了拼错的字段名、公告 id 打错一位
// —— 旧实现都可能返回 200，而运营会据此以为「已经改好了」。

import (
	"context"
	"encoding/json"
	"errors"
	"strconv"
	"strings"
	"testing"
)

func TestAdminUpdateRedeemCodeGuards(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	if _, err := ts.AdminUpdateRedeemCode(ctx, 1, nil, 1, nil); !errors.Is(err, ErrBadInput) {
		t.Fatalf("空奖励应返回 ErrBadInput，实际 %v", err)
	} else if !strings.Contains(err.Error(), "奖励不能为空") {
		t.Fatalf("错误应说明奖励为空，实际：%v", err)
	}

	if _, err := ts.AdminUpdateRedeemCode(ctx, 99999999, map[string]int{"coin": 1}, 1, nil); !errors.Is(err, ErrNotFound) {
		t.Fatalf("不存在的兑换码应返回 ErrNotFound，实际 %v", err)
	}

	// 造一张已用 3 次的码：把上限调到 2 必须被挡下（已发出的兑换收不回来）。
	var codeID int
	if err := ts.pool.QueryRow(ctx, `
		INSERT INTO redeem_codes (code, reward, max_uses, used_count)
		VALUES ('R96-CODE-GUARD', '{"coin":10}'::jsonb, 5, 3)
		ON CONFLICT (code) DO UPDATE SET reward = EXCLUDED.reward,
		     max_uses = EXCLUDED.max_uses, used_count = EXCLUDED.used_count
		RETURNING id`).Scan(&codeID); err != nil {
		t.Fatalf("造兑换码失败：%v", err)
	}
	_, err := ts.AdminUpdateRedeemCode(ctx, codeID, map[string]int{"coin": 10}, 2, nil)
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("上限低于已用次数应返回 ErrBadInput，实际 %v", err)
	}
	if !strings.Contains(err.Error(), "小于已使用次数") {
		t.Fatalf("错误应说明原因，实际：%v", err)
	}

	// maxUses<=0 收敛为 1（而不是写 0 让码永远换不了）。
	var clampID int
	if err := ts.pool.QueryRow(ctx, `
		INSERT INTO redeem_codes (code, reward, max_uses)
		VALUES ('R96-CODE-CLAMP', '{"coin":10}'::jsonb, 5)
		ON CONFLICT (code) DO UPDATE SET reward = EXCLUDED.reward,
		     max_uses = EXCLUDED.max_uses, used_count = 0
		RETURNING id`).Scan(&clampID); err != nil {
		t.Fatalf("造兑换码失败：%v", err)
	}
	v, err := ts.AdminUpdateRedeemCode(ctx, clampID, map[string]int{"coin": 10}, 0, nil)
	if err != nil {
		t.Fatalf("maxUses=0 应被收敛为 1，实际 %v", err)
	}
	if v.MaxUses != 1 {
		t.Fatalf("maxUses 应收敛为 1，实际 %d", v.MaxUses)
	}

	// 改前快照坏掉时不能继续写：审计与更新同事务。
	// 共享库：坏值必须在用例结束时还原，否则后面的列表/导出会读到这张毒码。
	t.Cleanup(func() {
		ts.exec(t, `UPDATE redeem_codes SET reward = '{"coin":10}'::jsonb WHERE id = $1`, codeID)
	})
	ts.exec(t, `UPDATE redeem_codes SET reward = '[]'::jsonb WHERE id = $1`, codeID)
	if _, err := ts.AdminUpdateRedeemCode(ctx, codeID, map[string]int{"coin": 10}, 1, nil); err == nil {
		t.Fatal("改前 reward 是数组时仍更新成功")
	} else if !strings.Contains(err.Error(), "unmarshal before reward") {
		t.Fatalf("错误应点明改前奖励无法解析，实际：%v", err)
	}
}

func TestAdminSetRedeemCodeEnabledGuards(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	if _, err := ts.AdminSetRedeemCodeEnabled(ctx, 99999999, false); !errors.Is(err, ErrNotFound) {
		t.Fatalf("不存在的兑换码应返回 ErrNotFound，实际 %v", err)
	}

	var codeID int
	if err := ts.pool.QueryRow(ctx, `
		INSERT INTO redeem_codes (code, reward, max_uses)
		VALUES ('R96-CODE-TOGGLE', '{"coin":10}'::jsonb, 5)
		ON CONFLICT (code) DO UPDATE SET reward = EXCLUDED.reward,
		     max_uses = EXCLUDED.max_uses, used_count = 0
		RETURNING id`).Scan(&codeID); err != nil {
		t.Fatalf("造兑换码失败：%v", err)
	}
	if v, err := ts.AdminSetRedeemCodeEnabled(ctx, codeID, false); err != nil {
		t.Fatalf("停用兑换码失败：%v", err)
	} else if v.Enabled {
		t.Fatal("停用后回执仍为启用")
	}

	// 回执要把 reward 解出来：坏 jsonb 必须响，而不是给一张 enabled 对的空卡。
	// 共享库：同样要有还原，见上一个用例的说明。
	t.Cleanup(func() {
		ts.exec(t, `UPDATE redeem_codes SET reward = '{"coin":10}'::jsonb WHERE id = $1`, codeID)
	})
	ts.exec(t, `UPDATE redeem_codes SET reward = '[]'::jsonb WHERE id = $1`, codeID)
	if _, err := ts.AdminSetRedeemCodeEnabled(ctx, codeID, true); err == nil {
		t.Fatal("reward 是数组时启用仍返回成功")
	}
}

func TestAdminAnnouncementWriteGuards(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	if _, err := ts.AdminUpdateAnnouncement(ctx, 1, "", "正文", false); !errors.Is(err, ErrBadInput) {
		t.Fatalf("空标题应返回 ErrBadInput，实际 %v", err)
	}
	if _, err := ts.AdminUpdateAnnouncement(ctx, 1, "标题", "", false); !errors.Is(err, ErrBadInput) {
		t.Fatalf("空正文应返回 ErrBadInput，实际 %v", err)
	}
	if _, err := ts.AdminUpdateAnnouncement(ctx, 99999999, "标题", "正文", false); !errors.Is(err, ErrNotFound) {
		t.Fatalf("不存在的公告应返回 ErrNotFound，实际 %v", err)
	}
	if err := ts.AdminDeleteAnnouncement(ctx, 99999999); !errors.Is(err, ErrNotFound) {
		t.Fatalf("删除不存在的公告应返回 ErrNotFound，实际 %v", err)
	}
}

func TestAdminUpdateShopItemRejectsBadFields(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	var itemID int64
	if err := ts.pool.QueryRow(ctx, `SELECT id FROM shop_items ORDER BY id LIMIT 1`).Scan(&itemID); err != nil {
		t.Fatalf("读商品 id 失败：%v", err)
	}

	decode := func(t *testing.T, raw string) map[string]any {
		t.Helper()
		var patch map[string]any
		if err := json.Unmarshal([]byte(raw), &patch); err != nil {
			t.Fatalf("测试输入不是合法 JSON：%v", err)
		}
		return patch
	}

	cases := []struct{ name, raw, want string }{
		{"名字为空", `{"name":""}`, "必须是非空字符串"},
		{"价格是标量", `{"price":"十块"}`, "不能是标量"},
		{"价格含未知货币", `{"price":{"gemx":1}}`, "未知货币"},
		{"限购为负", `{"limit_per_day":-1}`, "必须是非负整数"},
		{"enabled 不是布尔", `{"enabled":"yes"}`, "必须是布尔值"},
		{"未知键", `{"limit":5}`, "不在可更新字段白名单里"},
		{"没有字段", `{}`, "no updatable fields"},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			_, err := ts.AdminUpdateShopItem(ctx, itemID, decode(t, c.raw))
			if !errors.Is(err, ErrBadInput) {
				t.Fatalf("应返回 ErrBadInput，实际 %v", err)
			}
			if !strings.Contains(err.Error(), c.want) {
				t.Fatalf("错误应点明「%s」，实际：%v", c.want, err)
			}
		})
	}

	if _, err := ts.AdminUpdateShopItem(ctx, 99999999, map[string]any{"sort_order": 1}); !errors.Is(err, ErrNotFound) {
		t.Fatalf("不存在的商品应返回 ErrNotFound，实际 %v", err)
	}
}

// TestAdminSetUserStatusAuditsBothDirections 封禁是止损动作，解封同样是 ——
// 两个方向都必须留下「谁、什么时候、为什么」。解封没有理由，detail 必须是空，
// 而不是把上一条封禁理由复制过去。
func TestAdminSetUserStatusAuditsBothDirections(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	adminID := newAdminUser(t, ts)
	uid := ts.newUser(t, ctx)
	audit := AdminAuditCtx{AdminID: adminID, IP: "127.0.0.1"}

	lastAudit := func(t *testing.T) (string, []byte) {
		t.Helper()
		var action string
		var detail []byte
		if err := ts.pool.QueryRow(ctx,
			`SELECT action, detail FROM admin_audit_logs WHERE target = $1 ORDER BY id DESC LIMIT 1`,
			strconv.FormatInt(uid, 10)).Scan(&action, &detail); err != nil {
			t.Fatalf("读封禁审计失败：%v", err)
		}
		return action, detail
	}

	if _, err := ts.AdminSetUserStatus(ctx, uid, true, "R96封禁", audit); err != nil {
		t.Fatalf("封禁失败：%v", err)
	}
	action, detail := lastAudit(t)
	if action != "ban_user" {
		t.Fatalf("封禁审计 action=%q，期望 ban_user", action)
	}
	var m map[string]any
	if err := json.Unmarshal(detail, &m); err != nil {
		t.Fatalf("封禁审计 detail 不是合法 JSON：%v", err)
	}
	if m["reason"] != "R96封禁" {
		t.Fatalf("封禁审计 detail 缺理由，实际：%v", m)
	}

	if _, err := ts.AdminSetUserStatus(ctx, uid, false, "", audit); err != nil {
		t.Fatalf("解封失败：%v", err)
	}
	action, detail = lastAudit(t)
	if action != "unban_user" {
		t.Fatalf("解封审计 action=%q，期望 unban_user", action)
	}
	// 审计列是 jsonb：空 detail 落成 SQL NULL 或 JSON `null` 都算空。
	if len(detail) != 0 && string(detail) != "null" {
		t.Fatalf("解封审计 detail 应为空（没有理由），实际：%s", detail)
	}
}
