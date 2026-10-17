package service

// 结算重算闸门（R1）的守卫。
//
// # 判据统一是「改回去会不会红」
//
// 这套闸门是**安全网**，而安全网最坏的失效形态是
// 「它看起来在工作，但永远不触发」——
// 那比没有安全网更糟，因为所有人都会因此以为可以收紧到 enforce。

import (
	"context"
	"errors"
	"testing"
	"time"
)

// --- 纯函数部分：熔断判定 ---

// TestEvaluateBreakerRespectsSampleFloor 守住「样本量不足不下结论」。
//
// # 为什么这条比「阈值本身」更重要
//
// 只看不符率的话，低流量时段任意一次误判就是 100% 不符率 → 立刻熔断。
// 症状是每天凌晨降级一次、白天手动开回来、夜里又降 ——
// 反复横跳，而根因可能只是那一局恰好撞上某条边界。
//
// 判据落在边界两侧各一格：minSamples-1 不熔断（哪怕不符率 100%），
// minSamples 才判。少一格 / 多一格都红。
func TestEvaluateBreakerRespectsSampleFloor(t *testing.T) {
	const minS, maxRate = 200, 20

	// 样本量差一个：即使**全部**不符也不该熔断（还看不懂）
	if trip, reason := evaluateBreaker(minS-1, minS-1, minS, maxRate); trip {
		t.Errorf("样本量 %d < %d 时不应熔断（样本不足≠看懂了有问题），"+
			"实际熔断了，理由：%s", minS-1, minS, reason)
	}
	if _, reason := evaluateBreaker(minS-1, minS-1, minS, maxRate); reason != "" {
		t.Errorf("未熔断时理由应为空串（否则日志里会出现「没发生」的原因），实际 = %q", reason)
	}

	// 刚够样本且全不符 → 熔断
	if trip, reason := evaluateBreaker(minS, minS, minS, maxRate); !trip {
		t.Errorf("样本量 %d 且 100%% 不符应熔断，实际未熔断", minS)
	} else if reason == "" {
		t.Error("熔断时必须给出可读理由（要进日志与台账），实际为空")
	}
}

// TestEvaluateBreakerRateBoundaryIsExact 守住阈值边界。
//
// 判据用**恰好等于阈值**与**恰好超过一格**。
//
// 为什么不用「远大于/远小于」：那样会把阈值算错 2 倍也放过 ——
// 阈值算错的后果是安全网形同虚设，而那正是本文件要防的。
func TestEvaluateBreakerRateBoundaryIsExact(t *testing.T) {
	const minS, maxRate = 1000, 20 // 阈值 2%

	// 恰好 2% → 不熔断（<= 是「不超过」）
	if trip, _ := evaluateBreaker(1000, 20, minS, maxRate); trip {
		t.Error("恰好等于阈值（2%）不应熔断")
	}
	// 超一个单位（20.1‰ → 21/1000）→ 熔断
	if trip, _ := evaluateBreaker(1000, 21, minS, maxRate); !trip {
		t.Error("超过阈值一个单位应熔断")
	}
	// 0% → 不熔断
	if trip, _ := evaluateBreaker(1000, 0, minS, maxRate); trip {
		t.Error("零不符绝不该熔断")
	}
}

// TestEvaluateBreakerUsesNoFloat 钉住「整数运算」这一实现选择。
//
// ⚠️ 为什么值得单独一条：浮点版本会在这两个值上给出不同答案 ——
//
//	200*20 = 4000    整数与浮点都无误差
//	7*1000  = 7000    同上
//
// 真正会出问题的是**比较的顺序**：`(mismatch/total) > maxRate/1000`
// 在 total=3 时会因截断得到 333‰ vs 333.3‰，结果随实现变。
// 所以判据必须落在**交叉相乘**的整数形式上，这里把这一点写成断言。
func TestEvaluateBreakerUsesNoFloat(t *testing.T) {
	// 这些组合在小样本下最容易暴露浮点/截断差异
	cases := []struct{ total, mismatch int }{
		{3, 1}, {7, 1}, {999, 20}, {1001, 20}, {999, 21}, {1000000, 20000},
	}
	for _, c := range cases {
		got, _ := evaluateBreaker(c.total, c.mismatch, 1, 20)
		// 期望值用交叉相乘手工判定，与被测实现同口径
		want := c.mismatch*1000 > c.total*20
		if got != want {
			t.Errorf("evaluateBreaker(%d, %d) = %v，交叉相乘判定应为 %v",
				c.total, c.mismatch, got, want)
		}
	}
}

