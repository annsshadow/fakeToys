package service

// 守卫：攻方属性的**每一项成长来源都必须真的改变攻方**。
//
// ## 为什么需要这些
//
// 本轮之前，`computeAttacker` 只装了专精树的 3 种 kind，
// 于是下面这些**全部是空壳**：
//
//   - 18 件装备（6 槽 × 3 阶）的 BaseArmor / BaseBonusPct
//     只被 seed.go 写进 DB，战斗路径**从不读取**
//   - 8 种宝石的 affixes 存在 user_gems.affixes JSONB 里，
//     而**全项目没有任何代码读过 user_gems**
//   - 专精树的 heat_cap（16 个节点）、skill_damage（8 个）、
//     armor（8 个）、mechanic（8 个）四类惰性
//
// 玩家把资源投到三阶武器上，战斗里看不到任何变化。
// 之所以没人发现，是因为 `computeRating`（I-7 构筑评分）读了这些数据 ——
// **展示层读同一份数据，不等于战斗层也读。**
//
// 这三条测试的形式都是「改变一个输入 → 断言攻方某项数值变了」，
// 而不是断言"代码里写了那几行"。

import (
	"context"
	"reflect"
	"strings"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// TestAttackerViewCoversEveryAttackerField 用反射确认下发结构没有漏字段。
//
// ⚠️ 漏字段是**静默**的：JSON 少一个 key 完全合法，
// 编译通过、测试全绿，只有当某个字段真的参与计算时才会暴露 ——
// 而那正是本轮踩的坑（三个字段只存在于 domain.Attacker，没进 AttackerView）。
//
// 判据：domain.Attacker 的每个字段名都必须在 AttackerView 里出现。
// 刻意**不要求顺序一致**（顺序没有语义），也不要求 json tag 同名
// （Go 字段名与 json 名本来就可以不同）。
func TestAttackerViewCoversEveryAttackerField(t *testing.T) {
	attackerFields := fieldNames(reflect.TypeOf(domain.Attacker{}))
	viewFields := fieldNames(reflect.TypeOf(AttackerView{}))

	var missing []string
	for _, name := range attackerFields {
		found := false
		for _, v := range viewFields {
			if v == name {
				found = true
				break
			}
		}
		if !found {
			missing = append(missing, name)
		}
	}
	if len(missing) > 0 {
		t.Errorf(
			"AttackerView 缺少 domain.Attacker 的这些字段：%s\n"+
				"后果：客户端从 /battle/token 拿不到这些加成，"+
				"重放时算出的结果与原局不同 —— I-6 验真会把正常对局判成伪造。\n"+
				"修法：在 AttackerView 加同名字段，并在 toView 里赋值。",
			strings.Join(missing, ", "),
		)
	}
}

func fieldNames(t reflect.Type) []string {
	out := make([]string, 0, t.NumField())
	for i := 0; i < t.NumField(); i++ {
		out = append(out, t.Field(i).Name)
	}
	return out
}

// TestLoadoutContributionChangesAttacker 逐项断言装配的每一类贡献都进攻方。
//
// 这是本轮最核心的一条：**把"装备/宝石/专精无效"这个空壳钉死**。
// 任何一项被删掉或改回去，这里都会红。
func TestLoadoutContributionChangesAttacker(t *testing.T) {
	base := domain.DefaultAttacker()

	cases := []struct {
		name  string
		cont  loadoutContribution
		check func(t *testing.T, a domain.Attacker)
	}{
		{
			name: "装备/宝石的攻击力",
			cont: loadoutContribution{Attack: 250},
			check: func(t *testing.T, a domain.Attacker) {
				if a.Attack != base.Attack+250 {
					t.Errorf("Attack 期望 %d，实际 %d", base.Attack+250, a.Attack)
				}
			},
		},
		{
			name: "装备/宝石的护甲",
			cont: loadoutContribution{Armor: 120},
			check: func(t *testing.T, a domain.Attacker) {
				if a.ArmorPermille != 120 {
					t.Errorf("ArmorPermille 期望 120，实际 %d", a.ArmorPermille)
				}
			},
		},
		{
			name: "宝石的暴击率",
			cont: loadoutContribution{Crit: 90},
			check: func(t *testing.T, a domain.Attacker) {
				if a.CritPermille != base.CritPermille+90 {
					t.Errorf("CritPermille 期望 %d，实际 %d", base.CritPermille+90, a.CritPermille)
				}
			},
		},
		{
			name: "宝石的元素系数",
			cont: loadoutContribution{ElementCoef: 200},
			check: func(t *testing.T, a domain.Attacker) {
				if a.ElementCoefPermille != base.ElementCoefPermille+200 {
					t.Errorf("ElementCoefPermille 期望 %d，实际 %d",
						base.ElementCoefPermille+200, a.ElementCoefPermille)
				}
			},
		},
		{
			name: "宝石的层数上限",
			cont: loadoutContribution{ElementCap: 2},
			check: func(t *testing.T, a domain.Attacker) {
				if a.ElementCap != base.ElementCap+2 {
					t.Errorf("ElementCap 期望 %d，实际 %d", base.ElementCap+2, a.ElementCap)
				}
			},
		},
		{
			name: "宝石的反应倍率",
			cont: loadoutContribution{ReactionMult: 300},
			check: func(t *testing.T, a domain.Attacker) {
				if a.ReactionMultPermille != base.ReactionMultPermille+300 {
					t.Errorf("ReactionMultPermille 期望 %d，实际 %d",
						base.ReactionMultPermille+300, a.ReactionMultPermille)
				}
			},
		},
		{
			name: "专精/宝石的热量上限",
			cont: loadoutContribution{HeatCap: 150},
			check: func(t *testing.T, a domain.Attacker) {
				if a.HeatCapPermille != 150 {
					t.Errorf("HeatCapPermille 期望 150，实际 %d", a.HeatCapPermille)
				}
			},
		},
		{
			name: "专精的机制改造",
			cont: loadoutContribution{Mechanic: 100},
			check: func(t *testing.T, a domain.Attacker) {
				if a.MechanicPermille != 100 {
					t.Errorf("MechanicPermille 期望 100，实际 %d", a.MechanicPermille)
				}
			},
		},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			a := base
			applyLoadout(&a, tc.cont)
			tc.check(t, a)
		})
	}
}

