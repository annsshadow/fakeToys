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
  parent: El | null
  addEventListener: (t: string, fn: (e: unknown) => void) => unknown
  removeEventListener: (t: string, fn: (e: unknown) => void) => unknown
  querySelectorAll: (sel: string) => unknown[]
  querySelector: (sel: string) => unknown
  _fire: (t: string, e: unknown) => void
  [key: string]: unknown
}

/** 按 '.class' 选择器在本 el 的直接子节点中筛选（showToast 的容器统计只需要这一层）。 */
function byClass(el: El, sel: string): El[] {
  const cls = sel.replace(/^\./, '')
  return el.children.filter((c): c is El => c !== null && (c as El).className === cls)
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
    parent: null,
    appendChild: (c: unknown) => {
      el.children.push(c)
      if (c && typeof c === 'object') (c as El).parent = el
      return el
    },
    remove: () => {
      const i = all.indexOf(el)
      if (i >= 0) all.splice(i, 1)
      if (el.parent) {
        const pi = el.parent.children.indexOf(el)
        if (pi >= 0) el.parent.children.splice(pi, 1)
      }
    },
    addEventListener: (t: string, fn: (e: unknown) => void) => {
      if (!listeners[t]) listeners[t] = []
      listeners[t].push(fn)
      return el
    },
    removeEventListener: (t: string, fn: (e: unknown) => void) => {
      listeners[t] = (listeners[t] ?? []).filter((f) => f !== fn)
    },
    querySelectorAll: (sel: string) => byClass(el, sel),
    querySelector: (sel: string) => byClass(el, sel)[0] ?? null,
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
  const navigator = { clipboard: { writeText: vi.fn() } }
  vi.stubGlobal('navigator', navigator)
  return { doc, keydownListeners, navigator }
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

  it('closing protection: calling confirmMsg while an overlay is still open removes the old one first', async () => {
    // 语义钉死（第 144 行）：重复打开必须先把残留在 DOM 的旧 overlay 摘掉，
    // 否则两个遮罩叠加会挡住旧弹窗的按钮，用户只能靠 Esc 逃生。
    const p1 = toast.confirmMsg('first')
    const firstOverlay = dom.doc.getElementById('oa4-confirm-overlay') as El
    expect(firstOverlay).not.toBeNull()
    expect(dom.doc.body.children).toContain(firstOverlay)

    const p2 = toast.confirmMsg('second')
    expect(dom.doc.getElementById('oa4-confirm-overlay')).not.toBe(firstOverlay)
    expect(dom.doc.body.children).not.toContain(firstOverlay)

    const [, ok2] = buttons()
    ok2.onclick!()
    await expect(p2).resolves.toBe(true)
    expect(dom.doc.getElementById('oa4-confirm-overlay')).toBeNull()
    void p1 // 旧弹窗已被新调用抢占移除，允许其永久挂起
  })

  it('toggles hover styling on both buttons (mouseover brightens, mouseout restores)', () => {
    const p = toast.confirmMsg('删除？')
    const [cancel, ok] = buttons()

    cancel.onmouseover!()
    expect(cancel.style.borderColor).toBe('#94a3b8')
    expect(cancel.style.color).toBe('#e2e8f0')
    cancel.onmouseout!()
    expect(cancel.style.borderColor).toBe('#475569')
    expect(cancel.style.color).toBe('#94a3b8')

    ok.onmouseover!()
    expect(ok.style.background).toBe('#2563eb')
    expect(ok.style.boxShadow).toBe('0 0 15px rgba(37,99,235,0.5)')
    ok.onmouseout!()
    expect(ok.style.background).toBe('#3b82f6')
    expect(ok.style.boxShadow).toBe('0 0 10px rgba(59,130,246,0.3)')

    ok.onclick!()
    void p
  })
})

/** 取出当前 stub body 下的 toast 容器及其 toast 子节点。 */
function toastContainer(): { container: El; toasts: El[] } {
  const container = dom.doc.body.children.find(
    (c): c is El => c !== null && (c as El).className === 'oa4-toast-container',
  ) as El
  return {
    container,
    toasts: (container?.children ?? []) as El[],
  }
}

/** toast 节点的三段文本：[图标, 消息, 状态标签]。 */
function toastTexts(t: El): string[] {
  return (t.children as El[]).map((c) => c.textContent)
}

