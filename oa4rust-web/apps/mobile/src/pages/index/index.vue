<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { processApi } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'

const session = useSession()
const pendingCount = ref<number | null>(null)
const loading = ref(false)

function displayName(): string {
  return session.user?.name || session.user?.unique || '用户'
}

async function loadPending() {
  loading.value = true
  try {
    const resp = await processApi.pendingList(1, 50)
    // 信封顶层 count 为本页条数（后端 java_success 语义）；取不到时退回长度。
    pendingCount.value = resp.count ?? resp.data?.length ?? 0
  } catch {
    pendingCount.value = null
  } finally {
    loading.value = false
  }
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  loadPending()
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
</style>
