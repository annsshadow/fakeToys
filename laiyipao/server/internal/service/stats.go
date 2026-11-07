// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"encoding/csv"
	"encoding/json"
	"errors"
	"fmt"
	"sort"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/domain"
)

// --- 看板 ---

// Dashboard 是后台数据看板。
type Dashboard struct {
	Users struct {
		Total    int64 `json:"total"`
		Guests   int64 `json:"guests"`
		Wechat   int64 `json:"wechat"`
		Banned   int64 `json:"banned"`
		NewToday int64 `json:"new_today"`
	} `json:"users"`
	Battles struct {
		Total         int64   `json:"total"`
		Today         int64   `json:"today"`
		Wins          int64   `json:"wins"`
		AvgDurationMs float64 `json:"avg_duration_ms"`
	} `json:"battles"`
	// 2026-10-31（实质优化第 39 轮）：防线总数 / 今日挑战次数。
	Defenses struct {
		Total           int64 `json:"total"`
		ChallengesToday int64 `json:"challenges_today"`
	} `json:"defenses"`
	Progression struct {
		AvgMaxStage   float64 `json:"avg_max_stage"`
		AvgPower      float64 `json:"avg_power"`
		Stage100Clear int64   `json:"stage_100_clears"`
	} `json:"progression"`
	Economy struct {
		CoinIn        int64 `json:"coin_in"`
		CoinOut       int64 `json:"coin_out"`
		GemIn         int64 `json:"gem_in"`
		GemOut        int64 `json:"gem_out"`
		OrdersPaid    int64 `json:"orders_paid"`
		OrdersPending int64 `json:"orders_pending"`
		// 2026-10-31（实质优化第 28 轮）：今日签到数 / 今日兑换次数。
		SignInsToday int64 `json:"sign_ins_today"`
		RedeemsToday int64 `json:"redeems_today"`
	} `json:"economy"`
	// I-6：验真健康度
	Verification struct {
		Checked    int64 `json:"checked"`
		Matched    int64 `json:"matched"`
		Mismatched int64 `json:"mismatched"`
	} `json:"verification"`
	// I-1：反应使用分布。分布长期集中在一两种反应 = 元素设计没生效
	ReactionUsage []ReactionCount `json:"reaction_usage"`
	DailyActive   []DauPoint      `json:"daily_active"`
	StageFunnel   []StageFunnel   `json:"stage_funnel"`
}

// ReactionCount 是反应使用计数。
type ReactionCount struct {
	Reaction string `json:"reaction"`
	Count    int64  `json:"count"`
}

// DauPoint 是某日活跃数。
type DauPoint struct {
	Date string `json:"date"`
	DAU  int64  `json:"dau"`
}

// StageFunnel 是关卡漏斗。
type StageFunnel struct {
	LevelID  int   `json:"level_id"`
	Attempts int64 `json:"attempts"`
	Number   int64 `json:"clears"`
}

// AdminDashboard 汇总看板数据。
//
// # 缓存（2026-10-23）
//
// 这个函数串了 9 段查询，其中 5 段是全表 `COUNT(*)`/`AVG`，DAU 是
// 14 个日桶的范围扫描，漏斗是全表 `GROUP BY level_id`。运营看板
// 对 30 秒的陈旧不敏感（看的是趋势与异常），所以整包缓存。
//
// ⚠️ **它确实会返回旧数据**，这是有意的取舍，不是缺陷：
// 刚打完一局的运营在看板上 30 秒内看不到这一局。要立刻看到，
// 把 `DASHBOARD_CACHE_TTL` 设成 0 重启即可（不需要发版）。
//
// 错误**不缓存**：一次失败如果被缓存 30 秒，运营会看到连续两次同样的
// 报错，而第二次其实数据库已经好了。
func (s *Service) AdminDashboard(ctx context.Context) (Dashboard, error) {
	// singleflight：TTL 到期那一刻的并发未命中只跑一次查询，不惊群。
	//
	// R66 审计：返回值里的三段是**值类型切片**（无 map），与缓存共享底层数组；
	// handler 只做 JSON 编码（只读）。不每请求深拷贝：看板 p95 3.28ms 里
	// 复制 100+ 条结构没有收益，反而把只读路径变成写分配路径。
	return s.dashboardCache.getOrLoad(ctx, time.Now(), s.Cfg().DashboardCacheTTL, func(ctx context.Context) (Dashboard, error) {
		return s.adminDashboardUncached(ctx)
	})
}

