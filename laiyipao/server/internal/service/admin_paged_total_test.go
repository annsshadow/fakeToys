package service

import (
	"context"
	"testing"
)

// TestAdminPagedListsReportTotal 钉住三个分页列表的 `total`。
//
// # 背景（2026-10-21 第二轮审计 P2-6）
//
// 这三个接口早就支持 `offset`，但信封里**没有** `total`：
//
//	AdminAnnouncementsResp / AdminRedeemCodesResp / AdminAuditLogsResp
//
// 于是前端只能显示 `items.length` —— 那是**当前页**的条数，不是表的大小。
// 拿它算页数当然是错的（第 2 页 / 共 1 页）。
//
// 更糟的是：接口能翻页、页面却没有翻页器 ——
// **更早的公告 / 兑换码 / 审计记录根本看不到**。
//
// # 判据
//
// `total` 必须是**全量行数**，且随数据增减而变化；
// 翻到第二页时 `items` 变少但 `total` 不变。
func TestAdminPagedListsReportTotal(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// ── 公告 ───────────────────────────────────────────────────────────
	before, _, err := ts.AdminListAnnouncements(ctx, 50, 0)
	if err != nil {
		t.Fatalf("读公告列表失败：%v", err)
	}
	ann, err := ts.AdminCreateAnnouncement(ctx, "分页契约测试", "正文", false)
	if err != nil {
		t.Fatalf("建公告失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(), `DELETE FROM announcements WHERE id = $1`, ann.ID)
	})

	items, total, err := ts.AdminListAnnouncements(ctx, 50, 0)
	if err != nil {
		t.Fatalf("读公告列表失败：%v", err)
	}
	if total <= int64(len(before)) {
		t.Errorf("新建一条公告后 total 应变大：before=%d after=%d", len(before), total)
	}
	if int64(len(items)) != total && total < 50 {
		t.Errorf("total 与实际行数不一致：total=%d items=%d", total, len(items))
	}

	// 翻页：items 变少，total 不变
	_, total2, err := ts.AdminListAnnouncements(ctx, 1, 0)
	if err != nil {
		t.Fatalf("读公告第一页失败：%v", err)
	}
	if total2 != total {
		t.Errorf("换个 limit 不该改变 total：%d vs %d", total2, total)
	}

	// ── 兑换码 ─────────────────────────────────────────────────────────
	codes, codeTotal, err := ts.AdminListRedeemCodes(ctx, 50, 0)
	if err != nil {
		t.Fatalf("读兑换码列表失败：%v", err)
	}
	if codeTotal != int64(len(codes)) && codeTotal < 50 {
		t.Errorf("兑换码的 total 与实际行数不一致：total=%d items=%d", codeTotal, len(codes))
	}

	// ── 审计日志 ───────────────────────────────────────────────────────
	logs, logTotal, err := ts.AdminListAuditLogs(ctx, 50, 0)
	if err != nil {
		t.Fatalf("读审计日志失败：%v", err)
	}
	if logTotal != int64(len(logs)) && logTotal < 50 {
		t.Errorf("审计日志的 total 与实际行数不一致：total=%d items=%d", logTotal, len(logs))
	}
}
