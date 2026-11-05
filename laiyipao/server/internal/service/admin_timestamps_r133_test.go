// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 第 133 轮：写端点时间字段的三方对照（请求给出的时刻 → 库里的值 → 接口读回的值）。
//
// # 两类精度问题必须分开
//
//  1. **存储列精度不足**：PostgreSQL 的 `timestamptz` 上限是**微秒**，
//     而 Go 的 `time.Time` 是纳秒。写入时纳秒部分被丢弃/舍入 ——
//     这不是缺陷，但**响应必须回读库值**，否则响应里的时刻在库里不存在
//     （R132 公告 `published_at`、R133 战斗开局 `expires_at` 都是这一族）。
//  2. **格式化时截断**：`timeFormat = time.RFC3339` 只到秒（adminutil.go），
//     用在「展示字符串」上（后台登录 `expires_at`、回放/验真的 `created_at`）。
//     库里的微秒仍在，只是字符串不带 —— 与本轮修的是两件事，不混在一起改。
//
// # 判据
//
// 写入给定时刻 → 接口读回 == 库值 == 给定时刻（含微秒）；
// 覆盖跨零点（本地 00:03，UTC 仍在前一天）与跨时区（-05:00）。

import (
	"context"
	"testing"
	"time"
)

// TestRedeemCodeExpiresAtThreeWay 兑换码单码 / 批量 / 编辑三条写路径的时间三方一致。
func TestRedeemCodeExpiresAtThreeWay(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 跨零点 + 跨时区：+08:00 的 2026-11-06 00:03:00.123456
	// = UTC 的 2026-11-05 16:03:00.123456（日期都变了）。
	// ⚠️ time.Date 的第 8 个参数是**纳秒**：123456000ns == 123456µs（微秒对齐，
	// 恰好是 timestamptz 能精确表示的最小单位）。
	want := time.Date(2026, 11, 6, 0, 3, 0, 123456000, time.FixedZone("CST", 8*3600))

	// ① 单码创建：请求（want）→ 响应 → 库 → 列表
	created, err := ts.AdminCreateRedeemCode(ctx, "R133MID", map[string]int{"coin": 10}, 1, &want)
	if err != nil {
		t.Fatalf("创建兑换码失败：%v", err)
	}
	if created.ExpiresAt == nil {
		t.Fatalf("创建响应缺 expires_at（入参给了 %s）", want.Format(time.RFC3339Nano))
	}
	if !created.ExpiresAt.Equal(want) {
		t.Fatalf("创建响应的 expires_at 与请求时刻不一致：响应 %s，请求 %s",
			created.ExpiresAt.Format(time.RFC3339Nano), want.Format(time.RFC3339Nano))
	}

	var dbExpires time.Time
	if err := ts.pool.QueryRow(ctx,
		`SELECT expires_at FROM redeem_codes WHERE id = $1`, created.ID).Scan(&dbExpires); err != nil {
		t.Fatalf("读库失败：%v", err)
	}
	if !dbExpires.Equal(want) {
		t.Fatalf("库值与请求时刻不一致：库 %s，请求 %s",
			dbExpires.Format(time.RFC3339Nano), want.Format(time.RFC3339Nano))
	}
	if dbExpires.Nanosecond()%1000 != 0 {
		t.Fatalf("库值不是微秒对齐（说明不是 timestamptz 的正常精度）：%d", dbExpires.Nanosecond())
	}

	// 列表走另一条 SQL（SELECT ... FROM redeem_codes），必须与库值逐字段一致。
	items, _, err := ts.AdminListRedeemCodes(ctx, 50, 0, "R133MID", nil, nil)
	if err != nil {
		t.Fatalf("列表兑换码失败：%v", err)
	}
	listed := false
	for _, it := range items {
		if it.ID != created.ID {
			continue
		}
		listed = true
		if it.ExpiresAt == nil || !it.ExpiresAt.Equal(want) {
			t.Fatalf("列表 expires_at 与请求时刻不一致：列表 %v，请求 %s", it.ExpiresAt, want.Format(time.RFC3339Nano))
		}
	}
	if !listed {
		t.Fatalf("列表里找不到刚创建的码 id=%d", created.ID)
	}

	// ② 编辑路径：换一个时区（-05:00）再写一次，响应必须回读库值。
	next := time.Date(2026, 11, 6, 23, 59, 0, 654321000, time.FixedZone("EST", -5*3600))
	updated, err := ts.AdminUpdateRedeemCode(ctx, created.ID, map[string]int{"coin": 10}, 1, &next)
	if err != nil {
		t.Fatalf("编辑兑换码失败：%v", err)
	}
	if updated.ExpiresAt == nil || !updated.ExpiresAt.Equal(next) {
		t.Fatalf("编辑响应的 expires_at 与请求时刻不一致：响应 %v，请求 %s",
			updated.ExpiresAt, next.Format(time.RFC3339Nano))
	}
	var dbNext time.Time
	if err := ts.pool.QueryRow(ctx,
		`SELECT expires_at FROM redeem_codes WHERE id = $1`, created.ID).Scan(&dbNext); err != nil {
		t.Fatalf("读库失败：%v", err)
	}
	if !dbNext.Equal(next) {
		t.Fatalf("编辑后库值与请求时刻不一致：库 %s，请求 %s",
			dbNext.Format(time.RFC3339Nano), next.Format(time.RFC3339Nano))
	}

	// ③ 批量路径：每一行都必须回读（批量以前是入参回显）。
	batch, err := ts.AdminCreateRedeemCodes(ctx, 2, map[string]int{"coin": 5}, 1, &want)
	if err != nil {
		t.Fatalf("批量创建失败：%v", err)
	}
	if len(batch) != 2 {
		t.Fatalf("批量应返回 2 个码，实际 %d", len(batch))
	}
	for _, it := range batch {
		if it.ExpiresAt == nil || !it.ExpiresAt.Equal(want) {
			t.Fatalf("批量响应 %s 的 expires_at 与请求时刻不一致：%v", it.Code, it.ExpiresAt)
		}
		var dbBatch time.Time
		if err := ts.pool.QueryRow(ctx,
			`SELECT expires_at FROM redeem_codes WHERE id = $1`, it.ID).Scan(&dbBatch); err != nil {
			t.Fatalf("读库失败：%v", err)
		}
		if !dbBatch.Equal(want) {
			t.Fatalf("批量落库值 %s 与请求时刻不一致：%s", it.Code, dbBatch.Format(time.RFC3339Nano))
		}
	}
}

