package service

// 第 139 轮：level_stars 此前是「只写不读」的客户端面——
// 结算 GREATEST upsert（SettleBattle）只写，读方只有排行榜/看板（别家数据），
// 玩家自己的每关星级没有任何服务端出口，小程序选关页的星图因此永远空白。
//
// 判据：LevelStars 返回的是库里该用户的实际历史最好星级
// （重复结算取 GREATEST，后到的低星不覆盖），且严格按 user_id 隔离。

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func TestLevelStarsServiceMethod(t *testing.T) {
	ts := openScratchService(t) // 一次性库，已迁移+种子
	ctx := context.Background()

	var uid int64
	if err := ts.db.Pool.QueryRow(ctx,
		`INSERT INTO users (guest_token, nickname, is_guest) VALUES ($1,'stars_probe',TRUE) RETURNING id`,
		fmt.Sprintf("stars_probe_%d", time.Now().UnixNano())).Scan(&uid); err != nil {
		t.Fatalf("造用户失败：%v", err)
	}

	// 无结算记录 → 空 map（非 nil，非含 0 值的占位）
	got, err := ts.LevelStars(ctx, uid)
	if err != nil {
		t.Fatalf("查询星级失败：%v", err)
	}
	if got == nil || len(got) != 0 {
		t.Fatalf("无记录应得空 map，实际 %v", got)
	}

	// 模拟结算 upsert 两次同一关（与 SettleBattle 同语义）：
	// 先写 2 星再写 1 星，GREATEST 后必须仍是 2 —— 「历史最好」的判据。
	upsert := `
		INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
		VALUES ($1,$2,$3,0,1,0)
		ON CONFLICT (user_id, level_id) DO UPDATE SET
			stars = GREATEST(level_stars.stars, EXCLUDED.stars),
			best_score = GREATEST(level_stars.best_score, EXCLUDED.best_score),
			clears = level_stars.clears + EXCLUDED.clears`
	for _, row := range []struct {
		lvl, st int
	}{
		{11, 2},
		{11, 1},
		{12, 3},
	} {
		if _, err := ts.db.Pool.Exec(ctx, upsert, uid, row.lvl, row.st); err != nil {
			t.Fatalf("写星级行失败：%v", err)
		}
	}

	got, err = ts.LevelStars(ctx, uid)
	if err != nil {
		t.Fatalf("查询星级失败：%v", err)
	}
	if got[11] != 2 {
		t.Errorf("第 11 关应为历史最好 2 星（后到的 1 星不得覆盖 GREATEST），实际 %d", got[11])
	}
	if got[12] != 3 {
		t.Errorf("第 12 关应为 3 星，实际 %d", got[12])
	}
	if len(got) != 2 {
		t.Errorf("该用户只有两关有记录，实际 %d 条", len(got))
	}

	// 属主隔离：第二用户查不到第一用户的星级
	var uid2 int64
	if err := ts.db.Pool.QueryRow(ctx,
		`INSERT INTO users (guest_token, nickname, is_guest) VALUES ($1,'stars_probe2',TRUE) RETURNING id`,
		fmt.Sprintf("stars_probe2_%d", time.Now().UnixNano())).Scan(&uid2); err != nil {
		t.Fatalf("造第二用户失败：%v", err)
	}
	other, err := ts.LevelStars(ctx, uid2)
	if err != nil {
		t.Fatalf("查第二用户星级失败：%v", err)
	}
	if len(other) != 0 {
		t.Errorf("第二用户应无星级记录，实际 %v（属主泄漏）", other)
	}
}
