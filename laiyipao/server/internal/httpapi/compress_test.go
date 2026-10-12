package httpapi

// 响应压缩的契约测试。
//
// ## 为什么要测这个
//
// `/api/v1/config` 是本项目**最大的单个响应**，实测未压缩 **167,124 字节**，
// 其中 `levels` 一个字段就占 123,717 字节（74%）。
// 客户端是**每次冷启动都拉一次**（`loadConfig` 只有内存内缓存，
// 见 `miniapp/src/store/game.ts`），所以这是每次启动的固定流量。
//
// gzip 之后实测 23,964 字节（14.3%），省掉 85.7%。
//
// ## 判据为什么用「体积比」而不是「有没有 Content-Encoding」
//
// 后者只能证明「中间件挂上了」，证明不了「真的省了带宽」——
// 万一哪天数据变得不可压缩（比如换成二进制或已压缩的格式），
// 头还在，但一点体积都没省。
//
// 所以这里同时断言：
//  1. 客户端要求时**确实**被压缩（`Content-Encoding` 含 gzip）；
//  2. 压缩后**至少小一半**（实测 14.3%，留 3.5 倍余量，防抖动）；
//  3. **解压后与未压缩版逐字节相同** —— 压缩绝不能改变载荷。
//
// 第 3 条是这里最重要的一条：压缩中间件是**全局**的，
// 一旦配错（错误的 level、或对已压缩内容二次压缩），
// 表现是「客户端拿到坏数据」而不是「报错」。

import (
	"bytes"
	"compress/gzip"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/service"
)

// newConfigApp 用**真实的 Register 装配**（含全部中间件），
// 而不是手搭一个只有 compress 的最小 app ——
// 否则测的就不是线上那条链路。
//
// `&service.Service{}` 零值即可：`LoadGameConfig` 的方法体从未引用 `s`
// （纯内存构造），所以这里不连数据库。
func newConfigApp() *fiber.App {
	app := fiber.New()
	New(&service.Service{}).Register(app)
	return app
}

func fetchConfig(t *testing.T, app *fiber.App, acceptEncoding string) *http.Response {
	t.Helper()
	req := httptest.NewRequest(http.MethodGet, "/api/v1/config", nil)
	if acceptEncoding != "" {
		req.Header.Set("Accept-Encoding", acceptEncoding)
	}
	resp, err := app.Test(req, 20000)
	if err != nil {
		t.Fatalf("请求执行失败：%v", err)
	}
	return resp
}

func TestConfigRespectsAcceptEncoding(t *testing.T) {
	app := newConfigApp()

	t.Run("客户端要求 gzip 时确实被压缩", func(t *testing.T) {
		resp := fetchConfig(t, app, "gzip")
		defer resp.Body.Close()
		if resp.StatusCode != http.StatusOK {
			t.Fatalf("状态码 = %d，期望 200", resp.StatusCode)
		}
		ce := resp.Header.Get("Content-Encoding")
		if ce == "" {
			t.Fatal("Content-Encoding 为空 —— 压缩中间件没生效")
		}
		if !bytes.Contains([]byte(ce), []byte("gzip")) {
			t.Fatalf("Content-Encoding = %q，不含 gzip", ce)
		}
	})

	t.Run("客户端没要求时不压缩（不要白压）", func(t *testing.T) {
		resp := fetchConfig(t, app, "")
		defer resp.Body.Close()
		if ce := resp.Header.Get("Content-Encoding"); ce != "" {
			t.Errorf("客户端没声明 Accept-Encoding 却收到 Content-Encoding=%q —— "+
				"对不要求压缩的客户端压了，白白消耗 CPU", ce)
		}
		var m map[string]any
		if err := json.NewDecoder(resp.Body).Decode(&m); err != nil {
			t.Fatalf("未压缩响应应当是合法 JSON：%v", err)
		}
	})
}

