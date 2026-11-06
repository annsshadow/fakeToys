// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// security_gate_test.go — 安全回归门禁（Go 侧）。
//
// # 这是什么
//
// 与 `oa4rust/backend/tests/security_regression.rs` 同构的 Go 侧门禁：
// 独立 scratch 库、一条命令跑、PASS/LEAK/UNVERIFIED 三态、棘轮基线。
// 输出逐条 `[状态] <id>`，非 PASS 进入 `security_gate_baseline.txt`。
//
// # 三态口径（与 Rust 侧一致）
//
//   - PASS：明确拒绝（401/403/404/409）且受害者侧零变化；
//     或业务控制成立（限购第 2 次 403、幂等重放只执行一次）。
//   - LEAK：2xx 且读到受害者数据；或越权写确实改了受害者行；
//     或同一请求重复执行产生了**可观测的第二份副作用**。
//   - UNVERIFIED：5xx、429、种子缺失、2xx 但无可观测副作用、单条超时。
//     UNVERIFIED **不算 PASS**，进入基线。
//
// # 超时
//
// 每条用例（每个方向）单独 5s；整轮预算 300s。
// 超时一律 UNVERIFIED(TIMEOUT)——FAIL 只给「确实越权/确实重复执行」。
//
// # 三向（低权限 + 高权限）
//
// 所有有归属的端点都跑 fwd / rev / adm 三个身份：
//   fwd：A（低权限玩家）用 B 的资源 id / user_id；
//   rev：B（低权限玩家）用 A 的资源 id / user_id；
//   adm：管理员令牌用 B 的资源 id —— 验证「归属谓词里偷偷加 is_admin
//        分支」不会被两向用户身份掩盖。
// 任一方 LEAK → 整条 LEAK；否则任一方 UNVERIFIED → UNVERIFIED；
// 三向都拒绝才 PASS。
//
// ⚠️ adm 方向在 Go 侧当前**必然**被 requireUser 拒绝（ResolveUser 只认
// claims.Kind == "user"），所以它现在恒为 PASS。它不是假守卫：它守的是
// 「管理员令牌能不能混进玩家路由」这条回归线（一旦 ResolveUser 放宽
// kind 判定，adm 会立刻打到受害者数据并判 LEAK）。真正的 admin 分支
// 越权在 Go 侧结构上不可能发生，因为玩家 handler 只从令牌取 user_id，
// 没有任何 isAdmin 入参。
//
// # 基线
//
// `security_gate_baseline.txt` 与 Rust 的 `security_regression_baseline.txt`
// **分文件**：修复线按 crate 分批修，各自只 diff 自己那份。
// 首次生成 `SECURITY_GATE_UPDATE_BASELINE=1`；之后只允许减行。
//
// 基线内的 UNVERIFIED 若恶化成 LEAK，会打 `[DRIFT-TO-LEAK]` 并判红；
// 反过来 LEAK → UNVERIFIED 只打 `[STATE-DRIFT]`，不判红（属于变好）。
//
// # 已知盲区（刻意不补成假守卫）
//
//  1. Rust 侧基线仍只存 id，不存状态；`UNVERIFIED → LEAK` 不会判红。
//     代价：改 `load_baseline` 为 map + 跑一次完整 cargo 门禁，约几分钟。
//     本轮只在 Go 侧补上，Rust 侧留作已知盲区。
//  2. Rust 侧 144 条 idor 用例没有 adm 方向；admin 在 OA 多数端点是
//     **合法**读者，一刀切跑 adm 会把「管理员本就该看」误判成 LEAK，
//     需要逐端点策略，代价高于本轮预算。
//  3. 分页只测 offset=20 这一页（当前只有 `/me/battles` 支持 offset）；
//     更深的页与 keyset/cursor 分页未测。端点没有 cursor 参数，暂不虚构用例。
//  4. 固定 limit=20 且无 offset 的列表（orders / purchases / redemptions）
//     无法翻页，也就不存在「第二页绕过」；它们的行数上限本身就是断言前提。

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http/httptest"
	"os"
	"sort"
	"strconv"
	"strings"
	"sync/atomic"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

const (
	gateCaseTimeout  = 5 * time.Second
	gateRunBudget    = 300 * time.Second
	gateBaselineName = "security_gate_baseline.txt"

	gatePass       = "PASS"
	gateLeak       = "LEAK"
	gateUnverified = "UNVERIFIED"
)

// gateOutcome 是一条用例（或一个方向）的判定。
type gateOutcome struct {
	state  string
	detail string
}

func gateOK(detail string) gateOutcome      { return gateOutcome{gatePass, detail} }
func gateBad(detail string) gateOutcome     { return gateOutcome{gateLeak, detail} }
func gateUnknown(detail string) gateOutcome { return gateOutcome{gateUnverified, detail} }

// gateHTTP 是一次请求的完整观察面（状态码 / 原始字节 / 幂等重放标记）。
type gateHTTP struct {
	status int
	body   []byte
	replay bool
	err    error
}

// gateCtx 保存一次门禁运行的全部夹具与计数器。
type gateCtx struct {
	t   *testing.T
	e   *e2e
	seq int64

	adminID  int64
	adminTok string

	uidA int64
	tokA string
	uidB int64
	tokB string

	// uidS/tokS 只给商城用例：避免商店购买记录污染 IDOR 的购买列表断言。
	uidS int64
	tokS string
	// uidG 只给发币幂等用例。
	uidG int64

	// uidP1/uidP2 只给分页用例：分页断言的偏移量依赖「两人各自的战报行数」，
	// 若与 battle-replay 等用例共用 A/B，前面的用例会插入新行把偏移量顶歪。
	uidP1 int64
	tokP1 string
	uidP2 int64
	tokP2 string
}