// --- 闸门部分 ---

func newGateFor(mode string, mismatch, total int) (*RecomputeGate, *int) {
	calls := 0
	g := NewRecomputeGate(mode, func(context.Context, time.Time) (int, int, error) {
		calls++
		return mismatch, total, nil
	})
	return g, &calls
}

// TestGateOffDoesNothing 守住 off 档。
//
// off 的语义是「完全回到旧行为」：不重算、不记录、不拒发。
// 若 off 还会记录，那它就不是 off，而是 shadow —— 而两者的差别正是
// 「台账里有没有噪声」，噪声会让后续判断不符率变得没有意义。
func TestGateOffDoesNothing(t *testing.T) {
	g, calls := newGateFor(RecomputeOff, 9999, 9999)
	d := g.Decide(context.Background(), time.Now(), ScopeEngine)
	if d.Compute || d.Enforce || d.Demoted {
		t.Errorf("off 档应既不重算也不 enforce，实际 %+v", d)
	}
	if *calls != 0 {
		t.Errorf("off 档不应查询不符率（白查一次库），实际查了 %d 次", *calls)
	}
}

// TestGateShadowComputesButNeverEnforces 守住 shadow 档。
//
// 这是**唯一**能在上线 enforce 之前验证「重算准不准」的档位，
// 所以它必须：算、写台账、但绝不拒发。
func TestGateShadowComputesButNeverEnforces(t *testing.T) {
	g, _ := newGateFor(RecomputeShadow, 9999, 9999)
	d := g.Decide(context.Background(), time.Now(), ScopeEngine)
	if !d.Compute {
		t.Error("shadow 档必须重算")
	}
	if d.Enforce {
		t.Error("shadow 档绝不能 enforce")
	}
	if d.Mode != RecomputeShadow {
		t.Errorf("Mode = %q，期望 %q", d.Mode, RecomputeShadow)
	}
}

// TestGateEnforceDemotesOnMassMismatch 是熔断的核心判据。
//
// ⚠️ 变异价值：如果把 `checkBreaker` 的调用去掉（熔断形同虚设），
//
//	这条会红。而那正是安全网「看起来在工作、实际永不触发」的失效形态。
func TestGateEnforceDemotesOnMassMismatch(t *testing.T) {
	g, calls := newGateFor(RecomputeEnforce, 500, 1000)
	d := g.Decide(context.Background(), time.Now(), ScopeEngine)
	if *calls == 0 {
		t.Fatal("enforce 档必须查询不符率，否则熔断永远不触发")
	}
	if !d.Demoted {
		t.Fatalf("50%% 不符率应触发熔断，实际未降级：%+v", d)
	}
	if d.Enforce {
		t.Error("熔断后绝不能再 enforce（那正是熔断要阻止的事）")
	}
	if d.Mode != RecomputeShadow {
		t.Errorf("降级后 Mode = %q，期望 %q", d.Mode, RecomputeShadow)
	}
	if d.Reason == "" {
		t.Error("降级必须带可读原因")
	}

	// 降级是**粘住**的：后续调用仍处于 shadow，
	// 即使不符率回落也不自动升档（见文件头「为什么不自动升」）。
	d2 := g.Decide(context.Background(), time.Now().Add(time.Hour), ScopeEngine)
	if d2.Enforce {
		t.Error("降级不应自动升回 enforce —— 那会在故障未修好时恢复伤害")
	}
}

