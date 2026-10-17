package service

// 本轮安全审计的回归守卫。
//
// 每一条都写成「**改回去会不会红**」的判据，而不是「覆盖到了吗」。
// 判据本身见每条上方的注释。

import (
	"context"
	"errors"
	"fmt"
	"strings"
	"sync/atomic"
	"testing"

	"github.com/jackc/pgx/v5/pgconn"
)

// grantToken 负数扣减回归。
//
// # 缺陷回顾
//
// `grantToken` 原先只有一条 UPSERT，对 delta 正负不分。扣减时它必然撞上
// `CHECK (balance >= 0)` —— CHECK 是**语句级**判定，于是整笔事务带着
// SQLSTATE 23514 回滚，而那个可读的「道具不足」分支（`if balance < 0`）
// 永远执行不到。23514 不在 `isSettleRejection` 名单里 → 500「服务内部错误」。
//
// 原注释说「这条分支当前不可达」—— 它当时确实不可达，但现在不是了：
// `AdminUpdateShopItem` 的 `validWalletCurrency` **明确允许**把道具名写进
// 商品 price，`Buy` 的 deduct 于是带上负的道具 delta。
//
// 判据必须落在**错误类型**上：断言 `errors.Is(err, ErrBadInput)`，
// 而不是断言「返回了错误」。一个恒返回 `nil` 的实现能骗过后者，
// 而「返回 500 而不是 400」正是本条要防的形态。

// TestGrantTokenNegativeDeltaIsBusinessError：余额不足必须是 ErrBadInput。
func TestGrantTokenNegativeDeltaIsBusinessError(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := auditProbeUser(t, ts)

	_, err := ts.Service.AdminGrantCurrency(ctx, uid, "revive_token", -1)
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("回收不存在的道具应报 ErrBadInput（→400），实际 = %v", err)
	}
	// 且必须**不是**数据库约束错误：那正是修复前的形态。
	var pgErr *pgconn.PgError
	if errors.As(err, &pgErr) {
		t.Fatalf("不该把 PG 约束错误直接透出（23514 → 500）：%v", err)
	}
	if !strings.Contains(err.Error(), "不足") {
		t.Errorf("错误文案应说明是「不足」，实际 = %q", err.Error())
	}
}

// TestGrantTokenNegativeDeltaNeverAbortsTransaction 覆盖**事务内**那条路径。
//
// 为什么必须单独一条：上一条走 AdminGrantCurrency，
// 判定点在 grantToken 本身；而 Buy 的 deduct 是在**事务内**调用的 ——
// 如果事务已经因 CHECK 崩掉，那笔购买就是「钱扣了、货没发、响应 500」。
// 这一条断言「Buy 返回的是可读的 400，且商品仍然可买」。
func TestGrantTokenNegativeDeltaNeverAbortsTransaction(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := auditProbeUser(t, ts)

	var sid int
	ts.pool.QueryRow(ctx, `SELECT min(id) FROM shop_items WHERE enabled`).Scan(&sid)
	// 运营把道具设成价格 —— `validWalletCurrency` 明确允许这件事。
	//
	// ⚠️ 必须把原价存下来并在结束时还原。
	//
	// 本包**所有**用例共享一个一次性库（第 150 轮的 `packageDSN`），
	// 所以任何「改共享行却��还原」的用例都在给后面的用例埋雷。
	// 实测过一次：本用例把 price 改成 {"mastery_reset":5} 后没还原，
	// 于是后面的 `TestShopDayBoundaryMatchesTaskDayBoundary`
	// （它取 `ORDER BY id LIMIT 1` 的第一个在售商品）挑中了这一件，
	// `Buy` 返回「道具不足」→ **一条与商城日界毫无关系的测试变红**。
	//
	// 症状与「被测代码坏了」几乎无法区分，而这正是本项目反复记的那一类：
	// **让红指向错误的地方**。与 `TestNoScratchNameUsesTimeModulo` 守的
	// 「孤儿库」同源：测试自己污染环境，然后下一个用例替它背锅。
	var origPrice []byte
	if err := ts.pool.QueryRow(ctx, `SELECT price FROM shop_items WHERE id=$1`, sid).Scan(&origPrice); err != nil {
		t.Fatalf("读原价格失败：%v", err)
	}
	t.Cleanup(func() {
		// 还原失败必须**说出来** —— 静默失败会让后面几百个用例接着被污染，
		// 而症状指向的却是完全无关的代码。
		if _, err := ts.pool.Exec(context.Background(),
			`UPDATE shop_items SET price = $2::jsonb WHERE id = $1`, sid, origPrice); err != nil {
			t.Errorf("还原商品 %d 的 price 失败（会污染后续用例）：%v", sid, err)
		}
	})
	ts.exec(t, `UPDATE shop_items SET price = '{"mastery_reset":5}'::jsonb WHERE id=$1`, sid)

	_, err := ts.Service.Buy(ctx, uid, int64(sid))
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("买不起道具商品应报 ErrBadInput（→400），实际 = %v", err)
	}

	// 钱包必须**分文未动**：修复前事务带 CHECK 崩掉会回滚，
	// 而「回滚」是运气 —— 关键是这里断言的是可观测结果，不是内部机制。
	var coin int64
	ts.pool.QueryRow(ctx, `SELECT coin FROM user_wallets WHERE user_id=$1`, uid).Scan(&coin)
	if coin != 2000 {
		t.Errorf("购买失败后金币应为初始 2000，实际 %d", coin)
	}
	var n int
	ts.pool.QueryRow(ctx,
		`SELECT count(*) FROM user_purchases WHERE user_id=$1 AND item_id=$2`, uid, sid).Scan(&n)
	if n != 0 {
		t.Errorf("购买失败不应留下 user_purchases 行，实际 %d 行", n)
	}
}

