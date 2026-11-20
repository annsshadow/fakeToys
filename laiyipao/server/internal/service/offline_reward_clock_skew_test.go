// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 离线收益「时钟回拨 / DB now() 领先 Go time.Now()」边界回归。
//
// 行为锚点：last_claim_at 落在未来时（PG now() 与 Go 进程时钟不同源），
// 距上次领取的时长必须按 0 处理，不得算出负金币 / 负时长。
// 前端 signin.vue 直接展示 offlineReward.coin 与 elapsed_hours，
// 负值会让待领取卡片显示「-XXX 金币」「已领完」。

import (
	"testing"
	"time"
)

// 直接用零值 Service 做纯函数单测：Cfg() 会 CAS 注入独立配置快照，不依赖数据库。
func TestOfflineRewardForFutureLastClaimHasNoNegativeCoin(t *testing.T) {
	s := &Service{}
	cfg := s.Cfg()
	cfg.OfflineCapHours = 12
	cfg.OfflineCoinPerHour = 600

	now := time.Date(2026, 1, 1, 12, 0, 0, 0, time.UTC)
	// 模拟 PG now() 比 Go time.Now() 领先 3 小时（容器/宿主机/NTP 偏差）。
	futureLastClaim := now.Add(3 * time.Hour)

	coin, elapsed := s.offlineRewardFor(futureLastClaim, now)
	if coin < 0 {
		t.Fatalf("未来 last_claim_at 不应产生负金币，实际 coin=%d（elapsed=%v）", coin, elapsed)
	}
	if elapsed < 0 {
		t.Fatalf("未来 last_claim_at 的时长应钳为 0，实际 elapsed=%v", elapsed)
	}
}
