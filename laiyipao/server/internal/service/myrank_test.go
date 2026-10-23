package service

// `/me/rank` 的守卫（2026-10-23）。
//
// 这条端点补的是「榜单看得见但用不上」：`Leaderboard` 只回前 N 条，
// 客户端只能在这 N 条里高亮自己 —— 未进前 N 的玩家在榜上找不到自己。
//
// ## 这条测试真正在守什么
//
// 不是「返回一个数字」，而是**两个端点的名次口径必须一致**。
// 榜是按 `leaderboardOrderSQL` 排的，我的名次是按同一个方向数的；
// 一旦有人把效率榜的方向写反（升序榜却按「比我大的人数」数），
// 症状是**所有人看起来都并列第一**，而不会报任何错。
//
// 判据因此写成「拿 /me/rank 的名次去榜上取第 N 条，必须还是我」——
// 它同时约束了分数定义、过滤条件、排序方向与并列处理。

import (
	"context"
	"testing"
)

// seedRankUsers 造三个用户，分数分别为高 / 中 / 低，返回「中」那个的 id。
//
// 三个榜的「高分」方向不同（效率榜越低越好），所以分开写。
func seedRankUsers(t *testing.T, ts *testService, ctx context.Context, kind string) int64 {
	t.Helper()
	ids := make([]int64, 3)
	for i := range ids {
		ids[i] = ts.newUser(t, ctx)
	}
	// ids[0] 最好，ids[1] 居中，ids[2] 最差
	switch kind {
	case "power":
		for i, uid := range ids {
			ts.exec(t, `UPDATE user_progress SET level_exp = $2 WHERE user_id = $1`, uid, int64(3000-i*1000))
		}
	case "stage":
		for i, uid := range ids {
			ts.exec(t, `INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
			            VALUES ($1, $2, 3, 1000, 1, 500)`, uid, 30-i*10)
		}
	case "efficiency":
		for i, uid := range ids {
			// 越低越好：ids[0]=100 最好
			ts.exec(t, `INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
			            VALUES ($1, 5, 3, 1000, 1, $2)`, uid, int64(100+i*100))
		}
	default:
		t.Fatalf("未知榜单 %q", kind)
	}
	return ids[1]
}

// TestMyRankMatchesLeaderboardOrder 三个榜：名次必须能在榜上取回自己。
func TestMyRankMatchesLeaderboardOrder(t *testing.T) {
	for _, kind := range []string{"power", "stage", "efficiency"} {
		t.Run(kind, func(t *testing.T) {
			ts := openScratchService(t) // 干净库：名次是确定的
			ctx := context.Background()
			me := seedRankUsers(t, ts, ctx, kind)

			mr, err := ts.MyRank(ctx, me, kind)
			if err != nil {
				t.Fatalf("MyRank(%s) 失败：%v", kind, err)
			}
			if mr.Rank != 2 {
				t.Fatalf("%s 榜：居中用户的 rank 应为 2，实际 %d（total=%d score=%d）",
					kind, mr.Rank, mr.Total, mr.Score)
			}
			if mr.Total != 3 {
				t.Errorf("%s 榜：total 应为 3，实际 %d", kind, mr.Total)
			}
			if mr.Kind != kind {
				t.Errorf("响应里的 kind 应回显规范化后的 %q，实际 %q", kind, mr.Kind)
			}

			// 核心断言：拿名次去榜上取第 N 条，必须还是我。
			// 这一条同时约束分数定义、过滤、排序方向与并列处理 ——
			// 方向写反时 rank 会变成 1（「所有人都并列第一」）。
			items, err := ts.Leaderboard(ctx, kind, 50)
			if err != nil {
				t.Fatalf("Leaderboard(%s) 失败：%v", kind, err)
			}
			if len(items) < mr.Rank {
				t.Fatalf("%s 榜只回 %d 条，取不到第 %d 名", kind, len(items), mr.Rank)
			}
			got := items[mr.Rank-1]
			if got.UserID != me {
				t.Errorf("%s 榜：第 %d 名是 user %d，而 MyRank 说是我（user %d）—— "+
					"两个端点的排序方向或分数定义已经分叉", kind, mr.Rank, got.UserID, me)
			}
			if got.Score != mr.Score {
				t.Errorf("%s 榜：榜上第 %d 名分数 %d，MyRank 报 %d —— 分数定义不一致",
					kind, mr.Rank, got.Score, mr.Score)
			}
		})
	}
}

