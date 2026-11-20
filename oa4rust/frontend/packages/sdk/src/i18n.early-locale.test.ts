// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node
import { describe, expect, it, vi } from 'vitest'

describe('i18n locale bootstrap', () => {
  it('locale selected before the i18n instance exists is the locale the instance starts with', async () => {
    // 宿主应用常在读取用户偏好后、首个组件挂载前调用 setLocale('en')。
    // 此时 _i18n 尚未创建：getLocale() 报告 'en'，但实例仍用默认 'zh-cn'，
    // 界面语言与状态查询不一致。新实例必须继承已选定的 locale。
    vi.resetModules()
    const mod = await import('./i18n')

    mod.setLocale('en')
    expect(mod.getLocale()).toBe('en')

    const inst = mod.useI18nInstance()
    const active = (inst.global as never as { locale: { value: string } }).locale.value
    expect(active).toBe('en')
  })
})
