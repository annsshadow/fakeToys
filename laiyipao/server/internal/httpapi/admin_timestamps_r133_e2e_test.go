// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// 第 133 轮：请求字符串 → 库 → 接口读回 的端到端时间对照。
//
// service 层已经验证「time.Time 入参 → 库 → 读回」；这里补的是**字符串**这一段：
// handler 用 `time.Parse(time.RFC3339, s)` 解析（Go 在解析时接受小数秒，
// 即使 layout 里没有写），必须确认小数秒没有被 handler 吃掉。

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func TestE2ERedeemCodeExpiresAtStringRoundTrip(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "r133ts")

	code := fmt.Sprintf("E2ER133%d", time.Now().UnixNano()%1_000_000)
	// 跨零点 + 跨时区：+08:00 的 2026-11-06 00:03:00.123456 = UTC 前一天 16:03。
	raw := "2026-11-06T00:03:00.123456+08:00"
	want, err := time.Parse(time.RFC3339Nano, raw)
	if err != nil {
		t.Fatalf("测试夹具时间写错了：%v", err)
	}
	t.Cleanup(func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `DELETE FROM redeem_codes WHERE code = $1`, code)
	})

	status, body := e.post(t, "/api/v1/admin/redeem-codes", tok, map[string]any{
		"code": code, "reward": map[string]int{"coin": 1}, "max_uses": 1,
		"expires_at": raw,
	})
	if status != 200 {
		t.Fatalf("创建兑换码应 200，实际 %d %v", status, body)
	}
	item, ok := body["item"].(map[string]any)
	if !ok {
		t.Fatalf("响应缺 item：%v", body)
	}
	gotStr, ok := item["expires_at"].(string)
	if !ok || gotStr == "" {
		t.Fatalf("响应缺 expires_at（请求给了 %s）：%v", raw, item)
	}
	got, err := time.Parse(time.RFC3339Nano, gotStr)
	if err != nil {
		t.Fatalf("响应 expires_at 不是合法时刻：%q", gotStr)
	}
	// ① 请求字符串 → 响应字符串
	if !got.Equal(want) {
		t.Fatalf("响应 expires_at 与请求不一致：响应 %s，请求 %s", gotStr, raw)
	}

	// ② 请求 → 库
	id := int64(item["id"].(float64))
	var db time.Time
	if err := e.pool.QueryRow(context.Background(),
		`SELECT expires_at FROM redeem_codes WHERE id = $1`, id).Scan(&db); err != nil {
		t.Fatalf("读库失败：%v", err)
	}
	if !db.Equal(want) {
		t.Fatalf("库值与请求不一致：库 %s，请求 %s", db.Format(time.RFC3339Nano), raw)
	}

	// ③ 库 → 列表读回（另一条 SQL）
	status, body = e.get(t, "/api/v1/admin/redeem-codes?keyword="+code, tok)
	if status != 200 {
		t.Fatalf("列表应 200，实际 %d %v", status, body)
	}
	list, _ := body["items"].([]any)
	found := false
	for _, rv := range list {
		it, _ := rv.(map[string]any)
		if it["code"] != code {
			continue
		}
		found = true
		s, _ := it["expires_at"].(string)
		listed, err := time.Parse(time.RFC3339Nano, s)
		if err != nil || !listed.Equal(want) {
			t.Fatalf("列表读回的 expires_at 与请求不一致：%v", it["expires_at"])
		}
	}
	if !found {
		t.Fatalf("列表按 keyword=%s 找不到刚创建的码", code)
	}
}
