// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R216 全局进度条无障碍增强（WCAG 4.1.2 Name, Role, Value / 4.1.3 Status Messages）。
 *
 * 各视图的上传/模拟/执行/回放进度条均为纯视觉 <div>（仅内联 width 与百分比文本），
 * 读屏器无法获知进度数值。这里用全局 MutationObserver 为这些容器补 role=progressbar、
 * aria-valuemin/max/now 与可读标签，数值随 DOM 变化实时更新；尊重作者已有的 role /
 * aria-label / aria-valuenow，dispose 时还原我们添加的全部属性。
 */

type A11yRoot = Document | Element | DocumentFragment

export interface ProgressA11yHandle {
  dispose: () => void
}

const PROGRESS_SELECTOR = ['.progress-bar', '.sim-progress', '.st-progress', '.exec-bar', '.tr-progress'].join(', ')

const PERCENT_TEXT = /(-?\d+(?:\.\d+)?)\s*%/
const RATIO_TEXT = /^\s*(\d+)\s*\/\s*(\d+)\s*$/
const INLINE_WIDTH = /width\s*:\s*(-?\d+(?:\.\d+)?)\s*%/

const LABELS: Record<string, string> = {
  'progress-bar': '上传进度',
  'sim-progress': '模拟进度',
  'st-progress': '模拟进度',
  'exec-bar': '执行进度',
  'tr-progress': '回放进度',
}

type Tracked = { element: HTMLElement; added: string[] }

function readValue(el: HTMLElement): string | null {
  const text = el.textContent ?? ''
  const direct = PERCENT_TEXT.exec(text)
  if (direct) return direct[1]

  const fill = el.querySelector<HTMLElement>('[style*="width"], [class*="bar"], [class*="fill"]')
  const width = fill ? INLINE_WIDTH.exec(fill.getAttribute('style') ?? '') : null
  if (width) return width[1]

  const ratio = RATIO_TEXT.exec(text)
  if (ratio) {
    const total = Number(ratio[2])
    if (total > 0) return String(Math.round((Number(ratio[1]) / total) * 100))
  }
  return null
}

function matchClass(el: HTMLElement): string | null {
  for (const key of Object.keys(LABELS)) {
    if (el.classList.contains(key)) return key
  }
  return null
}

/**
 * 安装全局进度条增强器；返回带 dispose 的句柄。兼容 Vue v-if 异步插入，卸载时还原属性。
 */
export function installProgressA11y(root: A11yRoot = document): ProgressA11yHandle {
  let disposed = false
  const tracked: Tracked[] = []
  const seen = new WeakSet<HTMLElement>()

  function apply(): void {
    if (disposed) return
    for (const el of Array.from(root.querySelectorAll<HTMLElement>(PROGRESS_SELECTOR))) {
      const key = matchClass(el)
      if (!key || seen.has(el)) continue
      const added: string[] = []
      const addIfMissing = (attr: string, value: string): void => {
        if (el.hasAttribute(attr)) return
        el.setAttribute(attr, value)
        added.push(attr)
      }
      addIfMissing('role', 'progressbar')
      addIfMissing('aria-label', LABELS[key])
      addIfMissing('aria-valuemin', '0')
      addIfMissing('aria-valuemax', '100')
      addIfMissing('aria-valuenow', readValue(el) ?? '0')
      seen.add(el)
      tracked.push({ element: el, added })
    }
    for (const entry of tracked) {
      if (!entry.added.includes('aria-valuenow')) continue
      const value = readValue(entry.element)
      if (value !== null) entry.element.setAttribute('aria-valuenow', value)
    }
  }

  const observer = new MutationObserver(apply)
  observer.observe(root, { childList: true, subtree: true, attributes: true, attributeFilter: ['style'] })
  apply()

  return {
    dispose: () => {
      if (disposed) return
      disposed = true
      observer.disconnect()
      for (const { element, added } of tracked) {
        for (const attr of added) element.removeAttribute(attr)
      }
      tracked.length = 0
    },
  }
}
