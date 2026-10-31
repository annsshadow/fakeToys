package service

// 后台战报 CSV 导出的服务层测试（2026-10-30 实质优化第 13 轮）。
//
// 关键行为锚点：
//   - 输出是 UTF-8 BOM + CSV（Excel 中文兼容）；
//   - 表头包含列表的全部 18 列（含 total_enemies）；
//   - user_id / level_id 过滤与列表接口同口径；
//   - 库故障必须报错而不是静默返回空 CSV。

import (
	"bytes"
	"context"
	"encoding/csv"
	"fmt"
	"testing"
	"time"
)

// seedBattleRecord 插入一条战报，返回战报 id。
func seedBattleRecord(t *testing.T, ts *testService, userID int64, levelID int, result string) int64 {
	t.Helper()
	ctx := context.Background()
	var id int64
	if err := ts.db.Pool.QueryRow(ctx,
		`INSERT INTO battle_records (user_id, level_id, result, replay_hash)
		 VALUES ($1, $2, $3, $4)
		 RETURNING id`, userID, levelID, result,
		fmt.Sprintf("hash_%d", time.Now().UnixNano())).Scan(&id); err != nil {
		t.Fatalf("插入战报失败：%v", err)
	}
	return id
}

func TestAdminExportBattlesCSV(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "exportbattle")
	battleID := seedBattleRecord(t, ts, id, 1, "win")

	data, err := ts.AdminExportBattles(ctx, 0, 0, false, nil, nil)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	if !bytes.HasPrefix(data, []byte("\xEF\xBB\xBF")) {
		t.Error("CSV 必须以 UTF-8 BOM 开头")
	}

	records, err := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(data, []byte("\xEF\xBB\xBF")))).ReadAll()
	if err != nil {
		t.Fatalf("解析 CSV 失败：%v", err)
	}
	if len(records) < 2 {
		t.Fatalf("应有表头 + 数据，实际 %d 行", len(records))
	}

	wantHeader := []string{
		"id", "user_id", "nickname", "level_id", "result", "stars", "score",
		"kills", "leaked", "wave_reached", "duration_ms", "reactions", "heat_max",
		"replay_hash", "created_at", "verify_checked", "verify_mismatched", "total_enemies",
	}
	if len(records[0]) != len(wantHeader) {
		t.Fatalf("表头列数 = %d，期望 %d：%v", len(records[0]), len(wantHeader), records[0])
	}
	for i, h := range wantHeader {
		if records[0][i] != h {
			t.Errorf("表头第 %d 列 = %q，期望 %q", i, records[0][i], h)
		}
	}

	found := false
	for _, rec := range records[1:] {
		if rec[0] == fmt.Sprintf("%d", battleID) {
			found = true
			if rec[3] != "1" {
				t.Errorf("level_id 应为 1，实际 %q", rec[3])
			}
			if rec[4] != "win" {
				t.Errorf("result 应为 win，实际 %q", rec[4])
			}
		}
	}
	if !found {
		t.Error("导出应包含测试战报")
	}
}

func TestAdminExportBattlesFilterByUser(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	idA, _ := newGuestUser(t, ts, "battleA")
	idB, _ := newGuestUser(t, ts, "battleB")
	seedBattleRecord(t, ts, idA, 1, "win")
	seedBattleRecord(t, ts, idB, 2, "lose")

	data, err := ts.AdminExportBattles(ctx, idA, 0, false, nil, nil)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	records, err := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(data, []byte("\xEF\xBB\xBF")))).ReadAll()
	if err != nil {
		t.Fatalf("解析 CSV 失败：%v", err)
	}
	// 表头 + 用户 A 的 1 行
	if len(records) != 2 {
		t.Fatalf("按用户过滤后应有 1 行数据，实际 %d 行", len(records)-1)
	}
	if records[1][1] != fmt.Sprintf("%d", idA) {
		t.Errorf("匹配行应为用户 A，实际 %v", records[1])
	}
}

func TestAdminExportBattlesBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, err := ts.AdminExportBattles(ctx, 0, 0, false, nil, nil); err == nil {
		t.Fatal("连接池已关闭时导出必须报错，不能静默返回空 CSV")
	}
}
