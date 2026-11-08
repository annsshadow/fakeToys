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

// 后端 collect_list 返回全员收藏（与桌面 CollectApp 同口径），客户端按本人 personId 过滤。
async function load() {
  if (!session.user?.unique) return
  loading.value = true
  loadError.value = ''
  const result = await loadCollectRows(() => collectApi.list(), session.user.unique, rows.value)
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
</script>

<template>
  <view class="page">
    <view v-if="loading && !loaded" class="tip">加载中…</view>
    <view v-else-if="rows.length === 0 && loadError" class="tip tip-error">
      <view class="error-text">{{ loadError }}</view>
      <view class="retry-btn" @tap="load">重试</view>
    </view>
    <view v-else-if="loaded && rows.length === 0" class="tip">暂无收藏</view>
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
.page {
  padding: 12px;
  min-height: 100vh;
}
.tip {
  text-align: center;
  color: #8a94a6;
  padding: 40px 0;
  font-size: 14px;
}
.tip-error {
  color: #c0392b;
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
  border: 1px solid #c0392b;
  color: #c0392b;
  font-size: 13px;
}
.error-banner {
  padding: 12px 16px;
  text-align: center;
  border-bottom: 1px solid #f0f2f5;
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
  background: #ffffff;
}
.item-main {
  flex: 1;
  min-width: 0;
}
.item-title {
  font-size: 15px;
  font-weight: 600;
  color: #1f2733;
}
.item-url {
  margin-top: 4px;
  font-size: 12px;
  color: #8a94a6;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.item-del {
  margin-left: 10px;
  padding: 4px 10px;
  border-radius: 12px;
  background: #fdecec;
  color: #d65050;
  font-size: 12px;
}
.item-del.disabled {
  opacity: 0.6;
  pointer-events: none;
}
</style>
