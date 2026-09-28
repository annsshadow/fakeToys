package httpapi

// 运营侧验真 `POST /admin/battles/:id/verify` 的契约。
//
// ## 补的是哪一段
//
// 验真机制此前**只有玩家入口**。这产生了一个结构性后果：
// **能采取行动的人（运营），恰恰是唯一不能发起验真的人。**
// 于是「验真一致率」这个数字的分母永远只由玩家的自愿行为决定 ——
// 前两轮把「谁可疑」「哪几场可疑」都接上了，却没人能主动去验。
//
// ## ⚠️ 这里**不重算**哈希
//
// 本端点比对的是「运营提交的」与「结算时记录的」。
// 一个**原始作弊客户端**可以报一个相同的 hash 让结果「一致」——
// 这个机制防的是「改数据不改凭证」，不是「从一开始就伪造」。
// 后者要靠服务端重算，需要把 TS 引擎移植到 Go，属于另一个量级。
//
// 有专门一条用例把这个**局限**钉住，避免下一个人误以为它是完整验真。
//
// ⚠️ 需要数据库：admin 端点在 `requireAdmin` 后面。
// 没有 Postgres 时 `newE2E` 会 Skip —— 那一刻断言**未执行**，不是「通过」。

import (
	"testing"
)

// adminVerify 打一次运营验真，返回状态码与响应体。
func adminVerify(t *testing.T, e *e2e, adminTok string, battleID int64, hash string) (int, map[string]any) {
	t.Helper()
	status, body := e.post(t, "/api/v1/admin/battles/"+itoa(battleID)+"/verify", adminTok,
		map[string]any{"replay_hash": hash})
	return status, body
}

func itoa(v int64) string {
	if v == 0 {
		return "0"
	}
	neg := v < 0
	if neg {
		v = -v
	}
	var b [24]byte
	i := len(b)
	for v > 0 {
		i--
		b[i] = byte('0' + v%10)
		v /= 10
	}
	if neg {
		i--
		b[i] = '-'
	}
	return string(b[i:])
}

func TestAdminVerifyRecordsIntoSameTable(t *testing.T) {
	e := newE2E(t)
	adminID, adminTok := e.newAdmin(t, "admverify")
	_, tok := e.newPlayer(t, "verplayer")
	battleID := newSettledBattle(t, e, tok)

	var stored string
	if err := e.pool.QueryRow(t.Context(),
		`SELECT replay_hash FROM battle_records WHERE id = $1`, battleID).Scan(&stored); err != nil {
		t.Fatalf("读回结算时存的 replay_hash 失败：%v", err)
	}
	if stored == "" {
		t.Fatal("前置条件不成立：这场战报的 replay_hash 是空的，" +
			"而 settleBody 本应带一个 —— 测不到「一致」这个分支")
	}

	// 1) 报同一个 hash ⇒ 一致
	status, body := adminVerify(t, e, adminTok, battleID, stored)
	if status != 200 {
		t.Fatalf("应 200，实际 %d：%v", status, body)
	}
	if m, _ := body["matched"].(bool); !m {
		t.Errorf("报相同 hash 应 matched=true，实际 %v（body=%v）", body["matched"], body)
	}

	// 2) 落库进了**同一张表**，且验真人记的是管理员
	var adminVerif, userVerif int
	if err := e.pool.QueryRow(t.Context(),
		`SELECT count(*) FILTER (WHERE admin_verifier_id = $1),
		        count(*) FILTER (WHERE verifier_id IS NOT NULL)
		 FROM replay_verifications WHERE battle_id = $2`,
		adminID, battleID).Scan(&adminVerif, &userVerif); err != nil {
		t.Fatalf("查询验真记录失败：%v", err)
	}
	if adminVerif != 1 {
		t.Errorf("admin_verifier_id=%d 的记录数 = %d，期望 1 —— "+
			"运营发起的验真没有落进统计表，一致率的分母不会变", adminID, adminVerif)
	}
	if userVerif != 0 {
		t.Errorf("verifier_id 非空的记录数 = %d，期望 0 —— "+
			"运营不该被记成玩家（否则会污染「谁验的」）", userVerif)
	}
}

