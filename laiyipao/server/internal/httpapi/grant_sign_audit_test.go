package httpapi

import (
	"fmt"
	"testing"

	"github.com/gofiber/fiber/v2"
)

// 第 109 轮：发放/回收的**符号必须进审计**。
//
// 修前：POST /admin/users/:id/grant 无论 amount 正负，
// 审计一律记 `grant_currency`、钱包流水一律记 `admin_grant`。
// 运营在「发放资源」框里输负数就是静默回收玩家货币，
// 事后翻审计/流水都只看到「发放」——分不清哪笔是扣款。
//
// 判据落在能直接观测的那一层：
// 审计日志 `action` 列 + 钱包流水 `reason` 列（service 侧断言）
// 必须随符号分叉。
func TestGrantSignIsAuditable(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "signaudit109")
	uid, _ := e.newPlayer(t, "sign109p")

	grant := func(amount int64) int {
		st, body := e.post(t, fmt.Sprintf("/api/v1/admin/users/%d/grant", uid), tok,
			map[string]any{"currency": "coin", "amount": amount})
		t.Logf("grant(%d) -> %d %v", amount, st, body)
		return st
	}

	// 正数 = 发放
	if st := grant(100); st != 200 {
		t.Fatalf("正数发放应 200，实际 %d", st)
	}
	// 负数 = 回收（既有能力，老测试钉住的「回收场景」）
	if st := grant(-30); st != 200 {
		t.Fatalf("负数（回收）应 200，实际 %d", st)
	}
	// 零没有意义，必须拒绝
	if st := grant(0); st != fiber.StatusBadRequest {
		t.Errorf("数量为 0 应 400，实际 %d", st)
	}

	// 审计日志里两个事件必须同时可见
	_, body := e.get(t, "/api/v1/admin/audit-logs?limit=20", tok)
	items, _ := body["items"].([]any)
	actions := map[string]bool{}
	for _, it := range items {
		m, _ := it.(map[string]any)
		if m == nil {
			continue
		}
		a, _ := m["action"].(string)
		actions[a] = true
	}
	if !actions["grant_currency"] {
		t.Errorf("审计里缺少 grant_currency 事件：%v", actions)
	}
	if !actions["revoke_currency"] {
		t.Errorf("审计里缺少 revoke_currency 事件（负数发放仍被记成 grant —— 符号没进审计）：%v", actions)
	}

	// 钱包流水的 reason 列同族分叉
	if got := e.scalarInt(t, `SELECT COUNT(*) FROM wallet_flows WHERE user_id = $1 AND reason = 'admin_revoke'`, uid); got != 1 {
		t.Errorf("负数发放应恰好 1 条 admin_revoke 流水，实际 %d", got)
	}
	if got := e.scalarInt(t, `SELECT COUNT(*) FROM wallet_flows WHERE user_id = $1 AND reason = 'admin_grant'`, uid); got != 1 {
		t.Errorf("正数发放应恰好 1 条 admin_grant 流水，实际 %d", got)
	}
}
