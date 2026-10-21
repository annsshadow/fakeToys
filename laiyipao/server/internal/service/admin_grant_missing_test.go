package service

import (
	"context"
	"errors"
	"strings"
	"testing"
)

// TestAdminGrantCurrencyReportsMissingUser 钉住 P3-2。
//
// # 背景
//
// 「给一个**不存在**的用户发币」此前报的是：
//
//	coin 不足（需要 100）
//
// 那条消息来自 `grantWallet` 把「钱包行读不到」（pgx.ErrNoRows）
// 翻译成了「余额不足」。它与真实原因**完全无关** —— 运营会以为自己
// 发多了、或者怀疑货币名拼错了，而真相是「这个用户 id 不存在」。
//
// 一条与真实原因无关的错误消息，比没有消息更坏：它把人引向错误的排查方向。
//
// # 判据
//
// 不存在的用户 → ErrNotFound，且消息里说「用户」与「不存在」。
func TestAdminGrantCurrencyReportsMissingUser(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	_, err := ts.AdminGrantCurrency(ctx, 999_999_999, "coin", 100)
	if err == nil {
		t.Fatal("给不存在的用户发币应报错")
	}
	if !errors.Is(err, ErrNotFound) {
		t.Errorf("不存在的用户应 ErrNotFound，实际 %v", err)
	}
	// 消息必须说清是「用户不存在」，而不是「余额不足」。
	msg := err.Error()
	if strings.Contains(msg, "不足") {
		t.Errorf("错误消息说的是「余额不足」，与真实原因无关：%v", err)
	}
	if !strings.Contains(msg, "不存在") {
		t.Errorf("错误消息应说明「用户不存在」，实际：%v", err)
	}
}
