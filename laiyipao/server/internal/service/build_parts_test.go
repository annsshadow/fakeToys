// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// build_parts.go 的「算」部分是纯函数 —— 这是本轮 LoadBuildSnapshot
// 12→6 查库折叠之后新引入的共享装配逻辑。DB-free 单测直接钉住这些
// 纯计算：PostgreSQL 缺席（本地 / CI 快跑）时也能守住「同一件事只算一遍」
// 的正确性，而不必依赖 openTestService 那条要走真库的路径。

import (
	"errors"
	"testing"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgconn"
	"github.com/jackc/pgx/v5/pgtype"

	"github.com/laiyipao/server/internal/domain"
)

// 空构筑（无专精/无装备/无宝石）下，attackerFromParts 必须还原
// DefaultAttacker + 关卡线性成长，且各乘区落在封顶之内。
// 这是 I-6 的锚点：客户端拿到的 build.attacker 就是这里算出的这一份。
func TestAttackerFromPartsBaselineGrowth(t *testing.T) {
	a := attackerFromParts(buildParts{maxStage: 100})
	// Attack: 100（默认）+ 100 关 × 20‰
	if a.Attack != 100+100*20 {
		t.Errorf("Attack 应为 %d，实际 %d", 100+100*20, a.Attack)
	}
	// ElementCap: 3（默认）+ 100/25=4 → 7，未超封顶 8
	if a.ElementCap != 7 {
		t.Errorf("ElementCap 应为 7，实际 %d", a.ElementCap)
	}
	// 无专精 → 系数/暴击/反应阶取默认
	if a.ElementCoefPermille != 1000 || a.CritPermille != 50 || a.ReactionTier != 1 {
		t.Errorf("默认乘区被污染：coef=%d crit=%d tier=%d",
			a.ElementCoefPermille, a.CritPermille, a.ReactionTier)
	}
}

// 关卡数够大时 ElementCap 必须在**关卡成长**这一步就钳到 MaxElementCap，
// 与装配（宝石 element_cap）之后那次钳位共用同一常量。
func TestAttackerFromPartsElementCapClampedByStage(t *testing.T) {
	a := attackerFromParts(buildParts{maxStage: 200})
	if a.ElementCap > domain.MaxElementCap {
		t.Errorf("ElementCap 应封顶 %d，实际 %d", domain.MaxElementCap, a.ElementCap)
	}
}

// 负 max_stage（无 CHECK 约束）必须按 0 成长，不能产生负攻击力。
func TestAttackerFromPartsNegativeStageClamped(t *testing.T) {
	neg := attackerFromParts(buildParts{maxStage: -5})
	zero := attackerFromParts(buildParts{maxStage: 0})
	if neg.Attack != zero.Attack {
		t.Errorf("负关卡应按 0 成长：%d vs %d", neg.Attack, zero.Attack)
	}
}

// 关卡成长 + 未封顶装配：宝石 element_cap 的原始贡献在装配**之后**被再钳一次。
func TestAttackerFromPartsLoadoutElementCapFinalClamp(t *testing.T) {
	p := buildParts{maxStage: 0, loadout: loadoutContribution{ElementCap: 99}}
	a := attackerFromParts(p)
	if a.ElementCap > domain.MaxElementCap {
		t.Errorf("装配后 ElementCap 应封顶 %d，实际 %d", domain.MaxElementCap, a.ElementCap)
	}
}

// extraSlotsFrom：负数按 0（脏数据不产生负槽位），正数原样。
func TestExtraSlotsFrom(t *testing.T) {
	if got := extraSlotsFrom(domain.MasteryEffect{ExtraSlots: -3}); got != 0 {
		t.Errorf("负 ExtraSlots 应为 0，实际 %d", got)
	}
	if got := extraSlotsFrom(domain.MasteryEffect{ExtraSlots: 2}); got != 2 {
		t.Errorf("正 ExtraSlots 应原样，实际 %d", got)
	}
}

// masteryEffectFrom：无节点直接零值（不调用 EvaluateMastery，也无点数依赖）。
func TestMasteryEffectFromEmpty(t *testing.T) {
	eff := masteryEffectFrom(nil, 0)
	if eff != (domain.MasteryEffect{}) {
		t.Errorf("无节点应为零值效果，实际 %+v", eff)
	}
}

// slotBudgetCheck：判定用「去重后的槽位数」，预算 = 基础 + 专精额外。
func TestSlotBudgetCheck(t *testing.T) {
	occ5 := map[int]bool{0: true, 1: true, 2: true, 3: true, 4: true}
	occ6 := map[int]bool{0: true, 1: true, 2: true, 3: true, 4: true, 5: true}

	// 5 槽、无额外 → 恰好不超
	if err := slotBudgetCheck(occ5, 0); err != nil {
		t.Errorf("5 槽基础预算不应超，实际 %v", err)
	}
	// 6 槽、无额外 → 超
	if err := slotBudgetCheck(occ6, 0); !errors.Is(err, ErrSlotBudgetExceeded) {
		t.Errorf("6 槽应 ErrSlotBudgetExceeded，实际 %v", err)
	}
	// 6 槽、额外 +2 → 预算 7，不超
	if err := slotBudgetCheck(occ6, 2); err != nil {
		t.Errorf("6 槽 + 2 额外应不超，实际 %v", err)
	}
}

