package service

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"sort"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

// ComputeRatingFor / ComputePowerFor 是给 HTTP 层用的导出包装。
func (s *Service) ComputeRatingFor(userID int64, build map[string]any) domain.BuildRating {
	return s.computeRating(s.ctxBackground(), userID, build)
}

// ComputePowerFor 是给运营接口用的包装。
//
// ⚠️ 第 103 轮移除了 `userID` —— 第 102 轮把 `computePower` 变成纯函数之后，
// 它就成了纯装饰。而 `ComputeRatingFor`（**同名的兄弟**）**确实**要查
// `user_progress`，仍然需要 userID —— 于是两个签名一度一模一样。
//
// 由 `TestNoFunctionIgnoresAParameter` 抓到：**级联**发现，
// 因为第 102 轮那条守卫只盯 `ctx`。
func (s *Service) ComputePowerFor(build map[string]any) int64 {
	return computePower(build)
}

func (s *Service) ctxBackground() context.Context { return context.Background() }

// --- 专精点（I-3） ---

// LoadMastery 返回玩家专精点状态。
func (s *Service) LoadMastery(ctx context.Context, userID int64) (int, []int, error) {
	rows, err := s.pool.Query(ctx,
		`SELECT node_id FROM user_mastery_nodes WHERE user_id = $1 ORDER BY node_id`, userID)
	if err != nil {
		return 0, nil, fmt.Errorf("load mastery: %w", err)
	}
	defer rows.Close()
	var ids []int
	for rows.Next() {
		var id int
		if err := rows.Scan(&id); err != nil {
			return 0, nil, err
		}
		ids = append(ids, id)
	}
	if err := rows.Err(); err != nil {
		return 0, nil, err
	}

	var points int
	if err := s.pool.QueryRow(ctx,
		`SELECT mastery_points FROM user_progress WHERE user_id = $1`, userID).Scan(&points); err != nil {
		return 0, nil, fmt.Errorf("load mastery points: %w", err)
	}
	return points, ids, nil
}

// AllocateMastery 分配一个专精点。
//
// 校验全部交给 domain.EvaluateMastery —— HTTP 层不做"点一下就写入"，
// 避免前端绕过每层 2 个的限制。
func (s *Service) AllocateMastery(ctx context.Context, userID int64, nodeID int) error {
	return s.DB.Tx(ctx, func(tx txType) error {
		var points int
		if err := tx.QueryRow(ctx,
			`SELECT mastery_points FROM user_progress WHERE user_id = $1 FOR UPDATE`, userID).
			Scan(&points); err != nil {
			return fmt.Errorf("lock progress: %w", err)
		}

		selected, err := s.masterySelected(ctx, tx, userID)
		if err != nil {
			return err
		}
		if selected[nodeID] {
			return fmt.Errorf("%w: 该节点已点亮（取消请走重置）", ErrForbidden)
		}
		next := map[int]bool{}
		for k, v := range selected {
			next[k] = v
		}
		next[nodeID] = true

		var all []domain.MasteryNode
		for _, f := range domain.AllMasteryFamilies() {
			all = append(all, f.Nodes...)
		}
		if _, err := domain.EvaluateMastery(all, next, points); err != nil {
			return fmt.Errorf("%w: %s", ErrBadInput, err.Error())
		}
		if _, err := tx.Exec(ctx,
			`INSERT INTO user_mastery_nodes (user_id, node_id) VALUES ($1,$2)
			 ON CONFLICT (user_id, node_id) DO NOTHING`, userID, nodeID); err != nil {
			return fmt.Errorf("insert mastery node: %w", err)
		}
		return nil
	})
}

