package service

import (
	"context"
	"errors"
	"strings"
	"testing"
)

// `ChallengeDefense` 的窃取循环**只吞「余额不足」**，其余错误必须上抛（第 81 轮）。
//
// # 我第一版的结论是错的，实测推翻了它
//
// 我写下的缺陷描述是：
//
//	「钱已经从对方扣掉、流水没写、挑战者也没拿到，三方账目不一致」
//
// **实测不成立。** 注入「窃取流水写不进去」后：
//
//	MUT1（无差别吞掉）：err = 「当前事务被中止 事务块结束之前的查询被忽略」
//	                    owner 余额 5002000 -> 5002000（delta 0）
//	修复后：             err = 「steal coin from 2: insert flow: simulated failure」
//	                    owner 余额 5002000 -> 5002000（delta 0）
//
// 原因是 **PostgreSQL 的事务语义**：事务内任何一条语句报错，
// 整个事务进入 aborted 状态，后续所有语句都返回 `25P02`，
// 直到 ROLLBACK。所以「吞掉错误继续跑」在 PG 里**不会造成数据不一致** ——
// 它只会让后续语句报一个指向 nowhere 的 `25P02`。
//
// # 那么缺陷是什么
//
// **错误掩盖。** 上游看到的是
//
//	当前事务被中止 事务块结束之前的查询被忽略 (SQLSTATE 25P02)
//
// 而不是
//
//	insert flow: <真实原因>
//
// 排查方向被完全带偏：25P02 说的是「你前面某条语句错了」，
// 而真实原因（DB 断了 / 网络抖了 / 约束被破坏 / 表被改名）一个字都不提。
//
// 而且它**把基础设施失败伪装成了业务拒绝**的邻居 ——
// 与第 68 轮记的「匿名 errors.New 落成 500」是同一族：
// **错误的分类决定了它被排到哪条队列**，而这里分类错了。
//
// # 顺带：PG 的这个性质是「运气」，不是「设计」
//
// 数据没丢是因为 PG 的事务语义。若将来 `grantWallet` 内部出现
// **不中止事务**的错误路径（`ON CONFLICT DO NOTHING`、
// 子事务/SAVEPOINT、或把检查移到事务外），这个吞法会立刻变成真的数据丢失。
//
// 所以按**错误来源**分类仍然是对的 ——
// 这次修的不是「一个正在造成损失的洞」，是「一个把故障诊断带偏的洞」。
func TestStealMustNotMaskInfrastructureErrors(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	attacker := ts.newUser(t, ctx)
	owner := ts.newUser(t, ctx)

	dv, err := ts.SaveDefense(ctx, owner, SaveDefenseInput{
		Name: "窃取探针", Works: []string{"slow_belt"}, Skills: []int{1, 2, 3},
	})
	if err != nil {
		t.Fatalf("造防线失败：%v", err)
	}

	// 余额不足应当被跳过 —— 这是设计意图，不许被「收紧」破坏
	t.Run("余额不足应当被跳过（设计意图）", func(t *testing.T) {
		// ⚠️ 必须**显式清零**而不是假定为 0：`newUser` 会送初始余额
		// （第 76 轮踩过这个坑 —— 把夹具的巧合性质当成了前提）。
		if _, err := ts.pool.Exec(ctx,
			`UPDATE user_wallets SET coin = 0, gem = 0, energy = 0, keys = 0
			  WHERE user_id = $1`, owner); err != nil {
			t.Fatalf("清零 owner 钱包失败：%v", err)
		}
		res, err := ts.ChallengeDefense(ctx, attacker, dv.ID, validChallenge())
		if err != nil {
			t.Fatalf("余额不足不应让整次挑战失败：%v", err)
		}
		for k, v := range res.Stolen {
			if v > 0 {
				t.Errorf("Stolen[%q] = %d，但 owner 余额已清零 —— 不该偷到东西", k, v)
			}
		}
	})
}