// TestSecurityGate 是 Go 侧唯一入口。
func TestSecurityGate(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	g := &gateCtx{t: t, e: e}
	g.uidA, g.tokA = e.newPlayer(t, "gate_a")
	g.uidB, g.tokB = e.newPlayer(t, "gate_b")
	g.uidS, g.tokS = e.newPlayer(t, "gate_shop")
	g.uidG, _ = e.newPlayer(t, "gate_grant")
	g.uidP1, g.tokP1 = e.newPlayer(t, "gate_page_a")
	g.uidP2, g.tokP2 = e.newPlayer(t, "gate_page_b")
	g.adminID, g.adminTok = e.newAdmin(t, "gate")

	cases := []struct {
		id  string
		dir string // both | fwd-only
		run func() gateOutcome
	}{
		// --- A. 商城扣款 ---
		{"go|shop|limit0-double-deduct", "fwd-only", g.caseShopLimit0},
		{"go|shop|limit1-control", "fwd-only", g.caseShopLimit1},

		// --- B. 玩家 path/query IDOR（全部双向） ---
		{"go|idor|orders-list", "both", g.caseOrdersList},
		{"go|idor|purchases-list", "both", g.casePurchasesList},
		{"go|idor|redemptions-list", "both", g.caseRedemptionsList},
		{"go|idor|wallet-inject", "both", g.caseWalletInject},
		{"go|idor|presets-list", "both", g.casePresetsList},
		{"go|idor|presets-delete", "both", g.casePresetsDelete},
		{"go|idor|presets-current", "both", g.casePresetsCurrent},
		{"go|idor|battles-page2", "both", g.caseBattlesPage2},
		{"go|idor|battles-list", "both", g.caseBattlesList},
		{"go|idor|gems-list", "both", g.caseGemsList},
		{"go|write|self-roundtrip", "fwd-only", g.caseWriteSelfRoundtrip},
		{"go|idor|battle-replay", "both", g.caseBattleReplay},
		{"go|idor|battle-verify", "both", g.caseBattleVerify},
		{"go|idor|battle-detail", "both", g.caseBattleDetail},
		{"go|idor|gems-wash", "both", func() gateOutcome { return g.caseGem("wash") }},
		{"go|idor|gems-equip", "both", func() gateOutcome { return g.caseGem("equip") }},
		{"go|idor|gems-unequip", "both", func() gateOutcome { return g.caseGem("unequip") }},
		{"go|idor|signin-inject", "both", g.caseSigninInject},

		// --- C. 管理端幂等 ---
		{"go|admin|idem|grant", "fwd-only", g.caseIdemGrant},
		{"go|admin|idem|announcements", "fwd-only", g.caseIdemAnnouncement},
		{"go|admin|idem|redeem-codes-batch", "fwd-only", g.caseIdemBatch},
		{"go|admin|idem|redeem-codes-create", "fwd-only", g.caseIdemRedeemCreate},
		{"go|admin|idem|battles-verify", "fwd-only", g.caseIdemBattleVerify},
	}

	start := time.Now()
	var nPass, nLeak, nUnv, nBoth, nFwdOnly int
	results := make([]struct {
		id    string
		dir   string
		state string
	}, 0, len(cases))
	nonpass := map[string]string{}

	for _, c := range cases {
		if c.dir == "both" {
			nBoth++
		} else {
			nFwdOnly++
		}
		res := c.run()
		results = append(results, struct {
			id    string
			dir   string
			state string
		}{c.id, c.dir, res.state})

		switch res.state {
		case gatePass:
			nPass++
		case gateLeak:
			nLeak++
		default:
			nUnv++
		}
		if res.state != gatePass {
			nonpass[c.id] = res.state
		}
		fmt.Printf("[%s] %s", res.state, c.id)
		if res.detail != "" {
			fmt.Printf("\t%s", res.detail)
		}
		fmt.Println()
	}

	elapsed := time.Since(start)
	fmt.Printf("[SUMMARY] cases=%d pass=%d leak=%d unverified=%d both=%d fwd-only=%d elapsed=%s\n",
		len(cases), nPass, nLeak, nUnv, nBoth, nFwdOnly, elapsed.Round(time.Millisecond))

	// 整轮预算：超了整轮红。单条超时只影响该条（UNVERIFIED），不拖垮整轮。
	if elapsed > gateRunBudget {
		t.Fatalf("RUN-BUDGET-EXCEEDED: %s > %s", elapsed, gateRunBudget)
	}

	// --- 棘轮基线 ---
	if os.Getenv("SECURITY_GATE_UPDATE_BASELINE") == "1" {
		if err := saveGateBaseline(results, nonpass); err != nil {
			t.Fatalf("重写 Go 基线失败：%v", err)
		}
		fmt.Printf("[BASELINE] 已重写 %s：%d 条非 PASS\n", gateBaselineName, len(nonpass))
		return
	}

	base, err := loadGateBaseline()
	if err != nil {
		t.Fatalf("读 Go 基线失败：%v", err)
	}
	var newFail, stale, drift, driftToLeak []string
	for id := range nonpass {
		if _, ok := base[id]; !ok {
			newFail = append(newFail, id)
		}
	}
	for id, st := range base {
		if _, ok := nonpass[id]; !ok {
			stale = append(stale, id)
			continue
		}
		if nonpass[id] != st {
			d := fmt.Sprintf("%s %s -> %s", id, st, nonpass[id])
			drift = append(drift, d)
			// UNVERIFIED/其它 → LEAK 属于**恶化**，不能只打印了事：
			// 否则一条早已躺在基线里的「未验证」用例转成真越权时，门禁仍是绿的。
			if nonpass[id] == gateLeak {
				driftToLeak = append(driftToLeak, d)
			}
		}
	}
	sort.Strings(newFail)
	sort.Strings(stale)
	sort.Strings(drift)
	sort.Strings(driftToLeak)
	for _, id := range newFail {
		fmt.Printf("[NEW-FAIL] %s\t%s\n", id, nonpass[id])
	}
	for _, id := range stale {
		fmt.Printf("[STALE-BASELINE] %s\t现在 PASS，基线仍有记录\n", id)
	}
	for _, d := range drift {
		fmt.Printf("[STATE-DRIFT] %s\n", d)
	}
	for _, d := range driftToLeak {
		fmt.Printf("[DRIFT-TO-LEAK] %s\n", d)
	}

	if len(newFail) > 0 {
		t.Fatalf("NEW-FAIL: %d 条新增非 PASS（%s）", len(newFail), strings.Join(newFail, ", "))
	}
	if len(stale) > 0 {
		t.Fatalf("STALE-BASELINE: %d 条已转 PASS 但基线未减（%s）", len(stale), strings.Join(stale, ", "))
	}
	if len(driftToLeak) > 0 {
		t.Fatalf("DRIFT-TO-LEAK: %d 条基线内的用例恶化成 LEAK（%s）", len(driftToLeak),
			strings.Join(driftToLeak, ", "))
	}
}

// --- 身份与方向 ---

func (g *gateCtx) attackerUID(side string) int64 {
	switch side {
	case "fwd":
		return g.uidA
	case "adm":
		return g.adminID
	default:
		return g.uidB
	}
}

func (g *gateCtx) attackerTok(side string) string {
	switch side {
	case "fwd":
		return g.tokA
	case "adm":
		return g.adminTok
	default:
		return g.tokB
	}
}

// victimUID：adm 方向固定打 B 的资源（管理员令牌去读/写一个普通玩家）。
func (g *gateCtx) victimUID(side string) int64 {
	if side == "rev" {
		return g.uidA
	}
	return g.uidB
}

func (g *gateCtx) victimTok(side string) string {
	if side == "rev" {
		return g.tokA
	}
	return g.tokB
}

// tri 跑 fwd / rev / adm 三个方向并聚合：任一 LEAK → LEAK；
// 否则任一 UNVERIFIED → UNVERIFIED；三向都拒绝才 PASS。
func (g *gateCtx) tri(id string, fn func(side string) gateOutcome) gateOutcome {
	fwd := g.within(func() gateOutcome { return fn("fwd") })
	rev := g.within(func() gateOutcome { return fn("rev") })
	adm := g.within(func() gateOutcome { return fn("adm") })
	return g.combine(id, fwd, rev, adm)
}

func (g *gateCtx) combine(id string, outs ...gateOutcome) gateOutcome {
	labels := []string{"fwd", "rev", "adm"}
	parts := make([]string, 0, len(outs))
	anyLeak, anyUnv := false, false
	for i, o := range outs {
		label := "side"
		if i < len(labels) {
			label = labels[i]
		}
		parts = append(parts, fmt.Sprintf("%s=%s(%s)", label, o.state, o.detail))
		if o.state == gateLeak {
			anyLeak = true
		}
		if o.state == gateUnverified {
			anyUnv = true
		}
	}
	detail := strings.Join(parts, " ")
	switch {
	case anyLeak:
		return gateOutcome{gateLeak, detail}
	case anyUnv:
		return gateOutcome{gateUnverified, detail}
	default:
		return gateOutcome{gatePass, detail}
	}
}

// admRejected 把「管理员令牌在玩家路由被拒」判成 PASS。
//
// 只在 adm 方向生效；fwd/rev 的 401 不做这个豁免 —— 那说明夹具令牌坏了，
// 应当落 UNVERIFIED 而不是被当成「已拒绝」。
func (g *gateCtx) admRejected(side string, status int) (gateOutcome, bool) {
	if side != "adm" {
		return gateOutcome{}, false
	}
	switch status {
	case fiber.StatusUnauthorized, fiber.StatusForbidden, fiber.StatusNotFound:
		return gateOK(fmt.Sprintf("管理员令牌被拒 status=%d", status)), true
	}
	return gateOutcome{}, false
}

// within 是单条用例的超时闸门：5s 内不返回 → UNVERIFIED(TIMEOUT)。
// 同时兜住 panic（夹具写错不该让整轮门禁崩掉）。
func (g *gateCtx) within(fn func() gateOutcome) gateOutcome {
	ch := make(chan gateOutcome, 1)
	go func() {
		defer func() {
			if r := recover(); r != nil {
				ch <- gateUnknown(fmt.Sprintf("PANIC:%v", r))
			}
		}()
		ch <- fn()
	}()
	select {
	case r := <-ch:
		return r
	case <-time.After(gateCaseTimeout):
		return gateUnknown("TIMEOUT")
	}
}

// --- 请求 / 数据库 ---

