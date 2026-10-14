package domain

// 「引擎纯函数」的跨端契约守卫（R1 第一批移植）。
//
// # 判据：Go 必须与 `testdata/engine_pure_vectors.json` 逐位一致
//
// 那个文件由 `miniapp/scripts/gen-engine-pure-vectors.ts` 从**真实 TS 实现**产出。
// 期望值不由 Go 侧生成 —— 纯函数没有运行时状态，
// 用实现产出期望值是**定义**而不是自证（理由同 replay_vectors.json）。
//
// # 为什么这批比「整局重放」契约更该先做
//
// 整局重放一旦有偏差，症状是**哈希失配**：一眼看不出是哪一步算错了。
// 纯函数偏差的症状是「这个函数的返回值不对」—— 能精确定位、能单独修。
//
// 所以移植顺序是：先纯函数（本文件），最后 tick 循环。

import (
	"encoding/json"
	"math"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"testing"
	"time"
)

// --- 向量文件结构 ---

type pureVectors struct {
	Skill struct {
		Rules struct {
			MaxLevel     int   `json:"maxLevel"`
			CoefPermille int64 `json:"coefPermille"`
			BaseCost     int64 `json:"baseCost"`
		} `json:"rules"`
		Rows []struct {
			Level           int    `json:"level"`
			Base            string `json:"base"`
			ClampedLevel    int    `json:"clamped_level"`
			CoefPermille    int64  `json:"coef_permille"`
			BaseDamageAtLvl string `json:"base_damage_at_level"`
			UpgradeCost     int64  `json:"upgrade_cost"`
			IsMaxLevel      bool   `json:"is_max_level"`
		} `json:"rows"`
	} `json:"skill"`

	Works struct {
		BaseAttacker struct {
			Attack                 string `json:"attack"`
			CritPermille           string `json:"crit_permille"`
			CritMultiplierPermille string `json:"crit_multiplier_permille"`
			ReactionMultPermille   string `json:"reaction_mult_permille"`
			ElementCap             string `json:"element_cap"`
			ReactionTier           string `json:"reaction_tier"`
			ElementCoefPermille    string `json:"element_coef_permille"`
		} `json:"base_attacker"`
		Rows []struct {
			Label                string   `json:"label"`
			Works                []string `json:"works"`
			ElementCoefPermille  string   `json:"element_coef_permille"`
			ElementCap           string   `json:"element_cap"`
			ReactionMultPermille string   `json:"reaction_mult_permille"`
		} `json:"rows"`
	} `json:"works"`

	ChallengeSeed struct {
		Rows []struct {
			DefenseID    int64  `json:"defense_id"`
			NowMs        int64  `json:"now_ms"`
			MinuteBucket int64  `json:"minute_bucket"`
			Seed         string `json:"seed"`
		} `json:"rows"`
	} `json:"challenge_seed"`

	SnapshotValidation struct {
		Rows []struct {
			Label  string `json:"label"`
			NowMs  int64  `json:"now_ms"`
			OK     bool   `json:"ok"`
			Reason string `json:"reason"`
		} `json:"rows"`
	} `json:"snapshot_validation"`

	Shield struct {
		Rows []struct {
			Label  string  `json:"label"`
			Until  *string `json:"until"`
			NowMs  int64   `json:"now_ms"`
			Active bool    `json:"active"`
		} `json:"rows"`
	} `json:"shield"`

	Elements struct {
		Rows []struct {
			Label    string   `json:"label"`
			Elements []string `json:"elements"`
		} `json:"rows"`
	} `json:"elements"`

	Digest struct {
		Rows []struct {
			Label  string   `json:"label"`
			Works  []string `json:"works"`
			Skills []int64  `json:"skills"`
			Digest string   `json:"digest"`
		} `json:"rows"`
	} `json:"digest"`

	Fnv struct {
		Rows []struct {
			Input string `json:"input"`
			Hex   string `json:"hex"`
		} `json:"rows"`
	} `json:"fnv"`

	// Heat 用「脚本 + 状态轨迹」而不是逐函数取值。
	//
	// 热量那类缺陷（吸收态、余数截断）单看一个函数的返回值看不出来，
	// 要连续跑若干步才现形。
	Heat struct {
		Consts struct {
			ActiveSlots        int    `json:"ACTIVE_SLOTS"`
			PassiveSlot        int    `json:"PASSIVE_SLOT"`
			TotalSlots         int    `json:"TOTAL_SLOTS"`
			HeatMax            string `json:"HEAT_MAX"`
			OverheatDurationMs int    `json:"OVERHEAT_DURATION_MS"`
			HeatDecayPerSec    string `json:"HEAT_DECAY_PER_SEC"`
			DiscardPerWave     int    `json:"DISCARD_PER_WAVE"`
			DiscardHeatRefund  string `json:"DISCARD_HEAT_REFUND"`
		} `json:"consts"`
		Rows []struct {
			Label    string `json:"label"`
			CapBonus string `json:"cap_bonus"`
			Trace    []struct {
				Op string `json:"op"`
				// 输入参数（每种 op 只有一个非 nil）
				Cost  *string  `json:"cost"`
				N     *string  `json:"n"`
				DtMs  *float64 `json:"dt_ms"`
				Costs []string `json:"costs"`
				// 输出与状态（**每步之后**的值）
				Heat              string `json:"heat"`
				Overheated        bool   `json:"overheated"`
				OverheatRemaining int    `json:"overheat_remaining"`
				Cap               string `json:"cap"`
				MaxHeat           string `json:"max_heat"`
				// ⚠️ 衰减余数必须进向量：只比 heat 的话，
				// 「四舍五入」与「截断」在短脚本里给出同一个整数衰减，
				// 守卫就证明不了任何东西（实测：去掉 math.Round 测试没红）。
				DecayRemainder string `json:"decay_remainder"`
				Result         *bool  `json:"result"`
			} `json:"trace"`
		} `json:"rows"`
	} `json:"heat"`

	Deck struct {
		Rows []struct {
			Label string `json:"label"`
			Trace []struct {
				Op           string   `json:"op"`
				Hand         []string `json:"hand"`
				Size         int      `json:"size"`
				DiscardsLeft int      `json:"discards_left"`
				OK           *bool    `json:"ok"`
				Refund       *string  `json:"refund"`
				ID           string   `json:"id"`
			} `json:"trace"`
		} `json:"rows"`
	} `json:"deck"`

	Terrain struct {
		Consts struct {
			FieldW            int   `json:"FIELD_W"`
			FieldH            int   `json:"FIELD_H"`
			BaseX             int   `json:"BASE_X"`
			BaseY             int   `json:"BASE_Y"`
			OilBurnMs         int   `json:"OIL_BURN_MS"`
			RotorRadius       int   `json:"ROTOR_RADIUS"`
			RotorDirLen       int   `json:"ROTOR_DIR_LEN"`
			RotorDirSampleIdx []int `json:"ROTOR_DIR_SAMPLE_IDX"`
			RotorDirSample    []int `json:"ROTOR_DIR_SAMPLE"`
		} `json:"consts"`
		Rows []struct {
			Label     string `json:"label"`
			Placement struct {
				Kind  string `json:"kind"`
				X     int    `json:"x"`
				Y     int    `json:"y"`
				Param int    `json:"param"`
			} `json:"placement"`
			TerrainTick string `json:"terrain_tick"`
			ElementCap  string `json:"element_cap"`
			Enemies     []struct {
				ID string `json:"id"`
				HP string `json:"hp"`
				// armor 存字符串：向量里的整数一律走字符串，
				// 避免 2^53 以上的值在 JSON number 里失精。
				Armor string `json:"armor"`
				X     int    `json:"x"`
				Y     int    `json:"y"`
			} `json:"enemies"`
			Projectiles []struct {
				ID string `json:"id"`
				X  int    `json:"x"`
				Y  int    `json:"y"`
				VX string `json:"vx"`
				VY string `json:"vy"`
			} `json:"projectiles"`
			Trace []struct {
				Op string `json:"op"`
				// ⚠️ tag 必须是 `dt_ms`（snake_case，与向量文件一致）。
				//   写成 `dtMs` 时 JSON 反序列化**静默得到 0** ——
				//   于是一切 update 都用 dt=0 跑，看起来像「状态机不对」，
				//   而真正的原因是字段名拼错。
				//   这是我写这份契约时踩到的，记下来以免重犯。
				DtMs      int64  `json:"dt_ms"`
				Blocked   *bool  `json:"blocked"`
				State     string `json:"state"`
				Timer     int64  `json:"timer"`
				Charge    int    `json:"charge"`
				Angle     int    `json:"angle"`
				Triggered bool   `json:"triggered"`
				Element   string `json:"element"`
				Damage    string `json:"damage"`
				HitX      int64  `json:"hit_x"`
				HitY      int64  `json:"hit_y"`
				Cap       string `json:"cap"`
				Result    *bool  `json:"result"`
				// 敌人快照。向量里是对象数组，这里渲染成单行字符串
				// 以便与 Go 侧的 enemySnapshots 逐字比对。
				Enemies []struct {
					ID            string `json:"id"`
					HP            string `json:"hp"`
					ArmorPermille string `json:"armor_permille"`
					Dead          bool   `json:"dead"`
					HitFlashMs    int64  `json:"hit_flash_ms"`
					Elements      string `json:"elements"`
				} `json:"enemies"`
				Projectiles []string `json:"projectiles"`
				// ctx 调用序列。存数组而不是拼接字符串：
				// 报错时能指明「第几次调用不对」，而不只是「整串不一样」。
				Calls []string `json:"calls"`
			} `json:"trace"`
		} `json:"rows"`
	} `json:"terrain"`

	// Wave 是 Block B（波次与刷怪）的对齐数据。
	Wave struct {
		Consts struct {
			TickMs            int `json:"TICK_MS"`
			SpawnProgressFull int `json:"SPAWN_PROGRESS_FULL"`
			SpawnProgressStep int `json:"SPAWN_PROGRESS_STEP"`
		} `json:"consts"`
		Rows []struct {
			Label   string `json:"label"`
			LevelID int    `json:"level_id"`
			Seed    string `json:"seed"`
			Ticks   int    `json:"ticks"`
			// SampleEvery 是快照抽样间隔。
			// ⚠️ Go 侧**按它自己算**「哪几步要快照」，
			//   而不是照抄 trace 的下标 —— 照抄下标就等于
			//   「Go 跑到第 k 个采样点就断言第 k 个期望值」，
			//   两端之间没有任何时序约束，退化成逐点查表。
			SampleEvery int `json:"sample_every"`
			// AttackerArmorPermille 是攻方护甲（输入回显）。
			//
			// ⚠️ 必须带进文件：`defenseArmorPermille` 是三项相加，
			//   前两项（关卡护甲、卡牌护甲）在这批场景里恒为 0，
			//   不显式给一个非零值的话，第三项永远等于「加 0」——
			//   把它写成减号也照样全绿。
			AttackerArmorPermille int64 `json:"attacker_armor_permille"`
			// LevelBaseHP / LevelArmorPermille 是关卡的血与护甲。
			// ⚠️ 必须带进文件：漏怪伤害 = base_hp × attack / 3600，
			//   拿不到 base_hp 就只能走 100 的兜底，
			//   整条漏怪伤害都算错而**看不出**为什么。
			LevelBaseHP        int64 `json:"level_base_hp"`
			LevelArmorPermille int64 `json:"level_armor_permille"`
			// LevelWaves 是关卡的波次定义（输入回显）。
			// ⚠️ 必须带进文件：Go 侧要重建同样的刷怪组才能跑，
			// 而「delay/interval 是毫秒」正是最容易写错的那处换算。
			LevelWaves []waveLevelDef `json:"level_waves"`
			// Trace 用**具名类型** waveTraceStep（定义在 engine_wave_contract_test.go）。
			// ⚠️ 必须是具名的：匿名结构体与具名结构体之间不能隐式转换，
			// 而跨文件共享一个形状时匿名写法每次都要逐字复制一遍 tag ——
			// 复制过程中改错一个 tag 就是「静默得到零值」。
			Trace []waveTraceStep `json:"trace"`
		} `json:"rows"`
	} `json:"wave"`
}

