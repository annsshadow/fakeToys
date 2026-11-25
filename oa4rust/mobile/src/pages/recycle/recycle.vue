<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
/**
 * 回收站（阶段 G / F5）——只读列表 + 单条彻底删除 + 清空。
 * 端点：GET /api/recycle/list、DELETE /api/recycle/delete/{id}、DELETE /api/recycle/empty。
 */
import { onShow } from '@dcloudio/uni-app'
import { computed, ref } from 'vue'
import { recycleApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'
import { createLatestGate } from '@/utils/latest'
import { listState, loadList } from '@/utils/loadList'

type Row = Record<string, unknown>

const rows = ref<Row[]>([])
const loading = ref(false)
const loadError = ref('')
const busy = ref(false)
const listView = computed(() =>
  listState({ loading: loading.value, loaded: true, count: rows.value.length, error: loadError.value }),
)

async function load(previousRows: Row[] = rows.value): Promise<void> {
  loading.value = true
  loadError.value = ''
  const result = await loadList<Row>(() => recycleApi.list(), {
    previousRows,
    label: '回收站加载失败',
  })
  rows.value = result.rows
  if (!result.ok) loadError.value = result.error
  loading.value = false
}

onShow(async () => {
  if (!(await ensureAuthenticated())) return
  void load()
})

function nameOf(r: Row): string {
  return String(r.name ?? r.title ?? r.fileName ?? r.id ?? '未命名')
}
function metaOf(r: Row): string {
  const t = r.deletedAt ?? r.deleteTime ?? r.createTime
  return t ? String(t) : ''
}

/** 彻底删除写链信封判定：type 为主不强求 bool；后端成功字段为 id/deleted:true 无统一权威 bool 名。 */
function recycleDeleteSucceeded(resp: any): boolean {
  if (!resp || typeof resp !== 'object') return false
  if (resp.type === 'error' || resp.success === false) return false
  if (resp.type === 'success') return true
  if (resp.type) return false
  return true
}
async function remove(r: Row): Promise<void> {
  const id = String(r.id ?? '')
  if (!id || busy.value) return
  busy.value = true
  try {
    const res = await new Promise<boolean>((resolve) => {
      uni.showModal({
        title: '彻底删除',
        content: `删除「${nameOf(r)}」？此操作不可恢复。`,
        success: (m) => resolve(Boolean(m.confirm)),
        fail: () => resolve(false),
      })
    })
    if (!res) return
    const lastResp: any = await recycleApi.remove(id)
    if (!recycleDeleteSucceeded(lastResp)) {
      uni.showToast({ title: lastResp?.message || '删除失败，请刷新后重试', icon: 'none' })
      return
    }
    uni.showToast({ title: '已删除', icon: 'success' })
    // 刷新失败时 loadList 回退 previousRows；排除已删行，避免"已删除"提示后条目重现。
    await load(rows.value.filter((r) => String(r.id ?? '') !== id))
  } catch (e) {
    uni.showToast({ title: e instanceof Error ? e.message : '删除失败', icon: 'none' })
  } finally {
    busy.value = false
  }
}

// 单条详情（recycle/{id}）
const detail = ref<Row | null>(null)
const detailGate = createLatestGate()
async function showDetail(r: Row): Promise<void> {
  const id = String(r.id ?? '')
  if (!id) {
    detail.value = r
    return
  }
  const token = detailGate.begin()
  try {
    const fetched = ((await recycleApi.detail(id)) as { data?: Row }).data ?? r
    if (!detailGate.isLatest(token)) return
    detail.value = fetched
  } catch {
    if (!detailGate.isLatest(token)) return
    detail.value = r
  }
}
function detailText(): string {
  if (!detail.value) return ''
  try {
    return JSON.stringify(detail.value, null, 2)
  } catch {
    return String(detail.value)
  }
}

async function emptyAll(): Promise<void> {
  if (busy.value) return
  busy.value = true
  try {
    const res = await new Promise<boolean>((resolve) => {
      uni.showModal({
        title: '清空回收站',
        content: '所有项目将被永久删除，确定继续？',
        success: (m) => resolve(Boolean(m.confirm)),
        fail: () => resolve(false),
      })
    })
    if (!res) return
    await recycleApi.empty()
    uni.showToast({ title: '已清空', icon: 'success' })
    // 清空成功后刷新失败也不能让旧行重现，previousRows 传空。
    await load([])
  } catch (e) {
    uni.showToast({ title: e instanceof Error ? e.message : '清空失败', icon: 'none' })
  } finally {
    busy.value = false
  }
}
function goDoc() {
  uni.navigateTo({ url: '/pages/doc/doc' })
}

</script>

<template>
  <view class="page ai-surface-gradient">
    <view v-if="listView === 'loading'" class="ai-skeleton-list" aria-hidden="true">
      <view v-for="i in 3" :key="i" class="ai-skeleton-line"></view>
      <view class="ai-skeleton-line short"></view>
    </view>
    <view v-else-if="listView === 'error'" class="tip tip-error">
      <view class="error-text">{{ loadError }}</view>
      <view class="retry-btn" @tap="load()" role="button" tabindex="0" aria-label="重试" @keydown.enter="load()" @keydown.space.prevent="load()">重试</view>
    </view>
    <view v-else-if="listView === 'empty'" class="empty-guide ai-brand-fill">
      <view class="empty-guide-emoji" aria-hidden="true">🗑️</view>
      <text class="empty-guide-text">回收站为空</text>
      <text class="empty-guide-hint">删除的内容会出现在这里，便于查看或彻底清除</text>
      <view class="empty-guide-actions">
        <view class="empty-guide-link ai-pressable" @tap="load()" role="button" tabindex="0" aria-label="刷新" @keydown.enter="load()" @keydown.space.prevent="load()">刷新</view>
        <view class="empty-guide-link ai-pressable primary" @tap="goDoc" role="button" tabindex="0" aria-label="去文档" @keydown.enter="goDoc" @keydown.space.prevent="goDoc">去文档</view>
      </view>
    </view>

    <view v-if="loadError && rows.length > 0" class="tip-error error-banner">
      <text class="error-text">{{ loadError }}</text>
      <view class="retry-btn" @tap="load()" role="button" tabindex="0" aria-label="重试" @keydown.enter="load()" @keydown.space.prevent="load()">重试</view>
    </view>
    <view v-for="(r, i) in rows" :key="String(r.id ?? i)" class="card">
      <view class="card-title" @tap="showDetail(r)" role="button" tabindex="0" :aria-label="'查看' + nameOf(r) + '详情'" @keydown.enter="showDetail(r)" @keydown.space.prevent="showDetail(r)">{{ nameOf(r) }}</view>
      <view v-if="metaOf(r)" class="card-meta">{{ metaOf(r) }}</view>
      <button class="btn danger" :disabled="busy" @tap="remove(r)">彻底删除</button>
    </view>

    <view v-if="detail" class="card detail">
      <view class="card-head">
        <text class="card-title">{{ nameOf(detail) }}</text>
        <text class="detail-close" @tap="detail = null" role="button" tabindex="0" aria-label="关闭详情" @keydown.enter="detail = null" @keydown.space.prevent="detail = null">关闭</text>
      </view>
      <text class="detail-body">{{ detailText() }}</text>
    </view>

    <button v-if="rows.length > 0" class="btn empty" :disabled="busy" @tap="emptyAll">清空回收站</button>
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
  padding: 24rpx;
  box-sizing: border-box;
}
.card {
  background: var(--color-surface);
  border-radius: 16rpx;
  padding: 24rpx;
  margin-bottom: 20rpx;
}
.card-title {
  font-size: 30rpx;
  font-weight: 700;
  color: var(--color-text-primary);
}
.card-meta {
  margin-top: 8rpx;
  font-size: 24rpx;
  color: var(--color-text-secondary);
}
.btn {
  margin-top: 16rpx;
  font-size: 26rpx;
  border-radius: 12rpx;
  background: var(--color-surface);
  color: var(--color-text-body);
  border: 1px solid var(--color-border-strong);
}
.btn.danger {
  color: var(--color-error);
  border-color: var(--color-error-tint);
}
.btn.empty {
  background: var(--color-error);
  color: var(--color-surface);
  border: none;
  margin-top: 8rpx;
}
.btn[disabled] {
  opacity: 0.6;
}
.tip {
  color: var(--color-text-secondary);
  font-size: 26rpx;
  text-align: center;
  padding: 80rpx 0;
}
.tip-error {
  color: var(--color-error);
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
  text-align: center;
  background: var(--color-error-tint);
  border-bottom: 1px solid var(--color-divider);
  margin-bottom: 20rpx;
}
.card-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12rpx;
}
.detail-close {
  font-size: 26rpx;
  color: var(--color-primary);
}
.detail-body {
  display: block;
  margin-top: 12rpx;
  font-size: 22rpx;
  color: var(--color-text-body);
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-all;
}
</style>
