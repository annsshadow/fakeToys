package domain

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strconv"
	"testing"
)

// ConsistencyVectors 是 testdata/formula_vectors.json 的结构。
type ConsistencyVectors struct {
	LCG struct {
		Seed int64 `json:"seed"`
		// 用字符串承载 64 位值：JSON number 超过 2^53 后 JS 解析即失精，
		// 那样 TS 侧断言的是被截断的近似值，属于假通过。
		Expected []string `json:"expected"`
	} `json:"lcg"`
	Fnv struct {
		Vectors []struct {
			Input    string `json:"input"`
			Expected string `json:"expected"`
		} `json:"vectors"`
	} `json:"fnv1a64"`
	Damage struct {
		Cases []struct {
			Name     string `json:"name"`
			Attacker struct {
				Attack              int64 `json:"attack"`
				ReactionTier        int64 `json:"reaction_tier"`
				ElementCoefPermille int64 `json:"element_coef_permille"`
				ElementCap          int64 `json:"element_cap"`
			} `json:"attacker"`
			Defender struct {
				HP               int64            `json:"hp"`
				ArmorPermille    int64            `json:"armor_permille"`
				StacksPerElement int64            `json:"stacks_per_element"`
				Resist           map[string]int64 `json:"resist"`
			} `json:"defender"`
			Hit struct {
				SkillDamage     int64  `json:"skill_damage"`
				SkillElement    string `json:"skill_element"`
				ForceReaction   string `json:"force_reaction"`
				ReactionElement string `json:"reaction_element"`
				Roll            int64  `json:"roll"`
			} `json:"hit"`
			Expect struct {
				ReactionAttackPortion int64 `json:"reaction_attack_portion"`
				ReactionElemPortion   int64 `json:"reaction_elem_portion"`
				ReactionDamage        int64 `json:"reaction_damage"`
				DirectDamage          int64 `json:"direct_damage"`
				ElementDamage         int64 `json:"element_damage"`
				TotalDamage           int64 `json:"total_damage"`
				Crit                  bool  `json:"crit"`
				Killed                bool  `json:"killed"`
			} `json:"expect"`
		} `json:"cases"`
	} `json:"damage"`
	BalanceInvariants struct {
		MaxReactionAttackWeightPermille int64 `json:"max_reaction_attack_weight_permille"`
	} `json:"balance_invariants"`
	Constants struct {
		ElementPerStackBase  int64 `json:"element_per_stack_base"`
		ElementTickDivisor   int64 `json:"element_tick_divisor"`
		DirectWeightPermille int64 `json:"direct_weight_permille"`
		Permille             int64 `json:"permille"`
		MaxResistPermille    int64 `json:"max_resist_permille"`
		MaxArmorPermille     int64 `json:"max_armor_permille"`
	} `json:"constants"`
	Levelgen struct {
		Levels          int `json:"levels"`
		TerrainLevelMin int `json:"terrain_level_min"`
	} `json:"levelgen"`
}

func loadVectors(t *testing.T) *ConsistencyVectors {
	t.Helper()
	_, thisFile, _, _ := runtime.Caller(0)
	path := filepath.Join(filepath.Dir(thisFile), "..", "..", "testdata", "formula_vectors.json")
	raw, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("读取契约向量失败：%v", err)
	}
	var v ConsistencyVectors
	if err := json.Unmarshal(raw, &v); err != nil {
		t.Fatalf("解析契约向量失败：%v", err)
	}
	return &v
}

// 本文件存在的唯一目的：把"两端公式必须一致"这条约定变成**可执行的检查**。
// TS 侧 src/game/consistency.test.ts 读同一个 JSON 文件，断言同样的数字。
// 任一端单方面改动公式 → 本测试与 TS 测试中至少有一方红。

func TestConsistencyLCG(t *testing.T) {
	v := loadVectors(t)
	r := newLCG(v.LCG.Seed)
	for i, raw := range v.LCG.Expected {
		want, err := strconv.ParseUint(raw, 10, 64)
		if err != nil {
			t.Fatalf("契约向量第 %d 个值 %q 不是合法 uint64：%v", i+1, raw, err)
		}
		if got := r.Next(); got != want {
			t.Errorf("LCG 第 %d 个值：got=%d want=%d（TS 侧必须同步）", i+1, got, want)
		}
	}
}

func TestConsistencyFnv(t *testing.T) {
	v := loadVectors(t)
	for _, c := range v.Fnv.Vectors {
		got := fmt.Sprintf("%016x", fnvSum([]byte(c.Input)))
		if got != c.Expected {
			t.Errorf("FNV-1a(%q) = %s，契约要求 %s", c.Input, got, c.Expected)
		}
	}
}

