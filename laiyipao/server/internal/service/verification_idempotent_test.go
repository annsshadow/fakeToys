package service

import (
	"context"
	"testing"
)

// TestVerificationIsIdempotent 钉住 P3-5。
//
// # 背景
//
// `replay_verifications` 此前**既无唯一约束也无去重**：
// 同一场战报重复验真 200 次 → 写进 200 行
// （`httpapi/middleware.go` 的限流注释里记着这个实测事实）。
//
// 后果不是「表大了点」：
//
//  1. **写放大** —— 限流只是缓解，不是幂等；
//  2. **统计口径被污染** —— 后台的「验真一致率」是 checked/mismatched
//     两个计数，同一局验 20 次，分母就多 19；
//  3. 真正的结论（「这一局对不上」）被 200 条几乎一样的记录淹没。
//
// # 判据
//
//  1. 同一场 + 同一验真者 + **同一哈希**重复验 → 只留一行；
//  2. **换了哈希** → 必须新增一行（那是新信息，可能推翻之前的结论）。
func TestVerificationIsIdempotent(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 需要一条战报才能验真。
	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	in := loseSettleInput()
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, in); err != nil {
		t.Fatalf("结算失败：%v", err)
	}
	var battleID int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM battle_records WHERE user_id = $1 ORDER BY id DESC LIMIT 1`, uid).
		Scan(&battleID); err != nil {
		t.Fatalf("读战报 id 失败：%v", err)
	}

	const hash = "0123456789abcdef"
	for i := 0; i < 5; i++ {
		if _, err := ts.compareAndRecord(ctx, userVerifier(uid), battleID, hash); err != nil {
			t.Fatalf("第 %d 次验真失败：%v", i+1, err)
		}
	}
	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM replay_verifications WHERE battle_id = $1`, battleID).Scan(&n); err != nil {
		t.Fatalf("统计验真行数失败：%v", err)
	}
	if n != 1 {
		t.Errorf("同一哈希重复验真 5 次应只留 1 行，实际 %d 行 —— 写放大 + 统计口径被污染", n)
	}

	// 换哈希 = 新信息，必须留
	if _, err := ts.compareAndRecord(ctx, userVerifier(uid), battleID, "fedcba9876543210"); err != nil {
		t.Fatalf("换哈希验真失败：%v", err)
	}
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM replay_verifications WHERE battle_id = $1`, battleID).Scan(&n); err != nil {
		t.Fatalf("统计验真行数失败：%v", err)
	}
	if n != 2 {
		t.Errorf("换了哈希应新增一行（那是可能推翻之前结论的新信息），实际 %d 行", n)
	}
}