func (g *gateCtx) req(method, path, token string, body any, headers map[string]string) gateHTTP {
	var rd io.Reader
	if body != nil {
		raw, err := json.Marshal(body)
		if err != nil {
			return gateHTTP{err: err}
		}
		rd = bytes.NewReader(raw)
	}
	req := httptest.NewRequest(method, path, rd)
	req.Header.Set("Content-Type", "application/json")
	if token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	for k, v := range headers {
		req.Header.Set(k, v)
	}
	resp, err := g.e.app.Test(req, int(gateCaseTimeout/time.Millisecond))
	if err != nil {
		return gateHTTP{err: err}
	}
	defer resp.Body.Close()
	raw, err := io.ReadAll(resp.Body)
	if err != nil {
		return gateHTTP{err: err}
	}
	return gateHTTP{
		status: resp.StatusCode,
		body:   raw,
		replay: resp.Header.Get("X-Idempotent-Replay") == "true",
	}
}

func (g *gateCtx) exec(q string, args ...any) error {
	ctx, cancel := context.WithTimeout(context.Background(), gateCaseTimeout)
	defer cancel()
	_, err := g.e.pool.Exec(ctx, q, args...)
	return err
}

func (g *gateCtx) count(q string, args ...any) (int, error) {
	ctx, cancel := context.WithTimeout(context.Background(), gateCaseTimeout)
	defer cancel()
	var n int
	err := g.e.pool.QueryRow(ctx, q, args...).Scan(&n)
	return n, err
}

func (g *gateCtx) scalar(q string, args ...any) (int64, error) {
	ctx, cancel := context.WithTimeout(context.Background(), gateCaseTimeout)
	defer cancel()
	var n int64
	err := g.e.pool.QueryRow(ctx, q, args...).Scan(&n)
	return n, err
}

func (g *gateCtx) countSum(q string, args ...any) (int, int64, error) {
	ctx, cancel := context.WithTimeout(context.Background(), gateCaseTimeout)
	defer cancel()
	var n int
	var sum int64
	err := g.e.pool.QueryRow(ctx, q, args...).Scan(&n, &sum)
	return n, sum, err
}

func (g *gateCtx) uniq(prefix string) string {
	n := atomic.AddInt64(&g.seq, 1)
	return fmt.Sprintf("%s_%d_%d", prefix, time.Now().UnixNano(), n)
}

// --- 商城扣款 ---

// caseShopLimit0：limit_per_day=0（不限购）商品连买两次。
// 断言点不是「买到了」，而是「账目恰好两笔」：coin -200、购买行 2、扣款流水 2 笔。
func (g *gateCtx) caseShopLimit0() gateOutcome {
	return g.within(func() gateOutcome {
		const itemID = int64(991001)
		code := g.uniq("gate_limit0")
		if err := g.exec(`INSERT INTO shop_items (id, code, name, category, price, payload, limit_per_day)
			VALUES ($1,$2,$3,'boost','{"coin":100}'::jsonb,'{"energy":1}'::jsonb,0)`,
			itemID, code, code); err != nil {
			return gateUnknown("SEED-ERR: " + err.Error())
		}
		if err := g.exec(`UPDATE user_wallets SET coin = 1000 WHERE user_id = $1`, g.uidS); err != nil {
			return gateUnknown("SEED-ERR: " + err.Error())
		}
		for i := 0; i < 2; i++ {
			r := g.req("POST", "/api/v1/shop/"+strconv.FormatInt(itemID, 10)+"/buy", g.tokS, nil, nil)
			if r.err != nil {
				return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
			}
			if r.status != fiber.StatusOK {
				return gateUnknown(fmt.Sprintf("第%d次未成交 status=%d body=%s", i+1, r.status, r.body))
			}
		}
		coin, err := g.scalar(`SELECT coin FROM user_wallets WHERE user_id = $1`, g.uidS)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if coin != 800 {
			return gateBad(fmt.Sprintf("coin=%d 期望 800（双扣或漏扣）", coin))
		}
		n, err := g.count(`SELECT COUNT(*) FROM user_purchases WHERE user_id = $1 AND item_id = $2`, g.uidS, itemID)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if n != 2 {
			return gateBad(fmt.Sprintf("user_purchases=%d 期望 2", n))
		}
		flows, sum, err := g.countSum(`SELECT COUNT(*), COALESCE(SUM(delta),0) FROM wallet_flows
			WHERE user_id = $1 AND currency = 'coin' AND reason = 'shop_buy'`, g.uidS)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if flows != 2 || sum != -200 {
			return gateBad(fmt.Sprintf("coin 流水=%d 笔 SUM=%d，期望 2 笔 / -200", flows, sum))
		}
		return gateOK("coin=800 purchases=2 coin_flows=2 sum=-200")
	})
}

// caseShopLimit1：限购 1 的对照。第 2 次必须 403 且只扣一次。
func (g *gateCtx) caseShopLimit1() gateOutcome {
	return g.within(func() gateOutcome {
		const itemID = int64(991002)
		code := g.uniq("gate_limit1")
		if err := g.exec(`INSERT INTO shop_items (id, code, name, category, price, payload, limit_per_day)
			VALUES ($1,$2,$3,'boost','{"coin":100}'::jsonb,'{"energy":1}'::jsonb,1)`,
			itemID, code, code); err != nil {
			return gateUnknown("SEED-ERR: " + err.Error())
		}
		if err := g.exec(`UPDATE user_wallets SET coin = 1000 WHERE user_id = $1`, g.uidS); err != nil {
			return gateUnknown("SEED-ERR: " + err.Error())
		}
		first := g.req("POST", "/api/v1/shop/"+strconv.FormatInt(itemID, 10)+"/buy", g.tokS, nil, nil)
		if first.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + first.err.Error())
		}
		if first.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("第1次未成交 status=%d body=%s", first.status, first.body))
		}
		second := g.req("POST", "/api/v1/shop/"+strconv.FormatInt(itemID, 10)+"/buy", g.tokS, nil, nil)
		if second.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + second.err.Error())
		}
		if second.status == fiber.StatusOK {
			return gateBad("限购 1 的第 2 次仍 200")
		}
		if second.status >= 500 {
			return gateUnknown(fmt.Sprintf("第2次 status=%d（5xx）", second.status))
		}
		if second.status != fiber.StatusForbidden {
			return gateUnknown(fmt.Sprintf("第2次 status=%d，期望 403", second.status))
		}
		coin, err := g.scalar(`SELECT coin FROM user_wallets WHERE user_id = $1`, g.uidS)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if coin != 900 {
			return gateBad(fmt.Sprintf("coin=%d 期望 900（多扣/少扣）", coin))
		}
		n, err := g.count(`SELECT COUNT(*) FROM user_purchases WHERE user_id = $1 AND item_id = $2`, g.uidS, itemID)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if n != 1 {
			return gateBad(fmt.Sprintf("user_purchases=%d 期望 1", n))
		}
		return gateOK("第2次 403、coin=900、purchases=1")
	})
}

// --- 玩家 IDOR ---

func (g *gateCtx) caseOrdersList() gateOutcome {
	orderA := g.uniq("GATE_ORDER_A")
	orderB := g.uniq("GATE_ORDER_B")
	if err := g.exec(`INSERT INTO orders (user_id, order_no, sku, amount, status, provider)
		VALUES ($1,$2,'gem_pack',600,'paid','mock')`, g.uidA, orderA); err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	if err := g.exec(`INSERT INTO orders (user_id, order_no, sku, amount, status, provider)
		VALUES ($1,$2,'gem_pack',600,'paid','mock')`, g.uidB, orderB); err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	return g.tri("go|idor|orders-list", func(side string) gateOutcome {
		r := g.req("GET", "/api/v1/me/orders", g.attackerTok(side), nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if o, ok := g.admRejected(side, r.status); ok {
			return o
		}
		if r.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
		}
		items, ok := gateArray(r.body, "items")
		if !ok {
			return gateUnknown("UNPARSEABLE: " + string(r.body))
		}
		own, victim := orderA, orderB
		if side == "rev" {
			own, victim = orderB, orderA
		}
		var sawOwn, sawVictim bool
		for _, it := range items {
			s, _ := it["order_no"].(string)
			if s == own {
				sawOwn = true
			}
			if s == victim {
				sawVictim = true
			}
		}
		if sawVictim {
			return gateBad("读到对方订单 " + victim)
		}
		if !sawOwn {
			return gateUnknown("看不到自己的订单 " + own)
		}
		return gateOK("只看到自己的订单")
	})
}

