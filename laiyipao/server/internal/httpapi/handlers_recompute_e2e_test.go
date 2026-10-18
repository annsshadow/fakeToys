package httpapi

// 结算重算审计端点的端到端守卫（R1）。
//
// 判据落在**响应体的实际形状与数值**上，而不是「端点返回 200」——
// 后者一个永远返回 `{}` 的空实现也能通过，而后台页面正是照着这些字段渲染的。

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

// seedRecompute 造一行台账。返回该用户 id。
func (e *e2e) seedRecompute(t *testing.T, tag string, levelID int, mismatched bool, mode string, rejected bool) (int64, int64) {
	t.Helper()
	guest := fmt.Sprintf("g_rc_%s_%d", tag, time.Now().UnixNano()%1_000_000)
	var uid int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO users (guest_token, nickname, is_guest, status) VALUES ($1,$2,TRUE,1) RETURNING id`,
		guest, tag).Scan(&uid); err != nil {
		t.Fatalf("建用户失败：%v", err)
	}
	fields := []string{}
	if mismatched {
		fields = []string{"score"}
	}
	var rid int64
	if err := e.pool.QueryRow(context.Background(), `
		INSERT INTO settle_recomputes
		  (battle_id, user_id, level_id, reported_score, reported_kills,
		   reported_duration_ms, reported_hash, mismatched_fields, mode, rejected, engine_version)
		VALUES (0,$1,$2,100,10,60000,'h',$3,$4,$5,'skills-only/v1') RETURNING id`,
		uid, levelID, fields, mode, rejected).Scan(&rid); err != nil {
		t.Fatalf("写台账失败：%v", err)
	}
	t.Cleanup(func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `DELETE FROM settle_recomputes WHERE id = $1`, rid)
		_, _ = e.pool.Exec(ctx, `DELETE FROM users WHERE id = $1`, uid)
	})
	return uid, rid
}

// seedRecomputeFor 给**已存在**的用户再写一行台账。
//
// 与 seedRecompute 分开是因为「同一个玩家在多局里既有相符又有不符」
// 正是 `max_user_mismatches` 那条守卫要构造的形状 ——
// 用 seedRecompute 造不出「同一玩家」这个前提。
func (e *e2e) seedRecomputeFor(t *testing.T, uid int64, levelID int, mismatched bool) {
	t.Helper()
	fields := []string{}
	if mismatched {
		fields = []string{"score"}
	}
	var rid int64
	if err := e.pool.QueryRow(context.Background(), `
		INSERT INTO settle_recomputes
		  (battle_id, user_id, level_id, reported_score, reported_kills,
		   reported_duration_ms, reported_hash, mismatched_fields, mode, rejected, engine_version)
		VALUES (0,$1,$2,100,10,60000,'h',$3,'shadow',FALSE,'skills-only/v1') RETURNING id`,
		uid, levelID, fields).Scan(&rid); err != nil {
		t.Fatalf("写台账失败：%v", err)
	}
	t.Cleanup(func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `DELETE FROM settle_recomputes WHERE id = $1`, rid)
	})
}

func TestE2EAdminRecomputesList(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "rcl")
	_, _ = e.seedRecompute(t, "mism", 5, true, "enforce", true)
	_, _ = e.seedRecompute(t, "clean", 6, false, "shadow", false)

	// 全量：两行都在
	status, body := e.get(t, "/api/v1/admin/recomputes?limit=100", tok)
	if status != 200 {
		t.Fatalf("GET /admin/recomputes 应 200，实际 %d %v", status, body)
	}
	items := body["items"].([]any)
	if len(items) < 2 {
		t.Fatalf("应至少 2 行，实际 %d", len(items))
	}

	// 逐字段断言形状 —— 页面按这些 key 渲染，缺一个就是空白格
	first := items[0].(map[string]any)
	for _, k := range []string{
		"id", "battle_id", "user_id", "nickname", "level_id",
		"reported", "recomputed", "mismatched_fields",
		"mode", "rejected", "engine_version", "created_at",
	} {
		if _, ok := first[k]; !ok {
			t.Errorf("响应缺少字段 %q", k)
		}
	}
	if _, ok := first["reported"].(map[string]any)["score"]; !ok {
		t.Error("reported 必须是 {score,kills,duration_ms,hash} 形状")
	}
	if first["recomputed"] != nil {
		t.Error("引擎未接入时 recomputed 应为 null，而不是一个零值对象")
	}

	// only_mismatch 过滤
	status, body = e.get(t, "/api/v1/admin/recomputes?only_mismatch=1&limit=100", tok)
	if status != 200 {
		t.Fatalf("only_mismatch 应 200，实际 %d %v", status, body)
	}
	for _, it := range body["items"].([]any) {
		f := it.(map[string]any)["mismatched_fields"].([]any)
		if len(f) == 0 {
			t.Errorf("only_mismatch=1 返回了无不符字段的行 id=%v", it.(map[string]any)["id"])
		}
	}
}

func TestE2EAdminRecomputesListFiltersAndValidation(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "rcf")
	uid, _ := e.seedRecompute(t, "byuser", 7, true, "enforce", true)

	status, body := e.get(t, fmt.Sprintf("/api/v1/admin/recomputes?user_id=%d", uid), tok)
	if status != 200 {
		t.Fatalf("按 user_id 筛选应 200，实际 %d %v", status, body)
	}
	items := body["items"].([]any)
	found := false
	for _, it := range items {
		if int64(it.(map[string]any)["user_id"].(float64)) == uid {
			found = true
		}
	}
	if !found {
		t.Errorf("按 user_id=%d 筛选没命中自己刚写的那行", uid)
	}

	// level_id 也要能用
	status, body = e.get(t, "/api/v1/admin/recomputes?level_id=7", tok)
	if status != 200 {
		t.Fatalf("按 level_id 筛选应 200，实际 %d %v", status, body)
	}

	// ⚠️ 非法参数必须 400，不能静默当成「不筛选」。
	//
	// 静默忽略的话症状是「筛了没反应」，运营会以为是数据被清空了 ——
	// 而实际是参数名打错了。
	for _, url := range []string{
		"/api/v1/admin/recomputes?user_id=abc",
		"/api/v1/admin/recomputes?level_id=xyz",
		"/api/v1/admin/recomputes?limit=abc",
		"/api/v1/admin/recomputes?offset=abc",
	} {
		if st, b := e.get(t, url, tok); st != fiber.StatusBadRequest {
			t.Errorf("GET %s 应 400，实际 %d %v", url, st, b)
		}
	}
}

func TestE2EAdminRecomputesRequiresAdmin(t *testing.T) {
	e := newE2E(t)
	_, playerTok := e.newPlayer(t, "rcguard")

	// 无令牌 / 玩家令牌都必须 401 —— 台账里全是「谁在作弊」，
	// 泄露它等于把侦察能力送给对手。
	for _, tok := range []string{"", playerTok} {
		if st, _ := e.get(t, "/api/v1/admin/recomputes", tok); st != fiber.StatusUnauthorized {
			t.Errorf("无管理员身份打 /admin/recomputes 应 401，实际 %d", st)
		}
		if st, _ := e.get(t, "/api/v1/admin/recomputes/stats", tok); st != fiber.StatusUnauthorized {
			t.Errorf("无管理员身份打 /admin/recomputes/stats 应 401，实际 %d", st)
		}
	}
}

func TestE2EAdminRecomputeStats(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "rcs")
	_, _ = e.seedRecompute(t, "lvlA", 11, true, "enforce", true)
	_, _ = e.seedRecompute(t, "lvlB", 12, false, "enforce", false)

	status, body := e.get(t, "/api/v1/admin/recomputes/stats?hours=24", tok)
	if status != 200 {
		t.Fatalf("GET stats 应 200，实际 %d %v", status, body)
	}

	// 闸门状态必须出现，且区分「配置值」与「生效值」
	for _, k := range []string{
		"mode", "configured_mode", "demoted", "demoted_reason",
		"since", "breaker_rate_permille", "breaker_min_samples", "totals", "levels",
	} {
		if _, ok := body[k]; !ok {
			t.Errorf("stats 响应缺少字段 %q —— 页面靠它显示「还在不在拦」", k)
		}
	}
	if d, ok := body["demoted"].(bool); !ok || d {
		t.Errorf("未熔断时 demoted 应为 false，实际 %v", body["demoted"])
	}
	// 阈值必须真的下发到前端 —— 前端不另设一个数字
	if bp, _ := body["breaker_rate_permille"].(float64); bp <= 0 {
		t.Errorf("breaker_rate_permille 应为正数，实际 %v", body["breaker_rate_permille"])
	}

	totals := body["totals"].(map[string]any)
	for _, k := range []string{"total", "mismatched", "rejected", "rate_permille"} {
		if _, ok := totals[k]; !ok {
			t.Errorf("totals 缺少 %q", k)
		}
	}
	// 刚写了两行，窗口内至少能看到
	if totals["total"].(float64) < 2 {
		t.Errorf("totals.total 应至少为 2（刚写入两行），实际 %v", totals["total"])
	}
	if totals["mismatched"].(float64) < 1 {
		t.Errorf("totals.mismatched 应至少为 1，实际 %v", totals["mismatched"])
	}

	// 按关卡聚合要能命中刚写的关卡
	//
	// ⚠️ 必须断言 `[]any` 再逐个转型，不能直接写 `[]map[string]any`：
	//   响应是 JSON 解出来的，嵌套数组的真实类型是 `[]interface{}`。
	//   写成后者会 **panic** —— 而本包一条 panic 会带走整个 httpapi 测试包的结果。
	levels := body["levels"].([]any)
	seen := map[float64]bool{}
	for _, raw := range levels {
		l, ok := raw.(map[string]any)
		if !ok {
			t.Fatalf("levels 元素类型异常：%T", raw)
		}
		for _, k := range []string{"level_id", "max_user_mismatches"} {
			if _, ok := l[k]; !ok {
				t.Errorf("levels 行缺少 %q", k)
			}
		}
		seen[l["level_id"].(float64)] = true
	}
	for _, want := range []float64{11, 12} {
		if !seen[want] {
			t.Errorf("按关卡聚合里没有第 %v 关（写入的关卡）", want)
		}
	}
}

// TestE2EAdminRecomputeStatsMaxUserMismatches 单独钉住「单号最高不符」。
//
// 这条守卫是从一个**真实的 SQL bug** 长出来的：
// 内层 `GROUP BY` 里含 `mismatched_fields`，于是 `MAX(per_user)`
// 量到的是「该玩家相符的那些行」的计数 —— 不符 3 局、相符 50 局
// 会显示成 50。
//
// 而这一列正是区分漂移与作弊的唯一依据：
// 漂移是**每个**玩家都中，作弊是**某一个**玩家反复中。
// 算错它，两者就长得一样。
func TestE2EAdminRecomputeStatsMaxUserMismatches(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "rcm")

	// 同一关（第 21 关）、同一玩家：3 局相符 + 2 局不符
	const lvl = 21
	uid, _ := e.seedRecompute(t, "mixed_many", lvl, false, "shadow", false)
	e.seedRecomputeFor(t, uid, lvl, false)
	e.seedRecomputeFor(t, uid, lvl, false)
	e.seedRecomputeFor(t, uid, lvl, true)
	e.seedRecomputeFor(t, uid, lvl, true)

	status, body := e.get(t, "/api/v1/admin/recomputes/stats?hours=24", tok)
	if status != 200 {
		t.Fatalf("stats 应 200，实际 %d %v", status, body)
	}
	for _, raw := range body["levels"].([]any) {
		l := raw.(map[string]any)
		if l["level_id"].(float64) != lvl {
			continue
		}
		// ⚠️ 旧实现里这一列会返回 **3**（相符那组 MAX 更大），
		//   甚至在相符更多时返回 50 —— 看着像「这个号可疑 50 次」，
		//   实际上它只不符 2 次。
		if got := l["max_user_mismatches"].(float64); got != 2 {
			t.Errorf("第 %d 关的 max_user_mismatches 应为 2（该玩家只不符 2 局），实际 %v —— "+
				"它被算成了含相符行的计数，而这一列是区分漂移与作弊的唯一依据", lvl, got)
		}
		if got := l["mismatched"].(float64); got != 2 {
			t.Errorf("第 %d 关 mismatched 应为 2，实际 %v", lvl, got)
		}
		if got := l["total"].(float64); got != 5 {
			t.Errorf("第 %d 关 total 应为 5，实际 %v", lvl, got)
		}
		return
	}
	t.Error("按关卡聚合里没有第 21 关")
}
