// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 第 107 轮契约的回归测试：白名单字段**逐项**校验，非法键整单拒绝。
//
// 上一版实现是 `if 条件 { set(...) }` —— 条件不满足就跳过、循环继续，
// 于是 `{"name":"改","base_hp":-5}` 返回 200：名字改了、血量没改，
// 运营看到 200 以为两项都调过了。本文件把「每一个拒绝理由」和
// 「混合请求不得部分生效」都钉住。

import (
	"context"
	"encoding/json"
	"errors"
	"strings"
	"testing"
)

func TestAdminUpdateLevelRejectsEachBadField(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 从 JSON 解码出 patch，与 handler 拿到的形状一致（数字是 float64）。
	decode := func(t *testing.T, raw string) map[string]any {
		t.Helper()
		var patch map[string]any
		if err := json.Unmarshal([]byte(raw), &patch); err != nil {
			t.Fatalf("测试输入不是合法 JSON：%v", err)
		}
		return patch
	}

	cases := []struct{ name, raw, want string }{
		{"名字为空串", `{"name":""}`, "必须是非空字符串"},
		{"名字不是字符串", `{"name":42}`, "必须是非空字符串"},
		{"血量非正", `{"base_hp":0}`, "必须是正整数"},
		{"波次为 0", `{"wave_count":0}`, "必须在 1..50"},
		{"波次超上限", `{"wave_count":51}`, "必须在 1..50"},
		{"难度非正", `{"difficulty":0}`, "必须是正整数"},
		{"体力超上限", `{"energy_cost":101}`, "必须在 0..100"},
		{"体力为负", `{"energy_cost":-1}`, "必须在 0..100"},
		{"星级目标不是数组", `{"star_targets":"不是数组"}`, "必须是数组或 null"},
		{"星级目标含负数", `{"star_targets":[1,-2]}`, "数组元素必须是非负整数"},
		{"星级目标含非数字", `{"star_targets":[true]}`, "数组元素必须是非负整数"},
		{"地形配置不是数组", `{"terrain_config":42}`, "必须是数组或 null"},
		{"地形配置元素不是对象", `{"terrain_config":[1]}`, "数组元素必须是对象"},
		{"enabled 不是布尔", `{"enabled":"yes"}`, "必须是布尔值"},
		{"is_boss 不是布尔", `{"is_boss":1}`, "必须是布尔值"},
		{"未知键（拼错字段名）", `{"baseHP":100}`, "不在可更新字段的白名单里"},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			_, err := ts.AdminUpdateLevel(ctx, 1, decode(t, c.raw))
			if !errors.Is(err, ErrBadInput) {
				t.Fatalf("应返回 ErrBadInput，实际 %v", err)
			}
			if !strings.Contains(err.Error(), c.want) {
				t.Fatalf("错误应点明「%s」，实际：%v", c.want, err)
			}
		})
	}
}

// TestAdminUpdateLevelMixedPatchIsAllOrNothing 混合请求不得「部分生效」。
//
// 这是第 107 轮真正修的那个缺陷：单个非法键 + 单个合法键时，
// 旧实现只报「没有可更新字段」都兜不住（合法键让 fields 非空），
// 于是返回 200 且只有合法键落库。这里断言**数据库逐字未变**。
func TestAdminUpdateLevelMixedPatchIsAllOrNothing(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	var beforeName string
	var beforeHP int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT name, base_hp FROM levels WHERE id = 1`).Scan(&beforeName, &beforeHP); err != nil {
		t.Fatalf("读改前配置失败：%v", err)
	}

	var patch map[string]any
	if err := json.Unmarshal([]byte(`{"name":"R96改名","base_hp":-5}`), &patch); err != nil {
		t.Fatalf("构造 patch 失败：%v", err)
	}
	_, err := ts.AdminUpdateLevel(ctx, 1, patch)
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("混合请求应整单拒绝，实际 %v", err)
	}
	if !strings.Contains(err.Error(), "整单未执行") {
		t.Fatalf("错误应说明整单未执行，实际：%v", err)
	}

	var afterName string
	var afterHP int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT name, base_hp FROM levels WHERE id = 1`).Scan(&afterName, &afterHP); err != nil {
		t.Fatalf("读改后配置失败：%v", err)
	}
	if afterName != beforeName {
		t.Fatalf("合法的 name 被写进了库（%q → %q）—— 混合请求部分生效", beforeName, afterName)
	}
	if afterHP != beforeHP {
		t.Fatalf("base_hp 被改动（%d → %d）", beforeHP, afterHP)
	}
}

// TestAdminUpdateLevelAcceptsExplicitNull 显式 null 是「清空」语义，必须放行。
//
// 形状校验的第一版一刀切 `val.([]any)`，于是「清空星级目标」变成 400 ——
// 那是把运营真会做的操作误杀。
func TestAdminUpdateLevelAcceptsExplicitNull(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	var patch map[string]any
	if err := json.Unmarshal([]byte(`{"star_targets":null,"terrain_config":null}`), &patch); err != nil {
		t.Fatalf("构造 patch 失败：%v", err)
	}
	row, err := ts.AdminUpdateLevel(ctx, 1, patch)
	if err != nil {
		t.Fatalf("显式 null 应被接受，实际 %v", err)
	}
	if row.ID != 1 {
		t.Fatalf("回执的关卡 id 不符：%d", row.ID)
	}

	// 共享库卫生：把这两列还原成空数组。
	ts.exec(t, `UPDATE levels SET star_targets='[]'::jsonb, terrain_config='[]'::jsonb WHERE id = 1`)
}
