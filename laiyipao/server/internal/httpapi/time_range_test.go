// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// 时间范围参数语义测试（2026-11-02 第 10 轮）。
//
// 两条行为被固定在这里：
//  1. `date_to` 用日期格式时**包含当天整天**（右端 +1 天，配合服务层 `<` 半开区间）；
//  2. 范围反转/为空直接 400，而不是返回「合法的空集」。

import (
	"encoding/json"
	"net/url"
	"strings"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/httpapi/httperr"
)

// queryTimeRangeVia 走真实 fiber app，避免手搓 Ctx 与真实解码路径分叉。
func queryTimeRangeVia(t *testing.T, rawQuery string) (from, to string, errMsg string) {
	t.Helper()
	app := fiber.New(fiber.Config{ErrorHandler: httperr.Render})
	app.Get("/q", func(c *fiber.Ctx) error {
		f, tt, err := queryTimeRange(c, "from", "to")
		if err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"err": err.Error()})
		}
		out := fiber.Map{}
		if f != nil {
			out["from"] = f.UTC().Format(time.RFC3339)
		}
		if tt != nil {
			out["to"] = tt.UTC().Format(time.RFC3339)
		}
		return c.JSON(out)
	})
	resp, err := app.Test(get("/q"+rawQuery), 5000)
	if err != nil {
		t.Fatalf("请求 %q 失败：%v", rawQuery, err)
	}
	defer resp.Body.Close()
	var body map[string]any
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Fatalf("解析响应失败（status=%d）：%v", resp.StatusCode, err)
	}
	if msg, ok := body["err"].(string); ok {
		return "", "", msg
	}
	f, _ := body["from"].(string)
	tt, _ := body["to"].(string)
	return f, tt, ""
}

func TestQueryTimeRangeDateOnlyEndIncludesWholeDay(t *testing.T) {
	// 同一天作为左右端：左端 00:00，右端次日 00:00（半开区间即包含整天）。
	from, to, errMsg := queryTimeRangeVia(t, "?from=2026-11-02&to=2026-11-02")
	if errMsg != "" {
		t.Fatalf("同一天范围不应报错：%s", errMsg)
	}
	if from != "2026-11-02T00:00:00Z" {
		t.Errorf("from = %q，期望 2026-11-02T00:00:00Z", from)
	}
	if to != "2026-11-03T00:00:00Z" {
		t.Errorf("to = %q，期望 2026-11-03T00:00:00Z（当天整天 + 不含次日零点）", to)
	}
}

func TestQueryTimeRangeRFC3339EndIsNotShifted(t *testing.T) {
	from, to, errMsg := queryTimeRangeVia(t, "?from=2026-11-02T00:00:00Z&to=2026-11-02T12:00:00Z")
	if errMsg != "" {
		t.Fatalf("精确时间范围不应报错：%s", errMsg)
	}
	if to != "2026-11-02T12:00:00Z" {
		t.Errorf("RFC3339 右端被改写了：%q", to)
	}
	if from != "2026-11-02T00:00:00Z" {
		t.Errorf("from = %q", from)
	}
}

func TestQueryTimeRangeRejectsReversedOrEmpty(t *testing.T) {
	cases := []string{
		"?from=2026-11-05&to=2026-11-01",                     // 反转
		"?from=2026-11-02T12:00:00Z&to=2026-11-02T12:00:00Z", // 相等 → 半开区间为空
	}
	for _, q := range cases {
		_, _, errMsg := queryTimeRangeVia(t, q)
		if errMsg == "" {
			t.Errorf("%s 应报错（否则运营会看到合法的空结果）", q)
			continue
		}
		if !strings.Contains(errMsg, "时间范围为空") {
			t.Errorf("%s 的错误文案缺少「时间范围为空」：%s", q, errMsg)
		}
	}
}

func TestQueryTimeRangeAllowsOpenEnds(t *testing.T) {
	// 只给左端：右端为 nil，不报错。
	from, to, errMsg := queryTimeRangeVia(t, "?from=2026-11-02")
	if errMsg != "" || from == "" || to != "" {
		t.Errorf("只给 from 应 from=%q to=%q err=%q", from, to, errMsg)
	}
	// 只给右端：左端 nil，右端按日期格式 +1 天。若误按「左端」语义解析，
	// to 会变成 2026-11-02T00:00:00Z，整天的数据又会被排除。
	from, to, errMsg = queryTimeRangeVia(t, "?to=2026-11-02")
	if errMsg != "" || from != "" || to != "2026-11-03T00:00:00Z" {
		t.Errorf("只给 to 应 from=%q to=%q err=%q", from, to, errMsg)
	}
	// 显式空串 = 未给。
	_, _, errMsg = queryTimeRangeVia(t, "?from=&to=")
	if errMsg != "" {
		t.Errorf("空串应按未给处理，实际报错：%s", errMsg)
	}
}

func TestQueryTimeRangeRejectsBadFormat(t *testing.T) {
	_, _, errMsg := queryTimeRangeVia(t, "?from=not-a-date&to=2026-11-02")
	if errMsg == "" || !strings.Contains(errMsg, "不是合法日期") {
		t.Errorf("非法格式应报「不是合法日期」，实际：%q", errMsg)
	}
}

// E2E：`date_to=<今天>` 必须包含今天整天。
//
// 修前 date_to 按当天 00:00 传入，`created_at < 00:00` 会漏掉今天全部记录，
// 而响应仍是 200 + total=0 —— 运营据此认为「今天没有战报」。
func TestE2EAdminBattlesDateToIncludesWholeDay(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "daterange")
	uid, _ := e.newPlayer(t, "daterange")
	e.exec(t, `INSERT INTO battle_records (user_id, level_id, result, stars, replay_hash, created_at)
	           VALUES ($1, 1, 'win', 1, 'dr1', $2)`, uid, time.Now().UTC())

	today := time.Now().UTC().Format("2006-01-02")
	status, body := e.get(t,
		"/api/v1/admin/battles?date_from="+url.QueryEscape(today)+"&date_to="+url.QueryEscape(today),
		adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("应 200，实际 %d %v", status, body)
	}
	if total, _ := body["total"].(float64); total < 1 {
		t.Fatalf("date_to=%s 应包含今天写入的战报，实际 total=%v（整天被排除）", today, body["total"])
	}
}

// E2E：反转的时间范围在列表与导出两条路径都必须 400。
// 只测列表会漏导出 —— 它的参数解析是独立的一段代码。
func TestE2EAdminReversedRangeIsBadInput(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "datebad")

	for _, path := range []string{
		"/api/v1/admin/battles?date_from=2026-11-05&date_to=2026-11-01",
		"/api/v1/admin/battles/export?date_from=2026-11-05&date_to=2026-11-01",
		"/api/v1/admin/users?date_from=2026-11-05&date_to=2026-11-01",
		"/api/v1/admin/users/export?date_from=2026-11-05&date_to=2026-11-01",
	} {
		status, body := e.get(t, path, adminTok)
		if status != fiber.StatusBadRequest {
			t.Errorf("%s 应 400，实际 %d %v", path, status, body)
			continue
		}
		errObj, _ := body["error"].(map[string]any)
		if code, _ := errObj["code"].(string); code != "bad_input" {
			t.Errorf("%s 错误码应为 bad_input，实际 %q（%v）", path, code, body)
		}
		msg, _ := errObj["message"].(string)
		if !strings.Contains(msg, "时间范围为空") {
			t.Errorf("%s 文案缺少「时间范围为空」：%q", path, msg)
		}
	}
}