func loadPureVectors(t *testing.T) *pureVectors {
	t.Helper()
	path := filepath.Join("..", "..", "testdata", "engine_pure_vectors.json")
	raw, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("读契约向量失败（先跑 miniapp/scripts/gen-engine-pure-vectors.ts）：%v", err)
	}
	var v pureVectors
	if err := json.Unmarshal(raw, &v); err != nil {
		t.Fatalf("契约向量解析失败：%v", err)
	}
	return &v
}

func mustInt64(t *testing.T, s string) int64 {
	t.Helper()
	n, err := strconv.ParseInt(s, 10, 64)
	if err != nil {
		t.Fatalf("向量里的 %q 不是合法 int64：%v", s, err)
	}
	return n
}

// --- 技能 ---

// TestSkillVectorsMatchCrossEnd 断言技能系数、底伤、费用、满级判定两端一致。
func TestSkillVectorsMatchCrossEnd(t *testing.T) {
	v := loadPureVectors(t)
	rules := DefaultSkillRules()

	// 规则本身也要一致 —— 不一致的话下面每一行都会红，
	// 而报错信息会指向具体某个数值，看不出「是规则漂了还是函数漂了」。
	if rules.MaxLevel != v.Skill.Rules.MaxLevel ||
		rules.CoefPermille != v.Skill.Rules.CoefPermille ||
		rules.BaseCost != v.Skill.Rules.BaseCost {
		t.Fatalf("技能规则两端不一致：Go=%+v TS=%+v", rules, v.Skill.Rules)
	}

	for _, r := range v.Skill.Rows {
		base := mustInt64(t, r.Base)
		t.Run("L"+strconv.Itoa(r.Level)+"_b"+r.Base, func(t *testing.T) {
			if got := rules.ClampLevel(r.Level); got != r.ClampedLevel {
				t.Errorf("ClampLevel(%d) = %d，TS 期望 %d", r.Level, got, r.ClampedLevel)
			}
			if got := rules.CoefPermilleAt(r.Level); got != r.CoefPermille {
				t.Errorf("CoefPermilleAt(%d) = %d，TS 期望 %d", r.Level, got, r.CoefPermille)
			}
			want := mustInt64(t, r.BaseDamageAtLvl)
			if got := SkillBaseDamageAtLevel(rules, base, r.Level); got != want {
				t.Errorf("SkillBaseDamageAtLevel(lvl=%d, base=%s) = %d，TS 期望 %d",
					r.Level, r.Base, got, want)
			}
			if got := rules.CostFrom(r.Level); got != r.UpgradeCost {
				t.Errorf("CostFrom(%d) = %d，TS 期望 %d", r.Level, got, r.UpgradeCost)
			}
		})
	}
}

