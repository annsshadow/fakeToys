package service

import (
	"context"
	"github.com/laiyipao/server/internal/config"
	"testing"
	"time"
)

// TestModeStateSeparatesEngineAndSkillsScopes 钉住「状态上报」这条链。
//
// # 背景（2026-10-21 第二轮审计 P1）
//
// 闸门本身有两个 scope，行为一直是对的：
//
//	ScopeEngine —— 整局 `replay_hash` 重算。不符**有歧义**
//	               （玩家作弊 or Go 引擎漂移），所以熔断可以降它。
//	ScopeSkills —— `replay_skills` 重算。不符**无歧义**
//	               （服务端与客户端读同一份 DB），所以**永不**降级。
//
// **拒发奖励的是 S 段**（`game.go` 的 `ScopeSkills` 那一路）。
//
// 但 `RecomputeModeState.Mode` / `RecomputeSummary.Mode` 报的是
// **引擎侧**档位。于是熔断一触发，后台就显示：
//
//	生效中：shadow（已被熔断降级）——「本该拒发的奖励正在放行」
//
// **那是错的**：S 段照常拒发。运营据此会以为整个防护关了，
// 于是查错方向、甚至去改不需要改的那一段。
//
// # 判据
//
// 引擎侧被熔断降成 shadow 之后：
//   - `engine_mode` == shadow
//   - `skills_mode` == enforce（**没**被降级）
//   - `mode`（权威档位）== enforce
//   - `enforcing` == true（奖励**仍在**被拒发）
//   - `demoted` == true（但它只说明**引擎侧**）
func TestModeStateSeparatesEngineAndSkillsScopes(t *testing.T) {
	g, _ := newGateFor(RecomputeEnforce, 1000, 1000) // 100% 不符 → 熔断

	s := &Service{}
	// 2026-10-29：Cfg 是原子指针，空 Service 字面量不自带快照。
	// 用 Store 装一份，而不是改结构体字段（那是非原子写法）。
	s.cfg.Store(&config.Config{SettleRecomputeMode: RecomputeEnforce})
	s.gate.Store(g)

	// 先让**引擎侧**熔断
	d := g.Decide(context.Background(), time.Now(), ScopeEngine)
	if !d.Demoted {
		t.Fatalf("引擎侧应当熔断，实际 %+v", d)
	}

	state := s.RecomputeModeState()

	if state.EngineMode != RecomputeShadow {
		t.Errorf("引擎侧应报 shadow，实际 %q", state.EngineMode)
	}
	if state.SkillsMode != RecomputeEnforce {
		t.Errorf("S 段**永不**被熔断，应报 enforce，实际 %q", state.SkillsMode)
	}
	if state.Mode != RecomputeEnforce {
		t.Errorf("权威档位应是 S 段的 enforce，实际 %q —— "+
			"报成引擎侧会让运营以为整个防护都关了", state.Mode)
	}
	if !state.Enforcing {
		t.Error("S 段仍在 enforce，enforcing 应为 true —— " +
			"这是运营最需要知道的那个布尔值")
	}
	if !state.Demoted {
		t.Error("引擎侧确实被降级了，demoted 应为 true")
	}
	if state.DemotedReason == "" {
		t.Error("降级原因应当被记下来（运营要靠它判断要不要人工升档）")
	}
}

// TestModeStateReadsDoNotTripTheBreaker 钉住「状态端点是读」。
//
// `RecomputeModeState` / `AdminRecomputeSummary` 是**读**接口，
// 它们**不该**顺手触发一次熔断判定 ——
// `Decide` 有副作用（在 enforce 档会查库并可能降级）。
//
// 如果读接口用了 `Decide`，那么「谁看了一眼后台」就能把防护降级。
func TestModeStateReadsDoNotTripTheBreaker(t *testing.T) {
	g, calls := newGateFor(RecomputeEnforce, 1000, 1000)

	s := &Service{}
	// 2026-10-29：Cfg 是原子指针，空 Service 字面量不自带快照。
	// 用 Store 装一份，而不是改结构体字段（那是非原子写法）。
	s.cfg.Store(&config.Config{SettleRecomputeMode: RecomputeEnforce})
	s.gate.Store(g)

	before := *calls
	_ = s.RecomputeModeState()
	if *calls != before {
		t.Errorf("读状态不应查询不符率，实际多查了 %d 次 —— "+
			"读接口不该有触发熔断的副作用", *calls-before)
	}
}
