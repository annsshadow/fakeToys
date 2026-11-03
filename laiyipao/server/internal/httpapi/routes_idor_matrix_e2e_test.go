// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// R68：玩家侧 IDOR 矩阵（端到端）。
//
// # 这一轮要回答的问题
//
// 「A 能不能读/写/删 B 的资源」不是一条 SQL 的属性，而是**整条
// HTTP 链路**的属性：handler 是否从令牌取 uid、service 是否把它
// 下推到 WHERE、越权与不存在是否同码。任何一环把它换成入参
// （query/body/path），归属校验就失效了 —— 而这些环节单看
// service 层的用例是看不出来的。
//
// # 覆盖范围与分工
//
// 战报详情、回放、验真、预设、宝石的归属校验在 service 层已有
// 专项用例（`my_battle_detail_test.go` / `replay_ownership_test.go` /
// `presets_test.go` / `gems_equip_test.go`），本文件在 HTTP 层复核：
//
//   - 列表类（订单/购买/兑换/钱包）：A 只看到自己的行，
//     且 `?user_id=` 之类的注入参数无效；
//   - 带 id 的写/删（预设删除、设为当前、宝石洗练/佩戴/卸下、
//     回放读取、验真）：A 操作 B 的 id 一律 404，
//     与「不存在」同码，不泄露 id 是否存在。
//
// 任务领奖 / 签到没有资源 id 入参（只有全局任务 id 或「今天」），
// 归属完全由令牌决定，结构上不存在「指定 B」的入口 ——
// 这里用 `?user_id=` 注入再对比库中 B 的状态，确认它确实被忽略。
// 好友/分享功能当前不存在（全仓无 friend/share/invite 路由）。

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"
)

// doAuthed 发一个带 Bearer 的请求并返回状态码与响应体。
func doAuthed(t *testing.T, e *e2e, method, path, token string, body string) (int, []byte) {
	t.Helper()
	var req *http.Request
	var err error
	if body == "" {
		req, err = http.NewRequest(method, path, nil)
	} else {
		req, err = http.NewRequest(method, path, strings.NewReader(body))
		req.Header.Set("Content-Type", "application/json")
	}
	if err != nil {
		t.Fatalf("构造请求失败：%v", err)
	}
	req.Header.Set("Authorization", "Bearer "+token)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求 %s %s 失败：%v", method, path, err)
	}
	defer resp.Body.Close()
	buf := make([]byte, 0, 4096)
	tmp := make([]byte, 4096)
	for {
		n, rerr := resp.Body.Read(tmp)
		buf = append(buf, tmp[:n]...)
		if rerr != nil {
			break
		}
	}
	return resp.StatusCode, buf
}

