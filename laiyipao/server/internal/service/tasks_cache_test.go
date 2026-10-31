package service

// LoadTasks 静态任务配置缓存的集成判据（2026-10-31 新一轮第 8 轮）。
//
// 修前 `LoadTasks` 每次请求都把整张 `tasks` 配置表（seeder 纯静态、
// 无后台修改入口）与 `user_tasks` LEFT JOIN 一起查；修后任务配置
// 静态部分走 TTL 缓存（TTL 见 config.TasksCacheTTL，0 = 关闭），
// 玩家当日进度改为一条独立查询。
//
// 判据沿用本仓的「改底表」手法：**首次调用成功之后，把 tasks 改名**。
// 第二次调用若仍成功，说明任务配置来自缓存 —— 那正是「缓存命中」的定义。
//
// ⚠️ 无 Postgres 时 openScratchService 会 Skip —— 那一刻这条断言是
// **未执行**，不是「通过」。

import (
	"context"
	"testing"
)

func TestTasksServedFromCache(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid := ts.newUser(t, ctx)

	first, err := ts.LoadTasks(ctx, uid, "daily")
	if err != nil {
		t.Fatalf("首次 LoadTasks 失败：%v", err)
	}
	if len(first) == 0 {
		t.Fatal("夹具自检失败：种子后每日任务应非空")
	}

	// 把任务配置表改名 —— 之后任何真实查询都必然报错
	ts.renameTable(t, "tasks", "tasks_bak")
	defer ts.renameTable(t, "tasks_bak", "tasks")

	// TTL 内第二次调用应命中缓存（user_tasks 进度查询仍正常）
	second, err := ts.LoadTasks(ctx, uid, "daily")
	if err != nil {
		t.Fatalf("TTL 内第二次调用应命中缓存（任务表已改名，查库必失败），实际报错：%v", err)
	}
	if len(second) != len(first) {
		t.Fatalf("缓存快照应等于首次结果：first=%d 条 second=%d 条", len(first), len(second))
	}
	for i := range first {
		if first[i].ID != second[i].ID || first[i].Name != second[i].Name {
			t.Errorf("缓存快照第 %d 项不一致：first=%+v second=%+v", i, first[i], second[i])
		}
	}
}