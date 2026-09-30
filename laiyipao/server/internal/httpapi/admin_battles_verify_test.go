package httpapi

// `GET /admin/battles` 的验真字段与 `only_mismatched` 过滤。
//
// ## 补的是哪一段
//
// 上一轮把「每用户验真统计」接进了 `/admin/users` —— 运营知道**谁**可疑。
// 但「**哪一场**对局不匹配」仍然答不出来：得手工按 user_id 查战报，
// 再和 `replay_verifications` 交叉比对。
//
// 这里补上最后一段：战报行自带验真状态，并支持只看不匹配的那些。
//
// ## 这里**没有**防什么（我一开始写错了）
//
// 原来的文件头写着「防 LEFT JOIN 后 NULL 不是 false 的坑」——
// **那是错的，是我的第 16 次「前提不成立」**。
//
// 实测：把 `COALESCE(v.mismatched, 0) > 0` 改回 `v.mismatched > 0`，
// 下面三个用例**照样全绿**。因为 `> 0` 这种比较下两种写法行为相同：
// 未匹配时 v.mismatched 是 NULL，`NULL > 0` 求值为 NULL（被排除），
// 而 COALESCE 版本得 `0 > 0` = false（也被排除）。
//
// 也就是说「未验真的战报不出现在「只看不匹配」列表里」是**预期行为**，
// 不是 COALESCE 修掉的 bug。
//
// 所以这三个用例守的是**真正会错的东西**：
//   - 每场战报带的 0/0、n/0、n/m 三态是否正确
//   - 过滤是否**真的**只挑出不匹配的那些（不多不少）
//   - 过滤的 `total` 是否与返回行数一致（分页计数不错位）
//   - 关掉过滤时未验真的战报必须**仍然在**（防止把问题错认成「过滤太严」）
//
// ⚠️ 需要数据库：admin 端点在 `requireAdmin` 后面。
// 没有 Postgres 时 `newE2E` 会 Skip —— 那一刻断言**未执行**，不是「通过」。

import (
	"fmt"
	"testing"
)

// battleRows 拉一页战报。
func battleRows(t *testing.T, e *e2e, adminTok string, query string) ([]map[string]any, int64) {
	t.Helper()
	status, body := e.get(t, "/api/v1/admin/battles?limit=200"+query, adminTok)
	if status != 200 {
		t.Fatalf("战报列表应 200，实际 %d：%v", status, body)
	}
	raw, _ := body["items"].([]any)
	items := make([]map[string]any, 0, len(raw))
	for _, it := range raw {
		if m, ok := it.(map[string]any); ok {
			items = append(items, m)
		}
	}
	tot, _ := body["total"].(float64)
	return items, int64(tot)
}

func findBattle(t *testing.T, items []map[string]any, id int64) map[string]any {
	t.Helper()
	for _, m := range items {
		if int64(num(m, "id")) == id {
			return m
		}
	}
	t.Fatalf("列表里找不到 battle_id=%d（返回 %d 行）", id, len(items))
	return nil
}

// 新建一个玩家并造 n 场战报，返回 uid / playerTok / 各场 battle_id。
func playerWithBattles(t *testing.T, e *e2e, tag string, n int) (int64, string, []int64) {
	t.Helper()
	uid, tok := e.newPlayer(t, tag)
	ids := make([]int64, 0, n)
	for i := 0; i < n; i++ {
		ids = append(ids, newSettledBattle(t, e, tok))
	}
	return uid, tok, ids
}