func TestAdminVerifyDetectsMismatch(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "advmismatch")
	_, tok := e.newPlayer(t, "vermm")
	battleID := newSettledBattle(t, e, tok)

	status, body := adminVerify(t, e, adminTok, battleID, "deadbeefdeadbeef")
	if status != 200 {
		t.Fatalf("应 200，实际 %d：%v", status, body)
	}
	if m, _ := body["matched"].(bool); m {
		t.Errorf("报不同 hash 应 matched=false，实际 %v", body["matched"])
	}
	// 返回体必须带 seed —— 运营要拿它去重算，没有 seed 就没法复核
	if s, _ := body["seed"].(string); s == "" {
		t.Error("响应里没有 seed —— 运营无法据此重算，这个端点就只是「记一笔」")
	}
}

func TestAdminVerifyRejectsEmptyHash(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "admempty")
	_, tok := e.newPlayer(t, "verempty")
	battleID := newSettledBattle(t, e, tok)

	before := e.scalarInt(t, `SELECT COUNT(*) FROM replay_verifications WHERE battle_id = $1`, battleID)
	status, _ := adminVerify(t, e, adminTok, battleID, "")
	if status == 200 {
		t.Error("空 replay_hash 不该被接受")
	}
	after := e.scalarInt(t, `SELECT COUNT(*) FROM replay_verifications WHERE battle_id = $1`, battleID)
	if after != before {
		t.Errorf("空 hash 仍然落了一条记录（%d → %d）—— "+
			"那会把一条「没验过」写成「验过且不符」，凭空制造指控", before, after)
	}
}

func TestAdminVerifyNotFoundBattle(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "adm404")
	status, _ := adminVerify(t, e, adminTok, 999999999, "abc")
	if status == 200 {
		t.Error("对不存在的 battle 验真不该返回 200")
	}
}

// TestAdminVerifyDoesNotRecompute 钉住这个机制的**局限**。
//
// 它验证的是：报一个**从未有人算过**的 hash 也能「一致」——
// 因为 `expected_hash` 是结算时**客户端自己报上来**的。
//
// 这条用例的作用不是让功能变好，而是防止有人误以为
// 「运营验真端点 = 服务端重算」。真要那样得把 TS 引擎移植到 Go。
func TestAdminVerifyDoesNotRecompute(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "admnorecompute")
	_, tok := e.newPlayer(t, "verfake")

	// 造一场 replay_hash 是编造出来的战报（根本没跑过引擎）
	status, body := e.post(t, "/api/v1/battle/token", tok, map[string]any{"level_id": 1})
	if status != 200 {
		t.Fatalf("开 token 失败：%d %v", status, body)
	}
	forged := map[string]any{
		"token_id": num(body, "token_id"), "result": "lose", "score": 1,
		"duration_ms": 30000, "shots": 1, "hits": 1, "reactions": 0,
		"heat_max": 0, "hp_left": 1, "wave_reached": 1,
		"replay_hash": "forged-hash-never-computed",
	}
	status, body = e.post(t, "/api/v1/battle/settle", tok, forged)
	if status != 200 {
		t.Fatalf("结算失败：%d %v", status, body)
	}
	battleID := int64(num(body, "battle_id"))

	// 报同一个编造的 hash ⇒ 「一致」
	_, res := adminVerify(t, e, adminTok, battleID, "forged-hash-never-computed")
	if m, _ := res["matched"].(bool); !m {
		t.Errorf("报相同 hash 应 matched=true，实际 %v", res["matched"])
	}

	// ⚠️ 这就是局限本身：这场战斗**从未被任何引擎跑过**，却被判为「一致」。
	// 所以「验真一致率」**不能**被解读成「这些对局都是真实模拟的」。
	t.Log("已确认局限：伪造的 replay_hash 也能通过比对 —— " +
		"该指标防的是「改数据不改凭证」，不是「从一开始就伪造」")
}
