package domain

// 内容缩放（content.go）与装备查询的契约测试。
//
// Scale* 系列是「落库前的数值缩放」唯一入口：seeder、/config 下发、
// cmd/vectors 契约导出全部走它们。EnemyHpScale=8、SkillProjectileScale=4
// 这两个系数是手感调参的结果（见 content.go 的命中率实测注释），
// 这里钉住「缩放必须真的被应用、且逐项应用」——
// 若有人把 ScaleSkill 改成只缩放基础技能而漏掉合成技能，
// 客户端弹丸与服务端校验立即失配，I-6 回放哈希全线失败。

import "testing"

func TestScaleEnemy(t *testing.T) {
	in := SeedEnemy{ID: 1, HP: 100, ShieldHP: 20}
	got := ScaleEnemy(in)
	if got.HP != 100*EnemyHpScale {
		t.Errorf("HP 缩放后应为 %d，实际 %d", 100*EnemyHpScale, got.HP)
	}
	if got.ShieldHP != 20*EnemyHpScale {
		t.Errorf("ShieldHP 缩放后应为 %d，实际 %d", 20*EnemyHpScale, got.ShieldHP)
	}
	if got.ID != in.ID {
		t.Error("缩放不得改写身份字段")
	}
}

func TestScaleSkill(t *testing.T) {
	in := SeedSkill{ID: 1, ProjectileSpeed: 60}
	got := ScaleSkill(in)
	if got.ProjectileSpeed != 60*SkillProjectileScale {
		t.Errorf("ProjectileSpeed 缩放后应为 %d，实际 %d", 60*SkillProjectileScale, got.ProjectileSpeed)
	}
	if got.ID != in.ID {
		t.Error("缩放不得改写身份字段")
	}
}

func TestScaleAllEnemiesCoversEveryEnemy(t *testing.T) {
	all := ScaleAllEnemies()
	if len(all) != len(SeedEnemies) {
		t.Fatalf("缩放输出数量应与原始定义一致：%d != %d", len(all), len(SeedEnemies))
	}
	for i, e := range all {
		if e.HP != SeedEnemies[i].HP*EnemyHpScale || e.ShieldHP != SeedEnemies[i].ShieldHP*EnemyHpScale {
			t.Errorf("敌人 %d（%s）的缩放不正确：hp %d -> %d", e.ID, e.Code, SeedEnemies[i].HP, e.HP)
		}
	}
}

func TestScaleAllSkillsCoversBothSkillSets(t *testing.T) {
	t.Run("基础技能", func(t *testing.T) { assertScaled(t, SeedSkills, ScaleAllSkills()) })
	t.Run("合成技能", func(t *testing.T) { assertScaled(t, SeedCompositeSkills, ScaleAllCompositeSkills()) })
}

func assertScaled(t *testing.T, src, got []SeedSkill) {
	t.Helper()
	if len(got) != len(src) {
		t.Fatalf("缩放输出数量应与原始定义一致：%d != %d", len(got), len(src))
	}
	for i, s := range got {
		if s.ProjectileSpeed != src[i].ProjectileSpeed*SkillProjectileScale {
			t.Errorf("第 %d 项（%s）弹速缩放不正确：%d -> %d", i, s.Code, src[i].ProjectileSpeed, s.ProjectileSpeed)
		}
	}
}

func TestEquipmentByID(t *testing.T) {
	if len(SeedEquipmentList) == 0 {
		t.Fatal("装备种子表为空，测试前提不成立")
	}
	// 命中：按 id 查回的定义必须与种子表一致
	first := SeedEquipmentList[0]
	got, ok := EquipmentByID(int64(first.ID))
	if !ok {
		t.Fatalf("装备 %d 应能查到", first.ID)
	}
	if got.ID != first.ID || got.Code != first.Code {
		t.Errorf("查回的装备与种子表不符：%+v != %+v", got, first)
	}
	// 未命中：返回 false 而不是零值+true
	if _, ok := EquipmentByID(-999); ok {
		t.Error("不存在的 id 应返回 false")
	}
}
