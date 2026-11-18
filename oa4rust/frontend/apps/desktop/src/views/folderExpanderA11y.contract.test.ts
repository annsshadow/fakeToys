// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R236 思维导图目录树展开器无障碍契约测试（WCAG 2.1.1 Keyboard / 4.1.2 Name-Role-Value / 2.4.7 Focus Visible）。
 *
 * MindApp 目录树的行原本是 <button @click="selectFolder">，内部放一个 <span class="expander"
 * @click.stop="toggleFolder"> 作展开器。展开器只响应鼠标 click：无 tabindex、无 role、无
 * aria-expanded/aria-label，也不响应键盘；键盘与读屏用户无法展开/折叠目录，子目录对他们
 * 永久隐藏（folderRows 仅在 expandedFolders 含 id 时才 walk 子项）。
 *
 * 契约钉住：目录行改为非交互容器 <div>，展开器与目录名各自是原生 <button type="button">，
 * 二者均可 Tab 聚焦、原生支持 Enter/Space；展开器以 aria-expanded 暴露展开状态、以
 * aria-label 暴露"展开/折叠 + 目录名"的可访问名称；叶节点不产生空的可聚焦展开器。
 * 仓库无 @vue/test-utils，故沿用源码文本契约（与 minimapA11y.contract.test.ts 同口径）。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

function readView(name: string): string {
  return readFileSync(resolve(import.meta.dirname, name), 'utf8')
}

/** 截取首个 class="expander" 所属元素的开始标签（跨行）。 */
function expanderTag(source: string): string {
  const idx = source.indexOf('class="expander"')
  expect(idx, '未找到 class="expander"').toBeGreaterThan(-1)
  const start = source.lastIndexOf('<', idx)
  const end = source.indexOf('>', idx)
  expect(end, '未找到 expander 开始标签结束符').toBeGreaterThan(-1)
  return source.slice(start, end + 1)
}

/** 截取目录树 v-for 所在行的开始标签（跨行）。 */
function folderRowTag(source: string): string {
  const idx = source.indexOf('v-for="row in folderRows"')
  expect(idx, '未找到目录行 v-for').toBeGreaterThan(-1)
  const start = source.lastIndexOf('<', idx)
  const end = source.indexOf('>', idx)
  expect(end, '未找到目录行开始标签结束符').toBeGreaterThan(-1)
  return source.slice(start, end + 1)
}

const source = readView('MindApp.vue')

describe('MindApp 目录树展开器键盘与读屏可访问（R236 / WCAG 2.1.1、4.1.2、2.4.7）', () => {
  it('展开器是原生 button，键盘 Enter/Space 即可展开折叠（非纯 span click）', () => {
    expect(expanderTag(source)).toMatch(/^<button\b/)
    expect(expanderTag(source)).toMatch(/\btype="button"/)
  })

  it('展开器以 aria-expanded 暴露当前展开状态', () => {
    expect(expanderTag(source)).toMatch(/:aria-expanded="expandedFolders\.has\(row\.folder\.id\)"/)
  })

  it('展开器以 aria-label 暴露展开/折叠动作与目标目录名', () => {
    const tag = expanderTag(source)
    expect(tag).toMatch(/:aria-label=/)
    expect(tag).toMatch(/展开|折叠/)
    expect(tag).toMatch(/row\.folder\.name/)
  })

  it('目录行容器不是原生 button，避免嵌套交互控件', () => {
    const tag = folderRowTag(source)
    expect(tag).toMatch(/^<div\b/)
    expect(tag).not.toMatch(/^<button\b/)
  })

  it('目录名是独立 button，键盘用户可 Tab 后 Enter/Space 选中目录', () => {
    expect(source).toMatch(/<button[^>]*class="folder-name"[^>]*@click="selectFolder\(row\.folder\)"/)
  })

  it('叶节点展开器以 aria-hidden 占位，不制造空聚焦陷阱', () => {
    expect(source).toMatch(/<span v-else class="expander" aria-hidden="true"><\/span>/)
  })

  it('展开器与目录名提供 :focus-visible 可见焦点样式', () => {
    expect(source).toMatch(/\.expander:focus-visible[^{]*\{[^}]*outline/)
    expect(source).toMatch(/\.folder-name:focus-visible[^{]*\{[^}]*outline/)
  })
})
