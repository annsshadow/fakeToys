// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// idempotency_test.go — R96 P0：幂等键的真实行为测试。
//
// R86 给三条管理端写路径接上了 `Idempotency-Key`（发币/公告/批量兑换码），
// 但当时只有探针（idempotency_probe_test.go）证明「改前会重复执行」，
// **契约本身没有测试**：`RunIdempotent`、`HashRequestBody` 与三个 `*Idem`
// 方法在覆盖率报告里都是 0%。也就是说，如果有人把「查重」删掉、
// 或把记录写入挪到业务之前，没有任何用例会红。
//
// 这里补的就是那层守卫。断言的落点都是**后果**，不是实现细节：
// 余额只动一次、流水分只一条、审计只写一行、公告/兑换码只多一行、
// 业务失败后重试必须真的执行（不能拿上一次的残骸糊弄）。

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"strconv"
	"testing"
	"time"

	"github.com/jackc/pgx/v5"
)

// r96Key 生成本轮唯一的幂等键（8~128 字符、字母数字，符合 IdemKeyPattern）。
func r96Key(tag string) string {
	return fmt.Sprintf("r96-%s-%d", tag, time.Now().UnixNano())
}

func TestHashRequestBodyIsStableAndDistinct(t *testing.T) {
	a := HashRequestBody([]byte(`{"coin":10}`))
	if len(a) != 64 {
		t.Fatalf("sha256 hex 长度应为 64，实际 %d（%q）", len(a), a)
	}
	if got := HashRequestBody([]byte(`{"coin":10}`)); got != a {
		t.Errorf("同一请求体两次哈希应相同：%q vs %q", got, a)
	}
	// 空格也要算差异 —— 否则「同 key 不同请求」会被误判为重放。
	if got := HashRequestBody([]byte(`{"coin": 10}`)); got == a {
		t.Errorf("不同请求体（多了空格）必须得到不同哈希，实际都是 %q", got)
	}
}

// TestRunIdempotentReplaysAndRollsBack 直接测事务原语：
// ① 第一次执行业务并留记录；② 同 key 同体重放，业务不再执行且响应字节一致；
// ③ 业务失败时整笔回滚，重试必须**真的执行**而不是拿到上一次的残骸。
func TestRunIdempotentReplaysAndRollsBack(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// ① + ②：重放不重复执行
	ep := "test.r96.replay"
	key := r96Key("replay")
	hash := HashRequestBody([]byte(`{"x":1}`))

	var execs int
	body1, replayed1, err := ts.RunIdempotent(ctx, ep, key, hash,
		func(tx pgx.Tx) (any, error) {
			execs++
			return map[string]any{"n": 1}, nil
		})
	if err != nil {
		t.Fatalf("首次执行失败：%v", err)
	}
	if replayed1 {
		t.Fatal("首次调用不该是重放")
	}
	if execs != 1 {
		t.Fatalf("业务应执行 1 次，实际 %d", execs)
	}

	body2, replayed2, err := ts.RunIdempotent(ctx, ep, key, hash,
		func(tx pgx.Tx) (any, error) {
			execs++
			return map[string]any{"n": 2}, nil
		})
	if err != nil {
		t.Fatalf("重放失败：%v", err)
	}
	if !replayed2 {
		t.Fatal("同 key 同请求体的第二次调用必须标记为重放")
	}
	if execs != 1 {
		t.Fatalf("重放不该执行业务，实际执行了 %d 次", execs)
	}
	if !bytes.Equal(body1, body2) {
		t.Errorf("重放必须返回第一次的响应：\nfirst=%s\nsecond=%s", body1, body2)
	}

	// ③ 业务失败 → 记录随事务回滚；重试必须真正执行。
	epFail := "test.r96.rollback"
	keyFail := r96Key("rollback")
	hashFail := HashRequestBody([]byte(`{"x":2}`))
	errBoom := errors.New("boom")

	var tries int
	if _, _, err := ts.RunIdempotent(ctx, epFail, keyFail, hashFail,
		func(tx pgx.Tx) (any, error) {
			tries++
			return nil, errBoom
		}); !errors.Is(err, errBoom) {
		t.Fatalf("业务错误应原样上抛，实际 %v", err)
	}

	body3, replayed3, err := ts.RunIdempotent(ctx, epFail, keyFail, hashFail,
		func(tx pgx.Tx) (any, error) {
			tries++
			return map[string]any{"ok": true}, nil
		})
	if err != nil {
		t.Fatalf("失败后重试应当成功，实际 %v", err)
	}
	if replayed3 {
		t.Fatal("业务失败的那次不该留下幂等记录：重试被当成了重放")
	}
	if tries != 2 {
		t.Fatalf("重试必须真的执行业务（tries 应为 2），实际 %d", tries)
	}
	if len(body3) == 0 {
		t.Fatal("重试成功后应返回序列化响应")
	}

	// 同 key 换请求体 → 冲突（不能拿旧结果糊弄新请求）
	if _, _, err := ts.RunIdempotent(ctx, ep, key, HashRequestBody([]byte(`{"x":9}`)),
		func(tx pgx.Tx) (any, error) {
			t.Fatal("冲突路径不该执行业务")
			return nil, nil
		}); !errors.Is(err, ErrIdemConflict) {
		t.Fatalf("同 key 不同请求体应返回 ErrIdemConflict，实际 %v", err)
	}
}

