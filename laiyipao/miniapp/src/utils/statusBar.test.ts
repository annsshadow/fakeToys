/**
 * 状态栏避让的行为守卫（2026-10-21 P6-2 的配套判据）。
 *
 * 为什么值得单独测：这段代码**只在真机上出错**，而且是「被状态栏盖住」这种
 * 不会抛异常、只会难看的问题。三条硬约束：
 *
 * 1. 取不到高度时必须返回 0（而不是猜一个值）——猜错的后果由页面承担；
 * 2. 但**渲染**时 0 不能当「没有状态栏」：安卓/桌面 H5 常态是 0，
 *    若返回 `0rpx` 等于在这些设备上完全不让开顶部（P2-7 的坑）；
 * 3. px→rpx 必须按 `windowWidth` 换算，而不是固定比值（宽屏会偏大）。
 */
import { afterEach, describe, expect, it, vi } from 'vitest'
import { readStatusBarHeight, useStatusBarPad } from '@/utils/statusBar'

function stubSystemInfo(info: Record<string, unknown> | 'throw') {
  vi.stubGlobal('uni', {
    getSystemInfoSync: () => {
      if (info === 'throw') throw new Error('no runtime')
      return info
    },
  })
}

afterEach(() => {
  vi.unstubAllGlobals()
  vi.restoreAllMocks()
})

describe('readStatusBarHeight', () => {
  it('returns the platform value when available', () => {
    stubSystemInfo({ statusBarHeight: 44 })
    expect(readStatusBarHeight()).toBe(44)
  })

  it('returns 0 instead of guessing when the runtime throws', () => {
    // 小程序/H5 的 getSystemInfoSync 在某些宿主里会抛；不能因此让页面白屏
    stubSystemInfo('throw')
    expect(readStatusBarHeight()).toBe(0)
  })

  it('coerces missing or non-numeric values to 0', () => {
    stubSystemInfo({})
    expect(readStatusBarHeight()).toBe(0)

    stubSystemInfo({ statusBarHeight: Number.NaN })
    expect(readStatusBarHeight()).toBe(0)

    // 字符串数字是宿主可能给的形态，Number() 能接住
    stubSystemInfo({ statusBarHeight: '20' })
    expect(readStatusBarHeight()).toBe(20)
  })
})

describe('useStatusBarPad', () => {
  it('converts px to rpx using the actual window width', () => {
    // 750rpx = 屏宽；44px 在 375px 宽上 = 88rpx
    stubSystemInfo({ statusBarHeight: 44, windowWidth: 375 })
    readStatusBarHeight()
    expect(useStatusBarPad().statusBarPad.value).toBe('88rpx')
  })

  it('scales differently on a wide screen instead of using a fixed ratio', () => {
    // 同样是 44px，在 750px 宽上只占 44rpx —— 固定比值会算成 88rpx（偏大一倍）
    stubSystemInfo({ statusBarHeight: 44, windowWidth: 750 })
    readStatusBarHeight()
    expect(useStatusBarPad().statusBarPad.value).toBe('44rpx')
  })

  it('falls back to a conservative 20rpx when the platform reports 0', () => {
    // P2-7：安卓/桌面 H5 常态是 0。返回 '0rpx' 等于完全不让开顶部。
    stubSystemInfo({ statusBarHeight: 0, windowWidth: 375 })
    readStatusBarHeight()
    expect(useStatusBarPad().statusBarPad.value).toBe('20rpx')
  })

  it('treats negative heights as "unavailable"', () => {
    stubSystemInfo({ statusBarHeight: -5, windowWidth: 375 })
    readStatusBarHeight()
    expect(useStatusBarPad().statusBarPad.value).toBe('20rpx')
  })

  it('falls back to 20rpx when the runtime throws during conversion', () => {
    stubSystemInfo('throw')
    readStatusBarHeight()
    expect(useStatusBarPad().statusBarPad.value).toBe('20rpx')
  })

  it('defaults windowWidth to 375 when absent or zero', () => {
    // 除以 0 会得到 Infinity → `Infinityrpx` 是无效 CSS，顶栏会直接失效
    stubSystemInfo({ statusBarHeight: 40 })
    readStatusBarHeight()
    expect(useStatusBarPad().statusBarPad.value).toBe('80rpx')

    stubSystemInfo({ statusBarHeight: 40, windowWidth: 0 })
    readStatusBarHeight()
    expect(useStatusBarPad().statusBarPad.value).toBe('80rpx')
  })
})
