// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R52：经济写端点的限流**行为**证据（源码扫描之外的真实计数验证）。
//
// 源码扫描（route_ratelimit_guard_test.go）只能证明 `authRateLimit` 还挂在路由上，
// 证明不了额度真的生效。这里对 `/shop/:id/buy` 连打 economyWriteLimit+10 次：
// 期望前 60 次成功、其余 429，且落库行数恰好等于成功次数。
//
// 反面证据（改前）：同一探针在未挂限流时得到 70×200、70 行购买记录、0 次 429。

import (
	"fmt"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestEconomyWriteRateLimitActuallyBlocks(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "r52limit")
	e.exec(t, `UPDATE user_wallets SET coin = 10000 WHERE user_id = $1`, uid)

	itemCode := fmt.Sprintf("r52_shop_%d", time.Now().UnixNano())
	var itemID int
	if err := e.pool.QueryRow(t.Context(),
		`INSERT INTO shop_items (id, code, name, category, price, payload, limit_per_day, enabled)
		 VALUES ((SELECT COALESCE(MAX(id),0)+1 FROM shop_items), $1, 'R52 商品', 'currency',
		         $2, $3, 0, true) RETURNING id`,
		itemCode, []byte(`{"coin":1}`), []byte(`{"coin":0}`)).Scan(&itemID); err != nil {
		t.Fatalf("造商品失败：%v", err)
	}

	const n = economyWriteLimit + 10
	status := map[int]int{}
	for range n {
		st, _ := e.post(t, fmt.Sprintf("/api/v1/shop/%d/buy", itemID), tok, nil)
		status[st]++
	}

	if status[fiber.StatusOK] != economyWriteLimit {
		t.Errorf("成功次数 = %d，期望恰好 %d（额度），状态分布=%v",
			status[fiber.StatusOK], economyWriteLimit, status)
	}
	if status[fiber.StatusTooManyRequests] != 10 {
		t.Errorf("429 次数 = %d，期望 10，状态分布=%v",
			status[fiber.StatusTooManyRequests], status)
	}

	purchases := e.scalarInt(t, `SELECT COUNT(*) FROM user_purchases WHERE user_id=$1 AND item_id=$2`, uid, itemID)
	if purchases != economyWriteLimit {
		t.Errorf("落库购买记录 = %d，期望 %d（与放行次数一致，不多写）", purchases, economyWriteLimit)
	}
}