// TestGrantTokenPositiveDeltaStillUpserts 守住另一半。
//
// 扣减改走条件 UPDATE 之后，**发放**必须仍然是 UPSERT ——
// 道具是稀疏的（没买过的连行都没有），若发放也改成 UPDATE，
// 第一次拿到道具就会因「没有行」而静默不发放（UPDATE 匹配不到行不报错）。
//
// 这一条是防「修 A 破 B」的那一半，只测负数是不够的。
func TestGrantTokenPositiveDeltaStillUpserts(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := auditProbeUser(t, ts)

	// 先确认该用户确实没有这一行道具（排除「本来就有」这个前提）
	var n int
	ts.pool.QueryRow(ctx,
		`SELECT count(*) FROM user_tokens WHERE user_id=$1 AND token='revive_token'`, uid).Scan(&n)
	if n != 0 {
		t.Fatalf("前提不成立：新用户已持有 revive_token")
	}

	if _, err := ts.Service.AdminGrantCurrency(ctx, uid, "revive_token", 3); err != nil {
		t.Fatalf("首次发放应成功（UPSERT 造行），实际 = %v", err)
	}
	bal, err := ts.Service.TokenBalance(ctx, uid, []string{"revive_token"})
	if err != nil {
		t.Fatalf("TokenBalance: %v", err)
	}
	if bal["revive_token"] != 3 {
		t.Errorf("revive_token 余额 = %d，期望 3", bal["revive_token"])
	}
}

// TestGrantTokenPartialDebitWorks：扣减必须能**部分**成功，不是全有全无。
func TestGrantTokenPartialDebitWorks(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := auditProbeUser(t, ts)

	if _, err := ts.Service.AdminGrantCurrency(ctx, uid, "gem_wash_token", 5); err != nil {
		t.Fatalf("发放失败：%v", err)
	}
	if _, err := ts.Service.AdminGrantCurrency(ctx, uid, "gem_wash_token", -2); err != nil {
		t.Fatalf("扣减 2 应成功，实际 = %v", err)
	}
	bal, _ := ts.Service.TokenBalance(ctx, uid, []string{"gem_wash_token"})
	if bal["gem_wash_token"] != 3 {
		t.Errorf("余额 = %d，期望 3（5-2）", bal["gem_wash_token"])
	}
	// 超出余额必须被拒，且余额不能变成负数
	if _, err := ts.Service.AdminGrantCurrency(ctx, uid, "gem_wash_token", -4); !errors.Is(err, ErrBadInput) {
		t.Errorf("超出余额应报 ErrBadInput，实际 = %v", err)
	}
	bal, _ = ts.Service.TokenBalance(ctx, uid, []string{"gem_wash_token"})
	if bal["gem_wash_token"] != 3 {
		t.Errorf("被拒后余额必须保持 3，实际 %d", bal["gem_wash_token"])
	}
}

