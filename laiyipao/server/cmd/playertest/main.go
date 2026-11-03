// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// playertest — R82 玩家主链路 SLO 基线压测器。
//
// # 与 cmd/loadtest 的分工
//
// loadtest 打「无状态单 URL GET」。玩家主链路大多是有状态 POST 流程：
// 建号每次造新实体、refresh 轮换令牌（必须链式）、签到/领奖/兑换是
// 「每账号一次性」、结算要真回放哈希。这些形状塞不进单 URL 模型，
// 所以有本场景化压测器：每场景自己预置状态（SQL 直插，与种子/夹具同形状），
// worker 走真实 HTTP API（限流/中间件/序列化全在路径上）。
//
// # SLO 预算（R82 要求 2：先定预算再测，不是测完按结果编）
//
// 依据：玩家可感知的交互等待 <300ms 为「无感」（与 R80 结算同口径）；
// 首次进入是冷启动最敏感一步，稍宽；后台静默操作最宽。
//
//	/auth/guest       建号（首次进入）                  p95 ≤ 500ms
//	/auth/login       回归登录（bcrypt 硬成本 50~100ms） p95 ≤ 400ms
//	/auth/refresh     后台静默续期（玩家无感）           p95 ≤ 500ms
//	/signin           交互点击                          p95 ≤ 300ms
//	/tasks/:id/claim  交互点击                          p95 ≤ 300ms
//	/redeem           交互点击                          p95 ≤ 300ms
//	/me/battles       列表读（R60 基线 5.32ms）          p95 ≤ 300ms
//	/battle/settle    交互（R80 已定）                   p95 ≤ 300ms
//
// # 限流（R82 要求 4）
//
// 被测 API 以 RATE_LIMIT_SCALE=100 启动（生产=1.0，启动日志有 ⚠️ 行）。
// 本工具测的是**路径本身**的延迟，不是「含限流阈值的排队」；
// 报告里必须连同 scale 一起写，别把放宽后的数字当成生产数字。
// 延迟本身与限流无关（限流器成本 <1µs，R76 实测），scale 只影响 429 出现率。
//
// # 用法
//
//	go run ./cmd/playertest -dsn postgres://postgres@127.0.0.1:5432/laiyipao_r82_perf?sslmode=disable \
//	  -api http://127.0.0.1:8080 -scenario all -c 20 -d 12s -json /tmp/r82_c20.json
package main

import (
	"context"
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"net/http"
	"os"
	"sort"
	"strings"
	"sync"
	"sync/atomic"
	"time"

	"github.com/jackc/pgx/v5"
	"golang.org/x/crypto/bcrypt"

	"github.com/laiyipao/server/internal/domain"
)

// ---------- 预算表（判定在汇总时做，FAIL 非零退出） ----------

var budgets = map[string]float64{ // p95 毫秒
	"/auth/guest":      500,
	"/auth/login":      400,
	"/auth/refresh":    500,
	"/signin":          300,
	"/tasks/:id/claim": 300,
	"/redeem":          300,
	"/me/battles":      300,
	"/battle/settle":   300,
}

// ---------- 统计 ----------

type stats struct {
	mu     sync.Mutex
	lats   []float64 // 毫秒（仅 2xx）
	status map[int]int
	errs   int
	total  int
}

func (s *stats) add(code int, ms float64) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.total++
	s.status[code]++
	if code < 200 || code >= 300 {
		s.errs++
		return
	}
	s.lats = append(s.lats, ms)
}

func pct(lats []float64, p float64) float64 {
	if len(lats) == 0 {
		return 0
	}
	i := int(p * float64(len(lats)-1))
	if i < 0 {
		i = 0
	}
	if i >= len(lats) {
		i = len(lats) - 1
	}
	return lats[i]
}

type result struct {
	Label    string  `json:"label"`
	C        int     `json:"c"`
	Total    int     `json:"total"`
	QPS      float64 `json:"qps"`
	ErrRate  float64 `json:"err_rate"`
	P50Ms    float64 `json:"p50_ms"`
	P95Ms    float64 `json:"p95_ms"`
	P99Ms    float64 `json:"p99_ms"`
	BudgetMs float64 `json:"budget_ms"`
	Pass     bool    `json:"pass"`
}

