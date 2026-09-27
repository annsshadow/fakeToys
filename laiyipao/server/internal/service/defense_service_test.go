package service

// 防线（I-5）、专精（I-3）、诊断（L-1）与攻方属性的服务层测试。
//
// 攻方属性是 I-6 回放哈希的输入，任何"成长来源没接线"都会在这里被
// 「改输入 → 断言输出变化」的形式抓出来（attacker_loadout_test.go 的延续）。

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

func TestMasteryAllocationFlow(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 找一个第 1 层节点（无前置）
	var nodeID int
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM mastery_nodes WHERE layer = 1 ORDER BY id LIMIT 1`).Scan(&nodeID); err != nil {
		t.Skipf("无专精节点：%v", err)
	}

	// 分配成功
	if err := ts.AllocateMastery(ctx, uid, nodeID); err != nil {
		t.Fatalf("分配失败：%v", err)
	}
	pts, nodes, err := ts.LoadMastery(ctx, uid)
	if err != nil {
		t.Fatalf("LoadMastery 失败：%v", err)
	}
	if len(nodes) != 1 || nodes[0] != nodeID {
		t.Errorf("已点节点应包含 %d：%v", nodeID, nodes)
	}
	if pts < 0 {
		t.Errorf("点数异常：%d", pts)
	}

	// 重复点亮 → ErrForbidden
	if err := ts.AllocateMastery(ctx, uid, nodeID); !errors.Is(err, ErrForbidden) {
		t.Errorf("重复点亮应 ErrForbidden，实际 %v", err)
	}
	// 不存在的节点 → EvaluateMastery 失败 → ErrBadInput
	if err := ts.AllocateMastery(ctx, uid, 99999999); !errors.Is(err, ErrBadInput) {
		t.Errorf("非法节点应 ErrBadInput，实际 %v", err)
	}

	broken := openBrokenService(t)
	if _, _, err := broken.LoadMastery(ctx, uid); err == nil {
		t.Error("故障态应报错")
	}
	if err := broken.AllocateMastery(ctx, uid, nodeID); err == nil {
		t.Error("故障态应报错")
	}
	// 不存在的用户：LoadMastery 的点数查询失败
	if _, _, err := broken.LoadMastery(ctx, 99999999); err == nil {
		t.Error("无 progress 用户应报错")
	}
}

func TestDiagnoseFlow(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 非法关卡
	if _, err := ts.Diagnose(ctx, uid, 0, 1); !errors.Is(err, ErrBadInput) {
		t.Errorf("level 0 应 ErrBadInput，实际 %v", err)
	}
	if _, err := ts.Diagnose(ctx, uid, domain.TotalLevels+1, 1); !errors.Is(err, ErrBadInput) {
		t.Errorf("超界关卡应 ErrBadInput，实际 %v", err)
	}

	// 造两局战报（带元素使用）
	for i := 0; i < 2; i++ {
		ts.grant(t, ctx, uid, map[string]int64{"energy": 10})
		tp, err := ts.StartBattle(ctx, uid, 1)
		if err != nil {
			t.Skipf("开局失败：%v", err)
		}
		if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput()); err != nil {
			t.Fatalf("结算失败：%v", err)
		}
	}

	d, err := ts.Diagnose(ctx, uid, 1, 2)
	if err != nil {
		t.Fatalf("诊断失败：%v", err)
	}
	if d.Stage == "" || d.Title == "" {
		t.Errorf("诊断必须给出分类与结论：%+v", d)
	}

	broken := openBrokenService(t)
	if _, err := broken.Diagnose(ctx, uid, 1, 1); err == nil {
		t.Error("故障态应报错")
	}
}

func TestListDefensesViews(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	owner := ts.newUser(t, ctx)
	foe := ts.newUser(t, ctx)

	if _, err := ts.SaveDefense(ctx, owner, SaveDefenseInput{
		Name: "svc防线", Skills: []int{1, 2, 3},
	}); err != nil {
		t.Fatalf("建防线失败：%v", err)
	}
	// 候选集按战力取前 20 —— 开发库里有历史防线，把本用例的战力拉满
	// 保证进入候选集（要测的是读路径的过滤与标记，不是排序）。
	ts.exec(t, `UPDATE defenses SET power = 999999999 WHERE owner_id = $1`, owner)

	// foe 视角：owner 的防线是候选
	mine, candidates, err := ts.ListDefenses(ctx, foe)
	if err != nil {
		t.Fatalf("列表失败：%v", err)
	}
	if mine != nil {
		t.Error("foe 没有防线，mine 应为 nil")
	}
	var found *DefenseView
	for i := range candidates {
		if candidates[i].OwnerID == owner {
			found = &candidates[i]
		}
	}
	if found == nil {
		t.Fatal("候选列表应包含 owner 的防线")
	}
	if !found.CanChallenge {
		t.Error("未用额度时 can_challenge 应为 true")
	}

	// owner 视角：自己的防线在 mine
	mine, candidates, err = ts.ListDefenses(ctx, owner)
	if err != nil || mine == nil || mine.OwnerID != owner {
		t.Fatalf("own 防线应出现在 mine：%+v, %v", mine, err)
	}
	for _, c := range candidates {
		if c.OwnerID == owner {
			t.Error("候选列表绝不能包含自己的防线")
		}
	}

	// 开护盾后：候选的 can_challenge 必须同步关闭（重存会重算战力，再拉满一次）
	if _, err := ts.SaveDefense(ctx, owner, SaveDefenseInput{
		Name: "svc防线", Skills: []int{1}, ShieldHours: 24,
	}); err != nil {
		t.Fatalf("开护盾失败：%v", err)
	}
	ts.exec(t, `UPDATE defenses SET power = 999999999 WHERE owner_id = $1`, owner)
	_, candidates, _ = ts.ListDefenses(ctx, foe)
	for _, c := range candidates {
		if c.OwnerID == owner {
			if c.CanChallenge {
				t.Error("护盾中 can_challenge 必须为 false")
			}
			if c.ChallengeBlocked == "" {
				t.Error("护盾中必须给出 blocked 原因")
			}
		}
	}
}

func TestChallengeDefensePaths(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	owner := ts.newUser(t, ctx)
	foe := ts.newUser(t, ctx)

	dv, err := ts.SaveDefense(ctx, owner, SaveDefenseInput{Name: "被挑战", Skills: []int{1, 2, 3}})
	if err != nil {
		t.Fatalf("建防线失败：%v", err)
	}

	in := ChallengeInput{Seed: 1, Won: true, DurationMs: 60_000, HPLeftPct: 100, ReplayHash: "0000000000000000"}

	// 不存在
	if _, err := ts.ChallengeDefense(ctx, foe, 99999999, in); !errors.Is(err, ErrNotFound) {
		t.Errorf("未知防线应 ErrNotFound，实际 %v", err)
	}
	// 挑自己
	if _, err := ts.ChallengeDefense(ctx, owner, dv.ID, in); !errors.Is(err, ErrForbidden) {
		t.Errorf("挑战自己应 ErrForbidden，实际 %v", err)
	}

	// 胜利：战利品按比例入账（对方钱包余额可能不够扣，stolen 允许为空）
	res, err := ts.ChallengeDefense(ctx, foe, dv.ID, in)
	if err != nil {
		t.Fatalf("挑战失败：%v", err)
	}
	if !res.Won || res.MyAttempts != 1 {
		t.Errorf("挑战结果异常：%+v", res)
	}
	// 挑战记录落库
	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM defense_challenges WHERE defense_id = $1 AND challenger_id = $2 AND won`, dv.ID, foe).
		Scan(&n); err != nil || n != 1 {
		t.Errorf("挑战记录未落库（%d, %v）", n, err)
	}

	// 失败：losses 计数
	lose := in
	lose.Won = false
	if _, err := ts.ChallengeDefense(ctx, foe, dv.ID, lose); err != nil {
		t.Fatalf("失败挑战应被记录：%v", err)
	}
	var losses int
	if err := ts.pool.QueryRow(ctx, `SELECT losses FROM defenses WHERE id = $1`, dv.ID).Scan(&losses); err != nil || losses != 1 {
		t.Errorf("losses 应 +1（%d, %v）", losses, err)
	}

	// 每日额度用尽：前面已消耗 2 次（1 胜 1 负），最后 1 次放行，第 4 次拒绝
	if _, err := ts.ChallengeDefense(ctx, foe, dv.ID, lose); err != nil {
		t.Fatalf("额度内挑战不应报错：%v", err)
	}
	if _, err := ts.ChallengeDefense(ctx, foe, dv.ID, lose); !errors.Is(err, ErrForbidden) {
		t.Errorf("超出每日额度应 ErrForbidden，实际 %v", err)
	}
}

