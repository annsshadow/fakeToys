<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { processApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'

type TabKey = 'pending' | 'done'
const active = ref<TabKey>('pending')
const rows = ref<Record<string, unknown>[]>([])
const total = ref(0)
const loading = ref(false)
const busy = ref(false)

function pick(obj: Record<string, unknown>, keys: string[]): string {
  for (const k of keys) {
    const v = obj[k]
    if (typeof v === 'string' && v) return v
  }
  return ''
}
function payload(resp: { data: unknown }): { items: Record<string, unknown>[]; total: number } {
  const d = resp.data
  let items: Record<string, unknown>[] = []
  let t = 0
  if (Array.isArray(d)) {
    items = d as Record<string, unknown>[]
  } else if (d && typeof d === 'object') {
    const o = d as { data?: unknown; total?: unknown }
    if (Array.isArray(o.data)) items = o.data as Record<string, unknown>[]
    if (typeof o.total === 'number') t = o.total
  }
  return { items, total: t || items.length }
}

async function load() {
  loading.value = true
  try {
    const resp = active.value === 'pending' ? await processApi.workList(1, 50) : await processApi.completedList(1, 50)
    const p = payload(resp)
    rows.value = p.items
    total.value = p.total
  } catch {
    rows.value = []
    total.value = 0
  } finally {
    loading.value = false
  }
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  load()
})

function switchTab(key: TabKey) {
  if (active.value === key) return
  active.value = key
  load()
}

async function handle(row: Record<string, unknown>, action: string) {
  const taskId = pick(row, ['taskId', 'id', 'uuid', 'flag'])
  if (!taskId || busy.value) return
  busy.value = true
  try {
    await processApi.taskHandle(taskId, action)
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
      <view class="tab" :class="{ on: active === 'pending' }" @tap="switchTab('pending')">
        待办
      </view>
      <view class="tab" :class="{ on: active === 'done' }" @tap="switchTab('done')">
        已办
      </view>
    </view>

    <view v-if="loading && rows.length === 0" class="tip">加载中…</view>
    <view v-else-if="rows.length === 0" class="tip">暂无{{ active === 'pending' ? '待办' : '已办' }}</view>
    <view v-else class="list">
      <view v-for="(row, i) in rows" :key="i" class="item">
        <view class="title">
          {{ pick(row, ['title', 'subject', 'name', 'processName']) || `流程 #${i + 1}` }}
        </view>
        <view class="meta">
          {{ pick(row, ['status', 'applicant', 'createTime']) || ' ' }}
        </view>
        <view v-if="active === 'pending'" class="actions">
          <button size="mini" type="default" :disabled="busy" @tap="handle(row, 'reject')">
            驳回
          </button>
          <button size="mini" type="primary" :disabled="busy" @tap="handle(row, 'approve')">
            通过
          </button>
        </view>
      </view>
    </view>
    <view v-if="total > 0" class="footer">共 {{ total }} 条</view>
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
</style>
