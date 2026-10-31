package service

// 后台公告 CSV 导出的服务层测试（2026-10-31 实质优化第 38 轮）。
//
// 关键行为锚点：
//   - 输出是 UTF-8 BOM + CSV；
//   - 表头 6 列（id/title/body/published/published_at/created_at）；
//   - 库故障必须报错而不是静默返回空 CSV。

import (
	"bytes"
	"context"
	"encoding/csv"
	"testing"
)

func TestAdminExportAnnouncementsCSV(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	if _, err := ts.AdminCreateAnnouncement(ctx, "标题", "正文", true); err != nil {
		t.Fatalf("创建公告失败：%v", err)
	}

	data, err := ts.AdminExportAnnouncements(ctx)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	if !bytes.HasPrefix(data, []byte("\xEF\xBB\xBF")) {
		t.Error("CSV 必须以 UTF-8 BOM 开头")
	}

	records, err := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(data, []byte("\xEF\xBB\xBF")))).ReadAll()
	if err != nil {
		t.Fatalf("解析 CSV 失败：%v", err)
	}
	if len(records) < 2 {
		t.Fatalf("应有表头 + 数据，实际 %d 行", len(records))
	}
	wantHeader := []string{"id", "title", "body", "published", "published_at", "created_at"}
	if len(records[0]) != len(wantHeader) {
		t.Fatalf("表头列数 = %d，期望 %d：%v", len(records[0]), len(wantHeader), records[0])
	}
	for i, h := range wantHeader {
		if records[0][i] != h {
			t.Errorf("表头第 %d 列 = %q，期望 %q", i, records[0][i], h)
		}
	}

	found := false
	for _, rec := range records[1:] {
		if rec[1] == "标题" {
			found = true
			if rec[2] != "正文" {
				t.Errorf("body = %q，期望 正文", rec[2])
			}
			if rec[3] != "true" {
				t.Errorf("published = %q，期望 true", rec[3])
			}
			if rec[4] == "" {
				t.Error("published_at 不应为空")
			}
		}
	}
	if !found {
		t.Error("CSV 应包含测试公告")
	}
}

func TestAdminExportAnnouncementsBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, err := ts.AdminExportAnnouncements(ctx); err == nil {
		t.Fatal("连接池已关闭时导出必须报错，不能静默返回空 CSV")
	}
}
