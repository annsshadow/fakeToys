// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/** R308 真 bug：HotpicApp/AppInfoApp 统计卡「加载中」标签恒定显示，加载完成后值归 0，
 * 既误导用户又是历史 loading 文本正则假阳性根源。契约从源码提取 stats computed
 * 数组并用桩求值，真实验证 loading=false/true 下的数组内容（非文本匹配）。 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const pages = [
  { label: 'HotpicApp', file: 'HotpicApp.vue' },
  { label: 'AppInfoApp', file: 'AppInfoApp.vue' },
]

function statsExpr(file: string): string {
  const src = readFileSync(resolve(import.meta.dirname, file), 'utf8')
  const m = src.match(/const stats\s*=\s*computed\(\(\)\s*=>\s*(\[[\s\S]*?\n\s*\])\)/)
  if (!m) throw new Error(file + ': 未找到 stats computed 数组')
  return m[1]
}

function evalStats(file: string, items: any[], loading: boolean): any[] {
  const expr = statsExpr(file)
  const fn = new Function('items', 'loading', 'return ' + expr) as (i: any, l: any) => any[]
  return fn({ value: items }, { value: loading })
}

describe('R308 统计卡「加载中」标签修正', () => {
  for (const p of pages) {
    it(p.label + ': loading=false 时 stats 不含 label=加载中 项', () => {
      const stats = evalStats(p.file, [], false)
      expect(stats.some((s: any) => s.label === '加载中')).toBe(false)
    })
    it(p.label + ': loading=true 时允许呈现加载态提示', () => {
      const stats = evalStats(p.file, [], true)
      const item = stats.find((s: any) => s.label === '加载中')
      expect(item).toBeDefined()
      expect(item.value).toBe(1)
    })
    it(p.label + ': 保留总计等其余统计项', () => {
      const stats = evalStats(p.file, [{ isHot: true, isCream: false, status: 'enabled' }], false)
      expect(stats.length).toBeGreaterThanOrEqual(3)
      expect(stats.some((s: any) => s.label === '总计')).toBe(true)
    })
  }
})