func TestSaveDefenseService(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 默认名 + 空构筑
	dv, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{})
	if err != nil {
		t.Fatalf("默认保存失败：%v", err)
	}
	if dv.Name != "我的防线" {
		t.Errorf("空名应回落默认：%q", dv.Name)
	}
	if dv.SnapshotHash == "" {
		t.Error("快照哈希必填（挑战者校验用）")
	}

	// 名字超长
	if _, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{Name: string(make([]rune, MaxDefenseNameLen+1))}); !errors.Is(err, ErrBadInput) {
		t.Errorf("超长名应 ErrBadInput，实际 %v", err)
	}
	// 工程装置白名单
	if _, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{Works: []string{"god_mode"}}); !errors.Is(err, ErrBadInput) {
		t.Errorf("未知装置应 ErrBadInput，实际 %v", err)
	}
	// 未拥有的装备
	if _, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{Equipment: []int{9999}}); !errors.Is(err, ErrBadInput) {
		t.Errorf("未拥有装备应 ErrBadInput，实际 %v", err)
	}

	// upsert：同一玩家只有一条防线
	dv2, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{Name: "改名", Skills: []int{1}})
	if err != nil {
		t.Fatalf("二次保存失败：%v", err)
	}
	var count int
	if err := ts.pool.QueryRow(ctx, `SELECT COUNT(*) FROM defenses WHERE owner_id = $1`, uid).Scan(&count); err != nil || count != 1 {
		t.Errorf("防线应唯一（%d, %v）", count, err)
	}
	if dv2.ID != dv.ID {
		t.Logf("注意：upsert 生成了新 id（%d → %d）", dv.ID, dv2.ID)
	}

	broken := openBrokenService(t)
	if _, err := broken.SaveDefense(ctx, uid, SaveDefenseInput{}); err == nil {
		t.Error("故障态应报错")
	}
}

