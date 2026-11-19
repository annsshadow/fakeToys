// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node
/**
 * copyText 非安全上下文兜底：HTTP 局域网 / 内嵌 WebView 下 navigator.clipboard
 * 不存在，用户点「复制」仍应通过 execCommand('copy') 成功，而非一律报错。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

interface El {
  tagName: string
  id: string
  attributes: Record<string, string>
  style: Record<string, unknown>
  textContent: string
  value: string
  className: string
  children: El[]
  parent: El | null
  setAttribute: (n: string, v: string) => void
  getAttribute: (n: string) => string | null
  appendChild: (c: El) => El
  remove: () => void
  querySelectorAll: (sel: string) => El[]
  querySelector: (sel: string) => El | null
  select: () => void
  setSelectionRange: (s: number, e: number) => void
  addEventListener: (t: string, fn: (e: unknown) => void) => void
  removeEventListener: (t: string, fn: (e: unknown) => void) => void
  [key: string]: unknown
}

function byClass(el: El, sel: string): El[] {
  const cls = sel.replace(/^\./, '')
  return el.children.filter((c) => c.className === cls)
}

function makeEl(tag: string, all: El[]): El {
  const listeners: Record<string, Array<(e: unknown) => void>> = {}
  const el: El = {
    tagName: tag.toUpperCase(),
    id: '',
    attributes: {},
    style: {},
    textContent: '',
    value: '',
    className: '',
    children: [],
    parent: null,
    setAttribute(n, v) {
      el.attributes[n] = v
      if (n === 'class') el.className = v
    },
    getAttribute(n) {
      return el.attributes[n] ?? null
    },
    appendChild(c) {
      el.children.push(c)
      c.parent = el
      return c
    },
    remove() {
      const i = all.indexOf(el)
      if (i >= 0) all.splice(i, 1)
      if (el.parent) {
        const pi = el.parent.children.indexOf(el)
        if (pi >= 0) el.parent.children.splice(pi, 1)
      }
    },
    querySelectorAll(sel) {
      return byClass(el, sel)
    },
    querySelector(sel) {
      return byClass(el, sel)[0] ?? null
    },
    select() {},
    setSelectionRange() {},
    addEventListener(t, fn) {
      if (!listeners[t]) listeners[t] = []
      listeners[t].push(fn)
    },
    removeEventListener(t, fn) {
      listeners[t] = (listeners[t] ?? []).filter((f) => f !== fn)
    },
  }
  all.push(el)
  return el
}

function installDom(execCommand: (() => boolean) | undefined) {
  const all: El[] = []
  const doc = {
    createElement: (tag: string) => makeEl(tag, all),
    getElementById: (id: string) => all.find((e) => e.id === id) ?? null,
    body: makeEl('body', all),
    head: makeEl('head', all),
    execCommand,
  }
  vi.stubGlobal('document', doc)
  return { doc, all }
}

function toastTexts(el: El): string[] {
  return el.children.map((c) => c.textContent)
}

function container(all: El[]): El {
  return all.find((e) => e.className === 'oa4-toast-container')!
}

let toast: typeof import('./toast')

beforeEach(() => {
  vi.resetModules()
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.useRealTimers()
})

describe('copyText 非安全上下文 execCommand 兜底', () => {
  it('copyText 在 Clipboard API 不存在时经 execCommand 兜底复制并提示成功', async () => {
    // 修复前：直接 await navigator.clipboard.writeText → clipboard 为 undefined 时抛错，
    // 被 catch 后一律提示「复制失败」并返回 false，HTTP 局域网用户根本复制不了。
    const execCommand = vi.fn(() => true)
    const { doc, all } = installDom(execCommand)
    vi.stubGlobal('navigator', {})
    toast = await import('./toast')

    vi.useFakeTimers()
    const ok = await toast.copyText('hello')
    expect(ok).toBe(true)
    expect(execCommand).toHaveBeenCalledWith('copy')
    const [t] = byClass(container(all), 'oa4-toast')
    expect(toastTexts(t)).toEqual(['✓', '已复制到剪贴板', 'auto-hide'])
    // 临时 textarea 必须被清理，不能残留在页面上。
    expect(doc.body.children.some((c) => c.tagName === 'TEXTAREA')).toBe(false)
  })

  it('copyText 在剪贴板与 execCommand 都不可用时提示失败', async () => {
    const { all } = installDom(undefined)
    vi.stubGlobal('navigator', {})
    toast = await import('./toast')

    vi.useFakeTimers()
    const ok = await toast.copyText('hello')
    expect(ok).toBe(false)
    const [t] = byClass(container(all), 'oa4-toast')
    expect(toastTexts(t)).toEqual(['✗', '复制失败，请检查剪贴板权限', 'auto-hide'])
  })
})
