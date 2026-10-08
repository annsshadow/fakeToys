package service

// 第 144 轮：AdminDashboard 的关卡漏斗（stage_funnel）fail-loud。
// 修前漏斗迭代结束后 `return d, rows.Err()` —— rows 是**反应分布**查询的句柄
// （早已查过、恒 nil），漏斗 frows 的 Err() 从未被检查：漏斗查询中途失败时
// 看板静默少一半关卡却返回 nil 错误。
//
// 判据：看板在干净 scratch 库上正确聚合出每关的 attempts/clears，
// 且整个 AdminDashboard 无错误。

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func TestAdminDashboardStageFunnel(t *testing.T) {
	ts := openScratchService(t) // 干净库：漏斗聚合确定
	ctx := context.Background()

	var uid int64
	if err := ts.db.Pool.QueryRow(ctx,
		`INSERT INTO users (guest_token, nickname, is_guest) VALUES ($1,'dash_probe',TRUE) RETURNING id`,
		fmt.Sprintf("dashprobe_%d", time.Now().UnixNano())).Scan(&uid); err != nil {
		t.Fatalf("造用户失败：%v", err)
	}

	seedBattles := func(level int, results ...string) {
		for _, r := range results {
			if _, err := ts.db.Pool.Exec(ctx,
				`INSERT INTO battle_records (user_id, level_id, result) VALUES ($1,$2,$3)`,
				uid, level, r); err != nil {
				t.Fatalf("写战报 level=%d result=%s 失败：%v", level, r, err)
			}
		}
	}
	// 第 5 关 3 场（1 胜），第 9 关 2 场（2 胜）
	seedBattles(5, "win", "lose", "lose")
	seedBattles(9, "win", "win")

	d, err := ts.AdminDashboard(ctx)
	if err != nil {
		t.Fatalf("AdminDashboard 失败（修前漏斗 Err() 被吞、此处应无错）：%v", err)
	}

	got := map[int]StageFunnel{}
	for _, f := range d.StageFunnel {
		got[f.LevelID] = f
	}
	if f, ok := got[5]; !ok || f.Attempts != 3 || f.Number != 1 {
		t.Errorf("第 5 关漏斗应 attempts=3 clears=1，实际 %v（有=%v）", f, ok)
	}
	if f, ok := got[9]; !ok || f.Attempts != 2 || f.Number != 2 {
		t.Errorf("第 9 关漏斗应 attempts=2 clears=2，实际 %v（有=%v）", f, ok)
	}
	// 只种了 5/9 两关（scratch 库无其它战报）→ 漏斗应恰好两行
	if len(d.StageFunnel) != 2 {
		t.Errorf("漏斗应含 2 个关卡，实际 %d 行：%+v", len(d.StageFunnel), d.StageFunnel)
	}
}
