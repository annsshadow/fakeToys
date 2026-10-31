package service

// 后台用户 CSV 导出的服务层测试（2026-10-30 实质优化第 12 轮）。
//
// 关键行为锚点：
//   - 输出是 UTF-8 BOM + CSV（Excel 中文兼容）；
//   - 表头与数据字段齐全；
//   - keyword 过滤与列表接口同口径（模糊搜昵称）；
//   - guest_token 等敏感凭证绝不进入导出。

import (
	"bytes"
	"context"
	"encoding/csv"
	"fmt"
	"strings"
	"testing"
	"time"
)

func TestAdminExportUsersCSV(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 一个账号 + 一个游客
	username := fmt.Sprintf("export_%d", time.Now().UnixNano())
	if _, err := ts.EnsureBootstrapPlayer(ctx, username, "test-password-12345"); err != nil {
		t.Fatalf("引导账号失败：%v", err)
	}
	guestID, _ := newGuestUser(t, ts, "export")

	data, err := ts.AdminExportUsers(ctx, "", nil, nil)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}

	// BOM 必须在最前（Excel 中文兼容）
	if !bytes.HasPrefix(data, []byte("\xEF\xBB\xBF")) {
		t.Error("CSV 必须以 UTF-8 BOM 开头")
	}

	// 解析 CSV 正文（去掉 BOM）
	body := bytes.TrimPrefix(data, []byte("\xEF\xBB\xBF"))
	reader := csv.NewReader(bytes.NewReader(body))
	records, err := reader.ReadAll()
	if err != nil {
		t.Fatalf("解析 CSV 失败：%v", err)
	}
	if len(records) < 3 {
		t.Fatalf("应有表头 + 至少 2 行数据，实际 %d 行", len(records))
	}

	// 表头
	wantHeader := []string{
		"id", "nickname", "is_guest", "status", "max_stage", "power",
		"coin", "gem", "created_at", "last_login_at", "verify_checked", "verify_mismatched",
	}
	if len(records[0]) != len(wantHeader) {
		t.Fatalf("表头列数 = %d，期望 %d：%v", len(records[0]), len(wantHeader), records[0])
	}
	for i, h := range wantHeader {
		if records[0][i] != h {
			t.Errorf("表头第 %d 列 = %q，期望 %q", i, records[0][i], h)
		}
	}

	// 数据行必须包含两个测试用户
	var foundGuest bool
	for _, rec := range records[1:] {
		if rec[0] == fmt.Sprintf("%d", guestID) {
			foundGuest = true
			if rec[2] != "true" {
				t.Errorf("游客行 is_guest 应为 true，实际 %q", rec[2])
			}
		}
		// 任何一行都不允许出现 guest_token 形状的字段
		if strings.Contains(strings.Join(rec, ","), "guest_export_") {
			t.Errorf("导出行泄露了 guest_token：%v", rec)
		}
	}
	if !foundGuest {
		t.Error("导出应包含测试游客号")
	}
}

func TestAdminExportUsersKeywordFilter(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	tag := fmt.Sprintf("filter_%d", time.Now().UnixNano())
	usernameA := tag + "_aaa"
	usernameB := tag + "_bbb"
	if _, err := ts.EnsureBootstrapPlayer(ctx, usernameA, "test-password-12345"); err != nil {
		t.Fatalf("引导账号 A 失败：%v", err)
	}
	if _, err := ts.EnsureBootstrapPlayer(ctx, usernameB, "test-password-12345"); err != nil {
		t.Fatalf("引导账号 B 失败：%v", err)
	}

	data, err := ts.AdminExportUsers(ctx, "_aaa", nil, nil)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	body := bytes.TrimPrefix(data, []byte("\xEF\xBB\xBF"))
	records, err := csv.NewReader(bytes.NewReader(body)).ReadAll()
	if err != nil {
		t.Fatalf("解析 CSV 失败：%v", err)
	}
	// 只有表头 + 匹配的 A
	if len(records) != 2 {
		t.Fatalf("keyword 过滤后应有 1 行数据，实际 %d 行", len(records)-1)
	}
	if !strings.Contains(records[1][1], "_aaa") {
		t.Errorf("匹配行应为账号 A，实际 %v", records[1])
	}
}

func TestAdminExportUsersBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, err := ts.AdminExportUsers(ctx, "", nil, nil); err == nil {
		t.Fatal("连接池已关闭时导出必须报错，不能静默返回空 CSV")
	}
}