// lockChallengeStateOrdered 按**固定顺序**锁住一次挑战会触碰的全部跨用户行（第 79 轮）。
//
// # 缺陷：两个对向挑战必然死锁（有两处，不是��处）
//
// `ChallengeDefense` 会碰到**两个人的**三类行：
//
//  1. `user_daily_challenges`（挑战次数 / 被偷次数）
//     挑战者一行、被挑战者一行
//  2. `user_wallets`（窃取与发放）
//     被挑战者一行、挑战者一行
//  3. `defenses`（wins / losses）—— **只有被挑战者那一条**，无冲突
//
// 修复前的实际顺序是：
//
//	dailyChallengeCounters(挑战者)   ← 第 583 行，INSERT…ON CONFLICT DO UPDATE 会**拿行锁**
//	dailyChallengeCounters(被挑战者) ← 第 592 行
//	grantWallet(被挑战者, -take)     ← 第 702 行
//	grantWallet(挑战者, +stolen)     ← 第 726 行
//
// 也就是「挑战者→被挑战者」与「被挑战者→挑战者」**同时存在**。
// A 打 B 与 B 打 A 同时发生时：
//
//	Tx1  锁 counters[A] → counters[B] → wallets[B] → 等 wallets[A]
//	Tx2  锁 counters[B] → 等 counters[A]        → ...
//
// → PG `40P01 deadlock_detected` → failErr → **500**。
//
// 这是**自伤型 500**：两个玩家各点了一次挑战，服务端回 500，
// 而任何人看日志都得不出「是自己这边的问题」。
//
// 我第一版只修了钱包顺序（加 `ORDER BY user_id FOR UPDATE`），
// 实测**仍然死锁** 12 次里的 6 次 —— 因为计数器行在更早就被锁了。
// **只修一处并发原语，剩下那处会立刻把问题重新暴露出来。**
//
// # 修法：表分轮 + id 升序
//
//	第一轮：对**每个** id 升序 ensure+lock `user_daily_challenges`
//	第二轮：对**每个** id 升序 lock `user_wallets`
//
// 关键是**表顺序也必须固定**。若按「每个用户先 counters 再 wallets」，
// Tx1 是 `counters[A] wallets[A] counters[B] wallets[B]`，
// Tx2 是 `counters[B] wallets[B] counters[A] wallets[A]` —— 仍然互等。
// 分轮之后两个事务的加锁序列**逐字相同**，不可能互等。
//
// # 为什么不能用 pg_advisory_xact_lock
//
// advisory lock 与这两张表的行锁**不是同一把锁** ——
// 后续的 `INSERT … ON CONFLICT DO UPDATE` 与 `UPDATE user_wallets`
// 仍然按任意顺序拿行锁，死锁依旧。必须让**同一把**（行锁）有序获取。
//
// # 为什么 ensure 用 `ON CONFLICT DO UPDATE` 而不是 `DO NOTHING`
//
// `DO NOTHING` 对**已存在**的行不加锁 —— 那样预锁就漏掉了它们。
// `DO UPDATE SET challenge_date = $2`（$2 是同一个日期字面量）是个无操作更新
// （值没变），但它**确实**拿行锁。
// 「无操作更新」这个手法要写清楚，否则后来的人会以为是脏写法。
func lockChallengeStateOrdered(ctx context.Context, tx txType, userIDs []int64) error {
	// ⚠️ 第 98 轮：`challenge_date` 由 **Go** 决定，与每日任务 / 签到 / 商城同一个口径。
	//
	// 原来五处都用 `CURRENT_DATE`，由 **PG 会话时区**折算。
	// 防线挑战内部因此自洽，但它与**每日任务**的日界不同 —— 而两者
	// 语义上必须对齐：「今日通关 3 关」这个任务与「今日挑战次数上限」
	// 说的是同一个「今日」。
	//
	// 两者不同时刻翻页时（Go 本地 ≠ PG 会话时区，容器里 `time.Local` 常为 UTC）：
	// 玩家在凌晨到早上做的通关，任务在 08:00 就重置了，
	// 而挑战次数要等到北京 00:00 才重置 —— 于是「今日」有两个定义。
	//
	// 传**日期字面量**（与第 89 轮签到、第 97 轮商城同一手法）。
	today := periodStart(time.Now(), "daily").Format("2006-01-02")
	ids := dedupPositive(userIDs)
	if len(ids) == 0 {
		return nil
	}
	sort.Slice(ids, func(i, j int) bool { return ids[i] < ids[j] })

	// 第一轮：挑战计数器（**必须**先于钱包，全局固定）
	for _, id := range ids {
		// 无操作更新只为拿行锁 —— 见函数头的说明。
		if _, err := tx.Exec(ctx, `
			INSERT INTO user_daily_challenges (user_id, challenge_date, attempts, stolen_times)
			VALUES ($1, $2, 0, 0)
			ON CONFLICT (user_id, challenge_date)
			DO UPDATE SET challenge_date = $2`, id, today); err != nil {
			return fmt.Errorf("lock challenge counters of %d: %w", id, err)
		}
	}

	// 第二轮：钱包
	if _, err := tx.Exec(ctx, `
		SELECT user_id FROM user_wallets
		 WHERE user_id = ANY($1) ORDER BY user_id FOR UPDATE`, ids); err != nil {
		return fmt.Errorf("lock wallets: %w", err)
	}
	return nil
}

// dedupPositive 去重并丢弃非正 id。
//
// 去重的理由不是正确性（`ANY($1)` 命中重复无害）而是可观测性：
// 「锁了几行」这件事在日志/错误信息里要说得清。
func dedupPositive(ids []int64) []int64 {
	seen := make(map[int64]bool, len(ids))
	out := make([]int64, 0, len(ids))
	for _, id := range ids {
		if id <= 0 || seen[id] {
			continue
		}
		seen[id] = true
		out = append(out, id)
	}
	return out
}

func (s *Service) masterySelected(ctx context.Context, tx txType, userID int64) (map[int]bool, error) {
	rows, err := tx.Query(ctx,
		`SELECT node_id FROM user_mastery_nodes WHERE user_id = $1`, userID)
	if err != nil {
		return nil, fmt.Errorf("load selected: %w", err)
	}
	defer rows.Close()
	out := map[int]bool{}
	for rows.Next() {
		var id int
		if err := rows.Scan(&id); err != nil {
			return nil, err
		}
		out[id] = true
	}
	return out, rows.Err()
}

// --- 诊断（L-1） ---

