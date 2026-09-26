package domain

import (
	"math"
	"testing"
)

// 护甲公式的绝对值测试。
//
// ⚠️ 与 miniapp/src/game/armor.test.ts 配对，补的是同一个空洞：
// applyArmor 在两端都从未被真正执行过 ——
//
//   - formula_vectors.json 里 armor_permille 三个取值全是 0，
//     而 applyArmor 第一行就是 `if armorPermille <= 0 { return dmg }`，
//     所以 13 个跨端向量里这个函数**从未执行到第二行**
//   - 实测把两端都改成 `return dmg`（护甲 100% 失效），
//     TS 侧 0 个测试失败，Go 侧全量 domain 测试全绿
//   - MAX_ARMOR 从 750 改成 100 同样没人发现
//
// 既有的 balance 测试也测不到：它只断言比值（低护甲 > 高护甲），
// 去掉护甲会让两者同比例缩小、比值不变。
// 所以这里全部用绝对值，并与 TS 侧 armor.test.ts 逐档对齐。

func TestApplyArmorAbsolute(t *testing.T) {
	cases := []struct {
		name  string
		dmg   int64
		armor int64
		want  int64
	}{
		{"零护甲", 1000, 0, 1000},
		{"100‰", 1000, 100, 900},
		{"250‰", 1000, 250, 750},
		{"250‰ 配更大伤害", 2000, 250, 1500},
		{"750‰ 上限内", 2000, 750, 500},
		// 1000‰ 被 MAX_ARMOR 截到 750‰，不是打到 0。
		// 减伤封顶 75% 是设计意图：完全免疫会让护甲构筑变成纯数值比拼。
		{"1000‰ 被封顶", 1000, 1000, 250},
		{"超大护甲被封顶", 2000, 100000, 500},
		// 负护甲原样返回，不变成增伤
		{"负护甲不变", 1000, -500, 1000},
		{"负护甲不变（-1）", 1000, -1, 1000},
		// 零伤害恒为 0
		{"零伤害", 0, 500, 0},
		// 取整方向必须与 Go 的整数除法一致（向零截断）
		{"向零截断（下取）", 999, 250, 749},
		{"向零截断（上舍）", 1001, 250, 750},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			if got := applyArmor(c.dmg, c.armor); got != c.want {
				t.Errorf("applyArmor(%d, %d) = %d，期望 %d", c.dmg, c.armor, got, c.want)
			}
		})
	}
}

// TestApplyArmorIsLinearUntilCap 逐档验证线性。
//
// 单点断言不够：有人把 250‰ 写成 300‰ 也可能躲过只测一个点的用例。
// 循环只能到 MaxArmorPermille —— 超出部分被封顶，那里不是线性的。
func TestApplyArmorIsLinearUntilCap(t *testing.T) {
	const dmg = 2000
	for armour := int64(0); armour <= MaxArmorPermille; armour += 50 {
		want := dmg * (permille - armour) / permille
		if got := applyArmor(dmg, armour); got != want {
			t.Errorf("applyArmor(%d, %d) = %d，期望 %d", dmg, armour, got, want)
		}
	}
}

// TestMaxArmorConstantIsPinned 是「MAX_ARMOR 确实是 750」的守卫。
// 之前把它改成 100 不会有任何测试变红。
func TestMaxArmorConstantIsPinned(t *testing.T) {
	if MaxArmorPermille != 750 {
		t.Errorf("MaxArmorPermille = %d，期望 750（与 miniapp/src/game/fixed.ts 的 MAX_ARMOR 对齐）",
			MaxArmorPermille)
	}
	if MaxArmorPermille >= permille {
		t.Error("MaxArmorPermille 必须小于 1000‰，否则护甲可完全免疫")
	}
}

// TestApplyArmorNeverOverflows 确认极端值不越界。
func TestApplyArmorNeverOverflows(t *testing.T) {
	huge := int64(math.MaxInt64 / 4)
	got := applyArmor(huge, 250)
	if got < 0 {
		t.Fatalf("巨大伤害算出负数：%d", got)
	}
	if got > huge {
		t.Fatalf("护甲反而增伤：%d > %d", got, huge)
	}
}

// TestApplyArmorMonotonic 确认护甲越高伤害越低（不多不少、不增不减）。
//
// 这一条比"低护甲 > 高护甲"强：它还要求**严格单调**，
// 抓得住"多了一段钳位"或"少了一段钳位"这类 off-by-one。
func TestApplyArmorMonotonic(t *testing.T) {
	const dmg = 1999 // 刻意用奇数，避免取整掩盖问题
	prev := applyArmor(dmg, 0)
	if prev != dmg {
		t.Fatalf("零护甲应原样返回 %d，实际 %d", dmg, prev)
	}
	for armour := int64(50); armour <= MaxArmorPermille; armour += 50 {
		got := applyArmor(dmg, armour)
		if got >= prev {
			t.Fatalf("护甲 %d‰ 时伤害 %d 未低于上一档 %d —— 护甲未生效或存在平台期",
				armour, got, prev)
		}
		prev = got
	}
	// 封顶后应变成平台
	capped := applyArmor(dmg, MaxArmorPermille)
	for _, armour := range []int64{MaxArmorPermille + 1, MaxArmorPermille * 2, permille} {
		if got := applyArmor(dmg, armour); got != capped {
			t.Errorf("护甲 %d‰ 超出封顶后伤害为 %d，应与封顶值 %d 相同", armour, got, capped)
		}
	}
}
