<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { annApi, attendanceApi, processApi } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'
import { loadList } from '@/utils/loadList'
import { cachedRequest } from '@/utils/requestCache'
import { PENDING_PROBE_SIZE, pendingBadge } from './pendingCount'

type Notice = { id?: string; title?: string; content?: string; createTime?: string }

const session = useSession()
const pendingBadgeText = ref('–')
const pendingOverflow = ref(false)
const todayRecords = ref<number | null>(null)
const loading = ref(false)
const notices = ref<Notice[]>([])
const noticesError = ref('')
const viewNotice = ref<{ title?: string; content?: string; createTime?: string } | null>(null)

function displayName(): string {
  return session.user?.name || session.user?.unique || '用户'
}

async function loadPending() {
  loading.value = true
  try {
    // 多取一条做哨兵：后端 count 是本页条数，拿满 51 条才说明还有更多。
    const resp = await processApi.pendingList(1, PENDING_PROBE_SIZE)
    const total = resp.count ?? resp.data?.length ?? 0
    const badge = pendingBadge(total)
    pendingBadgeText.value = badge.text
    pendingOverflow.value = badge.overflow
  } catch {
    pendingBadgeText.value = '–'
    pendingOverflow.value = false
  } finally {
    loading.value = false
  }
}

/** 公告：后端 create_time 倒序，取前 5 条展示；点击看全文。 */
async function loadNotices(force = false) {
  // R137：重试前必须清空上一次的错误，否则成功之后横幅仍挂着旧错误（列表页唯一漏点）。
  noticesError.value = ''
  // 公告为只读高频端点：onShow 复用 TTL 缓存；重试按钮 force 绕过。
  const result = await loadList<Notice>(() => cachedRequest('announcement:list', () => annApi.list(), { ttlMs: 30_000, force }), {
    previousRows: notices.value,
    label: '公告加载失败',
    map: (data) => (Array.isArray(data) ? (data as Notice[]).slice(0, 5) : []),
  })
  notices.value = result.rows
  if (!result.ok) noticesError.value = result.error
}

/** 今日出勤概览：取按日聚合统计的最新一条记录数。 */
async function loadTodayAttendance() {
  try {
    const resp = await attendanceApi.statistics()
    todayRecords.value = resp.data?.[0]?.records ?? 0
  } catch {
    todayRecords.value = null
  }
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  loadPending()
  loadNotices()
  loadTodayAttendance()
})

function goTab(url: string) {
  uni.switchTab({ url })
}
function goPage(url: string) {
  uni.navigateTo({ url })
}
</script>

<template>
  <view class="page ai-surface-gradient">
    <view class="hero">
      <view class="hello">你好，{{ displayName() }}</view>
      <view class="sub">OA4Rust 工作台</view>
    </view>

    <view class="stat-card ai-brand-fill" @tap="goTab('/pages/process/process')">
      <view class="stat-num">
        {{ loading ? '…' : pendingBadgeText }}
      </view>
      <view class="stat-label">待办审批{{ pendingOverflow ? ' · 超过 50' : '' }}</view>
    </view>

    <view class="stat-card ai-brand-fill" @tap="goTab('/pages/attendance/attendance')">
      <view class="stat-num">{{ todayRecords === null ? '–' : todayRecords }}</view>
      <view class="stat-label">今日出勤</view>
    </view>

    <view class="section-card">
      <view class="sec-title">📢 公告</view>
      <view v-if="notices.length === 0 && noticesError" class="sec-error">
        <text class="error-text">{{ noticesError }}</text>
        <view class="retry-btn" @tap="loadNotices(true)">重试</view>
      </view>
      <view v-else-if="notices.length === 0" class="sec-empty">暂无公告</view>
      <view v-if="noticesError && notices.length > 0" class="sec-error">
        <text class="error-text">{{ noticesError }}</text>
        <view class="retry-btn" @tap="loadNotices(true)">重试</view>
      </view>
      <view v-for="n in notices" :key="n.id" class="notice o2-list-item" @tap="viewNotice = n">
        <view class="notice-title">{{ n.title || '无标题' }}</view>
        <view class="notice-meta">{{ (n.createTime || '').slice(0, 10) }}</view>
      </view>
    </view>

    <view class="grid">
      <view class="cell" @tap="goTab('/pages/message/message')">
        <text class="cell-emoji">💬</text>
        <text class="cell-text">消息</text>
      </view>
      <view class="cell" @tap="goTab('/pages/process/process')">
        <text class="cell-emoji">✅</text>
        <text class="cell-text">审批</text>
      </view>
      <view class="cell" @tap="goPage('/pages/doc/doc')">
        <text class="cell-emoji">📄</text>
        <text class="cell-text">文档</text>
      </view>
      <view class="cell" @tap="goPage('/pages/contacts/contacts')">
        <text class="cell-emoji">👥</text>
        <text class="cell-text">通讯录</text>
      </view>
      <view class="cell" @tap="goTab('/pages/attendance/attendance')">
        <text class="cell-emoji">🕘</text>
        <text class="cell-text">考勤</text>
      </view>
      <view class="cell" @tap="goPage('/pages/process/start')">
        <text class="cell-emoji">🚀</text>
        <text class="cell-text">发起流程</text>
      </view>
      <!-- 阶段 G：移动端能力扩展新增入口 -->
      <view class="cell" @tap="goPage('/pages/meeting/meeting')">
        <text class="cell-emoji">📅</text>
        <text class="cell-text">会议</text>
      </view>
      <view class="cell" @tap="goPage('/pages/calendar/calendar')">
        <text class="cell-emoji">🗓️</text>
        <text class="cell-text">日程</text>
      </view>
      <view class="cell" @tap="goPage('/pages/bbs/bbs')">
        <text class="cell-emoji">🗣</text>
        <text class="cell-text">论坛</text>
      </view>
      <view class="cell" @tap="goPage('/pages/browse/browse')">
        <text class="cell-emoji">📚</text>
        <text class="cell-text">浏览</text>
      </view>
      <view class="cell" @tap="goPage('/pages/search/search')">
        <text class="cell-emoji">🔍</text>
        <text class="cell-text">搜索</text>
      </view>
      <view class="cell" @tap="goPage('/pages/recycle/recycle')">
        <text class="cell-emoji">🗑️</text>
        <text class="cell-text">回收站</text>
      </view>
    </view>

    <!-- 公告全文 -->
    <view v-if="viewNotice" class="notice-mask" @tap="viewNotice = null">
      <view class="notice-dialog" @tap.stop>
        <view class="nd-title">{{ viewNotice.title || '无标题' }}</view>
        <view v-if="viewNotice.createTime" class="nd-meta">{{ viewNotice.createTime }}</view>
        <view class="nd-body">{{ viewNotice.content || '（无正文）' }}</view>
        <button class="nd-close" size="mini" @tap="viewNotice = null">关闭</button>
      </view>
    </view>
  </view>