func (s *stats) report(label string, c int, elapsed time.Duration) result {
	s.mu.Lock()
	defer s.mu.Unlock()
	sort.Float64s(s.lats)
	r := result{Label: label, C: c, Total: s.total, BudgetMs: budgets[label]}
	if elapsed > 0 {
		r.QPS = float64(s.total) / elapsed.Seconds()
	}
	if s.total > 0 {
		r.ErrRate = float64(s.errs) / float64(s.total) * 100
	}
	r.P50Ms, r.P95Ms, r.P99Ms = pct(s.lats, 0.50), pct(s.lats, 0.95), pct(s.lats, 0.99)
	r.Pass = r.ErrRate == 0 && r.P95Ms <= r.BudgetMs
	fmt.Printf("label=%s c=%d total=%d qps=%.1f err_rate=%.2f%% p50=%.2fms p95=%.2fms p99=%.2fms budget_p95=%.0fms %s\n",
		r.Label, r.C, r.Total, r.QPS, r.ErrRate, r.P50Ms, r.P95Ms, r.P99Ms, r.BudgetMs,
		map[bool]string{true: "PASS", false: "FAIL"}[r.Pass])
	for code, n := range s.status {
		if code < 200 || code >= 300 {
			fmt.Printf("  status %d=%d\n", code, n)
		}
	}
	return r
}

// ---------- 场景上下文与小助手 ----------

type sceneCtx struct {
	api string
	db  *pgx.Conn
	c   int
	dur time.Duration
	tag string // 本批次唯一前缀（guest_token/用户名/码）
}

// errDumpLeft 是**每场景**的错误详情打印配额（各 3 条；bench 开始时重置）。
var errDumpLeft atomic.Int32

// call 打一次 HTTP；返回状态码、响应体（截断 4KB）与耗时（毫秒）。
// 非 2xx 的前 3 个 body 打到 stderr 便于诊断。
func (sc *sceneCtx) call(client *http.Client, method, path, token string, body any) (int, []byte, float64) {
	var rdr io.Reader
	if body != nil {
		raw, _ := json.Marshal(body)
		rdr = strings.NewReader(string(raw))
	}
	req, err := http.NewRequest(method, sc.api+path, rdr)
	if err != nil {
		return 0, nil, 0
	}
	if body != nil {
		req.Header.Set("Content-Type", "application/json")
	}
	if token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	start := time.Now()
	resp, err := client.Do(req)
	if err != nil {
		if errDumpLeft.Add(-1) > 0 {
			fmt.Fprintf(os.Stderr, "  [err] %s %s → 客户端错误: %v\n", method, path, err)
		}
		return 0, nil, 0
	}
	defer resp.Body.Close()
	// 必须读全 body：开战响应含完整关卡+波次+构筑（几十 KB），
	// 截断会让调用方拿到半截 JSON（R82 实测：settle 场景 73% 假失败）。
	// 错误打印时再截断，不影响正常路径。
	raw, _ := io.ReadAll(resp.Body)
	ms := float64(time.Since(start).Microseconds()) / 1000.0
	if (resp.StatusCode < 200 || resp.StatusCode >= 300) && errDumpLeft.Add(-1) > 0 {
		preview := raw
		if len(preview) > 512 {
			preview = preview[:512]
		}
		fmt.Fprintf(os.Stderr, "  [err] %s %s → %d body=%s\n", method, path, resp.StatusCode, preview)
	}
	return resp.StatusCode, raw, ms
}

func newClient(conc int) *http.Client {
	return &http.Client{
		Transport: &http.Transport{
			MaxIdleConns:        conc * 2,
			MaxIdleConnsPerHost: conc,
			IdleConnTimeout:     30 * time.Second,
			DisableCompression:  true,
		},
		Timeout: 60 * time.Second,
	}
}

