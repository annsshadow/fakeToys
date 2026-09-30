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

.app-root {
  background-color: #0d1117;
  color: #e6edf3;
  min-height: 100vh;
  font-family: -apple-system, BlinkMacSystemFont, 'PingFang SC', 'Microsoft YaHei', sans-serif;
}
</style>
