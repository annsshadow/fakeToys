package service

// 「replay_skills 校验受闸门管辖」的端到端守卫。
//
// # 为什么这条重要
//
// 它是闸门在**真实结算路径**上的第一次执行 —— 整局引擎尚未移植，
// 于是这是唯一能证明「闸门确实接在业务路径上」的用例。
//
// 若结算代码绕过闸门直接 `return mismatch`（修之前的写法），
// 下面三条会全红。而一个没接在业务路径上的闸门，
// 在整局引擎接进来时才会被发现 —— 那时它已经「上线很久」了。

import (
	"context"
	"errors"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// settleWithReplaySkills 造一局并用给定的 replay_skills 结算。
//
// 复用真实的 SettleBattle，所以闸门、生效档位、台账三者是端到端连通的。
func settleWithReplaySkills(t *testing.T, ts *testService, uid int64, skills string) (SettleResp, error) {
	t.Helper()
	ctx := context.Background()
	tok, err := ts.Service.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Fatalf("StartBattle: %v", err)
	}
	// HPLeft 取满血：这一组用例只关心 replay_skills 那一段校验，
	// 血量必须合法，否则会先在更早的校验上失败，
	// 症状是「测试红了但看不出跟闸门有关」。
	gl := domain.GenerateLevel(tok.Level.ID)
	in := domain.SettleInput{
		Result: "lose", Score: 0, DurationMs: 60000,
		Shots: 1, Hits: 0, HPLeft: int(gl.BaseHP), WaveReached: 1,
		ReplaySkills: skills,
	}
	return ts.Service.SettleBattle(ctx, uid, tok.TokenID, in)
}

// correctReplaySkills 算出该用户此刻**正确**的 S 段。
func correctReplaySkills(t *testing.T, ts *testService, uid int64) string {
	t.Helper()
	got, err := ts.Service.replaySkillsSegment(context.Background(), uid)
	if err != nil {
		t.Fatalf("replaySkillsSegment: %v", err)
	}
	return got
}

// countRecomputeRows 数该用户台账里的行数。
func countRecomputeRows(t *testing.T, ts *testService, uid int64) int {
	t.Helper()
	var n int
	if err := ts.pool.QueryRow(context.Background(),
		`SELECT count(*) FROM settle_recomputes WHERE user_id=$1`, uid).Scan(&n); err != nil {
		t.Fatalf("数台账行数：%v", err)
	}
	return n
}

// TestReplaySkillsMismatchIsRejectedUnderEnforce 守住 enforce 档仍拒发。
//
// ⚠️ 这是本文件最关键的一条：加闸门的**目的是**给「拒发」加一道保险，
//
//	不是取消拒发。若 enforce 下不再拒发，R1 就失去了全部意义。
func TestReplaySkillsMismatchIsRejectedUnderEnforce(t *testing.T) {
	ts := openTestService(t)
	ts.Service.Cfg().SettleRecomputeMode = RecomputeEnforce
	ts.Service.initRecomputeGate()

	uid := recomputeProbeUser(t, ts, "enf")
	_, err := settleWithReplaySkills(t, ts, uid, "0:1:999999:1:20") // 故意错
	if !errors.Is(err, ErrReplaySkillsMismatch) {
		t.Fatalf("enforce 档下不符仍须拒发，实际 err = %v", err)
	}
}

// TestReplaySkillsMismatchIsNotRejectedUnderShadow 守住 shadow 档不拒发。
//
// 这一条与上一条必须**同时**成立 —— 它就是灰度的全部意义：
// 在这一档上可以放心跑，观察台账，确认重算准不准，再切 enforce。
func TestReplaySkillsMismatchIsNotRejectedUnderShadow(t *testing.T) {
	ts := openTestService(t)
	ts.Service.Cfg().SettleRecomputeMode = RecomputeShadow
	ts.Service.initRecomputeGate()

	uid := recomputeProbeUser(t, ts, "shd")
	if _, err := settleWithReplaySkills(t, ts, uid, "0:1:999999:1:20"); err != nil {
		t.Fatalf("shadow 档不应拒发（这正是灰度的意义），实际 err = %v", err)
	}
}

// TestReplaySkillsMismatchIsNotRejectedUnderOff 守住 off 档不拒发。
//
// off = 完全回到旧行为。这与「校验消失」是**两件事**：
// 校验仍然在跑，仍然写台账，只是不再据此拒发。
func TestReplaySkillsMismatchIsNotRejectedUnderOff(t *testing.T) {
	ts := openTestService(t)
	ts.Service.Cfg().SettleRecomputeMode = RecomputeOff
	ts.Service.initRecomputeGate()

	uid := recomputeProbeUser(t, ts, "off")
	if _, err := settleWithReplaySkills(t, ts, uid, "0:1:999999:1:20"); err != nil {
		t.Fatalf("off 档不应拒发，实际 err = %v", err)
	}
}

