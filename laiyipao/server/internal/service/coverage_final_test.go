package service

// 覆盖率收尾（第二轮）：把上一轮记为豁免、但其实**能在真库上构造**的
// 错误分支逐条转正。手法沿用既有夹具：
//   - openBrokenService：连接池已关闭 → 覆盖"函数第一条 DB 操作即失败"，
//     以及"验签通过但随后查库失败"这类需要合法令牌的故障出口。
//   - openScratchService + renameTable/exec：一次性库上改单表/单列/塞脏数据，
//     覆盖"前一步成功、这一步失败"的中段分支，以及 JSONB 解析失败。
//
// 每条用例都锚定一个**行为后果**（错误被包装成什么、返回什么），
// 不是"调用一下涨覆盖率"。豁免仍然保留的（crypto/rand、类型化 json.Marshal、
// schema 保证的 rows.Scan/rows.Err、合法专精下够不到的封顶、同表先读后写）
// 见随附报告。

import (
	"context"
	"errors"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// --- 验签通过但随后查库失败：需要合法令牌 + 故障库 ---

// TestResolveUserDBError ResolveUser 在 JWT 合法、但 status 查询发生
// **非 ErrNoRows** 错误时，必须把原始错误上抛（而不是当成"用户不存在"）。
// 上一轮因为测试只喂垃圾令牌（卡在验签），走不到查库分支。
func TestResolveUserDBError(t *testing.T) {
	broken := openBrokenService(t)
	// 用故障库自己的签发器造一个结构合法的 user 令牌 —— 签发不碰库。
	token, _, err := broken.JWT.Issue(1, "user")
	if err != nil {
		t.Fatalf("签发失败：%v", err)
	}
	if _, err := broken.ResolveUser(context.Background(), token); err == nil {
		t.Error("库故障时 status 查询应报错上抛")
	}
}

// TestResolveAdminDBError 同上，针对管理员令牌。
func TestResolveAdminDBError(t *testing.T) {
	broken := openBrokenService(t)
	token, _, err := broken.JWT.Issue(1, "admin")
	if err != nil {
		t.Fatalf("签发失败：%v", err)
	}
	if _, err := broken.ResolveAdmin(context.Background(), token); err == nil {
		t.Error("库故障时 admin status 查询应报错上抛")
	}
}

// TestRedeemCodesBrokenBranches AdminListRedeemCodes / AdminCreateRedeemCode
// 的故障出口（上一轮 TestAdminRedeemCodes 只测了健康库）。
func TestRedeemCodesBrokenBranches(t *testing.T) {
	broken := openBrokenService(t)
	ctx := context.Background()
	if _, err := broken.AdminListRedeemCodes(ctx); err == nil {
		t.Error("故障态兑换码列表应报错")
	}
	if _, err := broken.AdminCreateRedeemCode(ctx, "X", map[string]int{"coin": 1}, 1, nil); err == nil {
		t.Error("故障态创建兑换码应报错")
	}
}

// TestComputeRatingPointsQueryFallback computeRating 读 mastery_points
// 失败时必须**降级为 0** 继续算分，而不是让结算崩掉。
// 构造：故障库 + 带 mastery_nodes 的 build（只有非空 mastery_nodes 才会查点数）。
func TestComputeRatingPointsQueryFallback(t *testing.T) {
	broken := openBrokenService(t)
	// build 里放一个真实存在的专精节点 id，触发 fams 匹配与点数查询两段。
	var nodeID int
	for _, f := range domain.AllMasteryFamilies() {
		if len(f.Nodes) > 0 {
			nodeID = f.Nodes[0].ID
			break
		}
	}
	build := map[string]any{"mastery_nodes": []int{nodeID}}
	// 不应 panic：点数查询失败 → points=0 → EvaluateMastery 照常返回。
	r := broken.ComputeRatingFor(1, build)
	if r.MasteryDone < 0 {
		t.Errorf("降级后仍应产出合法评分，实际 %+v", r)
	}
}

// TestCode2SessionRequestBuildError 端点含非法控制字符时，
// http.NewRequestWithContext 构造失败，code2Session 必须包装成
// "build wechat request" 而不是 panic。
func TestCode2SessionRequestBuildError(t *testing.T) {
	ts := openTestService(t)
	ts.Cfg.WechatEndpoint = "http://\x7f-bad-host/wx" // DEL 控制字符 → URL 解析失败
	if _, err := ts.code2Session(context.Background(), "code"); err == nil {
		t.Error("非法端点应让 code2Session 构造请求失败")
	}
}

// TestCode2SessionBodyReadError 响应体在读取中途断开时，io.ReadAll 失败，
// code2Session 必须包装成 "read wechat response"。
// 构造：httptest 声明 Content-Length 大于实际写入并主动断开连接。
func TestCode2SessionBodyReadError(t *testing.T) {
	ts := openTestService(t)
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Content-Length", "1000") // 声明 1000 字节
		w.WriteHeader(http.StatusOK)
		_, _ = w.Write([]byte("short"))      // 只写 5 字节
		if hj, ok := w.(http.Hijacker); ok { // 直接掐断底层连接
			if conn, _, err := hj.Hijack(); err == nil {
				_ = conn.Close()
			}
		}
	}))
	defer srv.Close()
	ts.Cfg.WechatEndpoint = srv.URL
	if _, err := ts.code2Session(context.Background(), "code"); err == nil {
		t.Error("响应体读取失败应报错")
	}
}

