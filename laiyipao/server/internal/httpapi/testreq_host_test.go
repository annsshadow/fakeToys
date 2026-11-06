// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// testReq 构造一个能被 fiber app.Test 正常解析的请求。
//
// fasthttp v1.70（GO-2026-4950 的修复版）起，服务端按 RFC 7230 校验
// HTTP/1.1 请求必须带 Host；fiber 的 app.Test 会把 *http.Request 原样
// 写进连接，而本包测试大量使用相对 URL（http.NewRequest("GET", "/api/v1/me", nil)），
// 这类请求没有 Host，会被直接拒绝（"missing required Host header"）。
// 统一在这里补 Host，测试用例本身不做任何改动。

import (
	"io"
	"net/http"
)

func testReq(method, target string, body io.Reader) (*http.Request, error) {
	req, err := http.NewRequest(method, target, body)
	if err != nil {
		return nil, err
	}
	req.Host = "test.local"
	return req, nil
}
