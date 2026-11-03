// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R74：审计合规 —— 「谁在什么时候停用了这个兑换码」必须能完整还原。
//
// # 改前缺口（探针实测）
//
// 对一条真实停用操作读审计行：
//
//   - 谁/什么时候：✓（admin_id + username + created_at）
//   - 对什么对象：△（target 只有数字 id "158"，码文本没记 —— 运营看到的是
//     一个无意义的数字，得自己再去 redeem_codes 表里 join）
//   - 改前改后值：✗（detail 只有 {"enabled": false} 新值；
//     update_redeem_code 同理只记新 reward/max_uses/expires_at，
//     「改前是什么」被永久覆盖）
//
// 以及查询侧：列表/导出只能按 action/操作人/时间过滤，**不能按对象过滤** ——
// 「这个兑换码被动过几次、谁动的」只能全表翻页。
//
// # 改后契约（本文件钉住）
//
//  1. set_redeem_enabled / update_redeem_code 的 detail 必含
//     `code`（码文本）与 `before`/`after` 快照 —— 每行审计自包含，
//     不依赖被操作对象还存在；
//  2. 列表与导出都接受 `target` 精确过滤，口径一致。

import (
	"context"
	"encoding/json"
	"strconv"
	"strings"
	"testing"
	"time"
)

// r74MakeRedeemCode 造一个测试兑换码，返回 id。
func r74MakeRedeemCode(t *testing.T, ts *testService, ctx context.Context, code string, maxUses int) int {
	t.Helper()
	var id int
	if err := ts.pool.QueryRow(ctx,
		`INSERT INTO redeem_codes (id, code, reward, max_uses, used_count, enabled)
		 VALUES ((SELECT COALESCE(MAX(id),0)+1 FROM redeem_codes), $1, $2, $3, 0, true)
		 RETURNING id`,
		code, []byte(`{"coin":100}`), maxUses).Scan(&id); err != nil {
		t.Fatalf("造兑换码失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(), `DELETE FROM redeem_usages WHERE code_id=$1`, id)
		_, _ = ts.pool.Exec(context.Background(), `DELETE FROM admin_audit_logs WHERE target=$1`, strconv.Itoa(id))
		_, _ = ts.pool.Exec(context.Background(), `DELETE FROM redeem_codes WHERE id=$1`, id)
	})
	return id
}

// r74LastAuditDetail 读指定 action+target 最新一条审计的 detail。
func r74LastAuditDetail(t *testing.T, ts *testService, ctx context.Context, action, target string) map[string]any {
	t.Helper()
	var raw []byte
	if err := ts.pool.QueryRow(ctx,
		`SELECT detail FROM admin_audit_logs WHERE action=$1 AND target=$2 ORDER BY id DESC LIMIT 1`,
		action, target).Scan(&raw); err != nil {
		t.Fatalf("读审计行失败（%s/%s）：%v", action, target, err)
	}
	var m map[string]any
	if err := json.Unmarshal(raw, &m); err != nil {
		t.Fatalf("审计 detail 不是合法 JSON：%v", err)
	}
	return m
}

// TestSetRedeemEnabledAuditHasBeforeAfter 停用操作的审计行必须自包含：
// 码文本 + 改前 enabled + 改后 enabled。
func TestSetRedeemEnabledAuditHasBeforeAfter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	adminID := newAdminUser(t, ts)

	code := "R74-TOGGLE"
	id := r74MakeRedeemCode(t, ts, ctx, code, 5)

	if _, err := ts.AdminSetRedeemCodeEnabled(ctx, id, false,
		AdminAuditCtx{AdminID: adminID, IP: "10.0.0.9"}); err != nil {
		t.Fatalf("停用失败：%v", err)
	}

	d := r74LastAuditDetail(t, ts, ctx, "set_redeem_enabled", strconv.Itoa(id))
	if d["code"] != code {
		t.Errorf("detail 缺码文本：code=%v（运营看到 target=%d 只是一串数字）", d["code"], id)
	}
	before, _ := d["before"].(map[string]any)
	after, _ := d["after"].(map[string]any)
	if before["enabled"] != true {
		t.Errorf("before.enabled=%v，期望 true（停用前是启用态）", before["enabled"])
	}
	if after["enabled"] != false {
		t.Errorf("after.enabled=%v，期望 false", after["enabled"])
	}
}

