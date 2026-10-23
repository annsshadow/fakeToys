/**
 * 全局 `uni` 对象的可编程 mock。
 *
 * 小程序端所有平台能力都走全局 `uni`（request / storage / 导航 / toast...），
 * node 与 jsdom 环境都没有它。这里做一个**可编程**的 mock：
 * 各方法都是 vi.fn()，默认行为覆盖 storage 三件套与系统信息，
 * request / createSelectorQuery / showToast 等由每个测试按场景注入实现，
 * 从而能断言「页面确实调用了 uni.xxx 且参数正确」。
 */
import { vi } from 'vitest'

export type UniMock = ReturnType<typeof createUniMock>['mock']

export function createUniMock() {
  // 内存存储：get/set/remove 语义与 uni 对齐（get 未命中返回 ''）
  const storage = new Map<string, string>()
  /**
   * 创建过的音频实例（2026-10-23）。
   *
   * 音效模块的判据之一是「同一音效只建一个 context，复用而不是每次播放都建」
   * —— 那要求测试能数清楚 `createInnerAudioContext` 被调了几次、
   * 以及每个实例被 `play()` 了几次。所以实例本身也必须是可断言的。
   */
  const audioInstances: Array<{
    src: string
    volume: number
    loop: boolean
    play: ReturnType<typeof vi.fn>
    stop: ReturnType<typeof vi.fn>
    destroy: ReturnType<typeof vi.fn>
  }> = []
  const mock = {
    request: vi.fn(),
    uploadFile: vi.fn(),
    getStorageSync: vi.fn((key: string) => (storage.has(key) ? (storage.get(key) as string) : '')),
    setStorageSync: vi.fn((key: string, value: string) => {
      storage.set(key, value)
    }),
    removeStorageSync: vi.fn((key: string) => {
      storage.delete(key)
    }),
    showToast: vi.fn(),
    navigateTo: vi.fn(),
    navigateBack: vi.fn(),
    switchTab: vi.fn(),
    // ⚠️ reLaunch 与 navigateTo 是**两件事**：
    // reLaunch 会清掉页面栈（登录页是闸门，必须用它）。
    // 此前 mock 里没有它，于是 me.vue 的「退出登录」路径在测试里
    // 调的是 undefined —— 缺的桩会让「登出后去哪」这条分支形同虚测。
    reLaunch: vi.fn(),
    getSystemInfoSync: vi.fn(() => ({
      platform: 'devtools',
      SDKVersion: 'test',
      pixelRatio: 2,
      windowWidth: 375,
      windowHeight: 667,
    })),
    createSelectorQuery: vi.fn(),
    /**
     * 音频上下文。
     *
     * 默认返回一个**可用的假实例**而不是 undefined：音效模块一旦被某个页面
     * 间接调用，undefined 会让整页崩在「看起来无关」的地方。
     * 需要精确控制时，测试可以 `mockImplementation` 覆盖它。
     */
    createInnerAudioContext: vi.fn(() => {
      const a = {
        src: '',
        volume: 1,
        loop: false,
        play: vi.fn(),
        stop: vi.fn(),
        destroy: vi.fn(),
      }
      audioInstances.push(a)
      return a
    }),
  }
  return { mock, storage, audioInstances }
}

/** 安装到 globalThis.uni，返回 { mock, storage } 供测试注入行为与断言 */
export function installUniMock() {
  const { mock, storage, audioInstances } = createUniMock()
  vi.stubGlobal('uni', mock)
  // `getCurrentPages` 是 uni-app 的**全局函数**（不是 uni.xxx 成员），
  // 真机上恒存在。测试里补一个默认「空栈」的桩：
  // store 的登录闸门用它判断「是不是已经在登录页上」。
  vi.stubGlobal('getCurrentPages', vi.fn(() => [] as Array<{ route?: string }>))
  return { mock, storage, audioInstances }
}
