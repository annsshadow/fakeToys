<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { annApi, attendanceApi, processApi } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'

const session = useSession()
const pendingCount = ref<number | null>(null)
const todayRecords = ref<number | null>(null)
const loading = ref(false)
const notices = ref<Array<{ id?: string; title?: string; content?: string; createTime?: string }>>([])
const viewNotice = ref<{ title?: string; content?: string; createTime?: string } | null>(null)

function displayName(): string {
  return session.user?.name || session.user?.unique || '用户'
}

async function loadPending() {
  loading.value = true
  try {
    const resp = await processApi.pendingList(1, 50)
    // 信封顶层 count 为本页条数（后端 legacy_success 语义）；取不到时退回长度。
    pendingCount.value = resp.count ?? resp.data?.length ?? 0
  } catch {
    pendingCount.value = null
  } finally {
    loading.value = false
  }
}

/** 公告：后端 create_time 倒序，取前 5 条展示；点击看全文。 */
async function loadNotices() {
  try {
    const resp = await annApi.list()
    notices.value = (resp.data ?? []).slice(0, 5)
  } catch {
    notices.value = []
  }
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
  <view class="page">
    <view class="hero">
      <view class="hello">你好，{{ displayName() }}</view>
      <view class="sub">OA4Rust 工作台</view>
    </view>

    <view class="stat-card" @tap="goTab('/pages/process/process')">
      <view class="stat-num">
        {{ pendingCount === null ? (loading ? '…' : '–') : pendingCount }}
      </view>
      <view class="stat-label">待办审批</view>
    </view>

    <view class="stat-card" @tap="goTab('/pages/attendance/attendance')">
      <view class="stat-num">{{ todayRecords === null ? '–' : todayRecords }}</view>
      <view class="stat-label">今日出勤</view>
    </view>

    <view class="section-card">
      <view class="sec-title">📢 公告</view>
      <view v-if="notices.length === 0" class="sec-empty">暂无公告</view>
      <view v-for="n in notices" :key="n.id" class="notice" @tap="viewNotice = n">
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
  color: #263238;
}
.sub {
  margin-top: 8rpx;
  font-size: 26rpx;
  color: #90979f;
}
.stat-card {
  background: #2d8cf0;
  color: #fff;
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
  background: #fff;
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
  color: #4a4a4a;
}
.section-card {
  background: #fff;
  border-radius: 16rpx;
  padding: 24rpx;
  margin-bottom: 32rpx;
}
.sec-title {
  font-size: 30rpx;
  font-weight: 700;
  color: #263238;
  margin-bottom: 12rpx;
}
.sec-empty {
  color: #90979f;
  font-size: 24rpx;
  padding: 12rpx 0;
}
.notice {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18rpx 0;
  border-bottom: 1px solid #f0f2f5;
}
.notice:last-of-type {
  border-bottom: none;
}
.notice-title {
  flex: 1;
  font-size: 27rpx;
  color: #263238;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.notice-meta {
  flex-shrink: 0;
  margin-left: 16rpx;
  font-size: 22rpx;
  color: #90979f;
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
  background: #fff;
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
  color: #263238;
}
.nd-meta {
  margin-top: 8rpx;
  font-size: 22rpx;
  color: #90979f;
}
.nd-body {
  margin-top: 20rpx;
  font-size: 28rpx;
  color: #4a4a4a;
  line-height: 1.7;
  white-space: pre-wrap;
  word-break: break-word;
}
.nd-close {
  margin-top: 28rpx;
  background: #2d8cf0;
  color: #fff;
  font-size: 26rpx;
}
</style>
