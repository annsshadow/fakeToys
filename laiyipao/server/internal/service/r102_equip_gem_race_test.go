// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"errors"
	"fmt"
	"strings"
	"testing"
	"time"
)

// R102：宝石佩戴的「读→算→写」并发缺陷（同 R99/R100/R101 的形状）。
//
// # 缺陷形态
//
// EquipGem 的上限判定依赖这样一条读：
//
//	SELECT equipped_slot FROM user_gems
//	 WHERE user_id = $1 AND equipped_slot <> '' FOR UPDATE
//
// 它只锁住**当前已佩戴**的行。并发请求要佩戴的那颗宝石此刻
// `equipped_slot` 还是 ”，**不在这个结果集里**，因此既不被锁、
// 也不在对方语句的快照里。两个并发佩戴都读到「已戴 5 颗」，
// 都通过 `len(used) >= EquipSlotLimit` 检查，都挑「最小空闲槽」= 6，
// 各写各的行 —— 两条都返回成功。
//
// 后果（次数错误，不是性能问题）：
//   - `loadGemsAffixRows` 只按 `equipped_slot <> ”` 统计，于是**7 颗宝石的
//     词条同时生效**，超过 EquipSlotLimit=6，战力被静默抬高；
//   - 两颗宝石占同一个槽位 6，槽位唯一性（玩家排查与预设回放依赖它）
//     被破坏，且 DB 里没有 `(user_id, equipped_slot)` 唯一索引兜底。
//
// # 为什么现有用例没抓住
//
// `TestEquipGemRejectsBeyondLimit` 的并发段让两路去戴**同一颗**宝石：
// 第二轮会在 `equipGemTx` 的 `FOR UPDATE` 上等锁，等到的是一次幂等返回。
// 那验证的是「同一颗宝石不会被戴两次」，不是「同一空位的竞争」。
//
// # 注入方式
//
// BEFORE UPDATE OF equipped_slot 触发器在拿到行锁后 pg_sleep(1.2s)，
// 把「第一条已算完槽位、尚未提交」的窗口撑到可稳定命中
// （低于 DB_LOCK_TIMEOUT 2s，否则复现成 55P03 而不是目标缺陷）。
//
// 判据：恰好 1 条成功，另一条 ErrBadInput；落库佩戴数 == EquipSlotLimit，
// 槽位互不重复。修复前：两条都成功、落库 7 颗、槽位 6 出现两次。
func TestEquipGemConcurrentSlotCollision(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx)
	// 上限 +2 颗：先戴满到只剩 1 个空槽，再让两颗并发去抢它。
	gems := insertTestGems(t, ts, uid, EquipSlotLimit+2)

	for i := 0; i < EquipSlotLimit-1; i++ {
		if _, err := ts.EquipGem(ctx, uid, gems[i].ID); err != nil {
			t.Fatalf("预置第 %d 颗佩戴失败：%v", i+1, err)
		}
	}

	ts.exec(t, `CREATE OR REPLACE FUNCTION r102_slow_equip() RETURNS trigger AS $$
		BEGIN
			PERFORM pg_sleep(1.2);
			RETURN NEW;
		END;
	$$ LANGUAGE plpgsql`)
	ts.exec(t, fmt.Sprintf(`CREATE TRIGGER r102_slow_equip_trg
		BEFORE UPDATE OF equipped_slot ON user_gems
		FOR EACH ROW WHEN (NEW.user_id = %d) EXECUTE FUNCTION r102_slow_equip()`, uid))
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(),
			`DROP TRIGGER IF EXISTS r102_slow_equip_trg ON user_gems`)
		_, _ = ts.pool.Exec(context.Background(),
			`DROP FUNCTION IF EXISTS r102_slow_equip()`)
	})

	first, second := gems[EquipSlotLimit-1].ID, gems[EquipSlotLimit].ID
	errs := make([]error, 2)
	done := make(chan int, 2)

	go func() {
		_, err := ts.EquipGem(ctx, uid, first)
		errs[0] = err
		done <- 0
	}()
	// 等第一条算完槽位并进入触发器的 sleep（此时它尚未提交）。
	time.Sleep(400 * time.Millisecond)
	go func() {
		_, err := ts.EquipGem(ctx, uid, second)
		errs[1] = err
		done <- 1
	}()
	<-done
	<-done

	var equippedCount, distinctSlots int
	if err := ts.pool.QueryRow(ctx, `
		SELECT COUNT(*) FILTER (WHERE equipped_slot <> ''),
		       COUNT(DISTINCT equipped_slot) FILTER (WHERE equipped_slot <> '')
		  FROM user_gems WHERE user_id = $1`, uid).
		Scan(&equippedCount, &distinctSlots); err != nil {
		t.Fatalf("回读佩戴状态失败：%v", err)
	}

	success := 0
	for _, err := range errs {
		if err == nil {
			success++
		}
	}
	if success != 1 {
		t.Fatalf("同一空位的两路并发佩戴必须恰好成功一条，实际成功 %d 条（equipped=%d distinct_slots=%d errs=%v）",
			success, equippedCount, distinctSlots, errs)
	}
	loser := errs[0]
	if loser == nil {
		loser = errs[1]
	}
	if !errors.Is(loser, ErrBadInput) {
		t.Fatalf("失败的那条必须是明确拒绝（ErrBadInput），实际 %v", loser)
	}
	if !strings.Contains(loser.Error(), "最多佩戴") {
		t.Fatalf("拒绝文案应说明佩戴上限，实际 %v", loser)
	}
	if equippedCount != EquipSlotLimit {
		t.Fatalf("落库佩戴数必须恰好 %d 颗，实际 %d 颗（超出上限即词条多计）",
			EquipSlotLimit, equippedCount)
	}
	if distinctSlots != equippedCount {
		t.Fatalf("槽位必须互不重复：%d 颗宝石占了 %d 个槽位", equippedCount, distinctSlots)
	}
}
