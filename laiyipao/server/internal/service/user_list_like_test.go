package service

import (
	"context"
	"testing"
)

// 第 118 轮：AdminListUsers 的 keyword 必须把 `%`/`_` 当**字面量**。
//
// 旧实现直接拼 `%<kw>%`，运营搜 "100%" 退化成 `%100%%`（% 当通配符），
// 搜 "a_b" 时 `_` 匹配任意单字符 —— 「输入了东西却匹配到一大片」。
// 与第 111 轮关卡搜索同族（那里已修，这里是用户搜索的另一处）。
//
// 判据：造两个昵称里**含字面 % / 字面 _** 的用户 + 一个普通用户，
// 搜 "%" 应**只**命中含字面 % 的那一个，而不是全部。
func TestAdminListUsersKeywordLikeEscaped(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	mk := func(tag, nick, gt string) {
		ts.exec(t,
			`INSERT INTO users (guest_token, nickname, is_guest, status)
			 VALUES ($1,$2,TRUE,1)`, gt, nick)
	}
	// 昵称里带字面通配符的两个 + 一个干净对照。
	// ⚠️ 匹配同时打在 nickname 与 guest_token 上，
	// 所以 guest_token 一律不用 `_`/`%`，避免干扰字面量判读。
	mk("a", "L118-literal%user", "gtl118pct")
	mk("b", "L118-literal_user", "gtl118und")
	mk("c", "L118-plainuser", "gtl118plain")

	// 搜字面 "%"：只有 nick 含字面 % 的命中（guest_token 都无 %）
	if _, total, err := ts.AdminListUsers(ctx, "%", 200, 0); err != nil {
		t.Fatalf("搜百分号失败：%v", err)
	} else if total != 1 {
		t.Errorf("搜字面「%%」应命中 1 个（仅含字面 %% 的昵称），实际 %d（未转义时哨兵 %% 会命中全部）", total)
	}
	// 搜字面 "_"：只有 nick 含字面 _ 的命中
	if _, total, err := ts.AdminListUsers(ctx, "_", 200, 0); err != nil {
		t.Fatalf("搜下划线失败：%v", err)
	} else if total != 1 {
		t.Errorf("搜字面「_」应命中 1 个（_ 不该当「任意单字符」通配符），实际 %d", total)
	}
	// 正常关键词仍按子串匹配
	if _, total, err := ts.AdminListUsers(ctx, "plainuser", 200, 0); err != nil {
		t.Fatalf("搜 plainuser 失败：%v", err)
	} else if total != 1 {
		t.Errorf("搜 plainuser 应命中 1 个，实际 %d", total)
	}
	// 空关键词 = 不过滤（哨兵 %%），返回全部
	if _, total, err := ts.AdminListUsers(ctx, "", 200, 0); err != nil {
		t.Fatalf("空搜索失败：%v", err)
	} else if total != 3 {
		t.Errorf("空搜索应返回全部 3 个，实际 %d", total)
	}
}
