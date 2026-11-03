// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// idempotency_r86_test.go — R86 幂等键的端到端验收。
//
// 验收语义（用户要求 5）：改前探针证明的三条重复后果（发币 +200、
// 公告 ×2、随机码 ×2）在改后变成「第二次返回**同一响应**（逐字节）、
// 业务只执行一次」；失败不留幂等记录（可重试）；无 key 走老路径。

import (
	"context"
	"fmt"
	"io"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

// r86Key 生成**本次测试唯一**的幂等键：e2e 共享 dev 库，
// 常量 key 会撞上上一次跑留下的幂等记录（R86 实测：重放的是
// 上一个已被 cleanup 删除的用户的响应，断言全歪）。
func r86Key(t *testing.T) string {
	t.Helper()
	name := strings.TrimPrefix(t.Name(), "TestIdem")
	return fmt.Sprintf("r86-%s-%d", name, time.Now().UnixNano())
}

// idemResp 是一次幂等请求的完整观察面。
type idemResp struct {
	status int
	body   []byte
	replay bool
}

// postIdem 打一次管理端 POST（可带 Idempotency-Key），拿原始响应。
func postIdem(t *testing.T, e *e2e, path, adminTok, key string, body any) idemResp {
	t.Helper()
	req := httptest.NewRequest("POST", path, mustJSON(t, body))
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+adminTok)
	if key != "" {
		req.Header.Set("Idempotency-Key", key)
	}
	resp, err := e.app.Test(req, 30000)
	if err != nil {
		t.Fatalf("POST %s 失败：%v", path, err)
	}
	defer resp.Body.Close()
	raw, err := io.ReadAll(resp.Body)
	if err != nil {
		t.Fatalf("读响应失败：%v", err)
	}
	return idemResp{
		status: resp.StatusCode,
		body:   raw,
		replay: resp.Header.Get("X-Idempotent-Replay") == "true",
	}
}

func walletCoin(t *testing.T, e *e2e, uid int64) int64 {
	t.Helper()
	var coin int64
	if err := e.pool.QueryRow(context.Background(),
		`SELECT coin FROM user_wallets WHERE user_id = $1`, uid).Scan(&coin); err != nil {
		t.Fatalf("读钱包失败：%v", err)
	}
	return coin
}

// TestIdemGrantReplaySameBytes：发币带 key ×2 → 第二次重放（逐字节同）、余额只 +100。
func TestIdemGrantReplaySameBytes(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "r86grant")
	uid, _ := e.newPlayer(t, "r86grant")
	before := walletCoin(t, e, uid)

	path := fmt.Sprintf("/api/v1/admin/users/%d/grant", uid)
	reqBody := map[string]any{"currency": "coin", "amount": 100}
	key := r86Key(t)
	r1 := postIdem(t, e, path, adminTok, key, reqBody)
	r2 := postIdem(t, e, path, adminTok, key, reqBody)

	if r1.status != 200 || r2.status != 200 {
		t.Fatalf("两次应都 200，实际 %d/%d（r2=%s）", r1.status, r2.status, r2.body)
	}
	if r1.replay {
		t.Error("第一次不应是重放")
	}
	if !r2.replay {
		t.Error("第二次应是重放（X-Idempotent-Replay: true）")
	}
	if string(r1.body) != string(r2.body) {
		t.Errorf("重放响应体应与第一次**逐字节**相同：\n第一次 %s\n第二次 %s", r1.body, r2.body)
	}
	if got := walletCoin(t, e, uid) - before; got != 100 {
		t.Errorf("余额应只 +100，实际 %+d", got)
	}
	// 审计只写一次（重放不产生第二条）。
	var audits int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT COUNT(*) FROM admin_audit_logs WHERE action='grant_currency' AND target=$1`,
		fmt.Sprint(uid)).Scan(&audits); err != nil {
		t.Fatalf("数审计失败：%v", err)
	}
	if audits != 1 {
		t.Errorf("审计应恰好 1 行，实际 %d", audits)
	}
}

// TestIdemAnnouncementReplay：公告带 key ×2 → 只一行、字节同。
func TestIdemAnnouncementReplay(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "r86ann")
	title := fmt.Sprintf("R86幂等标题%d", time.Now().UnixNano()) // 唯一标题（dev 库共享）
	reqBody := map[string]any{"title": title, "body": "R86幂等正文", "published": true}
	key := r86Key(t)
	r1 := postIdem(t, e, "/api/v1/admin/announcements", adminTok, key, reqBody)
	r2 := postIdem(t, e, "/api/v1/admin/announcements", adminTok, key, reqBody)
	if r1.status != 200 || r2.status != 200 {
		t.Fatalf("两次应都 200，实际 %d/%d", r1.status, r2.status)
	}
	if string(r1.body) != string(r2.body) {
		t.Errorf("重放响应体应逐字节相同：\n第一次 %s\n第二次 %s", r1.body, r2.body)
	}
	var n int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT COUNT(*) FROM announcements WHERE title=$1`, title).Scan(&n); err != nil {
		t.Fatalf("数公告失败：%v", err)
	}
	if n != 1 {
		t.Errorf("公告应恰好 1 行，实际 %d", n)
	}
}

