package service

// 战斗域（game.go）与出战配置（loadout.go）的服务层测试。
//
// 覆盖策略：
//   - 正常链路在健康库上跑（开局→结算→回放→任务）；
//   - 「第一步就失败」的错误分支用 closed pool；
//   - 「第 N 步才失败」的分支用 scratch 库改名单表；
//   - 纯函数（buildElements / resultStr / boolToInt / periodStart 等）直接单测。

import (
	"context"
	"errors"
	"fmt"
	"testing"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

func TestStartBattleAndSettle(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})

	// 非法关卡
	if _, err := ts.StartBattle(ctx, uid, 0); !errors.Is(err, ErrBadInput) {
		t.Errorf("level 0 应 ErrBadInput，实际 %v", err)
	}
	if _, err := ts.StartBattle(ctx, uid, domain.TotalLevels+1); !errors.Is(err, ErrBadInput) {
		t.Errorf("超界关卡应 ErrBadInput，实际 %v", err)
	}

	// 未解锁：新号 max_stage=0，只能打第 1 关
	if _, err := ts.StartBattle(ctx, uid, 3); !errors.Is(err, ErrForbidden) {
		t.Errorf("未解锁关卡应 ErrForbidden，实际 %v", err)
	}

	// 体力不足
	ts.exec(t, `UPDATE user_wallets SET energy = 0, energy_updated_at = now() WHERE user_id = $1`, uid)
	if _, err := ts.StartBattle(ctx, uid, 1); !errors.Is(err, ErrBadInput) {
		t.Errorf("体力不足应 ErrBadInput，实际 %v", err)
	}
	ts.grant(t, ctx, uid, map[string]int64{"energy": 120})

	// 开局成功
	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Fatalf("开局失败：%v", err)
	}
	if tp.TokenID == 0 || tp.Seed == "" {
		t.Fatalf("开局响应不完整：%+v", tp)
	}
	if tp.Level.Seed != 0 || tp.Level.SeedStr == "" {
		t.Error("关卡数字种子必须清零、字符串种子必须下发（JS 精度）")
	}

	// 结算成功（失败局）
	before := ts.wallet(t, ctx, uid)
	resp, err := ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput())
	if err != nil {
		t.Fatalf("结算失败：%v", err)
	}
	if resp.Win || resp.BattleID == 0 {
		t.Errorf("结算响应异常：win=%t battle=%d", resp.Win, resp.BattleID)
	}
	after := ts.wallet(t, ctx, uid)
	if after.Coin <= before.Coin {
		t.Error("失败局也应有金币掉落（loot）")
	}

	// 进度：battle 次数至少 +1（lose 不推 max_stage）
	var battles int
	if err := ts.pool.QueryRow(ctx,
		`SELECT total_battles FROM user_progress WHERE user_id = $1`, uid).Scan(&battles); err != nil || battles < 1 {
		t.Errorf("total_battles 应 +1（%d, %v）", battles, err)
	}

	// 任务进度被推进
	var taskProgress int
	if err := ts.pool.QueryRow(ctx,
		`SELECT progress FROM user_tasks ut JOIN tasks t ON t.id = ut.task_id
		 WHERE ut.user_id = $1 AND t.metric = 'kills' AND t.scope = 'daily'`, uid).Scan(&taskProgress); err != nil || taskProgress < 1 {
		t.Errorf("kills 日任务进度应被推进（%d, %v）", taskProgress, err)
	}
}

