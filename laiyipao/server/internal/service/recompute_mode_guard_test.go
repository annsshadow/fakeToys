// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R49：config.ValidRecomputeModes 与 service.RecomputeModes 的一致性守卫。
//
// 合法档位在两个包各有一份（config 不能反向 import service，否则成环）。
// 启动校验用 config 那份、运行时闸门用 service 那份 —— 两份漂移的后果是
// 「Validate 放行的档位闸门不认」，或反过来。

import (
	"slices"
	"testing"

	"github.com/laiyipao/server/internal/config"
)

func TestRecomputeModesMatchConfigWhitelist(t *testing.T) {
	if !slices.Equal(RecomputeModes, config.ValidRecomputeModes) {
		t.Fatalf("重算档位白名单漂移：service=%v config=%v",
			RecomputeModes, config.ValidRecomputeModes)
	}
}
