package httpapi

// 运营用户列表里的**验真统计**。
//
// ## 要解决的是「不可执行的数字」
//
// `replay_verifications` 表此前全项目**只被一个查询读过** ——
// `stats.go` 里那个 `COUNT(*)`。也就是说一次验真不匹配会被
// **记录、被统计**，但没有任何后果：
//
//   - 不撤销奖励
//   - 不标记用户
//   - 运营在用户列表里**看不到任何验真信号**
//
// 于是看板上的「验真一致率」是个**不可执行的数字**：
// 运营看到「1000 场验真、10 场不匹配」，却**不知道该封谁**。
// 而 `POST /admin/users/:id/ban` 就在同一个后台里躺着 —— 缺的是「指向谁」。
//
// ## 为什么是两个字段而不是一个
//
// `verify_checked = 0` 与 `verify_mismatched = 0` 含义**完全不同**：
//
//	0 / 0  →  从没被验真过 = **未知**
//	8 / 0  →  验过且都一致 = 干净
//	8 / 3  →  **可疑**
//
// 合成一个 `mismatches` 的话，前两种都显示 0，
// 于是**最不可信的那批用户看起来最干净** ——
// 那是本项目反复修的「静默降级而不是失败」，换了个马甲。
//
// ⚠️ 需要数据库：admin 端点在 `requireAdmin` 后面，鉴权会查 `admin_users`。
// 没有 Postgres 时 `newE2E` 会 Skip —— 那一刻断言**未执行**，不是「通过」。

import "testing"

// adminUserRow 在管理员用户列表里找到指定 id 的那一行。
func adminUserRow(t *testing.T, e *e2e, adminTok string, userID int64) map[string]any {
	t.Helper()
	_, body := e.get(t, "/api/v1/admin/users?limit=200", adminTok)
	items, _ := body["items"].([]any)
	if items == nil {
		t.Fatalf("用户列表响应里没有 items 数组：%v", body)
	}
	for _, it := range items {
		m, ok := it.(map[string]any)
		if !ok {
			continue
		}
		if int64(num(m, "id")) == userID {
			return m
		}
	}
	t.Fatalf("用户列表里找不到 id=%d 的行（共 %d 行）", userID, len(items))
	return nil
}

// newSettledBattle 造一场真实结算出来的战报，返回 battle_id。
func newSettledBattle(t *testing.T, e *e2e, playerTok string) int64 {
	t.Helper()
	status, body := e.post(t, "/api/v1/battle/token", playerTok, map[string]any{"level_id": 1})
	if status != 200 {
		t.Fatalf("开 battle token 失败：%d %v", status, body)
	}
	status, body = e.post(t, "/api/v1/battle/settle", playerTok, settleBody(num(body, "token_id")))
	if status != 200 {
		t.Fatalf("结算失败：%d %v", status, body)
	}
	return int64(num(body, "battle_id"))
}

// recordVerification 直接写一条验真记录（走 SQL 而非 HTTP 端点）。
//
// 为什么不走 `POST /battle/verify`：那个端点会拿**客户端提供的** actual
// 与库里存的 expected 比。我们要造的是「不匹配」这一事实，
// 用端点也能造，但用 SQL 更直接，且不依赖端点实现。
func recordVerification(t *testing.T, e *e2e, battleID, verifierID int64, matched bool) {
	t.Helper()
	_, err := e.pool.Exec(t.Context(),
		`INSERT INTO replay_verifications (battle_id, verifier_id, expected_hash, actual_hash, matched)
		 VALUES ($1,$2,'expected-hash','actual-hash',$3)`, battleID, verifierID, matched)
	if err != nil {
		t.Fatalf("写验真记录失败：%v", err)
	}
}

