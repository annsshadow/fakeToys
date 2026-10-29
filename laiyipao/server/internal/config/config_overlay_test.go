package config

import (
	"os"
	"path/filepath"
	"testing"
	"time"
)

// 配置文件覆盖层：`CONFIG_FILE` 是热加载唯一的"可重读来源"。
//
// 环境变量在进程启动后改不了，所以「SIGHUP 重读配置」若只重读环境变量，
// 结果与启动时逐字节相同 —— 看起来成功、实际什么都没变。这里钉住
// 覆盖层的三条语义：标量转换、覆盖优先、空值回退。

func writeConfigFile(t *testing.T, body string) string {
	t.Helper()
	path := filepath.Join(t.TempDir(), "config.json")
	if err := os.WriteFile(path, []byte(body), 0o600); err != nil {
		t.Fatalf("写临时配置文件失败：%v", err)
	}
	return path
}

func TestLoadFileStringifiesScalars(t *testing.T) {
	path := writeConfigFile(t, `{
		"ADDR": ":9090",
		"DB_MAX_CONNS": 42,
		"METRICS_ENABLED": true,
		"RATE_PER_MIN": 12.0,
		"RATE_FRAC": 0.5,
		"EMPTY": null
	}`)
	m, err := LoadFile(path)
	if err != nil {
		t.Fatalf("LoadFile 失败：%v", err)
	}
	cases := map[string]string{
		"ADDR":            ":9090",
		"DB_MAX_CONNS":    "42", // 整数值不能变成 "42.000000"
		"METRICS_ENABLED": "true",
		"RATE_PER_MIN":    "12",
		"RATE_FRAC":       "0.5",
		"EMPTY":           "",
	}
	for k, want := range cases {
		if got := m[k]; got != want {
			t.Errorf("%s = %q，期望 %q", k, got, want)
		}
	}
}

func TestLoadFileRejectsNestedValues(t *testing.T) {
	path := writeConfigFile(t, `{"CORS_ALLOW_ORIGINS": ["https://a.example"]}`)
	if _, err := LoadFile(path); err == nil {
		t.Fatal("嵌套数组必须报错：猜一个序列化形式会让 envList 静默收到一个元素")
	}
}

func TestLoadFileReportsMissingFile(t *testing.T) {
	if _, err := LoadFile(filepath.Join(t.TempDir(), "nope.json")); err == nil {
		t.Fatal("文件不存在必须报错，而不是当成空配置继续")
	}
}

func TestOverlayOverridesEnvAndEmptyFallsBack(t *testing.T) {
	t.Setenv("DASHBOARD_CACHE_TTL", "30s")
	t.Setenv("ADDR", ":8080")
	t.Cleanup(func() { ApplyOverlay(nil) })

	ApplyOverlay(map[string]string{
		"DASHBOARD_CACHE_TTL": "90s",
		"ADDR":                "", // 空串 = 回退环境变量
	})

	if got := envDuration("DASHBOARD_CACHE_TTL", time.Second); got != 90*time.Second {
		t.Errorf("覆盖层应优先：实得 %v，期望 90s", got)
	}
	if got := env("ADDR", ":1"); got != ":8080" {
		t.Errorf("空值应回退环境变量：实得 %q，期望 :8080", got)
	}

	ApplyOverlay(nil)
	if got := envDuration("DASHBOARD_CACHE_TTL", time.Second); got != 30*time.Second {
		t.Errorf("清除覆盖层后应回到环境变量：实得 %v，期望 30s", got)
	}
}

func TestOverlaySnapshotIsACopy(t *testing.T) {
	t.Cleanup(func() { ApplyOverlay(nil) })
	ApplyOverlay(map[string]string{"ADDR": ":1111"})

	snap := OverlaySnapshot()
	snap["ADDR"] = ":2222"
	if got := env("ADDR", ":0"); got != ":1111" {
		t.Errorf("快照必须是副本：改动快照不应影响生效配置，实得 %q", got)
	}

	// 回滚语义：用快照还原后仍是原值。
	ApplyOverlay(map[string]string{"ADDR": ":3333"})
	ApplyOverlay(snap)
	if got := env("ADDR", ":0"); got != ":2222" {
		t.Errorf("用（被改过的）快照回滚应生效，实得 %q", got)
	}
}
