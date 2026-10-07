package service

// 第 148 轮：商城与任务列表的 jsonb 经济字段读侧必须 fail-loud。
// 修前 LoadShop / LoadTasks 用 `_ = json.Unmarshal(...)` 静默吞解析失败，
// 坏数据（合法 jsonb 但形状不对）会把商品/任务显示成「0 金 / 空白奖励」。
// 判据：把 price / reward 写成合法 jsonb 但非 map[string]int 的形状后，
// LoadShop 与 LoadTasks 都必须返回错误（而不是静默给空 map）。

import (
	"context"
	"strings"
	"testing"
)

func TestShopAndTaskListFailLoudOnBadJsonb(t *testing.T) {
	ts := openScratchService(t) // 一次性库，seeder 已灌 shop_items / tasks
	ctx := context.Background()

	// 合法 jsonb（数组）但无法 Unmarshal 进 map[string]int
	ts.exec(t, `UPDATE shop_items SET price = $1::jsonb WHERE id = (SELECT MIN(id) FROM shop_items)`, `[1,2]`)
	if _, err := ts.LoadShop(ctx, 1); err == nil {
		t.Errorf("LoadShop 应对坏 price 报错（fail-loud），却静默成功了")
	} else if !strings.Contains(err.Error(), "price") {
		t.Errorf("错误信息应指明 price，实际 %q", err.Error())
	}

	// 恢复 price，再单独验证 tasks（避免上一条污染后续断言的可读性）
	ts.exec(t, `UPDATE shop_items SET price = $1::jsonb WHERE id = (SELECT MIN(id) FROM shop_items)`, `{"coin": 1}`)

	ts.exec(t, `UPDATE tasks SET reward = $1::jsonb WHERE id = (SELECT MIN(id) FROM tasks)`, `[1]`)
	if _, err := ts.LoadTasks(ctx, 1, "daily"); err == nil {
		t.Errorf("LoadTasks 应对坏 reward 报错（fail-loud），却静默成功了")
	} else if !strings.Contains(err.Error(), "reward") {
		t.Errorf("错误信息应指明 reward，实际 %q", err.Error())
	}
}