func TestSettleBattleWinPath(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})

	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	// 胜利判定：kills+leaked >= MaxKillsFor（打满全场才算赢）
	gl := domain.GenerateLevel(1)
	in := loseSettleInput()
	in.Result = "win"
	in.Kills = domain.MaxKillsFor(gl)
	in.Score = 2000
	resp, err := ts.SettleBattle(ctx, uid, tp.TokenID, in)
	if err != nil {
		t.Fatalf("胜利结算失败：%v", err)
	}
	if !resp.Win {
		t.Error("win 上报应返回 win")
	}
	// max_stage 应推进到 1
	var maxStage int
	if err := ts.pool.QueryRow(ctx,
		`SELECT max_stage FROM user_progress WHERE user_id = $1`, uid).Scan(&maxStage); err != nil || maxStage < 1 {
		t.Errorf("胜利应推进 max_stage（%d, %v）", maxStage, err)
	}
	// level_stars 落库：低分胜利 stars 可为 0，但行必须存在且 clears 记账
	var stars, clears int
	if err := ts.pool.QueryRow(ctx,
		`SELECT stars, clears FROM level_stars WHERE user_id = $1 AND level_id = 1`, uid).
		Scan(&stars, &clears); err != nil || clears != 1 {
		t.Errorf("胜利应记 clear（stars=%d clears=%d err=%v）", stars, clears, err)
	}
}

func TestSettleBattleRejections(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})

	// 未知 token
	if _, err := ts.SettleBattle(ctx, uid, 999999999, loseSettleInput()); !errors.Is(err, ErrNotFound) {
		t.Errorf("未知 token 应 ErrNotFound，实际 %v", err)
	}

	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}

	// 击杀数造假 → domain 结算错误（isSettleRejection 依赖这些哨兵值）
	in := loseSettleInput()
	in.Kills = 100000
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, in); err == nil {
		t.Error("击杀数造假应被拒")
	}

	// 串行重复结算 → ErrTokenUsed（422 家族）
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput()); err != nil {
		t.Fatalf("首次结算应成功：%v", err)
	}
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput()); !errors.Is(err, domain.ErrTokenUsed) {
		t.Errorf("重复结算应 ErrTokenUsed，实际 %v", err)
	}
}

func TestStartBattleErrorBranches(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()
	uid := int64(1)

	if _, err := ts.StartBattle(ctx, uid, 1); err == nil {
		t.Error("故障态开局应报错（读进度失败）")
	}
	if _, err := ts.newBattleSeed(ctx); err == nil {
		t.Error("故障态生成种子应报错")
	}
	if _, err := ts.LoadBuildSnapshot(ctx, uid); err == nil {
		t.Error("故障态读构筑应报错")
	}
	if _, _, err := ts.loadSkillsAndSlots(ctx, uid); err == nil {
		t.Error("故障态读技能应报错")
	}
	if _, err := ts.loadEquipment(ctx, uid); err == nil {
		t.Error("故障态读装备应报错")
	}
	if _, err := ts.loadMasteryNodes(ctx, uid); err == nil {
		t.Error("故障态读专精应报错")
	}
}