func TestConsistencyDamage(t *testing.T) {
	v := loadVectors(t)
	for _, c := range v.Damage.Cases {
		t.Run(c.Name, func(t *testing.T) {
			att := DefaultAttacker()
			att.Attack = c.Attacker.Attack
			att.ReactionTier = c.Attacker.ReactionTier
			att.ElementCoefPermille = c.Attacker.ElementCoefPermille
			att.ElementCap = c.Attacker.ElementCap
			att.CritPermille = 0
			att.CritMultiplierPermille = 1500

			def := NewDefender(c.Defender.HP, 0, c.Defender.ArmorPermille)
			for k, val := range c.Defender.Resist {
				def.ResistPermille[Element(k)] = val
			}
			if c.Defender.StacksPerElement > 0 {
				for _, e := range AllElements() {
					def.ElementStacks[e] = c.Defender.StacksPerElement
				}
			}

			var force ReactionKey
			if c.Hit.ForceReaction != "" {
				force = ReactionKey(c.Hit.ForceReaction)
			}
			res := ResolveHit(att, &def, HitInput{
				SkillDamage:     c.Hit.SkillDamage,
				SkillElement:    Element(c.Hit.SkillElement),
				ForceReaction:   force,
				ReactionElement: Element(c.Hit.ReactionElement),
				Roll:            c.Hit.Roll,
			})

			type pair struct {
				name      string
				got, want int64
			}
			for _, p := range []pair{
				{"reaction_attack_portion", res.ReactionAttackPortion, c.Expect.ReactionAttackPortion},
				{"reaction_elem_portion", res.ReactionElemPortion, c.Expect.ReactionElemPortion},
				{"reaction_damage", res.ReactionDamage, c.Expect.ReactionDamage},
				{"direct_damage", res.DirectDamage, c.Expect.DirectDamage},
				{"element_damage", res.ElementDamage, c.Expect.ElementDamage},
				{"total_damage", res.TotalDamage, c.Expect.TotalDamage},
			} {
				if p.got != p.want {
					t.Errorf("%s = %d，契约要求 %d（TS 侧必须同步）", p.name, p.got, p.want)
				}
			}
			if res.Crit != c.Expect.Crit {
				t.Errorf("crit = %v，契约要求 %v", res.Crit, c.Expect.Crit)
			}
			if res.Killed != c.Expect.Killed {
				t.Errorf("killed = %v，契约要求 %v", res.Killed, c.Expect.Killed)
			}
		})
	}
}

func TestConsistencyConstants(t *testing.T) {
	v := loadVectors(t)
	c := v.Constants
	if ElementPerStackBase != c.ElementPerStackBase {
		t.Errorf("ElementPerStackBase = %d，契约要求 %d（TS 侧必须同步）", ElementPerStackBase, c.ElementPerStackBase)
	}
	if ElementTickDivisor != c.ElementTickDivisor {
		t.Errorf("ElementTickDivisor = %d，契约要求 %d", ElementTickDivisor, c.ElementTickDivisor)
	}
	if int64(DirectWeightPermille) != c.DirectWeightPermille {
		t.Errorf("DirectWeightPermille = %d，契约要求 %d", DirectWeightPermille, c.DirectWeightPermille)
	}
	if int64(permille) != c.Permille {
		t.Errorf("permille = %d，契约要求 %d", permille, c.Permille)
	}
	if int64(MaxResistPermille) != c.MaxResistPermille {
		t.Errorf("MaxResistPermille = %d，契约要求 %d", MaxResistPermille, c.MaxResistPermille)
	}
	if int64(MaxArmorPermille) != c.MaxArmorPermille {
		t.Errorf("MaxArmorPermille = %d，契约要求 %d", MaxArmorPermille, c.MaxArmorPermille)
	}
}

func TestConsistencyBalanceInvariants(t *testing.T) {
	v := loadVectors(t)
	want := v.BalanceInvariants.MaxReactionAttackWeightPermille
	if int64(MaxReactionAttackWeightPermille) != want {
		t.Errorf("MaxReactionAttackWeightPermille = %d，契约要求 %d",
			MaxReactionAttackWeightPermille, want)
	}
	// 逐反应复核
	for _, spec := range AllReactionSpecs() {
		if int64(spec.AttackWeightPct) > want {
			t.Errorf("反应 %s 攻击力权重 %d‰ > 契约上限 %d‰", spec.Name, spec.AttackWeightPct, want)
		}
	}
}

func TestConsistencyLevelgen(t *testing.T) {
	v := loadVectors(t)
	if v.Levelgen.Levels != TotalLevels {
		t.Errorf("TotalLevels = %d，契约要求 %d", TotalLevels, v.Levelgen.Levels)
	}
	all := GenerateAllLevels()
	if len(all) != v.Levelgen.Levels {
		t.Errorf("生成关卡数 %d，契约要求 %d", len(all), v.Levelgen.Levels)
	}
	terrain := 0
	for _, gl := range all {
		if len(gl.Terrain) > 0 {
			terrain++
		}
	}
	if terrain < v.Levelgen.TerrainLevelMin {
		t.Errorf("地形关 %d 关，契约要求至少 %d 关", terrain, v.Levelgen.TerrainLevelMin)
	}
}

// fnvSum 是 FNV-1a 64 位实现（与 TS 侧 lcg.ts 的 fnv1a64 等价）。
func fnvSum(data []byte) uint64 {
	const (
		offset = 14695981039346656037
		prime  = 1099511628211
	)
	h := uint64(offset)
	for _, b := range data {
		h ^= uint64(b)
		h *= prime
	}
	return h
}
