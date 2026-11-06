// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { ApiError, PermissionError } from '@oa4rust/sdk'
import { describe, expect, it } from 'vitest'
import { COLLECT_FORBIDDEN_MESSAGE, COLLECT_GENERIC_ERROR_MESSAGE, collectMutationErrorMessage } from './collectError'

describe('收藏写操作错误文案（403 不再静默）', () => {
  it('403（非属主）→ 明确的无权限文案', () => {
    expect(collectMutationErrorMessage(new PermissionError('forbidden'))).toBe(COLLECT_FORBIDDEN_MESSAGE)
  })

  it('其它错误 → 通用失败文案', () => {
    expect(collectMutationErrorMessage(new ApiError('boom', 500))).toBe(COLLECT_GENERIC_ERROR_MESSAGE)
    expect(collectMutationErrorMessage(new Error('network'))).toBe(COLLECT_GENERIC_ERROR_MESSAGE)
  })
})

describe('CollectApp 两条写 mutation 都挂了 onError（防静默回归）', () => {
  const src = readFileSync(resolve(import.meta.dirname, 'CollectApp.vue'), 'utf8')

  it('save 与 delete 各有一条 onError → toast.error', () => {
    const wired = src
      .split('\n')
      .filter((l) => l.includes('onError: (e) => toast.error(collectMutationErrorMessage(e))'))
    expect(wired).toHaveLength(2)
  })

  it('保存中的视觉状态在 finally 里复位（失败也不会卡在 saving）', () => {
    expect(src).toContain('finally {')
    expect(src).toContain('saving.value = false')
  })
})