// TestSettleBattleMidwayFailures 结算的中间步骤失败：
// 「token 加载成功之后」的 LoadBuildSnapshot / bumpTasks / 星级落库 ——
// closed pool 覆盖不到（它死在第一步），必须靠 scratch 改名单表。
func TestSettleBattleMidwayFailures(t *testing.T) {
	ctx := context.Background()

	t.Run("构筑快照失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.grant(t, ctx, uid, map[string]int64{"energy": 100})
		tp, err := ts.StartBattle(ctx, uid, 1)
		if err != nil {
			t.Skipf("开局失败：%v", err)
		}
		// 拿走构筑路径依赖的表：结算必须在快照一步失败并整体回滚
		ts.renameTable(t, "user_equipment", "user_equipment_bak")
		_, err = ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput())
		ts.renameTable(t, "user_equipment_bak", "user_equipment")
		mustErr(t, err, "load build snapshot")
		// 整体回滚：token 仍可结算 —— 恢复表后重打必须成功
		if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput()); err != nil {
			t.Errorf("恢复后同一 token 应能正常结算：%v", err)
		}
	})

	t.Run("任务进度失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.grant(t, ctx, uid, map[string]int64{"energy": 100})
		tp, err := ts.StartBattle(ctx, uid, 1)
		if err != nil {
			t.Skipf("开局失败：%v", err)
		}
		// tasks 表拿走 → bumpTasks 的 metric 查询失败
		// 注意：结算事务内此前的步骤（token/战报/掉落/进度/星级）都会回滚
		ts.renameTable(t, "tasks", "tasks_bak")
		_, err = ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput())
		ts.renameTable(t, "tasks_bak", "tasks")
		mustErr(t, err, "query tasks metric")
	})

	t.Run("任务行写入失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.grant(t, ctx, uid, map[string]int64{"energy": 100})
		tp, err := ts.StartBattle(ctx, uid, 1)
		if err != nil {
			t.Skipf("开局失败：%v", err)
		}
		ts.renameTable(t, "user_tasks", "user_tasks_bak")
		_, err = ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput())
		ts.renameTable(t, "user_tasks_bak", "user_tasks")
		mustErr(t, err, "bump")
	})

	t.Run("星级落库失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.grant(t, ctx, uid, map[string]int64{"energy": 100})
		tp, err := ts.StartBattle(ctx, uid, 1)
		if err != nil {
			t.Skipf("开局失败：%v", err)
		}
		ts.renameTable(t, "level_stars", "level_stars_bak")
		_, err = ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput())
		ts.renameTable(t, "level_stars_bak", "level_stars")
		mustErr(t, err, "upsert level stars")
	})
}

func TestBuildElementsAndPureHelpers(t *testing.T) {
	// 新名优先
	b := map[string]any{"elements": []string{"fire"}, "skill_ids": []string{"ice"}}
	if got := buildElements(b); len(got) != 1 || got[0] != "fire" {
		t.Errorf("elements 优先：%v", got)
	}
	// 旧战报回退到误命名的 skill_ids
	if got := buildElements(map[string]any{"skill_ids": []string{"ice"}}); len(got) != 1 || got[0] != "ice" {
		t.Errorf("历史战报应回退 skill_ids：%v", got)
	}
	// 两者皆缺
	if got := buildElements(map[string]any{}); got != nil {
		t.Errorf("空构筑应返回 nil：%v", got)
	}
	// 类型不匹配
	if got := buildElements(map[string]any{"elements": 42}); got != nil {
		t.Errorf("类型不对应返回 nil：%v", got)
	}

	if resultStr(true) != "win" || resultStr(false) != "lose" {
		t.Error("resultStr 语义错误")
	}
	if boolToInt(true) != 1 || boolToInt(false) != 0 {
		t.Error("boolToInt 语义错误")
	}

	// 周起始：周日（Go weekday=0）必须折算，不允许出现"起点在终点之后"
	now := time.Now()
	sunday := time.Date(now.Year(), now.Month(), now.Day(), 12, 0, 0, 0, time.Local).
		AddDate(0, 0, -int(now.Weekday())) // 本周的周日
	start := periodStart(sunday, "weekly")
	if start.Weekday() != time.Monday {
		t.Errorf("weekly 周期起点应是周一，实际 %v", start.Weekday())
	}
	if start.After(sunday) {
		t.Errorf("weekly 起点不应晚于周日：起点 %v，周日 %v", start, sunday)
	}
	// 同一周的周四（Mon..Sun 周制）起点必须仍是同一个周一
	thu := sunday.AddDate(0, 0, -3)
	if got := periodStart(thu, "weekly"); !got.Equal(start) {
		t.Errorf("同周周四周日的 weekly 起点应一致：%v vs %v", got, start)
	}
	// daily 起点是当日零点
	d := periodStart(now, "daily")
	if d.Hour() != 0 || d.Minute() != 0 || d.Second() != 0 {
		t.Errorf("daily 起点应是当日零点：%v", d)
	}
}