// TestApplyLoadoutIsClamped 断言封顶生效，且**在专精之后**才算。
func TestApplyLoadoutIsClamped(t *testing.T) {
	cases := []struct {
		name  string
		cont  loadoutContribution
		field func(domain.Attacker) int64
		max   int64
	}{
		{"攻击力", loadoutContribution{Attack: 99_999}, func(a domain.Attacker) int64 { return a.Attack }, MaxLoadoutAttackPermille},
		{"元素系数", loadoutContribution{ElementCoef: 99_999}, func(a domain.Attacker) int64 { return a.ElementCoefPermille }, MaxLoadoutElementCoefPermille},
		{"暴击率", loadoutContribution{Crit: 99_999}, func(a domain.Attacker) int64 { return a.CritPermille }, MaxLoadoutCritPermille},
		{"反应倍率", loadoutContribution{ReactionMult: 99_999}, func(a domain.Attacker) int64 { return a.ReactionMultPermille }, MaxLoadoutReactionMultPermille},
		{"热量上限", loadoutContribution{HeatCap: 99_999}, func(a domain.Attacker) int64 { return a.HeatCapPermille }, MaxLoadoutHeatCapPermille},
		{"机制强度", loadoutContribution{Mechanic: 99_999}, func(a domain.Attacker) int64 { return a.MechanicPermille }, MaxLoadoutMechanicPermille},
		{"护甲", loadoutContribution{Armor: 99_999}, func(a domain.Attacker) int64 { return a.ArmorPermille }, domain.MaxArmorPermille},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			a := domain.DefaultAttacker()
			applyLoadout(&a, tc.cont)
			got := tc.field(a)
			baseVal := tc.field(domain.DefaultAttacker())
			delta := got - baseVal
			if delta != tc.max {
				t.Errorf("%s 贡献被封到 %d，期望 %d（输入 99999）", tc.name, delta, tc.max)
			}
		})
	}

	t.Run("负值夹到 0 而不是原样保留", func(t *testing.T) {
		// 一个负的攻击力加成会让伤害变小 —— 而"负收益"从来不是
		// 任何一件装备或词条的效果，它只可能来自脏数据。
		// 脏数据该被夹住，不该被解释成设计。
		a := domain.DefaultAttacker()
		applyLoadout(&a, loadoutContribution{Attack: -5000, Armor: -100})
		if a.Attack != domain.DefaultAttacker().Attack {
			t.Errorf("负攻击力应夹到 0（等价于无加成），实际 Attack=%d", a.Attack)
		}
		if a.ArmorPermille != 0 {
			t.Errorf("负护甲应夹到 0，实际 %d", a.ArmorPermille)
		}
	})
}

