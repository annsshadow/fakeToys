package httpapi

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

// 第 110 轮：兑换码的过期时间必须**真的进库、真的被兑换路径消费**。
//
// 修前：UI 有「过期时间（可空）」输入框（RFC3339 占位示例），
// handler 也解析了 `expires_at`——但**从不传给 service**，
// INSERT 也不写这一列。运营设的过期时间被静默丢弃，
// 兑换码永久有效（可被无限期转卖滥用）。
//
// 兑换路径本身是认 expires_at 的（Redeem 的 SQL：
// `expires_at IS NULL OR expires_at > now()`），
// 所以本轮的判据全部落在**行为**上：
// 过期码兑换必须被拒、未来码必须能用、裸本地时间必须 400。
func TestRedeemCodeExpiresAtEndToEnd(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "redexp110")
	pid, ptok := e.newPlayer(t, "redexp110p")

	mk := func(suffix, expires string) (string, int, map[string]any) {
		code := fmt.Sprintf("E110-%s-%d", suffix, time.Now().UnixNano()%1_000_000)
		payload := map[string]any{"code": code, "reward": map[string]any{"coin": 1}, "max_uses": 1}
		if expires != "" {
			payload["expires_at"] = expires
		}
		st, body := e.post(t, "/api/v1/admin/redeem-codes", tok, payload)
		return code, st, body
	}

	// 1) 未来过期 → 200，库里非 NULL
	code, st, _ := mk("future", "2099-01-01T00:00:00Z")
	if st != 200 {
		t.Fatalf("带未来过期时间的创建应 200，实际 %d", st)
	}
	if got := e.scalarInt(t, `SELECT CASE WHEN expires_at IS NULL THEN 0 ELSE 1 END FROM redeem_codes WHERE code = $1`, code); got != 1 {
		t.Fatalf("expires_at 未落库（修前 INSERT 根本不写这一列）：code=%s", code)
	}
	t.Cleanup(func() {
		_, _ = e.pool.Exec(context.Background(), `DELETE FROM redeem_codes WHERE code = $1`, code)
	})

	// 2) 无过期时间 → NULL（可空语义保留）
	perm, st, _ := mk("permanent", "")
	if st != 200 {
		t.Fatalf("不带过期时间应 200，实际 %d", st)
	}
	if got := e.scalarInt(t, `SELECT CASE WHEN expires_at IS NULL THEN 1 ELSE 0 END FROM redeem_codes WHERE code = $1`, perm); got != 1 {
		t.Errorf("空过期时间应落 NULL，实际非 NULL")
	}
	t.Cleanup(func() {
		_, _ = e.pool.Exec(context.Background(), `DELETE FROM redeem_codes WHERE code = $1`, perm)
	})

	// 3) 裸本地时间（无时区）必须 400 ——
	// 兑换判定是 UTC 语义，接受裸本地时间等于把「过期」交给部署机时区漂 8 小时
	_, st, _ = mk("naive", "2026-12-31 23:59:59")
	if st != fiber.StatusBadRequest {
		t.Errorf("裸本地时间应 400（要求 RFC3339 带时区），实际 %d", st)
	}

	// 4) 行为判据：过期码兑换被拒（「在列表里」≠「能用」）
	past, st, _ := mk("past", "2000-01-01T00:00:00Z")
	if st != 200 {
		t.Fatalf("过去过期时间也应能创建（运营可能就是要一个作废码），实际 %d", st)
	}
	if st, _ := e.post(t, "/api/v1/redeem", ptok, map[string]any{"code": past}); st != 400 {
		t.Errorf("过期兑换码兑换应 400（Redeem 的 expires_at > now() 没拦住），实际 %d", st)
	}
	t.Cleanup(func() {
		_, _ = e.pool.Exec(context.Background(), `DELETE FROM redeem_codes WHERE code = $1`, past)
	})

	// 5) 对照：未来码必须**能用**（字段存在但误伤正常路径 = 另一种缺陷）
	if st, _ := e.post(t, "/api/v1/redeem", ptok, map[string]any{"code": code}); st != 200 {
		t.Errorf("未过期兑换码兑换应 200，实际 %d", st)
	}
	_ = pid
}
