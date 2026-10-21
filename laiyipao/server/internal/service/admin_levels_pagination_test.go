package service

import (
	"context"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// TestAdminListLevelsPaginatesWithARealTotal 钉住 P3-4。
//
// # 背景
//
// `GET /admin/levels` 的 `total` 此前是 `len(out)` ——
// 「这次返回了几行」，而不是「一共有几行」。
//
// 在没有分页时它**恰好**等于真实总数，所以看起来是对的；
// 一旦加了 limit，它就变成一句谎话：分页器显示「共 50 条」，
// 而库里其实有 100 关 —— 运营会以为「另外 50 关不见了」。
//
// # 三条判据
//
//  1. 不带 limit 时 total == 真实关卡数（回归守卫）；
//  2. 带 limit 时**返回的行数**变少，但 **total 不变**（仍是真实总数）；
//  3. offset 能翻到后面的行，且翻页内容不重复。
func TestAdminListLevelsPaginatesWithARealTotal(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 1) 全量
	all, total, err := ts.AdminListLevels(ctx, 0, "", 0, 0)
	if err != nil {
		t.Fatalf("列关卡失败：%v", err)
	}
	if total != int64(domain.TotalLevels) {
		t.Errorf("全量 total 应为 %d，实际 %d", domain.TotalLevels, total)
	}
	if int64(len(all)) != total {
		t.Errorf("全量返回行数(%d) 应与 total(%d) 一致", len(all), total)
	}

	// 2) 带 limit：行数变少，total 不变
	page, pageTotal, err := ts.AdminListLevels(ctx, 0, "", 10, 0)
	if err != nil {
		t.Fatalf("分页列关卡失败：%v", err)
	}
	if len(page) != 10 {
		t.Errorf("limit=10 应回 10 行，实际 %d", len(page))
	}
	if pageTotal != total {
		t.Errorf("分页时 total 应仍是真实总数(%d)，实际 %d —— "+
			"把它变成「这次返回了几行」会让分页器显示「共 10 条」而实际有 %d 关",
			total, pageTotal, total)
	}

	// 3) offset 翻到后面，且不与第一页重复
	page2, total2, err := ts.AdminListLevels(ctx, 0, "", 10, 10)
	if err != nil {
		t.Fatalf("第二页列关卡失败：%v", err)
	}
	if total2 != total {
		t.Errorf("第二页 total 应与第一页相同(%d)，实际 %d", total, total2)
	}
	if len(page2) != 10 {
		t.Errorf("第二页应回 10 行，实际 %d", len(page2))
	}
	seen := map[int]bool{}
	for _, lv := range page {
		seen[lv.ID] = true
	}
	for _, lv := range page2 {
		if seen[lv.ID] {
			t.Errorf("第 11 行与第 1 页重复（id=%d）—— 翻页语义坏了", lv.ID)
		}
	}
}
