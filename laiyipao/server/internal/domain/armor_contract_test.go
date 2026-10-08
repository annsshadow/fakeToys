package domain

import (
	"sort"
	"testing"
)

// applyArmor 的**跨端绝对值契约**（第 71 轮）。
//
// # 缺陷：负伤害的语义两端不一致
//
// Go 侧（damage.go:469）：
//
//	if dmg <= 0 {
//	    // 负伤害原样返回：负数在结算里表示"反伤/异常"，不该被护甲改写。
//	    return dmg
//	}
//
// TS 侧此前**没有**这一行，于是：
//
//	dmg=-1000, armor=500  →  TS 得 -500、Go 得 -1000
//
// # 当前不可达，但契约应当先于实现正确
//
// 三个调用点都传非负值（damage.ts 上游有 `if (d < 0n) d = 0n` 夹紧，
// engine.ts 的 breachDamage 与 attack>0 分支都非负）。
// 所以这不是**当前**的缺陷，而是「文件头声明的『逐行等价』与实现不符」。
//
// 但 Go 侧注释明确说负伤害**将来会有语义**（反伤/异常），
// 而 `formula_vectors.json` 的 damage.cases 全是合法路径 ——
// 那天两端分叉时**没有任何测试会响**。
//
// 这与 README 第 9.3 节记的 `reactionElement` 空串分歧同源：
// 契约向量只覆盖「两端传了同一个合法值」时的比对，
// 分歧恰好在**非法输入路径**上。
//
// # 与 armor_test.go 的分工
//
//	armor_test.go                   拿字面值盯住 Go 侧实现
//	本文件 + miniapp/armor_contract  拿同一份字面值盯住「两端逐位相同」

func TestApplyArmorMatchesCrossEndVectors(t *testing.T) {
	v := loadVectors(t)
	cases := v.ApplyArmor.Cases
	if len(cases) == 0 {
		t.Fatal("formula_vectors.json 里 apply_armor.cases 是空的 —— " +
			"向量被误删时必须红，否则这个测试会静默变成空转")
	}

	for _, c := range cases {
		t.Run(c.Name, func(t *testing.T) {
			got := applyArmor(c.Dmg, c.Armor)
			if got != c.Expected {
				t.Fatalf("applyArmor(%d, %d) = %d，契约向量写的是 %d\n"+
					"两端只要有一端漂了，合法玩家的对局就会算出不同的伤害 → "+
					"replayHash 不同 → I-6 判伪造。\n"+
					"若确认是公式改动，两端一起改并回填这里的 expected。",
					c.Dmg, c.Armor, got, c.Expected)
			}
		})
	}
}

// TestApplyArmorVectorsCoverNegativeDamage 钉住「负伤害确实被锁住了」。
//
// 这条不是重复上一条 —— 它防的是**向量被删**。
// 负伤害那 4 条是本轮新增的，而它们恰恰是最容易被后来人
// 当成「非法输入、不用测」而清掉的。
//
// 判据用**集合**而不是逐条：只要负伤害的用例从向量里消失，这里就红。
func TestApplyArmorVectorsCoverNegativeDamage(t *testing.T) {
	v := loadVectors(t)

	var negatives []string
	for _, c := range v.ApplyArmor.Cases {
		if c.Dmg < 0 {
			negatives = append(negatives, c.Name)
		}
	}
	if len(negatives) == 0 {
		t.Fatal("apply_armor 向量里没有任何负伤害用例 —— " +
			"负伤���的跨端语义就此失去守卫，而 Go 侧注释说它表示「反伤/异常」")
	}

	// 至少要覆盖「负伤害 + 有护甲」这一对 —— 那正是两端曾经分歧的组合。
	hasNegWithArmor := false
	for _, c := range v.ApplyArmor.Cases {
		if c.Dmg < 0 && c.Armor > 0 {
			hasNegWithArmor = true
			break
		}
	}
	if !hasNegWithArmor {
		t.Error("缺少「负伤害 + 正护甲」的用例 —— " +
			"那正是 TS 少一行 `if dmg <= 0` 时会分叉的组合")
	}

	sort.Strings(negatives)
	t.Logf("负伤害用例：%v", negatives)
}

// TestApplyArmorVectorNamesAreDistinct 让失败时能定位到具体哪一条。
func TestApplyArmorVectorNamesAreDistinct(t *testing.T) {
	v := loadVectors(t)
	seen := map[string]bool{}
	for _, c := range v.ApplyArmor.Cases {
		if c.Name == "" {
			t.Error("有向量没写 name —— 失败时无法定位是哪一条")
		}
		if seen[c.Name] {
			t.Errorf("向量名重复：%s（重复的名字会让子测试互相覆盖，失败时看不出是哪条数据）", c.Name)
		}
		seen[c.Name] = true
	}
}
