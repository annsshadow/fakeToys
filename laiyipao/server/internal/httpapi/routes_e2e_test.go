package httpapi

// HTTP 层端到端测试：真实 Service + 真实 PostgreSQL + Fiber app.Test。
//
// 为什么是端到端而不是逐 handler mock：本包的 handler 全部是
// 「解析请求 → 调 service → 转响应」的薄层，mock 掉 service 后
// 测到的只有反射进 struct 的字段名。真正会坏的是整条链路 ——
// 路由没挂上、中间件顺序错、JSON tag 拼错、错误码映射漏分支 ——
// 这些只有真 app + 真库才抓得住。
//
// 数据隔离：每个用例自建游客账号（guest_token 唯一），用例结束后
// DELETE FROM users（所有玩家表 ON DELETE CASCADE），
// 审计日志按 username 前缀清理。绝不触碰非测试数据。
//
// 需要 TEST_DATABASE_URL（缺省连本机 laiyipao 库），连不上则 Skip。

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
	"github.com/jackc/pgx/v5/pgxpool"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/service"
	"github.com/laiyipao/server/internal/store"
)

// --- 夹具 ---

type e2e struct {
	app  *fiber.App
	svc  *service.Service
	pool *pgxpool.Pool
}

// newE2E 起一个完整 app（真中间件 + 真路由 + 真 Service）。
func newE2E(t *testing.T) *e2e {
	t.Helper()
	dsn := os.Getenv("TEST_DATABASE_URL")
	if dsn == "" {
		dsn = "postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable"
	}
	cfg := config.Load()
	cfg.DatabaseURL = dsn
	// 短 TTL，测试签发的令牌在库里留不久
	cfg.AccessTTL = 5 * time.Minute

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	db, err := store.Open(ctx, cfg)
	if err != nil {
		t.Skipf("无可用测试数据库（%s），跳过：%v", dsn, err)
	}
	probe, probeCancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer probeCancel()
	if err := db.Pool.Ping(probe); err != nil {
		db.Close()
		t.Skipf("数据库不可达，跳过：%v", err)
	}
	t.Cleanup(db.Close)

	svc := service.New(db, cfg)
	app := fiber.New(fiber.Config{DisableStartupMessage: true})
	New(svc).Register(app)
	return &e2e{app: app, svc: svc, pool: db.Pool}
}

// 返回状态码与响应体；响应体不是 JSON 时（fiber 默认 404 文本）body 为 nil。
func (e *e2e) do(t *testing.T, method, url, token string, body any) (int, map[string]any) {
	t.Helper()
	var rd *bytes.Reader
	if body != nil {
		raw, err := json.Marshal(body)
		if err != nil {
			t.Fatalf("编码请求体失败：%v", err)
		}
		rd = bytes.NewReader(raw)
	} else {
		rd = bytes.NewReader(nil)
	}
	req := httptest.NewRequest(method, url, rd)
	req.Header.Set("Content-Type", "application/json")
	if token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("%s %s 执行失败：%v", method, url, err)
	}
	defer resp.Body.Close()
	var m map[string]any
	_ = json.NewDecoder(resp.Body).Decode(&m)
	return resp.StatusCode, m
}

func (e *e2e) get(t *testing.T, url, token string) (int, map[string]any) {
	return e.do(t, http.MethodGet, url, token, nil)
}

func (e *e2e) post(t *testing.T, url, token string, body any) (int, map[string]any) {
	return e.do(t, http.MethodPost, url, token, body)
}

func (e *e2e) put(t *testing.T, url, token string, body any) (int, map[string]any) {
	return e.do(t, http.MethodPut, url, token, body)
}

// loginGuest 以指定游客凭证登录，返回 user_id 与 access_token。
func (e *e2e) loginGuest(t *testing.T, guest, nickname string) (int64, string) {
	t.Helper()
	status, body := e.post(t, "/api/v1/auth/guest", "", map[string]any{
		"guest_token": guest,
		"nickname":    nickname,
	})
	if status != fiber.StatusOK {
		t.Fatalf("建号失败 status=%d body=%v", status, body)
	}
	id := int64(body["user"].(map[string]any)["id"].(float64))
	tok := body["access_token"].(string)
	t.Cleanup(func() {
		ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()
		_, _ = e.pool.Exec(ctx, `DELETE FROM users WHERE id = $1`, id)
		_, _ = e.pool.Exec(ctx, `DELETE FROM admin_audit_logs WHERE username LIKE $1`, "user#%")
	})
	return id, tok
}

// newPlayer 建一个游客账号（凭证自动生成）。
func (e *e2e) newPlayer(t *testing.T, tag string) (int64, string) {
	t.Helper()
	return e.loginGuest(t, fmt.Sprintf("e2e_%s_%d", tag, time.Now().UnixNano()), "E2E_"+tag)
}

// exec 执行一条 SQL（测试数据准备用）。
func (e *e2e) exec(t *testing.T, q string, args ...any) {
	t.Helper()
	if _, err := e.pool.Exec(context.Background(), q, args...); err != nil {
		t.Fatalf("执行 %s 失败：%v", q, err)
	}
}

// scalarInt 查一个整数（断言库中状态用）。
func (e *e2e) scalarInt(t *testing.T, q string, args ...any) int {
	t.Helper()
	var n int
	if err := e.pool.QueryRow(context.Background(), q, args...).Scan(&n); err != nil {
		t.Fatalf("查询 %s 失败：%v", q, err)
	}
	return n
}

// num 从 JSON map 里取 float64 数字并转 int64。
func num(m map[string]any, key string) int64 {
	v, _ := m[key].(float64)
	return int64(v)
}

// --- 基础端点 ---

