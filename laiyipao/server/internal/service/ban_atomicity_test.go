package service

import (
	"context"
	"testing"
)

// 第 114 轮：封禁与 refresh token 吊销必须**同生同死**。
//
// 修前两条独立 autocommit：封禁成功后第二条失败 →
// 库里是「status=2 但 token 还有效」的**半封状态**，
// 被禁账号在 RefreshTTL 内仍可换新令牌。
//
// 判据用**故障注入**（scratch 库专用：临时改名 refresh_tokens，
// 强制第二步必失败）：
//   - 封禁失败必须**报错**（不能吞掉）
//   - 且第一步的 status 写入必须**回滚**（半封状态不存在）
//   - 表改名复原后，正常封禁必须原子成功（status + 吊销一起落）
func TestBanAndTokenRevocationAreAtomic(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 造 2 张有效 refresh token
	mkToken := func(tag string) {
		ts.exec(t, `INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
			VALUES ($1, $2, now() + interval '30 days')`, uid, "hash_"+tag+"_114")
	}
	mkToken("a")
	mkToken("b")
	activeTokens := func() int {
		var n int
		_ = ts.pool.QueryRow(ctx,
			`SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL`, uid).Scan(&n)
		return n
	}
	// newUser 的登录夹具可能已自带 1 张 token，不假设具体数量：
	// 记下「改名前」的基线，故障后要求回到基线，封禁后要求归零。
	baseActive := activeTokens()
	if baseActive == 0 {
		t.Fatal("预置 token 后至少应有 1 张有效 token")
	}
	statusOf := func() int {
		var s int
		_ = ts.pool.QueryRow(ctx, `SELECT status FROM users WHERE id = $1`, uid).Scan(&s)
		return s
	}

	// —— 故障注入：让「吊销」这一步必然失败 ——
	ts.renameTable(t, "refresh_tokens", "refresh_tokens_gone")
	_, err := ts.AdminSetUserStatus(ctx, uid, true, "ban-fail-inject")
	if err == nil {
		t.Fatal("token 表故障时封禁必须报错（修前第二条失败被吞，status 已落）")
	}
	// 关键断言：第一步也回滚了 —— 半封状态（status=2 但 token 活着）不允许存在
	if got := statusOf(); got != 1 {
		t.Errorf("封禁失败后 status 应回滚为 1，实际 %d（修前这里会是 2：半封状态）", got)
	}
	ts.renameTable(t, "refresh_tokens_gone", "refresh_tokens")
	if got := activeTokens(); got != baseActive {
		t.Errorf("故障窗口后 token 应原样（%d 张有效），实际 %d", baseActive, got)
	}

	// —— 故障解除：正常封禁必须原子成功 ——
	if _, err := ts.AdminSetUserStatus(ctx, uid, true, "ok-ban"); err != nil {
		t.Fatalf("正常封禁应成功：%v", err)
	}
	if got := statusOf(); got != 2 {
		t.Errorf("封禁后 status 应为 2，实际 %d", got)
	}
	if got := activeTokens(); got != 0 {
		t.Errorf("封禁后有效 token 应为 0，实际 %d（封禁即吊销）", got)
	}

	// 解封是单步，不受影响
	if _, err := ts.AdminSetUserStatus(ctx, uid, false, ""); err != nil {
		t.Fatalf("解封应成功：%v", err)
	}
	if got := statusOf(); got != 1 {
		t.Errorf("解封后 status 应为 1，实际 %d", got)
	}
}