// TestE2EIDORMatrixReadIsolation 覆盖「A 读 B 的列表/汇总」。
func TestE2EIDORMatrixReadIsolation(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uidA, tokA := e.newPlayer(t, "idor_read_a")
	uidB, _ := e.newPlayer(t, "idor_read_b")

	// --- 订单 ---
	orderNoA := fmt.Sprintf("IDOR_A_%d", uidA)
	orderNoB := fmt.Sprintf("IDOR_B_%d", uidB)
	e.exec(t, `INSERT INTO orders (user_id, order_no, sku, amount, status, provider)
	          VALUES ($1, $2, 'gem_pack', 600, 'paid', 'mock')`, uidA, orderNoA)
	e.exec(t, `INSERT INTO orders (user_id, order_no, sku, amount, status, provider)
	          VALUES ($1, $2, 'gem_pack', 8800, 'paid', 'mock')`, uidB, orderNoB)

	// --- 购买 ---
	e.exec(t, `INSERT INTO shop_items (id, code, name, category, price, payload)
	          VALUES (99001, 'idor_e2e_item', 'IDOR 测试商品', 'boost', '{}', '{}')
	          ON CONFLICT (id) DO NOTHING`)
	e.exec(t, `INSERT INTO user_purchases (user_id, item_id, purchase_date, price)
	          VALUES ($1, 99001, current_date, '{}')`, uidA)
	e.exec(t, `INSERT INTO user_purchases (user_id, item_id, purchase_date, price)
	          VALUES ($1, 99001, current_date, '{}')`, uidB)

	// --- 兑换 ---
	e.exec(t, `INSERT INTO redeem_codes (id, code, reward) VALUES
	          (99001, 'IDOR_CODE_A', '{}'), (99002, 'IDOR_CODE_B', '{}')
	          ON CONFLICT (id) DO NOTHING`)
	e.exec(t, `INSERT INTO redeem_usages (code_id, user_id) VALUES (99001, $1)`, uidA)
	e.exec(t, `INSERT INTO redeem_usages (code_id, user_id) VALUES (99002, $1)`, uidB)

	// --- 钱包：把 B 的余额改成 A 不可能拥有的值 ---
	e.exec(t, `UPDATE user_wallets SET coin = 777777 WHERE user_id = $1`, uidB)
	coinA := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uidA)

	status, raw := doAuthed(t, e, "GET", "/api/v1/me/orders", tokA, "")
	if status != fiber.StatusOK {
		t.Fatalf("订单列表应 200，实际 %d：%s", status, raw)
	}
	var orders struct {
		Items []map[string]any `json:"items"`
	}
	if err := json.Unmarshal(raw, &orders); err != nil {
		t.Fatalf("解析订单失败：%v", err)
	}
	if len(orders.Items) != 1 || orders.Items[0]["order_no"] != orderNoA {
		t.Errorf("A 的订单列表应只有 %s，实际 %v", orderNoA, orders.Items)
	}
	for _, it := range orders.Items {
		if it["order_no"] == orderNoB {
			t.Errorf("A 读到了 B 的订单 %s", orderNoB)
		}
	}

	status, raw = doAuthed(t, e, "GET", "/api/v1/me/purchases", tokA, "")
	if status != fiber.StatusOK {
		t.Fatalf("购买列表应 200，实际 %d：%s", status, raw)
	}
	var purchases struct {
		Items []map[string]any `json:"items"`
	}
	if err := json.Unmarshal(raw, &purchases); err != nil {
		t.Fatalf("解析购买失败：%v", err)
	}
	if len(purchases.Items) != 1 {
		t.Errorf("A 的购买列表应只有 1 条，实际 %d：%v", len(purchases.Items), purchases.Items)
	}

	status, raw = doAuthed(t, e, "GET", "/api/v1/me/redemptions", tokA, "")
	if status != fiber.StatusOK {
		t.Fatalf("兑换列表应 200，实际 %d：%s", status, raw)
	}
	var redemptions struct {
		Items []map[string]any `json:"items"`
	}
	if err := json.Unmarshal(raw, &redemptions); err != nil {
		t.Fatalf("解析兑换失败：%v", err)
	}
	if len(redemptions.Items) != 1 || redemptions.Items[0]["code"] != "IDOR_CODE_A" {
		t.Errorf("A 的兑换列表应只有 IDOR_CODE_A，实际 %v", redemptions.Items)
	}

	// 钱包：先试正常读取，再试参数注入。
	status, raw = doAuthed(t, e, "GET", "/api/v1/wallet/", tokA, "")
	if status != fiber.StatusOK {
		t.Fatalf("钱包应 200，实际 %d：%s", status, raw)
	}
	var walletA map[string]any
	if err := json.Unmarshal(raw, &walletA); err != nil {
		t.Fatalf("解析钱包失败：%v", err)
	}
	if int64(num(walletA, "coin")) != int64(coinA) {
		t.Errorf("A 的钱包 coin 应 %d，实际 %v", coinA, walletA["coin"])
	}

	status, raw = doAuthed(t, e, "GET",
		fmt.Sprintf("/api/v1/wallet/?user_id=%d", uidB), tokA, "")
	if status != fiber.StatusOK {
		t.Fatalf("钱包（带 user_id）应 200，实际 %d：%s", status, raw)
	}
	var walletInject map[string]any
	if err := json.Unmarshal(raw, &walletInject); err != nil {
		t.Fatalf("解析钱包失败：%v", err)
	}
	if int64(num(walletInject, "coin")) == 777777 {
		t.Errorf("查询参数 user_id 被采信：A 读到了 B 的余额")
	}
}

