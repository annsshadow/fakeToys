package service

import (
	"context"
	"errors"
	"testing"
)

// TestAdminBattleDetailDistinguishesMissingFromBroken 钉住「不存在」与「查不了」
// 的区分。
//
// # 为什么这条要单独写
//
// 运营端此前走 `AdminGetReplay`，而后者把**任何**错误都报成 `ErrNotFound` ——
// 那是**玩家侧**的防预言机措施（不能让人二分出「某个 id 是否存在」）。
//
// 运营端本来就能看全部战报列表，**没有预言机要防**；于是库一挂，
// 运营看到的是「战报不存在」—— 一个**假的**答案。
// 「服务异常」与「这条记录没了」在处置上完全不同（前者等库恢复，后者查数据）。
//
// 判据：真的没有这一行 → ErrNotFound（404）；库坏 → 不是 ErrNotFound（500）。
func TestAdminBattleDetailDistinguishesMissingFromBroken(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 1) 真的不存在 → ErrNotFound
	_, err := ts.AdminBattleDetail(ctx, 999_999_999)
	if !errors.Is(err, ErrNotFound) {
		t.Errorf("不存在的战报应 ErrNotFound，实际 %v", err)
	}

	// 2) 库坏 → **不能**说成 ErrNotFound
	broken := openBrokenService(t)
	_, err = broken.AdminBattleDetail(ctx, 1)
	if err == nil {
		t.Fatal("数据库故障时应报错")
	}
	if errors.Is(err, ErrNotFound) {
		t.Errorf("数据库故障被说成了「战报不存在」（%v）—— 那是**假的**答案，"+
			"运营会以为记录被删了", err)
	}
}