// TestMyRankWithoutScore 无成绩时 rank=0，而不是 1。
//
// 返回 1 会让「从没通关过的人」看起来是关卡榜榜首 ——
// 那是最容易被截图传播的那种错误。
func TestMyRankWithoutScore(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 造一个有成绩的人，保证 total > 0（否则「total=0 所以 rank=0」是巧合）
	other := ts.newUser(t, ctx)
	ts.exec(t, `INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
	            VALUES ($1, 7, 3, 1000, 1, 200)`, other)
	me := ts.newUser(t, ctx) // 从没通关过

	mr, err := ts.MyRank(ctx, me, "stage")
	if err != nil {
		t.Fatalf("MyRank 失败：%v", err)
	}
	if mr.Rank != 0 {
		t.Errorf("无成绩的玩家 rank 必须是 0（哨兵），实际 %d", mr.Rank)
	}
	if mr.Score != 0 {
		t.Errorf("无成绩时 score 应为 0，实际 %d", mr.Score)
	}
	if mr.Total != 1 {
		t.Errorf("total 应是**真实**有成绩人数 1，实际 %d —— "+
			"没有成绩不该让人数消失", mr.Total)
	}
}

// TestMyRankTieBreaksByUserID 同分时的名次口径必须与榜单**列表下标**一致。
//
// # 这条测试的第一版是错的，值得记下来
//
// 第一版断言「同分的两人都是第 1」—— 那是竞赛排名（1,1,3）口径。
// 而 `Leaderboard` 的名次是**行号**（1,2,3），排序是
// `ORDER BY score DESC, u.id ASC`。于是同分的两人在榜上一个是第 1、
// 一个是第 2，而 `/me/rank` 给两人都算第 1 ——
// 玩家看到「我的排名 第 1」而榜上自己高亮在第 2 条。
//
// 那条测试**把错误口径固定成了期望**：谁按计划把实现改对，它反而变红。
// 现在两边都用双键（分数 + user_id），判据是「名次 == 列表下标 + 1」。
func TestMyRankTieBreaksByUserID(t *testing.T) {
	for _, kind := range []string{"power", "efficiency"} {
		t.Run(kind, func(t *testing.T) {
			ts := openScratchService(t)
			ctx := context.Background()

			// a 先建 → id 更小 → 同分时排在前面
			a := ts.newUser(t, ctx)
			b := ts.newUser(t, ctx)
			if kind == "power" {
				ts.exec(t, `UPDATE user_progress SET level_exp = 5000 WHERE user_id = $1`, a)
				ts.exec(t, `UPDATE user_progress SET level_exp = 5000 WHERE user_id = $1`, b)
			} else {
				ts.exec(t, `INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
				            VALUES ($1, 5, 3, 1000, 1, 300)`, a)
				ts.exec(t, `INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
				            VALUES ($1, 5, 3, 1000, 1, 300)`, b)
			}

			want := map[int64]int{a: 1, b: 2}
			for uid, wantRank := range want {
				mr, err := ts.MyRank(ctx, uid, kind)
				if err != nil {
					t.Fatalf("MyRank(%s) 失败：%v", kind, err)
				}
				if mr.Rank != wantRank {
					t.Errorf("%s 榜：user %d 同分时名次应为 %d（id 小者在前），实际 %d",
						kind, uid, wantRank, mr.Rank)
				}
			}

			// 与列表逐条对齐：这是「两个端点同口径」的最终判据
			items, err := ts.Leaderboard(ctx, kind, 50)
			if err != nil {
				t.Fatalf("Leaderboard(%s) 失败：%v", kind, err)
			}
			for uid, wantRank := range want {
				if len(items) < wantRank {
					t.Fatalf("%s 榜只回 %d 条，取不到第 %d 名", kind, len(items), wantRank)
				}
				if got := items[wantRank-1].UserID; got != uid {
					t.Errorf("%s 榜：第 %d 名是 user %d，而 MyRank 说 user %d 是第 %d 名 —— "+
						"并列处理已经分叉", kind, wantRank, got, uid, wantRank)
				}
			}
		})
	}
}

// TestMyRankUnknownKindFallsBackToPower 未识别的 kind 走 power 分支**且回显 power**。
//
// 这一条是**防御性**的：路由层已经校验过 kind，正常路径到不了这里。
// 但它保证「service 被别处直接调用时」不会因为 kind 拼错而
// 静默返回一个空榜单 —— 那会让调用方以为「没人有成绩」。
func TestMyRankUnknownKindFallsBackToPower(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.exec(t, `UPDATE user_progress SET level_exp = 42 WHERE user_id = $1`, uid)

	mr, err := ts.MyRank(ctx, uid, "not-a-kind")
	if err != nil {
		t.Fatalf("MyRank 失败：%v", err)
	}
	if mr.Score != 42 {
		t.Errorf("未知 kind 应回落到 power（score=level_exp=42），实际 score=%d", mr.Score)
	}
	// ⚠️ kind 必须回显**规范化后**的值，而不是原始输入。
	// 与 `routes.go` 的「响应里的 type 必须与实际返回的数据是同一件事」同一条纪律：
	// 回显 "not-a-kind" 会让调用方在「某个榜」的标题下展示战力榜数据。
	if mr.Kind != "power" {
		t.Errorf("未知 kind 应回显规范化后的 %q，实际 %q", "power", mr.Kind)
	}
}
