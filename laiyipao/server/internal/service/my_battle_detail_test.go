package service

// 单局战报详情（MyBattleDetail）的服务层测试（2026-10-31 新一轮第 10 轮）。
//
// 关键行为锚点：
//   - 返回完整字段：掉落 / 元素 / 反应 / 地形 / 命中 / 峰值热量等；
//   - jsonb 空值规范化：`{}` / `[]` 而不是 `null`；
//   - 归属隔离：别人的战报与不存在的战报返回同一个 ErrNotFound；
//   - 库故障必须报错。

import (
	"context"
	"errors"
	"testing"
	"time"
)

func TestMyBattleDetailSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "bdetail")
	battleID := seedBattleFor(t, ts, uid, 5, "win", 3, time.Now().Add(-1*time.Hour))

	// 补上 JSONB 明细字段
	if _, err := ts.db.Pool.Exec(ctx, `
		UPDATE battle_records SET
			elements_used = '{"fire":12,"ice":3}'::jsonb,
			reactions_used = '{"steam_burst":4}'::jsonb,
			terrain_used = '["lava_pool"]'::jsonb,
			loot = '{"coin":800,"gem":2}'::jsonb,
			duration_ms = 15000, shots = 300, hits = 290, reactions = 7, heat_max = 85,
			hp_left = 100, wave_reached = 5
		WHERE id = $1`, battleID); err != nil {
		t.Fatalf("补战报明细失败：%v", err)
	}

	d, err := ts.MyBattleDetail(ctx, uid, battleID)
	if err != nil {
		t.Fatalf("查询战报详情失败：%v", err)
	}
	if d.ID != battleID || d.LevelID != 5 || d.Result != "win" || d.Stars != 3 {
		t.Errorf("基本字段不完整：%+v", d)
	}
	if d.Loot["coin"] != 800 || d.Loot["gem"] != 2 {
		t.Errorf("掉落解析错误：%+v", d.Loot)
	}
	if d.ElementsUsed["fire"] != 12 || d.ElementsUsed["ice"] != 3 {
		t.Errorf("元素使用解析错误：%+v", d.ElementsUsed)
	}
	if d.ReactionsUsed["steam_burst"] != 4 {
		t.Errorf("反应使用解析错误：%+v", d.ReactionsUsed)
	}
	if len(d.TerrainUsed) != 1 || d.TerrainUsed[0] != "lava_pool" {
		t.Errorf("地形使用解析错误：%+v", d.TerrainUsed)
	}
	if d.DurationMs != 15000 || d.Shots != 300 || d.Hits != 290 || d.Reactions != 7 || d.HeatMax != 85 {
		t.Errorf("战斗统计字段不完整：%+v", d)
	}
	if d.HpLeft != 100 || d.WaveReached != 5 {
		t.Errorf("剩余血量/波次字段不完整：%+v", d)
	}
	if d.ReplayHash == "" {
		t.Error("replay_hash 不应为空")
	}
}

func TestMyBattleDetailNormalizesEmptyJsonb(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "bdetail_empty")
	battleID := seedBattleFor(t, ts, uid, 1, "lose", 0, time.Now())

	d, err := ts.MyBattleDetail(ctx, uid, battleID)
	if err != nil {
		t.Fatalf("查询战报详情失败：%v", err)
	}
	if d.ElementsUsed == nil || len(d.ElementsUsed) != 0 {
		t.Errorf("空 elements_used 应规范化为 {}，实际 %#v", d.ElementsUsed)
	}
	if d.ReactionsUsed == nil || len(d.ReactionsUsed) != 0 {
		t.Errorf("空 reactions_used 应规范化为 {}，实际 %#v", d.ReactionsUsed)
	}
	if d.TerrainUsed == nil || len(d.TerrainUsed) != 0 {
		t.Errorf("空 terrain_used 应规范化为 []，实际 %#v", d.TerrainUsed)
	}
	if d.Loot == nil || len(d.Loot) != 0 {
		t.Errorf("空 loot 应规范化为 {}，实际 %#v", d.Loot)
	}
}

func TestMyBattleDetailOwnershipIsolation(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	owner, _ := newGuestUser(t, ts, "bdetail_owner")
	other, _ := newGuestUser(t, ts, "bdetail_other")
	battleID := seedBattleFor(t, ts, owner, 2, "win", 2, time.Now())

	// 别人看：ErrNotFound
	_, err := ts.MyBattleDetail(ctx, other, battleID)
	if !errors.Is(err, ErrNotFound) {
		t.Errorf("别人的战报应 ErrNotFound，实际 %v", err)
	}
	// 不存在的 id：同样 ErrNotFound（不泄露存在性）
	_, err = ts.MyBattleDetail(ctx, owner, 999999)
	if !errors.Is(err, ErrNotFound) {
		t.Errorf("不存在的战报应 ErrNotFound，实际 %v", err)
	}
	// 本人可看
	if _, err := ts.MyBattleDetail(ctx, owner, battleID); err != nil {
		t.Errorf("本人应可查看，实际 %v", err)
	}
}

func TestMyBattleDetailBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, err := ts.MyBattleDetail(ctx, 1, 1); err == nil {
		t.Fatal("连接池已关闭时查询必须报错，不能静默返回空详情")
	}
}