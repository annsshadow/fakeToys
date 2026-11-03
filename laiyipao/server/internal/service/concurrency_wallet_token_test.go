// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R73：并发正确性第二轮 —— 业务不变量在「多连接、乱序、不同 token」
// 条件下的守卫（对 concurrency_test.go 第一轮单点的扩展）。
//
// # 与第一轮的区别
//
// 第一轮（concurrency_test.go）钉的是「单点限量」：每日限购、任务领取、
// 签到、同 token 结算、兑换码总上限、挑战上限。本轮钉的是**账本类不变量**：
//
//   - 余额并发扣减：不为负、成功数 = 实际扣款数、流水与余额对得上；
//   - 道具币并发消费（洗练券 / 重铸道具）：消费数 ≤ 库存，被拒必须是
//     业务错误（ErrBadInput「道具不足」），不能是数据库错误；
//   - 同一账号对同一兑换码的并发重复兑：唯一键兜底，恰好生效一次。
//
// 与 R72 的全局限流是同一主题的两侧：那边挡「打得多猛」，这边保「打进来了也不乱」。
//
// 部署口径（如实标注）：当前单实例部署，连接全部来自本进程连接池；
// 这些不变量全部由数据库原语（行锁 / 条件 UPDATE / 唯一键）保证，
// 因此结论同样适用于多实例 —— 但本测试本身只在单进程内验证。
//
// 全部走真实 DB（连接池多连接并发），DB 不可达时 Skip。

import (
	"context"
	"errors"
	"fmt"
	"testing"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

// TestConcurrentWalletDebitNeverNegative 余额并发扣减的总账不变量。
//
// 作弊手法：余额 50、单价 1 的商品，30 个并发购买。
// 若扣款不是条件 UPDATE（缺 `AND coin >= 1`），并发下余额会被扣成负数 ——
// 玩家的实际资产变成负值，后续任何消费判据全部失真。
//
// 断言四条，一条都不能少：
//
//	① 成功数 ≤ 50（库存上界）；
//	② 终值 = 50 − 成功数（不多扣不少扣）；
//	③ 终值 ≥ 0（绝不负余额）；
//	④ 流水与明细对账：shop_buy 流水条数 = 成功数、Σdelta = −成功数、
//	   最小流水余额 = 终值；购买明细行数 = 成功数。
func TestConcurrentWalletDebitNeverNegative(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx)
	// ⚠️ 必须精确设定余额：新号默认有 2000 金币（游客建号礼），
	// 只 grant 50 得到的是 2050 —— 60 个并发全部买得起，拒绝路径根本没被压到。
	ts.exec(t, `UPDATE user_wallets SET coin = 50 WHERE user_id = $1`, uid)

	// 单价 1 coin、不限购、不发实际货（payload coin:0 会被跳过）的探针商品。
	var itemID int64
	if err := ts.pool.QueryRow(ctx,
		`INSERT INTO shop_items (id, code, name, category, price, payload, limit_per_day, enabled)
		 VALUES ((SELECT COALESCE(MAX(id),0)+1 FROM shop_items), $1, 'R73 探针', 'currency',
		         $2, $3, 0, true) RETURNING id`,
		fmt.Sprintf("r73_debit_%d", time.Now().UnixNano()),
		[]byte(`{"coin":1}`), []byte(`{"coin":0}`)).Scan(&itemID); err != nil {
		t.Fatalf("造商品失败：%v", err)
	}

	const workers = 60
	ok, errs, peak := runParallel(workers, func(int) error {
		_, err := ts.Buy(ctx, uid, itemID)
		return err
	})
	mustOverlap(t, peak, workers)

	if ok == 0 {
		t.Fatalf("一次都没买成，夹具有问题。错误样本：%v", errs[:min(3, len(errs))])
	}
	if ok > 50 {
		t.Fatalf("成功 %d 次 > 余额可支撑的 50 次 —— 余额被扣穿", ok)
	}
	// 被拒的必须是「余额不足」业务错误，不能是数据库错误（23505/23514/40P01…）。
	for _, err := range errs {
		if !errors.Is(err, ErrBadInput) {
			t.Fatalf("余额耗尽后的拒绝不是业务错误 ErrBadInput：%v", err)
		}
	}

	finalCoin := ts.scalar(t, ctx, `SELECT coin FROM user_wallets WHERE user_id=$1`, uid)
	if want := 50 - ok; finalCoin != want {
		t.Errorf("终值 = %d，期望 %d（50 − 成功 %d）—— 多扣或少扣", finalCoin, want, ok)
	}
	if finalCoin < 0 {
		t.Errorf("负余额：%d", finalCoin)
	}

	flows := ts.scalar(t, ctx,
		`SELECT COUNT(*) FROM wallet_flows WHERE user_id=$1 AND currency='coin' AND reason='shop_buy'`, uid)
	if flows != ok {
		t.Errorf("shop_buy 流水 = %d，期望 = 成功数 %d", flows, ok)
	}
	sumDelta := ts.scalar(t, ctx,
		`SELECT COALESCE(SUM(delta),0) FROM wallet_flows WHERE user_id=$1 AND currency='coin' AND reason='shop_buy'`, uid)
	if sumDelta != -ok {
		t.Errorf("流水 Σdelta = %d，期望 %d", sumDelta, -ok)
	}
	minBal := ts.scalar(t, ctx,
		`SELECT COALESCE(MIN(balance),-1) FROM wallet_flows WHERE user_id=$1 AND currency='coin' AND reason='shop_buy'`, uid)
	if minBal != finalCoin {
		t.Errorf("流水最小余额 = %d，与钱包终值 %d 对不上 —— 流水记录的不是真实扣后值", minBal, finalCoin)
	}
	purchases := ts.scalar(t, ctx,
		`SELECT COUNT(*) FROM user_purchases WHERE user_id=$1 AND item_id=$2`, uid, itemID)
	if purchases != ok {
		t.Errorf("购买明细 = %d，期望 = 成功数 %d", purchases, ok)
	}
	t.Logf("余额 50、%d 并发购买：成功 %d，终值 %d，流水 %d 条 Σdelta=%d", workers, ok, finalCoin, flows, sumDelta)
}