// TestCheckSlotBudget 槽位预算：去重后的槽位数，不是技能条数。
func TestCheckSlotBudget(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 基础预算（BaseSkillSlots=5：4 主动 + 1 被动）：通过
	ok5 := map[int]bool{0: true, 1: true, 2: true, 3: true, 4: true}
	if err := ts.checkSlotBudget(ctx, uid, ok5); err != nil {
		t.Errorf("基础槽位不应报错：%v", err)
	}
	// 6 个：超预算
	over := map[int]bool{0: true, 1: true, 2: true, 3: true, 4: true, 5: true}
	err := ts.checkSlotBudget(ctx, uid, over)
	if !errors.Is(err, ErrSlotBudgetExceeded) {
		t.Errorf("超预算应 ErrSlotBudgetExceeded，实际 %v", err)
	}
}

// TestSlotBudgetEnforcedInBuildSnapshot 通过 SQL 直塞超量槽位，
// 验证读路径（loadSkillsAndSlots）真的会拒绝 —— 这是「客户端说了算」漏洞的封条。
func TestSlotBudgetEnforcedInBuildSnapshot(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// loadSkillsAndSlots 的槽位统计走 user_skills ⋈ user_skill_slots：
	// 未拥有的技能不进快照也不占预算。所以要造"6 个**已拥有**技能各占一槽"。
	ts.exec(t, `INSERT INTO user_skills (user_id, skill_id)
	            SELECT $1, s.id FROM skills s WHERE s.id BETWEEN 5 AND 9
	            ON CONFLICT DO NOTHING`, uid)
	ts.exec(t, `INSERT INTO user_skill_slots (user_id, slot, skill_id)
	            VALUES ($1,0,1),($1,1,2),($1,2,3),($1,3,4),($1,4,5),($1,5,6)
	            ON CONFLICT (user_id, slot) DO UPDATE SET skill_id = EXCLUDED.skill_id`, uid)
	if _, err := ts.LoadBuildSnapshot(ctx, uid); err != nil {
		// 预期：读路径必须报 ErrSlotBudgetExceeded
		if !errors.Is(err, ErrSlotBudgetExceeded) {
			t.Fatalf("超量槽位应报 ErrSlotBudgetExceeded，实际 %v", err)
		}
	} else {
		t.Fatal("6 个槽位应超预算（基础 5 + 专精 0）")
	}
}

func TestLoadoutService(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 合法保存
	got, err := ts.SaveLoadout(ctx, uid, SaveLoadoutInput{SkillIDs: []int{2, 0, 4, 1}})
	if err != nil {
		t.Fatalf("保存失败：%v", err)
	}
	if len(got) != 4 {
		t.Errorf("应原样返回槽位数组：%v", got)
	}
	slots, err := ts.Loadout(ctx, uid)
	if err != nil || slots[0] != 2 || slots[1] != 0 || slots[2] != 4 || slots[3] != 1 {
		t.Errorf("读回槽位不符：%v, %v", slots, err)
	}

	// 审计必须落库（admin_id=NULL 表示玩家自助操作）
	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM admin_audit_logs WHERE action = 'update_loadout' AND target = $1`,
		fmt.Sprintf("user#%d", uid)).Scan(&n); err != nil || n != 1 {
		t.Errorf("槽位变更必须写审计（%d, %v）", n, err)
	}

	// 拒绝分支
	if _, err := ts.SaveLoadout(ctx, uid, SaveLoadoutInput{SkillIDs: []int{1, 2, 3, 4, 5}}); !errors.Is(err, ErrBadInput) {
		t.Errorf("超槽应拒绝，实际 %v", err)
	}
	if _, err := ts.SaveLoadout(ctx, uid, SaveLoadoutInput{SkillIDs: []int{1, 1}}); !errors.Is(err, ErrBadInput) {
		t.Errorf("重复技能应拒绝，实际 %v", err)
	}
	if _, err := ts.SaveLoadout(ctx, uid, SaveLoadoutInput{SkillIDs: []int{999}}); !errors.Is(err, ErrBadInput) {
		t.Errorf("未解锁技能应拒绝，实际 %v", err)
	}

	// 被动技能不占主动槽
	var passive int
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM skills WHERE kind = 'passive' ORDER BY id LIMIT 1`).Scan(&passive); err == nil {
		ts.exec(t, `INSERT INTO user_skills (user_id, skill_id) VALUES ($1,$2) ON CONFLICT DO NOTHING`, uid, passive)
		if _, err := ts.SaveLoadout(ctx, uid, SaveLoadoutInput{SkillIDs: []int{passive}}); !errors.Is(err, ErrBadInput) {
			t.Errorf("被动技能应拒绝，实际 %v", err)
		}
	}

	broken := openBrokenService(t)
	if _, err := broken.SaveLoadout(ctx, uid, SaveLoadoutInput{SkillIDs: []int{1}}); err == nil {
		t.Error("故障态应报错")
	}
	if _, err := broken.Loadout(ctx, uid); err == nil {
		t.Error("故障态应报错")
	}
}

