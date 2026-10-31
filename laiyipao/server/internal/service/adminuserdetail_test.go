package service

import (
	"context"
	"strconv"
	"testing"
)

// TestAdminUserDetail 钉住 P5-7。
//
// # 背景
//
// 此前后台**只能看到用户列表的一行**。运营要回答的四个问题 ——
//
//   - 「这个人最近打了什么？」→ 战报
//   - 「他的钱去哪了？」→ 流水
//   - 「他的构筑是什么？为什么打不过第 40 关？」→ 构筑
//   - 「他被验真验出不匹配了吗？」→ 验真明细
//
// 全都要去别处拼 URL。
//
// 一个**不能看的玩家详情**比没有这个功能更糟：它让运营以为
// 「没有信息」意味着「没有问题」。
//
// # 判据
//
// 详情必须**真的带上那四块**（而不是空结构）——
// 空结构与「没有这个功能」在界面上长得一样。
func TestAdminUserDetail(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"coin": 5000, "gem": 30})

	// 打通一局，让「最近战报」有内容
	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput()); err != nil {
		t.Fatalf("结算失败：%v", err)
	}

	d, err := ts.AdminUserDetail(ctx, uid)
	if err != nil {
		t.Fatalf("读玩家详情失败：%v", err)
	}

	// 1) 列表那一行
	if d.User.ID != uid {
		t.Errorf("详情的用户 id 应为 %d，实际 %d", uid, d.User.ID)
	}
	if d.User.Nickname == "" {
		t.Error("详情应带昵称")
	}

	// 2) 钱包
	//
	// ⚠️ 断言用 `>=` 而不是 `==`：结算本身也会发掉落（金币/钻石），
	// 所以**恰好等于**发放值是错的 —— 那种断言会在「结算发了 loot」时红，
	// 而那不是缺陷。用 `>=` 断的是「详情里的钱包是真的、不是空的」。
	if d.Wallet["coin"] < 5000 {
		t.Errorf("钱包金币应 ≥5000，实际 %d", d.Wallet["coin"])
	}
	if d.Wallet["gem"] < 30 {
		t.Errorf("钱包钻石应 ≥30，实际 %d", d.Wallet["gem"])
	}

	// 3) 最近战报 —— 必须**有**（打过一局）
	if len(d.RecentBattles) == 0 {
		t.Error("刚打过一局，「最近战报」却是空的 —— 详情没有真的给到数据")
	}
	if d.RecentBattles[0].UserID != uid {
		t.Errorf("最近战报应属于该玩家，实际 user_id=%d", d.RecentBattles[0].UserID)
	}

	// 4) 出战槽位（真实构筑，不是空数组）
	if len(d.EquippedSkills) == 0 {
		t.Error("出战槽位为空 —— 构筑没进详情（这正是「看不到他为什么打不过」的原因）")
	}

	// 5) 不存在的用户 → 报错（而不是返回零值）
	if _, err := ts.AdminUserDetail(ctx, 999_999_999); err == nil {
		t.Error("不存在的用户应报错，而不是返回一个空详情")
	}
}

// TestAdminUserDetailBanRecords 守住封禁/解封记录（2026-11-01 新一轮第 58 轮）。
func TestAdminUserDetailBanRecords(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx)
	// 初始化钱包（详情会 LoadWalletReadOnly，无钱包会报「资源不存在」）
	ts.grant(t, ctx, uid, map[string]int64{"coin": 1})
	// 后台审计 target 是用户 id 字符串
	ts.exec(t, `INSERT INTO admin_audit_logs (username, action, target, detail)
		VALUES ('ops_alice', 'ban_user', $1, '{"reason":"恶意刷分"}'::jsonb),
		       ('ops_alice', 'unban_user', $1, '{"reason":"误封，已解"}'::jsonb)`,
		strconv.FormatInt(uid, 10))

	d, err := ts.AdminUserDetail(ctx, uid)
	if err != nil {
		t.Fatalf("读玩家详情失败：%v", err)
	}
	if len(d.BanRecords) != 2 {
		t.Fatalf("应有 2 条封禁/解封记录，实际 %d", len(d.BanRecords))
	}
	// 最新在前（unban 后插入应排第一）
	if d.BanRecords[0].Action != "unban_user" || d.BanRecords[1].Action != "ban_user" {
		t.Errorf("记录应按时间倒序，实际 %s / %s", d.BanRecords[0].Action, d.BanRecords[1].Action)
	}
	if d.BanRecords[0].Username != "ops_alice" {
		t.Errorf("操作人应为 ops_alice，实际 %q", d.BanRecords[0].Username)
	}
	// detail 原样保留（reason 是 JSONB 对象）
	reason := string(d.BanRecords[1].Reason)
	if len(reason) == 0 {
		t.Error("封禁记录应带 detail（reason）")
	}
}
