// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R135：修 `limit_per_day = 0` 商品的「同一请求连发两次 → 双扣」。
//
// # 为什么复用 `Idempotency-Key`（R86 第三套），而不是钱包行锁 / redeem_usages
//
//   - 钱包行锁 `SELECT ... FOR UPDATE` 只把同一用户的购买**串行化**：
//     顺序执行两次照样各扣一次，挡不住重试。
//   - `redeem_usages UNIQUE (code_id, user_id)` 的语义是「一个码只能用一次」；
//     套到「不限购商品」上会把「防止重复提交」变成「禁止再次购买」。
//   - `admin_idempotency_keys PRIMARY KEY (endpoint, key)` +
//     `RunIdempotent`（同事务查重/执行/记录）是唯一能区分
//     「同一次请求被重发」与「两次合法的重复购买」的机制，且已在 R86 跑通。
//
// # 契约：**可选**
//
//   带 `Idempotency-Key` → 24h 窗口内同 key + 同商品 → 重放第一次响应，业务不执行；
//   不带 → 保持 R134 现状（老客户端零影响）。
//
// 选「可选」而不是「必须」的依据（不是推断）：miniapp 的
// `src/api/client.ts` 首请求（355-358 行）与 401 重试（380-382 行）的 header
// 只有 Content-Type / Authorization，**从不发**这个头；admin 的
// `grantUser`（src/api/index.ts:329）也不发。选「必须」会让线上全部购买直接 400。

import (
	"crypto/sha256"
	"encoding/hex"
	"testing"

	"github.com/gofiber/fiber/v2"
)

// TestE2EDoubleSubmitShopBuyWithKeyChargedOnce 是 R135 的核心判据：
// 同一请求（同 key + 同商品）连发两次 → 只扣一次、只有 1 条记录、
// 第二次是重放（X-Idempotent-Replay: true，响应体逐字节相同）。
func TestE2EDoubleSubmitShopBuyWithKeyChargedOnce(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, tok := e.newPlayer(t, "double_buy_key")
	itemID := newTestShopItem(t, e, "r135_key", `{"gem":10}`, `{"coin":100}`, 0)

	gemBefore := e.scalarInt(t, `SELECT gem FROM user_wallets WHERE user_id = $1`, uid)
	coinBefore := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uid)

	path := "/api/v1/shop/" + fmtInt(itemID) + "/buy"
	key := "r135-buy-key-" + fmtInt(itemID)
	first := postIdem(t, e, path, tok, key, map[string]any{})
	second := postIdem(t, e, path, tok, key, map[string]any{})

	if first.status != fiber.StatusOK || second.status != fiber.StatusOK {
		t.Fatalf("两次购买都应 200，实际 %d / %d", first.status, second.status)
	}
	if !second.replay {
		t.Fatalf("第二次应是重放（X-Idempotent-Replay），实际不是")
	}
	if string(second.body) != string(first.body) {
		t.Fatalf("重放响应应与第一次逐字节相同\n第一次：%s\n第二次：%s", first.body, second.body)
	}

	if d := gemBefore - e.scalarInt(t, `SELECT gem FROM user_wallets WHERE user_id = $1`, uid); d != 10 {
		t.Fatalf("连发两次应只扣一次钱（-10 gem），实际 -%d", d)
	}
	if d := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uid) - coinBefore; d != 100 {
		t.Fatalf("连发两次应只发一份货（+100 coin），实际 +%d", d)
	}
	if n := e.scalarInt(t, `SELECT COUNT(*) FROM user_purchases WHERE user_id = $1 AND item_id = $2`, uid, itemID); n != 1 {
		t.Fatalf("购买记录应恰好 1 条，实际 %d", n)
	}
}

