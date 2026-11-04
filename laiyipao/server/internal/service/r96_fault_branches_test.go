// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 本文件补的是「数据坏掉 / 表没了」时的**响不响**，而不是正常路径。
//
// 这些分支平时没人走，但它们的产物是运营最需要的那种错误：
// 「这一条读不全」而不是「0 条，看起来一切正常」。
// 漏掉它们的典型症状：后台/客户端把坏数据展示成「这一局没用元素」
// 「这个玩家没有预设」「这个玩家没有流水」。

import (
	"context"
	"errors"
	"fmt"
	"strings"
	"testing"
)

// TestMyBattleDetailRejectsCorruptJSONB 四个 jsonb 列坏任何一个都必须报错。
func TestMyBattleDetailRejectsCorruptJSONB(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	userID := ts.newUser(t, ctx)
	var levelID int
	if err := ts.pool.QueryRow(ctx, `SELECT id FROM levels ORDER BY id LIMIT 1`).Scan(&levelID); err != nil {
		t.Fatalf("读关卡 id 失败：%v", err)
	}
	var battleID int64
	if err := ts.pool.QueryRow(ctx,
		`INSERT INTO battle_records (user_id, level_id, result) VALUES ($1, $2, 'win') RETURNING id`,
		userID, levelID).Scan(&battleID); err != nil {
		t.Fatalf("建战报失败：%v", err)
	}

	cases := []struct{ col, bad, want string }{
		{"elements_used", `'[]'::jsonb`, "elements_used"},
		{"reactions_used", `'[]'::jsonb`, "reactions_used"},
		{"terrain_used", `'{}'::jsonb`, "terrain_used"},
		{"loot", `'[]'::jsonb`, "loot"},
	}
	for _, c := range cases {
		t.Run(c.col, func(t *testing.T) {
			ts.exec(t, `UPDATE battle_records SET `+c.col+` = `+c.bad+` WHERE id = $1`, battleID)
			_, err := ts.MyBattleDetail(ctx, userID, battleID)
			if err == nil {
				t.Fatalf("%s 填了错误形状的 jsonb，详情却静默返回 200", c.col)
			}
			if !strings.Contains(err.Error(), c.want) || !strings.Contains(err.Error(), "非法 jsonb") {
				t.Fatalf("错误应点明 %s 非法 jsonb，实际：%v", c.want, err)
			}
			// 复原，避免后续子用例读到上一次的坏值。
			good := `'{}'::jsonb`
			if c.col == "terrain_used" {
				good = `'[]'::jsonb`
			}
			ts.exec(t, `UPDATE battle_records SET `+c.col+` = `+good+` WHERE id = $1`, battleID)
		})
	}
}

// TestClaimOfflineRewardHandlesFirstClaimAndEmptyPool 首次领取要建行；
// 再领一次必须给 0 且不重复发币。
func TestClaimOfflineRewardHandlesFirstClaimAndEmptyPool(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	userID := ts.newUser(t, ctx)

	first, err := ts.ClaimOfflineReward(ctx, userID)
	if err != nil {
		t.Fatalf("首次领取失败：%v", err)
	}
	if first == nil {
		t.Fatal("首次领取返回 nil")
	}

	second, err := ts.ClaimOfflineReward(ctx, userID)
	if err != nil {
		t.Fatalf("二次领取失败：%v", err)
	}
	var total int64
	for _, v := range second {
		if v < 0 {
			t.Fatalf("二次领取出现负数：%v", second)
		}
		total += v
	}
	if total != 0 {
		t.Fatalf("刚领完再领应为 0，实际 %d（%v）", total, second)
	}
}

