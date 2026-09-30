package service

// 覆盖率最后一批：computeAttacker 的装配/封顶分支与 Buy 的中段失败。
//
// 专精选点方案（每系每层限 2，超了会被 EvaluateMastery 拒）：
//   第 1 层取 element_cap + skill_damage
//   第 2 层取 crit + heat_cap
//   第 3 层取 armor + mechanic
// 这样五类加成（ElementCap/SkillDamage/Crit/HeatCap/Armor/Mechanic）
// 的装配分支全部走到；reaction_mult 每系每层只有 1 个节点，
// 合法选点下最多 3 个（rm=600）—— 永远够不到 800 封顶，
// 那几条封顶分支是不可达死代码（见报告豁免清单）。

import (
	"context"
	"errors"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// TestComputeAttackerAllBonusKinds 五类专精加成必须全部装配进攻方。
func TestComputeAttackerAllBonusKinds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	before, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("computeAttacker(基线)：%v", err)
	}

	// 合法选点：每系每层按上面的方案各点 1 个（≤ 每层 2 的上限）
	ts.exec(t, `UPDATE user_progress SET mastery_points = 100 WHERE user_id = $1`, uid)
	ts.exec(t, `INSERT INTO user_mastery_nodes (user_id, node_id)
	            SELECT $1, n.id FROM mastery_nodes n
	            WHERE (n.layer = 1 AND n.kind IN ('element_cap','skill_damage'))
	               OR (n.layer = 2 AND n.kind IN ('crit','heat_cap'))
	               OR (n.layer = 3 AND n.kind IN ('armor','mechanic'))
	            ON CONFLICT DO NOTHING`, uid)

	after, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("computeAttacker(满配)：%v", err)
	}

	// 每类加成 8 系 × 1 节点，全部应大于 0
	if after.ElementCap <= before.ElementCap {
		t.Errorf("element_cap 节点应抬升层数上限：%d → %d", before.ElementCap, after.ElementCap)
	}
	// 层数上限封顶 8：16 个节点必超，装配后必须被钳住
	if after.ElementCap > domain.MaxElementCap {
		t.Errorf("ElementCap 应封顶 %d，实际 %d", domain.MaxElementCap, after.ElementCap)
	}
	if after.Attack <= before.Attack {
		t.Errorf("skill_damage 节点应折进攻击力：%d → %d", before.Attack, after.Attack)
	}
	if after.CritPermille <= before.CritPermille {
		t.Errorf("crit 节点应抬升暴击率：%d → %d", before.CritPermille, after.CritPermille)
	}
	if after.HeatCapPermille <= before.HeatCapPermille {
		t.Errorf("heat_cap 节点应抬升热量上限：%d → %d", before.HeatCapPermille, after.HeatCapPermille)
	}
	if after.ArmorPermille <= before.ArmorPermille {
		t.Errorf("armor 节点应抬升护甲：%d → %d", before.ArmorPermille, after.ArmorPermille)
	}
	if after.MechanicPermille <= before.MechanicPermille {
		t.Errorf("mechanic 节点应抬升机制强度：%d → %d", before.MechanicPermille, after.MechanicPermille)
	}
}

// TestComputeAttackerNegativeStage max_stage 无 CHECK 约束，
// 负值必须被夹到 0 而不是产生负成长。
func TestComputeAttackerNegativeStage(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 装备贡献与 stage 无关：对比 max_stage=0 的基线，而不是裸 DefaultAttacker
	ts.exec(t, `UPDATE user_progress SET max_stage = 0 WHERE user_id = $1`, uid)
	zero, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("computeAttacker(0)：%v", err)
	}
	ts.exec(t, `UPDATE user_progress SET max_stage = -5 WHERE user_id = $1`, uid)
	neg, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("computeAttacker(-5)：%v", err)
	}
	if neg.Attack != zero.Attack {
		t.Errorf("负 stage 应按 0 成长（与 max_stage=0 一致）：%d vs %d", neg.Attack, zero.Attack)
	}
}

