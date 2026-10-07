package service

// 第 132 轮：防线挑战的 seed 走字符串（与结算面同口径）。
//
// 引擎（defense.ts）用 63-bit bigint 生成种子；int64 超过 2^53 后，
// JS 的 JSON number 解析即失精。旧实现 `ChallengeInput.Seed` 是 int64，
// 客户端 me.vue 又做 `Number(seed)` —— 大种子上报即被截断，
// 落库的 seed 与本地模拟用的 seed 不一致，replay_hash 再也对不上。
//
// 本文件把「seed 字符串解析边界」与「大种子精确落库」钉死。

import (
	"context"
	"errors"
	"strconv"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// TestChallengeSeedStringBounds 校验 seed 字符串的解析边界。
func TestChallengeSeedStringBounds(t *testing.T) {
	cases := []struct {
		seed    string
		wantErr bool
	}{
		{"0", false},
		{"1", false},
		{"9007199254740993", false},      // > 2^53，int64 内合法
		{"9223372036854775807", false},   // int64 最大
		{"", true},                        // 空
		{"-1", true},                      // 负数
		{"abc", true},                     // 非数字
		{"1.5", true},                     // 小数
		{"9223372036854775808", true},     // int64 溢出
	}
	base := ChallengeInput{
		Seed: "0", Won: true, DurationMs: 60_000, HPLeftPct: 100,
		ReplayHash: "0000000000000000",
	}
	for _, c := range cases {
		in := base
		in.Seed = c.seed
		err := ValidateChallengeInput(in)
		if c.wantErr && err == nil {
			t.Errorf("seed=%q 应被拒绝，实际放行", c.seed)
		}
		if !c.wantErr && err != nil {
			t.Errorf("seed=%q 应放行，实际报错 %v", c.seed, err)
		}
	}
	// 空 seed 的报错必须可分类（422 而非 500）
	in := base
	in.Seed = ""
	if err := ValidateChallengeInput(in); !errors.Is(err, domain.ErrInvalidField) {
		t.Errorf("空 seed 应报 ErrInvalidField，实际 %v", err)
	}
}

// TestChallengeSeedLargeValueRoundTrip 是缺陷的核心守卫：
// 一个超过 2^53 的种子必须**精确**落库（而不是被 JS Number 截断后的值）。
func TestChallengeSeedLargeValueRoundTrip(t *testing.T) {
	const bigSeed = "9007199254740993" // 2^53+1，JS Number 会舍成 9007199254740992
	ts := openScratchService(t)
	ctx := context.Background()
	challenger := ts.newUser(t, ctx)
	owner := ts.newUser(t, ctx)
	dv, err := ts.SaveDefense(ctx, owner, SaveDefenseInput{Name: "大种子线", Skills: []int{1}})
	if err != nil {
		t.Fatalf("建防线失败：%v", err)
	}

	_, err = ts.ChallengeDefense(ctx, challenger, dv.ID, ChallengeInput{
		Seed: bigSeed, Won: false, DurationMs: 60_000, HPLeftPct: 50,
		ReplayHash: "0000000000000000",
	})
	if err != nil {
		t.Fatalf("大种子挑战应成功（未胜路径）：%v", err)
	}

	var got int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT seed FROM defense_challenges WHERE defense_id=$1 AND challenger_id=$2 ORDER BY id DESC LIMIT 1`,
		dv.ID, challenger).Scan(&got); err != nil {
		t.Fatalf("读回 seed 失败：%v", err)
	}
	want, _ := strconv.ParseInt(bigSeed, 10, 64)
	if got != want {
		t.Fatalf("落库 seed = %d，期望精确值 %d（被截断即缺陷复现）", got, want)
	}
	// 反向：若落库的是 JS 会截断成的 9007199254740992，说明走了 Number(seed)
	if got == 9007199254740992 {
		t.Fatalf("落库 seed 被截断成 JS 安全整数 %d —— Number(seed) 缺陷未修", got)
	}
}
