// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node
/**
 * runInSandbox 单测：stub 掉 document/window，验证
 *   - 代码经 iframe contentWindow.postMessage 下发（__oa4rust_sandbox_code）
 *   - 结果消息必须来自沙盒 iframe 且带 __oa4rust_sandbox_result 标记（防伪造）
 *   - 超时兜底返回 ok:false
 *   - destroySandbox 解除 window message 监听
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

interface FakeElement {
  tagName: string
  attributes: Record<string, string>
  style: Record<string, string>
  children: unknown[]
  srcdoc: string
  contentWindow?: { postMessage: ReturnType<typeof vi.fn> }
  setAttribute: (k: string, v: string) => void
  getAttribute: (k: string) => string | null
  appendChild: (c: unknown) => unknown
  remove: () => void
  addEventListener: (t: string, fn: (e: unknown) => void) => void
  removeEventListener: (t: string, fn: (e: unknown) => void) => void
}

function makeElement(tag: string): FakeElement {
  const listeners: Record<string, Array<(e: unknown) => void>> = {}
  const el: FakeElement = {
    tagName: tag,
    attributes: {},
    style: {},
    children: [],
    srcdoc: '',
    setAttribute: (k, v) => (el.attributes[k] = v),
    getAttribute: (k) => el.attributes[k] ?? null,
    appendChild: (c) => {
      el.children.push(c)
      return c
    },
    remove: () => {},
    addEventListener: (t, fn) => {
      if (!listeners[t]) listeners[t] = []
      listeners[t].push(fn)
    },
    removeEventListener: (t, fn) => {
      listeners[t] = (listeners[t] ?? []).filter((f) => f !== fn)
    },
  }
  return el
}

async function loadSandbox() {
  const iframe: FakeElement & { contentWindow: { postMessage: ReturnType<typeof vi.fn> } } = {
    ...makeElement('iframe'),
    contentWindow: { postMessage: vi.fn() },
  }
  const messageListeners: Array<(e: unknown) => void> = []
  const bodyEl = makeElement('body')
  const created = new Map<string, FakeElement>()
  vi.stubGlobal('document', {
    createElement: (tag: string) => {
      if (tag === 'iframe') return iframe
      return created.get(tag) ?? makeElement(tag)
    },
    body: bodyEl,
    head: makeElement('head'),
  })
  vi.stubGlobal('window', {
    addEventListener: (t: string, fn: (e: unknown) => void) => {
      if (t === 'message') messageListeners.push(fn)
    },
    removeEventListener: (t: string, fn: (e: unknown) => void) => {
      const i = messageListeners.indexOf(fn)
      if (i >= 0) messageListeners.splice(i, 1)
    },
    location: { origin: 'https://oa.example' },
  })
  const mod = await import('./sandbox')
  return { mod, iframe, body: bodyEl, messageListeners }
}

describe('runInSandbox', () => {
  let sandbox: Awaited<ReturnType<typeof loadSandbox>>

  beforeEach(async () => {
    sandbox = await loadSandbox()
    // 每个用例从干净的 iframe 状态出发（模块级 iframe 单例）。
    sandbox.mod.destroySandbox()
  })

  afterEach(() => {
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
    // sandbox.ts 有模块级 iframe 单例；重置模块图保证每个用例拿到干净状态。
    vi.resetModules()
  })

  it('posts the code into the sandbox iframe and resolves the posted result', async () => {
    const p = sandbox.mod.runInSandbox('console.log(1)')
    // 代码消息经 contentWindow.postMessage 下发到 window.location.origin。
    expect(sandbox.iframe.contentWindow!.postMessage).toHaveBeenCalledWith(
      { __oa4rust_sandbox_code: 'console.log(1)' },
      'https://oa.example',
    )
    // 模拟沙盒回传结果（source 必须是沙盒 iframe）。
    const fire = () =>
      sandbox.messageListeners.forEach((fn) =>
        fn({
          source: sandbox.iframe.contentWindow,
          data: { __oa4rust_sandbox_result: true, ok: true, output: 'done', executionTimeMs: 7 },
        }),
      )
    fire()
    await expect(p).resolves.toEqual({ ok: true, output: 'done', error: undefined, executionTimeMs: 7 })
  })

  it('ignores result messages from foreign sources (防其它 window 伪造结果)', async () => {
    const p = sandbox.mod.runInSandbox('x')
    const fireForeign = (data: unknown) => sandbox.messageListeners.forEach((fn) => fn({ source: {}, data }))
    fireForeign({ __oa4rust_sandbox_result: true, ok: true, output: 'forged' })
    // 伪造消息不得提前 resolve；真正的结果才生效。
    let resolved = false
    p.then(() => (resolved = true))
    await new Promise((r) => setTimeout(r, 0))
    expect(resolved).toBe(false)
    sandbox.messageListeners.forEach((fn) =>
      fn({
        source: sandbox.iframe.contentWindow,
        data: { __oa4rust_sandbox_result: true, ok: true, output: 'real', executionTimeMs: 1 },
      }),
    )
    await expect(p).resolves.toMatchObject({ ok: true, output: 'real' })
  })

  it('ignores messages without the sandbox result marker', async () => {
    const p = sandbox.mod.runInSandbox('x')
    sandbox.messageListeners.forEach((fn) => fn({ source: sandbox.iframe.contentWindow, data: { unrelated: true } }))
    let resolved = false
    p.then(() => (resolved = true))
    await new Promise((r) => setTimeout(r, 0))
    expect(resolved).toBe(false)
  })

  it('force-resolves with ok:false when no result arrives within timeoutMs', async () => {
    const p = sandbox.mod.runInSandbox('hang()', 40)
    const result = await p
    expect(result.ok).toBe(false)
    expect(result.error).toBe('sandbox timed out after 40ms')
    expect(result.executionTimeMs).toBe(40)
  })

  it('the sandbox iframe is created with sandbox="allow-scripts" and no same-origin access', async () => {
    sandbox.mod.runInSandbox('x') // 触发 getOrCreateFrame
    expect(sandbox.iframe.attributes['sandbox']).toBe('allow-scripts')
    expect(sandbox.body.children.length).toBeGreaterThan(0) // 已挂到 body
  })

  it('bootstrapSandboxFrame writes srcdoc into the existing frame', () => {
    sandbox.mod.runInSandbox('x') // 先确保 iframe 已创建（bootstrap 对未创建的 frame 是 no-op）
    sandbox.mod.bootstrapSandboxFrame('<script>boot()</script>')
    expect(sandbox.iframe.srcdoc).toBe('<script>boot()</script>')
  })

  it('destroySandbox unregisters the window message listener', () => {
    sandbox.mod.runInSandbox('x')
    expect(sandbox.messageListeners.length).toBe(1)
    sandbox.mod.destroySandbox()
    expect(sandbox.messageListeners.length).toBe(0)
  })
})