// TestAdminGrantCurrencyIdemReplayCreditsOnce 是资金路径的核心守卫：
// 同 key 重放不许二次加币、二次写流水、二次写审计。
func TestAdminGrantCurrencyIdemReplayCreditsOnce(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	adminID, _, _ := ts.newAdminFixture(t, "r96grant", 1)
	uid := ts.newUser(t, ctx)
	audit := AdminAuditCtx{AdminID: adminID, IP: "127.0.0.1"}

	key := r96Key("grant")
	reqHash := HashRequestBody([]byte(`{"user":1,"currency":"coin","amount":100}`))
	before := ts.wallet(t, ctx, uid).Coin
	flowsBefore := ts.countFlowsByReason(t, ctx, uid, "admin_grant")
	auditQ := `SELECT COUNT(*) FROM admin_audit_logs WHERE action = 'grant_currency' AND target = $1`
	auditsBefore := ts.scalar(t, ctx, auditQ, strconv.FormatInt(uid, 10))

	body1, replayed1, err := ts.AdminGrantCurrencyIdem(ctx, uid, "coin", 100, key, reqHash, audit)
	if err != nil {
		t.Fatalf("发币失败：%v", err)
	}
	if replayed1 {
		t.Fatal("首次发币不该是重放")
	}
	if got := ts.wallet(t, ctx, uid).Coin; got != before+100 {
		t.Fatalf("余额应 %d → %d，实际 %d", before, before+100, got)
	}

	body2, replayed2, err := ts.AdminGrantCurrencyIdem(ctx, uid, "coin", 100, key, reqHash, audit)
	if err != nil {
		t.Fatalf("重放失败：%v", err)
	}
	if !replayed2 {
		t.Fatal("同 key 同体重放应标记 replayed")
	}
	if got := ts.wallet(t, ctx, uid).Coin; got != before+100 {
		t.Fatalf("重放不许二次加币：期望仍为 %d，实际 %d", before+100, got)
	}
	if got := ts.countFlowsByReason(t, ctx, uid, "admin_grant"); got != flowsBefore+1 {
		t.Fatalf("流水应只多 1 条（%d → %d），实际 %d", flowsBefore, flowsBefore+1, got)
	}
	if got := ts.scalar(t, ctx, auditQ, strconv.FormatInt(uid, 10)); got != auditsBefore+1 {
		t.Fatalf("审计应只多 1 行，实际 %d → %d", auditsBefore, got)
	}
	if !bytes.Equal(body1, body2) {
		t.Errorf("重放应返回第一次的响应体：\nfirst=%s\nsecond=%s", body1, body2)
	}

	// 同 key 换体 → 409 语义，余额不动
	if _, _, err := ts.AdminGrantCurrencyIdem(ctx, uid, "coin", 200, key,
		HashRequestBody([]byte(`{"user":1,"currency":"coin","amount":200}`)), audit); !errors.Is(err, ErrIdemConflict) {
		t.Fatalf("同 key 换金额应冲突，实际 %v", err)
	}
	if got := ts.wallet(t, ctx, uid).Coin; got != before+100 {
		t.Fatalf("冲突路径不许加币：期望 %d，实际 %d", before+100, got)
	}

	// 回收（负数）走 revoke_currency，流水 reason 变 admin_revoke
	keyRevoke := r96Key("revoke")
	revokeHash := HashRequestBody([]byte(`{"amount":-30}`))
	if _, _, err := ts.AdminGrantCurrencyIdem(ctx, uid, "coin", -30, keyRevoke, revokeHash, audit); err != nil {
		t.Fatalf("回收失败：%v", err)
	}
	if got := ts.wallet(t, ctx, uid).Coin; got != before+70 {
		t.Fatalf("回收后余额应为 %d，实际 %d", before+70, got)
	}
	if got := ts.countFlowsByReason(t, ctx, uid, "admin_revoke"); got != 1 {
		t.Fatalf("回收应写 1 条 admin_revoke 流水，实际 %d", got)
	}
}