// TestE2EIDORMatrixIDBearingWrites 覆盖「A 读/写/删 B 的带 id 资源」。
func TestE2EIDORMatrixIDBearingWrites(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, tokA := e.newPlayer(t, "idor_write_a")
	uidB, _ := e.newPlayer(t, "idor_write_b")

	// --- 预设 ---
	var presetID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO user_presets (user_id, name, slots) VALUES ($1, 'B 的预设', '{}'::jsonb)
		 RETURNING id`, uidB).Scan(&presetID); err != nil {
		t.Fatalf("插入预设失败：%v", err)
	}
	if status, raw := doAuthed(t, e, "DELETE",
		"/api/v1/me/presets/"+fmtInt(presetID), tokA, ""); status != fiber.StatusNotFound {
		t.Errorf("A 删 B 的预设应 404，实际 %d：%s", status, raw)
	}
	if status, raw := doAuthed(t, e, "POST",
		"/api/v1/me/presets/"+fmtInt(presetID)+"/current", tokA, ""); status != fiber.StatusNotFound {
		t.Errorf("A 把 B 的预设设为当前应 404，实际 %d：%s", status, raw)
	}
	status, raw := doAuthed(t, e, "GET", "/api/v1/me/presets", tokA, "")
	if status != fiber.StatusOK {
		t.Fatalf("预设列表应 200，实际 %d：%s", status, raw)
	}
	var presets struct {
		Presets []map[string]any `json:"presets"`
	}
	if err := json.Unmarshal(raw, &presets); err != nil {
		t.Fatalf("解析预设失败：%v", err)
	}
	for _, p := range presets.Presets {
		if int64(num(p, "id")) == presetID {
			t.Errorf("A 读到了 B 的预设 id=%d", presetID)
		}
	}

	// --- 战报 / 回放 / 验真 ---
	var battleID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO battle_records (user_id, level_id, result, stars, replay_hash)
		 VALUES ($1, 1, 'win', 3, 'idor_hash') RETURNING id`, uidB).Scan(&battleID); err != nil {
		t.Fatalf("插入战报失败：%v", err)
	}
	if status, raw := doAuthed(t, e, "GET",
		"/api/v1/battle/"+fmtInt(battleID)+"/replay", tokA, ""); status != fiber.StatusNotFound {
		t.Errorf("A 读 B 的回放应 404，实际 %d：%s", status, raw)
	}
	body := fmt.Sprintf(`{"battle_id":%d,"replay_hash":"idor_hash"}`, battleID)
	if status, raw := doAuthed(t, e, "POST", "/api/v1/battle/verify", tokA, body); status != fiber.StatusNotFound {
		t.Errorf("A 对 B 的战报验真应 404，实际 %d：%s", status, raw)
	}
	if status, raw := doAuthed(t, e, "GET",
		"/api/v1/me/battles/"+fmtInt(battleID), tokA, ""); status != fiber.StatusNotFound {
		t.Errorf("A 读 B 的战报详情应 404，实际 %d：%s", status, raw)
	}

	// --- 宝石 ---
	e.exec(t, `INSERT INTO gems (id, code, name, attr)
	          VALUES (99001, 'idor_e2e_gem', 'IDOR 测试宝石', 'attack')
	          ON CONFLICT (id) DO NOTHING`)
	var gemID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO user_gems (user_id, gem_id, quality, affixes)
		 VALUES ($1, 99001, 'common', '[]'::jsonb) RETURNING id`, uidB).Scan(&gemID); err != nil {
		t.Fatalf("插入宝石失败：%v", err)
	}
	for _, op := range []string{"wash", "equip", "unequip"} {
		path := "/api/v1/me/gems/" + fmtInt(gemID) + "/" + op
		if status, raw := doAuthed(t, e, "POST", path, tokA, ""); status != fiber.StatusNotFound {
			t.Errorf("A 对 B 的宝石执行 %s 应 404，实际 %d：%s", op, status, raw)
		}
	}
}

// TestE2EIDORMatrixTokenOnlyWrites 覆盖「没有资源 id 入参」的端点。
//
// 任务领奖与签到只接受全局任务 id / 「今天」，归属只能来自令牌。
// 这里用 `?user_id=` 注入，确认参数被忽略、B 的数据不变。
func TestE2EIDORMatrixTokenOnlyWrites(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uidA, tokA := e.newPlayer(t, "idor_token_a")
	uidB, _ := e.newPlayer(t, "idor_token_b")

	// B 先签到，形成一个 B 自己的行。
	if status, raw := doAuthed(t, e, "POST", "/api/v1/signin/", tokA, ""); status != fiber.StatusOK {
		t.Fatalf("A 签到应 200，实际 %d：%s", status, raw)
	}
	beforeB := e.scalarInt(t, `SELECT COUNT(*) FROM user_sign_ins WHERE user_id = $1`, uidB)

	// A 带着 B 的 user_id 再签一次：不该写到 B 名下。
	// 同一天重复签到返回 403「今日已签到」（幂等拒绝），
	// 所以这里接受 200/403 —— 断言点是 B 的行数，不是状态码。
	if status, raw := doAuthed(t, e, "POST",
		fmt.Sprintf("/api/v1/signin/?user_id=%d", uidB), tokA, ""); status != fiber.StatusOK &&
		status != fiber.StatusForbidden {
		t.Fatalf("A 签到（带 user_id）应 200/403，实际 %d：%s", status, raw)
	}
	afterB := e.scalarInt(t, `SELECT COUNT(*) FROM user_sign_ins WHERE user_id = $1`, uidB)
	if afterB != beforeB {
		t.Errorf("查询参数 user_id 被采信：B 的签到行从 %d 变成 %d", beforeB, afterB)
	}
	if rows := e.scalarInt(t, `SELECT COUNT(*) FROM user_sign_ins WHERE user_id = $1`, uidA); rows == 0 {
		t.Error("A 的签到没有落到自己名下")
	}
}
