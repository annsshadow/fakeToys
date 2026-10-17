package service

// 结算重算台账的端到端守卫（需真实 PostgreSQL）。
//
// 判据落在**可观测结果**（表里有什么行、rejected 是什么），
// 而不是「RecordRecompute 被调用过」——
// 后者能被一个什么都没写的实现通过。

import (
	"context"
	"fmt"
	"testing"
	"time"
)

// recomputeProbeUser 造一个新账号。
func recomputeProbeUser(t *testing.T, ts *testService, tag string) int64 {
	t.Helper()
	tok := uniqueGuestToken()
	ts.exec(t, `INSERT INTO users (guest_token, nickname, is_guest) VALUES ($1,$2,TRUE)`,
		tok, "rc_"+tag)
	var id int64
	ts.pool.QueryRow(context.Background(),
		`SELECT id FROM users WHERE guest_token=$1`, tok).Scan(&id)
	if err := ts.Service.initNewUser(context.Background(), id); err != nil {
		t.Fatalf("initNewUser: %v", err)
	}
	return id
}

// TestRecordRecomputeWritesRow 守住「写进去了」。
func TestRecordRecomputeWritesRow(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := recomputeProbeUser(t, ts, "write")

	ts.Service.RecordRecompute(ctx, RecomputeRecord{
		BattleID: 0, UserID: uid, LevelID: 7,
		ReportedScore: 100, ReportedKills: 10, ReportedDurationMs: 60000,
		ReportedHash:     "aa",
		MismatchedFields: []string{"replay_skills"},
		Mode:             RecomputeShadow, Rejected: false,
		EngineVersion: RecomputeEngineVersionSkillsOnly,
	})

	var (
		level     int
		rejected  bool
		mode, ver string
		mism      []string
	)
	if err := ts.pool.QueryRow(ctx, `
		SELECT level_id, mode, rejected, engine_version, mismatched_fields
		FROM settle_recomputes WHERE user_id=$1`, uid).
		Scan(&level, &mode, &rejected, &ver, &mism); err != nil {
		t.Fatalf("台账未落库：%v", err)
	}
	if level != 7 {
		t.Errorf("level_id = %d，期望 7", level)
	}
	if mode != RecomputeShadow {
		t.Errorf("mode = %q，期望 %q", mode, RecomputeShadow)
	}
	if rejected {
		t.Error("rejected 应为 false")
	}
	if ver != RecomputeEngineVersionSkillsOnly {
		t.Errorf("engine_version = %q", ver)
	}
	if mism != nil && len(mism) == 1 && mism[0] == "replay_skills" {
		// ok
	} else {
		t.Errorf("mismatched_fields = %v，期望 [\"replay_skills\"]", mism)
	}
}

