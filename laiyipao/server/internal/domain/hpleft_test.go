package domain

// 守卫：`hp_left` 的上界。
//
// ## 之前为什么只有下界
//
// `ValidateSettle` 里原本只有 `if in.HPLeft < 0 { … }`，
// 于是 `hp_left` 可以上报任意大的值。
//
// ## 诚实标注：这不是刷分漏洞
//
// `hp_left` 只写进 `battle_records`，**不参与任何奖励计算** ——
// 星星由分数决定，掉落由分数决定，胜负由 `kills`/`wave_reached` 决定。
//
// 它是**数据完整性**问题：
// 一条写着"血量 999999"的战报让整个战报库失去分析价值，
// 而 I-6 的叙事是"验真证明这局确实是这样打的" ——
// 血量本身就是假的，那这个叙事站不住。
//
// 补它只需要 3 行，但它守住的是"战报可信"这个前提。

import (
	"fmt"
	"strings"
	"testing"
)

func TestHPLeftBoundedByLevelBaseHP(t *testing.T) {
	// ⚠️ 收集前若干个 offender 再报，而不是每关都 Errorf。
	//
	// 第一版逐关 `t.Errorf`，结果一个变异会打出 **100 行**错误 ——
	// 逐行点名看着"很彻底"，实际把关键信息（哪一关、上界取多少）
	// 埋在 100 条几乎相同的输出里。
	// 与「变异失败信息要点名受害对象」相反：点名了 100 个等于没点名。
	var bad []string
	record := func(format string, args ...any) {
		if len(bad) < 5 {
			bad = append(bad, fmt.Sprintf(format, args...))
		}
	}

	for id := 1; id <= 100; id++ {
		gl := GenerateLevel(id)

		// 恰好等于初始血量：合法（一点没漏）
		if err := validateHPLeft(gl, int(gl.BaseHP)); err != nil {
			record("第 %d 关：hp_left == base_hp（%d）应被接受，却报 %v", id, gl.BaseHP, err)
		}
		// 超过 1 点：拒绝
		if err := validateHPLeft(gl, int(gl.BaseHP)+1); err == nil {
			record("第 %d 关：hp_left = base_hp+1（%d）应被拒绝", id, gl.BaseHP+1)
		}
		// 远超初值（作弊上报）：拒绝
		if huge := int(gl.BaseHP)*100 + 99999; validateHPLeft(gl, huge) == nil {
			record("第 %d 关：hp_left = %d 远超初值 %d，应被拒绝", id, huge, gl.BaseHP)
		}
		// 负数：下界（原有行为，本次未改动）
		if err := validateHPLeft(gl, -1); err == nil {
			record("第 %d 关：hp_left = -1 应被拒绝", id)
		}
	}
	if len(bad) > 0 {
		t.Fatalf("hp_left 边界校验不成立（共 %d 处问题，前 5 条）：\n%s",
			len(bad), strings.Join(bad, "\n"))
	}
}

// TestHPLeftUpperBoundIsTight 确认上界不是随手取的大数。
//
// 若有人把它改成 `BaseHP * 2` 或某个魔数，这里会红。
// 判据是「满配零漏怪的合法对局必须被接受」——
// 引擎里 baseHp 只减不增，所以 `BaseHP` 就是可达的最大值。
func TestHPLeftUpperBoundIsTight(t *testing.T) {
	var bad []string
	for id := 1; id <= 100; id++ {
		gl := GenerateLevel(id)
		if gl.BaseHP <= 0 {
			t.Fatalf("第 %d 关的 base_hp = %d，生成器有问题", id, gl.BaseHP)
		}
		if validateHPLeft(gl, int(gl.BaseHP)) != nil {
			bad = append(bad, fmt.Sprintf("第 %d 关：初值 %d 被拒，上界取得比初值还小", id, gl.BaseHP))
		}
		if validateHPLeft(gl, int(gl.BaseHP)+1) == nil {
			bad = append(bad, fmt.Sprintf("第 %d 关：初值+1 被接受，上界取得比初值还大", id))
		}
		if len(bad) >= 5 {
			break
		}
	}
	if len(bad) > 0 {
		t.Fatalf("hp_left 上界不紧：\n%s", strings.Join(bad, "\n"))
	}
}
