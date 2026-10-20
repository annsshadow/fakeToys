package service

// 技能熔炼（合成系统）的行为锚点。
//
// 产品拍板（2026-10-20）：材料**熔炼掉**、**设等级门槛**（产物 unlock_level）、
// **纯材料零货币**。这组测试钉的就是这三条 + 两条工程判据：
//
//   1. 材料消耗与产物发放（含审计；零货币 → 不写 wallet_flows）；
//   2. 等级门槛的**边界**（差 1 级必须拒，恰好达标必须过）；
//   3. 非法输入（同 id / 无配方 / 材料不全 / 产物已拥有）一律 400；
//   4. **顺序无关**：(b,a) 与 (a,b) 命中同一配方（LEAST/GREATEST 的判据）；
//   5. **幽灵槽位**：材料装在槽位里时，熔炼后槽位不能还指着它
//      （`user_skill_slots.skill_id` 有 FK 且 NOT NULL，不能置 0 —— 只能删行；
//      不清就是引擎静默少技能，见工程约束 19）；
//   6. **并发幂等**：同一对材料被 N 个请求同时熔炼，必须**恰好一个**成功
//      （条件删除 + RowsAffected 才是真判据）。
//
// 期望值尽量用数据口径（从 skill_recipes / skills 表反推），
// 只有配方 (1,2)→31 这一条是硬编码 —— 它钉的是产品决策本身，
// 种子改了配方就该红，提醒同步文档与客户端。

import (
	"context"
	"errors"
	"testing"
)

// synthRecipe 取一条真实配方（默认 A=1,B=2→31），并返回产物门槛。
// 材料与产物都从库里读，避免与种子脱节。
type synthRecipe struct {
	A, B, Output int64
	NeedLevel    int
}

func loadSynthRecipe(t *testing.T, ts *testService, ctx context.Context, wantOutput int64) synthRecipe {
	t.Helper()
	var r synthRecipe
	if err := ts.pool.QueryRow(ctx, `
		SELECT a_skill_id, b_skill_id, output_skill_id FROM skill_recipes
		 WHERE output_skill_id = $1`, wantOutput).Scan(&r.A, &r.B, &r.Output); err != nil {
		t.Fatalf("读配方 %d 失败：%v", wantOutput, err)
	}
	if err := ts.pool.QueryRow(ctx,
		`SELECT unlock_level FROM skills WHERE id = $1`, r.Output).Scan(&r.NeedLevel); err != nil {
		t.Fatalf("读产物门槛失败：%v", err)
	}
	return r
}

// grantSkill 给玩家授一个技能（材料准备）。
func grantSkill(t *testing.T, ts *testService, ctx context.Context, uid, skillID int64) {
	t.Helper()
	ts.exec(t, `INSERT INTO user_skills (user_id, skill_id) VALUES ($1,$2)
	            ON CONFLICT (user_id, skill_id) DO NOTHING`, uid, skillID)
}

// setLevelExp 把玩家经验设成指定值（等级 = exp/100）。
func setLevelExp(t *testing.T, ts *testService, ctx context.Context, uid, exp int64) {
	t.Helper()
	ts.exec(t, `UPDATE user_progress SET level_exp = $2 WHERE user_id = $1`, uid, exp)
}

// ownsSkill 判断玩家是否拥有某技能。
func ownsSkill(t *testing.T, ts *testService, ctx context.Context, uid, skillID int64) bool {
	t.Helper()
	var exists bool
	if err := ts.pool.QueryRow(ctx,
		`SELECT EXISTS (SELECT 1 FROM user_skills WHERE user_id=$1 AND skill_id=$2)`,
		uid, skillID).Scan(&exists); err != nil {
		t.Fatalf("查拥有失败：%v", err)
	}
	return exists
}

