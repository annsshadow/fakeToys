package service

// 第 149 轮：fail-loud 清扫 —— 剩余 `_ = json.Unmarshal` 读侧改报错。
// 覆盖：Diagnose（elements_used，玩家侧）与后台三处（AdminLevelWaves /
// AdminEconomy / AdminListRedeemCodes）。修前它们静默把坏 jsonb 显示成空，
// 与第 125/146/148 轮同族（出错必须响，AGENTS #11）。
//
// 判据：把各 jsonb 列写成「合法 jsonb 但形状不对」的值后，对应方法必须报错。

import (
	"context"
	"strings"
	"testing"
)

func TestFailLoudSweep(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 1) Diagnose：坏 elements_used（数组而非对象）→ 报错
	ts.exec(t, `INSERT INTO users (guest_token, nickname, is_guest) VALUES ('failloud_u', 'fl', TRUE)`)
	var uid int64
	if err := ts.pool.QueryRow(ctx, `SELECT id FROM users WHERE guest_token = 'failloud_u'`).Scan(&uid); err != nil {
		t.Fatalf("查用户失败：%v", err)
	}
	ts.exec(t, `INSERT INTO battle_records (user_id, level_id, result, elements_used)
		VALUES ($1, 1, 'lose', $2)`, uid, `[1,2,3]`)
	if _, err := ts.Diagnose(ctx, uid, 1, 3); err == nil || !strings.Contains(err.Error(), "非法 jsonb") {
		t.Errorf("Diagnose 应对坏 elements_used 报错，实际 err=%v", err)
	}
	// 复原
	ts.exec(t, `UPDATE battle_records SET elements_used = '{}'::jsonb WHERE user_id = $1`, uid)

	// 2) AdminLevelWaves：坏 spawns → 报错
	ts.exec(t, `UPDATE level_waves SET spawns = $1::jsonb WHERE level_id = (SELECT MIN(level_id) FROM level_waves) AND wave_index = (SELECT MIN(wave_index) FROM level_waves WHERE level_id = (SELECT MIN(level_id) FROM level_waves))`, `"not-array"`)
	if _, err := ts.AdminLevelWaves(ctx, 1); err == nil {
		t.Errorf("AdminLevelWaves 应对坏 spawns 报错（fail-loud），却静默成功了")
	}

	// 3) AdminEconomy：坏 shop price → 报错
	ts.exec(t, `UPDATE shop_items SET price = $1::jsonb WHERE id = (SELECT MIN(id) FROM shop_items)`, `[1]`)
	if _, err := ts.AdminEconomy(ctx); err == nil {
		t.Errorf("AdminEconomy 应对坏 price 报错（fail-loud），却静默成功了")
	}

	// 4) AdminListRedeemCodes：坏 reward → 报错
	ts.exec(t, `UPDATE redeem_codes SET reward = $1::jsonb WHERE id = (SELECT MIN(id) FROM redeem_codes)`, `[1]`)
	if _, err := ts.AdminListRedeemCodes(ctx); err == nil {
		t.Errorf("AdminListRedeemCodes 应对坏 reward 报错（fail-loud），却静默成功了")
	}
}