// TestEquipmentActuallyRaisesAttacker 打真实数据库：
// 穿上装备后攻方属性必须变化，且**脱下后复原**。
//
// 需要 TEST_DATABASE_URL（缺省连本机 laiyipao 库），连不上则 Skip。
func TestEquipmentActuallyRaisesAttacker(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	before, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("computeAttacker: %v", err)
	}

	// 找一件有 armor 与 bonus 的装备（钢鳞胸甲 id=7）
	const chestID = 7
	eq, ok := domain.EquipmentByID(chestID)
	if !ok {
		t.Fatalf("内容表里没有 id=%d 的装备", chestID)
	}
	if _, err := ts.pool.Exec(ctx,
		`INSERT INTO user_equipment (user_id, equipment_id, slot, equipped)
		 VALUES ($1, $2, $3, TRUE)
		 ON CONFLICT (user_id, equipment_id) DO UPDATE SET equipped = TRUE`,
		uid, chestID, eq.Slot,
	); err != nil {
		t.Skipf("插入装备失败（可能缺 user_equipment 表）：%v", err)
	}
	defer func() {
		_, _ = ts.pool.Exec(ctx,
			`DELETE FROM user_equipment WHERE user_id = $1 AND equipment_id = $2`, uid, chestID)
	}()

	after, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("computeAttacker(装备后): %v", err)
	}

	if after.Attack != before.Attack+eq.BaseBonusPct {
		t.Errorf("穿上「%s」后 Attack 应 +%d：%d → %d",
			eq.Name, eq.BaseBonusPct, before.Attack, after.Attack)
	}
	if after.ArmorPermille != before.ArmorPermille+eq.BaseArmor {
		t.Errorf("穿上「%s」后 ArmorPermille 应 +%d：%d → %d",
			eq.Name, eq.BaseArmor, before.ArmorPermille, after.ArmorPermille)
	}

	// 脱下后必须复原 —— 证明加成真的来自那一行 equip 行，
	// 而不是某个被顺手改了的全局默认值。
	if _, err := ts.pool.Exec(ctx,
		`UPDATE user_equipment SET equipped = FALSE WHERE user_id = $1 AND equipment_id = $2`,
		uid, chestID,
	); err != nil {
		t.Fatalf("脱下装备: %v", err)
	}
	off, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("computeAttacker(脱下后): %v", err)
	}
	if off.Attack != before.Attack || off.ArmorPermille != before.ArmorPermille {
		t.Errorf("脱下装备后应复原：Attack %d→%d→%d，Armor %d→%d→%d",
			before.Attack, after.Attack, off.Attack,
			before.ArmorPermille, after.ArmorPermille, off.ArmorPermille)
	}
}
