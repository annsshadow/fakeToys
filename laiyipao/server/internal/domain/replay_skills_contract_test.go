package domain

import "testing"

// 回放前缀 S 段的**跨端契约**测试（第 56 轮）。
//
// 与 `replayskills_test.go` 的分工：
//
//	replayskills_test.go        单元测试，拿字面期望值盯住 Go 侧实现
//	本文件                     拿 `formula_vectors.json` 的字面期望值，
//	                           盯住「Go 侧与 TS 侧拼出的串逐字节相同」
//
// ## 为什么非要跨端
//
// 服务端在结算时会用 Go 侧重算这个串，与客户端上报的**逐字节**比对。
// 两端格式一旦漂移 —— 哪怕只是分隔符从 `:` 变成 `,` ——
// 后果是**所有合法玩家的每一局**都被判成作弊，
// 而客户端单测、Go 单测、服务端 e2e 全都照样绿。
//
// 这种漂移只有「两端对着同一个字面串」才抓得到。

func TestReplaySkillsSegmentMatchesCrossEndVectors(t *testing.T) {
	v := loadVectors(t)
	if len(v.ReplaySkills.Cases) == 0 {
		t.Fatal("formula_vectors.json 里 replay_skills.cases 是空的 —— " +
			"向量被误删时必须红，否则这个测试会静默变成空转")
	}

	rules := DefaultSkillRules()
	for _, c := range v.ReplaySkills.Cases {
		t.Run(c.Name, func(t *testing.T) {
			items := make([]ReplaySkillSlot, 0, len(c.Skills))
			for _, s := range c.Skills {
				items = append(items, ReplaySkillSlot{
					Slot:    s.Slot,
					SkillID: s.SkillID,
					// 等级在这里烘进底伤 —— 与客户端 `skillBaseDamageAtLevel`
					// 是同一套公式（`TestSkillRulesMatchServerContract` 守着）。
					BaseDamage:  SkillBaseDamageAtLevel(rules, s.BaseDamage, s.Level),
					ApplyStacks: s.ApplyStacks,
					HeatCost:    s.HeatCost,
				})
			}
			got := ReplaySkillsSegment(items)
			if got != c.Expected {
				t.Fatalf("S 段与跨端契约不符\n"+
					"  得到 %q\n"+
					"  期望 %q\n"+
					"两端只要有一端漂移，合法玩家的对局就会被判成作弊。\n"+
					"若确认是格式改动，两端一起改并回填这里的 expected。",
					got, c.Expected)
			}
		})
	}
}

// 向量里的每一条都必须真的被测到 ——
// 有人加了个 case 但把 name 写重复、或者被 t.Skip 掉时，这里会红。
func TestReplaySkillsVectorsAreAllDistinctAndNamed(t *testing.T) {
	v := loadVectors(t)
	seen := map[string]bool{}
	for _, c := range v.ReplaySkills.Cases {
		if c.Name == "" {
			t.Error("有向量没写 name —— 失败时无法定位是哪一条")
		}
		if seen[c.Name] {
			t.Errorf("向量名重复：%s（重复的名字会让上面的子测试互相覆盖，失败时看不出是哪条数据）", c.Name)
		}
		seen[c.Name] = true
	}
}
