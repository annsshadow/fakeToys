package domain

import "testing"

// `replay_skills` 上限的两条守卫（第 57 轮）。
//
// 这两条都不是「覆盖率」意义上的凑数，各自有具体的失效场景。

// TestReplaySkillsLimitsMatchContractContractVectors 保证 Go 常量与跨端契约一致。
//
// ## 为什么要跨端一致
//
// 上限有两份消费者：
//
//	Go 的 ValidateSettle          —— 按它拒请求
//	TS 侧 settle_bounds_parity    —— 按它断言客户端产物不会越界
//
// 两份一旦漂移，客户端的守卫就会在服务端已经不拒的区间里报红，
// 或者反过来 —— 客户端全绿而真实玩家被拒。
// 和 `base_damage` 那次（README 第 16 条）是同一类漂移。
func TestReplaySkillsLimitsMatchContractVectors(t *testing.T) {
	v := loadVectors(t)
	lim := v.ReplaySkills.Limits

	if lim.MaxLen == 0 || lim.MaxEntries == 0 {
		t.Fatal("formula_vectors.json 的 replay_skills.limits 是空的 —— " +
			"上限被清空时 TS 侧守卫会静默失效，这里必须红")
	}
	if got := ReplaySkillsMaxLen; got != int(lim.MaxLen) {
		t.Errorf("ReplaySkillsMaxLen = %d，但契约向量写的是 %d", got, lim.MaxLen)
	}
	if got := ReplaySkillsMaxEntries; got != int(lim.MaxEntries) {
		t.Errorf("ReplaySkillsMaxEntries = %d，但契约向量写的是 %d", got, lim.MaxEntries)
	}

	// 实测值也要跟着在：它们是「上限为什么是这个数」的依据。
	// 上限被调小到实测值以下时，这里提醒一句。
	m := lim.Measured
	if m.PlayerMaxLenLevel1 == 0 || m.PlayerMaxLenLevel99 == 0 || m.PlayerMaxEntries == 0 {
		t.Fatal("limits.measured 是空的 —— 上限失去了实测依据")
	}
	// 越界判断用**较大**的那一档：上限必须容得下最坏情况，
	// 而不是容得下「默认等级」那一种。
	worst := m.PlayerMaxLenLevel99
	if m.PlayerMaxLenLevel1 > worst {
		worst = m.PlayerMaxLenLevel1
	}
	if worst > int64(ReplaySkillsMaxLen) {
		t.Errorf("客户端实测最长 %d（顶满等级）已超过上限 %d —— 合法玩家会被拒", worst, ReplaySkillsMaxLen)
	}
	if m.PlayerMaxEntries > int64(ReplaySkillsMaxEntries) {
		t.Errorf("客户端实测最多 %d 条已超过上限 %d —— 合法玩家会被拒", m.PlayerMaxEntries, ReplaySkillsMaxEntries)
	}
	// 条目上限处的长度必须仍然小于长度上限，否则「两条边界分开报」就失去意义：
	// 长度校验会先触发，报错会指向错的原因。
	if m.AtEntryCapLen > int64(ReplaySkillsMaxLen) {
		t.Errorf("条目上限处实测长度 %d 已超过长度上限 %d —— 长度校验会先于条目数触发，报错会指错原因",
			m.AtEntryCapLen, ReplaySkillsMaxLen)
	}
}

// TestReplaySkillsEntryCapCoversMaxPossibleSlots 保证条目上限覆盖**内容派生**的最大槽位数。
//
// ## 为什么这条要按内容算，而不是写死一个数字
//
// 槽位总数 = `BaseSkillSlots` + 专精 `extra_slot` 节点数。
// `extra_slot` 来自 `masteryLayerKinds[2][0]` —— 每个族恰好 1 个。
// 所以**每加一个专精族，玩家就多一个额外槽位**，上界自动 +1。
//
// 写死「13」的话，将来加到第 14 个族时这个测试照样绿，
// 而所有满级玩家会突然被结算拒绝 —— 一个只在内容扩充后才出现的全量误封。
//
// 这里直接数 `AllMasteryFamilies()`，无库依赖。
func TestReplaySkillsEntryCapCoversMaxPossibleSlots(t *testing.T) {
	families := len(AllMasteryFamilies())

	// 数一遍真正的 extra_slot 节点，而不是假定「每族一个」。
	// 两者的偏差本身就是 bug 的信号。
	extra := 0
	for _, f := range AllMasteryFamilies() {
		for _, n := range f.Nodes {
			if n.Kind == "extra_slot" {
				extra++
			}
		}
	}
	if extra != families {
		t.Errorf("extra_slot 节点数 %d != 族数 %d —— 「每族恰好 1 个」的前提变了，"+
			"槽位上界的推导要跟着改（这条守卫的存在就是为了让这种变化变红）",
			extra, families)
	}

	maxSlots := BaseSkillSlots + extra
	t.Logf("BaseSkillSlots=%d  extra_slot 节点=%d  最大槽位=%d  条目上限=%d",
		BaseSkillSlots, extra, maxSlots, ReplaySkillsMaxEntries)

	if ReplaySkillsMaxEntries < maxSlots {
		t.Errorf("条目上限 %d < 最大可能槽位 %d —— 全满玩家会被判上报条目过多"+
			"（该调的是 ReplaySkillsMaxEntries，不是去放宽形状校验）",
			ReplaySkillsMaxEntries, maxSlots)
	}
}