// adminDashboardUncached 是真正的查询。缓存逻辑只在上面一层，
// 这样「查什么」与「缓存多久」不会互相纠缠。
func (s *Service) adminDashboardUncached(ctx context.Context) (Dashboard, error) {
	var d Dashboard

	// 第 14 轮：各段互相独立，从**全串行**改并行，缓存未命中刷新的延迟
	// 从 Σ(各段) 降为 max(单段)；连接池不足时由 pgxpool 自然排队。
	//
	// 第 62 轮：原有 13 段，其中 7 段毫秒级小表统计合并成 1 段、
	// battles 与漏斗两段合并成 1 段（见下方 `dashboard counters` 与
	// `dashboard battles+funnel`），降到 6 段 —— 6 ≤ `DB_MAX_CONNS`
	// 默认 10，未命中刷新不再出现「后 3 段排队等槽位」。
	// 依据是压测而不是直觉：把 battle_records 的几段也合并成
	// `GROUPING SETS`（单条语句）实测反而 281ms，因为它退化成单进程内
	// 一次聚合，丢掉了 PG 的并行 worker；所以**只合并必要的段**，大扫描
	// 保持各占一条连接并行跑。
	//
	// 各 goroutine 只写自己的字段（第 62 轮起 `Defenses.Total` 只有一个
	// 写入点），主协程 wg.Wait() 后读；错误按**声明顺序**检查 —— 第一个
	// 失败段先报，与串行版的 fail-loud 顺序一致。
	scan := func(q string, dest ...any) error {
		return s.pool.QueryRow(ctx, q).Scan(dest...)
	}

	var steps []func() error
	appendStep := func(errPrefix string, fn func() error) {
		steps = append(steps, func() error {
			if err := fn(); err != nil {
				return fmt.Errorf("%s: %w", errPrefix, err)
			}
			return nil
		})
	}

	appendStep("dashboard users", func() error {
		return scan(`
		SELECT COUNT(*),
		       COUNT(*) FILTER (WHERE is_guest),
		       COUNT(*) FILTER (WHERE NOT is_guest),
		       COUNT(*) FILTER (WHERE status = 2),
		       COUNT(*) FILTER (WHERE created_at >= ((now() AT TIME ZONE 'Asia/Shanghai')::date AT TIME ZONE 'Asia/Shanghai'))
		FROM users`,
			&d.Users.Total, &d.Users.Guests, &d.Users.Wechat, &d.Users.Banned, &d.Users.NewToday)
	})

	// 第 62 轮：battles 四指标与关卡漏斗合成 1 次扫描（两段原本各扫一遍
	// battle_records，1M 行实测 ~78ms / ~72ms，每遍读 ~160MB 堆页）。
	// 当时拒绝过 `GROUPING SETS ((level_id), ())`：实测 281ms —— 它退化成
	// MixedAggregate、不并行，单进程扫完 1M 行，比分别跑两段还慢。
	//
	// 第 63 轮：这条扫描本身成了最后一项全表扫描 —— 并发刷新时实测
	// 131–147ms，是 p95 的天花板（跳过 reactions 段后 p95 仍 126ms）。
	// 战报总量/胜场/时长/漏斗都是**纯增量**口径，改为读
	// `dashboard_battle_stats`（写入时由触发器累加，迁移 00025），
	// 读侧只剩对 100 行的聚合，实测 ~1ms。
	//
	// 为什么不给增量表加「日」维度：只有 `Battles.Today` 需要今天，
	// 而今天那一小段走现有的 `idx_battle_records_created` 就是 index-only
	// scan（实测 30 万行 15ms；1M 铺 90 天时约 1.1 万行 → 数毫秒）。
	// 加日维度要处理跨日滚动与旧日行清理，只为省这条几毫秒的索引扫描。
	//
	// `AVG(duration_ms)` 口径逐位不变：老实现是 SUM/COUNT 的 numeric 除法，
	// 这里同样是 numeric 除法（`SUM(duration_sum)/NULLIF(SUM(attempts),0)`）。
	//
	// 漏斗只列「有战报」的关卡：战报被删光后增量表会留下 attempts=0 的行，
	// 所以读侧统一 `WHERE attempts > 0`，与旧 `GROUP BY battle_records`
	// 的语义一致（逐行比对在 TestDashboardMergedMatchesLegacy）。
	appendStep("dashboard battles+funnel", func() error {
		var (
			levels   []int32
			attempts []int64
			wins     []int64
		)
		// R149：计数器按 battle_records.id % 16 分片后，同一关有多行。
		// 先在 CTE 里按 level_id 合并，再对合并结果做全局 SUM / array_agg；
		// `HAVING SUM(attempts) > 0` 保持旧语义（战报删光后该关不出现在漏斗里）。
		if err := scan(`
		WITH per_level AS (
			SELECT level_id,
			       SUM(attempts)     AS attempts,
			       SUM(wins)         AS wins,
			       SUM(duration_sum) AS duration_sum
			  FROM dashboard_battle_stats
			 GROUP BY level_id
			HAVING SUM(attempts) > 0
		)
		SELECT COALESCE(SUM(attempts), 0),
		       COALESCE(SUM(wins), 0),
		       COALESCE(SUM(duration_sum)::numeric / NULLIF(SUM(attempts), 0), 0),
		       COALESCE(array_agg(level_id ORDER BY level_id), '{}'::int[]),
		       COALESCE(array_agg(attempts ORDER BY level_id), '{}'::bigint[]),
		       COALESCE(array_agg(wins ORDER BY level_id), '{}'::bigint[])
		FROM per_level`,
			&d.Battles.Total, &d.Battles.Wins, &d.Battles.AvgDurationMs,
			&levels, &attempts, &wins); err != nil {
			return err
		}
		for i := range levels {
			d.StageFunnel = append(d.StageFunnel, StageFunnel{
				LevelID: int(levels[i]), Attempts: attempts[i], Number: wins[i],
			})
		}
		// 今日战报：只扫「今天」这一段，走 created_at 索引，与增量表无关。
		return scan(`SELECT COUNT(*) FROM battle_records
			WHERE created_at >= ((now() AT TIME ZONE 'Asia/Shanghai')::date AT TIME ZONE 'Asia/Shanghai')`,
			&d.Battles.Today)
	})

	// 2026-11-03（第 62 轮）：7 段毫秒级小表统计合并成 1 条语句。
	//
	// 原实现给 defenses / defense_challenges / orders / user_sign_ins /
	// redeem_usages / replay_verifications / wallet_flows 各起一条查询，
	// 13 段并行需要 13 条连接，而池默认上限只有 10 —— 多出来的 3 段
	// 必须等别人释放。这 7 段各自都在 1ms 以内，合并后的总耗时仍是
	// 毫秒级，但连接占用从 7 降到 1，把槽位留给真正慢的大表扫描。
	//
	// 顺带修掉一个并发缺陷：旧的 `dashboard challenges` 段把
	// `COUNT(*) FROM defense_challenges` 写进 `d.Defenses.Total`，而
	// `dashboard defenses` 段又把 `COUNT(*) FROM defenses` 写进同一个
	// 字段 —— 两段并行执行，最终显示哪个值由调度决定。数据量不同时
	// （防线 3 条、累计挑战 5000 次），看板上的「防线数」会在 3 与 5000
	// 之间跳。合并后 `Total` 只有 `defenses` 一个来源，与前端
	// 「防线 N · 今日挑战 M」的口径一致。
	appendStep("dashboard counters", func() error {
		return scan(`
		SELECT c.today_challenges,
		       d.total_defenses,
		       o.paid_orders, o.pending_orders,
		       s.sign_ins_today,
		       r.redeems_today,
		       v.checked, v.matched, v.mismatched,
		       e.coin_in, e.coin_out, e.gem_in, e.gem_out
		FROM
		  (SELECT COUNT(*) FILTER (WHERE created_at >= ((now() AT TIME ZONE 'Asia/Shanghai')::date AT TIME ZONE 'Asia/Shanghai')) AS today_challenges
		     FROM defense_challenges) c,
		  (SELECT COUNT(*) AS total_defenses FROM defenses) d,
		  (SELECT COUNT(*) FILTER (WHERE status IN ('paid','delivered')) AS paid_orders,
		          COUNT(*) FILTER (WHERE status = 'pending') AS pending_orders
		     FROM orders) o,
		  (SELECT COUNT(*) AS sign_ins_today
		     FROM user_sign_ins WHERE sign_date = (now() AT TIME ZONE 'Asia/Shanghai')::date) s,
		  (SELECT COUNT(*) AS redeems_today
		     FROM redeem_usages WHERE created_at >= ((now() AT TIME ZONE 'Asia/Shanghai')::date AT TIME ZONE 'Asia/Shanghai')) r,
		  (SELECT COUNT(*) AS checked,
		          COUNT(*) FILTER (WHERE matched) AS matched,
		          COUNT(*) FILTER (WHERE NOT matched) AS mismatched
		     FROM replay_verifications) v,
		  (SELECT COALESCE(SUM(delta) FILTER (WHERE currency='coin' AND delta > 0), 0) AS coin_in,
		          COALESCE(-SUM(delta) FILTER (WHERE currency='coin' AND delta < 0), 0) AS coin_out,
		          COALESCE(SUM(delta) FILTER (WHERE currency='gem' AND delta > 0), 0) AS gem_in,
		          COALESCE(-SUM(delta) FILTER (WHERE currency='gem' AND delta < 0), 0) AS gem_out
		     FROM wallet_flows) e`,
			&d.Defenses.ChallengesToday, &d.Defenses.Total,
			&d.Economy.OrdersPaid, &d.Economy.OrdersPending,
			&d.Economy.SignInsToday,
			&d.Economy.RedeemsToday,
			&d.Verification.Checked, &d.Verification.Matched, &d.Verification.Mismatched,
			&d.Economy.CoinIn, &d.Economy.CoinOut, &d.Economy.GemIn, &d.Economy.GemOut)
	})

	appendStep("dashboard progression", func() error {
		return scan(`
		SELECT COALESCE(AVG(max_stage), 0), COALESCE(AVG(level_exp), 0),
		       (SELECT COUNT(*) FROM level_stars WHERE level_id = 100 AND clears > 0)
		FROM user_progress`,
			&d.Progression.AvgMaxStage, &d.Progression.AvgPower, &d.Progression.Stage100Clear)
	})

	// 反应使用分布
	//
	// 第 63 轮：本段从「每次刷新扫一遍 battle_records 拆 jsonb 求和」
	// 改为读 `dashboard_reaction_counts` 小表（写入时由触发器累加，
	// 见迁移 00025）。原因：`reactions_used` 是 jsonb，没有索引可用 ——
	// 1M 战报下单条 SQL 实测 110ms（并行顺序扫描 100+MB 堆页），
	// 对整次刷新 p95 的边际贡献 ≈46ms（跳过该段实测 173.6 → 126.0ms）。
	// 「按 key 求和」是纯增量口径，写在事务内累加比读路径反复全表扫描便宜得多。
	//
	// 口径与旧写法逐项一致：只统计 jsonb object 的键、逐键求和；
	// `jsonb null` 由触发器跳过（对 null 调 `jsonb_each_text` 会报错，
	// 这也是第 10 轮迁移把历史 null 归一化的原因之一）。
	// 一致性与删除回退由 `TestDashboardReactionCountsMatchAggregate` 钉死。
	appendStep("dashboard reactions", func() error {
		rows, err := s.pool.Query(ctx, `
			SELECT reaction, SUM(count) AS total
			  FROM dashboard_reaction_counts
			 GROUP BY reaction
			 ORDER BY total DESC
			 LIMIT 100`)
		if err != nil {
			return err
		}
		defer rows.Close()
		// SQL 已经按 key 聚合好了，这里直接读两列。
		//
		// ⚠️ 旧实现在 Go 里 `json.Unmarshal` 整个 map 再逐键累加，
		// 而错误被 `_ =` **静默吞掉** —— 解析失败时那一行就凭空消失，
		// 没有任何日志。一个统计数字悄悄少掉一部分，比直接报错更难发现。
		agg := map[string]int64{}
		for rows.Next() {
			var key string
			var total int64
			if err := rows.Scan(&key, &total); err != nil {
				return err
			}
			agg[key] = total
		}
		if err := rows.Err(); err != nil {
			return err
		}
		for _, spec := range domain.AllReactionSpecs() {
			d.ReactionUsage = append(d.ReactionUsage, ReactionCount{
				Reaction: string(spec.Key), Count: agg[string(spec.Key)],
			})
		}
		return nil
	})

	// 近 14 日 DAU
	//
	// 第 62 轮改写：原写法是 `generate_series LEFT JOIN battle_records`
	// 再 `GROUP BY d` —— 实测 1M 战报下 172ms，因为 JOIN 之后要在
	// 14 个日桶上做 `COUNT(DISTINCT user_id)` 的去重聚合。
	// 改成「每天一个走 created_at 索引的标量子查询」后实测 73ms，
	// 输出形状（恒 14 行、含零活跃日）与语义完全不变。
	//
	// 第 63 轮：这条语句显式关掉 JIT。14 个标量子查询让优化器的估算代价
	// 达到 ~54 万（`generate_series` 被估成 1000 行 × 14 个子计划），
	// 超过 `jit_above_cost`（默认 10 万），于是每次刷新都先编译一遍
	// 14 个函数的 JIT 代码 —— 实测 1M 战报下 JIT 自身 92.6ms，
	// 占该段 137.9ms 的 2/3（关掉后 33.9ms，且每次调用的
	// 分析/优化/发射开销约 90ms 一次，单次执行的读路径摊不薄）。
	// 只对这一条语句 `SET LOCAL`，不影响连接池里其它查询的 JIT 决策。
	appendStep("dashboard dau", func() error {
		tx, err := s.pool.Begin(ctx)
		if err != nil {
			return err
		}
		// 只读事务，回滚即释放；错误忽略是刻意的（提交与否都不改变结果）。
		defer func() { _ = tx.Rollback(ctx) }()
		if _, err := tx.Exec(ctx, "SET LOCAL jit = off"); err != nil {
			return err
		}
		drows, err := tx.Query(ctx, `
			SELECT d::date AS day,
			       (SELECT COUNT(DISTINCT br.user_id)
			          FROM battle_records br
			         WHERE br.created_at >= (d AT TIME ZONE 'Asia/Shanghai')
			           AND br.created_at < ((d + INTERVAL '1 day') AT TIME ZONE 'Asia/Shanghai')) AS dau
			FROM generate_series(
			       (now() AT TIME ZONE 'Asia/Shanghai')::date - INTERVAL '13 days',
			       (now() AT TIME ZONE 'Asia/Shanghai')::date,
			       '1 day') d
			ORDER BY 1`)
		if err != nil {
			return err
		}
		defer drows.Close()
		for drows.Next() {
			var day time.Time
			var dau int64
			if err := drows.Scan(&day, &dau); err != nil {
				return err
			}
			d.DailyActive = append(d.DailyActive, DauPoint{
				Date: day.Format("2006-01-02"), DAU: dau,
			})
		}
		return drows.Err()
	})

	// 关卡漏斗已在 `dashboard battles+funnel` 段内与 battles 指标同一次扫描产出。

	errs := make([]error, len(steps))
	var wg sync.WaitGroup
	wg.Add(len(steps))
	for i, step := range steps {
		go func(i int, step func() error) {
			defer wg.Done()
			errs[i] = step()
		}(i, step)
	}
	wg.Wait()
	for _, err := range errs {
		if err != nil {
			return d, err
		}
	}
	return d, nil
}

// --- 玩家管理 ---

// AdminUserView 是后台玩家列表条目。
type AdminUserView struct {
	ID          int64     `json:"id"`
	Nickname    string    `json:"nickname"`
	IsGuest     bool      `json:"is_guest"`
	Status      int       `json:"status"`
	MaxStage    int       `json:"max_stage"`
	Power       int64     `json:"power"`
	Coin        int64     `json:"coin"`
	Gem         int64     `json:"gem"`
	CreatedAt   time.Time `json:"created_at"`
	LastLoginAt time.Time `json:"last_login_at"`

	// 验真统计：把已有但**不可执行**的信号接到执法链上。
	//
	// 背景（实测查证）：`replay_verifications` 表全项目**只被一个查询读过** ——
	// `stats.go` 里的那个 COUNT。也就是说一次不匹配会被**记录、被统计**，
	// 但没有任何后果：不撤销奖励、不标记用户，运营也查不出是哪几场。
	// 看板上的「验真一致率」因此是个**不可执行的数字**：
	// 运营看到「10/1000 不匹配」，却不知道该封谁。
	//
	// ⚠️ **两个字段而不是一个**，因为这两件事必须可区分：
	//
	//   VerifyChecked = 0, VerifyMismatched = 0  →  从没被验真过 = **未知**
	//   VerifyChecked = 8, VerifyMismatched = 0  →  验过且都一致 = 干净
	//   VerifyChecked = 8, VerifyMismatched = 3  →  **可疑**
	//
	// 合成一个 `mismatches` 字段的话，「没验过」和「验过且一致」都会显示 0，
	// 于是**最不可信的那批用户看起来最干净** ——
	// 那正是本项目反复修的「静默降级而不是失败」。
	VerifyChecked    int64 `json:"verify_checked"`
	VerifyMismatched int64 `json:"verify_mismatched"`
}

// AdminUsersPage 是后台用户列表的一页。
//
// HasMore 由「查询是否真的多返回了一行」决定（LIMIT 取 limit+1 再裁掉末行），
// 不是拿**请求参数里的** limit 猜：服务端会把 limit 夹到上界（缺省 50、上限 200），
// 请求 `limit=201/500` 时旧实现拿 len(items)==201 判断，永远为假 ——
// `next_cursor` 返回空，尾部用户永远翻不到（R131 审计 G1）。
type AdminUsersPage struct {
	Items   []AdminUserView
	Total   int64
	HasMore bool
}

// AdminListUsers 分页查询玩家。
// cursor（R77）：keyset 分页游标 = 上一页最后一行的 id（列表按 id DESC，
// BIGSERIAL 单调递增，与 created_at 同序，单字段即严格游标，无需双字段）。
// cursor > 0 时生效并忽略 offset：深页从「排 50 万行再丢」变成索引范围扫描，
// 实测 offset=500k 的 525ms → cursor 模式 ~1ms（见台账 R76/R77 与 §4.1）。
// 兼容策略：offset 参数保留，老前端/老调用继续可用（浅页快、深页慢，
// 慢是 OFFSET 的固有成本，不是 bug）；导出/脚本类消费方应改用 cursor。
// ⚠️ cursor 用 variadic（与 `audit ...AdminAuditCtx` 同模式）：老调用点
// 不传即是 0（不走 keyset），44 处既有测试/调用零改动。
func (s *Service) AdminListUsers(ctx context.Context, keyword string, limit, offset int, dateFrom, dateTo *time.Time, isGuest *bool, lastLoginFrom, lastLoginTo *time.Time, suspicious *bool, cursorArg ...int64) ([]AdminUserView, int64, error) {
	p, err := s.AdminListUsersPage(ctx, keyword, limit, offset, dateFrom, dateTo, isGuest, lastLoginFrom, lastLoginTo, suspicious, cursorArg...)
	if err != nil {
		return nil, 0, err
	}
	return p.Items, p.Total, nil
}

