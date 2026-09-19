<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { messageApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'

const rows = ref<Record<string, unknown>[]>([])
const loading = ref(false)
const loaded = ref(false)

function conversationId(row: Record<string, unknown>): string {
  for (const k of ['id', 'conversationId', 'uuid', 'flag']) {
    const v = row[k]
    if (typeof v === 'string' && v) return v
  }
  return ''
}
function conversationTitle(row: Record<string, unknown>): string {
  for (const k of ['name', 'title', 'subject']) {
    const v = row[k]
    if (typeof v === 'string' && v) return v
  }
  return '会话'
}

async function load() {
  loading.value = true
  try {
    const resp = await messageApi.conversationList()
    rows.value = resp.data ?? []
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

function openConversation(row: Record<string, unknown>) {
  const id = conversationId(row)
  if (!id) return
  const name = conversationTitle(row)
  uni.navigateTo({ url: `/pages/chat/chat?id=${encodeURIComponent(id)}&name=${encodeURIComponent(name)}` })
}
</script>

<template>
  <view class="page">
    <view v-if="loading && !loaded" class="tip">加载中…</view>
    <view v-else-if="rows.length === 0" class="tip">暂无会话</view>
    <view v-else class="list">
      <view v-for="(row, i) in rows" :key="i" class="item" @tap="openConversation(row)">
        <view class="title">{{ conversationTitle(row) }}</view>
        <view class="meta">{{ (row.lastMessage as string) || ' ' }}</view>
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