</template>

<style scoped>
.page {
  padding: 32rpx;
}
.hero {
  margin-bottom: 40rpx;
}
.hello {
  font-size: 40rpx;
  font-weight: 700;
  color: var(--color-text-primary);
}
.sub {
  margin-top: 8rpx;
  font-size: 26rpx;
  color: var(--color-text-secondary);
}
.stat-card {
  background: var(--color-primary);
  color: var(--color-surface);
  border-radius: 20rpx;
  padding: 40rpx;
  margin-bottom: 40rpx;
  text-align: center;
}
.stat-num {
  font-size: 64rpx;
  font-weight: 700;
}
.stat-label {
  margin-top: 8rpx;
  font-size: 26rpx;
  opacity: 0.9;
}
.grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 24rpx;
}
.cell {
  background: var(--color-surface);
  border-radius: 16rpx;
  padding: 32rpx 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12rpx;
}
.cell-emoji {
  font-size: 48rpx;
}
.cell-text {
  font-size: 24rpx;
  color: var(--color-text-secondary);
}
.section-card {
  background: var(--color-surface);
  border-radius: 16rpx;
  padding: 24rpx;
  margin-bottom: 32rpx;
}
.sec-title {
  font-size: 30rpx;
  font-weight: 700;
  color: var(--color-text-primary);
  margin-bottom: 12rpx;
}
.sec-empty {
  color: var(--color-text-secondary);
  font-size: 24rpx;
  padding: 12rpx 0;
}
.sec-error {
  color: var(--color-error);
  font-size: 24rpx;
  padding: 12rpx 0;
}
.error-text {
  display: block;
  margin-bottom: 10rpx;
  line-height: 1.5;
}
.retry-btn {
  display: inline-block;
  padding: 6rpx 24rpx;
  border-radius: 22rpx;
  border: 1rpx solid var(--color-error);
  color: var(--color-error);
  font-size: 24rpx;
}
.notice {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18rpx 0;
  border-bottom: 1px solid var(--color-divider);
}
.notice:last-of-type {
  border-bottom: none;
}
.notice-title {
  flex: 1;
  font-size: 27rpx;
  color: var(--color-text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.notice-meta {
  flex-shrink: 0;
  margin-left: 16rpx;
  font-size: 22rpx;
  color: var(--color-text-secondary);
}
.notice-mask {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.55);
  z-index: 100;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 48rpx;
}
.notice-dialog {
  background: var(--color-surface);
  border-radius: 20rpx;
  padding: 32rpx;
  width: 100%;
  max-height: 80vh;
  overflow-y: auto;
  box-sizing: border-box;
}
.nd-title {
  font-size: 34rpx;
  font-weight: 700;
  color: var(--color-text-primary);
}
.nd-meta {
  margin-top: 8rpx;
  font-size: 22rpx;
  color: var(--color-text-secondary);
}
.nd-body {
  margin-top: 20rpx;
  font-size: 28rpx;
  color: var(--color-text-secondary);
  line-height: 1.7;
  white-space: pre-wrap;
  word-break: break-word;
}
.nd-close {
  margin-top: 28rpx;
  background: var(--color-primary);
  color: var(--color-surface);
  font-size: 26rpx;
}
</style>