package service

import (
	"context"
	"errors"
	"fmt"
	"regexp"
	"strings"
	"testing"
	"time"
)

// 战报复现信息与验真的**归属校验**（第 75 轮）。
//
// # 缺陷：两个端点都能遍历任意玩家的战报
//
// `GetReplay` 与 `compareAndRecord` 原来都只有 `WHERE br.id = $1`：
//
//   - `v.userID` 只被用来**记录**「谁验的」，不参与**授权**读哪条战报
//   - `GetReplay` 连 userID 都没有，签名里就没有这个概念
//
// 而 `battle_records.id` 是**连续 BIGSERIAL**，任何注册用户都能遍历。
// `GetReplay` 的源码注释甚至自己写着这一点：
//
//	「battle_records.id 是连续 BIGSERIAL，任何注册用户都能遍历，
//	  所以这不是理论风险。」
//
// 它据此给 `build_snapshot` 剥掉了 `settle_input` ——
// 但**构筑本身还在**：装备、词缀、技能等级，全是别人的。
//
// # 后果分级（诚实标注）
//
// 能读到的：别人的 level_id / seed / expected_hash / build_snapshot / card_picks。
// 能做的：往 `replay_verifications` 里写进自己发的 actual_hash，
//　　　　把别人的战报标成「验真不通过」。
//
// 读到什么 ≠ 能伪造什么：结算校验看的是 plausibility（击杀率、得分速率），
// 不是逐帧重放，所以「照抄别人的构筑」不等于「照抄别人的分数」。
// 但构筑泄露本身是实打实的竞争力问题 ——
// 它是「满级装备 + 关卡 + 种子 + 选牌序列」的一份完整蓝图。
//
// # 为什么给运营保留无过滤路径
//
// `adm.Post("/battles/:id/verify")` 与 adminBattleDetail 本来就需要跨玩家查。
// 归属校验的语义是「玩家只能查自己的」，不是「只有管理员能查」。
//
// ⚠️ 但**不能**做成 `GetReplay(ctx, uid, id, isAdmin bool)` 那种参数 ——
// 漏传时是**静默放行**，而那正是本轮修掉的洞。
// 所以拆成 `GetReplay` / `AdminGetReplay` 两个方法（见 economy.go）。

// newOwnedBattle 造一条属于 owner 的战报，返回 battle_id。
func newOwnedBattle(t *testing.T, ts *testService, owner int64) int64 {
	t.Helper()
	ctx := context.Background()
	ts.grant(t, ctx, owner, map[string]int64{"energy": 100})
	tp, err := ts.StartBattle(ctx, owner, 1)
	if err != nil {
		t.Skipf("起局失败：%v", err)
	}
	if _, err := ts.SettleBattle(ctx, owner, tp.TokenID, loseSettleInput()); err != nil {
		t.Fatalf("结算失败：%v", err)
	}
	var id int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM battle_records WHERE user_id = $1 ORDER BY id DESC LIMIT 1`, owner).
		Scan(&id); err != nil {
		t.Fatalf("取 battle_id 失败：%v", err)
	}
	return id
}

// newAdminUser 造一个 admin_users 行。
//
// ⚠️ 必须是 `admin_users` 而不是 `users`：`replay_verifications`
// 的 `admin_verifier_id` 外键指向 `admin_users`。
// 我第一版用 `ts.newUser()`，报的是 FK 违反 ——
// 那说明这条守卫顺带确认了「两侧验真人真的存在不同的表里」。
func newAdminUser(t *testing.T, ts *testService) int64 {
	t.Helper()
	var id int64
	if err := ts.pool.QueryRow(context.Background(),
		`INSERT INTO admin_users (username, password_hash, role)
		 VALUES ($1, 'x', 'admin') RETURNING id`,
		fmt.Sprintf("probe_%d", time.Now().UnixNano())).Scan(&id); err != nil {
		t.Fatalf("造 admin 失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(),
			`DELETE FROM admin_users WHERE id = $1`, id)
	})
	return id
}

// TestGetReplayRejectsOtherPlayersBattle 是本文件的核心。
func TestGetReplayRejectsOtherPlayersBattle(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	owner := ts.newUser(t, ctx)
	attacker := ts.newUser(t, ctx)
	battleID := newOwnedBattle(t, ts, owner)

	t.Run("本人可读", func(t *testing.T) {
		info, err := ts.GetReplay(ctx, owner, battleID)
		if err != nil {
			t.Fatalf("本人读自己的战报应成功：%v", err)
		}
		if info.BattleID != battleID {
			t.Errorf("BattleID = %d，期望 %d", info.BattleID, battleID)
		}
	})

	t.Run("他人不可读", func(t *testing.T) {
		info, err := ts.GetReplay(ctx, attacker, battleID)
		if err == nil {
			t.Fatalf("读到别人的战报了！Build = %v", info.Build)
		}
		if !errors.Is(err, ErrNotFound) {
			t.Errorf("应报 ErrNotFound，实得 %v", err)
		}
		if len(info.Build) != 0 || info.LevelID != 0 {
			t.Errorf("失败时不应带出任何字段：Build=%v LevelID=%d", info.Build, info.LevelID)
		}
	})
}

// TestVerifyReplayRejectsOtherPlayersBattle 覆盖验真侧。
func TestVerifyReplayRejectsOtherPlayersBattle(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	owner := ts.newUser(t, ctx)
	attacker := ts.newUser(t, ctx)
	battleID := newOwnedBattle(t, ts, owner)

	var expected string
	if err := ts.pool.QueryRow(ctx,
		`SELECT replay_hash FROM battle_records WHERE id = $1`, battleID).Scan(&expected); err != nil {
		t.Fatal(err)
	}

	if _, err := ts.VerifyReplay(ctx, attacker, battleID, expected); err == nil {
		t.Fatal("他人验真别人的战报成功了")
	} else if !errors.Is(err, ErrNotFound) {
		t.Errorf("应报 ErrNotFound，实得 %v", err)
	}

	// ⚠️ 更重要的：失败时**不能留下任何记录**。
	//
	// 归属校验写在 SQL 的 WHERE 里（而不是「先查再比」），
	// 所以 ErrNoRows 之后不会走到 recordVerification。
	// 若将来有人改成「先查再比」，这里会红。
	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM replay_verifications WHERE battle_id = $1`, battleID).
		Scan(&n); err != nil {
		t.Fatal(err)
	}
	if n != 0 {
		t.Errorf("被拒的验真留下了 %d 条记录 —— 攻击者能把自己的 actual_hash 写到别人的战报上", n)
	}
}