// TestPresetsLimitAndCorruptSlots 预设上限与坏 JSONB：
//   - 满 20 个后再存必须 400，不能悄悄突破；
//   - slots 被外部改成非对象形状时，列表必须报错而不是给空构筑。
func TestPresetsLimitAndCorruptSlots(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	userID := ts.newUser(t, ctx)

	for i := 0; i < maxPresetsPerUser; i++ {
		ts.exec(t, `
			INSERT INTO user_presets (user_id, name, slots, is_current)
			VALUES ($1, $2, '{"skills":[],"equipment":[],"gems":[]}'::jsonb, false)`,
			userID, fmt.Sprintf("预设%02d", i))
	}

	_, err := ts.SaveCurrentPreset(ctx, userID, "第 21 个")
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("超过预设上限应返回 ErrBadInput，实际 %v", err)
	}
	if !strings.Contains(err.Error(), "最多保存") {
		t.Fatalf("错误应说明上限，实际：%v", err)
	}

	// 把其中一行的 slots 改坏：外部脚本/迁移写坏 JSONB 是这个包的经典事故。
	ts.exec(t, `UPDATE user_presets SET slots = '[]'::jsonb WHERE user_id = $1 AND name = '预设00'`, userID)
	if _, err := ts.ListPresets(ctx, userID); err == nil {
		t.Fatal("slots 是数组时列表应报错，实际静默返回")
	} else if !strings.Contains(err.Error(), "非法 jsonb") {
		t.Fatalf("错误应点明非法 jsonb，实际：%v", err)
	}
}

// TestPresetsFailLoudWhenTablesAreGone 三段查询各自坏掉时都必须响。
//
// currentPresetSlots 是「技能 → 装备 → 宝石」三段顺序查询；只覆盖第一段
// 的失败会让另外两段在改表/改列后静默返回半份构筑。
func TestPresetsFailLoudWhenTablesAreGone(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	userID := ts.newUser(t, ctx)

	// 第一段：技能槽
	ts.renameTable(t, "user_skill_slots", "user_skill_slots_gone")
	if _, err := ts.SaveCurrentPreset(ctx, userID, "技能段坏"); err == nil {
		t.Fatal("user_skill_slots 不存在时应报错")
	}

	// 第二段：装备
	ts.renameTable(t, "user_equipment", "user_equipment_gone")
	if _, err := ts.SaveCurrentPreset(ctx, userID, "装备段坏"); err == nil {
		t.Fatal("user_equipment 不存在时应报错")
	}

	// 第三段：宝石
	ts.renameTable(t, "user_gems", "user_gems_gone")
	if _, err := ts.SaveCurrentPreset(ctx, userID, "宝石段坏"); err == nil {
		t.Fatal("user_gems 不存在时应报错")
	}

	// 存储段：预设表本身
	ts.renameTable(t, "user_presets", "user_presets_gone")
	if _, err := ts.ListPresets(ctx, userID); err == nil {
		t.Fatal("user_presets 不存在时列表应报错")
	}
	if _, err := ts.SaveCurrentPreset(ctx, userID, "存储段坏"); err == nil {
		t.Fatal("user_presets 不存在时应报错")
	}
	if err := ts.DeletePreset(ctx, userID, 1); err == nil {
		t.Fatal("user_presets 不存在时删除应报错")
	}
	if _, err := ts.SetCurrentPreset(ctx, userID, 1); err == nil {
		t.Fatal("user_presets 不存在时置位应报错")
	}

	// 宝石列表同样不能把「表没了」当成「没有宝石」。
	if _, err := ts.ListGems(ctx, userID); err == nil {
		t.Fatal("user_gems 不存在时宝石列表应报错")
	}
}

// TestShopAndOfflineTablesFailLoud 商店/离线收益的读路径同样不许静默降级。
func TestShopAndOfflineTablesFailLoud(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	userID := ts.newUser(t, ctx)

	ts.renameTable(t, "shop_items", "shop_items_gone")
	if _, err := ts.LoadShop(ctx, userID); err == nil {
		t.Fatal("shop_items 不存在时商店应报错")
	} else if !strings.Contains(err.Error(), "load shop") {
		t.Fatalf("错误应点明 load shop，实际：%v", err)
	}

	ts.renameTable(t, "offline_rewards", "offline_rewards_gone")
	if _, err := ts.ClaimOfflineReward(ctx, userID); err == nil {
		t.Fatal("offline_rewards 不存在时领取应报错")
	}
}
