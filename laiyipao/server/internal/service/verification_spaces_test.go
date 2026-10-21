package service

import (
	"context"
	"fmt"
	"testing"
	"time"

	"golang.org/x/crypto/bcrypt"
)

// TestVerificationDoesNotMixUserAndAdminIDSpaces 钉住 00018。
//
// # 背景（2026-10-21 第二轮审计 P1）
//
// 00017 的唯一索引是：
//
//	UNIQUE (battle_id, COALESCE(verifier_id, admin_verifier_id), actual_hash)
//
// 而 `verifier_id` 指向 `users(id)`、`admin_verifier_id` 指向
// `admin_users(id)` —— **两个独立的 ID 空间，各自从 1 开始**。
//
// `COALESCE` 把它们折叠成一个数，于是：
//
//	玩家 users.id=1        对战报 #42 验真，哈希 "0123…"  → (42, 1, NULL, "0123…")
//	运营 admin_users.id=1  对**同一场**战报验真，**同一个哈希** → (42, NULL, 1, "0123…")
//
// 索引值相同 → 冲突 → `DO NOTHING` → **运营那一行被静默丢弃**，
// 而接口照样返回 `matched: true`。
//
// 后果全是静默的：
//   - 运营以为「已记录」，`replay_verifications` 里却没有他的记录；
//   - 验真计数**按战报主人聚合** → 运营的验真被算到**玩家**头上
//     → 玩家的「可疑度」被虚高。
//
// # 判据
//
// 玩家与运营**各自**验同一场战报的同一哈希 → **两行都在**。
func TestVerificationDoesNotMixUserAndAdminIDSpaces(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput()); err != nil {
		t.Fatalf("结算失败：%v", err)
	}
	var battleID int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM battle_records WHERE user_id = $1 ORDER BY id DESC LIMIT 1`, uid).
		Scan(&battleID); err != nil {
		t.Fatalf("读战报 id 失败：%v", err)
	}

	// 造一个**与玩家同一个 id**的管理员 —— 这才是「两个 ID 空间」的真正冲突点。
	//
	// ⚠️ 不能用 `newAdminFixture`：它的 id 是自增的，与玩家的 id 大概率不同
	// → `COALESCE` 后是两个不同的数 → 根本不冲突 → 测试**测不到那个 bug**
	// （我在第一次写这个测试时就踩了这个坑：用旧索引跑，它照样绿）。
	//
	// 所以这里**显式指定 id**，与玩家相同。
	adminRowID := uid
	adminUser := fmt.Sprintf("svc_admin_spaces_%d", time.Now().UnixNano()%1_000_000)
	adminHash, err := bcrypt.GenerateFromPassword([]byte("svc-admin-pass-123"), bcrypt.MinCost)
	if err != nil {
		t.Fatalf("生成哈希失败：%v", err)
	}
	if _, err := ts.pool.Exec(ctx,
		`INSERT INTO admin_users (id, username, password_hash, role, status)
		 VALUES ($1,$2,$3,'admin',1)
		 ON CONFLICT (id) DO UPDATE SET username = EXCLUDED.username`,
		adminRowID, adminUser, string(adminHash)); err != nil {
		t.Fatalf("造同 id 管理员失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(),
			`DELETE FROM admin_audit_logs WHERE username = $1`, adminUser)
		_, _ = ts.pool.Exec(context.Background(),
			`DELETE FROM admin_users WHERE id = $1`, adminRowID)
	})

	const hash = "0123456789abcdef"
	if _, err := ts.compareAndRecord(ctx, userVerifier(uid), battleID, hash); err != nil {
		t.Fatalf("玩家验真失败：%v", err)
	}
	if _, err := ts.compareAndRecord(ctx, adminVerifier(adminRowID), battleID, hash); err != nil {
		t.Fatalf("运营验真失败：%v", err)
	}

	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM replay_verifications WHERE battle_id = $1`, battleID).Scan(&n); err != nil {
		t.Fatalf("统计验真行数失败：%v", err)
	}
	if n != 2 {
		t.Errorf("玩家与运营对同一场战报、同一哈希各验一次，应留下 2 行，实际 %d 行 —— "+
			"唯一索引把两个 ID 空间合并了，运营那一行被静默丢弃", n)
	}
}
