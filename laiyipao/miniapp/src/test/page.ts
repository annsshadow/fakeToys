/**
 * 页面测试共享辅助：创建独立 Pinia 并挂载页面组件。
 * 页面 setup 里 useGameStore() 会命中同一个 active pinia，
 * 测试因此能直接读写 store 状态（注入登录态/配置等前置条件）。
 */
import { mount, type VueWrapper } from '@vue/test-utils'
import { createPinia, setActivePinia, type Pinia } from 'pinia'
import { useGameStore } from '@/store/game'

export interface MountedPage {
  wrapper: VueWrapper<any>
  pinia: Pinia
  store: ReturnType<typeof useGameStore>
}

export function mountPage(component: unknown, beforeMount?: (store: ReturnType<typeof useGameStore>) => void): MountedPage {
  const pinia = createPinia()
  setActivePinia(pinia)
  const store = useGameStore()
  beforeMount?.(store)
  const wrapper = mount(component as any, { global: { plugins: [pinia] } })
  return { wrapper, pinia, store }
}
