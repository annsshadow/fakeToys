// @vitest-environment node
/**
 * toast / confirmMsg 单测：stub 最小 DOM，钉死
 *   - confirmMsg 的 4 种关闭路径（确认/取消/点遮罩/Esc）返回的布尔值
 *   - 无 DOM 环境（原生 App 壳）confirmMsg 直接 resolve false，不抛错
 * XSS 面（textContent 而非 innerHTML）由 security.test.ts 的源码守卫覆盖。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

interface El {
  tagName: string
  style: Record<string, unknown>
  textContent: string
  innerHTML: string
  children: unknown[]
  onclick: ((...args: unknown[]) => void) | null
  onmouseover: ((...args: unknown[]) => void) | null
  onmouseout: ((...args: unknown[]) => void) | null
  appendChild: (c: unknown) => unknown
  remove: () => void
  addEventListener: (t: string, fn: (e: unknown) => void) => unknown
  removeEventListener: (t: string, fn: (e: unknown) => void) => void
  _fire: (t: string, e: unknown) => void
  [key: string]: unknown
}

function makeEl(all: El[], tag = 'div'): El {
  const listeners: Record<string, Array<(e: unknown) => void>> = {}
  const el: El = {
    tagName: tag.toUpperCase(),
    style: {},
    textContent: '',
    innerHTML: '',
    children: [],
    onclick: null,
    onmouseover: null,
    onmouseout: null,
    appendChild: (c: unknown) => {
      el.children.push(c)
      return el
    },
    remove: () => {
      const i = all.indexOf(el)
      if (i >= 0) all.splice(i, 1)
    },
    addEventListener: (t: string, fn: (e: unknown) => void) => {
      if (!listeners[t]) listeners[t] = []
      listeners[t].push(fn)
      return el
    },
    removeEventListener: (t: string, fn: (e: unknown) => void) => {
      listeners[t] = (listeners[t] ?? []).filter((f) => f !== fn)
    },
    _fire: (t: string, e: unknown) => (listeners[t] ?? []).forEach((f) => f(e)),
  }
  all.push(el)
  return el
}

function installDom() {
  const all: El[] = []
  const keydownListeners: Array<(e: { key: string }) => void> = []
  const doc = {
    createElement: (tag: string) => makeEl(all, tag),
    getElementById: (id: string) => all.find((e) => (e as never as { id?: string }).id === id) ?? null,
    body: makeEl(all, 'body'),
    head: makeEl(all, 'head'),
    addEventListener: (t: string, fn: (e: { key: string }) => void) => {
      if (t === 'keydown') keydownListeners.push(fn)
    },
    removeEventListener: (t: string, fn: (e: { key: string }) => void) => {
      if (t === 'keydown') {
        const i = keydownListeners.indexOf(fn)
        if (i >= 0) keydownListeners.splice(i, 1)
      }
    },
  }
  vi.stubGlobal('document', doc)
  vi.stubGlobal('navigator', { clipboard: { writeText: vi.fn() } })
  return { doc, keydownListeners }
}

let toast: typeof import('./toast')
let dom: ReturnType<typeof installDom>

beforeEach(async () => {
  dom = installDom()
  vi.resetModules()
  toast = await import('./toast')
})

afterEach(() => {
  vi.unstubAllGlobals()
})

/** 从 confirmMsg 的 overlay 中取出 [取消, 确认] 两个按钮。 */
function buttons(): [El, El] {
  const overlay = dom.doc.getElementById('oa4-confirm-overlay') as El
  const box = overlay.children[0] as El
  const btnRow = box.children[2] as El
  return [btnRow.children[0] as El, btnRow.children[1] as El]
}

describe('confirmMsg', () => {
  it('resolves true when 确认 is clicked, then removes the overlay from the DOM', async () => {
    const p = toast.confirmMsg('删除？')
    const [, ok] = buttons()
    ok.onclick!()
    await expect(p).resolves.toBe(true)
    expect(dom.doc.getElementById('oa4-confirm-overlay')).toBeNull()
  })

  it('resolves false when 取消 is clicked', async () => {
    const p = toast.confirmMsg('删除？')
    const [cancel] = buttons()
    cancel.onclick!()
    await expect(p).resolves.toBe(false)
  })

  it('resolves false when the overlay backdrop is clicked (but not on an inner element)', async () => {
    const p = toast.confirmMsg('删除？')
    const overlay = dom.doc.getElementById('oa4-confirm-overlay') as El
    const inner = overlay.children[0] as El
    const handler = (overlay as never as { onclick: (e: unknown) => void }).onclick!
    handler({ target: inner }) // 内部元素点击不关闭
    let settled = false
    p.then(() => (settled = true))
    await new Promise((r) => setTimeout(r, 0))
    expect(settled).toBe(false)
    handler({ target: overlay }) // 点中遮罩本身才关闭
    await expect(p).resolves.toBe(false)
  })

  it('resolves false on the Escape key', async () => {
    const p = toast.confirmMsg('删除？')
    dom.keydownListeners.forEach((fn) => fn({ key: 'Escape' }))
    await expect(p).resolves.toBe(false)
  })

  it('a non-Escape key does not close the dialog', async () => {
    const p = toast.confirmMsg('删除？')
    dom.keydownListeners.forEach((fn) => fn({ key: 'Enter' }))
    let settled = false
    p.then(() => (settled = true))
    await new Promise((r) => setTimeout(r, 0))
    expect(settled).toBe(false)
  })

  it('without a DOM (native app shell) resolves false instead of crashing', async () => {
    vi.unstubAllGlobals()
    await expect(toast.confirmMsg('删除？')).resolves.toBe(false)
  })

  it('removes a still-registered overlay before opening a new one', async () => {
    const p1 = toast.confirmMsg('first')
    const [, ok1] = buttons()
    ok1.onclick!()
    await p1
    expect(dom.doc.getElementById('oa4-confirm-overlay')).toBeNull()
    const p2 = toast.confirmMsg('second')
    const [, ok2] = buttons()
    ok2.onclick!()
    await expect(p2).resolves.toBe(true)
    expect(dom.doc.getElementById('oa4-confirm-overlay')).toBeNull()
  })
})
