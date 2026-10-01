<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { collectApi } from '@/services'
import { useSession } from '@/store/session'
import { ensureAuthenticated } from '@/utils/auth-guard'

const session = useSession()
const rows = ref<{ id: string; title: string; url: string }[]>([])
const loading = ref(false)
const loaded = ref(false)

// 后端 collect_list 返回全员收藏（与桌面 CollectApp 同口径），客户端按本人 personId 过滤。
function titleOf(row: Record<string, unknown>): string {
  const v = row.title
  return typeof v === 'string' && v ? v : '（无标题）'
}
function urlOf(row: Record<string, unknown>): string {
  const v = row.url
  return typeof v === 'string' ? v : ''
}

async function load() {
  if (!session.user?.unique) return
  loading.value = true
  try {
    const resp = await collectApi.list()
    const mine = session.user.unique
    rows.value = (resp.data ?? [])
      .filter((r) => r.personId === mine || r.person_id === mine)
      .map((r) => ({
        id: typeof r.id === 'string' ? r.id : '',
        title: titleOf(r),
        url: urlOf(r),
      }))
      .filter((r) => r.id)
  } catch {
    rows.value = []
  } finally {
    loading.value = false
    loaded.value = true
  }
}

function openUrl(row: { url: string }) {
  if (!row.url) return
  uni.setClipboardData({
    data: row.url,
    success: () => uni.showToast({ title: '链接已复制', icon: 'none' }),
  })
}

async function removeRow(row: { id: string; title: string }) {
  try {
    await collectApi.remove(row.id)
    rows.value = rows.value.filter((r) => r.id !== row.id)
    uni.showToast({ title: '已删除', icon: 'none' })
  } catch {
    uni.showToast({ title: '删除失败，请重试', icon: 'none' })
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
    <view v-else-if="loaded && rows.length === 0" class="tip">暂无收藏</view>
    <view v-else class="list">
      <view v-for="r in rows" :key="r.id" class="item" @tap="openUrl(r)">
        <view class="item-main">
          <view class="item-title">{{ r.title }}</view>
          <view v-if="r.url" class="item-url">{{ r.url }}</view>
        </view>
        <view class="item-del" @tap.stop="removeRow(r)">删除</view>
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
</style>