// TestE2EDoubleSubmitShopBuyKeylessKeepsLegacy 钉住**契约边界**：
// 不带 Idempotency-Key → 不做去重，连发两次仍然扣两次。
//
// 这是刻意的（与 R86 管理端同一口径）：现有小程序客户端不发这个头，
// 若把 key 改成「必须」会让线上所有购买直接 400。要做完整幂等，
// 需要客户端在每次购买意图生成一个 key（记为候选，不在本轮）。
func TestE2EDoubleSubmitShopBuyKeylessKeepsLegacy(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, tok := e.newPlayer(t, "double_buy_keyless")
	itemID := newTestShopItem(t, e, "r135_keyless", `{"gem":10}`, `{"coin":100}`, 0)

	gemBefore := e.scalarInt(t, `SELECT gem FROM user_wallets WHERE user_id = $1`, uid)
	path := "/api/v1/shop/" + fmtInt(itemID) + "/buy"
	for i := 0; i < 2; i++ {
		if status, raw := doAuthed(t, e, "POST", path, tok, ""); status != fiber.StatusOK {
			t.Fatalf("第 %d 次购买应 200，实际 %d：%s", i+1, status, raw)
		}
	}
	if d := gemBefore - e.scalarInt(t, `SELECT gem FROM user_wallets WHERE user_id = $1`, uid); d != 20 {
		t.Fatalf("缺省路径应保持现状（-20 gem），实测 -%d", d)
	}
}

// TestE2EDoubleSubmitShopBuyDifferentKeysBothSucceed 是反向用例（比拒绝侧更重要）：
// 同一商品、两次**不同的合法购买**（不同 key）必须都成功，不能被误判成重复。
func TestE2EDoubleSubmitShopBuyDifferentKeysBothSucceed(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, tok := e.newPlayer(t, "double_buy_distinct")
	itemID := newTestShopItem(t, e, "r135_distinct", `{"gem":10}`, `{"coin":100}`, 0)

	gemBefore := e.scalarInt(t, `SELECT gem FROM user_wallets WHERE user_id = $1`, uid)
	path := "/api/v1/shop/" + fmtInt(itemID) + "/buy"
	for i := 0; i < 2; i++ {
		r := postIdem(t, e, path, tok, "r135-distinct-"+fmtInt(itemID)+"-"+fmtInt(int64(i)), map[string]any{})
		if r.status != fiber.StatusOK {
			t.Fatalf("第 %d 笔合法购买应 200，实际 %d：%s", i+1, r.status, r.body)
		}
		if r.replay {
			t.Fatalf("第 %d 笔不同 key 的合法购买被误判成重放", i+1)
		}
	}
	if d := gemBefore - e.scalarInt(t, `SELECT gem FROM user_wallets WHERE user_id = $1`, uid); d != 20 {
		t.Fatalf("两笔合法购买应各扣一次（-20 gem），实际 -%d", d)
	}
	if n := e.scalarInt(t, `SELECT COUNT(*) FROM user_purchases WHERE user_id = $1 AND item_id = $2`, uid, itemID); n != 2 {
		t.Fatalf("两笔合法购买应有 2 条记录，实际 %d", n)
	}
}

