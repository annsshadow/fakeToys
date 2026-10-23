package service

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strconv"
	"strings"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgconn"

	"github.com/laiyipao/server/internal/domain"
)

// code2Session 调用微信 jscode2session 换取 openid。
//
// 真实接入需要 AppID/Secret；未配置时 WechatLogin 会先行拒绝，
// 所以这里只在配置完整时才会被调用。
func (s *Service) code2Session(ctx context.Context, code string) (string, error) {
	q := url.Values{}
	q.Set("appid", s.Cfg.WechatAppID)
	q.Set("secret", s.Cfg.WechatAppSecret)
	q.Set("js_code", code)
	q.Set("grant_type", "authorization_code")

	req, err := http.NewRequestWithContext(ctx, http.MethodGet,
		s.Cfg.WechatEndpoint+"?"+q.Encode(), nil)
	if err != nil {
		return "", fmt.Errorf("build wechat request: %w", err)
	}
	reqCtx, cancel := context.WithTimeout(ctx, 8*time.Second)
	defer cancel()

	resp, err := http.DefaultClient.Do(req.WithContext(reqCtx))
	if err != nil {
		return "", fmt.Errorf("call wechat: %w", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(io.LimitReader(resp.Body, 1<<20))
	if err != nil {
		return "", fmt.Errorf("read wechat response: %w", err)
	}

	var payload struct {
		OpenID     string `json:"openid"`
		UnionID    string `json:"unionid"`
		SessionKey string `json:"session_key"`
		ErrCode    int    `json:"errcode"`
		ErrMsg     string `json:"errmsg"`
	}
	if err := json.Unmarshal(body, &payload); err != nil {
		return "", fmt.Errorf("parse wechat response: %w", err)
	}
	if payload.ErrCode != 0 {
		return "", fmt.Errorf("%w: 微信返回 errcode=%d %s", ErrForbidden, payload.ErrCode, payload.ErrMsg)
	}
	if payload.OpenID == "" {
		return "", fmt.Errorf("%w: 微信未返回 openid", ErrForbidden)
	}
	return payload.OpenID, nil
}

// --- 排行榜 ---

// LeaderboardEntry 是一条榜单记录。
type LeaderboardEntry struct {
	Rank      int    `json:"rank"`
	UserID    int64  `json:"user_id"`
	Nickname  string `json:"nickname"`
	AvatarURL string `json:"avatar_url"`
	Score     int64  `json:"score"`
	Detail    string `json:"detail,omitempty"`
}

// leaderboardCacheRows 是缓存里存的行数（也是单次查询取的行数）。
//
// 取 200 与 `limit` 的上界一致。做法是「总是取满 200 行、再按 limit 切片」：
// 这样每个榜只有**一个**缓存条目，不必为 limit=10 / limit=50 各存一份。
// 多取的 150 行是 150 个 `(id,nickname,avatar,score)`，代价可以忽略，
// 换来的是「缓存键只有 kind」这件简单的事。
const leaderboardCacheRows = 200

// Leaderboard 返回指定榜单。
//
// 三种榜单对应三种不同的"强"：战力榜=资源总量，效率榜=技巧，
// 关卡榜=进度。分开排才能让低养成高技巧的玩家也有位置。
//
// # 缓存（2026-10-23）
//
// 三个榜都是 `level_stars JOIN users` 的全表 `GROUP BY` + 排序。
// 玩家每进一次排行页就算一次，而 15 秒的陈旧没人察觉。
// 取舍与关闭方式见 `config.LeaderboardCacheTTL`。
func (s *Service) Leaderboard(ctx context.Context, kind string, limit int) ([]LeaderboardEntry, error) {
	if limit <= 0 || limit > 200 {
		limit = 50
	}
	all, err := s.leaderboardAll(ctx, kind)
	if err != nil {
		return nil, err
	}
	return sliceLeaderboard(all, limit), nil
}

// leaderboardAll 取榜单全量（固定 leaderboardCacheRows 行）。
//
// cached 的 kind 走 singleflight —— TTL 到期那一刻的并发未命中只查一次库，
// 不再惊群；未知 kind（不在 leaderboardCache 里）直接查库、不缓存，
// 与旧行为逐字一致。
func (s *Service) leaderboardAll(ctx context.Context, kind string) ([]LeaderboardEntry, error) {
	if c, ok := s.leaderboardCache[kind]; ok {
		return c.getOrLoad(time.Now(), s.Cfg.LeaderboardCacheTTL, func() ([]LeaderboardEntry, error) {
			return s.leaderboardQuery(ctx, kind)
		})
	}
	return s.leaderboardQuery(ctx, kind)
}

// leaderboardQuery 是真正的榜单查询。缓存与单飞逻辑只在上面一层，
// 这样「查什么」与「缓存多久 / 是否单飞」不互相纠缠。
func (s *Service) leaderboardQuery(ctx context.Context, kind string) ([]LeaderboardEntry, error) {
	rows, err := s.pool.Query(ctx, `
		WITH scores AS (`+leaderboardScoresSQL(kind)+`)
		SELECT u.id, u.nickname, u.avatar_url, s.score
		FROM scores s JOIN users u ON u.id = s.user_id
		ORDER BY s.score `+leaderboardOrderSQL(kind)+`, u.id ASC
		LIMIT $1`, leaderboardCacheRows)
	if err != nil {
		return nil, fmt.Errorf("leaderboard %s: %w", kind, err)
	}
	defer rows.Close()

	out := make([]LeaderboardEntry, 0, leaderboardCacheRows)
	rank := 1
	for rows.Next() {
		var e LeaderboardEntry
		if err := rows.Scan(&e.UserID, &e.Nickname, &e.AvatarURL, &e.Score); err != nil {
			return nil, err
		}
		e.Rank = rank
		rank++
		out = append(out, e)
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	return out, nil
}

// sliceLeaderboard 按 limit 截断。
//
// ⚠️ 用的是**三索引切片** `all[:limit:limit]`，不是 `all[:limit]`。
//
// 后者的 cap 仍是 200，于是调用方一旦 `append`，就会写进缓存那份底层数组的
// 第 limit+1 个位置 —— 榜单内容随机变化，且没有任何报错。
// `ttlcache` 的「调用方必须只读」是**口头契约**，编译器不检查；
// 三索引切片把它变成**结构约束**（append 会分配新数组）。
// `TestSliceLeaderboard` 有一条 `cap(got) == limit` 钉住这一点。
func sliceLeaderboard(all []LeaderboardEntry, limit int) []LeaderboardEntry {
	if limit >= len(all) {
		return all[:len(all):len(all)]
	}
	return all[:limit:limit]
}

// leaderboardScoresSQL 是「用户 → 该榜分数」的**唯一**定义。
//
// # 为什么抽出来（2026-10-23）
//
// 这一段的三个分支此前**内联在 Leaderboard 里**。加 `/me/rank` 时如果照抄一份，
// 就会出现两份「效率榜取 MIN、其余取 MAX」的实现 —— 而它们只在
// 「排序方向」与「过滤条件」上分叉。分叉之后**症状是静默的**：
// 榜单按 A 规则排、我的名次按 B 规则算，玩家看到「我在榜上第 3，
// 但榜上第 3 不是我」，且没有任何报错。
//
// 现在两个端点共用这一个字符串，排序方向也共用 `leaderboardOrderSQL`。
// 判据由 `TestMyRankMatchesLeaderboardOrder` 钉住（三榜各一次）。
func leaderboardScoresSQL(kind string) string {
	switch kind {
	case "stage":
		// 关卡榜：最高通关关卡
		return `SELECT user_id, MAX(level_id)::bigint AS score
		        FROM level_stars WHERE clears > 0 GROUP BY user_id`
	case "efficiency":
		// 效率榜：通关时使用的最低战力越低越靠前（L-2）
		return `SELECT user_id, MIN(min_power_clear) AS score
		        FROM level_stars WHERE min_power_clear > 0 GROUP BY user_id`
	default: // "power"
		// 战力榜：累计成长值
		return `SELECT user_id, level_exp AS score FROM user_progress`
	}
}

// leaderboardOrderSQL 是排序方向。返回值只可能是两个字面量之一 ——
// 它会被拼进 SQL，**不能**接受任何外部输入。
func leaderboardOrderSQL(kind string) string {
	if kind == "efficiency" {
		return "ASC"
	}
	return "DESC"
}

// MyRank 是「我在榜上的位置」（2026-10-23）。
//
// # 它补的是什么
//
// `Leaderboard` 只回前 N 条，而 `rank.vue` 的「是我」高亮**只在前 50 内有效**。
// 未进前 50 的玩家在榜上找不到自己，也不知道总共有多少人 ——
// 这是「有榜单但玩家用不上」的典型形态。
//
// # 为什么是独立端点而不是给 `/leaderboard` 加可选鉴权
//
// `/leaderboard` 是公开端点。给它加「带了 token 就多返回 me」会让同一个
// URL 因请求头不同而返回不同形状 —— 缓存、调试、契约测试都要跟着分叉。
// 独立端点挂在 `/me` 下，鉴权是它的全部意义，不引入隐式分支。
//
// `rank = 0` 表示**无有效成绩**（例如从未通关时的关卡榜）。
// 不能返回 1：那会让「没成绩的人」看起来是榜首。
type MyRank struct {
	Kind  string `json:"kind"`
	Rank  int    `json:"rank"`
	Total int64  `json:"total"`
	Score int64  `json:"score"`
}

// MyRank 计算玩家在指定榜单上的名次。
//
// # 名次口径：必须与 Leaderboard 的列表下标逐字一致
//
// `Leaderboard` 的名次是**行号**（1,2,3…），排序是
// `ORDER BY s.score <dir>, u.id ASC`。所以「优于我」不能只比分数 ——
// 同分时 id 小的排在我前面。
//
// 第一版只比了分数（`s.score > me.score`），于是并列同分的两人
// **都拿到第 1 名**，而列表里一个是第 1、一个是第 2。
// 玩家会看到「我的排名 第 1」而榜上自己高亮在第 2 条 ——
// 效率榜（`MIN(min_power_clear)`）并列极常见，这条不是理论问题。
//
// 现在的判据是**双键**，与 `ORDER BY` 的两列一一对应：
//
//	DESC 榜：s.score > me.score OR (s.score = me.score AND s.user_id < $1)
//	ASC  榜：s.score < me.score OR (s.score = me.score AND s.user_id < $1)
//
// 于是 rank 恒等于「我在完整榜上的下标 + 1」，由
// `TestMyRankMatchesLeaderboardOrder`（含同分用例）钉住。
func (s *Service) MyRank(ctx context.Context, userID int64, kind string) (MyRank, error) {
	// 与 Leaderboard 同源：分数定义 + 排序方向都不在这里另写一份。
	scores := leaderboardScoresSQL(kind)
	// ⚠️ 未知 kind 必须**规范化**成 power，而不是「用 power 的数据 + 回显原始 kind」。
	// `routes.go` 里同一条纪律：响应里的 type 必须与实际返回的数据是同一件事，
	// 否则调用方会在「日榜」标题下显示战力榜数据且无法察觉。
	if kind != "stage" && kind != "efficiency" {
		kind = "power"
	}
	cmp := ">"
	if leaderboardOrderSQL(kind) == "ASC" {
		// 效率榜「分数越低越好」，所以优于我 = 比我小
		cmp = "<"
	}
	q := `
		WITH scores AS (` + scores + `),
		     me AS (SELECT score FROM scores WHERE user_id = $1)
		SELECT
		    COALESCE((SELECT score FROM me), 0)::bigint,
		    (SELECT COUNT(*) FROM scores),
		    CASE WHEN NOT EXISTS (SELECT 1 FROM me) THEN 0
		         ELSE (SELECT COUNT(*) FROM scores s, me
		                WHERE s.score ` + cmp + ` me.score
		                   OR (s.score = me.score AND s.user_id < $1)) + 1
		    END`
	var out MyRank
	out.Kind = kind
	if err := s.pool.QueryRow(ctx, q, userID).Scan(&out.Score, &out.Total, &out.Rank); err != nil {
		return MyRank{}, fmt.Errorf("my rank %s: %w", kind, err)
	}
	return out, nil
}

// --- 签到 / 离线 / 兑换 / 商店 ---

// SignInResult 是签到结果。
type SignInResult struct {
	Already  bool           `json:"already"`
	DayIndex int            `json:"day_index"`
	Reward   map[string]int `json:"reward"`
	Wallet   Wallet         `json:"wallet"`
}

// SignInCalendarDay 是七日签到日历的一行（第 N 天给什么奖励）。
type SignInCalendarDay struct {
	Day    int            `json:"day_index"`
	Reward map[string]int `json:"reward"`
}

// SignInCalendar 返回服务端权威的七日签到奖励表。
//
// ⚠️ 第 135 轮：客户端此前的 7 天奖励预览是**本地硬编码公式**
// （miniapp signin.vue 的 rewardFor：coin=1000×天、第 3/7 天 gem、第 7 天
// 体力）。它与 seeder 写入 sign_in_calendar 的公式**今天**恰好一致，
// 但那是两份独立定义 —— 运营重灌/调节日历后，客户端预览会静默漂移，
// 与玩家实际拿到的 `SignInResult.Reward` 对不上。
// 故把日历作为权威来源下发，客户端预览改读它。
func (s *Service) SignInCalendar(ctx context.Context) ([]SignInCalendarDay, error) {
	rows, err := s.pool.Query(ctx,
		`SELECT day_index, reward FROM sign_in_calendar ORDER BY day_index`)
	if err != nil {
		return nil, fmt.Errorf("load sign-in calendar: %w", err)
	}
	defer rows.Close()
	out := []SignInCalendarDay{}
	for rows.Next() {
		var d SignInCalendarDay
		var rewardRaw []byte
		if err := rows.Scan(&d.Day, &rewardRaw); err != nil {
			return nil, err
		}
		if err := json.Unmarshal(rewardRaw, &d.Reward); err != nil {
			return nil, fmt.Errorf("unmarshal sign-in reward day %d: %w", d.Day, err)
		}
		out = append(out, d)
	}
	return out, rows.Err()
}

// SignIn 执行每日签到。
//
// 同一天只能签一次：靠 user_sign_ins 的 (user_id, sign_date) 主键保证，
// 不做"先查后写"的两步判断 —— 那是并发下的经典重复领取漏洞。
func (s *Service) SignIn(ctx context.Context, userID int64) (SignInResult, error) {
	res := SignInResult{}
	err := s.DB.Tx(ctx, func(tx pgx.Tx) error {
		var dayIndex int
		var rewardRaw []byte
		if err := tx.QueryRow(ctx,
			`SELECT day_index, reward FROM sign_in_calendar
			 WHERE day_index = (
			   SELECT COALESCE(MAX(day_index), 0) + 1 FROM user_sign_ins WHERE user_id = $1)`,
			userID).Scan(&dayIndex, &rewardRaw); err != nil {
			if err == pgx.ErrNoRows {
				res.Already = true // 七天已签完或今天已签
				return nil
			}
			return err
		}

		// ⚠️ 第 89 轮：日界由 **Go** 单层决定，不再交给 PG 去折算。
		//
		// 原来这里写的是 `time.Now()`，落进 `sign_date DATE` 时由
		// **PG 会话时区**做 timestamptz -> date 的隐式转换。
		// 而「同一天只能签一次」完全靠 `PRIMARY KEY (user_id, sign_date)` 保证，
		// 所以「今天几号」这件事由**两个不同的层**各算一半：
		//
		//	每日任务：Go 的 `periodStart(now, "daily")`（用 `time.Local`）
		//	每日签到：PG 会话时区
		//
		// # 实测（本地环境）
		//
		//	PG session TimeZone = "Asia/Shanghai"   与 Go 本地 +08:00 一致
		//
		// 所以**当前部署下两者一致，这不是正在发生的 bug** ——
		// 但正确性**依赖一条没有任何东西钉住的配置**：
		// 只要 PG 的会话时区是 UTC（托管实例的常见默认值），
		// 签到的「天」就会在北京时间 **08:00** 翻页 ——
		// 同一个自然日可以签两次，而每日任务却还在前一天。
		//
		// # 另一层证据：这段代码原本是打算在 Go 里算的
		//
		// 改前有一行 `today := time.Now().Truncate(24 * time.Hour)`
		// 紧接着 `_ = today` —— 算完就扔。
		//
		// 而 `Truncate` 本身也是 **UTC 对齐**的（它按零时刻起算，忽略 Location），
		// 所以即使当初保留它，得到的也不是「本地零点」。
		// 意图是对的，实现两头都不对。
		//
		// 本轮改用 `periodStart(time.Now(), "daily")` —— 与每日任务同一个函数，
		// 同一个 Location 口径。这样「今天几号」只由一处决定，
		// 而 `sign_date` 存的就是那个 date（不再是 timestamptz）。
		//
		// ⚠️ 必须传**字符串**而不是 time.Time —— 这是本轮的第二步。
		//
		// 我第一版改成 `periodStart(time.Now(), "daily")` 就以为完事了，
		// 结果守卫立刻抓到它仍然是错的：
		//
		//	today = 2026-10-06 00:00:00 +08:00   （Go 本地零点）
		//	pgx 按 timestamptz 发送
		//	PG 用**会话时区**把它折成 DATE：
		//	  会话时区 = Asia/Shanghai -> 2026-10-06  ✅
		//	  会话时区 = UTC           -> 2026-10-05  ❌ 差一天！
		//
		// 也就是说只要传 time.Time，**会话时区仍然在决定日界** ——
		// 我只是把「PG 折算 now()」换成了「PG 折算 Go 的本地零点」，
		// 而后者同样会被会话时区二次偏移。
		//
		// 传 `YYYY-MM-DD` 字符串则完全绕开 timestamptz：
		// PG 只需要把它当字面量写进 DATE 列，不做任何时区换算。
		// 这才是「日界由 Go 单层决定」的真含义。
		today := periodStart(time.Now(), "daily").Format("2006-01-02")
		if _, err := tx.Exec(ctx,
			`INSERT INTO user_sign_ins (user_id, sign_date, day_index, reward) VALUES ($1,$2,$3,$4)`,
			userID, today, dayIndex, rewardRaw); err != nil {
			//
			// ⚠️ 第 115 轮：只有「唯一键冲突」才是「今日已签到」。
			// 修前把 INSERT 的**任何**错误（连接抖断、锁超时、
			// 磁盘满、死锁……）都报成「今日已签到」——
			// 事务回滚、奖励没发，用户却被告知「已签到」，
			// 以为当天领过了，不会重试，7 日奖励静默漏发。
			// 「如实报错」永远比「谎报已签」便宜。
			// 判据落在**错误码**上（23505 = unique_violation），
			// 而不是「跑了没成功就当已签」。
			var pgErr *pgconn.PgError
			if errors.As(err, &pgErr) && pgErr.Code == "23505" &&
				(pgErr.TableName == "user_sign_ins" || pgErr.ConstraintName == "user_sign_ins_pkey") {
				return fmt.Errorf("%w: 今日已签到", ErrForbidden)
			}
			return err
		}

		// ⚠️ 第 91 轮：签到必须推进 `signin` 指标的任务。
		//
		// # 缺陷：任务「每日签到」永远无法推进、永远领不到
		//
		// 种子里有这条任务：
		//
		//	{4, 1, "daily_signin", "完成每日签到", "daily", "signin", …}
		//
		// 而 `bumpTasks` 全仓库**只有一个调用点**（`game.go` 的结算路径），
		// 它传的 metric 只有 4 个：`kills` / `clears` / `reactions` / `max_stage`。
		// **没有 `signin`** —— 于是这条任务的 `progress` 永远是 0，
		// 玩家每天签到、每天都看到它卡在 0/1、永远领不到那 500 金币。
		//
		// # 为什么没被测出来
		//
		// 既有测试都是「造一行 progress 然后 ClaimTask」，
		// **绕过了 bump 这一步** —— 于是「bump 从不发生」这件事完全不可见。
		//
		// 与第 76 轮（周任务 `task_date` 写死 daily）同族：
		// 那次是**找错了行**，这次是**根本没人写那一行**。
		//
		// 守卫：`task_metric_coverage_test.go` —— 种子里出现的每个 metric
		// 都必须在某个 `bumpTasks` 调用点出现。
		if err := s.bumpTasks(ctx, tx, userID,
			map[string]int64{"signin": 1}, time.Now()); err != nil {
			return err
		}

		var reward map[string]int
		if err := json.Unmarshal(rewardRaw, &reward); err != nil {
			return err
		}
		deltas := map[string]int64{}
		for k, v := range reward {
			deltas[k] = int64(v)
		}
		if err := s.grantWallet(ctx, tx, userID, deltas, "signin", int64(dayIndex)); err != nil {
			return err
		}
		res.DayIndex = dayIndex
		res.Reward = reward
		return nil
	})
	if err != nil {
		return SignInResult{}, err
	}
	w, err := s.LoadWallet(ctx, userID)
	if err != nil {
		return SignInResult{}, err
	}
	res.Wallet = w
	return res, nil
}

// SignInStatus 是签到页「初始展示」回读到的签到状态（第 141 轮）。
type SignInStatus struct {
	// 本周期已签天数（user_sign_ins 的 MAX(day_index)，没签过为 0）。
	// 客户端用它点亮「已签」天并标出「今天」该是第几天。
	ClaimedCount int `json:"claimed_count"`
	// 今天是否已签（按 SignIn 同一日界口径）。
	SignedToday bool `json:"signed_today"`
	// 今天还能不能签：今日未签 且 七日周期未满。
	CanSign bool `json:"can_sign"`
}

// SignInStatus 返回玩家当前签到状态，供签到页挂载时做**权威**初始展示。
//
// ⚠️ 第 141 轮：修前小程序签到页把 claimedCount=0 / canSign=true 写死，
// 挂载后从不回读 —— 玩家当天已签到（甚至周期已签完）后重进页面，
// 仍显示「今天还没签 / 可领取」，点按钮才吃一个「今日已签到」的错。
// 「今天」的日界口径与 SignIn 完全相同（periodStart(now,"daily") 的 DATE 串），
// 否则两处对「今天几号」的判断会裂开。
func (s *Service) SignInStatus(ctx context.Context, userID int64) (SignInStatus, error) {
	today := periodStart(time.Now(), "daily").Format("2006-01-02")
	var claimed int
	var signedToday bool
	if err := s.pool.QueryRow(ctx, `
		SELECT COALESCE(MAX(day_index), 0),
		       EXISTS (SELECT 1 FROM user_sign_ins WHERE user_id = $1 AND sign_date = $2)
		FROM user_sign_ins
		WHERE user_id = $1`, userID, today).Scan(&claimed, &signedToday); err != nil {
		return SignInStatus{}, fmt.Errorf("load signin status: %w", err)
	}

	// 周期是否签满：下一行（claimed+1）在日历里不存在即满（与 SignIn 的
	// 「MAX+1 查不到行 → Already」同口径，避免本地再数一个 7）。
	var cycleLen int
	if err := s.pool.QueryRow(ctx, `SELECT COUNT(*) FROM sign_in_calendar`).Scan(&cycleLen); err != nil {
		return SignInStatus{}, fmt.Errorf("count signin calendar: %w", err)
	}
	return SignInStatus{ClaimedCount: claimed, SignedToday: signedToday, CanSign: !signedToday && claimed < cycleLen}, nil
}

// Redeem 兑换兑换码。
func (s *Service) Redeem(ctx context.Context, userID int64, code string) (map[string]int, error) {
	var out map[string]int
	err := s.DB.Tx(ctx, func(tx pgx.Tx) error {
		var codeID int
		var rewardRaw []byte
		if err := tx.QueryRow(ctx,
			`SELECT id, reward FROM redeem_codes
			 WHERE code = $1 AND enabled AND (expires_at IS NULL OR expires_at > now())
			   AND used_count < max_uses`, code).Scan(&codeID, &rewardRaw); err != nil {
			if err == pgx.ErrNoRows {
				return fmt.Errorf("%w: 兑换码无效或已用尽", ErrBadInput)
			}
			return err
		}
		// 先占位：唯一键保证一个用户一个码只能用一次
		tag, err := tx.Exec(ctx,
			`INSERT INTO redeem_usages (code_id, user_id) VALUES ($1,$2)
			 ON CONFLICT (code_id, user_id) DO NOTHING`, codeID, userID)
		if err != nil {
			return err
		}
		if tag.RowsAffected() == 0 {
			return fmt.Errorf("%w: 你已经使用过该兑换码", ErrForbidden)
		}
		// ⚠️ 条件 UPDATE + 检查 RowsAffected 才是互斥点。
		//
		// 上面的 `used_count < max_uses` 只是前置 SELECT，在并发下是**无效预检**：
		// N 个并发事务全部在对方提交前读到 used_count=0，于是全部通过。
		// 旧实现是无条件 `used_count = used_count + 1 WHERE id = $1`，
		// 于是 used_count 一路冲过 max_uses，最终靠 redeem_codes 上的
		// CHECK (used_count <= max_uses) 整笔回滚。
		//
		// 资源上没超发，但代价有三个：
		//  1) 客户端收到 SQLSTATE 23514（数据库约束名），而不是"兑换码已被抢完"
		//  2) 一旦有人迁移掉那条 CHECK（它只是"恰好"存在，不是并发设计），立刻变成无限超发
		//  3) 报错信息泄露内部表结构
		//
		// 与 Buy 的 FOR UPDATE、ClaimTask 的 claimed_at IS NULL 是同一个模式：
		// 让数据库的条件写充当互斥量，用 RowsAffected 判断是否真的抢到。
		tag, err = tx.Exec(ctx,
			`UPDATE redeem_codes SET used_count = used_count + 1
			 WHERE id = $1 AND used_count < max_uses`, codeID)
		if err != nil {
			return err
		}
		if tag.RowsAffected() == 0 {
			return fmt.Errorf("%w: 兑换码已被抢完", ErrBadInput)
		}
		var reward map[string]int
		if err := json.Unmarshal(rewardRaw, &reward); err != nil {
			return err
		}
		deltas := map[string]int64{}
		for k, v := range reward {
			deltas[k] = int64(v)
		}
		if err := s.grantWallet(ctx, tx, userID, deltas, "redeem", int64(codeID)); err != nil {
			return err
		}
		out = reward
		return nil
	})
	return out, err
}

// ShopItemView 是商城条目。
type ShopItemView struct {
	ID       int            `json:"id"`
	Code     string         `json:"code"`
	Name     string         `json:"name"`
	Category string         `json:"category"`
	Price    map[string]int `json:"price"`
	Payload  map[string]int `json:"payload"`
	Limit    int            `json:"limit_per_day"`
	Bought   int            `json:"bought_today"`
}

// LoadShop 返回商城，附带玩家今日已购次数。
func (s *Service) LoadShop(ctx context.Context, userID int64) ([]ShopItemView, error) {
	// ⚠️ 第 97 轮：`purchase_date` 改由 **Go** 决定，与任务/签到同一个口径。
	//
	// 原来三处都用 `CURRENT_DATE`，由 **PG 会话时区**折算 ——
	// 商城**内部**因此是自洽的，但它与每日任务的日界**不是同一套**：
	//
	//	每日任务：Go 的 `periodStart(now, "daily")`（用 `time.Local`）
	//	商城限购：PG 会话时区
	//
	// 第 90 轮把 PG 会话时区钉成 `Asia/Shanghai`，但**没有钉 Go 那一侧** ——
	// 容器里 `time.Local` 常常是 UTC。
	// 两者不一致时，商城的「今日」与任务的「今日」在不同时刻翻页：
	// 玩家早上 8 点看到任务已刷新，商城却还显示昨天的限购次数。
	//
	// 传**日期字面量**（与第 89 轮签到同一手法）让日界只由一处决定。
	today := periodStart(time.Now(), "daily").Format("2006-01-02")
	rows, err := s.pool.Query(ctx, `
		SELECT si.id, si.code, si.name, si.category, si.price, si.payload, si.limit_per_day,
		       (SELECT COUNT(*) FROM user_purchases up
		         WHERE up.user_id = $1 AND up.item_id = si.id
		           AND up.purchase_date = $2)
		FROM shop_items si WHERE si.enabled ORDER BY si.sort_order`, userID, today)
	if err != nil {
		return nil, fmt.Errorf("load shop: %w", err)
	}
	defer rows.Close()

	var out []ShopItemView
	for rows.Next() {
		var v ShopItemView
		var priceRaw, payloadRaw []byte
		if err := rows.Scan(&v.ID, &v.Code, &v.Name, &v.Category, &priceRaw, &payloadRaw, &v.Limit, &v.Bought); err != nil {
			return nil, err
		}
		// 第 148 轮：fail-loud（AGENTS #11）。price/payload 是 jsonb 经济字段，
		// `_ =` 会把坏数据静默吞成空 map —— 商品显示成「0 金 / 无内容」。
		// 购买路径（Buy）本就 fail-loud，这里列表读侧也应同口径报错，
		// 让坏商品在后台/客户端可见，而不是无声显示成「免费/空包」。
		if err := json.Unmarshal(priceRaw, &v.Price); err != nil {
			return nil, fmt.Errorf("load shop: 商品 %d 的 price 非法 jsonb: %w", v.ID, err)
		}
		if err := json.Unmarshal(payloadRaw, &v.Payload); err != nil {
			return nil, fmt.Errorf("load shop: 商品 %d 的 payload 非法 jsonb: %w", v.ID, err)
		}
		out = append(out, v)
	}
	return out, rows.Err()
}

// Buy 购买商城商品。
func (s *Service) Buy(ctx context.Context, userID, itemID int64) (map[string]int, error) {
	// 第 97 轮：与 LoadShop 同一个口径（Go 的日期字面量），见那里的说明。
	today := periodStart(time.Now(), "daily").Format("2006-01-02")
	var out map[string]int
	err := s.DB.Tx(ctx, func(tx pgx.Tx) error {
		var priceRaw, payloadRaw []byte
		var limit int
		if err := tx.QueryRow(ctx,
			`SELECT price, payload, limit_per_day FROM shop_items WHERE id = $1 AND enabled`,
			itemID).Scan(&priceRaw, &payloadRaw, &limit); err != nil {
			if err == pgx.ErrNoRows {
				return fmt.Errorf("%w: 商品 %d", ErrNotFound, itemID)
			}
			return err
		}

		if limit > 0 {
			// 先锁住该用户的钱包行，再做 COUNT。
			//
			// 「SELECT COUNT(*) 再 INSERT」在并发下是**完全无效**的限购：
			// N 个并发事务全部在对方 INSERT 之前读到 bought=0，于是全部放行。
			// shop_firstpay 价格是 {coin: 0}（首充 0 元购），
			// 并发打 20 个请求就能白拿 20 份 gem/coin/energy。
			//
			// 锁 user_wallets 是因为它必然会被本事务后续的 grantWallet 更新，
			// 提前加锁把同一用户的购买串行化，且不引入新的锁顺序分支
			// （所有路径都是先钱包后 purchases）。
			var lockedUID int64
			if err := tx.QueryRow(ctx,
				`SELECT user_id FROM user_wallets WHERE user_id = $1 FOR UPDATE`,
				userID).Scan(&lockedUID); err != nil {
				if errors.Is(err, pgx.ErrNoRows) {
					return fmt.Errorf("%w: 钱包不存在", ErrNotFound)
				}
				return err
			}
			var bought int
			if err := tx.QueryRow(ctx,
				`SELECT COUNT(*) FROM user_purchases
				 WHERE user_id = $1 AND item_id = $2 AND purchase_date = $3`,
				userID, itemID, today).Scan(&bought); err != nil {
				return err
			}
			if bought >= limit {
				return fmt.Errorf("%w: 今日购买次数已达上限 %d", ErrForbidden, limit)
			}
		}

		var price, payload map[string]int
		if err := json.Unmarshal(priceRaw, &price); err != nil {
			return err
		}
		if err := json.Unmarshal(payloadRaw, &payload); err != nil {
			return err
		}

		// 先扣钱再发货，扣钱失败则整体回滚
		deduct := map[string]int64{}
		for k, v := range price {
			if v > 0 {
				deduct[k] = int64(-v)
			}
		}
		if err := s.grantWallet(ctx, tx, userID, deduct, "shop_buy", itemID); err != nil {
			return err
		}

		grant := map[string]int64{}
		for k, v := range payload {
			if v <= 0 {
				continue
			}
			// ⚠️ `gem_instance` 不是货币：它要造一颗**带随机词条的宝石实体**，
			// 走 grantWallet 会被判成「未知货币」而整笔回滚。
			// 宝石此前拿不到，就是因为没有任何路径能发出它。
			if k == gemInstanceKey {
				if _, _, err := s.mintGem(ctx, tx, userID); err != nil {
					return err
				}
				continue
			}
			// 2026-10-21（P4-1）：`equipment_<id>` 也不是货币 ——
			// 它要发一件**具体装备**到 `user_equipment`。
			//
			// 此前 18 件装备里 16 件玩家永远拿不到：既没有获取途径，
			// 商城里也没有对应的商品。装备的**战斗效果**一直存在
			// （base_armor / base_bonus_pct 会被读进攻方），
			// 但玩家碰不到它 —— 那等于没有（KNOWN_LIMITS 第 25 条）。
			if eqID, ok := equipmentPayloadID(k); ok {
				// ⚠️ 2026-10-21（二轮审计 P2）：多份要**显式拒绝**。
				//
				// `user_equipment` 的主键是 `(user_id, equipment_id)` —— 一个人
				// 不能持有同一件装备的两份。所以第 2、3 次 INSERT 全是
				// `ON CONFLICT DO NOTHING` 的静默 no-op：玩家付了 3 份的钱，
				// 只拿到 1 件，而且**不报任何错**。
				//
				// 装备**没有随机性**（不像宝石要 roll 词条），所以「多份」
				// 是一个**产品决策**而不是数据模型问题 —— 在这里显式拒绝，
				// 让运营改 payload 时立刻知道「这件商品不支持多份」。
				if v > 1 {
					return fmt.Errorf("%w: 装备不支持多份（同一部位只能装一件），请把数量设为 1",
						ErrBadInput)
				}
				if err := mintEquipment(ctx, tx, userID, eqID); err != nil {
					return err
				}
				continue
			}
			grant[k] = int64(v)
		}
		if len(grant) > 0 {
			if err := s.grantWallet(ctx, tx, userID, grant, "shop_buy", itemID); err != nil {
				return err
			}
		}
		if _, err := tx.Exec(ctx,
			`INSERT INTO user_purchases (user_id, item_id, purchase_date, price) VALUES ($1,$2,$3,$4)`,
			userID, itemID, today, priceRaw); err != nil {
			return err
		}
		out = payload
		return nil
	})
	return out, err
}

// --- 验真（I-6） ---

// VerifyInput 是验真请求。
type VerifyInput struct {
	BattleID   int64  `json:"battle_id"`
	ReplayHash string `json:"replay_hash"`
}

// VerifyResult 是验真结果。
type VerifyResult struct {
	BattleID int64  `json:"battle_id"`
	Expected string `json:"expected_hash"`
	Actual   string `json:"actual_hash"`
	Matched  bool   `json:"matched"`
	LevelID  int    `json:"level_id"`
	// Seed 必须是字符串：int64 超过 2^53 后 JSON number 在 JS 侧解析即失精，
	// 客户端据此重放会拿到不同的种子，算出的哈希必然对不上（I-6 失效）。
	Seed       string `json:"seed"`
	RecordedAt string `json:"recorded_at"`
}

// VerifyReplay 已迁到 verification.go。
//
// 迁移原因：要新增运营侧验真（AdminVerifyReplay），而两条路径必须在
// 「读哪些列、怎么比、往哪张表写」每一个细节上一致 —— 两份实现只要有一处漂移，
// 就会出现「玩家验真和运营验真结论不同」，而这种 bug 极难发现。
// 所以抽成共用的 compareAndRecord，两侧只差「验真人是谁」。

// ReplayInfo 返回复现一局所需的全部信息（供客户端本地重放）。
type ReplayInfo struct {
	BattleID int64 `json:"battle_id"`
	LevelID  int   `json:"level_id"`
	// Seed 用字符串下发：int64 超过 2^53 后 JSON number 在 JS 侧解析即失精，
	// 客户端拿到的种子与库里那个不是同一个数 → 重放必然算不出相同哈希。
	Seed       string    `json:"seed"`
	Level      LevelResp `json:"level"`
	ReplayHash string    `json:"expected_hash"`
	CreatedAt  string    `json:"created_at"`
	// Build 是该局结算时冻结的构筑快照。
	//
	// ⚠️ 这个字段是 I-6 能否成立的关键：回放哈希由「关卡 + 种子 + 构筑」共同决定，
	// 只给关卡和种子而不给构筑，客户端重放出来的哈希必然对不上，
	// 于是所有正常对局都会被判为"不一致"—— 验真机制就成了纯噪音。
	Build map[string]any `json:"build"`
	// CardPicks 每波选中的手牌索引（-1 = 跳过）。
	//
	// ⚠️ 没有它就复现不出原局：选牌会改变后续战斗（技能升格/属性加成/机制词条）。
	// 服务端存的是逗号分隔的紧凑文本，这里解析成数组下发。
	CardPicks []int `json:"card_picks"`
}

// parseCardPicks 解析逗号分隔的选牌序列。空串表示旧记录没有该字段。
func parseCardPicks(raw string) []int {
	if strings.TrimSpace(raw) == "" {
		return nil
	}
	parts := strings.Split(raw, ",")
	out := make([]int, 0, len(parts))
	for _, p := range parts {
		n, err := strconv.Atoi(strings.TrimSpace(p))
		if err != nil {
			// 容忍脏数据：跳过单个坏值，而不是让整条记录不可用
			continue
		}
		out = append(out, n)
	}
	return out
}

// GetReplay 返回复现信息。
//
// # ⚠️ 第 75 轮：补上归属校验
//
// 原来只有 `WHERE br.id = $1`。`battle_records.id` 是连续 BIGSERIAL，
// 任何注册用户都能遍历，于是任何人都能读走任意一局的
// **完整构筑快照** —— 装备、词缀、技能等级，全都是别人的。
//
// 这条路径比 `VerifyReplay` 泄露得更多：它多返回一个 `build_snapshot`，
// 而 `delete(r.Build, "settle_input")` 只剥掉了结算报文，
// **构筑本身还在**。照抄别人的满级装备 + 关卡 + 种子 + card_picks
// 就是一份「不需要自己打出来的高星战报蓝本」。
//
// # 为什么这不破坏验真的设计
//
// 下一行的注释说「这些数据公开是设计使然（否则无人能验真）」——
// 那句话说的是**数据内容**公开，不是**谁的**战报公开。
// 你验证自己的对局本来就不需要读别人的。
func (s *Service) GetReplay(ctx context.Context, userID, battleID int64) (ReplayInfo, error) {
	return s.getReplay(ctx, userID, battleID, true)
}

// AdminGetReplay 是运营侧读任意战报的复现信息（第 75 轮）。
//
// # 为什么单独一个方法而不是给 GetReplay 加个 bool 参数
//
// 与 `verification.go` 里的 `verifier{isAdmin}` 同一思路：
// **归属校验是权限，不是行为开关**。把它做成参数就意味着
// 调用方要自己记得传对，而漏传时是**静默放行**——
// 那正是本轮修掉的那个洞。
//
// 两条路径共用 `getReplay`，所以「读哪些列、怎么剥 settle_input」
// 不会漂。README 记的正是这类漂移：
// 「两份实现只要有一处漂移，就会出现玩家验真和运营验真结论不同」。
func (s *Service) AdminGetReplay(ctx context.Context, battleID int64) (ReplayInfo, error) {
	return s.getReplay(ctx, 0, battleID, false)
}

func (s *Service) getReplay(ctx context.Context, userID, battleID int64, enforceOwner bool) (ReplayInfo, error) {
	var r ReplayInfo
	var createdAt time.Time
	var seed int64
	var cardPicks string
	sql := `SELECT br.level_id, COALESCE(bt.seed, 0), br.replay_hash, br.created_at,
	              br.build_snapshot, COALESCE(br.card_picks, '')
	       FROM battle_records br
	       LEFT JOIN battle_tokens bt ON bt.id = br.battle_token_id
	       WHERE br.id = $1`
	args := []any{battleID}
	if enforceOwner {
		sql += ` AND br.user_id = $2`
		args = append(args, userID)
	}
	err := s.pool.QueryRow(ctx, sql, args...).
		Scan(&r.LevelID, &seed, &r.ReplayHash, &createdAt, &r.Build, &cardPicks)
	if err != nil {
		// 「不存在」与「不是你的」返回**同一个**错误（第 75 轮）。
		//
		// 分开报错就等于一个预言机：`ErrNotFound` vs `ErrForbidden`
		// 让人能二分出「某个 id 是否存在」，在连续 BIGSERIAL 上是 O(1) 的信息。
		//
		// ⚠️ 原来的代码把**任何** err 都报成 ErrNotFound，已经是安全的默认；
		// 本轮只是确认了它，不改它。
		return ReplayInfo{}, fmt.Errorf("%w: battle %d", ErrNotFound, battleID)
	}
	r.BattleID = battleID
	r.Seed = strconv.FormatInt(seed, 10)
	r.CardPicks = parseCardPicks(cardPicks)
	r.CreatedAt = createdAt.UTC().Format(timeFormat)
	r.Level = NewLevelResp(domain.GenerateLevel(r.LevelID))

	// ⚠️ 从玩家侧响应里剥离 settle_input。
	//
	// 验真（I-6）只需要 level + seed + build + card_picks ——
	// 这些都是服务端权威数据，公开是设计使然（否则无人能验真）。
	// 但 settle_input 是**客户端上报的原始结算报文**，对验真毫无用处，
	// 却是伪造高分战报的最佳蓝本：照抄一个高星玩家的
	// kills / reactions / duration_ms 就能构造出"合法"的上报。
	//
	// battle_records.id 是连续 BIGSERIAL，任何注册用户都能遍历，
	// 所以这不是理论风险。
	delete(r.Build, "settle_input")
	return r, nil
}

// parseInt64 宽容地把字符串/数字解析为 int64。
func parseInt64(v string) (int64, error) {
	return strconv.ParseInt(v, 10, 64)
}
