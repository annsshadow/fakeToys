package service

// 覆盖率收尾第二批：LoadGameConfig（纯内存组装）与若干可构造的错误分支。

import (
	"context"
	"encoding/json"
	"errors"
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// TestLoadGameConfig 配置下发是纯内存组装，不碰库 —— 全量字段一次断言。
// 客户端把这一包当作整个游戏的内容真相，缺一块就是端上白屏/功能缺失。
func TestLoadGameConfig(t *testing.T) {
	cfg, err := openTestService(t).LoadGameConfig(context.Background())
	if err != nil {
		t.Fatalf("LoadGameConfig 失败：%v", err)
	}
	if len(cfg.Levels) != domain.TotalLevels {
		t.Errorf("levels 应为 %d 关，实际 %d", domain.TotalLevels, len(cfg.Levels))
	}
	if len(cfg.Enemies) == 0 || len(cfg.Skills) == 0 || len(cfg.Composite) == 0 {
		t.Error("enemies/skills/composite 不应为空")
	}
	if len(cfg.Equipment) == 0 || len(cfg.Gems) == 0 || len(cfg.Skins) == 0 {
		t.Error("equipment/gems/skins 不应为空")
	}
	if len(cfg.Qualities) == 0 || len(cfg.Chapters) == 0 {
		t.Error("qualities/chapters 不应为空")
	}
	if cfg.ScoreRules.OnKillNormal == 0 {
		t.Error("score_rules 必须下发（跨端契约源，客户端曾硬编码 5 处）")
	}
	if cfg.SkillRules.MaxLevel == 0 || cfg.SkillRules.BaseCost == 0 {
		t.Errorf("skill_rules 必须下发：%+v", cfg.SkillRules)
	}
	// 必须可序列化：下发路径就是 JSON
	if _, err := json.Marshal(cfg); err != nil {
		t.Errorf("配置包不可序列化：%v", err)
	}
}

// TestToInt64JSONNumber json.Number 分支：decoder.UseNumber 场景。
func TestToInt64JSONNumber(t *testing.T) {
	v, ok := toInt64(json.Number("42"))
	if !ok || v != 42 {
		t.Errorf("toInt64(json.Number(42)) = %d, %t", v, ok)
	}
	if _, ok := toInt64(json.Number("x")); ok {
		t.Error("非法 json.Number 应返回 false")
	}
}

// TestBrokenAccountPaths 故障态下的账号域错误出口。
func TestBrokenAccountPaths(t *testing.T) {
	broken := openBrokenService(t)
	ctx := context.Background()
	if _, err := broken.GuestLogin(ctx, "tok", "n"); err == nil {
		t.Error("GuestLogin 故障态应报错")
	}
	if _, err := broken.ResolveUser(ctx, "any"); err == nil {
		t.Error("ResolveUser 故障态应报错")
	}
	if _, _, err := broken.ListDefenses(ctx, 1); err == nil {
		t.Error("ListDefenses 故障态应报错")
	}
	if err := broken.AllocateMastery(ctx, 1, 1); err == nil {
		t.Error("AllocateMastery 故障态应报错")
	}
}

// TestGuestLoginMidwayFailures 登录链路的中段失败（upsert 成功之后）。
func TestGuestLoginMidwayFailures(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// user_wallets 拿走：upsert 用户成功、initNewUser 第一步失败
	ts.renameTable(t, "user_wallets", "user_wallets_bak")
	_, err := ts.GuestLogin(ctx, "midway_guest", "n")
	ts.renameTable(t, "user_wallets_bak", "user_wallets")
	mustErr(t, err, "init wallet")
}

// TestValidateDefenseOwnershipQueryFailures 归属校验的三段查询各自失败。
func TestValidateDefenseOwnershipQueryFailures(t *testing.T) {
	ctx := context.Background()

	t.Run("装备归属查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_equipment", "user_equipment_bak")
		err := ts.validateDefenseOwnership(ctx, uid, SaveDefenseInput{Equipment: []int{1}})
		ts.renameTable(t, "user_equipment_bak", "user_equipment")
		if err == nil || errors.Is(err, ErrBadInput) {
			t.Fatalf("查询失败不应伪装成业务拒绝：%v", err)
		}
	})

	t.Run("专精归属查询失败", func(t *testing.T) {
		ts := openScratchService(t)
		uid := ts.newUser(t, ctx)
		ts.renameTable(t, "user_mastery_nodes", "user_mastery_nodes_bak")
		err := ts.validateDefenseOwnership(ctx, uid, SaveDefenseInput{MasteryNodes: []int{1}})
		ts.renameTable(t, "user_mastery_nodes_bak", "user_mastery_nodes")
		if err == nil || errors.Is(err, ErrBadInput) {
			t.Fatalf("查询失败不应伪装成业务拒绝：%v", err)
		}
	})
}
