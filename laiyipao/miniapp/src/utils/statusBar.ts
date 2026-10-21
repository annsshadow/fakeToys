/**
 * 状态栏高度（2026-10-21 P6-2）。
 *
 * # 为什么不能用 CSS 变量
 *
 * `env(safe-area-inset-*)` 在小程序与 H5 都可用，但**状态栏高度**没有对应的
 * CSS 变量 —— 它是设备相关的（iPhone 的状态栏比安卓高）。
 *
 * 而小程序运行时**没有 DOM**（`document` 不存在），所以也不能在运行时
 * 写一个 CSS 变量上去。
 *
 * 于是只能取到值之后**内联绑定**到元素上。
 *
 * # 为什么需要它
 *
 * 三个页面用了 `navigationStyle: custom`（首页 / 我的 / 战斗）——
 * 自定义导航栏意味着页面自己负责顶部区域。
 * 而项目此前 0 处避让状态栏，顶部那一行（HP 条、资源栏）**在真机上会被状态栏盖住**。
 *
 * `App.vue` 的注释写着「小程序胶囊按钮避让在这里统一处理」——
 * 那条注释描述的是一个**不存在**的行为。
 * 注释是最容易让所有人以为已经做了的形态。
 */
import { computed, ref, type ComputedRef } from 'vue'

const statusBarHeight = ref(0)

/** 取不到时给 0（而不是猜一个值）：猜错的后果是「顶栏偏低」，由页面决定要不要补。 */
export function readStatusBarHeight(): number {
  try {
    const info = uni.getSystemInfoSync() as unknown as { statusBarHeight?: number }
    statusBarHeight.value = Number(info?.statusBarHeight ?? 0) || 0
  } catch {
    statusBarHeight.value = 0
  }
  return statusBarHeight.value
}

/**
 * 顶部内边距（rpx）。
 *
 * 状态栏高度是 px，这里按 750rpx = 屏幕宽 的比例换算。
 * 换算用屏幕宽度而不是固定比值 —— 固定比值在宽屏上会偏大。
 */
export function useStatusBarPad(): { statusBarPad: ComputedRef<string> } {
  const px = computed(() => {
    if (statusBarHeight.value > 0) return statusBarHeight.value
    readStatusBarHeight()
    return statusBarHeight.value
  })
  const statusBarPad = computed(() => {
    // ⚠️ 2026-10-21（P2-7）：安卓/桌面 H5 上 `statusBarHeight` 常态是 **0**
    // （只有 iOS 才有状态栏）。所以「0 就当没有状态栏」是错的 ——
    // 那等于在安卓上**完全不让开顶部**。
    //
    // 正确做法：0 时给一个**保守的兜底**（20rpx ≈ 10px），
    // 与 store 里那句注释（「取不到时给 20rpx 而不是 0」）一致。
    if (px.value <= 0) return '20rpx'
    try {
      const info = uni.getSystemInfoSync() as unknown as { windowWidth?: number }
      const w = Number(info?.windowWidth ?? 375) || 375
      return `${Math.round((px.value * 750) / w)}rpx`
    } catch {
      return '20rpx'
    }
  })
  return { statusBarPad }
}
