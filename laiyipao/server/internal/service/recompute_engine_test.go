package service

// 整局重算接线后的结算侧行为（2026-10-15）。
//
// 这里覆盖接线前**完全没有**的两组路径：
//
//	组 2  重复结算                -> 本文件
//	组 3  build_snapshot 不可用   -> 本文件
//
// （组 1「玩家自配构筑」在向量层，见 domain/replay_build_coverage_test.go。）
//
// # 与 loseSettleInput 的分工
//
// `loseSettleInput()` 的 `ReplayHash` 是**空串**（老客户端路径），
// 所以那 25 处 e2e 结算**不经过**重算。本文件每一局都用
// **真种子 + 真构筑快照**算出**真哈希**，走完整条重算路径。
//
// 两者的分工是刻意的：
//   - loseSettleInput 测**结算机制**（幂等 / 回滚 / 发奖 / 进度）
//   - 本文件测**验真机制**（重算 / 不符 / 拒发 / 台账 / 计数器）

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"strconv"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// mustSeed 把 BattleTokenResp.Seed 转成 BattleToken.Seed。
//
// ⚠️ 两者类型不同不是疏忽：`BattleTokenResp.Seed` 是 **string**，
// 因为跨端要求 int64 字符串（JSON number 超过 2^53 后 JS 解析即失精）。
// 而 `domain.BattleToken.Seed` 是 int64 —— 服务端内部不需要那层保护。
func mustSeed(t *testing.T, s string) int64 {
	t.Helper()
	n, err := strconv.ParseInt(s, 10, 64)
	if err != nil {
		t.Fatalf("种子 %q 非法：%v", s, err)
	}
	return n
}

// realHashSettle 开一局，并造一份**能被服务端验真通过**的结算上报。
//
// ⚠️ 它**自己开的那局**必须拿来结算，不能再 StartBattle 一次：
// 种子是每局新生成的，用 A 局的种子去结算 B 局必然哈希不符。
// 所以 token 一并返回，由调用方拿去结算。
//
// 为什么能这么造：`domain.SettleInput` 里的 Kills / Score / DurationMs
// 参与 `ValidateSettle` 但**不参与** replay_hash —— 哈希只由
// （关卡 + 种子 + 构筑 + 选牌脚本）决定。所以「一份任意但合法的结算上报」
// 配「一份按真实输入算出的哈希」二者互不干扰。
func (ts *testService) realHashSettle(
	t *testing.T, ctx context.Context, userID int64, levelID int,
) (domain.SettleInput, BattleTokenResp, error) {
	t.Helper()

	tp, err := ts.StartBattle(ctx, userID, levelID)
	if err != nil {
		return domain.SettleInput{}, tp, fmt.Errorf("开局失败：%w", err)
	}
	in := loseSettleInput()
	in.ReplayHash = ts.computedHashFor(t, ctx, userID, levelID, in.CardPicks, tp)
	return in, tp, nil
}

// computedHashFor 按「关卡 + 种子 + 构筑 + 选牌」跑一遍引擎，拿真哈希。
//
// ⚠️ 它复刻的是 `recomputeEngine` 的输入组装，**不是**调用它 ——
// 因为要拿到的是「玩家本该上报的那个值」，而 `recomputeEngine` 的
// 入参里已经含了玩家上报的哈希。
func (ts *testService) computedHashFor(
	t *testing.T, ctx context.Context, userID int64, levelID int,
	cardPicks []int, tp BattleTokenResp,
) string {
	t.Helper()

	build, err := ts.LoadBuildSnapshot(ctx, userID)
	if err != nil {
		t.Fatalf("读构筑快照失败：%v", err)
	}
	buildRaw, err := json.Marshal(build)
	if err != nil {
		t.Fatalf("构筑快照编码失败：%v", err)
	}
	var bs domain.BuildSnapshot
	if err := json.Unmarshal(buildRaw, &bs); err != nil {
		t.Fatalf("构筑快照解码失败：%v", err)
	}

	out := domain.Replay(
		domain.GenerateLevel(levelID), bs, tp.Seed, cardPicks,
		"", // 期望值传空 —— 这里只要 ComputedHash
		domain.EnemiesByID(), domain.SkillsByID(), domain.DefaultScoreRules(),
	)
	if out.Err != nil {
		t.Fatalf("重算跑不起来：%v", out.Err)
	}
	return out.ComputedHash
}

