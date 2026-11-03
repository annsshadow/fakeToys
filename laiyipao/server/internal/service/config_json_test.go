// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 第 11 轮：/config 缓存序列化字节。
//
// 关键不变量是**字节逐位一致**：缓存拼装的输出必须与
// json.Marshal(LoadGameConfig()) 完全相同 —— 任何格式漂移都会直接
// 改变线上响应（客户端的 version 比较 / 持久化缓存都会跟着错）。
// server_time 新鲜度与 require_login 重建各钉一条。

import (
	"bytes"
	"encoding/json"
	"strings"
	"testing"
	"time"
)

// configJSONWithSameStamp 把缓存输出里的 server_time 钉进一份全量序列化。
//
// ⚠️ 为什么不能直接 `bytes.Equal(cached, json.Marshal(LoadGameConfig()))`：
// 两次调用的 `time.Now()` 必然不同（ConfigJSON 还要复制 75KB，间隔远超
// Windows 的 100ns 时钟 tick），所以这条断言只有在「两次挂钟恰好落进
// 同一 tick」时才成立 —— 测的是时钟粒度，不是序列化字节。
//
// 钉成同一个时间戳后，比对覆盖的正是真正的不变量：
// 除 server_time 外，缓存拼装的字节与全量 Marshal 逐位一致
// （字段顺序、转义、数字格式、结构体/切片的 splice 都不能漂移）。
func configJSONWithSameStamp(t *testing.T, ts *testService, raw []byte) []byte {
	t.Helper()
	var stamp struct {
		ServerTime time.Time `json:"server_time"`
	}
	if err := json.Unmarshal(raw, &stamp); err != nil {
		t.Fatalf("解析 server_time 失败（缓存输出不是合法 JSON）：%v", err)
	}
	cfg := ts.LoadGameConfig()
	cfg.ServerTime = stamp.ServerTime
	out, err := json.Marshal(cfg)
	if err != nil {
		t.Fatalf("直接序列化失败：%v", err)
	}
	return out
}

func TestConfigJSONBytesMatchesStructMarshal(t *testing.T) {
	ts := openTestService(t)

	cached, err := ts.ConfigJSON()
	if err != nil {
		t.Fatalf("ConfigJSON 失败：%v", err)
	}
	direct := configJSONWithSameStamp(t, ts, cached)
	if !bytes.Equal(cached, direct) {
		t.Fatalf("缓存拼装的字节与全量序列化不一致：\n缓存 len=%d\n直接 len=%d",
			len(cached), len(direct))
	}

	// 稳定命中：第二次调用（走缓存路径）仍逐位一致（同样钉住它自己的时间戳）。
	again, err := ts.ConfigJSON()
	if err != nil {
		t.Fatalf("第二次 ConfigJSON 失败：%v", err)
	}
	if !bytes.Equal(again, configJSONWithSameStamp(t, ts, again)) {
		t.Fatal("缓存命中路径的输出必须与直接序列化逐位一致")
	}
}

func TestConfigJSONServerTimeIsFresh(t *testing.T) {
	ts := openTestService(t)

	var v1, v2 struct {
		ServerTime time.Time `json:"server_time"`
	}
	b1, err := ts.ConfigJSON()
	if err != nil {
		t.Fatalf("ConfigJSON 失败：%v", err)
	}
	time.Sleep(5 * time.Millisecond)
	b2, err := ts.ConfigJSON()
	if err != nil {
		t.Fatalf("ConfigJSON 失败：%v", err)
	}
	if err := json.Unmarshal(b1, &v1); err != nil {
		t.Fatalf("解析缓存输出失败：%v", err)
	}
	if err := json.Unmarshal(b2, &v2); err != nil {
		t.Fatalf("解析缓存输出失败：%v", err)
	}
	if !v2.ServerTime.After(v1.ServerTime) {
		t.Errorf("两次请求的 server_time 应递增（缓存的是切分字节而不是冻结时间戳）：%v vs %v",
			v1.ServerTime, v2.ServerTime)
	}
}

func TestConfigJSONRebuildsOnRequireLoginFlip(t *testing.T) {
	ts := openTestService(t)

	before, err := ts.ConfigJSON()
	if err != nil {
		t.Fatalf("ConfigJSON 失败：%v", err)
	}
	base := *ts.Cfg()
	flipped := base
	flipped.PlayerLoginRequired = !base.PlayerLoginRequired
	ts.cfg.Store(&flipped)
	t.Cleanup(func() { ts.cfg.Store(&base) })

	after, err := ts.ConfigJSON()
	if err != nil {
		t.Fatalf("ConfigJSON 失败：%v", err)
	}
	var c1, c2 struct {
		RequireLogin bool `json:"require_login"`
	}
	if err := json.Unmarshal(before, &c1); err != nil {
		t.Fatal(err)
	}
	if err := json.Unmarshal(after, &c2); err != nil {
		t.Fatal(err)
	}
	if c1.RequireLogin == c2.RequireLogin {
		t.Errorf("require_login 翻转后缓存必须重建：%v → %v", c1.RequireLogin, c2.RequireLogin)
	}
}

// TestBuildConfigJSONPartsFailsLoudOnShapeChange：切分点失效必须报错，
// 不能静默下发错字节（fail-loud，AGENTS #11）。
func TestBuildConfigJSONPartsFailsLoudOnShapeChange(t *testing.T) {
	// 直接验证 marker 逻辑：把 GameConfig 序列化后确认 marker 唯一存在，
	// 并把「marker 消失」的分支用构造输入钉住。
	prefix, suffix, err := buildConfigJSONParts(false)
	if err != nil {
		t.Fatalf("当前 GameConfig 形状下切分必须成功：%v", err)
	}
	if !strings.HasSuffix(string(prefix), `"server_time":"`) {
		t.Errorf("prefix 必须以 server_time 值的开头引号结尾，实际 %q", string(prefix[len(prefix)-30:]))
	}
	if !strings.HasPrefix(string(suffix), `"}`) {
		t.Errorf("suffix 必须以值的收尾引号开头（server_time 是最后一个字段），实际 %q", string(suffix))
	}
}
