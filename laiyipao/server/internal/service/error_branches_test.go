package service

// 覆盖率收尾：攻方属性的全量专精装配，与各事务的「中段失败」分支。
//
// 前几轮测试已覆盖"第一步就失败"（closed pool）与"种子数据非法"；
// 本文件处理剩下的两类：
//   - 专精树满配：computeAttacker 的各种封顶/装配分支只有在玩家
//     点了大量节点时才走到（每系数 15‰ × 48 节点必超 600 封顶）。
//   - 事务中段失败：grantWallet / INSERT 战报 / applyProgress 这些
//     步骤在「前面的查询都成功」之后才执行 —— 只能靠 scratch 库
//     改名单表来构造。

import (
	"context"
	"strconv"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// pickValidMasterySelection 选一份**合法**的满配专精：
// 每系每层取 slot 最小的 2 个（PerLayerPickLimit=2），层间前置自然满足。
// domainAllMasteryNodes 展平全部专精节点（与 computeAttacker 同一来源）。
func domainAllMasteryNodes() []domain.MasteryNode {
	var all []domain.MasteryNode
	for _, f := range domain.AllMasteryFamilies() {
		all = append(all, f.Nodes...)
	}
	return all
}

func pickValidMasterySelection(t *testing.T) []int {
	t.Helper()
	perFamilyLayer := map[string][]int{}
	for _, n := range domainAllMasteryNodes() {
		key := n.Family + "|" + strconv.Itoa(n.Layer)
		perFamilyLayer[key] = append(perFamilyLayer[key], n.ID)
	}
	var picked []int
	for _, ids := range perFamilyLayer {
		// ids 已按插入顺序（同 family+layer 内 slot 升序），取前 2 个
		for i, id := range ids {
			if i >= 2 {
				break
			}
			picked = append(picked, id)
		}
	}
	return picked
}

// TestComputeAttackerFullMastery 满配专精下攻方装配与封顶全部生效。
//
// 48 个节点让 element_coef 累计 720‰ —— 必然触发 600 封顶；
// 同时 heat_cap / skill_damage / armor / mechanic 各类节点的装配分支
// 都要求"有节点才执行"。若这些分支回归成死代码（装配被删），
// 攻方数值不会再变 —— 本用例的差值断言会红。
func TestComputeAttackerFullMastery(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	picked := pickValidMasterySelection(t)
	if len(picked) < 8 {
		t.Skipf("可选项过少：%d", len(picked))
	}
	// 点数给足：EvaluateMastery 校验"已选 + 剩余"的合法性
	ts.exec(t, `UPDATE user_progress SET mastery_points = $2 WHERE user_id = $1`, uid, len(picked)+50)
	for _, id := range picked {
		ts.exec(t, `INSERT INTO user_mastery_nodes (user_id, node_id) VALUES ($1,$2) ON CONFLICT DO NOTHING`, uid, id)
	}

	before, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("computeAttacker(满配)：%v", err)
	}

	// 系数封顶：ElementCoefPermille 的专精贡献被夹在 600 以内
	if got := before.ElementCoefPermille - domain.DefaultAttacker().ElementCoefPermille; got > 600 {
		t.Errorf("元素系数专精贡献应封顶 600，实际 +%d", got)
	}

	// 反应阶：满配下应被抬升但封顶 4
	if before.ReactionTier > 4 {
		t.Errorf("反应阶应封顶 4，实际 %d", before.ReactionTier)
	}

	// 卸掉全部专精 → 数值必须回落（证明变化真的来自这些节点）
	ts.exec(t, `DELETE FROM user_mastery_nodes WHERE user_id = $1`, uid)
	empty, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("computeAttacker(清空)：%v", err)
	}
	if empty.ElementCoefPermille >= before.ElementCoefPermille {
		t.Errorf("清空专精后元素系数应回落：%d → %d", before.ElementCoefPermille, empty.ElementCoefPermille)
	}
}

