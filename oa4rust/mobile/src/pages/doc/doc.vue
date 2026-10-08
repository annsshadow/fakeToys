<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { fileApi } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'

const session = useSession()
const rows = ref<Record<string, unknown>[]>([])
const attachRows = ref<Record<string, unknown>[]>([])
const loading = ref(false)
const loaded = ref(false)
const uploading = ref(false)

function myFolder(): string {
  return session.user?.unique ?? ''
}

function nameOf(row: Record<string, unknown>): string {
  const v = row.name
  return typeof v === 'string' && v ? v : '文件'
}
function metaOf(row: Record<string, unknown>): string {
  const parts: string[] = []
  const size = row.size
  if (typeof size === 'number') parts.push(size > 1048576 ? `${(size / 1048576).toFixed(1)} MB` : `${size} B`)
  for (const k of ['createTime', 'creator']) {
    const v = row[k]
    if (typeof v === 'string' && v) parts.push(v)
  }
  return parts.join(' · ')
}
async function load() {
  // 未登录或会话未恢复时不请求，避免无意义 401。
  if (!session.user?.unique) return
  loading.value = true
  try {
    const resp = await fileApi.fileList(session.user.unique)
    rows.value = resp.data ?? []
  } catch {
    rows.value = []
  }
  // 附件存储 FILE_FILE：上传落点，单列展示（区别于 x_file「我的文件」）。
  try {
    const attachResp = await fileApi.attachmentList(session.user.unique)
    attachRows.value = attachResp.data ?? []
  } catch {
    attachRows.value = []
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
  const fileId = typeof row.id === 'string' ? row.id : ''
  if (!fileId) return
  const name = nameOf(row)
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

// 附件存储 FILE_FILE 下载（区别于 x_file 的 fileDownloadUrl）。
async function downloadAttachment(row: Record<string, unknown>) {
  const attId = typeof row.id === 'string' ? row.id : ''
  if (!attId) return
  const name = nameOf(row)
  uni.showLoading({ title: '下载中…' })
  const task = uni.downloadFile({ url: fileApi.attachmentDownloadUrl(attId) })
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

// ── 上传（二进制写入附件存储）────────────────────────────────
async function doUpload(filePath: string, fileName: string): Promise<void> {
  const folder = myFolder()
  if (!folder || !filePath) {
    uni.showToast({ title: '未登录或无文件', icon: 'none' })
    return
  }
  uploading.value = true
  uni.showLoading({ title: '上传中…' })
  try {
    await fileApi.upload(folder, filePath, fileName || 'file')
    uni.hideLoading()
    uni.showToast({ title: '上传成功', icon: 'success' })
  } catch (e) {
    uni.hideLoading()
    uni.showToast({ title: e instanceof Error ? e.message : '上传失败', icon: 'none' })
  } finally {
    uploading.value = false
  }
}

function pickFromAlbumOrCamera(): void {
  uni.chooseImage({
    count: 1,
    success: (res) => {
      const f = res.tempFiles?.[0]
      if (f) void doUpload(f.path, f.name || 'image')
    },
    fail: () => uni.showToast({ title: '未选择文件', icon: 'none' }),
  })
}

function pickFromFilePicker(): void {
  // 任意文件：小程序/App 提供 chooseMessageFile；H5 回退到图片选择。
  const picker = (uni as unknown as { chooseMessageFile?: (o: unknown) => void }).chooseMessageFile
  if (typeof picker === 'function') {
    picker({
      count: 1,
      type: 'file',
      success: (res: { tempFiles?: Array<{ path?: string; name?: string }> }) => {
        const f = res.tempFiles?.[0]
        if (f?.path) void doUpload(f.path, f.name || 'file')
      },
      fail: () => uni.showToast({ title: '未选择文件', icon: 'none' }),
    })
  } else {
    pickFromAlbumOrCamera()
  }
}
</script>

<template>
  <view class="page">
    <view class="toolbar">
      <button class="tool" size="mini" :disabled="uploading" @tap="pickFromAlbumOrCamera">
        拍照 / 相册
      </button>
      <button class="tool" size="mini" :disabled="uploading" @tap="pickFromFilePicker">
        上传文件
      </button>
    </view>
    <view v-if="loading && !loaded" class="tip">加载中…</view>
    <view v-else class="page-body">
      <view class="section">
        <view class="section-title">我的附件（附件存储 FILE_FILE，上传落点）</view>
        <view v-if="attachRows.length === 0" class="tip">暂无附件</view>
        <view v-else class="list">
          <view v-for="(row, i) in attachRows" :key="'att' + i" class="item" @tap="downloadAttachment(row)">
            <text class="icon">📎</text>
            <view class="body">
              <view class="title">{{ nameOf(row) }}</view>
              <view class="meta">{{ metaOf(row) || ' ' }}</view>
            </view>
          </view>
        </view>
      </view>
      <view class="section">
        <view class="section-title">我的文件（文档库 x_file）</view>
        <view v-if="rows.length === 0" class="tip">暂无文件</view>
        <view v-else class="list">
          <view v-for="(row, i) in rows" :key="i" class="item" @tap="downloadAndOpen(row)">
            <text class="icon">📄</text>
            <view class="body">
              <view class="title">{{ nameOf(row) }}</view>
              <view class="meta">{{ metaOf(row) || ' ' }}</view>
            </view>
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
.toolbar {
  display: flex;
  gap: 16rpx;
  padding: 24rpx;
  background: #fff;
}
.tool {
  flex: 1;
  font-size: 26rpx;
}
.tip {
  padding: 120rpx 0;
  text-align: center;
  color: #90979f;
  font-size: 28rpx;
}
.page-body {
  background: #fff;
}
.section {
  padding-top: 8rpx;
}
.section-title {
  font-size: 28rpx;
  font-weight: 700;
  color: #263238;
  padding: 20rpx 32rpx 8rpx;
  border-left: 6rpx solid #2d8cf0;
  margin-left: 24rpx;
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
