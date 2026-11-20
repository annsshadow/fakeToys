<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { collectApi } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'
import { loadCollectRows } from './collectFeed'

const session = useSession()
const rows = ref<{ id: string; title: string; url: string }[]>([])
const loading = ref(false)
const loaded = ref(false)
const loadError = ref('')
const removingId = ref('')

// onShow 与两处重试按钮共用 load：序号只让最新一次回写，避免旧响应后到覆盖新数据。
let loadSeq = 0

// 后端 collect_list 返回全员收藏（与桌面 CollectApp 同口径），客户端按本人 personId 过滤。
async function load() {
  if (!session.user?.unique) return
  const seq = ++loadSeq
  loading.value = true
  loadError.value = ''
  const result = await loadCollectRows(() => collectApi.list(), session.user.unique, rows.value)
  if (seq !== loadSeq) return
  rows.value = result.rows
  if (!result.ok) loadError.value = result.error
  loading.value = false
  loaded.value = true
}

function openUrl(row: { url: string }) {
  if (!row.url) return
  uni.setClipboardData({
    data: row.url,
    success: () => uni.showToast({ title: '链接已复制', icon: 'none' }),
  })
}

async function removeRow(row: { id: string; title: string }) {
  if (removingId.value) return
  // 删除不可恢复：先确认，再调接口；整个确认+请求期间加守卫，避免连点开多个确认框。
  removingId.value = row.id
  try {
    const confirmed = await new Promise<boolean>((resolve) => {
      uni.showModal({
        title: '删除收藏',
        content: `删除「${row.title}」后不可恢复，确定继续？`,
        success: (m) => resolve(Boolean(m.confirm)),
        fail: () => resolve(false),
      })
    })
    if (!confirmed) return
    await collectApi.remove(row.id)
    rows.value = rows.value.filter((r) => r.id !== row.id)
    uni.showToast({ title: '已删除', icon: 'none' })
  } catch {
    uni.showToast({ title: '删除失败，请重试', icon: 'none' })
  } finally {
    removingId.value = ''
  }
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  load()
})
function goBrowse() {
  uni.navigateTo({ url: '/pages/browse/browse' })
}

</script>

<template>
  <view class="page ai-surface-gradient">
    <view v-if="loading && !loaded" class="ai-skeleton-list" aria-hidden="true">
      <view v-for="i in 3" :key="i" class="ai-skeleton-line"></view>
      <view class="ai-skeleton-line short"></view>
    </view>
    <view v-else-if="rows.length === 0 && loadError" class="tip tip-error">
      <view class="error-text">{{ loadError }}</view>
      <view class="retry-btn" @tap="load">重试</view>
    </view>
    <view v-else-if="loaded && rows.length === 0" class="empty-guide">
      <view class="empty-guide-emoji" aria-hidden="true">⭐</view>
      <text class="empty-guide-text">暂无收藏</text>
      <text class="empty-guide-hint">还没有收藏，浏览内容时收藏后可在这里快速找到</text>
      <view class="empty-guide-actions">
        <view class="empty-guide-link ai-pressable primary" @tap="goBrowse">去浏览内容</view>
        <view class="empty-guide-link ai-pressable" @tap="load">刷新</view>
      </view>
    </view>
    <view v-else class="list">
      <view v-if="loadError" class="tip-error error-banner">
        <text class="error-text">{{ loadError }}</text>
        <view class="retry-btn" @tap="load">重试</view>
      </view>
      <view v-for="r in rows" :key="r.id" class="item" @tap="openUrl(r)">
        <view class="item-main">
          <view class="item-title">{{ r.title }}</view>
          <view v-if="r.url" class="item-url">{{ r.url }}</view>
        </view>
        <view class="item-del" :class="{ disabled: removingId !== '' }" @tap.stop="removeRow(r)">删除</view>
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
  padding: 12px;
  min-height: 100vh;
}
.tip {
  text-align: center;
  color: var(--color-text-muted);
  padding: 40px 0;
  font-size: 14px;
}
.tip-error {
  color: var(--color-error);
}
.error-text {
  display: block;
  margin-bottom: 6px;
  line-height: 1.5;
}
.retry-btn {
  display: inline-block;
  padding: 4px 14px;
  border-radius: 12px;
  border: 1px solid var(--color-error);
  color: var(--color-error);
  font-size: 13px;
}
.error-banner {
  padding: 12px 16px;
  text-align: center;
  border-bottom: 1px solid var(--color-divider);
}
.list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px;
  border-radius: 10px;
  background: var(--color-surface);
}
.item-main {
  flex: 1;
  min-width: 0;
}
.item-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--color-text-primary);
}
.item-url {
  margin-top: 4px;
  font-size: 12px;
  color: var(--color-text-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.item-del {
  margin-left: 10px;
  padding: 4px 10px;
  border-radius: 12px;
  background: var(--color-error-bg);
  color: var(--color-error);
  font-size: 12px;
}
.item-del.disabled {
  opacity: 0.6;
  pointer-events: none;
}
</style>