// TestConcurrentWashGemSpendsAtMostStock 道具币并发消费：洗练券。
//
// 作弊手法：只有 2 枚洗练券，10 个并发洗同一颗宝石。
// WashGem 的正确性靠两段互斥：宝石行 FOR UPDATE + spendToken 条件 UPDATE。
// 缺任何一段，成功次数都会超过库存 —— 玩家白洗。
func TestConcurrentWashGemSpendsAtMostStock(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"gem": domain.GemShopPrice, "gem_wash_token": 2})
	if _, err := ts.Buy(ctx, uid, shopItemIDByCode(t, ts, ctx, "shop_gem")); err != nil {
		t.Fatalf("买宝石失败：%v", err)
	}
	gems, err := ts.ListGems(ctx, uid)
	if err != nil || len(gems) != 1 {
		t.Fatalf("应正好一颗宝石，实际 %d（err=%v）", len(gems), err)
	}
	gemID := gems[0].ID
	affixCount := len(gems[0].Affixes)

	const workers = 10
	ok, errs, peak := runParallel(workers, func(int) error {
		_, err := ts.WashGem(ctx, uid, gemID)
		return err
	})
	mustOverlap(t, peak, workers)

	if ok == 0 {
		t.Fatalf("一次都没洗成，夹具有问题。错误样本：%v", errs[:min(3, len(errs))])
	}
	if ok > 2 {
		t.Fatalf("成功 %d 次 > 库存 2 枚 —— 洗练券被并发扣穿", ok)
	}
	for _, err := range errs {
		if !errors.Is(err, ErrBadInput) {
			t.Fatalf("券耗尽后的拒绝不是业务错误 ErrBadInput：%v", err)
		}
	}
	if bal := ts.tokenBalance(t, ctx, uid, "gem_wash_token"); bal != 2-ok {
		t.Errorf("券余额 = %d，期望 %d（2 − 成功 %d）", bal, 2-ok, ok)
	}
	// 品质不变是洗练的定义（并发下同样不许变）；条数不变是同一品质的推论。
	after, err := ts.ListGems(ctx, uid)
	if err != nil || len(after) != 1 {
		t.Fatalf("洗后读宝石失败：%v", err)
	}
	if after[0].Quality != gems[0].Quality {
		t.Errorf("并发洗练把品质从 %q 改成 %q —— 洗的是词条不是品质", gems[0].Quality, after[0].Quality)
	}
	if len(after[0].Affixes) != affixCount {
		t.Errorf("洗后词条数 = %d，洗前 %d（品质不变则条数不该变）", len(after[0].Affixes), affixCount)
	}
	t.Logf("2 枚券、10 并发洗同一颗：成功 %d，余额 %d，品质 %q 未变", ok, 2-ok, after[0].Quality)
}

