<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { bbsApi } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'
import { createLatestGate } from '@/utils/latest'
import { loadBbsTopics } from './bbsFeed'

const session = useSession()
const rows = ref<Record<string, unknown>[]>([])
const loading = ref(false)
const loaded = ref(false)
const bbsError = ref('')
const listGate = createLatestGate()

function titleOf(row: Record<string, unknown>): string {
  const v = row.title
  return typeof v === 'string' && v ? v : '（无标题）'
}
function creatorOf(row: Record<string, unknown>): string {
  const v = row.creator
  return typeof v === 'string' && v ? v : '匿名'
}
function timeOf(row: Record<string, unknown>): string {
  // 后端 TopicRow 序列化为 snake_case（create_time），camelCase 仅作旧形状兜底。
  const v = row.create_time ?? row.createTime
  return typeof v === 'string' && v ? v.slice(0, 16) : ''
}
function excerptOf(row: Record<string, unknown>): string {
  const v = row.content
  if (typeof v !== 'string' || !v) return ''
  const flat = v.replace(/\s+/g, ' ').trim()
  return flat.length > 60 ? `${flat.slice(0, 60)}…` : flat
}

async function load() {
  if (!session.user?.unique) {
    // 未登录也要可见：静默 return 会让页面停在空白态。
    bbsError.value = '未登录，无法加载主题'
    loaded.value = true
    return
  }
  const token = listGate.begin()
  loading.value = true
  bbsError.value = ''
  const result = await loadBbsTopics(() => bbsApi.mobileViewAll(), rows.value)
  // 下拉刷新 / 重试可并发触发：过期响应不得覆盖新请求的数据，也不得提前复位 loading。
  if (!listGate.isLatest(token)) return
  rows.value = result.rows
  if (!result.ok) bbsError.value = result.error
  loading.value = false
  loaded.value = true
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  load()
})

/** 下拉刷新（uni-app 约定 onPullDownRefresh，页面 style 已开 enablePullDownRefresh）。 */
async function onPullDownRefresh() {
  await load()
  uni.stopPullDownRefresh()
}

function openTopic(row: Record<string, unknown>) {
  const id = typeof row.id === 'string' ? row.id : ''
  if (!id) return
  uni.navigateTo({ url: `/pages/bbs/topic?id=${encodeURIComponent(id)}` })
}

function newTopic() {
  uni.navigateTo({ url: '/pages/bbs/new' })
}
</script>

<template>
  <view class="page ai-surface-gradient">
    <view class="toolbar">
      <view class="toolbar-title">最新主题</view>
      <view class="new-btn ai-brand-fill" @tap="newTopic">发帖</view>
    </view>
    <view v-if="loading && !loaded" class="skeleton" aria-hidden="true">
      <view v-for="i in 4" :key="i" class="o2-skeleton skeleton-row"></view>
    </view>
    <view v-else-if="rows.length === 0 && bbsError" class="tip tip-error">
      <view class="error-text">{{ bbsError }}</view>
      <view class="retry-btn" @tap="load">重试</view>
    </view>
    <view v-else-if="rows.length === 0" class="o2-empty">
      <view class="o2-empty-icon" aria-hidden="true"></view>
      <text class="o2-empty-text">论坛暂无主题</text>
    </view>
    <view v-else class="topic-list">
      <view v-if="bbsError" class="error-banner">
        <text class="error-text">{{ bbsError }}</text>
        <view class="retry-btn" @tap="load">重试</view>
      </view>
      <view v-for="(row, i) in rows" :key="i" class="topic o2-list-item" @tap="openTopic(row)">
        <view class="topic-title">{{ titleOf(row) }}</view>
        <view v-if="excerptOf(row)" class="topic-excerpt">{{ excerptOf(row) }}</view>
        <view class="topic-meta">{{ creatorOf(row) }} · {{ timeOf(row) }}</view>
      </view>
    </view>
  </view>
</template>

<style scoped>
.page {
  padding: 12px;
  min-height: 100vh;
}
.toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 10px;
}
.toolbar-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--color-text-primary);
}
.new-btn {
  padding: 5px 14px;
  border-radius: 15px;
  background: var(--color-primary);
  color: var(--color-surface);
  font-size: 13px;
}
.tip {
  text-align: center;
  color: var(--color-text-muted);
  padding: 40px 0;
  font-size: 14px;
}
.tip-error {
  color: var(--color-error);
}
.error-text {
  margin-bottom: 10px;
  line-height: 1.5;
}
.error-banner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  padding: 10px 12px;
  border-radius: 8px;
  background: var(--color-error-bg);
  color: var(--color-error);
  font-size: 13px;
}
.error-banner .error-text {
  margin-bottom: 0;
}
.retry-btn {
  flex: none;
  padding: 4px 12px;
  border-radius: 12px;
  border: 1px solid var(--color-error);
  color: var(--color-error);
  font-size: 13px;
}
.topic-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.topic {
  padding: 12px;
  border-radius: 10px;
  background: var(--color-surface);
}
.topic-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--color-text-primary);
}
.topic-excerpt {
  margin-top: 4px;
  font-size: 13px;
  color: var(--color-text-secondary);
}
.topic-meta {
  margin-top: 6px;
  font-size: 12px;
  color: var(--color-text-muted);
}
/* R293：首屏加载用骨架屏替代文字提示。 */
.skeleton {
  padding: 4px 0 0;
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.skeleton-row {
  height: 72px;
}
</style>