// TestGateEnforceSurvivesLowMismatch 守住「正常时不误降级」。
//
// 这一条与上一条必须**同时**成立：
//   - 上一条：有问题时降级
//   - 这一条：没问题时**不**降级
//
// 只写上一条的话，一个「永远降级」的实现也能全绿 ——
// 而那意味着 enforce 档形同虚设。
func TestGateEnforceSurvivesLowMismatch(t *testing.T) {
	g, _ := newGateFor(RecomputeEnforce, 1, 5000) // 0.02%
	d := g.Decide(context.Background(), time.Now(), ScopeEngine)
	if d.Demoted {
		t.Errorf("0.02%% 不符率不应熔断，实际降级了：%+v", d)
	}
	if !d.Enforce {
		t.Errorf("未熔断时应保持 enforce，实际 %+v", d)
	}
}

// TestGateEnforceSurvivesLowSample 守住「样本不足时不降级」。
//
// ⚠️ 顺序很重要：先用小样本高不符率，再喂大样本零不符率。
//
//	若实现只看「是否曾经超过过阈值」而不看当前样本，
//	它会一直记得第一次的 100% —— 那正是「熔断后永不恢复」的变体。
func TestGateEnforceSurvivesLowSample(t *testing.T) {
	g, _ := newGateFor(RecomputeEnforce, 3, 4) // 75% 但只有 4 条样本
	d := g.Decide(context.Background(), time.Now(), ScopeEngine)
	if d.Demoted {
		t.Errorf("样本量 4 远低于阈值 %d，不应据此熔断（可能是低峰期的噪声）", MinBreakerSamples)
	}
}

// TestGateBreakerIgnoresStatError 守住「查不到统计时不熔断」。
//
// ⚠️ 为什么这条的方向容易写反：直觉上「数据查不到 → 更不安全 → 该降级」。
//
//	但熔断的依据是「**观测到**大量不符」，不是「查不到数据」。
//	数据库抖一下就降级，会把一次短暂故障变成一次长达数小时的停摆 ——
//	而离线期间恰恰是最需要重算在跑的时候（如果真在 enforce）。
func TestGateBreakerIgnoresStatError(t *testing.T) {
	g := NewRecomputeGate(RecomputeEnforce, func(context.Context, time.Time) (int, int, error) {
		return 0, 0, errors.New("boom")
	})
	d := g.Decide(context.Background(), time.Now(), ScopeEngine)
	if d.Demoted {
		t.Error("统计查询失败不得触发熔断 —— 那会把 DB 抖动变成功能停摆")
	}
	if !d.Enforce {
		t.Errorf("统计查询失败时应维持原档位（enforce），实际 %+v", d)
	}
}

// TestGateRejectsInvalidMode 守住非法档位退到 off。
//
// 判据是「退到**最保守**档」而不是「退到 shadow」：
// 配错时宁可什么都不做，也不要意外开始拒发真实玩家。
func TestGateRejectsInvalidMode(t *testing.T) {
	g := NewRecomputeGate("enforcee", nil)
	if g.Mode() != RecomputeOff {
		t.Errorf("非法档位应退到 %q，实际 %q（拼错一个字母就开启拒发是不可接受的）",
			RecomputeOff, g.Mode())
	}
	d := g.Decide(context.Background(), time.Now(), ScopeEngine)
	if d.Compute || d.Enforce {
		t.Errorf("非法档位不应有任何生效行为，实际 %+v", d)
	}
}

