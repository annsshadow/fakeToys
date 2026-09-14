<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { ref } from 'vue'
import { orgApi } from '@/services'
import { ensureAuthenticated } from '@/utils/auth-guard'

const keyword = ref('')
const rows = ref<Record<string, unknown>[]>([])
const loading = ref(false)
const loaded = ref(false)

function nameOf(row: Record<string, unknown>): string {
  for (const k of ['name', 'displayName']) {
    const v = row[k]
    if (typeof v === 'string' && v) return v
  }
  return '未知成员'
}
function subOf(row: Record<string, unknown>): string {
  const parts: string[] = []
  for (const k of ['mobile', 'email', 'job']) {
    const v = row[k]
    if (typeof v === 'string' && v) parts.push(v)
  }
  return parts.join(' · ')
}

async function search() {
  loading.value = true
  try {
    const resp = await orgApi.personSearch(keyword.value.trim() || undefined)
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
  if (!loaded.value) search()
})

function doSearch() {
  void search()
}
</script>

<template>
  <view class="page">
    <view class="search">
      <input
        v-model="keyword"
        class="input"
        type="text"
        placeholder="搜索姓名 / 手机号 / 邮箱"
        confirm-type="search"
        @confirm="doSearch"
      />
      <button class="go" size="mini" type="primary" :disabled="loading" @tap="doSearch">
        搜索
      </button>
    </view>

    <view v-if="loading && !loaded" class="tip">加载中…</view>
    <view v-else-if="rows.length === 0" class="tip">{{ keyword ? '未找到相关人员' : '暂无联系人' }}</view>
    <view v-else class="list">
      <view v-for="(row, i) in rows" :key="i" class="item">
        <text class="avatar">{{ nameOf(row).slice(0, 1).toUpperCase() }}</text>
        <view class="body">
          <view class="title">{{ nameOf(row) }}</view>
          <view class="meta">{{ subOf(row) || ' ' }}</view>
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
.search {
  display: flex;
  gap: 16rpx;
  padding: 24rpx;
  background: #fff;
}
.input {
  flex: 1;
  border: 1px solid #e3e6eb;
  border-radius: 32rpx;
  padding: 14rpx 24rpx;
  font-size: 28rpx;
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
.avatar {
  width: 72rpx;
  height: 72rpx;
  border-radius: 50%;
  background: #2d8cf0;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32rpx;
  font-weight: 700;
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
  margin-top: 6rpx;
  font-size: 24rpx;
  color: #90979f;
}
</style>