// applyLoadoutContribution 的宝石词条：已知词条计入、未知词条记 ignored（不静默吞）。
func TestApplyLoadoutContributionGemAffixes(t *testing.T) {
	gemJSON := []byte(`[{"affix":"crit","value":80},{"affix":"nope","value":5}]`)
	c := applyLoadoutContribution(nil, [][]byte{gemJSON})
	if c.EquippedGemCount != 1 {
		t.Errorf("应计 1 颗已佩戴宝石，实际 %d", c.EquippedGemCount)
	}
	if c.Crit != 80 {
		t.Errorf("crit 词条应为 80，实际 %d", c.Crit)
	}
	if c.AffixCountIgnored != 1 {
		t.Errorf("未知词条应记 1 次 ignored，实际 %d", c.AffixCountIgnored)
	}
}

// applyLoadoutContribution 的装备白名单：未知 id 跳过（回落 = 白拿属性）。
func TestApplyLoadoutContributionSkipsUnknownEquipment(t *testing.T) {
	c := applyLoadoutContribution([]int{999999}, nil)
	if c.EquipmentCount != 0 {
		t.Errorf("未知装备 id 应被跳过，EquipmentCount 应为 0，实际 %d", c.EquipmentCount)
	}
	if c.Attack != 0 {
		t.Errorf("未知装备不应带来攻击力，实际 %d", c.Attack)
	}
}

// applyLoadout 的 Attack 封顶：I-1 反通胀红线，无界装备+宝石不能越过上限。
func TestApplyLoadoutClampsAttack(t *testing.T) {
	a := domain.DefaultAttacker()
	applyLoadout(&a, loadoutContribution{Attack: 5000})
	// 100（默认）+ clamp(5000, MaxLoadoutAttackPermille=1000)
	want := domain.DefaultAttacker().Attack + MaxLoadoutAttackPermille
	if a.Attack != want {
		t.Errorf("Attack 应封顶到 %d，实际 %d", want, a.Attack)
	}
}

// 同一个零件无论被调用多少次，算出的攻方属性逐位一致（I-6 逐位一致的来源）。
func TestAttackerFromPartsDeterministic(t *testing.T) {
	p := buildParts{maxStage: 37}
	a1 := attackerFromParts(p)
	a2 := attackerFromParts(p)
	if a1 != a2 {
		t.Errorf("同一零件两次计算应逐位一致：%+v vs %+v", a1, a2)
	}
}

// ---- skillsRowsToMaps 的坏行必须上抛（第 1 轮）----
// 这是 /me、/battle/token、settle 的共享装配路径：Scan 失败若被吞掉，
// 坏行会静默变成「半份构筑 + nil error」，回放哈希漂移把诚实玩家误判成作弊。
// 用桩 rows 注入第二行 Scan 失败（真实表全列 NOT NULL，无法用数据触发）。

type failingScanRows struct{ scanned int }

func (r *failingScanRows) Close()                                       {}
func (r *failingScanRows) Err() error                                   { return nil }
func (r *failingScanRows) CommandTag() pgconn.CommandTag                { return pgconn.CommandTag{} }
func (r *failingScanRows) FieldDescriptions() []pgconn.FieldDescription { return nil }
func (r *failingScanRows) Values() ([]any, error)                       { return nil, errors.New("not used") }
func (r *failingScanRows) RawValues() [][]byte                          { return nil }
func (r *failingScanRows) Conn() *pgx.Conn                              { return nil }
func (r *failingScanRows) TypeMap() *pgtype.Map                         { return nil }
func (r *failingScanRows) Next() bool                                   { return r.scanned < 2 }
func (r *failingScanRows) Scan(dest ...any) error {
	r.scanned++
	if r.scanned == 2 {
		return errors.New("injected scan failure")
	}
	// dest 顺序与 loadSkillsRows 的 SELECT 列一致：id,name,family,element,kind,level,slot
	*(dest[0].(*int)) = 1
	*(dest[1].(*string)) = "火球"
	*(dest[2].(*string)) = "元素"
	*(dest[3].(*string)) = "fire"
	*(dest[4].(*string)) = "active"
	*(dest[5].(*int)) = 1
	*(dest[6].(*int)) = 0
	return nil
}

func TestSkillsRowsToMapsScanErrorPropagates(t *testing.T) {
	rows := &failingScanRows{}
	skills, elements, occupied, err := skillsRowsToMaps(rows)
	if err == nil {
		t.Fatal("Scan 失败必须上抛错误，而不是把半份构筑静默返回")
	}
	if skills != nil || elements != nil || occupied != nil {
		t.Fatalf("出错时必须丢弃半成品，got skills=%d elements=%v occupied=%v",
			len(skills), elements, occupied)
	}
}
