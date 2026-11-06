// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 第 132 轮：审计实证的两条**写后必读**缺陷。
//
// # 缺陷一：公告创建响应丢字段 + 时间精度不一致
//
// 改前 `AdminCreateAnnouncement` 返回的 view 是拿入参拼的：
// `created_at` 从未赋值 → HTTP 响应里是 `null`（列表/详情却有值）；
// `published_at` 回显的是 Go 的 `time.Now()`（纳秒），而 PostgreSQL 只存
// 微秒 —— 「写入时刻 → 读回」在亚微秒上不相等。判据是三方一致：
// 创建响应、数据库值、列表读回值，全部逐字段相等（含微秒）。
//
// # 缺陷二：兑换码奖励接受未知币种
//
// 改前创建/编辑/批量三条路径都不校验奖励键，`{"diamond": 1}` 能建码成功；
// 玩家兑换时才由 `grantWallet` 报「未知货币」—— 运营看到的是一批没人能兑
// 的码。判据两方向：受支持币种（walletColumns ∪ walletTokens）建码且**真的
// 能兑**；未知币种创建/批量/编辑三条入口都返回 ErrBadInput。
// 破坏实验：把 `validateRedeemReward` 调成 `return nil`，本测试必须红。

import (
	"context"
	"errors"
	"testing"
	"time"
)

// TestAdminCreateAnnouncementWriteThenReadTimestamps 创建响应 × 数据库 × 列表三方逐字段一致。
func TestAdminCreateAnnouncementWriteThenReadTimestamps(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	created, err := ts.AdminCreateAnnouncement(ctx, "R132 标题", "R132 正文", true)
	if err != nil {
		t.Fatalf("创建公告失败：%v", err)
	}
	if created.CreatedAt == nil {
		t.Fatalf("创建响应缺 created_at（改前是 null）")
	}
	if created.PublishedAt == nil {
		t.Fatalf("已发布公告的创建响应缺 published_at")
	}

	// ② 数据库原始值
	var dbCreated time.Time
	var dbPublished *time.Time
	if err := ts.pool.QueryRow(ctx,
		`SELECT created_at, published_at FROM announcements WHERE id = $1`,
		created.ID).Scan(&dbCreated, &dbPublished); err != nil {
		t.Fatalf("读回公告失败：%v", err)
	}
	if !created.CreatedAt.Equal(dbCreated) {
		t.Fatalf("创建响应的 created_at 与库值不一致：响应 %s，库 %s",
			created.CreatedAt.Format(time.RFC3339Nano), dbCreated.Format(time.RFC3339Nano))
	}
	if dbPublished == nil || !created.PublishedAt.Equal(*dbPublished) {
		t.Fatalf("创建响应的 published_at 与库值不一致：响应 %v，库 %v", created.PublishedAt, dbPublished)
	}

	// ③ 列表读回（列表走的是另一条 SQL）
	items, _, err := ts.AdminListAnnouncements(ctx, 50, 0, nil, nil, nil)
	if err != nil {
		t.Fatalf("列表公告失败：%v", err)
	}
	found := false
	for _, it := range items {
		if it.ID != created.ID {
			continue
		}
		found = true
		if it.CreatedAt == nil || !it.CreatedAt.Equal(dbCreated) {
			t.Fatalf("列表 created_at 与库值不一致：%v vs %v", it.CreatedAt, dbCreated)
		}
	}
	if !found {
		t.Fatalf("列表里找不到刚创建的公告 id=%d", created.ID)
	}

	// ④ 精度：库与响应都必须是微秒精度（PostgreSQL timestamptz 上限），
	//    且不是被截到秒 —— 秒级截断会让 Nanosecond() 恒为 0。
	if dbCreated.Nanosecond()%1000 != 0 {
		t.Errorf("库值精度异常（不是微秒对齐）：%d", dbCreated.Nanosecond())
	}
	// 草稿（published=false）：published_at 三方都必须是 NULL
	draft, err := ts.AdminCreateAnnouncement(ctx, "R132 草稿", "正文", false)
	if err != nil {
		t.Fatalf("创建草稿失败：%v", err)
	}
	if draft.CreatedAt == nil {
		t.Fatalf("草稿创建响应缺 created_at")
	}
	if draft.PublishedAt != nil {
		t.Fatalf("草稿的 published_at 应为 nil，实际 %v", draft.PublishedAt)
	}
}

// TestAdminRedeemCodeRejectsUnknownCurrency 未知币种三入口（创建/批量/编辑）全部拒绝。
func TestAdminRedeemCodeRejectsUnknownCurrency(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// ① 合法币种：walletColumns 四列 + 道具
	legal := []map[string]int{
		{"coin": 100},
		{"gem": 5, "energy": 3, "keys": 1},
		{"revive_token": 1},
		{"mastery_reset": 2, "gem_wash_token": 1},
	}
	for i, reward := range legal {
		code := "R132OK" + string(rune('A'+i))
		if _, err := ts.AdminCreateRedeemCode(ctx, code, reward, 1, nil); err != nil {
			t.Fatalf("合法奖励 %v 应能建码：%v", reward, err)
		}
	}

	// ② 未知币种：创建拒绝
	_, err := ts.AdminCreateRedeemCode(ctx, "R132BAD", map[string]int{"diamond": 1}, 1, nil)
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("未知币种 diamond 创建应返回 ErrBadInput，实际 %v", err)
	}
	// 且没有落库
	if n := ts.scalar(t, ctx, `SELECT COUNT(*) FROM redeem_codes WHERE code = 'R132BAD'`); n != 0 {
		t.Fatalf("被拒绝的兑换码不应落库，实存 %d 行", n)
	}

	// ③ 批量拒绝，且整批零落库
	before := ts.scalar(t, ctx, `SELECT COUNT(*) FROM redeem_codes`)
	if _, err := ts.AdminCreateRedeemCodes(ctx, 5, map[string]int{"diamond": 1}, 1, nil); !errors.Is(err, ErrBadInput) {
		t.Fatalf("批量创建未知币种应返回 ErrBadInput，实际 %v", err)
	}
	if after := ts.scalar(t, ctx, `SELECT COUNT(*) FROM redeem_codes`); after != before {
		t.Fatalf("批量被拒后不应新增兑换码：前 %d，后 %d", before, after)
	}

	// ④ 编辑拒绝
	id := ts.scalar(t, ctx, `SELECT id FROM redeem_codes WHERE code = 'R132OKA'`)
	if _, err := ts.AdminUpdateRedeemCode(ctx, int(id), map[string]int{"diamond": 1}, 1, nil); !errors.Is(err, ErrBadInput) {
		t.Fatalf("编辑成未知币种应返回 ErrBadInput，实际 %v", err)
	}

	// ⑤ 白名单必须与发奖路径同集：合法建出来的道具码**真的能兑**
	uid := ts.newUser(t, ctx)
	if _, err := ts.Redeem(ctx, uid, "R132OKC"); err != nil {
		t.Fatalf("合法道具兑换码应能兑换：%v", err)
	}
	if bal := ts.tokenBalance(t, ctx, uid, "revive_token"); bal != 1 {
		t.Fatalf("兑换后道具余额应为 1，实际 %d", bal)
	}
}
