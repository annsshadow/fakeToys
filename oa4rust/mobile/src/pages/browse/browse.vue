<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
/**
 * 只读浏览（阶段 G / F4）——移动端以「看」为主：门户页面 / 查询视图 / CMS 栏目。
 * 三个数据源分页签，各自只读列表 + 详情。
 */
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { cmsApi, portalApi, queryviewApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'
import { createLatestGate } from '@/utils/latest'
import { loadBrowseRows } from './browseFeed'

type TabKey = 'portal' | 'query' | 'cms'

const tabs: Array<{ key: TabKey; label: string }> = [
  { key: 'portal', label: '门户' },
  { key: 'query', label: '查询' },
  { key: 'cms', label: '栏目' },
]

type Row = Record<string, unknown>

const tab = ref<TabKey>('portal')
const rows = ref<Row[]>([])
const loading = ref(false)
const loadError = ref('')
const detail = ref<Row | null>(null)
const detailLoading = ref(false)
const detailGate = createLatestGate()
// 页签切换/刷新的在途列表请求只认最新一次，见 utils/latest。
const listGate = createLatestGate()

function nameOf(r: Row): string {
  return String(r.name ?? r.title ?? r.flag ?? r.id ?? '未命名')
}
function descOf(r: Row): string {
  const v = r.description ?? r.alias ?? r.category ?? r.type
  return v ? String(v) : ''
}

function fetchList(): Promise<{ data?: unknown }> {
  if (tab.value === 'portal') return portalApi.pageList('default')
  // queryFlag='all' 表示不限查询（后端 list/query/{queryFlag}）
  if (tab.value === 'query') return queryviewApi.viewList('all')
  return cmsApi.columnList()
}

async function load(previousRows: Row[] = rows.value): Promise<void> {
  // 页签切得快时旧请求会晚到：只认最新一次，过期响应不写 rows/错误、不提前复位 loading。
  const token = listGate.begin()
  loading.value = true
  loadError.value = ''
  detail.value = null
  const result = await loadBrowseRows(() => fetchList(), previousRows)
  if (!listGate.isLatest(token)) return
  rows.value = result.rows
  if (!result.ok) loadError.value = result.error
  loading.value = false
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  void load()
})

function switchTab(key: TabKey): void {
  if (tab.value === key) return
  tab.value = key
  // 页签是用户主动换数据源：不沿用旧页签的行，失败就显示该页签自己的错误。
  void load([])
}

async function open(r: Row): Promise<void> {
  const token = detailGate.begin()
  detailLoading.value = true
  detail.value = null
  try {
    const id = String(r.id ?? r.flag ?? '')
    if (!id) {
      detail.value = r
      return
    }
    let fetched: Row
    if (tab.value === 'portal') {
      fetched = ((await portalApi.pageDetail(id)) as { data?: Row }).data ?? r
    } else if (tab.value === 'query') {
      const view = String(r.view_flag ?? r.viewFlag ?? 'view')
      fetched = ((await queryviewApi.execute(view, id)) as { data?: Row }).data ?? r
    } else {
      const resp = await cmsApi.documentSearch(nameOf(r))
      const list = (resp.data ?? []) as Row[]
      fetched = { 文档数: list.length, 栏目: nameOf(r), 首篇: list[0] ? nameOf(list[0]) : '—' }
    }
    if (!detailGate.isLatest(token)) return
    detail.value = fetched
  } catch (e) {
    if (!detailGate.isLatest(token)) return
    uni.showToast({ title: e instanceof Error ? e.message : '加载失败', icon: 'none' })
  } finally {
    if (detailGate.isLatest(token)) detailLoading.value = false
  }
}

function detailText(): string {
  if (!detail.value) return ''
  try {
    return JSON.stringify(detail.value, null, 2)
  } catch {
    return String(detail.value)
  }
}
</script>

<template>
  <view class="page">
    <view class="tabs">
      <view v-for="t in tabs" :key="t.key" class="tab" :class="{ on: tab === t.key }" @tap="switchTab(t.key)">
        {{ t.label }}
      </view>
    </view>

    <view v-if="loading" class="tip">加载中…</view>
    <view v-else-if="rows.length === 0 && loadError" class="tip tip-error">
      <view class="error-text">{{ loadError }}</view>
      <view class="retry-btn" @tap="load()">重试</view>
    </view>
    <view v-else-if="rows.length === 0" class="tip">暂无数据</view>

    <view v-else>
      <view v-if="loadError" class="tip-error error-banner">
        <text class="error-text">{{ loadError }}</text>
        <view class="retry-btn" @tap="load()">重试</view>
      </view>
      <view v-for="(r, i) in rows" :key="String(r.id ?? i)" class="card" @tap="open(r)">
        <view class="card-title">{{ nameOf(r) }}</view>
        <view v-if="descOf(r)" class="card-desc">{{ descOf(r) }}</view>
      </view>
    </view>

    <view v-if="detailLoading" class="tip">详情加载中…</view>
    <view v-else-if="detail" class="detail">
      <view class="detail-head">
        <text class="detail-title">详情</text>
        <text class="detail-close" @tap="detail = null">关闭</text>
      </view>
      <text class="detail-body">{{ detailText() }}</text>
    </view>
  </view>
</template>

<style scoped>
.page {
  min-height: 100vh;
  background: var(--color-bg-page);
  padding: 24rpx;
  box-sizing: border-box;
}
.tabs {
  display: flex;
  gap: 12rpx;
  margin-bottom: 24rpx;
}
.tab {
  flex: 1;
  text-align: center;
  padding: 16rpx 0;
  font-size: 28rpx;
  color: var(--color-text-body);
  background: var(--color-surface);
  border-radius: 12rpx;
}
.tab.on {
  background: var(--color-primary);
  color: var(--color-surface);
  font-weight: 600;
}
.card {
  background: var(--color-surface);
  border-radius: 16rpx;
  padding: 24rpx;
  margin-bottom: 20rpx;
}
.card-title {
  font-size: 30rpx;
  font-weight: 700;
  color: var(--color-text-primary);
}
.card-desc {
  margin-top: 8rpx;
  font-size: 24rpx;
  color: var(--color-text-secondary);
}
.detail {
  background: var(--color-surface);
  border-radius: 16rpx;
  padding: 24rpx;
  margin-top: 8rpx;
}
.detail-head {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16rpx;
}
.detail-title {
  font-size: 28rpx;
  font-weight: 700;
  color: var(--color-text-primary);
}
.detail-close {
  font-size: 26rpx;
  color: var(--color-primary);
}
.detail-body {
  font-size: 22rpx;
  color: var(--color-text-body);
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-all;
}
.tip {
  color: var(--color-text-secondary);
  font-size: 26rpx;
  text-align: center;
  padding: 80rpx 0;
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
</style>
