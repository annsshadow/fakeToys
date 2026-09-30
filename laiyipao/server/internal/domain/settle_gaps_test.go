package domain

// 结算补漏（第二轮）：computeLoot 的负数输入、诊断的兜底分支、
// 集合校验的总量与取值下界。
//
// 这三块共同的性质：都是「合法客户端永远发不出来的输入」。
// 但 computeLoot 是导出 API（不能指望调用方都校验），
// 诊断的兜底分支决定"归因不出来时系统说什么"，
// 集合校验的两个分支挡的是负计数与非法卡牌索引落库。

import (
	"math"
	"testing"
)

// TestComputeLootNegativeInputs 负数输入（溢出回绕的典型产物、
// 也是旧漏洞的利用形态）必须被夹到 0，最终金币落在 [0, cap]。
// 与 TestComputeLootNeverNegative 互补：那边扫大正数，这边扫负数。
func TestComputeLootNegativeInputs(t *testing.T) {
	gl := testLevel()
	caps := DefaultLootCaps()
	for _, reactions := range []int64{-1, -1000, math.MinInt64} {
		for _, kills := range []int64{-1, math.MinInt64} {
			for _, stars := range []int64{-1, math.MinInt64} {
				loot := computeLoot(gl, stars, kills, maxKillsOf(gl), reactions)
				coin := loot["coin"]
				if coin < 0 || coin > caps.MaxCoinPerBattle {
					t.Fatalf("stars=%d kills=%d reactions=%d → coin=%d 越界 [%d, %d]",
						stars, kills, reactions, coin, 0, caps.MaxCoinPerBattle)
				}
			}
		}
	}
}

func maxKillsOf(gl GeneratedLevel) int64 { return int64(MaxKillsFor(gl)) }

// TestComputeLootNegativeStarsNoKeys 负星级不得发放三星钥匙。
func TestComputeLootNegativeStarsNoKeys(t *testing.T) {
	gl := testLevel()
	if _, ok := computeLoot(gl, -3, 10, maxKillsOf(gl), 0)["keys"]; ok {
		t.Error("负星级不应发放 keys")
	}
}

// TestDiagnoseFailureUnclear 兜底分支：样本不足以归因时必须诚实地说
// "看不出来"，而不是硬套某个结论——宁可低置信度，也不要误导性的建议。
// 构造：元素用得不少（不触发 no_element / low_reaction）、
// 装载 3 种以上（不触发 narrow_element）、推进超过 30%（不触发 low_power）。
func TestDiagnoseFailureUnclear(t *testing.T) {
	gl := GenerateLevel(30)
	d := DiagnoseFailure(DiagnoseInput{
		LevelID:      30,
		FailedTimes:  1, // 只失败一次，reactionsLow 需要 >= 2
		Kills:        60,
		TotalEnemies: gl.TotalEnemies(),
		ElementsUsed: map[string]int{"fire": 100, "ice": 100, "lightning": 100},
		Loadout:      []Element{ElementFire, ElementIce, ElementLightning},
	}, gl)
	if d.Stage != "unclear" {
		t.Fatalf("应诊断为 unclear，实际 %s（%s）", d.Stage, d.Title)
	}
	if d.Confidence >= 60 {
		t.Errorf("兜底结论的置信度应低于默认 60，实际 %d", d.Confidence)
	}
}

// TestValidateReportCollectionsRejectsOversizedElementsUsed
// elements_used 的键数上界：合法元素只有 5 个，超过即报文被伪造
// （多余的键会进数据库与 GROUP BY）。
func TestValidateReportCollectionsRejectsOversizedElementsUsed(t *testing.T) {
	gl := testLevel()
	in := baseInput(gl)
	in.ElementsUsed = map[string]int{
		"fire": 1, "ice": 1, "lightning": 1, "corrosion": 1, "kinetic": 1,
		"shadow": 1, // 第 6 个键：伪造元素
	}
	err := validateReportCollections(gl, in)
	if err == nil {
		t.Fatal("6 个键应被拒绝（合法元素只有 5 个）")
	}
}

// TestValidateReportCollectionsRejectsNegativeCardPick
// card_picks 的取值下界：-1 是合法的「整波跳过」，-2 不是。
// 负索引会被客户端引擎按「跳过」处理，但落到分析库里就是脏数据。
func TestValidateReportCollectionsRejectsNegativeCardPick(t *testing.T) {
	gl := testLevel()

	t.Run("合法基线", func(t *testing.T) {
		in := baseInput(gl)
		in.CardPicks = []int{-1, 0, 2}
		if err := validateReportCollections(gl, in); err != nil {
			t.Fatalf("-1 与非负索引应被接受，实际 %v", err)
		}
	})

	t.Run("负索引", func(t *testing.T) {
		in := baseInput(gl)
		in.CardPicks = []int{-1, -2}
		if err := validateReportCollections(gl, in); err == nil {
			t.Fatal("card_picks 含 -2 应被拒绝")
		}
	})
}
