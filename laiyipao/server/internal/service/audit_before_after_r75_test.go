// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R75：审计 before/after 快照推广到关卡/技能/商城/公告/防线。
//
// 改前缺口（静态+实跑探针一致）：这些编辑操作的 detail 只记新值
// （patch 或新对象），「改前是什么」被永久覆盖 —— 与 R74 兑换码同型。
// 判定不改的（理由写进台账）：regenerate_levels（全量替换无差异语义，
// 100 关 before 是几十 KB）、ban/unban（reason 已记、状态隐含）、
// grant/revoke（账本轨迹在 wallet_flows）、create_*（新对象全量已记）、
// verify_replay（结果快照已全）、set_recompute_mode（已记 from/to）、
// reset_player_password（口令哈希绝不能进审计）、logout（会话类）。
//
// 体积纪律：patch 类 before 只抽被改字段；防线删除摘要不带 snapshot jsonb。

import (
	"context"
	"strconv"
	"strings"
	"testing"
)

// r75Detail 复用 R74 的读取器（同包）。
func r75Detail(t *testing.T, ts *testService, action, target string) map[string]any {
	t.Helper()
	return r74LastAuditDetail(t, ts, context.Background(), action, target)
}

// TestUpdateLevelAuditBeforeIsPatchKeyed：关卡编辑的 before 只含被改字段。
func TestUpdateLevelAuditBeforeIsPatchKeyed(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	adminID := newAdminUser(t, ts)

	var oldName string
	if err := ts.pool.QueryRow(ctx, `SELECT name FROM levels WHERE id = 1`).Scan(&oldName); err != nil {
		t.Fatalf("读关卡失败：%v", err)
	}
	if _, err := ts.AdminUpdateLevel(ctx, 1, map[string]any{"name": oldName + "-改"},
		AdminAuditCtx{AdminID: adminID, IP: "10.0.0.7"}); err != nil {
		t.Fatalf("改关卡失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(), `UPDATE levels SET name=$1 WHERE id=1`, oldName)
		_, _ = ts.pool.Exec(context.Background(), `DELETE FROM admin_audit_logs WHERE action='update_level' AND target='1'`)
	})

	d := r75Detail(t, ts, "update_level", "1")
	before, _ := d["before"].(map[string]any)
	if before["name"] != oldName {
		t.Errorf("before.name=%v，期望 %q", before["name"], oldName)
	}
	// 体积纪律：patch 只碰 name，before 不得夹带整关配置。
	if _, has := before["star_targets"]; has {
		t.Errorf("before 夹带了未变更的 star_targets（体积失控）：%v", before)
	}
	if _, has := before["terrain_config"]; has {
		t.Errorf("before 夹带了未变更的 terrain_config（体积失控）：%v", before)
	}
	patch, _ := d["patch"].(map[string]any)
	if patch["name"] != oldName+"-改" {
		t.Errorf("patch.name=%v，期望新值", patch["name"])
	}
}

// TestUpdateSkillAuditBefore：技能编辑 before 含旧 name/descr。
func TestUpdateSkillAuditBefore(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	adminID := newAdminUser(t, ts)

	var oldName string
	if err := ts.pool.QueryRow(ctx, `SELECT name FROM skills WHERE id = 1`).Scan(&oldName); err != nil {
		t.Fatalf("读技能失败：%v", err)
	}
	if _, err := ts.AdminUpdateSkill(ctx, 1, map[string]any{"name": oldName + "-改"},
		AdminAuditCtx{AdminID: adminID, IP: "10.0.0.7"}); err != nil {
		t.Fatalf("改技能失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(), `UPDATE skills SET name=$1 WHERE id=1`, oldName)
		_, _ = ts.pool.Exec(context.Background(), `DELETE FROM admin_audit_logs WHERE action='update_skill' AND target='1'`)
	})

	d := r75Detail(t, ts, "update_skill", "1")
	before, _ := d["before"].(map[string]any)
	if before["name"] != oldName {
		t.Errorf("before.name=%v，期望 %q", before["name"], oldName)
	}
}

// TestUpdateShopItemAuditBefore：商城编辑 before 含旧 limit_per_day。
func TestUpdateShopItemAuditBefore(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	adminID := newAdminUser(t, ts)

	var itemID int64
	var oldLimit int
	if err := ts.pool.QueryRow(ctx,
		`SELECT id, limit_per_day FROM shop_items WHERE enabled ORDER BY id LIMIT 1`).Scan(&itemID, &oldLimit); err != nil {
		t.Skipf("无商品：%v", err)
	}
	if _, err := ts.AdminUpdateShopItem(ctx, itemID, map[string]any{"limit_per_day": oldLimit + 9},
		AdminAuditCtx{AdminID: adminID, IP: "10.0.0.7"}); err != nil {
		t.Fatalf("改商品失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(), `UPDATE shop_items SET limit_per_day=$1 WHERE id=$2`, oldLimit, itemID)
		_, _ = ts.pool.Exec(context.Background(),
			`DELETE FROM admin_audit_logs WHERE action='update_shop_item' AND target=$1`, r75I64(itemID))
	})

	d := r75Detail(t, ts, "update_shop_item", r75I64(itemID))
	before, _ := d["before"].(map[string]any)
	if before["limit_per_day"] != float64(oldLimit) {
		t.Errorf("before.limit_per_day=%v，期望 %d", before["limit_per_day"], oldLimit)
	}
	if _, has := before["payload"]; has {
		t.Errorf("patch 没碰 payload，before 不该夹带：%v", before)
	}
}

