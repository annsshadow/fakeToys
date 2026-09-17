<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { processApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'

type TabKey = 'pending' | 'done' | 'started'
const tabs: { key: TabKey; label: string }[] = [
  { key: 'pending', label: '待办' },
  { key: 'done', label: '已办' },
  { key: 'started', label: '我发起' },
]
const active = ref<TabKey>('pending')
const rows = ref<Record<string, unknown>[]>([])
const total = ref(0)
const loading = ref(false)
const busy = ref(false)

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

async function load() {
  loading.value = true
  rows.value = []
  total.value = 0
  try {
    const resp =
      active.value === 'pending'
        ? await processApi.pendingList(1, 50)
        : active.value === 'done'
          ? await processApi.completedList(1, 50)
          : await processApi.startedList(1, 50)
    rows.value = resp.data ?? []
    total.value = resp.count ?? rows.value.length
  } catch {
    rows.value = []
  } finally {
    loading.value = false
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
  load()
})

function goStart() {
  uni.navigateTo({ url: '/pages/process/start' })
}

function switchTab(key: TabKey) {
  if (active.value === key) return
  active.value = key
  load()
}

/** 审批动作：先收集处理意见（可留空），再走 /api/task/{id}/complete|reject。 */
function act(row: Record<string, unknown>, action: 'approve' | 'reject') {
  const taskId = typeof row.id === 'string' ? row.id : ''
  if (!taskId || busy.value) return
  uni.showModal({
    title: action === 'approve' ? '审批通过' : '驳回',
    content: '填写处理意见（可选）',
    placeholder: '处理意见',
    showInput: true,
    success: (r) => {
      if (!r.confirm) return
      void runAction(taskId, action, r.content?.trim() || '')
    },
  })
}

async function runAction(taskId: string, action: 'approve' | 'reject', opinion: string) {
  busy.value = true
  try {
    if (action === 'approve') await processApi.completeTask(taskId, { opinion })
    else await processApi.rejectTask(taskId, { opinion })
    uni.showToast({ title: action === 'approve' ? '已通过' : '已驳回', icon: 'success' })
    load()
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
    <view v-else-if="rows.length === 0" class="tip">暂无{{ tabs.find((t) => t.key === active)?.label }}</view>
    <view v-else class="list">
      <view v-for="(row, i) in rows" :key="i" class="item">
        <view class="title">{{ titleOf(row, i) }}</view>
        <view class="meta">{{ metaOf(row) || ' ' }}</view>
        <view v-if="active === 'pending'" class="actions">
          <button size="mini" type="default" :disabled="busy" @tap="act(row, 'reject')">
            驳回
          </button>
          <button size="mini" type="primary" :disabled="busy" @tap="act(row, 'approve')">
            通过
          </button>
        </view>
      </view>
    </view>
    <view v-if="total > 0" class="footer">共 {{ total }} 条</view>
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
</style>