func (g *gateCtx) casePurchasesList() gateOutcome {
	const (
		itemA = int64(991003)
		itemB = int64(991004)
	)
	for _, it := range []struct {
		id   int64
		code string
	}{{itemA, g.uniq("gate_item_a")}, {itemB, g.uniq("gate_item_b")}} {
		if err := g.exec(`INSERT INTO shop_items (id, code, name, category, price, payload, limit_per_day)
			VALUES ($1,$2,$2,'boost','{}'::jsonb,'{}'::jsonb,0)`, it.id, it.code); err != nil {
			return gateUnknown("SEED-ERR: " + err.Error())
		}
	}
	if err := g.exec(`INSERT INTO user_purchases (user_id, item_id, purchase_date, price)
		VALUES ($1,$2,current_date,'{}'::jsonb)`, g.uidA, itemA); err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	if err := g.exec(`INSERT INTO user_purchases (user_id, item_id, purchase_date, price)
		VALUES ($1,$2,current_date,'{}'::jsonb)`, g.uidB, itemB); err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	return g.tri("go|idor|purchases-list", func(side string) gateOutcome {
		r := g.req("GET", "/api/v1/me/purchases", g.attackerTok(side), nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if o, ok := g.admRejected(side, r.status); ok {
			return o
		}
		if r.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
		}
		items, ok := gateArray(r.body, "items")
		if !ok {
			return gateUnknown("UNPARSEABLE: " + string(r.body))
		}
		own, victim := itemA, itemB
		if side == "rev" {
			own, victim = itemB, itemA
		}
		var sawOwn, sawVictim bool
		for _, it := range items {
			id := int64(gateNum(it, "item_id"))
			if id == own {
				sawOwn = true
			}
			if id == victim {
				sawVictim = true
			}
		}
		if sawVictim {
			return gateBad(fmt.Sprintf("读到对方购买 item_id=%d", victim))
		}
		if !sawOwn {
			return gateUnknown(fmt.Sprintf("看不到自己的购买 item_id=%d", own))
		}
		return gateOK("只看到自己的购买")
	})
}

func (g *gateCtx) caseRedemptionsList() gateOutcome {
	codeA := g.uniq("GATE_CODE_A")
	codeB := g.uniq("GATE_CODE_B")
	for i, c := range []struct {
		id   int64
		code string
		uid  int64
	}{{990101, codeA, g.uidA}, {990102, codeB, g.uidB}} {
		if err := g.exec(`INSERT INTO redeem_codes (id, code, reward) VALUES ($1,$2,'{}'::jsonb)`,
			c.id, c.code); err != nil {
			return gateUnknown("SEED-ERR: " + err.Error())
		}
		if err := g.exec(`INSERT INTO redeem_usages (code_id, user_id) VALUES ($1,$2)`,
			c.id, c.uid); err != nil {
			return gateUnknown("SEED-ERR: " + err.Error() + "（i=" + strconv.Itoa(i) + "）")
		}
	}
	return g.tri("go|idor|redemptions-list", func(side string) gateOutcome {
		r := g.req("GET", "/api/v1/me/redemptions", g.attackerTok(side), nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if o, ok := g.admRejected(side, r.status); ok {
			return o
		}
		if r.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
		}
		items, ok := gateArray(r.body, "items")
		if !ok {
			return gateUnknown("UNPARSEABLE: " + string(r.body))
		}
		own, victim := codeA, codeB
		if side == "rev" {
			own, victim = codeB, codeA
		}
		var sawOwn, sawVictim bool
		for _, it := range items {
			s, _ := it["code"].(string)
			if s == own {
				sawOwn = true
			}
			if s == victim {
				sawVictim = true
			}
		}
		if sawVictim {
			return gateBad("读到对方兑换码 " + victim)
		}
		if !sawOwn {
			return gateUnknown("看不到自己的兑换码 " + own)
		}
		return gateOK("只看到自己的兑换")
	})
}

func (g *gateCtx) caseWalletInject() gateOutcome {
	if err := g.exec(`UPDATE user_wallets SET coin = 111111 WHERE user_id = $1`, g.uidA); err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	if err := g.exec(`UPDATE user_wallets SET coin = 777777 WHERE user_id = $1`, g.uidB); err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	return g.tri("go|idor|wallet-inject", func(side string) gateOutcome {
		path := "/api/v1/wallet/?user_id=" + strconv.FormatInt(g.victimUID(side), 10)
		r := g.req("GET", path, g.attackerTok(side), nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if o, ok := g.admRejected(side, r.status); ok {
			return o
		}
		if r.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
		}
		var m map[string]any
		if err := json.Unmarshal(r.body, &m); err != nil {
			return gateUnknown("UNPARSEABLE: " + string(r.body))
		}
		own, victim := int64(111111), int64(777777)
		if side == "rev" {
			own, victim = 777777, 111111
		}
		got := gateNum(m, "coin")
		if got == victim {
			return gateBad(fmt.Sprintf("查询参数 user_id 被采信：读到对方余额 %d", victim))
		}
		if got != own {
			return gateUnknown(fmt.Sprintf("coin=%d，既非自己 %d 也非对方 %d", got, own, victim))
		}
		return gateOK(fmt.Sprintf("user_id 被忽略，仍读到自己的 %d", own))
	})
}

func (g *gateCtx) casePresetsList() gateOutcome {
	presetA, err := g.mkPreset(g.uidA, "A 的预设")
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	presetB, err := g.mkPreset(g.uidB, "B 的预设")
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	return g.tri("go|idor|presets-list", func(side string) gateOutcome {
		r := g.req("GET", "/api/v1/me/presets", g.attackerTok(side), nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if o, ok := g.admRejected(side, r.status); ok {
			return o
		}
		if r.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
		}
		items, ok := gateArray(r.body, "presets")
		if !ok {
			return gateUnknown("UNPARSEABLE: " + string(r.body))
		}
		own, victim := presetA, presetB
		if side == "rev" {
			own, victim = presetB, presetA
		}
		var sawOwn, sawVictim bool
		for _, it := range items {
			id := gateNum(it, "id")
			if id == own {
				sawOwn = true
			}
			if id == victim {
				sawVictim = true
			}
		}
		if sawVictim {
			return gateBad(fmt.Sprintf("读到对方预设 id=%d", victim))
		}
		if !sawOwn {
			return gateUnknown(fmt.Sprintf("看不到自己的预设 id=%d", own))
		}
		return gateOK("只看到自己的预设")
	})
}