// TestAdminGrantRejectsAbsurdAmount 守住金额上界。
//
// 判据：不是「返回了错误」，而是**返回 ErrBadInput**。
// 一个恒返回 nil 的实现（真的写进去）能让前者绿，
// 而「手滑大数把整笔发放打成 500」正是要防的形态。
func TestAdminGrantRejectsAbsurdAmount(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := auditProbeUser(t, ts)

	for _, amt := range []int64{MaxAdminGrantAmount + 1, -(MaxAdminGrantAmount + 1), 1 << 62} {
		_, err := ts.Service.AdminGrantCurrency(ctx, uid, "coin", amt)
		if !errors.Is(err, ErrBadInput) {
			t.Errorf("amount=%d 应被 ErrBadInput 拒绝，实际 = %v", amt, err)
		}
	}
	// 边界内必须仍然放行
	if _, err := ts.Service.AdminGrantCurrency(ctx, uid, "coin", MaxAdminGrantAmount); err != nil {
		t.Errorf("上限本身应放行，实际 = %v", err)
	}
}

// TestLogoutRevokesAllRefreshTokens 守住登出。
//
// 判据是「refresh token 再也换不出 access token」——
// 直接断言 `Refresh` 失败，而不是断言 `refresh_tokens` 表被更新了：
// 后者是**实现形状**，前者是**安全后果**。
// 一个只 UPDATE 表但没影响签发逻辑的实现能让前者绿。
func TestLogoutRevokesAllRefreshTokens(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 两次登录 → 两条 refresh token
	tp1, err := ts.Service.GuestLogin(ctx, "audit_probe_logout_1", "a")
	if err != nil {
		t.Fatalf("GuestLogin: %v", err)
	}
	tp2, err := ts.Service.GuestLogin(ctx, "audit_probe_logout_1", "a")
	if err != nil {
		t.Fatalf("GuestLogin 2: %v", err)
	}
	if tp1.User.ID != tp2.User.ID {
		t.Fatalf("同一 guest_token 应恢复同一账号")
	}

	// ⚠️ 这里**不**先调 Refresh：Refresh 会消费掉一条**并再签发一条**，
	// 于是待吊销数从 2 变成 3（2 条旧 + 1 条新），
	// 「Logout 应该吊销几条」的断言就会变成一个需要心算的数 ——
	// 而心算正是这类测试最容易写错的地方（我第一版就写错了）。
	// 本条只关心安全后果（别人的令牌再也换不出 access token），
	// 条数一致性由下一条 `TestLogoutRevokesAllRefreshTokens_Counts` 单独守。
	res, err := ts.Service.Logout(ctx, tp1.User.ID)
	if err != nil {
		t.Fatalf("Logout: %v", err)
	}
	if res.RevokedTokens != 2 {
		t.Errorf("两次登录 = 两条未吊销令牌，应吊销 2 条，实际 = %d", res.RevokedTokens)
	}

	for _, tok := range []string{tp1.RefreshToken, tp2.RefreshToken} {
		if _, err := ts.Service.Refresh(ctx, tok); err == nil {
			t.Fatal("登出后 refresh 必须失败（令牌已被吊销）")
		}
	}
	// 幂等：重复登出不报错
	if _, err := ts.Service.Logout(ctx, tp1.User.ID); err != nil {
		t.Errorf("重复登出应幂等返回，实际 = %v", err)
	}
}

// TestLogoutRevokesAllRefreshTokens_Counts 把「Refresh 会换发令牌」这件事
// 显式记成一条断言。
//
// 单独成条的理由：它是一条**易错的前置事实**，而它出错会让
// 上一条的条数断言看起来像「Logout 少吊销了」——
// 排查方向会滑向「Logout 写错了 WHERE」，而不是「我把前因算错了」。
func TestLogoutRevokesAllRefreshTokens_Counts(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	tp, err := ts.Service.GuestLogin(ctx, uniqueGuestToken(), "a")
	if err != nil {
		t.Fatalf("GuestLogin: %v", err)
	}
	if _, err := ts.Service.Refresh(ctx, tp.RefreshToken); err != nil {
		t.Fatalf("Refresh: %v", err)
	}
	// Refresh 消费旧的、换发新的 → 未吊销的仍然是 1 条，不是 0 也不是 2。
	res, err := ts.Service.Logout(ctx, tp.User.ID)
	if err != nil {
		t.Fatalf("Logout: %v", err)
	}
	if res.RevokedTokens != 1 {
		t.Errorf("Refresh 换发后未吊销令牌应为 1 条，实际 = %d", res.RevokedTokens)
	}
}

// --- 游客凭证轮换（R3）---