// TestSkillVectorsStayInsideSafeIntRange 断言契约文件没有让 Go 中间乘法溢出的用例。
//
// # 这条守的到底是什么（第一次写成时把它说错了，值得记下来）
//
// 我第一版的注释写「两端不可能一致」—— **不对**。
//
// 溢出的是 Go 侧 `base * coef` 这个**中间乘积**，不是最终结果。
// 举例：`base = 2^63-1`、`coef = 1450` 时，
// 最终结果 `base*1450/1000 ≈ 1.34e19` 确实超出 int64（装不下），
// 但取 `base = 6.4e15` 时最终结果是 `9.28e18`（**装得下**），
// 而中间乘积 `9.28e21` 早就溢出了 —— TS 是 BigInt，照样精确。
//
// 所以准确的说法是：**中间乘积溢出即两端不一致**，
// 判据必须是 `base*coef` 而不是 `base*coef/1000`。
//
// 写成后一种的话，一个 `base` 落在「结果装得下但中间装不下」的区间时
// 守卫会**放行**，而那正是会静默出错的地方。
//
// 生成器已避开这类用例；本条把它钉住。理由：
//   - 生成会让 Go 溢出的向量 = 给契约埋一颗永远红的雷；
//   - 而悄悄避开不提 = 藏了一个限制，将来有人把底伤上限调高就踩上去。
func TestSkillVectorsStayInsideSafeIntRange(t *testing.T) {
	v := loadPureVectors(t)
	rules := DefaultSkillRules()
	for _, r := range v.Skill.Rows {
		base := mustInt64(t, r.Base)
		coef := rules.CoefPermilleAt(r.Level)
		if coef == 1000 {
			continue // 恒等快路径，不做乘法
		}
		if base != 0 && base > math.MaxInt64/coef {
			t.Errorf("level=%d base=%s：中间乘积 base*%d 超出 int64，"+
				"Go 侧先乘后除会静默溢出（TS 是 BigInt 没有这个问题）。"+
				"要么把取值范围限制住，要么改实现避免中间溢出",
				r.Level, r.Base, coef)
		}
	}
}