// TestReplaySkillsMismatchAlwaysLeavesEvidence 守住「无论哪一档都留台账」。
//
// ⚠️ 为什么这条最重要：不留证据就等于把「关掉了防护」这件事**藏起来**。
//
//	off 档下不拒发是完全正当的操作，但它必须**可查** ——
//	否则某天有人把模式设成 off 之后，整站作弊数据源就断了，
//	而运维在界面上看到的和「没人在作弊」**完全一样**。
//
// 判据落在「表里有一行」这个可观测结果上。
func TestReplaySkillsMismatchAlwaysLeavesEvidence(t *testing.T) {
	for _, mode := range []string{RecomputeOff, RecomputeShadow, RecomputeEnforce} {
		t.Run(mode, func(t *testing.T) {
			ts := openTestService(t)
			ts.Service.Cfg().SettleRecomputeMode = mode
			ts.Service.initRecomputeGate()

			uid := recomputeProbeUser(t, ts, "ev_"+mode)
			settleWithReplaySkills(t, ts, uid, "0:1:999999:1:20") // 错误必被拒或被记

			if n := countRecomputeRows(t, ts, uid); n != 1 {
				t.Errorf("%s 档下不符仍须留 1 行台账，实际 %d 行 —— "+
					"不留证据就等于把「关掉了防护」藏起来了", mode, n)
			}

			var (
				fields    []string
				rejected  bool
				rowMode   string
				engineVer string
			)
			if err := ts.pool.QueryRow(context.Background(),
				`SELECT mismatched_fields, rejected, mode, engine_version
				 FROM settle_recomputes WHERE user_id=$1`, uid).
				Scan(&fields, &rejected, &rowMode, &engineVer); err != nil {
				t.Fatalf("读台账：%v", err)
			}
			if len(fields) != 1 || fields[0] != "replay_skills" {
				t.Errorf("mismatched_fields = %v，期望 [replay_skills]", fields)
			}
			// rejected 必须**等于** enforce —— 这正是它与 mode 分开存的理由：
			// 熔断可以在 enforce 下把单次处置降级，此时两列就不相等了。
			if wantEnforce := mode == RecomputeEnforce; rejected != wantEnforce {
				t.Errorf("mode=%s 时 rejected 应为 %v，实际 %v", mode, wantEnforce, rejected)
			}
			if engineVer != RecomputeEngineVersionSkillsOnly {
				t.Errorf("engine_version = %q，期望 %q —— "+
					"没有它就无法把一批不符对应到具体发布",
					engineVer, RecomputeEngineVersionSkillsOnly)
			}
		})
	}
}

// TestReplaySkillsMatchLeavesNoLedgerRow 守住「相符时不写台账」。
//
// ⚠️ 反向判据：台账若把相符的对局也记下来，不符率的分母就会虚高，
//
//	熔断永远不触发 —— 安全网「因为噪声太多而永远不响」是最坏的失效形态。
//	所以「相符不写」是熔断能工作的**前提**，不是省事。
func TestReplaySkillsMatchLeavesNoLedgerRow(t *testing.T) {
	ts := openTestService(t)
	ts.Service.Cfg().SettleRecomputeMode = RecomputeEnforce
	ts.Service.initRecomputeGate()

	uid := recomputeProbeUser(t, ts, "ok")
	if _, err := settleWithReplaySkills(t, ts, uid, correctReplaySkills(t, ts, uid)); err != nil {
		t.Fatalf("上报正确的 S 段不该被拒：%v", err)
	}
	if n := countRecomputeRows(t, ts, uid); n != 0 {
		t.Errorf("相符的对局不应写台账，实际 %d 行 —— "+
			"这会让不符率的分母虚高，熔断永远不触发", n)
	}
}

// TestEmptyReplaySkillsStillPasses 守住「没上报该字段仍放行」。
//
// 这一条与闸门无关，但它是本文件改动过的代码路径上最容易踩坏的一条：
// 老客户端不上报 `replay_skills`，若被闸门逻辑顺手改成「空 = 不符」，
// 全部老客户端会被立刻拒发 —— 而症状是「上线后所有旧玩家领不到奖励」。
func TestEmptyReplaySkillsStillPasses(t *testing.T) {
	for _, mode := range []string{RecomputeOff, RecomputeShadow, RecomputeEnforce} {
		t.Run(mode, func(t *testing.T) {
			ts := openTestService(t)
			ts.Service.Cfg().SettleRecomputeMode = mode
			ts.Service.initRecomputeGate()

			uid := recomputeProbeUser(t, ts, "empty_"+mode)
			if _, err := settleWithReplaySkills(t, ts, uid, ""); err != nil {
				t.Fatalf("%s 档：空 replay_skills 必须放行（老客户端），实际 %v", mode, err)
			}
			if n := countRecomputeRows(t, ts, uid); n != 0 {
				t.Errorf("%s 档：放行的对局不该写台账，实际 %d 行", mode, n)
			}
		})
	}
}
