<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { messageApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'

const rows = ref<Record<string, unknown>[]>([])
const loading = ref(false)
const loaded = ref(false)

/** 从未知形状的会话对象中稳妥取出展示字段（后端契约可能演进）。 */
function pick(obj: Record<string, unknown>, keys: string[]): string {
  for (const k of keys) {
    const v = obj[k]
    if (typeof v === 'string' && v) return v
  }
  return ''
}
function conversationId(obj: Record<string, unknown>): string {
  const v = pick(obj, ['conversationId', 'id', 'uuid', 'flag'])
  return v
}
function conversationTitle(obj: Record<string, unknown>): string {
  const v = pick(obj, ['title', 'name', 'subject', 'peerName', 'contact'])
  return v || '会话'
}

function payload(resp: { data: unknown }): Record<string, unknown>[] {
  const d = resp.data
  if (Array.isArray(d)) return d as Record<string, unknown>[]
  if (d && typeof d === 'object' && Array.isArray((d as { data?: unknown }).data)) {
    return (d as { data: Record<string, unknown>[] }).data
  }
  return []
}

async function load() {
  loading.value = true
  try {
    const resp = await messageApi.conversationList(1, 50)
    rows.value = payload(resp)
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

async function openConversation(row: Record<string, unknown>) {
  const id = conversationId(row)
  if (!id) return
  try {
    await messageApi.markRead(id)
  } catch {
    /* 标记失败不影响列表交互 */
  }
}
</script>

<template>
  <view class="page">
    <view v-if="loading && !loaded" class="tip">加载中…</view>
    <view v-else-if="rows.length === 0" class="tip">暂无会话</view>
    <view v-else class="list">
      <view v-for="(row, i) in rows" :key="i" class="item" @tap="openConversation(row)">
        <view class="title">{{ conversationTitle(row) }}</view>
        <view class="meta">{{ pick(row, ['lastMessage', 'preview', 'content']) || ' ' }}</view>
      </view>
    </view>
  </view>
</template>

<style scoped>
.page {
  min-height: 100vh;
  background: #f5f7fa;
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
  color: #263238;
  font-weight: 600;
}
.meta {
  margin-top: 8rpx;
  font-size: 24rpx;
  color: #90979f;
}
</style>