// TestAdminPathsSkipOwnership 守住运营侧没有被误伤。
//
// 这条是被「归属校验加得太狠」的风险逼出来的：
// 如果实现时顺手给 admin 也加了 user_id 过滤，
// 运营就再也查不了玩家的战报 —— 那是个更难发现的回归。
func TestAdminPathsSkipOwnership(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	owner := ts.newUser(t, ctx)
	battleID := newOwnedBattle(t, ts, owner)
	admin := newAdminUser(t, ts)

	info, err := ts.AdminGetReplay(ctx, battleID)
	if err != nil {
		t.Fatalf("运营读战报失败：%v", err)
	}
	if info.BattleID != battleID {
		t.Errorf("BattleID = %d，期望 %d", info.BattleID, battleID)
	}

	var expected string
	if err := ts.pool.QueryRow(ctx,
		`SELECT replay_hash FROM battle_records WHERE id = $1`, battleID).Scan(&expected); err != nil {
		t.Fatal(err)
	}
	// 运营故意报一个错的 hash -> 应当 matched=false 而不是被归属拦下
	res, err := ts.AdminVerifyReplay(ctx, admin, battleID, "ffffffffffffffff")
	if err != nil {
		t.Fatalf("运营验真失败：%v", err)
	}
	if res.Matched {
		t.Error("运营报错的 hash 竟然判成匹配")
	}
	if res.LevelID == 0 || res.Seed == "" {
		t.Errorf("运营验真应带出战报字段：%+v", res)
	}

	// 流水里 verifier_id 必须为空、admin_verifier_id 非空
	var userArg, adminArg *int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT verifier_id, admin_verifier_id FROM replay_verifications
		  WHERE battle_id = $1 ORDER BY id DESC LIMIT 1`, battleID).
		Scan(&userArg, &adminArg); err != nil {
		t.Fatal(err)
	}
	if userArg != nil {
		t.Errorf("运营的验真不该写 verifier_id，实得 %d", *userArg)
	}
	if adminArg == nil || *adminArg != admin {
		t.Errorf("admin_verifier_id 应为 %d", admin)
	}
}

// TestNotOwnedAndNotExistingAreIndistinguishable 防预言机。
//
// 若两者返回不同错误，`ErrForbidden` vs `ErrNotFound` 就让人能
// 二分出「某个 id 是否存在」—— 在连续 BIGSERIAL 上是 O(1) 的信息。
func TestNotOwnedAndNotExistingAreIndistinguishable(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	owner := ts.newUser(t, ctx)
	attacker := ts.newUser(t, ctx)
	battleID := newOwnedBattle(t, ts, owner)

	// ⚠️ 比较时必须把 **battle_id 归一化掉**。
	//
	// 错误信息里回显了 id（`资源不存在: battle %d`），而 id 是**调用方自己
	// 传进来的** —— 回显它不泄露任何东西。
	// 泄露的是「同一个 id 两种错误」这件事：
	// `ErrNotFound` vs `ErrForbidden` 会让人二分出「这局是否存在」。
	//
	// 我第一版直接比 `err.Error()` 逐字，红了——而那不是预言机，
	// 是我把「回显调用方自己的输入」误当成了泄露。
	norm := func(err error) string {
		return regexp.MustCompile(`\d+`).ReplaceAllString(err.Error(), "<id>")
	}
	classOf := func(err error) string {
		switch {
		case errors.Is(err, ErrNotFound):
			return "notfound"
		case errors.Is(err, ErrForbidden):
			return "forbidden"
		case errors.Is(err, ErrBadInput):
			return "badinput"
		default:
			return "other"
		}
	}

	_, notOwned := ts.GetReplay(ctx, attacker, battleID)
	_, notExist := ts.GetReplay(ctx, attacker, 999999999)
	if notOwned == nil || notExist == nil {
		t.Fatal("两者都应当失败")
	}
	if classOf(notOwned) != classOf(notExist) {
		t.Errorf("「不是你的」(%s) 与「不存在」(%s) 的错误类别不同 —— "+
			"这就是一个「这局是否存在」的预言机：\n  %v\n  %v",
			classOf(notOwned), classOf(notExist), notOwned, notExist)
	}
	if norm(notOwned) != norm(notExist) {
		t.Errorf("错误文案模板必须相同（归一化 id 后）：\n  %q\n  %q",
			norm(notOwned), norm(notExist))
	}

	// 验真侧同理
	_, vNotOwned := ts.VerifyReplay(ctx, attacker, battleID, "0000000000000000")
	_, vNotExist := ts.VerifyReplay(ctx, attacker, 999999999, "0000000000000000")
	if classOf(vNotOwned) != classOf(vNotExist) || norm(vNotOwned) != norm(vNotExist) {
		t.Errorf("验真侧同样必须相同：\n  %v\n  %v", vNotOwned, vNotExist)
	}
}

// TestVerifyRejectsOversizedHash 补上验真面的长度上界。
//
// 结算面早就有（`maxReplayHashLen = 64`），验真面没有 ——
// 同一个语义的字段，两条路径两道不同的规矩。
// BodyLimit 是 1MB，而 `actual_hash` 是 TEXT，
// 于是「每次验真写近 1MB」是可能的。
func TestVerifyRejectsOversizedHash(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	owner := ts.newUser(t, ctx)
	battleID := newOwnedBattle(t, ts, owner)

	cases := []struct {
		name  string
		hash  string
		valid bool
	}{
		{"引擎产出（16 hex）", "0123456789abcdef", true},
		{"上界 64", strings.Repeat("a", 64), true},
		{"上界 +1", strings.Repeat("a", 65), false},
		{"1MB（BodyLimit 内）", strings.Repeat("a", 1<<20), false},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			_, err := ts.VerifyReplay(ctx, owner, battleID, c.hash)
			if c.valid {
				if err != nil {
					t.Fatalf("合法长度被拒：%v", err)
				}
				return
			}
			if err == nil {
				t.Fatal("超长 hash 被放行")
			}
			if !errors.Is(err, ErrBadInput) {
				t.Errorf("应报 ErrBadInput（→422），实得 %v", err)
			}
		})
	}
}

// TestVerifyLengthBoundMatchesSettleFace 钉住两条路径的上界一致。
//
// 结算面（domain 包）与验真面（service 包）各有一份上界。
// 漂了就会出现「结算拒了但验真收下」或反之 ——
// 而它们描述的是**同一个引擎产出的同一个串**。
//
// 判据用**行为**而不是常量引用：常量是私有的，
// 且将来重命名不该让这条测试红。
func TestVerifyLengthBoundMatchesSettleFace(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	owner := ts.newUser(t, ctx)
	battleID := newOwnedBattle(t, ts, owner)

	for _, n := range []int{64, 65} {
		hash := strings.Repeat("a", n)
		_, err := ts.VerifyReplay(ctx, owner, battleID, hash)
		accepted := err == nil
		if accepted != (n <= 64) {
			t.Errorf("%d 字符：验真接受=%v，期望 %v（err=%v）",
				n, accepted, n <= 64, err)
		}
	}
}

// TestVerifyStillRecordsValidAttempts 确认归属校验没把正常路径也堵死。
func TestVerifyStillRecordsValidAttempts(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	owner := ts.newUser(t, ctx)
	battleID := newOwnedBattle(t, ts, owner)

	var expected string
	if err := ts.pool.QueryRow(ctx,
		`SELECT replay_hash FROM battle_records WHERE id = $1`, battleID).Scan(&expected); err != nil {
		t.Fatal(err)
	}
	if _, err := ts.VerifyReplay(ctx, owner, battleID, expected); err != nil {
		t.Fatalf("本人验真自己的战报失败：%v", err)
	}
	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM replay_verifications WHERE battle_id = $1 AND verifier_id = $2`,
		battleID, owner).Scan(&n); err != nil {
		t.Fatal(err)
	}
	if n != 1 {
		t.Errorf("正常验真应落 1 条流水，实得 %d", n)
	}
}