// TestConcurrentMasteryResetOnlyOnce 道具币并发消费：重铸。
//
// 只有 1 件 mastery_reset，10 个并发重置：条件 UPDATE（balance >= 1）
// 让恰好 1 次成功，其余全部「道具不足」。若成功 >1，等于免费无限重铸。
func TestConcurrentMasteryResetOnlyOnce(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx)
	nodeIDs := ts.takeMasteryNodeIDs(t, ctx, uid, 3)
	for _, id := range nodeIDs {
		ts.exec(t, `INSERT INTO user_mastery_nodes (user_id, node_id) VALUES ($1,$2)`, uid, id)
	}
	ts.grant(t, ctx, uid, map[string]int64{"mastery_reset": 1})

	const workers = 10
	ok, errs, peak := runParallel(workers, func(int) error {
		_, err := ts.ResetMastery(ctx, uid)
		return err
	})
	mustOverlap(t, peak, workers)

	if ok != 1 {
		t.Fatalf("1 件道具并发重置成功 %d 次，应恰好 1 次", ok)
	}
	for _, err := range errs {
		if !errors.Is(err, ErrBadInput) {
			t.Fatalf("道具耗尽后的拒绝不是业务错误 ErrBadInput：%v", err)
		}
	}
	if got := ts.scalar(t, ctx, `SELECT COUNT(*) FROM user_mastery_nodes WHERE user_id=$1`, uid); got != 0 {
		t.Errorf("节点应被清空，实存 %d", got)
	}
	if bal := ts.tokenBalance(t, ctx, uid, "mastery_reset"); bal != 0 {
		t.Errorf("道具应扣到 0，实际 %d", bal)
	}
	t.Logf("1 件重铸道具、10 并发：成功 %d（清空 %d 节点），拒绝 %d 次均 ErrBadInput", ok, len(nodeIDs), len(errs))
}

// TestConcurrentRedeemSameAccountOnlyOnce 同一账号对同一码的并发重复兑。
//
// 与 TestConcurrentRedeemRespectsMaxUses 互补：那边是 N 个账号抢总量，
// 这边是 1 个账号乱序连点/重试同一个码 —— 唯一键 (code_id, user_id)
// 必须让恰好 1 次生效，其余全部 ErrForbidden「已经使用过」，
// 且 used_count 只加 1（不能跟着请求数走）。
func TestConcurrentRedeemSameAccountOnlyOnce(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	code := fmt.Sprintf("R73SAME%d", time.Now().UnixNano()%1_000_000_000)
	if _, err := ts.pool.Exec(ctx,
		`INSERT INTO redeem_codes (id, code, reward, max_uses, used_count, enabled)
		 VALUES ((SELECT COALESCE(MAX(id),0)+1 FROM redeem_codes), $1, $2, 10, 0, true)`,
		code, []byte(`{"coin":100}`)); err != nil {
		t.Fatalf("造兑换码失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(),
			`DELETE FROM redeem_usages WHERE code_id IN (SELECT id FROM redeem_codes WHERE code=$1)`, code)
		_, _ = ts.pool.Exec(context.Background(), `DELETE FROM redeem_codes WHERE code=$1`, code)
	})

	uid := ts.newUser(t, ctx)
	before := ts.wallet(t, ctx, uid)

	const workers = 10
	ok, errs, peak := runParallel(workers, func(int) error {
		_, err := ts.Redeem(ctx, uid, code)
		return err
	})
	mustOverlap(t, peak, workers)

	if ok != 1 {
		t.Fatalf("同账号同码并发成功 %d 次，应恰好 1 次", ok)
	}
	for _, err := range errs {
		if !errors.Is(err, ErrForbidden) {
			t.Fatalf("重复兑换应 ErrForbidden（已经使用过），实际：%v", err)
		}
	}
	if used := ts.scalar(t, ctx,
		`SELECT used_count FROM redeem_codes WHERE code=$1`, code); used != 1 {
		t.Errorf("used_count = %d，期望 1（不能跟着并发数走）", used)
	}
	if after := ts.wallet(t, ctx, uid); after.Coin-before.Coin != 100 {
		t.Errorf("金币增量 = %d，期望 100（恰好一份奖励）", after.Coin-before.Coin)
	}
	t.Logf("同账号同码 10 并发：成功 %d，used_count=1，金币 +100", ok)
}