// TestAdminGrantCurrencyIdemValidatesBeforeTx 覆盖参数边界：0 与超限都在
// 进入幂等事务之前被挡下（不留下幂等记录，也就不需要为「无意义请求」
// 消耗一次查重）。
func TestAdminGrantCurrencyIdemValidatesBeforeTx(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	adminID, _, _ := ts.newAdminFixture(t, "r96grantbad", 1)
	uid := ts.newUser(t, ctx)
	audit := AdminAuditCtx{AdminID: adminID}

	if _, _, err := ts.AdminGrantCurrencyIdem(ctx, uid, "coin", 0, r96Key("zero"),
		HashRequestBody([]byte("{}")), audit); !errors.Is(err, ErrBadInput) {
		t.Fatalf("amount=0 应 ErrBadInput，实际 %v", err)
	}
	if _, _, err := ts.AdminGrantCurrencyIdem(ctx, uid, "coin", MaxAdminGrantAmount+1, r96Key("big"),
		HashRequestBody([]byte("{}")), audit); !errors.Is(err, ErrBadInput) {
		t.Fatalf("超上限应 ErrBadInput，实际 %v", err)
	}
	if _, _, err := ts.AdminGrantCurrencyIdem(ctx, 1<<40, "coin", 1, r96Key("ghost"),
		HashRequestBody([]byte("{}")), audit); !errors.Is(err, ErrNotFound) {
		t.Fatalf("不存在的用户应 ErrNotFound，实际 %v", err)
	}
}

// TestAdminCreateAnnouncementIdemReplayCreatesOnce 覆盖公告路径：
// 同 key 重放不许出现第二行公告。
func TestAdminCreateAnnouncementIdemReplayCreatesOnce(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	adminID, _, _ := ts.newAdminFixture(t, "r96ann", 1)
	audit := AdminAuditCtx{AdminID: adminID}

	title := "R96幂等公告-" + strconv.FormatInt(time.Now().UnixNano(), 10)
	key := r96Key("announcement")
	reqHash := HashRequestBody([]byte(`{"title":"` + title + `"}`))
	countQ := `SELECT COUNT(*) FROM announcements WHERE title = $1`

	body1, replayed1, err := ts.AdminCreateAnnouncementIdem(ctx, title, "正文", true, key, reqHash, audit)
	if err != nil {
		t.Fatalf("建公告失败：%v", err)
	}
	if replayed1 {
		t.Fatal("首次建公告不该是重放")
	}
	if got := ts.scalar(t, ctx, countQ, title); got != 1 {
		t.Fatalf("应只有 1 行公告，实际 %d", got)
	}

	body2, replayed2, err := ts.AdminCreateAnnouncementIdem(ctx, title, "正文", true, key, reqHash, audit)
	if err != nil {
		t.Fatalf("重放失败：%v", err)
	}
	if !replayed2 {
		t.Fatal("同 key 同体重放应标记 replayed")
	}
	if got := ts.scalar(t, ctx, countQ, title); got != 1 {
		t.Fatalf("重放不许再建公告，实际 %d 行", got)
	}
	if !bytes.Equal(body1, body2) {
		t.Errorf("重放应返回第一次的响应体")
	}

	// 同 key 换标题 → 冲突
	if _, _, err := ts.AdminCreateAnnouncementIdem(ctx, title+"-改", "正文", true, key,
		HashRequestBody([]byte(`{"title":"other"}`)), audit); !errors.Is(err, ErrIdemConflict) {
		t.Fatalf("同 key 换内容应冲突，实际 %v", err)
	}
	if got := ts.scalar(t, ctx, countQ, title+"-改"); got != 0 {
		t.Fatalf("冲突路径不许建公告，实际 %d 行", got)
	}

	// 空标题在进入事务前被挡下
	if _, _, err := ts.AdminCreateAnnouncementIdem(ctx, "", "正文", false, r96Key("annempty"),
		HashRequestBody([]byte("{}")), audit); !errors.Is(err, ErrBadInput) {
		t.Fatalf("空标题应 ErrBadInput，实际 %v", err)
	}
}