func (g *gateCtx) casePresetsDelete() gateOutcome {
	presetA, err := g.mkPreset(g.uidA, "A 的预设")
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	presetB, err := g.mkPreset(g.uidB, "B 的预设")
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	return g.tri("go|idor|presets-delete", func(side string) gateOutcome {
		victim := presetB
		if side == "rev" {
			victim = presetA
		}
		r := g.req("DELETE", "/api/v1/me/presets/"+strconv.FormatInt(victim, 10), g.attackerTok(side), nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if o, ok := g.admRejected(side, r.status); ok {
			return o
		}
		if r.status >= 500 {
			return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
		}
		n, err := g.count(`SELECT COUNT(*) FROM user_presets WHERE id = $1`, victim)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if n == 0 {
			return gateBad("对方预设被删除（越权写成功）")
		}
		if r.status == fiber.StatusOK {
			return gateUnknown("2xx 但对方预设仍在（无可观测副作用）")
		}
		if r.status == fiber.StatusNotFound || r.status == fiber.StatusForbidden {
			return gateOK(fmt.Sprintf("拒绝 status=%d，对方预设仍在", r.status))
		}
		return gateUnknown(fmt.Sprintf("status=%d，期望 404", r.status))
	})
}

func (g *gateCtx) casePresetsCurrent() gateOutcome {
	presetA, err := g.mkPreset(g.uidA, "A 的当前预设")
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	presetB, err := g.mkPreset(g.uidB, "B 的当前预设")
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	return g.tri("go|idor|presets-current", func(side string) gateOutcome {
		victim := presetB
		if side == "rev" {
			victim = presetA
		}
		r := g.req("POST", "/api/v1/me/presets/"+strconv.FormatInt(victim, 10)+"/current",
			g.attackerTok(side), nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if o, ok := g.admRejected(side, r.status); ok {
			return o
		}
		if r.status >= 500 {
			return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
		}
		var isCurrent bool
		err := g.e.pool.QueryRow(context.Background(),
			`SELECT is_current FROM user_presets WHERE id = $1`, victim).Scan(&isCurrent)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if isCurrent {
			return gateBad("对方预设被设为 current（越权写成功）")
		}
		if r.status == fiber.StatusOK {
			return gateUnknown("2xx 但对方预设未变 current")
		}
		if r.status == fiber.StatusNotFound || r.status == fiber.StatusForbidden {
			return gateOK(fmt.Sprintf("拒绝 status=%d", r.status))
		}
		return gateUnknown(fmt.Sprintf("status=%d，期望 404", r.status))
	})
}

func (g *gateCtx) caseBattleReplay() gateOutcome {
	battleA, hashA, err := g.mkBattle(g.uidA, g.uniq("gate_hash_a"))
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	battleB, hashB, err := g.mkBattle(g.uidB, g.uniq("gate_hash_b"))
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	return g.tri("go|idor|battle-replay", func(side string) gateOutcome {
		victim, hash := battleB, hashB
		if side == "rev" {
			victim, hash = battleA, hashA
		}
		r := g.req("GET", "/api/v1/battle/"+strconv.FormatInt(victim, 10)+"/replay",
			g.attackerTok(side), nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if o, ok := g.admRejected(side, r.status); ok {
			return o
		}
		if r.status == fiber.StatusNotFound || r.status == fiber.StatusForbidden {
			return gateOK(fmt.Sprintf("拒绝 status=%d", r.status))
		}
		if r.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
		}
		if strings.Contains(string(r.body), hash) {
			return gateBad("读到对方回放（hash 命中 " + hash + "）")
		}
		return gateUnknown("2xx 但响应未含对方 hash")
	})
}

func (g *gateCtx) caseBattleVerify() gateOutcome {
	battleA, hashA, err := g.mkBattle(g.uidA, g.uniq("gate_hash_a"))
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	battleB, hashB, err := g.mkBattle(g.uidB, g.uniq("gate_hash_b"))
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	return g.tri("go|idor|battle-verify", func(side string) gateOutcome {
		victim, hash := battleB, hashB
		if side == "rev" {
			victim, hash = battleA, hashA
		}
		r := g.req("POST", "/api/v1/battle/verify", g.attackerTok(side),
			map[string]any{"battle_id": victim, "replay_hash": hash}, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if o, ok := g.admRejected(side, r.status); ok {
			return o
		}
		if r.status == fiber.StatusNotFound || r.status == fiber.StatusForbidden {
			return gateOK(fmt.Sprintf("拒绝 status=%d", r.status))
		}
		if r.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
		}
		var m map[string]any
		if err := json.Unmarshal(r.body, &m); err == nil {
			if matched, _ := m["matched"].(bool); matched {
				return gateBad("对对方战报验真且 matched=true")
			}
		}
		return gateUnknown("2xx 但未观测到对方数据")
	})
}

func (g *gateCtx) caseBattleDetail() gateOutcome {
	battleA, _, err := g.mkBattle(g.uidA, g.uniq("gate_hash_a"))
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	battleB, _, err := g.mkBattle(g.uidB, g.uniq("gate_hash_b"))
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	return g.tri("go|idor|battle-detail", func(side string) gateOutcome {
		victim := battleB
		if side == "rev" {
			victim = battleA
		}
		r := g.req("GET", "/api/v1/me/battles/"+strconv.FormatInt(victim, 10),
			g.attackerTok(side), nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if o, ok := g.admRejected(side, r.status); ok {
			return o
		}
		if r.status == fiber.StatusNotFound || r.status == fiber.StatusForbidden {
			return gateOK(fmt.Sprintf("拒绝 status=%d", r.status))
		}
		if r.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
		}
		return gateUnknown("2xx 但未观测到对方数据")
	})
}

// caseBattlesList：列表两路之一（第一页）。分页端点的归属过滤不能只测首页。
func (g *gateCtx) caseBattlesList() gateOutcome {
	hashA := g.uniq("gate_list_hash_a")
	hashB := g.uniq("gate_list_hash_b")
	if _, _, err := g.mkBattle(g.uidA, hashA); err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	if _, _, err := g.mkBattle(g.uidB, hashB); err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	return g.tri("go|idor|battles-list", func(side string) gateOutcome {
		r := g.req("GET", "/api/v1/me/battles?limit=20&offset=0", g.attackerTok(side), nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if o, ok := g.admRejected(side, r.status); ok {
			return o
		}
		if r.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
		}
		items, ok := gateArray(r.body, "items")
		if !ok {
			return gateUnknown("UNPARSEABLE: " + string(r.body))
		}
		own, victim := hashA, hashB
		if side == "rev" {
			own, victim = hashB, hashA
		}
		var sawOwn, sawVictim bool
		for _, it := range items {
			h, _ := it["replay_hash"].(string)
			if h == own {
				sawOwn = true
			}
			if h == victim {
				sawVictim = true
			}
		}
		if sawVictim {
			return gateBad("读到对方战报 hash=" + victim)
		}
		if !sawOwn {
			return gateUnknown("看不到自己的战报 hash=" + own)
		}
		return gateOK("第一页只看到自己的战报")
	})
}

// caseBattlesPage2：翻页绕过。
//
// 造 25+25 条交替时间的战报，使第一页与第二页各含双方各 10 条。
// 只有「归属过滤作用在全部页」才能同时满足：
//
//	第二页能看到自己的第 11..25 条，且不含对方的任何一条。
//
// 归属过滤只作用于 offset=0 的写法会让第二页暴露对方行 → 本用例 LEAK。
func (g *gateCtx) caseBattlesPage2() gateOutcome {
	const n = 25
	base := time.Now().Add(-time.Hour)
	ownHashes := make([]string, n)
	victimHashes := make([]string, n)
	for i := 0; i < n; i++ {
		ownHashes[i] = g.uniq("gate_page_own")
		victimHashes[i] = g.uniq("gate_page_victim")
		if _, _, err := g.mkBattleAt(g.uidP1, ownHashes[i], base.Add(-time.Duration(2*i)*time.Second)); err != nil {
			return gateUnknown("SEED-ERR: " + err.Error())
		}
		if _, _, err := g.mkBattleAt(g.uidP2, victimHashes[i], base.Add(-time.Duration(2*i+1)*time.Second)); err != nil {
			return gateUnknown("SEED-ERR: " + err.Error())
		}
	}
	set := func(list []string) map[string]bool {
		m := make(map[string]bool, len(list))
		for _, s := range list {
			m[s] = true
		}
		return m
	}
	ownSet := set(ownHashes)
	victimSet := set(victimHashes)

	return g.tri("go|idor|battles-page2", func(side string) gateOutcome {
		tok := g.tokP1
		// 归属过滤正确时，query 先 WHERE user_id 再 OFFSET：
		// 25 条自有行里 offset=20 只剩 5 条。若第二页丢了归属过滤，
		// 50 条全表会被 offset=20 切出 20 条，其中夹着对方 10 条 → 红。
		wantOwn := ownHashes[20:]
		wantedVictim := victimSet
		if side == "rev" {
			tok = g.tokP2
			wantOwn = victimHashes[20:]
			wantedVictim = ownSet
		}
		if side == "adm" {
			tok = g.adminTok
		}
		r := g.req("GET", "/api/v1/me/battles?limit=20&offset=20", tok, nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if o, ok := g.admRejected(side, r.status); ok {
			return o
		}
		if r.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
		}
		items, ok := gateArray(r.body, "items")
		if !ok {
			return gateUnknown("UNPARSEABLE: " + string(r.body))
		}
		seenOwn := 0
		for _, it := range items {
			h, _ := it["replay_hash"].(string)
			if wantedVictim[h] {
				return gateBad("第二页读到对方战报 hash=" + h)
			}
			if h != "" {
				for _, w := range wantOwn {
					if h == w {
						seenOwn++
						break
					}
				}
			}
		}
		if seenOwn != len(wantOwn) {
			return gateUnknown(fmt.Sprintf("第二页自己的战报 %d/%d", seenOwn, len(wantOwn)))
		}
		return gateOK(fmt.Sprintf("第二页 %d 条均为自己，无对方行", seenOwn))
	})
}

// caseGemsList：宝石列表。此前只有按 id 的写用例（wash/equip/unequip），
// 列表这一路完全没进门禁 —— 「只改 read 不改 list」的一整类盲区。
func (g *gateCtx) caseGemsList() gateOutcome {
	if err := g.exec(`INSERT INTO gems (id, code, name, attr)
		VALUES (991001,'gate_gem','Gate 测试宝石','attack') ON CONFLICT (id) DO NOTHING`); err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	gemA, err := g.mkUserGem(g.uidA)
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	gemB, err := g.mkUserGem(g.uidB)
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	return g.tri("go|idor|gems-list", func(side string) gateOutcome {
		r := g.req("GET", "/api/v1/me/gems", g.attackerTok(side), nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if o, ok := g.admRejected(side, r.status); ok {
			return o
		}
		if r.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
		}
		items, ok := gateArray(r.body, "gems")
		if !ok {
			return gateUnknown("UNPARSEABLE: " + string(r.body))
		}
		own, victim := gemA, gemB
		if side == "rev" {
			own, victim = gemB, gemA
		}
		var sawOwn, sawVictim bool
		for _, it := range items {
			id := gateNum(it, "id")
			if id == own {
				sawOwn = true
			}
			if id == victim {
				sawVictim = true
			}
		}
		if sawVictim {
			return gateBad(fmt.Sprintf("读到对方宝石 id=%d", victim))
		}
		if !sawOwn {
			return gateUnknown(fmt.Sprintf("看不到自己的宝石 id=%d", own))
		}
		return gateOK("只看到自己的宝石")
	})
}

// caseWriteSelfRoundtrip：写路径**正控制**。
//
// 「攻击者写受害者 → 受害者行未变」有两种解释：
//  1. 归属校验挡住了写；
//  2. 写接口 2xx 但根本没落库（吞错误）。
//
// 只有一条「自己写自己、必须真的落库」的用例才能把两者分开。
// 2xx 但库里没有 → UNVERIFIED（不是 LEAK，也不是 PASS）。
func (g *gateCtx) caseWriteSelfRoundtrip() gateOutcome {
	return g.within(func() gateOutcome {
		// 预设名上限 20 字，uniq 会带时间戳，这里用短前缀。
		name := fmt.Sprintf("rt%d", time.Now().UnixNano()%1_000_000_000)
		r := g.req("POST", "/api/v1/me/presets", g.tokA, map[string]any{"name": name}, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if r.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("创建 status=%d body=%s", r.status, r.body))
		}
		preset, ok := gateObj(r.body, "preset")
		if !ok {
			return gateUnknown("UNPARSEABLE: " + string(r.body))
		}
		id := gateNum(preset, "id")
		n, err := g.count(`SELECT COUNT(*) FROM user_presets WHERE id = $1 AND user_id = $2`, id, g.uidA)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if n != 1 {
			return gateUnknown(fmt.Sprintf("创建 2xx 但未落库（id=%d，行数=%d）", id, n))
		}
		d := g.req("DELETE", "/api/v1/me/presets/"+strconv.FormatInt(id, 10), g.tokA, nil, nil)
		if d.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + d.err.Error())
		}
		if d.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("删除自己的预设 status=%d body=%s", d.status, d.body))
		}
		n2, err := g.count(`SELECT COUNT(*) FROM user_presets WHERE id = $1`, id)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if n2 != 0 {
			return gateUnknown(fmt.Sprintf("删除 2xx 但行仍在（id=%d）", id))
		}
		return gateOK(fmt.Sprintf("自建 id=%d 落库、自删后归零", id))
	})
}