// Diagnose 分析玩家在某关的卡点。
func (s *Service) Diagnose(ctx context.Context, userID int64, levelID, failedTimes int) (domain.Diagnose, error) {
	if levelID < 1 || levelID > domain.TotalLevels {
		return domain.Diagnose{}, fmt.Errorf("%w: 关卡 %d 不存在", ErrBadInput, levelID)
	}
	gl := domain.GenerateLevel(levelID)

	// 取最近若干局该关的战报做样本
	rows, err := s.pool.Query(ctx,
		`SELECT kills, elements_used FROM battle_records
		 WHERE user_id = $1 AND level_id = $2 ORDER BY created_at DESC LIMIT 10`, userID, levelID)
	if err != nil {
		return domain.Diagnose{}, fmt.Errorf("load recent battles: %w", err)
	}
	defer rows.Close()

	in := domain.DiagnoseInput{
		LevelID: levelID, FailedTimes: failedTimes,
		TotalEnemies: gl.TotalEnemies(),
		ElementsUsed: map[string]int{},
	}
	killsSum := 0
	samples := 0
	for rows.Next() {
		var kills int
		var elemRaw []byte
		if err := rows.Scan(&kills, &elemRaw); err != nil {
			return domain.Diagnose{}, err
		}
		elems := map[string]int{}
		_ = json.Unmarshal(elemRaw, &elems)
		for k, v := range elems {
			in.ElementsUsed[k] += v
		}
		killsSum += kills
		samples++
	}
	if err := rows.Err(); err != nil {
		return domain.Diagnose{}, err
	}
	if samples > 0 {
		in.Kills = killsSum / samples
	}

	// 取玩家当前技能携带的元素
	_, elements, err := s.loadSkillsAndSlots(ctx, int64(userID))
	if err != nil {
		return domain.Diagnose{}, err
	}
	for _, e := range elements {
		in.Loadout = append(in.Loadout, domain.Element(e))
	}
	sort.Slice(in.Loadout, func(i, j int) bool { return in.Loadout[i] < in.Loadout[j] })

	return domain.DiagnoseFailure(in, gl), nil
}

// --- 防线（I-5） ---

// DefenseView 是防线展示对象。
type DefenseView struct {
	ID               int64          `json:"id"`
	OwnerID          int64          `json:"owner_id"`
	OwnerName        string         `json:"owner_name"`
	Name             string         `json:"name"`
	Power            int64          `json:"power"`
	ElementCoverage  int            `json:"element_coverage"`
	MasteryDone      int            `json:"mastery_done"`
	Wins             int            `json:"wins"`
	Losses           int            `json:"losses"`
	ShieldedUntil    *time.Time     `json:"shielded_until,omitempty"`
	Snapshot         map[string]any `json:"snapshot,omitempty"`
	SnapshotHash     string         `json:"snapshot_hash,omitempty"`
	ChallengedToday  int            `json:"challenged_today"`
	MyAttemptsToday  int            `json:"my_attempts_today"`
	MyStolenToday    int            `json:"my_stolen_today"`
	AttemptLimit     int            `json:"attempt_limit"`
	CanChallenge     bool           `json:"can_challenge"`
	ChallengeBlocked string         `json:"challenge_blocked,omitempty"`
}

// DefenseAttemptLimit 每日挑战次数上限。
const DefenseAttemptLimit = 3

// DefenseStolenLimit 每日被偷次数上限。
const DefenseStolenLimit = 2

// ListDefenses 返回候选防线（不含自己）与自己的防线。
func (s *Service) ListDefenses(ctx context.Context, userID int64) (mine *DefenseView, candidates []DefenseView, err error) {
	attempts, _, err := s.dailyChallengeCounters(ctx, userID)
	if err != nil {
		return nil, nil, err
	}

	rows, err := s.pool.Query(ctx, `
		SELECT d.id, d.owner_id, u.nickname, d.name, d.power, d.element_coverage, d.mastery_done,
		       d.wins, d.losses, d.shielded_until, d.snapshot, d.snapshot_hash
		FROM defenses d JOIN users u ON u.id = d.owner_id
		WHERE d.owner_id <> $1 AND u.status = 1
		ORDER BY d.power DESC LIMIT 20`, userID)
	if err != nil {
		return nil, nil, fmt.Errorf("list defenses: %w", err)
	}
	defer rows.Close()
	for rows.Next() {
		var v DefenseView
		if err := rows.Scan(&v.ID, &v.OwnerID, &v.OwnerName, &v.Name, &v.Power,
			&v.ElementCoverage, &v.MasteryDone, &v.Wins, &v.Losses, &v.ShieldedUntil,
			&v.Snapshot, &v.SnapshotHash); err != nil {
			return nil, nil, err
		}
		v.CanChallenge = attempts < DefenseAttemptLimit
		if v.CanChallenge {
			v.ChallengeBlocked = ""
		} else {
			v.ChallengeBlocked = fmt.Sprintf("今日挑战次数已用尽（%d/%d）", attempts, DefenseAttemptLimit)
		}
		if v.ShieldedUntil != nil && v.ShieldedUntil.After(time.Now()) {
			// 护盾期内不可挑战，必须同时关掉 can_challenge，
			// 否则客户端只看 can_challenge 就会放行，服务端再拒绝 —— 体验割裂。
			v.CanChallenge = false
			v.ChallengeBlocked = "对方开启了 24 小时护盾"
		}
		candidates = append(candidates, v)
	}
	if err := rows.Err(); err != nil {
		return nil, nil, err
	}

	var own DefenseView
	err = s.pool.QueryRow(ctx, `
		SELECT d.id, d.owner_id, u.nickname, d.name, d.power, d.element_coverage, d.mastery_done,
		       d.wins, d.losses, d.shielded_until, d.snapshot, d.snapshot_hash
		FROM defenses d JOIN users u ON u.id = d.owner_id
		WHERE d.owner_id = $1`, userID).
		Scan(&own.ID, &own.OwnerID, &own.OwnerName, &own.Name, &own.Power,
			&own.ElementCoverage, &own.MasteryDone, &own.Wins, &own.Losses,
			&own.ShieldedUntil, &own.Snapshot, &own.SnapshotHash)
	if err == nil {
		// snapshot 落库是 jsonb，Scan 进 map 后即已解码，无需再 Unmarshal
		mine = &own
	}
	return mine, candidates, nil
}

