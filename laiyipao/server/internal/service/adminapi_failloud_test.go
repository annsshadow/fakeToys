package service

// 第 146 轮：后台关卡列表/详情的 jsonb 列必须 fail-loud。
// 修前 `AdminListLevels` / `AdminLevelRow` 用 `_ = json.Unmarshal(...)` 静默吞掉
// star_targets / terrain_config 的解析失败 —— 一旦某列被写成「合法 jsonb 但不是
// 整型数组」（例如对象 {"not":"array"}），后台就把该关的星级门槛/地形显示成空，
// 运营误以为「这关没门槛」，实际数据是坏的。与第 125/144 轮同族：出错必须响。
//
// 判据：把某关 star_targets 写成合法 jsonb 对象后，AdminLevelRow 与
// AdminListLevels 都必须返回错误（而不是静默返回空数组）。

import (
	"context"
	"strings"
	"testing"
)

func TestAdminLevelEndpointsFailLoudOnBadJsonb(t *testing.T) {
	ts := openScratchService(t) // 一次性库，seeder 已灌 100 关
	ctx := context.Background()

	var id int
	if err := ts.pool.QueryRow(ctx, `SELECT MIN(id) FROM levels`).Scan(&id); err != nil {
		t.Fatalf("读 levels 失败（seeder 应已灌数据）：%v", err)
	}

	// 合法 jsonb 但**不是整型数组**：正常 Unmarshal 进 []int64 会失败。
	// （jsonb 类型本身接受任意合法 JSON，所以这种脏数据是写得进去的。）
	ts.exec(t, `UPDATE levels SET star_targets = $1::jsonb WHERE id = $2`, `{"not":"array"}`, id)

	// AdminLevelRow 必须报出「非法 jsonb」，而不是静默给空 stars
	if _, err := ts.AdminLevelRow(ctx, id); err == nil {
		t.Errorf("AdminLevelRow 应对坏 star_targets 报错（fail-loud），却静默成功了")
	} else if !strings.Contains(err.Error(), "star_targets") {
		t.Errorf("错误信息应指明 star_targets，实际 %q", err.Error())
	}

	// AdminListLevels 同样必须失败（该关在列表里）
	if _, _, err := ts.AdminListLevels(ctx, 0, ""); err == nil {
		t.Errorf("AdminListLevels 应对坏 jsonb 报错（fail-loud），却静默成功了")
	}
}