func TestAdminUsersExposeVerificationSignal(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "verifysig")

	// 三种状态各一个玩家，互不干扰。
	neverVerified, tokNever := e.newPlayer(t, "ver-never")
	allMatched, tokMatched := e.newPlayer(t, "ver-ok")
	suspect, tokSuspect := e.newPlayer(t, "ver-bad")

	// 1) 从没被验真过
	bNever := newSettledBattle(t, e, tokNever)

	// 2) 验过 2 次，全部一致
	bOK := newSettledBattle(t, e, tokMatched)
	recordVerification(t, e, bOK, allMatched, true)
	recordVerification(t, e, bOK, allMatched, true)

	// 3) 验过 3 次，其中 2 次不匹配
	bBad := newSettledBattle(t, e, tokSuspect)
	recordVerification(t, e, bBad, suspect, true)
	recordVerification(t, e, bBad, suspect, false)
	recordVerification(t, e, bBad, suspect, false)

	cases := []struct {
		name           string
		userID         int64
		wantChecked    int64
		wantMismatched int64
	}{
		// 关键用例：**从没验过**必须是 0/0，而不是被算成「不匹配 0 次所以干净」
		{"从未验真", neverVerified, 0, 0},
		{"验过且一致", allMatched, 2, 0},
		{"验过且不匹配", suspect, 3, 2},
	}
	for _, c := range cases {
		row := adminUserRow(t, e, adminTok, c.userID)
		gotChecked := int64(num(row, "verify_checked"))
		gotMismatch := int64(num(row, "verify_mismatched"))
		if gotChecked != c.wantChecked || gotMismatch != c.wantMismatched {
			t.Errorf("%s（uid=%d）：verify_checked=%d / verify_mismatched=%d，期望 %d / %d",
				c.name, c.userID, gotChecked, gotMismatch, c.wantChecked, c.wantMismatched)
		}
	}

	// 战报存在但没被验真时，`verify_checked` 必须是 0 而不是 1 ——
	// 「有战报」与「验过真」是两件事，混起来就等于宣称未验真的战报是一致的。
	if n := e.scalarInt(t, `SELECT COUNT(*) FROM battle_records WHERE user_id = $1`, neverVerified); n != 1 {
		t.Fatalf("前置条件不成立：player %d 名下战报数 = %d，期望 1 —— "+
			"「战报存在但未验真」这个场景根本没构造出来", neverVerified, n)
	}
	_ = bNever
}

// TestVerificationCountedAgainstBattleOwner 钉住关联路径。
//
// `replay_verifications.verifier_id` 是**验真人**，
// 不是这场战斗的**主人**。统计必须走
// `replay_verifications → battle_records → users`。
//
// 这条在当前测试库里**证明不了「按 verifier 统计会错」**：
// 每个用例里验真人和战报主人恰好是同一个人，两者重合。
// 所以它是**结构性约束**的钉子 —— 关联路径写错时，
// 一旦出现「A 验 B 的战斗」就会立刻暴露。
func TestVerificationCountedAgainstBattleOwner(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "verifyowner")

	owner, ownerTok := e.newPlayer(t, "ver-owner")
	other, _ := e.newPlayer(t, "ver-other")

	battleID := newSettledBattle(t, e, ownerTok)
	// 让**另一个用户**当验真人
	recordVerification(t, e, battleID, other, false)

	ownerRow := adminUserRow(t, e, adminTok, owner)
	if got := int64(num(ownerRow, "verify_checked")); got != 1 {
		t.Errorf("战报主人 verify_checked = %d，期望 1 —— 这场战斗属于他", got)
	}
	otherRow := adminUserRow(t, e, adminTok, other)
	if got := int64(num(otherRow, "verify_checked")); got != 0 {
		t.Errorf("验真人 verify_checked = %d，期望 0 —— 他只是验了别人的战斗，"+
			"不是他自己提交的战报。按 verifier 统计会把他算成可疑用户。", got)
	}
}

// TestVerificationCountsSurvivePagination 钉住「子查询预聚合」这个选择。
//
// 一场战斗可以被验真多次。直接 JOIN 明细表会让 users 一行变成 N 行，
// 而这个接口是**分页**的 —— 行数被放大后 LIMIT/OFFSET 就切在错误粒度上，
// 表现为分页时**用户重复出现或整页漏掉**。
func TestVerificationCountsSurvivePagination(t *testing.T) {
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "verifypage")

	uid, tok := e.newPlayer(t, "ver-page")
	battleID := newSettledBattle(t, e, tok)
	for i := 0; i < 5; i++ {
		recordVerification(t, e, battleID, uid, i%2 == 0) // 3 真 2 假
	}

	// limit=1 时同一页反复取，必须拿到**同一个**用户（而不是被放大的行）
	var firstID int64 = -1
	for page := 0; page < 3; page++ {
		_, body := e.get(t, "/api/v1/admin/users?limit=1&offset=0", adminTok)
		items, _ := body["items"].([]any)
		if len(items) != 1 {
			t.Fatalf("limit=1 却返回 %d 行 —— JOIN 放大了行数，分页已失效", len(items))
		}
		id := int64(num(items[0].(map[string]any), "id"))
		if firstID == -1 {
			firstID = id
		} else if id != firstID {
			t.Errorf("同一个分页参数两次拿到不同用户（%d vs %d）—— 分页不稳定",
				firstID, id)
		}
	}

	row := adminUserRow(t, e, adminTok, uid)
	if got := int64(num(row, "verify_checked")); got != 5 {
		t.Errorf("verify_checked = %d，期望 5", got)
	}
	if got := int64(num(row, "verify_mismatched")); got != 2 {
		t.Errorf("verify_mismatched = %d，期望 2（5 次里 2 次不匹配）", got)
	}
}
