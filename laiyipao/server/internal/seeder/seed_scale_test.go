package seeder

// 第 128 轮：种子库必须落**投放用的缩放值**。
//
// 客户端实际收到的数值来自 `LoadGameConfig` →
// `domain.ScaleAllEnemies()`（血量 ×8）/ `domain.ScaleAllSkills()`（弹速 ×4），
// 全部在内存里生成、**一次库都不查**。
//
// 而运营/后台读 `enemies` / `skills` 表（AdminListSkills、分析脚本）
// 拿到的是库里那一份。修前 seeder 写的是 `domain.SeedEnemies` / `SeedSkills`
// 的**基准值**（未缩放），于是：
//
//	enemies.hp    后台看到 100，玩家游戏里实际是 800（×8）
//	skills.speed  后台看到 15000，玩家游戏里实际是 60000（×4）
//
// 后台据此做平衡判断（改数值、看分布）会整体错一个倍率。
// 本测试把「库值 == 投放值」这个不变量钉死，防止 seeder 退回写基准值。

import (
	"context"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

func TestSeededRowsMatchDeliveryScale(t *testing.T) {
	pool, _ := openScratch(t)
	ctx := context.Background()
	if _, err := Run(ctx, pool); err != nil {
		t.Fatalf("Run 失败：%v", err)
	}

	// 敌人：hp / shield_hp 必须等于缩放后值
	scaledEnemies := domain.ScaleAllEnemies()
	for _, want := range scaledEnemies {
		var hp, shield int64
		if err := pool.QueryRow(ctx,
			`SELECT hp, shield_hp FROM enemies WHERE id = $1`, want.ID).Scan(&hp, &shield); err != nil {
			t.Fatalf("读 enemy %d 失败：%v", want.ID, err)
		}
		if hp != want.HP || shield != want.ShieldHP {
			t.Errorf("enemy %d 库值 hp=%d shield=%d ≠ 投放值 hp=%d shield=%d（seeder 写了未缩放基准值？）",
				want.ID, hp, shield, want.HP, want.ShieldHP)
		}
	}

	// 技能：projectile_speed 必须等于缩放后值（基础 + 复合）
	scaled := append(domain.ScaleAllSkills(), domain.ScaleAllCompositeSkills()...)
	for _, want := range scaled {
		var speed int64
		if err := pool.QueryRow(ctx,
			`SELECT projectile_speed FROM skills WHERE id = $1`, want.ID).Scan(&speed); err != nil {
			t.Fatalf("读 skill %d 失败：%v", want.ID, err)
		}
		if speed != want.ProjectileSpeed {
			t.Errorf("skill %d 库值 projectile_speed=%d ≠ 投放值 %d（seeder 写了未缩放基准值？）",
				want.ID, speed, want.ProjectileSpeed)
		}
	}

	// 倍率自检：确认缩放确实改变了数值（防止 ScaleXxx 被改成恒等而全测试静默绿）
	if domain.EnemyHpScale <= 1 || domain.SkillProjectileScale <= 1 {
		t.Fatalf("缩放倍率异常：enemy=%d skill=%d，本测试失去意义",
			domain.EnemyHpScale, domain.SkillProjectileScale)
	}
}