// TestRecordRecomputeNullsRecomputedWhenAbsent 守住「没重算」与「重算为 0」的区别。
//
// ⚠️ 为什么这条重要：「算出来是 0」在合法战斗里不可能出现，
//
//	那是最强的可疑信号。若未重算与重算为 0 不可区分，
//	这个信号就没了 —— 而这正是用指针而不是值类型的原因。
func TestRecordRecomputeNullsRecomputedWhenAbsent(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := recomputeProbeUser(t, ts, "nulls")

	ts.Service.RecordRecompute(ctx, RecomputeRecord{
		BattleID: 0, UserID: uid, LevelID: 1,
		MismatchedFields: []string{"score"},
		Mode:             RecomputeShadow,
	})
	var score *int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT recomputed_score FROM settle_recomputes WHERE user_id=$1`, uid).
		Scan(&score); err != nil {
		t.Fatalf("查重算分数：%v", err)
	}
	if score != nil {
		t.Errorf("HasRecomputed=false 时 recomputed_score 应为 NULL，实际 = %v", *score)
	}
}

// TestRecordRecomputeRejectsUnknownModeByDowngrading 守住「非法 mode 退到最保守档」。
//
// ⚠️ 为什么不能让它报错或静默写成非法值：
//   - 报错 → 台账写不进去，这条拒绝的证据就没了；
//   - 写成非法值 → 迁移的 CHECK 会拒（23514），效果同上（证据丢失）。
//     所以必须是「退到 shadow 并写进去」—— 宁可档位偏保守，证据不能丢。
func TestRecordRecomputeRejectsUnknownModeByDowngrading(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := recomputeProbeUser(t, ts, "badmode")

	ts.Service.RecordRecompute(ctx, RecomputeRecord{
		BattleID: 0, UserID: uid, LevelID: 3,
		MismatchedFields: []string{"hash"},
		Mode:             "enforceee", // 拼错
	})
	var mode string
	if err := ts.pool.QueryRow(ctx,
		`SELECT mode FROM settle_recomputes WHERE user_id=$1`, uid).Scan(&mode); err != nil {
		t.Fatalf("非法 mode 的行没写进去（证据丢失）：%v", err)
	}
	if mode != RecomputeShadow {
		t.Errorf("非法 mode 应退到 %q，实际 %q", RecomputeShadow, mode)
	}
}

// TestMismatchRateSeparatesSampleFromZeroMismatch 守住「样本量」与「不符率」分开返回。
//
// 熔断判定必须能区分「样本不足」与「零不符」——
// 返回单个比率的话，分母为 0 只能返回 0，那两者就不可区分了。
func TestMismatchRateSeparatesSampleFromZeroMismatch(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := recomputeProbeUser(t, ts, "rate")

	mismatch, total, err := ts.Service.recomputeMismatchRate(ctx, time.Now().Add(-time.Hour))
	if err != nil {
		t.Fatalf("recomputeMismatchRate: %v", err)
	}
	// 本包共用库，前面其它用例可能已写过行，所以只断言「本用户那一行被算进 total」
	before := total
	mismatch, total, err = ts.Service.recomputeMismatchRate(ctx, time.Now().Add(-time.Hour))
	if err != nil || total != before {
		t.Fatalf("两次查询同一时刻应给出相同 total（%d vs %d, err=%v）", before, total, err)
	}

	ts.Service.RecordRecompute(ctx, RecomputeRecord{
		BattleID: 0, UserID: uid, LevelID: 5,
		MismatchedFields: []string{"kills"},
		Mode:             RecomputeEnforce,
	})
	m2, t2, err := ts.Service.recomputeMismatchRate(ctx, time.Now().Add(-time.Hour))
	if err != nil {
		t.Fatalf("recomputeMismatchRate: %v", err)
	}
	if t2 != before+1 {
		t.Errorf("写入一行后 total 应 +1（%d → %d）", before, t2)
	}
	if m2 < mismatch+1 {
		t.Errorf("写入一行不符后 mismatch 应 +1（%d → %d）", mismatch, m2)
	}
}

// TestAdminListRecomputesFilters 守住后台读侧的筛选与分页。
//
// 为什么在这条上花力气：这张表是 R1 的**唯一**观测面，
// 而「它筛不出来」与「它没有数据」在运维现场完全无法区分。
func TestAdminListRecomputesFilters(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	levels := []int{1, 2, 3}
	matched := [][]string{{"score"}, {"kills", "hash"}, {}}
	ids := make([]int64, 3)
	for i, tag := range []int64{11, 22, 33} {
		guest := fmt.Sprintf("g_rc_%d", tag)
		ts.exec(t, `INSERT INTO users (guest_token, nickname, is_guest, status)
			VALUES ($1,$2,TRUE,1)`, guest, fmt.Sprintf("rc%d", tag))
		// ⚠️ 必须回读真实 id：`settle_recomputes.user_id` 有指向 users 的外键，
		//   拿「打算用的 id」直接写会撞 23503 —— 而那个 id 与自增序列无关。
		var uid int64
		if err := ts.pool.QueryRow(ctx,
			`SELECT id FROM users WHERE guest_token=$1`, guest).Scan(&uid); err != nil {
			t.Fatalf("回读新用户 id：%v", err)
		}
		ts.exec(t, `
			INSERT INTO settle_recomputes
			  (battle_id, user_id, level_id, reported_score, reported_kills,
			   reported_duration_ms, reported_hash, mismatched_fields, mode, rejected, engine_version)
			VALUES ($1,$2,$3,100,10,60000,'h',$4,$5,FALSE,'skills-only/v1')`,
			tag*100, uid, levels[i], matched[i], RecomputeShadow)
		ids[i] = uid
	}

	items, total, err := ts.AdminListRecomputes(ctx, RecomputeFilter{Limit: 50})
	if err != nil {
		t.Fatalf("全量读取失败：%v", err)
	}
	if total != 3 || len(items) != 3 {
		t.Fatalf("全量应 3 行，实际 total=%d len=%d", total, len(items))
	}
	// 排序必须按 id DESC —— 这张表的全部价值在「最近的」
	if items[0].ID <= items[1].ID {
		t.Errorf("应按 id 降序（最近的在前），实际首行 %d 次行 %d", items[0].ID, items[1].ID)
	}

	items, total, err = ts.AdminListRecomputes(ctx, RecomputeFilter{OnlyMismatch: true, Limit: 50})
	if err != nil {
		t.Fatalf("只看不符失败：%v", err)
	}
	if total != 2 {
		t.Errorf("只有 2 行有不符字段，实际 %d", total)
	}
	for _, it := range items {
		if len(it.Mismatched) == 0 {
			t.Errorf("OnlyMismatch 返回了无不符字段的行 id=%d", it.ID)
		}
	}

	items, total, err = ts.AdminListRecomputes(ctx, RecomputeFilter{LevelID: 2, Limit: 50})
	if err != nil {
		t.Fatalf("按关卡筛选失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].LevelID != 2 {
		t.Errorf("level_id=2 应命中 1 行，实际 total=%d", total)
	}

	// ⚠️ 用回读到的真实 id，而不是上面那个「标签」——
	//   两者数值不同（标签 33 / 真实 id 可能是 5）。用标签会静默查不到，
	//   症状是 total=0，看起来像「筛选没实现」而实际是「参数用错了」。
	items, total, err = ts.AdminListRecomputes(ctx, RecomputeFilter{UserID: ids[2], Limit: 50})
	if err != nil {
		t.Fatalf("按玩家筛选失败：%v", err)
	}
	if total != 1 || items[0].UserID != ids[2] {
		t.Errorf("按玩家筛选应命中 1 行，实际 total=%d", total)
	}
}

// TestAdminRecomputeStatsByLevel 守住按关卡聚合。
//
// 为什么它是主要判读方式：系统性漂移集中在少数关卡上，
// 而零星作弊均匀散布在单个玩家身上。
// 两者在同一张表里，**聚合维度**决定了你能不能一眼分出来。
func TestAdminRecomputeStatsByLevel(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 关卡 1：3 条里 3 条不符（系统性漂移的形状）
	// 关卡 2：3 条里 0 条不符（干净）
	plan := []struct {
		level    int
		mismatch bool
		n        int
	}{
		{1, true, 3},
		{2, false, 3},
	}
	for _, p := range plan {
		for i := 0; i < p.n; i++ {
			guest := fmt.Sprintf("g_st_%d_%d", p.level, i)
			ts.exec(t, `INSERT INTO users (guest_token, nickname, is_guest, status)
				VALUES ($1,$2,TRUE,1)`, guest, guest)
			fields := []string{}
			if p.mismatch {
				fields = []string{"score"}
			}
			ts.exec(t, `
				INSERT INTO settle_recomputes
				  (battle_id, user_id, level_id, reported_score, reported_kills,
				   reported_duration_ms, reported_hash, mismatched_fields, mode, engine_version)
				SELECT $1, id, $2, 100, 10, 60000, 'h', $3, 'shadow', 'skills-only/v1'
				FROM users WHERE guest_token=$4`,
				9000+i+p.level, p.level, fields, guest)
		}
	}

	stats, err := ts.AdminRecomputeStatsByLevel(ctx, time.Now().Add(-time.Hour), 20)
	if err != nil {
		t.Fatalf("按关卡聚合失败：%v", err)
	}
	if len(stats) != 2 {
		t.Fatalf("应聚合出 2 个关卡，实际 %d", len(stats))
	}
	// 不符率最高的关卡必须排第一 —— 漂移的关卡要一眼可见
	if stats[0].LevelID != 1 {
		t.Errorf("不符率最高的关卡应排第一，实际 level=%d（rate=%d‰）",
			stats[0].LevelID, stats[0].RatePermille)
	}
	if stats[0].RatePermille != 1000 {
		t.Errorf("关卡 1 的不符率应为 1000‰（3/3），实际 %d", stats[0].RatePermille)
	}
	if stats[1].RatePermille != 0 {
		t.Errorf("关卡 2 应为 0‰，实际 %d", stats[1].RatePermille)
	}
}
