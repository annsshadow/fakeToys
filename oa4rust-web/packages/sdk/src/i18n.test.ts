// @vitest-environment node
import { describe, expect, it } from 'vitest'
import { getLocale, registerMessages, type SupportedLocale, setLocale, useI18nInstance } from './i18n'

// 文件内测试按声明顺序串行执行；模块级单例 (_i18n/_locale) 状态依赖此顺序。
describe('locale state (module singleton)', () => {
  it('defaults to zh-cn', () => {
    expect(getLocale()).toBe('zh-cn')
  })

  it('registerMessages before the i18n instance exists is a no-op, not a crash', () => {
    // 实例尚未创建：消息必须被安全忽略（后续 useI18nInstance 会重建空白 messages）。
    expect(() => registerMessages('en', { a: '1' })).not.toThrow()
  })

  it('setLocale switches the active locale', () => {
    setLocale('en')
    expect(getLocale()).toBe('en')
    setLocale('es')
    expect(getLocale()).toBe('es')
  })
})

describe('i18n instance', () => {
  it('useI18nInstance returns the shared singleton', () => {
    expect(useI18nInstance()).toBe(useI18nInstance())
  })

  it('setLocale keeps the instance locale in sync', () => {
    const inst = useI18nInstance()
    setLocale('en')
    expect((inst.global as never as { locale: { value: SupportedLocale } }).locale.value).toBe('en')
  })

  it('registerMessages merges into existing messages instead of clobbering', () => {
    const inst = useI18nInstance()
    registerMessages('en', { greeting: 'hello' })
    registerMessages('en', { farewell: 'bye' })
    const msg = (inst.global as never as { getLocaleMessage: (l: string) => Record<string, string> }).getLocaleMessage(
      'en',
    )
    expect(msg.greeting).toBe('hello')
    expect(msg.farewell).toBe('bye')
  })
})
