// @vitest-environment jsdom
/**
 * 应用入口（main.ts）测试。
 * uni-app 的入口约定是导出 createApp()：内部 createSSRApp + 挂载 Pinia。
 * 这里验证返回结构与 Pinia 安装副作用（$pinia 挂到 globalProperties）。
 */
import { describe, it, expect } from 'vitest'
import { createApp } from './main'

describe('main.ts 应用入口', () => {
  it('createApp 返回 { app }，且已安装 Pinia（$pinia 可用）', () => {
    const { app } = createApp()
    expect(app).toBeTruthy()
    // pinia 的 install 会把实例挂到全局属性上
    expect((app.config.globalProperties as any).$pinia).toBeTruthy()
  })

  it('多次调用返回独立的应用实例', () => {
    const a = createApp()
    const b = createApp()
    expect(a.app).not.toBe(b.app)
  })
})
