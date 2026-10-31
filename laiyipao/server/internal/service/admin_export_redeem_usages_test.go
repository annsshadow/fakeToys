package service

// 后台兑换码使用记录 CSV 导出的服务层测试（2026-10-31 新一轮第 34 轮）。

import (
	"context"
	"strings"
	"testing"
)

func TestAdminExportRedeemUsages(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "ruexp")
	var codeID int
	var code string
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT id, code FROM redeem_codes LIMIT 1`).Scan(&codeID, &code); err != nil {
		t.Fatalf("取兑换码失败：%v", err)
	}
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO redeem_usages (code_id, user_id) VALUES ($1, $2)`, codeID, uid); err != nil {
		t.Fatalf("插入使用记录失败：%v", err)
	}

	data, err := ts.AdminExportRedeemUsages(ctx, "", 0)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	if !strings.HasPrefix(string(data), "\xEF\xBB\xBF") {
		t.Error("CSV 应带 UTF-8 BOM")
	}
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 2 {
		t.Fatalf("应有 2 行（1 表头 + 1 数据），实际 %d", len(lines))
	}
	header := lines[0]
	for _, col := range []string{"id", "code", "user_id", "nickname", "reward", "created_at"} {
		if !strings.Contains(header, col) {
			t.Errorf("表头缺少列 %s：%s", col, header)
		}
	}
	row := lines[1]
	if !strings.Contains(row, code) {
		t.Errorf("数据行应含兑换码：%s", row)
	}
	if !strings.Contains(row, fmtIntForTest(uid)) {
		t.Errorf("数据行应含 user_id：%s", row)
	}
	if !strings.Contains(row, "coin:") {
		t.Errorf("数据行应含 reward 可读文本（coin:...）：%s", row)
	}
}

func TestAdminExportRedeemUsagesFiltered(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uidA, _ := newGuestUser(t, ts, "ruexp_f_a")
	uidB, _ := newGuestUser(t, ts, "ruexp_f_b")
	seedAdminRedeemCode(t, ts, 9301, "LYP-EXP-A", true, nil)
	seedAdminRedeemCode(t, ts, 9302, "LYP-EXP-B", true, nil)
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO redeem_usages (code_id, user_id) VALUES (9301, $1), (9302, $2)`, uidA, uidB); err != nil {
		t.Fatalf("插入使用记录失败：%v", err)
	}

	// 按码搜索 → 1 行数据
	data, err := ts.AdminExportRedeemUsages(ctx, "EXP-A", 0)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 2 {
		t.Errorf("按码搜索应 2 行，实际 %d", len(lines))
	}

	// 按用户过滤 → 1 行数据
	data, err = ts.AdminExportRedeemUsages(ctx, "", uidB)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	lines = strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 2 {
		t.Errorf("按用户过滤应 2 行，实际 %d", len(lines))
	}
}

func TestAdminExportRedeemUsagesBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, err := ts.AdminExportRedeemUsages(ctx, "", 0); err == nil {
		t.Fatal("连接池已关闭时必须报错")
	}
}

// fmtIntForTest 把 int64 转十进制字符串（测试内避免 strconv 导入噪音）。
func fmtIntForTest(n int64) string {
	if n == 0 {
		return "0"
	}
	var buf [20]byte
	i := len(buf)
	for n > 0 {
		i--
		buf[i] = byte('0' + n%10)
		n /= 10
	}
	return string(buf[i:])
}