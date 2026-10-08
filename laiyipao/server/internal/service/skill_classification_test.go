package service

import (
	"context"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// 第 119 轮：技能的「基础/复合」分类必须**跟着种子走**，
// 不再依赖写死的 `id <= 24`。
//
// 旧逻辑 `if id <= 24 { base } else { composite }` 里 24 是魔法数字，
// 它只是「今天 SeedSkills 恰好 1..24」的巧合。
// 判据钉在**事实源**上：基础技能 == domain.SeedSkills 的 id 集合，
// 复合技能 == domain.SeedCompositeSkills 的 id 集合。
// 将来往 SeedSkills 加一个新基础技能（id 任意），分类自动跟上，
// 不会再被「漏在 24 之外」悄悄归错桶。
func TestSkillClassificationFollowsSeed(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	base, composite, err := ts.fetchSkills(ctx)
	if err != nil {
		t.Fatalf("fetchSkills 失败：%v", err)
	}

	// 期望的 id 集合直接来自种子（事实源）
	wantBase := map[int]bool{}
	for _, sk := range domain.SeedSkills {
		wantBase[sk.ID] = true
	}
	wantComposite := map[int]bool{}
	for _, sk := range domain.SeedCompositeSkills {
		wantComposite[sk.ID] = true
	}

	gotBase := map[int]bool{}
	for _, m := range base {
		if id, ok := m["id"].(int); ok {
			gotBase[id] = true
		}
	}
	gotComposite := map[int]bool{}
	for _, m := range composite {
		if id, ok := m["id"].(int); ok {
			gotComposite[id] = true
		}

	}

	for id := range wantBase {
		if !gotBase[id] {
			t.Errorf("基础技能 id=%d 应归入 base 桶，实际没有（被误判成复合？)", id)
		}
		if gotComposite[id] {
			t.Errorf("基础技能 id=%d 竟出现在 composite 桶", id)
		}
	}
	for id := range wantComposite {
		if !gotComposite[id] {
			t.Errorf("复合技能 id=%d 应归入 composite 桶，实际没有", id)
		}
		if gotBase[id] {
			t.Errorf("复合技能 id=%d 竟出现在 base 桶", id)
		}
	}
	if len(gotBase) != len(wantBase) || len(gotComposite) != len(wantComposite) {
		t.Errorf("桶数量与种子不符：base %d/%d，composite %d/%d",
			len(gotBase), len(wantBase), len(gotComposite), len(wantComposite))
	}
}
