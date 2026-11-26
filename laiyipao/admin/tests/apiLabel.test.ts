// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 顶栏 API 地址脱敏契约（t33）。
 * 合规红线：真实服务器地址（域名/IP）不得出现在截图/日志/DOM。
 * 顶栏默认只显示脱敏占位；仅本地回环或显式调试开关才显示完整值。
 */
import { describe, expect, it } from 'vitest'
import { maskApiBase, isLoopbackApiBase } from '@/utils/apiLabel'

describe('顶栏 API 地址脱敏', () => {
  it('非回环域名不出现在标签文本（默认脱敏为 API 占位）', () => {
    const raw = 'https://ops.example.com/api/v1'
    const label = maskApiBase(raw)
    expect(label).toBe('API')
    expect(label).not.toContain('ops.example.com')
    expect(label).not.toContain('example')
  })

  it('非回环裸 IP 也不出现在标签文本', () => {
    const label = maskApiBase('203.0.113.42:8080')
    expect(label).not.toContain('203.0.113.42')
  })

  it('回环 127.0.0.1:8080 仍可见，便于本地调试', () => {
    expect(maskApiBase('127.0.0.1:8080')).toBe('API 127.0.0.1:8080')
  })

  it('localhost 仍可见', () => {
    expect(maskApiBase('http://localhost:5173/api')).toContain('localhost')
  })

  it('IPv6 回环 [::1] 判定为回环', () => {
    expect(isLoopbackApiBase('http://[::1]:8080')).toBe(true)
  })

  it('未配置时回退到回环默认值，本地可见', () => {
    expect(maskApiBase(undefined)).toBe('API 127.0.0.1:8080')
  })

  it('显式调试开关开启时非回环地址完整显示', () => {
    expect(maskApiBase('https://ops.example.com/api/v1', true)).toBe('API https://ops.example.com/api/v1')
  })
})