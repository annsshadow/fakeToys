// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package config

// R49：启动期配置语义校验的守卫。
//
// 改前证据（临时探针，已删）：
//
//	t.Setenv("ACCESS_TTL", "0s") / REFRESH_TTL=0s / DB_MAX_CONNS=0 /
//	EXPORT_MAX_CONCURRENCY=0 / SETTLE_RECOMPUTE_MODE=bogus 后
//	Load() 静默返回 ACCESS_TTL=0s REFRESH_TTL=0s DB_MAX_CONNS=0
//	EXPORT_MAX_CONCURRENCY=0 MODE="bogus"（无 error）
//
// 也就是说：这些值会让「令牌一签发即过期」「导出永远排不上」
// 「反作弊静默关闭」，而服务照样启动。

import (
	"strings"
	"testing"
)

// TestValidateAcceptsDefaults 保证守卫不会误伤正常启动：
// 什么都不配（全部默认值）时 Validate 必须通过。
func TestValidateAcceptsDefaults(t *testing.T) {
	clearEnv(t)
	cfg := Load()
	if err := cfg.Validate(); err != nil {
		t.Fatalf("默认配置不应被判为非法：%v", err)
	}
}

// TestValidateRejectsZeroTokenTTL 守住最隐蔽的锁死：
// 令牌有效期 <= 0 时签发成功但立刻失效，玩家「既不能用新也不能用旧」。
func TestValidateRejectsZeroTokenTTL(t *testing.T) {
	for _, key := range []string{"ACCESS_TTL", "REFRESH_TTL", "BATTLE_TOKEN_TTL"} {
		t.Run(key, func(t *testing.T) {
			clearEnv(t)
			t.Setenv(key, "0s")
			cfg := Load()
			err := cfg.Validate()
			if err == nil {
				t.Fatalf("%s=0s 必须被拒绝", key)
			}
			if !strings.Contains(err.Error(), key) {
				t.Errorf("错误信息应点名 %s，实际：%v", key, err)
			}
		})
	}
}

// TestValidateRejectsZeroAcquireTimeout 守住 R90 的防护：
// DB_ACQUIRE_TIMEOUT=0/负值意味着「池耗尽时请求无限挂起」（R89 F2 实测
// 挂起 15.0s、无任何一层掐断）。这不是「关闭开关」，而是缺失防护 ——
// 必须启动失败，而不是静默退化成 R89 的挂起语义。
func TestValidateRejectsZeroAcquireTimeout(t *testing.T) {
	for _, val := range []string{"0s", "0", "-1s"} {
		t.Run(val, func(t *testing.T) {
			clearEnv(t)
			t.Setenv("DB_ACQUIRE_TIMEOUT", val)
			err := Load().Validate()
			if err == nil {
				t.Fatalf("DB_ACQUIRE_TIMEOUT=%s 必须被拒绝", val)
			}
			if !strings.Contains(err.Error(), "DB_ACQUIRE_TIMEOUT") {
				t.Errorf("错误信息应点名 DB_ACQUIRE_TIMEOUT，实际：%v", err)
			}
		})
	}
}

// TestValidateRejectsUnknownRecomputeMode 守住安全默认：
// 非法重算档位此前静默降级成 off（关掉 replay_skills 反作弊），
// 现在必须让启动失败。
func TestValidateRejectsUnknownRecomputeMode(t *testing.T) {
	clearEnv(t)
	t.Setenv("SETTLE_RECOMPUTE_MODE", "bogus")
	err := Load().Validate()
	if err == nil {
		t.Fatal("非法 SETTLE_RECOMPUTE_MODE 必须被拒绝")
	}
	if !strings.Contains(err.Error(), "SETTLE_RECOMPUTE_MODE") {
		t.Errorf("错误信息应点名 SETTLE_RECOMPUTE_MODE，实际：%v", err)
	}

	// 三个合法档位必须全部通过。
	for _, mode := range ValidRecomputeModes {
		t.Run(mode, func(t *testing.T) {
			clearEnv(t)
			t.Setenv("SETTLE_RECOMPUTE_MODE", mode)
			if err := Load().Validate(); err != nil {
				t.Fatalf("合法档位 %s 不应被拒：%v", mode, err)
			}
		})
	}
}

// TestValidateAggregatesErrors 要求一次报告全部问题：
// 部署时不该改一个、重启一次、再发现下一个。
func TestValidateAggregatesErrors(t *testing.T) {
	clearEnv(t)
	t.Setenv("DB_MAX_CONNS", "0")
	t.Setenv("EXPORT_MAX_CONCURRENCY", "0")
	t.Setenv("SHOP_CACHE_TTL", "-1s")
	err := Load().Validate()
	if err == nil {
		t.Fatal("三个非法值必须整体被拒绝")
	}
	for _, key := range []string{"DB_MAX_CONNS", "EXPORT_MAX_CONCURRENCY", "SHOP_CACHE_TTL"} {
		if !strings.Contains(err.Error(), key) {
			t.Errorf("错误信息应同时包含 %s，实际：%v", key, err)
		}
	}
}

// TestValidRecomputeModesMatchesService 守住 config 与 service 的两份档位表。
//
// config 不能 import service（service 依赖 config），所以合法档位在两个包
// 各有一份。这里放在 config 侧会形成循环，所以由 service 侧守住 ——
// 见 internal/service/recompute_mode_guard_test.go。
