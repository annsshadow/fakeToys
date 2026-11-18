// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R203 契约：界面语言切换必须同步 <html lang>（WCAG 3.1.1 Language of Page，A 级）。
 *
 * 缺陷证据：Settings.vue 提供「语言」下拉（zh-cn / en / es），但全前端生产路径
 * 从未调用 setLocale()；SDK 的 setLocale 只改 i18n 实例与模块级 locale ref，
 * 从不更新 document.documentElement.lang，而 index.html 永远写死 zh-CN。
 * 结果：用户切到 English/Español 后，屏幕阅读器仍按中文发音朗读英文/西语内容。
 *
 * 本测试断言行为：setLocale 后 <html lang> 反映当前 UI 语言；无 document 的
 * node/SSR 环境安全跳过。静态导入先于 document 桩（Vue runtime-dom 在导入期读 document）。
 */
// @vitest-environment node
import { afterEach, describe, expect, it, vi } from 'vitest'
// 静态导入：让 vue / vue-i18n 在真实 node 环境完成模块初始化，再安装 document 桩。
import { getLocale, setLocale } from './i18n'

function installDocument(initialLang: string | null) {
  const attrs: Record<string, string> = {}
  if (initialLang !== null) attrs['lang'] = initialLang
  vi.stubGlobal('document', {
    documentElement: {
      setAttribute: (k: string, v: string) => {
        attrs[k] = v
      },
      getAttribute: (k: string) => attrs[k] ?? null,
    },
    createElement: () => ({ setAttribute: () => {}, appendChild: () => {} }),
  })
  return attrs
}

function currentLang(): string | null {
  return (
    (
      globalThis.document as unknown as { documentElement: { getAttribute: (k: string) => string | null } }
    ).documentElement.getAttribute('lang') ?? null
  )
}

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('界面语言与页面语言同步（WCAG 3.1.1）', () => {
  it('切到英文后 <html lang> 变为 en', () => {
    installDocument('zh-CN')
    setLocale('en')
    expect(getLocale()).toBe('en')
    expect(currentLang(), 'setLocale("en") 未同步 <html lang>，屏幕阅读器仍按中文发音').toBe('en')
  })

  it('切到西语后 <html lang> 变为 es', () => {
    installDocument('en')
    setLocale('es')
    expect(currentLang(), 'setLocale("es") 未同步 <html lang>').toBe('es')
  })

  it('切回中文后 <html lang> 变为 zh-cn', () => {
    installDocument('en')
    setLocale('zh-cn')
    expect(currentLang(), 'setLocale("zh-cn") 未同步 <html lang>').toBe('zh-cn')
  })

  it('无 document 的 SSR/node 环境下 setLocale 不抛错', () => {
    vi.stubGlobal('document', undefined)
    expect(() => setLocale('en')).not.toThrow()
    expect(getLocale()).toBe('en')
  })
})
