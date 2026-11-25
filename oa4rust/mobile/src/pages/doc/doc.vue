<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { computed, ref } from 'vue'
import { fileApi } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'
import { listState } from '@/utils/loadList'
import { docMetaSize, loadDocLists } from './docFeed'

const session = useSession()
const rows = ref<Record<string, unknown>[]>([])
const attachRows = ref<Record<string, unknown>[]>([])
const loading = ref(false)
const loaded = ref(false)
const filesError = ref('')
const attachError = ref('')
const uploading = ref(false)
const filesView = computed(() =>
  listState({ loading: loading.value, loaded: loaded.value, count: rows.value.length, error: filesError.value }),
)
const attachView = computed(() =>
  listState({ loading: loading.value, loaded: loaded.value, count: attachRows.value.length, error: attachError.value }),
)

function myFolder(): string {
  return session.user?.unique ?? ''
}

function nameOf(row: Record<string, unknown>): string {
  const v = row.name
  return typeof v === 'string' && v ? v : '文件'
}
function metaOf(row: Record<string, unknown>): string {
  const parts: string[] = []
  const sizeText = docMetaSize(row)
  if (sizeText) parts.push(sizeText)
  for (const k of ['createTime', 'creator']) {
    const v = row[k]
    if (typeof v === 'string' && v) parts.push(v)
  }
  return parts.join(' · ')
}
async function load() {
  // 未登录或会话未恢复时不请求，避免无意义 401。
  const unique = session.user?.unique
  if (!unique) return
  loading.value = true
  filesError.value = ''
  attachError.value = ''
  const { files, attachments } = await loadDocLists(
    { fileList: () => fileApi.fileList(unique), attachmentList: () => fileApi.attachmentList(unique) },
    { files: rows.value, attachments: attachRows.value },
  )
  rows.value = files.rows
  if (!files.ok) filesError.value = files.error
  attachRows.value = attachments.rows
  if (!attachments.ok) attachError.value = attachments.error
  loading.value = false
  loaded.value = true
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
  if (uploading.value) return
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
    // 上传落点即本页附件清单：刷新列表让新附件即时可见（失败时 loadList 保留旧行，不误报失败）。
    await load()
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
  <view class="page ai-surface-gradient">
    <view class="toolbar">
      <button class="tool" size="mini" :disabled="uploading" @tap="pickFromAlbumOrCamera">
        拍照 / 相册
      </button>
      <button class="tool" size="mini" :disabled="uploading" @tap="pickFromFilePicker">
        上传文件
      </button>
    </view>
    <view v-if="loading && !loaded" class="ai-skeleton-list" aria-hidden="true">
      <view v-for="i in 3" :key="i" class="ai-skeleton-line"></view>
      <view class="ai-skeleton-line short"></view>
    </view>
    <view v-else class="page-body">
      <view class="section">
        <view class="section-title">我的附件（附件存储 FILE_FILE，上传落点）</view>
        <view v-if="attachView === 'loading'" class="ai-skeleton-list" aria-hidden="true">
          <view v-for="i in 3" :key="i" class="ai-skeleton-line"></view>
          <view class="ai-skeleton-line short"></view>
        </view>
        <view v-else-if="attachView === 'error'" class="tip tip-error">
          <view class="error-text">{{ attachError }}</view>
          <view class="retry-btn" role="button" tabindex="0" aria-label="重试" @tap="load()" @keydown.enter="load()" @keydown.space.prevent="load()">重试</view>
        </view>
        <view v-else-if="attachView === 'empty'" class="empty-guide ai-brand-fill">
          <view class="empty-guide-emoji" aria-hidden="true">📎</view>
          <text class="empty-guide-text">还没有附件</text>
          <text class="empty-guide-hint">拍照或上传文件后，附件会显示在这里</text>
          <view class="empty-guide-actions">
            <button class="o2-btn" size="mini" @tap="pickFromAlbumOrCamera">拍照 / 相册</button>
            <button class="o2-btn" size="mini" @tap="pickFromFilePicker">上传文件</button>
            <view class="empty-guide-link ai-pressable" role="button" tabindex="0" aria-label="刷新" @tap="load()" @keydown.enter="load()" @keydown.space.prevent="load()">刷新</view>
          </view>
        </view>
        <view v-if="attachError && attachRows.length > 0" class="tip-error error-banner">
          <text class="error-text">{{ attachError }}</text>
          <view class="retry-btn" role="button" tabindex="0" aria-label="重试" @tap="load()" @keydown.enter="load()" @keydown.space.prevent="load()">重试</view>
        </view>
        <view v-if="attachRows.length > 0" class="list">
          <view v-for="(row, i) in attachRows" :key="'att' + i" class="item" role="button" tabindex="0" :aria-label="'查看' + nameOf(row) + '附件'" @tap="downloadAttachment(row)" @keydown.enter="downloadAttachment(row)" @keydown.space.prevent="downloadAttachment(row)">
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
        <view v-if="filesView === 'loading'" class="ai-skeleton-list" aria-hidden="true">
          <view v-for="i in 3" :key="i" class="ai-skeleton-line"></view>
          <view class="ai-skeleton-line short"></view>
        </view>
        <view v-else-if="filesView === 'error'" class="tip tip-error">
          <view class="error-text">{{ filesError }}</view>
          <view class="retry-btn" role="button" tabindex="0" aria-label="重试" @tap="load()" @keydown.enter="load()" @keydown.space.prevent="load()">重试</view>
        </view>
        <view v-else-if="filesView === 'empty'" class="empty-guide">
          <view class="empty-guide-emoji" aria-hidden="true">📄</view>
          <text class="empty-guide-text">文档库暂无文件</text>
          <text class="empty-guide-hint">上传一份文件，或刷新查看最新文档</text>
          <view class="empty-guide-actions">
            <button class="o2-btn" size="mini" @tap="pickFromFilePicker">上传文件</button>
            <view class="empty-guide-link ai-pressable" role="button" tabindex="0" aria-label="刷新" @tap="load()" @keydown.enter="load()" @keydown.space.prevent="load()">刷新</view>
          </view>
        </view>
        <view v-if="filesError && rows.length > 0" class="tip-error error-banner">
          <text class="error-text">{{ filesError }}</text>
          <view class="retry-btn" role="button" tabindex="0" aria-label="重试" @tap="load()" @keydown.enter="load()" @keydown.space.prevent="load()">重试</view>
        </view>
        <view v-if="rows.length > 0" class="list">
          <view v-for="(row, i) in rows" :key="i" class="item" role="button" tabindex="0" :aria-label="'查看' + nameOf(row) + '文档'" @tap="downloadAndOpen(row)" @keydown.enter="downloadAndOpen(row)" @keydown.space.prevent="downloadAndOpen(row)">
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
/* t32 空态引导：明确文案 + 可点击入口，不伪造数据 */
.empty-guide {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12rpx;
  padding: 56rpx 32rpx;
  text-align: center;
  color: var(--color-text-secondary);
}
.empty-guide-emoji {
  font-size: 64rpx;
  line-height: 1.2;
}
.empty-guide-text {
  font-size: 30rpx;
  font-weight: 600;
  color: var(--color-text-primary);
}
.empty-guide-hint {
  font-size: 24rpx;
  line-height: 1.6;
  color: var(--color-text-muted);
}
.empty-guide-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 16rpx;
  margin-top: 12rpx;
}
.empty-guide-link {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 44px;
  padding: 0 28rpx;
  border-radius: 999rpx;
  border: 1rpx solid var(--color-border);
  color: var(--color-text-secondary);
  font-size: 26rpx;
}
.empty-guide-link.primary {
  background: var(--color-primary-soft);
  border-color: var(--color-primary);
  color: var(--color-primary);
}

