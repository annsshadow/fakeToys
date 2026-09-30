package domain

// 饱和算术与伤害结算的**内部工具**直测。
//
// 为什么要单独测这些一行函数：它们是「客户端可控的 int64 输入」与
// 「金币/伤害落库」之间唯一的闸门。上一轮溢出漏洞（reactions=2^62
// 乘法回绕成负数骗过裁剪）的教训是——闸门函数自己的每个分支都必须
// 有用例钉住，"上游校验过"不构成理由（computeLoot 是导出 API）。

import (
	"math"
	"testing"
)

func TestNonNeg(t *testing.T) {
	for _, c := range []struct {
		in, want int64
		why      string
	}{
		{-1, 0, "负数输入必须归零——负数能骗过一切 if x > cap 形式的裁剪"},
		{math.MinInt64, 0, "最小 int64 也要归零（溢出回绕的典型产物）"},
		{0, 0, "零原样通过"},
		{7, 7, "正数原样通过"},
		{math.MaxInt64, math.MaxInt64, "最大 int64 原样通过"},
	} {
		if got := nonNeg(c.in); got != c.want {
			t.Errorf("nonNeg(%d) = %d，期望 %d（%s）", c.in, got, c.want, c.why)
		}
	}
}

func TestDivNonNeg(t *testing.T) {
	for _, c := range []struct {
		a, b, want int64
		why        string
	}{
		{100, 10, 10, "正常整除"},
		{105, 10, 10, "向零截断"},
		{-100, 10, 0, "负被除数归零——divNonNeg 的契约是结果非负"},
		{100, 0, 0, "除零必须返回 0 而不是 panic（输入来自客户端报文）"},
		{100, -5, 0, "负除数同样返回 0：负除数会把符号翻过来绕过非负契约"},
	} {
		if got := divNonNeg(c.a, c.b); got != c.want {
			t.Errorf("divNonNeg(%d, %d) = %d，期望 %d（%s）", c.a, c.b, got, c.want, c.why)
		}
	}
}

func TestSatMul(t *testing.T) {
	for _, c := range []struct {
		a, b, cap, want int64
		why             string
	}{
		{3, 4, 100, 12, "不封顶时精确相乘"},
		{1000, 1000, 100, 100, "结果超过 cap 时封顶，且不依赖中间值不溢出"},
		{math.MaxInt64, math.MaxInt64, 60000, 60000, "a*b 真溢出也必须封顶（先除后乘路径）"},
		{0, 5, 100, 0, "任一乘数为 0 结果为 0"},
		{5, 0, 100, 0, "乘法交换，另一侧为 0 同样为 0"},
		{5, 5, 0, 0, "cap 为 0 封顶为 0"},
		{-3, 4, 100, 0, "负乘数按 0 处理，不允许负结果外泄"},
		{3, -4, 100, 0, "另一侧负数同理"},
	} {
		if got := satMul(c.a, c.b, c.cap); got != c.want {
			t.Errorf("satMul(%d, %d, %d) = %d，期望 %d（%s）", c.a, c.b, c.cap, got, c.want, c.why)
		}
	}
}

func TestSatAdd(t *testing.T) {
	for _, c := range []struct {
		cap  int64
		term []int64
		want int64
		why  string
	}{
		{100, []int64{30, 40, 20}, 90, "各项之和未超 cap 时精确相加"},
		{100, []int64{30, 40, 50}, 100, "累计超过 cap 时封顶"},
		{100, []int64{math.MaxInt64, math.MaxInt64}, 100, "单项大到溢出也封顶"},
		{100, []int64{-50, 30}, 30, "负项按 0 处理，总和不会被负数拉低"},
		{-5, []int64{30}, 0, "cap 本身为负时按 0 处理（调用方把常量传反也不产生负数）"},
	} {
		if got := satAdd(c.cap, c.term...); got != c.want {
			t.Errorf("satAdd(%d, %v) = %d，期望 %d（%s）", c.cap, c.term, got, c.want, c.why)
		}
	}
}