func (s *Service) dailyChallengeCounters(ctx context.Context, userID int64) (attempts, stolen int, err error) {
	// ⚠️ 第 98 轮：`challenge_date` 由 **Go** 决定，与每日任务 / 签到 / 商城同一个口径。
	//
	// 原来五处都用 `CURRENT_DATE`，由 **PG 会话时区**折算。
	// 防线挑战内部因此自洽，但它与**每日任务**的日界不同 —— 而两者
	// 语义上必须对齐：「今日通关 3 关」这个任务与「今日挑战次数上限」
	// 说的是同一个「今日」。
	//
	// 两者不同时刻翻页时（Go 本地 ≠ PG 会话时区，容器里 `time.Local` 常为 UTC）：
	// 玩家在凌晨到早上做的通关，任务在 08:00 就重置了，
	// 而挑战次数要等到北京 00:00 才重置 —— 于是「今日」有两个定义。
	//
	// 传**日期字面量**（与第 89 轮签到、第 97 轮商城同一手法）。
	today := periodStart(time.Now(), "daily").Format("2006-01-02")
	err = s.pool.QueryRow(ctx, `
		INSERT INTO user_daily_challenges (user_id, challenge_date, attempts, stolen_times)
		VALUES ($1, $2, 0, 0)
		ON CONFLICT (user_id, challenge_date) DO UPDATE SET challenge_date = $2
		RETURNING attempts, stolen_times`, userID, today).Scan(&attempts, &stolen)
	return attempts, stolen, err
}

// SaveDefenseInput 是保存防线的请求。
type SaveDefenseInput struct {
	Name         string `json:"name"`
	Skills       []int  `json:"skills"`
	Equipment    []int  `json:"equipment"`
	MasteryNodes []int  `json:"mastery_nodes"`
	// Works 是三件防御工事的编码（slow_belt / block_wall / tesla_grid）
	Works       []string `json:"works"`
	ShieldHours int      `json:"shield_hours"`
}

// SaveDefense 保存/更新玩家防线快照。
//
// 快照是"我的构筑"的固化：挑战者在客户端用这套构筑本地模拟，
// 服务端不跑战斗引擎（这是 I-5 能成立的关键）。
// validDefenseWorks 是允许的工程装置白名单。
//
// ⚠️ 必须白名单而不是黑名单：装置直接折算成攻方属性加成
// （见 miniapp/src/game/defense.ts 的 applyWorks），
// 传一个未知 code 进来虽然会被忽略，但传已知 code 的**非法组合**
// 就能凭空获得三份加成。
var validDefenseWorks = map[string]bool{
	"slow_belt":  true,
	"block_wall": true,
	"tesla_grid": true,
}