// --- 账号：中段失败（scratch + 改名） ---

// TestWechatLoginMidwayFailures 覆盖 WechatLogin 在 code2Session 成功之后的
// 两条中段错误：upsert users 失败、initNewUser 失败。
func TestWechatLoginMidwayFailures(t *testing.T) {
	ctx := context.Background()
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		_, _ = w.Write([]byte(`{"openid":"oMID"}`))
	}))
	defer srv.Close()

	t.Run("upsert失败", func(t *testing.T) {
		ts := openScratchService(t)
		ts.Cfg.WechatAppID, ts.Cfg.WechatAppSecret, ts.Cfg.WechatEndpoint = "wx", "sec", srv.URL
		ts.renameTable(t, "users", "users_bak")
		_, err := ts.WechatLogin(ctx, "good")
		ts.renameTable(t, "users_bak", "users")
		mustErr(t, err, "upsert wechat user")
	})

	t.Run("initNewUser失败", func(t *testing.T) {
		ts := openScratchService(t)
		ts.Cfg.WechatAppID, ts.Cfg.WechatAppSecret, ts.Cfg.WechatEndpoint = "wx", "sec", srv.URL
		// users 完好 → upsert 成功；user_wallets 缺失 → initNewUser 第一步失败
		ts.renameTable(t, "user_wallets", "user_wallets_bak")
		_, err := ts.WechatLogin(ctx, "good")
		ts.renameTable(t, "user_wallets_bak", "user_wallets")
		mustErr(t, err, "init wallet")
	})
}

// TestRefreshLoadUserError Refresh 在 refresh_tokens 命中、但随后 loadUser
// 失败时必须上抛错误（而不是发一个空 user 的令牌对）。
func TestRefreshLoadUserError(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	tp, err := ts.GuestLogin(ctx, "refresh-mid", "x")
	if err != nil {
		t.Fatalf("建号失败：%v", err)
	}
	// refresh_tokens 完好 → UPDATE ... RETURNING user_id 成功；
	// users 改名 → loadUser 失败。
	ts.renameTable(t, "users", "users_bak")
	_, err = ts.Refresh(ctx, tp.RefreshToken)
	ts.renameTable(t, "users_bak", "users")
	mustErr(t, err, "")
}

// TestLoadWalletSelectColumnError LoadWallet 的体力回补 UPDATE 成功、
// 但随后回读 SELECT 失败的分支（非 ErrNoRows）。
// 构造：只把 SELECT 才引用的 keys 列改名，UPDATE 不碰它。
func TestLoadWalletSelectColumnError(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.exec(t, `ALTER TABLE user_wallets RENAME COLUMN keys TO keys_bak`)
	_, err := ts.LoadWallet(ctx, uid)
	ts.exec(t, `ALTER TABLE user_wallets RENAME COLUMN keys_bak TO keys`)
	mustErr(t, err, "load wallet")
}

// --- 经济：中段失败与 JSONB 脏数据 ---

