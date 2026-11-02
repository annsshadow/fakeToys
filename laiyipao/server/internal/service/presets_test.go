// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 第 48 轮：L-3 阵型预设。`user_presets` 表建表以来零代码引用，
// 本测试钉住服务层的保存/列表/删除/设当前/归属隔离。

import (
	"context"
	"errors"
	"testing"
)

func TestSaveCurrentPresetCapturesLoadout(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 新用户有默认技能/装备/宝石（initNewUser 铺设）→ 保存当前构筑。
	preset, err := ts.SaveCurrentPreset(ctx, uid, "新手套")
	if err != nil {
		t.Fatalf("保存预设失败：%v", err)
	}
	if preset.Name != "新手套" {
		t.Errorf("预设名应回显，实际 %q", preset.Name)
	}
	// 首个预设自动设为 current（部分唯一索引允许每用户一条 current）。
	if !preset.IsCurrent {
		t.Error("首个预设应自动设为 current")
	}
	if len(preset.Slots.Skills) == 0 {
		t.Error("新号应有出战技能（initNewUser 铺了 3 个槽）")
	}
	if len(preset.Slots.Equipment) == 0 {
		t.Error("新号应有已装配备（initNewUser 送两件且 equipped）")
	}
}

func TestPresetLifecycle(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	if _, err := ts.SaveCurrentPreset(ctx, uid, "A"); err != nil {
		t.Fatalf("保存预设 A 失败：%v", err)
	}
	if _, err := ts.SaveCurrentPreset(ctx, uid, "B"); err != nil {
		t.Fatalf("保存预设 B 失败：%v", err)
	}

	// 第二个预设不是 current（A 仍是）。
	presets, err := ts.ListPresets(ctx, uid)
	if err != nil {
		t.Fatalf("列表失败：%v", err)
	}
	if len(presets) != 2 {
		t.Fatalf("应有 2 个预设，实际 %d", len(presets))
	}
	currentCount := 0
	var presetB PresetView
	for _, p := range presets {
		if p.IsCurrent {
			currentCount++
		}
		if p.Name == "B" {
			presetB = p
		}
	}
	if currentCount != 1 {
		t.Errorf("任意时刻只能有 1 个 current 预设，实际 %d", currentCount)
	}

	// 设 B 为当前：A 的 current 被清掉，B 成为 current。
	updated, err := ts.SetCurrentPreset(ctx, uid, presetB.ID)
	if err != nil {
		t.Fatalf("设当前失败：%v", err)
	}
	if !updated.IsCurrent {
		t.Error("B 应成为 current")
	}
	after, _ := ts.ListPresets(ctx, uid)
	currentIDs := 0
	for _, p := range after {
		if p.IsCurrent {
			currentIDs++
		}
	}
	if currentIDs != 1 {
		t.Errorf("设当前后仍只能有 1 个 current，实际 %d", currentIDs)
	}

	// 删除 B：列表剩 A，B 不可再操作。
	if err := ts.DeletePreset(ctx, uid, presetB.ID); err != nil {
		t.Fatalf("删除失败：%v", err)
	}
	left, _ := ts.ListPresets(ctx, uid)
	if len(left) != 1 || left[0].Name != "A" {
		t.Errorf("删除后应只剩 A，实际 %v", left)
	}
	if _, err := ts.SetCurrentPreset(ctx, uid, presetB.ID); !errors.Is(err, ErrNotFound) {
		t.Errorf("删除后设当前应 ErrNotFound，实际 %v", err)
	}
}

func TestPresetOwnershipIsolation(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	other := ts.newUser(t, ctx)

	preset, err := ts.SaveCurrentPreset(ctx, uid, "私有的")
	if err != nil {
		t.Fatalf("保存预设失败：%v", err)
	}
	// 别人看不到、改不了、删不了。
	list, _ := ts.ListPresets(ctx, other)
	if len(list) != 0 {
		t.Fatalf("别人的预设列表应为空，实际 %v", list)
	}
	if err := ts.DeletePreset(ctx, other, preset.ID); !errors.Is(err, ErrNotFound) {
		t.Errorf("删除他人预设应 ErrNotFound，实际 %v", err)
	}
	if _, err := ts.SetCurrentPreset(ctx, other, preset.ID); !errors.Is(err, ErrNotFound) {
		t.Errorf("设他人预设为当前应 ErrNotFound，实际 %v", err)
	}
}

func TestSaveCurrentPresetValidation(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	if _, err := ts.SaveCurrentPreset(ctx, uid, ""); !errors.Is(err, ErrBadInput) {
		t.Errorf("空预设名应 ErrBadInput，实际 %v", err)
	}
	long := "这是一个非常非常非常非常长的预设名字超过二十个字"
	if _, err := ts.SaveCurrentPreset(ctx, uid, long); !errors.Is(err, ErrBadInput) {
		t.Errorf("超长预设名应 ErrBadInput，实际 %v", err)
	}
}