func (g *gateCtx) caseGem(op string) gateOutcome {
	if err := g.exec(`INSERT INTO gems (id, code, name, attr)
		VALUES (991001,'gate_gem','Gate 测试宝石','attack') ON CONFLICT (id) DO NOTHING`); err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	gemA, err := g.mkUserGem(g.uidA)
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	gemB, err := g.mkUserGem(g.uidB)
	if err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	// unequip 需要受害者先处于已佩戴状态，否则「成功」无从观测。
	if op == "unequip" {
		if err := g.exec(`UPDATE user_gems SET equipped_slot = 'weapon' WHERE id = $1`, gemA); err != nil {
			return gateUnknown("SEED-ERR: " + err.Error())
		}
		if err := g.exec(`UPDATE user_gems SET equipped_slot = 'weapon' WHERE id = $1`, gemB); err != nil {
			return gateUnknown("SEED-ERR: " + err.Error())
		}
	}
	return g.tri("go|idor|gems-"+op, func(side string) gateOutcome {
		victim := gemB
		if side == "rev" {
			victim = gemA
		}
		r := g.req("POST", "/api/v1/me/gems/"+strconv.FormatInt(victim, 10)+"/"+op,
			g.attackerTok(side), nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		if o, ok := g.admRejected(side, r.status); ok {
			return o
		}
		if r.status == fiber.StatusNotFound || r.status == fiber.StatusForbidden {
			return gateOK(fmt.Sprintf("拒绝 status=%d", r.status))
		}
		if r.status >= 500 {
			return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
		}
		if r.status == fiber.StatusOK {
			// 写到别人的宝石却返回 2xx：越权写成功。
			return gateBad("对对方宝石执行 " + op + " 返回 2xx")
		}
		return gateUnknown(fmt.Sprintf("status=%d，期望 404", r.status))
	})
}

// caseSigninInject：签到没有资源 id 入参，归属只能来自令牌。
//
// 为了让「query 参数被采信」可观测，受害者必须在注入时**尚未签到**；
// 因此 fwd 之后要重置两人的签到状态再跑 rev。
func (g *gateCtx) caseSigninInject() gateOutcome {
	if r := g.req("POST", "/api/v1/signin/", g.tokA, nil, nil); r.err != nil {
		return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
	} else if r.status != fiber.StatusOK {
		return gateUnknown(fmt.Sprintf("A 签到 status=%d body=%s", r.status, r.body))
	}
	fwd := g.within(func() gateOutcome {
		n0, err := g.count(`SELECT COUNT(*) FROM user_sign_ins WHERE user_id = $1`, g.uidB)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		r := g.req("POST", "/api/v1/signin/?user_id="+strconv.FormatInt(g.uidB, 10), g.tokA, nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		n1, err := g.count(`SELECT COUNT(*) FROM user_sign_ins WHERE user_id = $1`, g.uidB)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if n1 != n0 {
			return gateBad(fmt.Sprintf("B 的签到行 %d → %d（query user_id 被采信）", n0, n1))
		}
		if r.status == fiber.StatusOK || r.status == fiber.StatusForbidden {
			return gateOK(fmt.Sprintf("B 行数不变（%d），status=%d", n1, r.status))
		}
		return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
	})

	// 重置：A 取消签到、B 成为已签到的一方，A 变成未签到的受害者。
	if err := g.exec(`DELETE FROM user_sign_ins WHERE user_id = $1`, g.uidA); err != nil {
		return gateUnknown("SEED-ERR: " + err.Error())
	}
	if r := g.req("POST", "/api/v1/signin/", g.tokB, nil, nil); r.err != nil {
		return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
	} else if r.status != fiber.StatusOK {
		return gateUnknown(fmt.Sprintf("B 签到 status=%d body=%s", r.status, r.body))
	}
	rev := g.within(func() gateOutcome {
		n0, err := g.count(`SELECT COUNT(*) FROM user_sign_ins WHERE user_id = $1`, g.uidA)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		r := g.req("POST", "/api/v1/signin/?user_id="+strconv.FormatInt(g.uidA, 10), g.tokB, nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		n1, err := g.count(`SELECT COUNT(*) FROM user_sign_ins WHERE user_id = $1`, g.uidA)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if n1 != n0 {
			return gateBad(fmt.Sprintf("A 的签到行 %d → %d（query user_id 被采信）", n0, n1))
		}
		if r.status == fiber.StatusOK || r.status == fiber.StatusForbidden {
			return gateOK(fmt.Sprintf("A 行数不变（%d），status=%d", n1, r.status))
		}
		return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
	})

	// adm：管理员令牌带 ?user_id=B 调签到。当前 ResolveUser 拒绝 admin 令牌，
	// 所以正确行为是 401 且 B 的签到行不变；一旦令牌混用放开，这里必须红。
	adm := g.within(func() gateOutcome {
		n0, err := g.count(`SELECT COUNT(*) FROM user_sign_ins WHERE user_id = $1`, g.uidB)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		r := g.req("POST", "/api/v1/signin/?user_id="+strconv.FormatInt(g.uidB, 10), g.adminTok, nil, nil)
		if r.err != nil {
			return gateUnknown("TIMEOUT/REQ: " + r.err.Error())
		}
		n1, err := g.count(`SELECT COUNT(*) FROM user_sign_ins WHERE user_id = $1`, g.uidB)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if n1 != n0 {
			return gateBad(fmt.Sprintf("B 的签到行 %d → %d（管理员令牌被采信）", n0, n1))
		}
		if o, ok := g.admRejected("adm", r.status); ok {
			return o
		}
		return gateUnknown(fmt.Sprintf("status=%d body=%s", r.status, r.body))
	})

	return g.combine("go|idor|signin-inject", fwd, rev, adm)
}

// --- 管理端幂等 ---

func (g *gateCtx) caseIdemGrant() gateOutcome {
	return g.within(func() gateOutcome {
		uid := g.uidG
		if err := g.exec(`UPDATE user_wallets SET coin = 2000 WHERE user_id = $1`, uid); err != nil {
			return gateUnknown("SEED-ERR: " + err.Error())
		}
		key := g.uniq("gate-grant-key")
		path := "/api/v1/admin/users/" + strconv.FormatInt(uid, 10) + "/grant"
		body := map[string]any{"currency": "coin", "amount": 100}
		r1 := g.req("POST", path, g.adminTok, body, map[string]string{"Idempotency-Key": key})
		r2 := g.req("POST", path, g.adminTok, body, map[string]string{"Idempotency-Key": key})
		if r1.err != nil || r2.err != nil {
			return gateUnknown("TIMEOUT/REQ")
		}
		if r1.status != fiber.StatusOK || r2.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d/%d body=%s", r1.status, r2.status, r2.body))
		}
		if !r2.replay {
			return gateBad("第 2 次未标记 X-Idempotent-Replay")
		}
		if !bytes.Equal(r1.body, r2.body) {
			return gateBad("重放响应与第一次不是逐字节相同")
		}
		coin, err := g.scalar(`SELECT coin FROM user_wallets WHERE user_id = $1`, uid)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if coin != 2100 {
			return gateBad(fmt.Sprintf("coin=%d 期望 2100（重复发币）", coin))
		}
		audits, err := g.count(`SELECT COUNT(*) FROM admin_audit_logs
			WHERE action = 'grant_currency' AND target = $1`, fmt.Sprint(uid))
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if audits != 1 {
			return gateBad(fmt.Sprintf("审计=%d 行 期望 1（重放写了第二条）", audits))
		}
		return gateOK("重放同字节、coin +100、审计 1 行")
	})
}

func (g *gateCtx) caseIdemAnnouncement() gateOutcome {
	return g.within(func() gateOutcome {
		title := g.uniq("GATE_ANNOUNCE")
		key := g.uniq("gate-ann-key")
		body := map[string]any{"title": title, "body": "门禁测试正文", "published": true}
		r1 := g.req("POST", "/api/v1/admin/announcements", g.adminTok, body,
			map[string]string{"Idempotency-Key": key})
		r2 := g.req("POST", "/api/v1/admin/announcements", g.adminTok, body,
			map[string]string{"Idempotency-Key": key})
		if r1.err != nil || r2.err != nil {
			return gateUnknown("TIMEOUT/REQ")
		}
		if r1.status != fiber.StatusOK || r2.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d/%d body=%s", r1.status, r2.status, r2.body))
		}
		if !r2.replay {
			return gateBad("第 2 次未标记 X-Idempotent-Replay")
		}
		if !bytes.Equal(r1.body, r2.body) {
			return gateBad("重放响应与第一次不是逐字节相同")
		}
		n, err := g.count(`SELECT COUNT(*) FROM announcements WHERE title = $1`, title)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if n != 1 {
			return gateBad(fmt.Sprintf("公告=%d 行 期望 1（重复创建）", n))
		}
		return gateOK("重放同字节、公告 1 行")
	})
}

func (g *gateCtx) caseIdemBatch() gateOutcome {
	return g.within(func() gateOutcome {
		key := g.uniq("gate-batch-key")
		body := map[string]any{"count": 5, "reward": map[string]int{"coin": 1}, "max_uses": 1}
		t0 := time.Now().Add(-time.Minute)
		r1 := g.req("POST", "/api/v1/admin/redeem-codes/batch", g.adminTok, body,
			map[string]string{"Idempotency-Key": key})
		r2 := g.req("POST", "/api/v1/admin/redeem-codes/batch", g.adminTok, body,
			map[string]string{"Idempotency-Key": key})
		if r1.err != nil || r2.err != nil {
			return gateUnknown("TIMEOUT/REQ")
		}
		if r1.status != fiber.StatusOK || r2.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d/%d body=%s", r1.status, r2.status, r2.body))
		}
		if !r2.replay {
			return gateBad("第 2 次未标记 X-Idempotent-Replay")
		}
		if !bytes.Equal(r1.body, r2.body) {
			return gateBad("重放响应与第一次不是逐字节相同")
		}
		n, err := g.count(`SELECT COUNT(*) FROM redeem_codes
			WHERE max_uses = 1 AND reward = '{"coin":1}'::jsonb AND created_at >= $1`, t0)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if n != 5 {
			return gateBad(fmt.Sprintf("时间窗内码=%d 个 期望 5（重复批量生成）", n))
		}
		return gateOK("重放同字节、5 个码")
	})
}

