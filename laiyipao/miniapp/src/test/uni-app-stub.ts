/**
 * @dcloudio/uni-app 的测试替身（通过 vitest.config.ts 的 resolve.alias 挂载）。
 *
 * 为什么需要替身而不是真包：
 * uni-app 的运行时钩子（onLoad/onShow/onLaunch...）从 'vue' 导入内部 API
 * `injectHook`，该导出在 vue 3.4 存在、vue 3.5（本仓库实际解析到的版本）
 * 已被移除 —— 真包一被调用就抛 `vue.injectHook is not a function`。
 * 测试只需要「setup 期间注册回调 + 测试手动触发」，因此这里用最朴素的
 * 实例挂载实现，签名与真实 uni-app 保持一致。
 */
import { getCurrentInstance } from 'vue'

const HOOKS_KEY = '__uni_test_hooks__'

type HookFn = (arg?: unknown) => void

function createHook(name: string) {
  return (hook: HookFn): void => {
    const inst = getCurrentInstance()
    if (!inst) return
    // 组件外注册是 no-op：真实 uni-app 此时会告警，测试里直接忽略
    const bag = ((inst as unknown as Record<string, unknown>)[HOOKS_KEY] ??= {}) as Record<
      string,
      HookFn[]
    >
    ;(bag[name] ??= []).push(hook)
  }
}

export const onLoad = createHook('onLoad')
export const onShow = createHook('onShow')
export const onHide = createHook('onHide')
export const onUnload = createHook('onUnload')
export const onLaunch = createHook('onLaunch')
export const onReady = createHook('onReady')
export const onBackPress = createHook('onBackPress')
export const onReachBottom = createHook('onReachBottom')
export const onPageScroll = createHook('onPageScroll')
export const onPullDownRefresh = createHook('onPullDownRefresh')
export const onShareAppMessage = createHook('onShareAppMessage')
export const onResize = createHook('onResize')

/** 测试辅助：手动触发组件实例上注册的 uni 生命周期回调 */
export function triggerUniHook(
  vm: { $?: unknown } | null | undefined,
  name: string,
  arg?: unknown,
): void {
  const inst = vm?.$ as Record<string, any> | undefined
  const hooks = inst?.[HOOKS_KEY]?.[name] as HookFn[] | undefined
  if (!hooks) return
  for (const h of [...hooks]) h(arg)
}
