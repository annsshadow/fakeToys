// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node
/**
 * toast 模块的无 DOM 安全降级契约（原生 App 壳 / SSR）。
 *
 * confirmMsg 在 toast.ts:154 明确处理了 `typeof document === 'undefined'`，说明本模块
 * 被设计为可在无 DOM 环境加载。但模块顶层曾直接执行
 * `document.getElementById('oa4-toast-styles')` 注入 keyframes，导致 import 阶段
 * 就抛 `document is not defined`，无 DOM 调用方连 confirmMsg 的降级分支都到不了。
 */
import { afterEach, describe, expect, it, vi } from 'vitest'

describe('toast 模块在无 DOM 环境下安全加载并降级', () => {
  afterEach(() => {
    vi.unstubAllGlobals()
    vi.resetModules()
  })

  it('import 阶段不碰 document；无 DOM 时所有公开 API 静默降级而非抛错', async () => {
    vi.stubGlobal('document', undefined)
    vi.stubGlobal('navigator', undefined)
    vi.resetModules()

    await expect(import('./toast')).resolves.toBeDefined()

    const mod = await import('./toast')
    expect(() => mod.toast.success('保存成功')).not.toThrow()
    expect(() => mod.toast.error('保存失败')).not.toThrow()
    expect(() => mod.toast.info('提示')).not.toThrow()
    expect(() => mod.toast.warning('注意')).not.toThrow()
    await expect(mod.confirmMsg('确定删除？')).resolves.toBe(false)
  })
})