// countRecomputesOf 数**某个用户自己**在台账里留下的行数。
//
// ⚠️ 必须按 user_id 过滤，不能 `SELECT COUNT(*)`：
// `openTestService` 连的是**共享库**（packageDSN / TEST_DATABASE_URL），
// 不是每个用例一块干净的一次性库。数全表会把别的用例写的行算进来 ——
// 本文件第一版就是这么写的，于是「不符应写 1 行」实测出 2 行，
// 原因完全不在被测代码上。
func (ts *testService) countRecomputesOf(t *testing.T, ctx context.Context, userID int64) int {
	t.Helper()
	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM settle_recomputes WHERE user_id = $1`, userID).Scan(&n); err != nil {
		t.Fatalf("读台账行数失败：%v", err)
	}
	return n
}

// flipHex 把十六进制串的**最后一位** 0/1 互换。
//
// 改末位而不是首位：哈希的每一位都同等参与 fnv1a，改任何一位都会
// 得到一个格式合法但内容不同的哈希。返回非空保证它仍是个合法哈希串，
// 从而测试的是「算得出来但不一致」，而不是「格式非法被前置校验拦下」。
func flipHex(h string) string {
	if h == "" {
		return "deadbeefdeadbeef"
	}
	b := []byte(h)
	if b[len(b)-1] == '0' {
		b[len(b)-1] = '1'
	} else {
		b[len(b)-1] = '0'
	}
	return string(b)
}

// ── 组 2：重复结算 ───────────────────────────────────────────────────

// TestSettleRecomputeDoubleSettleWritesLedgerOnce 重复结算**不得**让台账写两次。
//
// 为什么单独一条：重算跑在事务**之前**，而 token 的 `used_at` 抢占在
// 事务**之内**。这两者的先后关系带来一个必须钉死的问题：
//
//	第二次结算能不能在「token 已被占用」这个错误之前，
//	先跑一遍完整引擎重算并写一条台账？
//
// 如果会，台账里就会出现「同一个 token 两条不符记录」，而其中一条
// 对应的其实是**同一个玩家重试了同一个网络请求** ——
// 计在不符率里会虚高不符率，在熔断里会加速降级。
// 也就是说，一次无害的客户端重试就能把整局引擎的防护推向 shadow。
func TestSettleRecomputeDoubleSettleWritesLedgerOnce(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})

	in, tp, err := ts.realHashSettle(t, ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	_ = tp // realHashSettle 已经开好局，token 由它返回
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}

	// 第一次：真哈希 -> 成功，且**不进台账**（一致不是台账的事）
	ts.resetEngineCounters()
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, in); err != nil {
		t.Fatalf("首结算应成功：%v", err)
	}
	if c := ts.EngineCounters(); c.Verified != 1 {
		t.Errorf("首结算应计 1 次 verified，实际 %d —— 整条重算路径可能没跑", c.Verified)
	}
	if got := ts.countRecomputesOf(t, ctx, uid); got != 0 {
		t.Errorf("一致的结算不该写台账，实际 %d 行", got)
	}

	// 第二次：同一 token -> ErrTokenUsed
	ts.resetEngineCounters()
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, in); !errors.Is(err, domain.ErrTokenUsed) {
		t.Fatalf("重复结算应 ErrTokenUsed，实际 %v", err)
	}
	if c := ts.EngineCounters(); c.Verified != 0 {
		t.Errorf("重复结算不该再跑一次重算，实际又计了 %d 次 verified", c.Verified)
	}
	if got := ts.countRecomputesOf(t, ctx, uid); got != 0 {
		t.Errorf("重复结算不该写台账，实际 %d 行", got)
	}
}

// TestSettleRecomputeMismatchIsRecordedOnce 同一局**不符**时台账只该有一行。
//
// 组 2 的另一半：上面那条验「一致时不写台账」，
// 这条验「不符时写**恰好一行**」——
// 写零行是静默失效，写两行会让不符率翻倍并加速熔断。
func TestSettleRecomputeMismatchIsRecordedOnce(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	ts.Service.Cfg.SettleRecomputeMode = RecomputeEnforce
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})

	in, tp, err := ts.realHashSettle(t, ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	_ = tp // realHashSettle 已经开好局，token 由它返回
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	in.ReplayHash = flipHex(in.ReplayHash) // -> 算得出来但不一致

	ts.resetEngineCounters()
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, in); !errors.Is(err, ErrReplaySkillsMismatch) {
		t.Fatalf("哈希不符应被拒（enforce 档），实际 %v", err)
	}
	if got := ts.countRecomputesOf(t, ctx, uid); got != 1 {
		t.Errorf("不符应写**恰好 1 行**台账，实际 %d 行", got)
	}
	if c := ts.EngineCounters(); c.Mismatch != 1 {
		t.Errorf("mismatch 计数应为 1，实际 %d", c.Mismatch)
	}
	if c := ts.EngineCounters(); c.Verified != 0 {
		t.Errorf("不符时不该计 verified，实际 %d", c.Verified)
	}

	// 台账内容：engine_version 必须是整局引擎那个，
	// 否则台账里分不清「哪个引擎判的不符」。
	//
	// ⚠️ 必须 `WHERE user_id = $1` 且**按 id 排序**取**最后**一行 ——
	// 共享库里别的用例也往同一张表写（recompute_live_test 写的是
	// `skills-only/v1` 的 S 段行）。本文件第一版写的是
	// `SELECT ... LIMIT 1`，在共享库里取到谁全看插入顺序，
	// 于是断言「mismatched_fields == [replay_hash]」会随机红。
	// 同一个坑栽了两次：先是 COUNT(*) 没按 user 过滤，这次是 LIMIT 1。
	var fields []string
	var engineVersion string
	if err := ts.pool.QueryRow(ctx,
		`SELECT mismatched_fields, engine_version FROM settle_recomputes
		 WHERE user_id = $1 ORDER BY id DESC LIMIT 1`, uid).
		Scan(&fields, &engineVersion); err != nil {
		t.Fatalf("读台账失败：%v", err)
	}
	if len(fields) != 1 || fields[0] != "replay_hash" {
		t.Errorf("mismatched_fields 应为 [replay_hash]，实际 %v", fields)
	}
	if engineVersion != RecomputeEngineVersionWholeBattle {
		t.Errorf("engine_version 应为 %q，实际 %q", RecomputeEngineVersionWholeBattle, engineVersion)
	}
}

// ── 组 3：build_snapshot 不可用 ──────────────────────────────────────

// TestSettleRecomputeSnapshotUnusableIsNotCheating 断言：
// **「算不出来」绝不被当成「作弊」**。
//
// `domain.Replay` 区分三种结果，其中 `Err != nil` 表示「根本没跑起来」
// （缺关卡 / 缺构筑 / 种子非法）。把它当成 `Matched == false` 会让上层
// 把「我方数据不完整」报成「玩家作弊」—— 而那会让一个真实 bug
// 被误判成攻击事件，进而触发熔断、批量误伤诚实玩家。
// 这是本条存在的全部理由。
//
// 三个子场景各对应 `Replay` 的一处早退。
func TestSettleRecomputeSnapshotUnusableIsNotCheating(t *testing.T) {
	cases := []struct {
		name  string
		build func(map[string]any) map[string]any
	}{
		{
			// Replay 第一道早退：len(build.Skills) == 0
			name: "构筑里没有任何技能",
			build: func(m map[string]any) map[string]any {
				m["skills"] = map[string]any{}
				return m
			},
		},
		{
			// Replay 第二道早退：EquippedFromSnapshot 出来是空的
			//（全部 slot < 0，即「都没放进槽位」）
			name: "全部技能的 slot 都是 -1",
			build: func(m map[string]any) map[string]any {
				skills, _ := m["skills"].(map[string]any)
				for k, v := range skills {
					if sk, ok := v.(map[string]any); ok {
						sk["slot"] = -1
						skills[k] = sk
					}
				}
				return m
			},
		},
		{
			// Replay 第三道早退：AttackerFromSnapshot 在
			// `build.Attacker == nil` 时报错。
			//
			// ⚠️ 必须**整个删掉** attacker 键，不能只把 attack 设成 -1：
			// 那会解出一个非 nil 的结构体，Attack=-1 被原样带进引擎，
			// 重算照跑不误，最后以「哈希不符」收场 —— 于是这条用例
			// 测到的根本不是 Err 分支，而是不符分支。
			// 本文件第一版就是这么写的，红在「算不出来被当成了拒绝」上。
			name: "构筑里没有 attacker 字段",
			build: func(m map[string]any) map[string]any {
				delete(m, "attacker")
				return m
			},
		},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			ts := openTestService(t)
			ctx := context.Background()
			ts.Service.Cfg.SettleRecomputeMode = RecomputeEnforce
			uid := ts.newUser(t, ctx)
			ts.grant(t, ctx, uid, map[string]int64{"energy": 200})

			in, tp, err := ts.realHashSettle(t, ctx, uid, 1)
			if err != nil {
				t.Skipf("开局失败：%v", err)
			}
			_ = tp // realHashSettle 已经开好局，token 由它返回
			if err != nil {
				t.Skipf("开局失败：%v", err)
			}

			// 构造「坏」的构筑快照，直接打 recomputeEngine。
			// 走 SettleBattle 没法注入坏快照 —— 它总是从库里读真数据。
			build, err := ts.LoadBuildSnapshot(ctx, uid)
			if err != nil {
				t.Fatalf("读构筑快照失败：%v", err)
			}
			buildRaw, err := json.Marshal(tc.build(build))
			if err != nil {
				t.Fatalf("坏快照编码失败：%v", err)
			}
			tok := domain.BattleToken{
				ID: tp.TokenID, UserID: uid, LevelID: 1, Seed: mustSeed(t, tp.Seed),
			}

			ts.resetEngineCounters()
			err = ts.recomputeEngine(ctx, uid, tok, domain.GenerateLevel(1), buildRaw, in)

			// 核心断言：无论哪种坏法，都**不**返回拒绝错误。
			if err != nil {
				t.Fatalf("「算不出来」被当成了拒绝：%v —— 这是数据问题不是作弊，必须放行", err)
			}
			if c := ts.EngineCounters(); c.CannotRun != 1 {
				t.Errorf("应计 1 次 cannot_run，实际 %d", c.CannotRun)
			}
			if c := ts.EngineCounters(); c.Mismatch != 0 {
				t.Errorf("算不出来不该计 mismatch（那会把数据问题记成作弊），实际 %d", c.Mismatch)
			}
			if c := ts.EngineCounters(); c.Verified != 0 {
				t.Errorf("算不出来不该计 verified，实际 %d", c.Verified)
			}
			// ⚠️ 最关键的一条：算不出来**不能**进台账。
			// 台账是不符率与熔断的输入，把数据问题写进去会污染它们。
			if got := ts.countRecomputesOf(t, ctx, uid); got != 0 {
				t.Errorf("算不出来不该写台账，实际 %d 行 —— "+
					"台账是不符率的输入，写进去会污染熔断", got)
			}
		})
	}
}

// TestSettleRecomputeBadSnapshotJSONIsNotCheating 坏 JSON 也放行。
//
// 与上面三条不同：这里连 `BuildSnapshot` 都解不出来。
// 形态更极端，但归属相同 —— 是我方数据坏了，不是玩家作弊。
func TestSettleRecomputeBadSnapshotJSONIsNotCheating(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	ts.Service.Cfg.SettleRecomputeMode = RecomputeEnforce
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})

	in, tp, err := ts.realHashSettle(t, ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	_ = tp // realHashSettle 已经开好局，token 由它返回
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	tok := domain.BattleToken{
		ID: tp.TokenID, UserID: uid, LevelID: 1, Seed: mustSeed(t, tp.Seed),
	}

	ts.resetEngineCounters()
	// 截断的 JSON —— 模拟「战报里那段 jsonb 被外部写坏」
	if err := ts.recomputeEngine(ctx, uid, tok, domain.GenerateLevel(1),
		[]byte(`{"skills":{"1":`), in); err != nil {
		t.Fatalf("坏 JSON 被当成了拒绝：%v", err)
	}
	if c := ts.EngineCounters(); c.CannotRun != 1 {
		t.Errorf("应计 1 次 cannot_run，实际 %+v", c)
	}
	if got := ts.countRecomputesOf(t, ctx, uid); got != 0 {
		t.Errorf("坏 JSON 不该写台账，实际 %d 行", got)
	}
}

// TestSettleRecomputeCountersDistinguishAllBranches 四个计数器各走各的。
//
// 这条守的是「计数器本身没坏」—— 上面几条都拿它们当判据。
// 最容易出的错是「某条早退路径忘了计数」，
// 那样上面的断言会因为 CannotRun != 1 而红，但原因会指向错误的地方。
func TestSettleRecomputeCountersDistinguishAllBranches(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	ts.Service.Cfg.SettleRecomputeMode = RecomputeEnforce
	ts.resetEngineCounters()

	// ① 空哈希 -> skipped
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 200})
	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	if _, err := ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput()); err != nil {
		t.Fatalf("空哈希结算应成功：%v", err)
	}
	if c := ts.EngineCounters(); c.Skipped != 1 || c.Verified != 0 || c.CannotRun != 0 {
		t.Errorf("空哈希应只计 skipped，实际 %+v", c)
	}

	// ② 真哈希 -> verified
	uid2 := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid2, map[string]int64{"energy": 200})
	real, tp2, err := ts.realHashSettle(t, ctx, uid2, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	if _, err := ts.SettleBattle(ctx, uid2, tp2.TokenID, real); err != nil {
		t.Fatalf("真哈希结算应成功：%v", err)
	}
	if c := ts.EngineCounters(); c.Verified != 1 || c.Skipped != 1 {
		t.Errorf("真哈希应计 verified 且保留 skipped，实际 %+v", c)
	}

	// ③ 篡改哈希 -> mismatch（且仍不增加 verified）
	uid3 := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid3, map[string]int64{"energy": 200})
	bad, tp3, err := ts.realHashSettle(t, ctx, uid3, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	bad.ReplayHash = flipHex(bad.ReplayHash)
	if _, err := ts.SettleBattle(ctx, uid3, tp3.TokenID, bad); err == nil {
		t.Fatal("篡改哈希应被拒")
	}
	if c := ts.EngineCounters(); c.Mismatch != 1 || c.Verified != 1 || c.Skipped != 1 {
		t.Errorf("篡改哈希应只增 mismatch，实际 %+v", c)
	}
}