// bench 闭环 worker 模型：work 返回一次操作的 (code, ms)；false 让 worker 停止（一次性场景）。
func (sc *sceneCtx) bench(label string, work func(w int) (int, float64, bool)) result {
	errDumpLeft.Store(3)
	st := stats{status: map[int]int{}}
	ctx, cancel := context.WithTimeout(context.Background(), sc.dur)
	defer cancel()
	var wg sync.WaitGroup
	for w := 0; w < sc.c; w++ {
		wg.Add(1)
		go func(w int) {
			defer wg.Done()
			for ctx.Err() == nil {
				code, ms, more := work(w)
				st.add(code, ms)
				if !more {
					return
				}
			}
		}(w)
	}
	wg.Wait()
	return st.report(label, sc.c, sc.dur)
}

// preseedUsers 批量直插 users + 初始包钱包 + user_progress，返回新用户 id 列表。
// 形状与种子/夹具一致（初始包 coin2000/gem50/keys1 直插不写流水 —— R78 对账口径）。
func (sc *sceneCtx) preseedUsers(prefix string, n int) []int64 {
	ctx := context.Background()
	tag := sc.tag
	rows, err := sc.db.Query(ctx, `
		WITH ins AS (
			INSERT INTO users (guest_token, nickname, is_guest)
			SELECT $1 || '_' || g, 'r82', true FROM generate_series(1, $2) g
			RETURNING id
		), w AS (
			INSERT INTO user_wallets (user_id, coin, gem, keys, energy)
			SELECT id, 2000, 50, 1, 100000 FROM ins
		), p AS (
			INSERT INTO user_progress (user_id, max_stage, level_exp)
			SELECT id, 1, 0 FROM ins
		)
		SELECT id FROM ins`, prefix+"_"+tag, n)
	if err != nil {
		fmt.Fprintf(os.Stderr, "预置用户失败: %v\n", err)
		os.Exit(1)
	}
	defer rows.Close()
	var ids []int64
	for rows.Next() {
		var id int64
		if rows.Scan(&id) == nil {
			ids = append(ids, id)
		}
	}
	return ids
}

// guestToken 用预置的 guest_token 换令牌对（不计样本的前置）。
func (sc *sceneCtx) guestToken(client *http.Client, guestToken string) (string, string) {
	code, raw, _ := sc.call(client, http.MethodPost, "/api/v1/auth/guest", "",
		map[string]any{"guest_token": guestToken})
	if code != 200 {
		return "", ""
	}
	var tp struct {
		AccessToken  string `json:"access_token"`
		RefreshToken string `json:"refresh_token"`
	}
	if json.Unmarshal(raw, &tp) != nil {
		return "", ""
	}
	return tp.AccessToken, tp.RefreshToken
}

// ---------- 场景 ----------

// guest：每次 POST 都造新号（空 guest_token）。无预置。
func sceneGuest(sc *sceneCtx) result {
	client := newClient(sc.c)
	return sc.bench("/auth/guest", func(w int) (int, float64, bool) {
		code, _, ms := sc.call(client, http.MethodPost, "/api/v1/auth/guest", "",
			map[string]any{"nickname": "r82压测"})
		return code, ms, true
	})
}

// login：预置 64 个账号密码用户；worker 循环真实 bcrypt 登录。
func sceneLogin(sc *sceneCtx) result {
	const accounts = 64
	pw := "R82bench-pass"
	hash, _ := bcrypt.GenerateFromPassword([]byte(pw), 10)
	ctx := context.Background()
	for i := 0; i < accounts; i++ {
		if _, err := sc.db.Exec(ctx, `
			INSERT INTO users (guest_token, nickname, is_guest, username, password_hash)
			VALUES ($1, 'r82', false, $2, $3)`,
			fmt.Sprintf("r82_lg_%s_%d", sc.tag, i),
			fmt.Sprintf("r82lg_%s_%d", sc.tag, i), string(hash)); err != nil {
			fmt.Fprintf(os.Stderr, "预置登录账号失败: %v\n", err)
			os.Exit(1)
		}
	}
	client := newClient(sc.c)
	return sc.bench("/auth/login", func(w int) (int, float64, bool) {
		code, _, ms := sc.call(client, http.MethodPost, "/api/v1/auth/login", "",
			map[string]any{"username": fmt.Sprintf("r82lg_%s_%d", sc.tag, w%accounts), "password": pw})
		return code, ms, true
	})
}

