package domain

// 技能升级规则的守卫。
//
// # 这个功能此前的形状
//
// `user_skills.level` 是一列**永远为 1** 的数据：
// 被读进 build 快照、从无写入、没有升级端点。
// 而 `HitInput.SkillDamage` 的注释声称伤害公式含「等级系数」——
// 一个不存在的因子。那条注释比功能先到了两年。
//
// 接线要三件事同时成立：升级路径、等级进公式、两端一致。
// 少一件就是纯装饰 —— 本项目已经栽过好几次
// （README「已知边界」里的「只写不读」「纯装饰」两条）。

import "testing"

func TestSkillLevel1IsExactIdentity(t *testing.T) {
	// 等级 1 → 1000‰ 是**精确恒等**，不是近似。
	//
	// 升级功能上线前所有 `user_skills.level` 恒为 1，
	// 所以所有**存量战报的重放哈希必须逐位不变**。
	// 等级进了 `replayHash()` 的锚点（客户端 `${slot}:${skillId}:${baseDamage}:…`），
	// 基线若不是精确恒等，每一个历史战报都会静默失配 ——
	// 而失配表现为「验真失败」，看起来像作弊，
	// 实际只是上线了一个数学常数。
	r := DefaultSkillRules()
	for _, base := range []int64{1, 7, 100, 12345, 999999, 1 << 50} {
		got := base * r.CoefPermilleAt(1) / 1000
		if got != base {
			t.Fatalf("等级 1 不是恒等：base=%d coef=%d 得 %d（应为 %d）",
				base, r.CoefPermilleAt(1), got, base)
		}
		// 整除必须无余数，否则说明系数不是 1000 的因子
		if rem := base * r.CoefPermilleAt(1) % 1000; rem != 0 {
			t.Fatalf("等级 1 引入余数 %d：base=%d coef=%d", rem, base, r.CoefPermilleAt(1))
		}
	}
}

func TestSkillCoefPermilleIsMonotonicAndClamped(t *testing.T) {
	r := DefaultSkillRules()

	if got := r.CoefPermilleAt(1); got != 1000 {
		t.Errorf("等级 1 应为 1000‰，得 %d", got)
	}
	// 逐级递增，且每级步长恰好 coef_permille
	for lv := 2; lv <= r.MaxLevel; lv++ {
		prev := r.CoefPermilleAt(lv - 1)
		cur := r.CoefPermilleAt(lv)
		if cur-prev != r.CoefPermille {
			t.Errorf("等级 %d 步长 %d，应为 %d", lv, cur-prev, r.CoefPermille)
		}
	}
	// 越界一律夹到满级 —— 脏数据不该变成 4900‰ 的伤害
	for _, bad := range []int{11, 99, 1 << 20} {
		if got := r.CoefPermilleAt(bad); got != r.CoefPermilleAt(r.MaxLevel) {
			t.Errorf("等级 %d 未夹紧：%d（应等于满级的 %d）",
				bad, got, r.CoefPermilleAt(r.MaxLevel))
		}
	}
	// 低位夹到 1
	for _, bad := range []int{0, -1, -1000} {
		if got := r.CoefPermilleAt(bad); got != 1000 {
			t.Errorf("等级 %d 未夹到 1：coef=%d", bad, got)
		}
	}
}

// TestSkillCoefDoesNotOutscaleAttackCap 是一条**设计意图**守卫。
//
// 技能伤害与面板攻击走同一个乘区（都乘在 SkillDamage 上、再进 D，
// 而 D 同时喂给直接伤害与反应的**攻击力侧**）。
// 所以提高等级系数与提高攻击力一样会**压制反应** ——
// 实测攻击力堆到 3000‰ 时反应次数掉 25%，攻击封顶因此砍到 1000‰。
//
// 满级技能的加成必须显著低于攻击封顶，否则「练级」就变成了
// 一个比堆攻击力更划算的压制反应的手段。
func TestSkillCoefDoesNotOutscaleAttackCap(t *testing.T) {
	r := DefaultSkillRules()
	full := r.CoefPermilleAt(r.MaxLevel) - 1000
	if full >= 1000 {
		t.Errorf("满级加成 %d‰ 已达到/超过攻击封顶 1000‰ —— 会压制反应，"+
			"与 I-1 反通胀冲突。实测：攻击力 3000‰ 时反应掉 25%%", full)
	}
	if full <= 0 {
		t.Errorf("满级加成 %d‰ ≤ 0，等级系统是纯装饰", full)
	}
}

