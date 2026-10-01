// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readdirSync, readFileSync, statSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { BACKEND_ROUTE_METHODS } from './backend-route-methods.fixture'

/**
 * HTTP 方法契约守卫（优化轮12 institutionalize 轮10 的修复）。
 *
 * 既有 desktop/mobile-endpoints 守卫只校验「路径是否注册」，不看 HTTP 方法——
 * 于是 RecycleApp 打 DELETE /api/recycle/{id}（后端仅 GET）、ProgramCenter 打
 * POST /api/reset（后端 PUT）这类 405 错配长期漏网（轮10 才靠临时脚本抓出）。
 * 本守卫把「前端调用的 (method, path) 必须命中后端注册的方法集」钉成 CI 断言。
 *
 * 匹配口径与 backend-route-methods.fixture 对齐：路径归一为小写、动态段/占位段
 * 折成 {}、去尾斜杠。fixture 由 gen_mcp_tools.py 从各 crate 的 axum .route 注册
 * 重生成，不手写。EXEMPT 收录经人工核实的合理例外（如同 handler 多方法别名）。
 */

const SRC_ROOTS = [
  resolve(import.meta.dirname, '../../frontend/apps/desktop/src'),
  resolve(import.meta.dirname, '../../mobile/src'),
]

// 经人工核实的方法契约例外（当前为空——轮10 已把两处真错配修正）。
const EXEMPT = new Set<string>([])

function norm(p: string): string {
  return p
    .replace(/\$\{[^}]*\}/g, '{}')
    .replace(/\{[^}]*\}/g, '{}')
    .replace(/\/+$/, '')
    .toLowerCase()
}

function collectCalls(): { method: string; path: string; file: string }[] {
  const out: { method: string; path: string; file: string }[] = []
  const re = /\b(?:api|mapi)\.(get|post|put|delete)(?:<[^>]*>)?\(\s*[`'"](\/api\/[^`'"?]*)/g
  function walk(dir: string): void {
    for (const name of readdirSync(dir)) {
      const full = resolve(dir, name)
      const st = statSync(full)
      if (st.isDirectory()) walk(full)
      else if (/\.(ts|vue)$/.test(name) && !name.endsWith('.test.ts')) {
        const text = readFileSync(full, 'utf8')
        for (const m of text.matchAll(re)) {
          out.push({ method: m[1]!.toUpperCase(), path: norm(m[2]!), file: name })
        }
      }
    }
  }
  for (const root of SRC_ROOTS) walk(root)
  return out
}

describe('前端 HTTP 方法契约', () => {
  it('每个前端 (method, path) 调用命中后端注册的方法集（路径已注册的前提下）', () => {
    const mismatches: string[] = []
    for (const { method, path, file } of collectCalls()) {
      const methods = BACKEND_ROUTE_METHODS[path]
      if (!methods) continue // 路径未注册是路径守卫的职责，此处只判方法
      if (methods.includes(method)) continue
      if (EXEMPT.has(`${method} ${path}`)) continue
      mismatches.push(`${file}: ${method} ${path} — 后端仅 [${methods.join(', ')}]`)
    }
    expect(mismatches, `方法错配（405 风险）:\n${mismatches.join('\n')}`).toEqual([])
  })
})