// refresh：每 worker 先建号拿 refresh_token，之后链式轮换（每次响应的
// 新 token 供下一次用 —— R33 轮换语义下不链式就全是 401）。
func sceneRefresh(sc *sceneCtx) result {
	client := newClient(sc.c)
	type chain struct{ refresh string }
	chains := make([]chain, sc.c)
	for w := 0; w < sc.c; w++ {
		_, rt := sc.guestToken(client, fmt.Sprintf("r82_rf_%s_%d", sc.tag, w))
		if rt == "" {
			fmt.Fprintf(os.Stderr, "refresh 前置建号失败 worker=%d\n", w)
			os.Exit(1)
		}
		chains[w].refresh = rt
	}
	return sc.bench("/auth/refresh", func(w int) (int, float64, bool) {
		code, raw, ms := sc.call(client, http.MethodPost, "/api/v1/auth/refresh", "",
			map[string]any{"refresh_token": chains[w].refresh})
		var tp struct {
			RefreshToken string `json:"refresh_token"`
		}
		if code == 200 && json.Unmarshal(raw, &tp) == nil && tp.RefreshToken != "" {
			chains[w].refresh = tp.RefreshToken
		}
		return code, ms, true
	})
}

// oneShotAccount 是一次性场景（签到/领奖）的账号+令牌对。
type oneShotAccount struct {
	guestToken string
	access     string
}

// prepareOneShot 预置 N 账号并逐一换好 access token（前置，不计样本）。
func (sc *sceneCtx) prepareOneShot(prefix string, n int) []oneShotAccount {
	ids := sc.preseedUsers(prefix, n)
	client := newClient(sc.c)
	out := make([]oneShotAccount, len(ids))
	var idx atomic.Int64
	var wg sync.WaitGroup
	for w := 0; w < 16; w++ { // 前置登录也并行，不然 6000 次串行登录要等很久
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				i := int(idx.Add(1) - 1)
				if i >= len(ids) {
					return
				}
				gt := fmt.Sprintf("%s_%s_%d", prefix, sc.tag, i+1) // 与 preseedUsers 的 generate_series(1,n) 对齐
				at, _ := sc.guestToken(client, gt)
				out[i] = oneShotAccount{guestToken: gt, access: at}
			}
		}()
	}
	wg.Wait()
	return out
}

// signin：每账号恰好一次（第二次必 409，那测的是幂等拒绝不是路径）。
func sceneSignin(sc *sceneCtx) result {
	accs := sc.prepareOneShot("r82_si", 6000)
	client := newClient(sc.c)
	var idx atomic.Int64
	return sc.bench("/signin", func(w int) (int, float64, bool) {
		i := int(idx.Add(1) - 1)
		if i >= len(accs) {
			return 200, 0, false
		}
		code, _, ms := sc.call(client, http.MethodPost, "/api/v1/signin/", accs[i].access, nil)
		return code, ms, true
	})
}

// taskclaim：预置每账号一条「完成未领」的 daily 任务行。
func sceneTaskclaim(sc *sceneCtx) result {
	ctx := context.Background()
	var taskID, target int
	if err := sc.db.QueryRow(ctx,
		`SELECT id, target FROM tasks WHERE scope='daily' AND enabled ORDER BY id LIMIT 1`).
		Scan(&taskID, &target); err != nil {
		fmt.Fprintf(os.Stderr, "找不到 daily 任务（seed 没灌？）: %v\n", err)
		os.Exit(1)
	}
	accs := sc.prepareOneShot("r82_tc", 6000)
	// 批量铺「完成未领」进度行（task_date=本周期起点=daily→今天）。
	if _, err := sc.db.Exec(ctx, `
		INSERT INTO user_tasks (user_id, task_id, task_date, progress)
		SELECT u.id, $1, CURRENT_DATE, $2 FROM users u
		 WHERE u.guest_token LIKE 'r82_tc_' || $3 || '_%'
		ON CONFLICT (user_id, task_id, task_date) DO NOTHING`,
		taskID, target, sc.tag); err != nil {
		fmt.Fprintf(os.Stderr, "预置任务进度失败: %v\n", err)
		os.Exit(1)
	}
	client := newClient(sc.c)
	var idx atomic.Int64
	path := fmt.Sprintf("/api/v1/tasks/%d/claim", taskID)
	return sc.bench("/tasks/:id/claim", func(w int) (int, float64, bool) {
		i := int(idx.Add(1) - 1)
		if i >= len(accs) {
			return 200, 0, false
		}
		code, _, ms := sc.call(client, http.MethodPost, path, accs[i].access, nil)
		return code, ms, true
	})
}