// TestSkillBaseDamageIdentityHoldsAtInt64Max 守住等级 1 的恒等性。
//
// # 为什么这条独立于契约
//
// 契约只能覆盖生成器挑的取值；而这一条守的是**性质**：
// 等级 1 的系数恒为 1000‰，结果必须**逐位等于**输入。
//
// 它是兼容性要求而非性能优化：升级功能上线前所有
// `user_skills.level` 恒为 1，所以所有**存量战报的哈希必须逐位不变**。
// 而哈希把 `baseDamage` 拼进锚点 ——
// 一旦这一位变了，每一个历史战报都会失配，
// 而失配表现为「验真失败」，看起来像作弊。
//
// ⚠️ 这里刻意用 `math.MaxInt64`：没有那条恒等快路径的话
// `MaxInt64 * 1000` 会溢出，结果是一个完全不同的数。
// 用普通取值（例如 100）的话这个测试**照样绿**，什么也证明不了。
func TestSkillBaseDamageIdentityHoldsAtInt64Max(t *testing.T) {
	rules := DefaultSkillRules()
	for _, base := range []int64{0, 1, 33, 100, 1 << 52, math.MaxInt64} {
		// level 1：恒等
		if got := SkillBaseDamageAtLevel(rules, base, 1); got != base {
			t.Errorf("等级 1 时底伤 %d 应逐位不变，实际 %d —— "+
				"存量战报的哈希会全部失配，且症状看起来像作弊", base, got)
		}
		// ⚠️ 只有**低于下界**的等级才夹到 1（系数 1000‰ → 恒等）。
		//
		//   我第一版把 0/-1/999 放在同一组断言「都夹到 1」，红了 ——
		//   `ClampLevel(999)` 夹到的是 **MaxLevel(10)**，系数 1450‰，
		//   结果是 base×1.45 而不是原值。
		//
		//   两处夹紧的**方向不同**是刻意的：
		//     低于 1   → 夹到 1（不存在「0 级技能」）
		//     高于上限 → 夹到上限（脏数据不该产出无限加成）
		//   混为一谈会写出「999 级等于 1 级」这种错误描述。
		for _, lvl := range []int{0, -1} {
			if got := SkillBaseDamageAtLevel(rules, base, lvl); got != base {
				t.Errorf("等级 %d 应夹到 1（系数 1000‰），底伤 %d 应不变，实际 %d", lvl, base, got)
			}
		}
		// 超上限等级夹到 MaxLevel，结果应是 base × 1.45
		for _, lvl := range []int{11, 999} {
			want := base * rules.CoefPermilleAt(rules.MaxLevel) / 1000
			if got := SkillBaseDamageAtLevel(rules, base, lvl); got != want {
				t.Errorf("等级 %d 应夹到 MaxLevel(%d)，底伤 %d → %d，实际 %d",
					lvl, rules.MaxLevel, base, want, got)
			}
		}
	}
}

// --- 工程装置 ---

// TestApplyWorksMatchesCrossEnd 断言装置加成两端一致。
func TestApplyWorksMatchesCrossEnd(t *testing.T) {
	v := loadPureVectors(t)
	base := Attacker{
		Attack:                 mustInt64(t, v.Works.BaseAttacker.Attack),
		CritPermille:           mustInt64(t, v.Works.BaseAttacker.CritPermille),
		CritMultiplierPermille: mustInt64(t, v.Works.BaseAttacker.CritMultiplierPermille),
		ReactionMultPermille:   mustInt64(t, v.Works.BaseAttacker.ReactionMultPermille),
		ElementCap:             mustInt64(t, v.Works.BaseAttacker.ElementCap),
		ReactionTier:           mustInt64(t, v.Works.BaseAttacker.ReactionTier),
		ElementCoefPermille:    mustInt64(t, v.Works.BaseAttacker.ElementCoefPermille),
	}
	for _, r := range v.Works.Rows {
		got := ApplyWorks(base, r.Works)
		if want := mustInt64(t, r.ElementCoefPermille); got.ElementCoefPermille != want {
			t.Errorf("%s: ElementCoefPermille = %d，TS 期望 %d", r.Label, got.ElementCoefPermille, want)
		}
		if want := mustInt64(t, r.ElementCap); got.ElementCap != want {
			t.Errorf("%s: ElementCap = %d，TS 期望 %d", r.Label, got.ElementCap, want)
		}
		if want := mustInt64(t, r.ReactionMultPermille); got.ReactionMultPermille != want {
			t.Errorf("%s: ReactionMultPermille = %d，TS 期望 %d", r.Label, got.ReactionMultPermille, want)
		}
		// ApplyWorks 必须**不改动入参**（TS 是 `{ ...att }` 展开）
		if base.ElementCoefPermille != mustInt64(t, v.Works.BaseAttacker.ElementCoefPermille) {
			t.Fatalf("ApplyWorks 改动了传入的 Attacker —— 调用点复用同一份攻方时会互相污染")
		}
	}
}

// TestApplyWorksMatchesDocumentedMapping 独立钉住「三种装置各加多少」。
//
// # 为什么这条独立于契约向量
//
// 向量生成器里 `applyWorks` 是**未导出**的，所以生成器只能复刻它的 switch。
// 也就是说那条向量证明的是「我复刻得对」，不是「TS 的实现是这样」。
//
// 复刻与原实现漂移时，只有本条会红 —— 因为它把映射表写成了
// **不依赖生成器**的字面量。所以两边都得有人守着。
func TestApplyWorksMatchesDocumentedMapping(t *testing.T) {
	var zero Attacker
	cases := []struct {
		work                   string
		coef, cap, reactionAdd int64
	}{
		{"slow_belt", 100, 0, 0},
		{"block_wall", 0, 1, 0},
		{"tesla_grid", 0, 0, 200},
	}
	for _, c := range cases {
		got := ApplyWorks(zero, []string{c.work})
		if got.ElementCoefPermille != c.coef {
			t.Errorf("%s → ElementCoefPermille +%d，实际 +%d", c.work, c.coef, got.ElementCoefPermille)
		}
		if got.ElementCap != c.cap {
			t.Errorf("%s → ElementCap +%d，实际 +%d", c.work, c.cap, got.ElementCap)
		}
		if got.ReactionMultPermille != c.reactionAdd {
			t.Errorf("%s → ReactionMultPermille +%d，实际 +%d", c.work, c.reactionAdd, got.ReactionMultPermille)
		}
	}
}

// --- 挑战种子 ---

