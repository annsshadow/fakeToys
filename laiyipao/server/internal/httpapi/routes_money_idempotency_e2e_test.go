// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R134（B 轮实证）：金钱类写操作的「同一请求连发两次」。
//
// # 后端的判据不是「接口是否 2xx」，而是**改余额的路径有没有互斥点**
//
//   - 兑换码：`redeem_usages UNIQUE (code_id, user_id)` +
//     `UPDATE redeem_codes SET used_count + 1 WHERE used_count < max_uses`
//     → 第二次必然 RowsAffected=0，只发一份奖励。
//   - 钱包发放（管理端）：`admin_idempotency_keys PRIMARY KEY (endpoint, key)`
//     → 带 `Idempotency-Key` 时第二次重放第一次响应、业务不执行；
//     不带 key 则按老路径**重复执行**（R86 的 TestIdemNoKeyKeepsOldBehavior 已钉住）。
//   - 商店购买：`user_purchases` 只有普通索引 `(user_id, item_id, purchase_date)`，
//     **没有唯一键**。限购 > 0 时靠 `SELECT user_id FROM user_wallets FOR UPDATE`
//     把同一用户的购买串行化，第二次被「今日次数已达上限」拒绝；
//     `limit_per_day = 0`（不限购）的商品没有这个保护 —— R134 实证会双扣，
//     修复与回归用例见 routes_shop_idempotency_r135_e2e_test.go。

import (
	"fmt"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

// TestE2EDoubleSubmitRedeemOnlyOnce：同一兑换码连发两次 → 只发一份奖励。
//
// 守住的是 `redeem_usages UNIQUE (code_id, user_id)` 与
// `UPDATE ... WHERE used_count < max_uses` 这两道条件写。
func TestE2EDoubleSubmitRedeemOnlyOnce(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, tok := e.newPlayer(t, "double_redeem")

	code := fmt.Sprintf("R134DBL%d", time.Now().UnixNano())
	e.exec(t, `INSERT INTO redeem_codes (id, code, reward, max_uses, used_count, enabled)
	           VALUES ((SELECT COALESCE(MAX(id),0)+1 FROM redeem_codes), $1, $2, 10, 0, true)`,
		code, []byte(`{"coin":100}`))
	t.Cleanup(func() {
		e.exec(t, `DELETE FROM redeem_usages WHERE code_id IN (SELECT id FROM redeem_codes WHERE code = $1)`, code)
		e.exec(t, `DELETE FROM redeem_codes WHERE code = $1`, code)
	})

	coinBefore := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uid)
	totalReward := 0
	for i := 0; i < 2; i++ {
		status, raw := doAuthed(t, e, "POST", "/api/v1/redeem/", tok,
			fmt.Sprintf(`{"code":%q}`, code))
		if i == 0 {
			if status != fiber.StatusOK {
				t.Fatalf("第一次兑换应 200，实际 %d：%s", status, raw)
			}
			totalReward = 100
			continue
		}
		if status != fiber.StatusForbidden {
			t.Fatalf("第二次兑换应 403（已经使用过），实际 %d：%s", status, raw)
		}
	}
	if got := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uid) - coinBefore; got != totalReward {
		t.Fatalf("连发两次应只发一份奖励（+%d），实际 %+d", totalReward, got)
	}
	if n := e.scalarInt(t, `SELECT COUNT(*) FROM redeem_usages WHERE user_id = $1`, uid); n != 1 {
		t.Fatalf("redeem_usages 应恰好 1 行，实际 %d", n)
	}
	if used := e.scalarInt(t, `SELECT used_count FROM redeem_codes WHERE code = $1`, code); used != 1 {
		t.Fatalf("used_count 应恰好 1，实际 %d", used)
	}
}

// TestE2EDoubleSubmitShopBuyDailyLimitRejected 守住限购商品的互斥点：
// 同一请求连发两次 → 第二次被 403 拒绝、只扣一次钱。
//
// 互斥点是 `SELECT user_id FROM user_wallets WHERE user_id = $1 FOR UPDATE`
// + `SELECT COUNT(*) ... purchase_date = today`（见 economy.go Buy）。
func TestE2EDoubleSubmitShopBuyDailyLimitRejected(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, tok := e.newPlayer(t, "double_buy_limit")
	itemID := newTestShopItem(t, e, "r134_capped", `{"gem":10}`, `{"coin":100}`, 1)

	gemBefore := e.scalarInt(t, `SELECT gem FROM user_wallets WHERE user_id = $1`, uid)
	coinBefore := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uid)

	path := "/api/v1/shop/" + fmtInt(itemID) + "/buy"
	if status, raw := doAuthed(t, e, "POST", path, tok, ""); status != fiber.StatusOK {
		t.Fatalf("第一次购买应 200，实际 %d：%s", status, raw)
	}
	if status, raw := doAuthed(t, e, "POST", path, tok, ""); status != fiber.StatusForbidden {
		t.Fatalf("第二次购买应 403（超每日限购），实际 %d：%s", status, raw)
	}

	if d := gemBefore - e.scalarInt(t, `SELECT gem FROM user_wallets WHERE user_id = $1`, uid); d != 10 {
		t.Fatalf("应只扣一次钱（-10 gem），实际 -%d", d)
	}
	if d := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uid) - coinBefore; d != 100 {
		t.Fatalf("应只发一份货（+100 coin），实际 +%d", d)
	}
	if n := e.scalarInt(t, `SELECT COUNT(*) FROM user_purchases WHERE user_id = $1 AND item_id = $2`, uid, itemID); n != 1 {
		t.Fatalf("购买记录应恰好 1 条，实际 %d", n)
	}
}

// newTestShopItem 造一个测试商品并注册清理。
func newTestShopItem(t *testing.T, e *e2e, code, price, payload string,
	limitPerDay int) int64 {
	t.Helper()
	var itemID int64
	if err := e.pool.QueryRow(t.Context(),
		`INSERT INTO shop_items (id, code, name, category, price, payload, limit_per_day, enabled)
		 VALUES ((SELECT COALESCE(MAX(id),0)+1 FROM shop_items), $1, $2, 'currency',
		         $3, $4, $5, true) RETURNING id`,
		code+"-"+fmtInt(time.Now().UnixNano()), code, []byte(price), []byte(payload),
		limitPerDay).Scan(&itemID); err != nil {
		t.Fatalf("造测试商品失败：%v", err)
	}
	t.Cleanup(func() {
		e.exec(t, `DELETE FROM user_purchases WHERE item_id = $1`, itemID)
		e.exec(t, `DELETE FROM shop_items WHERE id = $1`, itemID)
	})
	return itemID
}
