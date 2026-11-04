// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"errors"
	"strings"
	"testing"
)

// TestAdminBattleDetailDistinguishesMissingFromBroken 钉住「不存在」与「查不了」
// 的区分。
//
// # 为什么这条要单独写
//
// 运营端此前走 `AdminGetReplay`，而后者把**任何**错误都报成 `ErrNotFound` ——
// 那是**玩家侧**的防预言机措施（不能让人二分出「某个 id 是否存在」）。
//
// 运营端本来就能看全部战报列表，**没有预言机要防**；于是库一挂，
// 运营看到的是「战报不存在」—— 一个**假的**答案。
// 「服务异常」与「这条记录没了」在处置上完全不同（前者等库恢复，后者查数据）。
//
// 判据：真的没有这一行 → ErrNotFound（404）；库坏 → 不是 ErrNotFound（500）。
func TestAdminBattleDetailDistinguishesMissingFromBroken(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 1) 真的不存在 → ErrNotFound
	_, err := ts.AdminBattleDetail(ctx, 999_999_999)
	if !errors.Is(err, ErrNotFound) {
		t.Errorf("不存在的战报应 ErrNotFound，实际 %v", err)
	}

	// 2) 库坏 → **不能**说成 ErrNotFound
	broken := openBrokenService(t)
	_, err = broken.AdminBattleDetail(ctx, 1)
	if err == nil {
		t.Fatal("数据库故障时应报错")
	}
	if errors.Is(err, ErrNotFound) {
		t.Errorf("数据库故障被说成了「战报不存在」（%v）—— 那是**假的**答案，"+
			"运营会以为记录被删了", err)
	}
}

// TestAdminBattleDetailDecodesReplayFieldsAndSeed 钉住详情抽屉的取证字段。
//
// 为什么这条重要：此前该端点返回的是 `ReplayInfo`，前端按 `AdminBattle` 读，
// 于是 **14/16 个字段恒空** —— 运营看到的是一片空白，而不是报错。
// 这里逐项断言「数据库里存的」确实被读了出来：
//   - 三个 jsonb 列必须解码成 map/切片（默认值 '{}' / '[]' 也要能解）
//   - seed 必须以**字符串**下发（int64 超 2^53 后 JS Number 会失真）
func TestAdminBattleDetailDecodesReplayFieldsAndSeed(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	tag := "r96-battle-detail"
	ts.exec(t, `INSERT INTO users (guest_token, nickname) VALUES ($1, $2)`, tag, "R96详情")

	var (
		userID  int64
		levelID int
	)
	if err := ts.pool.QueryRow(ctx, `SELECT id FROM users WHERE guest_token = $1`, tag).Scan(&userID); err != nil {
		t.Fatalf("读用户 id 失败：%v", err)
	}
	if err := ts.pool.QueryRow(ctx, `SELECT id FROM levels ORDER BY id LIMIT 1`).Scan(&levelID); err != nil {
		t.Fatalf("读关卡 id 失败：%v", err)
	}

	// 种子取一个绝对值超过 2^53 的负数：既验证负数，也验证「字符串下发」的必要性。
	const seed int64 = -7046029255919282421
	var tokenID int64
	if err := ts.pool.QueryRow(ctx, `
		INSERT INTO battle_tokens (user_id, level_id, seed, expires_at)
		VALUES ($1, $2, $3, now() + interval '10 minutes') RETURNING id`,
		userID, levelID, seed).Scan(&tokenID); err != nil {
		t.Fatalf("建 battle_token 失败：%v", err)
	}

	var battleID int64
	if err := ts.pool.QueryRow(ctx, `
		INSERT INTO battle_records
		  (user_id, level_id, battle_token_id, result, stars, score, power_after,
		   kills, leaked, hp_left, wave_reached, duration_ms, shots, hits,
		   reactions, heat_max, elements_used, reactions_used, terrain_used, replay_hash)
		VALUES ($1, $2, $3, 'win', 3, 8800, 4321,
		        17, 1, 88, 12, 61000, 40, 31,
		        9, 77, '{"fire":12,"ice":3}', '{"steam_burst":3}', '["fire_zone"]', 'deadbeefdeadbeef')
		RETURNING id`, userID, levelID, tokenID).Scan(&battleID); err != nil {
		t.Fatalf("建战报失败：%v", err)
	}

	b, err := ts.AdminBattleDetail(ctx, battleID)
	if err != nil {
		t.Fatalf("读战报详情失败：%v", err)
	}
	if b.ID != battleID || b.UserID != userID || b.Nickname != "R96详情" {
		t.Fatalf("归属字段不符：%+v", b)
	}
	if b.Result != "win" || b.Stars != 3 || b.Score != 8800 || b.PowerAfter != 4321 {
		t.Fatalf("结算字段不符：%+v", b)
	}
	if b.ElementsUsed["fire"] != 12 || b.ElementsUsed["ice"] != 3 {
		t.Fatalf("elements_used 未解码：%v", b.ElementsUsed)
	}
	if b.ReactionsUsed["steam_burst"] != 3 {
		t.Fatalf("reactions_used 未解码：%v", b.ReactionsUsed)
	}
	if len(b.TerrainUsed) != 1 || b.TerrainUsed[0] != "fire_zone" {
		t.Fatalf("terrain_used 未解码：%v", b.TerrainUsed)
	}
	if b.Seed != "-7046029255919282421" {
		t.Fatalf("seed 必须以字符串原样下发，实际 %q", b.Seed)
	}
	if b.CreatedAt.IsZero() {
		t.Fatal("created_at 未读出")
	}

	// 坏 jsonb 必须响：静默给空会让运营以为「这一局没用任何元素」。
	ts.exec(t, `UPDATE battle_records SET elements_used = '[]'::jsonb WHERE id = $1`, battleID)
	_, err = ts.AdminBattleDetail(ctx, battleID)
	if err == nil {
		t.Fatal("elements_used 是数组时应报错，实际静默通过")
	}
	if !strings.Contains(err.Error(), "非法 jsonb") {
		t.Fatalf("错误信息应点明非法 jsonb，实际：%v", err)
	}
	if errors.Is(err, ErrNotFound) {
		t.Fatalf("坏数据被说成了「战报不存在」：%v", err)
	}
}
