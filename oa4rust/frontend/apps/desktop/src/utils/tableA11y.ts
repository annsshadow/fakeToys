// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R202 全局表格表头作用域增强（WCAG 1.3.1 Info and Relationships）。
 *
 * 全仓数据表格的 <th> 普遍未声明 scope，读屏器因此无法把数据单元格关联到
 * 对应列/行标题，表格导航时用户听不到当前单元格的语义上下文。这里提供全局
 * 运行时增强器：为 thead 内的 th 补 scope=col，为 tbody 行首 th 补 scope=row。
 * 属性变更由 MutationObserver 驱动，兼容 Vue 的 v-if 异步插入/移除；已显式
 * 声明 scope 的 th 保持原样。
 */

type A11yRoot = Document | Element | DocumentFragment

/**
 * 安装全局增强器；返回卸载函数。立即处理已存在的表头，并观察后续 DOM 变化。
 */
export function installTableA11y(root: A11yRoot = document): () => void {
  let disposed = false

  function apply(): void {
    if (disposed) return
    const tables = Array.from(root.querySelectorAll<HTMLTableElement>('table'))
    for (const table of tables) {
      const colHeaders = Array.from(table.querySelectorAll<HTMLTableCellElement>('thead th'))
      for (const th of colHeaders) {
        if (!th.hasAttribute('scope')) th.setAttribute('scope', 'col')
      }
      const rows = Array.from(table.querySelectorAll<HTMLTableRowElement>('tbody tr'))
      for (const row of rows) {
        const first = row.querySelector<HTMLTableCellElement>('th')
        if (first && !first.hasAttribute('scope')) first.setAttribute('scope', 'row')
      }
    }
  }

  const observer = new MutationObserver(apply)
  observer.observe(root, { childList: true, subtree: true })
  apply()

  return () => {
    if (disposed) return
    disposed = true
    observer.disconnect()
  }
}