// TestUpdateRedeemCodeAuditHasBeforeAfter 编辑操作的审计必须记改前/改后两值，
// 否则「奖励被从 100 改成 99999」查不出改前是多少。
func TestUpdateRedeemCodeAuditHasBeforeAfter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	adminID := newAdminUser(t, ts)

	id := r74MakeRedeemCode(t, ts, ctx, "R74-EDIT", 5)

	newReward := map[string]int{"coin": 99999}
	if _, err := ts.AdminUpdateRedeemCode(ctx, id, newReward, 50, nil,
		AdminAuditCtx{AdminID: adminID, IP: "10.0.0.9"}); err != nil {
		t.Fatalf("编辑失败：%v", err)
	}

	d := r74LastAuditDetail(t, ts, ctx, "update_redeem_code", strconv.Itoa(id))
	before, _ := d["before"].(map[string]any)
	after, _ := d["after"].(map[string]any)
	if before == nil || after == nil {
		t.Fatalf("detail 缺 before/after 快照：%v", d)
	}
	// 改前值：max_uses=5、reward coin=100。
	if before["max_uses"] != float64(5) {
		t.Errorf("before.max_uses=%v，期望 5", before["max_uses"])
	}
	if br, _ := before["reward"].(map[string]any); br["coin"] != float64(100) {
		t.Errorf("before.reward=%v，期望 {coin:100}", before["reward"])
	}
	// 改后值：max_uses=50、reward coin=99999。
	if after["max_uses"] != float64(50) {
		t.Errorf("after.max_uses=%v，期望 50", after["max_uses"])
	}
	if ar, _ := after["reward"].(map[string]any); ar["coin"] != float64(99999) {
		t.Errorf("after.reward=%v，期望 {coin:99999}", after["reward"])
	}
}

// TestAdminListAuditLogsFilterByTarget 按对象过滤：「这个兑换码被动过几次、谁动的」
// 必须一次查出来，不能全表翻页。
func TestAdminListAuditLogsFilterByTarget(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	adminA := newAdminUser(t, ts)
	adminB := newAdminUser(t, ts)

	now := time.Now()
	// 同一对象（target=code#7）两条不同操作人；另一对象一条。
	seedAuditTarget(t, ts, adminA, "set_redeem_enabled", "code#7", now.Add(-3*time.Hour))
	seedAuditTarget(t, ts, adminB, "update_redeem_code", "code#7", now.Add(-2*time.Hour))
	seedAuditTarget(t, ts, adminA, "update_redeem_code", "code#9", now.Add(-1*time.Hour))

	items, total, err := ts.AdminListAuditLogs(ctx, 50, 0, "", 0, nil, nil, "code#7")
	if err != nil {
		t.Fatalf("target 过滤失败：%v", err)
	}
	if total != 2 || len(items) != 2 {
		t.Fatalf("target=code#7 应 2 条，实际 total=%d len=%d", total, len(items))
	}
	for _, it := range items {
		if it.Target != "code#7" {
			t.Errorf("返回了其它对象的日志：%+v", it)
		}
	}

	// 组合：target + action
	items, total, err = ts.AdminListAuditLogs(ctx, 50, 0, "update_redeem_code", 0, nil, nil, "code#7")
	if err != nil {
		t.Fatalf("组合过滤失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].Action != "update_redeem_code" {
		t.Errorf("target+action 应恰好 1 条，实际 total=%d", total)
	}

	// 空 target 不过滤（既有行为不回退）
	_, total, err = ts.AdminListAuditLogs(ctx, 50, 0, "", 0, nil, nil, "")
	if err != nil {
		t.Fatalf("空 target 失败：%v", err)
	}
	if total != 3 {
		t.Errorf("空 target 应返回全部 3 条，实际 %d", total)
	}
}

// TestAdminExportAuditLogsFilterByTarget 导出与列表同口径（归档筛选必须一致）。
func TestAdminExportAuditLogsFilterByTarget(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	adminA := newAdminUser(t, ts)
	seedAuditTarget(t, ts, adminA, "set_redeem_enabled", "code#7", time.Now())
	seedAuditTarget(t, ts, adminA, "update_redeem_code", "code#9", time.Now())

	data, err := ts.AdminExportAuditLogs(ctx, "", 0, nil, nil, "code#7")
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	// 表头 + 恰好 1 行数据
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 2 {
		t.Fatalf("target=code#7 导出应 1 行数据，实际 %d 行（含表头 %d）", len(lines)-1, len(lines))
	}
	if !strings.Contains(string(data), "code#7") || strings.Contains(string(data), "code#9") {
		t.Errorf("导出内容口径不对：%q", string(data))
	}
}

func seedAuditTarget(t *testing.T, ts *testService, adminID int64, action, target string, at time.Time) {
	t.Helper()
	if _, err := ts.db.Pool.Exec(context.Background(),
		`INSERT INTO admin_audit_logs (admin_id, username, action, target, detail, created_at)
		 VALUES ($1, 'admin', $2, $3, '{}'::jsonb, $4)`, adminID, action, target, at); err != nil {
		t.Fatalf("插入审计日志失败：%v", err)
	}
}