// validateDefenseOwnership 校验上报的构筑项确实属于该用户。
//
// ⚠️ 这是必需的，不是防御性编程：I-5 的设计是「挑战者用这份快照
// 在本地模拟」，所以快照内容直接决定别人挑战你的基准。
// 不校验的话玩家可以冻结一份从未拥有过的技能/装备/专精构成的防线，
// 让所有挑战者的模拟结果失去意义。
//
// 参照实现是 SaveLoadout（loadout.go），它对技能做了同样的校验。
func (s *Service) validateDefenseOwnership(ctx context.Context, userID int64, in SaveDefenseInput) error {
	if len(in.Skills) > 12 {
		return fmt.Errorf("%w: 技能数量 %d 过多", ErrBadInput, len(in.Skills))
	}
	for _, id := range in.Skills {
		if id <= 0 {
			continue
		}
		var n int
		if err := s.pool.QueryRow(ctx,
			`SELECT COUNT(*) FROM user_skills WHERE user_id = $1 AND skill_id = $2`,
			userID, id).Scan(&n); err != nil {
			return err
		}
		if n == 0 {
			return fmt.Errorf("%w: 技能 %d 未解锁", ErrBadInput, id)
		}
	}
	for _, id := range in.Equipment {
		if id <= 0 {
			continue
		}
		var n int
		if err := s.pool.QueryRow(ctx,
			`SELECT COUNT(*) FROM user_equipment WHERE user_id = $1 AND equipment_id = $2`,
			userID, id).Scan(&n); err != nil {
			return err
		}
		if n == 0 {
			return fmt.Errorf("%w: 装备 %d 未拥有", ErrBadInput, id)
		}
	}
	for _, node := range in.MasteryNodes {
		if node <= 0 {
			continue
		}
		var n int
		if err := s.pool.QueryRow(ctx,
			`SELECT COUNT(*) FROM user_mastery_nodes WHERE user_id = $1 AND node_id = $2`,
			userID, node).Scan(&n); err != nil {
			return err
		}
		if n == 0 {
			return fmt.Errorf("%w: 专精节点 %d 未投入", ErrBadInput, node)
		}
	}
	if len(in.Works) > 3 {
		return fmt.Errorf("%w: 工程装置最多 3 个", ErrBadInput)
	}
	for _, w := range in.Works {
		if !validDefenseWorks[w] {
			return fmt.Errorf("%w: 未知工程装置 %q", ErrBadInput, w)
		}
	}
	// 护盾时长必须收敛。传 9999999 等于永久护盾，
	// 而重开护盾既无冷却也无成本 —— 等于免费获得无限免偷。
	if in.ShieldHours < 0 || in.ShieldHours > MaxDefenseShieldHours {
		return fmt.Errorf("%w: 护盾时长须在 0..%d 小时", ErrBadInput, MaxDefenseShieldHours)
	}
	return nil
}

// MaxDefenseShieldHours 护盾时长上限（小时）。
const MaxDefenseShieldHours = 24

// MaxDefenseNameLen 防线名长度上限。
const MaxDefenseNameLen = 32

func (s *Service) SaveDefense(ctx context.Context, userID int64, in SaveDefenseInput) (DefenseView, error) {
	if in.Name == "" {
		in.Name = "我的防线"
	}
	if len([]rune(in.Name)) > MaxDefenseNameLen {
		return DefenseView{}, fmt.Errorf("%w: 防线名超过 %d 字", ErrBadInput, MaxDefenseNameLen)
	}
	if err := s.validateDefenseOwnership(ctx, userID, in); err != nil {
		return DefenseView{}, err
	}
	build, err := s.LoadBuildSnapshot(ctx, userID)
	if err != nil {
		return DefenseView{}, err
	}
	elements := buildElements(build)
	mastery, _ := build["mastery_nodes"].([]int)
	rating := s.computeRating(ctx, userID, build)
	power := computePower(build)

	snapshot := map[string]any{
		"skills":        in.Skills,
		"equipment":     in.Equipment,
		"mastery_nodes": in.MasteryNodes,
		"works":         in.Works,
		"elements":      elements,
		"mastery":       mastery,
		"rating":        rating,
	}
	snapshotRaw, err := json.Marshal(snapshot)
	if err != nil {
		return DefenseView{}, err
	}
	hash := domain.SnapshotHash(snapshotRaw)

	var shielded any
	if in.ShieldHours > 0 {
		shielded = time.Now().Add(time.Duration(in.ShieldHours) * time.Hour)
	}

	// ⚠️ upsert / DELETE / INSERT 三条语句必须在**同一事务**内。
	// 之前逐条用 s.pool 执行：并发两次保存会交错成
	// A DELETE → B DELETE → A INSERT slot0 → B INSERT slot0，
	// 第二个撞上 idx_defense_works_slot 唯一索引 → 500，
	// 且 defense_works 停留在部分写入状态（工事数少于快照里的）。
	// 中途崩溃同样会让 works 被清空而 snapshot 已更新，两者永久不一致。
	var id int64
	err = s.DB.Tx(ctx, func(tx txType) error {
		// 一个玩家一条防线，defenses.owner_id 建了唯一索引，用 upsert
		if err := tx.QueryRow(ctx, `
			INSERT INTO defenses (owner_id, name, power, element_coverage, mastery_done,
			                      snapshot, snapshot_hash, shielded_until)
			VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
			ON CONFLICT (owner_id) DO UPDATE SET
			  name = EXCLUDED.name, power = EXCLUDED.power,
			  element_coverage = EXCLUDED.element_coverage, mastery_done = EXCLUDED.mastery_done,
			  snapshot = EXCLUDED.snapshot, snapshot_hash = EXCLUDED.snapshot_hash,
			  shielded_until = EXCLUDED.shielded_until, updated_at = now()
			RETURNING id`,
			userID, in.Name, power, rating.ElementCoverage, rating.MasteryDone,
			snapshotRaw, hash, shielded).Scan(&id); err != nil {
			return fmt.Errorf("save defense: %w", err)
		}

		// 同步工程装置
		if _, err := tx.Exec(ctx, `DELETE FROM defense_works WHERE user_id = $1`, userID); err != nil {
			return err
		}
		for i, code := range in.Works {
			if i >= 3 {
				break
			}
			if _, err := tx.Exec(ctx,
				`INSERT INTO defense_works (user_id, code, level, slot) VALUES ($1,$2,$3,$4)`,
				userID, code, 1, i); err != nil {
				return fmt.Errorf("save works: %w", err)
			}
		}
		return nil
	})
	if err != nil {
		return DefenseView{}, err
	}
	return DefenseView{
		ID: id, OwnerID: userID, Name: in.Name, Power: power,
		ElementCoverage: rating.ElementCoverage, MasteryDone: rating.MasteryDone,
		Snapshot: snapshot, SnapshotHash: hash,
	}, nil
}

