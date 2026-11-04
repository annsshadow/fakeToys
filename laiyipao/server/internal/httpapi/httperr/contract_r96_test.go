// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httperr_test

// 本文件补错误出口的「对外契约」：状态码 + 错误码 + Retry-After。
//
// 这三项是客户端分支的依据（401 重新登录、409 换幂等键、503 退避），
// 改错一项不会让编译失败、也不会让 happy path 红 —— 只会在线上
// 让客户端静默走进兜底分支。所以逐类锚死。

import (
	"fmt"
	"testing"

	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/httpapi/httperr"
	"github.com/laiyipao/server/internal/service"
)

func TestRenderContractPerErrorClass(t *testing.T) {
	cases := []struct {
		name    string
		err     error
		status  int
		code    string
		retryAF string
	}{
		{
			// 409 而不是 400：key 与请求体各自合法，只是组合冲突。
			name: "幂等键冲突", err: service.ErrIdemConflict,
			status: 409, code: httperr.CodeIdempotencyConflict,
		},
		{
			// 413 而不是 400：参数没写错，是筛选范围太宽。
			name: "导出超上限", err: fmt.Errorf("%w: 筛选范围过大", service.ErrExportTooLarge),
			status: 413, code: httperr.CodeExportTooLarge,
		},
		{
			// 503 必须带 Retry-After，否则客户端只能猜等多久。
			name: "导出闸门忙", err: httperr.ErrExportBusy,
			status: 503, code: httperr.CodeExportBusy, retryAF: "3",
		},
		{
			// 结算拒绝类：格式对、内容不可信 → 422。
			name: "结算拒绝", err: domain.ErrTokenUsed,
			status: 422, code: httperr.CodeSettleRejected,
		},
		{
			// 兜底：未知错误仍是 500，不能因为没登记就升格成 4xx。
			name: "未知错误", err: fmt.Errorf("boom"),
			status: 500, code: httperr.CodeInternalError,
		},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			assertRender(t, c.name, c.err, c.status, c.code, c.retryAF)
			if got := httperr.StatusOf(c.err); got != c.status {
				t.Fatalf("StatusOf 与 Render 分叉：StatusOf=%d Render=%d", got, c.status)
			}
		})
	}
}

// TestNilErrorIsNotTransient / TestRenderNilIsNoop 守住两个「空输入」守卫：
// 判据不能把 nil 当成故障，出口也不能对 nil 写响应。
func TestNilErrorGuards(t *testing.T) {
	if httperr.IsTransientDependency(nil) {
		t.Fatal("nil 不是依赖故障")
	}
	if err := httperr.Render(nil, nil); err != nil {
		t.Fatalf("Render(nil,nil) 应原样返回 nil（handler 已自行写响应），实际：%v", err)
	}
}
