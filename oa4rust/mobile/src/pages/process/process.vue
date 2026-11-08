<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { processApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'
import { createLatestGate } from '@/utils/latest'
import { describeLoadError } from '@/utils/loadList'
import { describeProcessError, loadProcessPage, mergeProcessRows } from './processList'

type TabKey = 'pending' | 'done' | 'started'
const tabs: { key: TabKey; label: string }[] = [
  { key: 'pending', label: '待办' },
  { key: 'done', label: '已办' },
  { key: 'started', label: '我发起' },
]
const active = ref<TabKey>('pending')
const rows = ref<Record<string, unknown>[]>([])
const page = ref(1)
const hasMore = ref(false)
const loadError = ref('')
const loading = ref(false)
const loadingMore = ref(false)
const busy = ref(false)
// 页签/加载更多的在途请求只认最新一次，见 utils/latest。
const pageGate = createLatestGate()

// 详情浮层：附件 / 流转记录 / 工作日志（按 work 维度拉取真实后端能力）
const detailOpen = ref(false)
const detailTitle = ref('')
const detailLoading = ref(false)
const detailAttachments = ref<Record<string, unknown>[]>([])
const detailRecords = ref<Record<string, unknown>[]>([])
const detailWorklogs = ref<Record<string, unknown>[]>([])
const detailError = ref('')
const detailTarget = ref<{ row: Record<string, unknown>; i: number } | null>(null)

function workIdOf(row: Record<string, unknown>): string {
  for (const k of ['work', 'workId', 'id']) {
    const v = row[k]
    if (typeof v === 'string' && v) return v
  }
  return ''
}

async function openDetail(row: Record<string, unknown>, i: number) {
  const id = workIdOf(row)
  if (!id) return
  detailOpen.value = true
  detailTitle.value = titleOf(row, i)
  detailTarget.value = { row, i }
  detailLoading.value = true
  detailError.value = ''
  detailAttachments.value = []
  detailRecords.value = []
  detailWorklogs.value = []
  const settle = async <T>(p: Promise<{ data?: T[] }>, label: string): Promise<T[]> => {
    try {
      return (await p).data ?? []
    } catch (e) {
      // 单节失败置空不该静默：记下第一条错误，详情浮层顶部统一展示。
      if (!detailError.value) detailError.value = describeLoadError(label, e)
      return []
    }
  }
  const [att, rec, log] = await Promise.all([
    settle(processApi.attachments(id), '附件加载失败'),
    settle(processApi.records(id), '流转记录加载失败'),
    settle(processApi.worklogs(id), '工作日志加载失败'),
  ])
  detailAttachments.value = att
  detailRecords.value = rec
  detailWorklogs.value = log
  detailLoading.value = false
}

function retryDetail() {
  const target = detailTarget.value
  if (target) void openDetail(target.row, target.i)
}

function closeDetail() {
  detailOpen.value = false
  detailError.value = ''
  detailTarget.value = null
}

function textOf(row: Record<string, unknown>, keys: string[], fallback = ''): string {
  for (const k of keys) {
    const v = row[k]
    if (typeof v === 'string' && v) return v
  }
  return fallback
}

function titleOf(row: Record<string, unknown>, i: number): string {
  for (const k of ['title', 'subject', 'processName']) {
    const v = row[k]
    if (typeof v === 'string' && v) return v
  }
  return `流程 #${i + 1}`
}
function metaOf(row: Record<string, unknown>): string {
  const parts: string[] = []
  for (const k of ['applicationName', 'taskStatus', 'workStatus', 'createTime']) {
    const v = row[k]
    if (typeof v === 'string' && v) parts.push(v)
  }
  return parts.join(' · ')
}

function fetchPage(tab: TabKey, targetPage: number, size: number) {
  return tab === 'pending'
    ? processApi.pendingList(targetPage, size)
    : tab === 'done'
      ? processApi.completedList(targetPage, size)
      : processApi.startedList(targetPage, size)
}

async function loadFirst() {
  // 页签切得快时旧请求会晚到：只认最新一次，过期响应不写 rows/hasMore。
  const token = pageGate.begin()
  const targetTab = active.value
  loading.value = true
  loadError.value = ''
  try {
    const result = await loadProcessPage<Record<string, unknown>>(fetchPage, targetTab, 1)
    if (!pageGate.isLatest(token)) return
    rows.value = mergeProcessRows<Record<string, unknown>>([], result.rows)
    page.value = result.page
    hasMore.value = result.hasMore
  } catch (e) {
    if (!pageGate.isLatest(token)) return
    // 失败与「真的没有待办」必须可区分，不能静默清空成空列表。
    rows.value = []
    hasMore.value = false
    loadError.value = describeProcessError('流程加载失败', e)
  } finally {
    if (pageGate.isLatest(token)) loading.value = false
  }
}

async function loadMore() {
  if (loading.value || loadingMore.value || !hasMore.value) return
  const token = pageGate.begin()
  const targetTab = active.value
  loadingMore.value = true
  loadError.value = ''
  try {
    const result = await loadProcessPage<Record<string, unknown>>(fetchPage, targetTab, page.value + 1)
    // 加载期间切了页签：这一页属于旧页签，丢弃，避免把旧数据并进新列表。
    if (!pageGate.isLatest(token) || active.value !== targetTab) return
    rows.value = mergeProcessRows<Record<string, unknown>>(rows.value, result.rows)
    page.value = result.page
    hasMore.value = result.hasMore
  } catch (e) {
    if (!pageGate.isLatest(token) || active.value !== targetTab) return
    // 加载更多失败保留已加载的行，只提示错误。
    loadError.value = describeProcessError('更多流程加载失败', e)
  } finally {
    loadingMore.value = false
  }
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  // 「发起流程」提交后 switchTab 回到本页，经存储指定打开"我发起的"。
  const stored = uni.getStorageSync('oa_process_active_tab')
  if (stored && tabs.some((t) => t.key === stored)) {
    active.value = stored as TabKey
    uni.removeStorageSync('oa_process_active_tab')
  }
  loadFirst()
})