// TestComputeAttackerGemElementCapClamp 宝石 element_cap 无上限词条时
// 最终封顶必须生效（装配后的第二次钳位）。
func TestComputeAttackerGemElementCapClamp(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	ts.exec(t, `INSERT INTO user_gems (user_id, gem_id, quality, affixes, equipped_slot)
	            VALUES ($1, 1, 'legend', '[{"affix":"element_cap","value":99}]', 'slot0')`, uid)

	a, err := ts.computeAttacker(ctx, uid)
	if err != nil {
		t.Fatalf("computeAttacker：%v", err)
	}
	if a.ElementCap > domain.MaxElementCap {
		t.Errorf("装配后 ElementCap 应封顶 %d，实际 %d", domain.MaxElementCap, a.ElementCap)
	}
	if a.ElementCap < domain.MaxElementCap {
		t.Errorf("宝石 +99 层应顶到上限，实际 %d", a.ElementCap)
	}
}

// TestLoadoutQueryFailures 装配/专精查询的失败出口（scratch 改名单表）。
func TestLoadoutQueryFailures(t *testing.T) {
	ctx := context.Background()

	t.Run("loadLoadout装备查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_equipment", "user_equipment_bak")
		_, err := ts.loadLoadout(ctx, uid)
		ts.renameTable(t, "user_equipment_bak", "user_equipment")
		mustErr(t, err, "load equipment")
	})

	t.Run("loadLoadout宝石查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_gems", "user_gems_bak")
		_, err := ts.loadLoadout(ctx, uid)
		ts.renameTable(t, "user_gems_bak", "user_gems")
		mustErr(t, err, "load gems")
	})

	t.Run("computeAttacker专精查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_mastery_nodes", "user_mastery_nodes_bak")
		_, err := ts.computeAttacker(ctx, uid)
		ts.renameTable(t, "user_mastery_nodes_bak", "user_mastery_nodes")
		mustErr(t, err, "load mastery")
	})

	t.Run("LoadBuildSnapshot装备查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_equipment", "user_equipment_bak")
		_, err := ts.LoadBuildSnapshot(ctx, uid)
		ts.renameTable(t, "user_equipment_bak", "user_equipment")
		if err == nil {
			t.Fatal("应报错")
		}
	})

	t.Run("LoadBuildSnapshot专精查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_mastery_nodes", "user_mastery_nodes_bak")
		_, err := ts.LoadBuildSnapshot(ctx, uid)
		ts.renameTable(t, "user_mastery_nodes_bak", "user_mastery_nodes")
		mustErr(t, err, "load mastery")
	})
}

// TestBuyMidwayFailures Buy 的中段失败（钱包锁、计数、扣款、发货）。
func TestBuyMidwayFailures(t *testing.T) {
	ctx := context.Background()

	t.Run("钱包不存在", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.exec(t, `DELETE FROM user_wallets WHERE user_id = $1`, uid)
		_, err := ts.Buy(ctx, uid, 1)
		if !errors.Is(err, ErrNotFound) {
			t.Fatalf("无钱包应 ErrNotFound，实际 %v", err)
		}
	})

	t.Run("钱包锁查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_wallets", "user_wallets_bak")
		_, err := ts.Buy(ctx, uid, 1)
		ts.renameTable(t, "user_wallets_bak", "user_wallets")
		if err == nil || errors.Is(err, ErrNotFound) {
			t.Fatalf("表缺失应产生通用错误，实际 %v", err)
		}
	})

	t.Run("限购计数失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_purchases", "user_purchases_bak")
		_, err := ts.Buy(ctx, uid, 7)
		ts.renameTable(t, "user_purchases_bak", "user_purchases")
		if err == nil {
			t.Fatal("应报错")
		}
	})

	t.Run("扣款失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "wallet_flows", "wallet_flows_bak")
		_, err := ts.Buy(ctx, uid, 1) // 有价格的商品，先扣钱
		ts.renameTable(t, "wallet_flows_bak", "wallet_flows")
		// 钱包 UPDATE 成功、流水 INSERT 失败 —— 资金变动不可追溯，必须整体回滚
		mustErr(t, err, "insert flow")
	})

	t.Run("购买记录写入失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		// shop_firstpay 0 元购：跳过扣款直达发货与落单
		ts.renameTable(t, "user_purchases", "user_purchases_bak")
		_, err := ts.Buy(ctx, uid, 7)
		ts.renameTable(t, "user_purchases_bak", "user_purchases")
		if err == nil {
			t.Fatal("应报错")
		}
	})

	t.Run("故障态首查失败", func(t *testing.T) {
		broken := openBrokenService(t)
		if _, err := broken.Buy(ctx, 1, 1); err == nil {
			t.Error("故障态应报错")
		}
	})
}