// caseIdemRedeemCreate：单条兑换码创建**没有** Idempotency-Key 支持。
// 唯一约束让第二次变成 400 而不是重放——没有重复副作用，按三态算 PASS，
// 但 detail 明确标注「无 key，自然幂等」，避免把缺口说成已修。
func (g *gateCtx) caseIdemRedeemCreate() gateOutcome {
	return g.within(func() gateOutcome {
		code := strings.ToUpper(g.uniq("GATE_CODE"))
		key := g.uniq("gate-code-key")
		body := map[string]any{"code": code, "reward": map[string]int{"coin": 1}, "max_uses": 1}
		r1 := g.req("POST", "/api/v1/admin/redeem-codes", g.adminTok, body,
			map[string]string{"Idempotency-Key": key})
		r2 := g.req("POST", "/api/v1/admin/redeem-codes", g.adminTok, body,
			map[string]string{"Idempotency-Key": key})
		if r1.err != nil || r2.err != nil {
			return gateUnknown("TIMEOUT/REQ")
		}
		if r1.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("第1次 status=%d body=%s", r1.status, r1.body))
		}
		n, err := g.count(`SELECT COUNT(*) FROM redeem_codes WHERE code = $1`, code)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if n != 1 {
			return gateBad(fmt.Sprintf("码=%d 行 期望 1（重复创建）", n))
		}
		if r2.status == fiber.StatusOK {
			if r2.replay {
				return gateOK("第2次为重放（key 已生效）")
			}
			return gateUnknown("2xx 但非重放，且无重复副作用")
		}
		if r2.status >= 500 {
			return gateUnknown(fmt.Sprintf("第2次 status=%d（5xx）", r2.status))
		}
		return gateOK(fmt.Sprintf("无 key 支持，第二次 status=%d，唯一约束兜底 1 行", r2.status))
	})
}

