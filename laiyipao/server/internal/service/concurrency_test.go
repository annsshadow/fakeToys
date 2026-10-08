package service

import (
	"context"
	"errors"
	"fmt"
	"os"
	"runtime"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/store"
)

// 并发安全集成测试。
//
// 为什么必须用真实并发验证：Buy 的每日限购与 ClaimTask 的重复领取，
// 漏洞都**只在并发下出现**。静态读代码能看到"缺了 FOR UPDATE"，
// 但真正判据是"N 个并发请求实际只生效一次"。
//
// 这类推理历史上错过多次 —— 正确写法与错误写法在单线程下
// 行为完全一致，静态判断不可靠。所以这里打真实数据库。
//
// 需要可用的 DATABASE_URL。默认连本机 laiyipao 库；
// 连不上则 Skip（而不是 Fail），这样无 DB 环境下纯逻辑单测仍能跑。
// ⚠️ 测试会创建真实用户与订单，跑之前请指向一个可丢弃的库。

// testService 包装 Service 并提供夹具方法。
type testService struct {
	*Service
	db *store.DB
}

// openTestService 连库。失败时 Skip。
func openTestService(t *testing.T) *testService {
	t.Helper()
	dsn := os.Getenv("TEST_DATABASE_URL")
	if dsn == "" {
		dsn = "postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable"
	}
	cfg := config.Load()
	cfg.DatabaseURL = dsn
	// 短 TTL，避免测试签发的令牌在库里堆积
	cfg.AccessTTL = 5 * time.Minute

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	db, err := store.Open(ctx, cfg)
	if err != nil {
		t.Skipf("无可用测试数据库（%s），跳过：%v", dsn, err)
	}
	// 探测连通性：连不上就跳过而不是让后面每个用例都失败
	probe, probeCancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer probeCancel()
	if err := db.Pool.Ping(probe); err != nil {
		db.Close()
		t.Skipf("数据库不可达，跳过：%v", err)
	}
	t.Cleanup(func() { db.Close() })
	return &testService{Service: New(db, cfg), db: db}
}

// runParallel 启动 N 个 goroutine 在同一时刻冲过 start 闸门，
// 返回成功数、错误列表，以及**同时在飞的峰值**。
//
// ⚠️ 峰值是必需的，不是可选的诊断信息。
// `close(start)` 只是**释放门**，不是会合点：排在 close 之后才被调度的
// goroutine 会直接通过，没有任何重叠保证。纯内存的 fn 很容易整体串行执行，
// 而"只生效一次"那几条断言在串行下**照样通过** ——
// 于是测试变成"可能撞上竞态的探测器"，日志里也看不出差别。
//
// 有了 peak，每个用例可以断言 peak >= 2，从而证明自己确实在并发；
// 将来若有人把 fn 挪出 goroutine 或让 GOMAXPROCS=1，测试会立刻红。
func runParallel(workers int, fn func(i int) error) (okCount int, errs []error, peak int64) {
	var wg sync.WaitGroup
	var mu sync.Mutex
	var inFlight, peakDepth int64
	start := make(chan struct{})
	for i := 0; i < workers; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			<-start
			// 进入临界区：登记在飞数并维护峰值
			n := atomic.AddInt64(&inFlight, 1)
			for {
				p := atomic.LoadInt64(&peakDepth)
				if n <= p || atomic.CompareAndSwapInt64(&peakDepth, p, n) {
					break
				}
			}
			// 让出一次，给兄弟 goroutine 留出重叠窗口。
			// 不加这一行时，先到的 goroutine 可能在峰值登记前就把活干完了。
			defer atomic.AddInt64(&inFlight, -1)
			runtime.Gosched()

			err := fn(idx)

			mu.Lock()
			defer mu.Unlock()
			if err == nil {
				okCount++
			} else {
				errs = append(errs, err)
			}
		}(i)
	}
	close(start)
	wg.Wait()
	return okCount, errs, atomic.LoadInt64(&peakDepth)
}