// TestChallengeSeedMatchesCrossEnd 断言种子派生两端一致。
//
// 这是本批契约里**安全意义最大**的一条：种子完全由
// (defense_id, 分钟桶) 决定，服务端自己就能算出来，
// 因此「客户端报的种子对不对」从此可验证。
func TestChallengeSeedMatchesCrossEnd(t *testing.T) {
	v := loadPureVectors(t)
	for _, r := range v.ChallengeSeed.Rows {
		now := time.UnixMilli(r.NowMs).UTC()
		got := DeriveChallengeSeed(r.DefenseID, now)
		want := mustInt64(t, r.Seed)
		if got != want {
			t.Errorf("DeriveChallengeSeed(id=%d, now=%d) = %d，TS 期望 %d",
				r.DefenseID, r.NowMs, got, want)
		}
		// 分钟桶也要一致 —— 它是种子公式里的中间量，
		// 两端对「分钟」的边界理解不同会让整条公式错开一格。
		if minute := now.UnixMilli() / 60000; minute != r.MinuteBucket {
			t.Errorf("分钟桶 = %d，向量期望 %d", minute, r.MinuteBucket)
		}
	}
}

// TestChallengeSeedIsStableWithinMinuteAndChangesAcross 守住种子派生的两条设计意图。
//
// 这两条正是 `ChallengeDeps.seedOverride` 注释里说的：
// 「同一对组合在一分钟内结果固定、跨分钟变化，避免反复挑战得到完全相同的战局」。
//
// ⚠️ 判据用**分钟边界两侧**（+1ms 与 -1ms），而不是随便取两个时刻：
//
//	否则一个按秒取桶的实现也能全绿，而它会让「同一分钟内固定」这条失效
//	—— 玩家在一分钟内反复挑战就能拿到不同战局，正是要避免的那个。
func TestChallengeSeedIsStableWithinMinuteAndChangesAcross(t *testing.T) {
	// ⚠️ 基准时刻必须**对齐到分钟边界**。
	//
	//   第一版直接用 1_700_000_000_000，而它除以 60000 是 28333333.33 ——
	//   不在整分上。于是「+59999ms 仍应同一分钟」这条断言跨过了边界，
	//   而**被测代码是对的**，红的是我的前提。
	//
	//   判据落在一个错误的前提上时，它守的不是代码而是我的想当然 ——
	//   这类失败比「测试红」更值得记，因为它差点让我去改对的实现。
	const baseMs = 1_700_000_040_000 // 28333334 × 60000，整分
	if baseMs%60000 != 0 {
		t.Fatalf("测试前提错误：基准时刻 %d 不在分钟边界上", baseMs)
	}
	base := time.UnixMilli(baseMs).UTC()

	s0 := DeriveChallengeSeed(42, base)
	if s1 := DeriveChallengeSeed(42, base.Add(time.Millisecond)); s1 != s0 {
		t.Errorf("同一分钟内种子应不变：%d → %d（+1ms 就变了会让「一分钟内固定」失效）", s0, s1)
	}
	if s1 := DeriveChallengeSeed(42, base.Add(59*time.Second+999*time.Millisecond)); s1 != s0 {
		t.Errorf("同一分钟末尾（+59.999s）种子应不变：%d → %d", s0, s1)
	}
	if s1 := DeriveChallengeSeed(42, base.Add(60*time.Second)); s1 == s0 {
		t.Error("跨分钟后种子必须变化，否则反复挑战得到完全相同的战局")
	}
	// 不同防线也不同种子 —— 否则「挑战 A」与「挑战 B」拿到同一个战局
	if DeriveChallengeSeed(43, base) == s0 {
		t.Error("不同 defense_id 应派生出不同种子")
	}
}

// TestChallengeSeedNeverNegative 守住 63 位掩码。
//
// 掩码存在的理由：超大 defense_id 或负 id 都会溢出 int64。
// 没有掩码时两端各自溢出成不同的东西，而症状是「种子偶尔对不上」——
// 那在排查时几乎不可能联想到「掩码」。
func TestChallengeSeedNeverNegative(t *testing.T) {
	now := time.UnixMilli(1_700_000_000_000).UTC()
	for _, id := range []int64{0, 1, -1, math.MaxInt64, math.MinInt64} {
		if got := DeriveChallengeSeed(id, now); got < 0 {
			t.Errorf("DeriveChallengeSeed(%d) = %d，必须非负（掩码失效）", id, got)
		}
	}
}

// --- 快照校验 ---

// parseRFC3339 复刻 TS `Date.parse` 在本契约用到的输入上的行为。
//
// ⚠️ **只接受 RFC3339**，其它一律 nil。
//
// TS 那边 `Date.parse('1700000000000')` 返回 NaN（纯数字串不是
// ES 规范的日期时间格式），于是护盾被视为无效。
// 若 Go 侧用更宽松的解析（比如把纯数字当毫秒时间戳），
// 两端会对「1700000000000」给出相反结论 —— 而这种输入只可能来自
// 老数据或手改，症状是「部分用户的护盾判断与客户端不一致」。
func parseRFC3339(s *string) *time.Time {
	if s == nil || *s == "" {
		return nil
	}
	t, err := time.Parse(time.RFC3339, *s)
	if err != nil {
		return nil
	}
	return &t
}

