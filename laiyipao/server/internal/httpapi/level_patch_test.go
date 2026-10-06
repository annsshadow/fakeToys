package httpapi

import (
	"context"
	"testing"
)

// 关卡 patch 端点：**要么全做，要么全不做**（第 107 轮）。
//
// # 缺陷 A：部分生效 + 成功回执
//
// 修前每个 case 都是 `if 条件 { set(...) }`，条件不满足就什么都不做，
// 而循环继续。于是：
//
//	PUT {"name":"P107改名", "base_hp":-5}  →  200
//	  库里 name 改了，base_hp **没改**
//
// 实测（修前）：
//
//	混合 patch -> status=200  name="P107改名" base_hp=1000（未变）
//	合法+未知键 -> status=200
//	合法+类型错 -> status=200
//
// 运营看到 200 就以为两个字段都调过了。
// 「部分生效 + 成功回执」比直接报错坏得多 —— 直接报错至少会说「你的请求有问题」。
//
// 只有一个键时 `len(fields)==0` 已经能兜住（400「没有可更新的字段」），
// 但**混合**情况兜不住 —— 而混合恰恰是运营调参时最常见的形状。
//
// # 缺陷 B：响应与自己的写入相矛盾
//
// `AdminUpdateLevel` 修前返回 `domain.GenerateLevel(levelID)`，
// 那是**纯函数**（只从 ChapterOf + LCG 算，从不查库）。
//
// 实测（修前）：
//
//	库里 base_hp=987654 name="P107调参关卡"
//	响应里的 level.base_hp=1000 level.name=边境哨站 · 1
//
// 运营刚把 base_hp 改成 987654，响应却回显改**前**的 1000。
//
// 一个与自己的写入矛盾的响应，不管背后是哪条设计路线，都是缺陷。

func TestLevelPatchIsAllOrNothing(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "patch107")

	read := func() (int64, string, int) {
		var hp int64
		var name string
		var waves int
		if err := e.pool.QueryRow(context.Background(),
			`SELECT base_hp, name, wave_count FROM levels WHERE id = 1`).
			Scan(&hp, &name, &waves); err != nil {
			t.Fatalf("读关卡失败：%v", err)
		}
		return hp, name, waves
	}

	// 先把基线定住
	if st, _ := e.put(t, "/api/v1/admin/levels/1", tok, map[string]any{
		"name": "P107基线", "base_hp": 12345.0, "wave_count": 4.0,
	}); st != 200 {
		t.Fatalf("建立基线应 200，实际 %d", st)
	}
	baseHP, baseName, baseWaves := read()
	t.Logf("基线：base_hp=%d name=%q wave_count=%d", baseHP, baseName, baseWaves)

	// 混合：一个合法键 + 一个非法值键
	cases := []struct {
		name  string
		patch map[string]any
	}{
		{"合法 name + 负 base_hp", map[string]any{"name": "P107改了", "base_hp": -5.0}},
		{"合法 name + 字符串 base_hp", map[string]any{"name": "P107改了", "base_hp": "不是数字"}},
		{"合法 name + 未知键", map[string]any{"name": "P107改了", "unknown_key": 1}},
		{"合法 name + 波数超界", map[string]any{"name": "P107改了", "wave_count": 999.0}},
		{"合法 name + enabled 类型错", map[string]any{"name": "P107改了", "enabled": "yes"}},
		{"合法 name + 空 name", map[string]any{"name": "", "base_hp": 1000.0}},
		{"合法 name + star_targets 不是数组", map[string]any{"name": "P107改了", "star_targets": "不是数组"}},
		{"合法 name + terrain_config 是数字", map[string]any{"name": "P107改了", "terrain_config": 42.0}},
		{"合法 name + star_targets 元素是负数", map[string]any{"name": "P107改了", "star_targets": []any{-1.0}}},
	}

	for _, c := range cases {
		st, body := e.put(t, "/api/v1/admin/levels/1", tok, c.patch)
		if st != 400 {
			t.Errorf("%s：应 400，实际 %d：%v", c.name, st, body)
		}
		//
		// ⚠️ 关键断言：**一个字段都不能被写进去**。
		// 只断言状态码是不够的 —— 把 `set("name", …)` 挪到校验之前
		// （也就是「先写再报错」）就照样能返回 400，而 name 已经改了。
		hp, name, waves := read()
		if name != baseName {
			t.Errorf("%s：name 被改成了 %q（应为 %q）—— 整单拒绝必须是**零副作用**",
				c.name, name, baseName)
		}
		if hp != baseHP {
			t.Errorf("%s：base_hp 被改成了 %d（应为 %d）", c.name, hp, baseHP)
		}
		if waves != baseWaves {
			t.Errorf("%s：wave_count 被改成了 %d（应为 %d）", c.name, waves, baseWaves)
		}
	}

	// 全部合法 → 200，且**每一个**字段都写进去了
	st, body := e.put(t, "/api/v1/admin/levels/1", tok, map[string]any{
		"name": "P107全部合法", "base_hp": 54321.0, "wave_count": 6.0,
	})
	if st != 200 {
		t.Fatalf("全部合法应 200，实际 %d：%v", st, body)
	}
	hp, name, waves := read()
	if hp != 54321 || waves != 6 || name != "P107全部合法" {
		t.Errorf("三个字段应全部落库，实际 base_hp=%d name=%q wave_count=%d", hp, name, waves)
	}
}