// mustOverlap 断言确实产生了并发重叠。
//
// peak < 2 说明所有 fn 都是串行跑的，本用例的结论无效 ——
// 「只生效一次」在串行下必然成立，于是断言会变成永远为真的废话。
//
// 峰值同时打进测试日志：出问题时能直接看出并发窗口有多宽，
// 而不是只能猜测"是不是根本没并发"。
func mustOverlap(t *testing.T, peak int64, workers int) {
	t.Helper()
	if peak < 2 {
		t.Fatalf("峰值在飞数 %d：%d 个 worker 实际串行执行，"+
			"本用例已失去意义（串行下「只生效一次」必然成立）。"+
			"检查 runParallel 是否还在用 go func 启动 worker", peak, workers)
	}
	t.Logf("并发窗口：%d 个 worker，峰值同时在飞 %d", workers, peak)
}

func (ts *testService) newUser(t *testing.T, ctx context.Context) int64 {
	t.Helper()
	tp, err := ts.GuestLogin(ctx, "", "并发测试")
	if err != nil {
		t.Fatalf("建号失败：%v", err)
	}
	return tp.User.ID
}

func (ts *testService) wallet(t *testing.T, ctx context.Context, userID int64) Wallet {
	t.Helper()
	w, err := ts.LoadWallet(ctx, userID)
	if err != nil {
		t.Fatalf("读钱包失败：%v", err)
	}
	return w
}

func (ts *testService) grant(t *testing.T, ctx context.Context, userID int64, deltas map[string]int64) {
	t.Helper()
	if err := ts.DB.Tx(ctx, func(tx pgx.Tx) error {
		return ts.grantWallet(ctx, tx, userID, deltas, "test_grant", 0)
	}); err != nil {
		t.Fatalf("发放测试货币失败：%v", err)
	}
}

func (ts *testService) scalar(t *testing.T, ctx context.Context, q string, args ...any) int {
	t.Helper()
	var n int
	if err := ts.pool.QueryRow(ctx, q, args...).Scan(&n); err != nil {
		t.Fatalf("查询失败（%s）：%v", q, err)
	}
	return n
}

// ---- 用例 ----

