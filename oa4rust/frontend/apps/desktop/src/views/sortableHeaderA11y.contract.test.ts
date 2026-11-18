// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R220 可排序表头无障碍契约测试（WCAG 2.1.1 Keyboard / 2.4.7 Focus / 1.3.1）。
 *
 * QueryManagerDeep 与 QueryStatementDesigner 的结果表通过点击 <th> 切换列排序，
 * 但这些 <th> 只有 @click：键盘用户无法聚焦/激活，读屏用户也听不到当前排序列与
 * 方向。契约钉住：每个可排序 th 必须可 Tab 聚焦、响应 Enter/Space，并以 aria-sort
 * 暴露 ascending / descending / none 三态（WCAG 1.3.1 Info and Relationships）。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

function readView(name: string): string {
  return readFileSync(resolve(import.meta.dirname, name), 'utf8')
}

/** 截取包含指定 @click 表达式的 <th> 开始标签（跨行）。 */
function sortableThTag(source: string, clickExpr: string): string {
  const clickIdx = source.indexOf(`@click="${clickExpr}"`)
  expect(clickIdx, `未找到 @click="${clickExpr}"`).toBeGreaterThan(-1)
  const thIdx = source.lastIndexOf('<th', clickIdx)
  expect(thIdx, '未找到对应的 <th 开始标签').toBeGreaterThan(-1)
  const endIdx = source.indexOf('>', clickIdx)
  expect(endIdx, '未找到 <th> 结束符').toBeGreaterThan(-1)
  return source.slice(thIdx, endIdx + 1)
}

const cases = [
  { file: 'QueryManagerDeep.vue', click: 'toggleSort(h)', label: '查询管理深化结果表' },
  { file: 'QueryStatementDesigner.vue', click: 'sortResult(h)', label: 'SQL 语句设计器结果表' },
]

for (const { file, click, label } of cases) {
  describe(`${label} 可排序表头无障碍（R220 / WCAG 2.1.1、1.3.1）`, () => {
    it('表头可键盘聚焦（tabindex=0）', () => {
      expect(sortableThTag(readView(file), click)).toMatch(/\btabindex="0"/)
    })

    it('Enter 与 Space 都能触发排序，且 Space 阻止页面滚动', () => {
      const tag = sortableThTag(readView(file), click)
      expect(tag).toMatch(/@keydown\.enter=/)
      expect(tag).toMatch(/@keydown\.space\.prevent=/)
    })

    it('以 aria-sort 暴露 ascending/descending/none 三态', () => {
      const tag = sortableThTag(readView(file), click)
      expect(tag).toMatch(/:aria-sort=/)
      expect(tag).toMatch(/ascending/)
      expect(tag).toMatch(/descending/)
      expect(tag).toMatch(/'none'/)
    })
  })
}