// TestLogoutRotatesGuestToken 守住凭证轮换。
//
// # 为什么这是独立的一条而不是上面那条的附带断言
//
// 旧 guest_token 与口令同级：给出它调 `POST /auth/guest` 就能拿到该账号的
// access + refresh token（`TestGuestTokenIsABearerCredential` 证明这一点）。
//
// 它此前**永远不变、没有任何吊销手段** —— 所以「登出」对持有旧凭证的人
// 毫无意义。若把这条只写成上面那条的附带断言，将来有人把轮换单独删掉
// （理由可能是「重装后进度没了」）时，那条仍会绿。
func TestLogoutRotatesGuestToken(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	guest := uniqueGuestToken()

	tp, err := ts.Service.GuestLogin(ctx, guest, "rotate")
	if err != nil {
		t.Fatalf("GuestLogin: %v", err)
	}
	res, err := ts.Service.Logout(ctx, tp.User.ID)
	if err != nil {
		t.Fatalf("Logout: %v", err)
	}
	if res.GuestToken == "" {
		t.Fatal("登出必须返回轮换后的 guest_token")
	}
	if res.GuestToken == guest {
		t.Fatal("登出后 guest_token 没有变 —— 旧凭证仍然可用，登出等于没做")
	}

	// 关键判据是**账号归属**，不是「有没有报错」。
	//
	// ⚠️ 我第一版断言的是「旧凭证登录必须报错」—— 那是错的：
	//   `GuestLogin` 对**未知** guest_token 的语义是**建一个新号**
	//   （ON CONFLICT ... RETURNING id，无匹配则 INSERT）。
	//   所以它永远不会因凭证作废而报错，而是安静地返回一个**空账号**。
	//
	//   两种断言的差别很要紧：
	//     - 「必须报错」→ 描述的是一个**不存在**的行为
	//     - 「必须不回到受害者账号」→ 描述的是**安全后果**
	//
	//   而这个形态本身值得写进文档：**凭证轮换之后，
	//   拿旧凭证来的人不会看到任何异常，只会得到一个空号。**
	//   这正是客户端必须在登出后**立刻存下新凭证**的原因 ——
	//   否则受害者自己也走这条路，进度凭空消失且毫无报错。
	stolen, err := ts.Service.GuestLogin(ctx, guest, "attacker")
	if err != nil {
		t.Fatalf("用未知凭证登录应建新号而非报错，实际 = %v", err)
	}
	if stolen.User.ID == tp.User.ID {
		t.Fatal("旧 guest_token 仍回到了受害者账号 —— 轮换没生效")
	}

	// 更强的一条：那个新号必须是**空的**。
	// 「不同的账号」+「带着受害者的金币」两件事同时发生的话，
	// 说明服务端把数据复制了过去 —— 那是比凭证没轮换更糟的情况。
	var coin int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT coin FROM user_wallets WHERE user_id = $1`, stolen.User.ID).Scan(&coin); err != nil {
		t.Fatalf("读新号钱包失败：%v", err)
	}
	if coin != 2000 {
		t.Errorf("用旧凭证建出的新号金币 = %d，应为新号初始值 2000 —— "+
			"说明受害者数据被复制过去了", coin)
	}

	// 而新凭证必须恢复**同一个账号** —— 轮换不等于换号。
	again, err := ts.Service.GuestLogin(ctx, res.GuestToken, "rotate")
	if err != nil {
		t.Fatalf("新凭证应能登录：%v", err)
	}
	if again.User.ID != tp.User.ID {
		t.Errorf("新凭证恢复了另一个账号（%d ≠ %d）—— 轮换把玩家换号了",
			again.User.ID, tp.User.ID)
	}
}

// TestGuestTokenIsABearerCredential 把「guest_token 与口令同级」这件事
// 写成可执行事实。
//
// 为什么值得单独一条：它是上面那条的**依据**。
// 若将来 guest_token 变成「不可单独登录的随机 id」，
// TestLogoutRotatesGuestToken 的「旧凭证登不进去」会因为**别的原因**通过
// （本来就登不进去），于是那条守卫失去意义 —— 而这里会先红，
// 提醒「依据变了，上面那条要重写」。
func TestGuestTokenIsABearerCredential(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	guest := uniqueGuestToken()

	tp, err := ts.Service.GuestLogin(ctx, guest, "owner")
	if err != nil {
		t.Fatalf("GuestLogin: %v", err)
	}
	// 任何持有该凭证的人都能拿到可用的 access token —— 没有任何第二因素。
	stolen, err := ts.Service.GuestLogin(ctx, guest, "someone-else")
	if err != nil {
		t.Fatalf("重入应成功：%v", err)
	}
	if stolen.AccessToken == "" || stolen.User.ID != tp.User.ID {
		t.Fatalf("持有 guest_token 应能取回同一账号的令牌，实际 user=%d",
			stolen.User.ID)
	}
}

// TestAdminSearchNeverMatchesGuestToken 守住「凭证不可检索」这条不变式。
//
// 为什么值得单独一条：把 guest_token 从后台搜索里去掉是**功能删减**，
// 而删减的东西最容易被下一个人「加回来」——
// 因为「按凭证找人」听起来像个合理需求。
//
// 判据落在**行为**上：拿一个只可能命中 guest_token 的关键词去搜，
// 结果必须是 0 条。
func TestAdminSearchNeverMatchesGuestToken(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 一个昵称与关键词**毫无关系**的用户；关键词只可能命中它的 guest_token。
	const gt = "R3SEARCHONLYguesttokenvalue"
	const nick = "昵称完全无关的用户"
	ts.exec(t,
		`INSERT INTO users (guest_token, nickname, is_guest, status)
		 VALUES ($1,$2,TRUE,1)`, gt, nick)

	if _, total, err := ts.AdminListUsers(ctx, gt, 200, 0); err != nil {
		t.Fatalf("搜索失败：%v", err)
	} else if total != 0 {
		t.Errorf("按 guest_token 搜到了 %d 个用户 —— 凭证又回到了检索面。\n"+
			"guest_token 与口令同级，后台一旦可检索它，\n"+
			"后台账号被盗即等于「批量导出所有玩家凭证」", total)
	}
	// 反向：昵称仍然可搜（别把功能删过头）
	if _, total, err := ts.AdminListUsers(ctx, nick, 200, 0); err != nil {
		t.Fatalf("按昵称搜索失败：%v", err)
	} else if total != 1 {
		t.Errorf("按昵称应仍能搜到 1 个，实际 %d —— 功能删过头了", total)
	}
}

// TestLogoutOnlyTouchesItsOwnUser 守住越权面。
//
// 登出必须只吊销**调用者自己**的令牌。
// 一个写错 WHERE 子句（少 user_id 条件）的实现能让上一条绿，
// 却会把全服玩家一次性全部登出 —— 所以必须单独钉住。
func TestLogoutOnlyTouchesItsOwnUser(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	other, err := ts.Service.GuestLogin(ctx, "audit_probe_logout_other", "other")
	if err != nil {
		t.Fatalf("GuestLogin: %v", err)
	}
	victim, err := ts.Service.GuestLogin(ctx, "audit_probe_logout_victim", "victim")
	if err != nil {
		t.Fatalf("GuestLogin: %v", err)
	}

	if _, err := ts.Service.Logout(ctx, other.User.ID); err != nil {
		t.Fatalf("Logout: %v", err)
	}
	if _, err := ts.Service.Refresh(ctx, victim.RefreshToken); err != nil {
		t.Fatalf("别人的令牌不应被吊销，实际 = %v", err)
	}
}

// auditProbeSeq 保证同一次跑测里的 guest_token 互不相同。
//
// ⚠️ 为什么不能直接用 `t.Name()`：同一包内的用例共享一张表，
// 而 `-count=2` 或重跑会把名字用掉第二遍，于是撞上 `users.guest_token`
// 的唯一约束（23505）—— 症状是「第二次跑必红」，与代码对错无关。
var auditProbeSeq atomic.Int64

// uniqueGuestToken 生成进程内唯一的游客凭证。
func uniqueGuestToken() string {
	return fmt.Sprintf("g_audit_%d", auditProbeSeq.Add(1))
}

// auditProbeUser 造一个带完整初始数据的新账号。
func auditProbeUser(t *testing.T, ts *testService) int64 {
	t.Helper()
	tok := uniqueGuestToken()
	ts.exec(t, `INSERT INTO users (guest_token, nickname, is_guest) VALUES ($1,$2,TRUE)`,
		tok, "audit_"+tok)
	var id int64
	ts.pool.QueryRow(context.Background(),
		`SELECT id FROM users WHERE guest_token=$1`, tok).Scan(&id)
	if err := ts.Service.initNewUser(context.Background(), id); err != nil {
		t.Fatalf("initNewUser: %v", err)
	}
	return id
}
