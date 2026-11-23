// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"context"
	"testing"

	"github.com/gofiber/fiber/v2"
)

// TestE2EHighPierceSettleAccepted 把「一发穿多敌」的真实上报口径打穿到 HTTP 边界。
//
// R13 真实事故：电+腐蚀 127 反应局上报 hits=165 > shots=160 被
// ValidateSettle 以 ErrInvalidHitRate 拒掉。根因在引擎侧：一发穿透弹丸
// 命中 4 只敌人时旧实现 hits++ 四次，而 shots 只在开火时 +1。
// 服务端校验必须保留，所以修的是引擎计数：一发穿 4 只只记 hits=1。
//
// 本测试用真实 HTTP 结算端点证明：按修正后口径上报的高穿透战报能入账，
// 而不是靠放宽 ValidateSettle 蒙混过关。
func TestE2EHighPierceSettleAccepted(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "pierce")

	status, body := e.post(t, "/api/v1/battle/token", tok, map[string]any{"level_id": 1})
	if status != fiber.StatusOK {
		t.Fatalf("开局应成功，实际 %d %v", status, body)
	}
	tokenID := num(body, "token_id")

	settle := e.settleBody(t, tokenID)
	// 一发穿 4 只：shots=1，hits=1（旧口径会报 hits=4 被 422 拒绝）。
	// reactions 必须同步收紧：ValidateSettle 限制 reactions <= shots*MaxReactionsPerHit。
	settle["shots"] = 1
	settle["hits"] = 1
	settle["kills"] = 4
	settle["reactions"] = 1
	settle["reactions_used"] = map[string]int{"overheat": 1}

	// 旧口径（一发穿 4 只记 hits=4）仍必须被服务端拒绝：
	// 修的是引擎计数，绝不能放宽 ValidateSettle 来掩盖前端错误。
	bad := make(map[string]any, len(settle))
	for k, v := range settle {
		bad[k] = v
	}
	bad["hits"] = 4
	status, resp := e.post(t, "/api/v1/battle/settle", tok, bad)
	if status != fiber.StatusUnprocessableEntity {
		t.Fatalf("旧口径 hits=4 > shots=1 仍应 422，实际 %d %v", status, resp)
	}

	status, resp = e.post(t, "/api/v1/battle/settle", tok, settle)
	if status != fiber.StatusOK {
		t.Fatalf("合法高穿透结算应 200，实际 %d %v", status, resp)
	}

	// 战报必须真的落库，而不是只返回 200。
	var hits, shots int64
	if err := e.pool.QueryRow(context.Background(),
		`SELECT hits, shots FROM battle_records WHERE user_id = $1 ORDER BY id DESC LIMIT 1`, uid).
		Scan(&hits, &shots); err != nil {
		t.Fatalf("查落库战报失败：%v", err)
	}
	if hits != 1 || shots != 1 {
		t.Errorf("落库命中口径应为 hits=1/shots=1，实际 hits=%d shots=%d", hits, shots)
	}
}
