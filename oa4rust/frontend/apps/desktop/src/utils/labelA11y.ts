// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R213 孤儿表单标签无障碍关联（WCAG 1.3.1 Info and Relationships / 4.1.2）。
 *
 * 大量 CRUD 视图用 <div class="form-group"><label>标题</label><input /></div>，
 * label 既无 for 也不包裹控件，读屏器无法把可见文本与输入框关联。这里提供全局
 * 运行时增强器：为这类孤儿 label 与其紧随的 labelable 控件建立 id/htmlFor 关联。
 * 已显式声明 for 或已包裹控件的 label 保持原样；DOM 变化由 MutationObserver
 * 驱动，兼容 Vue 的 v-if 异步插入/移除。卸载时还原我们添加的属性。
 */

type A11yRoot = Document | Element | DocumentFragment

/** HTML 规范中可与 <label> 关联的控件集合。 */
const LABELABLE = 'button, input, meter, output, progress, select, textarea'

type Link = { label: HTMLLabelElement; control: HTMLElement; generatedId: boolean }

/** 从 label 之后的兄弟里找最近的 labelable 控件；最多看 3 层，避免跨字段误关联。 */
function findControl(label: HTMLLabelElement): HTMLElement | null {
  let sibling = label.nextElementSibling
  let hops = 0
  while (sibling && hops < 3) {
    if (sibling.matches(LABELABLE)) return sibling as HTMLElement
    sibling = sibling.nextElementSibling
    hops += 1
  }
  return null
}

/**
 * 安装全局增强器；返回卸载函数。立即处理已有 label，并观察后续 DOM 变化。
 */
export function installLabelA11y(root: A11yRoot = document): () => void {
  let disposed = false
  let idSequence = 0
  const links: Link[] = []
  const seen = new WeakSet<HTMLLabelElement>()

  function apply(): void {
    if (disposed) return
    const labels = Array.from(root.querySelectorAll<HTMLLabelElement>('label'))
    for (const label of labels) {
      // 已有显式关联或包裹控件的 label 不动（如 XformRuntime / LoginScreen）。
      if (label.hasAttribute('for') || label.querySelector(LABELABLE)) continue
      if (seen.has(label)) continue
      const control = findControl(label)
      if (!control) continue

      const existingId = control.getAttribute('id')
      let generatedId = false
      let id = existingId
      if (!id) {
        const doc = control.ownerDocument
        do {
          idSequence += 1
          id = `oa-label-control-${String(idSequence).padStart(6, '0')}`
        } while (doc?.getElementById(id))
        control.setAttribute('id', id)
        generatedId = true
      }
      label.htmlFor = id
      seen.add(label)
      links.push({ label, control, generatedId })
    }
  }

  const observer = new MutationObserver(apply)
  observer.observe(root, { childList: true, subtree: true })
  apply()

  return () => {
    if (disposed) return
    disposed = true
    observer.disconnect()
    for (const link of links) {
      link.label.removeAttribute('for')
      if (link.generatedId) link.control.removeAttribute('id')
    }
  }
}