// TestAdminCreateRedeemCodesIdemReplayCreatesOnce 覆盖批量兑换码：
// 重放不许翻倍出码。
func TestAdminCreateRedeemCodesIdemReplayCreatesOnce(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	adminID, _, _ := ts.newAdminFixture(t, "r96codes", 1)
	audit := AdminAuditCtx{AdminID: adminID}

	reward := map[string]int{"coin": 98765}
	key := r96Key("codes")
	reqHash := HashRequestBody([]byte(`{"count":3,"coin":98765}`))
	countQ := `SELECT COUNT(*) FROM redeem_codes WHERE reward = $1`
	rewardJSON := `{"coin": 98765}`

	before := ts.scalar(t, ctx, countQ, rewardJSON)

	body1, replayed1, err := ts.AdminCreateRedeemCodesIdem(ctx, 3, reward, 1, nil, key, reqHash, audit)
	if err != nil {
		t.Fatalf("批量建码失败：%v", err)
	}
	if replayed1 {
		t.Fatal("首次建码不该是重放")
	}
	if got := ts.scalar(t, ctx, countQ, rewardJSON); got != before+3 {
		t.Fatalf("应新增 3 个码（%d → %d），实际 %d", before, before+3, got)
	}

	body2, replayed2, err := ts.AdminCreateRedeemCodesIdem(ctx, 3, reward, 1, nil, key, reqHash, audit)
	if err != nil {
		t.Fatalf("重放失败：%v", err)
	}
	if !replayed2 {
		t.Fatal("同 key 同体重放应标记 replayed")
	}
	if got := ts.scalar(t, ctx, countQ, rewardJSON); got != before+3 {
		t.Fatalf("重放不许再出码，实际总数 %d", got)
	}
	if !bytes.Equal(body1, body2) {
		t.Errorf("重放应返回第一次的响应体")
	}

	// 同 key 换参数 → 冲突，且不再出码
	beforeConflict := ts.scalar(t, ctx, countQ, rewardJSON)
	if _, _, err := ts.AdminCreateRedeemCodesIdem(ctx, 5, reward, 1, nil, key,
		HashRequestBody([]byte(`{"count":5}`)), audit); !errors.Is(err, ErrIdemConflict) {
		t.Fatalf("同 key 换参数应冲突，实际 %v", err)
	}
	if got := ts.scalar(t, ctx, countQ, rewardJSON); got != beforeConflict {
		t.Fatalf("冲突路径不许出码：期望 %d，实际 %d", beforeConflict, got)
	}

	// 数量边界在进入事务前被挡下
	for _, bad := range []int{0, -1, 101} {
		if _, _, err := ts.AdminCreateRedeemCodesIdem(ctx, bad, reward, 1, nil,
			r96Key("codesbad"), HashRequestBody([]byte("{}")), audit); !errors.Is(err, ErrBadInput) {
			t.Fatalf("count=%d 应 ErrBadInput，实际 %v", bad, err)
		}
	}
}
