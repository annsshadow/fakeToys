package service

import (
	"fmt"
	"strings"
)

// 运行时切档（2026-10-15 随整局重算接线一起加）。
//
// # 为什么需要它
//
// `config.SettleRecomputeMode` 的注释写着「切档不需要发版：改环境变量重启即可」。
// 但那条路径在**测试**里用不了：`New()` 在构造时就调了 `initRecomputeGate`，
// 把档位固化进了 gate；此后改 `Cfg` 不会有任何效果 ——
// 而这个「改了却不生效」正是 `initRecomputeGate` 里那段 fail loud
// 注释最担心的那种静默失败的反面版本：
// 运营以为切了档，实际上没切。
//
// 所以这里给出显式的 setter：改档位与重建 gate 是**同一个动作**，
// 调用方不可能只做一半。

// SetRecomputeMode 运行时切换重算处置档位。
//
// 非法值**不静默降级**：返回错误，由调用方决定怎么办。
// 与 `initRecomputeGate` 的差别是——后者在启动路径上，
// 只能打印警告后降级（服务要能起来）；这里是显式调用，
// 调用方有机会处理。
//
// 并发安全：它替换整个 gate 指针。当前调用方都是启动期或测试期，
// 没有并发切档的需求；若将来要支持热切换，得改成 `atomic.Pointer`
// 并给 gate 内部加读写锁 —— 那时再说，不提前做。
func (s *Service) SetRecomputeMode(mode string) error {
	if !ValidRecomputeMode(mode) {
		return fmt.Errorf("非法的重算档位 %q（合法值：%s）",
			mode, strings.Join(RecomputeModes, "/"))
	}
	s.Cfg.SettleRecomputeMode = mode
	s.initRecomputeGate()
	return nil
}

// RecomputeModeState 是重算闸门的**当前状态快照**（给后台看的）。
//
// ⚠️ 2026-10-21（P3-3）：此前档位**没有任何 HTTP 端点**可读可改 ——
// 运营看着后台的「重算审计」页，知道有「档位」这个东西，
// 却既看不到当前是哪个档、也切不了。
//
// `mode` 与 `configured_mode` 的区别很重要（两者不一致 = 被熔断降级了）：
//   - mode            —— **现在实际生效**的档；
//   - configured_mode —— 配置里写的档；
//   - demoted        —— enforce 被熔断降成了 shadow
//     （此时 enforce 该拒发的奖励正在**放行**）。
type RecomputeModeState struct {
	// Mode 是**权威档位** —— 此刻真正生效的那一档。
	//
	// ⚠️ 2026-10-21（二轮审计 P1）：它**等于 SkillsMode**（S 段），
	// 因为**拒发奖励的是 S 段**（replay_skills 重算）。
	//
	// 此前这个字段报的是**引擎侧**档位，而引擎侧是**可以被熔断降级**的
	// —— 于是熔断一触发，后台就显示「生效中：shadow」，
	// 运营据此以为「本该拒发的奖励正在放行」。**那是错的**：
	// S 段永不降级，防护一直开着。
	//
	// 引擎侧的真实档位在 EngineMode 里。
	Mode string `json:"mode"`
	// ConfiguredMode 是配置里写的档（原始值，从未被改过）。
	ConfiguredMode string `json:"configured_mode"`
	// EngineMode 是**引擎侧**（S 段之外的整局重算）实际生效的档。
	// 它**可以被熔断降级** —— 降级时奖励在放行，但**只限引擎侧**。
	EngineMode string `json:"engine_mode"`
	// SkillsMode 是 **S 段**（replay_skills）实际生效的档。
	// 它**永不**被熔断降级 —— 因为 S 段的不符就是作弊证据，
	// 在攻击发生时把防护降级等于「把安全网装在刹车上」。
	SkillsMode string `json:"skills_mode"`
	// Enforcing = S 段此刻**是否会拒发奖励**。
	// 这是运营最需要知道的一个布尔值。
	Enforcing bool `json:"enforcing"`
	// Demoted 为真 = **引擎侧**被熔断降级了。
	// ⚠️ 它**不代表**整个防护失效 —— S 段照常拒发。
	Demoted bool `json:"demoted"`
	// DemotedReason 说明为什么被降级（为空表示没降级）。
	DemotedReason string `json:"demoted_reason"`
	// BreakerRatePermille 是熔断阈值（不符率千分比）。
	BreakerRatePermille int `json:"breaker_rate_permille"`
	// BreakerMinSamples 是熔断所需的最少样本数。
	BreakerMinSamples int `json:"breaker_min_samples"`
}

// RecomputeModeState 读当前闸门状态。
func (s *Service) RecomputeModeState() RecomputeModeState {
	out := RecomputeModeState{
		Mode:           s.RecomputeMode(),
		ConfiguredMode: s.Cfg.SettleRecomputeMode,
	}
	if g := s.gate.Load(); g != nil {
		// 引擎侧（S 段之外的整局重算）—— **可被熔断降级**。
		//
		// 用 EffectiveMode 而不是 Decide：状态端点是**读**，
		// 不该顺手触发一次熔断判定（Decide 有副作用）。
		out.EngineMode = g.EffectiveMode(ScopeEngine)
		demoted, reason, _ := g.Demoted(ScopeEngine)
		out.Demoted = demoted
		out.DemotedReason = reason
		// S 段（replay_skills）—— **永不**被熔断降级。
		out.SkillsMode = g.EffectiveMode(ScopeSkills)
		// 权威档位 = S 段：**拒发奖励的是它**。
		out.Mode = out.SkillsMode
		out.Enforcing = out.SkillsMode == RecomputeEnforce
		out.BreakerRatePermille = MaxBreakerRate
		out.BreakerMinSamples = MinBreakerSamples
	} else {
		out.EngineMode = RecomputeOff
		out.SkillsMode = RecomputeOff
	}
	return out
}

// RecomputeMode 读当前生效的档位。
//
// 读的是 **gate 里的**，不是 `Cfg` 里的 ——
// 两者在「有人只改了 Cfg 没重建 gate」时会不一致，
// 而这里要回答的是「现在到底哪个档在生效」。
func (s *Service) RecomputeMode() string {
	g := s.gate.Load()
	if g == nil {
		return RecomputeOff
	}
	return g.Mode()
}
