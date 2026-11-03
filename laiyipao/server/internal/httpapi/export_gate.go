// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// 导出并发闸门（2026-11-02 第 13 轮）。
//
// 为什么要限并发：CSV 导出是后台最贵的读路径 ——
//   - 每个导出最多物化 10 万行（`exportTooLarge` 的上限）在内存里，
//     单次峰值几十 MB，且整块 `[]byte` 还要再走一遍响应写；
//   - 单次导出独占一条池连接，最长可耗到 `DB_STATEMENT_TIMEOUT`（默认 30s）。
//
// 池只有 `DB_MAX_CONNS`（默认 10）条连接。不限并发时，N 个运营同时点
// 「导出」就能把连接与内存一起吃光，**普通玩家请求**跟着排队/超时 ——
// 一个只读的运维操作不该有这种破坏力。
//
// 行为：超过上限的请求先排队（最多 `EXPORT_WAIT`，默认 3s），排不上就
// 立刻 503 `export_busy`（可重试），而不是无限期挂着或拖垮池。

import (
	"context"
	"time"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/httpapi/httperr"
)

// errExportBusy 是 httperr.ErrExportBusy 的本地别名（既有测试按此名引用）。
var errExportBusy = httperr.ErrExportBusy

type exportGate struct {
	sem  chan struct{}
	wait time.Duration
}

func newExportGate(max int, wait time.Duration) *exportGate {
	if max < 1 {
		max = 1
	}
	if wait < 0 {
		wait = 0
	}
	return &exportGate{sem: make(chan struct{}, max), wait: wait}
}

// acquire 占用一个导出槽位；返回的释放函数必须由调用方 defer 执行。
// 空槽位直接拿走；满则排队至多 wait（同时受 ctx 截止时间约束），
// 排不上返回 errExportBusy。
func (g *exportGate) acquire(ctx context.Context) (func(), error) {
	select {
	case g.sem <- struct{}{}:
		return g.release, nil
	default:
	}

	wait := g.wait
	if deadline, ok := ctx.Deadline(); ok {
		if remain := time.Until(deadline); remain < wait {
			wait = remain
		}
	}
	if wait <= 0 {
		return nil, errExportBusy
	}

	timer := time.NewTimer(wait)
	defer timer.Stop()
	select {
	case g.sem <- struct{}{}:
		return g.release, nil
	case <-timer.C:
		return nil, errExportBusy
	case <-ctx.Done():
		return nil, ctx.Err()
	}
}

func (g *exportGate) release() { <-g.sem }

// exportGate 返回进程内唯一的导出闸门（首次读取配置构造）。
func (s *Server) exportGate() *exportGate {
	s.exportsOnce.Do(func() {
		cfg := s.Svc.Cfg()
		s.exports = newExportGate(cfg.ExportMaxConcurrency, cfg.ExportWait)
	})
	return s.exports
}

// withExportSlot 把一个导出处理器包进并发闸门。
// 在路由层统一套用，避免「新增导出忘记加闸」——导出清单在 routes.go，
// 与其它读路由同处一处，漏加会在评审时直接可见。
func (s *Server) withExportSlot(h fiber.Handler) fiber.Handler {
	return func(c *fiber.Ctx) error {
		release, err := s.exportGate().acquire(c.UserContext())
		if err != nil {
			return err
		}
		defer release()
		return h(c)
	}
}