// TestLevelUpdateResponseMatchesWhatWasWritten 守「响应不得与自己的写入矛盾」。
func TestLevelUpdateResponseMatchesWhatWasWritten(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "resp107")

	const wantHP = 987654
	const wantName = "P107响应一致"
	const wantWaves = 9

	_, body := e.put(t, "/api/v1/admin/levels/1", tok, map[string]any{
		"base_hp": float64(wantHP), "name": wantName, "wave_count": float64(wantWaves),
	})

	lvl := pick(body, "level")
	m, ok := lvl.(map[string]any)
	if !ok {
		t.Fatalf("响应里没有 level 对象：%v", body)
	}

	// 逐项与「我们要求写入的值」比，而不是与库里比 ——
	// 两个来源都用，才能区分「响应没跟上写入」与「写入没生效」。
	want := map[string]float64{
		"base_hp":    wantHP,
		"wave_count": wantWaves,
	}
	for k, v := range want {
		if got := toF(m[k]); got != v {
			t.Errorf("响应里 %s=%v，我们写的是 %v\n"+
				"修前这里返回的是 `domain.GenerateLevel(id)` —— 纯函数，从不查库，\n"+
				"所以它回显的是**改前**的值。", k, got, v)
		}
	}
	if got, _ := m["name"].(string); got != wantName {
		t.Errorf("响应里 name=%q，我们写的是 %q", got, wantName)
	}

	// 并且与库里的值一致（三方一致：请求 / 响应 / 库）
	var dbHP int64
	var dbName string
	if err := e.pool.QueryRow(context.Background(),
		`SELECT base_hp, name FROM levels WHERE id = 1`).Scan(&dbHP, &dbName); err != nil {
		t.Fatalf("读库失败：%v", err)
	}
	if dbHP != wantHP || dbName != wantName {
		t.Fatalf("库里是 base_hp=%d name=%q，与我们写的不一致 —— 那就不是响应的问题", dbHP, dbName)
	}
	if toF(m["base_hp"]) != float64(dbHP) || m["name"] != dbName {
		t.Errorf("响应与库不一致：响应 base_hp=%v name=%v，库 base_hp=%d name=%q",
			m["base_hp"], m["name"], dbHP, dbName)
	}
}

// TestLevelPatchAcceptsEmptyJSONArrays 确认形状校验没有把**合法**输入打死。
//
// ⚠️ `star_targets: null` 看起来像「非法」，但它其实是有意义的清空语义
// （而且实测**不会**撞 NOT NULL —— JSONB 的 null 是合法 JSONB 值，不是 SQL NULL）。
//
// 判据踩在「什么是合法」的分界线上时，要么误杀，要么漏判 ——
// 所以「该接受的」也必须逐条列出来。
func TestLevelPatchAcceptsEmptyJSONArrays(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "shape107")

	for _, patch := range []map[string]any{
		{"star_targets": nil},
		{"star_targets": []any{}},
		{"terrain_config": nil},
		{"terrain_config": []any{}},
		{"terrain_config": []any{map[string]any{"kind": "oil_drum", "x": 1, "y": 2}}},
	} {
		st, body := e.put(t, "/api/v1/admin/levels/1", tok, patch)
		if st != 200 {
			t.Errorf("patch=%v 应 200，实际 %d：%v\n"+
				"「清空」是有意义的语义；形状校验不能把它一起打死。", patch, st, body)
		}
	}
}

// pick 沿键路径取嵌套值；任一层不是对象就返回 nil。
func pick(m map[string]any, keys ...string) any {
	cur := any(m)
	for _, k := range keys {
		mm, ok := cur.(map[string]any)
		if !ok {
			return nil
		}
		cur = mm[k]
	}
	return cur
}

// toF 把 JSON 数值（float64）取出来；取不出给 0。
func toF(v any) float64 {
	f, _ := v.(float64)
	return f
}