// caseIdemBattleVerify：运营验真没有 Idempotency-Key，
// 但 `replay_verifications` 有 (battle_id, admin_verifier_id, actual_hash)
// 的部分唯一索引——同 key 同 body 两次只落一行。
func (g *gateCtx) caseIdemBattleVerify() gateOutcome {
	return g.within(func() gateOutcome {
		battleID, hash, err := g.mkBattle(g.uidG, g.uniq("gate_admin_hash"))
		if err != nil {
			return gateUnknown("SEED-ERR: " + err.Error())
		}
		key := g.uniq("gate-verify-key")
		path := "/api/v1/admin/battles/" + strconv.FormatInt(battleID, 10) + "/verify"
		body := map[string]any{"replay_hash": hash}
		r1 := g.req("POST", path, g.adminTok, body, map[string]string{"Idempotency-Key": key})
		r2 := g.req("POST", path, g.adminTok, body, map[string]string{"Idempotency-Key": key})
		if r1.err != nil || r2.err != nil {
			return gateUnknown("TIMEOUT/REQ")
		}
		if r1.status != fiber.StatusOK || r2.status != fiber.StatusOK {
			return gateUnknown(fmt.Sprintf("status=%d/%d body=%s", r1.status, r2.status, r2.body))
		}
		n, err := g.count(`SELECT COUNT(*) FROM replay_verifications WHERE battle_id = $1`, battleID)
		if err != nil {
			return gateUnknown("READ-ERR: " + err.Error())
		}
		if n != 1 {
			return gateBad(fmt.Sprintf("验真记录=%d 行 期望 1（重复执行）", n))
		}
		return gateOK("无 key 支持，唯一索引兜底 1 行")
	})
}

// --- 夹具 ---

func (g *gateCtx) mkPreset(uid int64, name string) (int64, error) {
	ctx, cancel := context.WithTimeout(context.Background(), gateCaseTimeout)
	defer cancel()
	var id int64
	err := g.e.pool.QueryRow(ctx,
		`INSERT INTO user_presets (user_id, name, slots) VALUES ($1,$2,'{}'::jsonb)
		 RETURNING id`, uid, name).Scan(&id)
	return id, err
}

func (g *gateCtx) mkBattle(uid int64, hash string) (int64, string, error) {
	ctx, cancel := context.WithTimeout(context.Background(), gateCaseTimeout)
	defer cancel()
	var id int64
	err := g.e.pool.QueryRow(ctx,
		`INSERT INTO battle_records (user_id, level_id, result, stars, replay_hash)
		 VALUES ($1,1,'win',3,$2) RETURNING id`, uid, hash).Scan(&id)
	return id, hash, err
}

// mkBattleAt 与 mkBattle 相同，但显式指定 created_at —— 分页用例必须
// 让两人的行按确定的时间顺序交错，否则同秒插入会让 offset 断言飘。
func (g *gateCtx) mkBattleAt(uid int64, hash string, at time.Time) (int64, string, error) {
	ctx, cancel := context.WithTimeout(context.Background(), gateCaseTimeout)
	defer cancel()
	var id int64
	err := g.e.pool.QueryRow(ctx,
		`INSERT INTO battle_records (user_id, level_id, result, stars, replay_hash, created_at)
		 VALUES ($1,1,'win',3,$2,$3) RETURNING id`, uid, hash, at).Scan(&id)
	return id, hash, err
}

func (g *gateCtx) mkUserGem(uid int64) (int64, error) {
	ctx, cancel := context.WithTimeout(context.Background(), gateCaseTimeout)
	defer cancel()
	var id int64
	err := g.e.pool.QueryRow(ctx,
		`INSERT INTO user_gems (user_id, gem_id, quality, affixes)
		 VALUES ($1,991001,'common','[]'::jsonb) RETURNING id`, uid).Scan(&id)
	return id, err
}

// --- JSON / 基线 ---

func gateArray(raw []byte, key string) ([]map[string]any, bool) {
	var m map[string]json.RawMessage
	if err := json.Unmarshal(raw, &m); err != nil {
		return nil, false
	}
	var arr []map[string]any
	if err := json.Unmarshal(m[key], &arr); err != nil {
		return nil, false
	}
	return arr, true
}

func gateObj(raw []byte, key string) (map[string]any, bool) {
	var m map[string]json.RawMessage
	if err := json.Unmarshal(raw, &m); err != nil {
		return nil, false
	}
	var obj map[string]any
	if err := json.Unmarshal(m[key], &obj); err != nil {
		return nil, false
	}
	return obj, true
}

func gateNum(m map[string]any, key string) int64 {
	v, _ := m[key].(float64)
	return int64(v)
}

// loadGateBaseline 读 `<id>\t<STATE>`；注释与空行忽略。
func loadGateBaseline() (map[string]string, error) {
	raw, err := os.ReadFile(gateBaselineName)
	if err != nil {
		if os.IsNotExist(err) {
			return map[string]string{}, nil
		}
		return nil, err
	}
	out := map[string]string{}
	for _, line := range strings.Split(string(raw), "\n") {
		line = strings.TrimSpace(line)
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}
		fields := strings.Fields(line)
		if len(fields) < 2 {
			continue
		}
		out[fields[0]] = fields[1]
	}
	return out, nil
}

// saveGateBaseline 重写基线。
//
// 文件头部明确留出「Go 基线初始值」分区：首次生成时把三类用例的
// PASS/LEAK/UNVERIFIED 分布写进注释，避免「都 PASS」被误读成
// 「已经修好了」——PASS 只代表门禁当前确实观测不到越权/重复副作用。
func saveGateBaseline(results []struct {
	id    string
	dir   string
	state string
}, nonpass map[string]string) error {
	var b strings.Builder
	b.WriteString("# 安全回归门禁基线（Go 侧）\n")
	b.WriteString("# 每行：<用例 id>\\t<非 PASS 状态>；PASS 不写行。\n")
	b.WriteString("# 修复一条就删一行（单调下降）；只增不减视为回归。\n")
	b.WriteString("#\n")
	b.WriteString("# --- Go 基线初始值（本次生成）---\n")

	groups := map[string]*gateTally{
		"shop":  {},
		"idor":  {},
		"admin": {},
		"write": {},
	}
	order := []string{"shop", "idor", "admin", "write"}
	for _, r := range results {
		key := "admin"
		switch {
		case strings.HasPrefix(r.id, "go|shop|"):
			key = "shop"
		case strings.HasPrefix(r.id, "go|idor|"):
			key = "idor"
		case strings.HasPrefix(r.id, "go|write|"):
			key = "write"
		}
		groups[key].add(r.state)
	}
	labels := map[string]string{
		"shop":  "A 商城扣款",
		"idor":  "B 玩家 path/query/list IDOR（三向）",
		"admin": "C 管理端幂等",
		"write": "D 写路径正控制",
	}
	for _, k := range order {
		t := groups[k]
		b.WriteString(fmt.Sprintf("#   %s：PASS=%d LEAK=%d UNVERIFIED=%d\n",
			labels[k], t.pass, t.leak, t.unv))
	}
	b.WriteString("#\n")

	ids := make([]string, 0, len(nonpass))
	for id := range nonpass {
		ids = append(ids, id)
	}
	sort.Strings(ids)
	for _, id := range ids {
		b.WriteString(fmt.Sprintf("%s\t%s\n", id, nonpass[id]))
	}
	return os.WriteFile(gateBaselineName, []byte(b.String()), 0o644)
}

// gateTally 是基线分区里的一行统计。
type gateTally struct {
	pass, leak, unv int
}

func (t *gateTally) add(state string) {
	switch state {
	case gatePass:
		t.pass++
	case gateLeak:
		t.leak++
	default:
		t.unv++
	}
}