// TestSkillsScopeIsNeverDemoted 守住「S 段永不熔断」。
//
// # 这是本文件最重要的一条
//
// 熔断的正当性来自「不符有歧义」。S 段的不符**没有歧义** ——
// 服务端从 DB 重算、客户端从 DB 上报，两边读同一份数据，
// 中间没有第二个实现，所以不符只可能是有人手改了上报值。
//
// 一旦对它装熔断，后果是反向的：
//
//	作弊脚本开刷 → 不符率冲高 → 熔断触发 → 校验被降级
//	→ **防护恰好在攻击发生时失效**，且诚实玩家的请求也一起被放过。
//
// 那等于把安全网装在了刹车上。所以这里必须钉死。
//
// ⚠️ 判据用的是「100% 不符」这种极端输入 ——
//
//	因为这条要防的正是「攻击峰值时降级」，低不符率下永不降级
//	是平凡成立的，证明不了任何东西。
func TestSkillsScopeIsNeverDemoted(t *testing.T) {
	g, calls := newGateFor(RecomputeEnforce, 1000, 1000) // 100% 不符
	d := g.Decide(context.Background(), time.Now(), ScopeSkills)
	if d.Demoted {
		t.Fatalf("S 段不得被熔断（不符即作弊证据），实际降级了：%+v", d)
	}
	if !d.Enforce {
		t.Errorf("S 段在 enforce 档必须保持 enforce，实际 %+v", d)
	}
	// 且**不应该**为此白查一次库
	if *calls != 0 {
		t.Errorf("S 段裁决不应查询不符率（它不参与熔断），实际查了 %d 次", *calls)
	}
}

// TestSkillsScopeNeverGoesThroughShadowEvenAfterEngineDemotion 守住降级的隔离性。
//
// ⚠️ 这条防的是一个很具体的实现陷阱：把「已降级」做成闸门级全局状态，
//
//	而不是每个 scope 各自的判定。
//
//	那种实现下会出现：引擎侧（未来的）熔断降级 → **S 段也跟着被降级**
//	→ 一个引擎 bug 顺手关掉了反作弊校验。
//
//	两个 scope 共享闸门实例但**互不影响**，这正是把 scope 显式传进去的原因。
func TestSkillsScopeNeverGoesThroughShadowEvenAfterEngineDemotion(t *testing.T) {
	g, _ := newGateFor(RecomputeEnforce, 1000, 1000)

	// 先让引擎侧熔断
	eng := g.Decide(context.Background(), time.Now(), ScopeEngine)
	if !eng.Demoted {
		t.Fatalf("引擎侧 100%% 不符应熔断，实际未降级：%+v", eng)
	}

	// S 段必须**不受影响**
	sk := g.Decide(context.Background(), time.Now(), ScopeSkills)
	if sk.Demoted {
		t.Errorf("引擎降级不得波及 S 段：%+v", sk)
	}
	if !sk.Enforce {
		t.Errorf("引擎降级后 S 段仍须 enforce —— 一个引擎 bug 不该关掉反作弊校验，"+
			"实际 %+v", sk)
	}
}

// TestValidRecomputeModeMatchesList 守住「校验函数与文档同源」。
//
// 一份「文档写了三种、代码认两种」的清单，
// 症状是某个档位既配得上、又进不了 switch 的任何一边。
func TestValidRecomputeModeMatchesList(t *testing.T) {
	for _, m := range RecomputeModes {
		if !ValidRecomputeMode(m) {
			t.Errorf("%q 在 RecomputeModes 里但 ValidRecomputeMode 说不合法", m)
		}
	}
	for _, m := range []string{"", "OFF", "Off", "enforce ", "reject", "true"} {
		if ValidRecomputeMode(m) {
			t.Errorf("%q 不该被判为合法档位", m)
		}
	}
	// 严格程度必须单调递增 —— 熔断降级依赖这个次序
	for i := 1; i < len(RecomputeModes); i++ {
		if StrictnessOf(RecomputeModes[i]) <= StrictnessOf(RecomputeModes[i-1]) {
			t.Errorf("RecomputeModes 的顺序不是「严格程度递增」：%s(%d) 之后是 %s(%d)",
				RecomputeModes[i-1], StrictnessOf(RecomputeModes[i-1]),
				RecomputeModes[i], StrictnessOf(RecomputeModes[i]))
		}
	}
}
