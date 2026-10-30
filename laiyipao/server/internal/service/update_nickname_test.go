package service

// 玩家修改昵称的服务层测试。
//
// 关键行为锚点：
//   - 合法昵称落库（`users.nickname`）；
//   - 空昵称 / 超过 MaxNicknameLen（64 字）→ ErrBadInput；
//   - 不存在的用户 → ErrNotFound；
//   - 库故障必须报错而不是静默成功。

import (
	"context"
	"errors"
	"strings"
	"testing"
)

func TestUpdateNicknameSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "nickname")
	if err := ts.UpdateNickname(ctx, id, "新昵称"); err != nil {
		t.Fatalf("改昵称失败：%v", err)
	}

	var got string
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT nickname FROM users WHERE id = $1`, id).Scan(&got); err != nil {
		t.Fatalf("查询昵称失败：%v", err)
	}
	if got != "新昵称" {
		t.Errorf("库中昵称 = %q，期望 %q", got, "新昵称")
	}
}

func TestUpdateNicknameValidation(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "nicknameval")
	overlong := strings.Repeat("名", MaxNicknameLen+1)

	cases := []struct {
		name     string
		nickname string
	}{
		{"空昵称", "   "},
		{"超长", overlong},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			err := ts.UpdateNickname(ctx, id, tc.nickname)
			if !errors.Is(err, ErrBadInput) {
				t.Errorf("应返回 ErrBadInput，实际 %v", err)
			}
		})
	}
}

func TestUpdateNicknameAtLimitAllowed(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "nicknamemax")
	atLimit := strings.Repeat("昵", MaxNicknameLen)
	if err := ts.UpdateNickname(ctx, id, atLimit); err != nil {
		t.Fatalf("恰好 %d 字应放行：%v", MaxNicknameLen, err)
	}
}

func TestUpdateNicknameUnknownUser(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	err := ts.UpdateNickname(ctx, 999_999_999, "随便")
	if !errors.Is(err, ErrNotFound) {
		t.Fatalf("不存在的用户应返回 ErrNotFound，实际 %v", err)
	}
}

func TestUpdateNicknameBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	err := ts.UpdateNickname(ctx, 1, "随便")
	if err == nil {
		t.Fatal("连接池已关闭时改昵称必须报错，不能静默成功")
	}
}
