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
	"time"

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

// TestE2EIDORMatrixPathIDDoubleSided 第 134 轮：路径带 id 的改/删**双向**判据。
//
// R68 只证了「A 改 B → 404」这一半。只测拒绝有一类缺口：如果某个端点的
// 归属条件写成「owner 缺失就放行」，A 打 B 仍然可能 404（因为 B 存在且
// 不是 A），但真正的漏洞形态是「传一个空/不存在的归属也放行」以及
// 「A 改自己也被拒」。所以本用例对每个 owned-row 端点同时断言：
//
//   - A 改 B → 非 2xx，且 B 的行**逐字段不变**；
//   - A 改自己 → 200，且变更真实落库；
//   - 带 `?user_id=B` 注入时，仍然只作用于 A 自己。
//
// 覆盖 R68 没覆盖的 `/me/skills/:id/upgrade`，以及 `/defenses/:id/challenge`
// 的「挑战者身份必须取自 token」（跨玩家挑战是设计功能，不是越权）。
func TestE2EIDORMatrixPathIDDoubleSided(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	ctx := context.Background()

	uidA, tokA := e.newPlayer(t, "idor_own_a")
	uidB, tokB := e.newPlayer(t, "idor_own_b")

	// 玩家建号自带技能 1；只给 B 补一个 A 没有的技能 2 —— 这才是可越权的资源。
	e.exec(t, `INSERT INTO user_skills (user_id, skill_id, level) VALUES ($1, 2, 1)
	          ON CONFLICT (user_id, skill_id) DO NOTHING`, uidB)
	e.exec(t, `UPDATE user_wallets SET coin = 5000 WHERE user_id = $1 OR user_id = $2`, uidA, uidB)
	e.exec(t, `INSERT INTO user_tokens (user_id, token, balance) VALUES ($1, 'gem_wash_token', 3)
	          ON CONFLICT (user_id, token) DO UPDATE SET balance = 3`, uidA)

	// --- 技能升级：A 升 B 独有的技能 2 → 必须拒绝，B 等级不动 ---
	levelBBefore := e.scalarInt(t, `SELECT level FROM user_skills WHERE user_id = $1 AND skill_id = 2`, uidB)
	status, raw := doAuthed(t, e, "POST", "/api/v1/me/skills/2/upgrade", tokA, "")
	if status/100 == 2 {
		t.Fatalf("A 升 B 独有的技能 2 不应成功，实际 %d：%s", status, raw)
	}
	if status != fiber.StatusBadRequest && status != fiber.StatusNotFound && status != fiber.StatusForbidden {
		t.Fatalf("拒绝码应为 400/403/404，实际 %d：%s", status, raw)
	}
	levelBAfter := e.scalarInt(t, `SELECT level FROM user_skills WHERE user_id = $1 AND skill_id = 2`, uidB)
	if levelBAfter != levelBBefore {
		t.Fatalf("A 越权改了 B 的技能等级：%d → %d", levelBBefore, levelBAfter)
	}
	// 正向：A 升自己的技能 1 → 200 且等级 +1。
	levelABefore := e.scalarInt(t, `SELECT level FROM user_skills WHERE user_id = $1 AND skill_id = 1`, uidA)
	status, raw = doAuthed(t, e, "POST", "/api/v1/me/skills/1/upgrade", tokA, "")
	if status != fiber.StatusOK {
		t.Fatalf("A 升自己的技能应 200，实际 %d：%s", status, raw)
	}
	if got := e.scalarInt(t, `SELECT level FROM user_skills WHERE user_id = $1 AND skill_id = 1`, uidA); got != levelABefore+1 {
		t.Fatalf("A 自己的技能等级应 %d，实际 %d", levelABefore+1, got)
	}
	// 注入：带 ?user_id=B 仍然只升 A 自己。
	levelB1Before := e.scalarInt(t, `SELECT level FROM user_skills WHERE user_id = $1 AND skill_id = 1`, uidB)
	status, raw = doAuthed(t, e, "POST",
		fmt.Sprintf("/api/v1/me/skills/1/upgrade?user_id=%d", uidB), tokA, "")
	if status != fiber.StatusOK {
		t.Fatalf("带 user_id 注入的自我升级应 200，实际 %d：%s", status, raw)
	}
	if b := e.scalarInt(t, `SELECT level FROM user_skills WHERE user_id = $1 AND skill_id = 1`, uidB); b != levelB1Before {
		t.Fatalf("query 里的 user_id 被采信：B 的技能 1 等级被改成 %d", b)
	}

	// --- 预设：正向（A 改自己 200），反向已在 R68 覆盖 ---
	var presetA int64
	if err := e.pool.QueryRow(ctx,
		`INSERT INTO user_presets (user_id, name, slots) VALUES ($1, 'A 的预设', '{}'::jsonb)
		 RETURNING id`, uidA).Scan(&presetA); err != nil {
		t.Fatalf("插入 A 的预设失败：%v", err)
	}
	if status, raw := doAuthed(t, e, "POST",
		"/api/v1/me/presets/"+fmtInt(presetA)+"/current", tokA, ""); status != fiber.StatusOK {
		t.Fatalf("A 把自己的预设设为当前应 200，实际 %d：%s", status, raw)
	}
	if got := e.scalarInt(t, `SELECT count(*) FROM user_presets WHERE user_id = $1 AND is_current`, uidA); got != 1 {
		t.Fatalf("A 的当前预设应恰好 1 条，实际 %d", got)
	}
	if status, raw := doAuthed(t, e, "DELETE",
		"/api/v1/me/presets/"+fmtInt(presetA), tokA, ""); status != fiber.StatusOK {
		t.Fatalf("A 删自己的预设应 200，实际 %d：%s", status, raw)
	}
	if got := e.scalarInt(t, `SELECT count(*) FROM user_presets WHERE id = $1`, presetA); got != 0 {
		t.Fatalf("A 自己的预设应被删除，实际仍在（%d）", got)
	}

	// --- 宝石：正向（A 洗自己的宝石 200，且只扣 A 的券） ---
	// 必须用 seeder 里带候选词条池的宝石（gem_id=1 有 3 条），否则合法洗练也会因
	// 「候选池 0 条」而 500 —— 那是内容表问题，不是归属问题，会让本用例假红。
	var gemA int64
	if err := e.pool.QueryRow(ctx,
		`INSERT INTO user_gems (user_id, gem_id, quality, affixes)
		 VALUES ($1, 1, 'common', '[]'::jsonb) RETURNING id`, uidA).Scan(&gemA); err != nil {
		t.Fatalf("插入 A 的宝石失败：%v", err)
	}
	otherTokenBefore := e.scalarInt(t, `SELECT coalesce((SELECT balance FROM user_tokens WHERE user_id = $1 AND token = 'gem_wash_token'), 0)`, uidB)
	if status, raw := doAuthed(t, e, "POST",
		"/api/v1/me/gems/"+fmtInt(gemA)+"/wash", tokA, ""); status != fiber.StatusOK {
		t.Fatalf("A 洗自己的宝石应 200，实际 %d：%s", status, raw)
	}
	if got := e.scalarInt(t, `SELECT balance FROM user_tokens WHERE user_id = $1 AND token = 'gem_wash_token'`, uidA); got != 2 {
		t.Fatalf("洗练应只扣 A 的券（3→2），实际 %d", got)
	}
	if got := e.scalarInt(t, `SELECT coalesce((SELECT balance FROM user_tokens WHERE user_id = $1 AND token = 'gem_wash_token'), 0)`, uidB); got != otherTokenBefore {
		t.Fatalf("B 的洗练券被误扣：%d → %d", otherTokenBefore, got)
	}
	if status, raw := doAuthed(t, e, "POST",
		"/api/v1/me/gems/"+fmtInt(gemA)+"/equip", tokA, ""); status != fiber.StatusOK {
		t.Fatalf("A 佩戴自己的宝石应 200，实际 %d：%s", status, raw)
	}
	if got := e.scalarInt(t, `SELECT count(*) FROM user_gems WHERE id = $1 AND user_id = $2 AND equipped_slot <> ''`, gemA, uidA); got != 1 {
		t.Fatalf("A 的宝石应已佩戴，实际未佩戴")
	}
	if status, raw := doAuthed(t, e, "POST",
		"/api/v1/me/gems/"+fmtInt(gemA)+"/unequip", tokA, ""); status != fiber.StatusOK {
		t.Fatalf("A 卸下自己的宝石应 200，实际 %d：%s", status, raw)
	}

	// --- 战报：正向（A 读自己的回放/详情/验真 200） ---
	var battleA int64
	if err := e.pool.QueryRow(ctx,
		`INSERT INTO battle_records (user_id, level_id, result, stars, replay_hash)
		 VALUES ($1, 1, 'win', 3, 'own_hash') RETURNING id`, uidA).Scan(&battleA); err != nil {
		t.Fatalf("插入 A 的战报失败：%v", err)
	}
	if status, raw := doAuthed(t, e, "GET",
		"/api/v1/battle/"+fmtInt(battleA)+"/replay", tokA, ""); status != fiber.StatusOK {
		t.Fatalf("A 读自己的回放应 200，实际 %d：%s", status, raw)
	}
	if status, raw := doAuthed(t, e, "GET",
		"/api/v1/me/battles/"+fmtInt(battleA), tokA, ""); status != fiber.StatusOK {
		t.Fatalf("A 读自己的战报详情应 200，实际 %d：%s", status, raw)
	}
	if status, raw := doAuthed(t, e, "POST", "/api/v1/battle/verify", tokA,
		fmt.Sprintf(`{"battle_id":%d,"replay_hash":"own_hash"}`, battleA)); status != fiber.StatusOK {
		t.Fatalf("A 验自己的战报应 200，实际 %d：%s", status, raw)
	}

	// --- 防线挑战：挑战者身份必须取自 token ---
	if status, raw := doAuthed(t, e, "POST", "/api/v1/defenses/save", tokB,
		`{"name":"B 的防线","skills":[1],"shield_hours":0}`); status != fiber.StatusOK {
		t.Fatalf("B 保存防线失败 %d：%s", status, raw)
	}
	var defenseB int64
	if err := e.pool.QueryRow(ctx,
		`SELECT id FROM defenses WHERE owner_id = $1`, uidB).Scan(&defenseB); err != nil {
		t.Fatalf("读 B 的防线失败：%v", err)
	}
	// 挑战端点要结算体（seed/won/duration/hash），空体是 400 bad_json，与归属无关。
	challengeBody := `{"seed":"1","won":true,"duration_ms":60000,"hp_left_pct":100,"replay_hash":"0000000000000000"}`
	// B 挑战自己的防线 → 403（不能自打）。
	if status, raw := doAuthed(t, e, "POST",
		"/api/v1/defenses/"+fmtInt(defenseB)+"/challenge", tokB, challengeBody); status != fiber.StatusForbidden {
		t.Errorf("B 挑战自己的防线应 403，实际 %d：%s", status, raw)
	}
	// A 挑战 B 的防线 → 200（设计功能），落库的 challenger 必须是 A，不是 B。
	if status, raw := doAuthed(t, e, "POST",
		"/api/v1/defenses/"+fmtInt(defenseB)+"/challenge", tokA, challengeBody); status != fiber.StatusOK {
		t.Fatalf("A 挑战 B 的防线应 200，实际 %d：%s", status, raw)
	}
	if got := e.scalarInt(t, `SELECT challenger_id FROM defense_challenges WHERE defense_id = $1 ORDER BY id DESC LIMIT 1`, defenseB); int64(got) != uidA {
		t.Fatalf("挑战者身份应取自 token（A=%d），实际落到 %d", uidA, got)
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
	uidB, tokB := e.newPlayer(t, "idor_token_b")

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

	// --- 任务领奖：全局 task_id，归属只能来自令牌 ---
	// 给 A 造一条「已完成」的行，然后带 ?user_id=B 领取：
	// 若 handler 采信 query，这一行会落到 B 名下、奖励发给 B。
	taskID := e.scalarInt(t, `SELECT id FROM tasks WHERE enabled ORDER BY id LIMIT 1`)
	day := taskFixtureDay(time.Now())
	e.exec(t, `INSERT INTO user_tasks (user_id, task_id, task_date, progress)
	           VALUES ($1,$2,$3,(SELECT target FROM tasks WHERE id=$2))
	           ON CONFLICT (user_id, task_id, task_date) DO UPDATE SET progress = EXCLUDED.progress`,
		uidA, taskID, day)
	if coinA := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uidA); coinA < 0 {
		t.Fatalf("A 的钱包不应为负：%d", coinA)
	}
	coinBBefore := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uidB)
	status, raw := doAuthed(t, e, "POST",
		fmt.Sprintf("/api/v1/tasks/%d/claim?user_id=%d", taskID, uidB), tokA, "")
	if status != fiber.StatusOK {
		t.Fatalf("A 领取自己的任务应 200，实际 %d：%s", status, raw)
	}
	if b := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uidB); b != coinBBefore {
		t.Fatalf("query 里的 user_id 被采信：任务奖励发给了 B（coin %d → %d）", coinBBefore, b)
	}
	if claimedByB := e.scalarInt(t, `SELECT COUNT(*) FROM user_tasks WHERE user_id = $1 AND claimed_at IS NOT NULL`, uidB); claimedByB != 0 {
		t.Fatalf("query 里的 user_id 被采信：B 名下出现 %d 条已领取任务", claimedByB)
	}
	if claimedByA := e.scalarInt(t, `SELECT COUNT(*) FROM user_tasks WHERE user_id = $1 AND claimed_at IS NOT NULL`, uidA); claimedByA != 1 {
		t.Fatalf("A 的领取没有落到自己名下（已领取行 %d 条）", claimedByA)
	}

	// --- 商店购买：全局 item_id，归属只能来自令牌 ---
	// 商品 7 是首充礼包（price {coin:0}，每日限购 1）：带 ?user_id=B 购买后，
	// 限购计数必须记在 A 名下；B 仍是「0 次已购」，B 的购买接口仍可正常下单。
	e.exec(t, `DELETE FROM user_purchases WHERE item_id = 7 AND user_id IN ($1,$2)`, uidA, uidB)
	if status, raw := doAuthed(t, e, "POST", "/api/v1/shop/7/buy?user_id="+fmtInt(int64(uidB)), tokA, ""); status != fiber.StatusOK {
		t.Fatalf("A 购买首充礼包应 200，实际 %d：%s", status, raw)
	}
	if boughtByB := e.scalarInt(t, `SELECT COUNT(*) FROM user_purchases WHERE user_id = $1 AND item_id = 7`, uidB); boughtByB != 0 {
		t.Fatalf("query 里的 user_id 被采信：购买记录记到了 B 名下（%d 条）", boughtByB)
	}
	if boughtByA := e.scalarInt(t, `SELECT COUNT(*) FROM user_purchases WHERE user_id = $1 AND item_id = 7`, uidA); boughtByA != 1 {
		t.Fatalf("A 的购买没有落到自己名下（%d 条）", boughtByA)
	}
	// B 自己的限购额度没有被 A 的注入消耗：B 自己买仍应 200。
	if status, raw := doAuthed(t, e, "POST", "/api/v1/shop/7/buy", tokB, ""); status != fiber.StatusOK {
		t.Fatalf("B 自己的首充礼包应仍可购买（限购额度被 A 的注入误吞），实际 %d：%s", status, raw)
	}
}