// TestAnnouncementAuditBeforeAfter：公告更新/删除都记 before。
func TestAnnouncementAuditBeforeAfter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	adminID := newAdminUser(t, ts)
	actx := AdminAuditCtx{AdminID: adminID, IP: "10.0.0.7"}

	created, err := ts.AdminCreateAnnouncement(ctx, "R75原题", "R75原文", false, actx)
	if err != nil {
		t.Fatalf("建公告失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(), `DELETE FROM announcements WHERE id=$1`, created.ID)
		_, _ = ts.pool.Exec(context.Background(),
			`DELETE FROM admin_audit_logs WHERE target=$1 AND action IN ('update_announcement','delete_announcement','create_announcement')`,
			strconv.Itoa(created.ID))
	})

	if _, err := ts.AdminUpdateAnnouncement(ctx, created.ID, "R75新题", "R75新文", true, actx); err != nil {
		t.Fatalf("改公告失败：%v", err)
	}
	d := r75Detail(t, ts, "update_announcement", strconv.Itoa(created.ID))
	before, _ := d["before"].(map[string]any)
	after, _ := d["after"].(map[string]any)
	if before["title"] != "R75原题" || before["published"] != false {
		t.Errorf("before=%v，期望 {R75原题, published:false}", before)
	}
	if after["title"] != "R75新题" || after["published"] != true {
		t.Errorf("after=%v，期望 {R75新题, published:true}", after)
	}

	if err := ts.AdminDeleteAnnouncement(ctx, created.ID, actx); err != nil {
		t.Fatalf("删公告失败：%v", err)
	}
	d = r75Detail(t, ts, "delete_announcement", strconv.Itoa(created.ID))
	before, _ = d["before"].(map[string]any)
	if before["title"] != "R75新题" || before["body"] != "R75新文" {
		t.Errorf("删除行 before=%v，期望含最终标题与正文（删除不可逆，审计是唯一遗迹）", before)
	}
}

// TestDeleteDefenseAuditBefore：防线删除记摘要（不含大 snapshot jsonb）。
func TestDeleteDefenseAuditBefore(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	adminID := newAdminUser(t, ts)

	uid := ts.newUser(t, ctx)
	dv, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{
		Name: "R75待删防线", Works: []string{"slow_belt"}, Skills: []int{1, 4, 7},
	})
	if err != nil {
		t.Fatalf("建防线失败：%v", err)
	}
	if err := ts.AdminDeleteDefense(ctx, dv.ID, AdminAuditCtx{AdminID: adminID, IP: "10.0.0.7"}); err != nil {
		t.Fatalf("删防线失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(), `DELETE FROM admin_audit_logs WHERE action='delete_defense' AND target=$1`, r75I64(dv.ID))
	})

	d := r75Detail(t, ts, "delete_defense", r75I64(dv.ID))
	before, _ := d["before"].(map[string]any)
	if before["name"] != "R75待删防线" {
		t.Errorf("before.name=%v，期望防线名", before["name"])
	}
	if before["owner_id"] != float64(uid) {
		t.Errorf("before.owner_id=%v，期望 %d", before["owner_id"], uid)
	}
	if _, has := before["snapshot"]; has {
		t.Errorf("before 不该夹带大体积 snapshot jsonb（snapshot_hash 已能标识内容）")
	}
}

// TestAuditRowVolumeBounded：体积实测——最重的编辑（关卡 star_targets +
// terrain_config + name 一起改）产生的审计行也不得吹到几十 KB。
func TestAuditRowVolumeBounded(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	adminID := newAdminUser(t, ts)

	// 真实场景里最重的 patch：两块 jsonb + 名称。
	patch := map[string]any{
		"name":         "体积测量关卡",
		"star_targets": []any{float64(100), float64(200), float64(300)},
		"terrain_config": []any{
			map[string]any{"kind": "frost", "col": 1, "row": 2},
			map[string]any{"kind": "lava", "col": 3, "row": 4},
		},
	}
	if _, err := ts.AdminUpdateLevel(ctx, 1, patch, AdminAuditCtx{AdminID: adminID, IP: "10.0.0.7"}); err != nil {
		t.Fatalf("改关卡失败：%v", err)
	}
	var size int
	if err := ts.pool.QueryRow(ctx,
		`SELECT pg_column_size(admin_audit_logs.*) FROM admin_audit_logs
		 WHERE action='update_level' AND target='1' ORDER BY id DESC LIMIT 1`).Scan(&size); err != nil {
		t.Fatalf("量审计行失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(), `DELETE FROM admin_audit_logs WHERE action='update_level' AND target='1'`)
	})

	// R74 基线 184B。这里两块 jsonb 前后值都在，允许到 4KB；
	// 若破 4KB 说明有字段被全量夹带（体积失控）。
	if size > 4096 {
		t.Errorf("最重关卡编辑的审计行 = %dB，上限 4KB —— before 抽取可能夹带了整行", size)
	}
	t.Logf("最重关卡编辑的审计行体积 = %dB（R74 基线 184B）", size)
}

// TestUpdateLevelNotFoundIsFailLoud：改不存在的关卡必须 404，
// 不能是 UPDATE 零行的静默 200（R75 顺带钉住的语义变化）。
func TestUpdateLevelNotFoundIsFailLoud(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	adminID := newAdminUser(t, ts)

	// ⚠️ 种子有 100 关（id 1..100），99 是真实存在的 —— 第一版误用 99，
	// 结果「改名成功」而不是报错。用一个真不存在的 id 才测得到语义。
	_, err := ts.AdminUpdateLevel(ctx, 424242, map[string]any{"name": "不存在"},
		AdminAuditCtx{AdminID: adminID, IP: "10.0.0.7"})
	if err == nil || !strings.Contains(err.Error(), "不存在") {
		t.Fatalf("改不存在关卡应报「不存在」，实际 %v", err)
	}
}

func r75I64(id int64) string {
	return strconv.FormatInt(id, 10)
}
