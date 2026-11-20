<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onHide, onShow, onUnload } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { messageApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'
// biome-ignore lint/correctness/noUnusedImports: Vue template consumes the offline copy.
import { NETWORK_OFFLINE_TEXT, type NetworkStatusApi, networkOffline, watchNetworkStatus } from '@/utils/networkStatus'
import { loadConversations } from './messageFeed'

const rows = ref<Record<string, unknown>[]>([])
const loading = ref(false)
const loaded = ref(false)
const loadError = ref('')
// R144：系统网络状态监听；onShow/onHide 可能多次触发，必须只注册一份。
let unwatchNetwork: (() => void) | null = null

function conversationId(row: Record<string, unknown>): string {
  for (const k of ['id', 'conversationId', 'uuid', 'flag']) {
    const v = row[k]
    if (typeof v === 'string' && v) return v
  }
  return ''
}
function conversationTitle(row: Record<string, unknown>): string {
  for (const k of ['name', 'title', 'subject']) {
    const v = row[k]
    if (typeof v === 'string' && v) return v
  }
  return '会话'
}

let loadSeq = 0

async function load() {
  const seq = ++loadSeq
  loading.value = true
  loadError.value = ''
  const result = await loadConversations(() => messageApi.conversationList(), rows.value)
  // onShow 与两处重试按钮共用本函数；序号只让最新一次回写，避免旧响应后到覆盖新数据。
  if (seq !== loadSeq) return
  rows.value = result.rows
  if (!result.ok) loadError.value = result.error
  // 列表能拉到就说明网络可用，撤掉断网提示。
  else networkOffline.value = false
  loading.value = false
  loaded.value = true
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  if (!unwatchNetwork) unwatchNetwork = watchNetworkStatus(uni as unknown as NetworkStatusApi)
  load()
})
onHide(() => {
  unwatchNetwork?.()
  unwatchNetwork = null
})
onUnload(() => {
  unwatchNetwork?.()
  unwatchNetwork = null
})

function openConversation(row: Record<string, unknown>) {
  const id = conversationId(row)
  if (!id) return
  const name = conversationTitle(row)
  uni.navigateTo({ url: `/pages/chat/chat?id=${encodeURIComponent(id)}&name=${encodeURIComponent(name)}` })
}
</script>

<template>
  <view class="page ai-surface-gradient">
    <view v-if="networkOffline" class="offline-banner">{{ NETWORK_OFFLINE_TEXT }}</view>
    <view v-if="loading && !loaded" class="skeleton" aria-hidden="true">
      <view v-for="i in 4" :key="i" class="o2-skeleton skeleton-row"></view>
    </view>
    <view v-else-if="rows.length === 0 && loadError" class="tip tip-error">
      <view class="error-text">{{ loadError }}</view>
      <view class="retry-btn" @tap="load">重试</view>
    </view>
    <view v-else-if="rows.length === 0" class="o2-empty">
      <view class="o2-empty-icon" aria-hidden="true"></view>
      <text class="o2-empty-text">暂无会话</text>
    </view>
    <view v-else class="list">
      <view v-if="loadError" class="tip-error error-banner">
        <text class="error-text">{{ loadError }}</text>
        <view class="retry-btn" @tap="load">重试</view>
      </view>
      <view v-for="(row, i) in rows" :key="i" class="item o2-list-item" @tap="openConversation(row)">
        <view class="title">{{ conversationTitle(row) }}</view>
        <view class="meta">{{ (row.lastMessage as string) || ' ' }}</view>
      </view>
    </view>
  </view>
</template>

<style scoped>
.page {
  min-height: 100vh;
  background: var(--color-bg-page);
}
.offline-banner {
  padding: 16rpx 24rpx;
  background: var(--color-warning-bg);
  color: var(--color-warning-text);
  font-size: 26rpx;
  text-align: center;
}
.tip {
  padding: 120rpx 0;
  text-align: center;
  color: var(--color-text-secondary);
  font-size: 28rpx;
}
.tip-error {
  color: var(--color-error);
}
.error-text {
  display: block;
  margin-bottom: 12rpx;
  line-height: 1.5;
}
.retry-btn {
  display: inline-block;
  padding: 8rpx 28rpx;
  border-radius: 24rpx;
  border: 1rpx solid var(--color-error);
  color: var(--color-error);
  font-size: 26rpx;
}
.error-banner {
  padding: 24rpx 32rpx;
  text-align: center;
  border-bottom: 1px solid var(--color-divider);
}
.list {
  background: var(--color-surface);
}
.item {
  padding: 28rpx 32rpx;
  border-bottom: 1px solid var(--color-divider);
}
.title {
  font-size: 30rpx;
  color: var(--color-text-primary);
  font-weight: 600;
}
.meta {
  margin-top: 8rpx;
  font-size: 24rpx;
  color: var(--color-text-secondary);
}
/* R293：首屏加载用骨架屏替代文字提示，降低等待白屏感。 */
.skeleton {
  padding: 24rpx 32rpx;
  display: flex;
  flex-direction: column;
  gap: 20rpx;
}
.skeleton-row {
  height: 104rpx;
}
</style>
