<script setup lang="ts">
// 应用根组件。平台差异（H5 标题栏 / 小程序胶囊按钮避让）在这里统一处理。
import { onLaunch } from '@dcloudio/uni-app'

onLaunch(() => {
  const info = uni.getSystemInfoSync()
  console.log('[来一炮] launch on', info.platform, 'engine', info.SDKVersion ?? '-')
})
</script>

<template>
  <view class="app-root">
    <slot />
  </view>
</template>

<style>
@import './styles/common.css';

/*
 * 2026-11-16：删除这里与 common.css 重复的背景/文字/字体声明 ——
 * 同一套主题之前定义了两次（`page` 一次、`.app-root` 一次），字体栈逐字重复。
 * 主题真源在 styles/common.css；.app-root 只保留布局职责。
 */
.app-root {
  min-height: 100vh;
}
</style>
