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

async function load() {
  loading.value = true
  loadError.value = ''
  const result = await loadConversations(() => messageApi.conversationList(), rows.value)
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
  <view class="page">
    <view v-if="networkOffline" class="offline-banner">{{ NETWORK_OFFLINE_TEXT }}</view>
    <view v-if="loading && !loaded" class="tip">加载中…</view>
    <view v-else-if="rows.length === 0 && loadError" class="tip tip-error">
      <view class="error-text">{{ loadError }}</view>
      <view class="retry-btn" @tap="load">重试</view>
    </view>
    <view v-else-if="rows.length === 0" class="tip">暂无会话</view>
    <view v-else class="list">
      <view v-if="loadError" class="tip-error error-banner">
        <text class="error-text">{{ loadError }}</text>
        <view class="retry-btn" @tap="load">重试</view>
      </view>
      <view v-for="(row, i) in rows" :key="i" class="item" @tap="openConversation(row)">
        <view class="title">{{ conversationTitle(row) }}</view>
        <view class="meta">{{ (row.lastMessage as string) || ' ' }}</view>
      </view>
    </view>
  </view>
</template>

<style scoped>
.page {
  min-height: 100vh;
  background: #f5f7fa;
}
.offline-banner {
  padding: 16rpx 24rpx;
  background: #fff4e5;
  color: #b26a00;
  font-size: 26rpx;
  text-align: center;
}
.tip {
  padding: 120rpx 0;
  text-align: center;
  color: #90979f;
  font-size: 28rpx;
}
.tip-error {
  color: #c0392b;
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
  border: 1rpx solid #c0392b;
  color: #c0392b;
  font-size: 26rpx;
}
.error-banner {
  padding: 24rpx 32rpx;
  text-align: center;
  border-bottom: 1px solid #f0f2f5;
}
.list {
  background: #fff;
}
.item {
  padding: 28rpx 32rpx;
  border-bottom: 1px solid #f0f2f5;
}
.title {
  font-size: 30rpx;
  color: #263238;
  font-weight: 600;
}
.meta {
  margin-top: 8rpx;
  font-size: 24rpx;
  color: #90979f;
}
</style>