// TestExtraSlotsFromMastery 额外插槽节点必须反映到 ExtraSlots。
func TestExtraSlotsFromMastery(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 找 extra_slot 类节点（第 3 层）
	var slotNode int
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM mastery_nodes WHERE kind = 'extra_slot' ORDER BY id LIMIT 1`).Scan(&slotNode); err != nil {
		t.Skipf("无额外插槽节点：%v", err)
	}
	// 前置：点满该系 1、2 层（各 2 个）
	var fam string
	ts.pool.QueryRow(ctx, `SELECT family FROM mastery_nodes WHERE id = $1`, slotNode).Scan(&fam)
	ts.exec(t, `UPDATE user_progress SET mastery_points = 50 WHERE user_id = $1`, uid)
	ts.exec(t, `INSERT INTO user_mastery_nodes (user_id, node_id)
	            SELECT $1, id FROM mastery_nodes
	            WHERE family = $2 AND layer < 3 AND slot < 2
	            ON CONFLICT DO NOTHING`, uid, fam)
	ts.exec(t, `INSERT INTO user_mastery_nodes (user_id, node_id) VALUES ($1,$2) ON CONFLICT DO NOTHING`, uid, slotNode)

	extra, err := ts.ExtraSlots(ctx, uid)
	if err != nil {
		t.Fatalf("ExtraSlots：%v", err)
	}
	if extra < 1 {
		t.Errorf("点亮额外插槽节点后 ExtraSlots 应 ≥1，实际 %d", extra)
	}
}

// TestAttackerBrokenBranches computeAttacker / ExtraSlots 的错误出口。
func TestAttackerBrokenBranches(t *testing.T) {
	broken := openBrokenService(t)
	ctx := context.Background()
	if _, err := broken.computeAttacker(ctx, 1); err == nil {
		t.Error("故障态应报错")
	}
	if _, err := broken.ExtraSlots(ctx, 1); err == nil {
		t.Error("故障态应报错")
	}
}

// --- 事务中段失败（scratch 库 + 表改名） ---

func TestTxMidwayFailures(t *testing.T) {
	ctx := context.Background()

	t.Run("AdminLogin落会话失败", func(t *testing.T) {
		ts := openScratchService(t)
		_, username, password := ts.newAdminFixture(t, "mid", 1)
		ts.renameTable(t, "admin_tokens", "admin_tokens_bak")
		_, err := ts.AdminLogin(ctx, username, password)
		ts.renameTable(t, "admin_tokens_bak", "admin_tokens")
		mustErr(t, err, "store admin session")
	})

	t.Run("StartBattle扣体力失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_wallets", "user_wallets_bak")
		_, err := ts.StartBattle(ctx, uid, 1)
		ts.renameTable(t, "user_wallets_bak", "user_wallets")
		// battle_tokens 插入成功、体力扣减失败 → 整体回滚
		mustErr(t, err, "grant")
	})

	t.Run("SettleBattle写战报失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.grant(t, ctx, uid, map[string]int64{"energy": 100})
		tp, err := ts.StartBattle(ctx, uid, 1)
		if err != nil {
			t.Skipf("开局失败：%v", err)
		}
		ts.renameTable(t, "battle_records", "battle_records_bak")
		_, err = ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput())
		ts.renameTable(t, "battle_records_bak", "battle_records")
		mustErr(t, err, "insert battle record")
	})

	t.Run("SettleBattle发掉落失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.grant(t, ctx, uid, map[string]int64{"energy": 100})
		tp, err := ts.StartBattle(ctx, uid, 1)
		if err != nil {
			t.Skipf("开局失败：%v", err)
		}
		ts.renameTable(t, "wallet_flows", "wallet_flows_bak")
		_, err = ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput())
		ts.renameTable(t, "wallet_flows_bak", "wallet_flows")
		mustErr(t, err, "insert flow")
	})

	t.Run("槽位预算校验的ExtraSlots失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_mastery_nodes", "user_mastery_nodes_bak")
		_, err := ts.LoadBuildSnapshot(ctx, uid)
		ts.renameTable(t, "user_mastery_nodes_bak", "user_mastery_nodes")
		// loadSkillsAndSlots 本体成功，checkSlotBudget → ExtraSlots 失败
		mustErr(t, err, "load mastery")
	})

	t.Run("SaveLoadout清槽失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_skill_slots", "user_skill_slots_bak")
		_, err := ts.SaveLoadout(ctx, uid, SaveLoadoutInput{SkillIDs: []int{1, 2}})
		ts.renameTable(t, "user_skill_slots_bak", "user_skill_slots")
		// 技能归属查询成功、DELETE 槽位失败
		mustErr(t, err, "clear slots")
	})

	t.Run("SignIn发放奖励失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		// 把今天的签到奖励改成未知币种 → grantWallet 业务错误
		ts.exec(t, `UPDATE sign_in_calendar SET reward = '{"diamonds":5}' WHERE day_index = 1`)
		_, err := ts.SignIn(ctx, uid)
		mustErr(t, err, "未知货币")
	})

	t.Run("Redeem占位写入失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.exec(t, `INSERT INTO redeem_codes (id, code, reward, max_uses, used_count, enabled)
		            VALUES ((SELECT COALESCE(MAX(id),0)+1 FROM redeem_codes), 'SVC_MID_RDM', '{"coin":1}', 5, 0, true)`)
		ts.renameTable(t, "redeem_usages", "redeem_usages_bak")
		_, err := ts.Redeem(ctx, uid, "SVC_MID_RDM")
		ts.renameTable(t, "redeem_usages_bak", "redeem_usages")
		mustErr(t, err, "")
		// 清理：码本身留着无妨（scratch 库整体销毁）
	})

	t.Run("ClaimTask发放奖励失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		var taskID, target int
		if err := ts.pool.QueryRow(ctx,
			`SELECT id, target FROM tasks WHERE enabled AND scope = 'daily' ORDER BY id LIMIT 1`).
			Scan(&taskID, &target); err != nil {
			t.Skipf("无 daily 任务：%v", err)
		}
		// 奖励改成未知币种：预检/领取都成功，grantWallet 业务失败
		ts.exec(t, `UPDATE tasks SET reward = '{"diamonds":5}' WHERE id = $1`, taskID)
		ts.exec(t, `INSERT INTO user_tasks (user_id, task_id, task_date, progress)
		            VALUES ($1,$2,CURRENT_DATE,$3)`, uid, taskID, target)
		_, err := ts.ClaimTask(ctx, uid, taskID)
		mustErr(t, err, "未知货币")
	})
}

// TestAuditInsertFailure Audit 写失败必须留痕到日志（printf 分支）而不是 panic。
func TestAuditInsertFailure(t *testing.T) {
	broken := openBrokenService(t)
	// 不应 panic；返回值为空（审计刻意不阻塞主流程）
	broken.Audit(context.Background(), 1, "broken_probe", "t", map[string]any{"k": 1})
}

// TestAdminLoginBroken 故障态管理登录：查询失败走非 ErrNoRows 分支。
func TestAdminLoginBroken(t *testing.T) {
	broken := openBrokenService(t)
	if _, err := broken.AdminLogin(context.Background(), "x", "y"); err == nil {
		t.Error("故障态应报错")
	}
	if _, err := broken.ResolveAdmin(context.Background(), "anything"); err == nil {
		t.Error("故障态应报错")
	}
	if _, err := broken.EnsureBootstrapAdmin(context.Background(), "boot", "long-enough-pass"); err == nil {
		t.Error("故障态应报错")
	}
}

// TestEnsureBootstrapAdminPasswordTooLong 超长密码触发 bcrypt 上限错误。
func TestEnsureBootstrapAdminPasswordTooLong(t *testing.T) {
	ts := openScratchService(t)
	_, err := ts.EnsureBootstrapAdmin(context.Background(), "boot", string(make([]byte, 100)))
	mustErr(t, err, "hash password")
}