.page {
  min-height: 100vh;
  background: var(--color-bg-page);
}
.toolbar {
  display: flex;
  gap: 16rpx;
  padding: 24rpx;
  background: var(--color-surface);
}
.tool {
  flex: 1;
  font-size: 26rpx;
}
.tip {
  padding: 120rpx 0;
  text-align: center;
  color: var(--color-text-secondary);
  font-size: 28rpx;
}
.tip-error {
  padding: 60rpx 32rpx;
  color: var(--color-error);
  font-size: 28rpx;
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
  border: 1rpx solid var(--color-error);
  color: var(--color-error);
  font-size: 26rpx;
}
.error-banner {
  padding: 24rpx 32rpx;
  background: var(--color-error-tint);
  border-bottom: 1px solid var(--color-divider);
}
.page-body {
  background: var(--color-surface);
}
.section {
  padding-top: 8rpx;
}
.section-title {
  font-size: 28rpx;
  font-weight: 700;
  color: var(--color-text-primary);
  padding: 20rpx 32rpx 8rpx;
  border-left: 6rpx solid var(--color-primary);
  margin-left: 24rpx;
}
.list {
  background: var(--color-surface);
}
.item {
  display: flex;
  align-items: center;
  gap: 20rpx;
  padding: 28rpx 32rpx;
  border-bottom: 1px solid var(--color-divider);
}
.icon {
  font-size: 40rpx;
}
.body {
  flex: 1;
}
.title {
  font-size: 30rpx;
  color: var(--color-text-primary);
  font-weight: 600;
}
.meta {
  margin-top: 8rpx;
  font-size: 24rpx;
  color: var(--color-text-secondary);
}
</style>
