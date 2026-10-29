package service

import (
	"sync"
	"testing"
	"time"
)

// 配置热加载（LY-5）：`ReloadConfig` 换的是**快照**，而档位的运行期真相
// 仍是 gate。这两件事的边界必须钉住，否则「改了文件但被 API 切档覆盖」或者
// 「一次只改 TTL 的重载把档位抹回去」都会静默发生。

// TestReloadConfigSwapsSnapshot 钉住「重载后读到的就是新值」。
//
// 判据取 TTL 而不是随便一个字段：`AdminDashboard` 与 `Leaderboard` 是
// **每次请求现读** `s.Cfg().X`（stats.go / economy.go），所以这条断言
// 对应的就是真实生效路径，不是"字段被赋值了"。
func TestReloadConfigSwapsSnapshot(t *testing.T) {
	ts := openTestService(t)
	svc := ts.Service

	base := *svc.Cfg()
	next := base
	next.DashboardCacheTTL = base.DashboardCacheTTL + 17*time.Second
	next.LeaderboardCacheTTL = base.LeaderboardCacheTTL + 3*time.Second

	svc.ReloadConfig(next)

	if got := svc.Cfg().DashboardCacheTTL; got != next.DashboardCacheTTL {
		t.Fatalf("看板 TTL 未生效：实得 %v，期望 %v", got, next.DashboardCacheTTL)
	}
	if got := svc.Cfg().LeaderboardCacheTTL; got != next.LeaderboardCacheTTL {
		t.Fatalf("榜单 TTL 未生效：实得 %v，期望 %v", got, next.LeaderboardCacheTTL)
	}
}

// TestReloadConfigKeepsRuntimeModeWhenFileModeUnchanged
//
// 场景：运维用 `PUT /admin/recomputes/mode` 把档切到 shadow（只换 gate、
// 不写文件），随后有人改了别的配置并发 SIGHUP。此时文件里的档位**没变**，
// 重载不该按文件重建 gate —— 否则刚切的档会被悄悄抹回 enforce。
func TestReloadConfigKeepsRuntimeModeWhenFileModeUnchanged(t *testing.T) {
	ts := openTestService(t)
	svc := ts.Service

	base := *svc.Cfg()
	if base.SettleRecomputeMode == RecomputeShadow {
		// 默认档就是 shadow 时换一个基线，否则这条用例证明不了「没被覆盖」。
		base.SettleRecomputeMode = RecomputeEnforce
		svc.ReloadConfig(base)
	}
	if err := svc.SetRecomputeMode(RecomputeShadow); err != nil {
		t.Fatalf("切档失败：%v", err)
	}

	next := base // 档位字段与快照里的一致
	next.DashboardCacheTTL = base.DashboardCacheTTL + time.Second
	svc.ReloadConfig(next)

	if got := svc.RecomputeMode(); got != RecomputeShadow {
		t.Fatalf("文件里档位没变，重载不该覆盖运行期切档：实得 %q，期望 %q", got, RecomputeShadow)
	}
	if got := svc.Cfg().DashboardCacheTTL; got != next.DashboardCacheTTL {
		t.Fatalf("其它字段仍应生效：实得 %v，期望 %v", got, next.DashboardCacheTTL)
	}
}

// TestReloadConfigRebuildsGateWhenFileModeChanges
//
// 反过来：文件里**显式改了**档位，重载就必须重建 gate —— 否则热加载
// 在这一项上又是表演（配置改了、生效档没变）。
func TestReloadConfigRebuildsGateWhenFileModeChanges(t *testing.T) {
	ts := openTestService(t)
	svc := ts.Service

	base := *svc.Cfg()
	target := RecomputeEnforce
	if base.SettleRecomputeMode == RecomputeEnforce {
		target = RecomputeShadow
	}

	next := base
	next.SettleRecomputeMode = target
	svc.ReloadConfig(next)

	if got := svc.RecomputeMode(); got != target {
		t.Fatalf("文件里改了档位，gate 必须跟着重建：实得 %q，期望 %q", got, target)
	}
	if got := svc.Cfg().SettleRecomputeMode; got != target {
		t.Fatalf("快照里的档位应为 %q，实得 %q", target, got)
	}
}

// TestReloadConfigConcurrentWithReads 钉住「热加载 × 请求读配置」无数据竞争。
//
// 这是本次改动引入的**新并发面**：`ReloadConfig` 换指针，而每个请求都会
// `s.Cfg()` 现读 TTL、`RecomputeModeState` 读 gate。`-race` 下必须干净。
//
// 判据不是"跑完没崩"：是 `go test -race` 不报竞争。所以这条用例只在
// `-race` 下有意义，非 race 构建里它退化成一个并发冒烟。
func TestReloadConfigConcurrentWithReads(t *testing.T) {
	ts := openTestService(t)
	svc := ts.Service
	base := *svc.Cfg()

	var wg sync.WaitGroup
	stop := make(chan struct{})

	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				select {
				case <-stop:
					return
				default:
				}
				c := svc.Cfg()
				_ = c.DashboardCacheTTL
				_ = c.LeaderboardCacheTTL
				_ = svc.RecomputeModeState()
			}
		}()
	}

	for i := 0; i < 100; i++ {
		next := base
		next.DashboardCacheTTL = base.DashboardCacheTTL + time.Duration(i)*time.Millisecond
		svc.ReloadConfig(next)
	}
	close(stop)
	wg.Wait()
}
