// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { downloadBlob } from './download'

describe('downloadBlob 工具（node 环境 + 最小 DOM stub）', () => {
  beforeEach(() => {
    vi.stubGlobal('document', {
      createElement: () => ({ click: vi.fn(), href: '', download: '' }),
    })
  })

  afterEach(() => {
    vi.unstubAllGlobals()
    vi.useRealTimers()
  })

  it('创建 ObjectURL 触发下载，并在下一宏任务 revoke（不泄漏）', () => {
    vi.useFakeTimers()
    const create = vi.spyOn(URL, 'createObjectURL').mockReturnValue('blob:mock')
    const revoke = vi.spyOn(URL, 'revokeObjectURL').mockImplementation(() => {})

    downloadBlob(new Blob(['x']), 'a.txt')

    expect(create).toHaveBeenCalledTimes(1)
    expect(revoke).not.toHaveBeenCalled()
    vi.runAllTimers()
    expect(revoke).toHaveBeenCalledWith('blob:mock')
  })
})