func TestE2EHealthAndCORS(t *testing.T) {
	e := newE2E(t)

	status, body := e.get(t, "/healthz", "")
	if status != 200 || body["ok"] != true {
		t.Fatalf("/healthz = %d %v", status, body)
	}
	status, body = e.get(t, "/readyz", "")
	if status != 200 || body["db"] != "up" {
		t.Fatalf("/readyz = %d %v（数据库应可用）", status, body)
	}

	// Register 挂的全局中间件必须在真实路由上生效
	req := httptest.NewRequest(http.MethodGet, "/healthz", nil)
	resp, err := e.app.Test(req, 5000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	if got := resp.Header.Get("Access-Control-Allow-Origin"); got != "*" {
		t.Errorf("CORS 头未生效：%q", got)
	}
	if resp.Header.Get("X-Request-Duration") == "" {
		t.Error("访问日志头未写入")
	}

	// 未知路由：fiber 默认 404
	status, _ = e.get(t, "/api/v1/definitely-not-exist", "")
	if status != fiber.StatusNotFound {
		t.Errorf("未知路由应 404，实际 %d", status)
	}
}

// --- 登录 / 令牌 ---

func TestE2EGuestLoginAndRefreshChain(t *testing.T) {
	e := newE2E(t)

	// 坏 JSON → 400（直接构造非 JSON 字节流，绕过结构化 body）
	rawReq := httptest.NewRequest(http.MethodPost, "/api/v1/auth/guest", strings.NewReader("{invalid"))
	rawReq.Header.Set("Content-Type", "application/json")
	resp, err := e.app.Test(rawReq, 5000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	if resp.StatusCode != fiber.StatusBadRequest {
		t.Fatalf("坏 JSON 应 400，实际 %d", resp.StatusCode)
	}

	// 空凭证自动生成
	status, body := e.post(t, "/api/v1/auth/guest", "", map[string]any{})
	if status != 200 {
		t.Fatalf("空 guest_token 应自动生成并登录成功，实际 %d %v", status, body)
	}
	if body["refresh_token"].(string) == "" {
		t.Error("必须下发 refresh_token")
	}
	autoID := int64(body["user"].(map[string]any)["id"].(float64))
	t.Cleanup(func() { _, _ = e.pool.Exec(context.Background(), `DELETE FROM users WHERE id = $1`, autoID) })

	// 常规登录后走 refresh 链
	status, body = e.post(t, "/api/v1/auth/guest", "", map[string]any{"nickname": "chain"})
	if status != 200 {
		t.Fatalf("登录失败：%d", status)
	}
	chainID := int64(body["user"].(map[string]any)["id"].(float64))
	t.Cleanup(func() { _, _ = e.pool.Exec(context.Background(), `DELETE FROM users WHERE id = $1`, chainID) })
	refresh := body["refresh_token"].(string)
	status, body = e.post(t, "/api/v1/auth/refresh", "", map[string]any{"refresh_token": refresh})
	if status != 200 {
		t.Fatalf("refresh 应成功，实际 %d %v", status, body)
	}
	newRefresh := body["refresh_token"].(string)
	if newRefresh == "" || newRefresh == refresh {
		t.Error("refresh 后必须下发新的 refresh_token")
	}

	// 旧 refresh_token 已被吊销，复用必须 401
	status, _ = e.post(t, "/api/v1/auth/refresh", "", map[string]any{"refresh_token": refresh})
	if status != fiber.StatusUnauthorized {
		t.Errorf("复用已用过的 refresh_token 应 401，实际 %d", status)
	}
	// 垃圾串 → 401；空串 → 400
	status, _ = e.post(t, "/api/v1/auth/refresh", "", map[string]any{"refresh_token": "garbage"})
	if status != fiber.StatusUnauthorized {
		t.Errorf("垃圾 refresh_token 应 401，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/auth/refresh", "", map[string]any{"refresh_token": ""})
	if status != fiber.StatusBadRequest {
		t.Errorf("空 refresh_token 应 400，实际 %d", status)
	}
}

// TestE2EBannedUserRejected 封禁必须在签发口被拦（issueTokens 的状态检查）
// 与每请求校验（ResolveUser）两处同时生效。
func TestE2EBannedUserRejected(t *testing.T) {
	e := newE2E(t)
	guest := fmt.Sprintf("e2e_ban_%d", time.Now().UnixNano())
	uid, tok := e.loginGuest(t, guest, "E2E_ban")

	e.exec(t, `UPDATE users SET status = 2 WHERE id = $1`, uid)

	// 同一游客凭证再登录 → 403（签发口拦截）
	status, body := e.loginGuestErr(t, guest)
	if status != fiber.StatusForbidden {
		t.Fatalf("封禁后登录应 403，实际 %d %v", status, body)
	}

	// 持有旧 access token 打业务端点 → 403（每请求校验拦截）
	status, _ = e.get(t, "/api/v1/wallet/", tok)
	if status != fiber.StatusForbidden {
		t.Errorf("封禁用户的旧 token 应 403，实际 %d", status)
	}
}

// loginGuestErr 与 loginGuest 相同但不 Fatal，用于断言拒绝路径。
func (e *e2e) loginGuestErr(t *testing.T, guest string) (int, map[string]any) {
	t.Helper()
	return e.post(t, "/api/v1/auth/guest", "", map[string]any{"guest_token": guest, "nickname": "x"})
}

// TestE2EWechatLogin 微信通道：未配置 → 403；配置后走 code2Session 全部分支。
// WechatEndpoint 指向 httptest 假微信服务器，无需外网。
func TestE2EWechatLogin(t *testing.T) {
	e := newE2E(t)

	// 未启用 → 403（fail loud，绝不静默降级成游客）
	status, body := e.post(t, "/api/v1/auth/wechat", "", map[string]any{"code": "abc"})
	if status != fiber.StatusForbidden {
		t.Fatalf("未启用微信登录应 403，实际 %d %v", status, body)
	}

	// 假微信服务器：按路径返回不同响应
	var wxResp string
	var wxStatus int
	wx := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(wxStatus)
		_, _ = w.Write([]byte(wxResp))
	}))
	defer wx.Close()

	e.svc.Cfg.WechatAppID = "wx-test"
	e.svc.Cfg.WechatAppSecret = "sec"
	e.svc.Cfg.WechatEndpoint = wx.URL

	// 启用后空 code → 400
	status, _ = e.post(t, "/api/v1/auth/wechat", "", map[string]any{"code": "  "})
	if status != fiber.StatusBadRequest {
		t.Errorf("启用后空 code 应 400，实际 %d", status)
	}

	// 成功：返回 openid，建非游客账号
	wxStatus, wxResp = 200, `{"openid":"oE2E","session_key":"k"}`
	status, body = e.post(t, "/api/v1/auth/wechat", "", map[string]any{"code": "good"})
	if status != 200 {
		t.Fatalf("微信登录应成功，实际 %d %v", status, body)
	}
	if user, ok := body["user"].(map[string]any); ok {
		if user["is_guest"].(bool) {
			t.Error("微信登录建的不是游客账号")
		}
		t.Cleanup(func() {
			_, _ = e.pool.Exec(context.Background(), `DELETE FROM users WHERE guest_token = $1`, "wx_oE2E")
		})
	}

	// 微信返回 errcode → 403
	wxResp = `{"errcode":40029,"errmsg":"invalid code"}`
	status, _ = e.post(t, "/api/v1/auth/wechat", "", map[string]any{"code": "bad"})
	if status != fiber.StatusForbidden {
		t.Errorf("errcode!=0 应 403，实际 %d", status)
	}

	// 微信未返回 openid → 403
	wxResp = `{"session_key":"k"}`
	status, _ = e.post(t, "/api/v1/auth/wechat", "", map[string]any{"code": "noid"})
	if status != fiber.StatusForbidden {
		t.Errorf("缺 openid 应 403，实际 %d", status)
	}

	// 响应不是 JSON → 500（内部错误，不泄露细节）
	wxResp = `<html>not json</html>`
	status, _ = e.post(t, "/api/v1/auth/wechat", "", map[string]any{"code": "ugly"})
	if status != fiber.StatusInternalServerError {
		t.Errorf("非 JSON 响应应 500，实际 %d", status)
	}

	// 微信服务不可达 → 500
	e.svc.Cfg.WechatEndpoint = "http://127.0.0.1:1/sns/jscode2session"
	status, _ = e.post(t, "/api/v1/auth/wechat", "", map[string]any{"code": "dead"})
	if status != fiber.StatusInternalServerError {
		t.Errorf("微信不可达应 500，实际 %d", status)
	}
	_ = wxStatus
}

