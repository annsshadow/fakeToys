// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R202 表格表头作用域无障碍增强的契约测试（WCAG 1.3.1 Info and Relationships）。
 *
 * 全仓数据表格的 <th> 普遍未声明 scope，读屏器无法把数据单元格关联到行/列标题，
 * 用户在表格内导航时听不到当前单元格属于哪一列/哪一行。全局增强器安装后应自动
 * 为 <thead> 内的 th 补 scope="col"，为 <tbody> 行首 th 补 scope="row"。
 * 全部通过 jsdom DOM 行为断言，不检查源码字符串。
 */
import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { installTableA11y } from './tableA11y'

async function flush() {
  await new Promise((resolve) => setTimeout(resolve, 0))
}

function tableWithHeaders(): HTMLElement {
  const wrap = document.createElement('div')
  wrap.innerHTML = `
    <table>
      <thead><tr><th>标题</th><th>状态</th></tr></thead>
      <tbody>
        <tr><th>行一</th><td>启用</td></tr>
        <tr><th>行二</th><td>停用</td></tr>
      </tbody>
    </table>
  `
  return wrap
}

let dispose: () => void = () => {}

beforeEach(() => {
  document.body.innerHTML = ''
})

afterEach(() => {
  dispose()
  document.body.innerHTML = ''
})

describe('表格表头作用域无障碍增强（WCAG 1.3.1）', () => {
  it('安装后为 thead 内的 th 补 scope=col，为 tbody 行首 th 补 scope=row', async () => {
    const wrap = tableWithHeaders()
    document.body.appendChild(wrap)
    dispose = installTableA11y(document.body)
    await flush()

    const colHeaders = wrap.querySelectorAll<HTMLTableCellElement>('thead th')
    expect(colHeaders.length).toBe(2)
    for (const th of colHeaders) expect(th.getAttribute('scope')).toBe('col')

    const rowHeaders = wrap.querySelectorAll<HTMLTableCellElement>('tbody th')
    expect(rowHeaders.length).toBe(2)
    for (const th of rowHeaders) expect(th.getAttribute('scope')).toBe('row')
  })

  it('尊重视图已有的 scope 声明，不覆盖', async () => {
    const wrap = tableWithHeaders()
    wrap.querySelector('thead th')!.setAttribute('scope', 'colgroup')
    wrap.querySelector('tbody th')!.setAttribute('scope', 'rowgroup')
    document.body.appendChild(wrap)
    dispose = installTableA11y(document.body)
    await flush()

    expect(wrap.querySelector('thead th')!.getAttribute('scope')).toBe('colgroup')
    expect(wrap.querySelector('tbody th')!.getAttribute('scope')).toBe('rowgroup')
  })

  it('Vue 异步插入的表格行 th 也会被补 scope', async () => {
    dispose = installTableA11y(document.body)
    const wrap = tableWithHeaders()
    document.body.appendChild(wrap)
    await flush()

    for (const th of wrap.querySelectorAll<HTMLTableCellElement>('th')) {
      expect(th.hasAttribute('scope')).toBe(true)
    }
  })

  it('卸载后新增的表格不再被处理，也不残留观察器', async () => {
    dispose = installTableA11y(document.body)
    dispose()
    dispose = () => {}

    const wrap = tableWithHeaders()
    document.body.appendChild(wrap)
    await flush()
    for (const th of wrap.querySelectorAll<HTMLTableCellElement>('th')) {
      expect(th.hasAttribute('scope')).toBe(false)
    }
  })
})
