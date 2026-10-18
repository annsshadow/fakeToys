package service

// admin 改技能配置**不会**让整局重算失配（2026-10-15）。
//
// # 这条测试是为了把一个**错误的担心**变成可执行的事实
//
// 接线时我提出过这样一个风险：
// 「admin 有 `PUT /admin/skills/:id` 能改技能数值，客户端从 `/api/v1/config`
// 拿数据且有 HTTP 缓存；admin 在一局战斗进行中改技能，客户端可能用旧值、
// 服务端用新值，重算就会失配 —— 于是误拒诚实玩家。」
//
// **那个担心是错的。** 三条实测证据：
//
//  1. `AdminUpdateSkill`（stats.go）只 `UPDATE skills SET ...`，
//     写的是 **DB 行**，不碰 `domain.SeedSkills`（进程内静态表）。
//  2. `/api/v1/config` 出的是 `staticConfig()`，而它用
//     `domain.ScaleAllSkills()` / `ScaleAllEnemies()` —— **同一份静态表**。
//     所以 admin 改 DB **根本不会改变 `/config` 的输出**。
//  3. 战斗路径 `loadSkillsRows` 从 DB 只取
//     `(id, name, family, element, kind, level, slot)` ——
//     **不取任何战斗数值**（base_damage / heat_cost / cooldown 都不取）。
//     而重放里 `EquippedFromSnapshot` 只用快照的 `ID`/`Slot`/`Level`，
//     其余（Name/Element/Kind/BaseDamage/…）全部取自静态表的 `def`。
//
// 也就是说快照里的 `name`/`family`/`element`/`kind` 对重算是**装饰性的**，
// DB 与静态表在战斗路径上**根本没有交集**。
//
// # 为什么要写成测试而不是删掉那句提醒
//
// 「读代码看着像没问题」在本项目已经被证伪过多次。
// 与其留一条注释说「应该不会」，不如让这条测试在未来任何人
// 把某条战斗路径改成读 DB 时**立刻变红**。

import (
	"context"
	"testing"
)

// TestAdminSkillEditDoesNotAffectRecompute admin 改技能配置后，
// 用**真哈希**的结算仍必须通过。
//
// 它同时覆盖两类编辑：
//   - **战斗数值**（base_damage）—— 它压根没进 DB→战斗的路径，
//     改它对重算零影响；
//   - **装饰字段**（name / element）—— 它们进了 build_snapshot，
//     但重放不读，所以也零影响。
func TestAdminSkillEditDoesNotAffectRecompute(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	ts.Service.Cfg.SettleRecomputeMode = RecomputeEnforce
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})

	// ⚠️ 必须用**同一个 token** 算两次哈希。
	//
	// `StartBattle` 每局生成一个**新的随机种子**（newBattleSeed 走
	// `random()`），所以两次 realHashSettle 拿的是两局不同的战斗 ——
	// 哈希本来就该不同，拿它们比较只会得到一个恒真的失败。
	// 本测试第一版就是这么写的，红在「改后哈希变了」上，
	// 而真正的原因是我比错了对象。
	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	picks := loseSettleInput().CardPicks

	before := ts.computedHashFor(t, ctx, uid, 1, picks, tp)

	// admin 改技能：战斗数值 + 会进快照的字段（name/element）都动
	for _, patch := range []map[string]any{
		{"base_damage": 999999},
		{"heat_cost": 77},
		{"name": "ADMIN-RENAMED-SKILL"},
		{"element": "corrosion"},
	} {
		for id := 1; id <= 4; id++ {
			if _, err := ts.AdminUpdateSkill(ctx, id, patch); err != nil {
				t.Fatalf("admin 改技能 %d（%v）失败：%v", id, patch, err)
			}
		}
	}

	// 同一个 token、同一份选牌，只多了一次 admin 编辑
	after := ts.computedHashFor(t, ctx, uid, 1, picks, tp)

	if before != after {
		t.Errorf("admin 改技能后重算哈希变了：\n  改前 %q\n  改后 %q\n"+
			"DB 里的技能配置**不该**进战斗路径 —— 若本用例变红，"+
			"说明有人把某条战斗路径改成读 DB 了，两端会因此失配。",
			before, after)
	}

	// 带这个哈希的结算必须仍然通过（不该误拒诚实玩家）
	in := loseSettleInput()
	in.ReplayHash = after
	in.CardPicks = picks
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, in); err != nil {
		t.Errorf("admin 改技能后的结算应照常通过：%v", err)
	}
}

// TestAdminSkillEditDoesNotLeakIntoConfig 与上一条互补：
// 它断言 admin 改 DB **不会**改变 `/api/v1/config` 的输出。
//
// 上一条证的是「战斗路径不读 DB」，这一条证的是「客户端拿到的也不是 DB」——
// 两端读的是同一份进程内静态表，所以**没有可漂移的缝隙**。
func TestAdminSkillEditDoesNotLeakIntoConfig(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	before := ts.LoadGameConfig()
	skillBefore := before.Skills[0]
	if skillBefore.ID == 0 {
		t.Skip("config 里没有技能，取不到基线")
	}

	for id := 1; id <= 4; id++ {
		if _, err := ts.AdminUpdateSkill(ctx, id, map[string]any{
			"base_damage": 888888,
			"name":        "CONFIG-LEAK-PROBE",
		}); err != nil {
			t.Fatalf("admin 改技能 %d 失败：%v", id, err)
		}
	}

	after := ts.LoadGameConfig()
	if len(after.Skills) != len(before.Skills) {
		t.Fatalf("config 技能数变了：%d -> %d", len(before.Skills), len(after.Skills))
	}
	skillAfter := after.Skills[0]
	if skillAfter.BaseDamage != skillBefore.BaseDamage {
		t.Errorf("admin 改 DB 后 /config 的 base_damage 变了：%d -> %d\n"+
			"这说明 config 改成读 DB 了 —— 那么 admin 改动会让"+
			"「已缓存旧 config 的客户端」与服务端失配。",
			skillBefore.BaseDamage, skillAfter.BaseDamage)
	}
	if skillAfter.Name == "CONFIG-LEAK-PROBE" {
		t.Error("admin 改的名字漏进了 /config —— 同上")
	}
}
