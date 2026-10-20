package service

// 防线挑战的构筑 S 段校验（D4，2026-10-20，DECISIONS D4）。
//
// 产品决策走**中间档**：不跑服务端引擎（I-5 取舍不变），但把
// 「挑战者谎报一套与出战配置不同的构筑」这条最常见的作弊面堵上。
//
// 判据与结算面同源：`replay_skills` 非空时，必须与服务端按
// `user_skill_slots` 重算的 S 段**逐字相等**；空串放行（老客户端）。
//
// 抓不到什么（诚实标注）：**胜负本身**仍然由客户端裁决 ——
// 谎报 won=true 而构筑正确，这条校验不会拦。后台的胜率异常告警仍是
// 那条风险的唯一兜底（KNOWN_LIMITS 第 3 条）。

import (
	"context"
	"errors"
	"strings"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// mkDefense 建一条可挑战的防线（owner 拥有三元素入门包）。
func mkDefense(t *testing.T, ts *testService, ctx context.Context) (owner int64, defenseID int64) {
	t.Helper()
	owner = ts.newUser(t, ctx)
	dv, err := ts.SaveDefense(ctx, owner, SaveDefenseInput{Name: "S段防线", Skills: []int{1, 4, 7}})
	if err != nil {
		t.Fatalf("建防线失败：%v", err)
	}
	return owner, dv.ID
}

// validChallengeWithSkills 造一份除 S 段外都合法的挑战上报。
func validChallengeWithSkills(replaySkills string) ChallengeInput {
	return ChallengeInput{
		Seed: "1", Won: true, DurationMs: 60_000, HPLeftPct: 100,
		ReplayHash: "0000000000000000", ReplaySkills: replaySkills,
	}
}

// TestChallengeRejectsForgedReplaySkills 谎报构筑必须被拒（422 哨兵）。
func TestChallengeRejectsForgedReplaySkills(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	_, defID := mkDefense(t, ts, ctx)
	foe := ts.newUser(t, ctx)

	_, err := ts.ChallengeDefense(ctx, foe, defID, validChallengeWithSkills("0:999:1:1:1"))
	if !errors.Is(err, ErrReplaySkillsMismatch) {
		t.Fatalf("谎报构筑应 ErrReplaySkillsMismatch，实际 %v", err)
	}
}

// TestChallengeAcceptsMatchingReplaySkills 正确 S 段必须放行（不能误伤诚实玩家）。
func TestChallengeAcceptsMatchingReplaySkills(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	_, defID := mkDefense(t, ts, ctx)
	foe := ts.newUser(t, ctx)

	expect, err := ts.replaySkillsSegment(ctx, foe)
	if err != nil {
		t.Fatalf("重算 S 段失败：%v", err)
	}
	if expect == "" {
		t.Fatalf("新号出战槽位非空，S 段不应为空串")
	}
	if _, err := ts.ChallengeDefense(ctx, foe, defID, validChallengeWithSkills(expect)); err != nil {
		t.Fatalf("正确 S 段应放行，实际 %v", err)
	}
}

// TestChallengeEmptyReplaySkillsStillAllowed 空串放行（老客户端兼容）。
//
// 这条是**兼容性判据**而不是安全判据：空串等于「没上报」，
// 与结算面同一口径 —— 老客户端不上报这个字段，直接拒会把它们全部锁死。
func TestChallengeEmptyReplaySkillsStillAllowed(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	_, defID := mkDefense(t, ts, ctx)
	foe := ts.newUser(t, ctx)

	if _, err := ts.ChallengeDefense(ctx, foe, defID, validChallengeWithSkills("")); err != nil {
		t.Fatalf("空串应放行（老客户端），实际 %v", err)
	}
}

// TestChallengeReplaySkillsLengthBound S 段长度上界与结算面同一个常量。
func TestChallengeReplaySkillsLengthBound(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	_, defID := mkDefense(t, ts, ctx)
	foe := ts.newUser(t, ctx)

	huge := strings.Repeat("1", domain.ReplaySkillsMaxLen+1)
	_, err := ts.ChallengeDefense(ctx, foe, defID, validChallengeWithSkills(huge))
	if !errors.Is(err, domain.ErrInvalidField) {
		t.Fatalf("超长 S 段应 ErrInvalidField，实际 %v", err)
	}
}
