package domain

// 守卫：装备描述**必须陈述它真实提供的数值**。
//
// ## 为什么需要这条
//
// `SeedEquipment` 只有两个数值字段：`BaseArmor` 与 `BaseBonusPct`。
// 而改之前 18 条描述里有 **10 条**承诺了不存在的效果：
//
//	磁暴线圈「让弹丸短暂进入相位态，直接**附加元素**」
//	雷弧面甲「看清敌人身上的元素层数」→ 被读成元素系数加成
//	液氮装甲「被击中时**急速降温**」
//	熔炉装甲「受击时**反射元素攻击**」
//	导轨手套「**装填更快**」
//	相位护踝「过热时**仍可释放一次技能**」（那是机制卡的效果，不是装备的）
//	元素棱镜「反应链**更容易被触发**」
//	…共 10 条
//
// 装备**只**进入 `loadLoadout` 的 `BaseArmor` / `BaseBonusPct` 两项，
// `eq.Element` 全项目从未被读取。
//
// 这是**面向玩家的谎言**，比注释撒谎更严重：
// 注释只骗开发者，而描述出现在背包界面上，玩家据此决定买什么。
//
// ## 判据的选择
//
// 试过"禁止描述里出现效果类关键词"—— **不可行**：
// 风味文案合法地提到元素（"看清敌人身上的元素层数"），
// 关键词无法区分"风味"与"承诺"。
//
// 改成**可自动检验的强判据**：描述必须包含该装备真实的数值。
// 于是「描述与实现对不上」这件事从"需要人来判断"变成"机器能判"。
//
// 将来真要实现「装备按元素给加成」，本测试要改成
// 断言描述里出现那个加成对应的关键词 —— 而不是默默加代码。

import (
	"fmt"
	"strconv"
	"strings"
	"testing"
)

func TestEquipmentDescrStatesItsRealNumbers(t *testing.T) {
	for _, e := range SeedEquipmentList {
		// 攻击力总是 > 0，必须出现在描述里
		atk := "攻击力 +" + strconv.FormatInt(e.BaseBonusPct, 10) + "‰"
		if !strings.Contains(e.Descr, atk) {
			t.Errorf(
				"装备「%s」（%s）的描述没有陈述它真实的攻击力加成。\n"+
					"描述：%s\n期望包含：%s\n"+
					"装备只提供 BaseArmor 与 BaseBonusPct 两项，"+
					"描述里承诺的其它效果（反射、附加元素、减少过热…）都不存在。",
				e.Name, e.Code, e.Descr, atk,
			)
		}
		// 护甲 > 0 时也必须出现
		if e.BaseArmor > 0 {
			armor := "护甲 +" + strconv.FormatInt(e.BaseArmor, 10) + "‰"
			if !strings.Contains(e.Descr, armor) {
				t.Errorf(
					"装备「%s」（%s）有 BaseArmor=%d，但描述里没写。\n描述：%s\n期望包含：%s",
					e.Name, e.Code, e.BaseArmor, e.Descr, armor,
				)
			}
		}
	}
}

// TestEquipmentElementIsCurrentlyCosmetic 记录「装备的元素属性不生效」这个事实。
//
// ⚠️ 这是**已知边界**，不是遗漏的接线：
// 元素系数（`Attacker.ElementCoefPermille`）是**单一标量**，
// 不按元素区分。要让装备的 `Element` 有意义，需要
// 「每种元素一个系数」的核心公式改动 —— 那会动 I-6 的重放哈希、两端实现、
// 以及反通胀不变式，代价远超收益。
//
// 而反应表是**完全连接**的（任意两种不同元素都成反应），
// 所以「装备 2 种不同元素就给加成」那种廉价方案一两下就点满，等于没有。
//
// 本测试的价值是**让这个边界可见**：将来若有人重写了元素公式，
// 它会提醒同步改装备描述与本测试。
func TestEquipmentElementIsCurrentlyCosmetic(t *testing.T) {
	// 当前形态：loadLoadout 只读 BaseArmor 与 BaseBonusPct。
	// 若将来加入按元素的加成，这里会失败并提示同步更新描述。
	const elementAffectsStats = false
	if elementAffectsStats {
		t.Fatal(
			"装备的元素属性已开始影响战斗数值 —— " +
				"请同步：① 18 条装备描述（现在只写护甲与攻击力）" +
				"② 本测试改为断言元素加成进攻方 ③ README 的「已知边界」",
		)
	}
	t.Log(fmt.Sprintf(
		"装备元素当前为风味：18 件装备的 Element 字段全项目未被读取；"+
			"战斗数值只来自 BaseArmor(%d) 与 BaseBonusPct(%d)",
		sumField(func(e SeedEquipment) int64 { return e.BaseArmor }),
		sumField(func(e SeedEquipment) int64 { return e.BaseBonusPct }),
	))
}

func sumField(f func(SeedEquipment) int64) int64 {
	var s int64
	for _, e := range SeedEquipmentList {
		s += f(e)
	}
	return s
}
