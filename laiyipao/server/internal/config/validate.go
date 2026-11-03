// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package config

import (
	"errors"
	"fmt"
	"slices"
	"strings"
	"time"
)

// ValidRecomputeModes 是 `SETTLE_RECOMPUTE_MODE` 的合法取值。
//
// 与 service.RecomputeModes 必须保持一致；config 不能反向 import service
// （service 依赖 config），所以这里是独立的一份。值本身只有三个，
// 漂移由 `internal/service/recompute_mode_guard_test.go` 守住。
var ValidRecomputeModes = []string{"off", "shadow", "enforce"}

// Validate 在**启动期**校验配置的语义合法性（解析失败已在读 env 的助手里
// 回退默认值，这里查的是「解析成功但值不该被接受」）。
//
// # 为什么必须存在
//
// 修前 `Load()` 对下面这些值全部静默接受：
//
//	ACCESS_TTL=0s            → 签发的 access token 立刻过期，玩家「密码对、
//	                           账号正常，但一登录就掉线」；
//	DB_MAX_CONNS=0           → 池在后来的连接请求上才报错；
//	EXPORT_MAX_CONCURRENCY=0 → 导出永远排不上槽；
//	SETTLE_RECOMPUTE_MODE=bogus → 重算闸门退到 off，**反作弊校验静默关闭**，
//	                           只往 stderr 打一行。
//
// 这些错误的共同点是「服务能起来，但行为已经错了」，而且大多只在
// 某个请求路径上才暴露。配置错误应该在启动时就吵起来（AGENTS #11）。
//
// 汇总所有错误再一次性返回：部署时一次看到全部配错项，不用改一个重启一次。
func (c Config) Validate() error {
	var errs []error
	add := func(format string, args ...any) {
		errs = append(errs, fmt.Errorf(format, args...))
	}

	if c.DBMaxConns < 1 {
		add("DB_MAX_CONNS 必须 >= 1，实际 %d（0 会让连接池在后续请求才报错）", c.DBMaxConns)
	}
	if c.DBMinConns < 0 || c.DBMinConns > c.DBMaxConns {
		add("DB_MIN_CONNS 必须在 [0, DB_MAX_CONNS=%d] 内，实际 %d", c.DBMaxConns, c.DBMinConns)
	}
	if c.DBConnLifetime < 0 {
		add("DB_CONN_LIFETIME 不能为负，实际 %v", c.DBConnLifetime)
	}
	if c.DBConnIdleTime < 0 {
		add("DB_CONN_IDLE_TIME 不能为负，实际 %v", c.DBConnIdleTime)
	}
	if c.DBStatementTimeout < 0 {
		add("DB_STATEMENT_TIMEOUT 不能为负（0 = 不限），实际 %v", c.DBStatementTimeout)
	}
	// DB_ACQUIRE_TIMEOUT 必须为正（R90）：0/负值意味着「池耗尽时请求
	// 无限挂起」（R89 F2 实测挂起 15s 无任何一层掐断）——那不是默认值，
	// 是缺失的防护。无法解析的值也被 envDurationStrict 折成 0 走到这里
	// （不静默回落默认），否则把 5s 敲成 "5" 会让防护悄悄消失。
	// 过短的阈值会把正常突发打成 503 假失败，建议 3s-10s。
	if c.DBAcquireTimeout <= 0 {
		add("DB_ACQUIRE_TIMEOUT 必须为正且可解析（0/负值/非法值 = 池耗尽时请求无限挂起；过短会把正常突发变成假失败 503，建议 3s-10s），实际 %v", c.DBAcquireTimeout)
	}

	// 令牌有效期 <= 0 是最隐蔽的一种锁死：签发成功，但客户端拿到的
	// access token 已过期、refresh token 也已过期 —— 既不能用新也不能用旧。
	if c.AccessTTL <= 0 {
		add("ACCESS_TTL 必须 > 0，实际 %v（0/负值会让签发的令牌立刻失效）", c.AccessTTL)
	}
	if c.RefreshTTL <= 0 {
		add("REFRESH_TTL 必须 > 0，实际 %v（0/负值会让刷新永远失败）", c.RefreshTTL)
	}
	if c.BattleTokenTTL <= 0 {
		add("BATTLE_TOKEN_TTL 必须 > 0，实际 %v", c.BattleTokenTTL)
	}
	if c.MinBattleDuration < 0 {
		add("MIN_BATTLE_DURATION 不能为负，实际 %v", c.MinBattleDuration)
	}

	if c.ExportMaxConcurrency < 1 {
		add("EXPORT_MAX_CONCURRENCY 必须 >= 1，实际 %d（0 会让导出永远排不上槽）",
			c.ExportMaxConcurrency)
	}
	if c.ExportWait < 0 {
		add("EXPORT_WAIT 不能为负，实际 %v", c.ExportWait)
	}
	if c.ExportMaxBytes < 0 {
		add("EXPORT_MAX_BYTES 不能为负（0 = 不限），实际 %d", c.ExportMaxBytes)
	}

	if c.OfflineCapHours < 0 {
		add("OFFLINE_CAP_HOURS 不能为负，实际 %d", c.OfflineCapHours)
	}
	if c.OfflineCoinPerHour < 0 {
		add("OFFLINE_COIN_PER_HOUR 不能为负，实际 %d", c.OfflineCoinPerHour)
	}

	// 缓存 TTL：0 = 明确关闭（排障手段），负数不是「更快的关闭」而是笔误。
	for _, ttl := range []struct {
		name string
		v    time.Duration
	}{
		{"DASHBOARD_CACHE_TTL", c.DashboardCacheTTL},
		{"LEADERBOARD_CACHE_TTL", c.LeaderboardCacheTTL},
		{"SIGNIN_CALENDAR_CACHE_TTL", c.SignInCalendarCacheTTL},
		{"ANNOUNCEMENTS_CACHE_TTL", c.AnnouncementsCacheTTL},
		{"SHOP_CACHE_TTL", c.ShopCacheTTL},
		{"TASKS_CACHE_TTL", c.TasksCacheTTL},
		{"ECONOMY_CACHE_TTL", c.EconomyCacheTTL},
	} {
		if ttl.v < 0 {
			add("%s 不能为负（0 = 关闭缓存），实际 %v", ttl.name, ttl.v)
		}
	}

	// 非法档位此前会被闸门静默降级成 off —— 也就是**关掉反作弊**。
	// 那是安全上最不该静默的一种失败，改为启动失败。
	if !slices.Contains(ValidRecomputeModes, c.SettleRecomputeMode) {
		add("SETTLE_RECOMPUTE_MODE=%q 不是合法值（合法：%s）",
			c.SettleRecomputeMode, strings.Join(ValidRecomputeModes, "/"))
	}

	return errors.Join(errs...)
}