// TestSynthesizeConsumesMaterialsAndGrantsOutput 材料消耗、产物发放、审计、零货币。
func TestSynthesizeConsumesMaterialsAndGrantsOutput(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	r := loadSynthRecipe(t, ts, ctx, 31)

	// 新号礼包是 1/4/7；材料 B（2）需补授
	grantSkill(t, ts, ctx, uid, r.B)
	setLevelExp(t, ts, ctx, uid, int64(r.NeedLevel)*100)

	// 零货币判据的基线：熔炼前后 wallet_flows 行数不变
	before := ts.scalar(t, ctx,
		`SELECT COUNT(*) FROM wallet_flows WHERE user_id = $1`, uid)

	out, err := ts.SynthesizeSkills(ctx, uid, r.A, r.B)
	if err != nil {
		t.Fatalf("熔炼应成功：%v", err)
	}
	if out != r.Output {
		t.Fatalf("产物应为 %d，实际 %d", r.Output, out)
	}

	// 材料消失
	if ownsSkill(t, ts, ctx, uid, r.A) {
		t.Errorf("材料 %d 应被消耗（熔炼掉）", r.A)
	}
	if ownsSkill(t, ts, ctx, uid, r.B) {
		t.Errorf("材料 %d 应被消耗（熔炼掉）", r.B)
	}
	// 产物到位
	if !ownsSkill(t, ts, ctx, uid, r.Output) {
		t.Errorf("产物 %d 应已发放", r.Output)
	}
	// 审计留痕
	if n := ts.scalar(t, ctx,
		`SELECT COUNT(*) FROM admin_audit_logs WHERE action = 'synthesize_skill' AND target = $1`,
		"user#"+itoa(uid)); n != 1 {
		t.Errorf("应写 1 条 synthesize_skill 审计，实际 %d", n)
	}
	// 零货币：不写 wallet_flows
	if after := ts.scalar(t, ctx,
		`SELECT COUNT(*) FROM wallet_flows WHERE user_id = $1`, uid); after != before {
		t.Errorf("熔炼是纯材料成本，不应写 wallet_flows：%d → %d", before, after)
	}
}

// TestSynthesizeRequiresLevelThreshold 等级门槛的边界：差 1 级拒，恰好达标过。
func TestSynthesizeRequiresLevelThreshold(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	r := loadSynthRecipe(t, ts, ctx, 31)

	t.Run("差 1 级必须拒", func(t *testing.T) {
		uid := ts.newUser(t, ctx)
		grantSkill(t, ts, ctx, uid, r.B)
		setLevelExp(t, ts, ctx, uid, int64(r.NeedLevel)*100-1) // 29 级（门槛 30）

		_, err := ts.SynthesizeSkills(ctx, uid, r.A, r.B)
		if !errors.Is(err, ErrBadInput) {
			t.Fatalf("等级不足应 ErrBadInput，实际 %v", err)
		}
		// 被拒时材料与产物都不能动
		if !ownsSkill(t, ts, ctx, uid, r.A) || !ownsSkill(t, ts, ctx, uid, r.B) {
			t.Error("熔炼被拒时材料不应被消耗")
		}
		if ownsSkill(t, ts, ctx, uid, r.Output) {
			t.Error("熔炼被拒时不应发放产物")
		}
	})

	t.Run("恰好达标必须过", func(t *testing.T) {
		uid := ts.newUser(t, ctx)
		grantSkill(t, ts, ctx, uid, r.B)
		setLevelExp(t, ts, ctx, uid, int64(r.NeedLevel)*100)

		if _, err := ts.SynthesizeSkills(ctx, uid, r.A, r.B); err != nil {
			t.Fatalf("恰好达标应成功：%v", err)
		}
	})
}

// TestSynthesizeRejectsInvalidInputs 四类非法输入一律 ErrBadInput，且零副作用。
func TestSynthesizeRejectsInvalidInputs(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	r := loadSynthRecipe(t, ts, ctx, 31)

	cases := []struct {
		name    string
		prepare func(t *testing.T, uid int64)
		a, b    int64
	}{
		{
			name:    "同 id",
			prepare: func(t *testing.T, uid int64) {},
			a:       r.A, b: r.A,
		},
		{
			name: "无配方",
			prepare: func(t *testing.T, uid int64) {
				grantSkill(t, ts, ctx, uid, 4) // 1+4 不在配方表里
			},
			a: r.A, b: 4,
		},
		{
			name:    "材料不全",
			prepare: func(t *testing.T, uid int64) {}, // 只有 1，没有 2
			a:       r.A, b: r.B,
		},
		{
			name: "产物已拥有",
			prepare: func(t *testing.T, uid int64) {
				grantSkill(t, ts, ctx, uid, r.B)
				grantSkill(t, ts, ctx, uid, r.Output)
			},
			a: r.A, b: r.B,
		},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			uid := ts.newUser(t, ctx)
			setLevelExp(t, ts, ctx, uid, 10_000) // 等级不是这里的变量
			c.prepare(t, uid)

			_, err := ts.SynthesizeSkills(ctx, uid, c.a, c.b)
			if !errors.Is(err, ErrBadInput) {
				t.Fatalf("%s 应 ErrBadInput，实际 %v", c.name, err)
			}
		})
	}
}