// TestIdemBatchCodesReplay：批量码带 key ×2 → 只 5 个码、items 字节同。
func TestIdemBatchCodesReplay(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "r86batch")
	reqBody := map[string]any{"count": 5, "reward": map[string]int{"coin": 1}, "max_uses": 1}
	key := r86Key(t)
	r1 := postIdem(t, e, "/api/v1/admin/redeem-codes/batch", adminTok, key, reqBody)
	r2 := postIdem(t, e, "/api/v1/admin/redeem-codes/batch", adminTok, key, reqBody)
	if r1.status != 200 || r2.status != 200 {
		t.Fatalf("两次应都 200，实际 %d/%d（r2=%s）", r1.status, r2.status, r2.body)
	}
	if string(r1.body) != string(r2.body) {
		t.Errorf("重放响应体应逐字节相同")
	}
	// 本测试时间窗内只有 5 个码（dev 库共享，不能按全表数）。
	t0 := time.Now().Add(-time.Minute)
	var n int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT COUNT(*) FROM redeem_codes WHERE max_uses=1 AND reward='{"coin":1}' AND created_at >= $1`,
		t0).Scan(&n); err != nil {
		t.Fatalf("数码失败：%v", err)
	}
	if n != 5 {
		t.Errorf("时间窗内应恰好 5 个码，实际 %d", n)
	}
}

// TestIdemConflictOnDifferentBody：同 key 换请求体 → 409 idempotency_conflict。
func TestIdemConflictOnDifferentBody(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "r86conf")
	uid, _ := e.newPlayer(t, "r86conf")
	path := fmt.Sprintf("/api/v1/admin/users/%d/grant", uid)
	key := r86Key(t)
	r1 := postIdem(t, e, path, adminTok, key, map[string]any{"currency": "coin", "amount": 100})
	if r1.status != 200 {
		t.Fatalf("第一次应 200，实际 %d", r1.status)
	}
	r2 := postIdem(t, e, path, adminTok, key, map[string]any{"currency": "coin", "amount": 200})
	if r2.status != fiber.StatusConflict {
		t.Fatalf("同 key 换 body 应 409，实际 %d body=%s", r2.status, r2.body)
	}
	if !strings.Contains(string(r2.body), "idempotency_conflict") {
		t.Errorf("响应码应为 idempotency_conflict：%s", r2.body)
	}
}

// TestIdemNoKeyKeepsOldBehavior：无 key ×2 → 重复执行（现状保持）。
func TestIdemNoKeyKeepsOldBehavior(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "r86nokey")
	uid, _ := e.newPlayer(t, "r86nokey")
	before := walletCoin(t, e, uid)
	path := fmt.Sprintf("/api/v1/admin/users/%d/grant", uid)
	reqBody := map[string]any{"currency": "coin", "amount": 100}
	for i := 0; i < 2; i++ {
		r := postIdem(t, e, path, adminTok, "", reqBody)
		if r.status != 200 {
			t.Fatalf("第 %d 次应 200，实际 %d", i, r.status)
		}
		if r.replay {
			t.Errorf("无 key 不应有重放标记")
		}
	}
	if got := walletCoin(t, e, uid) - before; got != 200 {
		t.Errorf("无 key 应重复执行（+200），实际 %+d", got)
	}
}

// TestIdemKeyMalformed：key 白名单（太短/太长/非法字符）→ 400，不截断。
func TestIdemKeyMalformed(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "r86key")
	uid, _ := e.newPlayer(t, "r86key")
	path := fmt.Sprintf("/api/v1/admin/users/%d/grant", uid)
	reqBody := map[string]any{"currency": "coin", "amount": 1}
	for _, bad := range []string{"short", strings.Repeat("k", 129), "bad key!", "中文键值对"} {
		r := postIdem(t, e, path, adminTok, bad, reqBody)
		if r.status != fiber.StatusBadRequest {
			t.Errorf("key %q 应 400，实际 %d", bad, r.status)
		}
	}
}

// TestIdemFailureLeavesNoRecord：业务失败不留幂等记录（要求 3）——
// 带 key 的失败请求（回收超额 → 事务内 CHECK 23514）之后，
// 同 key 修正重试必须能成功（而不是被「已处理」假记录挡住）。
func TestIdemFailureLeavesNoRecord(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "r86fail")
	uid, _ := e.newPlayer(t, "r86fail")
	path := fmt.Sprintf("/api/v1/admin/users/%d/grant", uid)

	key := r86Key(t)
	// 第一次：回收超过余额（新号 2000，收 99999）→ 事务内失败。
	r1 := postIdem(t, e, path, adminTok, key, map[string]any{"currency": "coin", "amount": -99999})
	if r1.status == 200 {
		t.Fatalf("超额回收应失败，实际 200")
	}
	// 同 key 换「合法请求体」重试：不是重放（失败没留记录），而是真正执行。
	r2 := postIdem(t, e, path, adminTok, key, map[string]any{"currency": "coin", "amount": 100})
	if r2.status != 200 {
		t.Fatalf("失败后同 key 重试应可成功，实际 %d body=%s", r2.status, r2.body)
	}
	if r2.replay {
		t.Error("失败的请求不应留下可重放的记录")
	}
}

// TestIdemConcurrentSameKey：同 key 并发 8 个 → 恰好执行一次（余额只 +100）。
func TestIdemConcurrentSameKey(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "r86conc")
	uid, _ := e.newPlayer(t, "r86conc")
	before := walletCoin(t, e, uid)
	path := fmt.Sprintf("/api/v1/admin/users/%d/grant", uid)
	reqBody := map[string]any{"currency": "coin", "amount": 100}

	type out struct {
		status  int
		replay  bool
		rawBody string
	}
	key := r86Key(t)
	results := make([]out, 8)
	done := make(chan int, 8)
	for i := 0; i < 8; i++ {
		go func(i int) {
			r := postIdem(t, e, path, adminTok, key, reqBody)
			results[i] = out{r.status, r.replay, string(r.body)}
			done <- i
		}(i)
	}
	for i := 0; i < 8; i++ {
		<-done
	}
	// 余额只 +100（恰好执行一次）。
	if got := walletCoin(t, e, uid) - before; got != 100 {
		t.Errorf("并发同 key 应恰好执行一次（+100），实际 %+d", got)
	}
	// 全部 200，无 5xx/409；且所有成功响应体一致（一次执行 + N 次重放）。
	for i, r := range results {
		if r.status != 200 {
			t.Errorf("第 %d 个并发应 200，实际 %d", i, r.status)
		}
		if r.rawBody != results[0].rawBody {
			t.Errorf("第 %d 个响应体与第一个不一致", i)
		}
	}
}
