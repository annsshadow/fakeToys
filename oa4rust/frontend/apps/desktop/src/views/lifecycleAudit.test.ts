// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readdirSync, readFileSync } from 'node:fs'
import { relative, resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

// 相对本文件：views → src → desktop → apps → frontend → oa4rust
const OA4RUST_ROOT = resolve(import.meta.dirname, '../../../../..')
const SCAN_ROOTS = [resolve(OA4RUST_ROOT, 'frontend/apps/desktop/src'), resolve(OA4RUST_ROOT, 'mobile/src')]

/**
 * 应用级单例：监听器随整个应用生命周期存在，不随页面卸载。
 * 只允许这四个「启动一次」的文件 + 移动端入口，新增文件必须显式在这里说明理由。
 */
const APP_LIFETIME_LISTENERS = new Set([
  'frontend/apps/desktop/src/main.ts',
  'frontend/apps/desktop/src/registerSW.ts',
  'frontend/apps/desktop/src/utils/sandbox.ts',
  'frontend/apps/desktop/src/utils/toast.ts',
  'mobile/src/main.ts',
])

function walk(dir: string, out: string[] = []): string[] {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = resolve(dir, entry.name)
    if (entry.isDirectory()) walk(full, out)
    else if (/\.(vue|ts)$/.test(entry.name) && !/\.test\.ts$/.test(entry.name)) out.push(full)
  }
  return out
}

function sources(): Array<{ file: string; path: string; source: string }> {
  return SCAN_ROOTS.flatMap((root) => walk(root)).map((file) => ({
    file,
    path: relative(OA4RUST_ROOT, file).replaceAll('\\', '/'),
    source: readFileSync(file, 'utf8'),
  }))
}

function count(source: string, needle: string): number {
  return [...source.matchAll(new RegExp(needle, 'g'))].length
}

describe('定时器/监听器泄漏审计（R142）', () => {
  it('注册了 setInterval 的文件必须同时清理（页面卸载后定时器不得残留）', () => {
    const files = sources()
    // 规模下限：防止扫描路径写错后断言空转。
    expect(files.length).toBeGreaterThanOrEqual(120)

    const intervalUsers: string[] = []
    for (const { path, source } of files) {
      const registered = count(source, 'setInterval\\(')
      if (registered === 0) continue
      intervalUsers.push(path)
      expect(
        count(source, 'clearInterval\\('),
        `${path} 注册了 setInterval 但没有 clearInterval`,
      ).toBeGreaterThanOrEqual(registered)
    }
    // 现状：Dashboard 时钟、FormDesigner、ProcessDesigner 三处；数量骤降说明扫描失效。
    expect(intervalUsers.length).toBeGreaterThanOrEqual(3)
  })

  it('页面级 addEventListener 必须解绑，只有应用级单例允许常驻', () => {
    const files = sources()
    const listenerUsers: string[] = []
    const unpaired: string[] = []

    for (const { path, source } of files) {
      const registered = count(source, 'addEventListener\\(')
      if (registered === 0) continue
      listenerUsers.push(path)
      if (APP_LIFETIME_LISTENERS.has(path)) continue
      const removed = count(source, 'removeEventListener\\(')
      if (removed < registered) unpaired.push(`${path}（注册 ${registered} / 解绑 ${removed}）`)
    }

    expect(listenerUsers.length).toBeGreaterThanOrEqual(5)
    expect(unpaired).toEqual([])
  })
})
