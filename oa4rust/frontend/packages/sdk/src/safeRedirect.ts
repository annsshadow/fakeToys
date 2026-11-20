// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/** 登录后回跳的默认落地页。 */
const DEFAULT_REDIRECT = '/app/dashboard'

/**
 * 把一个可能来自 URL query 的回跳值收敛为同源站内路径。
 *
 * 历史写法只用 startsWith('/') && !startsWith('//') 判断，但 WHATWG URL 解析器把
 * 反斜杠当作斜杠处理：'/\\evil.example' 会被解析到外部源，造成登录后的开放重定向。
 * 这里先拒绝反斜杠/控制字符与跨 scheme，再用 URL 解析复核落地点 origin，
 * 确保 router.replace 永远留在当前站点。
 */
export function resolveSafeRedirect(raw: unknown): string {
  // vue-router 的可重复 query 值是数组；只取第一个，其余忽略。
  const candidate = typeof raw === 'string' ? raw : Array.isArray(raw) && typeof raw[0] === 'string' ? raw[0] : ''

  if (!candidate.startsWith('/') || candidate.startsWith('//')) return DEFAULT_REDIRECT

  for (const char of candidate) {
    const code = char.charCodeAt(0)
    if (char === '\\' || code < 0x20 || code === 0x7f) return DEFAULT_REDIRECT
  }

  try {
    const parsed = new URL(candidate, 'http://localhost')
    if (parsed.origin !== 'http://localhost' || parsed.protocol !== 'http:') return DEFAULT_REDIRECT
  } catch {
    return DEFAULT_REDIRECT
  }

  return candidate
}