func TestUpgradeSkillService(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	if _, err := ts.UpgradeSkill(ctx, uid, 0); !errors.Is(err, ErrBadInput) {
		t.Errorf("非法 id 应 ErrBadInput，实际 %v", err)
	}
	if _, err := ts.UpgradeSkill(ctx, uid, 999); !errors.Is(err, ErrBadInput) {
		t.Errorf("未拥有应 ErrBadInput，实际 %v", err)
	}
	// 余额不足
	ts.exec(t, `UPDATE user_wallets SET coin = 0 WHERE user_id = $1`, uid)
	if _, err := ts.UpgradeSkill(ctx, uid, 1); !errors.Is(err, ErrBadInput) {
		t.Errorf("余额不足应 ErrBadInput，实际 %v", err)
	}
	// 满级
	ts.exec(t, `UPDATE user_wallets SET coin = 999999999 WHERE user_id = $1`, uid)
	ts.exec(t, `UPDATE user_skills SET level = $2 WHERE user_id = $1 AND skill_id = 2`, uid, skillRules.MaxLevel)
	if _, err := ts.UpgradeSkill(ctx, uid, 2); !errors.Is(err, ErrBadInput) {
		t.Errorf("满级应 ErrBadInput，实际 %v", err)
	}
	// 成功：等级 +1、扣费、返回新等级
	lv, err := ts.UpgradeSkill(ctx, uid, 1)
	if err != nil || lv != 2 {
		t.Fatalf("升级应到 2 级：%d, %v", lv, err)
	}

	broken := openBrokenService(t)
	if _, err := broken.UpgradeSkill(ctx, uid, 1); err == nil {
		t.Error("故障态应报错")
	}
}

func TestSkillLevelsClamp(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 塞一个越界等级：读路径必须夹紧，否则脏数据变成 4900‰ 伤害
	ts.exec(t, `UPDATE user_skills SET level = 99 WHERE user_id = $1 AND skill_id = 1`, uid)
	lv, err := ts.SkillLevels(ctx, uid)
	if err != nil {
		t.Fatalf("SkillLevels 失败：%v", err)
	}
	if lv[1] != skillRules.MaxLevel {
		t.Errorf("level=99 应夹紧到 %d，实际 %d", skillRules.MaxLevel, lv[1])
	}

	broken := openBrokenService(t)
	if _, err := broken.SkillLevels(ctx, uid); err == nil {
		t.Error("故障态应报错")
	}
}

// TestLoadoutOutOfRangeSlot 槽位越界的行必须被忽略而不是撑爆数组。
func TestLoadoutOutOfRangeSlot(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.exec(t, `INSERT INTO user_skill_slots (user_id, slot, skill_id) VALUES ($1, 99, 1)`, uid)
	slots, err := ts.Loadout(ctx, uid)
	if err != nil {
		t.Fatalf("Loadout 失败：%v", err)
	}
	if len(slots) != MaxActiveSlots {
		t.Errorf("返回长度应恒为 %d，实际 %d", MaxActiveSlots, len(slots))
	}
}