// redeem：预置 20000 个码（每码大 max_uses）+ 64 个账号；
// worker 循环兑**不同**码（同码同号一次，这是路径语义不是限流）。
func sceneRedeem(sc *sceneCtx) result {
	ctx := context.Background()
	const codes = 20000
	// 幂等重跑：清掉本工具的所有历史码与使用记录（id 段固定 900001+，
	// ON CONFLICT 跳过会让「上一轮 tag 的码」顶替本轮 —— R82 实测踩过）。
	// usages 只有 code_id 没有 code 文本列（00006），按 id 段删。
	if _, err := sc.db.Exec(ctx, `
		DELETE FROM redeem_usages WHERE code_id BETWEEN 900001 AND 920000;
		DELETE FROM redeem_codes WHERE code LIKE 'R82B\_%'`); err != nil {
		fmt.Fprintf(os.Stderr, "清理历史兑换码失败: %v\n", err)
		os.Exit(1)
	}
	if _, err := sc.db.Exec(ctx, `
		INSERT INTO redeem_codes (id, code, reward, max_uses, enabled)
		SELECT 900000 + g, 'R82B_' || $1 || '_' || g, '{"coin": 1}', 100000, true
		  FROM generate_series(1, $2) g`, sc.tag, codes); err != nil {
		fmt.Fprintf(os.Stderr, "预置兑换码失败: %v\n", err)
		os.Exit(1)
	}
	accs := sc.prepareOneShot("r82_rd", 64)
	client := newClient(sc.c)
	var pos atomic.Int64
	return sc.bench("/redeem", func(w int) (int, float64, bool) {
		p := pos.Add(1) - 1
		if p >= codes {
			return 200, 0, false
		}
		code, _, ms := sc.call(client, http.MethodPost, "/api/v1/redeem/", accs[w%len(accs)].access,
			map[string]any{"code": fmt.Sprintf("R82B_%s_%d", sc.tag, p+1)})
		return code, ms, true
	})
}

// mebattles：单账号循环读（R60 基线路径，对照用）。
func sceneMebattles(sc *sceneCtx) result {
	accs := sc.prepareOneShot("r82_mb", 1)
	client := newClient(sc.c)
	return sc.bench("/me/battles", func(w int) (int, float64, bool) {
		code, _, ms := sc.call(client, http.MethodGet, "/api/v1/me/battles", accs[0].access, nil)
		return code, ms, true
	})
}

