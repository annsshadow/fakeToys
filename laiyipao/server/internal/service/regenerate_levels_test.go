package service

import (
	"context"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// 第 116 轮：AdminRegenerateLevels 的三件事
//  1. 保留运营自定义的 base_hp（与后台确认文案一致，不覆盖）
//  2. 仍把生成器内容（wave_count/difficulty…）同步回来
//  3. 整批原子：中途失败不留「半新半旧」混合态
func TestAdminRegeneratePreservesBaseHPAndResyncsContent(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 第 1 关生成器默认值
	gen := domain.GenerateLevel(1)

	// 运营手工把 base_hp 改成一个绝不可能等于生成器值的数
	const customHP = 888888
	_, err := ts.AdminUpdateLevel(ctx, 1, map[string]any{"base_hp": float64(customHP)})
	if err != nil {
		t.Fatalf("先写入自定义 base_hp 失败：%v", err)
	}
	// 把 wave_count 改成 1（非生成器值），用来验证它会被同步回去
	if _, err := ts.AdminUpdateLevel(ctx, 1, map[string]any{"wave_count": 1.0}); err != nil {
		t.Fatalf("先把 wave_count 改成 1 失败：%v", err)
	}

	// 重新生成
	if _, err := ts.AdminRegenerateLevels(ctx); err != nil {
		t.Fatalf("重新生成失败：%v", err)
	}

	// 1) base_hp 必须保留自定义值（修前这里会被重置成生成器默认值）
	if got := ts.scalar(t, ctx, `SELECT base_hp FROM levels WHERE id = 1`); got != customHP {
		t.Errorf("重新生成应保留运营自定义 base_hp=%d，实际 %d", customHP, got)
	}
	// 2) wave_count 必须被同步回生成器值（不再是运营改的 1）
	if got := ts.scalar(t, ctx, `SELECT wave_count FROM levels WHERE id = 1`); got != gen.WaveCount {
		t.Errorf("重新生成应把 wave_count 同步回生成器值 %d，实际 %d", gen.WaveCount, got)
	}
}

// 整批原子性：让某一关的 UPDATE 必失败（临时把 levels 表改名），
// 事务必须整体回滚——**所有**关卡都保持旧值，不允许半新半旧。
func TestAdminRegenerateIsAllOrNothing(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 快照第 1 关与第 50 关的 wave_count
	before1 := ts.scalar(t, ctx, `SELECT wave_count FROM levels WHERE id = 1`)
	before50 := ts.scalar(t, ctx, `SELECT wave_count FROM levels WHERE id = 50`)

	// 改名 levels 表让 UPDATE 必失败
	ts.renameTable(t, "levels", "levels_gone")
	_, err := ts.AdminRegenerateLevels(ctx)
	if err == nil {
		t.Fatal("levels 表故障时重新生成必须报错（修前逐条 autocommit，失败点之后的关卡已被改）")
	}
	// 复原表
	ts.renameTable(t, "levels_gone", "levels")

	// 原子性：故障回滚后所有关卡保持旧值
	if got := ts.scalar(t, ctx, `SELECT wave_count FROM levels WHERE id = 1`); got != before1 {
		t.Errorf("故障回滚后第 1 关 wave_count 应保持 %d，实际 %d（半新半旧=非原子）", before1, got)
	}
	if got := ts.scalar(t, ctx, `SELECT wave_count FROM levels WHERE id = 50`); got != before50 {
		t.Errorf("故障回滚后第 50 关 wave_count 应保持 %d，实际 %d（半新半旧=非原子）", before50, got)
	}
}

// 第 117 轮：重新生成必须**同步 level_waves**（波次敌人排布表）。
//
// 判据：手工塞一条生成器绝不会产出的「陈波次」(wave_index=999)，
// 重新生成后它必须被删掉，且第 1 关的波次集合 == 生成器当前产出。
// 修前 regenerate 从不碰 level_waves，陈波次会永久残留。
func TestAdminRegenerateSyncsLevelWaves(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	gl := domain.GenerateLevel(1)

	// 塞一条生成器绝不会产出的陈波次
	if _, err := ts.pool.Exec(ctx,
		`INSERT INTO level_waves (level_id, wave_index, spawns)
		 VALUES (1, 999, '[]'::jsonb) ON CONFLICT (level_id, wave_index) DO NOTHING`); err != nil {
		t.Fatalf("预置陈波次失败：%v", err)
	}
	// 把某一波的 spawns 改脏，确认重新生成会覆盖回生成器值
	if _, err := ts.pool.Exec(ctx,
		`UPDATE level_waves SET spawns = '[]'::jsonb WHERE level_id = 1
		 AND wave_index = (SELECT MIN(wave_index) FROM level_waves WHERE level_id = 1)`); err != nil {
		t.Fatalf("改脏波次失败：%v", err)
	}

	if _, err := ts.AdminRegenerateLevels(ctx); err != nil {
		t.Fatalf("重新生成失败：%v", err)
	}

	// 陈波次必须被清掉
	if stale := ts.scalar(t, ctx, `SELECT COUNT(*) FROM level_waves WHERE level_id = 1 AND wave_index = 999`); stale != 0 {
		t.Errorf("重新生成应删掉陈波次(999)，实际仍 %d 条（修前从不碰 level_waves）", stale)
	}

	// 波次总数必须回到生成器值
	if got, want := ts.scalar(t, ctx, `SELECT COUNT(*) FROM level_waves WHERE level_id = 1`), len(gl.Waves); got != want {
		t.Errorf("第 1 关波次数应为生成器值 %d，实际 %d", want, got)
	}

	// 最早一波的 spawns 数必须恢复成生成器内容（不再是改脏的 0）
	if len(gl.Waves) == 0 {
		t.Skipf("第 1 关生成器无波次，无法比对")
	}
	minIdx := gl.Waves[0].Index
	var gotSpawns int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COALESCE(jsonb_array_length(spawns), 0) FROM level_waves
		 WHERE level_id = 1 AND wave_index = $1`, minIdx).Scan(&gotSpawns); err != nil {
		t.Fatalf("读最早波 spawns 长度失败：%v", err)
	}
	if want := len(gl.Waves[0].Spawns); gotSpawns != want {
		t.Errorf("最早波(wave_index=%d) spawns 数应为生成器值 %d，实际 %d（没被同步）", minIdx, want, gotSpawns)
	}
}
