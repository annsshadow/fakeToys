package service

import (
	"context"
	"errors"
	"testing"
)

// mastery_reset 道具消费端点（2026-10 轮）的行为守卫。
//
// `AllocateMastery` 是**只插不删**：点错方向没有途径改向。
// `ResetMastery` 是唯一的「重铸」入口 —— 花 1 件 mastery_reset 清空
// 全部已点亮节点。本文件钉住它的三条关键性质：
//
//  1. **道具不足时 400，节点不动**（`spendToken` 的条件 UPDATE 拦下）；
//  2. **成功后节点真的清空、道具真的扣掉、流水真的写**（不是只写回执）；
//  3. **扣掉就没了** —— 第二次 reset 必然「不足」，证明道具被真实消耗
//     而不是无限重铸。
//
// 全部走真实 DB（scratch 库迁移 + 种子），DB 不可达时整体 Skip。

func TestMasteryResetInsufficientToken(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx)
	// 直接塞 3 个节点（绕开 4 选 2 约束，只测「清空」这一步）。
	nodeIDs := ts.takeMasteryNodeIDs(t, ctx, uid, 3)
	for _, id := range nodeIDs {
		ts.exec(t, `INSERT INTO user_mastery_nodes (user_id, node_id) VALUES ($1,$2)`, uid, id)
	}

	// 新号没有 mastery_reset 道具 → 必须报 ErrBadInput，且节点**不动**。
	if _, err := ts.ResetMastery(ctx, uid); !errors.Is(err, ErrBadInput) {
		t.Fatalf("无道具 reset 应 ErrBadInput（道具不足），实际 %v", err)
	}
	if got := ts.scalar(t, ctx, `SELECT COUNT(*) FROM user_mastery_nodes WHERE user_id=$1`, uid); got != len(nodeIDs) {
		t.Errorf("道具不足时节点不应被清，实存 %d 期望 %d", got, len(nodeIDs))
	}
}

func TestMasteryResetConsumesAndClears(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx)
	nodeIDs := ts.takeMasteryNodeIDs(t, ctx, uid, 3)
	for _, id := range nodeIDs {
		ts.exec(t, `INSERT INTO user_mastery_nodes (user_id, node_id) VALUES ($1,$2)`, uid, id)
	}
	// 发 1 件 mastery_reset（走 grantWallet → grantToken 入账路径）。
	ts.grant(t, ctx, uid, map[string]int64{"mastery_reset": 1})
	if bal := ts.tokenBalance(t, ctx, uid, "mastery_reset"); bal != 1 {
		t.Fatalf("预置道具余额应为 1，实际 %d", bal)
	}

	cleared, err := ts.ResetMastery(ctx, uid)
	if err != nil {
		t.Fatalf("有道具时 reset 应成功：%v", err)
	}
	if cleared != len(nodeIDs) {
		t.Errorf("清掉的节点数应为 %d，实际 %d", len(nodeIDs), cleared)
	}
	if got := ts.scalar(t, ctx, `SELECT COUNT(*) FROM user_mastery_nodes WHERE user_id=$1`, uid); got != 0 {
		t.Errorf("reset 后节点应清空，实存 %d", got)
	}
	if bal := ts.tokenBalance(t, ctx, uid, "mastery_reset"); bal != 0 {
		t.Errorf("道具应被扣到 0，实际 %d", bal)
	}
	// 流水必须写（看板按 currency 聚合时「花了 1 件」才可见）。
	if flows := ts.scalar(t, ctx,
		`SELECT COUNT(*) FROM wallet_flows WHERE user_id=$1 AND currency='mastery_reset' AND delta=-1`,
		uid); flows != 1 {
		t.Errorf("应写 1 条 mastery_reset 消费流水，实际 %d", flows)
	}
}

func TestMasteryResetIsNotRepeatableFree(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx)
	nodeIDs := ts.takeMasteryNodeIDs(t, ctx, uid, 2)
	for _, id := range nodeIDs {
		ts.exec(t, `INSERT INTO user_mastery_nodes (user_id, node_id) VALUES ($1,$2)`, uid, id)
	}
	ts.grant(t, ctx, uid, map[string]int64{"mastery_reset": 1})

	if _, err := ts.ResetMastery(ctx, uid); err != nil {
		t.Fatalf("第一次 reset 应成功：%v", err)
	}
	// 道具已被扣掉 → 第二次必须「不足」。若这里能成功，等于免费无限重铸。
	if _, err := ts.ResetMastery(ctx, uid); !errors.Is(err, ErrBadInput) {
		t.Fatalf("道具已扣光后第二次 reset 应 ErrBadInput，实际 %v", err)
	}
}

// takeMasteryNodeIDs 取 n 个真实存在且**未被 uid 点亮**的专精节点 id。
func (ts *testService) takeMasteryNodeIDs(t *testing.T, ctx context.Context, uid int64, n int) []int {
	t.Helper()
	rows, err := ts.pool.Query(ctx,
		`SELECT n.id FROM mastery_nodes n
		    WHERE NOT EXISTS (SELECT 1 FROM user_mastery_nodes m WHERE m.node_id=n.id AND m.user_id=$1)
		    ORDER BY n.id LIMIT $2`, uid, n)
	if err != nil {
		t.Fatalf("取专精节点失败：%v", err)
	}
	defer rows.Close()
	ids := make([]int, 0, n)
	for rows.Next() {
		var id int
		if err := rows.Scan(&id); err != nil {
			t.Fatalf("scan 节点 id：%v", err)
		}
		ids = append(ids, id)
	}
	if err := rows.Err(); err != nil {
		t.Fatalf("迭代节点：%v", err)
	}
	if len(ids) < n {
		t.Fatalf("专精节点不足 %d 个（内容表缺失？）", n)
	}
	return ids
}

// tokenBalance 读一件道具当前余额（没有行 = 0）。
func (ts *testService) tokenBalance(t *testing.T, ctx context.Context, uid int64, token string) int {
	t.Helper()
	var bal int
	err := ts.pool.QueryRow(ctx,
		`SELECT COALESCE(SUM(balance),0) FROM user_tokens WHERE user_id=$1 AND token=$2`,
		uid, token).Scan(&bal)
	if err != nil {
		t.Fatalf("读道具余额：%v", err)
	}
	return bal
}
