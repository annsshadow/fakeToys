package service

import (
	"context"
	"errors"
	"strings"
	"testing"
)

// 第 115 轮：INSERT 的错误必须**分类**，不能一律谎报「今日已签到」。
//
// 修前 SignIn 对 INSERT 的**任何**错误都返回 `今日已签到`：
//
//	_, err := tx.Exec(ctx, `INSERT INTO user_sign_ins ...`)
//	if err != nil {
//	    return fmt.Errorf("%w: 今日已签到", ErrForbidden)  // ← 唯一键冲突之外的错误也这么说
//	}
//
// 连接抖断、CHECK 违反、死锁、磁盘满…… 事务回滚、奖励没发，
// 用户却被告「已签到」→ 以为领过了、不重试 → 7 日奖励静默漏发。
// 「如实报错」永远比「谎报已签」便宜。
//
// 修法：只有 23505（unique_violation）且落在 user_sign_ins 上才算「今日已签」。
// 其余原样上抛。
func TestSignInClassifiesInsertErrors(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	uid := scratch.newUser(t, ctx)

	// 1) 正常首次签到必须成功
	if _, err := scratch.SignIn(ctx, uid); err != nil {
		t.Fatalf("首次签到失败：%v", err)
	}

	// 2) 同一天第二次 = 真·唯一键冲突 → 必须报「今日已签到」（ErrForbidden）
	_, dupErr := scratch.SignIn(ctx, uid)
	if !errors.Is(dupErr, ErrForbidden) {
		t.Fatalf("同日重复签到应 ErrForbidden（唯一键冲突），实际 %v", dupErr)
	}
	if dupErr == nil || !strings.Contains(dupErr.Error(), "今日已签到") {
		t.Errorf("重复签到错误文案应含「今日已签到」，实际 %q", dupErr)
	}

	// 3) 制造一个**非唯一键**的 INSERT 失败：
	//    加一个 day_index 必须 >1000 的 CHECK（NOT VALID：不校验存量行，
	//    否则历史签到行会让 ADD CONSTRAINT 本身失败），让**新** INSERT 撞 CHECK。
	scratch.exec(t, `ALTER TABLE user_sign_ins
		ADD CONSTRAINT check115_day_index CHECK (day_index > 1000) NOT VALID`)
	t.Cleanup(func() {
		scratch.exec(t, `ALTER TABLE user_sign_ins DROP CONSTRAINT IF EXISTS check115_day_index`)
	})
	// 先让该用户当天已签（满足日界），再改 day_index 使下一次 INSERT 撞 CHECK。
	// 由于 CHECK 已存在，任何新的合法 INSERT 都会失败——
	// 用一个尚未签到的新用户触发它。
	uid2 := scratch.newUser(t, ctx)
	_, chkErr := scratch.SignIn(ctx, uid2)
	// 修前：任何 INSERT 错误 → ErrForbidden「今日已签到」。
	// 修后：CHECK 违反不是唯一键冲突 → 必须原样上抛（不是 ErrForbidden）。
	if errors.Is(chkErr, ErrForbidden) {
		t.Errorf("CHECK 违反被误报成「今日已签到」(ErrForbidden)——"+
			"非唯一键的 INSERT 失败必须如实上抛，实际 %v", chkErr)
	}
	if chkErr == nil {
		t.Error("CHECK 违反时签到应失败，实际成功")
	}
}