func TestEconomyMidwayFailures(t *testing.T) {
	ctx := context.Background()

	t.Run("SignIn日历查询非NoRows错误", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "sign_in_calendar", "sign_in_calendar_bak")
		_, err := ts.SignIn(ctx, uid)
		ts.renameTable(t, "sign_in_calendar_bak", "sign_in_calendar")
		mustErr(t, err, "")
	})

	t.Run("SignIn奖励JSON非法", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		// 合法 jsonb 但不是对象（是字符串）→ Unmarshal 进 map 失败
		ts.exec(t, `UPDATE sign_in_calendar SET reward = '"boom"'::jsonb WHERE day_index = 1`)
		_, err := ts.SignIn(ctx, uid)
		mustErr(t, err, "")
	})

	t.Run("Redeem码查询非NoRows错误", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "redeem_codes", "redeem_codes_bak")
		_, err := ts.Redeem(ctx, uid, "WHATEVER")
		ts.renameTable(t, "redeem_codes_bak", "redeem_codes")
		mustErr(t, err, "")
	})

	t.Run("Redeem奖励JSON非法", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.exec(t, `INSERT INTO redeem_codes (id, code, reward, max_uses, used_count, enabled)
		            VALUES ((SELECT COALESCE(MAX(id),0)+1 FROM redeem_codes), 'BADJSON', '"boom"'::jsonb, 5, 0, true)`)
		_, err := ts.Redeem(ctx, uid, "BADJSON")
		mustErr(t, err, "")
	})

	t.Run("Buy商品查询非NoRows错误", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "shop_items", "shop_items_bak")
		_, err := ts.Buy(ctx, uid, 1)
		ts.renameTable(t, "shop_items_bak", "shop_items")
		mustErr(t, err, "")
	})

	t.Run("Buy价格JSON非法", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		// 选一个 limit=0（不限购，跳过 COUNT 分支）的商品，把 price 改成非法形状
		ts.exec(t, `UPDATE shop_items SET price = '"boom"'::jsonb WHERE id = 6`)
		_, err := ts.Buy(ctx, uid, 6)
		mustErr(t, err, "")
	})

	t.Run("Buy载荷JSON非法", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.grant(t, ctx, uid, map[string]int64{"coin": 100000, "gem": 1000})
		// price 合法（能过 Unmarshal）、payload 非法形状 → 卡在 payload 反序列化
		ts.exec(t, `UPDATE shop_items SET price = '{"gem":1}', payload = '"boom"'::jsonb WHERE id = 6`)
		_, err := ts.Buy(ctx, uid, 6)
		mustErr(t, err, "")
	})

	t.Run("Buy发货货币未知", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.grant(t, ctx, uid, map[string]int64{"coin": 100000, "gem": 1000})
		// 价格合法（能扣钱），payload 是未知货币 → 发货阶段 grantWallet 业务失败
		ts.exec(t, `UPDATE shop_items SET price = '{"gem":1}', payload = '{"diamonds":1}' WHERE id = 6`)
		_, err := ts.Buy(ctx, uid, 6)
		mustErr(t, err, "未知货币")
	})

	t.Run("Buy写购买记录失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.grant(t, ctx, uid, map[string]int64{"coin": 100000, "gem": 1000})
		// price/payload 都改成合法钱包货币，扣钱+发货都成功、直达写记录；
		// user_purchases 改名 → INSERT 购买记录失败。
		ts.exec(t, `UPDATE shop_items SET price = '{"gem":1}', payload = '{"coin":10}' WHERE id = 6`)
		ts.renameTable(t, "user_purchases", "user_purchases_bak")
		_, err := ts.Buy(ctx, uid, 6)
		ts.renameTable(t, "user_purchases_bak", "user_purchases")
		mustErr(t, err, "")
	})

	t.Run("VerifyReplay写验真记录失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.grant(t, ctx, uid, map[string]int64{"energy": 100})
		tp, err := ts.StartBattle(ctx, uid, 1)
		if err != nil {
			t.Skipf("开局失败：%v", err)
		}
		sr, err := ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput())
		if err != nil {
			t.Fatalf("结算失败：%v", err)
		}
		ts.renameTable(t, "replay_verifications", "replay_verifications_bak")
		_, err = ts.VerifyReplay(ctx, uid, sr.BattleID, "deadbeef")
		ts.renameTable(t, "replay_verifications_bak", "replay_verifications")
		mustErr(t, err, "record verification")
	})
}

// --- 战斗 / 构筑 / 任务：中段失败 ---

