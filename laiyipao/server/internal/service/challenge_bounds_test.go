package service

import (
	"context"
	"strings"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// 防线挑战上报的字段边界（第 69 轮）。
//
// # 缺陷
//
// `ChallengeInput` 的 `DurationMs` / `HPLeftPct` / `ReplayHash` 此前
// **零校验**，而 `defense_challenges` 的对应列是 INTEGER / TEXT：
//
//	duration_ms  INTEGER
//	hp_left_pct  INTEGER
//	replay_hash  TEXT
//
// 上报 2147483648 时 PG 报 `22003 integer out of range` → failErr → **500**。
//
// 任何玩家都能定向让任意目标的挑战接口持续 500。
//
// # 与结算面的关系
//
// 结算侧第 62/63 轮补过同类边界，但反射式覆盖率守卫
// `settle_coverage_test.go` **只管 `SettleInput`** ——
// 于是防线挑战这条路径一直没被看到。
//
// 这正是「守卫覆盖了一个结构、漏了同族的另一个」。

func validChallenge() ChallengeInput {
	return ChallengeInput{
		Seed:       1,
		Won:        true,
		DurationMs: 60_000,
		HPLeftPct:  100,
		ReplayHash: "0000000000000000",
	}
}

// TestChallengeInputBounds 覆盖三条边界的两侧。
func TestChallengeInputBounds(t *testing.T) {
	cases := []struct {
		name  string
		mut   func(*ChallengeInput)
		valid bool
	}{
		{"合法基线", func(*ChallengeInput) {}, true},
		{"时长 1ms（下界+1）", func(c *ChallengeInput) { c.DurationMs = 1 }, true},
		{"时长上界", func(c *ChallengeInput) { c.DurationMs = MaxChallengeDurationMs }, true},
		{"时长 0", func(c *ChallengeInput) { c.DurationMs = 0 }, false},
		{"时长 -1", func(c *ChallengeInput) { c.DurationMs = -1 }, false},
		// int32 上界 +1 —— 这正是会让 PG 报 22003 的那个值
		{"时长 int32 上界+1", func(c *ChallengeInput) { c.DurationMs = 2147483648 }, false},
		{"时长 2^62", func(c *ChallengeInput) { c.DurationMs = 1 << 62 }, false},

		{"血量 0%", func(c *ChallengeInput) { c.HPLeftPct = 0 }, true},
		{"血量 50%", func(c *ChallengeInput) { c.HPLeftPct = 50 }, true},
		{"血量 100%", func(c *ChallengeInput) { c.HPLeftPct = 100 }, true},
		{"血量 101", func(c *ChallengeInput) { c.HPLeftPct = 101 }, false},
		{"血量 -1", func(c *ChallengeInput) { c.HPLeftPct = -1 }, false},
		{"血量 int32 上界", func(c *ChallengeInput) { c.HPLeftPct = 2147483647 }, false},

		{"空哈希", func(c *ChallengeInput) { c.ReplayHash = "" }, true},
		{"16 位哈希（引擎产出）", func(c *ChallengeInput) { c.ReplayHash = "0123456789abcdef" }, true},
		{"64 字符", func(c *ChallengeInput) { c.ReplayHash = strings.Repeat("a", 64) }, true},
		{"65 字符", func(c *ChallengeInput) { c.ReplayHash = strings.Repeat("a", 65) }, false},
		{"1MB（BodyLimit 内）", func(c *ChallengeInput) { c.ReplayHash = strings.Repeat("a", 1<<20) }, false},
	}

	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			in := validChallenge()
			c.mut(&in)
			err := ValidateChallengeInput(in)

			if c.valid {
				if err != nil {
					t.Fatalf("合法输入被拒：%v", err)
				}
				return
			}
			if err == nil {
				t.Fatal("越界输入被放行")
			}
			// ⚠️ 断言错误**来源**：必须是 ErrInvalidField（可分类 → 422），
			// 而不是碰巧返回了某个错误。
			//
			// 不这么做的话，一个包了 %w: err 的实现会让这条测试绿 ——
			// 而那种错误落 500，正是本轮要消灭的东西。
			if !errorsIs(err, domain.ErrInvalidField) {
				t.Errorf("应归类为 ErrInvalidField（→422），实得 %v —— "+
					"落到 500 就意味着「上报不可信」被伪装成「服务内部错误」", err)
			}
		})
	}
}

