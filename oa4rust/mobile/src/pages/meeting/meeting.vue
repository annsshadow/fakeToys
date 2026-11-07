<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
/**
 * 会议（阶段 G / F2）——移动场景刚需：我的会议 + 接受/拒绝/确认/签到。
 * 端点全部对齐 meeting_assemble_control 已注册路由（见 services/index.ts meetingApi）。
 */
import { onShow } from '@dcloudio/uni-app'
import { computed, ref } from 'vue'
import { type MeetingRow, meetingApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'
import { listState, loadList } from '@/utils/loadList'
import { MEETING_PAGE_SIZE, meetingHasMore, mergeMeetingRows } from './meetingList'

type TabKey = 'wait' | 'confirm' | 'mine'

const TABS: Array<{ key: TabKey; label: string }> = [
  { key: 'wait', label: '待接受' },
  { key: 'confirm', label: '待确认' },
  { key: 'mine', label: '我的会议' },
]

const tab = ref<TabKey>('wait')
const rows = ref<MeetingRow[]>([])
const loading = ref(false)
const loadingMore = ref(false)
const loadError = ref('')
const page = ref(1)
const hasMore = ref(false)
const busyId = ref('')
const listView = computed(() =>
  listState({ loading: loading.value, loaded: true, count: rows.value.length, error: loadError.value }),
)

async function load(previousRows: MeetingRow[] = rows.value): Promise<void> {
  loading.value = true
  loadError.value = ''
  const result = await loadList<MeetingRow>(
    () => {
      if (tab.value === 'wait') return meetingApi.invitedWait()
      if (tab.value === 'confirm') return meetingApi.waitConfirm()
      return meetingApi.applied(1, MEETING_PAGE_SIZE)
    },
    { previousRows, label: '会议加载失败' },
  )
  rows.value = result.rows
  page.value = 1
  // 只有「我的会议」端点支持分页；待接受/待确认返回全量，不提供加载更多。
  hasMore.value = tab.value === 'mine' && result.ok && meetingHasMore(result.rows.length)
  if (!result.ok) loadError.value = result.error
  loading.value = false
}

async function loadMore(): Promise<void> {
  if (loading.value || loadingMore.value || !hasMore.value || tab.value !== 'mine') return
  loadingMore.value = true
  loadError.value = ''
  const next = page.value + 1
  const result = await loadList<MeetingRow>(() => meetingApi.applied(next, MEETING_PAGE_SIZE), {
    previousRows: rows.value,
    label: '更多会议加载失败',
  })
  if (result.ok) {
    rows.value = mergeMeetingRows(rows.value, result.rows)
    page.value = next
    hasMore.value = meetingHasMore(result.rows.length)
  } else {
    // 加载更多失败保留已加载的行，只提示错误。
    loadError.value = result.error
  }
  loadingMore.value = false
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  void load()
})

function switchTab(key: TabKey): void {
  if (tab.value === key) return
  tab.value = key
  // 页签是用户主动换数据源，失败时显示该页签自己的错误，不沿用旧页签的行。
  void load([])
}

async function act(id: string, fn: () => Promise<unknown>, okText: string): Promise<void> {
  if (busyId.value) return
  busyId.value = id
  try {
    await fn()
    uni.showToast({ title: okText, icon: 'success' })
    await load()
  } catch (e) {
    uni.showToast({ title: e instanceof Error ? e.message : '操作失败', icon: 'none' })
  } finally {
    busyId.value = ''
  }
}

const accept = (m: MeetingRow) => act(m.id, () => meetingApi.accept(m.id), '已接受')
const reject = (m: MeetingRow) => act(m.id, () => meetingApi.reject(m.id), '已拒绝')
const allow = (m: MeetingRow) => act(m.id, () => meetingApi.confirmAllow(m.id), '已确认出席')
const deny = (m: MeetingRow) => act(m.id, () => meetingApi.confirmDeny(m.id), '已确认缺席')
const checkin = (m: MeetingRow) => act(m.id, () => meetingApi.checkin(m.id), '签到成功')

// 签到码（checkin/code）：组织者在会上出示给参会人扫码
const codeText = ref('')
async function showCode(m: MeetingRow): Promise<void> {
  try {
    const r = await meetingApi.checkinCode(m.id)
    codeText.value = String((r as { data?: { code?: string } }).data?.code ?? '') || '暂无签到码'
  } catch (e) {
    codeText.value = e instanceof Error ? e.message : '获取签到码失败'
  }
}

function titleOf(m: MeetingRow): string {
  return String(m.title ?? m.name ?? '未命名会议')
}
function fmtRange(m: MeetingRow): string {
  const s = m.startTime ? String(m.startTime) : ''
  const e = m.endTime ? String(m.endTime) : ''
  if (!s && !e) return '时间待定'
  return e ? `${s} ~ ${e}` : s
}
function statusText(m: MeetingRow): string {
  const s = String(m.status ?? '')
  if (s === '1') return '进行中'
  if (s === '2') return '已结束'
  if (s === '3') return '已取消'
  return '未开始'
}
const emptyText = computed(() =>
  tab.value === 'wait' ? '暂无待接受的会议邀请' : tab.value === 'confirm' ? '暂无待确认的会议' : '暂无会议',
)
</script>

