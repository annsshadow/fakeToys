package service

// 后台兑换码批量生成的服务层测试（2026-11-01 新一轮第 57 轮）。
//
// 关键行为锚点：
//   - 一次生成 count 个不重复的随机码；
//   - 奖励/次数上限/过期时间与单个创建一致；
//   - count 越界（<=0 或 >100）报 ErrBadInput；
//   - 库故障必须报错而不是静默返回空列表。

import (
	"context"
	"testing"
	"time"
)

func TestAdminCreateRedeemCodesBatch(t *testing.T) {
	ts := openScratchService(t) // 一次性库，避免共享库数据干扰
	ctx := context.Background()

	expiresAt := time.Now().Add(24 * time.Hour).UTC().Truncate(time.Second)
	items, err := ts.AdminCreateRedeemCodes(ctx, 10, map[string]int{"coin": 100, "gem": 5}, 3, &expiresAt)
	if err != nil {
		t.Fatalf("批量生成失败：%v", err)
	}
	if len(items) != 10 {
		t.Fatalf("应生成 10 个码，实际 %d", len(items))
	}
	// 码不重复且格式正确（12 位，字符集无 0/O/1/I）
	seen := map[string]bool{}
	for _, it := range items {
		if seen[it.Code] {
			t.Errorf("出现重复码：%s", it.Code)
		}
		seen[it.Code] = true
		if len(it.Code) != 12 {
			t.Errorf("码长度应为 12，实际 %q（%d）", it.Code, len(it.Code))
		}
		for _, ch := range it.Code {
			if ch == '0' || ch == 'O' || ch == '1' || ch == 'I' {
				t.Errorf("码包含易混淆字符 %q：%s", string(ch), it.Code)
			}
		}
		if it.Reward["coin"] != 100 || it.Reward["gem"] != 5 {
			t.Errorf("奖励不正确：%+v", it.Reward)
		}
		if it.MaxUses != 3 {
			t.Errorf("次数上限应为 3，实际 %d", it.MaxUses)
		}
		if it.ExpiresAt == nil || !it.ExpiresAt.Equal(expiresAt) {
			t.Errorf("过期时间不正确：%v", it.ExpiresAt)
		}
		if !it.Enabled {
			t.Error("新生成的码应默认启用（Enabled=true）")
		}
	}
}

func TestAdminCreateRedeemCodesBatchInvalidCount(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	if _, err := ts.AdminCreateRedeemCodes(ctx, 0, map[string]int{"coin": 1}, 1, nil); err == nil {
		t.Error("count=0 应报错")
	}
	if _, err := ts.AdminCreateRedeemCodes(ctx, 101, map[string]int{"coin": 1}, 1, nil); err == nil {
		t.Error("count=101 应报错")
	}
}

func TestAdminCreateRedeemCodesBatchBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, err := ts.AdminCreateRedeemCodes(ctx, 3, map[string]int{"coin": 1}, 1, nil); err == nil {
		t.Fatal("连接池已关闭时批量生成必须报错")
	}
}