function goStart() {
  uni.navigateTo({ url: '/pages/process/start' })
}

function switchTab(key: TabKey) {
  if (active.value === key) return
  active.value = key
  loadFirst()
}

/** 审批动作：先收集处理意见（可留空），再走 /api/task/{id}/complete|reject。 */
function act(row: Record<string, unknown>, action: 'approve' | 'reject') {
  const taskId = typeof row.id === 'string' ? row.id : ''
  if (!taskId || busy.value) return
  // 从弹窗阶段就占住守卫：否则确认框打开期间再点一次会叠出第二个弹窗。
  busy.value = true
  uni.showModal({
    title: action === 'approve' ? '审批通过' : '驳回',
    content: '填写处理意见（可选）',
    placeholder: '处理意见',
    showInput: true,
    success: (r) => {
      if (!r.confirm) {
        busy.value = false
        return
      }
      void runAction(taskId, action, r.content?.trim() || '')
    },
    fail: () => {
      busy.value = false
    },
  })
}

async function runAction(taskId: string, action: 'approve' | 'reject', opinion: string) {
  try {
    if (action === 'approve') await processApi.completeTask(taskId, { opinion })
    else await processApi.rejectTask(taskId, { opinion })
    uni.showToast({ title: action === 'approve' ? '已通过' : '已驳回', icon: 'success' })
    loadFirst()
  } catch (e) {
    uni.showToast({ title: e instanceof Error ? e.message : '操作失败', icon: 'none' })
  } finally {
    busy.value = false
  }
}
</script>