// AdminListUsersPage 与 AdminListUsers 同一查询，额外返回 HasMore。
func (s *Service) AdminListUsersPage(ctx context.Context, keyword string, limit, offset int, dateFrom, dateTo *time.Time, isGuest *bool, lastLoginFrom, lastLoginTo *time.Time, suspicious *bool, cursorArg ...int64) (AdminUsersPage, error) {
	limit = clampLimit(limit, 50, 200)
	if offset < 0 {
		offset = 0
	}
	// 多取一行：实际行数 > limit 才是「确实还有下一页」，与请求 limit 是否被夹无关。
	fetchLimit := limit + 1
	var cursor int64
	if len(cursorArg) > 0 {
		cursor = cursorArg[0]
	}
	if cursor > 0 {
		offset = 0 // keyset 模式下 offset 无意义（游标已是位置）
	}
	// 第 118 轮：keyword 里的 `%`/`_`/`\` 必须转义成**字面量**。
	// 旧实现直接拼 `%<kw>%`，搜 "100%" 会退化成 `%100%%`（% 当通配符），
	// 结果「输入了东西却匹配到一大片」—— 与第 111 轮关卡搜索同族。
	// 空 keyword 的 pattern 仍是 `%%`，`$1='%%'` 哨兵语义不变。
	//
	// 第 154 轮：去空白与长度上限统一走 `trimAdminKeyword`，
	// 与 `AdminListLevels` 同一个 choke point（理由见那个函数的说明）。
	kw, err := trimAdminKeyword(keyword)
	if err != nil {
		return AdminUsersPage{}, err
	}
	escaped := strings.NewReplacer(`\`, `\\`, `%`, `\%`, `_`, `\_`).Replace(kw)
	pattern := "%" + escaped + "%"

	// ⚠️ 搜索**只打 nickname，不再打 guest_token**（2026-10-14 安全审计轮）。
	//
	// `guest_token` 是与口令同级的凭证 —— 实测：给出同一个 `guest_token`
	// 调 `POST /auth/guest` 就能拿到该账号的 access + refresh token。
	// 让它在后台可搜索，意味着：
	//
	//  1. **凭证进了运营的检索面。** 后台账号被盗用（令牌被读走，
	//     或后台被点击劫持诱导操作 —— 见 httpapi.securityHeaders）
	//     时，攻击者立刻获得「批量列出所有玩家凭证」的能力 ——
	//     而运营自己根本不需要它。
	//  2. **凭证进了审计轨迹与截图。** 运营查一个疑似作弊玩家时
	//     很容易把这一列连同搜索词复制到工单/群里。
	//
	// 而它对运营的价值接近于零：真要处理某个玩家，用 nickname 或
	// user_id 就够了 —— 后台列表两列都有。
	//
	// ⚠️ 这**不是**一个「顺便也加上」的改法：它让一条既有功能消失，
	// 所以必须同时说清楚「谁因此失去了什么」——见下面 `TestAdminSearchNeverMatchesGuestToken`
	// 的断言，它守的正是「这一列不再可检索」这条新不变式。
	// 日期范围（2026-10-31 新一轮第 15 轮）：可选，nil 不过滤。
	// 列表查询的 $5/$6 是日期（limit/offset 占 $3/$4）；
	// COUNT 查询没有 limit/offset，日期改占 $3/$4。
	// 日期范围（2026-10-31 新一轮第 15 轮）与游客/账号筛选
	// （2026-11-01 新一轮第 37 轮）、最后登录时间（2026-11-01 新一轮第 46 轮）：
	// 均为可选，nil 不过滤（NULL 时 `IS NULL` 分支恒真）。
	// 列表查询的 $3/$4 是 limit/offset，筛选从 $5 起；
	// COUNT 查询没有 limit/offset，筛选映射到 $3..$7。
	where := `($1 = '' OR u.nickname ILIKE $2 ESCAPE '\'::char OR u.id::text = $1)
	          AND ($5::timestamptz IS NULL OR u.created_at >= $5)
	          AND ($6::timestamptz IS NULL OR u.created_at < $6)
	          AND ($7::boolean IS NULL OR u.is_guest = $7)
	          AND ($8::timestamptz IS NULL OR u.last_login_at >= $8)
	          AND ($9::timestamptz IS NULL OR u.last_login_at < $9)`
	countArgs := []any{kw, pattern, dateFrom, dateTo, isGuest, lastLoginFrom, lastLoginTo}
	listArgs := []any{kw, pattern, fetchLimit, offset, dateFrom, dateTo, isGuest, lastLoginFrom, lastLoginTo}
	// 需核查筛选（2026-11-01 新一轮第 60 轮）：仅当开启时拼入条件并引用 p。
	// 不开启时 COUNT 保持不 JOIN user_progress（历史行为，中段失败测试依赖它）。
	if suspicious != nil {
		where += ` AND (COALESCE(p.level_exp, 0) > 5000 AND COALESCE(p.max_stage, 0) <= 1) = $10`
		countArgs = append(countArgs, suspicious)
		listArgs = append(listArgs, suspicious)
	}
	// 统一做 COUNT 参数编号映射（列表 $5-$10 → COUNT $3-$8）。
	countWhere := strings.NewReplacer(`$5`, `$3`, `$6`, `$4`, `$7`, `$5`, `$8`, `$6`, `$9`, `$7`, `$10`, `$8`).Replace(where)
	// R77：cursor 只进列表 WHERE（不进 COUNT —— 游标是位置不是筛选，
	// total 语义不变），占位符取当前参数表的下一位。
	cursorPH := fmt.Sprintf("$%d", len(listArgs)+1)
	listWhere := where + fmt.Sprintf(` AND (%s = 0 OR u.id < %s)`, cursorPH, cursorPH)
	listArgs = append(listArgs, cursor)

	var total int64
	// 需核查开启时 COUNT 也要 JOIN p（user_progress 每用户一行，不放大）。
	countSQL := `SELECT COUNT(*) FROM users u WHERE ` + countWhere
	if suspicious != nil {
		countSQL = `SELECT COUNT(*) FROM users u
		 LEFT JOIN user_progress p ON p.user_id = u.id
		 WHERE ` + countWhere
	}
	// R61：total 是纯重复计算（翻页不改变总数），按完整筛选条件缓存。
	// 懒加载值见 config.AdminUsersCountCacheTTL 的说明：最多旧 5 秒，
	// 到期必然重算，不会长期漂移。
	key := adminUsersCountKey(kw, dateFrom, dateTo, isGuest, lastLoginFrom, lastLoginTo, suspicious)
	total, err = s.adminUsersCountCache.getOrLoad(ctx, time.Now(), s.Cfg().AdminUsersCountCacheTTL, key,
		func(ctx context.Context) (int64, error) {
			var n int64
			if err := s.pool.QueryRow(ctx, countSQL, countArgs...).Scan(&n); err != nil {
				return 0, fmt.Errorf("count users: %w", err)
			}
			return n, nil
		})
	if err != nil {
		return AdminUsersPage{}, err
	}

	// 验真统计改为**两阶段**（第 12 轮）：先取页内用户，再按页内 id 聚合
	// 验真计数。原因：原实现的子查询对 replay_verifications⋈battle_records
	// 做**全表** GROUP BY，而 LEFT JOIN 消费它的只有本页 ≤50 个用户 ——
	// 每次翻页都为 50 人付整张验真明细表的聚合代价（该表随运营验真持续增长）。
	// 第二阶段聚合被 `br.user_id = ANY($1)` 限定到页内，代价与页大小成正比。
	//
	// 列表本身仍保留「子查询预聚合」的原始教训（一场战斗可被验真多次，
	// 直接 JOIN 明细会放大行数破坏分页粒度）—— 两阶段不改变这一点：
	// 聚合按 user_id 分组后每用户至多一行，回填按 id 精确匹配。
	rows, err := s.pool.Query(ctx, `
		SELECT u.id, u.nickname, u.is_guest, u.status,
		       COALESCE(p.max_stage, 0), COALESCE(p.level_exp, 0),
		       COALESCE(w.coin, 0), COALESCE(w.gem, 0), u.created_at, u.last_login_at
		FROM users u
		LEFT JOIN user_progress p ON p.user_id = u.id
		LEFT JOIN user_wallets w ON w.user_id = u.id
		WHERE `+listWhere+`
		ORDER BY u.id DESC LIMIT $3 OFFSET $4`, listArgs...)
	if err != nil {
		return AdminUsersPage{}, fmt.Errorf("list users: %w", err)
	}
	defer rows.Close()

	var out []AdminUserView
	for rows.Next() {
		var v AdminUserView
		if err := rows.Scan(&v.ID, &v.Nickname, &v.IsGuest, &v.Status, &v.MaxStage,
			&v.Power, &v.Coin, &v.Gem, &v.CreatedAt, &v.LastLoginAt); err != nil {
			return AdminUsersPage{}, err
		}
		out = append(out, v)
	}
	if err := rows.Err(); err != nil {
		return AdminUsersPage{}, err
	}

	// 多取的那一行只用来判「还有没有下一页」，不返回。
	hasMore := len(out) > limit
	if hasMore {
		out = out[:limit]
	}

	// 第二阶段：页内用户的验真计数（错误 fail-loud，与原单查询同口径）。
	// 关联路径 replay_verifications → battle_records 的 owner 才是被验人；
	// verifier 是验真人，绝不能按它统计（原注释的教训，保持）。
	if len(out) > 0 {
		ids := make([]int64, len(out))
		for i := range out {
			ids[i] = out[i].ID
		}
		vrows, err := s.pool.Query(ctx, `
			SELECT br.user_id,
			       COUNT(*) AS checked,
			       COUNT(*) FILTER (WHERE NOT rv.matched) AS mismatched
			FROM replay_verifications rv
			JOIN battle_records br ON br.id = rv.battle_id
			WHERE br.user_id = ANY($1)
			GROUP BY br.user_id`, ids)
		if err != nil {
			return AdminUsersPage{}, fmt.Errorf("verify stats: %w", err)
		}
		defer vrows.Close()
		type verifyStat struct{ checked, mismatched int64 }
		stats := make(map[int64]verifyStat, len(out))
		for vrows.Next() {
			var id int64
			var st verifyStat
			if err := vrows.Scan(&id, &st.checked, &st.mismatched); err != nil {
				return AdminUsersPage{}, fmt.Errorf("scan verify stats: %w", err)
			}
			stats[id] = st
		}
		if err := vrows.Err(); err != nil {
			return AdminUsersPage{}, fmt.Errorf("iterate verify stats: %w", err)
		}
		for i := range out {
			out[i].VerifyChecked = stats[out[i].ID].checked
			out[i].VerifyMismatched = stats[out[i].ID].mismatched
		}
	}
	return AdminUsersPage{Items: out, Total: total, HasMore: hasMore}, nil
}

// AdminExportUsers 导出用户列表为 CSV（2026-10-30 实质优化第 12 轮）。
//
// 后台列表（用户 / 战报 / 流水）此前**没有任何导出入口** —— 运营做
// 数据分析只能手动翻页或直接查库。这里补上用户列表导出：
//
//   - 过滤条件与 `AdminListUsers` 完全一致（keyword 走同一转义与口径），
//     只是不带 LIMIT/OFFSET —— 导出是「全量快照」，不是分页。
//   - 上限 100_000 行：CSV 全量在运营常规使用里远用不到，这个上限
//     只是防止一次导出把 API 进程内存打爆。
//   - 列头用英文，内容用 UTF-8 原文 + BOM（Excel 打开中文不乱码）。
//
// 注意：guest_token 等敏感凭证**绝不导出**（理由见 AdminListUsers 的注释）。
func (s *Service) AdminExportUsers(ctx context.Context, keyword string, dateFrom, dateTo *time.Time, isGuest *bool, lastLoginFrom, lastLoginTo *time.Time, suspicious *bool) ([]byte, error) {
	kw, err := trimAdminKeyword(keyword)
	if err != nil {
		return nil, err
	}
	escaped := strings.NewReplacer(`\`, `\\`, `%`, `\%`, `_`, `\_`).Replace(kw)
	pattern := "%" + escaped + "%"

	// 日期范围（2026-10-31 新一轮第 15 轮）、游客/账号筛选（2026-11-03 R29）
	// 与最后登录时间（2026-11-01 新一轮第 46 轮）：**始终**拼入（nil → NULL，
	// `IS NULL` 分支恒真），保证导出查询参数编号稳定。
	// 导出查询没有 limit/offset，筛选从 $3 起。
	where := `($1 = '' OR u.nickname ILIKE $2 ESCAPE '\'::char OR u.id::text = $1)
	          AND ($3::timestamptz IS NULL OR u.created_at >= $3)
	          AND ($4::timestamptz IS NULL OR u.created_at < $4)
	          AND ($7::boolean IS NULL OR u.is_guest = $7)
	          AND ($5::timestamptz IS NULL OR u.last_login_at >= $5)
	          AND ($6::timestamptz IS NULL OR u.last_login_at < $6)`
	args := []any{kw, pattern, dateFrom, dateTo, lastLoginFrom, lastLoginTo, isGuest}
	// 需核查筛选（2026-11-01 新一轮第 60 轮）：仅开启时拼入（引用 p，
	// 导出查询始终 JOIN user_progress，无解析问题）。
	if suspicious != nil {
		where += ` AND (COALESCE(p.level_exp, 0) > 5000 AND COALESCE(p.max_stage, 0) <= 1) = $8`
		args = append(args, suspicious)
	}

	rows, err := s.pool.Query(ctx, `
		SELECT u.id, u.nickname, u.is_guest, u.status,
		       COALESCE(p.max_stage, 0), COALESCE(p.level_exp, 0),
		       COALESCE(w.coin, 0), COALESCE(w.gem, 0), u.created_at, u.last_login_at,
		       COALESCE(v.checked, 0), COALESCE(v.mismatched, 0)
		FROM users u
		LEFT JOIN user_progress p ON p.user_id = u.id
		LEFT JOIN user_wallets w ON w.user_id = u.id
		LEFT JOIN (
			SELECT br.user_id,
			       COUNT(*) AS checked,
			       COUNT(*) FILTER (WHERE NOT rv.matched) AS mismatched
			FROM replay_verifications rv
			JOIN battle_records br ON br.id = rv.battle_id
			GROUP BY br.user_id
		) v ON v.user_id = u.id
		WHERE `+where+`
		ORDER BY u.id DESC LIMIT 100001`, args...)
	if err != nil {
		return nil, fmt.Errorf("export users: %w", err)
	}
	defer rows.Close()

	buf := s.newExportBuffer()
	// UTF-8 BOM：Excel 打开含中文的 CSV 时按 UTF-8 解码，否则乱码。
	buf.WriteString("\xEF\xBB\xBF")
	w := csv.NewWriter(buf)
	header := []string{
		"id", "nickname", "is_guest", "status", "max_stage", "power",
		"coin", "gem", "created_at", "last_login_at", "verify_checked", "verify_mismatched",
	}
	if err := w.Write(header); err != nil {
		return nil, fmt.Errorf("write csv header: %w", err)
	}
	n := 0
	for rows.Next() {
		n++
		var v AdminUserView
		if err := rows.Scan(&v.ID, &v.Nickname, &v.IsGuest, &v.Status, &v.MaxStage,
			&v.Power, &v.Coin, &v.Gem, &v.CreatedAt, &v.LastLoginAt,
			&v.VerifyChecked, &v.VerifyMismatched); err != nil {
			return nil, fmt.Errorf("scan export user: %w", err)
		}
		rec := []string{
			strconv.FormatInt(v.ID, 10),
			v.Nickname,
			strconv.FormatBool(v.IsGuest),
			strconv.Itoa(v.Status),
			strconv.Itoa(v.MaxStage),
			strconv.FormatInt(v.Power, 10),
			strconv.FormatInt(v.Coin, 10),
			strconv.FormatInt(v.Gem, 10),
			v.CreatedAt.Format(time.RFC3339),
			v.LastLoginAt.Format(time.RFC3339),
			strconv.FormatInt(v.VerifyChecked, 10),
			strconv.FormatInt(v.VerifyMismatched, 10),
		}
		if err := writeCSVRow(w, rec); err != nil {
			return nil, fmt.Errorf("write csv row: %w", err)
		}
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	if n > adminExportMaxRows {
		return nil, exportTooLarge(n)
	}
	w.Flush()
	if err := w.Error(); err != nil {
		return nil, fmt.Errorf("flush csv: %w", err)
	}
	return buf.Bytes(), nil
}

// AdminSetUserStatus 封禁 / 解封玩家。
// AdminSetUserStatus 封禁/解封玩家。
//
// ⚠️ 第 114 轮：状态写入与 refresh token 吊销必须在**同一事务**里。
// 修前是两条独立 autocommit：封禁成功后第二条失败时，
// users.status 已落库回不去，而 refresh token 还有效——
// 被禁账号在 RefreshTTL 内仍可换访问令牌「续命」，
// 运营止损动作实际没止住。下面的注释「封禁即吊销」
// 与旧实现（两条可分别失败）自相矛盾，这正是缺陷能活下来的原因。
func (s *Service) AdminSetUserStatus(ctx context.Context, userID int64, banned bool, reason string, audit ...AdminAuditCtx) (AdminUserView, error) {
	status := 1
	if banned {
		status = 2
	}
	err := s.DB.Tx(ctx, func(tx txType) error {
		if _, err := tx.Exec(ctx,
			`UPDATE users SET status = $2, ban_reason = $3 WHERE id = $1`, userID, status, reason); err != nil {
			return fmt.Errorf("set user status: %w", err)
		}
		// 封禁即吊销所有 refresh token，否则封禁后仍可用长期令牌续命；
		// 失败则整单回滚（status 恢复原值），不允许「半封」状态。
		if banned {
			if _, err := tx.Exec(ctx,
				`UPDATE refresh_tokens SET revoked_at = now() WHERE user_id = $1 AND revoked_at IS NULL`,
				userID); err != nil {
				return fmt.Errorf("revoke tokens: %w", err)
			}
		}
		// R36：封禁/解封与审计同一事务。封禁是止损动作，事后必须能回答
		// 「谁在什么时候以什么理由封了谁」；审计缺行等于这次止损没有证据。
		if len(audit) > 0 {
			action, detail := "unban_user", any(nil)
			if banned {
				action = "ban_user"
				detail = map[string]any{"reason": reason}
			}
			if err := s.AuditTx(ctx, tx, audit[0], action,
				strconv.FormatInt(userID, 10), detail); err != nil {
				return err
			}
		}
		return nil
	})
	if err != nil {
		return AdminUserView{}, err
	}
	var v AdminUserView
	// ⚠️ 2026-10-21（二轮审计 P1-1）：验真两列**必须**一起查。
	//
	// 此前这个回读只 SELECT 了 10 列，**没有** `v.checked / v.mismatched`，
	// 所以 `AdminUserView.VerifyChecked/VerifyMismatched` 恒为 0。
	//
	// 而后台封禁/解封后做的是 `Object.assign(row, res.user)` ——
	// 于是那两列被 0 覆盖，「不匹配 3/5」（红）当场变成「未验真」（灰）。
	// 运营在封禁后第一眼看到的结论是**错的**，而且方向是「看起来更干净」。
	//
	// 那段注释自己写过这句话：「最不可信的那批用户看起来最干净」。
	// 这里就是那个坑的入口。
	err = s.pool.QueryRow(ctx, `
		SELECT u.id, u.nickname, u.is_guest, u.status,
		       COALESCE(p.max_stage, 0), COALESCE(p.level_exp, 0),
		       COALESCE(w.coin, 0), COALESCE(w.gem, 0), u.created_at, u.last_login_at,
		       COALESCE(v.checked, 0), COALESCE(v.mismatched, 0)
		FROM users u
		LEFT JOIN user_progress p ON p.user_id = u.id
		LEFT JOIN user_wallets w ON w.user_id = u.id
		LEFT JOIN (
			SELECT br.user_id,
			       COUNT(*) AS checked,
			       COUNT(*) FILTER (WHERE NOT rv.matched) AS mismatched
			FROM replay_verifications rv
			JOIN battle_records br ON br.id = rv.battle_id
			GROUP BY br.user_id
		) v ON v.user_id = u.id
		WHERE u.id = $1`, userID).
		Scan(&v.ID, &v.Nickname, &v.IsGuest, &v.Status, &v.MaxStage,
			&v.Power, &v.Coin, &v.Gem, &v.CreatedAt, &v.LastLoginAt,
			&v.VerifyChecked, &v.VerifyMismatched)
	return v, err
}

// AdminGrantCurrency 后台发币（负数为回收）。
//
// ⚠️ 第 109 轮：**符号必须进审计**。
// 修前无论正负，钱包流水一律记 `admin_grant`、
// 后台审计一律记 `grant_currency` ——
// 运营在「发放资源」框里输入负数就是**静默回收**玩家货币，
// 事后翻审计只看到「发放」，分不清哪笔是扣款。
// 回收是真实需求（老测试写明的「回收场景」），但需求不该靠
// 「正数事件里混负数」实现 —— 符号本身必须成为审计字段。
// MaxAdminGrantAmount 是后台单次发放/回收的绝对值上限。
//
// ⚠️ 为什么需要它：金额来自 JSON 的 `int64`，没有上界。
//   - `amount = 2^62` 本身能写进 BIGINT（实测成功），
//     但**再叠一次**就溢出 → PG `22003 numeric_value_out_of_range`
//     → 不在 `isSettleRejection` 名单里 → 500「服务内部错误」。
//   - 也就是说一次手滑（大数粘贴 / 小数点位置搞错）会把整笔发放打成
//     500，而运营看到的不是「数额不合理」而是「服务端坏了」。
//
// 取 1e12：比任何真实的运营发放大 6 个数量级（百万级的活动奖励），
// 又离 int64 溢出（9.2e18）足够远 —— 连续累加到 9.2e15 次才会碰到边界。
var MaxAdminGrantAmount = int64(1_000_000_000_000)

// AdminResetPlayerPassword 后台重置玩家账号密码（2026-10-30 实质优化第 11 轮）。
//
// 玩家侧已有修改密码（`ChangePassword`）与游客转正（`UpgradeGuestAccount`），
// 但运营缺少兜底：账号密码遗忘 / 账号被盗需要强制轮换时，只能封禁或
// 改环境变量重启引导 —— 前者过重，后者只对共享测试账号有效。
//
// 语义要点：
//
//   - 只对已有用户名密码的账号开放（`password_hash` 非 NULL）。
//     游客号没有用户名，重置密码后也无法登录，直接拒绝。
//   - 新密码门槛与其他账号一致（至少 10 位）。
//   - 更新哈希与吊销全部 refresh token 在**同一事务**里：
//     中途失败不能出现「密码已改但旧会话仍有效」的中间态。
//
// 注意与共享测试账号的交互：`EnsureBootstrapPlayer` 只在**服务启动**时
// 同步密码，运行中后台重置不会立即被配置覆盖；下次重启后该账号会
// 恢复为环境变量里的口令 —— 这是「测试账号口令与超管一致」的运维
// 约定本身要求的，文档见 player_account.go 的 EnsureBootstrapPlayer。
func (s *Service) AdminResetPlayerPassword(ctx context.Context, userID int64, newPassword string, audit ...AdminAuditCtx) error {
	newPassword = strings.TrimSpace(newPassword)
	if len(newPassword) < 10 {
		return fmt.Errorf("%w: 新密码至少 10 位（当前 %d 位）", ErrBadInput, len(newPassword))
	}

	var hash *string
	err := s.pool.QueryRow(ctx,
		`SELECT password_hash FROM users WHERE id = $1`, userID).Scan(&hash)
	if err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return fmt.Errorf("%w: 用户 %d 不存在", ErrNotFound, userID)
		}
		return fmt.Errorf("load player account: %w", err)
	}
	if hash == nil || *hash == "" {
		return fmt.Errorf("%w: 该账号没有用户名密码，无法重置（游客号请先引导转正）", ErrBadInput)
	}

	newHash, err := HashPassword(newPassword)
	if err != nil {
		return err
	}
	err = s.DB.Tx(ctx, func(tx txType) error {
		if _, err := tx.Exec(ctx,
			`UPDATE users SET password_hash = $2 WHERE id = $1`, userID, newHash); err != nil {
			return fmt.Errorf("update password hash: %w", err)
		}
		if _, err := tx.Exec(ctx,
			`UPDATE refresh_tokens SET revoked_at = now()
			  WHERE user_id = $1 AND revoked_at IS NULL`, userID); err != nil {
			return fmt.Errorf("revoke refresh tokens: %w", err)
		}
		// R41：改密 + 吊销 + 审计同事务。这是能把玩家从所有设备踢下线的
		// 高危动作；审计写不进去时若照常生效，运营侧看到的是
		// 「所有人都被登出了，但没有任何操作记录」。
		if len(audit) > 0 {
			if err := s.AuditTx(ctx, tx, audit[0], "reset_password",
				strconv.FormatInt(userID, 10), map[string]any{"user_id": userID}); err != nil {
				return err
			}
		}
		return nil
	})
	if err != nil {
		return err
	}
	return nil
}

func (s *Service) AdminGrantCurrency(ctx context.Context, userID int64, currency string, amount int64, audit ...AdminAuditCtx) (map[string]int64, error) {
	if amount == 0 {
		return nil, fmt.Errorf("%w: 数量为 0 的发放没有意义", ErrBadInput)
	}
	if amount > MaxAdminGrantAmount || amount < -MaxAdminGrantAmount {
		return nil, fmt.Errorf("%w: 数量 %d 超出单次发放上限 %d（多半是手滑）",
			ErrBadInput, amount, MaxAdminGrantAmount)
	}
	reason := "admin_grant"
	if amount < 0 {
		reason = "admin_revoke"
	}
	// ⚠️ 2026-10-21（P3-2）：**先确认用户存在**。
	//
	// 此前不存在性检查完全依赖 `grantWallet` 的报错形状：
	// 钱包行读不到 → `pgx.ErrNoRows` → 被翻译成
	// 「coin 不足（需要 100）」。
	//
	// 于是「给一个不存在的用户发币」报的是「**金币不足**」——
	// 一个与真实原因**完全无关**的提示。运营会以为自己发多了、
	// 或者怀疑货币名写错了，而真相是「这个用户 id 不存在」。
	//
	// 一条与真实原因无关的错误消息，比没有消息更坏：它把人引向错误的排查方向。
	if err := s.assertUserExists(ctx, userID); err != nil {
		return nil, err
	}
	out := map[string]int64{}
	var auditCtx *AdminAuditCtx
	if len(audit) > 0 {
		auditCtx = &audit[0]
	}
	err := s.DB.Tx(ctx, func(tx txType) error {
		var txErr error
		out, txErr = s.grantCurrencyTx(ctx, tx, userID, currency, amount, reason, auditCtx)
		return txErr
	})
	return out, err
}

// grantCurrencyTx 是 AdminGrantCurrency 的事务内部分（R86 抽出：
// 幂等键版本要在「查重 → 业务 → 记录」的**同一个事务**里复用它）。
func (s *Service) grantCurrencyTx(ctx context.Context, tx pgx.Tx, userID int64,
	currency string, amount int64, reason string, auditCtx *AdminAuditCtx) (map[string]int64, error) {
	out := map[string]int64{}
	if err := s.grantWallet(ctx, tx, userID,
		map[string]int64{currency: amount}, reason, 0); err != nil {
		return nil, err
	}
	// R37：钱包变动与审计同一事务。发币/回收是价值转移，
	// 「余额变了但审计没写」等于一笔没有来源的资产流动；
	// 符号仍按 R109 的口径落进事件名，负数不是「发放」。
	if auditCtx != nil {
		action := "grant_currency"
		if amount < 0 {
			action = "revoke_currency"
		}
		if err := s.AuditTx(ctx, tx, *auditCtx, action,
			strconv.FormatInt(userID, 10),
			map[string]any{"currency": currency, "amount": amount}); err != nil {
			return nil, err
		}
	}
	// 第 21 轮：道具（revive_token / gem_wash_token / mastery_reset）的
	// 发放与回显。grantWallet 早就把 isWalletToken 转发到 grantToken
	// （第 74 轮），但响应的 out 只装四货币 —— 后台发道具时**余额
	// 不回显**，运营无法确认发放生效（只能去审计日志猜）。
	if isWalletToken(currency) {
		var balance int64
		if err := tx.QueryRow(ctx,
			`SELECT COALESCE((SELECT balance FROM user_tokens WHERE user_id = $1 AND token = $2), 0)`,
			userID, currency).Scan(&balance); err != nil {
			return nil, fmt.Errorf("read token balance: %w", err)
		}
		out[currency] = balance
		return out, nil
	}
	w, err := s.loadWalletTx(ctx, tx, userID)
	if err != nil {
		return nil, err
	}
	out["coin"], out["gem"], out["energy"], out["keys"] = w.Coin, w.Gem, w.Energy, w.Keys
	return out, nil
}

// AdminGrantCurrencyIdem 是发币/回收的幂等键版本（R86）。
// 同 key+同请求体重放返回第一次的响应，余额只动一次；语义契约见 idempotency.go。
func (s *Service) AdminGrantCurrencyIdem(ctx context.Context, userID int64,
	currency string, amount int64, key, reqHash string, audit AdminAuditCtx) (body []byte, replayed bool, err error) {
	// 校验与原方法同一顺序：参数 → 存在性 → 幂等事务。
	if amount == 0 {
		return nil, false, fmt.Errorf("%w: 数量为 0 的发放没有意义", ErrBadInput)
	}
	if amount > MaxAdminGrantAmount || amount < -MaxAdminGrantAmount {
		return nil, false, fmt.Errorf("%w: 数量 %d 超出单次发放上限 %d（多半是手滑）",
			ErrBadInput, amount, MaxAdminGrantAmount)
	}
	if err := s.assertUserExists(ctx, userID); err != nil {
		return nil, false, err
	}
	reason := "admin_grant"
	if amount < 0 {
		reason = "admin_revoke"
	}
	return s.RunIdempotent(ctx, "admin.users.grant", key, reqHash,
		func(tx pgx.Tx) (any, error) {
			wallet, err := s.grantCurrencyTx(ctx, tx, userID, currency, amount, reason, &audit)
			if err != nil {
				return nil, err
			}
			return AdminGrantResp{Wallet: wallet}, nil
		})
}

// assertUserExists 确认用户存在，不存在则 ErrNotFound。
//
// 2026-10-21（P3-2）：运营写操作的第一道门。
// 「用户不存在」与「余额不足」是两件完全不同的事，错误消息必须分开说。
func (s *Service) assertUserExists(ctx context.Context, userID int64) error {
	var exists bool
	if err := s.pool.QueryRow(ctx,
		`SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)`, userID).Scan(&exists); err != nil {
		return fmt.Errorf("check user exists: %w", err)
	}
	if !exists {
		return fmt.Errorf("%w: 用户 %d 不存在", ErrNotFound, userID)
	}
	return nil
}

func (s *Service) loadWalletTx(ctx context.Context, tx txType, userID int64) (Wallet, error) {
	var w Wallet
	err := tx.QueryRow(ctx,
		`SELECT coin, gem, energy, keys, energy_updated_at FROM user_wallets WHERE user_id = $1`, userID).
		Scan(&w.Coin, &w.Gem, &w.Energy, &w.Keys, &w.EnergyUpdatedAt)
	return w, err
}

// --- 战报 ---

// AdminBattleView 是后台战报条目。
type AdminBattleView struct {
	ID          int64     `json:"id"`
	UserID      int64     `json:"user_id"`
	Nickname    string    `json:"nickname"`
	LevelID     int       `json:"level_id"`
	Result      string    `json:"result"`
	Stars       int       `json:"stars"`
	Score       int64     `json:"score"`
	Kills       int       `json:"kills"`
	Leaked      int       `json:"leaked"`
	WaveReached int       `json:"wave_reached"`
	DurationMs  int       `json:"duration_ms"`
	Reactions   int       `json:"reactions"`
	HeatMax     int       `json:"heat_max"`
	ReplayHash  string    `json:"replay_hash"`
	CreatedAt   time.Time `json:"created_at"`

	// 第 123 轮：本关的总怪数（生成器权威值）。
	// 后台「守恒」列此前只判 kills+leaked===0（无接触），其余一律显示「守恒」
	// —— 没有任何总怪数来源，伪造战报的「一致性」列永远绿。
	// 给了 total_enemies，kills+leaked **超出**它才是可判定的伪造信号。
	TotalEnemies int `json:"total_enemies"`

	// 本场战报的验真状态。
	//
	// 上一轮把「每用户」的验真统计接进了 `/admin/users`，运营于是知道**谁**可疑；
	// 但「**哪一场**对局不匹配」仍然答不出来 —— 还得手工按 user_id 去查战报、
	// 再和 `replay_verifications` 交叉比对。这里补上最后一段。
	//
	// 同样是**两个字段**：没被验真过（0/0）不等于「验过且一致」。
	VerifyChecked    int64 `json:"verify_checked"`
	VerifyMismatched int64 `json:"verify_mismatched"`
}

// AdminListBattles 分页查询战报。
//
// onlyMismatched 为真时**只**返回「被验真过且有不匹配记录」的战报，
// 用来从「这个用户有 3 次不匹配」直接跳到「是哪 3 场」。
// cursor（R77）：与 AdminListUsers 同一 keyset 机制（id DESC，游标=上一页
// 末行 id）。深页实测 offset=500k 的 556ms → cursor ~1ms。offset 保留兼容。
// ⚠️ cursor 用 variadic（与 `audit ...AdminAuditCtx` 同模式）：不传即 0。
func (s *Service) AdminListBattles(ctx context.Context, userID int64, levelID int, limit int, onlyMismatched bool, offset int, dateFrom, dateTo *time.Time, result string, cursorArg ...int64) ([]AdminBattleView, int64, error) {
	limit = clampLimit(limit, 50, 200)
	// ⚠️ 2026-10-21（二轮审计 P2）：负 offset 必须夹到 0。
	//
	// `queryInt` **不**拒绝负数（`Atoi("-1")` 成功）→ 负 offset 原样进 SQL
	// → PostgreSQL `ERROR: OFFSET must not be negative` → **整页 500**。
	//
	// 一个运营手滑打出来的负号，不该让整页报错。
	if offset < 0 {
		offset = 0
	}
	var cursor int64
	if len(cursorArg) > 0 {
		cursor = cursorArg[0]
	}
	if cursor > 0 {
		offset = 0 // keyset 模式下游标即位置（R77）
	}

	// 验真统计走**子查询预聚合**：一场战斗可以被验真多次，
	// 直接 JOIN 会放大行数，让分页切在错误粒度上。
	//
	// 2026-10-24：按用户过滤时（userID != 0，典型如后台玩家详情），
	// 子查询从**全表** GROUP BY 收窄为「只聚合该用户的战报」——
	// 否则每次分页都对整张 replay_verifications 做聚合，
	// 验真记录增长后即全表扫描。userID = 0（运营全量视图）
	// 无法收窄，保持全表聚合，语义不变。
	// 两种形态对**结果**完全等价：外层 WHERE 已把行限制在
	// 该用户的战报上，收窄只是让聚合少扫数据。
	//
	// 第 13 轮：聚合 JOIN 只在**需要它**的查询里出现 ——
	//   1. COUNT 仅在 onlyMismatched 时带 JOIN（WHERE 引用 v.mismatched）；
	//      普通翻页的 COUNT 对聚合结果零消费，纯多余一整张表的聚合。
	//   2. 列表在 onlyMismatched=false 时改**两阶段**（与 AdminListUsers
	//      第 12 轮同款）：先取页，再按页内 battle_id = ANY 聚合回填 ——
	//      每次翻页只为 ≤50 行付聚合代价。onlyMismatched=true 时
	//      WHERE 依赖聚合列，保持单查询（低频运营筛选，不动）。
	vsub := `LEFT JOIN (
		SELECT battle_id,
		       COUNT(*) AS checked,
		       COUNT(*) FILTER (WHERE NOT matched) AS mismatched
		FROM replay_verifications GROUP BY battle_id
	) v ON v.battle_id = br.id`
	if userID != 0 {
		vsub = `LEFT JOIN (
		SELECT battle_id,
		       COUNT(*) AS checked,
		       COUNT(*) FILTER (WHERE NOT matched) AS mismatched
		FROM replay_verifications
		WHERE battle_id IN (SELECT id FROM battle_records WHERE user_id = $1)
		GROUP BY battle_id
	) v ON v.battle_id = br.id`
	}
	twoPhase := !onlyMismatched
	countVsub := vsub
	listVsub := vsub
	if twoPhase {
		countVsub = ""
		listVsub = ""
	}

	// 过滤条件写成 `COALESCE(v.mismatched, 0) > 0` 而不是 `v.mismatched > 0`。
	//
	// ⚠️ 这里我一开始写了「后者会让未验真的行被静默丢弃」—— **那句话是错的**，
	// 是我的第 16 次「前提不成立」。实测：把 COALESCE 去掉，三个测试**照样全绿**。
	//
	// 原因是 `> 0` 这种比较下两种写法**行为完全相同**：
	// LEFT JOIN 未匹配时 v.mismatched 是 NULL，`NULL > 0` 求值为 NULL（不是 true），
	// 于是那行被排除；而 COALESCE 版本得 `0 > 0` = false，也被排除。
	// 「未验真的战报不该出现在「只看不匹配」的列表里」是**预期行为**，
	// 不是 COALESCE 修掉的 bug。
	//
	// 那为什么还保留 COALESCE？**为了可读性**：它把「没有验真记录 ⇒ 计数为 0」
	// 这件事写在脸上，而不是依赖「NULL 会顺着比较运算传播」这条不直观的规则。
	// 真正会区分两者的写法是 `IS NOT TRUE` 那种（NULL 与 false 分开对待），
	// 本查询不需要。
	//
	// 保留这条注释而不是删掉：下一个人很可能也会认为这里有坑，
	// 然后为了「修」它而改动别的地方。
	where := `($1 = 0 OR br.user_id = $1) AND ($2 = 0 OR br.level_id = $2)`
	if onlyMismatched {
		where += ` AND COALESCE(v.mismatched, 0) > 0`
	}
	// 日期范围（2026-10-31 新一轮第 14 轮）：可选，nil 不过滤。
	// 列表查询的 $5/$6 是日期（limit/offset 占 $3/$4）；
	// COUNT 查询没有 limit/offset，日期改占 $3/$4。
	countArgs := []any{userID, levelID}
	listArgs := []any{userID, levelID, limit, offset}
	// 日期范围：**始终**拼入（nil → NULL，`IS NULL` 分支恒真），
	// 保证 COUNT 与列表的参数编号稳定（列表 $5/$6、COUNT $3/$4）。
	// 若只在非 nil 时拼入，result 筛选（占 $7/$5）会悬空，
	// PostgreSQL 报 `could not determine data type of parameter`。
	where += ` AND ($5::timestamptz IS NULL OR br.created_at >= $5)
	             AND ($6::timestamptz IS NULL OR br.created_at < $6)`
	countArgs = append(countArgs, dateFrom, dateTo)
	listArgs = append(listArgs, dateFrom, dateTo)
	// 结果筛选（2026-11-01 新一轮第 44 轮）：win / lose，空串不过滤。
	// 列表占 $7（COUNT 经下方映射到 $5）；COUNT 与列表参数顺序一致：
	// 日期在前、result 在后。
	result = strings.TrimSpace(result)
	if result != "" {
		where += ` AND br.result = $7`
		countArgs = append(countArgs, result)
		listArgs = append(listArgs, result)
	}
	countWhere := strings.ReplaceAll(strings.ReplaceAll(strings.ReplaceAll(where, "$5", "$3"), "$6", "$4"), "$7", "$5")
	// R77：cursor 只进列表 WHERE（不进 COUNT），占位符取参数表下一位。
	cursorPH := fmt.Sprintf("$%d", len(listArgs)+1)
	listWhere := where + fmt.Sprintf(` AND (%s = 0 OR br.id < %s)`, cursorPH, cursorPH)
	listArgs = append(listArgs, cursor)

	var total int64
	if err := s.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM battle_records br `+countVsub+` WHERE `+countWhere,
		countArgs...).Scan(&total); err != nil {
		return nil, 0, fmt.Errorf("count battles: %w", err)
	}
	// 两阶段形态下列表不选聚合列（SELECT 里引用 v 会直接 SQL 报错）。
	verifyCols := `,
		       COALESCE(v.checked, 0), COALESCE(v.mismatched, 0)`
	if twoPhase {
		verifyCols = ""
	}
	rows, err := s.pool.Query(ctx, `
		SELECT br.id, br.user_id, COALESCE(u.nickname,''), br.level_id, br.result, br.stars, br.score,
		       br.kills, br.leaked, br.wave_reached, br.duration_ms, br.reactions, br.heat_max,
		       br.replay_hash, br.created_at`+verifyCols+`
		FROM battle_records br
		LEFT JOIN users u ON u.id = br.user_id
		`+listVsub+`
		WHERE `+listWhere+`
		ORDER BY br.id DESC LIMIT $3 OFFSET $4`, listArgs...)
	if err != nil {
		return nil, 0, fmt.Errorf("list battles: %w", err)
	}
	defer rows.Close()
	var out []AdminBattleView
	// 第 123 轮：本关总怪数取自**生成器**（确定性、与 level_waves 同源）。
	// 按 level_id 缓存，避免同一关反复 GenerateLevel。
	totalCache := map[int]int{}
	totalEnemies := func(levelID int) int {
		if n, ok := totalCache[levelID]; ok {
			return n
		}
		n := 0
		for _, w := range domain.CachedLevel(levelID).Waves {
			for _, sp := range w.Spawns {
				n += sp.Count
			}
		}
		totalCache[levelID] = n
		return n
	}
	for rows.Next() {
		var v AdminBattleView
		if twoPhase {
			if err := rows.Scan(&v.ID, &v.UserID, &v.Nickname, &v.LevelID, &v.Result, &v.Stars,
				&v.Score, &v.Kills, &v.Leaked, &v.WaveReached, &v.DurationMs, &v.Reactions,
				&v.HeatMax, &v.ReplayHash, &v.CreatedAt); err != nil {
				return nil, 0, err
			}
		} else {
			if err := rows.Scan(&v.ID, &v.UserID, &v.Nickname, &v.LevelID, &v.Result, &v.Stars,
				&v.Score, &v.Kills, &v.Leaked, &v.WaveReached, &v.DurationMs, &v.Reactions,
				&v.HeatMax, &v.ReplayHash, &v.CreatedAt,
				&v.VerifyChecked, &v.VerifyMismatched); err != nil {
				return nil, 0, err
			}
		}
		v.TotalEnemies = totalEnemies(v.LevelID)
		out = append(out, v)
	}
	if err := rows.Err(); err != nil {
		return nil, 0, err
	}

	// 两阶段第二段：页内战报的验真计数（battle_id = ANY 限定，fail-loud）。
	if twoPhase && len(out) > 0 {
		ids := make([]int64, len(out))
		for i := range out {
			ids[i] = out[i].ID
		}
		vrows, err := s.pool.Query(ctx, `
			SELECT battle_id,
			       COUNT(*) AS checked,
			       COUNT(*) FILTER (WHERE NOT matched) AS mismatched
			FROM replay_verifications
			WHERE battle_id = ANY($1)
			GROUP BY battle_id`, ids)
		if err != nil {
			return nil, 0, fmt.Errorf("verify stats: %w", err)
		}
		defer vrows.Close()
		stats := make(map[int64][2]int64, len(out))
		for vrows.Next() {
			var id int64
			var st [2]int64
			if err := vrows.Scan(&id, &st[0], &st[1]); err != nil {
				return nil, 0, fmt.Errorf("scan verify stats: %w", err)
			}
			stats[id] = st
		}
		if err := vrows.Err(); err != nil {
			return nil, 0, fmt.Errorf("iterate verify stats: %w", err)
		}
		for i := range out {
			out[i].VerifyChecked = stats[out[i].ID][0]
			out[i].VerifyMismatched = stats[out[i].ID][1]
		}
	}
	return out, total, nil
}

// AdminExportBattles 导出战报列表为 CSV（2026-10-30 实质优化第 13 轮）。
//
// 与 `AdminExportUsers` 同族：运营做数据分析需要全量快照而不是分页。
// 过滤条件与 `AdminListBattles` 完全一致（user_id / level_id / only_mismatched），
// 上限 100_000 行。replay_hash 是回放校验哈希（后台本来就可读），
// 不是账号凭证，可以导出；guest_token 等敏感字段绝不进入。
func (s *Service) AdminExportBattles(ctx context.Context, userID int64, levelID int, onlyMismatched bool, dateFrom, dateTo *time.Time, result string) ([]byte, error) {
	// 与 AdminListBattles 相同的子查询形态（注释见那边）。
	vsub := `LEFT JOIN (
		SELECT battle_id,
		       COUNT(*) AS checked,
		       COUNT(*) FILTER (WHERE NOT matched) AS mismatched
		FROM replay_verifications GROUP BY battle_id
	) v ON v.battle_id = br.id`
	if userID != 0 {
		vsub = `LEFT JOIN (
		SELECT battle_id,
		       COUNT(*) AS checked,
		       COUNT(*) FILTER (WHERE NOT matched) AS mismatched
		FROM replay_verifications
		WHERE battle_id IN (SELECT id FROM battle_records WHERE user_id = $1)
		GROUP BY battle_id
	) v ON v.battle_id = br.id`
	}

	where := `($1 = 0 OR br.user_id = $1) AND ($2 = 0 OR br.level_id = $2)`
	args := []any{userID, levelID}
	if onlyMismatched {
		where += ` AND COALESCE(v.mismatched, 0) > 0`
	}
	// 日期范围（2026-10-31 新一轮第 14 轮）：**始终**拼入（nil → NULL，
	// `IS NULL` 分支恒真），保证 result 筛选的 $5 稳定存在。
	// 导出查询没有 limit/offset，日期直接占 $3/$4。
	where += ` AND ($3::timestamptz IS NULL OR br.created_at >= $3)
	             AND ($4::timestamptz IS NULL OR br.created_at < $4)`
	args = append(args, dateFrom, dateTo)
	// 结果筛选（2026-11-01 新一轮第 44 轮）：与列表同口径。
	result = strings.TrimSpace(result)
	if result != "" {
		where += ` AND br.result = $5`
		args = append(args, result)
	}

	rows, err := s.pool.Query(ctx, `
		SELECT br.id, br.user_id, COALESCE(u.nickname,''), br.level_id, br.result, br.stars, br.score,
		       br.kills, br.leaked, br.wave_reached, br.duration_ms, br.reactions, br.heat_max,
		       br.replay_hash, br.created_at,
		       COALESCE(v.checked, 0), COALESCE(v.mismatched, 0)
		FROM battle_records br
		LEFT JOIN users u ON u.id = br.user_id
		`+vsub+`
		WHERE `+where+`
		ORDER BY br.id DESC LIMIT 100001`, args...)
	if err != nil {
		return nil, fmt.Errorf("export battles: %w", err)
	}
	defer rows.Close()

	buf := s.newExportBuffer()
	buf.WriteString("\xEF\xBB\xBF")
	w := csv.NewWriter(buf)
	header := []string{
		"id", "user_id", "nickname", "level_id", "result", "stars", "score",
		"kills", "leaked", "wave_reached", "duration_ms", "reactions", "heat_max",
		"replay_hash", "created_at", "verify_checked", "verify_mismatched", "total_enemies",
	}
	if err := w.Write(header); err != nil {
		return nil, fmt.Errorf("write csv header: %w", err)
	}

	// 与列表同源：本关总怪数取自生成器（确定性），按 level_id 缓存。
	totalCache := map[int]int{}
	totalEnemies := func(levelID int) int {
		if n, ok := totalCache[levelID]; ok {
			return n
		}
		n := 0
		for _, wave := range domain.CachedLevel(levelID).Waves {
			for _, sp := range wave.Spawns {
				n += sp.Count
			}
		}
		totalCache[levelID] = n
		return n
	}

	n := 0
	for rows.Next() {
		n++
		var v AdminBattleView
		if err := rows.Scan(&v.ID, &v.UserID, &v.Nickname, &v.LevelID, &v.Result, &v.Stars,
			&v.Score, &v.Kills, &v.Leaked, &v.WaveReached, &v.DurationMs, &v.Reactions,
			&v.HeatMax, &v.ReplayHash, &v.CreatedAt,
			&v.VerifyChecked, &v.VerifyMismatched); err != nil {
			return nil, fmt.Errorf("scan export battle: %w", err)
		}
		v.TotalEnemies = totalEnemies(v.LevelID)
		rec := []string{
			strconv.FormatInt(v.ID, 10),
			strconv.FormatInt(v.UserID, 10),
			v.Nickname,
			strconv.Itoa(v.LevelID),
			v.Result,
			strconv.Itoa(v.Stars),
			strconv.FormatInt(v.Score, 10),
			strconv.Itoa(v.Kills),
			strconv.Itoa(v.Leaked),
			strconv.Itoa(v.WaveReached),
			strconv.Itoa(v.DurationMs),
			strconv.Itoa(v.Reactions),
			strconv.Itoa(v.HeatMax),
			v.ReplayHash,
			v.CreatedAt.Format(time.RFC3339),
			strconv.FormatInt(v.VerifyChecked, 10),
			strconv.FormatInt(v.VerifyMismatched, 10),
			strconv.Itoa(v.TotalEnemies),
		}
		if err := writeCSVRow(w, rec); err != nil {
			return nil, fmt.Errorf("write csv row: %w", err)
		}
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	if n > adminExportMaxRows {
		return nil, exportTooLarge(n)
	}
	w.Flush()
	if err := w.Error(); err != nil {
		return nil, fmt.Errorf("flush csv: %w", err)
	}
	return buf.Bytes(), nil
}

// --- 内容配置 ---

// AdminUpdateLevel 更新关卡配置。
//
// 只有白名单字段可写 —— 客户端上报的任意字段绝不能直接进库。
//
// R43：关卡参数与审计同一事务。改一关的血量/波次/星级门槛是运营日常动作，
// 而审计此前由 handler 提交后补写、失败只打日志 —— 一旦审计写不进去，
// 玩家遇到的难度变化没有任何「谁在什么时候调过」的记录，排查时只能靠猜。
func (s *Service) AdminUpdateLevel(ctx context.Context, levelID int, patch map[string]any,
	audit ...AdminAuditCtx) (AdminLevelView, error) {
	fields := []string{}
	args := []any{levelID}
	set := func(col string, v any) {
		args = append(args, v)
		fields = append(fields, fmt.Sprintf("%s = $%d", col, len(args)))
	}
	//
	// ⚠️ 第 107 轮：**非法键整单拒绝**，不再「静默丢弃那一项」。
	//
	// 修前每个 case 都是 `if 条件 { set(...) }`，条件不满足就什么都不做，
	// 而循环继续。于是：
	//
	//	PUT {"name":"P107改名", "base_hp":-5}  →  200
	//	  库里 name 改了，base_hp **没改**
	//
	// 运营看到 200 就以为两个字段都调过了。
	// 那是「部分生效 + 成功回执」，比直接报错坏得多 ——
	// 直接报错至少会说「你的请求有问题」。
	//
	// 只有一个键时 `len(fields)==0` 已经能兜住（返回 400「没有可更新的字段」），
	// 但**混合**情况兜不住 —— 而混合恰恰是运营调参时最常见的形状。
	var rejected []string
	reject := func(key, why string) {
		rejected = append(rejected, fmt.Sprintf("%s（%s）", key, why))
	}

	for key, val := range patch {
		switch key {
		case "name":
			v, ok := val.(string)
			if !ok || v == "" {
				reject(key, "必须是非空字符串")
				continue
			}
			set("name", v)
		case "base_hp":
			v, ok := toInt64(val)
			if !ok || v <= 0 {
				reject(key, "必须是正整数")
				continue
			}
			set("base_hp", v)
		case "wave_count":
			v, ok := toInt64(val)
			if !ok || v <= 0 || v > 50 {
				reject(key, "必须在 1..50")
				continue
			}
			set("wave_count", v)
		case "difficulty":
			v, ok := toInt64(val)
			if !ok || v <= 0 {
				reject(key, "必须是正整数")
				continue
			}
			set("difficulty", v)
		case "energy_cost":
			v, ok := toInt64(val)
			if !ok || v < 0 || v > 100 {
				reject(key, "必须在 0..100")
				continue
			}
			set("energy_cost", v)
		case "star_targets":
			//
			// ⚠️ 第 107 轮：原来只有 `json.Marshal` 成不成功这一个判据。
			//
			// 我**原以为** `{"star_targets": null}` 会撞 NOT NULL 变成 500 ——
			// **实测否证**：JSONB 的 `null` 是一个**合法的 JSONB 值**，
			// 不是 SQL NULL，所以 NOT NULL 约束照样通过。
			// （第 4 次「结论下得太早」：第 85/87/92/103 轮之后。）
			//
			// 但实测确实暴露了另一件事：`{"star_targets": "不是数组"}` 与
			// `{"terrain_config": 42}` 都会被**接受**，
			// 于是 `AdminListLevels` 的 `json.Unmarshal` 失败、
			// `stars` / `terrain` 保持 nil，后台看到的是「星级目标为空」。
			//
			// 那是「写进去一个永远读不出来的东西」——
			// 合法 JSONB、合法约束，但语义上已坏，且没有任何报错。
			// 所以判据必须落在**形状**上，不是「能不能序列化」。
			//
			// 显式的 JSON `null` 是**清空语义**，必须放行。
			// ⚠️ 我第一版的形状校验一刀切 `val.([]any)`，
			// 于是 `{"star_targets": null}` 变成 400 ——
			// 而「清空星级目标」是运营会真的想做的事。
			//
			// 判据踩在「合法」的边界上时，要么误杀、要么漏判；
			// 所以「**该接受的**」也要逐条列出（见
			// `TestLevelPatchAcceptsEmptyJSONArrays`）。
			//
			// 而且 `json.Unmarshal([]byte("null"), &[]int64{})` 是**成功**的
			// （结果为 nil 切片），所以存 `null` 读回来不会报错。
			if val == nil {
				set(key, json.RawMessage("null"))
				continue
			}
			arr, ok := val.([]any)
			if !ok {
				reject(key, "必须是数组或 null")
				continue
			}
			badElem := false
			for _, e := range arr {
				n, ok := toInt64(e)
				if !ok || n < 0 {
					reject(key, "数组元素必须是非负整数")
					badElem = true
					break
				}
			}
			if badElem {
				continue
			}
			raw, err := json.Marshal(arr)
			if err != nil {
				reject(key, "无法序列化")
				continue
			}
			set("star_targets", raw)
		case "terrain_config":
			if val == nil {
				set(key, json.RawMessage("null"))
				continue
			}
			arr, ok := val.([]any)
			if !ok {
				reject(key, "必须是数组或 null")
				continue
			}
			badElem := false
			for _, e := range arr {
				if _, ok := e.(map[string]any); !ok {
					reject(key, "数组元素必须是对象")
					badElem = true
					break
				}
			}
			if badElem {
				continue
			}
			raw, err := json.Marshal(arr)
			if err != nil {
				reject(key, "无法序列化")
				continue
			}
			set("terrain_config", raw)
		case "enabled":
			v, ok := val.(bool)
			if !ok {
				reject(key, "必须是布尔值")
				continue
			}
			set("enabled", v)
		case "is_boss":
			v, ok := val.(bool)
			if !ok {
				reject(key, "必须是布尔值")
				continue
			}
			set("is_boss", v)
		default:
			// 未知键必须报错 —— 「白名单外的字段被忽略」是个**陷阱**：
			// 运营拼错字段名（`baseHP` / `hp`）时会被静默吞掉，
			// 而请求整体因为有别的合法键而返回 200。
			reject(key, "不在可更新字段的白名单里")
		}
	}

	if len(rejected) > 0 {
		sort.Strings(rejected)
		return AdminLevelView{}, fmt.Errorf("%w: 这些字段没被接受，整单未执行：%s",
			ErrBadInput, strings.Join(rejected, "、"))
	}
	if len(fields) == 0 {
		return AdminLevelView{}, fmt.Errorf("%w: 没有可更新的字段", ErrBadInput)
	}
	sql := fmt.Sprintf(`UPDATE levels SET %s, updated_at = now() WHERE id = $1`,
		joinComma(fields))
	err := s.DB.Tx(ctx, func(tx txType) error {
		// R75：审计记「改前」——只抽 patch 触及字段的旧值（整关配置
		// 含 star_targets/terrain_config jsonb，全量进审计会吹到几十 KB）。
		before, berr := auditRowSnapshot(ctx, tx, "levels", levelID, patchKeySet(patch))
		if berr != nil {
			// 顺手钉住语义：改前 UPDATE 零行不报错（改了个不存在的关卡也 200）。
			if errors.Is(berr, pgx.ErrNoRows) {
				return fmt.Errorf("%w: 关卡 %d 不存在", ErrNotFound, levelID)
			}
			return fmt.Errorf("update level: read before snapshot: %w", berr)
		}
		if _, err := tx.Exec(ctx, sql, args...); err != nil {
			return fmt.Errorf("update level: %w", err)
		}
		if len(audit) > 0 {
			return s.AuditTx(ctx, tx, audit[0], "update_level",
				strconv.Itoa(levelID), map[string]any{"before": before, "patch": patch})
		}
		return nil
	})
	if err != nil {
		return AdminLevelView{}, err
	}
	//
	// ⚠️ 第 107 轮：这里原来返回 `domain.GenerateLevel(levelID)`。
	//
	// 那是**纯函数**（只从 ChapterOf + LCG 算，从不查库），
	// 所以运营刚把 base_hp 改成 987654，响应里却是改**前**的 1000。
	//
	// 一个与自己的写入相矛盾的响应，不管背后是哪条设计路线，都是缺陷。
	// 现在回读**真实落库的那一行**。
	//
	// ⚠️ 这**不代表**改动对玩家生效 —— 玩家侧走的是纯生成器。
	// 见 `admin_level_authority_test.go` 的现状刻画测试，
	// 以及 README 已知边界第 23 条（需要产品决策）。
	row, err := s.AdminLevelRow(ctx, levelID)
	if err != nil {
		return AdminLevelView{}, fmt.Errorf("read back level: %w", err)
	}
	return row, nil
}

// AdminUpdateSkill 更新技能配置，并**回读落库后的那一行**。
//
// ⚠️ 第 120 轮：修前只返回 error，handler 只好退回去整个重列 `adminSkills`
// （`{items, recipes}`）当响应。但后台契约与 UI 的 `updateSkill` 写的是
// `{ skill: AdminSkill }`，于是 `Object.assign(row, res.skill)` 里
// `res.skill` 恒为 undefined —— 改完技能表行**停在上次刷新的旧值**
// 却弹「已保存」（写后读与写入矛盾，第 107/108 轮同族）。
// 现在返回读回的行，响应即「后台刷新列表会看到的那一行」。
//
// R45：技能展示文案（name/descr）的修改与审计同一事务。改名字看着轻，
// 但它直接是玩家在技能面板看到的文案；审计此前在 handler 提交后补写、
// 失败只打日志，运营改错一次没有「谁改的、改成什么」可查。
func (s *Service) AdminUpdateSkill(ctx context.Context, skillID int, patch map[string]any,
	audit ...AdminAuditCtx) (AdminSkillView, error) {
	fields := []string{}
	args := []any{skillID}
	set := func(col string, v any) {
		args = append(args, v)
		fields = append(fields, fmt.Sprintf("%s = $%d", col, len(args)))
	}
	//
	// 2026-10-21 (P2-10): unknown/invalid keys are now rejected as a whole
	// request, exactly like AdminUpdateLevel and AdminUpdateShopItem.
	//
	// Before, every case was `if ok { set(...) }` and the loop continued, so
	//
	//	PUT {"name":"X", "base_damage":-5}  ->  200
	//	  the name was written, base_damage was silently dropped
	//
	// Ops saw 200 and believed both fields had been tuned. "Partially applied
	// with a success receipt" is worse than an outright error, because an error
	// at least says "your request was wrong".
	var rejected []string
	reject := func(key, why string) {
		rejected = append(rejected, fmt.Sprintf("%s (%s)", key, why))
	}

	for key, val := range patch {
		switch key {
		case "name", "descr":
			v, ok := val.(string)
			if !ok || v == "" {
				reject(key, "must be a non-empty string")
				continue
			}
			set(key, v)
		case "base_damage", "heat_cost", "cooldown_ms", "pierce", "aoe_radius",
			"apply_stacks", "projectile_speed", "chain", "element", "family", "kind":
			// 第 23 轮：战斗数值/语义字段从白名单**移除**（关卡数值同款口径，
			// 见 level_authority_test.go）。
			//
			// # 为什么它们不能进后台
			//
			// 结算的 S 段校验（replaySkillsSegment）按**库值**重算
			// `slot:skill_id:base_damage:apply_stacks:heat_cost`，
			// 而客户端引擎按 **/config 静态种子**（domain.SeedSkills →
			// ScaleAllSkills）算同一段。两端各读一套的缝隙里，
			// admin 改 base_damage = 全部带 replay_skills 的结算 422
			// （enforce 模式默认开启）——「诚实玩家被误判」的最坏形态。
			// element/family/kind 同理：它们经 /config 之外的路径（构筑
			// 元素列表、装备校验）影响战斗，改库同样造成两端失配。
			//
			// 这不是「后台功能不全」，是**权威源只有一个**：战斗数值
			// 只能随版本发布（domain 种子），不能被运行时改出缝。
			reject(key, "gameplay value is version-controlled (domain seed); runtime edits break the replay-skills contract")
		default:
			// An unknown key must be reported: silently ignoring it is a trap,
			// because the caller gets 200 and believes the field was tuned.
			reject(key, "not in the updatable-field whitelist")
		}
	}
	if len(rejected) > 0 {
		sort.Strings(rejected)
		return AdminSkillView{}, fmt.Errorf("%w: these fields were rejected, nothing was written: %s",
			ErrBadInput, strings.Join(rejected, ", "))
	}
	if len(fields) == 0 {
		return AdminSkillView{}, fmt.Errorf("%w: no updatable fields", ErrBadInput)
	}
	sql := fmt.Sprintf(`UPDATE skills SET %s WHERE id = $1`, joinComma(fields))
	if err := s.DB.Tx(ctx, func(tx txType) error {
		// R75：审计记「改前」——只抽 patch 触及字段（name/descr）的旧值。
		before, berr := auditRowSnapshot(ctx, tx, "skills", skillID, patchKeySet(patch))
		if berr != nil {
			if errors.Is(berr, pgx.ErrNoRows) {
				return fmt.Errorf("%w: 技能 %d 不存在", ErrNotFound, skillID)
			}
			return fmt.Errorf("update skill: read before snapshot: %w", berr)
		}
		if _, err := tx.Exec(ctx, sql, args...); err != nil {
			return fmt.Errorf("update skill: %w", err)
		}
		if len(audit) > 0 {
			return s.AuditTx(ctx, tx, audit[0], "update_skill",
				strconv.Itoa(skillID), map[string]any{"before": before, "patch": patch})
		}
		return nil
	}); err != nil {
		return AdminSkillView{}, err
	}
	// Read the row back (round 120: the response IS the row the admin table
	// would show after a refresh).
	row, err := s.skillRow(ctx, skillID)
	if err != nil {
		return AdminSkillView{}, fmt.Errorf("read back skill: %w", err)
	}
	return row, nil
}

// skillRow 读回单行技能配置（与 fetchSkills 的列一致，供写后读与后台契约对齐）。
func (s *Service) skillRow(ctx context.Context, skillID int) (AdminSkillView, error) {
	var (
		id, heatCost, cooldown, pierce, aoe, stacks, speed, chain, unlock int
		code, name, family, element, kind, descr, applyElement            string
		baseDamage                                                        int64
	)
	err := s.pool.QueryRow(ctx, `
		SELECT id, code, name, family, element, kind, descr, base_damage, heat_cost,
		       cooldown_ms, pierce, aoe_radius, apply_element, apply_stacks,
		       projectile_speed, chain, unlock_level
		FROM skills WHERE id = $1`, skillID).
		Scan(&id, &code, &name, &family, &element, &kind, &descr,
			&baseDamage, &heatCost, &cooldown, &pierce, &aoe, &applyElement, &stacks,
			&speed, &chain, &unlock)
	if err != nil {
		return AdminSkillView{}, err
	}
	return AdminSkillView{
		ID: id, Code: code, Name: name, Family: family, Element: element,
		Kind: kind, Descr: descr, BaseDamage: baseDamage,
		HeatCost: heatCost, CooldownMs: cooldown, Pierce: pierce,
		AoeRadius: aoe, ApplyElement: applyElement, ApplyStacks: stacks,
		ProjectileSpeed: speed, Chain: chain, UnlockLevel: unlock,
	}, nil
}

// AdminRegenerateLevels 重新生成全部关卡（覆盖后台的手工改动）。
//
// # 第 116 轮两处修正
//
//  1. **保留运营的 `base_hp`**（与后台确认文案一致）：
//     文案承诺「自定义的防线血量会保留」，但旧实现把 `base_hp` 也按生成器值
//     覆盖掉 —— 运营手工调过的防线血量被无声重置。
//     重新生成的语义是「同步生成器内容（波次/难度/地形/星级）」，
//     不该连运营自己调的 base_hp 一起冲掉。
//
//  2. **整批原子 + 不再吞错**：旧实现 100 条 UPDATE 各走独立 autocommit、
//     且 `starRaw,_ := json.Marshal` 静默吞错。中途失败会留下
//     「前 N 关已重生成 + 后 M 关保留旧值」的**混合态**且不可回滚。
//     现在整批进同一事务，任一失败整体回滚；Marshal 失败显式报错。
//
// # 第 117 轮
//  3. **同步 `level_waves`**：旧实现只改 levels 行，从不碰波次表。
//     生成器升级后点「重新生成」，后台波次抽屉与 levels.wave_count
//     仍停在旧版本。现在同一事务里整体替换每关的 level_waves。
func (s *Service) AdminRegenerateLevels(ctx context.Context, audit ...AdminAuditCtx) (int, error) {
	levels := domain.GenerateAllLevels()
	err := s.DB.Tx(ctx, func(tx txType) error {
		for _, gl := range levels {
			starRaw, err := json.Marshal(gl.StarTargets)
			if err != nil {
				return fmt.Errorf("regenerate level %d: marshal star_targets: %w", gl.ID, err)
			}
			terrainRaw, err := json.Marshal(gl.Terrain)
			if err != nil {
				return fmt.Errorf("regenerate level %d: marshal terrain: %w", gl.ID, err)
			}
			// 注意：**没有 base_hp** —— 保留库里运营自定义的值。
			if _, err := tx.Exec(ctx, `
				UPDATE levels SET chapter=$2, name=$3, seed=$4, wave_count=$5,
				                 difficulty=$6, energy_cost=$7, star_targets=$8, terrain_config=$9,
				                 is_boss=$10, updated_at=now()
				 WHERE id=$1`,
				gl.ID, gl.Chapter, gl.Name, gl.Seed, gl.WaveCount, gl.Difficulty,
				gl.EnergyCost, starRaw, terrainRaw, gl.IsBoss); err != nil {
				return fmt.Errorf("regenerate level %d: %w", gl.ID, err)
			}
			//
			// ⚠️ 第 117 轮：同步 `level_waves`。
			// 「重新生成」的完整含义是让库里的**关卡内容**对齐生成器，
			// 而波次的敌人排布就在 level_waves 表里。旧实现只改 levels 行、
			// 从不碰 level_waves —— 生成器升级（波次数/刷怪变化）后点「重新生成」，
			// 后台「波次」抽屉与 levels.wave_count 仍停在旧版本，两者永久失同步。
			// level_waves 没有运营编辑入口（后台只有只读抽屉），
			// 是纯生成器派生内容，整体替换安全：先删旧波次、再写生成器的。
			if _, err := tx.Exec(ctx,
				`DELETE FROM level_waves WHERE level_id = $1`, gl.ID); err != nil {
				return fmt.Errorf("regenerate level %d: clear waves: %w", gl.ID, err)
			}
			for _, w := range gl.Waves {
				spawns, err := json.Marshal(w.Spawns)
				if err != nil {
					return fmt.Errorf("regenerate level %d: marshal wave %d: %w", gl.ID, w.Index, err)
				}
				if _, err := tx.Exec(ctx,
					`INSERT INTO level_waves (level_id, wave_index, spawns) VALUES ($1,$2,$3)`,
					gl.ID, w.Index, spawns); err != nil {
					return fmt.Errorf("regenerate level %d: insert wave %d: %w", gl.ID, w.Index, err)
				}
			}
		}
		// R42：重建全部 100 关与审计同一事务。这是全库**最重**的写操作 ——
		// 覆盖运营手工调过的所有关卡内容且不可回滚。审计写不进去时若照常生效，
		// 事后只能看到「关卡全变了」，没有任何「谁在什么时候按了重建」的记录。
		if len(audit) > 0 {
			if err := s.AuditTx(ctx, tx, audit[0], "regenerate_levels", "",
				map[string]any{"count": len(levels)}); err != nil {
				return err
			}
		}
		return nil
	})
	if err != nil {
		return 0, err
	}
	return len(levels), nil
}

func joinComma(s []string) string {
	out := ""
	for i, v := range s {
		if i > 0 {
			out += ", "
		}
		out += v
	}
	return out
}

func toInt64(v any) (int64, bool) {
	switch n := v.(type) {
	case float64:
		return int64(n), true
	case int:
		return int64(n), true
	case int64:
		return n, true
	case json.Number:
		i, err := n.Int64()
		return i, err == nil
	}
	return 0, false
}