func TestMulDiv(t *testing.T) {
	for _, c := range []struct {
		a, b, c, want int64
		why           string
	}{
		{6, 5, 10, 3, "正常 (a*b)/c"},
		{6, 5, 0, 0, "除数为 0 返回 0 而不是 panic——c 可能来自配置或报文"},
		{-6, 5, 10, -3, "负数向零截断，与 TS 的 Math.trunc 语义一致"},
	} {
		if got := mulDiv(c.a, c.b, c.c); got != c.want {
			t.Errorf("mulDiv(%d, %d, %d) = %d，期望 %d（%s）", c.a, c.b, c.c, got, c.want, c.why)
		}
	}
}

func TestClampInt64(t *testing.T) {
	for _, c := range []struct {
		v, lo, hi, want int64
		why             string
	}{
		{5, 0, 10, 5, "区间内原样返回"},
		{-1, 0, 10, 0, "低于下界夹到下界"},
		{11, 0, 10, 10, "高于上界夹到上界"},
	} {
		if got := clampInt64(c.v, c.lo, c.hi); got != c.want {
			t.Errorf("clampInt64(%d, %d, %d) = %d，期望 %d（%s）", c.v, c.lo, c.hi, got, c.want, c.why)
		}
	}
}

func TestMaxInt(t *testing.T) {
	if got := maxInt(3, 7); got != 7 {
		t.Errorf("maxInt(3, 7) = %d，期望 7", got)
	}
	if got := maxInt(7, 3); got != 7 {
		t.Errorf("maxInt(7, 3) = %d，期望 7——顺序不能影响结果", got)
	}
	if got := maxInt(2, 2); got != 2 {
		t.Errorf("maxInt(2, 2) = %d，期望 2", got)
	}
}

// TestReactionAttackCapDefensiveBranches 钉住反应攻击力上限的两个防御分支。
//
// 分支 1（w >= 1000）：设计上 AttackWeightPct 全表 ≤ 300，这个分支是
// "把上限设为无穷大而不是除零 panic"的保险丝。若有人往反应表里加
// w=1000 的新反应，行为应是"上限放开"而不是服务端崩。
// 分支 2（mult <= 0）：倍率为 0 表示攻击侧完全无效，上限必须是 0，
// 而不是把 elemPortion 放出去。
func TestReactionAttackCapDefensiveBranches(t *testing.T) {
	const elemPortion = 1000

	if got := reactionAttackCap(permille, elemPortion, 1000); got != elemPortion*permille {
		t.Errorf("w=1000 时上限应放开为 elemPortion*1000 = %d，实际 %d", elemPortion*permille, got)
	}
	if got := reactionAttackCap(300, elemPortion, 0); got != 0 {
		t.Errorf("倍率为 0 时上限必须为 0，实际 %d", got)
	}
	if got := reactionAttackCap(300, elemPortion, -100); got != 0 {
		t.Errorf("倍率为负（坏数据）时上限必须为 0，实际 %d", got)
	}
	// 正常路径抽查：w=300、mult=1000 时上限 = 300*1000/700 = 428（向零截断）
	if got := reactionAttackCap(300, elemPortion, permille); got != 428 {
		t.Errorf("w=300 mult=1000 的上限应为 428（w*E/(1-w)），实际 %d", got)
	}
}

// TestApplyArmorClampsAboveMax 钉住护甲封顶分支：ArmorPermille=900（客户端可报）
// 必须被夹到 MaxArmorPermille=750，而不是让减伤超过设计上限。
func TestApplyArmorClampsAboveMax(t *testing.T) {
	dmg := int64(10000)
	if got := applyArmor(dmg, MaxArmorPermille+150); got != applyArmor(dmg, MaxArmorPermille) {
		t.Errorf("超过封顶的护甲应与恰好封顶等价：got=%d want=%d", got, applyArmor(dmg, MaxArmorPermille))
	}
	// 精确值：10000 * 250 / 1000 = 2500
	if got := applyArmor(dmg, MaxArmorPermille); got != 2500 {
		t.Errorf("封顶护甲 750 的减伤结果应为 2500，实际 %d", got)
	}
}

