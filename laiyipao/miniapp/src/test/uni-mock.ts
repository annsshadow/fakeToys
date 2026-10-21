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
  }
  return { mock, storage }
}

/** 安装到 globalThis.uni，返回 { mock, storage } 供测试注入行为与断言 */
export function installUniMock() {
  const { mock, storage } = createUniMock()
  vi.stubGlobal('uni', mock)
  // `getCurrentPages` 是 uni-app 的**全局函数**（不是 uni.xxx 成员），
  // 真机上恒存在。测试里补一个默认「空栈」的桩：
  // store 的登录闸门用它判断「是不是已经在登录页上」。
  vi.stubGlobal('getCurrentPages', vi.fn(() => [] as Array<{ route?: string }>))
  return { mock, storage }
}
