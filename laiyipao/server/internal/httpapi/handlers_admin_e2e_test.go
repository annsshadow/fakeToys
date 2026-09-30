package httpapi

// 管理端端到端测试。与玩家端同一夹具（见 routes_e2e_test.go）。
//
// 管理员的特殊性：账号由测试自建（admin_users 直接插入 bcrypt 哈希），
// 不依赖 BOOTSTRAP_ADMIN_PASS —— 开发库的管理员状态不可预测，
// 测试必须自己控制夹具而不是假设环境。

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
	"golang.org/x/crypto/bcrypt"
)

// newAdmin 建一个测试管理员并走真实登录拿令牌。
func (e *e2e) newAdmin(t *testing.T, tag string) (int64, string) {
	t.Helper()
	username := fmt.Sprintf("e2e_admin_%s_%d", tag, time.Now().UnixNano()%1_000_000)
	password := "e2e-admin-pass-123"
	hash, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.MinCost)
	if err != nil {
		t.Fatalf("生成哈希失败：%v", err)
	}
	var adminID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO admin_users (username, password_hash, role) VALUES ($1,$2,'admin') RETURNING id`,
		username, string(hash)).Scan(&adminID); err != nil {
		t.Fatalf("建管理员失败：%v", err)
	}
	t.Cleanup(func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `DELETE FROM admin_audit_logs WHERE username = $1`, username)
		_, _ = e.pool.Exec(ctx, `DELETE FROM admin_users WHERE id = $1`, adminID) // admin_tokens 级联
	})

	status, body := e.post(t, "/api/v1/admin/login", "", map[string]any{
		"username": username, "password": password,
	})
	if status != 200 {
		t.Fatalf("管理员登录失败：%d %v", status, body)
	}
	return adminID, body["access_token"].(string)
}

func TestE2EAdminLoginAndGuard(t *testing.T) {
	e := newE2E(t)
	adminID, adminTok := e.newAdmin(t, "login")

	// 坏 JSON / 未知用户 / 错误密码
	status, _ := e.post(t, "/api/v1/admin/login", "", map[string]any{"username": "nobody", "password": "x"})
	if status != fiber.StatusUnauthorized {
		t.Errorf("未知用户应 401，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/admin/login", "", map[string]any{"username": "e2e_wrong", "password": "x"})
	if status != fiber.StatusUnauthorized {
		t.Errorf("未知用户应 401（与错密码同文案，防枚举），实际 %d", status)
	}

	// 管理端守卫：无 token / 玩家 token / 正确 admin token
	status, body := e.get(t, "/api/v1/admin/dashboard", "")
	if status != fiber.StatusUnauthorized {
		t.Errorf("无 token 应 401，实际 %d", status)
	}
	_, playerTok := e.newPlayer(t, "guard")
	status, _ = e.get(t, "/api/v1/admin/dashboard", playerTok)
	if status != fiber.StatusUnauthorized {
		t.Errorf("玩家 token 打管理端点应 401（kind 不匹配），实际 %d", status)
	}
	status, body = e.get(t, "/api/v1/admin/me", adminTok)
	if status != 200 {
		t.Fatalf("admin/me 应 200，实际 %d %v", status, body)
	}
	if got := body["admin"].(map[string]any)["id"].(float64); int64(got) != adminID {
		t.Errorf("admin id = %v，期望 %d", got, adminID)
	}
}

func TestE2EAdminReadEndpoints(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "read")

	reads := []string{
		"/api/v1/admin/dashboard",
		"/api/v1/admin/levels",
		"/api/v1/admin/levels/1/waves",
		"/api/v1/admin/skills",
		"/api/v1/admin/equipment",
		"/api/v1/admin/users",
		"/api/v1/admin/users?keyword=E2E&limit=10&offset=0",
		"/api/v1/admin/battles",
		"/api/v1/admin/defenses",
		"/api/v1/admin/economy",
		"/api/v1/admin/announcements",
		"/api/v1/admin/redeem-codes",
		"/api/v1/admin/audit-logs",
	}
	for _, url := range reads {
		status, body := e.get(t, url, tok)
		if status != 200 {
			t.Errorf("GET %s 应 200，实际 %d %v", url, status, body)
		}
	}

	// 结构断言：这些端点的消费方是后台前端，形状漂移比报错更难发现
	status, body := e.get(t, "/api/v1/admin/levels", tok)
	if total := num(body, "total"); total != 100 {
		t.Errorf("levels total 应为 100，实际 %d", total)
	}
	status, body = e.get(t, "/api/v1/admin/dashboard", tok)
	if body["users"] == nil || body["battles"] == nil || body["verification"] == nil {
		t.Error("dashboard 必须包含 users/battles/verification 板块")
	}
	status, _ = e.get(t, "/api/v1/admin/levels/abc/waves", tok)
	if status != fiber.StatusBadRequest {
		t.Errorf("level id 非法应 400，实际 %d", status)
	}
	status, _ = e.get(t, "/api/v1/admin/battles/abc", tok)
	if status != fiber.StatusBadRequest {
		t.Errorf("battle id 非法应 400，实际 %d", status)
	}
}

func TestE2EAdminUpdateLevelAndRegenerate(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "lvl")
	t.Cleanup(func() {
		// 关卡是全局内容：改完必须恢复，不能污染后续用例与开发库
		e.post(t, "/api/v1/admin/levels/regenerate", tok, nil)
	})

	// 非法路径 / 空 patch / 坏 JSON
	status, _ := e.put(t, "/api/v1/admin/levels/0", tok, map[string]any{"name": "x"})
	if status != fiber.StatusBadRequest {
		t.Errorf("level id=0 应 400，实际 %d", status)
	}
	status, _ = e.put(t, "/api/v1/admin/levels/101", tok, map[string]any{"name": "x"})
	if status != fiber.StatusBadRequest {
		t.Errorf("level id=101 应 400，实际 %d", status)
	}
	status, _ = e.put(t, "/api/v1/admin/levels/1", tok, map[string]any{})
	if status != fiber.StatusBadRequest {
		t.Errorf("空 patch 应 400（没有可更新字段），实际 %d", status)
	}
	status, _ = e.put(t, "/api/v1/admin/levels/1", tok, map[string]any{"unknown_key": 1})
	if status != fiber.StatusBadRequest {
		t.Errorf("白名单外字段应被忽略并判空 patch → 400，实际 %d", status)
	}

	// 合法更新（数值走 float64，模拟真实 JSON 反序列化形状）
	status, body := e.put(t, "/api/v1/admin/levels/1", tok, map[string]any{
		"name": "E2E关卡", "base_hp": 5000.0, "wave_count": 3.0,
		"difficulty": 2.0, "energy_cost": 6.0,
		"star_targets": []int{100, 200, 300}, "enabled": true, "is_boss": false,
	})
	if status != 200 {
		t.Fatalf("合法更新应 200，实际 %d %v", status, body)
	}
	if got := e.scalarInt(t, `SELECT base_hp FROM levels WHERE id = 1`); got != 5000 {
		t.Errorf("base_hp 应写库为 5000，实际 %d", got)
	}
	if got := e.scalarInt(t, `SELECT enabled::int FROM levels WHERE id = 1`); got != 1 {
		t.Errorf("enabled 应写库为 true，实际 %d", got)
	}

	// regenerate 恢复生成器默认
	status, body = e.post(t, "/api/v1/admin/levels/regenerate", tok, nil)
	if status != 200 || num(body, "generated") != 100 {
		t.Errorf("regenerate 应生成 100 关：%d %v", status, body)
	}
	if got := e.scalarInt(t, `SELECT base_hp FROM levels WHERE id = 1`); got == 5000 {
		t.Error("regenerate 后 base_hp 应恢复生成器默认值")
	}
}

func TestE2EAdminUpdateSkill(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "skill")

	// 先取原名用于恢复（skills 是全局内容）
	_, body := e.get(t, "/api/v1/admin/skills", tok)
	items, _ := body["items"].([]any)
	if len(items) == 0 {
		t.Fatal("技能列表不应为空")
	}
	origName := items[0].(map[string]any)["name"].(string)
	origDescr := items[0].(map[string]any)["descr"].(string)
	t.Cleanup(func() {
		e.put(t, "/api/v1/admin/skills/1", tok, map[string]any{"name": origName, "descr": origDescr})
	})

	status, _ := e.put(t, "/api/v1/admin/skills/abc", tok, map[string]any{"name": "x"})
	if status != fiber.StatusBadRequest {
		t.Errorf("skill id 非法应 400，实际 %d", status)
	}
	status, _ = e.put(t, "/api/v1/admin/skills/1", tok, map[string]any{})
	if status != fiber.StatusBadRequest {
		t.Errorf("空 patch 应 400，实际 %d", status)
	}
	status, _ = e.put(t, "/api/v1/admin/skills/1", tok, map[string]any{"base_damage": -5})
	if status != fiber.StatusBadRequest {
		t.Errorf("负数值被过滤后应为空 patch → 400，实际 %d", status)
	}

	status, body = e.put(t, "/api/v1/admin/skills/1", tok, map[string]any{
		"name": "E2E技能", "base_damage": 42.0, "element": "fire", "kind": "active",
	})
	if status != 200 {
		t.Fatalf("合法更新应 200，实际 %d %v", status, body)
	}
	if got := e.scalarInt(t, `SELECT base_damage FROM skills WHERE id = 1`); got != 42 {
		t.Errorf("base_damage 应写库为 42，实际 %d", got)
	}
}

func TestE2EAdminUserManagement(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "usermgmt")
	uid, playerTok := e.newPlayer(t, "victim")

	// bad id
	status, _ := e.post(t, "/api/v1/admin/users/abc/ban", tok, map[string]any{"reason": "x"})
	if status != fiber.StatusBadRequest {
		t.Errorf("user id 非法应 400，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/admin/users/abc/grant", tok, map[string]any{"currency": "coin", "amount": 1})
	if status != fiber.StatusBadRequest {
		t.Errorf("user id 非法应 400，实际 %d", status)
	}

	// 封禁：默认理由、状态写库、refresh token 全部吊销
	status, body := e.post(t, fmt.Sprintf("/api/v1/admin/users/%d/ban", uid), tok, map[string]any{})
	if status != 200 {
		t.Fatalf("封禁应 200，实际 %d %v", status, body)
	}
	if got := e.scalarInt(t, `SELECT status FROM users WHERE id = $1`, uid); got != 2 {
		t.Errorf("封禁后 status 应为 2，实际 %d", got)
	}
	if got := e.scalarInt(t, `SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL`, uid); got != 0 {
		t.Errorf("封禁必须吊销全部 refresh token，仍剩 %d 个有效", got)
	}

	// 解封
	status, _ = e.post(t, fmt.Sprintf("/api/v1/admin/users/%d/unban", uid), tok, nil)
	if status != 200 {
		t.Fatalf("解封应 200，实际 %d", status)
	}
	if got := e.scalarInt(t, `SELECT status FROM users WHERE id = $1`, uid); got != 1 {
		t.Errorf("解封后 status 应为 1，实际 %d", got)
	}

	// 发币：成功 / 未知货币 / 坏 JSON
	before := e.scalarInt(t, `SELECT coin FROM user_wallets WHERE user_id = $1`, uid)
	status, body = e.post(t, fmt.Sprintf("/api/v1/admin/users/%d/grant", uid), tok,
		map[string]any{"currency": "coin", "amount": 123})
	if status != 200 {
		t.Fatalf("发币应 200，实际 %d %v", status, body)
	}
	if wallet, ok := body["wallet"].(map[string]any); !ok || int64(wallet["coin"].(float64)) != int64(before)+123 {
		t.Errorf("发币后钱包应返回且 coin=%d：%v", before+123, body["wallet"])
	}
	status, _ = e.post(t, fmt.Sprintf("/api/v1/admin/users/%d/grant", uid), tok,
		map[string]any{"currency": "diamonds", "amount": 1})
	if status != fiber.StatusBadRequest {
		t.Errorf("未知货币应 400，实际 %d", status)
	}
	status, _ = e.post(t, fmt.Sprintf("/api/v1/admin/users/%d/grant", uid), tok, "{bad")
	if status != fiber.StatusBadRequest {
		t.Errorf("坏 JSON 应 400，实际 %d", status)
	}

	// 玩家列表按关键词能搜到
	status, body = e.get(t, "/api/v1/admin/users?keyword=E2E_", tok)
	if status != 200 {
		t.Fatalf("玩家列表应 200，实际 %d", status)
	}
	if num(body, "total") < 1 {
		t.Errorf("按昵称关键词应能搜到测试玩家：%v", body["total"])
	}
	_ = playerTok
}

func TestE2EAdminShopAnnouncementsRedeem(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "content")

	// --- 商城 ---
	status, _ := e.put(t, "/api/v1/admin/shop/abc", tok, map[string]any{"limit_per_day": 1})
	if status != fiber.StatusBadRequest {
		t.Errorf("item id 非法应 400，实际 %d", status)
	}
	status, _ = e.put(t, "/api/v1/admin/shop/1", tok, map[string]any{})
	if status != fiber.StatusBadRequest {
		t.Errorf("空 patch 应 400，实际 %d", status)
	}
	status, _ = e.put(t, "/api/v1/admin/shop/1", tok, map[string]any{"name": "", "limit_per_day": -1})
	if status != fiber.StatusBadRequest {
		t.Errorf("全部字段被过滤应为空 patch → 400，实际 %d", status)
	}
	status, body := e.put(t, "/api/v1/admin/shop/1", tok, map[string]any{
		"name": "金币袋E2E", "limit_per_day": 5.0, "sort_order": 1.0, "enabled": true,
		"price": map[string]int{"gem": 10}, "payload": map[string]int{"coin": 20000},
	})
	if status != 200 {
		t.Fatalf("合法商城更新应 200，实际 %d %v", status, body)
	}
	if got := e.scalarInt(t, `SELECT limit_per_day FROM shop_items WHERE id = 1`); got != 5 {
		t.Errorf("limit_per_day 应写库为 5，实际 %d", got)
	}
	// 恢复种子值
	e.put(t, "/api/v1/admin/shop/1", tok, map[string]any{"name": "金币袋", "limit_per_day": 3.0})

	// --- 公告 ---
	status, _ = e.post(t, "/api/v1/admin/announcements", tok, map[string]any{"title": "", "body": ""})
	if status != fiber.StatusBadRequest {
		t.Errorf("空标题/正文应 400，实际 %d", status)
	}
	status, body = e.post(t, "/api/v1/admin/announcements", tok, map[string]any{
		"title": "E2E公告", "body": "内容", "published": true,
	})
	if status != 200 {
		t.Fatalf("创建公告应 200，实际 %d %v", status, body)
	}
	t.Cleanup(func() {
		_, _ = e.pool.Exec(context.Background(), `DELETE FROM announcements WHERE title = 'E2E公告'`)
	})
	status, body = e.get(t, "/api/v1/admin/announcements", tok)
	if status != 200 {
		t.Errorf("公告列表应 200，实际 %d", status)
	}

	// --- 兑换码 ---
	status, _ = e.post(t, "/api/v1/admin/redeem-codes", tok, map[string]any{"code": "", "reward": map[string]int{}})
	if status != fiber.StatusBadRequest {
		t.Errorf("空码/空奖励应 400，实际 %d", status)
	}
	status, body = e.get(t, "/api/v1/admin/redeem-codes", tok)
	if status != 200 {
		t.Errorf("兑换码列表应 200，实际 %d", status)
	}

	// 00009 修复后（id 改为 IDENTITY），创建必须成功且返回创建项。
	// 修复前这里是特征化断言"必然 500"（id 无默认值，not-null violation）。
	code := fmt.Sprintf("E2EBUG%d", time.Now().UnixNano()%1_000_000)
	status, body = e.post(t, "/api/v1/admin/redeem-codes", tok, map[string]any{
		"code": code, "reward": map[string]int{"coin": 1}, "max_uses": 3,
	})
	if status != fiber.StatusOK {
		t.Fatalf("创建兑换码应 200（00009 修复 id 无默认值缺陷），实际 %d %v", status, body)
	}
	item := body["item"].(map[string]any)
	if item["code"].(string) != code {
		t.Errorf("返回的 code = %v，期望 %s", item["code"], code)
	}
	if item["id"].(float64) <= 3 {
		t.Errorf("自动生成的 id 应与种子数据（1..3）不冲突，实际 %v", item["id"])
	}
	// 清理：审计日志随 admin_users 级联，兑换码需显式删
	t.Cleanup(func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `DELETE FROM redeem_codes WHERE code = $1`, code)
	})
}

func TestE2EAdminBattlesAndAudit(t *testing.T) {
	e := newE2E(t)
	adminID, tok := e.newAdmin(t, "audit")
	uid, playerTok := e.newPlayer(t, "fighter")

	// 造一条战报
	status, body := e.post(t, "/api/v1/battle/token", playerTok, map[string]any{"level_id": 1})
	if status != 200 {
		t.Skipf("开局失败（体力/种子数据异常）：%d %v", status, body)
	}
	tokenID := num(body, "token_id")
	status, body = e.post(t, "/api/v1/battle/settle", playerTok, settleBody(tokenID))
	if status != 200 {
		t.Skipf("结算失败：%d %v", status, body)
	}
	battleID := num(body, "battle_id")

	// 列表与详情（带过滤参数走 WHERE 分支）
	status, body = e.get(t, fmt.Sprintf("/api/v1/admin/battles?user_id=%d&level_id=1&limit=10", uid), tok)
	if status != 200 {
		t.Fatalf("战报列表应 200，实际 %d", status)
	}
	items, _ := body["items"].([]any)
	if len(items) == 0 {
		t.Fatal("过滤后的战报列表不应为空")
	}
	status, body = e.get(t, fmt.Sprintf("/api/v1/admin/battles/%d", battleID), tok)
	if status != 200 {
		t.Fatalf("战报详情应 200，实际 %d %v", status, body)
	}

	// 审计：管理操作必须留痕。用真实 adminID —— admin_id=0 会撞外键
	// 且 Audit 刻意吞掉写失败（不影响主流程），探针就无声消失了。
	e.svc.Audit(context.Background(), adminID, "e2e_audit_probe", fmt.Sprintf("user#%d", uid), map[string]any{"k": "v"})
	status, body = e.get(t, "/api/v1/admin/audit-logs?limit=200", tok)
	if status != 200 {
		t.Fatalf("审计列表应 200，实际 %d", status)
	}
	logs, _ := body["items"].([]any)
	found := false
	for _, it := range logs {
		if m, ok := it.(map[string]any); ok && m["action"] == "e2e_audit_probe" {
			found = true
			break
		}
	}
	if !found {
		t.Error("Audit 写入的记录应出现在审计列表里")
	}
}