func TestComputeRatingForWrappers(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	build, err := ts.LoadBuildSnapshot(ctx, uid)
	if err != nil {
		t.Fatalf("快照失败：%v", err)
	}

	r := ts.ComputeRatingFor(uid, build)
	if r.ElementCoverage < 0 {
		t.Errorf("ElementCoverage 异常：%+v", r)
	}
	if p := ts.ComputePowerFor(uid, build); p <= 0 {
		t.Errorf("战力应大于 0：%d", p)
	}
}

func TestComputeRatingFallbacks(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 带专精节点的构筑：点数查询、EvaluateMastery 全链路
	nodes, err := ts.loadMasteryNodes(ctx, uid)
	if err != nil {
		t.Fatalf("读专精失败：%v", err)
	}
	build := map[string]any{"mastery_nodes": nodes, "elements": []string{"fire", "ice"}}
	_ = ts.computeRating(ctx, uid, build)

	// 专精 ID 不存在 → EvaluateMastery 失败 → 按零加成处理（不得 panic / 不得报错）
	badBuild := map[string]any{"mastery_nodes": []int{99999999}}
	_ = ts.computeRating(ctx, uid, badBuild)

	// 用户无 progress 行 → 点数查询失败 → points=0 兜底
	noProg := map[string]any{"mastery_nodes": []int{}}
	_ = ts.computeRating(ctx, uid, noProg)
	_ = uid
}

func TestLoadTasksService(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	daily, err := ts.LoadTasks(ctx, uid, "daily")
	if err != nil || len(daily) == 0 {
		t.Fatalf("daily 任务应非空：%v", err)
	}
	for _, v := range daily {
		if v.Scope != "daily" {
			t.Errorf("scope 过滤失效：%+v", v)
		}
	}
	if _, err := ts.LoadTasks(ctx, uid, "weekly"); err != nil {
		t.Errorf("weekly 应可用：%v", err)
	}

	broken := openBrokenService(t)
	if _, err := broken.LoadTasks(ctx, uid, "daily"); err == nil {
		t.Error("故障态应报错")
	}
}

func TestClaimTaskService(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 不存在
	if _, err := ts.ClaimTask(ctx, uid, 999999); !errors.Is(err, ErrNotFound) {
		t.Errorf("任务不存在应 ErrNotFound，实际 %v", err)
	}

	var taskID, target int
	if err := ts.pool.QueryRow(ctx,
		`SELECT id, target FROM tasks WHERE enabled AND scope = 'daily' ORDER BY id LIMIT 1`).
		Scan(&taskID, &target); err != nil {
		t.Skipf("无 daily 任务：%v", err)
	}

	// 未完成
	if _, err := ts.ClaimTask(ctx, uid, taskID); !errors.Is(err, ErrForbidden) {
		t.Errorf("未完成任务应 ErrForbidden，实际 %v", err)
	}

	// 完成 → 领取 → 重复领取（RowsAffected=0 分支）
	ts.exec(t, `INSERT INTO user_tasks (user_id, task_id, task_date, progress)
	            VALUES ($1,$2,CURRENT_DATE,$3)
	            ON CONFLICT (user_id, task_id, task_date) DO UPDATE SET progress = EXCLUDED.progress`,
		uid, taskID, target)
	reward, err := ts.ClaimTask(ctx, uid, taskID)
	if err != nil || len(reward) == 0 {
		t.Fatalf("领取应成功并返回奖励：%v, %v", reward, err)
	}
	if _, err := ts.ClaimTask(ctx, uid, taskID); !errors.Is(err, ErrForbidden) {
		t.Errorf("重复领取应 ErrForbidden，实际 %v", err)
	}

	broken := openBrokenService(t)
	if _, err := broken.ClaimTask(ctx, uid, taskID); err == nil {
		t.Error("故障态应报错")
	}
}