func TestValidateSnapshotMatchesCrossEnd(t *testing.T) {
	v := loadPureVectors(t)
	full := &DefenseSnapshot{
		Skills: []int64{1, 2, 3},
		Works:  []string{"slow_belt"},
	}
	for _, r := range v.SnapshotValidation.Rows {
		// 输入按 label 在本文件里重建，**不**从 JSON 读 ——
		// 向量只存期望值。好处是「输入写错」与「实现算错」
		// 表现为不同用例失败；代价是新增 label 时必须同步在这里补，
		// 所以下面有 default 分支显式报错，而不是默默跳过。
		var view DefenseView
		now := time.UnixMilli(r.NowMs).UTC()
		switch r.Label {
		case "ok":
			view = DefenseView{Snapshot: full, SnapshotHash: "0123456789abcdef"}
		case "no_snapshot":
			view = DefenseView{SnapshotHash: "0123456789abcdef"}
		case "empty_skills":
			view = DefenseView{Snapshot: &DefenseSnapshot{}, SnapshotHash: "0123456789abcdef"}
		case "empty_skills_beats_missing_hash":
			// 技能空 + 无哈希：必须报「无技能」，验证**顺序**
			view = DefenseView{Snapshot: &DefenseSnapshot{}}
		case "missing_hash":
			view = DefenseView{Snapshot: full}
		case "shield_future":
			until := now.Add(time.Hour)
			view = DefenseView{Snapshot: full, SnapshotHash: "h", ShieldedUntil: &until}
		case "shield_past":
			until := now.Add(-time.Hour)
			view = DefenseView{Snapshot: full, SnapshotHash: "h", ShieldedUntil: &until}
		case "shield_exactly_now":
			until := now
			view = DefenseView{Snapshot: full, SnapshotHash: "h", ShieldedUntil: &until}
		case "shield_unparseable", "no_shield_field":
			// 这两条在 Go 侧都是 ShieldedUntil=nil ——
			// 差别只在 TS 那边一个是「字符串解析失败」一个是「字段缺失」，
			// 而 Go 的 `*time.Time` 表达这两种情况的方式相同。
			// 护盾字符串解析的独立性由 TestShieldMatchesCrossEnd 覆盖。
			view = DefenseView{Snapshot: full, SnapshotHash: "h"}
		default:
			t.Fatalf("向量里有本文件未覆盖的 label %q —— "+
				"新加用例时要在这里补对应输入，否则这一条会假绿", r.Label)
		}

		ok, reason := ValidateSnapshot(view, now)
		if ok != r.OK {
			t.Errorf("%s: ok = %v，TS 期望 %v（Go 给出的 reason=%q）", r.Label, ok, r.OK, reason)
		}
		if reason != r.Reason {
			t.Errorf("%s: reason = %q，TS 期望 %q —— "+
				"话术是排查问题的第一线索，两端不一致会出现「客户端说能打、服务端说不能打」",
				r.Label, reason, r.Reason)
		}
	}
}

// TestShieldMatchesCrossEnd 断言护盾判断两端一致。
func TestShieldMatchesCrossEnd(t *testing.T) {
	v := loadPureVectors(t)
	for _, r := range v.Shield.Rows {
		now := time.UnixMilli(r.NowMs).UTC()
		got := IsShieldActive(parseRFC3339(r.Until), now)
		if got != r.Active {
			t.Errorf("%s: IsShieldActive(%v) = %v，TS 期望 %v", r.Label, r.Until, got, r.Active)
		}
	}
}

// TestSnapshotElementsMatchesCrossEnd 断言元素过滤两端一致。
func TestSnapshotElementsMatchesCrossEnd(t *testing.T) {
	v := loadPureVectors(t)
	for _, r := range v.Elements.Rows {
		// 'FIRE' 大写不在合法集合里 —— 大小写敏感是刻意的，
		// 元素 key 全是小写常量，放开大小写会让快照能塞进畸形值。
		snap := &DefenseSnapshot{Elements: []string{"fire", "plasma", "ice", "void", "kinetic", "FIRE"}}
		got := SnapshotElements(snap)
		if strings.Join(got, ",") != strings.Join(r.Elements, ",") {
			t.Errorf("%s: SnapshotElements = %v，TS 期望 %v", r.Label, got, r.Elements)
		}
	}
	// nil 快照必须返回空数组而不是 nil —— nil 会在 JSON 序列化成 null，
	// 而前端按数组渲染。
	if got := SnapshotElements(nil); got == nil {
		t.Error("SnapshotElements(nil) 应返回空切片，nil 会序列化成 null")
	}
}

// --- 热量系统 ---

// TestHeatConstantsMatchCrossEnd 断言热量常量两端一致。
//
// 这些常量进结算上限校验（`MaxReportedHeat` 的推导），漂移会让
// 服务端按一个客户端根本产生不出的上界去放宽校验。
func TestHeatConstantsMatchCrossEnd(t *testing.T) {
	v := loadPureVectors(t)
	c := v.Heat.Consts
	pairs := []struct {
		name     string
		got, exp int
	}{
		{"ACTIVE_SLOTS", ActiveSlots, c.ActiveSlots},
		{"PASSIVE_SLOT", PassiveSlot, c.PassiveSlot},
		{"TOTAL_SLOTS", TotalSlots, c.TotalSlots},
		{"HEAT_MAX", int(HeatMax), int(mustInt64(t, c.HeatMax))},
		{"OVERHEAT_DURATION_MS", OverheatDurationMs, c.OverheatDurationMs},
		{"HEAT_DECAY_PER_SEC", int(HeatDecayPerSec), int(mustInt64(t, c.HeatDecayPerSec))},
		{"DISCARD_PER_WAVE", DiscardPerWave, c.DiscardPerWave},
		{"DISCARD_HEAT_REFUND", int(DiscardHeatRefund), int(mustInt64(t, c.DiscardHeatRefund))},
	}
	for _, p := range pairs {
		if p.got != p.exp {
			t.Errorf("%s = %d，TS 期望 %d", p.name, p.got, p.exp)
		}
	}
}