describe('toast notifications (showToast pipeline)', () => {
  it('renders a success toast and auto-hides it after the given duration', () => {
    vi.useFakeTimers()
    try {
      toast.toast.success('已保存', 500)
      const { container, toasts } = toastContainer()
      expect(container.tagName).toBe('DIV')
      expect(toasts).toHaveLength(1)
      const [t] = toasts
      expect(t.className).toBe('oa4-toast')
      expect(toastTexts(t)).toEqual(['✓', '已保存', 'auto-hide'])

      const removeSpy = vi.spyOn(t, 'remove')
      vi.advanceTimersByTime(499)
      expect(removeSpy).not.toHaveBeenCalled()
      // 到期先走 200ms 的退场动画，再真正从 DOM 摘除
      vi.advanceTimersByTime(1)
      expect(t.style.animation).toBe('oa4ToastOut 0.2s ease forwards')
      vi.advanceTimersByTime(200)
      expect(removeSpy).toHaveBeenCalledTimes(1)
    } finally {
      vi.useRealTimers()
    }
  })

  it('evicts the oldest toast once MAX_VISIBLE is reached', () => {
    vi.useFakeTimers()
    try {
      const long = 60_000
      for (let i = 0; i < 4; i++) toast.toast.info(`t${i}`, long)
      const [, toastsBefore] = (() => {
        const { container, toasts } = toastContainer()
        return [container, toasts]
      })()
      expect(toastsBefore).toHaveLength(4)

      const evictSpy = vi.spyOn(toastsBefore[0], 'remove')
      toast.toast.info('t4', long) // 第 5 条：最旧一条被立即摘除
      expect(evictSpy).toHaveBeenCalledTimes(1)
      const { toasts } = toastContainer()
      expect(toastTexts(toasts[0])).toEqual(['ℹ', 't1', 'auto-hide'])
      expect(toasts).toHaveLength(4)
    } finally {
      vi.useRealTimers()
    }
  })

  it('pauses auto-hide on hover and resumes on mouse leave', () => {
    vi.useFakeTimers()
    try {
      toast.toast.warning('慢一点', 1000)
      const [t] = toastContainer().toasts
      const removeSpy = vi.spyOn(t, 'remove')

      t._fire('mouseenter', undefined)
      vi.advanceTimersByTime(5000)
      expect(removeSpy).not.toHaveBeenCalled()

      t._fire('mouseleave', undefined)
      vi.advanceTimersByTime(500) // duration/2
      expect(t.style.animation).toBe('oa4ToastOut 0.2s ease forwards')
      vi.advanceTimersByTime(200)
      expect(removeSpy).toHaveBeenCalledTimes(1)
    } finally {
      vi.useRealTimers()
    }
  })

  it('dismisses a toast on click', () => {
    vi.useFakeTimers()
    try {
      toast.toast.error('出错', 30_000)
      const [t] = toastContainer().toasts
      const removeSpy = vi.spyOn(t, 'remove')
      t._fire('click', undefined)
      vi.advanceTimersByTime(200)
      expect(removeSpy).toHaveBeenCalledTimes(1)
      expect(toastTexts(t)[1]).toBe('出错')
    } finally {
      vi.useRealTimers()
    }
  })

  it('copyText reports success via a success toast and returns true', async () => {
    dom.navigator.clipboard.writeText.mockResolvedValue(undefined)
    vi.useFakeTimers()
    const ok = await toast.copyText('abc')
    vi.useRealTimers()
    expect(ok).toBe(true)
    expect(dom.navigator.clipboard.writeText).toHaveBeenCalledWith('abc')
    const [t] = toastContainer().toasts
    expect(toastTexts(t)).toEqual(['✓', '已复制到剪贴板', 'auto-hide'])
  })

  it('copyText falls back to an error toast when the clipboard rejects', async () => {
    dom.navigator.clipboard.writeText.mockRejectedValueOnce(new Error('denied'))
    vi.useFakeTimers()
    const ok = await toast.copyText('abc')
    vi.useRealTimers()
    expect(ok).toBe(false)
    const [t] = toastContainer().toasts
    expect(toastTexts(t)).toEqual(['✗', '复制失败，请检查剪贴板权限', 'auto-hide'])
  })
})

describe('useToast', () => {
  it('returns the same toast object so call sites can grab any helper', () => {
    expect(toast.useToast()).toBe(toast.toast)
  })
})