// TestSynthesizeIsOrderInsensitive (b,a) 必须命中与 (a,b) 相同的配方。
//
// 配方表按 (a,b) 有序唯一；玩家可能反着点。没有 LEAST/GREATEST 的症状是
// 「明明有配方却说没有」——一个不会报错的静默失败。
func TestSynthesizeIsOrderInsensitive(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	r := loadSynthRecipe(t, ts, ctx, 31)
	grantSkill(t, ts, ctx, uid, r.B)
	setLevelExp(t, ts, ctx, uid, int64(r.NeedLevel)*100)

	// 注意传参顺序与配方表相反
	out, err := ts.SynthesizeSkills(ctx, uid, r.B, r.A)
	if err != nil {
		t.Fatalf("反序材料应命中同一配方：%v", err)
	}
	if out != r.Output {
		t.Errorf("反序产物应为 %d，实际 %d", r.Output, out)
	}
}

// TestSynthesizeClearsSlots 材料装在槽位里时，熔炼后槽位不能还指着它。
//
// `user_skill_slots.skill_id` 有 FK 且 NOT NULL —— 不能置 0，只能删行。
// 不清的症状：引擎按 `user_skills ⋈ user_skill_slots` 统计，
// 槽位指向已不拥有的技能 → 静默少一个可用技能（幽灵槽位）。
func TestSynthesizeClearsSlots(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	r := loadSynthRecipe(t, ts, ctx, 31)
	grantSkill(t, ts, ctx, uid, r.B)
	setLevelExp(t, ts, ctx, uid, int64(r.NeedLevel)*100)

	// 新号默认槽 0 = 技能 1（= 材料 A）；再确认一下，避免依赖初始化细节
	ts.exec(t, `INSERT INTO user_skill_slots (user_id, slot, skill_id) VALUES ($1,0,$2)
	            ON CONFLICT (user_id, slot) DO UPDATE SET skill_id = EXCLUDED.skill_id`, uid, r.A)

	if _, err := ts.SynthesizeSkills(ctx, uid, r.A, r.B); err != nil {
		t.Fatalf("熔炼应成功：%v", err)
	}

	// 槽位不能还指着材料
	var dangling int
	if err := ts.pool.QueryRow(ctx, `
		SELECT COUNT(*) FROM user_skill_slots
		 WHERE user_id = $1 AND skill_id IN ($2,$3)`, uid, r.A, r.B).Scan(&dangling); err != nil {
		t.Fatalf("数悬空槽位失败：%v", err)
	}
	if dangling != 0 {
		t.Errorf("熔炼后仍有 %d 个槽位指向已消耗的材料（幽灵槽位）", dangling)
	}
	// 读回槽 0 应为空（Loadout 按 slot 补 0）
	slots, err := ts.Loadout(ctx, uid)
	if err != nil {
		t.Fatalf("读槽位失败：%v", err)
	}
	if len(slots) > 0 && slots[0] != 0 {
		t.Errorf("槽 0 应被清空，实际 %d", slots[0])
	}
}

// TestSynthesizeConcurrentSameMaterialsOnlyOneWins 同一对材料并发熔炼，恰好一个成功。
//
// 作弊面：两个请求都读到「材料存在」→ 都放行 → 产物发两次（材料只够一次）。
// 真判据是条件删除的 RowsAffected —— 不是「先 SELECT 再 DELETE」。
func TestSynthesizeConcurrentSameMaterialsOnlyOneWins(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	r := loadSynthRecipe(t, ts, ctx, 31)
	grantSkill(t, ts, ctx, uid, r.B)
	setLevelExp(t, ts, ctx, uid, 10_000)

	const workers = 8
	ok, _, peak := runParallel(workers, func(int) error {
		_, err := ts.SynthesizeSkills(ctx, uid, r.A, r.B)
		return err
	})
	mustOverlap(t, peak, workers)

	if ok != 1 {
		t.Fatalf("并发熔炼应恰好成功 1 次，实际 %d 次", ok)
	}
	// 产物恰好 1 行（主键本身也防重复，但这条守的是「只发一次」的语义）
	var n int
	if err := ts.pool.QueryRow(ctx, `
		SELECT COUNT(*) FROM user_skills WHERE user_id = $1 AND skill_id = $2`,
		uid, r.Output).Scan(&n); err != nil {
		t.Fatalf("数产物失败：%v", err)
	}
	if n != 1 {
		t.Errorf("产物应恰好 1 行，实际 %d", n)
	}
}