// TestHeatScriptsMatchCrossEnd 重放热量脚本并**逐步**比对状态。
//
// # 判据为什么是逐步比对而不是只比最终状态
//
// 只比最终状态的话，中间若干步算错了又绕回来的实现能全绿。
// 逐步比对让「第 37 帧的衰减余数不对」这种偏移立刻暴露 ——
// 而热量偏移一点点 → 过热时机差 → 事件流差 → 哈希差。
func TestHeatScriptsMatchCrossEnd(t *testing.T) {
	v := loadPureVectors(t)
	for _, script := range v.Heat.Rows {
		t.Run(script.Label, func(t *testing.T) {
			h := NewHeatSystem()
			h.CapBonus = mustInt64(t, script.CapBonus)

			for i, step := range script.Trace {
				switch step.Op {
				case "setHeat":
					h.Heat = mustInt64(t, step.Heat)
				case "cast":
					ok := h.TryCast(needInt64(t, step.Cost, "cost"))
					if !matchBool(step.Result, ok) {
						t.Fatalf("第 %d 步 cast(%s) = %v，TS 期望 %s",
							i, *step.Cost, ok, showBool(step.Result))
					}
				case "refund":
					h.RefundHeat(needInt64(t, step.N, "n"))
				case "checkOverheat":
					ok := h.CheckOverheat()
					if !matchBool(step.Result, ok) {
						t.Fatalf("第 %d 步 checkOverheat = %v，TS 期望 %s", i, ok, showBool(step.Result))
					}
				case "forceOverheat":
					ok := h.ForceOverheat()
					if !matchBool(step.Result, ok) {
						t.Fatalf("第 %d 步 forceOverheat = %v，TS 期望 %s", i, ok, showBool(step.Result))
					}
				case "absorbOverheat":
					h.AbsorbOverheat()
				case "isStuckFor":
					costs := make([]int64, 0, len(step.Costs))
					for _, c := range step.Costs {
						costs = append(costs, mustInt64(t, c))
					}
					ok := h.IsStuckFor(costs)
					if !matchBool(step.Result, ok) {
						t.Fatalf("第 %d 步 isStuckFor(%v) = %v，TS 期望 %s",
							i, costs, ok, showBool(step.Result))
					}
				case "update":
					if step.DtMs == nil {
						t.Fatalf("第 %d 步 update 缺少 dt_ms", i)
					}
					// ⚠️ 传**原始小数**，不要在测试里先取整。
					//
					//   我第一版写了 `h.Update(int(math.Round(*step.DtMs)))`，
					//   结果变异验证把实现里的取整换成截断时**测试没红** ——
					//   因为取整已经发生在测试里，实现拿到的永远是整数，
					//   它内部取整还是截断都一样。
					//
					//   取整规则必须留在被测代码内部（Update 的签名收 float64
					//   就是为此），测试只负责把输入原样递进去。
					h.Update(*step.DtMs)
				default:
					// ⚠️ 显式失败而不是跳过：跳过会让向量里的新 op
					// 被无声忽略，而这条会继续显示为「通过」。
					t.Fatalf("向量里有本文件未实现的 op %q（第 %d 步）", step.Op, i)
				}

				// 逐步比对状态
				if got := strconv.FormatInt(h.Heat, 10); got != step.Heat {
					t.Fatalf("第 %d 步（%s）后 heat = %s，TS 期望 %s", i, step.Op, got, step.Heat)
				}
				if h.Overheated != step.Overheated {
					t.Fatalf("第 %d 步（%s）后 overheated = %v，TS 期望 %v",
						i, step.Op, h.Overheated, step.Overheated)
				}
				if h.OverheatRemaining != step.OverheatRemaining {
					t.Fatalf("第 %d 步（%s）后 overheatRemaining = %d，TS 期望 %d",
						i, step.Op, h.OverheatRemaining, step.OverheatRemaining)
				}
				if got := strconv.FormatInt(h.Cap(), 10); got != step.Cap {
					t.Fatalf("第 %d 步（%s）后 cap = %s，TS 期望 %s", i, step.Op, got, step.Cap)
				}
				if got := strconv.FormatInt(h.MaxHeatThisBattle, 10); got != step.MaxHeat {
					t.Fatalf("第 %d 步（%s）后 maxHeat = %s，TS 期望 %s", i, step.Op, got, step.MaxHeat)
				}
				// 衰减余数：单步就能分辨「四舍五入」与「截断」
				if got := strconv.FormatInt(h.DecayRemainder, 10); got != step.DecayRemainder {
					t.Fatalf("第 %d 步（%s）后 decayRemainder = %s，TS 期望 %s —— "+
						"这里通常意味着 dtMs 的取整方式不一致（JS Math.round vs 直接截断）",
						i, step.Op, got, step.DecayRemainder)
				}
			}
		})
	}
}

// needInt64 读一个必需的字符串字段。
func needInt64(t *testing.T, p *string, what string) int64 {
	t.Helper()
	if p == nil {
		t.Fatalf("向量缺少 %s 字段", what)
	}
	return mustInt64(t, *p)
}

func matchBool(want *bool, got bool) bool {
	return want != nil && *want == got
}

func showBool(b *bool) string {
	if b == nil {
		return "<nil>"
	}
	return strconv.FormatBool(*b)
}

// --- 手牌 ---

func TestDeckScriptsMatchCrossEnd(t *testing.T) {
	v := loadPureVectors(t)
	for _, script := range v.Deck.Rows {
		t.Run(script.Label, func(t *testing.T) {
			if len(script.Trace) == 0 {
				t.Fatal("手牌脚本的轨迹为空 —— 生成器坏了")
			}
			// 第 0 步是 setHand，从它的 hand 里读初始手牌
			d := NewCardDeck()
			for _, id := range script.Trace[0].Hand {
				d.Hand = append(d.Hand, Card{ID: id})
			}
			for i, step := range script.Trace {
				if i == 0 {
					continue // setHand
				}
				switch step.Op {
				case "newWave":
					d.NewWave()
				case "discard":
					got := d.Discard(step.ID)
					if !matchBool(step.OK, got.OK) {
						t.Fatalf("第 %d 步 discard(%q).ok = %v，TS 期望 %s",
							i, step.ID, got.OK, showBool(step.OK))
					}
					if step.Refund == nil || mustInt64(t, *step.Refund) != got.Refund {
						want := "<nil>"
						if step.Refund != nil {
							want = *step.Refund
						}
						t.Fatalf("第 %d 步 discard(%q).refund = %d，TS 期望 %s",
							i, step.ID, got.Refund, want)
					}
				case "drop":
					got := d.Drop(step.ID)
					if !matchBool(step.OK, got) {
						t.Fatalf("第 %d 步 drop(%q) = %v，TS 期望 %s",
							i, step.ID, got, showBool(step.OK))
					}
				default:
					t.Fatalf("向量里有本文件未实现的 op %q（第 %d 步）", step.Op, i)
				}
				// 逐步比对手牌与次数 —— 只比最终值会让中途绕回来的实现全绿
				if got := strings.Join(cardIDs(d.Hand), ","); got != strings.Join(step.Hand, ",") {
					t.Fatalf("第 %d 步后手牌 = [%s]，TS 期望 [%s]", i, got, strings.Join(step.Hand, ","))
				}
				if d.Size() != step.Size {
					t.Fatalf("第 %d 步 size = %d，TS 期望 %d", i, d.Size(), step.Size)
				}
				if d.DiscardsLeft != step.DiscardsLeft {
					t.Fatalf("第 %d 步 discardsLeft = %d，TS 期望 %d",
						i, d.DiscardsLeft, step.DiscardsLeft)
				}
			}
		})
	}
}