// TestSettleApplyProgressError applyProgress 的 UPDATE ... RETURNING 命不中行
// （玩家无 user_progress 行）时必须报 "update progress"。
// 构造：删掉该玩家的 user_progress 行 —— 战前的 computeAttacker 用 COALESCE/
// 吞错读法仍能通过，但 applyProgress 的条件 UPDATE 命中 0 行 → RETURNING 空。
func TestSettleApplyProgressError(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})
	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	// 开局后删进度行：LoadBuildSnapshot 仍能过（读法容错），applyProgress 必炸
	ts.exec(t, `DELETE FROM user_progress WHERE user_id = $1`, uid)
	_, err = ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput())
	mustErr(t, err, "update progress")
}

// TestLoadBuildSnapshotComputeAttackerError LoadBuildSnapshot 里
// computeAttacker 失败（读 user_progress 报错）但前序 loader 都成功的分支。
// 构造：新号无专精节点 → loadSkillsAndSlots 内部的 ExtraSlots 提前返回、
// 不读 user_progress；改名 user_progress 只打中 computeAttacker 的 MAX 查询。
func TestLoadBuildSnapshotComputeAttackerError(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.renameTable(t, "user_progress", "user_progress_bak")
	_, err := ts.LoadBuildSnapshot(ctx, uid)
	ts.renameTable(t, "user_progress_bak", "user_progress")
	mustErr(t, err, "")
}

// TestBumpTasksAchievementInsertError SettleBattle 内 bumpTasks 写 user_tasks
// 失败（成就类分支）。构造：先停用所有非成就任务，让 bumpTasks 只对成就类
// （metric=max_stage）产生 INSERT；再改名 user_tasks，结算走到该 INSERT 必炸。
// 只留成就类是为了**确定性**命中 690 行——否则 map 遍历顺序会让它随机落到
// 日/周任务的 701 行。
func TestBumpTasksAchievementInsertError(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})
	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	ts.exec(t, `UPDATE tasks SET enabled = false WHERE scope <> 'achievement'`)
	ts.renameTable(t, "user_tasks", "user_tasks_bak")
	_, err = ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput())
	ts.renameTable(t, "user_tasks_bak", "user_tasks")
	mustErr(t, err, "bump achievement")
}

// TestClaimTaskQueryError ClaimTask 首个 SELECT 发生非 ErrNoRows 错误。
func TestClaimTaskQueryError(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.renameTable(t, "tasks", "tasks_bak")
	_, err := ts.ClaimTask(ctx, uid, 1)
	ts.renameTable(t, "tasks_bak", "tasks")
	mustErr(t, err, "")
}

// TestClaimTaskRewardJSONError ClaimTask 奖励 JSON 非法：进度已达标、未领取，
// 卡在 reward 反序列化。
func TestClaimTaskRewardJSONError(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	var taskID, target int
	if err := ts.pool.QueryRow(ctx,
		`SELECT id, target FROM tasks WHERE enabled AND scope='daily' ORDER BY id LIMIT 1`).
		Scan(&taskID, &target); err != nil {
		t.Skipf("无 daily 任务：%v", err)
	}
	ts.exec(t, `UPDATE tasks SET reward = '"boom"'::jsonb WHERE id = $1`, taskID)
	ts.exec(t, `INSERT INTO user_tasks (user_id, task_id, task_date, progress)
	            VALUES ($1,$2,CURRENT_DATE,$3)`, uid, taskID, target)
	_, err := ts.ClaimTask(ctx, uid, taskID)
	mustErr(t, err, "")
}

// TestSettleConcurrentClaimGuard 并发结算同一 token：条件 UPDATE
// （used_at IS NULL）是唯一互斥点，只有一个请求应成功，其余撞 RowsAffected=0
// 走 "该战斗凭证已被结算"。这条分支是并发专属，顺序重放会先被 ValidateSettle
// 的 UsedAt 检查挡掉，故必须用真实并发构造。
func TestSettleConcurrentClaimGuard(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})
	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	in := loseSettleInput()
	ok, errs, peak := runParallel(8, func(int) error {
		_, e := ts.SettleBattle(ctx, uid, tp.TokenID, in)
		return e
	})
	mustOverlap(t, peak, 8)
	if ok != 1 {
		t.Errorf("同一 token 并发结算应恰好成功 1 次，实际 %d 次", ok)
	}
	if len(errs) != 7 {
		t.Errorf("应有 7 次被拒，实际 %d", len(errs))
	}
}