func TestAdminBattlesCarryVerificationState(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "battleverify")

	// 玩家 A：3 场，其中第 1 场验过 2 次全对，第 2 场验过 1 次不匹配，第 3 场没验
	uidA, _, idsA := playerWithBattles(t, e, "bv-a", 3)
	recordVerification(t, e, idsA[0], uidA, true)
	recordVerification(t, e, idsA[0], uidA, true)
	recordVerification(t, e, idsA[1], uidA, false)

	items, _ := battleRows(t, e, adminTok, fmt.Sprintf("&user_id=%d", uidA))
	if len(items) != 3 {
		t.Fatalf("该玩家应返回 3 场战报，实际 %d", len(items))
	}

	want := map[int64][2]int64{
		idsA[0]: {2, 0}, // 验过 2 次，全一致
		idsA[1]: {1, 1}, // 验过 1 次，不匹配
		idsA[2]: {0, 0}, // 从没验过
	}
	for id, w := range want {
		row := findBattle(t, items, id)
		gotChecked := int64(num(row, "verify_checked"))
		gotMismatch := int64(num(row, "verify_mismatched"))
		if gotChecked != w[0] || gotMismatch != w[1] {
			t.Errorf("battle %d: verify %d/%d，期望 %d/%d", id, gotChecked, gotMismatch, w[0], w[1])
		}
	}
}

func TestAdminBattlesOnlyMismatchedFilter(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "battlefilter")

	uid, _, ids := playerWithBattles(t, e, "bv-f", 3)
	recordVerification(t, e, ids[0], uid, true)  // 一致
	recordVerification(t, e, ids[1], uid, false) // 不匹配
	// ids[2] 从没验过

	items, total := battleRows(t, e, adminTok, fmt.Sprintf("&user_id=%d&only_mismatched=1", uid))

	// 核心断言：只该返回**不匹配的那一场**（一致的那场、未验真的那场都不该出现）。
	//
	// 同时断言 total：**过滤后的 total 必须等于返回行数** ——
	// 「total 与实际集合不一致」是分页里最难查的一类错，
	// 只断言行数是抓不到它的。
	if len(items) != 1 {
		t.Fatalf("only_mismatched=1 应只返回 1 场，实际 %d 场", len(items))
	}
	if total != int64(len(items)) {
		t.Errorf("total=%d 与返回行数 %d 不一致 —— 分页计数已错位", total, len(items))
	}
	if got := int64(num(items[0], "id")); got != ids[1] {
		t.Errorf("返回的是 battle %d，期望不匹配的那场 %d", got, ids[1])
	}
}

// TestAdminBattlesFilterKeepsUnverifiedOut 钉住「过滤不是把东西弄丢了」。
//
// 拆成独立用例的价值在于那个**对照**：如果开启过滤后返回 0 场，
// 必须再确认「关掉过滤时这 3 场还在」——
// 否则「0 场」这个结果有两种完全不同的成因（过滤太严 vs 查询坏了），
// 而它们需要完全不同的修法。
func TestAdminBattlesFilterKeepsUnverifiedOut(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "bvnfilter")

	// 全部 3 场都没被验真过
	uid, _, ids := playerWithBattles(t, e, "bv-n", 3)

	// 开启过滤后应该**一场都不剩**（没有任何不匹配记录）
	items, _ := battleRows(t, e, adminTok, fmt.Sprintf("&user_id=%d&only_mismatched=1", uid))
	if len(items) != 0 {
		t.Errorf("三场都没验真，only_mismatched=1 却返回 %d 场：%v", len(items), items)
	}

	// 关键对照：**关闭**过滤时它们必须在。
	// 若这里也是 0，说明问题不在过滤条件，而是整个战报查询坏了。
	all, allTotal := battleRows(t, e, adminTok, fmt.Sprintf("&user_id=%d", uid))
	if len(all) != 3 {
		t.Errorf("不开启过滤时应返回 3 场，实际 %d —— 战报查询本身有问题", len(all))
	}
	if allTotal != 3 {
		t.Errorf("不开启过滤时 total=%d，期望 3", allTotal)
	}

	// 再确认「未验真」在未过滤时是 0/0，而不是被算成不匹配
	row := findBattle(t, all, ids[0])
	if int64(num(row, "verify_checked")) != 0 || int64(num(row, "verify_mismatched")) != 0 {
		t.Errorf("未验真的战报应报 0/0，实际 %d/%d",
			num(row, "verify_checked"), num(row, "verify_mismatched"))
	}
}