func TestConfigCompressionActuallySavesBytes(t *testing.T) {
	app := newConfigApp()

	raw := func() []byte {
		resp := fetchConfig(t, app, "")
		defer resp.Body.Close()
		b, err := io.ReadAll(resp.Body)
		if err != nil {
			t.Fatalf("读未压缩响应失败：%v", err)
		}
		return b
	}()

	resp := fetchConfig(t, app, "gzip")
	defer resp.Body.Close()
	packed, err := io.ReadAll(resp.Body)
	if err != nil {
		t.Fatalf("读压缩响应失败：%v", err)
	}

	// 前提守卫：raw 非空，否则下面「至少小一半」会变成
	// 「0 小于 0」这类恒真断言 —— 而恒真断言等于没有断言。
	if len(raw) < 1000 {
		t.Fatalf("未压缩响应只有 %d 字节，低于预期量级 —— "+
			"这个测试的前提（config 是个大响应）可能已不成立，请重新确认", len(raw))
	}
	ratio := float64(len(packed)) * 100 / float64(len(raw))
	t.Logf("config 未压缩 %d 字节 → gzip %d 字节（%.1f%%）", len(raw), len(packed), ratio)

	// 实测 14.3%。阈值取 50%，留 3.5 倍余量：
	// 判据是「压缩确实省了带宽」，不是「精确命中某个数字」。
	if len(packed)*2 > len(raw) {
		t.Errorf("压缩后 %d 字节，超过了未压缩 %d 字节的一半 —— "+
			"压缩中间件虽然挂上了，但没真正省下带宽", len(packed), len(raw))
	}
}

func TestCompressedPayloadIsByteIdenticalToPlain(t *testing.T) {
	app := newConfigApp()

	plainResp := fetchConfig(t, app, "")
	defer plainResp.Body.Close()
	plain, err := io.ReadAll(plainResp.Body)
	if err != nil {
		t.Fatalf("读未压缩响应失败：%v", err)
	}

	gzResp := fetchConfig(t, app, "gzip")
	defer gzResp.Body.Close()
	packed, err := io.ReadAll(gzResp.Body)
	if err != nil {
		t.Fatalf("读压缩响应失败：%v", err)
	}

	zr, err := gzip.NewReader(bytes.NewReader(packed))
	if err != nil {
		t.Fatalf("响应体不是合法 gzip：%v", err)
	}
	defer zr.Close()
	back, err := io.ReadAll(zr)
	if err != nil {
		t.Fatalf("解压失败：%v", err)
	}

	// ⚠️ **不能**直接 `bytes.Equal(back, plain)` —— 那是我的第 15 次「前提不成立」。
	//
	// 第一版就是这么写的，红了。失败信息其实已经把真相说出来了：
	// 解压后 167,124 字节 vs 未压缩 167,124 字节 —— **长度相同但内容不同**。
	//
	// 原因是 `/config` 里有一个易变字段：`server_time` = time.Now()。
	// 也就是说**两次请求的响应本来就不该相同**，跟压缩无关。
	//
	// 第 150 轮起 `version` 改成了**内容哈希**，所以它不再易变，
	// `volatile` 里只剩 `server_time` 一个。这也正是客户端现在能
	// 按 version 复用本地缓存的原因（见 service.go 的 LoadGameConfig）。
	//
	// 所以判据改成：剔除这个字段后**深比较**。
	// 守的仍然是「压缩不能改变载荷」，只是承认了配置里确实有随时间变的部分。
	var a, b map[string]any
	if err := json.Unmarshal(plain, &a); err != nil {
		t.Fatalf("未压缩响应不是合法 JSON：%v", err)
	}
	if err := json.Unmarshal(back, &b); err != nil {
		t.Fatalf("解压后不是合法 JSON：%v", err)
	}
	volatile := []string{"server_time"}
	// 先确认易变字段确实在响应里 —— 否则上面的「剔除」就成了无意义的仪式。
	var rawPlain map[string]any
	if err := json.Unmarshal(plain, &rawPlain); err != nil {
		t.Fatalf("未压缩响应不是合法 JSON：%v", err)
	}
	for _, k := range volatile {
		if _, ok := rawPlain[k]; !ok {
			t.Errorf("响应里没有 %q 字段 —— 本测试「剔除易变字段」的前提已不成立，"+
				"请重新确认 config 是否已变成确定性的", k)
		}
	}

	// 深比较：剔除易变字段后两者必须完全一致。
	strip := func(m map[string]any) []byte {
		cp := make(map[string]any, len(m))
		for k, v := range m {
			cp[k] = v
		}
		for _, k := range volatile {
			delete(cp, k)
		}
		b, _ := json.Marshal(cp)
		return b
	}
	ja, jb := strip(rawPlain), strip(b)
	if !bytes.Equal(ja, jb) {
		t.Errorf("剔除 %v 后两份载荷仍不一致 —— 压缩改变了内容\n未压缩 %d 字节\n解压后 %d 字节",
			volatile, len(ja), len(jb))
	}
}