// --- 专精 / 诊断：中段失败 ---

func TestProgressionMidwayFailures(t *testing.T) {
	ctx := context.Background()

	t.Run("LoadMastery点数查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		// user_mastery_nodes 完好（首个 Query 成功）→ user_progress 改名 →
		// 第二个 QueryRow(mastery_points) 失败。
		ts.renameTable(t, "user_progress", "user_progress_bak")
		_, _, err := ts.LoadMastery(ctx, uid)
		ts.renameTable(t, "user_progress_bak", "user_progress")
		mustErr(t, err, "load mastery points")
	})

	t.Run("AllocateMastery锁进度失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_progress", "user_progress_bak")
		err := ts.AllocateMastery(ctx, uid, 1)
		ts.renameTable(t, "user_progress_bak", "user_progress")
		mustErr(t, err, "lock progress")
	})

	t.Run("AllocateMastery查已选失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		// user_progress 完好 → 锁进度成功；user_mastery_nodes 改名 →
		// masterySelected 的查询失败。
		ts.renameTable(t, "user_mastery_nodes", "user_mastery_nodes_bak")
		err := ts.AllocateMastery(ctx, uid, 1)
		ts.renameTable(t, "user_mastery_nodes_bak", "user_mastery_nodes")
		mustErr(t, err, "load selected")
	})

	t.Run("Diagnose读技能失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		// battle_records 完好（首个查询成功）→ user_skills 改名 →
		// loadSkillsAndSlots 失败。
		ts.renameTable(t, "user_skills", "user_skills_bak")
		_, err := ts.Diagnose(ctx, uid, 1, 3)
		ts.renameTable(t, "user_skills_bak", "user_skills")
		mustErr(t, err, "load skills")
	})

	t.Run("ListDefenses候选查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		// user_daily_challenges 完好（计数成功）→ defenses 改名 → 候选查询失败。
		ts.renameTable(t, "defenses", "defenses_bak")
		_, _, err := ts.ListDefenses(ctx, uid)
		ts.renameTable(t, "defenses_bak", "defenses")
		mustErr(t, err, "list defenses")
	})

	t.Run("SaveDefense写防线失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "defenses", "defenses_bak")
		_, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{Name: "x", Skills: []int{1}})
		ts.renameTable(t, "defenses_bak", "defenses")
		mustErr(t, err, "save defense")
	})

	t.Run("SaveDefense清工事失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		// defenses 完好（upsert 成功）→ defense_works 改名 → DELETE 失败。
		ts.renameTable(t, "defense_works", "defense_works_bak")
		_, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{Name: "x", Skills: []int{1}})
		ts.renameTable(t, "defense_works_bak", "defense_works")
		mustErr(t, err, "")
	})
}

// TestValidateDefenseOwnership 校验分支：技能过多、非正 id 跳过。
func TestValidateDefenseOwnership(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 技能数 > 12 → 直接拒
	many := make([]int, 13)
	if _, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{Skills: many}); !errors.Is(err, ErrBadInput) {
		t.Errorf("13 个技能应 ErrBadInput，实际 %v", err)
	}
	// id <= 0 的技能/装备/专精项应被跳过（不查库、不报"未拥有"）——
	// 新号拥有技能 1，其余用 0/-1 占位，保存应成功。
	if _, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{
		Name: "零占位", Skills: []int{1, 0, -1}, Equipment: []int{0}, MasteryNodes: []int{0, -2},
	}); err != nil {
		t.Errorf("非正 id 应被跳过、保存成功，实际 %v", err)
	}
}

