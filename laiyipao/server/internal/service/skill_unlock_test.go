package service

// 技能等级解锁（README 已知边界第 29 条发放侧）的行为锚点。
//
// 三条判据，期望值全部用**数据口径**从 skills / skill_recipes 表反推，
// 不硬编码"24 个"或 ID 清单 —— 「注释与种子数据脱节」正是新号三焰开局
// 那个 bug 的成因（ID 重排后没人同步），测试再抄一份清单等于把同一个坑
// 埋进第三处。唯一例外是入门包 {1,4,7}：它钉的是产品决策本身，
// 未来改入门包时这条就该红，提醒同步改注释与文档（fail loud）。
//
//   1. 新号入门包是真三元素（焰/冰/电），槽位技能全部在拥有列表里
//      —— 装备校验（loadout.go「技能未拥有」）与 build 组装都以
//      user_skills 为准，槽位缺一个就是幽灵槽位（引擎静默少技能）；
//   2. 等级解锁按 floor(level_exp/100) 对标 unlock_level 补发**基础技能**，
//      合成技能（skill_recipes 的产出）无论多高等级都不经此途径发放，
//      装备它仍报「未解锁」—— 既有 400 契约不动；
//   3. 发放幂等：重复结算不重复发放；等级差 1 一个都不多发。

import (
	"context"
	"testing"

	"github.com/jackc/pgx/v5"
)

// starterOwned 返回新号拥有的技能 ID 集。
func starterOwned(t *testing.T, ts *testService, ctx context.Context, uid int64) map[int]bool {
	t.Helper()
	rows, err := ts.pool.Query(ctx,
		`SELECT skill_id FROM user_skills WHERE user_id = $1`, uid)
	if err != nil {
		t.Fatalf("读 user_skills 失败：%v", err)
	}
	defer rows.Close()
	out := map[int]bool{}
	for rows.Next() {
		var id int
		if err := rows.Scan(&id); err != nil {
			t.Fatalf("扫 skill_id 失败：%v", err)
		}
		out[id] = true
	}
	return out
}

// settleOnce 走一次 applyProgress（结算事务内的进度推进 + 等级解锁）。
func settleOnce(t *testing.T, ts *testService, ctx context.Context, uid int64, levelID int, kills int64) {
	t.Helper()
	if err := ts.DB.Tx(ctx, func(tx pgx.Tx) error {
		_, err := ts.applyProgress(ctx, tx, uid, levelID, true, kills)
		return err
	}); err != nil {
		t.Fatalf("applyProgress 失败：%v", err)
	}
}

// TestNewUserStarterIsTriElement 新号入门包必须真三元素且无幽灵槽位。
//
// 意图层（element 互异、槽位⊆拥有）+ 决策层（恰为 1/4/7）双钉：
// 前者保住 I-1「三元素触发反应链」的设计意图，后者钉住本轮产品决策 ——
// 两者任一被未来的 ID 重排/入门包调整破坏时，这条都会当场红，
// 而不是像上一版那样静默漂移成三焰开局。
func TestNewUserStarterIsTriElement(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	owned := starterOwned(t, ts, ctx, uid)
	if len(owned) != 3 {
		t.Fatalf("新号入门包应为 3 个技能，实际 %d 个：%v", len(owned), owned)
	}

	// 决策层：恰为 1 焰 ember / 4 冰 dryice / 7 电 arc。
	for _, id := range []int{1, 4, 7} {
		if !owned[id] {
			t.Errorf("新号应拥有技能 %d（三元素入门包）", id)
		}
	}

	// 意图层：三个技能的 element 互不相同（真三元素，能触发跨元素反应链）。
	var elements []string
	rows, err := ts.pool.Query(ctx, `
		SELECT DISTINCT s.element FROM user_skills us
		JOIN skills s ON s.id = us.skill_id WHERE us.user_id = $1`, uid)
	if err != nil {
		t.Fatalf("读入门包元素失败：%v", err)
	}
	for rows.Next() {
		var e string
		if err := rows.Scan(&e); err != nil {
			t.Fatalf("扫 element 失败：%v", err)
		}
		elements = append(elements, e)
	}
	rows.Close()
	if len(elements) != 3 {
		t.Errorf("入门包应覆盖 3 种元素（实际 %d 种：%v）—— 跨元素反应链触发不了", len(elements), elements)
	}

	// 幽灵槽位：槽位上的技能必须全部已拥有（build 组装按 user_skills 过滤，
	// 未拥有的槽位技能会被静默跳过 —— 引擎实际可用技能比界面显示的少）。
	var ghost int
	if err := ts.pool.QueryRow(ctx, `
		SELECT COUNT(*) FROM user_skill_slots uss
		WHERE uss.user_id = $1 AND NOT EXISTS (
			SELECT 1 FROM user_skills us WHERE us.user_id = uss.user_id AND us.skill_id = uss.skill_id)`,
		uid).Scan(&ghost); err != nil {
		t.Fatalf("数幽灵槽位失败：%v", err)
	}
	if ghost != 0 {
		t.Errorf("新号有 %d 个幽灵槽位（槽位技能未拥有，引擎会静默跳过）", ghost)
	}

	// 出战槽位非空（既有死局守卫的等价重申：slots[0] 必须有技能）。
	slots, err := ts.Loadout(ctx, uid)
	if err != nil || len(slots) == 0 || slots[0] == 0 {
		t.Errorf("新号默认出战槽位异常：%v, %v", slots, err)
	}
}

