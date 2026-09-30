package domain

import (
	"strings"
	"testing"
	"time"
)

// 回放前缀 S 段的守卫（第 56 轮）。
//
// 这里的每条断言都对应一个**具体踩过的坑**或**具体能抓的作弊**，
// 不是「覆盖率」意义上的凑数。

func TestReplaySkillsSegmentSortsStringsNotSlots(t *testing.T) {
	// ⚠️ 这是本文件最重要的一条。
	//
	// TS 侧写的是 `.map(...).sort().join(',')` ——
	// `Array.prototype.sort()` **按字符串码点**比较，不是按槽位数值。
	// 所以 slot=10 会排在 slot=2 **前面**。
	//
	// 我第一版在 Go 里写成按 Slot 排序，结果所有 slot ≥ 10 的对局
	// 都会被判「不匹配」，而代码读起来完全合理、测试也全绿
	//（因为当时的用例只有 slot < 10）。
	items := []ReplaySkillSlot{
		{Slot: 2, SkillID: 20, BaseDamage: 100, ApplyStacks: 1, HeatCost: 20},
		{Slot: 10, SkillID: 30, BaseDamage: 100, ApplyStacks: 1, HeatCost: 20},
	}
	got := ReplaySkillsSegment(items)
	want := "10:30:100:1:20,2:20:100:1:20" // 「1」<「2」→ slot 10 在前
	if got != want {
		t.Fatalf("S 段排序必须是**字符串**排序（与 TS 的 Array.sort 对齐）\n得到 %q\n期望 %q", got, want)
	}
}

func TestSkillBaseDamageAtLevel(t *testing.T) {
	rules := DefaultSkillRules()
	// level 1：coef = 1000 → 不变
	if got := SkillBaseDamageAtLevel(rules, 100, 1); got != 100 {
		t.Errorf("level 1 底伤应不变，得到 %d", got)
	}
	// level 10：coef = 1000 + 9*50 = 1450 → 145
	if got := SkillBaseDamageAtLevel(rules, 100, 10); got != 145 {
		t.Errorf("level 10 底伤应 145，得到 %d", got)
	}
	// 超上限被 clamp 到 MaxSkillLevel → 与 level 10 相同
	if a, b := SkillBaseDamageAtLevel(rules, 100, 99), SkillBaseDamageAtLevel(rules, 100, 10); a != b {
		t.Errorf("level 99 应被 clamp 到 %d，得到 %d（说明 ClampLevel 没走到）", b, a)
	}
}

func TestBuildReplaySkillsSegmentSkipsUnknownSkill(t *testing.T) {
	content := []SeedSkill{
		{ID: 1, BaseDamage: 100, HeatCost: 20, ApplyStacks: 1},
	}
	slots := map[int]int{0: 1, 1: 999} // 999 不在内容表里
	levels := map[int]int{1: 3}
	got := BuildReplaySkillsSegment(content, slots, levels, DefaultSkillRules())

	// 未知技能必须**跳过**而不是回落到默认值 ——
	// 回落会给作弊者一份白拿的底伤。
	if strings.Contains(got, "999") {
		t.Fatalf("未知技能 id 不应出现在 S 段里，得到 %q", got)
	}
	if !strings.Contains(got, "0:1:110:1:20") {
		t.Fatalf("level 3 的底伤应为 100×1150/1000=110，得到 %q", got)
	}
}

func TestReplaySkillsBounds(t *testing.T) {
	gl := GenerateLevel(1)
	now := time.Now()
	tok := BattleToken{LevelID: gl.ID, ExpiresAt: now.Add(time.Hour)}

	// 合法值必须通过
	ok := SettleInput{
		Result: "lose", Kills: 0, Leaked: 0, HPLeft: int(gl.BaseHP),
		WaveReached: 0, DurationMs: 5000, Shots: 0, Hits: 0, Reactions: 0, HeatMax: 0,
		ReplaySkills: "0:1:100:1:20,1:2:140:2:30",
	}
	if _, err := ValidateSettle(tok, gl, ok, limitsFor(gl), now); err != nil {
		t.Fatalf("合法的 replay_skills 被拒了：%v", err)
	}

	// 以下每一条都必须被拒，且错误要说清是哪一类
	cases := []struct {
		name string
		val  string
	}{
		{"字段数不是 5", "0:1:1:1:1:1"},
		{"字段数为 4", "0:1:1:1"},
		{"含非数字字符", "0:1:abc:1:1"},
		{"含空字段", "0::1:1:1"},
		{"负数（负号不是数字字符）", "-1:1:1:1:1"},
	}
	for _, c := range cases {
		in := ok
		in.ReplaySkills = c.val
		if _, err := ValidateSettle(tok, gl, in, limitsFor(gl), now); err == nil {
			t.Errorf("replay_skills %s：应被拒，却通过了（值 %q）", c.name, c.val)
		}
	}
}

func TestReplaySkillsEmptyIsAllowed(t *testing.T) {
	// 空串必须放行 —— 「没上报」与「上报了但不对」要分开：
	// 前者是老客户端/漏报，后者才是作弊。
	// 语义比对在 service 层做（那里才知道玩家的真实构筑），
	// 这里只管形状与长度。
	gl := GenerateLevel(1)
	now := time.Now()
	tok := BattleToken{LevelID: gl.ID, ExpiresAt: now.Add(time.Hour)}
	in := SettleInput{
		Result: "lose", HPLeft: int(gl.BaseHP), DurationMs: 5000, ReplaySkills: "",
	}
	if _, err := ValidateSettle(tok, gl, in, limitsFor(gl), now); err != nil {
		t.Fatalf("空的 replay_skills 应放行（老客户端不上报），却被拒：%v", err)
	}
}