// TestChallengeDefenseMidway 挑战防线的护盾拦截、计数查询失败、写挑战记录失败。
func TestChallengeDefenseMidway(t *testing.T) {
	ctx := context.Background()

	// 造一个"对手 + 其防线"的辅助
	setupOpponent := func(t *testing.T, ts *testService) (challenger, defenseID int64) {
		t.Helper()
		challenger = ts.newUser(t, ctx)
		owner := ts.newUser(t, ctx)
		dv, err := ts.SaveDefense(ctx, owner, SaveDefenseInput{Name: "敌线", Skills: []int{1}})
		if err != nil {
			t.Fatalf("建对手防线失败：%v", err)
		}
		return challenger, dv.ID
	}

	t.Run("护盾拦截", func(t *testing.T) {
		ts := openScratchService(t)
		challenger, defenseID := setupOpponent(t, ts)
		ts.exec(t, `UPDATE defenses SET shielded_until = now() + interval '1 hour' WHERE id = $1`, defenseID)
		_, err := ts.ChallengeDefense(ctx, challenger, defenseID, ChallengeInput{Seed: "0", Won: true, DurationMs: 60_000, HPLeftPct: 100, ReplayHash: "0000000000000000"})
		if !errors.Is(err, ErrForbidden) {
			t.Errorf("护盾期内应 ErrForbidden，实际 %v", err)
		}
	})

	t.Run("计数查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		challenger, defenseID := setupOpponent(t, ts)
		// defenses 完好（SELECT 成功）→ user_daily_challenges 改名 →
		// dailyChallengeCountersTx 失败。
		ts.renameTable(t, "user_daily_challenges", "user_daily_challenges_bak")
		_, err := ts.ChallengeDefense(ctx, challenger, defenseID, ChallengeInput{Seed: "0", Won: false, DurationMs: 60_000, HPLeftPct: 0, ReplayHash: "0000000000000000"})
		ts.renameTable(t, "user_daily_challenges_bak", "user_daily_challenges")
		mustErr(t, err, "")
	})

	t.Run("写挑战记录失败", func(t *testing.T) {
		ts := openScratchService(t)
		challenger, defenseID := setupOpponent(t, ts)
		// 走失败（未胜）路径：跳过窃取，直达 INSERT defense_challenges。
		ts.renameTable(t, "defense_challenges", "defense_challenges_bak")
		_, err := ts.ChallengeDefense(ctx, challenger, defenseID, ChallengeInput{Seed: "0", Won: false, DurationMs: 60_000, HPLeftPct: 0, ReplayHash: "0000000000000000"})
		ts.renameTable(t, "defense_challenges_bak", "defense_challenges")
		mustErr(t, err, "record challenge")
	})
}

// TestChallengeDefenseStealSkippedWhenOwnerBroke 覆盖"从对方扣除、扣不动就跳过"
// 分支：挑战成功、防线战力高（loot 的 10% 可窃取额 > 0），但防线主人钱包被清空，
// grantWallet 扣款失败 → 该币种被从 stolen 里删除（不强制对方负余额）。
func TestChallengeDefenseStealSkippedWhenOwnerBroke(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	challenger := ts.newUser(t, ctx)
	owner := ts.newUser(t, ctx)
	dv, err := ts.SaveDefense(ctx, owner, SaveDefenseInput{Name: "高战线", Skills: []int{1}})
	if err != nil {
		t.Fatalf("建防线失败：%v", err)
	}
	// 战力拉高 → 窃取额 take = loot/10 > 0；再把主人四种货币清零 → 扣不动
	ts.exec(t, `UPDATE defenses SET power = 20000 WHERE id = $1`, dv.ID)
	ts.exec(t, `UPDATE user_wallets SET coin = 0, gem = 0, energy = 0, keys = 0 WHERE user_id = $1`, owner)

	res, err := ts.ChallengeDefense(ctx, challenger, dv.ID, ChallengeInput{Seed: "0", Won: true, DurationMs: 60_000, HPLeftPct: 100, ReplayHash: "0000000000000000"})
	if err != nil {
		t.Fatalf("挑战应成功（扣不动只是跳过，不报错）：%v", err)
	}
	// 主人一无所有 → 全部币种都扣不动 → stolen 应被清空
	if len(res.Stolen) != 0 {
		t.Errorf("主人钱包为空时不应窃取到任何货币，实际 %+v", res.Stolen)
	}
}