// TestApplyElementGuards 钉住施加元素的三个守卫分支。
func TestApplyElementGuards(t *testing.T) {
	d := NewDefender(100, 0, 0)

	// 空元素：必须拒绝，否则 "" 会混进层数表污染哈希与遍历
	if d.ApplyElement("", 3, 8) {
		t.Error("空元素应返回 false")
	}
	// 非正增量：不加层数
	if d.ApplyElement(ElementFire, 0, 8) {
		t.Error("增量为 0 应返回 false")
	}
	if d.ApplyElement(ElementFire, -2, 8) {
		t.Error("负增量应返回 false——负层数会翻转诊断与反应判定")
	}
	// 三条守卫都不应留下状态
	if n := len(d.ElementStacks); n != 0 {
		t.Errorf("守卫分支不应写入层数表，实际 %d 项", n)
	}
}

// TestApplyElementInitializesNilMap 零值 Defender（map 为 nil）直接施加元素
// 必须惰性建表，而不是 panic——客户端引擎的等价实现就是按这个顺序调用的。
func TestApplyElementInitializesNilMap(t *testing.T) {
	d := Defender{HP: 100} // 未走 NewDefender，ElementStacks 为 nil
	if !d.ApplyElement(ElementIce, 2, 8) {
		t.Fatal("nil map 上首次施加应成功")
	}
	if d.Stacks(ElementIce) != 2 {
		t.Errorf("施加后层数应为 2，实际 %d", d.Stacks(ElementIce))
	}
}

// TestApplyElementCapStacks 封顶：叠加超过 cap 时停在 cap，且恰好等于 cap
// 时再施加返回 false（不再有状态变化，避免无意义的哈希扰动）。
func TestApplyElementCapStacks(t *testing.T) {
	d := NewDefender(100, 0, 0)
	if !d.ApplyElement(ElementFire, 5, 8) {
		t.Fatal("首次施加应成功")
	}
	if !d.ApplyElement(ElementFire, 10, 8) {
		t.Fatal("超量施加应报告成功（层数发生了变化：5 -> 8）")
	}
	if d.Stacks(ElementFire) != 8 {
		t.Errorf("封顶后层数应为 8，实际 %d", d.Stacks(ElementFire))
	}
	if d.ApplyElement(ElementFire, 1, 8) {
		t.Error("已到封顶再施加应返回 false")
	}
}

// TestClearElements 过热结束后清层是 I-2 的显式契约（TS 侧 damage.ts
// 有同名的 clearElements），Go 侧这份是同一契约的参考实现。
// 没有调用方不代表死代码——它是双端镜像 API 的一半。
func TestClearElements(t *testing.T) {
	d := NewDefender(100, 0, 0)
	d.ApplyElement(ElementFire, 3, 8)
	d.ApplyElement(ElementIce, 2, 8)
	d.ClearElements()
	for _, e := range AllElements() {
		if s := d.Stacks(e); s != 0 {
			t.Errorf("清除后 %s 层数应为 0，实际 %d", e, s)
		}
	}
	// 清除后表必须可用（不是 nil），继续施加不 panic
	if !d.ApplyElement(ElementKinetic, 1, 8) {
		t.Error("清除后应能继续施加元素")
	}
}

