<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { fileApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'

const rows = ref<Record<string, unknown>[]>([])
const loading = ref(false)
const loaded = ref(false)

function pick(obj: Record<string, unknown>, keys: string[]): string {
  for (const k of keys) {
    const v = obj[k]
    if (typeof v === 'string' && v) return v
  }
  return ''
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
    const resp = await fileApi.fileList(undefined, 1, 50)
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

function openDocument(filePath: string, name?: string) {
  const ext = (name || filePath).split('.').pop()
  uni.openDocument({
    filePath,
    fileType: ext,
    showMenu: true,
    fail: () => uni.showToast({ title: '打开失败', icon: 'none' }),
  })
}

async function downloadAndOpen(row: Record<string, unknown>) {
  const fileId = pick(row, ['fileId', 'id', 'uuid', 'flag'])
  if (!fileId) return
  const name = pick(row, ['name', 'fileName', 'title'])
  uni.showLoading({ title: '下载中…' })
  const task = uni.downloadFile({ url: fileApi.fileDownloadUrl(fileId) })
  task
    .then((res) => {
      uni.hideLoading()
      if (res.statusCode === 200) openDocument(res.tempFilePath, name)
      else uni.showToast({ title: '下载失败', icon: 'none' })
    })
    .catch(() => {
      uni.hideLoading()
      uni.showToast({ title: '下载失败', icon: 'none' })
    })
}
</script>

<template>
  <view class="page">
    <view v-if="loading && !loaded" class="tip">加载中…</view>
    <view v-else-if="rows.length === 0" class="tip">暂无文件</view>
    <view v-else class="list">
      <view v-for="(row, i) in rows" :key="i" class="item" @tap="downloadAndOpen(row)">
        <text class="icon">📄</text>
        <view class="body">
          <view class="title">{{ pick(row, ['name', 'fileName', 'title']) || `文件 #${i + 1}` }}</view>
          <view class="meta">
            {{ pick(row, ['size', 'createTime', 'owner']) || '' }}
          </view>
        </view>
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
  display: flex;
  align-items: center;
  gap: 20rpx;
  padding: 28rpx 32rpx;
  border-bottom: 1px solid #f0f2f5;
}
.icon {
  font-size: 40rpx;
}
.body {
  flex: 1;
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
