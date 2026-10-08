package service

// 第 129 轮：启动期内容表体检（EnsureContentSeeded）。
//
// 全新库只跑迁移不跑 cmd/seed 时，签到日历/任务/商城/兑换码全是空的，
// 玩家登录拿到的游戏内容缺一半，而服务「正常」启动、无人知晓。
// 断言两条铁律：
//  1. 全空 → 自动补种子（幂等），并且**第二次体检必须安静**（不重复刷）；
//  2. 行少数（库落后于代码版本）→ 只警示、**绝不自动重灌**
//     （内容表里的行可能是运营自定义过的，自动覆盖就是事故）。

import (
	"context"
	"strings"
	"testing"
	"time"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/store"
)

// openScratchNoSeedService：只迁移、**不**灌种子的 scratch 服务（模拟全新库）。
func openScratchNoSeedService(t *testing.T) *testService {
	t.Helper()
	dsn := scratchDSN(t)
	ctx, cancel := context.WithTimeout(context.Background(), 120*time.Second)
	defer cancel()
	cfg := config.Load()
	cfg.DatabaseURL = dsn
	cfg.DBMaxConns = 4
	cfg.DBMinConns = 1
	db, err := store.Open(ctx, cfg)
	if err != nil {
		t.Skipf("scratch 库连不上，跳过：%v", err)
	}
	t.Cleanup(db.Close)
	if err := db.Migrate(ctx); err != nil {
		t.Fatalf("scratch 库迁移失败：%v", err)
	}
	return &testService{Service: New(db, cfg), db: db}
}

func TestEnsureContentSeededAutoSeedsWhenEmpty(t *testing.T) {
	ts := openScratchNoSeedService(t)
	ctx := context.Background()

	msg, err := ts.EnsureContentSeeded(ctx)
	if err != nil {
		t.Fatalf("空库体检应自动灌种子成功：%v", err)
	}
	if !strings.Contains(msg, "自动写入") {
		t.Errorf("应报告自动灌种，实际 %q", msg)
	}
	// 关键内容必须真的在库里（不只是返回了个字符串）
	var enemiesN, skillsN int
	if err := ts.pool.QueryRow(ctx, `SELECT COUNT(*) FROM enemies`).Scan(&enemiesN); err != nil {
		t.Fatalf("数 enemies 失败：%v", err)
	}
	if enemiesN != len(domain.SeedEnemies) {
		t.Errorf("enemies=%d（期望 %d）", enemiesN, len(domain.SeedEnemies))
	}
	if err := ts.pool.QueryRow(ctx, `SELECT COUNT(*) FROM skills`).Scan(&skillsN); err != nil {
		t.Fatalf("数 skills 失败：%v", err)
	}
	if skillsN != len(domain.SeedSkills)+len(domain.SeedCompositeSkills) {
		t.Errorf("skills=%d（期望基础+复合）", skillsN)
	}

	// 幂等：第二次体检必须完全安静（不重刷、不提示）
	if msg, err := ts.EnsureContentSeeded(ctx); err != nil || msg != "" {
		t.Errorf("齐平库体检应静默（msg=%q err=%v）", msg, err)
	}
}

func TestEnsureContentSeededWarnsButNeverOverwritesWhenStale(t *testing.T) {
	ts := openScratchService(t) // 迁移 + 已灌种子
	ctx := context.Background()

	// 模拟「应用升级后库没跟上」：删一行 skills（recipe 行级联，用户表为空无碍）。
	// 只删 1 行 —— 全删会落进「全空」分支触发自动灌种，那不是陈旧场景。
	if _, err := ts.pool.Exec(ctx, `DELETE FROM skills WHERE id = 2`); err != nil {
		t.Fatalf("删 skills 行失败：%v", err)
	}

	msg, err := ts.EnsureContentSeeded(ctx)
	if err != nil {
		t.Fatalf("陈旧库体检不应报错：%v", err)
	}
	if !strings.Contains(msg, "陈旧") || !strings.Contains(msg, "cmd/seed") {
		t.Errorf("应警示并指引 cmd/seed，实际 %q", msg)
	}

	// 铁律：陈旧只警示，绝不自动重灌（skill 2 必须仍然缺失）
	var n int
	if err := ts.pool.QueryRow(ctx, `SELECT COUNT(*) FROM skills WHERE id = 2`).Scan(&n); err != nil {
		t.Fatalf("查 skill 2 失败：%v", err)
	}
	if n != 0 {
		t.Fatalf("陈旧体检竟然自动重灌了 —— 运营自定义行会被冲掉")
	}
}