// TestE2EDoubleSubmitShopBuyExpiredKeyExecutesAgain 是时间窗外的反向用例：
// 同 key 但上一次记录已经超过 24h → 这是一笔**新的合法购买**，必须重新执行，
// 不能拿窗口外的旧响应重放。
//
// 它同时守住 RunIdempotent 的过期分支：写入时的清理未必已经删掉这行，
// 若不先删过期行，INSERT 撞 PK 会把刚执行完的业务整体回滚，再回一份
// 「成功」的旧响应 —— 客户端以为买了，实际什么都没发生。
func TestE2EDoubleSubmitShopBuyExpiredKeyExecutesAgain(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, tok := e.newPlayer(t, "double_buy_expired")
	itemID := newTestShopItem(t, e, "r135_expired", `{"gem":10}`, `{"coin":100}`, 0)

	key := "r135-expired-key-" + fmtInt(itemID)
	endpoint := "player.shop.buy:" + fmtInt(uid)
	// 指纹与 handler 一致：path 里的商品 id + 原始请求体（postIdem 发的是 `{}`）。
	sum := sha256.Sum256([]byte(fmtInt(itemID) + "|{}"))
	reqHash := hex.EncodeToString(sum[:])

	if _, err := e.pool.Exec(t.Context(), `
		INSERT INTO admin_idempotency_keys (endpoint, key, request_hash, status, response, created_at)
		VALUES ($1, $2, $3, 200, $4, now() - interval '25 hours')`,
		endpoint, key, reqHash, []byte(`{"granted":{"gem":10}}`)); err != nil {
		t.Fatalf("预置过期幂等记录失败：%v", err)
	}
	t.Cleanup(func() {
		e.exec(t, `DELETE FROM admin_idempotency_keys WHERE endpoint = $1 AND key = $2`, endpoint, key)
	})

	gemBefore := e.scalarInt(t, `SELECT gem FROM user_wallets WHERE user_id = $1`, uid)
	coinBefore := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uid)

	r := postIdem(t, e, "/api/v1/shop/"+fmtInt(itemID)+"/buy", tok, key, map[string]any{})
	if r.status != fiber.StatusOK {
		t.Fatalf("窗口外的同 key 购买应 200，实际 %d：%s", r.status, r.body)
	}
	if r.replay {
		t.Fatalf("窗口外的同 key 购买是新的合法购买，不能被当成重放")
	}

	if d := gemBefore - e.scalarInt(t, `SELECT gem FROM user_wallets WHERE user_id = $1`, uid); d != 10 {
		t.Fatalf("窗口外的购买应正常扣一次钱（-10 gem），实际 -%d", d)
	}
	if d := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uid) - coinBefore; d != 100 {
		t.Fatalf("窗口外的购买应正常发一份货（+100 coin），实际 +%d", d)
	}
	if n := e.scalarInt(t, `SELECT COUNT(*) FROM user_purchases WHERE user_id = $1 AND item_id = $2`, uid, itemID); n != 1 {
		t.Fatalf("购买记录应恰好 1 条，实际 %d", n)
	}
	// 过期行应被替换成这次的新记录（仍是 1 行，且回到 24h 窗口内）。
	if n := e.scalarInt(t, `SELECT COUNT(*) FROM admin_idempotency_keys
		WHERE endpoint = $1 AND key = $2 AND created_at > now() - interval '1 hour'`,
		endpoint, key); n != 1 {
		t.Fatalf("幂等记录应刷新为本次请求（窗口内恰好 1 行），实际 %d", n)
	}
}

// TestE2EDoubleSubmitShopBuySameKeyDifferentItemConflict 守住「同 key 换请求」：
// 同一个 key 拿去买另一个商品 → 409 idempotency_conflict，绝不重放旧响应。
func TestE2EDoubleSubmitShopBuySameKeyDifferentItemConflict(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, tok := e.newPlayer(t, "double_buy_conflict")
	itemA := newTestShopItem(t, e, "r135_conflict_a", `{"gem":10}`, `{"coin":100}`, 0)
	itemB := newTestShopItem(t, e, "r135_conflict_b", `{"gem":10}`, `{"coin":100}`, 0)

	key := "r135-conflict-key"
	if r := postIdem(t, e, "/api/v1/shop/"+fmtInt(itemA)+"/buy", tok, key, map[string]any{}); r.status != fiber.StatusOK {
		t.Fatalf("第一次购买应 200，实际 %d：%s", r.status, r.body)
	}
	r := postIdem(t, e, "/api/v1/shop/"+fmtInt(itemB)+"/buy", tok, key, map[string]any{})
	if r.status != fiber.StatusConflict {
		t.Fatalf("同 key 换商品应 409，实际 %d：%s", r.status, r.body)
	}
	if r.replay {
		t.Fatalf("409 不能被标成重放")
	}
}