// ChallengeInput 是挑战请求。
type ChallengeInput struct {
	Seed       int64  `json:"seed"`
	Won        bool   `json:"won"`
	DurationMs int    `json:"duration_ms"`
	HPLeftPct  int    `json:"hp_left_pct"`
	ReplayHash string `json:"replay_hash"`
}

// 挑战上报字段的边界（第 69 轮）。
//
// # 缺陷：三个字段零校验，而目标列都是 INTEGER / TEXT
//
// `ChallengeInput` 的 `DurationMs` / `HPLeftPct` / `ReplayHash` 此前
// **一个都没有校验**，而 `defense_challenges` 的对应列是：
//
//	duration_ms  INTEGER
//	hp_left_pct  INTEGER  -- 挑战者剩余血量百分比
//	replay_hash  TEXT
//
// 于是上报 `duration_ms: 2147483648` 或 `hp_left_pct: 2147483648` 时：
//
//	PG ERROR 22003 integer out of range -> failErr -> **500**
//
// 任何玩家都能定向让任意目标的挑战接口持续 500。
//
// # 与结算面的对比
//
// `SettleInput` 那一侧第 62/63 轮刚补过边界（集合字段溢出、duration_ms 上界）。
// 这里是同一族的**另一个入口** —— 反射式覆盖率守卫 `settle_coverage_test.go`
// 只管 `SettleInput`，所以防线挑战这条路径一直没人看。
//
// 这正是「守卫覆盖了一个结构、漏了同族的另一个」。
const (
	// MaxChallengeDurationMs 与结算面的 MaxPlausibleDurationMs 同量级（24h）。
	MaxChallengeDurationMs = 24 * 60 * 60 * 1000
	// MaxChallengeReplayHashLen 与结算面的 maxReplayHashLen 同值。
	//
	// 引擎产出的是 `hex16(fnv1a64(...))` = 16 个十六进制字符，
	// 取 64 是给「将来换哈希算法」留余量，同时拒绝「传一段文章」。
	MaxChallengeReplayHashLen = 64
)

// ValidateChallengeInput 校验挑战上报的字段边界。
func ValidateChallengeInput(in ChallengeInput) error {
	// 时长：必须为正（与结算面同），且有上界。
	// 下界 0 意味着「这一局 0 毫秒就结束了」—— 那不是任何真实对局。
	if in.DurationMs <= 0 || in.DurationMs > MaxChallengeDurationMs {
		return fmt.Errorf("%w：挑战时长 %dms 必须在 (0, %d] 内",
			domain.ErrInvalidField, in.DurationMs, MaxChallengeDurationMs)
	}
	// 剩余血量百分比：语义上是 0..100 的整数。
	//
	// 上界不是「随便取个大的」—— hp_left_pct 是**百分比**，
	// 100 之外的值在分析与展示上都没有意义，
	// 而它会直接进运营看板上「挑战者剩余血量」那一列。
	if in.HPLeftPct < 0 || in.HPLeftPct > 100 {
		return fmt.Errorf("%w：剩余血量百分比 %d 必须在 [0, 100] 内",
			domain.ErrInvalidField, in.HPLeftPct)
	}
	// 回放哈希：长度上限与结算面一致。
	//
	// 同一处 TEXT 列若不限长，单次可写入接近 1MB ——
	// 而这个端点**没有归属校验**（见 README 已知边界的第 3 条），
	// 任意注册用户都能对任意 defense_id 写入。
	if len(in.ReplayHash) > MaxChallengeReplayHashLen {
		return fmt.Errorf("%w：replay_hash 长 %d 字符，上限 %d",
			domain.ErrInvalidField, len(in.ReplayHash), MaxChallengeReplayHashLen)
	}
	return nil
}

// ChallengeResult 是挑战结算。
type ChallengeResult struct {
	Won              bool             `json:"won"`
	Stolen           map[string]int64 `json:"stolen"`
	Owner            string           `json:"owner_name"`
	Power            int64            `json:"owner_power"`
	MyAttempts       int              `json:"my_attempts_today"`
	OwnerStolenToday int              `json:"owner_stolen_today"`
}