// TestSettleUnlocksBaseSkillsByLevel 结算按等级补发基础技能（集合精确 + 幂等 + 差 1 不发）。
func TestSettleUnlocksBaseSkillsByLevel(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 把经验推到「恰好解锁全部基础技能」的等级：基础技能最大 unlock_level × 100。
	// applyProgress 每场再 +50，所以预置 exp = maxUL*100 - 50，结算后恰好跨过阈值。
	var maxUL int
	if err := ts.pool.QueryRow(ctx, `
		SELECT COALESCE(MAX(unlock_level), 0) FROM skills
		WHERE id NOT IN (SELECT output_skill_id FROM skill_recipes)`).Scan(&maxUL); err != nil {
		t.Fatalf("读基础技能最大 unlock_level 失败：%v", err)
	}
	ts.exec(t, `UPDATE user_progress SET level_exp = $2 WHERE user_id = $1`,
		uid, int64(maxUL)*100-50)

	settleOnce(t, ts, ctx, uid, 1, 0)

	// 集合精确：拥有 == 全部基础技能（双向 EXCEPT 都为空）。
	for name, q := range map[string]string{
		"应有而未发": `SELECT COUNT(*) FROM (
			SELECT id FROM skills WHERE id NOT IN (SELECT output_skill_id FROM skill_recipes)
			  AND unlock_level <= FLOOR((SELECT level_exp FROM user_progress WHERE user_id = $1) / 100)
			EXCEPT
			SELECT skill_id FROM user_skills WHERE user_id = $1) t`,
		"发了不该发的": `SELECT COUNT(*) FROM (
			SELECT skill_id FROM user_skills WHERE user_id = $1
			EXCEPT
			SELECT id FROM skills WHERE unlock_level <= FLOOR((SELECT level_exp FROM user_progress WHERE user_id = $1) / 100)) t`,
	} {
		var n int
		if err := ts.pool.QueryRow(ctx, q, uid).Scan(&n); err != nil {
			t.Fatalf("%s 计数失败：%v", name, err)
		}
		if n != 0 {
			t.Errorf("等级解锁集合不精确（%s 差 %d 行）", name, n)
		}
	}

	// 幂等：再来一场（等级只会更高，但集合已封顶）不重复发放。
	settleOnce(t, ts, ctx, uid, 1, 0)
	var cnt int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM user_skills WHERE user_id = $1`, uid).Scan(&cnt); err != nil {
		t.Fatalf("数 user_skills 失败：%v", err)
	}
	var baseCnt int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM skills WHERE id NOT IN (SELECT output_skill_id FROM skill_recipes)`).
		Scan(&baseCnt); err != nil {
		t.Fatalf("数基础技能失败：%v", err)
	}
	if cnt != baseCnt {
		t.Errorf("重复结算后应为恰好 %d 行（基础技能数），实际 %d", baseCnt, cnt)
	}

	// 差 1 不发：另一个号 exp 推到「2 级 - 1」，结算 +50 后落在 2 级。
	// 判据用**结算前后差分**：礼包技能（4 冰 Lv3 / 7 电 Lv6）本就是超等级白送的
	// 礼包语义，等级解锁只补发差额 —— 新增行必须恰好等于
	// 「unlock_level<=2 的基础技能 ∧ 不在礼包里」，一个都不多发。
	uid2 := ts.newUser(t, ctx)
	before := starterOwned(t, ts, ctx, uid2)
	ts.exec(t, `UPDATE user_progress SET level_exp = $2 WHERE user_id = $1`, uid2, 199)
	settleOnce(t, ts, ctx, uid2, 1, 0)
	after := starterOwned(t, ts, ctx, uid2)

	newly := map[int]bool{}
	for id := range after {
		if !before[id] {
			newly[id] = true
		}
	}
	// 期望新增 = {unlock_level<=2 的基础技能} \ before（礼包里已有的不受等级影响）。
	rows, err := ts.pool.Query(ctx, `
		SELECT s.id FROM skills s
		WHERE s.unlock_level <= 2
		  AND s.id NOT IN (SELECT output_skill_id FROM skill_recipes)`)
	if err != nil {
		t.Fatalf("数 2 级应解锁的技能失败：%v", err)
	}
	want := map[int]bool{}
	for rows.Next() {
		var id int
		if err := rows.Scan(&id); err != nil {
			t.Fatalf("扫技能 ID 失败：%v", err)
		}
		if !before[id] {
			want[id] = true
		}
	}
	rows.Close()
	if err := rows.Err(); err != nil {
		t.Fatalf("遍历应解锁集合失败：%v", err)
	}
	if len(newly) != len(want) {
		t.Errorf("2 级结算应恰好新增 %d 个技能，实际新增 %d 个：%v", len(want), len(newly), newly)
	}
	for id := range want {
		if !newly[id] {
			t.Errorf("2 级应解锁技能 %d，实际未发", id)
		}
	}
	for id := range newly {
		if !want[id] {
			t.Errorf("技能 %d 不该在 2 级发放（礼包外多发）", id)
		}
	}
}

