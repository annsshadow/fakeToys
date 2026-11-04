// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package config

// 本文件补配置解析/校验的**拒绝路径**。
//
// 这些值单独看都不致命，但错误的组合会在运行时变成静默故障：
// 连接寿命 -1s 等于每语句重建连接；离线收益上限 -1 小时等于永远不可领；
// 导出上限 -1 字节等于所有导出 413。启动时吵起来是唯一能救回来的时机
// （AGENTS #11 fail loud），所以逐项锚死「点名变量的错误」。

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// TestValidateRejectsNegativeTuning 负值的时间/数量参数必须全部被拒。
func TestValidateRejectsNegativeTuning(t *testing.T) {
	cases := map[string]string{
		"DB_CONN_LIFETIME":      "-1s",
		"DB_CONN_IDLE_TIME":     "-1s",
		"DB_STATEMENT_TIMEOUT":  "-1s",
		"MIN_BATTLE_DURATION":   "-1s",
		"EXPORT_WAIT":           "-1s",
		"EXPORT_MAX_BYTES":      "-1",
		"OFFLINE_CAP_HOURS":     "-1",
		"OFFLINE_COIN_PER_HOUR": "-1",
	}
	for key, val := range cases {
		t.Run(key, func(t *testing.T) {
			clearEnv(t)
			t.Setenv(key, val)
			err := Load().Validate()
			if err == nil {
				t.Fatalf("%s=%s 必须被拒绝", key, val)
			}
			if !strings.Contains(err.Error(), key) {
				t.Errorf("错误信息应点名 %s，实际：%v", key, err)
			}
		})
	}
}

// TestLoadFileRejectsMalformedJSON 配置文件语法坏掉时不能回退成默认值。
func TestLoadFileRejectsMalformedJSON(t *testing.T) {
	p := filepath.Join(t.TempDir(), "broken.json")
	if err := os.WriteFile(p, []byte(`{"JWT_SECRET": `), 0o600); err != nil {
		t.Fatalf("写临时配置失败：%v", err)
	}
	if _, err := LoadFile(p); err == nil || !strings.Contains(err.Error(), "解析配置文件") {
		t.Fatalf("坏 JSON 应报解析错误，实际：%v", err)
	}
}

// TestEnvListTrimsAndDropsEmpty 列表项的空白与空项必须被清掉。
//
// 前导空格会让 CORS 白名单永远匹配不上（Origin 头不含空格）；
// 空项会变成 allowSet[""]，让「无 Origin」的请求命中白名单。
func TestEnvListTrimsAndDropsEmpty(t *testing.T) {
	ApplyOverlay(nil)
	t.Cleanup(func() { ApplyOverlay(nil) })

	t.Setenv("R96_ORIGINS", " alpha , , beta ")
	got := envList("R96_ORIGINS")
	if len(got) != 2 || got[0] != "alpha" || got[1] != "beta" {
		t.Fatalf("应得到 [alpha beta]，实际：%#v", got)
	}

	t.Setenv("R96_ORIGINS", "   ")
	if got := envList("R96_ORIGINS"); got != nil {
		t.Fatalf("全空白应返回 nil，实际：%#v", got)
	}
}

// TestEnvFloatFallsBackOnGarbage 非法浮点回退默认值，合法值原样读出。
func TestEnvFloatFallsBackOnGarbage(t *testing.T) {
	ApplyOverlay(nil)
	t.Cleanup(func() { ApplyOverlay(nil) })

	t.Setenv("R96_RATIO", "1.5")
	if got := envFloat("R96_RATIO", 0); got != 1.5 {
		t.Fatalf("合法值应读出 1.5，实际：%v", got)
	}

	t.Setenv("R96_RATIO", "abc")
	if got := envFloat("R96_RATIO", 0.75); got != 0.75 {
		t.Fatalf("非法值应回退 0.75，实际：%v", got)
	}
}
