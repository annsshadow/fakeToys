<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { bbsApi } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'

const session = useSession()
const rows = ref<Record<string, unknown>[]>([])
const loading = ref(false)
const loaded = ref(false)

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
  if (!session.user?.unique) return
  loading.value = true
  try {
    const resp = await bbsApi.mobileViewAll()
    rows.value = resp.data ?? []
  } catch {
    rows.value = []
  } finally {
    loading.value = false
    loaded.value = true
  }
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
  <view class="page">
    <view class="toolbar">
      <view class="toolbar-title">最新主题</view>
      <view class="new-btn" @tap="newTopic">发帖</view>
    </view>
    <view v-if="loading && !loaded" class="tip">加载中…</view>
    <view v-else-if="loaded && rows.length === 0" class="tip">论坛暂无主题</view>
    <view v-else class="topic-list">
      <view v-for="(row, i) in rows" :key="i" class="topic" @tap="openTopic(row)">
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
  color: #1f2733;
}
.new-btn {
  padding: 5px 14px;
  border-radius: 15px;
  background: #2f6bff;
  color: #ffffff;
  font-size: 13px;
}
.tip {
  text-align: center;
  color: #8a94a6;
  padding: 40px 0;
  font-size: 14px;
}
.topic-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.topic {
  padding: 12px;
  border-radius: 10px;
  background: #ffffff;
}
.topic-title {
  font-size: 15px;
  font-weight: 600;
  color: #1f2733;
}
.topic-excerpt {
  margin-top: 4px;
  font-size: 13px;
  color: #5a6478;
}
.topic-meta {
  margin-top: 6px;
  font-size: 12px;
  color: #8a94a6;
}
</style>
