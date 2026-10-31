package service

// 玩家战报历史记录的服务层测试（2026-10-30 实质优化第 19 轮）。
//
// 关键行为锚点：
//   - 返回按时间倒序的战斗记录，字段完整（关卡/结果/星级/分数/击杀/漏怪）；
//   - 无记录返回空列表（不是 nil）；
//   - limit 有上界（>50 夹到 20）；
//   - 库故障必须报错而不是静默返回空列表。

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func seedBattleFor(t *testing.T, ts *testService, userID int64, levelID int, result string, stars int, createdAt time.Time) int64 {
	t.Helper()
	ctx := context.Background()
	var id int64
	if err := ts.db.Pool.QueryRow(ctx,
		`INSERT INTO battle_records (user_id, level_id, result, stars, score, kills, leaked, replay_hash, created_at)
		 VALUES ($1, $2, $3, $4, 500, 10, 2, $5, $6)
		 RETURNING id`,
		userID, levelID, result, stars, fmt.Sprintf("bh_%d", time.Now().UnixNano()), createdAt).Scan(&id); err != nil {
		t.Fatalf("插入战报失败：%v", err)
	}
	return id
}

func TestMyBattleHistorySucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "bhist")
	// 两条：一条赢（较新）、一条输（较旧）
	winID := seedBattleFor(t, ts, id, 3, "win", 3, time.Now().Add(-1*time.Hour))
	loseID := seedBattleFor(t, ts, id, 2, "lose", 1, time.Now().Add(-2*time.Hour))

	items, err := ts.MyBattleHistory(ctx, id, 20, 0)
	if err != nil {
		t.Fatalf("查询战报历史失败：%v", err)
	}
	if len(items) != 2 {
		t.Fatalf("应有 2 条记录，实际 %d", len(items))
	}
	// 倒序：新的在前
	if items[0].ID != winID {
		t.Errorf("第一条应是最新（win）的战报，实际 id=%d", items[0].ID)
	}
	if items[1].ID != loseID {
		t.Errorf("第二条应是较旧（lose）的战报，实际 id=%d", items[1].ID)
	}
	first := items[0]
	if first.LevelID != 3 || first.Result != "win" || first.Stars != 3 {
		t.Errorf("字段不完整：%+v", first)
	}
	if first.Score != 500 || first.Kills != 10 || first.Leaked != 2 {
		t.Errorf("战斗统计字段不完整：%+v", first)
	}
}

func TestMyBattleHistoryEmpty(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "bhist_empty")
	items, err := ts.MyBattleHistory(ctx, id, 20, 0)
	if err != nil {
		t.Fatalf("查询战报历史失败：%v", err)
	}
	if items == nil || len(items) != 0 {
		t.Errorf("无记录应返回空列表（非 nil），实际 %v", items)
	}
}

func TestMyBattleHistoryLimitCap(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "bhist_cap")
	for i := 0; i < 25; i++ {
		seedBattleFor(t, ts, id, i%100+1, "win", 3, time.Now().Add(-time.Duration(i)*time.Minute))
	}

	items, err := ts.MyBattleHistory(ctx, id, 1000, 0)
	if err != nil {
		t.Fatalf("查询战报历史失败：%v", err)
	}
	if len(items) != 20 {
		t.Errorf("limit 超上限应夹到 20 条，实际 %d", len(items))
	}
}

func TestMyBattleHistoryBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	_, err := ts.MyBattleHistory(ctx, 1, 20, 0)
	if err == nil {
		t.Fatal("连接池已关闭时查询必须报错，不能静默返回空列表")
	}
}

// TestMyBattleHistoryPagination 守住分页（2026-11-01 新一轮第 55 轮）。
func TestMyBattleHistoryPagination(t *testing.T) {
	ts := openScratchService(t) // 一次性库，避免共享库数据干扰
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "bhist_page")
	// 造 5 条战报：i=0 是 now（最新），i=4 是最旧
	ids := make([]int64, 0, 5)
	for i := 0; i < 5; i++ {
		ids = append(ids, seedBattleFor(t, ts, id, i+1, "win", 3, time.Now().Add(-time.Duration(i)*time.Minute)))
	}

	// 第一页 2 条：最新的两条（i=0 / i=1）
	page1, err := ts.MyBattleHistory(ctx, id, 2, 0)
	if err != nil {
		t.Fatalf("第一页查询失败：%v", err)
	}
	if len(page1) != 2 {
		t.Fatalf("第一页应有 2 条，实际 %d", len(page1))
	}
	if page1[0].ID != ids[0] || page1[1].ID != ids[1] {
		t.Errorf("第一页应按时间倒序取最新两条，实际 %d/%d", page1[0].ID, page1[1].ID)
	}

	// 第二页 2 条：offset=2
	page2, err := ts.MyBattleHistory(ctx, id, 2, 2)
	if err != nil {
		t.Fatalf("第二页查询失败：%v", err)
	}
	if len(page2) != 2 {
		t.Fatalf("第二页应有 2 条，实际 %d", len(page2))
	}
	if page2[0].ID != ids[2] || page2[1].ID != ids[3] {
		t.Errorf("第二页应取第 3/4 新，实际 %d/%d", page2[0].ID, page2[1].ID)
	}

	// 第三页（offset=4）：只剩 1 条（最旧）
	page3, err := ts.MyBattleHistory(ctx, id, 2, 4)
	if err != nil {
		t.Fatalf("第三页查询失败：%v", err)
	}
	if len(page3) != 1 || page3[0].ID != ids[4] {
		t.Errorf("第三页应只剩最旧的 1 条，实际 %d", len(page3))
	}

	// 负 offset 夹到 0（不报错）
	neg, err := ts.MyBattleHistory(ctx, id, 2, -1)
	if err != nil {
		t.Fatalf("负 offset 查询失败：%v", err)
	}
	if len(neg) != 2 {
		t.Errorf("负 offset 应夹到 0 取第一页，实际 %d", len(neg))
	}
}
