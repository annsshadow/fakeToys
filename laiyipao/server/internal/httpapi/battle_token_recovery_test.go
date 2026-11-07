// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"testing"

	"github.com/gofiber/fiber/v2"
)

// TestE2EBattleTokenLostRetrySettlesOnce 是 R151 的破坏实验：
// 模拟 `synchronous_commit=off` 只在 OS 崩溃/断电（PG 自身崩溃安全）时
// 丢掉已下发的 battle token 行，然后复刻客户端（battle.vue settle）的
// 「404 not_found → 重取一次同关凭证 → 重报」路径。
//
// 判据（用户口径）：
//  1. 令牌丢失必须是可恢复的，不能硬失败：旧 token 结算得到 404 not_found，
//     客户端据此触发重取，而不是把这一局判掉；
//  2. 重报后正常结算，战报只落 1 条（不掉奖励、不重复发奖）；
//  3. 重报成功后再结算同一 token 仍 422，钱包不再变化；
//  4. 重取会再扣一次体力 —— 这是该路径的显式代价，台账 R151 记录。
func TestE2EBattleTokenLostRetrySettlesOnce(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "tokenlost")
	e.exec(t, `UPDATE user_wallets SET energy = 120, energy_updated_at = now() WHERE user_id = $1`, uid)

	status, body := e.post(t, "/api/v1/battle/token", tok, map[string]any{"level_id": 1})
	if status != fiber.StatusOK {
		t.Fatalf("开局应成功，实际 %d %v", status, body)
	}
	token1 := num(body, "token_id")
	energy1 := e.scalarInt(t, `SELECT energy FROM user_wallets WHERE user_id = $1`, uid)
	cost := 120 - energy1
	if token1 <= 0 || cost <= 0 {
		t.Fatalf("开局必须下发 token 并扣体力：token=%d energy=%d", token1, energy1)
	}

	// 破坏点：直接删掉 token 行，等价于 off 提交在 OS 崩溃后没落盘。
	e.exec(t, `DELETE FROM battle_tokens WHERE id = $1`, token1)

	// 1) 旧 token 结算 → 404 not_found（硬失败会变成客户端判掉这一局）。
	status, body = e.post(t, "/api/v1/battle/settle", tok, map[string]any{"token_id": token1})
	if status != fiber.StatusNotFound {
		t.Fatalf("token 行丢失后结算应 404，实际 %d %v", status, body)
	}
	eb, _ := body["error"].(map[string]any)
	if eb["code"] != "not_found" {
		t.Fatalf("404 必须带 not_found 码（客户端重试判据），实际 %v", body)
	}
	if got := e.scalarInt(t, `SELECT energy FROM user_wallets WHERE user_id = $1`, uid); got != energy1 {
		t.Errorf("失败的结算不得动钱包：energy %d -> %d", energy1, got)
	}

	// 2) 客户端重取一次同关凭证，用新 token 重报。
	status, body = e.post(t, "/api/v1/battle/token", tok, map[string]any{"level_id": 1})
	if status != fiber.StatusOK {
		t.Fatalf("重取凭证应成功，实际 %d %v", status, body)
	}
	token2 := num(body, "token_id")
	energy2 := e.scalarInt(t, `SELECT energy FROM user_wallets WHERE user_id = $1`, uid)
	if energy2 != energy1-cost {
		t.Errorf("重取会再扣一次体力（R151 已知代价）：%d -> %d，单次 %d", energy1, energy2, cost)
	}

	status, body = e.post(t, "/api/v1/battle/settle", tok, e.settleBody(t, token2))
	if status != fiber.StatusOK {
		t.Fatalf("重报应结算成功，实际 %d %v", status, body)
	}
	if n := e.scalarInt(t, `SELECT COUNT(*) FROM battle_records WHERE user_id = $1`, uid); n != 1 {
		t.Errorf("重试路径必须只落 1 条战报，实际 %d", n)
	}
	coinAfter := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uid)

	// 3) 同一 token 再结算 → 422，钱包与战报都不再变化（不重复发奖）。
	status, body = e.post(t, "/api/v1/battle/settle", tok, e.settleBody(t, token2))
	if status != fiber.StatusUnprocessableEntity {
		t.Errorf("重复结算应 422，实际 %d %v", status, body)
	}
	if got := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uid); got != coinAfter {
		t.Errorf("重复结算不得再发奖：coin %d -> %d", coinAfter, got)
	}
	if n := e.scalarInt(t, `SELECT COUNT(*) FROM battle_records WHERE user_id = $1`, uid); n != 1 {
		t.Errorf("重复结算后仍应只有 1 条战报，实际 %d", n)
	}
}