// ChallengeDefense 挑战他人防线。
//
// 防刷的三道闸：
//  1. 每日挑战次数上限（3）
//  2. 对方每日被偷次数上限（2）与 24h 护盾
//  3. 窃取比例固定 10%，且受全局掉落封顶
func (s *Service) ChallengeDefense(ctx context.Context, userID, defenseID int64, in ChallengeInput) (ChallengeResult, error) {
	// ⚠️ 第 98 轮：`challenge_date` 由 **Go** 决定，与每日任务 / 签到 / 商城同一个口径。
	//
	// 原来五处都用 `CURRENT_DATE`，由 **PG 会话时区**折算。
	// 防线挑战内部因此自洽，但它与**每日任务**的日界不同 —— 而两者
	// 语义上必须对齐：「今日通关 3 关」这个任务与「今日挑战次数上限」
	// 说的是同一个「今日」。
	//
	// 两者不同时刻翻页时（Go 本地 ≠ PG 会话时区，容器里 `time.Local` 常为 UTC）：
	// 玩家在凌晨到早上做的通关，任务在 08:00 就重置了，
	// 而挑战次数要等到北京 00:00 才重置 —— 于是「今日」有两个定义。
	//
	// 传**日期字面量**（与第 89 轮签到、第 97 轮商城同一手法）。
	today := periodStart(time.Now(), "daily").Format("2006-01-02")
	// ⚠️ 第 69 轮：上报字段边界校验放在**事务之外**。
	//
	// 理由有两条，都不是风格问题：
	//  1. 它是**请求校验**而不是业务规则 —— 一个 duration_ms 越界的请求
	//     本来就不该开事务、开行锁。
	//  2. 放在事务内的话，PG 的 22003 会在事务中间炸出来，
	//     而它是个**真 500** —— 混进服务故障告警把真故障淹掉。
	//
	// 放在外面则它是一个可分类的 ErrInvalidField -> 422，
	// 客户端能看到「上报不可信」而不是「服务内部错误」。
	if err := ValidateChallengeInput(in); err != nil {
		return ChallengeResult{}, err
	}

	var res ChallengeResult
	err := s.DB.Tx(ctx, func(tx txType) error {
		var ownerID int64
		var ownerName string
		var power int64
		var shielded *time.Time
		if err := tx.QueryRow(ctx,
			`SELECT owner_id, name, power, shielded_until FROM defenses WHERE id = $1`,
			defenseID).Scan(&ownerID, &ownerName, &power, &shielded); err != nil {
			return fmt.Errorf("%w: defense %d", ErrNotFound, defenseID)
		}
		if ownerID == userID {
			return fmt.Errorf("%w: 不能挑战自己的防线", ErrForbidden)
		}
		if shielded != nil && shielded.After(time.Now()) {
			return fmt.Errorf("%w: 对方开启了护盾", ErrForbidden)
		}

		attempts, _, err := s.dailyChallengeCountersTx(ctx, tx, userID)
		if err != nil {
			return err
		}
		if attempts >= DefenseAttemptLimit {
			return fmt.Errorf("%w: 今日挑战次数已用尽（%d/%d）", ErrForbidden, attempts, DefenseAttemptLimit)
		}

		// 判定能否实际窃取
		_, ownerStolen, err := s.dailyChallengeCountersTx(ctx, tx, ownerID)
		if err != nil {
			return err
		}

		// ⚠️ 第 79 轮：**按 user_id 升序预锁**两个钱包行。
		//
		// # 缺陷：两个对向挑战必然死锁
		//
		// 下面两处会锁 `user_wallets` 的行：
		//
		//   608 行  grantWallet(ownerID, -take)   ← 先锁【对方】
		//   632 行  grantWallet(userID,  stolen)  ← 后锁【自己】
		//
		// 于是锁顺序是「对方 → 自己」。而两个玩家同时互打时：
		//
		//   Tx1（A 打 B 的防线）  锁 wallets[B] → 等 wallets[A]
		//   Tx2（B 打 A 的防线）  锁 wallets[A] → 等 wallets[B]
		//
		// 互等 → PG 报 `40P01 deadlock_detected` → failErr → **500**。
		//
		// 这是**自伤型 500**：两个玩家各点了一次挑战，服务端回 500，
		// 而任何人看日志都得不出「是自己这边的问题」。
		//
		// 频率不高（要求两人同一瞬间互打），但它是**确定存在**的：
		// 只要两人互相点，就可能发生。
		//
		// # 修法：升序一次锁齐
		//
		// `ORDER BY user_id FOR UPDATE` 让 PG **按主键顺序**加锁，
		// 于是任意两个挑战事务的加锁序列都一致 → 不可能互等。
		//
		// ⚠️ 为什么不能用「先锁小的那个」这种应用层判断：
		// 那需要把比较结果带进后续所有分支，而 `grantWallet` 内部
		// 还会再 UPDATE 同一行。**一处有序预锁**比**处处记得有序**可靠。
		//
		// 只锁两个：`userID`（挑战者）与 `ownerID`（被挑战者）。
		// 钱包是本事务唯一会跨用户触碰的资源
		// （`user_daily_challenges` / `defenses` 的 UPDATE 不跨用户）。
		if err := lockChallengeStateOrdered(ctx, tx, []int64{userID, ownerID}); err != nil {
			return err
		}
		stolen := map[string]int64{}
		if in.Won && ownerStolen < DefenseStolenLimit {
			// 窃取 10% 战力等价资源
			loot := domain.ComputeLoot(GeneratedPowerLevel(power), 2, 10, 10, 0)
			for k, v := range loot {
				if v <= 0 {
					continue
				}
				take := v / 10
				if take > 0 {
					stolen[k] = take
					// 从对方扣除，扣不动就跳过（不强制负值）
					//
					// ⚠️ 第 81 轮：**只吞「余额不足」**，其余错误必须上抛。
					//
					// 原来是无差别 `delete(stolen, k)` —— 而
					// `grantWallet` 内部是这样的：
					//
					//	UPDATE user_wallets SET coin = coin + $2 … RETURNING coin   ← 已生效
					//	INSERT INTO wallet_flows …                                    ← 这里可能失败
					//
					// 所以它在**两个不同位置**返回错误：
					//
					//  (a) `pgx.ErrNoRows` → 余额不足     → 应当跳过（设计意图）
					//  (b) UPDATE 成功、`insert flow` 失败 → **钱已经扣了**
					//
					// (b) 被当成 (a) 吞掉之后：事务照常提交 →
					// **对方少了钱、流水没记录、挑战者什么也没拿到**。
					// 三方全不一致，且没有任何错误日志。
					//
					// 更糟的一层：数据库连接断了、网络抖了、约束被别的改动破坏了 ——
					// 全部被静默解释成「他没钱」，而真故障被彻底掩盖。
					//
					// 修法：`ErrBadInput`（余额不足 / 未知货币）是**业务拒绝**，
					// 可以跳过；其余是**基础设施失败**，上抛让事务回滚。
					if err := s.grantWallet(ctx, tx, ownerID, map[string]int64{k: -take},
						"defense_stolen", defenseID); err != nil {
						if errors.Is(err, ErrBadInput) {
							delete(stolen, k)
							continue
						}
						return fmt.Errorf("steal %s from %d: %w", k, ownerID, err)
					}
				}
			}
			if _, err := tx.Exec(ctx,
				`UPDATE defenses SET wins = wins + 1 WHERE id = $1`, defenseID); err != nil {
				return err
			}
			// 记录被偷
			if _, err := tx.Exec(ctx, `
				UPDATE user_daily_challenges SET stolen_times = stolen_times + 1
				 WHERE user_id = $1 AND challenge_date = $2`, ownerID, today); err != nil {
				return err
			}
		} else if !in.Won {
			if _, err := tx.Exec(ctx,
				`UPDATE defenses SET losses = losses + 1 WHERE id = $1`, defenseID); err != nil {
				return err
			}
		}

		if len(stolen) > 0 {
			if err := s.grantWallet(ctx, tx, userID, stolen, "defense_reward", defenseID); err != nil {
				return err
			}
		}
		if _, err := tx.Exec(ctx, `
			INSERT INTO defense_challenges (defense_id, challenger_id, seed, won, duration_ms,
			                                hp_left_pct, replay_hash, settled)
			VALUES ($1,$2,$3,$4,$5,$6,$7,TRUE)`,
			defenseID, userID, in.Seed, in.Won, in.DurationMs, in.HPLeftPct, in.ReplayHash); err != nil {
			return fmt.Errorf("record challenge: %w", err)
		}
		if _, err := tx.Exec(ctx, `
			UPDATE user_daily_challenges SET attempts = attempts + 1
			 WHERE user_id = $1 AND challenge_date = $2`, userID, today); err != nil {
			return err
		}

		res = ChallengeResult{
			Won: in.Won, Stolen: stolen, Owner: ownerName, Power: power,
			MyAttempts: attempts + 1, OwnerStolenToday: ownerStolen,
		}
		return nil
	})
	return res, err
}

