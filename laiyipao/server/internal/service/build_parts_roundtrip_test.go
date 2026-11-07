// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R148：构筑读路径（build_load_pre）的往返守卫。
//
// # 为什么要有这条守卫
//
// 改前 LoadBuildSnapshot 是 5 条**串行**读（技能 → 装备 → 专精节点 →
// 进度 → 宝石），每次都单独从连接池取连接。
// 1M 库实测（c=8）：build_load_pre 段 6.76ms；而 pg_stat_statements 里
// 这五条 SQL 的执行时间合计只有 0.199ms —— 差额全是 5 次 Acquire + 往返。
//
// R148 把它们压进一次 pgx.Batch：一个连接、一个往返。
// 这条用例把「一次构筑读 = 一次 SendBatch、批内恰好 5 条」钉住，并用
// 串行旧方法的结果做语义对照。
//
// ⚠️ 破坏实验（改前应红、改后绿）：
// 把 loadBuildPartsOn 改回逐条查询 / 拆成 5 次 SendBatch，batches 会变 5。

import (
	"context"
	"reflect"
	"testing"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

type countingBatchSender struct {
	*pgxpool.Conn
	batches int
	lens    []int
}

func (c *countingBatchSender) SendBatch(ctx context.Context, b *pgx.Batch) pgx.BatchResults {
	c.batches++
	c.lens = append(c.lens, b.Len())
	return c.Conn.SendBatch(ctx, b)
}

// TestBuildPartsOneBatchRoundTrip：一次构筑读只发一次 SendBatch（往返 1），
// 且批内恰好 5 条语句；结果与逐条串行读逐字段一致。
func TestBuildPartsOneBatchRoundTrip(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 让数据非空，避免「空结果 vs 空结果」把顺序错位掩盖掉。
	ts.exec(t, `UPDATE user_progress SET max_stage = 12, mastery_points = 7 WHERE user_id = $1`, uid)
	ts.exec(t, `INSERT INTO user_gems (user_id, gem_id, quality, affixes, equipped_slot)
	            VALUES ($1, 1, 'blue', $2, 'slot0')`,
		uid, []byte(`[{"affix":"attack","value":50}]`))

	// 参照值：改前的串行读法（方法仍在，供对照）。
	var want buildParts
	var err error
	want.skills, want.elements, want.occupied, err = ts.loadSkillsRows(ctx, uid)
	if err != nil {
		t.Fatalf("串行读技能失败：%v", err)
	}
	want.equipment, err = ts.loadEquipmentRows(ctx, uid)
	if err != nil {
		t.Fatalf("串行读装备失败：%v", err)
	}
	want.masteryNodes, err = ts.loadMasteryNodeRows(ctx, uid)
	if err != nil {
		t.Fatalf("串行读专精失败：%v", err)
	}
	if err := ts.pool.QueryRow(ctx, buildProgressSQL, uid).Scan(&want.maxStage, &want.masteryPoints); err != nil {
		t.Fatalf("串行读进度失败：%v", err)
	}
	gemRaw, err := ts.loadGemsAffixRows(ctx, uid)
	if err != nil {
		t.Fatalf("串行读宝石失败：%v", err)
	}
	want.loadout = applyLoadoutContribution(want.equipment, gemRaw)

	conn, err := ts.pool.Acquire(ctx)
	if err != nil {
		t.Fatalf("取连接失败：%v", err)
	}
	cs := &countingBatchSender{Conn: conn}
	got, err := loadBuildPartsOn(ctx, cs, uid)
	conn.Release()
	if err != nil {
		t.Fatalf("批量读失败：%v", err)
	}

	if cs.batches != 1 {
		t.Errorf("一次构筑读应只发 1 次 SendBatch（= 1 个往返），实际 %d 次："+
			"回到逐条查询时这里会变成 5。", cs.batches)
	}
	if len(cs.lens) != 1 || cs.lens[0] != 5 {
		t.Errorf("批内语句数应为 5，实际 %v", cs.lens)
	}
	if !reflect.DeepEqual(want, got) {
		t.Errorf("批量读与串行读结果不一致：\n串行=%+v\n批量=%+v", want, got)
	}
}
