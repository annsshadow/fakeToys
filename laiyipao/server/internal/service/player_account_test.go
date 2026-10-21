package service

// 玩家账号（用户名 + 密码）的服务层测试。
//
// 关键行为锚点：
//   - 正确口令 → 拿到与游客登录同一种令牌（kind=user）；
//   - 口令错与「用户不存在」返回**同一条**错误（防用户名枚举）；
//   - 封禁的账号即使口令正确也登不进；
//   - **重复引导不碰玩家数据** —— 这是整个功能的前提：
//     「测试数据散落/丢失」正是它要解决的事，重建账号等于把它清空。

import (
	"context"
	"errors"
	"fmt"
	"testing"
	"time"
)

func TestPlayerLoginSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	username := fmt.Sprintf("login_%d", time.Now().UnixNano())
	const password = "test-password-12345"

	created, err := ts.EnsureBootstrapPlayer(ctx, username, password)
	if err != nil {
		t.Fatalf("引导玩家账号失败：%v", err)
	}
	if !created {
		t.Fatal("首次引导应报告「新建」")
	}

	tp, err := ts.Login(ctx, username, password)
	if err != nil {
		t.Fatalf("口令正确却登录失败：%v", err)
	}
	if tp.User.ID == 0 || tp.AccessToken == "" || tp.RefreshToken == "" {
		t.Fatalf("令牌对不完整：%+v", tp)
	}
	if tp.User.IsGuest {
		t.Error("账号登录拿到的应是账号身份（is_guest=false）")
	}
	// 新号必须发新手包 —— 否则「登录成功但进游戏是死局」。
	w := ts.wallet(t, ctx, tp.User.ID)
	if w.Coin != 2000 || w.Energy != 30 {
		t.Errorf("新账号初始钱包 = %+v", w)
	}
}

// TestPlayerLoginDoesNotLeakUserExistence 钉住「不枚举用户名」。
//
// 用户不存在与口令错误必须返回**同一条**错误。
// 分开报等于免费送一个用户名枚举器：攻击者能据此判断哪些账号存在。
func TestPlayerLoginDoesNotLeakUserExistence(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	username := fmt.Sprintf("enum_%d", time.Now().UnixNano())
	const password = "test-password-12345"
	if _, err := ts.EnsureBootstrapPlayer(ctx, username, password); err != nil {
		t.Fatalf("引导玩家账号失败：%v", err)
	}

	_, wrongPassErr := ts.Login(ctx, username, "wrong-password-0000")
	_, noSuchUserErr := ts.Login(ctx, username+"-不存在", password)

	if !errors.Is(wrongPassErr, ErrUnauthorized) {
		t.Errorf("口令错应是 ErrUnauthorized，实际 %v", wrongPassErr)
	}
	if !errors.Is(noSuchUserErr, ErrUnauthorized) {
		t.Errorf("用户不存在应是 ErrUnauthorized，实际 %v", noSuchUserErr)
	}
	if wrongPassErr.Error() != noSuchUserErr.Error() {
		t.Errorf("两条错误信息必须完全一致，否则就是用户名枚举器：\n 口令错：%v\n 不存在：%v",
			wrongPassErr, noSuchUserErr)
	}
}

// TestPlayerLoginBannedAccount 封禁的账号即使口令正确也登不进。
//
// 顺序很关键：**先验口令再查状态**。
// 反过来（先查状态）会让「账号已停用」这条错误成为枚举信号。
func TestPlayerLoginBannedAccount(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	username := fmt.Sprintf("banned_%d", time.Now().UnixNano())
	const password = "test-password-12345"
	if _, err := ts.EnsureBootstrapPlayer(ctx, username, password); err != nil {
		t.Fatalf("引导玩家账号失败：%v", err)
	}
	if _, err := ts.pool.Exec(ctx, `UPDATE users SET status = 2 WHERE username = $1`, username); err != nil {
		t.Fatalf("封禁账号失败：%v", err)
	}

	if _, err := ts.Login(ctx, username, password); !errors.Is(err, ErrForbidden) {
		t.Errorf("封禁账号应被拒（ErrForbidden），实际 %v", err)
	}
	// 口令错 + 封禁 → 仍然是「用户名或密码错误」，不能泄露封禁状态。
	_, err := ts.Login(ctx, username, "wrong-password-0000")
	if err == nil || err.Error() == "" {
		t.Fatal("口令错必须被拒")
	}
	if errors.Is(err, ErrForbidden) {
		t.Errorf("口令错时不应暴露「已封禁」：%v", err)
	}
}

// TestEnsureBootstrapPlayerKeepsPlayerData 是这个功能的**核心守卫**。
//
// 重复引导（每次启动都会跑）只能同步口令，**绝不能重建账号** ——
// 重建会把钱包、关卡进度、专精、战报、防线全清空，
// 而「测试数据不要丢」正是这个功能存在的理由。
func TestEnsureBootstrapPlayerKeepsPlayerData(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	username := fmt.Sprintf("keep_%d", time.Now().UnixNano())
	const password = "test-password-12345"
	if _, err := ts.EnsureBootstrapPlayer(ctx, username, password); err != nil {
		t.Fatalf("引导玩家账号失败：%v", err)
	}

	tp, err := ts.Login(ctx, username, password)
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}
	uid := tp.User.ID

	// 造一点「测试数据」：发一笔币。
	ts.grant(t, ctx, uid, map[string]int64{"coin": 777})
	before := ts.wallet(t, ctx, uid)
	if before.Coin != 2000+777 {
		t.Fatalf("前置状态异常：%+v", before)
	}

	// 再次引导（模拟重启）：口令同步为新值。
	const newPassword = "another-password-654321"
	created, err := ts.EnsureBootstrapPlayer(ctx, username, newPassword)
	if err != nil {
		t.Fatalf("二次引导失败：%v", err)
	}
	if created {
		t.Error("二次引导不应报告「新建」——重建账号等于清空测试数据")
	}

	after := ts.wallet(t, ctx, uid)
	if after.Coin != before.Coin || after.Energy != before.Energy {
		t.Errorf("二次引导动了玩家数据：%+v → %+v", before, after)
	}

	// 口令必须已同步为新值，且旧口令失效。
	if _, err := ts.Login(ctx, username, newPassword); err != nil {
		t.Errorf("同步后的口令应可登录：%v", err)
	}
	if _, err := ts.Login(ctx, username, password); err == nil {
		t.Error("旧口令应已失效")
	}
}

// TestEnsureBootstrapPlayerRejectsShortPassword 口令太短直接拒绝。
//
// 不是「安全强度」判断（长度不等于强度），而是防配置写错：
// 一个 1 位口令被写进库，通常意味着环境变量没配对。
func TestEnsureBootstrapPlayerRejectsShortPassword(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	username := fmt.Sprintf("short_%d", time.Now().UnixNano())
	if _, err := ts.EnsureBootstrapPlayer(ctx, username, "short"); err == nil {
		t.Error("口令短于 10 位应被拒绝")
	}
}