// TestRedeemCodeExpiresAtNanosecondsTruncatedAtStorage 纳秒入参 → 微秒落库：
// 这是**存储列精度**（timestamptz 上限微秒），不是格式化截断（后者会掉到秒）。
func TestRedeemCodeExpiresAtNanosecondsTruncatedAtStorage(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 亚微秒部分取 400ns（<500ns）：PostgreSQL 的舍入与 Go 的截断都得到 .123456。
	raw := time.Date(2026, 11, 6, 0, 3, 0, 123456400, time.FixedZone("CST", 8*3600))
	stored := raw.Truncate(time.Microsecond)

	created, err := ts.AdminCreateRedeemCode(ctx, "R133NS", map[string]int{"coin": 10}, 1, &raw)
	if err != nil {
		t.Fatalf("创建兑换码失败：%v", err)
	}
	if created.ExpiresAt == nil {
		t.Fatalf("创建响应缺 expires_at")
	}
	// 响应必须是库值（微秒对齐），不是入参的纳秒。
	if !created.ExpiresAt.Equal(stored) {
		t.Fatalf("响应的 expires_at 应为库值 %s，实际 %s（入参 %s）",
			stored.Format(time.RFC3339Nano),
			created.ExpiresAt.Format(time.RFC3339Nano),
			raw.Format(time.RFC3339Nano))
	}
	if created.ExpiresAt.Nanosecond()%1000 != 0 {
		t.Fatalf("响应不是微秒对齐：%d", created.ExpiresAt.Nanosecond())
	}
	// 反证「不是格式化截断」：格式化截断会掉到整秒（Nanosecond()==0）。
	if created.ExpiresAt.Nanosecond() == 0 {
		t.Fatalf("响应被截到整秒（这是格式化截断，不是 timestamptz 的微秒上限）")
	}
}

// TestBattleTokenExpiresAtWriteThenRead 战斗开局凭证：响应必须回读库值。
func TestBattleTokenExpiresAtWriteThenRead(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx)
	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Fatalf("开局失败：%v", err)
	}

	var dbExpires time.Time
	if err := ts.pool.QueryRow(ctx,
		`SELECT expires_at FROM battle_tokens WHERE id = $1`, tp.TokenID).Scan(&dbExpires); err != nil {
		t.Fatalf("读 battle_tokens 失败：%v", err)
	}
	if !tp.ExpiresAt.Equal(dbExpires) {
		t.Fatalf("开局响应的 expires_at 与库值不一致：响应 %s，库 %s",
			tp.ExpiresAt.Format(time.RFC3339Nano), dbExpires.Format(time.RFC3339Nano))
	}
	// 只有从库里读回才会是微秒对齐；回显 `time.Now().Add(TTL)` 会带纳秒。
	if tp.ExpiresAt.Nanosecond()%1000 != 0 {
		t.Fatalf("开局响应的 expires_at 不是微秒对齐（说明回显的是入参纳秒，不是库值）：%d",
			tp.ExpiresAt.Nanosecond())
	}
}

