// @vitest-environment jsdom
/**
 * 应用根组件（App.vue）测试。
 * onLaunch 里会读取系统信息并打印平台日志 —— 这是平台差异统一处理的入口，
 * 替身生命周期钩子手动触发以验证其调用与渲染。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import App from './App.vue'
import { installUniMock } from './test/uni-mock'
import { triggerUniHook } from './test/uni-app-stub'
import { mountPage } from './test/page'

let um: ReturnType<typeof installUniMock>
let logSpy: ReturnType<typeof vi.spyOn>

beforeEach(() => {
  um = installUniMock()
  logSpy = vi.spyOn(console, 'log').mockImplementation(() => {})
})

afterEach(() => {
  logSpy.mockRestore()
  vi.unstubAllGlobals()
})

describe('App.vue 应用根组件', () => {
  it('onLaunch：读取系统信息并打印平台与 SDK 版本', async () => {
    const { wrapper } = mountPage(App)
    await flushPromises()
    triggerUniHook(wrapper.vm, 'onLaunch')
    await flushPromises()
    expect(um.mock.getSystemInfoSync).toHaveBeenCalledTimes(1)
    expect(logSpy).toHaveBeenCalledWith(
      '[来一炮] launch on', 'devtools', 'engine', 'test',
    )
  })

  it('SDKVersion 缺失 → 用 "-" 占位（?? 分支）', async () => {
    um.mock.getSystemInfoSync.mockImplementation(() => ({
      platform: 'mp-weixin',
      SDKVersion: undefined,
    }) as any)
    const { wrapper } = mountPage(App)
    await flushPromises()
    triggerUniHook(wrapper.vm, 'onLaunch')
    await flushPromises()
    expect(logSpy).toHaveBeenCalledWith(
      '[来一炮] launch on', 'mp-weixin', 'engine', '-',
    )
  })

  it('渲染 slot 内容', async () => {
    const { mount } = await import('@vue/test-utils')
    const { h } = await import('vue')
    const wrapper = mount(App, {
      // 用渲染函数作 slot 子组件（测试环境无运行时模板编译器）
      slots: { default: () => h('text', { class: 'slot-probe' }, '页面内容') },
    })
    expect(wrapper.find('.app-root').exists()).toBe(true)
    expect(wrapper.find('.slot-probe').exists()).toBe(true)
    wrapper.unmount()
  })
})
