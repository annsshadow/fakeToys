package domain

// 第 122 轮：把平衡缩放常量的「值」钉死。
//
// content_scale_test.go 的既有断言全是**自抵消**形式：
//
//	got.HP != 100 * EnemyHpScale
//
// 判据两边同用一个常量——把 EnemyHpScale 从 8 改成 16，
// `100*EnemyHpScale` 跟着变，测试**照样全绿**。
// 缩放系数是手感调参的结果（×8 命中率、×4 弹速，见 content.go 实测注释），
// 被静默改动后，星级门槛 / 地形阈值 / 平衡手感全静默漂移，
// 而 domain 包测试给不出任何信号（唯一的跨端锁是 cmd/vectors -check，
// 只在 CI 真跑时才生效）。
//
// 这里钉死**值**本身（常量 + 缩放后绝对值），改一个就红。

import "testing"

// 常量值本身。改动 = 平衡意图变更，必须显式改这里并同步跨端基线。
func TestBalanceScaleConstantsPinned(t *testing.T) {
	if EnemyHpScale != 8 {
		t.Errorf("EnemyHpScale 被改成 %d（基准 8）——血量缩放系数是实测调参结果，"+
			"改动会让全关卡难度曲线漂移，须同步 cmd/vectors 基线与客户端", EnemyHpScale)
	}
	if SkillProjectileScale != 4 {
		t.Errorf("SkillProjectileScale 被改成 %d（基准 4）——弹速缩放系数是命中率实测结果"+
			"（×4=76%% 命中），改动会同时改变手感与 I-6 重放哈希基线", SkillProjectileScale)
	}
}

// 缩放后**绝对值**（真正流进客户端/库里的数）。
// 即便有人不动常量、只改 ScaleEnemy/ScaleSkill 的算法，
// 这些绝对值锚点也会红。
func TestScaledContentAbsoluteValuesPinned(t *testing.T) {
	// 敌人 1（游荡者）种子 HP=100 → ×8 = 800
	e := ScaleEnemy(SeedEnemies[0])
	if e.ID != 1 {
		t.Fatalf("种子敌人顺序前提被破坏（第 0 项不再是 id=1）")
	}
	if e.HP != 800 {
		t.Errorf("游荡者缩放后 HP 应为 800（100×8），实际 %d", e.HP)
	}
	// 技能 1（ember）种子 ProjectileSpeed=60000 → ×4 = 240000
	s := ScaleSkill(SeedSkills[0])
	if s.ID != 1 {
		t.Fatalf("种子技能顺序前提被破坏（第 0 项不再是 id=1）")
	}
	if s.ProjectileSpeed != 240000 {
		t.Errorf("ember 缩放后弹速应为 240000（60000×4），实际 %d", s.ProjectileSpeed)
	}
}