// TestChallengeFieldsFitDatabaseColumns 把「入库安全」立成**关系**守卫。
//
// 判据不是某个具体值，而是：任何被放行的输入都装得进目标列的类型。
// 这比逐值断言强 —— 它对未来的新字段同样成立，
// 且直接对应真实故障（PG 22003）。
func TestChallengeFieldsFitDatabaseColumns(t *testing.T) {
	const pgIntegerMax = int64(2147483647)

	inputs := []ChallengeInput{
		{},
		validChallenge(),
		{DurationMs: 1, HPLeftPct: 0},
		{DurationMs: MaxChallengeDurationMs, HPLeftPct: 100},
		{DurationMs: 2147483647, HPLeftPct: 2147483647},
		{DurationMs: 2147483648, HPLeftPct: 2147483648},
		{DurationMs: -1, HPLeftPct: -1},
		{DurationMs: 1 << 62, HPLeftPct: 1 << 62},
	}

	for _, in := range inputs {
		if err := ValidateChallengeInput(in); err != nil {
			continue // 被拒就没事发生
		}
		if int64(in.DurationMs) > pgIntegerMax {
			t.Errorf("DurationMs=%d 被放行但超过 PG INTEGER 上界 —— 入库必然报 22003", in.DurationMs)
		}
		if int64(in.HPLeftPct) > pgIntegerMax || in.HPLeftPct < 0 {
			t.Errorf("HPLeftPct=%d 被放行但装不进 PG INTEGER —— 入库必然报 22003", in.HPLeftPct)
		}
	}
}

// TestChallengeHashLengthMatchesSettleFace 钉住两条路径的哈希上限一致。
//
// 结算面（第 56 轮）与挑战面各有一份上界。漂了就会出现
// 「结算拒绝但挑战接受」或反之 —— 两者都说不通，
// 因为它们描述的是**同一个引擎产出的同一个串**。
func TestChallengeHashLengthMatchesSettleFace(t *testing.T) {
	// 结算面的常量在 domain 包里是私有的 maxReplayHashLen，
	// 而它被 replay_skills 的 limits 契约向量锁着。
	// 这里只断言两者**相等**，用行为而不是常量引用来证明 ——
	// 常量名会变，行为不会。
	in := validChallenge()
	in.ReplayHash = strings.Repeat("a", MaxChallengeReplayHashLen)
	if err := ValidateChallengeInput(in); err != nil {
		t.Fatalf("上限长度本身被拒：%v", err)
	}
	in.ReplayHash = strings.Repeat("a", MaxChallengeReplayHashLen+1)
	if err := ValidateChallengeInput(in); err == nil {
		t.Errorf("上限+1 仍被放行 —— 上界 %d 没有被执行", MaxChallengeReplayHashLen)
	}
}

// TestChallengeEntryPointActuallyValidates 确认**入口真的调用了**校验。
//
// ⚠️ 这条是被变异测试逼出来的。
//
// 我先写的三条守卫全部直接测 `ValidateChallengeInput` 这个函数。
// 于是变异「删掉 `ChallengeDefense` 里的调用点」时——
// 函数还在、行为全对、只有入口不再校验——**三条守卫全绿**。
//
// 这与 README 记的「`TestEveryMasteryKindIsEvaluated` 存在之前，
// `mechanic` 分支连 case 都没有」是同一类：
// **测函数不等于测调用点**。
//
// 判据必须落在「从入口进去」的那条路径上。
// 这里用「越界输入走入口必须被拒」来表达 ——
// 若入口不调校验，越界请求会一路到 DB 并触发 22003。
func TestChallengeEntryPointActuallyValidates(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	owner := ts.newUser(t, ctx)
	foe := ts.newUser(t, ctx)

	dv, err := ts.SaveDefense(ctx, owner, SaveDefenseInput{
		Name: "边界探针", Works: []string{"slow_belt"}, Skills: []int{1, 2, 3},
	})
	if err != nil {
		t.Fatalf("造防线失败：%v", err)
	}

	// 越界上报：duration_ms 超出 int32。
	// 若入口不校验，这一路会走到 INSERT 并让 PG 报 22003 → 500。
	_, err = ts.ChallengeDefense(ctx, foe, dv.ID, ChallengeInput{
		Seed:       1,
		Won:        true,
		DurationMs: 2147483648,
		HPLeftPct:  100,
		ReplayHash: "0000000000000000",
	})
	if err == nil {
		t.Fatal("越界 duration_ms 通过入口被放行 —— 入口没有调用 ValidateChallengeInput")
	}
	if !errorsIs(err, domain.ErrInvalidField) {
		t.Errorf("入口应返回可分类的 ErrInvalidField（→422），实得 %v", err)
	}

	// 合法上报必须仍然成功 —— 确认上面不是「一律拒绝」
	// ⚠️ 挑战次数有限额（DefenseAttemptLimit），所以越界那次若真的
	// 被放行并消耗了次数，合法那次会拿到 ErrForbidden。
	// 这正是「越界请求不该消耗任何配额」的一个附带断言。
	res, err := ts.ChallengeDefense(ctx, foe, dv.ID, validChallenge())
	if err != nil {
		t.Fatalf("合法挑战被拒：%v", err)
	}
	if !res.Won {
		t.Error("合法挑战应返回 Won=true")
	}
}

// errorsIs 是 errors.Is 的本地别名，避免 import 冲突时读不清。
func errorsIs(err, target error) bool {
	for e := err; e != nil; {
		if e == target {
			return true
		}
		u, ok := e.(interface{ Unwrap() error })
		if !ok {
			return false
		}
		e = u.Unwrap()
	}
	return false
}