// TestConfigVolumeIsKnown 记录 /config 的体积量级。
//
// 它不是性能断言（那类断言容易抖动），而是**量级哨兵**：
// 有人加了一大批关卡或把某个字段换成不可压缩的格式时，这里会先响。
//
// 判据给到 2 倍余量：实测 167,124 字节，阈值 350,000。
func TestConfigVolumeIsKnown(t *testing.T) {
	app := newConfigApp()
	resp := fetchConfig(t, app, "")
	defer resp.Body.Close()
	n, err := io.ReadAll(resp.Body)
	if err != nil {
		t.Fatalf("读响应失败：%v", err)
	}
	t.Logf("config 未压缩体积 %d 字节（%.1f KB）", len(n), float64(len(n))/1024)
	if len(n) > 350_000 {
		t.Errorf("config 未压缩体积 %d 字节，超过已知量级（实测 167,124 / 阈值 350,000）—— "+
			"若是有意扩充数据，请同步更新这个阈值与 service.go 里的注释", len(n))
	}
}

// TestConfigConditionalRequest 钉住 `?if_version=` 条件请求。
//
// 这是 `version` 字段唯一的使用方式，也是每轮冷启动能省下
// 24KB gzip 传输的原因。三条判据各守一半：
//
//	① version 对得上 → 304 且**零载荷**（不是 200 带一份重复内容）；
//	② version 对不上（含空串/乱码）→ 200 带全量；
//	③ 304 路径绝不泄漏响应体 —— 否则客户端会解析出一个空 config，
//	   而症状是「界面空白但不是网络错误」，极难排查。
func TestConfigConditionalRequest(t *testing.T) {
	app := newConfigApp()

	// 先拿一次真实响应，取出里面的 version 当作"客户端已有的版本"
	first := fetchConfig(t, app, "")
	raw, err := io.ReadAll(first.Body)
	first.Body.Close()
	if err != nil {
		t.Fatalf("读配置失败：%v", err)
	}
	var body map[string]any
	if err := json.Unmarshal(raw, &body); err != nil {
		t.Fatalf("配置不是合法 JSON：%v", err)
	}
	version, _ := body["version"].(string)
	if version == "" {
		t.Fatal("配置里没有 version（内容哈希）—— 条件请求无从比起")
	}

	t.Run("version 匹配 → 304 零载荷", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/api/v1/config?if_version="+version, nil)
		resp, err := app.Test(req, 20000)
		if err != nil {
			t.Fatalf("请求执行失败：%v", err)
		}
		defer resp.Body.Close()
		if resp.StatusCode != http.StatusNotModified {
			b, _ := io.ReadAll(resp.Body)
			t.Fatalf("version 匹配应返回 304，实际 %d（body %.120s）", resp.StatusCode, b)
		}
		b, err := io.ReadAll(resp.Body)
		if err != nil {
			t.Fatalf("读 304 响应体失败：%v", err)
		}
		if len(b) != 0 {
			t.Errorf("304 响应体应为空，实际 %d 字节 —— 客户端会把它当成一份空配置", len(b))
		}
	})

	t.Run("version 不匹配 → 200 全量", func(t *testing.T) {
		for _, v := range []string{"", "deadbeefdeadbeef", "not-a-real-version"} {
			req := httptest.NewRequest(http.MethodGet, "/api/v1/config?if_version="+v, nil)
			resp, err := app.Test(req, 20000)
			if err != nil {
				t.Fatalf("请求执行失败：%v", err)
			}
			b, err := io.ReadAll(resp.Body)
			resp.Body.Close()
			if err != nil {
				t.Fatalf("读响应失败：%v", err)
			}
			if resp.StatusCode != 200 {
				t.Errorf("if_version=%q 应返回 200，实际 %d", v, resp.StatusCode)
			}
			if len(b) == 0 {
				t.Errorf("if_version=%q 返回了空响应体 —— 客户端会当成本地缓存就是最新", v)
			}
		}
	})
}