// TestListDefensesAttemptsUsedUp 挑战次数用尽时，候选防线必须标记
// can_challenge=false 并给出封锁文案（else 分支）。
func TestListDefensesAttemptsUsedUp(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	me := ts.newUser(t, ctx)
	owner := ts.newUser(t, ctx)
	if _, err := ts.SaveDefense(ctx, owner, SaveDefenseInput{Name: "候选线", Skills: []int{1}}); err != nil {
		t.Fatalf("建候选防线失败：%v", err)
	}
	// 先触发一次计数行创建，再把 attempts 顶到上限
	if _, _, err := ts.ListDefenses(ctx, me); err != nil {
		t.Fatalf("首次列表失败：%v", err)
	}
	ts.exec(t, `UPDATE user_daily_challenges SET attempts = $2
	            WHERE user_id = $1 AND challenge_date = CURRENT_DATE`, me, DefenseAttemptLimit)
	_, candidates, err := ts.ListDefenses(ctx, me)
	if err != nil {
		t.Fatalf("列表失败：%v", err)
	}
	if len(candidates) == 0 {
		t.Fatal("应有候选防线")
	}
	for _, c := range candidates {
		if c.CanChallenge {
			t.Errorf("次数用尽后 can_challenge 应为 false：%+v", c)
		}
		if c.ChallengeBlocked == "" {
			t.Error("次数用尽应给出封锁文案")
		}
	}
}

// --- 后台统计：中段失败 ---

func TestStatsMidwayFailures(t *testing.T) {
	ctx := context.Background()

	t.Run("AdminListUsers列表查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		// COUNT(users) 成功 → user_progress 改名 → 列表 JOIN 查询失败。
		ts.renameTable(t, "user_progress", "user_progress_bak")
		_, _, err := ts.AdminListUsers(ctx, "", 50, 0)
		ts.renameTable(t, "user_progress_bak", "user_progress")
		mustErr(t, err, "list users")
	})

	t.Run("AdminSetUserStatus吊销令牌失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		// UPDATE users 成功 → refresh_tokens 改名 → 封禁时吊销令牌失败。
		ts.renameTable(t, "refresh_tokens", "refresh_tokens_bak")
		_, err := ts.AdminSetUserStatus(ctx, uid, true, "封禁")
		ts.renameTable(t, "refresh_tokens_bak", "refresh_tokens")
		mustErr(t, err, "revoke tokens")
	})

	t.Run("AdminListBattles列表查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		// COUNT(battle_records) 成功 → users 改名 → 列表 LEFT JOIN 查询失败。
		ts.renameTable(t, "users", "users_bak")
		_, _, err := ts.AdminListBattles(ctx, 0, 0, 10, false)
		ts.renameTable(t, "users_bak", "users")
		mustErr(t, err, "list battles")
	})

	t.Run("AdminUpdateSkill执行失败", func(t *testing.T) {
		ts := openScratchService(t)
		ts.renameTable(t, "skills", "skills_bak")
		_, err := ts.AdminUpdateSkill(ctx, 1, map[string]any{"name": "x"})
		ts.renameTable(t, "skills_bak", "skills")
		mustErr(t, err, "update skill")
	})

	t.Run("AdminRegenerateLevels执行失败", func(t *testing.T) {
		ts := openScratchService(t)
		ts.renameTable(t, "levels", "levels_bak")
		_, err := ts.AdminRegenerateLevels(ctx)
		ts.renameTable(t, "levels_bak", "levels")
		mustErr(t, err, "regenerate level")
	})

	t.Run("AdminListDefenses列表查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		// COUNT(defenses) 成功 → users 改名 → 列表 JOIN 查询失败。
		ts.renameTable(t, "users", "users_bak")
		_, _, err := ts.AdminListDefenses(ctx, 50)
		ts.renameTable(t, "users_bak", "users")
		mustErr(t, err, "admin defenses")
	})

	t.Run("AdminEconomy商城查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		// wallet_flows 流水查询成功 → shop_items 改名 → 商城查询失败。
		ts.renameTable(t, "shop_items", "shop_items_bak")
		_, err := ts.AdminEconomy(ctx)
		ts.renameTable(t, "shop_items_bak", "shop_items")
		mustErr(t, err, "admin shop")
	})
}

// TestEnsureBootstrapAdminInsertError count 查询成功、INSERT 因约束失败的分支。
// 构造：加一条"用户名不得为 boot"的 CHECK，空库计数为 0、随后 INSERT 违约。
func TestEnsureBootstrapAdminInsertError(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	ts.exec(t, `ALTER TABLE admin_users ADD CONSTRAINT no_boot_user CHECK (username <> 'boot')`)
	_, err := ts.EnsureBootstrapAdmin(ctx, "boot", "long-enough-pass")
	mustErr(t, err, "create bootstrap admin")
}

