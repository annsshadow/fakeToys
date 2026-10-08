package service

// 第 125 轮：审计记录不得「以省略撒谎」。
//
// Service.Audit 的 detail 序列化失败时，旧实现静默把 detail 写成 `{}` ——
// 那行审计日志与「这个操作本来就没有明细」**逐字节相同**，
// 运营后读审计轨迹（ContentView 审计 tab）时被误导：
// 证据没了，且没有任何痕迹说明它曾经存在。
//
// 现在失败路径必须诚实：detail 写成 `{"detail_error":"json marshal failed: ..."}`，
// 行本身即证据。本文件把「失败路径的落库形状」钉死。

import (
	"context"
	"encoding/json"
	"strings"
	"testing"
)

// TestAuditDetailMarshalFailure 序列化必然失败的 detail（chan）
// 必须落到「带 detail_error 标记」的合法 JSONB，而不是裸 `{}`。
func TestAuditDetailMarshalFailure(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	adminID, _, _ := ts.newAdminFixture(t, "audithonesty", 1)

	// chan 无法被 json.Marshal —— 必进失败分支，不依赖机型
	chanDetail := make(chan int)
	ts.Audit(ctx, adminID, "audit_honesty_act", "tgt-1", chanDetail)

	var detail, action, target, username string
	if err := ts.pool.QueryRow(ctx,
		`SELECT detail::text, action, target, username
		 FROM admin_audit_logs
		 WHERE admin_id = $1 AND action = 'audit_honesty_act' AND target = 'tgt-1'`,
		adminID).Scan(&detail, &action, &target, &username); err != nil {
		t.Fatalf("审计行未落库：%v", err)
	}
	// 主字段一个不能少——缺字段同样是撒谎
	if action != "audit_honesty_act" || target != "tgt-1" {
		t.Fatalf("主字段错：%q / %q", action, target)
	}
	if username == "" {
		t.Fatalf("username 丢失：审计行无法定位操作人")
	}

	// detail 必须是合法 JSONB 且带 detail_error 标记
	var m map[string]any
	if err := json.Unmarshal([]byte(detail), &m); err != nil {
		t.Fatalf("detail 不是合法 JSON：%v", err)
	}
	if _, ok := m["detail_error"]; !ok {
		t.Fatalf("detail 缺 detail_error 标记（旧实现会静默写 %s）：%s", "{}", detail)
	}
	derr, _ := m["detail_error"].(string)
	if !strings.Contains(derr, "marshal") {
		t.Fatalf("detail_error 未记丢因：%s", derr)
	}
}

// TestAuditDetailNilPreserved detail=nil 是**正常语义**
// （handlers 里 unban_user 就传 nil），必须保持 JSONB null，
// 不得被失败路径的标记污染。
func TestAuditDetailNilPreserved(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	adminID, _, _ := ts.newAdminFixture(t, "auditnil", 1)

	ts.Audit(ctx, adminID, "audit_nil_act", "tgt-0", nil)

	var detailText string
	if err := ts.pool.QueryRow(ctx,
		`SELECT detail::text FROM admin_audit_logs
		 WHERE admin_id = $1 AND action = 'audit_nil_act'`, adminID).
		Scan(&detailText); err != nil {
		t.Fatalf("审计行未落库：%v", err)
	}
	if detailText != "null" {
		t.Fatalf("nil detail 应保持 JSONB null，实际 %q", detailText)
	}
}

// TestAuditDetailNormalRoundTrip 普通 map 明细原样往返，
// 失败路径的改动不得影响正常路径。
func TestAuditDetailNormalRoundTrip(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	adminID, _, _ := ts.newAdminFixture(t, "auditok", 1)

	ts.Audit(ctx, adminID, "audit_ok_act", "tgt-2", map[string]any{"k": "v"})

	// ⚠️ 不用 `detail::text` 比对：Postgres JSONB 输出会**规范化**空白
	// （`{"k":"v"}` → `{"k": "v"}`），只能按 JSONB 取值断。
	var k string
	var typ string
	if err := ts.pool.QueryRow(ctx,
		`SELECT detail->>'k', jsonb_typeof(detail)
		 FROM admin_audit_logs WHERE admin_id = $1 AND action = 'audit_ok_act'`,
		adminID).Scan(&k, &typ); err != nil {
		t.Fatalf("审计行未落库：%v", err)
	}
	if k != "v" || typ != "object" {
		t.Fatalf("正常明细被改写：%q (type=%s)", k, typ)
	}
}
