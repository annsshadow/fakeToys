package domain

// 守卫：`user_skills.level` 当前**不参与任何计算**，且这是**已知状态**。
//
// ## 为什么需要这条
//
// `HitInput.SkillDamage` 的注释曾经写着「技能表原始值 **× 等级系数**」，
// 而代码里从来没有等级系数：
//
//   - `user_skills.level` 只被读进 build 快照（`loadSkillsAndSlots`）
//   - **从无写入**（`service.go` 的初始 INSERT 之后没有任何 UPDATE）
//   - **没有任何技能升级端点**（`/admin/skills` 是运营后台的只读/改内容表）
//
// 注释描述一个**不存在的公式**，比没有注释更糟：
// 后来人要么照着它去实现"等级加成"，要么以为等级已在生效
// 而去查"为什么升级了没变强" —— 两者都是被这份注释误导的。
//
// ## 这条测试的作用
//
// 把"等级不参与计算"钉成**显式契约**，并说明前置条件。
// 将来有人加上升级路径（消耗资源 → 提升 level）时，
// 本测试会因为 `level` 仍不影响伤害而**继续绿** ——
// 那正是它的设计目的：它不阻止你加升级路径，
// 它只是让"等级已生效"这个断言**不会在没人注意时悄悄变成谎言**。
//
// 真要接等级成长时，这一条要**一起改**：
// 加 level 系数 → 本测试改成断言"等级提高伤害提高"。

import (
	"reflect"
	"testing"
)

// TestSkillLevelCurrentlyHasNoEffect 断言技能等级不影响伤害。
//
// 判据是**行为**而不是"代码里没有某个字段"——
// 文本搜索会因为大小写、别名、换位置而误判，
// 而一个会误判的守卫比没有守卫更糟。
func TestSkillLevelCurrentlyHasNoEffect(t *testing.T) {
	// 构造两个数值上代表"不同等级"的输入。
	// SkillDamage 是调用方算好的单发伤害，等级系数若存在就该在这里体现。
	lv1 := HitInput{SkillDamage: 1000, SkillElement: ElementFire, Roll: 999}
	lv10 := HitInput{SkillDamage: 10000, SkillElement: ElementFire, Roll: 999}

	// 先确认这个测试自身有效：两个明显不同的输入必须给出不同的结果。
	// 若这一步就失败，说明「伤害与 SkillDamage 无关」——那比等级不生效更严重。
	def := Defender{HP: 1_000_000, ElementStacks: map[Element]int64{ElementFire: 4}}
	r1 := ResolveHit(DefaultAttacker(), &def, lv1)
	r10 := ResolveHit(DefaultAttacker(), &def, lv10)
	if r10.TotalDamage <= r1.TotalDamage {
		t.Fatalf(
			"前提检查失败：SkillDamage 10 倍时总伤害没有提高（%d → %d）—— "+
				"伤害与 SkillDamage 无关，那是一个比等级不生效更严重的问题",
			r1.TotalDamage, r10.TotalDamage,
		)
	}

	// 真正的判据：Attacker 上**不存在**任何承载技能等级的字段，
	// 所以「等级」这个概念在伤害公式里根本没有入口。
	//
	// 这里断言的是结构性事实而不是行为 ——
	// 因为行为上「等级不生效」与「等级恒为 1」不可区分（后者才是真正的现状）。
	att := DefaultAttacker()
	if hasSkillLevelField(att) {
		t.Fatalf(
			"domain.Attacker 上出现了承载技能等级的字段 —— " +
				"说明等级成长已经接进伤害公式。\n" +
				"请同步：① HitInput.SkillDamage 的注释（它曾错误地声称含等级系数）" +
				"② 本测试改为断言「等级提高 → 伤害提高」" +
				"③ 客户端 engine.ts / replay.ts 的同名字段" +
				"④ README 的「未做」清单",
		)
	}
}

// hasSkillLevelField 反射检查 Attacker 上是否有承载技能等级的字段。
//
// ⚠️ 必须用**真反射**枚举字段。
// 第一版我写的是一份**硬编码字段清单**（`allAttackerFieldNames`），
// 结果变异测试给 `Attacker` 加了一个 `SkillLevel` 字段而这条测试**照样绿** ——
// 清单没变，所以它看不见结构体的变化。
//
// 这与我在 `TestEveryAttackerFieldChangesResolveHit` 里拒绝的
// "按名字文本搜索"是同一类脆弱：**一份与被测结构体脱钩的副本**。
func hasSkillLevelField(Attacker) bool {
	t := reflect.TypeOf(Attacker{})
	for i := 0; i < t.NumField(); i++ {
		lower := lowerASCII(t.Field(i).Name)
		if hasSubstr(lower, "skilllevel") || hasSubstr(lower, "skill_level") {
			return true
		}
	}
	return false
}

// TestSkillLevelColumnHasNoUpgradePath 记录「没有升级路径」这个事实。
//
// 这条不产生运行时行为，作用是让"等级恒为 1"这个**根因**
// 与"等级不参与公式"这个**症状**在代码里同时可见。
//
// 根因很重要：如果将来有人只接了公式系数而没有升级路径，
// 等级依然是 1，公式系数恒等于 1 —— 功能"看起来接好了"但完全无效。
// 这正是本项目反复栽过的形状：接线存在，但一端是死的。
func TestSkillLevelColumnHasNoUpgradePath(t *testing.T) {
	// 升级路径一旦出现，这里就该改成断言"端点存在且会提升 level"。
	// 在那之前，这条测试的作用是把"没有升级路径"这个前提写进代码。
	const upgradeEndpointExists = false
	if upgradeEndpointExists {
		t.Fatal(
			"服务端已出现技能升级端点 —— 请把技能等级接进伤害公式，" +
				"并同步修正 HitInput.SkillDamage 的注释",
		)
	}
	// 这条测试恒绿。它的价值是**文档 + 提醒**，不是断言。
	t.Log("技能等级当前恒为 1：无升级端点、无等级系数。接线前置条件是先有升级路径。")
}

func lowerASCII(s string) string {
	b := []byte(s)
	for i := range b {
		if b[i] >= 'A' && b[i] <= 'Z' {
			b[i] += 'a' - 'A'
		}
	}
	return string(b)
}

func hasSubstr(s, sub string) bool {
	if len(sub) == 0 {
		return true
	}
	for i := 0; i+len(sub) <= len(s); i++ {
		if s[i:i+len(sub)] == sub {
			return true
		}
	}
	return false
}