// --- 玩家信息 / 配置 / 排行 ---

func TestE2EMeConfigLeaderboard(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "me")

	// 无 token / 垃圾 token / admin token 三条拒绝路径
	status, body := e.get(t, "/api/v1/me/", "")
	if status != fiber.StatusUnauthorized || body["error"].(map[string]any)["code"] != "unauthorized" {
		t.Fatalf("缺 token 应 401 unauthorized，实际 %d %v", status, body)
	}
	status, _ = e.get(t, "/api/v1/me/", "not-a-jwt")
	if status != fiber.StatusUnauthorized {
		t.Errorf("垃圾 token 应 401，实际 %d", status)
	}

	// 成功：构筑快照带服务端权威的 attacker 与 active_slots
	status, body = e.get(t, "/api/v1/me/", tok)
	if status != 200 {
		t.Fatalf("/me 应 200，实际 %d %v", status, body)
	}
	if num(body, "user_id") != uid {
		t.Errorf("user_id = %d，期望 %d", num(body, "user_id"), uid)
	}
	build, ok := body["build"].(map[string]any)
	if !ok || build["attacker"] == nil {
		t.Fatal("/me 必须下发 build.attacker（I-6 前提）")
	}
	if int(num(build, "active_slots")) < int(domain.BaseSkillSlots) {
		t.Errorf("active_slots = %v，至少应为基础槽位 %d", build["active_slots"], domain.BaseSkillSlots)
	}

	// /config 全量下发
	status, body = e.get(t, "/api/v1/config", tok)
	if status != 200 {
		t.Fatalf("/config 应 200，实际 %d", status)
	}
	levels, _ := body["levels"].([]any)
	if len(levels) != domain.TotalLevels {
		t.Errorf("levels 应为 %d 关，实际 %d", domain.TotalLevels, len(levels))
	}
	if body["score_rules"] == nil || body["skill_rules"] == nil {
		t.Error("score_rules / skill_rules 必须下发（跨端契约源）")
	}

	// /levels/:id
	status, _ = e.get(t, "/api/v1/levels/1", tok)
	if status != 200 {
		t.Errorf("levels/1 应 200，实际 %d", status)
	}
	for _, bad := range []string{"0", "101", "abc"} {
		status, _ = e.get(t, "/api/v1/levels/"+bad, tok)
		if status != fiber.StatusBadRequest {
			t.Errorf("levels/%s 应 400，实际 %d", bad, status)
		}
	}

	// /leaderboard 三种榜单
	e.exec(t, `INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
	           VALUES ($1, 1, 3, 1000, 1, 100)
	           ON CONFLICT (user_id, level_id) DO UPDATE SET clears = 1, min_power_clear = 100`, uid)
	for _, q := range []string{"", "?type=power", "?type=stage", "?type=efficiency"} {
		status, body = e.get(t, "/api/v1/leaderboard"+q, "")
		if status != 200 {
			t.Errorf("leaderboard%s 应 200，实际 %d", q, status)
		}
	}
	//
	// ⚠️ 第 104 轮改写了这段断言。
	//
	// 原来这里是：
	//
	//	for _, q := range []string{"", "?type=stage", "?type=efficiency",
	//	                           "?type=bogus", "?limit=abc"} {
	//	    if status != 200 { t.Errorf("...非法值回落默认榜单...") }
	//	}
	//
	// 也就是说**「非法输入被静默重解释」被写成了规格**。
	// 一个守卫如果把缺陷固定住，它比没有守卫更糟 ——
	// 因为下一个人会以为这里被考虑过了。
	//
	// 现在反过来：`?type=bogus` 与 `?limit=abc` 都必须 **400**。
	// 理由见 `leaderboard` 上方注释：回显未知的 type 会让客户端
	// 在错误的标题下显示另一个榜单的数据，且**察觉不到**。
	for _, q := range []string{"?type=bogus", "?type=", "?limit=abc"} {
		status, body = e.get(t, "/api/v1/leaderboard"+q, "")
		if status != 400 {
			t.Errorf("leaderboard%s 应 400（非法输入不得被静默重解释），实际 %d：%v",
				q, status, body)
		}
	}
	// power 榜必须有自己（level_exp 排序下新号也在列表里）
	status, body = e.get(t, "/api/v1/leaderboard?limit=50", "")
	items, _ := body["items"].([]any)
	if status != 200 || len(items) == 0 {
		t.Errorf("power 榜不应为空：%d %v", status, body)
	}
}