// TestConcurrentBuyRespectsDailyLimit 覆盖 C3（Critical）。
//
// 作弊手法：并发打 N 个 /shop/{id}/buy。旧实现的限购是
// 「SELECT COUNT(*) 再 INSERT」，无任何锁 —— N 个事务全部在对方
// INSERT 之前读到 bought=0，于是全部放行。shop_firstpay 价格是
// {coin: 0}（0 元首充），所以并发一次能白拿 N 份 gem/coin/energy。
func TestConcurrentBuyRespectsDailyLimit(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	userID := ts.newUser(t, ctx)
	var itemID int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM shop_items WHERE enabled AND limit_per_day > 0 ORDER BY id LIMIT 1`).
		Scan(&itemID); err != nil {
		t.Skipf("无限购商品：%v", err)
	}
	limit := ts.scalar(t, ctx, `SELECT limit_per_day FROM shop_items WHERE id = $1`, itemID)
	ts.grant(t, ctx, userID, map[string]int64{"coin": 50_000_000, "gem": 50_000_000})

	const workers = 20
	ok, errs, peak := runParallel(workers, func(int) error {
		_, err := ts.Buy(ctx, userID, itemID)
		return err
	})
	mustOverlap(t, peak, workers)

	if ok > limit {
		t.Fatalf("并发购买成功 %d 次，超过每日限购 %d 次 —— 限购被绕过", ok, limit)
	}
	if ok == 0 {
		t.Fatalf("并发购买一次都没成功（错误：%v），夹具可能有问题", errs[:min(3, len(errs))])
	}
	t.Logf("商品 %d 限购 %d：并发 %d 次购买成功 %d 次，另有 %d 次被拒",
		itemID, limit, workers, ok, len(errs))
}

// TestConcurrentClaimTaskOnlyOnce 覆盖 H2（High）。
//
// 作弊手法：并发打 N 个 /tasks/{id}/claim。旧实现丢弃了
// UPDATE user_tasks ... WHERE claimed_at IS NULL 的 RowsAffected，
// 而上面的预检在并发下无效（N 个请求都在彼此提交前读到"未领取"）。
func TestConcurrentClaimTaskOnlyOnce(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	userID := ts.newUser(t, ctx)
	var taskID int
	if err := ts.pool.QueryRow(ctx,
		`SELECT id FROM tasks WHERE enabled AND scope = 'daily' ORDER BY id LIMIT 1`).Scan(&taskID); err != nil {
		t.Skipf("无 daily 任务：%v", err)
	}
	ts.scalar(t, ctx, `SELECT target FROM tasks WHERE id = $1`, taskID) // 确认存在
	target := ts.scalar(t, ctx, `SELECT target FROM tasks WHERE id = $1`, taskID)
	if _, err := ts.pool.Exec(ctx,
		`INSERT INTO user_tasks (user_id, task_id, task_date, progress)
		 VALUES ($1,$2,CURRENT_DATE,$3)
		 ON CONFLICT (user_id, task_id, task_date) DO UPDATE SET progress = EXCLUDED.progress`,
		userID, taskID, target); err != nil {
		t.Fatalf("预置任务进度失败：%v", err)
	}
	before := ts.wallet(t, ctx, userID)

	const workers = 20
	ok, _, peak := runParallel(workers, func(int) error {
		_, err := ts.ClaimTask(ctx, userID, taskID)
		return err
	})
	mustOverlap(t, peak, workers)

	if ok != 1 {
		t.Fatalf("并发领取任务成功 %d 次，应恰好 1 次 —— 奖励被重复发放", ok)
	}
	after := ts.wallet(t, ctx, userID)
	t.Logf("并发领取：成功 %d 次，金币 %d → %d（增量 %d）",
		ok, before.Coin, after.Coin, after.Coin-before.Coin)
}

func TestConcurrentSignInOnlyOnce(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	userID := ts.newUser(t, ctx)

	const workers = 10
	ok, errs, peak := runParallel(workers, func(int) error {
		res, err := ts.SignIn(ctx, userID)
		if err != nil {
			return err
		}
		if res.Already {
			return context.Canceled // 已签到不算"实际发放"
		}
		return nil
	})
	mustOverlap(t, peak, workers)
	if ok > 1 {
		t.Fatalf("并发签到实际发放 %d 次，应至多 1 次", ok)
	}
	// ⚠️ 夹具自检。只断言 `ok > 1` 时，SignIn 恒返回错误会让 ok 恒为 0，
	// `0 > 1` 为假 —— 测试照绿，功能完全失效也发现不了。
	// 变异测试实证：注入"恒拒"后本用例输出「实际发放 0」仍然 PASS。
	if ok == 0 {
		t.Fatalf("签到一次都没成功，夹具有问题。错误样本：%v", errs[:min(3, len(errs))])
	}
	t.Logf("并发签到：实际发放 %d 次", ok)
}

// TestConcurrentSettleSameTokenOnlyOnce 固化 battle_token 的一次性。
// SettleBattle 认领 token 时检查了 RowsAffected —— 这是**正确**的参照实现，
// 本用例确保它不会在重构中退化。
func TestConcurrentSettleSameTokenOnlyOnce(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	userID := ts.newUser(t, ctx)
	ts.grant(t, ctx, userID, map[string]int64{"energy": 100})

	tp, err := ts.StartBattle(ctx, userID, 1)
	if err != nil {
		t.Skipf("开局失败（可能体力不足）：%v", err)
	}
	before := ts.wallet(t, ctx, userID)
	gl := domain.GenerateLevel(1)
	now := time.Now()
	in := domain.SettleInput{
		Result: "lose", Score: 100, DurationMs: 30_000,
		Shots: 100, Hits: 50, Reactions: 5, HeatMax: 50,
		HPLeft: 900, WaveReached: 1, ReplayHash: "0000000000000000",
	}

	const workers = 10
	ok, _, peak := runParallel(workers, func(int) error {
		tokenID := int64(tp.TokenID)
		_, err := ts.SettleBattle(ctx, userID, tokenID, in)
		return err
	})
	mustOverlap(t, peak, workers)
	_ = gl
	_ = now

	if ok != 1 {
		t.Fatalf("并发结算同一凭证成功 %d 次，应恰好 1 次", ok)
	}
	after := ts.wallet(t, ctx, userID)
	t.Logf("并发结算：成功 %d 次，金币 %d → %d", ok, before.Coin, after.Coin)
}

// TestConcurrentRedeemRespectsMaxUses 覆盖兑换码的 total 上限。
//
// ⚠️ 这个用例**自建兑换码**，不去抢 seeded 的那一个。
// 早期版本去查 `used_count < max_uses` 的存量码，结果是：
// 同一个 test binary 里，第二个要跑兑换的用例永远 skip ——
// 前面那个已经把它兑换满了。测试之间共享可变状态，
// 表现就是"本地全绿、CI 上莫名少一条"。
// 自建码让每个用例独立，谁先跑谁后跑都一样。
func TestConcurrentRedeemRespectsMaxUses(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	const maxUses = 4
	code := fmt.Sprintf("CONC%d", time.Now().UnixNano()%1_000_000_000)
	reward := []byte(`{"coin":500}`)
	// redeem_codes.id 是裸 INTEGER（不是 serial），必须显式给。
	// 测试内单线程创建，MAX+1 足够；真要并发创建才需要序列。
	if _, err := ts.pool.Exec(ctx,
		`INSERT INTO redeem_codes (id, code, reward, max_uses, used_count, enabled)
		 VALUES ((SELECT COALESCE(MAX(id),0)+1 FROM redeem_codes), $1,$2,$3,0,true)`,
		code, reward, maxUses); err != nil {
		t.Fatalf("创建测试兑换码失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(),
			`DELETE FROM redeem_usages WHERE code_id IN (SELECT id FROM redeem_codes WHERE code = $1)`, code)
		_, _ = ts.pool.Exec(context.Background(), `DELETE FROM redeem_codes WHERE code = $1`, code)
	})

	const users = 15
	ids := make([]int64, users)
	for i := range ids {
		ids[i] = ts.newUser(t, ctx)
	}
	ok, errs, peak := runParallel(users, func(i int) error {
		_, err := ts.Redeem(ctx, ids[i], code)
		return err
	})
	mustOverlap(t, peak, users)
	// 关键断言：总兑换数不得超过 max_uses。
	if ok > maxUses {
		t.Fatalf("兑换码 %s 被兑换 %d 次，超过 max_uses=%d", code, ok, maxUses)
	}
	if ok == 0 {
		t.Fatalf("兑换码 %s 一个都兑换不成，max_uses 检查可能写成了永假", code)
	}
	// 第二个断言，也是本用例真正的价值所在：**被拒绝的必须是业务错误，
	// 不能是数据库约束错误**。
	//
	// 旧实现是 `UPDATE redeem_codes SET used_count = used_count + 1 WHERE id = $1`
	// —— 没有 `AND used_count < max_uses`。并发下 15 个事务全部在
	// 对方提交前通过了前置 SELECT，于是全部执行 UPDATE，
	// used_count 冲到 15 > 4，撞上 chk_redeem_uses CHECK 约束整笔回滚。
	// 资源上没超发，但：
	//   1) 客户端收到的是数据库 23502 而不是"兑换码已被抢完"
	//   2) 一旦有人迁移掉那条 CHECK（它只是"恰好"存在，不是并发设计），
	//      就直接变成无限超发
	// 正确的实现应该在应用层用条件 UPDATE 挡住，返回 ErrBadInput/ErrForbidden。
	for _, err := range errs {
		if !errors.Is(err, ErrBadInput) && !errors.Is(err, ErrForbidden) {
			t.Fatalf("超限兑换返回的不是业务错误：%v", err)
		}
	}
	t.Logf("兑换码 %s：%d 用户并发兑换，成功 %d（上限 %d），拒绝 %d 次均为业务错误",
		code, users, ok, maxUses, len(errs))
}

func TestConcurrentChallengeRespectsDailyLimit(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	owner := ts.newUser(t, ctx)
	victim := ts.newUser(t, ctx)
	dv, err := ts.SaveDefense(ctx, victim, SaveDefenseInput{
		Name: "目标", Works: []string{"slow_belt"}, Skills: []int{1, 2, 3},
	})
	if err != nil {
		t.Fatalf("建防线失败：%v", err)
	}

	const workers = 12
	ok, errs, peak := runParallel(workers, func(int) error {
		_, err := ts.ChallengeDefense(ctx, owner, dv.ID, ChallengeInput{
			Seed: "1", Won: true, DurationMs: 60_000, HPLeftPct: 100,
			ReplayHash: "0000000000000000",
		})
		return err
	})
	mustOverlap(t, peak, workers)
	if ok > DefenseAttemptLimit {
		t.Fatalf("并发挑战成功 %d 次，超过每日上限 %d", ok, DefenseAttemptLimit)
	}
	// ⚠️ 夹具自检，不能省。
	// 只断言 `ok > limit` 的话，把「每日上限」整个删掉（让所有请求都成功）
	// 会被上面那条抓住；但反过来把 ChallengeDefense 改成恒返回错误
	// （上限检查写成永假、目标不存在、任何原因）时，ok 恒为 0，
	// `0 > 3` 为假 —— 测试照绿。
	// 变异测试实证：注入"恒拒"后本用例输出「成功 0」仍然 PASS。
	if ok == 0 {
		t.Fatalf("并发挑战一次都没成功，夹具有问题（或上限检查写成了永假）。错误样本：%v",
			errs[:min(3, len(errs))])
	}
	t.Logf("并发挑战 %d 次：成功 %d（上限 %d），拒绝 %d", workers, ok, DefenseAttemptLimit, len(errs))
}

// TestConcurrentSaveDefenseNoPartialWrite 覆盖 M4（Medium）。
//
// 旧实现 upsert / DELETE / INSERT 逐条用 pool 执行，不在同一事务：
// 并发会交错成 A DELETE → B DELETE → A INSERT slot0 → B INSERT slot0，
// 第二个撞唯一索引 → 500，且 defense_works 停留在部分写入状态。
func TestConcurrentSaveDefenseNoPartialWrite(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	userID := ts.newUser(t, ctx)

	const workers = 8
	ok, errs, peak := runParallel(workers, func(int) error {
		_, err := ts.SaveDefense(ctx, userID, SaveDefenseInput{
			Name:   "并发",
			Works:  []string{"slow_belt", "block_wall", "tesla_grid"},
			Skills: []int{1, 2, 3},
		})
		return err
	})
	mustOverlap(t, peak, workers)
	works := ts.scalar(t, ctx, `SELECT COUNT(*) FROM defense_works WHERE user_id = $1`, userID)
	if works != 3 {
		t.Fatalf("defense_works 应为 3 行（与快照一致），实际 %d —— 出现部分写入", works)
	}
	t.Logf("并发保存防线：成功 %d，报错 %d，最终 works=%d 行", ok, len(errs), works)
}

// TestSaveDefenseRejectsForgedOwnership 覆盖 H4（High）。
//
// 挑战者用这份快照在本地模拟，所以快照必须只含玩家真正拥有的东西。
func TestSaveDefenseRejectsForgedOwnership(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	userID := ts.newUser(t, ctx)

	cases := []struct {
		name string
		in   SaveDefenseInput
	}{
		{"未解锁的技能", SaveDefenseInput{Name: "x", Skills: []int{9999}}},
		{"未拥有的装备", SaveDefenseInput{Name: "x", Equipment: []int{9999}}},
		{"未投入的专精节点", SaveDefenseInput{Name: "x", MasteryNodes: []int{9999}}},
		{"未知工程装置", SaveDefenseInput{Name: "x", Works: []string{"god_mode"}}},
		{"装置数量超限", SaveDefenseInput{Name: "x", Works: []string{"slow_belt", "block_wall", "tesla_grid", "slow_belt"}}},
		{"永久护盾", SaveDefenseInput{Name: "x", ShieldHours: 9999999}},
		{"负数护盾", SaveDefenseInput{Name: "x", ShieldHours: -1}},
		{"超长名称", SaveDefenseInput{Name: string(make([]rune, 200))}},
	}
	for _, c := range cases {
		if _, err := ts.SaveDefense(ctx, userID, c.in); err == nil {
			t.Errorf("%s：应被拒绝却通过了", c.name)
		}
	}
	// 合法输入必须仍然通过
	if _, err := ts.SaveDefense(ctx, userID, SaveDefenseInput{
		Name: "合法防线", Works: []string{"slow_belt", "block_wall"}, Skills: []int{1, 2, 3},
		ShieldHours: 24,
	}); err != nil {
		t.Fatalf("合法输入被误拒：%v", err)
	}
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
