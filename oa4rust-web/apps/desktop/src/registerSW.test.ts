// @vitest-environment node
/**
 * Service Worker 注册守卫单测：只在（a）navigator 支持 serviceWorker
 * 且（b）生产构建（import.meta.env.PROD）时才注册；其余场景静默 no-op。
 */
import { afterEach, describe, expect, it, vi } from 'vitest'
import { registerSW } from './registerSW'

describe('registerSW', () => {
  afterEach(() => {
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('no-ops when navigator lacks serviceWorker support (e.g. 非安全上下文)', async () => {
    vi.stubGlobal('navigator', {})
    await expect(registerSW()).resolves.toBeUndefined()
  })

  it('never registers outside a production build (dev/test 环境走 no-op 分支)', async () => {
    const register = vi.fn().mockRejectedValue(new Error('insecure'))
    vi.stubGlobal('navigator', { serviceWorker: { register, controller: null } })
    // vitest 运行在 MODE=test → import.meta.env.PROD 为 false，注册分支不得进入，
    // 即使 register 本身是坏的也不得抛错 / 触发警告。
    const warn = vi.spyOn(console, 'warn').mockImplementation(() => {})
    await expect(registerSW()).resolves.toBeUndefined()
    expect(register).not.toHaveBeenCalled()
    expect(warn).not.toHaveBeenCalled()
  })
})
