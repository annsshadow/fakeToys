package service

// 后台兑换码 CSV 导出的服务层测试（2026-10-31 实质优化第 37 轮）。
//
// 关键行为锚点：
//   - 输出是 UTF-8 BOM + CSV；
//   - 表头 7 列（id/code/reward/max_uses/used_count/expires_at/enabled）；
//   - reward 以 `coin:100` 可读文本导出；
//   - 库故障必须报错而不是静默返回空 CSV。

import (
	"bytes"
	"context"
	"encoding/csv"
	"fmt"
	"testing"
	"time"
)

func TestAdminExportRedeemCodesCSV(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	code := fmt.Sprintf("EXPORT_%d", time.Now().UnixNano())
	if _, err := ts.AdminCreateRedeemCode(ctx, code, map[string]int{"coin": 100, "gem": 5}, 10, nil); err != nil {
		t.Fatalf("创建兑换码失败：%v", err)
	}

	data, err := ts.AdminExportRedeemCodes(ctx)
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
	wantHeader := []string{"id", "code", "reward", "max_uses", "used_count", "expires_at", "enabled"}
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
		if rec[1] == code {
			found = true
			// reward 文本（排序后）：coin:100 gem:5
			if rec[2] != "coin:100 gem:5" {
				t.Errorf("reward 文本 = %q", rec[2])
			}
			if rec[3] != "10" {
				t.Errorf("max_uses = %q，期望 10", rec[3])
			}
			if rec[6] != "true" {
				t.Errorf("enabled = %q，期望 true", rec[6])
			}
		}
	}
	if !found {
		t.Error("CSV 应包含测试兑换码")
	}
}

func TestAdminExportRedeemCodesBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, err := ts.AdminExportRedeemCodes(ctx); err == nil {
		t.Fatal("连接池已关闭时导出必须报错，不能静默返回空 CSV")
	}
}