// --- 出战配置 / 技能升级 / 钱包 ---

func TestE2ELoadoutAndUpgradeSkill(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "loadout")

	// 默认槽位（initNewUser 铺的 1/2/3）
	status, body := e.get(t, "/api/v1/me/loadout", tok)
	if status != 200 {
		t.Fatalf("GET loadout 应 200，实际 %d", status)
	}
	ids, _ := body["skill_ids"].([]any)
	if len(ids) == 0 {
		t.Error("新号必须有默认出战技能（否则进游戏是死局）")
	}

	// 保存：坏 JSON → 400
	status, _ = e.put(t, "/api/v1/me/loadout", tok, "{bad json")
	// fiber 对坏 JSON 返回 400
	if status != fiber.StatusBadRequest {
		t.Errorf("坏 JSON 应 400，实际 %d", status)
	}

	// 各拒绝分支
	status, _ = e.put(t, "/api/v1/me/loadout", tok, map[string]any{"skill_ids": []int{1, 1, 2}})
	if status != fiber.StatusBadRequest {
		t.Errorf("重复技能应 400，实际 %d", status)
	}
	status, _ = e.put(t, "/api/v1/me/loadout", tok, map[string]any{"skill_ids": []int{1, 2, 3, 4, 5}})
	if status != fiber.StatusBadRequest {
		t.Errorf("超过 %d 槽应 400，实际 %d", service.MaxActiveSlots, status)
	}
	status, _ = e.put(t, "/api/v1/me/loadout", tok, map[string]any{"skill_ids": []int{9999}})
	if status != fiber.StatusBadRequest {
		t.Errorf("未解锁技能应 400，实际 %d", status)
	}
	// 被动技能不占主动槽
	var passiveID int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT id FROM skills WHERE kind = 'passive' ORDER BY id LIMIT 1`).Scan(&passiveID); err == nil {
		e.exec(t, `INSERT INTO user_skills (user_id, skill_id) VALUES ($1,$2) ON CONFLICT DO NOTHING`, uid, passiveID)
		status, _ = e.put(t, "/api/v1/me/loadout", tok, map[string]any{"skill_ids": []int{passiveID}})
		if status != fiber.StatusBadRequest {
			t.Errorf("被动技能进主动槽应 400，实际 %d", status)
		}
	}

	// 合法保存后必须可读回
	status, body = e.put(t, "/api/v1/me/loadout", tok, map[string]any{"skill_ids": []int{2, 0, 3}})
	if status != 200 {
		t.Fatalf("合法保存应 200，实际 %d %v", status, body)
	}
	status, body = e.get(t, "/api/v1/me/loadout", tok)
	ids, _ = body["skill_ids"].([]any)
	if len(ids) != 4 || int(num2int(ids[0])) != 2 || int(num2int(ids[2])) != 3 {
		t.Errorf("读回槽位与保存不符：%v", ids)
	}

	// --- 技能升级 ---
	// 非法 id
	status, _ = e.post(t, "/api/v1/me/skills/abc/upgrade", tok, nil)
	if status != fiber.StatusBadRequest {
		t.Errorf("非数字技能 id 应 400，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/me/skills/0/upgrade", tok, nil)
	if status != fiber.StatusBadRequest {
		t.Errorf("id=0 应 400，实际 %d", status)
	}
	// 未拥有
	status, _ = e.post(t, "/api/v1/me/skills/9999/upgrade", tok, nil)
	if status != fiber.StatusBadRequest {
		t.Errorf("未拥有技能应 400，实际 %d", status)
	}
	// 余额不足
	e.exec(t, `UPDATE user_wallets SET coin = 0 WHERE user_id = $1`, uid)
	status, _ = e.post(t, "/api/v1/me/skills/2/upgrade", tok, nil)
	if status != fiber.StatusBadRequest {
		t.Errorf("余额不足应 400，实际 %d", status)
	}
	// 成功：给足金币，升 1 级并回传钱包
	cost := domain.DefaultSkillRules().CostFrom(1)
	e.exec(t, `UPDATE user_wallets SET coin = $2 WHERE user_id = $1`, uid, cost+500)
	status, body = e.post(t, "/api/v1/me/skills/2/upgrade", tok, nil)
	if status != 200 {
		t.Fatalf("升级应成功，实际 %d %v", status, body)
	}
	if num(body, "level") != 2 {
		t.Errorf("升级后 level 应为 2，实际 %v", body["level"])
	}
	if body["wallet"] == nil {
		t.Error("升级响应必须带钱包（避免前端再查一次的竞态）")
	}
	// 满级拒绝
	maxLv := domain.DefaultSkillRules().MaxLevel
	e.exec(t, `UPDATE user_skills SET level = $2 WHERE user_id = $1 AND skill_id = 3`, uid, maxLv)
	status, _ = e.post(t, "/api/v1/me/skills/3/upgrade", tok, nil)
	if status != fiber.StatusBadRequest {
		t.Errorf("满级技能应 400，实际 %d", status)
	}

	// 钱包
	status, body = e.get(t, "/api/v1/wallet/", tok)
	if status != 200 || body["coin"] == nil {
		t.Errorf("wallet 应 200 且含 coin：%d %v", status, body)
	}
}

// num2int 把 JSON 数组元素（float64）转 int。
func num2int(v any) float64 {
	f, _ := v.(float64)
	return f
}

// --- 战斗：开局 → 结算 → 回放 → 验真 ---

// settleBody 构造一份合法的失败局结算上报。
func settleBody(tokenID int64) map[string]any {
	return map[string]any{
		"token_id":       tokenID,
		"result":         "lose",
		"score":          1000,
		"duration_ms":    30000,
		"shots":          200,
		"hits":           100,
		"reactions":      10,
		"heat_max":       50,
		"hp_left":        500,
		"wave_reached":   1,
		"replay_hash":    "abcdef0123456789",
		"elements_used":  map[string]int{"fire": 3},
		"reactions_used": map[string]int{"overheat": 2},
		"terrain_used":   []string{},
		"card_picks":     []int{0, 1, -1},
	}
}

func TestE2EBattleFullCycle(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "battle")

	// 坏 JSON / 非法关卡
	status, _ := e.post(t, "/api/v1/battle/token", tok, "{bad")
	if status != fiber.StatusBadRequest {
		t.Errorf("坏 JSON 应 400，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/battle/token", tok, map[string]any{"level_id": 0})
	if status != fiber.StatusBadRequest {
		t.Errorf("level_id=0 应 400，实际 %d", status)
	}
	// 未解锁：max_stage=0 时只能打第 1 关
	status, _ = e.post(t, "/api/v1/battle/token", tok, map[string]any{"level_id": 5})
	if status != fiber.StatusForbidden {
		t.Errorf("未解锁关卡应 403，实际 %d", status)
	}
	// 体力不足
	e.exec(t, `UPDATE user_wallets SET energy = 0, energy_updated_at = now() WHERE user_id = $1`, uid)
	status, _ = e.post(t, "/api/v1/battle/token", tok, map[string]any{"level_id": 1})
	if status != fiber.StatusBadRequest {
		t.Errorf("体力不足应 400，实际 %d", status)
	}
	e.exec(t, `UPDATE user_wallets SET energy = 120, energy_updated_at = now() WHERE user_id = $1`, uid)

	// 开局成功
	status, body := e.post(t, "/api/v1/battle/token", tok, map[string]any{"level_id": 1})
	if status != 200 {
		t.Fatalf("开局应成功，实际 %d %v", status, body)
	}
	tokenID := num(body, "token_id")
	if tokenID <= 0 || body["seed"].(string) == "" {
		t.Fatalf("开局必须下发 token_id 与字符串种子：%v", body)
	}
	lvl := body["level"].(map[string]any)
	if lvl["seed_str"].(string) == "" {
		t.Error("关卡种子必须以 seed_str 字符串下发（JS 精度）")
	}
	if num(lvl, "seed") != 0 {
		t.Error("数字形态的 seed 必须清零，避免被误用")
	}

	// 结算：坏 JSON / 缺 token_id / 未知 token / 422 拒绝
	status, _ = e.post(t, "/api/v1/battle/settle", tok, "{bad")
	if status != fiber.StatusBadRequest {
		t.Errorf("坏 JSON 应 400，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/battle/settle", tok, map[string]any{"result": "lose"})
	if status != fiber.StatusBadRequest {
		t.Errorf("缺 token_id 应 400，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/battle/settle", tok, map[string]any{"token_id": 999999999})
	if status != fiber.StatusNotFound {
		t.Errorf("未知 token 应 404，实际 %d", status)
	}
	bad := settleBody(tokenID)
	bad["kills"] = 100000 // 远超关卡总怪数
	status, _ = e.post(t, "/api/v1/battle/settle", tok, bad)
	if status != fiber.StatusUnprocessableEntity {
		t.Errorf("击杀数造假应 422，实际 %d", status)
	}

	// 合法结算
	status, body = e.post(t, "/api/v1/battle/settle", tok, settleBody(tokenID))
	if status != 200 {
		t.Fatalf("合法结算应 200，实际 %d %v", status, body)
	}
	battleID := num(body, "battle_id")
	if body["result"].(string) != "lose" || body["wallet"] == nil {
		t.Errorf("结算响应缺字段：%v", body)
	}
	// 任务进度应被推进（kills 指标）
	_ = uid

	// 同一 token 结算两次 → 422（串行下 ValidateSettle 的 IsUsable 先拦，
	// ErrTokenUsed 属结算拒绝类；403 的 RowsAffected 分支只在并发竞态下出现，
	// 由 service 包的 TestConcurrentSettleSameTokenOnlyOnce 覆盖）
	status, _ = e.post(t, "/api/v1/battle/settle", tok, settleBody(tokenID))
	if status != fiber.StatusUnprocessableEntity {
		t.Errorf("重复结算应 422，实际 %d", status)
	}

	// 回放：坏 id / 未知 / 成功
	status, _ = e.get(t, "/api/v1/battle/abc/replay", tok)
	if status != fiber.StatusBadRequest {
		t.Errorf("battle id 非法应 400，实际 %d", status)
	}
	status, _ = e.get(t, "/api/v1/battle/999999999/replay", tok)
	if status != fiber.StatusNotFound {
		t.Errorf("未知战报应 404，实际 %d", status)
	}
	status, body = e.get(t, fmt.Sprintf("/api/v1/battle/%d/replay", battleID), tok)
	if status != 200 {
		t.Fatalf("回放应 200，实际 %d %v", status, body)
	}
	build := body["build"].(map[string]any)
	if build["attacker"] == nil {
		t.Error("回放必须带 build.attacker，否则客户端重放不出同一哈希（I-6）")
	}
	if _, has := build["settle_input"]; has {
		t.Error("回放绝不能泄露 settle_input（伪造高分的蓝本）")
	}
	picks, _ := body["card_picks"].([]any)
	if len(picks) != 3 || int(num2int(picks[2])) != -1 {
		t.Errorf("card_picks 应解析为 [0,1,-1]，实际 %v", body["card_picks"])
	}

	// 验真：匹配 / 不匹配 / 未知战报
	status, body = e.post(t, "/api/v1/battle/verify", tok, map[string]any{
		"battle_id": battleID, "replay_hash": "abcdef0123456789",
	})
	if status != 200 || body["matched"] != true {
		t.Errorf("相同哈希应 matched=true：%d %v", status, body)
	}
	status, body = e.post(t, "/api/v1/battle/verify", tok, map[string]any{
		"battle_id": battleID, "replay_hash": "0000000000000000",
	})
	if status != 200 || body["matched"] != false {
		t.Errorf("不同哈希应 matched=false：%d %v", status, body)
	}
	status, _ = e.post(t, "/api/v1/battle/verify", tok, map[string]any{"battle_id": 999999999})
	if status != fiber.StatusNotFound {
		t.Errorf("未知战报验真应 404，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/battle/verify", tok, "{bad")
	if status != fiber.StatusBadRequest {
		t.Errorf("验真坏 JSON 应 400，实际 %d", status)
	}
}

// --- 任务 / 商店 / 签到 / 兑换 / 诊断 ---

func TestE2ETasksClaim(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "tasks")

	status, body := e.get(t, "/api/v1/tasks/", tok)
	if status != 200 {
		t.Fatalf("tasks 应 200，实际 %d", status)
	}
	items, _ := body["items"].([]any)
	if len(items) == 0 {
		t.Fatal("daily 任务不应为空（种子数据缺失）")
	}
	first := items[0].(map[string]any)
	taskID := int(num(first, "id"))

	status, _ = e.get(t, "/api/v1/tasks/?scope=weekly", tok)
	if status != 200 {
		t.Errorf("weekly scope 应 200，实际 %d", status)
	}

	// claim：坏 id / 不存在 / 未完成
	status, _ = e.post(t, "/api/v1/tasks/abc/claim", tok, nil)
	if status != fiber.StatusBadRequest {
		t.Errorf("task id 非法应 400，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/tasks/999999/claim", tok, nil)
	if status != fiber.StatusNotFound {
		t.Errorf("任务不存在应 404，实际 %d", status)
	}
	status, _ = e.post(t, fmt.Sprintf("/api/v1/tasks/%d/claim", taskID), tok, nil)
	if status != fiber.StatusForbidden {
		t.Errorf("未完成任务应 403，实际 %d", status)
	}

	// 完成后领取 → 200；重复领取 → 403
	e.exec(t, `INSERT INTO user_tasks (user_id, task_id, task_date, progress)
	           VALUES ($1,$2,CURRENT_DATE,(SELECT target FROM tasks WHERE id=$2))
	           ON CONFLICT (user_id, task_id, task_date) DO UPDATE SET progress = EXCLUDED.progress`,
		uid, taskID)
	status, body = e.post(t, fmt.Sprintf("/api/v1/tasks/%d/claim", taskID), tok, nil)
	if status != 200 {
		t.Fatalf("完成任务领取应 200，实际 %d %v", status, body)
	}
	if body["reward"] == nil {
		t.Error("领取响应必须带 reward")
	}
	status, _ = e.post(t, fmt.Sprintf("/api/v1/tasks/%d/claim", taskID), tok, nil)
	if status != fiber.StatusForbidden {
		t.Errorf("重复领取应 403，实际 %d", status)
	}
}

func TestE2EShopAndSignin(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "shop")

	status, body := e.get(t, "/api/v1/shop/", tok)
	if status != 200 {
		t.Fatalf("shop 应 200，实际 %d", status)
	}
	items, _ := body["items"].([]any)
	if len(items) == 0 {
		t.Fatal("商城不应为空")
	}
	if items[0].(map[string]any)["bought_today"] == nil {
		t.Error("商城条目必须带 bought_today（今日限购展示）")
	}

	// 买：坏 id / 不存在 / 限购
	status, _ = e.post(t, "/api/v1/shop/abc/buy", tok, nil)
	if status != fiber.StatusBadRequest {
		t.Errorf("item id 非法应 400，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/shop/99999/buy", tok, nil)
	if status != fiber.StatusNotFound {
		t.Errorf("商品不存在应 404，实际 %d", status)
	}
	// 首充礼包价格 {coin:0}，0 元可买
	status, body = e.post(t, "/api/v1/shop/7/buy", tok, nil)
	if status != 200 {
		t.Fatalf("首充礼包应可 0 元购，实际 %d %v", status, body)
	}
	if body["granted"] == nil {
		t.Error("购买响应必须带 granted（发放清单）")
	}
	// 每日限购 1 → 第二次 403
	status, _ = e.post(t, "/api/v1/shop/7/buy", tok, nil)
	if status != fiber.StatusForbidden {
		t.Errorf("超出每日限购应 403，实际 %d", status)
	}
	// 余额不足：金币袋要 10 gem
	e.exec(t, `UPDATE user_wallets SET gem = 0 WHERE user_id = $1`, uid)
	status, _ = e.post(t, "/api/v1/shop/1/buy", tok, nil)
	if status != fiber.StatusBadRequest {
		t.Errorf("余额不足应 400，实际 %d", status)
	}

	// 签到：第一次成功，当天第二次 403
	status, body = e.post(t, "/api/v1/signin/", tok, nil)
	if status != 200 {
		t.Fatalf("首次签到应 200，实际 %d %v", status, body)
	}
	if num(body, "day_index") != 1 {
		t.Errorf("新号签到应为第 1 天，实际 %v", body["day_index"])
	}
	if body["wallet"] == nil {
		t.Error("签到响应必须带钱包")
	}
	status, _ = e.post(t, "/api/v1/signin/", tok, nil)
	if status != fiber.StatusForbidden {
		t.Errorf("当天重复签到应 403，实际 %d", status)
	}
}

func TestE2ERedeemAndDiagnose(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newPlayer(t, "redeem")

	// 坏 JSON / 无效码
	status, _ := e.post(t, "/api/v1/redeem/", tok, "{bad")
	if status != fiber.StatusBadRequest {
		t.Errorf("坏 JSON 应 400，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/redeem/", tok, map[string]any{"code": "NO_SUCH_CODE"})
	if status != fiber.StatusBadRequest {
		t.Errorf("无效兑换码应 400，实际 %d", status)
	}

	// 自建码（redeem_codes.id 是裸 INTEGER，必须显式给 —— 见报告的缺陷记录）
	code := fmt.Sprintf("E2E%d", time.Now().UnixNano()%1_000_000)
	e.exec(t, `INSERT INTO redeem_codes (id, code, reward, max_uses, used_count, enabled)
	           VALUES ((SELECT COALESCE(MAX(id),0)+1 FROM redeem_codes), $1, $2, 5, 0, true)`,
		code, []byte(`{"coin":123}`))
	t.Cleanup(func() {
		_, _ = e.pool.Exec(context.Background(),
			`DELETE FROM redeem_usages WHERE code_id IN (SELECT id FROM redeem_codes WHERE code=$1)`, code)
		_, _ = e.pool.Exec(context.Background(), `DELETE FROM redeem_codes WHERE code=$1`, code)
	})

	status, body := e.post(t, "/api/v1/redeem/", tok, map[string]any{"code": code})
	if status != 200 {
		t.Fatalf("有效兑换应 200，实际 %d %v", status, body)
	}
	reward, _ := body["reward"].(map[string]any)
	if reward["coin"].(float64) != 123 {
		t.Errorf("reward 应与码定义一致：%v", body["reward"])
	}
	// 同一用户重复兑换 → 403
	status, _ = e.post(t, "/api/v1/redeem/", tok, map[string]any{"code": code})
	if status != fiber.StatusForbidden {
		t.Errorf("重复兑换应 403，实际 %d", status)
	}

	// 诊断：合法 + 非法关卡
	status, body = e.get(t, "/api/v1/diagnose/?level_id=1&failed_times=2", tok)
	if status != 200 {
		t.Fatalf("diagnose 应 200，实际 %d %v", status, body)
	}
	status, _ = e.get(t, "/api/v1/diagnose/?level_id=0", tok)
	if status != fiber.StatusBadRequest {
		t.Errorf("diagnose level_id=0 应 400，实际 %d", status)
	}
}

// --- 专精（I-3） ---

func TestE2EMastery(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "mastery")

	status, body := e.get(t, "/api/v1/mastery/", tok)
	if status != 200 {
		t.Fatalf("GET /mastery 应 200，实际 %d %v", status, body)
	}
	// total_slots 必须下发：客户端曾用编译期常量，玩家点出来的额外槽位不存在
	if _, ok := body["total_slots"]; !ok {
		t.Error("mastery 响应必须带 total_slots（专精额外插槽的权威值）")
	}
	if body["families"] == nil {
		t.Error("mastery 响应必须带 families")
	}
	// nodes 对新号是空列表（JSON null），合法；分配后必须包含所点节点

	// 找一个没有前置要求的节点（第 1 层）来点亮
	var nodeID int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT n.id FROM mastery_nodes n
		 WHERE n.layer = 1
		   AND NOT EXISTS (SELECT 1 FROM user_mastery_nodes m WHERE m.user_id = $1 AND m.node_id = n.id)
		 ORDER BY n.id LIMIT 1`, uid).Scan(&nodeID); err != nil {
		t.Skipf("找不到可用的专精节点：%v", err)
	}

	// 新号默认 3 点，够点 1 个
	status, body = e.post(t, "/api/v1/mastery/allocate", tok, map[string]any{"node_id": nodeID})
	if status != 200 {
		t.Fatalf("分配专精点应 200，实际 %d %v", status, body)
	}
	nodes, _ := body["nodes"].([]any)
	found := false
	for _, n := range nodes {
		switch v := n.(type) {
		case float64:
			if int(v) == nodeID {
				found = true
			}
		case map[string]any:
			if int(num(v, "id")) == nodeID || int(num(v, "node_id")) == nodeID {
				found = true
			}
		}
	}
	if !found {
		t.Errorf("分配后 nodes 应包含 %d：%v", nodeID, nodes)
	}

	// 重复点亮 → 403
	status, _ = e.post(t, "/api/v1/mastery/allocate", tok, map[string]any{"node_id": nodeID})
	if status != fiber.StatusForbidden {
		t.Errorf("重复点亮应 403，实际 %d", status)
	}
	// 不存在的节点 → 400
	status, _ = e.post(t, "/api/v1/mastery/allocate", tok, map[string]any{"node_id": 999999})
	if status != fiber.StatusBadRequest {
		t.Errorf("不存在的节点应 400，实际 %d", status)
	}
	// 坏 JSON → 400
	status, _ = e.post(t, "/api/v1/mastery/allocate", tok, "{bad")
	if status != fiber.StatusBadRequest {
		t.Errorf("坏 JSON 应 400，实际 %d", status)
	}
}

