// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 顶栏 API 地址标签的脱敏规则（t33 合规红线）。
 *
 * 真实服务器地址（域名/IP）一律不入库/日志/截图/对话。顶栏默认只显示
 * 「API」占位；仅两类豁免可显示完整值：
 *   1. 本地回环（127.0.0.1 / localhost / ::1 / 0.0.0.0）—— 开发调试用，非真值；
 *   2. 显式调试开关 VITE_SHOW_API_HOST=1 —— 运维排障时主动开启。
 *
 * 判定集中在此纯函数，便于契约测试钉死两场景，UI 不再自己拼标签。
 */

/** 是否本地回环地址（支持裸 host:port 与带协议 URL 两种写法）。 */
export function isLoopbackApiBase(raw: string | undefined): boolean {
  const value = (raw ?? '').trim()
  if (!value) return false
  const hostMatch = value.match(/^(?:[a-z][a-z0-9+.-]*:\/\/)?(\[[^\]]+\]|[^/:?#]+)/i)
  const host = (hostMatch?.[1] ?? value).replace(/^\[|\]$/g, '').toLowerCase()
  return host === 'localhost' || host === '::1' || host === '0.0.0.0' || /^127\./.test(host)
}

export function maskApiBase(raw: string | undefined, debugShow = false): string {
  const fallback = '127.0.0.1:8080'
  const value = (raw ?? fallback).trim() || fallback
  if (debugShow || isLoopbackApiBase(value)) return `API ${value}`
  return 'API'
}