func (s *Service) dailyChallengeCountersTx(ctx context.Context, tx txType, userID int64) (attempts, stolen int, err error) {
	// ⚠️ 第 98 轮：`challenge_date` 由 **Go** 决定，与每日任务 / 签到 / 商城同一个口径。
	// 详见 dailyChallengeCounters 上方的说明。
	today := periodStart(time.Now(), "daily").Format("2006-01-02")
	err = tx.QueryRow(ctx, `
		INSERT INTO user_daily_challenges (user_id, challenge_date, attempts, stolen_times)
		VALUES ($1, $2, 0, 0)
		ON CONFLICT (user_id, challenge_date) DO UPDATE SET challenge_date = $2
		RETURNING attempts, stolen_times`, userID, today).Scan(&attempts, &stolen)
	return attempts, stolen, err
}

// GeneratedPowerLevel 由战力反推一个"等价关卡"用于计算窃取掉落。
// 战力与关卡的映射是线性的：power 0 ≈ 第 1 关，power 20000 ≈ 第 100 关。
func GeneratedPowerLevel(power int64) domain.GeneratedLevel {
	id := int(power / 200)
	if id < 1 {
		id = 1
	}
	if id > domain.TotalLevels {
		id = domain.TotalLevels
	}
	return domain.GenerateLevel(id)
}