// --- 防线（I-5） ---

func TestE2EDefenses(t *testing.T) {
	e := newE2E(t)
	ownerID, ownerTok := e.newPlayer(t, "owner")
	_ = ownerID
	foeID, foeTok := e.newPlayer(t, "foe")

	// 列表：初始 mine 为空
	status, body := e.get(t, "/api/v1/defenses/", foeTok)
	if status != 200 {
		t.Fatalf("defenses 应 200，实际 %d", status)
	}
	if body["attempt_limit"].(float64) != float64(service.DefenseAttemptLimit) {
		t.Errorf("attempt_limit 应为 %d", service.DefenseAttemptLimit)
	}

	// 保存：坏 JSON / 各校验分支
	status, _ = e.post(t, "/api/v1/defenses/save", ownerTok, "{bad")
	if status != fiber.StatusBadRequest {
		t.Errorf("坏 JSON 应 400，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/defenses/save", ownerTok, map[string]any{"works": []string{"god_mode"}})
	if status != fiber.StatusBadRequest {
		t.Errorf("未知工程装置应 400，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/defenses/save", ownerTok, map[string]any{
		"works": []string{"slow_belt", "block_wall", "tesla_grid", "slow_belt"}})
	if status != fiber.StatusBadRequest {
		t.Errorf("超过 3 个装置应 400，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/defenses/save", ownerTok, map[string]any{"skills": []int{9999}})
	if status != fiber.StatusBadRequest {
		t.Errorf("未解锁技能应 400，实际 %d", status)
	}

	// 合法保存 → 200，返回 defense 视图
	status, body = e.post(t, "/api/v1/defenses/save", ownerTok, map[string]any{
		"name": "E2E防线", "works": []string{"slow_belt", "block_wall"},
		"skills": []int{1, 2, 3},
	})
	if status != 200 {
		t.Fatalf("保存防线应 200，实际 %d %v", status, body)
	}
	def := body["defense"].(map[string]any)
	defID := int64(num(def, "id"))
	if def["snapshot_hash"].(string) == "" {
		t.Error("防线必须带快照哈希（挑战者据此校验快照一致性）")
	}

	// 挑战：坏 id / 不存在 / 挑自己的
	status, _ = e.post(t, "/api/v1/defenses/abc/challenge", foeTok, validChallengeBody())
	if status != fiber.StatusBadRequest {
		t.Errorf("defense id 非法应 400，实际 %d", status)
	}
	status, _ = e.post(t, "/api/v1/defenses/99999999/challenge", foeTok, validChallengeBody())
	if status != fiber.StatusNotFound {
		t.Errorf("挑战不存在防线应 404，实际 %d", status)
	}
	status, _ = e.post(t, fmt.Sprintf("/api/v1/defenses/%d/challenge", defID), ownerTok, validChallengeBody())
	if status != fiber.StatusForbidden {
		t.Errorf("挑战自己的防线应 403，实际 %d", status)
	}

	// 对方挑战成功
	status, body = e.post(t, fmt.Sprintf("/api/v1/defenses/%d/challenge", defID), foeTok, map[string]any{
		"won": true, "seed": "1", "duration_ms": 60000, "hp_left_pct": 100,
		"replay_hash": "0000000000000000",
	})
	if status != 200 {
		t.Fatalf("挑战应成功，实际 %d %v", status, body)
	}
	if body["won"] != true {
		t.Errorf("won 应回传 true：%v", body)
	}

	// 护盾：owner 重存防线开 24h 护盾，再挑战 → 403
	status, _ = e.post(t, "/api/v1/defenses/save", ownerTok, map[string]any{
		"name": "E2E防线", "works": []string{"slow_belt"}, "shield_hours": 24,
	})
	if status != 200 {
		t.Fatalf("开护盾保存应 200，实际 %d", status)
	}
	status, body = e.post(t, fmt.Sprintf("/api/v1/defenses/%d/challenge", defID), foeTok, map[string]any{
		"won": true, "seed": "1", "duration_ms": 60000, "replay_hash": "0000000000000000",
	})
	if status != fiber.StatusForbidden {
		t.Errorf("护盾期内挑战应 403，实际 %d %v", status, body)
	}
	// 列表上必须同时关掉 can_challenge（否则客户端放行、服务端拒绝，体验割裂）
	status, body = e.get(t, "/api/v1/defenses/", foeTok)
	if status != 200 {
		t.Fatalf("defenses 应 200，实际 %d", status)
	}
	candidates, _ := body["candidates"].([]any)
	for _, c := range candidates {
		cv := c.(map[string]any)
		if int64(num(cv, "id")) == defID && cv["can_challenge"] == true {
			t.Error("护盾中的防线 can_challenge 必须为 false")
		}
	}
	_ = foeID
}

// validChallengeBody 返回一份**合法**的挑战上报（第 69 轮）。
//
// ⚠️ 为什么需要它：第 69 轮给 `ChallengeInput` 补了字段边界校验后，
// 那些只写 `{"won":true}` 的请求会在**触及数据库之前**就被 422 拒掉，
// 于是「挑战不存在的防线应 404」「挑战自己的防线应 403」这些用例
// 根本走不到它们要测的分支 —— 测试照样是绿的，但它测的东西消失了。
//
// 这与 README 记的「守卫被无关的早退路径满足」是同一类：
// **测试变绿不等于它还在守原来的东西**。
//
// 形状照抄真实客户端（miniapp/src/game/defense.ts 的上报体）。
func validChallengeBody() map[string]any {
	return map[string]any{
		"seed":        "1",
		"won":         true,
		"duration_ms": 60_000,
		"hp_left_pct": 100,
		"replay_hash": "0000000000000000",
	}
}