// TestSaveDefenseShieldBoundary 护盾时间写库正确。
func TestSaveDefenseShieldBoundary(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	if _, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{ShieldHours: MaxDefenseShieldHours}); err != nil {
		t.Fatalf("合法护盾失败：%v", err)
	}
	var until *time.Time
	if err := ts.pool.QueryRow(ctx, `SELECT shielded_until FROM defenses WHERE owner_id = $1`, uid).Scan(&until); err != nil {
		t.Fatalf("读护盾失败：%v", err)
	}
	if until == nil || until.Before(time.Now()) {
		t.Errorf("护盾到期时间应在未来：%v", until)
	}
}

func TestGeneratedPowerLevelMapping(t *testing.T) {
	if gl := GeneratedPowerLevel(0); gl.ID != 1 {
		t.Errorf("power=0 应映射第 1 关，实际 %d", gl.ID)
	}
	if gl := GeneratedPowerLevel(150); gl.ID != 1 {
		t.Errorf("power=150 应映射第 1 关（<200），实际 %d", gl.ID)
	}
	if gl := GeneratedPowerLevel(40_000); gl.ID != 100 {
		t.Errorf("power=40000 应封顶第 100 关，实际 %d", gl.ID)
	}
	mid := GeneratedPowerLevel(10_000)
	if mid.ID != 50 {
		t.Errorf("power=10000 应映射第 50 关，实际 %d", mid.ID)
	}
}

func TestAttackerGrowthFromStage(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	before, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("computeAttacker：%v", err)
	}
	// max_stage 推高 → 攻击力线性成长
	ts.exec(t, `UPDATE user_progress SET max_stage = 50 WHERE user_id = $1`, uid)
	after, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("computeAttacker(after)：%v", err)
	}
	if after.Attack != before.Attack+50*20 {
		t.Errorf("50 关应 +1000‰ 攻击：%d → %d", before.Attack, after.Attack)
	}
	if after.ElementCap <= before.ElementCap {
		t.Errorf("50 关应提升元素层数上限：%d → %d", before.ElementCap, after.ElementCap)
	}
}