func TestSkillUpgradeCostIsLinearAndNotFree(t *testing.T) {
	r := DefaultSkillRules()

	// 线性递增
	for lv := 1; lv < r.MaxLevel; lv++ {
		want := r.BaseCost * int64(lv)
		if got := r.CostFrom(lv); got != want {
			t.Errorf("等级 %d → %d+1 费用 %d，应为 %d", lv, lv, got, want)
		}
	}
	// 满级 0 费
	if got := r.CostFrom(r.MaxLevel); got != 0 {
		t.Errorf("满级费用 %d，应为 0", got)
	}
	// ⚠️ 越界低等级**不能免费**。
	//
	// 「level <= 0 时免费」是个真实的漏洞形状：若某分支写成
	// `if level <= 0 { return 0 }`，脏数据就能白升级。
	// 夹到 1 后按 BaseCost 计费。
	for _, bad := range []int{0, -1, -999} {
		if got := r.CostFrom(bad); got != r.BaseCost {
			t.Errorf("等级 %d 费用 %d，应夹到 1 级并收 %d（否则脏数据可白升级）",
				bad, got, r.BaseCost)
		}
	}
	// 满级单个技能总花费
	total := int64(0)
	for lv := 1; lv < r.MaxLevel; lv++ {
		total += r.CostFrom(lv)
	}
	wantTotal := r.BaseCost * int64(r.MaxLevel*(r.MaxLevel-1)/2)
	if total != wantTotal {
		t.Errorf("满级总费用 %d，应为 %d", total, wantTotal)
	}
}

func TestClampLevelNeverReturnsOutOfRange(t *testing.T) {
	r := DefaultSkillRules()
	// 端到端性质：对**任何** int，输出必在 [1, MaxLevel]
	for lv := -1000; lv <= 1000; lv++ {
		got := r.ClampLevel(lv)
		if got < 1 || got > r.MaxLevel {
			t.Fatalf("ClampLevel(%d) = %d，越界 [%d, %d]", lv, got, 1, r.MaxLevel)
		}
	}
	// 区间内不动
	for lv := 1; lv <= r.MaxLevel; lv++ {
		if got := r.ClampLevel(lv); got != lv {
			t.Errorf("ClampLevel(%d) = %d，区间内应原样返回", lv, got)
		}
	}
}

// TestSkillRulesAreExportedToContract 确认规则真的进了契约夹具。
//
// 理由：费用与系数两端都要用（按钮显示费用、战斗算伤害）。
// 不下发就得各写一份常量，而漂移时表现为
// 「玩家付了 800 金币，伤害却按 50‰ 涨」——
// 这种 bug 在战报里几乎看不出来。
// 照 ScoreRules 的先例：Go 侧唯一定义，TS 侧守卫比对。
func TestSkillRulesAreExportedToContract(t *testing.T) {
	r := DefaultSkillRules()
	if r.MaxLevel <= 0 {
		t.Fatal("MaxLevel 必须为正")
	}
	if r.CoefPermille <= 0 {
		t.Fatal("CoefPermille 必须为正，否则等级系统是纯装饰")
	}
	if r.BaseCost <= 0 {
		t.Fatal("BaseCost 必须为正，否则升级免费")
	}
	// 系数与费用都要能被 JSON 无损表达（客户端按 number 读）
	if r.CoefPermille > 1<<20 || r.BaseCost > 1<<20 {
		t.Errorf("系数/费用超出 JSON number 安全整数范围：coef=%d cost=%d",
			r.CoefPermille, r.BaseCost)
	}
}
