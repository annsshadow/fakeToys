// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// 导出并发闸门测试（2026-11-02 第 13 轮）。
//
// 覆盖三件事：
//  1. 槽位满且排队超时 → 明确的 busy 错误（不是无限期挂住）；
//  2. 槽位在 wait 窗口内释放 → 排队者能拿到槽位（排队是有意义的）；
//  3. HTTP 层：占满槽位时导出返回 503 `export_busy`，释放后恢复 200。

import (
	"context"
	"net/http"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/config"
)

func TestExportGateRejectsWhenFullAndRecovers(t *testing.T) {
	gate := newExportGate(1, 10*time.Millisecond)

	release, err := gate.acquire(context.Background())
	if err != nil {
		t.Fatalf("空槽位应直接拿到：%v", err)
	}

	start := time.Now()
	if _, err := gate.acquire(context.Background()); err == nil {
		t.Fatal("槽位已满且排队超时，必须返回 busy")
	}
	if elapsed := time.Since(start); elapsed > 500*time.Millisecond {
		t.Fatalf("排队应受 EXPORT_WAIT 约束，实际等了 %v", elapsed)
	}

	release()

	release2, err := gate.acquire(context.Background())
	if err != nil {
		t.Fatalf("释放后应能重新拿到槽位：%v", err)
	}
	release2()
}

func TestExportGateWaitsForSlotReleasedInTime(t *testing.T) {
	gate := newExportGate(1, 200*time.Millisecond)

	release, err := gate.acquire(context.Background())
	if err != nil {
		t.Fatalf("首个槽位应成功：%v", err)
	}
	go func() {
		time.Sleep(20 * time.Millisecond)
		release()
	}()

	start := time.Now()
	release2, err := gate.acquire(context.Background())
	if err != nil {
		t.Fatalf("wait 窗口内释放的槽位应被排队者拿到：%v", err)
	}
	if elapsed := time.Since(start); elapsed < 10*time.Millisecond {
		t.Fatalf("排队者必须真的等待，实际 %v", elapsed)
	}
	release2()
}

func TestExportGateHonoursContextDeadline(t *testing.T) {
	// wait 很长，但请求自身的 deadline 更短：必须按 deadline 退出，
	// 否则一个已没人在等的导出会继续占着槽位。
	gate := newExportGate(1, 5*time.Second)
	release, err := gate.acquire(context.Background())
	if err != nil {
		t.Fatalf("首个槽位应成功：%v", err)
	}
	defer release()

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Millisecond)
	defer cancel()

	start := time.Now()
	if _, err := gate.acquire(ctx); err == nil {
		t.Fatal("已过期的 ctx 不应拿到槽位")
	}
	if elapsed := time.Since(start); elapsed > 2*time.Second {
		t.Fatalf("应按 ctx deadline 退出，实际等了 %v", elapsed)
	}
}

// 占满槽位后导出端点必须 fail loud：503 + export_busy（可重试），
// 而不是无限期排队、也不是 500。释放后同一个端点恢复 200。
func TestE2EExportBusyReturns503(t *testing.T) {
	e2eScratchDB(t)
	e := newE2EWith(t, func(cfg *config.Config) {
		cfg.ExportMaxConcurrency = 1
		cfg.ExportWait = 0 // 满则立即 503，测试无需等 3 秒
	})

	_, adminTok := e.newAdmin(t, "exportbusy")

	release, err := e.srv.exportGate().acquire(context.Background())
	if err != nil {
		t.Fatalf("占位失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/admin/orders/export", nil)
	req.Header.Set("Authorization", "Bearer "+adminTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusServiceUnavailable {
		t.Fatalf("槽位占满时应 503，实际 %d", resp.StatusCode)
	}
	// 释放后同一端点必须恢复（闸门不能永久卡死）。
	release()

	req2, _ := http.NewRequest("GET", "/api/v1/admin/orders/export", nil)
	req2.Header.Set("Authorization", "Bearer "+adminTok)
	resp2, err := e.app.Test(req2, 10000)
	if err != nil {
		t.Fatalf("第二次请求失败：%v", err)
	}
	defer resp2.Body.Close()
	if resp2.StatusCode != fiber.StatusOK {
		t.Fatalf("释放后导出应 200，实际 %d", resp2.StatusCode)
	}
}