<template>
  <view class="page">
    <view class="tabs">
      <view
        v-for="t in TABS"
        :key="t.key"
        class="tab"
        :class="{ on: tab === t.key }"
        @tap="switchTab(t.key)"
      >
        {{ t.label }}
      </view>
    </view>

    <view v-if="listView === 'loading'" class="tip">加载中…</view>
    <view v-else-if="listView === 'error'" class="tip tip-error">
      <view class="error-text">{{ loadError }}</view>
      <view class="retry-btn" @tap="load()">重试</view>
    </view>
    <view v-else-if="listView === 'empty'" class="tip">{{ emptyText }}</view>

    <view v-for="m in rows" :key="m.id" class="card">
      <view class="card-head">
        <text class="card-title">{{ titleOf(m) }}</text>
        <text class="card-status">{{ statusText(m) }}</text>
      </view>
      <view class="card-meta">{{ fmtRange(m) }}</view>

      <view v-if="tab === 'wait'" class="actions">
        <button class="btn" :disabled="!!busyId" @tap="accept(m)">接受</button>
        <button class="btn ghost" :disabled="!!busyId" @tap="reject(m)">拒绝</button>
      </view>
      <view v-else-if="tab === 'confirm'" class="actions">
        <button class="btn" :disabled="!!busyId" @tap="allow(m)">确认出席</button>
        <button class="btn ghost" :disabled="!!busyId" @tap="deny(m)">确认缺席</button>
      </view>
      <view v-else class="actions">
        <button class="btn" :disabled="!!busyId" @tap="checkin(m)">签到</button>
        <button class="btn ghost" :disabled="!!busyId" @tap="showCode(m)">签到码</button>
      </view>
    </view>

    <view v-if="tab === 'mine' && rows.length > 0" class="footer">
      <view v-if="loadError" class="tip-error footer-error">
        <text class="error-text">{{ loadError }}</text>
        <view class="retry-btn" @tap="loadMore">重试</view>
      </view>
      <view v-else-if="loadingMore">加载更多…</view>
      <view v-else-if="hasMore" class="load-more" @tap="loadMore">加载更多</view>
      <view v-else>已加载 {{ rows.length }} 条</view>
    </view>

    <view v-if="codeText" class="code-mask" @tap="codeText = ''">
      <view class="code-dialog" @tap.stop>
        <view class="code-title">签到码</view>
        <text class="code-body">{{ codeText }}</text>
        <button class="btn" @tap="codeText = ''">关闭</button>
      </view>
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
  color: #5b6572;
  background: #fff;
  border-radius: 12rpx;
}
.tab.on {
  background: #2d8cf0;
  color: #fff;
  font-weight: 600;
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
}
.card-title {
  font-size: 30rpx;
  font-weight: 700;
  color: #263238;
}
.card-status {
  font-size: 24rpx;
  color: #90979f;
}
.card-meta {
  margin-top: 10rpx;
  font-size: 24rpx;
  color: #90979f;
}
.actions {
  display: flex;
  gap: 16rpx;
  margin-top: 20rpx;
}
.btn {
  flex: 1;
  font-size: 28rpx;
  background: #2d8cf0;
  color: #fff;
  border-radius: 12rpx;
}
.btn.ghost {
  background: #fff;
  color: #5b6572;
  border: 1px solid #dcdfe6;
}
.btn[disabled] {
  background: #c0c4cc;
}
.tip {
  color: #90979f;
  font-size: 26rpx;
  text-align: center;
  padding: 80rpx 0;
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
.footer {
  padding: 8rpx 0 40rpx;
  text-align: center;
  font-size: 24rpx;
  color: #90979f;
}
.footer-error {
  color: #c0392b;
}
.load-more {
  display: inline-block;
  padding: 10rpx 36rpx;
  color: #2d8cf0;
  font-size: 26rpx;
}
.error-banner {
  padding: 24rpx 32rpx;
  text-align: center;
  background: #fdf2f0;
  border-bottom: 1px solid #f0f2f5;
  margin-bottom: 20rpx;
}
.code-mask {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.45);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 99;
}
.code-dialog {
  width: 560rpx;
  background: #fff;
  border-radius: 16rpx;
  padding: 32rpx;
  text-align: center;
}
.code-title {
  font-size: 30rpx;
  font-weight: 700;
  color: #263238;
  margin-bottom: 16rpx;
}
.code-body {
  display: block;
  font-size: 34rpx;
  color: #2d8cf0;
  letter-spacing: 4rpx;
  word-break: break-all;
  margin-bottom: 24rpx;
}
</style>