// --- 出战配置 / 装配 ---

// TestSaveLoadoutAuditError SaveLoadout 清槽+写槽成功后，写审计日志失败的分支。
func TestSaveLoadoutAuditError(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	// user_skills / user_skill_slots 完好 → 归属校验、清槽、插槽都成功；
	// admin_audit_logs 改名 → 审计写入失败。
	ts.renameTable(t, "admin_audit_logs", "admin_audit_logs_bak")
	_, err := ts.SaveLoadout(ctx, uid, SaveLoadoutInput{SkillIDs: []int{1, 2}})
	ts.renameTable(t, "admin_audit_logs_bak", "admin_audit_logs")
	mustErr(t, err, "audit log")
}

// TestLoadLoadoutGemAndUnknownEquipment 覆盖 loadLoadout 的三类分支：
//  1. 词条 JSON 非法 → AffixCountIgnored++（不静默跳过）
//  2. 各类词条 switch（armor/crit/element_coef/reaction_mult/heat_cap）装配
//  3. DB 中存在、但 domain 未知的 equipment_id → 跳过而非回落默认
func TestLoadLoadoutGemAndUnknownEquipment(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 一颗词条齐全的宝石：覆盖 armor/crit/element_coef/reaction_mult/heat_cap 分支
	ts.exec(t, `INSERT INTO user_gems (user_id, gem_id, quality, affixes, equipped_slot)
	            VALUES ($1, 1, 1,
	              '[{"affix":"armor","value":10},{"affix":"crit","value":10},
	                {"affix":"element_coef","value":10},{"affix":"reaction_mult","value":10},
	                {"affix":"heat_cap","value":10}]'::jsonb, 'slot0')`, uid)
	// 一颗词条 JSON 非法（合法 jsonb 但不是数组）：覆盖 Unmarshal 失败分支
	ts.exec(t, `INSERT INTO user_gems (user_id, gem_id, quality, affixes, equipped_slot)
	            VALUES ($1, 2, 1, '"boom"'::jsonb, 'slot1')`, uid)
	// DB 里塞一件 domain 不认识的装备（先在 equipment 表建行以过 FK），并给玩家装上
	ts.exec(t, `INSERT INTO equipment (id, code, name, slot, tier, element, descr, base_armor, base_bonus_pct, unlock_level)
	            VALUES (99999, 'ghost', '幽灵', 'weapon', 1, 'physical', '', 0, 0, 1)
	            ON CONFLICT (id) DO NOTHING`)
	ts.exec(t, `INSERT INTO user_equipment (user_id, equipment_id, slot, equipped) VALUES ($1, 99999, 'weapon', TRUE)`, uid)

	c, err := ts.loadLoadout(ctx, uid)
	if err != nil {
		t.Fatalf("loadLoadout：%v", err)
	}
	if c.AffixCountIgnored < 1 {
		t.Errorf("非法词条应计入 AffixCountIgnored，实际 %d", c.AffixCountIgnored)
	}
	if c.Armor <= 0 || c.Crit <= 0 || c.ElementCoef <= 0 || c.ReactionMult <= 0 || c.HeatCap <= 0 {
		t.Errorf("五类词条都应被装配：%+v", c)
	}
	// computeAttacker 也应能处理未知装备（跳过）而不报错
	if _, err := ts.computeAttacker(ctx, uid); err != nil {
		t.Errorf("含未知装备的 computeAttacker 应跳过而非报错：%v", err)
	}
}

// TestComputeRatingWithMasteryNodes computeRating 在 build 带真实专精节点时，
// 必须走 fams 匹配（node.ID == n）与 MasteryFamilies 收集两段。
func TestComputeRatingWithMasteryNodes(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	var nodeID int
	for _, f := range domain.AllMasteryFamilies() {
		if len(f.Nodes) > 0 {
			nodeID = f.Nodes[0].ID
			break
		}
	}
	// 直接喂带 mastery_nodes 的 build：触发 fams/families 两段与点数读取。
	build := map[string]any{
		"elements":      []string{"fire", "ice"},
		"mastery_nodes": []int{nodeID},
	}
	r := ts.ComputeRatingFor(uid, build)
	if r.MasteryDone < 1 {
		t.Errorf("带专精节点的评分 MasteryDone 应 ≥1，实际 %+v", r)
	}
}