<template>
  <view class="page">
    <view class="tabs">
      <view v-for="t in tabs" :key="t.key" class="tab" :class="{ on: active === t.key }" @tap="switchTab(t.key)">
        {{ t.label }}
      </view>
    </view>

    <view v-if="loading && rows.length === 0" class="tip">加载中…</view>
    <view v-else-if="rows.length === 0 && loadError" class="tip tip-error">
      <view class="error-text">{{ loadError }}</view>
      <view class="retry-btn" @tap="loadFirst">重试</view>
    </view>
    <view v-else-if="rows.length === 0" class="tip">暂无{{ tabs.find((t) => t.key === active)?.label }}</view>
    <view v-else class="list">
      <view v-for="(row, i) in rows" :key="i" class="item" @tap="openDetail(row, i)">
        <view class="title">{{ titleOf(row, i) }}</view>
        <view class="meta">{{ metaOf(row) || ' ' }}</view>
        <view v-if="active === 'pending'" class="actions">
          <button size="mini" type="default" :disabled="busy" @tap.stop="act(row, 'reject')">
            驳回
          </button>
          <button size="mini" type="primary" :disabled="busy" @tap.stop="act(row, 'approve')">
            通过
          </button>
        </view>
      </view>
    </view>
    <view v-if="rows.length > 0" class="footer">
      <view v-if="loadError" class="tip-error footer-error">
        <text class="error-text">{{ loadError }}</text>
        <view class="retry-btn" @tap="loadMore">重试</view>
      </view>
      <view v-else-if="loadingMore">加载更多…</view>
      <view v-else-if="hasMore" class="load-more" @tap="loadMore">加载更多</view>
      <view v-else>已加载 {{ rows.length }} 条</view>
    </view>

    <view v-if="detailOpen" class="mask" @tap="closeDetail">
      <view class="sheet" @tap.stop>
        <view class="sheet-head">
          <text class="sheet-title">{{ detailTitle }}</text>
          <text class="sheet-close" @tap="closeDetail">✕</text>
        </view>
        <view v-if="detailLoading" class="tip">加载中…</view>
        <scroll-view v-else scroll-y class="sheet-body">
          <view v-if="detailError" class="tip-error error-banner">
            <text class="error-text">{{ detailError }}</text>
            <view class="retry-btn" @tap="retryDetail">重试</view>
          </view>
          <view class="sec-title">附件 ({{ detailAttachments.length }})</view>
          <view v-if="detailAttachments.length === 0" class="sec-empty">无附件</view>
          <view v-for="(a, ai) in detailAttachments" :key="`a${ai}`" class="sec-row">
            {{ textOf(a, ['name', 'id'], '附件') }}
          </view>
          <view class="sec-title">流转记录 ({{ detailRecords.length }})</view>
          <view v-if="detailRecords.length === 0" class="sec-empty">无流转记录</view>
          <view v-for="(r, ri) in detailRecords" :key="`r${ri}`" class="sec-row">
            {{ textOf(r, ['title', 'id'], '记录') }}
            <text class="sec-time">{{ textOf(r, ['createTime']) }}</text>
          </view>
          <view class="sec-title">工作日志 ({{ detailWorklogs.length }})</view>
          <view v-if="detailWorklogs.length === 0" class="sec-empty">无工作日志</view>
          <view v-for="(w, wi) in detailWorklogs" :key="`w${wi}`" class="sec-row">
            {{ textOf(w, ['activityName', 'title', 'id'], '日志') }}
            <text class="sec-time">{{ textOf(w, ['createTime']) }}</text>
          </view>
        </scroll-view>
      </view>
    </view>
    <view class="fab" @tap="goStart">
      <text class="fab-plus">＋</text>
      <text class="fab-label">发起流程</text>
    </view>
  </view>
</template>

<style scoped>
.page {
  min-height: 100vh;
  background: #f5f7fa;
}
.tabs {
  display: flex;
  background: #fff;
}
.tab {
  flex: 1;
  text-align: center;
  padding: 24rpx 0;
  font-size: 30rpx;
  color: #666;
}
.tab.on {
  color: #2d8cf0;
  font-weight: 700;
  border-bottom: 4rpx solid #2d8cf0;
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
.footer-error {
  color: #c0392b;
}
.error-banner {
  padding: 20rpx 24rpx;
  background: #fdf2f0;
  border-radius: 12rpx;
  margin-top: 16rpx;
}
.load-more {
  color: #2d8cf0;
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
  font-weight: 600;
  color: #263238;
}
.meta {
  margin-top: 8rpx;
  font-size: 24rpx;
  color: #90979f;
}
.actions {
  margin-top: 16rpx;
  display: flex;
  gap: 16rpx;
  justify-content: flex-end;
}
.footer {
  padding: 24rpx;
  text-align: center;
  color: #90979f;
  font-size: 24rpx;
}
.fab {
  position: fixed;
  right: 32rpx;
  bottom: 64rpx;
  display: flex;
  align-items: center;
  gap: 8rpx;
  background: #2d8cf0;
  color: #fff;
  padding: 18rpx 28rpx;
  border-radius: 44rpx;
  box-shadow: 0 8rpx 24rpx rgba(45, 140, 240, 0.4);
}
.fab-plus {
  font-size: 36rpx;
  line-height: 1;
}
.fab-label {
  font-size: 26rpx;
}
.mask {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.45);
  z-index: 100;
  display: flex;
  align-items: flex-end;
}
.sheet {
  width: 100%;
  max-height: 76vh;
  background: #fff;
  border-radius: 24rpx 24rpx 0 0;
  display: flex;
  flex-direction: column;
}
.sheet-head {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 28rpx 32rpx;
  border-bottom: 1px solid #f0f2f5;
}
.sheet-title {
  font-size: 30rpx;
  font-weight: 700;
  color: #263238;
}
.sheet-close {
  font-size: 32rpx;
  color: #90979f;
}
.sheet-body {
  padding: 16rpx 32rpx 40rpx;
}
.sec-title {
  margin-top: 20rpx;
  font-size: 26rpx;
  font-weight: 700;
  color: #2d8cf0;
}
.sec-empty {
  padding: 12rpx 0;
  font-size: 24rpx;
  color: #90979f;
}
.sec-row {
  display: flex;
  justify-content: space-between;
  gap: 16rpx;
  padding: 14rpx 0;
  font-size: 26rpx;
  color: #4a5560;
  border-bottom: 1px solid #f7f8fa;
}
.sec-time {
  color: #90979f;
  font-size: 22rpx;
}
</style>
