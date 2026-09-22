<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
/**
 * 回收站（阶段 G / F5）——只读列表 + 单条彻底删除 + 清空。
 * 端点：GET /api/recycle/list、DELETE /api/recycle/delete/{id}、DELETE /api/recycle/empty。
 */
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { recycleApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'

type Row = Record<string, unknown>

const rows = ref<Row[]>([])
const loading = ref(false)

async function load(): Promise<void> {
  loading.value = true
  try {
    const r = await recycleApi.list()
    rows.value = (r.data ?? []) as Row[]
  } catch {
    rows.value = []
  } finally {
    loading.value = false
  }
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

async function remove(r: Row): Promise<void> {
  const id = String(r.id ?? '')
  if (!id) return
  const res = await new Promise<boolean>((resolve) => {
    uni.showModal({
      title: '彻底删除',
      content: `删除「${nameOf(r)}」？此操作不可恢复。`,
      success: (m) => resolve(Boolean(m.confirm)),
      fail: () => resolve(false),
    })
  })
  if (!res) return
  try {
    await recycleApi.remove(id)
    uni.showToast({ title: '已删除', icon: 'success' })
    await load()
  } catch (e) {
    uni.showToast({ title: e instanceof Error ? e.message : '删除失败', icon: 'none' })
  }
}

// 单条详情（recycle/{id}）
const detail = ref<Row | null>(null)
async function showDetail(r: Row): Promise<void> {
  const id = String(r.id ?? '')
  if (!id) {
    detail.value = r
    return
  }
  try {
    detail.value = ((await recycleApi.detail(id)) as { data?: Row }).data ?? r
  } catch {
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
  const res = await new Promise<boolean>((resolve) => {
    uni.showModal({
      title: '清空回收站',
      content: '所有项目将被永久删除，确定继续？',
      success: (m) => resolve(Boolean(m.confirm)),
      fail: () => resolve(false),
    })
  })
  if (!res) return
  try {
    await recycleApi.empty()
    uni.showToast({ title: '已清空', icon: 'success' })
    await load()
  } catch (e) {
    uni.showToast({ title: e instanceof Error ? e.message : '清空失败', icon: 'none' })
  }
}
</script>

<template>
  <view class="page">
    <view v-if="loading && rows.length === 0" class="tip">加载中…</view>
    <view v-else-if="rows.length === 0" class="tip">回收站为空</view>

    <view v-for="(r, i) in rows" :key="String(r.id ?? i)" class="card">
      <view class="card-title" @tap="showDetail(r)">{{ nameOf(r) }}</view>
      <view v-if="metaOf(r)" class="card-meta">{{ metaOf(r) }}</view>
      <button class="btn danger" @tap="remove(r)">彻底删除</button>
    </view>

    <view v-if="detail" class="card detail">
      <view class="card-head">
        <text class="card-title">{{ nameOf(detail) }}</text>
        <text class="detail-close" @tap="detail = null">关闭</text>
      </view>
      <text class="detail-body">{{ detailText() }}</text>
    </view>

    <button v-if="rows.length > 0" class="btn empty" @tap="emptyAll">清空回收站</button>
  </view>
</template>

<style scoped>
.page {
  min-height: 100vh;
  background: #f5f7fa;
  padding: 24rpx;
  box-sizing: border-box;
}
.card {
  background: #fff;
  border-radius: 16rpx;
  padding: 24rpx;
  margin-bottom: 20rpx;
}
.card-title {
  font-size: 30rpx;
  font-weight: 700;
  color: #263238;
}
.card-meta {
  margin-top: 8rpx;
  font-size: 24rpx;
  color: #90979f;
}
.btn {
  margin-top: 16rpx;
  font-size: 26rpx;
  border-radius: 12rpx;
  background: #fff;
  color: #5b6572;
  border: 1px solid #dcdfe6;
}
.btn.danger {
  color: #f56c6c;
  border-color: #fbc4c4;
}
.btn.empty {
  background: #f56c6c;
  color: #fff;
  border: none;
  margin-top: 8rpx;
}
.tip {
  color: #90979f;
  font-size: 26rpx;
  text-align: center;
  padding: 80rpx 0;
}
.card-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12rpx;
}
.detail-close {
  font-size: 26rpx;
  color: #2d8cf0;
}
.detail-body {
  display: block;
  margin-top: 12rpx;
  font-size: 22rpx;
  color: #5b6572;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-all;
}
</style>