// TestSaveDefenseShieldedUntilWriteThenRead 防线保存：`shielded_until` 由服务端
// 按 shield_hours 计算后写库，响应必须回读这个时刻（改前 DefenseView 根本不填它）。
func TestSaveDefenseShieldedUntilWriteThenRead(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx)
	before := time.Now()
	view, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{
		Name: "R133盾", Skills: []int{1}, ShieldHours: 24,
	})
	if err != nil {
		t.Fatalf("保存防线失败：%v", err)
	}
	if view.ShieldedUntil == nil {
		t.Fatalf("请求 shield_hours=24，但保存响应缺 shielded_until（写后没回读）")
	}

	var stored *time.Time
	if err := ts.pool.QueryRow(ctx,
		`SELECT shielded_until FROM defenses WHERE id = $1`, view.ID).Scan(&stored); err != nil {
		t.Fatalf("读库失败：%v", err)
	}
	if stored == nil || !view.ShieldedUntil.Equal(*stored) {
		t.Fatalf("响应 shielded_until=%v 与库值 %v 不一致", view.ShieldedUntil, stored)
	}
	// 只有从库里读回才会是微秒对齐；回显 `time.Now().Add(...)` 会带纳秒。
	if view.ShieldedUntil.Nanosecond()%1000 != 0 {
		t.Fatalf("响应 shielded_until 不是微秒对齐（说明回显的是入参纳秒）：%d",
			view.ShieldedUntil.Nanosecond())
	}
	// 时长必须落在 24h 附近（用注入前的真实时钟做宽松上下界）。
	if d := view.ShieldedUntil.Sub(before); d < 23*time.Hour+59*time.Minute || d > 24*time.Hour+time.Minute {
		t.Fatalf("shielded_until 与请求的 24 小时不符：%v", d)
	}

	// 不带护盾再保存一次：库与响应都必须回到 nil（不是残留上次的值）。
	view2, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{Name: "R133盾", Skills: []int{1}})
	if err != nil {
		t.Fatalf("再保存失败：%v", err)
	}
	if view2.ShieldedUntil != nil {
		t.Fatalf("未请求护盾时应为 nil，实际 %v", view2.ShieldedUntil)
	}
	var stored2 *time.Time
	if err := ts.pool.QueryRow(ctx,
		`SELECT shielded_until FROM defenses WHERE id = $1`, view2.ID).Scan(&stored2); err != nil {
		t.Fatalf("读库失败：%v", err)
	}
	if stored2 != nil {
		t.Fatalf("未请求护盾时库值应清空，实际 %v", stored2)
	}
}

// TestDefaultedTimeColumnsAreFilled 「列有默认值、INSERT 没写该列」的列必须由默认值填上，
// 且不能是 NULL / 零值 —— R132 公告 created_at 为 null 就是这一族的反例。
func TestDefaultedTimeColumnsAreFilled(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// redeem_codes：INSERT 只写 code/reward/max_uses/expires_at，created_at 走 default now()。
	created, err := ts.AdminCreateRedeemCode(ctx, "R133DEF", map[string]int{"coin": 1}, 1, nil)
	if err != nil {
		t.Fatalf("创建兑换码失败：%v", err)
	}
	var createdAt *time.Time
	if err := ts.pool.QueryRow(ctx,
		`SELECT created_at FROM redeem_codes WHERE id = $1`, created.ID).Scan(&createdAt); err != nil {
		t.Fatalf("读 created_at 失败：%v", err)
	}
	if createdAt == nil || createdAt.IsZero() {
		t.Fatalf("redeem_codes.created_at 应由默认值填充，实际 %v", createdAt)
	}

	// battle_tokens：issued_at 走 default now()，expires_at 显式写。
	uid := ts.newUser(t, ctx)
	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Fatalf("开局失败：%v", err)
	}
	var issuedAt time.Time
	if err := ts.pool.QueryRow(ctx,
		`SELECT issued_at FROM battle_tokens WHERE id = $1`, tp.TokenID).Scan(&issuedAt); err != nil {
		t.Fatalf("读 issued_at 失败：%v", err)
	}
	if issuedAt.IsZero() {
		t.Fatalf("battle_tokens.issued_at 应由默认值填充，实际零值")
	}
	// 默认值 now() 必须真的在「签发时刻」附近，而不是某个静态常量。
	if d := tp.ExpiresAt.Sub(issuedAt); d <= 0 || d > 24*time.Hour {
		t.Fatalf("issued_at 与 expires_at 的时间差异常：%v", d)
	}
}