// TestAttackerMasteryEffects 点亮专精节点后攻方必须变化 ——
// 这是「专精树只写不读」空壳的封条。
func TestAttackerMasteryEffects(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.exec(t, `UPDATE user_progress SET mastery_points = 20 WHERE user_id = $1`, uid)

	before, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("computeAttacker：%v", err)
	}

	// 找 element_coef 类节点（每节点 +15‰）
	var coefNodes []int
	rows, err := ts.pool.Query(ctx,
		`SELECT id FROM mastery_nodes WHERE kind = 'element_coef' ORDER BY id LIMIT 3`)
	if err != nil {
		t.Skipf("无节点：%v", err)
	}
	for rows.Next() {
		var id int
		_ = rows.Scan(&id)
		coefNodes = append(coefNodes, id)
	}
	rows.Close()
	if len(coefNodes) == 0 {
		t.Skip("无 element_coef 节点")
	}
	for _, id := range coefNodes {
		ts.exec(t, `INSERT INTO user_mastery_nodes (user_id, node_id) VALUES ($1,$2) ON CONFLICT DO NOTHING`, uid, id)
	}

	after, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("computeAttacker(after)：%v", err)
	}
	if after.ElementCoefPermille <= before.ElementCoefPermille {
		t.Errorf("点亮 %d 个系数节点应提升元素系数：%d → %d",
			len(coefNodes), before.ElementCoefPermille, after.ElementCoefPermille)
	}

	// ExtraSlots：无插槽节点时为 0，非负
	extra, err := ts.ExtraSlots(ctx, uid)
	if err != nil || extra < 0 {
		t.Errorf("ExtraSlots = %d, %v", extra, err)
	}
}

// TestLoadLoadoutGemsAffixes 宝石词条必须进入攻方装配（user_gems 读路径）。
func TestLoadLoadoutGemsAffixes(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 基线：初始装备（initNewUser 送武器+护手）本身有贡献，断言用差值
	base, err := ts.loadLoadout(ctx, uid)
	if err != nil {
		t.Fatalf("loadLoadout(基线)：%v", err)
	}

	// 两颗宝石：合法 attack、未知词条。
	// ⚠️ 无法用真实库构造"坏 JSON"分支 —— JSONB 列在 INSERT 时就校验语法，
	// loadLoadout 的 json.Unmarshal 失败分支是防御性死代码（见报告豁免清单）。
	ts.exec(t, `INSERT INTO user_gems (user_id, gem_id, quality, affixes, equipped_slot)
	            VALUES ($1, 1, 'blue', $2, 'slot0'), ($1, 2, 'blue', $3, 'slot1')`,
		uid,
		[]byte(`[{"affix":"attack","value":50}]`),
		[]byte(`[{"affix":"mystery","value":10}]`))

	c, err := ts.loadLoadout(ctx, uid)
	if err != nil {
		t.Fatalf("loadLoadout：%v", err)
	}
	if c.Attack != base.Attack+50 {
		t.Errorf("attack 词条应进 Attack：基线 %d + 50，实际 %d", base.Attack, c.Attack)
	}
	if c.EquippedGemCount != base.EquippedGemCount+2 {
		t.Errorf("两颗宝石都应解析成功，实际 %d（基线 %d）", c.EquippedGemCount, base.EquippedGemCount)
	}
	if c.AffixCountIgnored != base.AffixCountIgnored+1 {
		t.Errorf("未知词条应计入 Ignored，实际 %d（基线 %d）", c.AffixCountIgnored, base.AffixCountIgnored)
	}

	// 故障态
	broken := openBrokenService(t)
	if _, err := broken.loadLoadout(ctx, uid); err == nil {
		t.Error("故障态应报错")
	}
}

// TestLoadMasteryEffectZeroNodes 未点专精：零值效果 + 空列表，不得报错。
func TestLoadMasteryEffectZeroNodes(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	eff, nodes, err := ts.loadMasteryEffect(ctx, uid)
	if err != nil {
		t.Fatalf("loadMasteryEffect：%v", err)
	}
	if len(nodes) != 0 || eff.ExtraSlots != 0 {
		t.Errorf("新号应无节点零效果：%+v %v", eff, nodes)
	}
}

// TestLoadMasteryEffectPointsFallback 点数行缺失时按 0 点降级，不得报错。
func TestLoadMasteryEffectPointsFallback(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 点一个节点但没有 mastery_points（progress 行被拿走 → 查询失败 → points=0）
	ts.renameTable(t, "user_progress", "user_progress_bak")
	ts.exec(t, `INSERT INTO user_mastery_nodes (user_id, node_id)
	            SELECT $1, id FROM mastery_nodes WHERE layer = 1 ORDER BY id LIMIT 1`, uid)
	_, _, err := ts.loadMasteryEffect(ctx, uid)
	ts.renameTable(t, "user_progress_bak", "user_progress")
	if err != nil {
		t.Errorf("点数缺失应降级而不是报错：%v", err)
	}
}
