// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it, vi } from 'vitest'
import { nextTick, ref } from 'vue'
import { watchSessionCacheReset } from './sessionCache'

describe('watchSessionCacheReset', () => {
  it('登录人变化即清空查询缓存', async () => {
    const clear = vi.fn()
    const who = ref<string | null>('user-a')
    watchSessionCacheReset({ clear }, () => who.value)
    who.value = 'user-b'
    await nextTick()
    expect(clear).toHaveBeenCalledTimes(1)
  })

  it('登出（user → null）同样清缓存，重登（null → user）再清', async () => {
    const clear = vi.fn()
    const who = ref<string | null>('user-a')
    watchSessionCacheReset({ clear }, () => who.value)
    who.value = null
    await nextTick()
    expect(clear).toHaveBeenCalledTimes(1)
    who.value = 'user-b'
    await nextTick()
    expect(clear).toHaveBeenCalledTimes(2)
  })

  it('登录人未变（其他字段刷新）不触发清缓存', async () => {
    const clear = vi.fn()
    const who = ref<string | null>('user-a')
    watchSessionCacheReset({ clear }, () => who.value)
    who.value = 'user-a'
    await nextTick()
    expect(clear).not.toHaveBeenCalled()
  })
})
