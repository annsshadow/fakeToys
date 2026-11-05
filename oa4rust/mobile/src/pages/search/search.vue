<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
/**
 * 搜索（阶段 G / F5）——全局检索（queryview）+ 论坛主题。
 */
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { searchApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'
import { describeLoadError } from '@/utils/loadList'

type Row = Record<string, unknown>

const keyword = ref('')
const rows = ref<Row[]>([])
const loading = ref(false)
const searched = ref(false)
const searchError = ref('')

onShow(async () => {
  await ensureAuthenticated()
})

async function doSearch(): Promise<void> {
  const kw = keyword.value.trim()
  if (!kw || loading.value) return
  loading.value = true
  searched.value = true
  searchError.value = ''
  const failed: string[] = []
  try {
    const out: Row[] = []
    // 两个来源各自容错：任一失败不影响另一来源结果，但失败必须可见。
    try {
      const g = await searchApi.global(kw)
      out.push(...((g.data ?? []) as Row[]).map((r) => ({ ...r, _from: '全局' })))
    } catch (error) {
      failed.push(describeLoadError('全局检索失败', error))
    }
    try {
      const b = await searchApi.bbsSubject(kw)
      out.push(...((b.data ?? []) as Row[]).map((r) => ({ ...r, _from: '论坛' })))
    } catch (error) {
      failed.push(describeLoadError('论坛检索失败', error))
    }
    // 两个来源都失败时保留上一次结果；只要有来源成功，就以本次结果为准。
    if (!failed.length || out.length > 0) rows.value = out
    searchError.value = failed.join('；')
  } finally {
    loading.value = false
  }
}

function titleOf(r: Row): string {
  return String(r.title ?? r.name ?? r.subject ?? r.id ?? '未命名')
}
function descOf(r: Row): string {
  const v = r.content ?? r.description ?? r.summary
  return v ? String(v).slice(0, 120) : ''
}

// 论坛主题结果可深链主题详情（其余来源暂无移动详情页）。
function openRow(r: Row) {
  const id = typeof r.id === 'string' ? r.id : ''
  if (r._from === '论坛' && id) {
    uni.navigateTo({ url: `/pages/bbs/topic?id=${encodeURIComponent(id)}` })
  }
}
</script>

<template>
  <view class="page">
    <view class="searchbar">
      <input
        v-model="keyword"
        class="input"
        placeholder="搜索文档、主题、数据…"
        confirm-type="search"
        @confirm="doSearch"
      />
      <button class="btn" :disabled="loading" @tap="doSearch">搜索</button>
    </view>

    <view v-if="loading" class="tip">搜索中…</view>
    <view v-else-if="rows.length === 0 && searchError" class="tip tip-error">
      <view class="error-text">{{ searchError }}</view>
      <view class="retry-btn" @tap="doSearch">重试</view>
    </view>
    <view v-else-if="searched && rows.length === 0" class="tip">未找到结果</view>

    <view v-if="rows.length > 0 && searchError" class="error-banner">
      <text class="error-text">{{ searchError }}</text>
      <view class="retry-btn" @tap="doSearch">重试</view>
    </view>

    <view v-for="(r, i) in rows" :key="String(r.id ?? i)" class="card" @tap="openRow(r)">
      <view class="card-head">
        <text class="card-title">{{ titleOf(r) }}</text>
        <text class="tag">{{ r._from }}</text>
      </view>
      <view v-if="descOf(r)" class="card-desc">{{ descOf(r) }}</view>
      <view v-if="r._from === '论坛' && r.id" class="card-go">查看主题 ›</view>
    </view>
  </view>
</template>

<style scoped>
.page {
  min-height: 100vh;
  background: #f5f7fa;
  padding: 24rpx;
  box-sizing: border-box;
}
.searchbar {
  display: flex;
  gap: 16rpx;
  margin-bottom: 24rpx;
}
.input {
  flex: 1;
  background: #fff;
  border-radius: 12rpx;
  padding: 18rpx 24rpx;
  font-size: 28rpx;
}
.btn {
  font-size: 28rpx;
  background: #2d8cf0;
  color: #fff;
  border-radius: 12rpx;
  padding: 0 32rpx;
}
.btn[disabled] {
  background: #c0c4cc;
}
.card {
  background: #fff;
  border-radius: 16rpx;
  padding: 24rpx;
  margin-bottom: 20rpx;
}
.card-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12rpx;
}
.card-title {
  font-size: 30rpx;
  font-weight: 700;
  color: #263238;
  flex: 1;
}
.tag {
  font-size: 22rpx;
  color: #2d8cf0;
  background: #e8f3ff;
  border-radius: 8rpx;
  padding: 4rpx 12rpx;
}
.card-desc {
  margin-top: 10rpx;
  font-size: 26rpx;
  color: #5b6572;
  line-height: 1.5;
}
.card-go {
  margin-top: 10rpx;
  font-size: 24rpx;
  color: #2d8cf0;
}
.tip {
  color: #90979f;
  font-size: 26rpx;
  text-align: center;
  padding: 80rpx 0;
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
  background: #fdecea;
  color: #c0392b;
  font-size: 13px;
}
.error-banner .error-text {
  margin-bottom: 0;
}
.retry-btn {
  flex: none;
  padding: 4px 12px;
  border-radius: 12px;
  border: 1px solid #c0392b;
  color: #c0392b;
  font-size: 13px;
}
</style>