// TestStealSurfacesRealCauseInsteadOfAbortedTx 是本文件的核心。
//
// 判据：**错误信息里必须能看到真实原因**。
//
// ⚠️ 不能断言「owner 余额不变」—— 那条由 PG 的事务语义保证，
// 与本修复无关（我在注释里实测过：无差别吞掉时余额同样不变）。
// 把它当判据会给人「修复防住了数据丢失」的错觉。
func TestStealSurfacesRealCauseInsteadOfAbortedTx(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	a := scratch.newUser(t, ctx)
	o := scratch.newUser(t, ctx)

	dv, err := scratch.SaveDefense(ctx, o, SaveDefenseInput{
		Name: "流水故障探针", Works: []string{"slow_belt"}, Skills: []int{1, 2, 3},
	})
	if err != nil {
		t.Fatalf("造防线失败：%v", err)
	}
	scratch.grant(t, ctx, o, map[string]int64{"coin": 5_000_000})

	// 精确注入：只让 `defense_stolen` 这一种流水写不进去。
	//
	// ⚠️ 我第一版把整张 `wallet_flows` 表改名 —— 结果**两条**流水都写不进去：
	// 挑战者入账那步（defense_reward）也失败并上抛，事务回滚，
	// 于是「无差别吞掉」那个变异也**通过了**。
	// 是挑战者入账那一步替我兜住了底。
	//
	// **故障注入必须精确对准被测路径**，
	// 否则它测的是「整体能不能回滚」，不是「吞错误会不会掩盖故障」。
	scratch.exec(t, `
		CREATE OR REPLACE FUNCTION reject_defense_stolen() RETURNS trigger AS $fn$
		BEGIN
		  IF NEW.reason = 'defense_stolen' THEN
		    RAISE EXCEPTION 'simulated wallet_flows failure for defense_stolen';
		  END IF;
		  RETURN NEW;
		END $fn$ LANGUAGE plpgsql`)
	scratch.exec(t, `
		CREATE TRIGGER t_reject_defense_stolen
		  BEFORE INSERT ON wallet_flows
		  FOR EACH ROW EXECUTE FUNCTION reject_defense_stolen()`)
	t.Cleanup(func() {
		scratch.exec(t, `DROP TRIGGER IF EXISTS t_reject_defense_stolen ON wallet_flows`)
		scratch.exec(t, `DROP FUNCTION IF EXISTS reject_defense_stolen()`)
	})

	_, err = scratch.ChallengeDefense(ctx, a, dv.ID, validChallenge())
	if err == nil {
		t.Fatal("窃取流水写不进去时挑战仍然成功了 —— " +
			"钱已扣、流水没写、挑战者也没拿到")
	}

	msg := err.Error()
	for _, forbidden := range []string{"25P02", "当前事务被中止", "事务块结束之前的查询被忽略"} {
		if strings.Contains(msg, forbidden) {
			t.Errorf("错误信息是 %q —— 那是 PG 的「事务已中止」通用码，\n"+
				"它只说明「前面某条语句错了」，**真实原因一个字都没提**。\n"+
				"排查方向会被完全带偏。", msg)
		}
	}
	if !strings.Contains(msg, "simulated wallet_flows failure") {
		t.Errorf("错误信息里应当能看到真实原因（simulated wallet_flows failure），实得 %q", msg)
	}
	if errors.Is(err, ErrBadInput) {
		t.Errorf("基础设施失败被归类成 ErrBadInput（余额不足）：%v", err)
	}
}

// TestStealCreditsAttackerOnlyWhenOwnerWasDebited 钉住「谁扣了就给谁」的恒等式。
func TestStealCreditsAttackerOnlyWhenOwnerWasDebited(t *testing.T) {
	scratch := openScratchService(t)
	ctx := context.Background()
	attacker := scratch.newUser(t, ctx)
	owner := scratch.newUser(t, ctx)

	dv, err := scratch.SaveDefense(ctx, owner, SaveDefenseInput{
		Name: "恒等式探针", Works: []string{"slow_belt"}, Skills: []int{1, 2, 3},
	})
	if err != nil {
		t.Fatalf("造防线失败：%v", err)
	}
	scratch.grant(t, ctx, owner, map[string]int64{"coin": 5_000_000})

	ownerBefore := scratch.wallet(t, ctx, owner)
	atkBefore := scratch.wallet(t, ctx, attacker)

	res, err := scratch.ChallengeDefense(ctx, attacker, dv.ID, validChallenge())
	if err != nil {
		t.Fatalf("挑战失败：%v", err)
	}
	ownerAfter := scratch.wallet(t, ctx, owner)
	atkAfter := scratch.wallet(t, ctx, attacker)

	dropped := ownerBefore.Coin - ownerAfter.Coin
	gained := atkAfter.Coin - atkBefore.Coin
	if dropped != gained {
		t.Errorf("owner 少了 %d coin，attacker 多了 %d —— 两侧不一致（差 %d）",
			dropped, gained, dropped-gained)
	}
	if res.Stolen != nil {
		total := int64(0)
		for _, v := range res.Stolen {
			total += int64(v)
		}
		if total != gained {
			t.Errorf("返回的 Stolen 合计 %d，实际到手 %d", total, gained)
		}
	}

	var n int
	if err := scratch.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM wallet_flows
		  WHERE user_id = $1 AND reason = 'defense_stolen' AND delta < 0`,
		owner).Scan(&n); err != nil {
		t.Fatal(err)
	}
	if n == 0 && gained > 0 {
		t.Errorf("attacker 到手 %d coin，但 owner 侧没有任何 defense_stolen 流水", gained)
	}
}
