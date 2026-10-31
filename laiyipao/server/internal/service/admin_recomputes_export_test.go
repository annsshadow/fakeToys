package service

// 后台重算台账 CSV 导出的服务层测试（2026-11-01 新一轮第 45 轮）。
//
// 关键行为锚点：
//   - 输出是 UTF-8 BOM + CSV（Excel 中文兼容）；
//   - 表头 18 列齐全（id/battle_id/user_id/nickname/level_id/reported 四列/
//     recomputed 四列/mismatched_fields/mode/rejected/engine_version/created_at）；
//   - 重算值为 NULL 时导出空串；
//   - mismatched_fields 以分号连接；
//   - 筛选（only_mismatch / level_id / user_id）与列表同口径；
//   - 库故障必须报错而不是静默返回空 CSV。

import (
	"bytes"
	"context"
	"encoding/csv"
	"testing"
)

func TestAdminExportRecomputesCSV(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 造一个用户 + 两条重算记录：一条有不符字段，一条全相符（recomputed 为 NULL）
	guest := "g_rc_exp"
	ts.exec(t, `INSERT INTO users (guest_token, nickname, is_guest, status)
		VALUES ($1,'rc_exp',TRUE,1)`, guest)
	var uid int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM users WHERE guest_token=$1`, guest).Scan(&uid); err != nil {
		t.Fatalf("回读用户 id：%v", err)
	}
	ts.exec(t, `
		INSERT INTO settle_recomputes
		  (battle_id, user_id, level_id, reported_score, reported_kills,
		   reported_duration_ms, reported_hash, recomputed_score, recomputed_kills,
		   recomputed_duration_ms, recomputed_hash, mismatched_fields, mode, rejected, engine_version)
		VALUES ($1,$2,1,100,10,60000,'hh',100,10,60000,'hh',ARRAY['score'],'shadow',FALSE,'skills-only/v1')`,
		uid*100, uid)
	ts.exec(t, `
		INSERT INTO settle_recomputes
		  (battle_id, user_id, level_id, reported_score, reported_kills,
		   reported_duration_ms, reported_hash, mode, rejected, engine_version)
		VALUES ($1,$2,2,200,20,70000,'cc','enforce',TRUE,'full/v1')`,
		uid*100+1, uid)

	data, err := ts.AdminExportRecomputes(ctx, RecomputeFilter{})
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
	if len(records) < 3 {
		t.Fatalf("应有表头 + 至少 2 行数据，实际 %d 行", len(records))
	}

	wantHeader := []string{"id", "battle_id", "user_id", "nickname", "level_id",
		"reported_score", "reported_kills", "reported_duration_ms", "reported_hash",
		"recomputed_score", "recomputed_kills", "recomputed_duration_ms", "recomputed_hash",
		"mismatched_fields", "mode", "rejected", "engine_version", "created_at"}
	if len(records[0]) != len(wantHeader) {
		t.Fatalf("表头列数 = %d，期望 %d：%v", len(records[0]), len(wantHeader), records[0])
	}
	for i, h := range wantHeader {
		if records[0][i] != h {
			t.Errorf("表头第 %d 列 = %q，期望 %q", i, records[0][i], h)
		}
	}

	// 找到有不符字段的行（level_id=1）：recomputed 四列非空、mismatched 为 "score"
	var foundMismatch bool
	for _, rec := range records[1:] {
		if rec[4] == "1" {
			foundMismatch = true
			if rec[13] != "score" {
				t.Errorf("mismatched_fields 应以分号连接，实际 %q", rec[13])
			}
			if rec[9] == "" || rec[10] == "" || rec[11] == "" || rec[12] == "" {
				t.Error("有不符字段的行应带 recomputed 值")
			}
			if rec[2] == "" || rec[3] == "" {
				t.Error("导出应包含玩家昵称")
			}
		}
		// 全相符行（level_id=2）：recomputed 四列为空串
		if rec[4] == "2" {
			if rec[9] != "" || rec[10] != "" || rec[11] != "" || rec[12] != "" {
				t.Errorf("recomputed 为 NULL 时应导出空串，实际 %q/%q/%q/%q",
					rec[9], rec[10], rec[11], rec[12])
			}
			if rec[13] != "" {
				t.Errorf("全相符行的 mismatched_fields 应为空串，实际 %q", rec[13])
			}
			if rec[15] != "true" {
				t.Errorf("rejected 应为 true，实际 %q", rec[15])
			}
		}
	}
	if !foundMismatch {
		t.Error("导出应包含有不符字段的重算行")
	}
}

func TestAdminExportRecomputesFilters(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid := recomputeProbeUser(t, ts, "rc_exp_f")
	ts.exec(t, `
		INSERT INTO settle_recomputes
		  (battle_id, user_id, level_id, reported_score, reported_kills,
		   reported_duration_ms, reported_hash, mismatched_fields, mode, rejected, engine_version)
		VALUES ($1,$2,5,100,10,60000,'hh',ARRAY['score'],'shadow',FALSE,'v1')`,
		uid*100, uid)
	ts.exec(t, `
		INSERT INTO settle_recomputes
		  (battle_id, user_id, level_id, reported_score, reported_kills,
		   reported_duration_ms, reported_hash, mode, rejected, engine_version)
		VALUES ($1,$2,6,200,20,70000,'cc','enforce',TRUE,'v1')`,
		uid*100+1, uid)

	// only_mismatch：只导出有不符字段的行
	data, err := ts.AdminExportRecomputes(ctx, RecomputeFilter{OnlyMismatch: true})
	if err != nil {
		t.Fatalf("only_mismatch 导出失败：%v", err)
	}
	records, _ := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(data, []byte("\xEF\xBB\xBF")))).ReadAll()
	if len(records) != 2 {
		t.Errorf("only_mismatch 导出行数应为 2（1 表头 + 1 数据），实际 %d", len(records))
	}

	// level_id=6：只导出该关
	data, err = ts.AdminExportRecomputes(ctx, RecomputeFilter{LevelID: 6})
	if err != nil {
		t.Fatalf("level_id 导出失败：%v", err)
	}
	records, _ = csv.NewReader(bytes.NewReader(bytes.TrimPrefix(data, []byte("\xEF\xBB\xBF")))).ReadAll()
	if len(records) != 2 {
		t.Errorf("level_id=6 导出行数应为 2，实际 %d", len(records))
	}

	// user_id：只导出该玩家
	data, err = ts.AdminExportRecomputes(ctx, RecomputeFilter{UserID: uid})
	if err != nil {
		t.Fatalf("user_id 导出失败：%v", err)
	}
	records, _ = csv.NewReader(bytes.NewReader(bytes.TrimPrefix(data, []byte("\xEF\xBB\xBF")))).ReadAll()
	if len(records) != 3 {
		t.Errorf("user_id 导出行数应为 3（1 表头 + 2 数据），实际 %d", len(records))
	}
}

func TestAdminExportRecomputesBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, err := ts.AdminExportRecomputes(ctx, RecomputeFilter{}); err == nil {
		t.Fatal("连接池已关闭时导出必须报错，不能静默返回空 CSV")
	}
}