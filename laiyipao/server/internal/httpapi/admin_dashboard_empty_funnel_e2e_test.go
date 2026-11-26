// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// F-1 后端侧（Round 1 基线审计）：零战报时 /admin/dashboard 的
// stage_funnel 是 JSON null（Go nil slice 的编码结果）。而生成的 TS 类型
// 声明 `stage_funnel: StageFunnel[]`（非 null），看板直接 .slice() 消费，
// null 会让整页渲染崩溃。本仓已有 NormalizeNilSlices 响应边界契约：服务端
// 应让 wire 符合自己的类型声明，而不是让前端到处判空。
//
// 在一次性 scratch 库上读真实端点：seeder 只写内容表、不写 battle_records，
// 所以漏斗必然为空。修前 handler 直接 c.JSON(Dashboard{}) → null，RED。
// 同时钉住其它两个数组字段，避免契约漂移留给下一个字段。

import (
	"context"
	"testing"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/config"
)

func TestE2EAdminDashboardEmptyFunnelIsArray(t *testing.T) {
	e2eScratchDB(t) // 干净空库：无 battle_records
	e := newE2E(t)

	var n int
	if err := e.pool.QueryRow(context.Background(), `SELECT COUNT(*) FROM battle_records`).Scan(&n); err != nil {
		t.Fatalf("核对 scratch 库战报数失败：%v", err)
	}
	if n != 0 {
		t.Fatalf("夹具失效：scratch 库有 %d 条战报，零战报场景不可构造", n)
	}

	_, tok := e.newAdmin(t, "dashfunnel")
	status, body := e.get(t, "/api/v1/admin/dashboard", tok)
	if status != fiber.StatusOK {
		t.Fatalf("dashboard 应 200，实际 %d：%v", status, body)
	}
	for _, key := range []string{"stage_funnel", "reaction_usage", "daily_active"} {
		v, ok := body[key]
		if !ok {
			t.Fatalf("dashboard 缺字段 %s（顶层键 %v）", key, keysOf(body))
		}
		if v == nil {
			t.Errorf("%s 编码成 null：前端按数组使用会崩溃（types.gen.ts 声明 T[]）", key)
			continue
		}
		arr, ok := v.([]any)
		if !ok {
			t.Errorf("%s 不是数组，实际 %T", key, v)
			continue
		}
		if key == "stage_funnel" && len(arr) != 0 {
			t.Errorf("零战报时 stage_funnel 应为空数组，实际 %d 行：%v", len(arr), arr)
		}
	}
}

// 归一不能抹掉真实数据：非空漏斗的关卡/场次/胜场必须逐项保留。
func TestE2EAdminDashboardFunnelKeepsRows(t *testing.T) {
	e2eScratchDB(t)
	// 关闭看板缓存：本用例要在同一进程内先写战报再读聚合。
	e := newE2EWith(t, func(c *config.Config) { c.DashboardCacheTTL = 0 })
	uid, _ := e.newPlayer(t, "dashrows")
	_, tok := e.newAdmin(t, "dashrows")
	e.exec(t, `INSERT INTO battle_records (user_id, level_id, result) VALUES ($1, 5, 'win')`, uid)

	status, body := e.get(t, "/api/v1/admin/dashboard", tok)
	if status != fiber.StatusOK {
		t.Fatalf("dashboard 应 200，实际 %d：%v", status, body)
	}
	arr, ok := body["stage_funnel"].([]any)
	if !ok {
		t.Fatalf("stage_funnel 应为数组，实际 %T：%v", body["stage_funnel"], body["stage_funnel"])
	}
	for _, v := range arr {
		row, ok := v.(map[string]any)
		if !ok {
			t.Fatalf("漏斗行不是对象：%T", v)
		}
		if row["level_id"] == float64(5) {
			if row["attempts"] != float64(1) || row["clears"] != float64(1) {
				t.Errorf("第 5 关应 attempts=1 clears=1，实际 %v", row)
			}
			return
		}
	}
	t.Fatalf("漏斗里未见第 5 关：%v", arr)
}
