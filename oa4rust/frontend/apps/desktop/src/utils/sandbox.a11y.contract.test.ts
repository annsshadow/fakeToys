// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R224 沙箱 iframe 无障碍契约（WCAG 4.1.2 Name/Role/Value、2.4.3 Focus Order）。
 *
 * runInSandbox 惰性创建的 iframe 被绝对定位到 (-9999px,-9999px) 且仅 1px 大小，
 * 它纯粹是脚本执行容器，对键盘/读屏用户没有任何可交互内容。但 iframe 天生可聚焦：
 *   - 没有 title → 读屏把屏幕外 1px 帧当无名 frame 念出来，遮蔽真实页面；
 *   - 没有 aria-hidden → 键盘 Tab 会跳进这个看不见的帧，焦点"消失"。
 * 修复：给它一个描述性 title，并用 aria-hidden="true" + tabindex="-1" 把它从
 * 读屏树和 Tab 顺序中移除。本契约先于实现提交，RED 证明缺口真实存在。
 */
// @vitest-environment node
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

interface FakeEl {
  tagName: string
  attributes: Record<string, string>
  style: Record<string, string>
  children: unknown[]
  src: string
  srcdoc: string
  contentWindow: { postMessage: ReturnType<typeof vi.fn> }
  setAttribute: (k: string, v: string) => void
  getAttribute: (k: string) => string | null
  appendChild: (c: unknown) => unknown
  remove: () => void
  addEventListener: (t: string, fn: (e: unknown) => void) => void
  removeEventListener: (t: string, fn: (e: unknown) => void) => void
  __fire: (t: string, e?: unknown) => void
}

function makeEl(tag: string): FakeEl {
  const listeners: Record<string, Array<(e: unknown) => void>> = {}
  const el: FakeEl = {
    tagName: tag,
    attributes: {},
    style: {},
    children: [],
    src: '',
    srcdoc: '',
    contentWindow: { postMessage: vi.fn() },
    setAttribute: (k, v) => {
      el.attributes[k] = v
    },
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
    __fire: (t, e) => {
      for (const fn of listeners[t] ?? []) fn(e)
    },
  }
  return el
}

async function loadSandbox() {
  const iframe = makeEl('iframe')
  const body = makeEl('body')
  const messageListeners: Array<(e: unknown) => void> = []
  vi.stubGlobal('document', {
    createElement: (tag: string) => (tag === 'iframe' ? iframe : makeEl(tag)),
    body,
    head: makeEl('head'),
  })
  vi.stubGlobal('window', {
    addEventListener: (t: string, fn: (e: unknown) => void) => {
      if (t === 'message') messageListeners.push(fn)
    },
    removeEventListener: () => {},
  })
  const mod = await import('./sandbox')
  return { mod, iframe }
}

describe('sandbox iframe 无障碍（屏幕外执行帧不得暴露给键盘/读屏）', () => {
  let sandbox: Awaited<ReturnType<typeof loadSandbox>>

  beforeEach(async () => {
    sandbox = await loadSandbox()
    sandbox.mod.destroySandbox()
  })
  afterEach(() => {
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
    vi.resetModules()
  })

  it('iframe 带有描述性 title：读屏不会把它念成无名 frame（WCAG 4.1.2）', () => {
    sandbox.mod.runInSandbox('1+1')
    const title = sandbox.iframe.getAttribute('title')?.trim() ?? ''
    expect(title.length, '屏幕外沙箱 iframe 缺少可访问名称 title').toBeGreaterThan(0)
  })

  it('iframe 从读屏树与 Tab 顺序中移除：aria-hidden=true 且 tabindex=-1（WCAG 2.4.3）', () => {
    sandbox.mod.runInSandbox('1+1')
    expect(sandbox.iframe.getAttribute('aria-hidden')).toBe('true')
    expect(sandbox.iframe.getAttribute('tabindex')).toBe('-1')
  })
})