// settle：真 token + 真回放哈希的完整结算（R81 同组装，走 HTTP）。
// 样本只计 /battle/settle 段；/battle/token 是闭环前置（不占样本，
// 报告里说明：端到端体感 ≈ token 段 + settle 段）。
func sceneSettle(sc *sceneCtx) result {
	accs := sc.prepareOneShot("r82_st", 64)
	client := newClient(sc.c)
	return sc.bench("/battle/settle", func(w int) (int, float64, bool) {
		acc := accs[w%len(accs)]
		// 1) 开战拿 token + seed + 构筑（前置，不计样本）
		code, raw, _ := sc.call(client, http.MethodPost, "/api/v1/battle/token", acc.access,
			map[string]any{"level_id": 1})
		if code != 200 {
			return code, 0, true
		}
		var tok struct {
			TokenID int64          `json:"token_id"`
			Seed    string         `json:"seed"`
			Build   map[string]any `json:"build"`
		}
		if err := json.Unmarshal(raw, &tok); err != nil {
			if errDumpLeft.Add(-1) > 0 {
				fmt.Fprintf(os.Stderr, "  [err] settle token 响应解码失败: %v raw=%.200s\n", err, raw)
			}
			return 0, 0, true
		}
		// 2) 真哈希（与 r80Settle 同组装）
		buildRaw, _ := json.Marshal(tok.Build)
		var bs domain.BuildSnapshot
		if err := json.Unmarshal(buildRaw, &bs); err != nil {
			if errDumpLeft.Add(-1) > 0 {
				fmt.Fprintf(os.Stderr, "  [err] settle 构筑解码失败: %v\n", err)
			}
			return 0, 0, true
		}
		out := domain.Replay(domain.GenerateLevel(1), bs, tok.Seed,
			[]int{0, 1, -1}, "", domain.EnemiesByID(), domain.SkillsByID(), domain.DefaultScoreRules())
		if out.Err != nil {
			if errDumpLeft.Add(-1) > 0 {
				fmt.Fprintf(os.Stderr, "  [err] settle 重算失败: %v\n", out.Err)
			}
			return 0, 0, true
		}
		// 3) 结算（计样本）
		code, _, ms := sc.call(client, http.MethodPost, "/api/v1/battle/settle", acc.access,
			map[string]any{
				"token_id": tok.TokenID, "result": "lose", "score": 1000, "duration_ms": 30000,
				"shots": 200, "hits": 100, "reactions": 20, "heat_max": 50,
				"hp_left": 500, "wave_reached": 1, "kills": 30, "leaked": 0,
				"replay_hash": out.ComputedHash, "elements_used": map[string]int{"fire": 3},
				"reactions_used": map[string]int{"overheat": 20}, "terrain_used": []string{},
				"card_picks": []int{0, 1, -1},
			})
		return code, ms, true
	})
}

// ---------- main ----------

func main() {
	dsn := flag.String("dsn", "", "预置状态的直连 DSN（必填）")
	api := flag.String("api", "http://127.0.0.1:8080", "被测 API 根")
	scenario := flag.String("scenario", "all", "all|guest|login|refresh|signin|taskclaim|redeem|mebattles|settle")
	conc := flag.Int("c", 20, "并发 worker 数")
	dur := flag.Duration("d", 12*time.Second, "每场景时长上限（一次性场景提前耗尽账号即停）")
	jsonOut := flag.String("json", "", "可选：结果 JSON 输出路径（check-slo 的输入）")
	flag.Parse()

	if *dsn == "" {
		fmt.Fprintln(os.Stderr, "必须给 -dsn（预置状态要直连库）")
		os.Exit(2)
	}
	db, err := pgx.Connect(context.Background(), *dsn)
	if err != nil {
		fmt.Fprintln(os.Stderr, "连库失败:", err)
		os.Exit(1)
	}
	defer db.Close(context.Background())

	sc := &sceneCtx{
		api: strings.TrimRight(*api, "/"), db: db, c: *conc, dur: *dur,
		tag: fmt.Sprintf("%d", time.Now().UnixNano()%1_000_000_000),
	}

	scenes := map[string]func(*sceneCtx) result{
		"guest": sceneGuest, "login": sceneLogin, "refresh": sceneRefresh,
		"signin": sceneSignin, "taskclaim": sceneTaskclaim, "redeem": sceneRedeem,
		"mebattles": sceneMebattles, "settle": sceneSettle,
	}
	order := []string{"guest", "login", "refresh", "signin", "taskclaim", "redeem", "mebattles", "settle"}

	var results []result
	fail := false
	for _, name := range order {
		if *scenario != "all" && *scenario != name {
			continue
		}
		fmt.Printf("== scenario %s (c=%d) ==\n", name, *conc)
		r := scenes[name](sc)
		results = append(results, r)
		if !r.Pass {
			fail = true
		}
	}

	if *jsonOut != "" {
		raw, _ := json.MarshalIndent(results, "", "  ")
		if err := os.WriteFile(*jsonOut, raw, 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "写 JSON 失败:", err)
			os.Exit(1)
		}
	}
	if fail {
		os.Exit(1)
	}
}
