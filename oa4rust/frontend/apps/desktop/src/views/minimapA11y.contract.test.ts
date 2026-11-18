// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R221 迷你地图无障碍契约测试（WCAG 2.1.1 Keyboard / 4.1.2 Name-Role-Value / 2.4.7 Focus Visible）。
 *
 * ProcessDesigner 的迷你地图 canvas 带 @click="minimapClick"，鼠标用户可点击它
 * 将画布平移到最近的流程节点；但 canvas 默认不可聚焦、没有角色与可访问名称、也不响应
 * 任何键盘事件，键盘与读屏用户完全无法使用这项导航。契约钉住：该 canvas 必须
 * 可 Tab 聚焦（tabindex=0）、以 role/aria-label 暴露名称、支持方向键选择节点与
 * Enter/Space 定位，并提供 :focus-visible 可见焦点样式。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

function readView(name: string): string {
  return readFileSync(resolve(import.meta.dirname, name), 'utf8')
}

/** 截取包含 @click="minimapClick" 的 <canvas> 开始标签（跨行）。 */
function minimapCanvasTag(source: string): string {
  const clickIdx = source.indexOf('@click="minimapClick"')
  expect(clickIdx, '未找到 @click="minimapClick"').toBeGreaterThan(-1)
  const canvasIdx = source.lastIndexOf('<canvas', clickIdx)
  expect(canvasIdx, '未找到对应的 <canvas 开始标签').toBeGreaterThan(-1)
  const endIdx = source.indexOf('>', clickIdx)
  expect(endIdx, '未找到 <canvas> 结束符').toBeGreaterThan(-1)
  return source.slice(canvasIdx, endIdx + 1)
}

const source = readView('ProcessDesigner.vue')

describe('ProcessDesigner 迷你地图键盘无障碍（R221 / WCAG 2.1.1、4.1.2、2.4.7）', () => {
  it('canvas 可 Tab 聚焦（tabindex=0）', () => {
    expect(minimapCanvasTag(source)).toMatch(/\btabindex="0"/)
  })

  it('以 role 与 aria-label 暴露按钮角色与可访问名称', () => {
    const tag = minimapCanvasTag(source)
    expect(tag).toMatch(/\brole="button"/)
    expect(tag).toMatch(/\baria-label="[^"]+"/)
  })

  it('方向键可选择节点，Enter/Space 定位且 Space 阻止滚动', () => {
    const tag = minimapCanvasTag(source)
    expect(tag).toMatch(/@keydown\.left(?:\.\w+)*=/)
    expect(tag).toMatch(/@keydown\.right(?:\.\w+)*=/)
    expect(tag).toMatch(/@keydown\.enter(?:\.\w+)*=/)
    expect(tag).toMatch(/@keydown\.space\.prevent=/)
  })

  it('存在 minimap-canvas 的 :focus-visible 可见焦点样式', () => {
    expect(source).toMatch(/\.minimap-canvas[^{]*:focus-visible/)
  })
})