// TestCompositeSkillsNeverLevelUnlocked 合成技能不等级解锁、装备仍报未拥有。
//
// 这是「合成留给二期」的封条：若有人把 applyProgress 里的
// `NOT IN (SELECT output_skill_id FROM skill_recipes)` 删掉，
// 42 级以上的合成技能就会被等级解锁白送 —— 合成玩法等于提前报废。
func TestCompositeSkillsNeverLevelUnlocked(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 经验推到超过全部合成技能的 unlock_level（30..62），再结算。
	var cmpMaxUL int
	if err := ts.pool.QueryRow(ctx, `
		SELECT COALESCE(MAX(s.unlock_level), 0) FROM skills s
		WHERE s.id IN (SELECT output_skill_id FROM skill_recipes)`).Scan(&cmpMaxUL); err != nil {
		t.Fatalf("读合成技能最大 unlock_level 失败：%v", err)
	}
	ts.exec(t, `UPDATE user_progress SET level_exp = $2 WHERE user_id = $1`,
		uid, int64(cmpMaxUL)*100+1000)
	settleOnce(t, ts, ctx, uid, 1, 0)

	var leaked int
	if err := ts.pool.QueryRow(ctx, `
		SELECT COUNT(*) FROM user_skills us
		WHERE us.user_id = $1 AND us.skill_id IN (SELECT output_skill_id FROM skill_recipes)`,
		uid).Scan(&leaked); err != nil {
		t.Fatalf("数被等级解锁的合成技能失败：%v", err)
	}
	if leaked != 0 {
		t.Errorf("有 %d 个合成技能被等级解锁发放（应留给二期合成系统）", leaked)
	}

	// 装备合成技能仍报「未解锁」（SaveLoadout 以 user_skills 为拥有事实源）
	// —— 消费侧 400 契约不动；升级端点同语义（报「未拥有」，loadout.go UpgradeSkill）。
	var cmpID int
	if err := ts.pool.QueryRow(ctx,
		`SELECT output_skill_id FROM skill_recipes ORDER BY id LIMIT 1`).Scan(&cmpID); err != nil {
		t.Fatalf("取合成技能样本失败：%v", err)
	}
	_, err := ts.SaveLoadout(ctx, uid, SaveLoadoutInput{SkillIDs: []int{cmpID}})
	mustErr(t, err, "未解锁")
}