// TestApplyElementStacksGuards 钉住命中后施加层数的早退分支：
// 无元素技能 / 非正增量 都不得改变守方状态。
func TestApplyElementStacksGuards(t *testing.T) {
	att := DefaultAttacker()

	t.Run("无元素技能", func(t *testing.T) {
		def := NewDefender(100, 0, 0)
		ApplyElementStacks(att, &def, HitInput{SkillElement: "", ApplyStacks: 3})
		if totalElementStacks(&def) != 0 {
			t.Error("无元素技能不应留下层数")
		}
	})

	t.Run("非正增量", func(t *testing.T) {
		def := NewDefender(100, 0, 0)
		ApplyElementStacks(att, &def, HitInput{SkillElement: ElementFire, ApplyStacks: 0})
		ApplyElementStacks(att, &def, HitInput{SkillElement: ElementFire, ApplyStacks: -1})
		if totalElementStacks(&def) != 0 {
			t.Error("非正增量不应留下层数")
		}
	})
}

// TestReactionAttackRatioZeroDenominator 反应伤害为 0 时占比必须返回 0，
// 而不是除零 panic——诊断与红线断言都会拿它当分母。
func TestReactionAttackRatioZeroDenominator(t *testing.T) {
	if got := ReactionAttackRatio(HitResult{}); got != 0 {
		t.Errorf("无反应伤害时占比应为 0，实际 %d", got)
	}
}

// TestResolveHitClampsNegativeBaseDamage 负的技能基准伤害（客户端可报负数）
// 必须被夹到 0：否则 direct 为负，crit 放大后仍是负数，"伤害"变"回血"。
func TestResolveHitClampsNegativeBaseDamage(t *testing.T) {
	att := DefaultAttacker()
	def := NewDefender(10000, 0, 0)
	res := ResolveHit(att, &def, HitInput{SkillDamage: -5000, Roll: 999})
	if res.DirectDamage != 0 || res.TotalDamage != 0 {
		t.Errorf("负基准伤害应结算为 0，实际 direct=%d total=%d", res.DirectDamage, res.TotalDamage)
	}
	if def.HP != 10000 {
		t.Errorf("守方血量不应变化，实际 %d", def.HP)
	}
}

// TestResolveHitNegativeElementTick 基准合法但元素系数为负（坏养成数据）时，
// 元素持续伤害段必须夹到 0，不允许负数混进总伤害。
func TestResolveHitNegativeElementTick(t *testing.T) {
	att := DefaultAttacker()
	att.ElementCoefPermille = -1000
	def := NewDefender(100000, 0, 0)
	def.ApplyElement(ElementFire, 3, 8)
	res := ResolveHit(att, &def, HitInput{SkillDamage: 100, SkillElement: ElementFire, Roll: 0})
	if res.ElementDamage < 0 {
		t.Errorf("负系数的元素持续伤害应夹到 0，实际 %d", res.ElementDamage)
	}
}

// TestResolveHitShieldBrokenAndKilled 钉住结算落地的两个状态位：
// 护盾恰好打碎 → ShieldBroken；血量扣穿 → Killed 且 HP 归 0。
// 二者此前都只有"未触发"路径被执行过。
func TestResolveHitShieldBrokenAndKilled(t *testing.T) {
	t.Run("护盾打碎", func(t *testing.T) {
		att := DefaultAttacker()
		def := NewDefender(100000, 100, 0)
		res := ResolveHit(att, &def, HitInput{SkillDamage: 100000, Roll: 0})
		if !res.ShieldBroken {
			t.Error("护盾被一次性打碎应置 ShieldBroken")
		}
		if def.Shield != 0 {
			t.Errorf("护盾应归零，实际 %d", def.Shield)
		}
	})

	t.Run("击杀", func(t *testing.T) {
		att := DefaultAttacker()
		def := NewDefender(50, 0, 0)
		res := ResolveHit(att, &def, HitInput{SkillDamage: 100000, Roll: 0})
		if !res.Killed {
			t.Error("血量扣穿应置 Killed")
		}
		if def.HP != 0 {
			t.Errorf("击杀后 HP 应归 0，实际 %d", def.HP)
		}
	})
}