func cardIDs(cards []Card) []string {
	out := make([]string, 0, len(cards))
	for _, c := range cards {
		out = append(out, c.ID)
	}
	return out
}

func TestSnapshotDigestMatchesCrossEnd(t *testing.T) {
	v := loadPureVectors(t)
	for _, r := range v.Digest.Rows {
		got := SnapshotDigest(r.Works, r.Skills)
		if got != r.Digest {
			t.Errorf("%s: SnapshotDigest = %s，TS 期望 %s", r.Label, got, r.Digest)
		}
	}
}

// TestSnapshotDigestIgnoresInputOrder 守住「排序」这条语义。
//
// ⚠️ 这一条是契约向量证明不了的：向量里 `unsorted_skills` 与
//
//	`sorted_skills_same_set` 的期望值**相同**，但那只是因为它们
//	本来就长一样 —— 一个「不排序」的实现在别的用例上也可能碰巧全绿。
//
// 真正要守的是**性质**：同一集合的不同排列必须给出同一摘要。
// 否则「玩家改一次装备顺序就算换了一份构筑」这个缺陷会静默存在。
func TestSnapshotDigestIgnoresInputOrder(t *testing.T) {
	pairs := [][2][]int64{
		{{3, 1, 2}, {1, 2, 3}},
		{{100, 9, 10, 2}, {2, 9, 10, 100}},
		{{5}, {5}},
	}
	for _, p := range pairs {
		if a, b := SnapshotDigest(nil, p[0]), SnapshotDigest(nil, p[1]); a != b {
			t.Errorf("技能 %v 与 %v 应给出同一摘要，实际 %s vs %s", p[0], p[1], a, b)
		}
	}
	worksPairs := [][2][]string{
		{{"tesla_grid", "slow_belt", "block_wall"}, {"block_wall", "slow_belt", "tesla_grid"}},
	}
	for _, p := range worksPairs {
		if a, b := SnapshotDigest(p[0], nil), SnapshotDigest(p[1], nil); a != b {
			t.Errorf("装置 %v 与 %v 应给出同一摘要，实际 %s vs %s", p[0], p[1], a, b)
		}
	}
	// 但**内容不同**必须给出不同摘要 —— 否则摘要根本没用
	if SnapshotDigest([]string{"slow_belt"}, nil) == SnapshotDigest([]string{"block_wall"}, nil) {
		t.Error("不同装置必须给出不同摘要")
	}
}

// TestFnvMatchesCrossEnd 断言 FNV-1a 本体两端一致。
//
// 摘要的地基。若这里漂移，上面所有摘要用例会一起红但看不出原因，
// 所以单列一条让失败信息指向真正的源头。
func TestFnvMatchesCrossEnd(t *testing.T) {
	v := loadPureVectors(t)
	for _, r := range v.Fnv.Rows {
		if got := hex16(fnv1a64(r.Input)); got != r.Hex {
			t.Errorf("fnv1a64(%q) = %s，TS 期望 %s", r.Input, got, r.Hex)
		}
	}
}

// TestPureVectorFileIsNotTriviallySmall 守住契约文件本身的规模。
//
// 契约向量最坏的失效形态不是「红」，而是「绿但什么都没覆盖」——
// 比如生成器改了用例列表却忘了跑，而守卫只检查「跑出来的都一致」。
//
// 所以要求它至少覆盖到每个分组都有一个下限，
// 且**必须包含** int64 边界那条用例（那是本轮修正的判据）。
func TestPureVectorFileIsNotTriviallySmall(t *testing.T) {
	v := loadPureVectors(t)
	min := []struct {
		name string
		got  int
	}{
		{"skill", len(v.Skill.Rows)},
		{"works", len(v.Works.Rows)},
		{"challenge_seed", len(v.ChallengeSeed.Rows)},
		{"snapshot_validation", len(v.SnapshotValidation.Rows)},
		{"shield", len(v.Shield.Rows)},
		{"digest", len(v.Digest.Rows)},
		{"fnv", len(v.Fnv.Rows)},
	}
	for _, m := range min {
		if m.got < 3 {
			t.Errorf("分组 %q 只有 %d 条用例 —— 守卫的覆盖度失效了", m.name, m.got)
		}
	}
	// 每条拒绝理由都要有用例
	wantReasons := []string{
		"未获取到对方防线快照，无法模拟挑战",
		"对方快照里没有任何技能，无法模拟",
		"对方快照缺少哈希，无法确认挑战的是同一份构筑",
		"对方护盾尚未过期",
	}
	seen := map[string]bool{}
	for _, r := range v.SnapshotValidation.Rows {
		seen[r.Reason] = true
	}
	for _, want := range wantReasons {
		if !seen[want] {
			t.Errorf("契约向量缺少拒绝理由 %q 的用例 —— 少一条话术，两端不一致时无从发现", want)
		}
	}
	// int64 边界用例必须在（那是 SkillBaseDamageAtLevel 修正的判据）
	found := false
	for _, r := range v.Skill.Rows {
		if r.Base == strconv.FormatInt(math.MaxInt64, 10) {
			found = true
			if r.Level != 1 {
				t.Errorf("2^63-1 的用例出现在 level=%d —— 等级 ≥2 时结果超出 int64，两端不可能一致", r.Level)
			}
		}
	}
	if !found {
		t.Error("契约向量缺少 2^63-1 的用例 —— 那正是「等级 1 恒等快路径」的跨端判据")
	}
}
